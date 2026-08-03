//! Der Ereignisabgriff: der einzige Eintrittspunkt fuer Tastendruecke.
//!
//! **Ein Abgriff, kein zweiter Weg.** Jeder Tastendruck laeuft durch den
//! lokalen Ereignisabgriff `NSEvent.addLocalMonitorForEventsMatchingMask`, und
//! keine Ansicht bekommt eine eigene `keyDown:`-Behandlung. Das ist die
//! Voraussetzung dafuer, dass die Belegung aus Schritt 11 spaeter wirklich
//! jede Taste traegt: eine Ansicht, die eine Taste selbst abfaengt, waere die
//! Sonderregel mit eigenem Rueckfallweg, die die Maxime "supersimpel"
//! ausschliesst.
//!
//! Der Abgriff ist **lokal** und nicht global. Ein globaler Abgriff sieht die
//! Tasten anderer Anwendungen und braucht dafuer die Freigabe fuer
//! Bedienungshilfen. Die Messung vom 260802-1137 hat belegt, dass der lokale
//! Abgriff einer gewoehnlichen Anwendung im Vordergrund auch die
//! Funktionstasten sieht; KRK braucht die Freigabe deshalb nicht.
//!
//! **Der Weg eines Tastendrucks**, vom Ereignis bis in das Ordnermodell:
//!
//! ```text
//! NSEvent ──> Tastendruck::aus_ereignis ──> tasten::kommando
//!                  (Maske normalisiert)          │
//!                                                v
//!                       DateifensterQuelle::kommando_ausfuehren
//! ```
//!
//! Trifft der Nachschlag, schluckt der Abgriff das Ereignis (er liefert
//! `nil`); sonst reicht er es unveraendert weiter, damit Cmd+Q, Cmd+W und die
//! Texteingabe des Systems ihren gewohnten Weg gehen.

use std::ptr::NonNull;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{NSEvent, NSEventMask};

use krk_core::tasten::{self, Tastendruck};

use super::tabelle::DateifensterQuelle;

/// Ein eingerichteter Ereignisabgriff.
///
/// Der Abgriff bleibt bestehen, solange dieser Wert lebt. Wer ihn fallen
/// laesst, nimmt ihn damit zurueck.
pub struct Tastenabgriff {
    /// Das Merkzeichen, das AppKit beim Einrichten liefert. Es gibt nichts
    /// preis; es wird allein gebraucht, um den Abgriff wieder abzumelden.
    merkzeichen: Retained<AnyObject>,
}

impl Tastenabgriff {
    /// Richtet den Abgriff ein und leitet die Kommandos an `ziel`.
    ///
    /// Liefert `None`, wenn AppKit den Abgriff nicht einrichtet. Der Aufrufer
    /// meldet das; still ohne Tastatur weiterzulaufen waere der schlechteste
    /// aller Ausgaenge.
    ///
    /// `protokoll` schaltet den Modus `--tasten-protokoll`: jeder empfangene
    /// Tastendruck geht mit seinem Code und seiner normalisierten Maske auf die
    /// Standardausgabe, gleich ob die Tabelle ihn kennt.
    pub fn einrichten(ziel: Retained<DateifensterQuelle>, protokoll: bool) -> Option<Self> {
        let block = RcBlock::new(move |ereignis: NonNull<NSEvent>| -> *mut NSEvent {
            // SAFETY: AppKit reicht dem Block einen gueltigen Zeiger auf das
            // Ereignis, das fuer die Dauer des Aufrufs lebt.
            let geschluckt = behandeln(&ziel, unsafe { ereignis.as_ref() }, protokoll);
            if geschluckt {
                // `nil` heisst: das Ereignis geht nicht weiter.
                std::ptr::null_mut()
            } else {
                // Unveraendert weiterreichen. Der Zeiger ist derselbe, den
                // AppKit hereingegeben hat; er wechselt keinen Besitzer.
                ereignis.as_ptr()
            }
        });

        // SAFETY: Der Block hat die Signatur, die der Abgriff verlangt, und
        // AppKit kopiert ihn beim Einrichten auf den Haldenspeicher. Er haelt
        // `ziel` fest und ueberlebt damit den Aufruf.
        let merkzeichen = unsafe {
            NSEvent::addLocalMonitorForEventsMatchingMask_handler(NSEventMask::KeyDown, &block)
        }?;
        Some(Self { merkzeichen })
    }
}

impl Drop for Tastenabgriff {
    fn drop(&mut self) {
        // SAFETY: Das Merkzeichen stammt aus
        // `addLocalMonitorForEventsMatchingMask:handler:` und ist damit von der
        // Art, die `removeMonitor:` erwartet.
        unsafe { NSEvent::removeMonitor(&self.merkzeichen) };
    }
}

/// Wertet ein Tastenereignis aus. Liefert, ob es geschluckt wurde.
fn behandeln(ziel: &DateifensterQuelle, ereignis: &NSEvent, protokoll: bool) -> bool {
    let druck = Tastendruck::aus_ereignis(ereignis.keyCode(), ereignis.modifierFlags().0 as u64);
    let kommando = tasten::kommando(druck);

    if protokoll {
        // Auf die Standardausgabe, wie der Plan es vorschreibt. Sichtbar ist
        // sie nur, wenn KRK aus einem Terminal gestartet wurde: ein ueber
        // `open` gestartetes Buendel bekommt von LaunchServices keine.
        let nachschlag = match kommando {
            Some(kommando) => format!("{kommando:?}"),
            None => "unbelegt".to_owned(),
        };
        println!(
            "tastencode={} maske={} kommando={nachschlag}",
            druck.code, druck.maske
        );
    }

    match kommando {
        Some(kommando) => {
            ziel.kommando_ausfuehren(kommando);
            true
        }
        None => false,
    }
}
