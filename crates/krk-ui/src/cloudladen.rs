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

    /// Ein Faden, der auf einen belegten Platz wartet, gibt binnen 20 ms nach
    /// dem Abbruch auf, ohne zu belegen.
    #[test]
    fn ein_wartender_gibt_nach_dem_abbruch_ohne_belegung_auf() {
        let platz = Arc::new(Ladeplatz::neu());
        let gehalten = platz
            .belegen(&|| true)
            .expect("der freie Platz wird belegt");
        let marke = Arc::new(AtomicBool::new(false));
        let (meldung, ankunft) = std::sync::mpsc::channel();
        let faden = {
            let platz = Arc::clone(&platz);
            let marke = Arc::clone(&marke);
            thread::spawn(move || {
                let belegt = platz.belegen(&|| !marke.load(Ordering::Relaxed)).is_some();
                meldung.send(Instant::now()).expect("die Probe hoert zu");
                belegt
            })
        };
        // Der Faden steht jetzt im Warten auf den Platz.
        thread::sleep(Duration::from_millis(50));
        let abgebrochen = Instant::now();
        marke.store(true, Ordering::Relaxed);
        let zurueck = ankunft
            .recv_timeout(Duration::from_secs(5))
            .expect("der Wartende kehrt zurueck");
        assert!(
            zurueck.duration_since(abgebrochen) < Duration::from_millis(20),
            "der Wartende brauchte {:?} nach dem Abbruch",
            zurueck.duration_since(abgebrochen)
        );
        assert!(
            !faden.join().expect("der Faden endet ohne Absturz"),
            "nach dem Abbruch wird nicht belegt"
        );
        drop(gehalten);
        assert_eq!(platz.belegungen(), 1, "allein die Probe hat belegt");
    }

    /// `abwarten` kehrt nach einem Nein binnen 20 ms mit `false` zurueck und
    /// mit Ja nach der Dauer mit `true`.
    #[test]
    fn abwarten_bricht_ab_oder_wartet_die_dauer() {
        let beginn = Instant::now();
        assert!(!abwarten(Duration::from_secs(5), &|| false));
        assert!(
            beginn.elapsed() < Duration::from_millis(20),
            "das Nein ab dem ersten Schritt brauchte {:?}",
            beginn.elapsed()
        );

        let beginn = Instant::now();
        assert!(abwarten(Duration::from_millis(50), &|| true));
        assert!(
            beginn.elapsed() >= Duration::from_millis(50),
            "mit Ja kehrt abwarten erst nach der Dauer zurueck"
        );
    }

    /// Ein Nein mitten in der Wartezeit beendet sie binnen 20 ms.
    #[test]
    fn ein_nein_mitten_in_der_wartezeit_beendet_sie() {
        let marke = Arc::new(AtomicBool::new(false));
        let faden = {
            let marke = Arc::clone(&marke);
            thread::spawn(move || {
                let ergebnis = abwarten(Duration::from_secs(5), &|| !marke.load(Ordering::Relaxed));
                (ergebnis, Instant::now())
            })
        };
        thread::sleep(Duration::from_millis(30));
        let abgebrochen = Instant::now();
        marke.store(true, Ordering::Relaxed);
        let (ergebnis, zurueck) = faden.join().expect("der Faden endet ohne Absturz");
        assert!(!ergebnis);
        assert!(
            zurueck.duration_since(abgebrochen) < Duration::from_millis(20),
            "das Nein brauchte {:?}",
            zurueck.duration_since(abgebrochen)
        );
    }
}
