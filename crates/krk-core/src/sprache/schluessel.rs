//! Die Schluessel der Sprachtabelle: [`Text`] fuer feste Texte und Saetze mit
//! Platzhaltern, [`Zahlwort`] fuer Mengenangaben mit Einzahl und Mehrzahl.
//!
//! **Ein Schluessel heisst nach der Flaeche und der Sache**, ASCII und ohne
//! Umschrift-Zweideutigkeit: `WirkungsbereichDateifenster`,
//! `AbweisungUmbruchImThema`, `EinheitKilobyte`. Ein Text, der heute an zwei
//! Stellen gleich steht, bekommt einen Schluessel und nicht zwei; so traegt
//! `NameLeer` den Grund fuer einen leeren Dateinamen und fuer einen leeren
//! Lesezeichennamen.
//!
//! Jede Aufzaehlung fuehrt daneben `ALLE`, und die Probe
//! `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` in
//! `crates/krk-core/tests/baum.rs` haelt die Liste gegen die Aufzaehlung; die
//! Proben in `crates/krk-core/tests/sprache.rs` laufen ueber `ALLE` und
//! halten je Sprache und Schluessel, was der Uebersetzer nicht haelt.
//!
//! Die Eintraege stehen in `tabelle/de.rs`, `tabelle/fr.rs` und
//! `tabelle/en.rs`; der Modulkopf von [`super`] traegt die Regel.

/// Ein fester Text oder ein Satz mit benannten Platzhaltern.
///
/// Die Doc-Zeile je Wert nennt die Stelle, die ihn zeigt, und nicht den
/// Wortlaut: der steht in der Tabelle, einmal je Sprache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Text {
    /// `Wirkungsbereich::Dateifenster` in der Markdown-Ausgabe der Belegung.
    WirkungsbereichDateifenster,
    /// `Wirkungsbereich::Leiste`.
    WirkungsbereichLeiste,
    /// `Wirkungsbereich::Dateibereiche`.
    WirkungsbereichDateibereiche,
    /// `Wirkungsbereich::Editor`.
    WirkungsbereichEditor,
    /// `Wirkungsbereich::Editortext`.
    WirkungsbereichEditortext,
    /// `Wirkungsbereich::Eintraege`.
    WirkungsbereichEintraege,
    /// `Wirkungsbereich::Reihenfolge`.
    WirkungsbereichReihenfolge,
    /// `Wirkungsbereich::Aufgaben`.
    WirkungsbereichAufgaben,
    /// `Wirkungsbereich::Termine`.
    WirkungsbereichTermine,
    /// `Wirkungsbereich::Geheimnisse`.
    WirkungsbereichGeheimnisse,
    /// `Wirkungsbereich::Quicknote`.
    WirkungsbereichQuicknote,
    /// `Wirkungsbereich::Tabbereich`.
    WirkungsbereichTabbereich,
    /// `Wirkungsbereich::Navigator`.
    WirkungsbereichNavigator,
    /// `Wirkungsbereich::Vorschau`.
    WirkungsbereichVorschau,
    /// `Wirkungsbereich::Bildfolge`.
    WirkungsbereichBildfolge,
    /// `Wirkungsbereich::Ueberall`.
    WirkungsbereichUeberall,
    /// `Ortsmangel::Absolut`: der Satzteil in der Statuszeile zu einer
    /// Ortsangabe in `readers.toml`.
    OrtsmangelAbsolut,
    /// `Ortsmangel::LeeresStueck`.
    OrtsmangelLeeresStueck,
    /// `Ortsmangel::Punktstueck`.
    OrtsmangelPunktstueck,
    /// `Ortsmangel::MehrerePlatzhalter`.
    OrtsmangelMehrerePlatzhalter,
    /// `Namensfehler::Leer` und `Namenshinweis::Leer`: der Grund, aus dem ein
    /// Name nicht vergeben wird.
    NameLeer,
    /// `Namensfehler::Schraegstrich`.
    NameMitSchraegstrich,
    /// `Namensfehler::Nullbyte`.
    NameMitNullbyte,
    /// `Namensfehler::Punktname`.
    NamePunktname,
    /// `Kollision::Bestehender` in der Spalte der Vorschau beim
    /// Stapelumbenennen.
    KollisionBestehender,
    /// `Kollision::Doppelt`.
    KollisionDoppelt,
    /// `ablage::Grund::NichtLesbar`: der Satzteil in der Meldung zu einer
    /// ersetzten Ablagedatei.
    AblagegrundNichtLesbar,
    /// `ablage::Grund::Beschaedigt`.
    AblagegrundBeschaedigt,
    /// `ablage::Grund::NichtAnlegbar`.
    AblagegrundNichtAnlegbar,
    /// `Ersatz::Auslieferungszustand`: was an die Stelle der ersetzten
    /// Ablagedatei tritt.
    ErsatzAuslieferungszustand,
    /// `Ersatz::Nichts`.
    ErsatzNichts,
    /// `Pinfehler::KeineVierZiffern` im Blatt der PIN-Abfrage.
    PinKeineVierZiffern,
    /// `Abweisung::UmbruchImAufgabentext` in der Statuszeile des Editors.
    AbweisungUmbruchImAufgabentext,
    /// `Abweisung::UmbruchImThema`.
    AbweisungUmbruchImThema,
    /// `Abweisung::ThemenzeileImNotiztext`.
    AbweisungThemenzeileImNotiztext,
    /// `Abweisung::UngueltigesDatum`.
    AbweisungUngueltigesDatum,
    /// `Abweisung::KopfzeileImTermintext`.
    AbweisungKopfzeileImTermintext,
    /// `Marke::Geaendert` in der Zusammenfassung des Git-Bereichs, hinter
    /// einer Zahl.
    MarkeGeaendert,
    /// `Marke::Vorgemerkt`.
    MarkeVorgemerkt,
    /// `Marke::Neu`.
    MarkeNeu,
    /// `Marke::Konflikt`.
    MarkeKonflikt,
    /// `Marke::Umbenannt`.
    MarkeUmbenannt,
    /// Die erste Zaehlzeile des Default-Profils in der Vorschau.
    ZaehlzeileDateien,
    /// Die zweite Zaehlzeile des Default-Profils.
    ZaehlzeileOrdner,
    /// Die dritte Zaehlzeile des Default-Profils.
    ZaehlzeileVerknuepfungen,
    /// Die Einheit hinter einer Datenmenge ab 1.000 Bytes.
    EinheitKilobyte,
    /// Die Einheit ab 1.000.000 Bytes.
    EinheitMegabyte,
    /// Die Einheit ab 1.000.000.000 Bytes.
    EinheitGigabyte,
    /// Die Einheit ab 1.000.000.000.000 Bytes.
    EinheitTerabyte,
}

impl Text {
    /// Alle Schluessel, in der Reihenfolge der Aufzaehlung.
    pub const ALLE: [Text; 49] = [
        Text::WirkungsbereichDateifenster,
        Text::WirkungsbereichLeiste,
        Text::WirkungsbereichDateibereiche,
        Text::WirkungsbereichEditor,
        Text::WirkungsbereichEditortext,
        Text::WirkungsbereichEintraege,
        Text::WirkungsbereichReihenfolge,
        Text::WirkungsbereichAufgaben,
        Text::WirkungsbereichTermine,
        Text::WirkungsbereichGeheimnisse,
        Text::WirkungsbereichQuicknote,
        Text::WirkungsbereichTabbereich,
        Text::WirkungsbereichNavigator,
        Text::WirkungsbereichVorschau,
        Text::WirkungsbereichBildfolge,
        Text::WirkungsbereichUeberall,
        Text::OrtsmangelAbsolut,
        Text::OrtsmangelLeeresStueck,
        Text::OrtsmangelPunktstueck,
        Text::OrtsmangelMehrerePlatzhalter,
        Text::NameLeer,
        Text::NameMitSchraegstrich,
        Text::NameMitNullbyte,
        Text::NamePunktname,
        Text::KollisionBestehender,
        Text::KollisionDoppelt,
        Text::AblagegrundNichtLesbar,
        Text::AblagegrundBeschaedigt,
        Text::AblagegrundNichtAnlegbar,
        Text::ErsatzAuslieferungszustand,
        Text::ErsatzNichts,
        Text::PinKeineVierZiffern,
        Text::AbweisungUmbruchImAufgabentext,
        Text::AbweisungUmbruchImThema,
        Text::AbweisungThemenzeileImNotiztext,
        Text::AbweisungUngueltigesDatum,
        Text::AbweisungKopfzeileImTermintext,
        Text::MarkeGeaendert,
        Text::MarkeVorgemerkt,
        Text::MarkeNeu,
        Text::MarkeKonflikt,
        Text::MarkeUmbenannt,
        Text::ZaehlzeileDateien,
        Text::ZaehlzeileOrdner,
        Text::ZaehlzeileVerknuepfungen,
        Text::EinheitKilobyte,
        Text::EinheitMegabyte,
        Text::EinheitGigabyte,
        Text::EinheitTerabyte,
    ];
}

/// Eine Mengenangabe mit Einzahl und Mehrzahl.
///
/// Welche Form welche Zahl bekommt, sagt `Sprache::mehrzahl`; `{n}` steht
/// in der Form fuer die gruppierte Zahl, und die Einzahl darf es auslassen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zahlwort {
    /// Eine Datenmenge unter 1.000 Bytes, hinter `Sprache::menge`.
    Byte,
}

impl Zahlwort {
    /// Alle Zahlwoerter, in der Reihenfolge der Aufzaehlung.
    pub const ALLE: [Zahlwort; 1] = [Zahlwort::Byte];
}
