//! Wie ein Ordner an eine ueber ihre Buendelkennung benannte Anwendung kommt.
//!
//! ```text
//! Buendelkennung ──> URLForApplicationWithBundleIdentifier: ──> Ort der Anwendung
//!                                                                     │
//!             Ordner ──> openURLs:withApplicationAtURL:… <────────────┘
//! ```
//!
//! # Zwei Wege stellen diese Frage
//!
//! Das Modul heisst nach dem ersten und beantwortet die Frage fuer beide. Der
//! Terminal-Befehl aus C11 stellt sie mit der Kennung aus `settings.toml`, die
//! der Nutzer selbst eintraegt; der Eintrag „Im Finder oeffnen" des
//! Kontextmenues aus der Runde 17 stellt sie mit der festen Kennung des
//! Finders. **Beide gehen durch [`ordner_oeffnen`]**, und eine zweite Huelle
//! daneben waere derselbe Doppelbau, den der Zuschnitt dieses Moduls im
//! naechsten Absatz vermeidet. Was die zwei Wege unterscheidet, liegt
//! ausserhalb dieser Datei: woher die Kennung kommt, und welchen Satz der
//! Aufrufer meldet, wenn keine Anwendung dazu installiert ist
//! ([`crate::kommandos::operationen::kein_terminal`] nennt die eingestellte
//! Kennung, weil der Nutzer sie berichtigen kann,
//! [`crate::kommandos::operationen::kein_finder`] nennt keine).
//!
//! # Die eine Aufloesung, und ihr zweiter Frager
//!
//! [`anwendungsort`] ist die eine Stelle des Programms, die eine
//! Buendelkennung in einen Anwendungsort aufloest. Sie hat zwei Frager, und
//! der zweite braucht den Ort gar nicht: [`anwendung_vorhanden`] gibt allein
//! heraus, **ob** eine Anwendung dieser Kennung installiert ist. Gefragt wird
//! das vom Eintrag „Im Finder anzeigen" (260907), dessen eigener Aufruf in
//! [`super::finder`] steht und keine Kennung aufloest — er reicht dem
//! aufdeckenden `NSWorkspace` unmittelbar Dateiadressen und bekommt von ihm
//! nichts zurueck. Ohne den Frager hier haette jener Weg gar keine Antwort auf
//! die Frage, ob ein Finder ueberhaupt dasteht; mit einer zweiten Aufloesung
//! dort haette dieses Programm zwei.
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
//! Pruefung in [`crate::kommandos::operationen::ordner_fehlt`]. Was
//! danach noch scheitern kann, ist der Start eines aufgeloesten Buendels
//! selbst, und dafuer bleibt der Rueckruf leer: ein Block mit einem Sprung auf
//! den Hauptfaden waere der zweite asynchrone Weg neben dem Vermittlerfaden aus
//! Schritt 16, und der traegt dort einen laufenden Vorgang und nicht eine
//! einzelne Meldung.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWorkspace`, `NSArray`, `NSString` und `NSURL` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `sharedWorkspace` und `fileURLWithPath:`.
//! **`NSWorkspaceOpenConfiguration` ist die juengste Klasse dieser Datei und
//! steht seit macOS 10.15** (`NSWorkspace.h`, `API_AVAILABLE(macos(10.15))`
//! ueber dem `@interface`); ihre Bauform `configuration` traegt keine eigene
//! Angabe und steht damit ebenfalls ab 10.15. Zwei Methoden sind juenger als
//! `NSWorkspace`: `URLForApplicationWithBundleIdentifier:` seit 10.6 und
//! `openURLs:withApplicationAtURL:configuration:completionHandler:` seit 10.15
//! — letztere ist genau der Weg, der die im Abschnitt "Warum `NSWorkspace` und
//! nicht `open -a`" genannten, mit `#[deprecated]` verworfenen Namensformen
//! ersetzt. Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen
//! ist nach macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht
//! deshalb eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::path::Path;

use objc2::rc::Retained;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSArray, NSString, NSURL};

/// Wo die Anwendung mit dieser Buendelkennung liegt, sofern eine installiert
/// ist.
///
/// **Die eine Stelle des Programms, die eine Buendelkennung in einen
/// Anwendungsort aufloest.** Ihre zwei Frager stehen im Modulkopf; sie ist
/// privat, damit kein dritter an ihr vorbei eine eigene Aufloesung baut, und
/// ein `NSURL` verlaesst diese Datei so auch weiterhin nicht.
fn anwendungsort(kennung: &str) -> Option<Retained<NSURL>> {
    NSWorkspace::sharedWorkspace()
        .URLForApplicationWithBundleIdentifier(&NSString::from_str(kennung))
}

/// Ob eine Anwendung mit dieser Buendelkennung installiert ist.
///
/// **Die Antwort ist notwendig und nicht hinreichend.** `false` heisst, dass
/// das System keine Anwendung dieser Kennung nennt, und damit steht fest, dass
/// ein Aufruf an sie nichts bewirkte. `true` heisst allein, dass eine dasteht;
/// ob sie startet und tut, worum sie gebeten wird, sagt es nicht. Der eine
/// Aufrufer ([`crate::appkit::anwendung`], Zweig „Im Finder anzeigen") meldet
/// deshalb auf `false` und behauptet auf `true` nichts.
///
/// **Sie gibt einen [`bool`] heraus und keinen Ort**, obwohl ihr Rumpf einen
/// hat: der Aufrufer braucht ihn nicht, und ein `NSURL` aus dieser Datei
/// heraus waere die Grenzverletzung, gegen die der Modulkopf geschrieben ist.
#[must_use = "die Antwort sagt, ob eine Anwendung dieser Kennung installiert ist; fallengelassen bleibt der Nutzer ohne Meldung"]
pub fn anwendung_vorhanden(kennung: &str) -> bool {
    anwendungsort(kennung).is_some()
}

/// Oeffnet den Ordner in der Anwendung mit dieser Buendelkennung (C11).
///
/// Liefert `false`, wenn keine Anwendung dieser Kennung installiert ist; dann
/// ist nichts gerufen worden und der Aufrufer meldet die Kennung. **Auf die
/// Vorbelegung weicht diese Huelle nicht aus**: der Nutzer hat die Kennung
/// eingetragen, und eine stillschweigend andere Anwendung waere die Antwort,
/// die ihn seinen Tippfehler nicht finden laesst.
///
/// Aufgeloest wird ueber [`anwendungsort`], die eine Stelle des Programms, die
/// eine Buendelkennung in einen Anwendungsort umsetzt.
#[must_use = "die Antwort sagt, ob eine Anwendung dieser Kennung installiert ist; fallengelassen bleibt der Nutzer ohne Meldung"]
pub fn ordner_oeffnen(kennung: &str, ordner: &Path) -> bool {
    // **Kein `to_string_lossy`.** Ein Pfad ohne gueltiges UTF-8 wuerde damit zu
    // einem Pfad mit `U+FFFD`, und das Terminal oeffnete einen anderen Ordner,
    // ohne dass der Rufer es erfuehre. `false` heisst hier dasselbe wie bei
    // einer fehlenden Anwendung: der Rufer meldet es. Dieselbe Antwort geben
    // `super::papierkorb`, `super::abwurf` und `super::volumes`
    // (`issues/260826-1421_*_pfade-ohne-gueltiges-utf-8-vier-huellen-glaetten-still-mit-to-string-lossy-und-drei-weisen-ab.md`).
    let Some(ordner) = ordner.to_str() else {
        return false;
    };
    let Some(anwendung) = anwendungsort(kennung) else {
        return false;
    };

    let arbeitsflaeche = NSWorkspace::sharedWorkspace();
    let ziel = NSURL::fileURLWithPath(&NSString::from_str(ordner));
    let ziele = NSArray::from_retained_slice(&[ziel]);
    arbeitsflaeche.openURLs_withApplicationAtURL_configuration_completionHandler(
        &ziele,
        &anwendung,
        &NSWorkspaceOpenConfiguration::configuration(),
        None,
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ein Pfad ohne gueltiges UTF-8 wird abgewiesen und nicht geglaettet.
    ///
    /// **Dieselbe Bauform wie in [`super::papierkorb`]**, und aus demselben
    /// Grund: `to_string_lossy` machte daraus einen Pfad mit `U+FFFD`, also
    /// einen anderen Ordner, und das Terminal oeffnete ihn, ohne dass jemand es
    /// erfuehre. Das Byte `0xff` ist in keiner UTF-8-Folge zulaessig.
    ///
    /// **Die Probe fasst AppKit nicht an**: die Abweisung steht vor dem Auflegen
    /// der Anwendung und vor jedem Ruf an `NSWorkspace`. Deshalb steht der
    /// krumme Pfad in der Zeile und keine gueltige Kennung daneben.
    #[test]
    fn ein_pfad_ohne_gueltiges_utf8_oeffnet_kein_terminal() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        use std::path::PathBuf;

        let krumm = PathBuf::from(OsStr::from_bytes(b"/tmp/krk-terminal-\xffkrumm"));
        assert!(
            krumm.to_str().is_none(),
            "der Pfad der Probe ist gueltiges UTF-8 und misst damit nicht, was sie messen soll"
        );
        assert!(
            !ordner_oeffnen("com.apple.Terminal", &krumm),
            "ein Pfad ohne gueltiges UTF-8 wird nicht abgewiesen"
        );
    }
}
