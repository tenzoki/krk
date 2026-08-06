//! Die Kollisionspruefung der Vorschau: welcher neue Name nicht geht, und
//! warum (C4).
//!
//! Drei Faelle, und der Spec zaehlt genau diese drei auf: der neue Name
//! kollidiert mit einem bestehenden Eintrag, er kollidiert mit einem anderen
//! neuen Namen aus derselben Regel, oder er waere leer.
//!
//! ```text
//!  neuer Name ──> name_pruefen (S15) ──leer/unzulaessig──> Unzulaessig
//!             ──> steht im Bestand? ────────────────────> Bestehender
//!             ──> kommt zweimal vor? ───────────────────> Doppelt
//! ```
//!
//! # Der Bestand ist der ganze Ordner, nicht die sichtbare Liste
//!
//! Ein ausgeblendeter Eintrag belegt seinen Namen genauso wie ein sichtbarer.
//! Wer den Bestand aus der Sichtreihenfolge zusammensuchte, uebersaehe jede
//! Kollision mit einer versteckten Datei, und das Umbenennen scheiterte erst
//! im Dateisystem.
//!
//! # Ein Eintrag, der selbst umbenannt wird, zaehlt trotzdem zum Bestand
//!
//! `a.txt` → `b.txt` und `b.txt` → `c.txt` sieht auf den ersten Blick
//! aufloesbar aus, ist es aber nur in einer bestimmten Reihenfolge: laeuft
//! `a.txt` zuerst, scheitert es an dem noch vorhandenen `b.txt`. Die Pruefung
//! meldet den Fall deshalb als Kollision, statt sich auf eine Reihenfolge zu
//! verlassen, die der Nutzer nicht sieht. Ausgenommen ist allein der Eintrag
//! selbst: ein Name, der sich nicht aendert, kollidiert nicht mit sich.

use std::collections::{HashMap, HashSet};

use crate::operation::{Namensfehler, name_pruefen};

/// Warum ein neuer Name nicht vergeben werden kann (C4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kollision {
    /// Der neue Name ist kein Name. Der leere Name aus C4 ist dieser Fall mit
    /// [`Namensfehler::Leer`].
    Unzulaessig(Namensfehler),
    /// Im Ordner steht schon ein Eintrag dieses Namens.
    Bestehender,
    /// Ein anderer Eintrag desselben Stapels bekaeme denselben neuen Namen.
    Doppelt,
}

impl Kollision {
    /// Der Grund im Klartext, so wie ihn die Vorschau zeigt.
    ///
    /// Der Text steht **neben** der farbigen Auszeichnung und nicht statt
    /// ihrer: eine Markierung, die allein an der Farbe zu erkennen waere, ist
    /// als Defekt festgehalten
    /// (`issues/260804-1309_*_die-markierung-ist-allein-an-der-farbe-erkennbar.md`).
    ///
    /// **Kurz gehalten, und das ist eine Anforderung und kein Geschmack.** Der
    /// Grund steht in einer Spalte der Vorschau; ein Satz, der dort abgeschnitten
    /// wird, nennt den Grund nicht mehr. Am laufenden Buendel gemessen am
    /// 260804-2033: bei 240 Punkten Spaltenbreite passen rund dreissig Zeichen.
    pub fn grund(self) -> &'static str {
        match self {
            Kollision::Unzulaessig(fehler) => fehler.grund(),
            Kollision::Bestehender => "der Name ist schon vergeben",
            Kollision::Doppelt => "zweimal derselbe neue Name",
        }
    }
}

/// Prueft die neuen Namen eines Stapels und nennt je Eintrag den Grund.
///
/// `alte` und `neue` stehen Position fuer Position nebeneinander, beide in
/// Sichtreihenfolge. `bestand` sind alle Namen des Ordners, auch die
/// ausgeblendeten. Das Ergebnis hat dieselbe Laenge wie `neue`; `None` heisst,
/// dass dieser Name vergeben werden kann.
///
/// **Die Reihenfolge der Pruefungen ist die Reihenfolge der Genauigkeit.**
/// Zuerst, ob der Name ueberhaupt einer ist; dann, ob der Ordner ihn schon
/// traegt; zuletzt, ob die Regel ihn zweimal vergibt. Trifft mehr als eines zu,
/// steht der erste Grund da: er ist der, den der Nutzer zuerst beheben muss.
pub fn pruefen(alte: &[String], neue: &[String], bestand: &[String]) -> Vec<Option<Kollision>> {
    let vorhanden: HashSet<&str> = bestand.iter().map(String::as_str).collect();
    let mut haeufigkeit: HashMap<&str, usize> = HashMap::with_capacity(neue.len());
    for name in neue {
        *haeufigkeit.entry(name.as_str()).or_default() += 1;
    }

    neue.iter()
        .enumerate()
        .map(|(stelle, neu)| {
            if let Err(fehler) = name_pruefen(neu) {
                return Some(Kollision::Unzulaessig(fehler));
            }
            let alt = alte.get(stelle).map(String::as_str);
            if Some(neu.as_str()) != alt && vorhanden.contains(neu.as_str()) {
                return Some(Kollision::Bestehender);
            }
            if haeufigkeit.get(neu.as_str()).copied().unwrap_or_default() > 1 {
                return Some(Kollision::Doppelt);
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn namen(liste: &[&str]) -> Vec<String> {
        liste.iter().map(|name| (*name).to_owned()).collect()
    }

    #[test]
    fn ein_freier_name_ist_keine_kollision() {
        let ergebnis = pruefen(
            &namen(&["a.txt"]),
            &namen(&["neu.txt"]),
            &namen(&["a.txt", "b.txt"]),
        );
        assert_eq!(ergebnis, [None]);
    }

    #[test]
    fn ein_unveraenderter_name_kollidiert_nicht_mit_sich_selbst() {
        let ergebnis = pruefen(
            &namen(&["a.txt"]),
            &namen(&["a.txt"]),
            &namen(&["a.txt", "b.txt"]),
        );
        assert_eq!(ergebnis, [None]);
    }

    #[test]
    fn ein_ausgeblendeter_eintrag_belegt_seinen_namen_ebenso() {
        let ergebnis = pruefen(
            &namen(&["a.txt"]),
            &namen(&[".versteckt"]),
            &namen(&["a.txt", ".versteckt"]),
        );
        assert_eq!(ergebnis, [Some(Kollision::Bestehender)]);
    }

    #[test]
    fn der_grund_steht_in_worten_da() {
        assert_eq!(
            Kollision::Unzulaessig(Namensfehler::Leer).grund(),
            "der Name ist leer"
        );
        assert!(Kollision::Bestehender.grund().contains("vergeben"));
        assert!(Kollision::Doppelt.grund().contains("zweimal"));
        // Die Spalte der Vorschau schneidet laengere Gruende ab.
        for kollision in [Kollision::Bestehender, Kollision::Doppelt] {
            assert!(
                kollision.grund().chars().count() <= 30,
                "der Grund passt nicht in die Spalte: {}",
                kollision.grund()
            );
        }
    }
}
