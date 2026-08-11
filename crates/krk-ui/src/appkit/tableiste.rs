//! Die Tableiste am Kopf eines Dateifensters.
//!
//! Ein Tab je Abschnitt eines `NSSegmentedControl`, beschriftet mit dem Namen
//! des Ordners. Ein Klick waehlt den Tab; die vier Tastenbefehle aus C1 gehen
//! nicht hier durch, sondern ueber den Ereignisabgriff und das Tabmodell in
//! [`crate::tabs`]. Die Leiste zeigt an, was das Modell sagt, und trifft keine
//! eigene Entscheidung: [`Tableiste::setzen`] bekommt die Beschriftungen und
//! die sichtbare Stelle und schreibt beides hinein.
//!
//! **Warum ein `NSSegmentedControl` und keine eigene Ansicht.** Eine Reihe
//! gleichrangiger, sich gegenseitig ausschliessender Schalter ist genau das,
//! wofuer AppKit dieses Steuerelement fuehrt. Eine selbst gezeichnete Leiste
//! muesste Beschriftung, Auswahl, Anfassbarkeit und das Erscheinungsbild von
//! Hell und Dunkel nachbauen, und keine Zusage dieser Runde verlangt etwas, das
//! das Steuerelement nicht kann.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! `NSSegmentedControl`, seine Oberklassen `NSControl` und `NSView`, dazu
//! `NSObject` und `NSString` stehen seit macOS 10.0 zur Verfuegung, ebenso
//! `initWithFrame:`, `setSegmentCount:`, `setLabel:forSegment:`,
//! `setSelectedSegment:`, `selectedSegment`, `setTarget:`, `setAction:` und
//! `setAutoresizingMask:`; keine der vier gesetzten Aufzaehlungskonstanten
//! (`NSSegmentStyleAutomatic`, `NSSegmentSwitchTrackingSelectOne`,
//! `NSSegmentDistributionFill`, `NSControlSizeSmall`) traegt eine eigene Angabe.
//! Vier **Methoden** sind juenger als ihre Klasse: `setSegmentStyle:` seit 10.5,
//! `setControlSize:` seit 10.10, `setTrackingMode:` seit 10.10.3 und
//! `setSegmentDistribution:` seit 10.13. Das Buendel zielt auf 15.0
//! (`.cargo/config.toml`); keine von ihnen ist nach macOS 15 hinzugekommen, und
//! keine Beruehrung in dieser Datei braucht deshalb eine Verfuegbarkeitspruefung
//! zur Laufzeit. `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, und der
//! Uebersetzer haelt die Untergrenze nicht; die Nennung hier ist die
//! Gegenmassnahme.

use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSControlSize, NSSegmentDistribution, NSSegmentStyle,
    NSSegmentSwitchTracking, NSSegmentedControl, NSView,
};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize};
use objc2_foundation::{NSInteger, NSString};

/// Die Hoehe der Leiste in Punkten.
pub const HOEHE: f64 = 24.0;

/// Was das Rueckrufziel der Leiste haelt.
pub struct LeistenIvars {
    /// Die Senke, an die eine angeklickte Stelle geht.
    senke: Box<dyn Fn(usize)>,
}

define_class!(
    /// Das Ziel, das der Klick auf einen Abschnitt anspricht.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = LeistenIvars]
    struct Leistenziel;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Leistenziel {}

    impl Leistenziel {
        /// Der Nutzer hat einen Abschnitt angeklickt.
        // SAFETY: Die Signatur passt zu der, die `NSControl` aufruft: ein
        // Argument, der Absender.
        #[unsafe(method(tabGewaehlt:))]
        fn tab_gewaehlt(&self, absender: &NSSegmentedControl) {
            if let Ok(stelle) = usize::try_from(absender.selectedSegment()) {
                (self.ivars().senke)(stelle);
            }
        }
    }
);

impl Leistenziel {
    /// Ein Ziel, das jede Wahl an die genannte Senke reicht.
    fn neu(mtm: MainThreadMarker, senke: Box<dyn Fn(usize)>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(LeistenIvars { senke });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

/// Die Tableiste eines Dateifensters.
///
/// `NSControl` haelt sein Ziel nur schwach. Wer die Leiste baut, muss das Ziel
/// anderswo festhalten, sonst faellt es noch vor dem ersten Klick; hier ist
/// dieses Anderswo. Der Ring, den die Gegenrichtung aufspannen wuerde, entsteht
/// nicht: das Ziel haelt allein die Senke und nicht das Steuerelement.
pub struct Tableiste {
    steuerung: Retained<NSSegmentedControl>,
    _ziel: Retained<Leistenziel>,
}

impl Tableiste {
    /// Baut eine Leiste, die jede Wahl an die genannte Senke meldet.
    pub fn bauen(mtm: MainThreadMarker, senke: impl Fn(usize) + 'static) -> Self {
        let ziel = Leistenziel::neu(mtm, Box::new(senke));
        let steuerung = NSSegmentedControl::initWithFrame(
            NSSegmentedControl::alloc(mtm),
            NSRect::new(NSPoint::ZERO, NSSize::new(0.0, HOEHE)),
        );
        steuerung.setSegmentStyle(NSSegmentStyle::Automatic);
        steuerung.setTrackingMode(NSSegmentSwitchTracking::SelectOne);
        steuerung.setSegmentDistribution(NSSegmentDistribution::Fill);
        steuerung.setControlSize(NSControlSize::Small);
        // Am oberen Rand festgemacht, in der Breite mitwachsend.
        steuerung.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMinYMargin,
        );
        // SAFETY: `ziel` ist von der Klasse, die `tabGewaehlt:` mit der
        // erwarteten Signatur beantwortet, und `sel!` liefert einen gueltigen
        // Selektor. Ueber die Lebensdauer verlangt die Bindung nichts; getragen
        // wird sie davon, dass `NSControl.target` eine schwache Eigenschaft ist
        // ("This value is weak in apps built with ARC",
        // `objc2-app-kit-0.3.2/src/generated/NSControl.rs:91-99`) und dass
        // `Tableiste` das Ziel darum selbst festhaelt.
        unsafe {
            steuerung.setTarget(Some(&*ziel as &AnyObject));
            steuerung.setAction(Some(sel!(tabGewaehlt:)));
        }
        Self {
            steuerung,
            _ziel: ziel,
        }
    }

    /// Die Ansicht, die in das Dateifenster gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.steuerung
    }

    /// Setzt die Beschriftungen und die sichtbare Stelle.
    ///
    /// Gerufen nach jeder Aenderung am Tabmodell. Die Leiste haelt keinen
    /// eigenen Stand: sie wird aus dem Modell geschrieben und nie daraus
    /// gelesen.
    pub fn setzen(&self, titel: &[String], aktiv: usize) {
        self.steuerung.setSegmentCount(titel.len() as NSInteger);
        for (stelle, name) in titel.iter().enumerate() {
            self.steuerung
                .setLabel_forSegment(&NSString::from_str(name), stelle as NSInteger);
        }
        self.steuerung.setSelectedSegment(aktiv as NSInteger);
    }
}
