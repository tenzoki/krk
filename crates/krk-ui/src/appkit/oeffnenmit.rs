//! Die eine Beruehrung mit der Anwendungsliste des Systems, die „Öffnen mit"
//! braucht (260918).
//!
//! ```text
//!  ein Eintrag ──> NSURL ──> URLsForApplicationsToOpenURL: ──> Anwendung, Anwendung, …
//!                                                                  │
//!                                          displayNameAtPath: ─────┘
//!
//!  alle Eintraege ──> NSArray<NSURL> ──┐
//!  die gewaehlte Anwendung ──> NSURL ──┴─> openURLs:withApplicationAtURL:… ──> LaunchServices
//! ```
//!
//! Die eine Frage dieses Moduls: **welche Anwendungen nennt das System fuer
//! einen Eintrag, und wie kommen Eintraege an eine davon.** Ein eigenes Modul
//! und kein Zusatz zu [`super::standardprogramm`], obwohl beide bei
//! LaunchServices ankommen: jenes fragt nach dem **einen** Programm, das das
//! System fuehrt, und nennt dabei keines; hier wird die ganze Liste erfragt und
//! eine daraus **benannt**. Sein Kopf schreibt den Unterschied seit der Runde 4
//! aus — „fuenf markierte Dateien koennen zu fuenf verschiedenen Programmen
//! gehoeren, und eine Sammeluebergabe an ein einzelnes Programm waere genau das
//! ‚Oeffnen mit', das C3 ausschliesst". Genau das steht jetzt hier, und deshalb
//! nimmt [`oeffnen_mit`] die Mehrzahl, wo [`super::standardprogramm::oeffnen`]
//! die Einzahl nimmt.
//!
//! Der Zuschnitt ist der von [`super::standardprogramm`], [`super::finder`],
//! [`super::volumes`] und [`super::papierkorb`]: ein Modul je Frage, eine
//! sichere Huelle je Aufruf, und was die Huelle verlaesst, ist ein gewoehnlicher
//! Rust-Wert. Hier sind es [`Anwendung`] und ein [`bool`]; ein `NSURL` kommt aus
//! dieser Datei nicht heraus.
//!
//! # Was `true` heisst, und was es nicht heisst
//!
//! `openURLs:withApplicationAtURL:configuration:completionHandler:` liefert
//! **nichts**. Ob die Anwendung startet und die Eintraege zeigt, erfuehre KRK
//! allein ueber den Rueckruf am Ende der Signatur, und der kaeme auf einer
//! beliebigen Schlange an; dieses Vorhaben fuehrt keinen solchen Rueckruf, aus
//! den Gruenden, die [`super::terminal`] aufschreibt. Das `true` von
//! [`oeffnen_mit`] sagt deshalb weniger als das von
//! [`super::standardprogramm::oeffnen`], das immerhin die Annahme durch das
//! System meldet: es sagt, dass die Uebergabe **stattgefunden** hat, und nicht,
//! dass das System sie angenommen hat. Der Aufrufer meldet entsprechend die
//! Uebergabe ([`crate::kommandos::operationen::oeffnungsmeldung_an`]) und
//! behauptet kein Oeffnen.
//!
//! `false` hat damit genau einen Grund, und er liegt **vor** dem Aufruf: ein
//! Pfad ohne gueltiges UTF-8. Die Antwort ist dieselbe, die
//! [`super::standardprogramm`], [`super::teilen`], [`super::papierkorb`],
//! [`super::abwurf`] und [`super::volumes`] geben, und aus demselben Grund:
//! `to_string_lossy` baute mit dem Ersatzzeichen einen **anderen** Pfad, und
//! die Anwendung bekaeme eine Datei, die es nicht gibt.
//!
//! **Abgewiesen wird die ganze Menge und nicht der einzelne Eintrag.** Eine
//! Uebergabe, die stillschweigend weniger Dateien mitnimmt, als der Nutzer
//! markiert hat, ist der stille Fehlschlag, den dieses Vorhaben ueberall
//! ausschliesst; [`super::teilen::anbieten`] entscheidet dieselbe Frage
//! ebenso.
//!
//! # Die Liste kommt geordnet herein, und diese Datei ordnet sie nicht um
//!
//! Der Kopf des Systems sagt es zu: „The system sorts the resulting array based
//! on each application's suitability to open the given URL. The first
//! application is considered the best available match."
//! (`NSWorkspace.h:122-123`). Das Untermenue zeigt damit dieselbe Reihenfolge
//! wie der Finder, und das Standardprogramm steht oben. Eine eigene Sortierung
//! daneben — etwa nach dem Namen — waere eine zweite Meinung ueber eine Frage,
//! die das System bereits beantwortet hat.
//!
//! Doppelte Namen bleiben stehen: zwei Fassungen derselben Anwendung heissen
//! gleich und sind zwei Buendel. Sie zusammenzufassen hiesse zu entscheiden,
//! welche der beiden der Nutzer meint, und das entscheidet er selbst.
//!
//! # Diese beiden Huellen tragen keine Probe, die etwas oeffnet, und das ist Absicht
//!
//! Ein Aufruf von [`oeffnen_mit`] startet ein Programm des angemeldeten
//! Nutzers. Eine Probe, die ihn ausloeste, oeffnete bei jedem `make check`
//! Fenster, die niemand bestellt hat; das ist derselbe Grund, aus dem
//! [`super::standardprogramm::oeffnen`], [`super::finder::aufdecken`] und
//! [`super::zwischenablage::text_schreiben`] keine tragen. Geprueft wird
//! stattdessen, was ohne AppKit pruefbar ist: der Bezugseintrag in
//! [`crate::kommandos::kontextmenue::oeffnungsbezug`], der Rundweg ueber die
//! Marke daneben und die Saetze in
//! [`crate::kommandos::operationen`]. Dass die Liste aufgeht und die richtigen
//! Anwendungen nennt, sieht der Nutzer am gebauten Buendel.
//!
//! **Die eine Probe, die hier steht, oeffnet nichts.** Sie reicht einen Pfad
//! ohne gueltiges UTF-8 herein, und der wird abgewiesen, bevor `NSWorkspace`
//! ueberhaupt gefragt wird — dieselbe Bauform wie in
//! [`super::standardprogramm`]. [`anwendungen_fuer`] bleibt ungeprueft: seine
//! Antwort haengt daran, welche Programme auf dem Geraet stehen, und eine Probe
//! darueber pruefte das Geraet und nicht den Baum.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSWorkspace` (`NSWorkspace.h:28`), `NSFileManager`
//! (`NSFileManager.h:98`), `NSURL` (`NSURL.h:22`), `NSString` und `NSArray`
//! (`NSArray.h:17`) stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `sharedWorkspace` (`NSWorkspace.h:31`), `defaultManager`
//! (`NSFileManager.h:100`), `displayNameAtPath:` (`NSFileManager.h:254`),
//! `fileURLWithPath:` (`NSURL.h:52`) und `NSURL`s Eigenschaft `path`
//! (`NSURL.h:124`); keine dieser Deklarationen traegt im Kopf des Systems ein
//! `API_AVAILABLE`.
//!
//! **Drei Beruehrungen sind juenger, und die juengste ist `12.0`:**
//!
//! - 10.15: `NSWorkspaceOpenConfiguration` (`NSWorkspace.h:150-151`), sein
//!   Erzeuger `configuration` (`:153`) und
//!   `openURLs:withApplicationAtURL:configuration:completionHandler:` (`:43`),
//!   jede mit `API_AVAILABLE(macos(10.15))`.
//! - 12.0: `URLsForApplicationsToOpenURL:` (`:123`),
//!   `API_AVAILABLE(macos(12.0))`. **Die hoechste Untergrenze dieser Datei**,
//!   und damit drei Hauptfassungen unter dem Zielsystem.
//!
//! Die Zeilenangaben sind am 260918 in
//! `$(xcrun --show-sdk-path)/System/Library/Frameworks/` nachgelesen und nicht
//! uebernommen. Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine der
//! genannten ist nach macOS 15 hinzugekommen, und keine Beruehrung in dieser
//! Datei braucht deshalb eine Verfuegbarkeitspruefung zur Laufzeit. `objc2`
//! fuehrt keine Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die
//! Untergrenze nicht; die Nennung hier ist die Gegenmassnahme.
//!
//! **Was die `use`-Zeilen daneben hereinholen, und warum keines davon die
//! Untergrenze dieser Datei anhebt:** `Retained` ist ein Rust-Typ der Kiste und
//! hat kein macOS-Alter.

use std::path::{Path, PathBuf};

use objc2::rc::Retained;
use objc2_app_kit::{NSWorkspace, NSWorkspaceOpenConfiguration};
use objc2_foundation::{NSArray, NSFileManager, NSString, NSURL};

use crate::kommandos::kontextmenue::Anwendung;

/// Die Anwendungen, die das System fuer diesen Eintrag nennt (260918).
///
/// Leer heisst: das System nennt keine. Der Kopf des Systems fuehrt zwei Wege
/// dorthin, und beide enden hier gleich — „This returns the empty array if no
/// app can open it, or if the file does not exist." Ein Eintrag, den es nicht
/// mehr gibt, bekommt damit dieselbe Antwort wie einer, fuer den keine
/// Anwendung angemeldet ist, und der Aufrufer meldet in beiden Faellen
/// dasselbe: das System nennt keine. Eine Trennung waere eine Vermutung mit
/// zwei Texten, wie sie [`super::standardprogramm`] fuer sein `false` seit der
/// Runde 4 ablehnt.
///
/// **Vorher geprueft wird nichts.** `fileURLWithPath:` fragt das Dateisystem
/// nicht, und diese Huelle fragt es auch nicht; ob der Eintrag noch dasteht,
/// beantwortet die Liste selbst, indem sie leer bleibt.
///
/// **Ein Pfad ohne gueltiges UTF-8 liefert die leere Liste**, statt mit dem
/// Ersatzzeichen einen anderen Eintrag zu erfragen. Der Modulkopf sagt, warum.
///
/// **Eine Anwendung ohne lesbaren Pfad wird uebergangen und nicht geraten**,
/// wie in [`super::volumes::eingehaengte`]: aus einem `NSURL`, dessen `path`
/// `nil` ist, laesst sich kein Programmbuendel machen.
#[must_use]
pub fn anwendungen_fuer(pfad: &Path) -> Vec<Anwendung> {
    let Some(pfad) = pfad.to_str() else {
        return Vec::new();
    };
    let eintrag = NSURL::fileURLWithPath(&NSString::from_str(pfad));
    let verwalter = NSFileManager::defaultManager();
    NSWorkspace::sharedWorkspace()
        .URLsForApplicationsToOpenURL(&eintrag)
        .iter()
        .filter_map(|anwendung| {
            let pfad = anwendung.path()?;
            let name = verwalter.displayNameAtPath(&pfad).to_string();
            Some(Anwendung {
                name,
                pfad: PathBuf::from(pfad.to_string()),
            })
        })
        .collect()
}

/// Uebergibt die genannten Eintraege an die genannte Anwendung (260918).
///
/// Liefert, ob die Uebergabe stattgefunden hat — **nicht**, ob das System sie
/// angenommen hat und erst recht nicht, ob die Anwendung die Eintraege zeigt.
/// Der Unterschied steht im Modulkopf, und die Meldung des Aufrufers haelt ihn
/// ein.
///
/// **Ein Aufruf fuer die ganze Menge, und keiner je Eintrag.** Das ist die
/// Zusage des Auftrags fuer die Mehrfachauswahl: alle markierten Eintraege
/// gehen an **dieselbe** gewaehlte Anwendung, wie im Finder. Ein Aufruf je
/// Eintrag koennte dieselbe Anwendung mehrfach starten lassen und liesse die
/// Reihenfolge offen, in der sie ankommen.
///
/// **Der Typ eines Eintrags wird nicht geprueft und die Menge nicht
/// beschraenkt.** Ordner gehen mit; was eine Anwendung mit einem Ordner tut,
/// entscheidet sie und nicht KRK. Dieselbe Zurueckhaltung wie bei
/// [`super::standardprogramm::oeffnen`] und [`super::teilen::anbieten`].
///
/// **Die Vorgabe-Zusammenstellung geht mit, und kein Wert daran wird
/// gesetzt.** `NSWorkspaceOpenConfiguration::configuration` liefert sie mit den
/// Vorgaben des Systems, und die sind genau das, was ein Nutzer vom Oeffnen
/// erwartet: die Anwendung kommt nach vorn, die Eintraege landen unter „Zuletzt
/// benutzt", und eine noetige Rueckfrage stellt das System selbst. Jeder
/// gesetzte Wert waere eine Abweichung vom Finder, und keine ist verlangt.
///
/// **Der Rueckruf am Ende bleibt leer.** Er ist die einzige Stelle, an der ein
/// Fehlschlag ankaeme; er kaeme auf einer beliebigen Schlange an, und dieses
/// Vorhaben fuehrt keinen solchen Rueckruf (siehe [`super::terminal`]). Der
/// Aufrufer meldet deshalb die Uebergabe und nicht das Oeffnen.
///
/// Eine leere Liste erreicht diese Funktion nicht: der Aufrufer faengt sie
/// vorher ab, denn ohne betroffene Eintraege traegt das Untermenue keine
/// Anwendung.
#[must_use = "die Antwort sagt, ob die Uebergabe stattgefunden hat; fallengelassen bleibt der Nutzer ohne Meldung vor einem Programm, das nicht aufgeht"]
pub fn oeffnen_mit(pfade: &[PathBuf], anwendung: &Path) -> bool {
    // Erst die Anwendung: ohne sie gibt es keine Uebergabe, und die Liste
    // darunter braucht dann gar nicht erst zu entstehen.
    let Some(anwendung) = anwendung.to_str() else {
        return false;
    };
    // **Kein `to_string_lossy`, und die ganze Menge faellt oder keine.** Die
    // Begruendung steht im Modulkopf.
    let Some(ziele) = pfade
        .iter()
        .map(|pfad| Some(NSURL::fileURLWithPath(&NSString::from_str(pfad.to_str()?))))
        .collect::<Option<Vec<Retained<NSURL>>>>()
    else {
        return false;
    };
    let programm = NSURL::fileURLWithPath(&NSString::from_str(anwendung));
    NSWorkspace::sharedWorkspace().openURLs_withApplicationAtURL_configuration_completionHandler(
        &NSArray::from_retained_slice(&ziele),
        &programm,
        &NSWorkspaceOpenConfiguration::configuration(),
        None,
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ein Pfad ohne gueltiges UTF-8 geht nicht an das System, und keiner der
    /// beiden Eingaenge laesst ihn durch.
    ///
    /// **Dieselbe Bauform wie in [`super::super::standardprogramm`]**, und aus
    /// demselben Grund: `to_string_lossy` machte daraus einen anderen Eintrag,
    /// und die Anwendung bekaeme eine Datei, die es nicht gibt. Das Byte `0xff`
    /// ist in keiner UTF-8-Folge zulaessig.
    ///
    /// **Die Probe fasst AppKit nicht an**: beide Abweisungen stehen vor dem
    /// `NSWorkspace`. Ein gueltiger Pfad in derselben Probe wuerde ein fremdes
    /// Programm oeffnen und gehoert deshalb nicht hierher.
    ///
    /// Geprueft werden drei Lagen, weil die Menge und die Anwendung zwei
    /// Eingaenge sind: ein krummer Eintrag neben einem geraden, ein krummer
    /// Anwendungspfad, und beides zusammen.
    #[test]
    fn ein_pfad_ohne_gueltiges_utf8_geht_nicht_an_das_system() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let krumm = PathBuf::from(OsStr::from_bytes(b"/tmp/krk-oeffnenmit-\xffkrumm"));
        let gerade = PathBuf::from("/tmp/krk-oeffnenmit-gerade.txt");
        let programm = PathBuf::from("/Applications/TextEdit.app");
        assert!(
            krumm.to_str().is_none(),
            "der Pfad der Probe ist gueltiges UTF-8 und misst damit nicht, was sie messen soll"
        );
        assert!(
            !oeffnen_mit(&[gerade.clone(), krumm.clone()], &programm),
            "eine Menge mit einem krummen Pfad wird nicht abgewiesen"
        );
        assert!(
            !oeffnen_mit(&[gerade], &krumm),
            "eine krumme Anwendung wird nicht abgewiesen"
        );
        assert!(
            !oeffnen_mit(std::slice::from_ref(&krumm), &krumm),
            "zwei krumme Pfade werden nicht abgewiesen"
        );
    }

    /// Eine Anwendung fuer einen Pfad ohne gueltiges UTF-8 wird nicht erfragt.
    ///
    /// Die Gegenprobe zur Abweisung darueber, am anderen Eingang. Sie fasst
    /// AppKit ebenfalls nicht an, denn die Abweisung steht vor dem `NSURL`.
    #[test]
    fn ein_krummer_pfad_bekommt_keine_anwendungsliste() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let krumm = PathBuf::from(OsStr::from_bytes(b"/tmp/krk-oeffnenmit-\xffkrumm"));
        assert!(
            anwendungen_fuer(&krumm).is_empty(),
            "fuer einen krummen Pfad kommt eine Anwendungsliste zurueck"
        );
    }
}
