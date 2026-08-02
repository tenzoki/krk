#![warn(unsafe_code)]
//! Das Binaerziel von KRK: Fenster, Menue, Dateifenster, Ereignisabgriff.
//!
//! Jeder AppKit-Aufruf ist ein unsicherer Fremdaufruf. Bezahlt wird das an
//! genau einer Stelle: das spaetere Modul `appkit` traegt `#[allow(unsafe_code)]`
//! und haelt die sicheren Huellen; ausserhalb davon warnt der Uebersetzer.

fn main() {}
