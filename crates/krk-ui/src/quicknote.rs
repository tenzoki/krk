//! Was die Quicknote auf F10 entscheidet, ohne AppKit (Plan
//! `260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md`).
//!
//! **Keine Zeile AppKit.** Wie [`crate::fenstermodell`] und
//! [`crate::fenstertitel`] rechnet dieses Modul und zeichnet nicht: welche der
//! drei Wirkungen F10 in einer Lage hat, sagt [`f10_wirkung`], und was beim
//! Schliessen zurueckkommt, traegt [`Rueckkehr`]. Die Flaeche selbst steht in
//! `crate::appkit::quicknote`, der Tausch im Editorbereich.
//!
//! # Offen heisst sichtbar
//!
//! **Die Quicknote ist offen genau dann, wenn der Editorbereich eine
//! [`Rueckkehr`] haelt**; ein zweites Kennzeichen „offen" daneben gibt es
//! nicht. Daraus folgt die Zusage, auf der F10 als Umschalter ruht: **ist die
//! Quicknote offen, ist der Editorbereich sichtbar.** Zwei Stellen halten sie.
//! Das Oeffnen blendet den Editorbereich ein, bevor es die Rueckkehr setzt,
//! und bricht ab, wenn das Fenster zu schmal ist; und jeder Weg, der den
//! Editorbereich ausblendet, geht durch den Nachzug nach dem
//! Sichtbarkeitswechsel beim Anwendungsdelegierten, der die Rueckkehr dort
//! fallen laesst.

use crate::fenstermodell::Randrueckkehr;
use crate::kommandos::fokus::Fokus;

/// Was ein F10 in dieser Lage tut.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F10Wirkung {
    /// Die Quicknote ist zu: sie geht auf, und der Fokus geht in sie.
    Oeffnen,
    /// Die Quicknote ist offen, der Fokus steht anderswo: er geht in sie.
    FokusHinein,
    /// Die Quicknote ist offen und hat den Fokus: sie schliesst.
    Schliessen,
}

/// Die Wirkung eines F10 aus der Frage, ob die Quicknote offen ist, und dem
/// Fokus.
///
/// **Vollstaendig ueber [`Fokus`] und ohne Auffangzweig**: ein weiterer
/// Fokuswert haelt den Bau hier an. Offen und mit dem Fokus im Editor heisst
/// der Fokus in der Quicknote, denn solange sie offen ist, zeigt der
/// Editorbereich allein sie.
#[must_use]
pub fn f10_wirkung(offen: bool, fokus: Fokus) -> F10Wirkung {
    if !offen {
        return F10Wirkung::Oeffnen;
    }
    match fokus {
        Fokus::Editor => F10Wirkung::Schliessen,
        Fokus::Dateifenster | Fokus::Leiste | Fokus::Vorschau | Fokus::Git | Fokus::Anderswo => {
            F10Wirkung::FokusHinein
        }
    }
}

/// Was beim Schliessen der Quicknote zurueckkommt: der Fokus von vor F10 und
/// der rechte Rand.
///
/// Entsteht beim Oeffnen, lebt im Editorbereich und wird beim Schliessen
/// herausgegeben. Solange er steht, ist die Quicknote offen (Modulkopf).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rueckkehr {
    /// Wo der Fokus vor F10 stand.
    pub fokus: Fokus,
    /// Was am rechten Rand zurueckzustellen ist.
    pub rand: Randrueckkehr,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `f10_wirkung` ueber beide Werte von `offen` und jeden Fokuswert, an
    /// einem Stueck.
    #[test]
    fn f10_wirkt_nach_der_tafel() {
        for fokus in Fokus::ALLE {
            assert_eq!(
                f10_wirkung(false, fokus),
                F10Wirkung::Oeffnen,
                "zu, {fokus:?}"
            );
            let erwartet = if fokus == Fokus::Editor {
                F10Wirkung::Schliessen
            } else {
                F10Wirkung::FokusHinein
            };
            assert_eq!(f10_wirkung(true, fokus), erwartet, "offen, {fokus:?}");
        }
    }
}
