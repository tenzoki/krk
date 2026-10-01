//! Uebersetzt von einem Agenten am 261001; vom Nutzer noch nicht Flaeche
//! fuer Flaeche durchgesehen.
//!
//! Die englische Tabelle. Das Glossar und die Typografie stehen im Kopf von
//! [`super`].

use super::super::{Text, Zahlwort};

/// Der englische Eintrag zu einem Schluessel.
pub(in super::super) const fn text(schluessel: Text) -> &'static str {
    match schluessel {
        Text::WirkungsbereichDateifenster => "file pane",
        Text::WirkungsbereichLeiste => "bookmark and volume bar",
        Text::WirkungsbereichDateibereiche => "file pane, Preview and Editor",
        Text::WirkungsbereichEditor => "Editor",
        Text::WirkungsbereichEditortext => "text in the Editor",
        Text::WirkungsbereichEintraege => "entries in the Editor",
        Text::WirkungsbereichReihenfolge => "entries in file order in the Editor",
        Text::WirkungsbereichAufgaben => "tasks in the Editor",
        Text::WirkungsbereichTermine => "appointments in the Editor",
        Text::WirkungsbereichGeheimnisse => "secrets in the Editor",
        Text::WirkungsbereichQuicknote => "Quicknote in the Editor",
        Text::WirkungsbereichTabbereich => "file pane and Preview",
        Text::WirkungsbereichNavigator => "file pane, bar, Preview and Git area",
        Text::WirkungsbereichVorschau => "Preview",
        Text::WirkungsbereichBildfolge => "file pane, while the Preview shows a photo sequence",
        Text::WirkungsbereichUeberall => "everywhere",
        Text::OrtsmangelAbsolut => "is an absolute path",
        Text::OrtsmangelLeeresStueck => "contains an empty segment",
        Text::OrtsmangelPunktstueck => "contains a segment . or ..",
        Text::OrtsmangelMehrerePlatzhalter => {
            "contains more than one wildcard * and thus a cost that would only be known from the contents"
        }
        Text::NameLeer => "the name is empty",
        Text::NameMitSchraegstrich => "a name must not contain a slash",
        Text::NameMitNullbyte => "a name must not contain a null byte",
        Text::NamePunktname => "“.” and “..” are not names",
        Text::KollisionBestehender => "the name is already taken",
        Text::KollisionDoppelt => "the same new name twice",
        Text::AblagegrundNichtLesbar => "cannot be read",
        Text::AblagegrundBeschaedigt => "is damaged",
        Text::AblagegrundNichtAnlegbar => "could not be created",
        Text::ErsatzAuslieferungszustand => "and is replaced by the shipped version",
        Text::ErsatzNichts => "and nothing takes its place",
        Text::PinKeineVierZiffern => "The PIN consists of exactly four digits.",
        Text::AbweisungUmbruchImAufgabentext => "A task is one line and contains no line break.",
        Text::AbweisungUmbruchImThema => "A topic is one line and contains no line break.",
        Text::AbweisungThemenzeileImNotiztext => {
            "A line in the note text must not begin with “## ”, because that is how the next note begins."
        }
        Text::AbweisungUngueltigesDatum => {
            "A date is written as YYMMDD or YYMMDD HH:MM, for example 261002 or 261002 09:30."
        }
        Text::AbweisungKopfzeileImTermintext => {
            "A line in the appointment text must not begin with “## ”, because that is how the next appointment begins."
        }
        Text::MarkeGeaendert => "modified",
        Text::MarkeVorgemerkt => "staged",
        Text::MarkeNeu => "new",
        Text::MarkeKonflikt => "in conflict",
        Text::MarkeUmbenannt => "renamed",
        Text::ZaehlzeileDateien => "Files",
        Text::ZaehlzeileOrdner => "Folders",
        Text::ZaehlzeileVerknuepfungen => "Symbolic links",
        Text::EinheitKilobyte => "kB",
        Text::EinheitMegabyte => "MB",
        Text::EinheitGigabyte => "GB",
        Text::EinheitTerabyte => "TB",
    }
}

/// Einzahl und Mehrzahl zu einem englischen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} byte", "{n} bytes"),
    }
}
