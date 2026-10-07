//! Was das Etikett neben „Deep“ und „Content“ ueber den stehenden Filtertext
//! sagt.
//!
//! **Keine Zeile AppKit.** Die Ansicht dazu ist das Etikett in
//! `crate::appkit::bereichsleiste`; geschrieben wird es allein von
//! `Anwendungsdelegierter::bereichsleiste_nachziehen`, und den Text dafuer
//! rechnet [`anzeigetext`] hier, ohne Fenster pruefbar.
//!
//! # Warum es diese Anzeige gibt
//!
//! Der Filtertext uebersteht jeden Ordnerwechsel (Nutzerentscheid vom
//! 260815-0955). Bis zum 261007 nannte ihn allein die Statuszeile, auf dem
//! fuenften ihrer Raenge, und jeder der vier Raenge darueber verdraengte ihn,
//! drei davon auch aus dem anderen Dateifenster. Wer in dieser Lage filterte und
//! in einen Ordner stieg, hielt den Ordner fuer leer. Der Nutzer hat am 261007
//! Moeglichkeit 2 aus
//! `shared/issues/260815-1047_*_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md`
//! gewaehlt: „lege den filtertext neben deep“. Das Etikett hat keinen Rang und
//! keinen Mitbewerber; es zeigt den Filtertext, solange er steht.
//!
//! **Die Statuszeile bleibt daneben, wie sie war.** Sie nennt zusaetzlich die
//! Trefferzahlen, den Lesefortschritt und die ausgeblendeten Markierungen; das
//! Etikett nennt allein den Text. Beide lesen denselben Wert am `Ordnermodell`
//! des sichtbaren Tabs, also koennen sie einander nicht widersprechen.

use krk_core::sprache::{Text, satz};

/// Der Text des Etiketts, oder `None`, wenn kein Filtertext steht.
///
/// **Der Filtertext erscheint woertlich**, ohne Kuerzung und ohne Umschrift:
/// gekuerzt wird auf dem Schirm, mit Auslassungszeichen am Ende, und das
/// entscheidet das Etikett an seiner Breite und nicht diese Funktion. Ein
/// Zeilenumbruch kann darin nicht stehen, weil
/// `krk_core::verzeichnis::filter::traegt_ein_dateiname` keinen aufnimmt.
///
/// **`None` heisst: das Etikett ist leer.** Es bleibt an seiner Stelle stehen,
/// und die Ankreuzfelder links davon ruecken nicht; leer ist es unsichtbar.
///
/// Gefragt wird mit derselben Bedingung wie `Ordnermodell::filter_steht`, an
/// demselben Wert: der Aufrufer reicht den Filtertext herein und nicht die
/// Antwort getrennt davon, wie bei `statuszeile::filterstand_text`.
#[must_use]
pub fn anzeigetext(filtertext: &str) -> Option<String> {
    if filtertext.is_empty() {
        return None;
    }
    Some(satz(Text::LeisteFilter, &[("filtertext", &filtertext)]))
}

#[cfg(test)]
mod tests {
    use super::*;

    use krk_core::sprache::Sprache;

    /// Ohne Filtertext zeigt das Etikett nichts.
    #[test]
    fn ohne_filtertext_zeigt_das_etikett_nichts() {
        assert_eq!(anzeigetext(""), None);
    }

    /// Ein stehender Filtertext erscheint genau so, wie er getippt ist, im
    /// Satz der geltenden Sprache (im Test immer Deutsch, weil kein
    /// Probenprozess `sprache::festlegen` ruft).
    #[test]
    fn ein_stehender_filtertext_erscheint_woertlich() {
        assert_eq!(anzeigetext("rs").as_deref(), Some("Filter „rs“"));
        assert_eq!(
            anzeigetext(".cl*Notiz").as_deref(),
            Some("Filter „.cl*Notiz“"),
            "Punkt, Platzhalter und Grossbuchstaben bleiben stehen"
        );
    }

    /// Jede der drei Sprachen fuehrt den Platzhalter `{filtertext}`; ohne ihn
    /// zeigte das Etikett in dieser Sprache einen Vorsatz ohne Text.
    #[test]
    fn jede_sprache_nennt_den_filtertext() {
        for sprache in [Sprache::De, Sprache::Fr, Sprache::En] {
            assert!(
                sprache.text(Text::LeisteFilter).contains("{filtertext}"),
                "{sprache:?} nennt den Filtertext nicht"
            );
        }
    }
}
