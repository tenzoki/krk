//! Der Quellbaum dieser Kiste als Lesestoff fuer die Zaehlproben.
//!
//! **Nur im Probenbau uebersetzt.** `main.rs` meldet das Modul mit
//! `#[cfg(test)]` an; kein ausgeliefertes Programm liest seinen eigenen
//! Quelltext.
//!
//! # Wozu eine Probe den Baum liest
//!
//! Etliche Zusagen dieses Vorhabens sind Aussagen ueber den **Baum** und nicht
//! ueber ein Ergebnis: „es gibt genau einen Menuebauer", „die Frage nach dem
//! Ersthelfer ist genau einmal erklaert", „die Zulaessigkeitsfrage steht an
//! genau einer Stelle". An keinem Rueckgabewert ist abzulesen, dass es keine
//! zweite Fassung gibt. Die Proben lesen deshalb die Quelldateien und zaehlen
//! darin.
//!
//! # Zwei Sorten Zaehlung, und der Unterschied ist nicht kosmetisch
//!
//! **Erklaerungen zaehlen** heisst: wie oft wird eine Sache im Baum ueberhaupt
//! erklaert. Eine solche Zaehlung haelt, was sie verspricht — eine zweite
//! Fassung derselben Sache laesst sie rot werden.
//!
//! **Aufrufer zaehlen** heisst: wie viele Stellen rufen eine Sache. Das ist
//! etwas anderes, und es ist in beide Richtungen blind: ein Doppelbau an
//! anderer Stelle laesst die Zahl der Aufrufer unveraendert, und ein weiterer
//! berechtigter Frager macht sie rot, worauf der billigste Weg zurueck ins
//! Gruene das Streichen eines Fragers waere. Eine Aufruferzaehlung steht
//! deshalb nur dort, wo ein Abnahmekriterium die Zahl selbst zusagt, und
//! nirgends als Stellvertreter fuer „es gibt keinen Doppelbau".
//!
//! # Die Nadel steht zusammengesetzt da
//!
//! Die Proben liegen in dem Baum, den sie lesen. Eine Nadel, die als ein Stueck
//! im Quelltext steht, faende sich selbst und zaehlte eine Fundstelle zu viel;
//! sie wird deshalb mit `concat!` aus zwei Teilen gebaut. Die Bauform stammt
//! von `es_gibt_genau_einen_menuebauer` in [`crate::appkit::teilen`], der
//! aeltesten Probe dieser Art.

/// Jede `.rs`-Datei unter `crates/krk-ui/src/`, mit ihrem Pfad unterhalb von
/// `src/` und ihrem Inhalt, in fester Reihenfolge.
///
/// **Die Grundlage jeder Zaehlprobe dieser Kiste.** Sie stand bis zur Runde 7
/// privat im Pruefmodul von [`crate::appkit::teilen`]; seit die Runde sie in
/// mehreren Pruefmodulen braucht, wohnt sie hier. Eine zweite Fassung waere
/// genau die Art von Doppelbau, die die Proben darueber verhindern sollen.
///
/// `CARGO_MANIFEST_DIR` steht beim Uebersetzen fest und zeigt auf
/// `crates/krk-ui`; die Probe braucht deshalb den Baum zur Laufzeit an
/// derselben Stelle. Fehlt er, schlaegt sie fehl statt still nichts zu zaehlen
/// — eine leere Liste waere eine Probe, die alles bestaetigt.
pub(crate) fn quelldateien() -> Vec<(String, String)> {
    let wurzel = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut gefunden = Vec::new();
    einsammeln(&wurzel, &wurzel, &mut gefunden);
    assert!(
        gefunden.len() > 1,
        "unter {} steht kein Quellbaum; die Zaehlproben haetten nichts zu zaehlen",
        wurzel.display()
    );
    gefunden.sort();
    gefunden
}

/// Haengt alle `.rs`-Dateien unter `ordner` an `gefunden`, in die Tiefe.
fn einsammeln(
    wurzel: &std::path::Path,
    ordner: &std::path::Path,
    gefunden: &mut Vec<(String, String)>,
) {
    let eintraege = std::fs::read_dir(ordner)
        .unwrap_or_else(|fehler| panic!("{} nicht lesbar: {fehler}", ordner.display()));
    for eintrag in eintraege {
        let pfad = eintrag
            .expect("Eintrag des Quellordners nicht lesbar")
            .path();
        if pfad.is_dir() {
            einsammeln(wurzel, &pfad, gefunden);
        } else if pfad.extension().is_some_and(|endung| endung == "rs") {
            let name = pfad
                .strip_prefix(wurzel)
                .expect("der Pfad kommt aus der Wurzel")
                .to_string_lossy()
                .into_owned();
            let inhalt = std::fs::read_to_string(&pfad)
                .unwrap_or_else(|fehler| panic!("{} nicht lesbar: {fehler}", pfad.display()));
            gefunden.push((name, inhalt));
        }
    }
}
