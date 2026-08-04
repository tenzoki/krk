//! Das Fenster und sein Delegierter.
//!
//! Die Inhaltsansicht des Fensters ist seit Schritt 12 die Aufteilung aus
//! [`super::aufteilung`] mit ihren vier Bereichen und nicht mehr die eine
//! Tabelle aus Schritt 6.
//!
//! Der Delegierte hat eine Aufgabe, und sie ist nicht kosmetisch: er bricht die
//! laufenden Lesevorgaenge **beider** Dateifenster ab, sobald das Fenster
//! schliesst. Ohne ihn liesse ein Ordner mit 100.000 Eintraegen seinen
//! Arbeitsfaden und seinen Zeitgeber gegen eine Tabelle weiterlaufen, die
//! niemand mehr sieht.
//!
//! **Das Fenster ueberlebt sein Schliessen.** `setReleasedWhenClosed(false)`
//! sorgt dafuer, und der Anwendungsdelegierte haelt es weiter. Genau darauf
//! baut der Rueckweg aus C7: "Fenster einblenden" auf Cmd+N und der Klick auf
//! das Dock-Symbol holen dieses eine Fenster nach vorn, statt ein zweites
//! anzulegen.

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
///
/// Breiter als in Schritt 6: vier Bereiche nebeneinander brauchen mehr Platz
/// als eine Dateiliste.
const ANFANGSGROESSE: NSSize = NSSize::new(1280.0, 720.0);

/// Die Groesse, unter die sich das Fenster nicht ziehen laesst.
///
/// Die Summe der vier Mindestbreiten aus [`crate::fenstermodell::Bereich`] plus
/// Luft fuer die Trennlinien. Darunter faenden die Bereiche keinen Platz mehr,
/// und die Zusage aus C7, dass jeder von ihnen bedienbar bleibt, waere nicht zu
/// halten.
const MINDESTGROESSE: NSSize = NSSize::new(780.0, 300.0);

/// Was der Fensterdelegierte haelt.
pub struct FensterIvars {
    /// Die Datenquellen der beiden Dateifenster, links zuerst.
    quellen: [Retained<DateifensterQuelle>; 2],
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
            for quelle in &self.ivars().quellen {
                quelle.lesen_abbrechen();
            }
        }
    }
);

impl FensterDelegierter {
    /// Einen Delegierten fuer das Fenster mit den genannten Dateifenstern.
    pub fn neu(
        mtm: MainThreadMarker,
        quellen: [Retained<DateifensterQuelle>; 2],
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(FensterIvars { quellen });
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
    // Referenz, die der Anwendungsdelegierte haelt, nach dem Schliessen tot,
    // und der Rueckweg aus C7 zeigte auf ein freigegebenes Objekt.
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
