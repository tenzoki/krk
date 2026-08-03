//! Das Fenster und sein Delegierter.
//!
//! Der Delegierte hat in Runde 1, Schritt 6 eine einzige Aufgabe, und sie ist
//! nicht kosmetisch: er bricht den laufenden Lesevorgang ab, sobald das Fenster
//! schliesst. Ohne ihn liesse ein Ordner mit 100.000 Eintraegen seinen
//! Arbeitsfaden und seinen Zeitgeber gegen eine Tabelle weiterlaufen, die
//! niemand mehr sieht.
//!
//! Die Groessenaenderung, die derselbe Delegierte spaeter traegt, kommt mit den
//! vier Bereichen aus Schritt 12. Bis dahin regelt die Autogroesse der
//! Bildlaufansicht sie vollstaendig.

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{NSBackingStoreType, NSView, NSWindow, NSWindowDelegate, NSWindowStyleMask};
use objc2_foundation::{
    MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize,
    ns_string,
};

use super::tabelle::DateifensterQuelle;

/// Die Groesse, mit der das Fenster beim ersten Start aufgeht.
const ANFANGSGROESSE: NSSize = NSSize::new(900.0, 600.0);

/// Die Groesse, unter die sich das Fenster nicht ziehen laesst.
///
/// Vier Spalten brauchen Platz; darunter waere die Namensspalte ein Schlitz.
const MINDESTGROESSE: NSSize = NSSize::new(520.0, 240.0);

/// Was der Fensterdelegierte haelt.
pub struct FensterIvars {
    /// Die Datenquelle des Dateifensters in diesem Fenster.
    quelle: Retained<DateifensterQuelle>,
}

define_class!(
    /// Der Delegierte des Hauptfensters.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = FensterIvars]
    pub struct FensterDelegierter;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for FensterDelegierter {}

    // SAFETY: `NSWindowDelegate` stellt keine Bedingungen.
    unsafe impl NSWindowDelegate for FensterDelegierter {
        // SAFETY: Die Signatur entspricht der des Protokolls.
        #[unsafe(method(windowWillClose:))]
        fn fenster_schliesst(&self, _meldung: &NSNotification) {
            self.ivars().quelle.lesen_abbrechen();
        }
    }
);

impl FensterDelegierter {
    /// Einen Delegierten fuer das Fenster, das die genannte Quelle anzeigt.
    pub fn neu(mtm: MainThreadMarker, quelle: Retained<DateifensterQuelle>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(FensterIvars { quelle });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

/// Baut das Hauptfenster um die genannte Ansicht.
pub fn hauptfenster(
    mtm: MainThreadMarker,
    inhalt: &NSView,
    delegierter: &FensterDelegierter,
) -> Retained<NSWindow> {
    // SAFETY: Das Fenster gibt sich beim Schliessen nicht selbst frei, siehe
    // `setReleasedWhenClosed` unten. Ohne diese Abschaltung waere die
    // Referenz, die der Anwendungsdelegierte haelt, nach dem Schliessen tot.
    let fenster = unsafe {
        let fenster = NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            NSRect::new(NSPoint::new(0.0, 0.0), ANFANGSGROESSE),
            NSWindowStyleMask::Titled
                | NSWindowStyleMask::Closable
                | NSWindowStyleMask::Miniaturizable
                | NSWindowStyleMask::Resizable,
            NSBackingStoreType::Buffered,
            false,
        );
        fenster.setReleasedWhenClosed(false);
        fenster
    };

    fenster.setTitle(ns_string!("KRK"));
    fenster.setContentMinSize(MINDESTGROESSE);
    fenster.setContentView(Some(inhalt));
    fenster.setDelegate(Some(ProtocolObject::from_ref(delegierter)));
    fenster.center();
    fenster
}
