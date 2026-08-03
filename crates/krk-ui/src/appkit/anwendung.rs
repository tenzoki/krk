//! Der Start: `NSApplication`, der Anwendungsdelegierte, das erste Fenster.
//!
//! KRK laeuft als gewoehnliche Anwendung im Vordergrund
//! (`NSApplicationActivationPolicy::Regular`), auch wenn `cargo run` sie ohne
//! Buendel startet. Fuer die Abnahme zaehlt trotzdem allein der Start ueber
//! `target/KRK.app`: nur ein signiertes Buendel loest die Rueckfragen von TCC
//! aus, und ein nacktes Binaerprogramm erbt stattdessen die Freigaben des
//! Terminals.

use std::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, Message, define_class, msg_send};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate, NSWindow,
};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

use super::fenster::{self, FensterDelegierter};
use super::menue;
use super::tabelle::Dateifenster;

/// Was der Anwendungsdelegierte haelt.
///
/// Alle drei Felder tragen Objekte, die AppKit nur schwach referenziert oder
/// gar nicht kennt. Faellt eines von ihnen, faellt das Fenster mit.
#[derive(Default)]
pub struct AnwendungsIvars {
    fenster: OnceCell<Retained<NSWindow>>,
    fenster_delegierter: OnceCell<Retained<FensterDelegierter>>,
    dateifenster: OnceCell<Dateifenster>,
}

define_class!(
    /// Der Anwendungsdelegierte.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = AnwendungsIvars]
    pub struct Anwendungsdelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Anwendungsdelegierter {}

    // SAFETY: `NSApplicationDelegate` stellt keine Bedingungen.
    unsafe impl NSApplicationDelegate for Anwendungsdelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn start_abgeschlossen(&self, _meldung: &NSNotification) {
            self.oberflaeche_aufbauen();
        }
    }
);

impl Anwendungsdelegierter {
    /// Einen Anwendungsdelegierten ohne Oberflaeche.
    fn neu(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(AnwendungsIvars::default());
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }

    /// Baut Fenster und Dateifenster und liest das Benutzerverzeichnis.
    fn oberflaeche_aufbauen(&self) {
        let mtm = self.mtm();

        let dateifenster = Dateifenster::bauen(mtm);
        let fenster_delegierter = FensterDelegierter::neu(mtm, dateifenster.quelle().retain());
        let fenster = fenster::hauptfenster(mtm, dateifenster.sicht(), &fenster_delegierter);

        // Erst festhalten, dann anzeigen: das Fenster haelt seinen Delegierten
        // schwach, die Tabelle haelt Datenquelle und Delegierten schwach.
        let ivars = self.ivars();
        let _ = ivars.dateifenster.set(dateifenster);
        let _ = ivars.fenster_delegierter.set(fenster_delegierter);
        let _ = ivars.fenster.set(fenster);

        if let Some(dateifenster) = ivars.dateifenster.get() {
            dateifenster.quelle().ordner_lesen(&heimatverzeichnis());
        }
        if let Some(fenster) = ivars.fenster.get() {
            fenster.makeKeyAndOrderFront(None);
        }
    }
}

/// Startet die Anwendung. Kehrt zurueck, wenn sie beendet ist.
pub fn starten() {
    let mtm = MainThreadMarker::new()
        .expect("die Oberflaeche von KRK laeuft ausschliesslich auf dem Hauptfaden");

    let anwendung = NSApplication::sharedApplication(mtm);
    anwendung.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    anwendung.setMainMenu(Some(&menue::hauptmenue(mtm)));

    // Der Delegierte bleibt bis zum Ende von `starten` am Leben, weil
    // `NSApplication` ihn nur schwach haelt.
    let delegierter = Anwendungsdelegierter::neu(mtm);
    anwendung.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));

    anwendung.run();
}

/// Der Ordner, den KRK beim Start zeigt.
///
/// `$HOME` ist auf einem Mac gesetzt, solange die Anwendung fuer einen
/// angemeldeten Nutzer laeuft. Fehlt die Variable trotzdem, ist das
/// Wurzelverzeichnis der einzige Ordner, den es mit Sicherheit gibt.
fn heimatverzeichnis() -> std::path::PathBuf {
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/"))
}
