//! Die eine Beruehrung mit dem System, die C3 der Runde 4 braucht.
//!
//! ```text
//! Pfad ──> NSURL::fileURLWithPath: ──> openURL: ──> LaunchServices
//!                                          │
//!                                          └──> bool: angenommen, ja oder nein
//! ```
//!
//! Die eine Frage dieses Moduls: **wie kommt ein Eintrag an das Programm, das
//! das System fuer ihn fuehrt.** Ein eigenes Modul und kein Zusatz zu den
//! beiden Nachbarn, weil keiner von ihnen diese Frage stellt.
//! [`super::zwischenablage`] beantwortet nach seinem eigenen Kopf, was in der
//! Zwischenablage steht und wohin KRK damit geht; ein Oeffnen, das die
//! Zwischenablage nicht anfasst, gaebe ihm eine zweite Frage.
//! [`super::terminal`] beantwortet, wie eine **benannte** Anwendung einen
//! Ordner bekommt, aufgeloest ueber eine Buendelkennung aus `settings.toml`;
//! ein Standardprogramm ist keine benannte, und wer es waere, weiss allein das
//! System. Der Zuschnitt ist damit derselbe wie bei [`super::volumes`] und
//! [`super::papierkorb`]: ein Modul je Frage, eine sichere Huelle je Aufruf,
//! und was die Huelle verlaesst, ist ein gewoehnlicher Rust-Wert. Hier ist es
//! ein [`bool`]; ein `NSURL` kommt aus dieser Datei nicht heraus.
//!
//! # Was `true` heisst, und was es nicht heisst
//!
//! `openURL:` liefert synchron, ob das System die Adresse **angenommen** hat.
//! Ob das aufgeloeste Programm danach startet und den Eintrag zeigt, steht
//! damit nicht fest, und KRK kann es nicht feststellen: die Antwort darauf
//! kaeme ueber einen Rueckruf auf einer beliebigen Schlange, den dieses Projekt
//! aus den Gruenden in [`super::terminal`] nicht fuehrt. **Der Aufrufer meldet
//! deshalb die Uebergabe und nicht das Oeffnen**; die Meldungstexte stehen in
//! [`crate::kommandos::operationen::oeffnungsmeldung`] und sagen "an das System
//! uebergeben", wo sie sonst "geoeffnet" sagen muessten, ohne es zu wissen.
//!
//! Aus demselben Grund trennt diese Runde den Fall "es gibt kein
//! Standardprogramm" nicht vom Fall "das System hat abgelehnt": beide kommen
//! als dasselbe `false` an, und `openURL:` nennt keinen Grund. Eine Trennung
//! waere eine Vermutung mit zwei Texten (Nutzerantwort vom 260811-1610 und
//! Frage 8 des Umsetzungsplans).
//!
//! # Ein Aufruf je Eintrag, und keine Sammeluebergabe
//!
//! [`oeffnen`] nimmt **einen** Pfad; die Mehrzahl gehoert dem Aufrufer, der
//! ohnehin zaehlen muss, was abgewiesen wurde. Der Grund ist nicht die
//! Bequemlichkeit der Signatur: fuenf markierte Dateien koennen zu fuenf
//! verschiedenen Programmen gehoeren, und eine Sammeluebergabe an ein einzelnes
//! Programm waere genau das "Oeffnen mit", das C3 ausschliesst.
//!
//! # Diese Huelle traegt keine Probe, und das ist Absicht
//!
//! Ein Aufruf startet ein Programm des angemeldeten Nutzers. Eine Probe, die
//! ihn ausloeste, oeffnete bei jedem `make check` Fenster, die niemand
//! bestellt hat; das ist derselbe Grund, aus dem
//! [`super::zwischenablage::text_schreiben`] keine traegt. Geprueft wird
//! stattdessen, was ohne AppKit pruefbar ist: die Menge der betroffenen
//! Eintraege in [`crate::kommandos::operationen::betroffene`] und die Meldungen
//! in [`crate::kommandos::operationen::oeffnungsmeldung`]. Dass `openURL:` den
//! Eintrag an LaunchServices gibt, sieht der Nutzer am gebauten Buendel.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWorkspace`, `NSURL` und `NSString` stehen seit macOS 10.0 zur Verfuegung,
//! ebenso `sharedWorkspace`, `fileURLWithPath:` und `openURL:`. Das Buendel
//! zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach macOS 15
//! hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb eine
//! Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::path::Path;

use objc2_app_kit::NSWorkspace;
use objc2_foundation::{NSString, NSURL};

/// Uebergibt einen Eintrag an das Standardprogramm des Systems (C3).
///
/// Liefert, ob das System die Adresse angenommen hat — **nicht**, ob ein
/// Programm sie danach zeigt. Der Unterschied steht im Modulkopf, und die
/// Meldung des Aufrufers haelt ihn ein.
///
/// Der Typ des Eintrags wird nicht geprueft. Die Taste verzweigt nach der
/// Nutzerantwort vom 260811-1505 ausdruecklich nicht, und ein Ordner geht damit
/// an das System, das ihn im Finder zeigt.
///
/// **Vorher geprueft wird nichts.** `fileURLWithPath:` fragt das Dateisystem
/// nicht, und diese Huelle fragt es auch nicht: ein Eintrag, den es nicht mehr
/// gibt, geht bis zum Aufruf durch, und was das System dazu sagt, meldet der
/// Aufrufer. Der Gegenentwurf waere eine Pruefung nach dem Vorbild von
/// [`crate::kommandos::operationen::ordner_fehlt`]; die steht dort, weil
/// jener Aufruf **keine** Antwort liefert, und hier liefert er eine.
pub fn oeffnen(pfad: &Path) -> bool {
    let ziel = NSURL::fileURLWithPath(&NSString::from_str(&pfad.to_string_lossy()));
    NSWorkspace::sharedWorkspace().openURL(&ziel)
}
