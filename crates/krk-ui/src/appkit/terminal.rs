//! Wie ein Pfad, ein Ordner oder ein Skript, an eine ueber ihre Buendelkennung
//! benannte Anwendung kommt.
//!
//! ```text
//! Buendelkennung ──> URLForApplicationWithBundleIdentifier: ──> Ort der Anwendung
//!                                                                     │
//!   Ordner oder Skript ──> openURLs:withApplicationAtURL:… <──────────┘
//! ```
//!
//! # Wer diese Frage stellt
//!
//! Das Modul heisst nach dem ersten Weg und beantwortet die Frage fuer alle. Der
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
//! **Seit dem 261029 stellt ein dritter Weg die Frage, und er reicht keinen
//! Ordner, sondern ein Skript.** „Im Terminal bearbeiten" (`opt+cmd+e`)
//! uebergibt der eingestellten Terminal-Anwendung eine `.command`-Datei, die
//! [`krk_core::terminalskript::vorbereiten`] angelegt hat und die `vim` oder
//! `sudo -e` an der Datei unter dem Cursor startet. Er geht durch
//! [`skript_uebergeben`], und beide oeffentlichen Huellen rufen denselben
//! privaten Rumpf [`oeffnen_mit`]: die Abweisung ohne UTF-8, die eine
//! Aufloesung und der eine Ruf an `NSWorkspace` stehen damit weiter genau
//! einmal da. [`ordner_oeffnen`] bleibt in Name und Vertrag, wie es war.
//!
//! # Der Faden `krk-terminal`
//!
//! Das Anlegen des Skripts fragt die Platte (`metadata`, `access(2)`, das
//! Schreiben), und die Datei kann auf einer eingehaengten Freigabe liegen,
//! deren Haengen sonst die Ereignisschleife anhielte. [`vorbereiten_lassen`]
//! stellt diese Fragen deshalb auf einem eigenen, benannten Faden und reicht
//! das Ergebnis ueber `DispatchQueue::main()` an den Hauptfaden zurueck —
//! dasselbe Muster wie [`super::auswurf`]. Auf dem Hauptfaden bleibt allein
//! die Uebergabe an `NSWorkspace` und, scheitert sie, das Verwerfen des
//! eigenen Skripts unter `$TMPDIR` (Entscheidung E4 des Plans
//! `261029-0920-plan-datei-im-terminal-mit-vim-und-sudo-e-oeffnen.md`).
//!
//! **Seit dem 261030 geht auch `ctrl+o` ueber diesen Faden**:
//! [`ordner_pruefen_lassen`] fragt dort, ob der angezeigte Ordner noch
//! dasteht, und erst die Antwort auf dem Hauptfaden oeffnet ihn ueber
//! [`ordner_oeffnen`]. Beide Wege teilen den einen Fadenweg
//! [`auf_dem_faden`]; einen zweiten Mechanismus daneben gibt es nicht.
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
//! fest: eine Kennung ohne installierte Anwendung an der `None`-Antwort der
//! Aufloesung, einen nicht mehr erreichbaren Ordner an der Pruefung in
//! [`crate::kommandos::operationen::ordner_fehlt`], fuer `ctrl+o` auf dem
//! Faden `krk-terminal` und fuer „Im Finder oeffnen" weiter auf dem
//! Hauptfaden. Was
//! danach noch scheitern kann, ist der Start eines aufgeloesten Buendels
//! selbst, und dafuer bleibt der Rueckruf leer: ein Block mit einem Sprung auf
//! den Hauptfaden waere der zweite asynchrone Weg neben dem Vermittlerfaden aus
//! Schritt 16, und der traegt dort einen laufenden Vorgang und nicht eine
//! einzelne Meldung.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWorkspace`, `NSArray`, `NSString` und `NSURL` stehen seit macOS 10.0 zur
//! Verfuegung, ebenso `sharedWorkspace`; `fileURLWithPath:isDirectory:` steht
//! seit macOS 10.5.
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
//!
//! **Was die `use`-Zeilen fuer den Faden `krk-terminal` hereinholen, und warum
//! keines davon die Untergrenze dieser Datei anhebt:** `MainThreadMarker` ist
//! ein Rust-Typ der Kiste und hat kein macOS-Alter; `DispatchQueue` aus
//! `dispatch2` ist eine Rust-Huelle um `libdispatch`, seit 10.6 im System.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use dispatch2::DispatchQueue;
use krk_core::terminalskript::{self, Skriptfehler, Vorbereitet};
use objc2::rc::Retained;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{MainThreadMarker, NSArray, NSString, NSURL};

use crate::kommandos::operationen;

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
/// eine Buendelkennung in einen Anwendungsort umsetzt; den Ruf an
/// `NSWorkspace` teilt sie mit [`skript_uebergeben`] ueber [`oeffnen_mit`].
#[must_use = "die Antwort sagt, ob eine Anwendung dieser Kennung installiert ist; fallengelassen bleibt der Nutzer ohne Meldung"]
pub fn ordner_oeffnen(kennung: &str, ordner: &Path) -> bool {
    oeffnen_mit(kennung, ordner, true)
}

/// Uebergibt ein Skript aus [`krk_core::terminalskript::vorbereiten`] der
/// Anwendung mit dieser Buendelkennung (261029, „Im Terminal bearbeiten").
///
/// Derselbe Vertrag wie [`ordner_oeffnen`]: `false` heisst, dass nichts
/// gerufen worden ist, weil der Pfad kein UTF-8 ist oder keine Anwendung
/// dieser Kennung installiert ist; der Aufrufer verwirft dann das Skript und
/// meldet die Kennung. `true` heisst allein, dass das System den Auftrag
/// angenommen hat. **Ob die Anwendung eine `.command`-Datei ausfuehrt, ist aus
/// KRKs Eingaben nicht entscheidbar** (Decidability (4) des Plans); das zeigt
/// das Terminal, und der Rueckruf bleibt aus dem Grund im Modulkopf leer.
#[must_use = "die Antwort sagt, ob das Skript uebergeben ist; fallengelassen bliebe das Skript liegen und der Nutzer ohne Meldung"]
pub fn skript_uebergeben(kennung: &str, skript: &Path) -> bool {
    oeffnen_mit(kennung, skript, false)
}

/// Der eine Ruf an `NSWorkspace`, den [`ordner_oeffnen`] und
/// [`skript_uebergeben`] teilen.
///
/// **`ist_ordner` sagt der Rufer, und gefragt wird die Platte dafuer nicht.**
/// `fileURLWithPath:` ohne diese Angabe prueft am Dateisystem, ob der Pfad ein
/// Ordner ist, und auf einer haengenden Freigabe hielte das den Hauptfaden an,
/// nachdem [`ordner_pruefen_lassen`] die Frage gerade von ihm weggenommen hat.
/// Der Ordner ist an dieser Stelle schon geprueft, das Skript ist eine Datei,
/// die KRK selbst angelegt hat.
fn oeffnen_mit(kennung: &str, pfad: &Path, ist_ordner: bool) -> bool {
    // **Kein `to_string_lossy`.** Ein Pfad ohne gueltiges UTF-8 wuerde damit zu
    // einem Pfad mit `U+FFFD`, und das Terminal oeffnete einen anderen Ordner,
    // ohne dass der Rufer es erfuehre. `false` heisst hier dasselbe wie bei
    // einer fehlenden Anwendung: der Rufer meldet es. Dieselbe Antwort geben
    // `super::papierkorb`, `super::abwurf` und `super::volumes`
    // (`issues/260826-1421_*_pfade-ohne-gueltiges-utf-8-vier-huellen-glaetten-still-mit-to-string-lossy-und-drei-weisen-ab.md`).
    let Some(pfad) = pfad.to_str() else {
        return false;
    };
    let Some(anwendung) = anwendungsort(kennung) else {
        return false;
    };

    let arbeitsflaeche = NSWorkspace::sharedWorkspace();
    let ziel = NSURL::fileURLWithPath_isDirectory(&NSString::from_str(pfad), ist_ordner);
    let ziele = NSArray::from_retained_slice(&[ziel]);
    arbeitsflaeche.openURLs_withApplicationAtURL_configuration_completionHandler(
        &ziele,
        &anwendung,
        &NSWorkspaceOpenConfiguration::configuration(),
        None,
    );
    true
}

/// Was [`vorbereiten_lassen`] an den Hauptfaden zurueckreicht.
pub type Vorbereitung = Result<Vorbereitet, Skriptfehler>;

/// Legt auf dem Faden `krk-terminal` das Skript fuer `datei` an und reicht das
/// Ergebnis ueber `DispatchQueue::main()` an `fertig` (261029).
///
/// **Auf dem rufenden Faden stellt sie keinen Systemaufruf**: das `metadata`,
/// die zwei Fragen an `access(2)` und das Schreiben stehen allein in
/// [`terminalskript::vorbereiten`], und das laeuft auf dem neuen Faden. Das
/// Muster ist das von [`super::auswurf::auswerfen`]: `fertig` bekommt den
/// [`MainThreadMarker`], damit der Rufer seinen Delegierten dort sucht, wo er
/// steht, statt ein `Retained` ueber die Fadengrenze zu reichen.
///
/// **Scheitert schon der Fadenstart, bekommt `fertig` das als
/// [`Skriptfehler::Schreiben`]**, ebenfalls ueber die Hauptschlange: es ist
/// kein Skript entstanden, und der Rufer meldet es wie jeden anderen Fehler
/// beim Anlegen. `fertig` wird in jedem Fall genau einmal gerufen, ausser die
/// Hauptschlange laeuft nie wieder.
pub fn vorbereiten_lassen(
    datei: PathBuf,
    fertig: impl FnOnce(MainThreadMarker, Vorbereitung) + Send + 'static,
) {
    auf_dem_faden(
        move || terminalskript::vorbereiten(&terminalskript::skriptverzeichnis(), &datei),
        |fehler| Err(Skriptfehler::Schreiben(fehler)),
        fertig,
    );
}

/// Fragt auf dem Faden `krk-terminal`, ob `ordner` noch ein erreichbarer
/// Ordner ist, und reicht die Antwort ueber `DispatchQueue::main()` an
/// `fertig` (C11, `ctrl+o`).
///
/// Die Antwort ist die von [`operationen::ordner_fehlt`]: `None`, wenn der
/// Ordner dasteht, sonst der Satz fuer die Statuszeile. **Auf dem rufenden
/// Faden stellt sie keinen Systemaufruf**, denn das `metadata` darin wartet
/// auf einer haengenden Freigabe auf das Zeitlimit des Dateisystems, und auf
/// dem Hauptfaden hielte es so lange die Ereignisschleife an
/// (`261029-0958-ordner-im-terminal-oeffnen-fragt-metadata-auf-dem-hauptfaden-und-ein-haengendes-netzlaufwerk-haelt-die-ereignisschleife-an.md`).
///
/// **Scheitert schon der Fadenstart, bekommt `fertig` den Satz fuer einen
/// unerreichbaren Ordner mit dem Fehler des Starts**, ebenfalls ueber die
/// Hauptschlange: gefragt ist dann nichts, und der Ordner zu oeffnen, ohne
/// gefragt zu haben, hiesse die Pruefung still auszulassen. `fertig` wird in
/// jedem Fall genau einmal gerufen, ausser die Hauptschlange laeuft nie
/// wieder.
pub fn ordner_pruefen_lassen(
    ordner: PathBuf,
    fertig: impl FnOnce(MainThreadMarker, Option<String>) + Send + 'static,
) {
    let fuer_den_startfehler = ordner.clone();
    auf_dem_faden(
        move || operationen::ordner_fehlt(&ordner),
        move |fehler| {
            Some(operationen::ordner_unerreichbar(
                &fuer_den_startfehler,
                &fehler,
            ))
        },
        fertig,
    );
}

/// Der eine Fadenweg dieses Moduls: `arbeit` laeuft auf dem Faden
/// `krk-terminal`, ihr Ergebnis geht ueber `DispatchQueue::main()` an
/// `fertig`.
///
/// Scheitert der Fadenstart, macht `startfehler` aus dem Fehler das Ergebnis,
/// und `fertig` bekommt es ebenso ueber die Hauptschlange. Beide oeffentlichen
/// Wege, [`vorbereiten_lassen`] und [`ordner_pruefen_lassen`], gehen hier
/// durch, damit Halter, Fadenname und Rueckweg genau einmal dastehen.
fn auf_dem_faden<T: Send + 'static>(
    arbeit: impl FnOnce() -> T + Send + 'static,
    startfehler: impl FnOnce(std::io::Error) -> T,
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
) {
    // `Builder::spawn` nimmt den Abschluss und gibt ihn bei einem Fehlschlag
    // nicht zurueck; der Halter laesst `fertig` deshalb auch dann erreichbar,
    // wenn kein Faden entsteht.
    let halter = Arc::new(Mutex::new(Some(fertig)));
    let auf_dem_faden = Arc::clone(&halter);
    let gestartet = std::thread::Builder::new()
        .name("krk-terminal".to_owned())
        .spawn(move || {
            let ergebnis = arbeit();
            if let Some(fertig) = nehmen(&auf_dem_faden) {
                auf_dem_hauptfaden(fertig, ergebnis);
            }
        });
    if let Err(fehler) = gestartet
        && let Some(fertig) = nehmen(&halter)
    {
        auf_dem_hauptfaden(fertig, startfehler(fehler));
    }
}

/// Nimmt `fertig` aus dem Halter, auch aus einem vergifteten: der Wert darin
/// ist trotzdem der richtige, und ihn liegen zu lassen hiesse, die Antwort zu
/// verschlucken (dieselbe Ueberlegung wie in [`super::auswurf`]).
fn nehmen<F>(halter: &Mutex<Option<F>>) -> Option<F> {
    match halter.lock() {
        Ok(mut fertig) => fertig.take(),
        Err(vergiftet) => vergiftet.into_inner().take(),
    }
}

/// Reicht das Ergebnis an den Hauptfaden weiter.
fn auf_dem_hauptfaden<T: Send + 'static>(
    fertig: impl FnOnce(MainThreadMarker, T) + Send + 'static,
    ergebnis: T,
) {
    DispatchQueue::main().exec_async(move || {
        let Some(mtm) = MainThreadMarker::new() else {
            // Kann nicht eintreten: die Hauptschlange laeuft auf dem
            // Hauptfaden. Ein angelegtes Skript bliebe hier liegen, mit `0700`
            // im privaten `$TMPDIR`, und das ist der Rest, den E2 hinnimmt.
            return;
        };
        fertig(mtm, ergebnis);
    });
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
