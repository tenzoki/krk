//! Die Huelle um `NSFileManager.trashItemAtURL:resultingItemURL:error:`.
//!
//! Sie ist die Implementierung der Schnittstelle
//! [`krk_core::operation::Papierkorb`], und sie liegt hier, weil sie AppKit
//! ruft. Die Schnittstelle selbst steht im Kern und kennt AppKit nicht:
//!
//! ```text
//!   krk-core::operation::loeschen        hier
//!   ─────────────────────────────        ────
//!   trait Papierkorb            <──────  impl Papierkorb for Systempapierkorb
//!        ^                                       │
//!        └── die Operationsmaschine ruft         └─> NSFileManager
//! ```
//!
//! Das ist die eine Abhaengigkeitsumkehr des Entwurfs: der **Aufruf** laeuft
//! von unten nach oben, die **Uebersetzungsabhaengigkeit** weiterhin von oben
//! nach unten. `krk-core` nennt keine `objc2`-Kiste.
//!
//! Was ueber die Grenze geht, sind gewoehnliche Rust-Werte: ein [`Path`] hinein,
//! ein [`PathBuf`] oder ein [`io::Error`] heraus. Kein `NSURL`, kein `NSError`.
//!
//! # Warum der Papierkorb keinen eigenen Faden braucht
//!
//! `NSFileManager` ist von jedem Faden aus zu rufen, und die
//! Operationsmaschine ruft ihn von ihrem Arbeitsfaden. Der Hauptfaden bleibt
//! damit auch beim Loeschen frei, was L9 verlangt. Eine Ruecknahme fuehrt KRK
//! nicht selbst: der Rueckweg ist der Papierkorb des Systems (C4).

use std::io;
use std::path::{Path, PathBuf};

use objc2_foundation::{NSFileManager, NSString, NSURL};

use krk_core::operation::Papierkorb;

/// Der Papierkorb des Systems.
#[derive(Debug, Clone, Copy, Default)]
// Eingehaengt wird er in S16, wo die Oberflaeche zu C4 entsteht und die
// Operationsmaschine ihren Auftrag bekommt. Bis dahin hat die Schnittstelle im
// Kern im laufenden Programm keine Implementierung.
#[expect(
    dead_code,
    reason = "wird in S16 in die Operationsmaschine injiziert (Plan, Dateiliste von Schritt 16)"
)]
pub struct Systempapierkorb;

impl Papierkorb for Systempapierkorb {
    fn in_den_papierkorb(&self, pfad: &Path) -> io::Result<PathBuf> {
        let Some(text) = pfad.to_str() else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} ist kein gueltiger UTF-8-Pfad", pfad.display()),
            ));
        };
        let url = NSURL::fileURLWithPath(&NSString::from_str(text));

        let mut neuer_ort: Option<objc2::rc::Retained<NSURL>> = None;
        NSFileManager::defaultManager()
            .trashItemAtURL_resultingItemURL_error(&url, Some(&mut neuer_ort))
            .map_err(|fehler| io::Error::other(fehler.localizedDescription().to_string()))?;

        // Das System nennt den neuen Ort; nennt es keinen, ist der Eintrag
        // trotzdem im Papierkorb. Dann bleibt der alte Pfad die einzige
        // Auskunft, die wir haben, und die ist besser als ein Fehler ueber eine
        // Loeschung, die geklappt hat.
        Ok(neuer_ort.and_then(|ort| ort.path()).map_or_else(
            || pfad.to_path_buf(),
            |pfad| PathBuf::from(pfad.to_string()),
        ))
    }
}
