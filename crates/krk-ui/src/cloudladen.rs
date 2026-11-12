//! Ein Ladeplatz und eine abbrechbare Wartezeit fuer die Vorschau einer
//! Datei, die nur in der Cloud liegt.
//!
//! **Keine Zeile AppKit und keine Vorschau.** Das Modul kennt weder ein
//! Fenster noch einen [`Inhalt`](crate::vorschaumodell::Inhalt); es haelt die
//! zwei Bausteine, mit denen der Arbeitsfaden der Vorschau einen Download
//! erst nach einer Ruhezeit und hoechstens einen zur Zeit ausloest. Wie die
//! Vorschau sie zusammensetzt, steht in `crate::vorschaumodell` unter
//! „Eine Datei, die nur in der Cloud liegt“.
//!
//! # Was „abbrechen“ hier heisst
//!
//! Die Antwort ist die aus Moeglichkeit 1 des Entscheids
//! `261025-1444-was-heisst-abbrechen-fuer-einen-download-den-die-vorschau-schon-angestossen-hat.md`:
//! **ein Download, der noch nicht begonnen hat, beginnt nach einem
//! Auswahlwechsel nicht mehr, und hoechstens ein von der Vorschau
//! ausgeloester Download laeuft zugleich.** Die Wartezeit ([`abwarten`]) und
//! das Warten auf den Platz ([`Ladeplatz::belegen`]) fragen beide in kurzen
//! Schritten die Frage `weiter` des Rufers, hinter der die Abbruchmarke des
//! Vorgangs steht; sagt sie nein, endet das Warten ohne Belegung, und
//! geoeffnet wird nichts.
//!
//! **Einen laufenden Download beendet dieses Modul nicht.** Die
//! [`Belegung`] ist ein Waechter, der den Platz in `Drop` freigibt. Steht der
//! Faden, der ihn haelt, in einem `open(2)` oder `read(2)`, das auf den
//! File-Provider wartet, faellt der Waechter erst, wenn dieser Systemaufruf
//! zurueckgekehrt ist; bis dahin wartet die naechste Vorschau auf den Platz.
//! Das ist gewollt: so gilt „hoechstens einer zugleich“ auch fuer einen
//! ueberholten Download, und was er noch kostet, begrenzt die Groessengrenze
//! der Vorschau.
//!
//! **Kein `unsafe`, kein `objc2`.** Der Platz ist ein `Mutex` mit einer
//! `Condvar`; gewartet wird mit einer Zeitschranke von [`SCHRITT`], damit
//! auch ein Wartender, den niemand weckt, seine Marke liest.

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

/// Wie lange die Auswahl auf einer Datei, die nur in der Cloud liegt, ruhen
/// muss, bevor die Vorschau sie laedt (Vorgabe des Nutzers vom 261025).
///
/// Gilt allein fuer Platzhalter: eine lokale Datei wartet nicht, weil L7 die
/// Vorschau einer lokalen Datei in 100 ms zusagt.
pub const CLOUDVERZOEGERUNG: Duration = Duration::from_millis(200);

/// Der laengste Schritt, in dem gewartet wird, bevor die Frage `weiter`
/// erneut gestellt wird. Ein Auswahlwechsel beendet ein Warten damit
/// spaetestens nach dieser Zeit.
const SCHRITT: Duration = Duration::from_millis(10);

/// Wartet `dauer` und fragt dabei hoechstens alle [`SCHRITT`] die Frage
/// `weiter`.
///
/// Liefert `false`, sobald `weiter` nein sagt, sonst nach der Dauer die
/// letzte Antwort von `weiter`, also `true`, wenn bis dahin niemand
/// abgebrochen hat.
#[must_use = "false heisst, dass der Rufer abbrechen soll"]
pub fn abwarten(dauer: Duration, weiter: &dyn Fn() -> bool) -> bool {
    let beginn = Instant::now();
    loop {
        if !weiter() {
            return false;
        }
        let verstrichen = beginn.elapsed();
        if verstrichen >= dauer {
            return true;
        }
        thread::sleep(SCHRITT.min(dauer - verstrichen));
    }
}

/// Der eine Platz, den ein Download der Vorschau belegt.
///
/// Geteilt als `Arc` zwischen dem Modell und jedem Arbeitsfaden, den es
/// startet. Wer laden will, belegt ihn ueber [`Ladeplatz::belegen`] und haelt
/// die [`Belegung`], solange er liest.
#[derive(Debug, Default)]
pub struct Ladeplatz {
    stand: Mutex<Platzstand>,
    frei: Condvar,
}

/// Was hinter der Sperre des Platzes steht.
#[derive(Debug, Default)]
struct Platzstand {
    /// Wie viele Belegungen gerade bestehen; nach dem Bau des Platzes
    /// hoechstens eine.
    aktive: usize,
    /// Wie oft der Platz belegt wurde, fuer die Proben.
    #[cfg(test)]
    belegungen: usize,
    /// Wie viele Belegungen hoechstens zugleich bestanden, fuer die Proben.
    #[cfg(test)]
    hoechstens_zugleich: usize,
}

impl Ladeplatz {
    /// Ein freier Platz.
    #[must_use]
    pub fn neu() -> Self {
        Self::default()
    }

    /// Die Sperre des Platzes.
    ///
    /// **Eine vergiftete Sperre wird weiter benutzt**: vergiftet ist sie,
    /// wenn ein Faden mit gehaltener Sperre abgestuerzt ist, und der Stand
    /// darin ist dann eine Zahl, die vor dem Absturz geschrieben wurde. Ein
    /// Abbruch hier liesse jede weitere Vorschau einer Cloud-Datei stehen.
    fn sperren(&self) -> MutexGuard<'_, Platzstand> {
        self.stand.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Belegt den Platz, sobald er frei ist, oder gibt auf, sobald `weiter`
    /// nein sagt.
    ///
    /// Gefragt wird `weiter` vor jedem Versuch und nach jedem Warten von
    /// hoechstens [`SCHRITT`], also endet ein abgebrochenes Warten spaetestens
    /// nach dieser Zeit. `None` heisst: nicht belegt, der Rufer oeffnet
    /// nichts.
    #[must_use = "die Belegung haelt den Platz; faellt sie sofort, ist er sofort wieder frei"]
    pub fn belegen(&self, weiter: &dyn Fn() -> bool) -> Option<Belegung<'_>> {
        let mut stand = self.sperren();
        loop {
            if !weiter() {
                return None;
            }
            if stand.aktive == 0 {
                stand.aktive = 1;
                #[cfg(test)]
                {
                    stand.belegungen += 1;
                    stand.hoechstens_zugleich = stand.hoechstens_zugleich.max(stand.aktive);
                }
                return Some(Belegung { platz: self });
            }
            stand = self
                .frei
                .wait_timeout(stand, SCHRITT)
                .map(|(stand, _)| stand)
                .unwrap_or_else(|vergiftet| vergiftet.into_inner().0);
        }
    }

    /// Wie oft der Platz bisher belegt wurde.
    #[cfg(test)]
    #[must_use]
    pub fn belegungen(&self) -> usize {
        self.sperren().belegungen
    }

    /// Wie viele Belegungen hoechstens zugleich bestanden.
    #[cfg(test)]
    #[must_use]
    pub fn hoechstens_zugleich(&self) -> usize {
        self.sperren().hoechstens_zugleich
    }
}

/// Der Waechter einer Belegung: solange er lebt, ist der Platz belegt.
///
/// Er faellt am Ende des Lesens, auch eines abgebrochenen, und erst dann; ein
/// Faden, der in einem Systemaufruf steht, gibt den Platz also erst nach
/// dessen Rueckkehr frei (siehe den Modulkopf).
#[derive(Debug)]
pub struct Belegung<'a> {
    platz: &'a Ladeplatz,
}

impl Drop for Belegung<'_> {
    fn drop(&mut self) {
        let mut stand = self.platz.sperren();
        stand.aktive = stand.aktive.saturating_sub(1);
        drop(stand);
        self.platz.frei.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use super::*;

    /// Zwei Faeden belegen nacheinander und nie zugleich.
    #[test]
    fn zwei_faeden_belegen_nacheinander_und_nie_zugleich() {
        let platz = Arc::new(Ladeplatz::neu());
        let faeden: Vec<_> = (0..2)
            .map(|_| {
                let platz = Arc::clone(&platz);
                thread::spawn(move || {
                    for _ in 0..20 {
                        let belegung = platz.belegen(&|| true);
                        assert!(belegung.is_some(), "ohne Abbruch wird der Platz belegt");
                        thread::sleep(Duration::from_millis(1));
                    }
                })
            })
            .collect();
        for faden in faeden {
            faden.join().expect("der Faden endet ohne Absturz");
        }
        assert_eq!(platz.belegungen(), 40);
        assert_eq!(platz.hoechstens_zugleich(), 1);
    }

    /// Wie lange die Probe hoechstens auf einen Faden wartet, bevor sie ihn
    /// fuer haengend haelt: zweihundert Schritte des Wartens.
    ///
    /// **Keine Reaktionszeit, sondern eine Haengegrenze.** Sie ist am Takt
    /// [`SCHRITT`] festgemacht und nicht an einer freien Zahl, und sie liegt so
    /// weit ueber ihm, dass die Zuteilung des Fadens unter der Last der
    /// parallel laufenden Proben sie nicht erreicht; eine Wanduhrschranke in
    /// der Groesse des Takts selbst faellt dort (Defekt
    /// `261111-2349-die-zeitprobe-zum-abbruch-eines-wartenden-im-cloudladen-faellt-unter-last-an-ihrer-20-ms-schranke.md`).
    /// Was sie nicht faengt: einen Takt, der laenger wird und unter ihr bleibt.
    const HAENGT: Duration = SCHRITT.saturating_mul(200);

    /// Ein Faden, der auf einen belegten Platz wartet, liest seine Marke, ohne
    /// dass ihn jemand weckt, und gibt mit dem ersten Nein auf, ohne zu
    /// belegen.
    ///
    /// Geprueft wird an Zustaenden und nicht an Millisekunden: wie oft der
    /// Wartende `weiter` gefragt hat, wie oft die Antwort nein war und was der
    /// Platz danach zaehlt. Die einzige Zeitgrenze ist [`HAENGT`].
    #[test]
    fn ein_wartender_gibt_nach_dem_abbruch_ohne_belegung_auf() {
        use std::sync::atomic::AtomicUsize;

        let platz = Arc::new(Ladeplatz::neu());
        let gehalten = platz
            .belegen(&|| true)
            .expect("der freie Platz wird belegt");
        let marke = Arc::new(AtomicBool::new(false));
        let fragen = Arc::new(AtomicUsize::new(0));
        let neins = Arc::new(AtomicUsize::new(0));
        let (meldung, ankunft) = std::sync::mpsc::channel();
        let faden = {
            let platz = Arc::clone(&platz);
            let marke = Arc::clone(&marke);
            let fragen = Arc::clone(&fragen);
            let neins = Arc::clone(&neins);
            thread::spawn(move || {
                let weiter = || {
                    fragen.fetch_add(1, Ordering::SeqCst);
                    let abgebrochen = marke.load(Ordering::SeqCst);
                    if abgebrochen {
                        neins.fetch_add(1, Ordering::SeqCst);
                    }
                    !abgebrochen
                };
                let belegt = platz.belegen(&weiter).is_some();
                meldung.send(belegt).expect("die Probe hoert zu");
            })
        };
        // Die erste Frage steht vor dem ersten Versuch, jede weitere nach
        // einem Warten. Der Platz wird nie frei, also weckt den Faden niemand:
        // drei Fragen heissen, dass er zweimal von selbst aus dem Warten
        // zurueckgekommen ist und seine Marke gelesen hat.
        let beginn = Instant::now();
        while fragen.load(Ordering::SeqCst) < 3 {
            assert!(
                beginn.elapsed() < HAENGT,
                "der Wartende hat nach {HAENGT:?} erst {} Mal gefragt; er schlaeft an der Marke vorbei",
                fragen.load(Ordering::SeqCst)
            );
            thread::sleep(SCHRITT);
        }
        assert!(
            ankunft.try_recv().is_err(),
            "vor dem Abbruch kehrt der Wartende nicht zurueck"
        );

        marke.store(true, Ordering::SeqCst);
        let belegt = ankunft
            .recv_timeout(HAENGT)
            .expect("der Wartende kehrt nach dem Abbruch zurueck");
        faden.join().expect("der Faden endet ohne Absturz");

        assert!(!belegt, "nach dem Abbruch wird nicht belegt");
        assert_eq!(
            neins.load(Ordering::SeqCst),
            1,
            "das erste Nein beendet das Warten; ein zweites Mal wird nicht gefragt"
        );
        // Der Platz war die ganze Zeit gehalten: zurueckgekehrt ist der
        // Wartende wegen der Marke und nicht, weil der Platz frei wurde.
        assert_eq!(platz.belegungen(), 1, "allein die Probe hat belegt");
        assert_eq!(platz.hoechstens_zugleich(), 1);
        drop(gehalten);
        assert!(
            platz.belegen(&|| true).is_some(),
            "der Abbruch des Wartenden laesst den Platz benutzbar"
        );
    }

    /// `abwarten` kehrt mit dem ersten Nein zurueck, ohne zu warten, und
    /// wartet mit lauter Ja die Dauer.
    ///
    /// Die erste Haelfte steht an Zustaenden: genau eine Frage, und sie war
    /// die mit dem Nein. Ihre einzige Zeitgrenze ist [`HAENGT`], gegen eine
    /// Dauer weit darueber; sie faengt ein `abwarten`, das trotz Nein die
    /// Dauer wartet, und nicht mehr eines, das vor der ersten Frage weniger
    /// als [`HAENGT`] schlaeft.
    ///
    /// Die zweite Haelfte traegt die eine echte Zeitaussage, und allein als
    /// untere Schranke: nicht kuerzer als die Dauer. Last macht ein Warten
    /// laenger und nie kuerzer, also faellt sie unter Last nicht. Eine obere
    /// Schranke an der Wanduhr steht hier nicht.
    ///
    /// Die Zahl der Fragen haelt zweierlei und nicht mehr: `abwarten` kreist
    /// nicht ohne Schlaf (hoechstens sechs Fragen), und es antwortet nicht mit
    /// Ja, ohne gefragt zu haben (mindestens eine). Dass es die Dauer nicht in
    /// einem Stueck verschlaeft, sondern in Takten fragt, haelt sie nicht; das
    /// haelt `ein_nein_mitten_in_der_wartezeit_beendet_sie`.
    #[test]
    fn abwarten_bricht_ab_oder_wartet_die_dauer() {
        use std::sync::atomic::AtomicUsize;

        let fragen = AtomicUsize::new(0);
        let beginn = Instant::now();
        assert!(!abwarten(HAENGT.saturating_mul(10), &|| {
            fragen.fetch_add(1, Ordering::SeqCst);
            false
        }));
        assert_eq!(
            fragen.load(Ordering::SeqCst),
            1,
            "das erste Nein beendet das Warten; ein zweites Mal wird nicht gefragt"
        );
        assert!(
            beginn.elapsed() < HAENGT,
            "das Nein auf die erste Frage brauchte {:?}; abwarten wartet trotz Nein",
            beginn.elapsed()
        );

        let dauer = SCHRITT.saturating_mul(5);
        let fragen = AtomicUsize::new(0);
        let beginn = Instant::now();
        assert!(abwarten(dauer, &|| {
            fragen.fetch_add(1, Ordering::SeqCst);
            true
        }));
        assert!(
            beginn.elapsed() >= dauer,
            "mit Ja kehrt abwarten erst nach der Dauer zurueck"
        );
        // Oben: jeder Schlaf dauert mindestens einen Schritt oder den Rest der
        // Dauer, und nach einem Schlaf ueber den Rest ist die Dauer um. Also
        // liegen in fuenf Schritten Dauer hoechstens fuenf Schlaefe und damit
        // hoechstens sechs Fragen; Last macht jeden Schlaf laenger und die
        // Fragen weniger, nie mehr. Unten: die erste Frage steht vor jeder
        // Rueckkehr. Eine Zwei steht dort nicht, weil ein Faden, der zwischen
        // dem Beginn und dem Lesen der verstrichenen Zeit die ganze Dauer lang
        // nicht zugeteilt wird, zu Recht nach einer Frage zurueckkehrt.
        let gefragt = fragen.load(Ordering::SeqCst);
        assert!(
            (1..=6).contains(&gefragt),
            "in fuenf Schritten Dauer wurde {gefragt} Mal gefragt"
        );
    }

    /// Ein Nein mitten in der Wartezeit beendet sie: der Wartende fragt in
    /// Takten weiter, liest die Marke und kehrt mit dem ersten Nein zurueck.
    ///
    /// Geprueft wird an Zustaenden und nicht an Millisekunden, wie in
    /// `ein_wartender_gibt_nach_dem_abbruch_ohne_belegung_auf`: wie oft
    /// `weiter` gefragt wurde und wie oft die Antwort nein war. Die einzige
    /// Zeitgrenze ist [`HAENGT`], und die Dauer liegt weit ueber ihr, sodass
    /// eine Rueckkehr unter ihr vom Nein kommt und nicht vom Ablauf.
    #[test]
    fn ein_nein_mitten_in_der_wartezeit_beendet_sie() {
        use std::sync::atomic::AtomicUsize;

        let marke = Arc::new(AtomicBool::new(false));
        let fragen = Arc::new(AtomicUsize::new(0));
        let neins = Arc::new(AtomicUsize::new(0));
        let (meldung, ankunft) = std::sync::mpsc::channel();
        let faden = {
            let marke = Arc::clone(&marke);
            let fragen = Arc::clone(&fragen);
            let neins = Arc::clone(&neins);
            thread::spawn(move || {
                let weiter = || {
                    fragen.fetch_add(1, Ordering::SeqCst);
                    let abgebrochen = marke.load(Ordering::SeqCst);
                    if abgebrochen {
                        neins.fetch_add(1, Ordering::SeqCst);
                    }
                    !abgebrochen
                };
                let ergebnis = abwarten(HAENGT.saturating_mul(10), &weiter);
                meldung.send(ergebnis).expect("die Probe hoert zu");
            })
        };
        // Die erste Frage steht vor dem ersten Schlaf, jede weitere nach
        // einem: drei Fragen heissen, dass der Wartende mitten in der
        // Wartezeit steht und zweimal aus dem Schlaf an die Marke
        // zurueckgekommen ist.
        let beginn = Instant::now();
        while fragen.load(Ordering::SeqCst) < 3 {
            assert!(
                beginn.elapsed() < HAENGT,
                "der Wartende hat nach {HAENGT:?} erst {} Mal gefragt; er schlaeft an der Marke vorbei",
                fragen.load(Ordering::SeqCst)
            );
            thread::sleep(SCHRITT);
        }
        assert!(
            ankunft.try_recv().is_err(),
            "vor dem Nein kehrt der Wartende nicht zurueck"
        );

        marke.store(true, Ordering::SeqCst);
        let ergebnis = ankunft
            .recv_timeout(HAENGT)
            .expect("der Wartende kehrt nach dem Nein zurueck");
        faden.join().expect("der Faden endet ohne Absturz");

        assert!(!ergebnis, "nach dem Nein liefert abwarten false");
        assert_eq!(
            neins.load(Ordering::SeqCst),
            1,
            "das erste Nein beendet das Warten; ein zweites Mal wird nicht gefragt"
        );
    }
}
