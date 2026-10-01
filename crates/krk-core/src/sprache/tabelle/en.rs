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
        Text::VorgangKeineRechte => "no permission",
        Text::VorgangGibtEsNichtMehr => "no longer exists",
        Text::VorgangAmZielStehtEintrag => "an entry already exists at the destination",
        Text::VorgangKeinPlatzAufDemDatentraeger => "no space left on the volume",
        Text::VorgangKeinArbeitsfaden => "no worker thread available: {grund}",
        Text::VorgangNeuerNameFehlt => "the new name is missing",
        Text::VorgangZielordnerFehlt => "the destination folder is missing",
        Text::VorgangPackenNichtQuelleFuerQuelle => "zipping does not run source by source",
        Text::VorgangPfadBenenntKeinenEintrag => "the path does not name an entry",
        Text::VorgangQuelleUndZielDerselbeEintrag => "source and destination are the same entry",
        Text::VorgangZielLiegtInDerQuelle => "the destination lies inside the source",
        Text::VorgangZielNichtErsetzt => "the destination could not be replaced: {grund}",
        Text::VorgangNachAbbruchNichtWeggeraeumt => "not removed after cancelling: {grund}",
        Text::VorgangOrdnerangabenNichtKopiert => {
            "contents copied, but not the folder’s permissions and date: {grund}"
        }
        Text::VorgangOrdnerSelbstBlieb => "contents moved, but the folder itself remained: {grund}",
        Text::VorgangNichtVollstaendigKopiert => "not completely copied, left in the source",
        Text::VorgangKopiertAberInQuelleGeblieben => "copied, but left in the source: {grund}",
        Text::VorgangKeinPapierkorb => "no Trash attached; nothing was deleted",
        Text::VorgangKeineGewoehnlicheDatei => "not a regular file",
        Text::VorgangZielNichtInPapierkorb => {
            "the destination could not be moved to the Trash: {grund}"
        }
        Text::EntpackenEintragFuehrtHeraus => {
            "“{name}” leads out of the destination folder and is skipped"
        }
        Text::EntpackenEintragMitGrund => "“{name}”: {grund}",
        Text::EntpackenAmZielStehtVerknuepfung => {
            "“{name}”: a symbolic link already exists at the destination"
        }
        Text::EntpackenWegMitUnzulaessigemBestandteil => {
            "the path to the entry contains an invalid component"
        }
        Text::EntpackenWegDurchVerknuepfung => {
            "the path to the entry passes through a symbolic link out of the destination folder"
        }
        Text::EntpackenDateiStattOrdnerAufDemWeg => {
            "on the path to the entry there is a file where a folder should be"
        }
        Text::EntpackenVerweiszielKeinText => "the link target is not valid text",
        Text::PackenArchivUnfertig => "the archive was left unfinished: {fehler}",
        Text::PackenKeinPlatzImArchiv => "no room in the archive: {fehler}",
        Text::PackenHalberEintragImArchiv => {
            "the half-written entry remained in the archive: {fehler}"
        }
        Text::PackenNichtInsArchivGeschrieben => "not written to the archive: {fehler}",
        Text::StapelKeinStartwert => "“{text}” is not a starting value for the numbering",
        Text::StapelKeineStellenzahl => {
            "“{text}” is not a number of digits between 1 and {hoechste}"
        }
        Text::EditorKeinFreierDateizugriff => {
            "{pfad} cannot be opened right now: KRK has no file descriptor left ({grund}); try again after the running search has finished"
        }
        Text::EditorNichtZuOeffnen => "{pfad} cannot be opened in the Editor: {grund}",
        Text::EditorZuGross => {
            "{pfad} is {groesse} bytes, too large for the Editor; the limit is {grenze} bytes"
        }
        Text::EditorKeineTextdatei => "{pfad} is not a text file and will not be opened",
        Text::EditorOrdnerHatKeinenText => "a folder has no text the Editor could show",
        Text::EditorKeineGewoehnlicheDatei => "that is not a regular file",
        Text::LesenDatenschutzsperre => {
            "macOS blocks access to “{name}”. To allow it: System Settings › Privacy & Security › Full Disk Access › KRK, then restart KRK."
        }
        Text::LesenKeinVerzeichnis => "{pfad} is not a directory",
        Text::LesenPfadMitNullbyte => "{pfad} contains a null byte",
    }
}

/// Einzahl und Mehrzahl zu einem englischen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} byte", "{n} bytes"),
    }
}
