//! Der eine Fadenweg, auf dem eine Frage an die Platte vom Hauptfaden
//! genommen wird und ihre Antwort ueber die Hauptschlange zurueckkommt.
//!
//! ```text
//! Hauptfaden ──> fragen_lassen ──> benannter Faden: arbeit()
//!                                          │
//!   fertig(mtm, ergebnis) <── DispatchQueue::main() <──┘
//! ```
//!
//! # Woher der Weg kommt
//!
//! Bis zum 261102 stand er privat in [`super::terminal`] und hiess dort
//! `auf_dem_faden`; `ctrl+o`, „Im Finder oeffnen" und „Im Terminal
//! bearbeiten" gingen ueber ihn, alle drei auf dem Faden `krk-terminal`.
//! Gehoben ist er, weil weitere Menue- und Tastenwege dieselbe Frage vom
//! Hauptfaden nehmen sollen (Plan
//! `261102-1314-plan-dateisystemfragen-vom-hauptfaden-nehmen.md`, Schritt 1),
//! und ein zweiter Mechanismus daneben waere der Doppelbau, den der Zuschnitt
//! von [`super::terminal`] schon einmal vermieden hat. **Neu ist allein, dass
//! der Rufer den Fadennamen angibt**; Halter, Rueckweg und das genau einmalige
//! `fertig` sind dieselben wie vorher.
//!
//! # Was der Rufer zusagt und was dieser Weg zusagt
//!
//! `arbeit` laeuft auf dem neuen Faden und bekommt ihre Eingaben als
//! gewoehnliche Rust-Werte mit, denn ein `Retained` geht nicht ueber die
//! Fadengrenze. `fertig` bekommt auf dem Hauptfaden den [`MainThreadMarker`],
//! damit der Rufer seinen Delegierten dort sucht, wo er steht. **`fertig` wird
//! genau einmal gerufen, sobald die Frage ausgegangen ist**: mit dem Ergebnis
//! von `arbeit`, und wenn schon der Fadenstart scheitert oder `arbeit` in Panik
//! geraet, mit dem Ergebnis, das `startfehler` aus einem Fehler macht. Die Panik
//! steht mit ihrer Meldung weiter auf der Standardfehlerausgabe. Das Muster ist
//! das von [`super::auswurf`].
//!
//! **Ohne Antwort bleibt der Rufer in drei Faellen**: `arbeit` kehrt nie
//! zurueck, etwa auf einer haengenden Freigabe; `startfehler` selbst geraet in
//! Panik; die Hauptschlange laeuft nie wieder. Abbrechen laesst sich ein
//! wartender Faden nicht, also darf kein Rufer etwas sperren, das erst seine
//! Antwort wieder freigibt (`Loeschbefehle` in [`super::anwendung`] ist aus
//! diesem Grund eine Nummer und keine Sperre).
//!
//! # Was ein Befehl des Nutzers dazubekommt
//!
//! Stellt ein Befehl des Nutzers die Frage, nimmt er
//! [`befehl_fragen_lassen`] und nicht [`fragen_lassen`]: bleibt die Antwort ueber
//! die [`FRIST`] hinaus aus, sagt die Statuszeile, dass
//! er wartet, und sobald die Antwort da ist, ist der Satz weg, auch wenn der Rufer
//! sie danach verwirft. Die Regel steht in [`crate::plattenwarte`]. **Eine Frage im
//! Hintergrund bleibt bei [`fragen_lassen`]**, denn auf sie wartet kein Nutzer.
//!
//! **Der Zeitgeber laeuft auf der Hauptschlange und stellt keine Frage an die
//! Platte.** Er ist ein `dispatch_after` je Frage, das einmal laeuft und nichts
//! haelt ausser zwei Zahlen, und die Statuszeile hat keinen eigenen Zeitgeber,
//! den er ersetzen koennte. Die Fragen selbst deckelt er nicht: jede weitere
//! Ausloesung startet weiter ihren Faden, eine Obergrenze hat der Nutzer nicht
//! verlangt.
//!
//! **Der Stand steht in einem `thread_local!`** und nicht im
//! Anwendungsdelegierten, damit dieser Weg nichts ueber die Schichten darueber
//! wissen muss und die Dateifenster und der Editor ihn ohne Griff auf den
//! Delegierten nehmen koennen. Zugang gibt es allein mit dem
//! [`MainThreadMarker`], also allein auf dem Hauptfaden. Was die Zeile nachzieht,
//! reicht der Aufbau als Melder herein ([`wartemelder_setzen`]).
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Der Betriebscode dieser Datei spricht keine Objective-C-Klasse an; allein
//! ihr Pruefmodul faehrt die Hauptschlange mit `NSRunLoop` und `NSDate` aus
//! Foundation, beide seit macOS 10.0, und nennt dafuer den Modus
//! `NSDefaultRunLoopMode`. `MainThreadMarker` aus
//! `objc2_foundation` ist ein Rust-Typ der Kiste und hat kein macOS-Alter;
//! `DispatchQueue` und `DispatchTime` aus `dispatch2` sind eine Rust-Huelle um
//! `libdispatch`, seit macOS 10.6 im System, auch `dispatch_after`. Das Buendel zielt auf 15.0 (`.cargo/config.toml`);
//! keines von beiden hebt die Untergrenze an, und keine Beruehrung in dieser
//! Datei braucht eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt
//! keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die
//! Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.

use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dispatch2::{DispatchQueue, DispatchTime};
use krk_core::sprache::{Text, text};
use objc2_foundation::MainThreadMarker;

use crate::plattenwarte::{FRIST, Plattenbefehl, Plattenwarte};

thread_local! {
    /// Was die Befehle von der Platte erwarten; siehe den Modulkopf.
    static WARTE: RefCell<Plattenwarte> = RefCell::default();
    /// Was die Statuszeile nachzieht; siehe [`wartemelder_setzen`].
    static WARTEMELDER: RefCell<Option<Rc<dyn Fn()>>> = RefCell::default();
}

/// Der Name des Fadens, auf dem eine Frage an die Platte gestellt wird, die
/// mit dem Terminal nichts zu tun hat: eine Frage, die ein Hauptfadenweg ueber
/// [`fragen_lassen`] stellt, weil die Ereignisschleife nicht auf ihre Antwort
/// warten darf.
///
/// Ein eigener Name und nicht `krk-terminal`: im Fadenbild eines haengenden
/// Prozesses soll stehen, welche Art Frage dort wartet. Er steht hier und
/// nicht bei einem seiner Rufer, damit es ihn genau einmal gibt.
pub(crate) const PLATTENFRAGE: &str = "krk-plattenfrage";

/// `arbeit` laeuft auf einem eigenen Faden namens `fadenname`, ihr Ergebnis
/// geht ueber `DispatchQueue::main()` an `fertig`.
///
/// Scheitert der Fadenstart oder geraet `arbeit` in Panik, macht `startfehler`
/// aus dem Fehler das Ergebnis, und `fertig` bekommt es ebenso ueber die
/// Hauptschlange. **Auf dem rufenden Faden laeuft nichts von `arbeit`**; dort
/// stehen allein der Fadenstart und, scheitert er, `startfehler`. Bei einer Panik
/// laeuft `startfehler` auf dem neuen Faden und bekommt den Fehler aus
/// [`abgebrochen`].
pub(crate) fn fragen_lassen<T: Send + 'static>(
    fadenname: &'static str,
    arbeit: impl FnOnce() -> T + Send + 'static,
    startfehler: impl FnOnce(std::io::Error) -> T + Send + 'static,
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
) {
    // `Builder::spawn` nimmt den Abschluss und gibt ihn bei einem Fehlschlag
    // nicht zurueck; der Halter laesst `startfehler` und `fertig` deshalb auch
    // dann erreichbar, wenn kein Faden entsteht.
    let halter = Arc::new(Mutex::new(Some((startfehler, fertig))));
    let auf_dem_faden = Arc::clone(&halter);
    let gestartet = std::thread::Builder::new()
        .name(fadenname.to_owned())
        .spawn(move || {
            // `AssertUnwindSafe`, weil nach der Panik niemand mehr liest, was
            // `arbeit` hielt: ihre Eingaben sind in sie hineingereist und fallen
            // mit ihr, und das Ergebnis ist der Ersatzwert.
            let ausgang = catch_unwind(AssertUnwindSafe(arbeit));
            if let Some((startfehler, fertig)) = nehmen(&auf_dem_faden) {
                let ergebnis = ausgang.unwrap_or_else(|_| startfehler(abgebrochen()));
                auf_dem_hauptfaden(fertig, ergebnis);
            }
        });
    if let Err(fehler) = gestartet
        && let Some((startfehler, fertig)) = nehmen(&halter)
    {
        auf_dem_hauptfaden(fertig, startfehler(fehler));
    }
}

/// Wie [`fragen_lassen`], fuer einen Befehl des Nutzers: bleibt die Antwort ueber
/// die [`FRIST`] hinaus aus, sagt die Statuszeile, dass er wartet (Modulkopf).
///
/// **Das Warten endet, bevor `fertig` laeuft**, und unabhaengig davon, was
/// `fertig` aus der Antwort macht: eine verfallene Antwort ist auch eine Antwort.
/// Eine Meldung, die `fertig` dann setzt, steht ohnehin ueber diesem Rang
/// ([`super::statuszeile::Rang::Plattenwarte`]).
pub(crate) fn befehl_fragen_lassen<T: Send + 'static>(
    mtm: MainThreadMarker,
    befehl: Plattenbefehl,
    fadenname: &'static str,
    arbeit: impl FnOnce() -> T + Send + 'static,
    startfehler: impl FnOnce(std::io::Error) -> T + Send + 'static,
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
) {
    befehl_fragen_lassen_nach(FRIST, mtm, befehl, fadenname, arbeit, startfehler, fertig);
}

/// [`befehl_fragen_lassen`] mit genannter Frist, damit eine Probe nicht die volle
/// Frist abwarten muss.
fn befehl_fragen_lassen_nach<T: Send + 'static>(
    frist: Duration,
    _mtm: MainThreadMarker,
    befehl: Plattenbefehl,
    fadenname: &'static str,
    arbeit: impl FnOnce() -> T + Send + 'static,
    startfehler: impl FnOnce(std::io::Error) -> T + Send + 'static,
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
) {
    let nummer = WARTE.with_borrow_mut(|warte| warte.gestellt(befehl));
    frist_stellen(frist, befehl, nummer);
    fragen_lassen(fadenname, arbeit, startfehler, move |mtm, ergebnis| {
        if WARTE.with_borrow_mut(|warte| warte.beantwortet(befehl, nummer)) {
            zeile_nachziehen();
        }
        fertig(mtm, ergebnis);
    });
}

/// Legt den Ablauf der Frist auf die Hauptschlange. Er fragt allein den Stand.
fn frist_stellen(frist: Duration, befehl: Plattenbefehl, nummer: u64) {
    // `try_from` scheitert allein am Ueberlauf der Nanosekunden, und `after` hat
    // keinen anderen Ausgang als `Ok`. Bei einer Frist von Sekunden geschieht
    // beides nicht; geschaehe es, fehlte allein der Satz, und die Frage liefe.
    let Ok(zeitpunkt) = DispatchTime::try_from(frist) else {
        return;
    };
    let _ = DispatchQueue::main().after(zeitpunkt, move || {
        // Dieselbe Hauptschlange wie in `auf_dem_hauptfaden`. Der Marker belegt,
        // dass der Stand derselbe ist, den die Frage angelegt hat.
        if MainThreadMarker::new().is_some()
            && WARTE.with_borrow_mut(|warte| warte.frist_um(befehl, nummer))
        {
            zeile_nachziehen();
        }
    });
}

/// Ruft den Melder ausserhalb jeder Ausleihe: er liest [`platte_wartet`], und
/// ein Setzen eines neuen Melders aus ihm heraus stiesse sonst auf die eigene.
fn zeile_nachziehen() {
    let melder = WARTEMELDER.with_borrow(Clone::clone);
    if let Some(melder) = melder {
        melder();
    }
}

/// Ob die Statuszeile gerade sagen soll, dass ein Befehl auf die Platte wartet.
pub(crate) fn platte_wartet(_mtm: MainThreadMarker) -> bool {
    WARTE.with_borrow(Plattenwarte::meldet)
}

/// Setzt, was gerufen wird, wenn der Satz neu steht oder gefallen ist. Der Aufbau
/// der Oberflaeche ruft das einmal; ohne Melder bleibt die Zeile, wie sie ist.
pub(crate) fn wartemelder_setzen(_mtm: MainThreadMarker, melder: impl Fn() + 'static) {
    WARTEMELDER.set(Some(Rc::new(melder)));
}

/// Der Fehler, den `startfehler` bei einer Panik in `arbeit` bekommt. Sein Satz
/// kann in der Statuszeile ankommen und ist deshalb ein Eintrag der Sprachtabelle.
fn abgebrochen() -> std::io::Error {
    std::io::Error::other(text(Text::FrageAbgebrochen))
}

/// Nimmt den Inhalt aus dem Halter, auch aus einem vergifteten: der Wert darin
/// ist trotzdem der richtige, und ihn liegen zu lassen hiesse, die Antwort zu
/// verschlucken (dieselbe Ueberlegung wie in [`super::auswurf`]).
fn nehmen<F>(halter: &Mutex<Option<F>>) -> Option<F> {
    match halter.lock() {
        Ok(mut inhalt) => inhalt.take(),
        Err(vergiftet) => vergiftet.into_inner().take(),
    }
}

/// Reicht das Ergebnis an den Hauptfaden weiter.
fn auf_dem_hauptfaden<T: Send + 'static>(
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
    ergebnis: T,
) {
    DispatchQueue::main().exec_async(move || {
        let Some(mtm) = MainThreadMarker::new() else {
            // Kann nicht eintreten: die Hauptschlange laeuft auf dem
            // Hauptfaden. Was das Ergebnis haelt, bliebe hier ungenutzt; beim
            // Skript aus `super::terminal` ist das eine Datei mit `0700` im
            // privaten `$TMPDIR`, und das ist der Rest, den E2 des Plans
            // `261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md`
            // hinnimmt.
            return;
        };
        fertig(mtm, ergebnis);
    });
}

#[cfg(test)]
pub(super) use tests::HAUPTFADENPROBEN;

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    use objc2_foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop};

    use super::super::hauptfaden::{Probe, hauptfadenprobe};
    use super::*;

    /// Die Proben dieses Moduls: die Hauptschlange laeuft allein auf dem
    /// Hauptfaden, und `libtest` gibt ihn nicht her.
    pub(crate) const HAUPTFADENPROBEN: &[Probe] = &[
        hauptfadenprobe!(eine_panik_in_der_arbeit_ruft_fertig_genau_einmal_mit_dem_ersatzwert),
        hauptfadenprobe!(ohne_panik_bekommt_fertig_das_ergebnis_und_startfehler_laeuft_nicht),
        hauptfadenprobe!(der_satz_kommt_nach_der_frist_und_geht_mit_der_antwort),
        hauptfadenprobe!(eine_schnelle_antwort_laesst_keinen_satz_stehen),
    ];

    /// Laesst die Hauptschlange laufen, bis `fertig` gerufen ist, und danach noch
    /// kurz, damit ein zweiter Ruf auffiele. Die Frist haelt der Laeufer.
    fn hauptschlange_bis(aufrufe: &AtomicUsize) {
        let beginn = Instant::now();
        while aufrufe.load(Ordering::SeqCst) == 0 && beginn.elapsed() < Duration::from_secs(30) {
            umlauf();
        }
        for _ in 0..5 {
            umlauf();
        }
    }

    /// Laesst die Hauptschlange laufen, bis `erfuellt` gilt. Die Frist haelt der
    /// Laeufer, und nach 30 s ruft die Probe mit ihrer eigenen Aussage aus.
    fn hauptschlange_bis_gilt(erfuellt: impl Fn() -> bool) {
        let beginn = Instant::now();
        while !erfuellt() && beginn.elapsed() < Duration::from_secs(30) {
            umlauf();
        }
    }

    /// Ein Umlauf der Schleife im Vorgabemodus, der die Hauptschlange bedient.
    fn umlauf() {
        // SAFETY: `NSDefaultRunLoopMode` ist ein Fremdsymbol von Foundation,
        // dieselbe Form wie in den Proben von `super::super::editor`.
        let _ = NSRunLoop::currentRunLoop().runMode_beforeDate(
            unsafe { NSDefaultRunLoopMode },
            &NSDate::dateWithTimeIntervalSinceNow(0.02),
        );
    }

    /// Eine Panik in `arbeit` laesst `fertig` nicht mit dem Halter fallen: es
    /// wird genau einmal gerufen, mit dem Ersatzwert aus `startfehler`.
    ///
    /// **Was sie nicht sieht:** den gescheiterten Fadenstart, den keine Probe
    /// erzwingen kann, und was die Rufer aus dem Ersatzwert machen.
    fn eine_panik_in_der_arbeit_ruft_fertig_genau_einmal_mit_dem_ersatzwert(
        _mtm: MainThreadMarker,
    ) {
        let aufrufe = Arc::new(AtomicUsize::new(0));
        let antwort = Arc::new(Mutex::new(None));
        let (fuer_fertig, fuer_antwort) = (Arc::clone(&aufrufe), Arc::clone(&antwort));
        fragen_lassen(
            "krk-probe-panik",
            || -> String { panic!("die Panik dieser Probe ist gewollt") },
            |fehler| fehler.to_string(),
            move |_mtm, ergebnis| {
                fuer_fertig.fetch_add(1, Ordering::SeqCst);
                *fuer_antwort.lock().expect("die Antwort ist vergiftet") = Some(ergebnis);
            },
        );
        hauptschlange_bis(&aufrufe);
        assert_eq!(
            aufrufe.load(Ordering::SeqCst),
            1,
            "fertig wurde nicht genau einmal gerufen"
        );
        assert_eq!(
            antwort
                .lock()
                .expect("die Antwort ist vergiftet")
                .as_deref(),
            Some(text(Text::FrageAbgebrochen)),
            "fertig bekam nicht den Ersatzwert aus startfehler"
        );
    }

    /// Ohne Panik bleibt alles beim Alten: `fertig` bekommt das Ergebnis von
    /// `arbeit`, und `startfehler` faellt ungerufen.
    fn ohne_panik_bekommt_fertig_das_ergebnis_und_startfehler_laeuft_nicht(_mtm: MainThreadMarker) {
        let aufrufe = Arc::new(AtomicUsize::new(0));
        let antwort = Arc::new(Mutex::new(None));
        let (fuer_fertig, fuer_antwort) = (Arc::clone(&aufrufe), Arc::clone(&antwort));
        fragen_lassen(
            "krk-probe-ergebnis",
            || 7,
            |_fehler| panic!("startfehler laeuft ohne Fehler"),
            move |_mtm, ergebnis: i32| {
                fuer_fertig.fetch_add(1, Ordering::SeqCst);
                *fuer_antwort.lock().expect("die Antwort ist vergiftet") = Some(ergebnis);
            },
        );
        hauptschlange_bis(&aufrufe);
        assert_eq!(aufrufe.load(Ordering::SeqCst), 1);
        assert_eq!(*antwort.lock().expect("die Antwort ist vergiftet"), Some(7));
    }

    /// Der ganze Weg mit dem echten Zeitgeber: die Arbeit haelt an, bis die Probe
    /// sie freigibt, also entscheidet keine Uhr, ob der Satz vor der Antwort kommt.
    fn der_satz_kommt_nach_der_frist_und_geht_mit_der_antwort(mtm: MainThreadMarker) {
        let melder = Arc::new(AtomicUsize::new(0));
        let fuer_den_melder = Arc::clone(&melder);
        wartemelder_setzen(mtm, move || {
            fuer_den_melder.fetch_add(1, Ordering::SeqCst);
        });
        let aufrufe = Arc::new(AtomicUsize::new(0));
        let fuer_fertig = Arc::clone(&aufrufe);
        let (freigabe, warten) = std::sync::mpsc::channel::<()>();
        befehl_fragen_lassen_nach(
            Duration::from_millis(10),
            mtm,
            Plattenbefehl::Pfadsprung,
            "krk-probe-warte",
            move || warten.recv().is_ok(),
            |_fehler| false,
            move |mtm, _ergebnis| {
                assert!(
                    !platte_wartet(mtm),
                    "der Satz steht noch, wenn fertig laeuft"
                );
                fuer_fertig.fetch_add(1, Ordering::SeqCst);
            },
        );
        hauptschlange_bis_gilt(|| platte_wartet(mtm));
        assert!(platte_wartet(mtm), "der Satz kam nach der Frist nicht");
        assert_eq!(melder.load(Ordering::SeqCst), 1, "der Melder lief nicht");

        freigabe.send(()).expect("die Arbeit wartet nicht mehr");
        hauptschlange_bis(&aufrufe);
        assert_eq!(aufrufe.load(Ordering::SeqCst), 1);
        assert!(
            !platte_wartet(mtm),
            "der Satz blieb nach der Antwort stehen"
        );
        assert_eq!(melder.load(Ordering::SeqCst), 2, "der Melder lief nicht");
        WARTEMELDER.set(None);
    }

    /// Eine Antwort vor der Frist: auch nach deren Ablauf steht nichts.
    fn eine_schnelle_antwort_laesst_keinen_satz_stehen(mtm: MainThreadMarker) {
        let aufrufe = Arc::new(AtomicUsize::new(0));
        let fuer_fertig = Arc::clone(&aufrufe);
        let frist = Duration::from_millis(50);
        befehl_fragen_lassen_nach(
            frist,
            mtm,
            Plattenbefehl::Lesezeichen,
            "krk-probe-warte",
            || (),
            |_fehler| (),
            move |_mtm, ()| {
                fuer_fertig.fetch_add(1, Ordering::SeqCst);
            },
        );
        hauptschlange_bis(&aufrufe);
        let beginn = Instant::now();
        while beginn.elapsed() < frist * 3 {
            umlauf();
        }
        assert!(!platte_wartet(mtm));
    }

    /// Die fuenf Befehle gehen ueber den meldenden Weg, die Fragen im Hintergrund
    /// nicht, und der Aufbau und die Zeile sind daran angeschlossen.
    ///
    /// **Was sie nicht sieht:** ob der Satz im Fenster erscheint. Das zeigt allein
    /// der Lauf an einer getrennten Freigabe.
    #[test]
    fn genau_die_fuenf_befehle_melden_ein_warten() {
        use super::super::anwendung::quelltextproben::{datei, rumpf};
        let anwendung = datei("krk-ui/src/appkit/anwendung.rs");
        let tabelle = datei("krk-ui/src/appkit/tabelle.rs");
        let editor = datei("krk-ui/src/appkit/editor.rs");
        let leiste = datei("krk-ui/src/appkit/leiste.rs");
        let meldend = concat!("nebenfaden::befehl_", "fragen_lassen(");
        for (quelle, huelle, befehl) in [
            (
                &anwendung,
                "loeschbefund_erheben_lassen",
                "Plattenbefehl::Loeschen",
            ),
            (
                &anwendung,
                "notizordner_oeffnen",
                "Plattenbefehl::Notizordner",
            ),
            (
                &editor,
                "geheimnisgroesse_fragen_lassen",
                "Plattenbefehl::Geheimnis",
            ),
            (&tabelle, "pfad_pruefen_lassen", "Plattenbefehl::Pfadsprung"),
            (
                &anwendung,
                "leistenauswahl_fragen_lassen",
                "Plattenbefehl::Lesezeichen",
            ),
        ] {
            let rumpf = rumpf(quelle, huelle);
            assert!(rumpf.contains(meldend), "{huelle} meldet kein Warten");
            assert!(rumpf.contains(befehl), "{huelle} fragt nicht als {befehl}");
        }
        for (quelle, huelle) in [
            (&anwendung, "geraete_aufzaehlen_lassen"),
            (&leiste, "gueltigkeit_erheben_lassen"),
            (&tabelle, "schreibrecht_fragen_lassen"),
        ] {
            assert!(
                !rumpf(quelle, huelle).contains(meldend),
                "{huelle} fragt im Hintergrund und meldet ein Warten"
            );
        }
        assert!(
            rumpf(&anwendung, "oberflaeche_aufbauen")
                .contains(concat!("nebenfaden::wartemelder_", "setzen(")),
            "der Aufbau setzt den Melder nicht: der Satz kaeme nie in die Zeile"
        );
        let nachzug = rumpf(&anwendung, "statuszeile_nachziehen");
        assert!(
            nachzug.contains(concat!("nebenfaden::platte_", "wartet("))
                && nachzug.contains("Text::WartetAufDiePlatte"),
            "der Schreiber der Zeile fragt das Warten nicht"
        );
    }
}
