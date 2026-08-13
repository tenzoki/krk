//! Abnahme der Zusagen, die Aussagen ueber den **Quellbaum** sind.
//!
//! Drei Kriterien der Runde 7 sagen eine Zahl von Stellen zu und keinen
//! Rueckgabewert: genau zwei Dateien mit `#![allow(unsafe_code)]` (C4.5), genau
//! drei Pruefordner-Fassungen (C4.6) und genau zwei Absprachen ueber der Ablage
//! (C3.14). An keinem Wert ist abzulesen, dass es keine dritte gibt; geprueft
//! wird deshalb am Baum.
//!
//! # Gezaehlt werden Erklaerungen und keine Aufrufer
//!
//! Die Unterscheidung ist nicht kosmetisch, und `krk_ui::quellbaum` schreibt sie
//! aus. Eine Erklaerungszaehlung haelt, was sie verspricht: eine zweite Fassung
//! derselben Sache laesst sie rot werden. Eine Aufruferzaehlung ist in beide
//! Richtungen blind und steht nur dort, wo ein Kriterium die Zahl selbst
//! zusagt. Keine Probe dieser Datei zaehlt Aufrufer.
//!
//! # Die Nadel steht zusammengesetzt da
//!
//! Diese Datei liegt in dem Baum, den sie liest. Eine Nadel, die als ein Stueck
//! im Quelltext staende, faende sich selbst und zaehlte eine Fundstelle zu viel;
//! sie wird deshalb mit `concat!` aus zwei Teilen gebaut. Die Bauform stammt von
//! `es_gibt_genau_einen_menuebauer` in `krk_ui::appkit::teilen`, der aeltesten
//! Probe dieser Art.

mod gemeinsam;
use gemeinsam::quelldateien;

/// C4.5: Die Ausnahme von `deny(unsafe_code)` steht an genau zwei Stellen.
///
/// **Die Runde 7 bringt einen fuenften Fremdaufruf, `flock(2)`, und trotzdem
/// keine dritte Ausnahme.** Er ist in `verzeichnis/sys.rs` gelandet, der einen
/// Datei des Kerns mit dieser Ausnahme; eine eigene Datei fuer die beiden
/// Sperren der Ablage waere die dritte gewesen. Die Probe nennt die beiden
/// Dateien beim Namen, damit ein Umzug hier auffaellt und nicht nur eine Zahl
/// gleich bleibt.
#[test]
fn genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code() {
    let nadel = concat!("#![allow(unsafe", "_code)]");
    // **Verglichen wird die ganze Zeile und nicht ihr Vorkommen im Text.** Die
    // Ausnahme wird an mehreren Stellen des Baums besprochen, unter anderem im
    // Kopf dieser Datei; ein `contains` zaehlte jede Erwaehnung mit und machte
    // aus einer Zusage ueber den Bau eine ueber die Prosa.
    let dateien: Vec<String> = quelldateien()
        .into_iter()
        .filter(|(_, inhalt)| inhalt.lines().any(|zeile| zeile.trim() == nadel))
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        dateien,
        vec![
            "krk-core/src/verzeichnis/sys.rs".to_owned(),
            "krk-ui/src/appkit/mod.rs".to_owned(),
        ],
        "die Liste der Ausnahmen von deny(unsafe_code) hat sich geaendert"
    );
}

/// C4.6: Es gibt genau drei Pruefordner-Fassungen, eine je Kiste.
///
/// Dass es drei sind und nicht eine, liegt an den Kistengrenzen und nicht an
/// Nachlaessigkeit: `krk-ui` und `krk-bench` haben nur ein Binaerziel, und ein
/// Testziel erreicht den Code eines Binaerziels nicht. Der Modulkopf von
/// `tests/gemeinsam/mod.rs` schreibt es aus. Eine vierte waere ein Doppelbau.
///
/// Gezaehlt wird die **Erklaerung** des selbstabraeumenden Ordners, also das
/// `impl Drop` daneben; ein blosser Name faende auch jede Benutzung.
#[test]
fn genau_drei_pruefordner_fassungen_stehen_im_baum() {
    let fassungen = [
        ("krk-core/tests/gemeinsam/mod.rs", "struct Pruefordner"),
        ("krk-ui/src/pruefordner.rs", "struct Pruefordner"),
        ("krk-bench/src/wegwerfordner.rs", "struct Wegwerfordner"),
    ];
    let baum = quelldateien();
    for (datei, nadel) in fassungen {
        let (_, inhalt) = baum
            .iter()
            .find(|(name, _)| name == datei)
            .unwrap_or_else(|| panic!("{datei} steht nicht mehr im Baum"));
        assert!(
            inhalt.contains(nadel),
            "{datei} erklaert seinen Pruefordner nicht mehr"
        );
    }

    // Und keine vierte: ein selbstabraeumender Ordner erklaert sich ueber ein
    // `impl Drop`, und die drei Fassungen sind die einzigen, die eines tragen.
    let nadel = concat!("impl Drop for Pruef", "ordner");
    let weitere: Vec<String> = baum
        .iter()
        .filter(|(name, inhalt)| {
            inhalt.contains(nadel) && !fassungen.iter().any(|(fassung, _)| fassung == name)
        })
        .map(|(name, _)| name.clone())
        .collect();
    assert!(
        weitere.is_empty(),
        "eine vierte Pruefordner-Fassung steht im Baum: {weitere:?}"
    );
}

/// C3.14: Ueber der Ablage stehen genau zwei Absprachen und keine dritte.
///
/// Die Schreibsperre und das Sitzungsrecht, jede auf ihrer eigenen Datei im
/// Ablageordner. Gezaehlt werden die **erklaerten** Sperrdateinamen: eine
/// dritte Absprache braeuchte eine dritte Datei, und sie faellt hier auf.
#[test]
fn ueber_der_ablage_stehen_genau_zwei_absprachen() {
    let nadel = concat!(".lo", "ck\"");
    let (_, sperre) = quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/ablage/sperre.rs")
        .expect("krk-core/src/ablage/sperre.rs steht nicht mehr im Baum");
    let benannt: Vec<&str> = sperre
        .lines()
        .filter(|zeile| zeile.trim_start().starts_with("pub const") && zeile.contains(nadel))
        .collect();
    assert_eq!(
        benannt.len(),
        2,
        "ueber der Ablage stehen nicht mehr genau zwei Absprachen: {benannt:?}"
    );
    assert_eq!(
        krk_core::ablage::sperre::SCHREIBSPERRE,
        "schreiben.lock",
        "die Schreibsperre hat ihren Dateinamen gewechselt"
    );
    assert_eq!(
        krk_core::ablage::sperre::SITZUNGSRECHT,
        "sitzungsrecht.lock",
        "das Sitzungsrecht hat seinen Dateinamen gewechselt"
    );
}
