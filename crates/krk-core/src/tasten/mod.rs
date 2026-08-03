//! Tastendruecke: Normalisierung der Modifikatoren und die Zuordnung auf
//! Kommandos.
//!
//! Zwei Module:
//!
//! ```text
//! normalisierung ──> mod (Tastendruck, Kommando, kommando)
//! ```
//!
//! [`normalisierung`] macht aus den rohen Bits eines AppKit-Ereignisses die
//! Nachschlagemaske. Dieses Modul setzt darauf den Nachschlag: ein
//! [`Tastendruck`] aus Tastencode und Maske ergibt hoechstens ein
//! [`Kommando`].
//!
//! **Die Tabelle unten ist fest verdrahtet und bleibt es nicht.** Runde 1,
//! Schritt 7 braucht fuenf Tasten, um den Weg vom Ereignisabgriff bis in das
//! Ordnermodell einmal ganz zu gehen. Schritt 11 ersetzt sie durch die
//! Belegungsmaschine, die `resources/default-keymap.toml` liest und die
//! Abweichungen des Nutzers darueberlegt. Was hier bleibt, sind [`ModMaske`]
//! und [`normalisieren`]: der Nachschlag der Belegungsmaschine laeuft ueber
//! dieselbe Maske.
//!
//! Der Kern kennt AppKit nicht; alles hier ist ohne Fenster testbar.

pub mod normalisierung;

pub use normalisierung::{ModMaske, normalisieren};

/// Die virtuellen Tastencodes, die Schritt 7 belegt.
///
/// Ein virtueller Tastencode benennt die Stelle auf der Tastatur und nicht das
/// Zeichen. Er ist damit unabhaengig von der Tastaturbelegung des Systems, und
/// genau deshalb belegt KRK ihn und nicht das gemeldete Zeichen.
pub mod code {
    /// Die Zeilenschaltung (`return`).
    pub const RETURN: u16 = 36;
    /// Bild auf (`page up`).
    pub const BILD_AUF: u16 = 116;
    /// Bild ab (`page down`).
    pub const BILD_AB: u16 = 121;
    /// Pfeil nach unten.
    pub const PFEIL_AB: u16 = 125;
    /// Pfeil nach oben.
    pub const PFEIL_AUF: u16 = 126;
}

/// Ein Tastendruck, wie ihn der Nachschlag sieht.
///
/// Die Maske ist bereits normalisiert. Zwei Ereignisse, die sich nur in einem
/// geloeschten Bit unterscheiden, ergeben denselben Tastendruck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Was ein Tastendruck im Dateifenster ausloest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kommando {
    /// Die Auswahl einen Eintrag nach oben.
    AuswahlHoch,
    /// Die Auswahl einen Eintrag nach unten.
    AuswahlRunter,
    /// Die Auswahl eine Bildschirmseite nach oben.
    SeiteHoch,
    /// Die Auswahl eine Bildschirmseite nach unten.
    SeiteRunter,
    /// In den ausgewaehlten Ordner hineinsteigen.
    Oeffnen,
}

/// Die fest verdrahtete Belegung der Runde 1, Schritt 7.
const VERDRAHTET: [(u16, Kommando); 5] = [
    (code::PFEIL_AUF, Kommando::AuswahlHoch),
    (code::PFEIL_AB, Kommando::AuswahlRunter),
    (code::BILD_AUF, Kommando::SeiteHoch),
    (code::BILD_AB, Kommando::SeiteRunter),
    (code::RETURN, Kommando::Oeffnen),
];

/// Das Kommando zu einem Tastendruck, falls die Tabelle eines kennt.
///
/// **Eine gehaltene Zusatztaste schlaegt nicht durch.** Umschalt+Pfeil ab
/// bleibt unbelegt, statt wie ein nacktes Pfeil ab zu wirken; die Taste gehoert
/// spaeter der Bereichsauswahl aus C2. Ein Nachschlag, der die Maske ignoriert,
/// haette den Platz schon vergeben.
pub fn kommando(druck: Tastendruck) -> Option<Kommando> {
    if !druck.maske.ist_leer() {
        return None;
    }
    VERDRAHTET
        .into_iter()
        .find(|(code, _)| *code == druck.code)
        .map(|(_, kommando)| kommando)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jeder_verdrahtete_code_steht_genau_einmal() {
        for (stelle, (code, _)) in VERDRAHTET.into_iter().enumerate() {
            for (anderer, _) in VERDRAHTET.into_iter().skip(stelle + 1) {
                assert_ne!(code, anderer);
            }
        }
    }

    #[test]
    fn jedes_kommando_steht_genau_einmal() {
        for (stelle, (_, kommando)) in VERDRAHTET.into_iter().enumerate() {
            for (_, anderes) in VERDRAHTET.into_iter().skip(stelle + 1) {
                assert_ne!(kommando, anderes);
            }
        }
    }
}
