//! Tastendruecke: Normalisierung der Modifikatoren, die Belegung und der
//! Nachschlag.
//!
//! Vier Module, in der Reihenfolge, in der ein Tastendruck sie durchlaeuft:
//!
//! ```text
//! normalisierung ──> mod (Tastendruck) ──> belegung (Nachschlag) ──> Kommando
//!                          ^                   ^          ^
//!                          │                   │          │
//!                       parser ────────────────┘       konflikt
//! ```
//!
//! [`normalisierung`] macht aus den rohen Bits eines AppKit-Ereignisses die
//! Nachschlagemaske. Dieses Modul setzt darauf den [`Tastendruck`] aus
//! Tastencode und Maske. [`parser`] haelt die eine Tabelle der Tastencodes und
//! liest die Kombinationsschreibweise; [`belegung`] ordnet Kombinationen
//! Funktionen zu und schlaegt nach; [`konflikt`] benennt eine doppelt vergebene
//! Kombination.
//!
//! **Die fest verdrahtete Tabelle aus Schritt 7 ist fort.** Sie hatte fuenf
//! Tasten getragen, damit der Durchstich eine Auswahl bewegen kann, und sie ist
//! mit Schritt 11 abgeloest und nicht ergaenzt worden: es gibt genau einen Weg
//! von einer Taste zu einer Funktion, und er beginnt in
//! `resources/default-keymap.toml`. Dasselbe gilt fuer die Tastencodes, die
//! allein in [`parser::TASTEN`] stehen.
//!
//! Der Kern kennt AppKit nicht; alles hier ist ohne Fenster testbar.

pub mod belegung;
pub mod konflikt;
pub mod normalisierung;
pub mod parser;

pub use belegung::{
    Belegung, Belegungsdatei, Belegungsfehler, Funktion, Kommando, Nachschlag, Zuweisungsfehler,
};
pub use konflikt::{Funktionsname, Konflikt};
pub use normalisierung::{ModMaske, normalisieren};
pub use parser::{Herkunft, Kombination, Schreibfehler, Taste, code_von, code_von_pflicht};

/// Ein Tastendruck, wie ihn der Nachschlag sieht.
///
/// Die Maske ist bereits normalisiert. Zwei Ereignisse, die sich nur in einem
/// geloeschten Bit unterscheiden, ergeben denselben Tastendruck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tastendruck {
    /// Der virtuelle Tastencode aus `NSEvent.keyCode`.
    pub code: u16,
    /// Die normalisierte Maske der Zusatztasten.
    pub maske: ModMaske,
}

impl Tastendruck {
    /// Ein Tastendruck aus Code und bereits normalisierter Maske.
    pub const fn neu(code: u16, maske: ModMaske) -> Self {
        Self { code, maske }
    }

    /// Ein Tastendruck aus den rohen Angaben eines AppKit-Ereignisses.
    ///
    /// `rohe_flaggen` ist der Wert aus `NSEvent.modifierFlags`. Dies ist der
    /// einzige Weg, auf dem ein Ereignis in den Nachschlag gelangt.
    pub fn aus_ereignis(code: u16, rohe_flaggen: u64) -> Self {
        Self::neu(code, normalisieren(rohe_flaggen))
    }
}
