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

/// Wie ein Kopieren der Quicknote ausgegangen ist (Q2 des Spec).
///
/// **Drei Werte, ueberschneidungsfrei und vollstaendig.** Ein leerer Puffer
/// wird nicht kopiert und laesst die Zwischenablage, wie sie war (A12); ein
/// gelungenes Kopieren nennt die Zahl der Zeichen; ein gescheitertes laesst den
/// Text stehen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "der Ausgang entscheidet, ob der Puffer geleert und die Quicknote geschlossen wird, und traegt die Meldung"]
pub enum Kopierausgang {
    /// Der Puffer war leer; die Zwischenablage ist unberuehrt.
    Leer,
    /// Der ganze Text liegt in der Zwischenablage.
    Kopiert {
        /// Wie viele Zeichen, gezaehlt als Unicode-Zeichen und nicht als Bytes.
        zeichen: usize,
    },
    /// Die Zwischenablage hat den Text nicht angenommen.
    Gescheitert,
}

impl Kopierausgang {
    /// Ob der Puffer nach diesem Ausgang zu leeren ist: allein nach einem
    /// gelungenen Kopieren.
    #[allow(
        clippy::match_like_matches_macro,
        reason = "`matches!` prueft die Vollstaendigkeit nicht, und genau die ist hier der Zweck"
    )]
    #[must_use]
    pub fn leert_den_puffer(self) -> bool {
        match self {
            Kopierausgang::Kopiert { .. } => true,
            Kopierausgang::Leer | Kopierausgang::Gescheitert => false,
        }
    }

    /// Ob die Quicknote nach diesem Ausgang schliesst: nach einem gelungenen
    /// Kopieren und bei leerem Puffer, nicht nach einem gescheiterten, denn
    /// dann steht ihr Text noch da und gehoert vor Augen.
    #[allow(
        clippy::match_like_matches_macro,
        reason = "`matches!` prueft die Vollstaendigkeit nicht, und genau die ist hier der Zweck"
    )]
    #[must_use]
    pub fn schliesst(self) -> bool {
        match self {
            Kopierausgang::Kopiert { .. } | Kopierausgang::Leer => true,
            Kopierausgang::Gescheitert => false,
        }
    }
}

/// Kopiert den ganzen Text ueber `schreiben`, ausser er ist leer.
///
/// `schreiben` ist der Weg in die Zwischenablage und liefert, ob sie den Text
/// angenommen hat; beim Anwendungsdelegierten ist es die eine Huelle
/// `appkit::zwischenablage::text_schreiben`. Bei leerem Text wird er nicht
/// gerufen.
pub fn kopieren(text: &str, schreiben: impl FnOnce(&str) -> bool) -> Kopierausgang {
    if text.is_empty() {
        return Kopierausgang::Leer;
    }
    if schreiben(text) {
        Kopierausgang::Kopiert {
            zeichen: text.chars().count(),
        }
    } else {
        Kopierausgang::Gescheitert
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ein leerer Puffer ruft `schreiben` nicht und ergibt `Leer`.
    #[test]
    fn ein_leerer_puffer_wird_nicht_kopiert() {
        let ausgang = kopieren("", |_| panic!("ein leerer Puffer ruft schreiben"));
        assert_eq!(ausgang, Kopierausgang::Leer);
        assert!(!ausgang.leert_den_puffer());
        assert!(ausgang.schliesst());
    }

    /// Ein mehrzeiliger Text mit Umlauten kommt Zeichen fuer Zeichen an, und
    /// gezaehlt werden Zeichen und nicht Bytes.
    #[test]
    fn ein_gelungenes_kopieren_zaehlt_zeichen_und_reicht_den_ganzen_text() {
        let text = "Grüße\r\naus Köln\nund Zürich\n";
        let mut angekommen = String::new();
        let ausgang = kopieren(text, |geschrieben| {
            angekommen.push_str(geschrieben);
            true
        });
        assert_eq!(angekommen, text);
        assert_eq!(
            ausgang,
            Kopierausgang::Kopiert {
                zeichen: text.chars().count()
            }
        );
        assert_ne!(
            text.chars().count(),
            text.len(),
            "die Probe misst keine Umlaute"
        );
        assert!(ausgang.leert_den_puffer());
        assert!(ausgang.schliesst());
    }

    /// Ein gescheitertes Kopieren leert nicht und schliesst nicht.
    #[test]
    fn ein_gescheitertes_kopieren_laesst_den_text_stehen() {
        let ausgang = kopieren("Notiz", |_| false);
        assert_eq!(ausgang, Kopierausgang::Gescheitert);
        assert!(!ausgang.leert_den_puffer());
        assert!(!ausgang.schliesst());
    }

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
