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
        Text::MenueUeberKrk => "About KRK",
        Text::MenueTastenbelegungAlsMarkdown => "Save Key Bindings as Markdown",
        Text::FunktionsbereichAnwendung => "Application",
        Text::FunktionsbereichHome => "Home",
        Text::FunktionsbereichDateilisting => "File Listing",
        Text::FunktionsbereichDateioperationen => "File Operations",
        Text::FunktionsbereichTabs => "Tabs",
        Text::FunktionsbereichVorschau => "Preview",
        Text::FunktionsbereichLeisteUndFokus => "Bar and Focus",
        Text::FunktionsbereichEditor => "Editor",
        Text::FunktionsbereichGit => "Git",
        Text::FunktionsbereichTextbefehle => "Edit",
        Text::FunktionsbereichFenster => "Window",
        Text::KontextOeffnenMit => "Open With",
        Text::KontextZippen => "Zip",
        Text::KontextEntpacken => "Unzip",
        Text::KontextDuplizieren => "Duplicate…",
        Text::KontextImFinderOeffnen => "Open in Finder",
        Text::KontextImFinderAnzeigen => "Show in Finder",
        Text::BereichLesezeichen => "Bookmarks",
        Text::BereichLinks => "Left",
        Text::BereichRechts => "Right",
        Text::BereichVorschau => "Preview",
        Text::BereichEditor => "Editor",
        Text::BereichGit => "Git",
        Text::BereichLesezeichenLang => "the bookmarks and volumes bar",
        Text::BereichLinksLang => "the left file pane",
        Text::BereichRechtsLang => "the right file pane",
        Text::BereichVorschauLang => "the preview pane",
        Text::BereichEditorLang => "the built-in editor",
        Text::BereichGitLang => "the Git area",
        Text::SpalteName => "Name",
        Text::SpalteGroesse => "Size",
        Text::SpalteDatum => "Date",
        Text::SpalteTyp => "Type",
        Text::SpalteMarke => "Mark",
        Text::SpalteAenderungsdatum => "Date Modified",
        Text::LeisteBereichUmschalten => "Show or hide {bereich}",
        Text::LeisteSpalteUmschalten => "Show or hide the “{spalte}” column in both file lists",
        Text::LeisteTiefeHinweis => "Extend the current filter to the subtree",
        Text::LeisteInhaltHinweis => "Apply the current filter to the file contents as well",
        Text::LeisteUeberschriftLesezeichen => "Bookmarks",
        Text::LeisteUeberschriftGeraete => "Volumes and Locations",
        Text::LeisteLesezeichenFehlt => "{name} (missing)",
        Text::LeisteSinnbildTextstelle => "text location",
        Text::TypOrdner => "folder",
        Text::TypDatei => "file",
        Text::TypVerknuepfung => "symbolic link",
        Text::EintragsspalteAufgabe => "Task",
        Text::EintragsspalteThema => "Topic",
        Text::EintragsspalteNotiz => "Note",
        Text::EintragsspalteDatum => "Date",
        Text::EintragsspalteTermin => "Appointment",
        Text::BelegungReserviertEditor => "the Editor",
        Text::BelegungZusatzReserviert => "(reserved for {wofuer})",
        Text::BelegungZustellerMenue => "menu shortcut",
        Text::BelegungZusatzZusteller => "({zusteller})",
        Text::BelegungKeineFunktionGewaehlt => "no function is selected",
        Text::BelegungSucheLeer => "The search text is empty; every typed character searches.",
        Text::BelegungSucheTreffer => "Search “{text}”: match {stelle} of {anzahl}.",
        Text::BelegungSucheKeinTreffer => "Search “{text}”: no match.",
        Text::BelegungsansichtTitel => "Key Bindings",
        Text::BelegungsansichtZuweisen => "Assign",
        Text::BelegungsansichtAuslieferungszustand => "Shipped Version",
        Text::BelegungsansichtFertig => "Done",
        Text::BelegungsansichtTasteEingabe => "Return",
        Text::BelegungsansichtErlaeuterung => {
            "Every typed character searches both columns and jumps to the first match; the Return key goes to the next one, the Backspace key shortens the search text. Arrow keys choose the function. {zuweisen} ({zuweisen_taste}) records the next pressed combination; esc cancels the recording. {zuruecksetzen} ({zuruecksetzen_taste}) resets everything. {fertig} ({fertig_taste}) or esc leaves the view and saves the changes."
        }
        Text::BelegungsansichtErstWaehlen => "Select a function first, then press Assign.",
        Text::BelegungsansichtAufnahme => {
            "Now press the desired combination for “{name}”; esc cancels."
        }
        Text::BelegungsansichtZurueckgesetzt => {
            "The key bindings are reset to the shipped version."
        }
        Text::BelegungsansichtAufnahmeAbgebrochen => {
            "The recording is cancelled; the key bindings are unchanged."
        }
        Text::BelegungsansichtKeineFunktion => {
            "No function is selected; the key bindings are unchanged."
        }
        Text::BelegungsansichtZugewiesen => "“{funktion}” is now on {kombination}.",
        Text::BelegungsansichtTasteOhneNamen => {
            "This key has no name in the combination notation and cannot be stored; the key bindings are unchanged."
        }
        Text::BelegungsansichtSpalteFunktion => "Function",
        Text::BelegungsansichtSpalteBelegung => "Binding",
        Text::MarkdownUeberschrift => "# Key Bindings of KRK",
        Text::MarkdownTabellenkopf => "| Function | Combinations | Acts in |",
        Text::MarkdownNichtEingeordnet => "(not classified by KRK)",
        Text::MarkdownWirktTextfelderUndEditor => "text fields and Editor",
        Text::MarkdownGeschrieben => "Key bindings written: {pfad}",
        Text::MarkdownKeinBenutzerverzeichnis => {
            "the key bindings could not be written: the system names no home directory"
        }
        Text::MarkdownOrdnerFehlt => {
            "the key bindings could not be written: the folder of {pfad} is missing"
        }
        Text::MarkdownZugriffAbgelehnt => {
            "the key bindings could not be written: access to {pfad} is denied"
        }
        Text::MarkdownFehlgeschlagen => "the key bindings could not be written to {pfad}: {grund}",
        Text::TabelleKeineDateiAufDatentraeger => "the source provides no file on the volume",
        Text::TabelleNichtZuOeffnen => "{pfad} cannot be opened: {grund}",
        Text::TabelleZwischenablageLeer => "the clipboard is empty",
        Text::TabelleNichtAnBrowser => "{adresse} could not be handed to the system browser",
        Text::TabelleZwischenablageKeinZiel => {
            "the clipboard carries neither an absolute path nor a web address"
        }
        Text::TabelleNichtInDerListe => "{name} is not in the list",
        Text::FenstertitelQuicknote => "Quicknote",
        Text::StatuszeileFilterstand => {
            "Filter “{filtertext}”: {gezeigt} of {vorhanden} shown{liest}{zu_gross}{ausgeblendet}"
        }
        Text::StatuszeileInhaltWirdGelesen => ", reading content",
        Text::StatuszeileSeiteVon => "Page {aktuell} of {gesamt}",
        Text::StatuszeileBildVon => "Image {aktuell} of {gesamt}",
        Text::StatuszeileFolgeGekuerzt => "{grundsatz} (sequence truncated {gruende})",
        Text::StatuszeileLinkesDateifenster => "left file pane",
        Text::StatuszeileRechtesDateifenster => "right file pane",
        Text::StatuszeileMeldungMitSeite => "{seite}: {text}",
        Text::VorgangsartKopieren => "Copying",
        Text::VorgangsartVerschieben => "Moving",
        Text::VorgangsartInDenPapierkorb => "Moving to Trash",
        Text::VorgangsartUmbenennen => "Renaming",
        Text::VorgangsartPacken => "Zipping",
        Text::VorgangsartEntpacken => "Unzipping",
        Text::VorgangsartDuplizieren => "Duplicating",
        Text::VorgangAbbruchhinweis => "Esc cancels",
        Text::VorgangWirdVorbereitet => "{was} is being prepared: {positionen} · {abbruch}",
        Text::VorgangZeile => "{was}: {eintraege}, {menge}, {positionen} · {name} · {abbruch}",
        Text::VorgangWirdAbgebrochen => {
            "{was} is being cancelled, the operation will end shortly …"
        }
        Text::VorgangSchonEiner => "an operation is already running: {was}",
        Text::VorgangUebertragen => "{eintraege}, {menge} ({positionen})",
        Text::VorgangAbgebrochen => "{was} cancelled: {uebertragen} transferred",
        Text::VorgangFertig => "{was} finished: {uebertragen}",
        Text::UebersprungenZeile => "{name}: {grund}",
        Text::AnlegenFrageOrdner => "What should the new folder be called?",
        Text::AnlegenFrageDatei => "What should the new file be called?",
        Text::AnlegenBestaetigen => "Create",
        Text::AngelegtOrdner => "Folder “{name}” created",
        Text::AngelegtDatei => "File “{name}” created",
        Text::AnlegenKeineRechteOrdner => "no permission to create the folder “{name}” here",
        Text::AnlegenKeineRechteDatei => "no permission to create the file “{name}” here",
        Text::AnlegenGescheitert => "“{name}” could not be created: {fehler}",
        Text::NameSchonVergeben => "an entry named “{name}” already exists",
        Text::DuplikatFrage => "What should the duplicate be called?",
        Text::DuplikatBestaetigen => "Duplicate",
        Text::DuplikatMehrere => {
            "nothing to duplicate: several entries are marked, and exactly one file is duplicated"
        }
        Text::DuplikatNichtGewoehnlich => {
            "nothing to duplicate: “{name}” is {typ}, and only a regular file is duplicated"
        }
        Text::DuplikatTypOrdner => "a folder",
        Text::DuplikatTypVerknuepfung => "a symbolic link",
        Text::UmbenennenKeineRechte => "no permission to rename to “{name}” here",
        Text::UmbenennenGescheitert => "“{name}” could not be assigned: {fehler}",
        Text::OrdnerKeinOrdnerMehr => "{pfad} is no longer a folder",
        Text::OrdnerNichtMehrErreichbar => "{pfad} is no longer reachable: {fehler}",
        Text::KeinTerminal => {
            "no application with the bundle identifier “{kennung}” is installed; settings.toml names it under terminal, and a change takes effect only after a restart"
        }
        Text::PfadKopiert => "Path copied: {pfad}",
        Text::PfadeKopiert => "{n} paths copied",
        Text::NichtsBetroffen => "nothing {nennform}: nothing marked and nothing selected",
        Text::NennformZuKopieren => "to copy",
        Text::NennformZuOeffnen => "to open",
        Text::NennformZuPacken => "to zip",
        Text::NennformAnzuzeigen => "to show",
        Text::NennformZuDuplizieren => "to duplicate",
        Text::NichtsZuTeilen => "nothing to share: nothing here could go to the sharing services",
        Text::KeinArchiv => "nothing to unzip: there is no file with the extension .zip here",
        Text::MehrereArchive => {
            "nothing to unzip: there are several archives here, and the selection points at none of them"
        }
        Text::KeinFinder => "the Finder is not reachable: the system named no application for it",
        Text::AblageWeistTextAb => "the clipboard did not accept the text",
        Text::AbgelegtEiner => "copied: {name}",
        Text::AbgelegtMehrere => "{n} entries copied",
        Text::AbgelegtAusgeschnitten => {
            "{kopiert} – the destination does the moving (Finder: opt+cmd+v)"
        }
        Text::AblageWeistVerweiseAb => "the clipboard did not accept the entries",
        Text::EinfuegenKeinText => "nothing to paste: the clipboard holds no text",
        Text::EinfuegenMehrzeilig => "not pasted: the text has several lines",
        Text::EinfuegenNichtsTragbar => {
            "nothing to paste: the text has no character a name can carry"
        }
        Text::UebergebenEiner => "handed to the system: {name}",
        Text::UebergebenMehrere => "{n} entries handed to the system",
        Text::NichtAngenommenEiner => "the system did not accept {name}",
        Text::NichtAngenommenMehrere => "the system did not accept {n} of {gesamt} entries",
        Text::UebergebenUndAbgelehnt => "{genommen}; {abgelehnt}",
        Text::KeineAnwendung => "nothing to open: the system names no application for this entry",
        Text::UebergebenAnEiner => "handed to {anwendung}: {name}",
        Text::UebergebenAnMehrere => "{n} entries handed to {anwendung}",
        Text::NichtUebergebenAn => "not handed to {anwendung}: a path is not valid UTF-8",
        Text::BelegungsdateiZweiSchreiber => {
            "{datei} has two writers: a change by hand takes effect only at the next start, and the key-binding view (F1) overwrites it on leaving"
        }
        Text::BelegungsdateiFehltNoch => {
            "{datei} does not exist yet: it is created as soon as the key-binding view (F1) is left with a change"
        }
        Text::BelegungsdateiOhneAblageordner => {
            "{datei} cannot be shown: KRK is running without a data folder"
        }
        Text::Markierungsstand => "{n} marked, including {ordner}, {groesse}",
        Text::LoeschenOhnePapierkorb => {
            "the destination has no Trash, nothing was deleted; delete in the Finder"
        }
        Text::WarngrundUnentscheidbar => "from a destination of unknown kind",
        Text::WarngrundNetzlaufwerk => "from a network volume",
        Text::WarngrundCloudort => "from a cloud folder",
        Text::WarngrundAusserhalbBenutzerordner => "outside the home directory",
        Text::WarngrundImBenutzerordner => "directly in the home directory",
        Text::WarngrundArbeitsbaum => "from a Git working tree",
        Text::WarngrundGenauDieSchwelle => "with 25 entries in total",
        Text::WarngrundMehrAlsDieSchwelle => "with more than 25 entries in total",
        Text::LoeschenGeraeumtAus => "Removing from {ordner}.",
        Text::LoeschenAusserdem => "Also: {gruende}.",
        Text::LoeschenDarunterOrdner => "Among them {ordner}, each with its entire contents.",
        Text::BlattSteht => "not executed: a sheet is open over the window",
        Text::PfadNichtAbsolut => "{pfad} is not an absolute path",
        Text::PfadGibtEsNicht => "{pfad} does not exist: {fehler}",
        Text::PfadNichtLesbar => "{pfad} cannot be read: {fehler}",
        Text::PfadInKeinemOrdner => "{pfad} is in no folder",
        Text::WerksSchaltflaeche => "Reset",
        Text::WerksNotizordnerBleibt => "The notes folder and all files in it remain untouched.",
        Text::WerksFrage => {
            "Reset readers.toml, settings.toml and keymap.toml to factory settings?"
        }
        Text::WerksErlaeuterung => {
            "Each of the three files present in the data folder is set aside by KRK under its name with a timestamp appended, for example readers.toml.YYMMDD-HHMM, and none of them is deleted. Afterwards readers.toml and settings.toml are as this version of KRK ships them, except that settings.toml keeps the configured notes folder; keymap.toml is absent, and the shipped key bindings apply. {notizordner} KRK reads the new state immediately."
        }
        Text::WerksEigeneZuweisungen => {
            "All your own key assignments from keymap.toml thereby go out of use; afterwards they exist only in the backup."
        }
        Text::TabAusgefiltert => "{name} is filtered out.",
        Text::TabNichtMehrDa => "{name} is no longer there.",
        Text::TabNichtVollstaendigGelesen => "{ordner} could not be read completely: {fehler}",
        Text::PapierkorbKeinUtf8Pfad => "{pfad} is not a valid UTF-8 path",
        Text::BlattSchliessen => "Close",
        Text::BlattAbbrechen => "Cancel",
        Text::BlattUmbenennen => "Rename",
        Text::BlattFeldSuchenNach => "Find:",
        Text::BlattFeldErsetzenDurch => "Replace with:",
        Text::KonfliktUeberspringen => "Skip",
        Text::KonfliktInDenPapierkorbUndErsetzen => "Move to Trash and Replace",
        Text::KonfliktEndgueltigLoeschenUndErsetzen => "Delete Permanently and Replace",
        Text::KonfliktTastenhinweisEinZiel => {
            "Return and Esc cancel, Cmd+Return replaces, Opt+Return renames."
        }
        Text::KonfliktTastenhinweisMehrereZiele => {
            "Return skips, Cmd+Return replaces, Opt+Return renames, Esc cancels."
        }
        Text::KonfliktFrage => "“{name}” already exists at the destination",
        Text::KonfliktErlaeuterung => "Source: {quelle}\nDestination: {ziel}\n\n{hinweis}",
        Text::KonfliktFuerAlleWeiteren => "Apply to All Remaining",
        Text::LoeschblattErlaeuterung => {
            "{erlaeuterung}\n\nReturn and Esc cancel. Cmd+Return to confirm."
        }
        Text::NeuerungenBlattFrage => "Your data files and what this version brings",
        Text::OrtwahlWaehlen => "Choose",
        Text::OrtwahlFrage => "Where should the notes folder be?",
        Text::PinHinweis => {
            "The PIN keeps programs and agents from reading along. It does not protect against someone who copies the file and attacks it deliberately. A forgotten PIN locks the contents for good."
        }
        Text::PinAbweichung => "The two entries do not match.",
        Text::PinFrageFestlegen => "Set a new PIN for the secrets",
        Text::PinFrageEingeben => "Enter the PIN for the secrets",
        Text::PinFrageAendern => "Change the PIN for the secrets",
        Text::PinBestaetigenFestlegen => "Set",
        Text::PinBestaetigenEingeben => "Open",
        Text::PinBestaetigenAendern => "Change",
        Text::PinFeldNeuePin => "New PIN:",
        Text::PinFeldWiederholen => "Repeat:",
        Text::PinFeldPin => "PIN:",
        Text::PinFeldAltePin => "Old PIN:",
        Text::StapelSpalteBisher => "Current",
        Text::StapelSpalteNeu => "New",
        Text::StapelSpalteHinweis => "Note",
        Text::StapelErlaeuterung => {
            "The preview shows what the command would do. Renaming happens only with Return; Esc cancels. Entries with a note stay as they are."
        }
        Text::StapelFeldNummerAb => "Number from:",
        Text::StapelFeldStellen => "Digits:",
        Text::StapelZusammenfassungOhneKollisionen => {
            "{eintraege}, {umzubenennen} of them with a new name"
        }
        Text::StapelZusammenfassungMitKollisionen => "{eintraege}: {umbenannt}, {stehend}",
        Text::SucheWeitersuchen => "Find Next",
        Text::SucheErsetzen => "Replace",
        Text::SucheAlleErsetzen => "Replace All",
        Text::SucheErlaeuterung => {
            "Return finds the next match, Cmd+Return replaces the match, Opt+Return replaces all, Esc cancels."
        }
        Text::SucheFrage => "Find what?",
        Text::UngesichertSichern => "Save",
        Text::UngesichertVerwerfen => "Don’t Save",
        Text::UngesichertFrage => "“{name}” has unsaved changes",
        Text::UngesichertErlaeuterung => {
            "{pfad}\n\nReturn saves, Cmd+Return discards the changes, Esc cancels."
        }
        Text::FremdaenderungNeuLaden => "Reload",
        Text::FremdaenderungUeberschreiben => "Overwrite Anyway",
        Text::FremdaenderungFrage => "“{name}” has changed outside KRK",
        Text::FremdaenderungErlaeuterung => {
            "{pfad}\n\nReload discards the changes in the Editor, Overwrite replaces the version on disk. Return and Esc cancel, Opt+Return reloads, Cmd+Return overwrites."
        }
        Text::ZeilennummerFrage => "Which line?",
        Text::ZeilennummerSpringe => "Go",
        Text::PfadeingabeFrage => "Which folder?",
        Text::PfadeingabeGehe => "Go",
        Text::HinweisOk => "OK",
        Text::EditorTrefferVon => "Match {nummer} of {anzahl}",
        Text::EditorKeinTrefferFuer => "No match for “{text}”",
        Text::EditorKeinWeitererTrefferFuer => "No further match for “{text}”",
        Text::PinBleibt => "the PIN stays as it was",
        Text::PinNichtAbleitbarGrund => "the new PIN cannot be derived: {fehler}; {bleibt}",
        Text::EditorKeineTextmarkeInGeheimnissen => {
            "for secrets.txt KRK creates no text bookmark: it would write a line of the secrets in plain text into the bookmarks"
        }
        Text::EditorOhnePin => "it is encrypted and opens only with the PIN",
        Text::EditorKeineVerschluesselteDatei => "it is not an encrypted file in the notes folder",
        Text::EditorGeheimnisseZuGross => "it is too large for the Editor",
        Text::EditorGeheimnisseKeinDateizugriff => "KRK has no free file descriptor left",
        Text::EditorGeheimnisseNichtLesbar => "it cannot be read",
        Text::EditorFremdGeaendertNichtUeberschrieben => {
            "{pfad} has changed outside KRK and will not be overwritten"
        }
        Text::EditorVerschluesseltKeinKlartext => {
            "{pfad} is encrypted and will not be written in plain text"
        }
        Text::EditorNichtGesichert => "{pfad} could not be saved: {fehler}",
        Text::EditorFremdGeaendert => "{pfad} has changed outside KRK",
        Text::PinKeineDatei => "the Editor holds no file; {bleibt}",
        Text::PinNochNichtGesichert => "{pfad} carries no saved PIN yet; save first, then change",
        Text::PinWirdSchonGeaendert => "the PIN is already being changed",
        Text::PinFremdGeaendert => "{pfad} has changed outside KRK; {bleibt}",
        Text::PinNichtVerschluesselt => "{pfad} is not encrypted; {bleibt}",
        Text::PinAlteStimmtNicht => "the old PIN is wrong; {bleibt}",
        Text::PinNichtAbleitbar => "the new PIN could not be derived; {bleibt}",
        Text::PinGrundBleibt => "{grund}; {bleibt}",
        Text::PinDateiGrundBleibt => "{pfad}: {grund}; {bleibt}",
        Text::PinNichtMehrOffen => "{pfad} is no longer open; {bleibt}",
        Text::PinNichtMehrEntsperrt => "{pfad} is no longer unlocked; {bleibt}",
        Text::PinNichtLesbar => "{pfad} cannot be read; {bleibt}",
        Text::PinNichtAlsTextLesbar => "{pfad} is not readable as text; {bleibt}",
        Text::PinNichtGeschrieben => "{pfad} could not be written: {fehler}; {bleibt}",
        Text::EditorMarkeFuehrtAufZeile => "the bookmark leads to line {zeile}",
        Text::EditorZeilenZaehlenAbEins => {
            "lines count from 1; the cursor is at the start of the file"
        }
        Text::EditorKeineZeileMehr => {
            "the file no longer has a line {zeile}; the cursor is at the end of the file"
        }
        Text::EditorMarkenstelleGeaendert => "the remembered location has changed; {wohin}",
        Text::EditorGesichert => "{pfad} saved",
        Text::EditorKeineZeilennummer => "“{eingabe}” is not a line number",
        Text::EditorKeineSuche => "no search is running",
        Text::EditorKeinTrefferErsetzt => "no match replaced",
        Text::EditorPinGeaendert => "the PIN of {pfad} is changed",
        Text::EditorTermineAufsteigend => "Appointments sorted ascending",
        Text::EditorTermineAbsteigend => "Appointments sorted descending",
        Text::QuicknoteLeer => "The Quicknote is empty; the clipboard stays as it was.",
        Text::QuicknoteNichtKopiert => {
            "The Quicknote could not be copied to the clipboard; its text stays in place."
        }
        Text::QuicknoteZuGross => "The text is too large for the Quicknote; nothing was pasted.",
        Text::QuicknoteKeineTextmarken => "There are no text bookmarks in the Quicknote.",
        Text::QuicknoteLeeren => "Clear",
        Text::QuicknoteKopieren => "Copy",
        Text::EintragKeineAufgabentabelle => "the Editor shows no task table",
        Text::EintragKeineNotiztabelle => "the Editor shows no note table",
        Text::EintragKeineTermintabelle => "the Editor shows no appointment table",
        Text::EintragKeineAufgabeGewaehlt => "no task is selected",
        Text::EintragKeineNotizGewaehlt => "no note is selected",
        Text::EintragKeinTerminGewaehlt => "no appointment is selected",
        Text::EintragAufgabeHinzugefuegt => "new task at the end; return accepts the text",
        Text::EintragNotizHinzugefuegt => {
            "new note at the end; tab moves to the text, cmd+return accepts"
        }
        Text::EintragTerminHinzugefuegt => {
            "appointment added, with today’s date; tab moves to the text, cmd+return accepts"
        }
        Text::EintragAufgabeBearbeitung => "return accepts, esc discards",
        Text::EintragNotizBearbeitung => {
            "cmd+return accepts, tab changes the cell, return writes a line break in the text"
        }
        Text::EintragTerminBearbeitung => {
            "cmd+return accepts, tab changes the cell, return writes a line break in the appointment"
        }
        Text::EintragAufgabeUebernommen => "task accepted",
        Text::EintragNotizUebernommen => "note accepted",
        Text::EintragTerminUebernommen => "appointment accepted",
        Text::EintragAufgabeAbgehakt => "task checked off",
        Text::EintragNotizOhneKaestchen => "a note has no checkbox",
        Text::EintragTerminOhneKaestchen => "an appointment has no checkbox",
        Text::EintragAufgabeWiederOffen => "task open again",
        Text::EintragAufgabeVerschoben => "task moved",
        Text::EintragNotizVerschoben => "note moved",
        Text::EintragAufgabeSchonOben => "the task is already at the top",
        Text::EintragNotizSchonOben => "the note is already at the top",
        Text::EintragAufgabeSchonUnten => "the task is already at the bottom",
        Text::EintragNotizSchonUnten => "the note is already at the bottom",
        Text::EintragTermineNachDatum => {
            "Appointments are ordered by their date; none can be moved."
        }
        Text::EintragAufgabeGeloescht => "task deleted; cmd+z brings it back",
        Text::EintragNotizGeloescht => "note deleted; cmd+z brings it back",
        Text::EintragTerminGeloescht => "appointment deleted; cmd+z brings it back",
        Text::EintragZelleBleibt => "the cell stays in editing",
        Text::EintragAufgabeMitEscUebernommen => "task accepted; cmd+z takes it back",
        Text::EintragNotizMitEscUebernommen => "note accepted; cmd+z takes it back",
        Text::EintragTerminMitEscUebernommen => "appointment accepted; cmd+z takes it back",
        Text::EintragHeuteUnbestimmt => "Today’s date could not be determined.",
        Text::VorschauGeheimnishinweis => {
            "This file is encrypted and opens with F4 and the PIN in the Editor."
        }
        Text::VorschautabLeer => "Empty",
        Text::VorschautabZwischenablage => "Clipboard",
        Text::VorschauZwischenablageLeer => "The clipboard is empty.",
        Text::VorschauBildZuGross => {
            "The image in the clipboard is {groesse} MB. The Preview shows images up to {grenze} MB."
        }
        Text::VorschauNichtLesbar => "{pfad} could not be read: {fehler}",
        Text::VorschauLeertext => "No content. The selection in the file pane fills this tab.",
        Text::VorschauBildNichtDarstellbar => {
            "The image from the clipboard could not be displayed."
        }
        Text::MetadatenName => "Name: {name}",
        Text::MetadatenPfad => "Path: {pfad}",
        Text::MetadatenGroesse => "Size: {groesse}",
        Text::MetadatenGeaendert => "Modified: {datum}",
        Text::MetadatenRechte => "Permissions: {rechte}",
        Text::MetadatenTyp => "Type: {typ}",
        Text::StartAblageordnerNichtGeoeffnet => {
            "the data folder could not be opened, the session will not be saved: {fehler}"
        }
        Text::OrtUrsacheAblageordnerNichtGeoeffnet => {
            "the data folder could not be opened: {fehler}"
        }
        Text::StartSchreibsperreNichtGenommen => {
            "the write lock of the data folder cannot be taken, nothing is loaded and nothing is saved: {fehler}"
        }
        Text::OrtUrsacheSchreibsperreNichtGenommen => {
            "the write lock of the data folder could not be taken: {fehler}"
        }
        Text::OrtUrsacheStartNochNichtGelesen => "start-up has not read it yet",
        Text::LesezeichenZielFehlt => "“{name}” is missing: {pfad} no longer exists",
        Text::LesezeichenGesperrt => {
            "the bookmarks could not be changed, the write lock of the data folder cannot be taken: {fehler}"
        }
        Text::LesezeichenVonAndererInstanzGeaendert => {
            "this bookmark is no longer in the list as it was; another instance of KRK has changed or deleted it"
        }
        Text::LesezeichenNichtGesichert => "the bookmarks could not be saved: {fehler}",
        Text::LesezeichenNameFrage => "What should the bookmark be called?",
        Text::LesezeichenAngelegt => "Bookmark “{name}” created",
        Text::EditorHaeltKeineDatei => "the Editor holds no file",
        Text::HinweisTastenabgriffTitel => "KRK cannot read keystrokes",
        Text::HinweisTastenabgriffText => {
            "The keyboard tap could not be set up. Without it no key moves the selection and no keyboard shortcut works. KRK quits rather than continuing with a window that cannot be controlled from the keyboard."
        }
        Text::OrdnerNichtBeobachtet => {
            "the folders cannot be watched; outside changes appear only after a folder change"
        }
        Text::KeineAngezeigteDatei => "no file is shown that could be jumped to",
        Text::BildfolgeNochInVorbereitung => "The photo sequence is still being prepared.",
        Text::AngleichenFensterZuSchmal => {
            "the window is too narrow; nothing was shown and nothing was set"
        }
        Text::AngleichenZeigtSchon => "the other file pane already shows this folder",
        Text::AngleichenEingeblendetZeigtSchon => {
            "the other file pane was shown and already shows this folder"
        }
        Text::NeuerungenOhneAblageordner => {
            "there is no data folder, and so no files that could lag behind this version"
        }
        Text::NeuerungenGesperrt => {
            "What’s New cannot be checked: the write lock of the data folder cannot be taken ({fehler})"
        }
        Text::WerksOhneAblageordner => "There is no data folder, and so nothing to reset.",
        Text::WerksGesperrt => {
            "Nothing has been reset: the write lock of the data folder cannot be taken ({fehler})."
        }
        Text::BelegungNichtGesichert => "the key bindings apply, but could not be saved: {fehler}",
        Text::BelegungOhneAblageordner => {
            "the key bindings apply, but without a data folder they are not saved and will be lost on quitting"
        }
        Text::BelegungGesperrt => {
            "the key bindings apply, but are not saved: the write lock of the data folder cannot be taken ({fehler})"
        }
        Text::OrtKeinUtf8 => {
            "The chosen location cannot be written to settings.toml: its path is not valid UTF-8"
        }
        Text::OrtOhneAblageordner => {
            "There is no data folder and so no settings.toml into which KRK could write the location; the notes folder stays where it is"
        }
        Text::OrtGesperrt => {
            "The location cannot be written to settings.toml: the write lock of the data folder cannot be taken ({fehler}); the notes folder stays where it is"
        }
        Text::LoeschblattSchaltflaeche => "Move to Trash",
        Text::NichtsAusgewaehlt => "nothing is selected",
        Text::StapelKeineRegel => "nothing to rename: no rule could be built from the fields",
        Text::StapelJedeZeileMitHinweis => "nothing to rename: every line carries a note",
        Text::QuelleUndZielDerselbeOrdner => "source and destination are the same folder",
        Text::VorgangNichtGestartet => "the operation could not be started: {fehler}",
        Text::VorschauKeineDateiZumBearbeiten => "the Preview shows no file to edit",
        Text::QuicknoteFensterZuSchmal => "The window is too narrow for the Quicknote.",
        Text::SitzungNichtGesichert => "the session could not be saved: {fehler}",
        Text::WeitereInstanzOhneBuendel => {
            "KRK is not running from a bundle; another instance only starts the built KRK.app"
        }
        Text::AuswurfDateifensterZeigt => "{name} was ejected; the file pane now shows {ziel}",
        Text::StartOhneSitzungsrecht => {
            "another instance of KRK is already running; tabs and layout of this window will not be saved"
        }
        Text::StartNeuerungenNichtVermerkt => {
            "it could not be recorded that this version’s What’s New has been reported; the message will return at the next start: {fehler}"
        }
        Text::StartSitzungsrechtNichtAngefordert => {
            "the session right cannot be requested, the session will not be saved: {fehler}"
        }
        Text::StartLesezeichenNichtGeladen => {
            "the bookmarks could not be loaded, the write lock of the data folder cannot be taken: {fehler}"
        }
        Text::HeimOhneSperreAngelegt => {
            "the write lock of the data folder cannot be taken ({fehler}); F2 created without it"
        }
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
        Zahlwort::StatuszeileDateienZuGross => (", one file too large", ", {n} files too large"),
        Zahlwort::StatuszeileMarkierungenAusgeblendet => {
            (", one mark hidden", ", {n} marks hidden")
        }
        Zahlwort::BildfolgeGrenzeFotos => ("after {n} photo", "after {n} photos"),
        Zahlwort::BildfolgeGrenzeOrdner => ("after {n} folder", "after {n} folders"),
        Zahlwort::BildfolgeGrenzeEintraege => {
            ("at {n} entry of one folder", "at {n} entries of one folder")
        }
        Zahlwort::Eintraege => ("one entry", "{n} entries"),
        Zahlwort::AusgewaehltePositionen => ("one selected position", "{n} selected positions"),
        Zahlwort::Ordner => ("one folder", "{n} folders"),
        Zahlwort::VorgangUebersprungen => (", one entry skipped", ", {n} entries skipped"),
        Zahlwort::VorgangAusgelassen => (
            ", one entry left out as the destination of this run",
            ", {n} entries left out as destinations of this run",
        ),
        Zahlwort::UebersprungenFrage => ("One entry was skipped", "{n} entries were skipped"),
        Zahlwort::EinfuegenDateiverweise => (
            "not pasted: the clipboard holds {n} file reference",
            "not pasted: the clipboard holds {n} file references",
        ),
        Zahlwort::LoeschfrageEintraege => (
            "Move this entry {grund}to the Trash?",
            "Move these {n} entries {grund}to the Trash?",
        ),
        Zahlwort::StartMeldungen => (
            "There was one message at start",
            "There were {n} messages at start",
        ),
        Zahlwort::StapelFrage => ("Rename one entry", "Rename {n} entries in a batch"),
        Zahlwort::StapelEintraege => ("{n} entry", "{n} entries"),
        Zahlwort::StapelWerdenUmbenannt => ("{n} will be renamed", "{n} will be renamed"),
        Zahlwort::StapelBleibenStehen => ("{n} stays as it is", "{n} stay as they are"),
        Zahlwort::EditorTrefferErsetzt => ("one match replaced", "{n} matches replaced"),
        Zahlwort::EditorZeilenHinterDerLetzten => (
            "the file has {n} line; the cursor is at the end of the file",
            "the file has {n} lines; the cursor is at the end of the file",
        ),
        Zahlwort::QuicknoteKopiert => (
            "The Quicknote is in the clipboard: {n} character.",
            "The Quicknote is in the clipboard: {n} characters.",
        ),
        Zahlwort::BildfolgeVorbereitet => (
            "The photo sequence is being prepared: {n} photo.",
            "The photo sequence is being prepared: {n} photos.",
        ),
        Zahlwort::AuswurfDateifensterUndVerdeckteTabs => (
            "{name} was ejected; the file pane and one hidden tab now show {ziel}",
            "{name} was ejected; the file pane and {n} hidden tabs now show {ziel}",
        ),
        Zahlwort::AuswurfVerdeckteTabs => (
            "{name} was ejected; one hidden tab now shows {ziel}",
            "{name} was ejected; {n} hidden tabs now show {ziel}",
        ),
        Zahlwort::GekuerztWeitere => ("… and {n} more", "… and {n} more"),
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
