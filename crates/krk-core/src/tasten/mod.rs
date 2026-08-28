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
//! Tastencode, gemeldetem Zeichen und Maske. [`parser`] haelt die eine Tabelle
//! der Tastencodes, liest die Kombinationsschreibweise und sagt ueber
//! [`Tastenkennung`], **wonach** eine Taste nachgeschlagen wird; [`belegung`]
//! ordnet Kombinationen Funktionen zu und schlaegt nach; [`konflikt`] benennt
//! eine doppelt vergebene Kombination.
//!
//! **Zwei Groessen laufen von links nach rechts, nicht eine.** Ein Tastendruck
//! traegt die Stelle auf der Tastatur und das Zeichen, das sie meldet; welche
//! der beiden der Nachschlag vergleicht, sagt [`Tastendruck::kennung`], und
//! warum es zwei sein muessen, sagt der Kopf von [`parser`].
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
    Belegung, Belegungsdatei, Belegungsfehler, Funktion, Kommando, Nachschlag, Wirkungsbereich,
    Zuweisungsfehler,
};
pub use konflikt::{Funktionsname, Konflikt};
pub use normalisierung::{ModMaske, normalisieren};
pub use parser::{
    Herkunft, Kombination, Schreibfehler, Taste, Tastenkennung, code_von, code_von_pflicht,
};

/// Ein Tastendruck, wie ihn der Nachschlag sieht.
///
/// Die Maske ist bereits normalisiert. Zwei Ereignisse, die sich nur in einem
/// geloeschten Bit unterscheiden, ergeben denselben Tastendruck.
///
/// **Er traegt beide Groessen, die Stelle und das Zeichen, und die Kennung
/// waehlt zwischen ihnen.** Welche der beiden gilt, entscheidet allein
/// [`Tastendruck::kennung`]; der Modulkopf von [`parser`] sagt, warum es zwei
/// sind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tastendruck {
    /// Der virtuelle Tastencode aus `NSEvent.keyCode`: die **Stelle** auf der
    /// Tastatur.
    ///
    /// Er bleibt fuer die Diagnose (`--tasten-protokoll`) und fuer das
    /// synthetische Ereignis der Messstrecke erhalten, auch wo der Nachschlag
    /// ihn nicht ansieht.
    pub code: u16,
    /// Das gemeldete Zeichen, sofern es als Kennung taugt.
    ///
    /// Immer schon durch [`parser::zeichen_als_kennung`] gegangen: ein
    /// ASCII-Kleinbuchstabe, eine ASCII-Ziffer, `+` oder `-`, oder `None`.
    /// `None` heisst, dass dieser Druck ueber seine Stelle nachgeschlagen wird.
    pub zeichen: Option<char>,
    /// Die normalisierte Maske der Zusatztasten.
    pub maske: ModMaske,
}

impl Tastendruck {
    /// Ein Tastendruck aus Code und bereits normalisierter Maske.
    ///
    /// Das Zeichen kommt aus der Tabelle: der Aufruf beschreibt einen Druck auf
    /// die Stelle `code` an einer Tastatur, die so belegt ist wie
    /// [`parser::TASTEN`] es fuehrt. Wer ein wirklich gemeldetes Zeichen hat,
    /// nimmt [`Tastendruck::aus_ereignis`].
    pub const fn neu(code: u16, maske: ModMaske) -> Self {
        Self {
            code,
            zeichen: parser::zeichen_der_stelle(code),
            maske,
        }
    }

    /// Ein Tastendruck aus den rohen Angaben eines AppKit-Ereignisses.
    ///
    /// `rohe_flaggen` ist der Wert aus `NSEvent.modifierFlags`, `gemeldet` das
    /// Zeichen, das die gedrueckte Taste **ohne Zusatztasten** meldet. Dies ist
    /// der einzige Weg, auf dem ein Ereignis in den Nachschlag gelangt.
    pub fn aus_ereignis(code: u16, gemeldet: Option<char>, rohe_flaggen: u64) -> Self {
        Self {
            code,
            zeichen: gemeldet.and_then(parser::zeichen_als_kennung),
            maske: normalisieren(rohe_flaggen),
        }
    }

    /// Wonach dieser Druck nachgeschlagen wird.
    ///
    /// Die eine Ableitung, und die einzige Stelle, an der die beiden
    /// Nachschlagarten fuer einen Tastendruck auseinandergehen.
    pub const fn kennung(self) -> Tastenkennung {
        match self.zeichen {
            Some(zeichen) => Tastenkennung::Zeichen(zeichen),
            None => Tastenkennung::Code(self.code),
        }
    }
}
