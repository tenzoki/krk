//! Wie betroffene Eintraege im Finder aufgedeckt werden.
//!
//! ```text
//! Pfade ──> NSURL::fileURLWithPath: ──> NSArray ──> activateFileViewerSelectingURLs:
//!                                                              │
//!                                                              └──> nichts
//! ```
//!
//! Die eine Frage dieses Moduls: **wie kommen Eintraege in einem Finder-Fenster
//! ausgewaehlt vor den Nutzer.** Ein eigenes Modul und kein Zusatz zu
//! [`super::terminal`], obwohl beide beim Finder ankommen: jenes beantwortet
//! nach seinem eigenen Kopf, wie ein **Ordner** an eine ueber ihre
//! Buendelkennung **benannte** Anwendung kommt, und deckt dabei nichts auf.
//! Hier wird nichts benannt und nichts geoeffnet; das System weiss selbst,
//! welcher Dateibetrachter zustaendig ist. Der Zuschnitt ist der von
//! [`super::standardprogramm`], [`super::volumes`] und [`super::papierkorb`]:
//! ein Modul je Frage, eine sichere Huelle je Aufruf, und was die Huelle
//! verlaesst, ist ein gewoehnlicher Rust-Wert. Hier verlaesst sie gar nichts;
//! warum, steht im naechsten Abschnitt.
//!
//! # Der Aufruf gibt nichts zurueck, und das faerbt auf den Aufrufer ab
//!
//! `activateFileViewerSelectingURLs:` liefert `void`. Ob der Finder die
//! Eintraege danach zeigt, steht damit nicht fest, und dieses Programm kann es
//! nicht feststellen: es gibt weder einen Rueckgabewert wie bei
//! [`super::standardprogramm::oeffnen`] noch einen Rueckruf wie bei
//! `openURLs:…`, den [`super::terminal`] aus seinen eigenen Gruenden ohnehin
//! nicht fuehrt. **Was der Aufrufer meldet, ist deshalb keine
//! Erfolgsmeldung**, sondern das, was vor dem Aufruf entscheidbar ist: eine
//! leere Menge betroffener Eintraege
//! ([`crate::kommandos::operationen::nichts_anzuzeigen`]) und ein System, das
//! keinen Finder nennt ([`crate::kommandos::operationen::kein_finder`], gefragt
//! ueber [`super::terminal::anwendung_vorhanden`]).
//!
//! **Ein Pfad, den es nicht mehr gibt, bleibt danach unbemerkt.** Der Finder
//! kommt in den Vordergrund und waehlt ihn nicht aus; eine Meldung dazu gibt es
//! nicht. Ob dieser Weg eine Vorpruefung nach dem Vorbild von
//! [`crate::kommandos::operationen::ordner_fehlt`] bekommt, ist die offene
//! Frage
//! `shared/decisions/260907-0726_*_meldet-im-finder-anzeigen-einen-eintrag-der-nicht-mehr-dasteht.md`.
//!
//! # Diese Huelle traegt keine Probe, und das ist Absicht
//!
//! Ein Aufruf holt den Finder in den Vordergrund. Eine Probe, die ihn
//! ausloeste, riss bei jedem `make check` das Fenster einer fremden Anwendung
//! nach vorn; das ist derselbe Grund, aus dem
//! [`super::standardprogramm::oeffnen`] und
//! [`super::zwischenablage::text_schreiben`] keine tragen. Geprueft wird
//! stattdessen, was ohne AppKit pruefbar ist: die Menge der betroffenen
//! Eintraege in [`crate::kommandos::operationen::betroffene`] und die Saetze
//! des Aufrufers.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWorkspace`, `NSArray`, `NSString` und `NSURL` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `sharedWorkspace` und `fileURLWithPath:`.
//! **`activateFileViewerSelectingURLs:` ist die juengste Beruehrung dieser
//! Datei und steht seit macOS 10.6** (`NSWorkspace.h`,
//! `API_AVAILABLE(macos(10.6))` an der Methode). Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::path::PathBuf;

use objc2_app_kit::NSWorkspace;
use objc2_foundation::{NSArray, NSString, NSURL};

/// Deckt die genannten Eintraege in einem Finder-Fenster auf (260907).
///
/// Jeder Pfad wird im Finder **ausgewaehlt**, nicht geoeffnet; liegen die
/// Eintraege in verschiedenen Ordnern, oeffnet der Finder je ein Fenster.
///
/// **Vorher geprueft wird nichts.** `fileURLWithPath:` fragt das Dateisystem
/// nicht, und diese Huelle fragt es auch nicht — dieselbe Haltung wie in
/// [`super::standardprogramm::oeffnen`], hier allerdings ohne dessen Antwort:
/// was der Aufruf vorfindet, bleibt unbeantwortet. Der Modulkopf schreibt aus,
/// was der Aufrufer stattdessen prueft und was offenbleibt.
///
/// **Kein Rueckgabewert und damit kein `#[must_use]`.**
/// `activateFileViewerSelectingURLs:` liefert `void`; es gibt nichts, dessen
/// stilles Fallenlassen unbemerkt bliebe.
///
/// Eine leere Liste erreicht diese Funktion nicht: der Aufrufer faengt sie
/// vorher mit [`crate::kommandos::operationen::nichts_anzuzeigen`] ab. Kaeme
/// sie doch an, holte `activateFileViewerSelectingURLs:` den Finder in den
/// Vordergrund und waehlte nichts aus.
pub fn aufdecken(pfade: &[PathBuf]) {
    let ziele: Vec<_> = pfade
        .iter()
        .map(|pfad| NSURL::fileURLWithPath(&NSString::from_str(&pfad.to_string_lossy())))
        .collect();
    NSWorkspace::sharedWorkspace()
        .activateFileViewerSelectingURLs(&NSArray::from_retained_slice(&ziele));
}
