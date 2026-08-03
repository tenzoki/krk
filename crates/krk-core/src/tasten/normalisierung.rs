//! Die Normalisierung der Modifikatoren, als reine Funktion.
//!
//! AppKit meldet in `modifierFlags` eines Tastenereignisses acht Bits. Vier
//! davon taugen als Zusatztaste einer Belegung, vier nicht. [`normalisieren`]
//! trennt sie: uebrig bleiben `command`, `control`, `option` und `shift`,
//! geloescht werden `function`, die Feststelltaste, der Zehnerblock und die
//! Hilfetaste.
//!
//! **Die Loeschung von `function` ist keine Vorsichtsmassnahme, sondern die
//! Umsetzung eines Abnahmekriteriums.** Die Messung vom 260802-1137
//! (`spikes/fn-tasten/messung-A-neuauswertung.txt`) hat gezeigt, dass AppKit
//! `function` bei jeder Taste aus dem Funktionstasten-Zeichenbereich setzt,
//! auch bei den Pfeiltasten, und dass Fn+F3 und ein nacktes F3 dasselbe
//! Ereignis erzeugen. Das Bit sagt damit nichts darueber, ob der Nutzer fn
//! gehalten hat. Ein Nachschlag, der es mitfuehrte, haette fuer dieselbe Taste
//! zwei Eintraege, von denen der Nutzer nur einen erreichen kann. C3 des Specs
//! verlangt genau das Gegenteil: "Der Nutzer kann fn nicht als Zusatztaste
//! einer Belegung verwenden".
//!
//! Die Feststelltaste faellt aus demselben Grund weg: sie ist ein Zustand der
//! Tastatur und keine gehaltene Taste. Der Zehnerblock ebenfalls, denn AppKit
//! setzt sein Bit auch bei den Pfeiltasten.

use std::fmt;
use std::ops::{BitOr, BitOrAssign};

/// Die Bits, wie AppKit sie in `NSEvent.modifierFlags` liefert.
///
/// Sie stehen hier als nackte Zahlen und nicht als Verweis auf
/// `NSEventModifierFlags`, weil der Kern AppKit nicht kennt. Die Werte sind
/// Teil der binaeren Schnittstelle von AppKit und aendern sich nicht. Sie sind
/// oeffentlich, damit die Tests von [`normalisieren`] die Bits benennen
/// koennen, statt Zahlen zu wiederholen.
pub mod roh {
    /// `NSEventModifierFlagCapsLock`.
    pub const FESTSTELLTASTE: u64 = 1 << 16;
    /// `NSEventModifierFlagShift`.
    pub const UMSCHALT: u64 = 1 << 17;
    /// `NSEventModifierFlagControl`.
    pub const STEUERUNG: u64 = 1 << 18;
    /// `NSEventModifierFlagOption`.
    pub const WAHL: u64 = 1 << 19;
    /// `NSEventModifierFlagCommand`.
    pub const BEFEHL: u64 = 1 << 20;
    /// `NSEventModifierFlagNumericPad`.
    pub const ZEHNERBLOCK: u64 = 1 << 21;
    /// `NSEventModifierFlagHelp`.
    pub const HILFE: u64 = 1 << 22;
    /// `NSEventModifierFlagFunction`.
    pub const FUNKTION: u64 = 1 << 23;
}

/// Die vier Zusatztasten, auf die KRK eine Belegung stuetzt.
///
/// Eine Maske ist der Nachschlagteil eines Tastendrucks: zwei Ereignisse mit
/// demselben Tastencode und derselben Maske loesen dieselbe Funktion aus.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ModMaske(u8);

impl ModMaske {
    /// Keine Zusatztaste.
    pub const LEER: Self = Self(0);
    /// Die Befehlstaste (`command`).
    pub const BEFEHL: Self = Self(1 << 0);
    /// Die Steuerungstaste (`control`).
    pub const STEUERUNG: Self = Self(1 << 1);
    /// Die Wahltaste (`option`).
    pub const WAHL: Self = Self(1 << 2);
    /// Die Umschalttaste (`shift`).
    pub const UMSCHALT: Self = Self(1 << 3);

    /// Die vier Zusatztasten mit ihrem Namen, in der Reihenfolge der Anzeige.
    ///
    /// Die Reihenfolge ist fest, damit `command+shift` und `shift+command`
    /// denselben Text ergeben. Die Namen sind die, unter denen
    /// `resources/default-keymap.toml` die Zusatztasten spaeter fuehrt.
    pub const BENANNT: [(Self, &'static str); 4] = [
        (Self::BEFEHL, "command"),
        (Self::STEUERUNG, "control"),
        (Self::WAHL, "option"),
        (Self::UMSCHALT, "shift"),
    ];

    /// Die gesetzten Bits als Zahl.
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Wahr, wenn keine Zusatztaste gehalten ist.
    pub const fn ist_leer(self) -> bool {
        self.0 == 0
    }

    /// Wahr, wenn alle Bits der genannten Maske gesetzt sind.
    pub const fn enthaelt(self, andere: Self) -> bool {
        self.0 & andere.0 == andere.0
    }
}

impl BitOr for ModMaske {
    type Output = Self;

    fn bitor(self, andere: Self) -> Self {
        Self(self.0 | andere.0)
    }
}

impl BitOrAssign for ModMaske {
    fn bitor_assign(&mut self, andere: Self) {
        self.0 |= andere.0;
    }
}

impl fmt::Display for ModMaske {
    /// Schreibt die Maske als `command+shift`, die leere als `keine`.
    ///
    /// Diese Schreibweise geht in den Protokollmodus `--tasten-protokoll` und
    /// ist damit das, was der Nutzer bei der Abnahme liest.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ist_leer() {
            return f.write_str("keine");
        }
        let mut getrennt = false;
        for (maske, name) in ModMaske::BENANNT {
            if !self.enthaelt(maske) {
                continue;
            }
            if getrennt {
                f.write_str("+")?;
            }
            f.write_str(name)?;
            getrennt = true;
        }
        Ok(())
    }
}

impl fmt::Debug for ModMaske {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ModMaske({self})")
    }
}

/// Macht aus den rohen Bits eines Tastenereignisses die Nachschlagemaske.
///
/// `rohe_flaggen` ist der Wert aus `NSEvent.modifierFlags`. Alles ausserhalb
/// der vier gehaltenen Zusatztasten faellt weg, siehe den Modulkopf.
pub fn normalisieren(rohe_flaggen: u64) -> ModMaske {
    let mut maske = ModMaske::LEER;
    if rohe_flaggen & roh::BEFEHL != 0 {
        maske |= ModMaske::BEFEHL;
    }
    if rohe_flaggen & roh::STEUERUNG != 0 {
        maske |= ModMaske::STEUERUNG;
    }
    if rohe_flaggen & roh::WAHL != 0 {
        maske |= ModMaske::WAHL;
    }
    if rohe_flaggen & roh::UMSCHALT != 0 {
        maske |= ModMaske::UMSCHALT;
    }
    maske
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eine_leere_maske_heisst_keine() {
        assert_eq!(ModMaske::LEER.to_string(), "keine");
        assert!(ModMaske::LEER.ist_leer());
    }

    #[test]
    fn die_anzeige_haelt_eine_feste_reihenfolge() {
        let erst_umschalt = ModMaske::UMSCHALT | ModMaske::BEFEHL;
        let erst_befehl = ModMaske::BEFEHL | ModMaske::UMSCHALT;
        assert_eq!(erst_umschalt.to_string(), "command+shift");
        assert_eq!(erst_befehl.to_string(), "command+shift");
    }

    #[test]
    fn jede_zusatztaste_traegt_ein_eigenes_bit() {
        for (stelle, (maske, _)) in ModMaske::BENANNT.into_iter().enumerate() {
            for (andere, _) in ModMaske::BENANNT.into_iter().skip(stelle + 1) {
                assert_ne!(maske.bits(), andere.bits());
            }
        }
    }
}
