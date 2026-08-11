//! Der Bildtakt: die beiden Beruehrungen mit AppKit, die eine Messung braucht.
//!
//! Geschnitten wie die fuenf uebrigen Module dieses Verzeichnisses, naemlich
//! nach dem, was AppKit als eigenstaendiges Objekt fuehrt:
//!
//! - [`Zeichenende`] haelt den `CADisplayLink` auf einer Ansicht. Er nimmt beim
//!   Einrichten eine gewoehnliche Rust-Senke entgegen und meldet ihr jedes
//!   Bildende als [`Instant`]. Beim Fallenlassen gibt er den Takt wieder frei,
//!   dieselbe Form wie [`Tastenabgriff`](super::ereignisse::Tastenabgriff), der
//!   sich in seinem `Drop` bei AppKit abmeldet.
//! - [`bildwiederholrate`] schlaegt `maximumFramesPerSecond` auf dem Bildschirm
//!   nach, auf dem das genannte Fenster steht. Steht es auf keinem, liefert die
//!   Huelle `None`, damit der Aufrufer abbrechen kann, statt auf den
//!   Hauptbildschirm auszuweichen.
//!
//! **Ueber die Grenze gehen zwei gewoehnliche Rust-Werte**, die Rate als Zahl
//! und die Zeitpunkte der Zeichenenden. `crates/krk-ui/src/messmodus.rs` nennt
//! deshalb keine `objc2`-Kiste.
//!
//! # Was ein Zeichenende hier heisst, und was nicht
//!
//! Ein `CADisplayLink`-Rueckruf ist eine Bildgrenze: das System ruft ihn einmal
//! je Bildwiederholung. Er sagt **nicht**, dass ein bestimmtes Pixel auf dem
//! Schirm steht. Aus dem eigenen Prozess heraus ist das auch nicht feststellbar.
//! Die Messung, die darauf aufsetzt, ist deshalb die Spanne bis zur ersten
//! Bildgrenze, an der die Aenderung im Modell steht — die erreichbare Naeherung
//! an L1s Formulierung "bis die Auswahl sichtbar umspringt", und der Bericht
//! schreibt sie als solche aus.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! **Dieses Modul haelt die juengste Klasse des Verzeichnisses.**
//! `CADisplayLink` fuehrt QuartzCore auf dem Mac erst ab macOS 14
//! (`CADisplayLink.h`, `API_AVAILABLE(macos(14.0))` ueber dem `@interface`); auf
//! iOS steht sie seit 3.1, und diese Zahl gilt hier ausdruecklich nicht. Ihre
//! beiden angesprochenen Methoden `addToRunLoop:forMode:` und `invalidate`
//! tragen keine eigene Angabe und stehen damit ebenfalls ab 14.
//!
//! `NSView`, `NSWindow`, `NSScreen`, `NSRunLoop` und `NSObject` stehen seit
//! macOS 10.0 zur Verfuegung, ebenso `NSWindow.screen` und
//! `NSRunLoop.currentRunLoop`. Drei Beruehrungen sind juenger:
//! `NSView.displayLinkWithTarget:selector:` seit macOS 14 (die ganze Kategorie
//! `NSView (NSDisplayLink)` traegt die Angabe),
//! `NSScreen.maximumFramesPerSecond` seit 12.0 und das Fremdsymbol
//! `NSRunLoopCommonModes` seit 10.5.
//!
//! Das Buendel zielt auf 15.0 (`.cargo/config.toml`); keine von ihnen ist nach
//! macOS 15 hinzugekommen, und keine Beruehrung in dieser Datei braucht deshalb
//! eine Verfuegbarkeitspruefung zur Laufzeit. `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und der Uebersetzer haelt die Untergrenze
//! nicht; die Nennung hier ist die Gegenmassnahme.

use std::time::Instant;

use objc2::rc::Retained;
use objc2::{DefinedClass, MainThreadOnly, define_class, msg_send, sel};
use objc2_app_kit::{NSView, NSWindow};
use objc2_foundation::{
    MainThreadMarker, NSObject, NSObjectProtocol, NSRunLoop, NSRunLoopCommonModes,
};
use objc2_quartz_core::CADisplayLink;

/// Was das Rueckrufziel des Bildtakts haelt.
pub struct TaktIvars {
    /// Die Senke, an die jede Bildgrenze geht.
    senke: Box<dyn Fn(Instant)>,
}

define_class!(
    /// Das Rueckrufziel, das der `CADisplayLink` anspricht.
    // SAFETY:
    // - Die Oberklasse NSObject stellt keine Bedingungen an Unterklassen.
    // - Die Klasse implementiert `Drop` nicht.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = TaktIvars]
    struct Taktziel;

    // SAFETY: `NSObjectProtocol` stellt keine Bedingungen.
    unsafe impl NSObjectProtocol for Taktziel {}

    impl Taktziel {
        /// Eine Bildgrenze ist erreicht.
        // SAFETY: Die Signatur passt zu der, die `CADisplayLink` aufruft: ein
        // Argument, der Takt selbst.
        #[unsafe(method(bildEnde:))]
        fn bild_ende(&self, _takt: &CADisplayLink) {
            (self.ivars().senke)(Instant::now());
        }
    }
);

impl Taktziel {
    /// Ein Ziel, das jede Bildgrenze an die genannte Senke reicht.
    fn neu(mtm: MainThreadMarker, senke: Box<dyn Fn(Instant)>) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(TaktIvars { senke });
        // SAFETY: `init` von NSObject hat die hier angenommene Signatur.
        unsafe { msg_send![super(this), init] }
    }
}

/// Ein eingerichteter Bildtakt.
///
/// Der Takt laeuft, solange dieser Wert lebt. Wer ihn fallen laesst, nimmt ihn
/// damit zurueck; `invalidate` loest zugleich den Ring aus Takt und Ziel, den
/// `CADisplayLink` mit seiner starken Referenz auf das Ziel aufspannt.
pub struct Zeichenende {
    takt: Retained<CADisplayLink>,
    /// Das Ziel bleibt hier festgehalten, damit es den Takt sicher ueberlebt.
    _ziel: Retained<Taktziel>,
}

impl Zeichenende {
    /// Richtet den Bildtakt auf der genannten Ansicht ein.
    ///
    /// Die Ansicht bestimmt, welcher Bildschirm den Takt vorgibt: sie haengt in
    /// einem Fenster, und das Fenster steht auf einem Bildschirm. Ein Takt am
    /// Hauptbildschirm waere bei zwei verschieden schnellen Bildschirmen die
    /// falsche Zahl.
    pub fn einrichten(
        mtm: MainThreadMarker,
        ansicht: &NSView,
        senke: impl Fn(Instant) + 'static,
    ) -> Self {
        let ziel = Taktziel::neu(mtm, Box::new(senke));
        // SAFETY: `ziel` ist von der Klasse, die `bildEnde:` beantwortet, und
        // die Signatur der Methode entspricht der eines
        // `CADisplayLink`-Rueckrufs. `sel!` liefert einen gueltigen Selektor.
        let takt = unsafe { ansicht.displayLinkWithTarget_selector(&ziel, sel!(bildEnde:)) };
        // SAFETY: Die Laufschleife des Hauptfadens ist die, in der der Takt
        // laufen soll; `NSRunLoopCommonModes` ist ein Fremdsymbol von
        // Foundation. Wie beim Einzugstakt des Dateifensters sind es die
        // gemeinsamen Modi und nicht der gewoehnliche, damit der Takt auch
        // waehrend eines Bildlaufs oder eines offenen Menues weiterlaeuft.
        unsafe { takt.addToRunLoop_forMode(&NSRunLoop::currentRunLoop(), NSRunLoopCommonModes) };
        Self { takt, _ziel: ziel }
    }
}

impl Drop for Zeichenende {
    fn drop(&mut self) {
        self.takt.invalidate();
    }
}

/// Die Bildwiederholrate des Bildschirms, auf dem das Fenster steht.
///
/// Liefert `None`, wenn das Fenster auf keinem Bildschirm steht. Der Aufrufer
/// bricht dann ab, statt auf den Hauptbildschirm auszuweichen: eine Messung
/// ohne sichtbares Fenster misst L1 nicht, und ein Ersatzweg waere genau die
/// Sonderregel, die die Maxime "supersimpel" ausschliesst. Dieselbe Haltung wie
/// bei `--kalt` ohne Rechte.
///
/// `system_profiler` ist ausdruecklich **nicht** der Weg: am Referenzgeraet
/// `MacBookPro15,1` fuehrt es zum eingebauten Bildschirm keine Zeile
/// `Refresh Rate`, festgehalten im geschlossenen Defekt
/// `260802-1900_*_bildwiederholrate-am-referenzgeraet-nicht-per-system-profiler-erhebbar.md`.
pub fn bildwiederholrate(fenster: &NSWindow) -> Option<isize> {
    fenster
        .screen()
        .map(|bildschirm| bildschirm.maximumFramesPerSecond())
}
