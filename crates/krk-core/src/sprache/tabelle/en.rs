//! Uebersetzt von einem Agenten am 261001; vom Nutzer noch nicht Flaeche
//! fuer Flaeche durchgesehen.
//!
//! Die englische Tabelle. Das Glossar und die Typografie stehen im Kopf von
//! [`super`].

use super::super::{Text, Zahlwort};
use crate::tasten::belegung::{Kommando, Zugestellt};

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
        Text::AblageKeinGueltigesUtf8 => "not a valid UTF-8 sequence",
        Text::AblageOhneOberstenSchluessel => {
            "the file carries no top-level key at all, and KRK never writes it that way"
        }
        Text::AblageKeinBenutzerverzeichnis => "the system names no home directory",
        Text::AtomarOhneDateinamen => "{pfad} carries no file name",
        Text::AtomarRechteNichtUebertragen => {
            "the permissions {soll} of {pfad} could not be transferred; the neighboring file is at {gesetzt}"
        }
        Text::ErsetzungOhneSicherung => "{datei} {beschreibung} {ersatz}: {einzelheit}",
        Text::ErsetzungGesichert => {
            "The previous version is under {sicherung}; {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::ErsetzungGekuerzt => {
            "The previous version is under {sicherung}, truncated: only its first {grenze} bytes are saved; {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::ErsetzungSchonVorhanden => {
            "The previous version has been under {sicherung} since an earlier start and stays there; {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::ErsetzungSicherungGescheitert => {
            "The content could not be set aside ({fehler}); {datei} {beschreibung} {ersatz}: {einzelheit}"
        }
        Text::EinstellungenVerweis => {
            "settings.toml is a symbolic link, and KRK does not replace it with a file; the location stays as it is. Enter by hand in the target file: {zeile}"
        }
        Text::EinstellungenBeschaedigt => {
            "settings.toml must first be corrected by hand, KRK does not write it like this: {befund}"
        }
        Text::EinstellungenNichtLesbar => {
            "settings.toml cannot be read, KRK does not write it: {befund}"
        }
        Text::EinstellungenIntern => {
            "settings.toml stays as it is: the result would have changed more than the notes folder ({befund})"
        }
        Text::EinstellungenNichtGeschrieben => {
            "settings.toml could not be written and stays as it was: {befund}"
        }
        Text::EinstellungenKeineGewoehnlicheDatei => {
            "in its place stands something other than an ordinary file"
        }
        Text::EinstellungenNotizordnerKeinEinzelwert => {
            "notizordner does not stand as a single value on a line “notizordner = …”"
        }
        Text::EinstellungenNeuerWertFehlt => "the new value does not stand as notizordner",
        Text::EinstellungenAndererWertGeaendert => "another value of the file would have changed",
        Text::WerksVerweis => {
            "{datei} is a symbolic link, and KRK does not replace it with a file; nothing has been reset."
        }
        Text::WerksKeineDatei => {
            "In the place of {datei} stands something other than an ordinary file; nothing has been reset."
        }
        Text::WerksNichtLesbar => "{datei} cannot be queried ({befund}); nothing has been reset.",
        Text::WerksKeineOrtszeit => {
            "The clock of this device yields no timestamp for the backups; nothing has been reset."
        }
        Text::WerksKeinFreierName => {
            "For {datei} no free backup name is left in this minute; nothing has been reset."
        }
        Text::WerksNichtBeiseitegelegt => {
            "{datei} could not be set aside ({befund}); nothing has been reset."
        }
        Text::WerksNichtVorbereitet => {
            "The new version of {datei} could not be written ({befund}); nothing has been reset."
        }
        Text::WerksEinstellungen => "{befund}. Nothing has been reset.",
        Text::WerksNichtZurueckgebaut => {
            "{hindernis} A second copy of the unchanged content remains under {pfade}."
        }
        Text::WerksZurueckgesetzt => "Reset to factory settings.",
        Text::WerksTeilweiseZurueckgesetzt => "Only partly reset to factory settings.",
        Text::WerksBeiseitegelegt => "Set aside: {pfade}.",
        Text::WerksStandNichtDa => "{datei} was not there, nothing has been set aside for it.",
        Text::WerksDateiNichtZurueckgesetzt => "{datei} has not been reset: {fehler}.",
        Text::NeuerungenStartzeile => {
            "New in this version: {teile}. Your files are under {ordner}."
        }
        Text::NeuerungenDateiFehlt => {
            "This file is not in your data folder; only what is there is compared."
        }
        Text::NeuerungenDateiBeschaedigt => "This file is damaged and is therefore not compared.",
        Text::NeuerungenNeuInDieserFassung => "New in this version",
        Text::NeuerungenNurInIhrerDatei => "Only in your file",
        Text::NeuerungenZeileLeer => "{ueberschrift}: —",
        Text::NeuerungenZeile => "{ueberschrift}: {namen}",
        Text::NeuerungenKeineEigenenEintraege => {
            "{zeile} (this file cannot carry entries of its own; KRK rejects an unknown entry as damaged)"
        }
        Text::NeuerungenPreisLeser => {
            "A profile your file does not carry costs the summary for that location: the Preview shows the metadata there."
        }
        Text::NeuerungenPreisEinstellungen => {
            "A key your file does not carry costs only the explanatory comment block; the value itself KRK takes from the shipped version."
        }
        Text::NeuerungenPreisBelegung => {
            "A function your file does not carry costs its shipped key combinations; it remains reachable through the main menu."
        }
        Text::NeuerungenSchlusssatz => {
            "Shown is the state KRK read last. What KRK works with has been fixed since start-up: a changed file takes effect only at the next start."
        }
        Text::OrtKeinBenutzerverzeichnis => {
            "The system names no home directory, so there is no notes folder"
        }
        Text::OrtLeer => {
            "The notes folder in settings.toml is empty; valid is a location starting with “~/” or “/”, by default “~/{ordnername}”"
        }
        Text::OrtNichtAbsolut => {
            "The notes folder “{wert}” in settings.toml starts neither with “~/” nor with “/”"
        }
        Text::OrtFremdesBenutzerverzeichnis => {
            "The notes folder “{wert}” in settings.toml names another user’s home directory; valid is “~/” for your own or a path starting with “/”"
        }
        Text::OrtKeinText => {
            "The notes folder in settings.toml is not a text but {wert}; valid is a location in quotation marks, such as {beispiel}"
        }
        Text::OrtImAblageordner => {
            "The notes folder “{wert}” lies in KRK’s data folder; a tool that removes KRK would take it along, so it does not count"
        }
        Text::OrtEinstellungenBeschaedigt => {
            "settings.toml {satzteil}, so no notes folder applies, and F2 creates nothing: correct settings.toml and restart KRK, or after correcting it set the location via “Home” → “Choose Location…”"
        }
        Text::OrtEinstellungenUngelesen => {
            "KRK could not read settings.toml at start-up ({ursache}), so no notes folder applies, and F2 creates nothing: restart KRK"
        }
        Text::OrtWechsel => {
            "The notes folder is now “{neu}”; at the old location “{alt}” everything stays in place, and F2 leads to the new one"
        }
        Text::OrtAbweisung => {
            "First close {datei} in the Editor; as long as the Editor holds a file of the notes folder, KRK chooses no other location"
        }
        Text::OrtGewaehlt => "The notes folder is now “{neu}”, and F2 leads there",
        Text::OrtSchonDerOrt => "“{neu}” is already the notes folder; settings.toml stays as it is",
        Text::HeimKeinOrdner => {
            "{ordner} is not a folder; KRK creates nothing there and opens no tab"
        }
        Text::HeimNichtErreichbar => "{ordner} is not reachable: {grund}",
        Text::HeimNichtAnlegbar => "{name} cannot be created: {grund}",
        Text::HeimObererOrdnerFehlt => {
            "{ordner} cannot be created because the folder above it is missing, such as a volume that is not mounted; KRK creates nothing"
        }
        Text::HeimMerkerNichtVermerkt => {
            "KRK cannot remember that the old notes have been taken over ({grund}); the next F2 tries again, and if {ort} is deleted before then, it takes them over once more"
        }
        Text::HeimMerkerOhneAblage => {
            "Without its data folder KRK cannot remember that the old notes have been taken over; a later F2 catches up on that, and if {ort} is deleted before then, it takes them over once more"
        }
        Text::HeimGeheimnisseUmbenannt => "{alt} is now called {neu}",
        Text::HeimGeheimnisseBeideStehen => {
            "{neu} and {alt} both exist in {ort}; KRK renames neither, and {neu} counts"
        }
        Text::HeimGeheimnisseAlterNameBleibt => {
            "{alt} is now also called {neu}, the old name cannot be removed: {grund}"
        }
        Text::HeimGeheimnisseNichtUmbenannt => {
            "{alt} cannot be renamed to {neu} ({grund}); it stays unchanged, and {neu} is not created"
        }
        Text::HeimZettelNotizenStandenSchon => {
            "The old notes have not been taken over because {notizen} already existed; {dateien} remain unchanged in the data folder"
        }
        Text::HeimZettelGescheitert => {
            "The old notes have not been taken over ({grund}); {dateien} remain unchanged in the data folder"
        }
        Text::HeimZettelThemenzeile => {
            "{thema} has not been taken over because it carries a line starting with “## ”; {datei} remains unchanged in the data folder"
        }
        Text::HeimZettelUnlesbar => {
            "{thema} has not been taken over ({grund}); {datei} remains unchanged in the data folder"
        }
        Text::HeimZettelZuGross => "too large at {groesse} bytes",
        Text::HeimZettelKeinText => "no readable text",
        Text::HeimAngefangeneDateiBleibt => {
            "{fehler}; the started file cannot be removed: {entfernen}"
        }
        Text::Und => "and",
        Text::TresorKeinZufall => "The system provides no random value: {grund}",
        Text::TresorAbleitung => "The key cannot be derived: {grund}",
        Text::TresorVerschluesselung => "The content cannot be encrypted",
        Text::TresorPinFalschOderVeraendert => "Wrong PIN or file changed",
        Text::TresorKopfBeschaedigt => "The header of the file is damaged: {grund}",
        Text::TresorKopfKennungFehlt => "the identifier at the start is missing",
        Text::TresorKopfAbgeschnitten => "the file is truncated",
        Text::TresorKopfUnbekannteVersion => "unknown format version {version}",
        Text::TresorKopfUnbekannteAbleitung => "unknown derivation {ableitung}",
        Text::TresorKopfUngueltigeParameter => "the parameters of the derivation are invalid",
        Text::ZusammenfassungKopf => "Name: {name}\nPath: {pfad}",
        Text::ZusammenfassungBlockzeile => "{beschriftung}:",
        Text::ZusammenfassungZeile => "{beschriftung}: {wert}",
        Text::ZusammenfassungPlatzhalter => "--",
        Text::WertUeberGrenze => "at least {gezaehlt} (reading stopped at {grenze} entries)",
        Text::Ja => "yes",
        Text::Nein => "no",
        Text::ProfilMeldung => "Profile “{profil}”: {grund}",
        Text::ProfilZeilenmeldung => "Profile “{profil}”, line “{beschriftung}”: {grund}",
        Text::ProfilMehrereBausteine => {
            "it names {anzahl} building blocks ({namen}) instead of exactly one"
        }
        Text::ProfilKeinBaustein => "it names none of the four building blocks ({namen})",
        Text::ProfilOhneErkennung => {
            "it names neither a path pattern nor a marker file and could therefore never match"
        }
        Text::ProfilBildfolge => "the photo sequence: {grund}",
        Text::ProfilPfadmuster => "the path pattern",
        Text::ProfilKennzeichendatei => "the marker file",
        Text::ProfilErkennungsmusterNichtUebersetzt => "{was} {muster} cannot be compiled: {grund}",
        Text::ProfilMusterNichtUebersetzt => "the pattern {muster} cannot be compiled: {grund}",
        Text::ProfilFeldmusterFanggruppen => {
            "the field pattern {muster} carries {gruppen} capture groups instead of exactly one"
        }
        Text::ProfilOrtsangabe => "the location {angabe} {mangel}",
        Text::ProfilOrtsangabeMitPlatzhalter => {
            "the location {angabe} carries a wildcard, and the building block “{baustein}” accepts none: it reads files and needs their path for that, which a merged reading state does not carry"
        }
        Text::ProfilJuengsteNull => "juengste with anzahl = 0 can never show an entry",
        Text::GitKeinRepository => "This folder is not in any Git repository.",
        Text::GitOhneCommit => "no commit yet",
        Text::GitUnveraendert => "unchanged",
        Text::GitImOrdner => "{marken} in this folder",
        Text::GitKopfAbgeloest => "{kurzhash} (detached)",
        Text::TasteNameFehlt => "the key name is missing",
        Text::TasteKeineZusatztaste => "“{text}” is not a modifier key; allowed are {erlaubt}",
        Text::TasteFnKeineZusatztaste => {
            "fn is not a modifier key of a key binding; KRK looks function keys up by their key code, and F3 with fn held produces the same key code as a bare F3"
        }
        Text::TasteZusatztasteDoppelt => "the modifier key “{text}” appears twice",
        Text::TasteReihenfolgeVerletzt => {
            "“{zusatztaste}” comes after “{hinter}”; the order is {reihenfolge}"
        }
        Text::TasteUnbekannterName => "“{text}” is not a key name of this notation",
        Text::FunktionsnameMitKennung => "“{name}” ({kennung})",
        Text::BelegungKonflikt => {
            "the combination {kombination} already belongs to the function {andere} and cannot be assigned to the function {bewerber} as well"
        }
        Text::BelegungSchreibweise => {
            "the function {kennung} carries the combination “{text}”: {fehler}"
        }
        Text::BelegungUnbekannteFunktion => "KRK knows no function named {kennung}",
        Text::BelegungFunktionDoppelt => "the function {kennung} appears twice",
    }
}

/// Einzahl und Mehrzahl zu einem englischen Zahlwort.
pub(in super::super) const fn zahlwort(schluessel: Zahlwort) -> (&'static str, &'static str) {
    match schluessel {
        Zahlwort::Byte => ("{n} byte", "{n} bytes"),
        Zahlwort::NeuerungenEintraegeIn => ("{n} entry in {datei}", "{n} entries in {datei}"),
        Zahlwort::HeimZettelUebernommen => (
            "{themen} taken over as a note into {notizen}",
            "{themen} taken over as notes into {notizen}",
        ),
        Zahlwort::MarkeGeaendert => ("{n} modified", "{n} modified"),
        Zahlwort::MarkeVorgemerkt => ("{n} staged", "{n} staged"),
        Zahlwort::MarkeNeu => ("{n} new", "{n} new"),
        Zahlwort::MarkeKonflikt => ("{n} in conflict", "{n} in conflict"),
        Zahlwort::MarkeUmbenannt => ("{n} renamed", "{n} renamed"),
    }
}

/// Der englische Name eines Kommandos: der Eintrag des Hauptmenues, der
/// Belegungsansicht und jeder Konfliktmeldung.
pub(in super::super) const fn kommandoname(schluessel: Kommando) -> &'static str {
    match schluessel {
        Kommando::AuswahlHoch => "Move Selection Up One Entry",
        Kommando::AuswahlRunter => "Move Selection Down One Entry",
        Kommando::SeiteHoch => "Move Selection Up One Screen Page",
        Kommando::SeiteRunter => "Move Selection Down One Screen Page",
        Kommando::Listenanfang => "Go to Top of List",
        Kommando::Listenende => "Go to End of List",
        Kommando::Oeffnen => "Enter Selected Folder",
        Kommando::OrdnerAufwaerts => "Go to Parent Folder",
        Kommando::OrdnerDerDatei => "Show Folder of Displayed File",
        Kommando::OrdnerAngleichen => "Set Other File Pane to This Folder",
        Kommando::Pfadeingabe => "Enter Path and Go There",
        Kommando::MarkierungUmschalten => "Mark Entry and Move to Next",
        Kommando::AlleMarkieren => "Mark All Entries",
        Kommando::MarkierungAufheben => "Clear All Marks",
        Kommando::MarkierungUmkehren => "Invert Marks",
        Kommando::SortierungName => "Sort by Name",
        Kommando::SortierungGroesse => "Sort by Size",
        Kommando::SortierungDatum => "Sort by Date Modified",
        Kommando::SortierungTyp => "Sort by Type",
        Kommando::SortierrichtungUmkehren => "Reverse Sort Order",
        Kommando::VersteckteUmschalten => "Show or Hide Hidden Files",
        Kommando::SpalteGroesseUmschalten => "Show or Hide Size Column",
        Kommando::SpalteDatumUmschalten => "Show or Hide Date Modified Column",
        Kommando::SpalteTypUmschalten => "Show or Hide Type Column",
        Kommando::TiefeSucheUmschalten => "Turn Deep Search On or Off",
        Kommando::InhaltssucheUmschalten => "Turn Content Search On or Off",
        Kommando::ZwischenablageSpringen => "Go to Clipboard Contents",
        Kommando::ZwischenablageAnsehen => "Show Clipboard",
        Kommando::TabNeu => "Open New Tab",
        Kommando::TabSchliessen => "Close Active Tab",
        Kommando::TabNaechster => "Next Tab",
        Kommando::TabVoriger => "Previous Tab",
        Kommando::FensterWechseln => "Switch Active File Pane",
        Kommando::LeisteUmschalten => "Show or Hide Bookmarks and Volumes Bar",
        Kommando::ErstesFensterUmschalten => "Show or Hide Left File Pane",
        Kommando::ZweitesFensterUmschalten => "Show or Hide Second File Pane",
        Kommando::VorschauUmschalten => "Show or Hide Preview",
        Kommando::FensterEinblenden => "Show Window",
        Kommando::FensterSchliessen => "Close Window",
        Kommando::BereichVerbreitern => "Widen Active Area",
        Kommando::BereichVerschmaelern => "Narrow Active Area",
        Kommando::Kopieren => "Copy to Other Pane",
        Kommando::Verschieben => "Move to Other Pane",
        Kommando::InPapierkorb => "Move to Trash",
        Kommando::Abbrechen => "Cancel Running Operation",
        Kommando::OrdnerAnlegen => "New Folder",
        Kommando::DateiAnlegen => "New Empty File",
        Kommando::UmbenennenStapel => "Batch Rename",
        Kommando::Umbenennen => "Rename",
        Kommando::TerminalOeffnen => "Open Folder in Terminal",
        Kommando::OrdnerpfadKopieren => "Copy Path of Displayed Folder",
        Kommando::EintragspfadKopieren => "Copy Path of Entry",
        Kommando::MitStandardprogrammOeffnen => "Open with Default Application",
        Kommando::Teilen => "Share",
        Kommando::LesezeichenAnlegen => "Add Bookmark",
        Kommando::LesezeichenUmbenennen => "Rename Bookmark",
        Kommando::LesezeichenLoeschen => "Delete Bookmark",
        Kommando::LesezeichenHoch => "Move Bookmark Up",
        Kommando::LesezeichenRunter => "Move Bookmark Down",
        Kommando::FokusLeiste => "Focus Bookmarks and Volumes Bar",
        Kommando::FokusDateifenster => "Focus Back to File Pane",
        Kommando::FokusVorschau => "Focus Preview",
        Kommando::Bearbeiten => "Edit",
        Kommando::EditorRundweg => "To Editor and Back",
        Kommando::FokusEditor => "Focus Editor",
        Kommando::EditorSchliessen => "Close Editor",
        Kommando::EditorUmschalten => "Show or Hide Editor",
        Kommando::EditorAnsichtUmschalten => "Switch Between Raw and Formatted View",
        Kommando::EditorSichern => "Save",
        Kommando::EditorZeileSpringen => "Go to Line",
        Kommando::EditorSuchen => "Find in Text",
        Kommando::EditorWeitersuchen => "Find Next",
        Kommando::EditorRueckwaertsSuchen => "Find Previous",
        Kommando::EditorErsetzen => "Replace",
        Kommando::EditorAlleErsetzen => "Replace All",
        Kommando::QuicknoteUmschalten => "Open or Close Quicknote",
        Kommando::QuicknoteKopieren => "Copy Quicknote and Close",
        Kommando::QuicknoteLeeren => "Clear Quicknote",
        Kommando::EintragHinzufuegen => "Add Entry",
        Kommando::EintragBearbeiten => "Edit Entry",
        Kommando::EintragHoch => "Move Entry Up",
        Kommando::EintragRunter => "Move Entry Down",
        Kommando::EintragLoeschen => "Delete Entry",
        Kommando::AufgabeAbhaken => "Check Off or Reopen Task",
        Kommando::PinAendern => "Change PIN",
        Kommando::TermineRichtungUmkehren => "Appointments: Reverse Sort Order",
        Kommando::BelegungAnsehen => "Show Key Bindings",
        Kommando::BelegungsdateiAnsehen => "Open Key Bindings File",
        Kommando::Beenden => "Quit KRK",
        Kommando::WeitereInstanz => "Launch Another Instance",
        Kommando::Notizordner => "Open Notes Folder",
        Kommando::OrtWaehlen => "Choose Location…",
        Kommando::VorschauVergroessern => "Zoom In Preview",
        Kommando::VorschauVerkleinern => "Zoom Out Preview",
        Kommando::VorschauAusgangsgroesse => "Preview at Actual Size",
        Kommando::GitBereichUmschalten => "Show or Hide Git Area",
        Kommando::FokusGit => "Focus Git Area",
        Kommando::SpalteMarkeUmschalten => "Show or Hide Mark Column",
        Kommando::NeuerungenZeigen => "Show What’s New",
        Kommando::Werkseinstellungen => "Reset to Factory Settings…",
        Kommando::BildVor => "Next Image",
        Kommando::BildZurueck => "Previous Image",
        Kommando::ZumBild => "Go to Displayed Image",
    }
}

/// Der englische Name einer vom Hauptmenue zugestellten Funktion.
pub(in super::super) const fn zugestellt_name(schluessel: Zugestellt) -> &'static str {
    match schluessel {
        Zugestellt::FilterEinfuegen => "Paste into Filter",
        Zugestellt::TextAusschneiden => "Cut",
        Zugestellt::TextKopieren => "Copy",
        Zugestellt::TextEinfuegen => "Paste",
        Zugestellt::TextAllesAuswaehlen => "Select All",
        Zugestellt::TextRueckgaengig => "Undo",
        Zugestellt::TextWiederholen => "Redo",
    }
}
