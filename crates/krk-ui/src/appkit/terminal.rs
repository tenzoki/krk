//! Die eine Beruehrung mit dem System, die C11 braucht.
//!
//! ```text
//! Buendelkennung ──> URLForApplicationWithBundleIdentifier: ──> Ort der Anwendung
//!                                                                     │
//!             Ordner ──> openURLs:withApplicationAtURL:… <────────────┘
//! ```
//!
//! Ein eigenes Modul und kein Zusatz zu [`super::zwischenablage`]: jenes
//! beantwortet nach seinem eigenen Kopf die eine Frage "was steht in der
//! Zwischenablage, und wohin geht KRK damit", und ein Terminal-Aufruf, der die
//! Zwischenablage nicht anfasst, gaebe ihm eine zweite. Der Zuschnitt ist der
//! von [`super::volumes`] und [`super::papierkorb`]: ein Modul je Frage, eine
//! sichere Huelle je Aufruf, und was die Huelle verlaesst, ist ein
//! gewoehnlicher Rust-Wert. Hier ist es ein [`bool`]; ein `NSURL` kommt aus
//! dieser Datei nicht heraus.
//!
//! # Warum `NSWorkspace` und nicht `open -a`
//!
//! `open -a Terminal <pfad>` waere der Kommandozeilenweg, und er scheidet aus
//! drei Gruenden aus. Er loest ueber den **Namen** auf, also ueber den Weg, den
//! `objc2-app-kit` 0.3.2 mit `#[deprecated]` verwirft. Er meldet seinen Fehler
//! auf der Standardfehlerausgabe, die C1 als Kanal an den Nutzer ausschliesst,
//! also muesste KRK den Unterprozess abwarten und seine Ausgabe deuten. Und er
//! waere der erste Unterprozess dieses Vorhabens, mit den Fragen, wer ihn
//! abholt und was der Hauptfaden solange tut. `open(1)` ruft seinerseits
//! LaunchServices, also dasselbe, was [`NSWorkspace`] unmittelbar erreicht.
//!
//! # Der Rueckrufparameter bleibt leer
//!
//! `openURLs:withApplicationAtURL:configuration:completionHandler:` arbeitet
//! asynchron und meldet den Erfolg ueber einen Block auf einer beliebigen
//! Schlange. Die beiden Fehler, die der Nutzer beheben kann, stellt KRK vorher
//! und synchron fest: eine Kennung ohne installierte Anwendung an der
//! `None`-Antwort der Aufloesung, einen nicht mehr erreichbaren Ordner an der
//! Pruefung in [`crate::kommandos::operationen::terminalordner_fehlt`]. Was
//! danach noch scheitern kann, ist der Start eines aufgeloesten Buendels
//! selbst, und dafuer bleibt der Rueckruf leer: ein Block mit einem Sprung auf
//! den Hauptfaden waere der zweite asynchrone Weg neben dem Vermittlerfaden aus
//! Schritt 16, und der traegt dort einen laufenden Vorgang und nicht eine
//! einzelne Meldung.

use std::path::Path;

use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSArray, NSString, NSURL};

/// Oeffnet den Ordner in der Anwendung mit dieser Buendelkennung (C11).
///
/// Liefert `false`, wenn keine Anwendung dieser Kennung installiert ist; dann
/// ist nichts gerufen worden und der Aufrufer meldet die Kennung. **Auf die
/// Vorbelegung weicht diese Huelle nicht aus**: der Nutzer hat die Kennung
/// eingetragen, und eine stillschweigend andere Anwendung waere die Antwort,
/// die ihn seinen Tippfehler nicht finden laesst.
///
/// Die eine Stelle des Programms, die eine Buendelkennung in einen
/// Anwendungsort aufloest.
pub fn ordner_oeffnen(kennung: &str, ordner: &Path) -> bool {
    let arbeitsflaeche = NSWorkspace::sharedWorkspace();
    let Some(anwendung) =
        arbeitsflaeche.URLForApplicationWithBundleIdentifier(&NSString::from_str(kennung))
    else {
        return false;
    };

    let ziel = NSURL::fileURLWithPath(&NSString::from_str(&ordner.to_string_lossy()));
    let ziele = NSArray::from_retained_slice(&[ziel]);
    arbeitsflaeche.openURLs_withApplicationAtURL_configuration_completionHandler(
        &ziele,
        &anwendung,
        &NSWorkspaceOpenConfiguration::configuration(),
        None,
    );
    true
}
