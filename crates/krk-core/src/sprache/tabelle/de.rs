//! Die deutsche Tabelle: die Quelle.
//!
//! Jeder Eintrag ist der Wortlaut, den die Oberflaeche vor dieser Arbeit an
//! der genannten Stelle trug, Zeichen fuer Zeichen; die Wortlautproben des
//! Baums halten ihn weiter gegen ihr eigenes Literal, und ein Vergleich mit
//! diesem Eintrag waere dort eine Tautologie. Mit Umlauten, wie es die
//! Umlautregel vom 260907 fuer nutzersichtbaren Text verlangt.

use super::super::{Text, Zahlwort};

/// Der deutsche Eintrag zu einem Schluessel.
pub(in super::super) const fn text(schluessel: Text) -> &'static str {
    match schluessel {
        Text::WirkungsbereichDateifenster => "Dateifenster",
        Text::WirkungsbereichLeiste => "Lesezeichen- und Geräteleiste",
        Text::WirkungsbereichDateibereiche => "Dateifenster, Vorschau und Editor",
        Text::WirkungsbereichEditor => "Editor",
        Text::WirkungsbereichEditortext => "Text im Editor",
        Text::WirkungsbereichEintraege => "Einträge im Editor",
        Text::WirkungsbereichReihenfolge => "Einträge in Dateireihenfolge im Editor",
        Text::WirkungsbereichAufgaben => "Aufgaben im Editor",
        Text::WirkungsbereichTermine => "Termine im Editor",
        Text::WirkungsbereichGeheimnisse => "Geheimnisse im Editor",
        Text::WirkungsbereichQuicknote => "Quicknote im Editor",
        Text::WirkungsbereichTabbereich => "Dateifenster und Vorschau",
        Text::WirkungsbereichNavigator => "Dateifenster, Leiste, Vorschau und Git-Bereich",
        Text::WirkungsbereichVorschau => "Vorschau",
        Text::WirkungsbereichBildfolge => "Dateifenster, solange die Vorschau eine Bildfolge zeigt",
        Text::WirkungsbereichUeberall => "überall",
        Text::OrtsmangelAbsolut => "ist ein absoluter Pfad",
        Text::OrtsmangelLeeresStueck => "trägt ein leeres Stück",
        Text::OrtsmangelPunktstueck => "trägt ein Stück . oder ..",
        Text::OrtsmangelMehrerePlatzhalter => {
            "trägt mehr als einen Platzhalter * und damit Kosten, die erst am Bestand feststünden"
        }
        Text::NameLeer => "der Name ist leer",
        Text::NameMitSchraegstrich => "ein Name darf keinen Schrägstrich enthalten",
        Text::NameMitNullbyte => "ein Name darf kein Nullbyte enthalten",
        Text::NamePunktname => "'.' und '..' sind keine Namen",
        Text::KollisionBestehender => "der Name ist schon vergeben",
        Text::KollisionDoppelt => "zweimal derselbe neue Name",
        Text::AblagegrundNichtLesbar => "ist nicht lesbar",
        Text::AblagegrundBeschaedigt => "ist beschädigt",
        Text::AblagegrundNichtAnlegbar => "ließ sich nicht anlegen",
        Text::ErsatzAuslieferungszustand => "und wird durch den Auslieferungszustand ersetzt",
        Text::ErsatzNichts => "und nichts tritt an ihre Stelle",
        Text::PinKeineVierZiffern => "Die PIN besteht aus genau vier Ziffern.",
        Text::AbweisungUmbruchImAufgabentext => {
            "Eine Aufgabe ist eine Zeile und trägt keinen Zeilenumbruch."
        }
        Text::AbweisungUmbruchImThema => "Ein Thema ist eine Zeile und trägt keinen Zeilenumbruch.",
        Text::AbweisungThemenzeileImNotiztext => {
            "Eine Zeile im Notiztext darf nicht mit „## “ beginnen, denn so beginnt die nächste Notiz."
        }
        Text::AbweisungUngueltigesDatum => {
            "Ein Datum steht als YYMMDD oder YYMMDD HH:MM, etwa 261002 oder 261002 09:30."
        }
        Text::AbweisungKopfzeileImTermintext => {
            "Eine Zeile im Termintext darf nicht mit „## “ beginnen, denn so beginnt der nächste Termin."
        }
        Text::MarkeGeaendert => "geändert",
        Text::MarkeVorgemerkt => "vorgemerkt",
        Text::MarkeNeu => "neu",
        Text::MarkeKonflikt => "in Konflikt",
        Text::MarkeUmbenannt => "umbenannt",
        Text::ZaehlzeileDateien => "Dateien",
        Text::ZaehlzeileOrdner => "Ordner",
        Text::ZaehlzeileVerknuepfungen => "Verknüpfungen",
        Text::EinheitKilobyte => "kB",
        Text::EinheitMegabyte => "MB",
        Text::EinheitGigabyte => "GB",
        Text::EinheitTerabyte => "TB",
    }
}

/// Einzahl und Mehrzahl zu einem deutschen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} Byte", "{n} Bytes"),
    }
}
