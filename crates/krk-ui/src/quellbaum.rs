//! Der Quellbaum des Vorhabens als Lesestoff fuer die Zaehlproben.
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
//! # Alle Kisten und nicht nur diese
//!
//! Bis zur Runde 7 las [`quelldateien`] `crates/krk-ui/src` und sonst nichts,
//! waehrend sechs Proben darueber von „im Baum" sprachen. Fuer `isKindOfClass`
//! und `keyDown:` war die Verengung harmlos — `krk-core` und `krk-bench`
//! duerfen `objc2` nicht kennen —, fuer „`fn zulaessig` steht an genau einer
//! Stelle" nicht: eine zweite Fassung der Zulaessigkeitsregel in `krk-core`
//! waere unsichtbar geblieben
//! (`issues/260813-0540_*_die-zaehlproben-in-krk-ui-sagen-im-baum-und-lesen-nur-eine-kiste.md`).
//!
//! Seither liest sie `crates/` und damit denselben Umfang wie
//! `krk-core/tests/gemeinsam::quelldateien`. **Die zwei Fassungen sind nicht
//! zusammenzulegen und unterscheiden sich jetzt in nichts als ihrem Wohnort:**
//! `krk-ui` hat kein Bibliotheksziel, also erreicht kein Testziel dieses Modul,
//! und aus derselben Kistengrenze folgt, dass es drei Pruefordner-Fassungen
//! gibt. Dass die beiden vorher **verschieden weit** lasen und es nirgends
//! nebeneinanderstand, war die eigentliche Falle: wer eine Zaehlprobe schrieb,
//! waehlte damit unbemerkt ihre Reichweite mit.
//!
//! # Zwei Sorten Zaehlung, und der Unterschied ist nicht kosmetisch
//!
//! **Erklaerungen zaehlen** heisst: wie oft wird eine Sache im Baum ueberhaupt
//! erklaert. Eine solche Zaehlung haelt gegen eine zweite Fassung **desselben
//! Namens** und laesst sie rot werden.
//!
//! **Aufrufer zaehlen** heisst: wie viele Stellen rufen eine Sache. Das ist
//! etwas anderes, und es ist in beide Richtungen blind: ein Doppelbau an
//! anderer Stelle laesst die Zahl der Aufrufer unveraendert, und ein weiterer
//! berechtigter Frager macht sie rot, worauf der billigste Weg zurueck ins
//! Gruene das Streichen eines Fragers waere. Eine Aufruferzaehlung steht
//! deshalb nur dort, wo ein Abnahmekriterium die Zahl selbst zusagt, und
//! nirgends als Stellvertreter fuer „es gibt keinen Doppelbau".
//!
//! # Was keine Zaehlung entscheiden kann
//!
//! Bis zur Runde 7 stand hier, eine Erklaerungszaehlung „haelt, was sie
//! verspricht". **Das war zu weit gegriffen, und die Runde hat den Gegenbeweis
//! selbst geliefert:** eine vierte Pruefordner-Fassung namens `Ordner` stand im
//! Baum, und die Probe, die genau sie zaehlen sollte, sah sie nicht, weil sie
//! nach `Pruefordner` suchte.
//!
//! Richtig ist: eine Erklaerungszaehlung haelt gegen eine Kopie **unter
//! demselben Namen**. Ob irgendwo dieselbe Sache unter anderem Namen oder in
//! anderer Schreibweise noch einmal gebaut ist, entscheidet keine Suche im
//! Quelltext. Drei Folgerungen, und sie sind die Bauanleitung fuer jede neue
//! Zaehlprobe dieses Baums:
//!
//! 1. **Nach dem Gegenstand suchen, wo es geht, und nicht nach seinem Namen.**
//!    Ein selbstabraeumender Pruefordner ist ein `impl Drop` neben einem
//!    Temporaerordner, gleich wie der Typ heisst.
//! 2. **Jede Schreibweise erfassen, die der Baum schon kennt.** Steht eine
//!    Typpruefung als `isKindOfClass` und daneben als `downcast_ref`, gehoeren
//!    beide in die Nadel.
//! 3. **Die verbleibende Blindheit am Doc-Kommentar der Probe benennen** statt
//!    sie im Namen der Probe zu ueberschreiben.
//!
//! # Die Nadel steht zusammengesetzt da
//!
//! Die Proben liegen in dem Baum, den sie lesen. Eine Nadel, die als ein Stueck
//! im Quelltext steht, faende sich selbst und zaehlte eine Fundstelle zu viel;
//! sie wird deshalb mit `concat!` aus zwei Teilen gebaut. Die Bauform stammt
//! von `es_gibt_genau_einen_menuebauer` in [`crate::appkit::teilen`], der
//! aeltesten Probe dieser Art.

/// Jede `.rs`-Datei unter `crates/`, mit ihrem Pfad unterhalb von `crates/` und
/// ihrem Inhalt, in fester Reihenfolge.
///
/// **Die Grundlage jeder Zaehlprobe dieser Kiste.** Sie stand bis zur Runde 7
/// privat im Pruefmodul von [`crate::appkit::teilen`]; seit die Runde sie in
/// mehreren Pruefmodulen braucht, wohnt sie hier. Eine zweite Fassung in dieser
/// Kiste waere genau die Art von Doppelbau, die die Proben darueber verhindern
/// sollen; warum es die zweite in `krk-core/tests/gemeinsam/` trotzdem geben
/// muss, steht im Modulkopf.
///
/// `CARGO_MANIFEST_DIR` steht beim Uebersetzen fest und zeigt auf
/// `crates/krk-ui`; eine Ebene darueber liegt `crates/`. Die Probe braucht den
/// Baum zur Laufzeit an derselben Stelle. Fehlt er, schlaegt sie fehl statt
/// still nichts zu zaehlen — eine leere Liste waere eine Probe, die alles
/// bestaetigt.
pub(crate) fn quelldateien() -> Vec<(String, String)> {
    let wurzel = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/krk-ui liegt eine Ebene unter crates/")
        .to_path_buf();
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

/// Die Codezeilen einer Datei, also alles ausser den Kommentarzeilen.
///
/// **Die eine Fassung dieses Filters in dieser Kiste.** Er stand bis zum 260908
/// achtmal eingesetzt da — siebenmal in den Zaehlproben von
/// [`crate::appkit::vorschau`], einmal im Pruefmodul von
/// [`crate::appkit::betrachter`] und einmal in [`aufrufstellen`] hier —, und
/// zwei eingesetzte Fassungen desselben Filters driften: wer den einen um
/// Blockkommentare erweitert, erweitert den anderen nicht, und eine Probe sieht
/// dann Zeilen, die die andere nicht sieht
/// (`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/issues/260828-1046_*_der-variantenleser-*`).
///
/// **Was er nicht sieht:** ein Blockkommentar `/* … */` und eine Kommentarzeile
/// hinter Code (`let x = 1; // dazu`). Beide Formen kennt dieser Baum nicht;
/// `rustfmt` bricht keine Zeile so um, und die Bauanleitung im Modulkopf
/// verlangt, die verbleibende Blindheit zu benennen statt sie zu ueberschreiben.
pub(crate) fn codezeilen(inhalt: &str) -> impl Iterator<Item = &str> {
    inhalt
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
}

/// Die Varianten einer Aufzaehlung ohne Daten, aus dem Quelltext gelesen.
///
/// Gelesen wird ab `pub enum <name> {` bis zur schliessenden Klammer am
/// Zeilenanfang; Leer-, Kommentar- und Attributzeilen fallen heraus, und von
/// jeder verbleibenden Zeile bleibt, was vor dem Komma steht.
///
/// **Die eine Fassung dieser Lesart in dieser Kiste**, und die Doppelung, gegen
/// die sie steht, war eine echte: bis zum 260908 stand sie zweimal eingesetzt
/// da, im Pruefmodul von [`crate::appkit::betrachter`] und in
/// `kommandos::zulaessigkeit::jeder_wirkungsbereich_hat_einen_stellvertreter`
/// (`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/issues/260828-1046_*_der-variantenleser-*`).
///
/// **Die anerkannte Kernfassung ist `varianten_der_aufzaehlung` in
/// `krk-core/tests/gemeinsam/mod.rs`**, und beide tragen dieselbe Lesart. Sie
/// zusammenzulegen geht nicht: `krk-ui` hat kein Bibliotheksziel, also erreicht
/// kein Testziel des Kerns dieses Modul, und `tests/gemeinsam` gehoert den
/// Probenzielen des Kerns. Wer die eine Lesart aendert — um Varianten mit Daten
/// etwa, oder um `#[doc]`-Zeilen —, aendert die andere mit.
///
/// **Die Nadel steht zusammengesetzt da**, wie der Modulkopf es verlangt: diese
/// Datei liegt in dem Baum, den [`quelldateien`] liest, und ein `pub enum` als
/// ein Stueck waere eine Fundstelle fuer jede Zaehlprobe darueber.
pub(crate) fn varianten(inhalt: &str, name: &str) -> Vec<String> {
    // **Mit und ohne `pub`, wie die Kernfassung seit dem 260908**: die
    // Sichtbarkeit sagt, wer die Aufzaehlung sehen darf, und nichts darueber,
    // ob eine Liste daneben vollstaendig zu halten ist.
    let koepfe = [
        format!(concat!("pub ", "enum {} {{"), name),
        format!(concat!("", "enum {} {{"), name),
    ];
    let varianten: Vec<String> = inhalt
        .lines()
        .skip_while(|zeile| !koepfe.iter().any(|kopf| zeile == kopf))
        .skip(1)
        .take_while(|zeile| *zeile != "}")
        .map(str::trim)
        .filter(|zeile| !zeile.is_empty() && !zeile.starts_with("//") && !zeile.starts_with("#["))
        .map(|zeile| zeile.trim_end_matches(',').to_owned())
        .collect();
    // Leer laufen kann sie nicht, und das ist dieselbe Vorkehrung, die die
    // Kernfassung traegt: ein verschobener oder umbenannter Block liesse die
    // rufende Probe sonst still bestehen.
    assert!(
        !varianten.is_empty(),
        "die Aufzaehlung {name} steht nicht in Spalte 0 im gelesenen Quelltext; \
         umbenannt oder verschoben?"
    );
    varianten
}

/// Zaehlt die Aufrufstellen einer Funktion in einer Datei, unabhaengig davon,
/// **wie** der Aufruf geschrieben ist.
///
/// Eine Aufruferzaehlung, die die Nadel `modul::funktion(` sucht, zaehlt einen
/// Aufrufer nicht mit, der die Funktion ueber ein `use` in den Geltungsbereich
/// geholt und unqualifiziert gerufen hat — also genau den Fall, fuer den sie
/// gebaut ist
/// (`issues/260813-0540_*_zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs.md`).
/// Diese Funktion zaehlt stattdessen den Namen mit oeffnender Klammer und zieht
/// die drei Sorten Fundstellen ab, die keine Aufrufe sind:
///
/// - **Fundstellen mitten in einem laengeren Namen.** `Unzulaessig(` ist kein
///   Aufruf von `zulaessig`. Entschieden wird das am Zeichen davor: gehoert es
///   zu einem Bezeichner, faellt die Fundstelle heraus. Ein Punkt, ein
///   Doppelpunkt, ein Ausrufezeichen oder ein Leerzeichen davor bleiben drin,
///   und damit jede Empfaengerform und jeder Pfad.
/// - **Die Erklaerung selbst**, erkannt am `fn` unmittelbar davor.
/// - **Nennungen in Kommentaren.** Eine Prosa-Zeile ruft nichts.
///
/// **Was bleibt:** ein Aufruf unter einem anderen Namen, also ein
/// `use … as anders;`. Der Kopf dieses Moduls sagt, warum keine Suche im
/// Quelltext restlos dicht ist.
pub(crate) fn aufrufstellen(inhalt: &str, name: &str) -> usize {
    let nadel = format!("{name}(");
    codezeilen(inhalt)
        .map(|zeile| {
            zeile
                .match_indices(&nadel)
                .filter(|(stelle, _)| {
                    let davor = &zeile[..*stelle];
                    let letztes = davor.chars().next_back();
                    let teil_eines_namens =
                        letztes.is_some_and(|zeichen| zeichen.is_alphanumeric() || zeichen == '_');
                    !teil_eines_namens && !davor.trim_end().ends_with("fn")
                })
                .count()
        })
        .sum()
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

#[cfg(test)]
mod tests {
    use super::{aufrufstellen, codezeilen, quelldateien};

    /// Der Aufzaehlungsleser steht in dieser Kiste genau einmal, naemlich in
    /// [`super::varianten`].
    ///
    /// **Gesucht wird der Gegenstand und nicht der Name**, wie die
    /// Bauanleitung im Modulkopf es verlangt: was einen Aufzaehlungsleser
    /// ausmacht, ist der Abbruch am schliessenden `}` einer Zeile, also
    /// `take_while` ueber `zeile`. Eine zweite Fassung unter anderem Namen
    /// faellt damit genauso auf wie eine Abschrift — und genau zwei Abschriften
    /// hat diese Probe zu spaet gefunden
    /// (`circles/260827-2028-vorschau-rendert-pdf-als-betrachter/issues/260828-1046_*_der-variantenleser-*`).
    ///
    /// **Die Nadel steht zusammengesetzt da**, sonst faende die Probe die
    /// Zeile, mit der sie sich selbst beschreibt.
    ///
    /// **Was sie nicht findet:** ein Leser, der den Block ueber
    /// Zeilennummern statt ueber `take_while` abgrenzt, und die Kernfassung
    /// `varianten_der_aufzaehlung` unter `crates/krk-core/tests/`, die sie
    /// bewusst nicht meint — sie gehoert den Probenzielen des Kerns, und warum
    /// es sie geben muss, steht bei [`super::varianten`].
    #[test]
    fn der_aufzaehlungsleser_steht_in_dieser_kiste_genau_einmal() {
        let nadel = concat!("take_while(|zeile| ", "*zeile != \"}\")");
        let leser: Vec<String> = quelldateien()
            .into_iter()
            .filter(|(name, inhalt)| {
                name.starts_with("krk-ui/") && codezeilen(inhalt).any(|zeile| zeile.contains(nadel))
            })
            .map(|(name, _)| name)
            .collect();
        assert_eq!(
            leser,
            vec!["krk-ui/src/quellbaum.rs".to_owned()],
            "eine zweite Fassung des Aufzaehlungslesers steht in dieser Kiste"
        );
    }

    /// Die drei Abzuege und die vier Formen, die stehen bleiben.
    ///
    /// Eine Probe ueber das Werkzeug und nicht ueber den Baum: die
    /// Zaehlproben darueber verlassen sich darauf, dass diese Funktion einen
    /// Aufruf von einem Namen unterscheidet, in dem der gesuchte Name nur
    /// steckt. Ohne sie waere die Zusicherung „jede Schreibweise wird erfasst"
    /// selbst wieder nur behauptet.
    ///
    /// **Der Beispielname ist erfunden und kommt im Baum sonst nicht vor.**
    /// Diese Datei liegt in dem Baum, den [`quelldateien`] liest, und ein
    /// echter Name im Beispiel waere eine Fundstelle, die jede Zaehlprobe
    /// darueber mitzaehlt — beim ersten Versuch mit `zulaessig` ist genau das
    /// passiert.
    #[test]
    fn eine_aufrufzaehlung_sieht_jede_schreibweise_und_keine_nennung() {
        let quelle = "\
pub fn wirkthier(a: u8) -> bool { true }
// wirkthier(1) steht hier nur in Prosa
/// und hier auch: wirkthier(2)
let x = Kollision::Unwirkthier(fehler);
let a = regel::wirkthier(1);
let b = wirkthier(2);
let c = !wirkthier(3);
let d = self.wirkthier(4);
";
        assert_eq!(
            aufrufstellen(quelle, "wirkthier"),
            4,
            "gezaehlt gehoeren die vier Aufrufe und weder die Erklaerung noch \
             die zwei Kommentarzeilen noch der laengere Name"
        );
    }
}
