//! Die Vorschau: je markiertem Eintrag der alte und der neue Name (C4).
//!
//! Sie ist das, was das Umbenennen im Stapel ungefaehrlich macht. C4 verlangt
//! sie ausdruecklich vor der Ausfuehrung, und erst ein zweiter, ausdruecklicher
//! Befehl fuehrt aus, was hier steht.
//!
//! ```text
//!  Regel + markierte Namen + Bestand ──> Vorschau
//!                                          │
//!                        Zeile: alt, neu, Grund (falls Kollision)
//! ```
//!
//! [`Vorschau::auszufuehren`] ist die eine Auskunft darueber, **was die
//! Ausfuehrung anfasst**: jede Zeile ohne Kollision, deren Name sich aendert.
//! Ein Eintrag mit Kollision bleibt stehen, statt den Stapel abzubrechen; das
//! ist dieselbe Haltung, die C4 fuer die Operationsmaschine festhaelt ("eine
//! gescheiterte Einzelposition bricht den Stapel nicht ab").

use super::kollision::{self, Kollision};
use super::regel::Regel;

/// Eine Zeile der Vorschau: ein markierter Eintrag mit seinem neuen Namen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vorschauzeile {
    /// Wie der Eintrag heisst.
    pub alt: String,
    /// Wie er nach der Regel hiesse.
    pub neu: String,
    /// Warum er so nicht heissen kann, falls es einen Grund gibt.
    pub kollision: Option<Kollision>,
}

impl Vorschauzeile {
    /// Ob diese Zeile bei der Ausfuehrung angefasst wird.
    ///
    /// Eine Kollision haelt sie zurueck, und ein unveraenderter Name gibt
    /// nichts zu tun.
    ///
    /// **Das ist nicht dieselbe Frage wie `Regel::ist_wirkungslos`**
    /// (`super::regel`), die fuer die ganze Regel fragt, ob sie ueberhaupt
    /// etwas aendert. Jene rechnet allein aus den Feldern der Regel und
    /// braucht keine einzige Datei; diese hier rechnet aus dem Ergebnis und
    /// braucht die markierten Namen und den Bestand des Ordners. Die
    /// Implikation traegt nur von dort nach hier: eine wirkungslose Regel
    /// laesst jeden Namen gleich, also wird keine Zeile umbenannt. Von hier
    /// nach dort traegt sie nicht — enthaelt keiner der markierten Namen den
    /// Suchtext, ist `wird_umbenannt` auf jeder Zeile falsch und die Regel
    /// gleichwohl nicht wirkungslos; dasselbe, wenn saemtliche Zeilen eine
    /// Kollision tragen. Wer von "keine Zeile wird angefasst" auf eine
    /// wirkungslose Regel schliesst, schliesst deshalb falsch. So am 260912
    /// entschieden
    /// (`shared/decisions/260912-1335_*_bleibt-die-frage-je-regel-neben-der-frage-je-zeile-bestehen.md`).
    #[must_use]
    pub fn wird_umbenannt(&self) -> bool {
        self.kollision.is_none() && self.neu != self.alt
    }
}

/// Was die Regel aus den markierten Eintraegen machen wuerde.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Vorschau {
    zeilen: Vec<Vorschauzeile>,
}

impl Vorschau {
    /// Alle Zeilen, in Sichtreihenfolge.
    #[must_use]
    pub fn zeilen(&self) -> &[Vorschauzeile] {
        &self.zeilen
    }

    /// Wie viele Eintraege einen Grund tragen, der sie zurueckhaelt.
    #[must_use]
    pub fn kollisionen(&self) -> usize {
        self.zeilen
            .iter()
            .filter(|zeile| zeile.kollision.is_some())
            .count()
    }

    /// Die Zeilen, die die Ausfuehrung anfasst, in Sichtreihenfolge.
    #[must_use = "die Auskunft darueber, was die Ausfuehrung anfasst, steht \
                  nur hier; ein fallen gelassener Iterator laeuft nie"]
    pub fn auszufuehren(&self) -> impl Iterator<Item = &Vorschauzeile> {
        self.zeilen.iter().filter(|zeile| zeile.wird_umbenannt())
    }
}

/// Rechnet die Vorschau aus.
///
/// `markierte` sind die Namen der markierten Eintraege in Sichtreihenfolge;
/// sie bestimmen zugleich die Reihenfolge der fortlaufenden Nummer. `bestand`
/// sind alle Namen des Ordners, auch die ausgeblendeten.
#[must_use]
pub fn vorschau(regel: &Regel, markierte: &[String], bestand: &[String]) -> Vorschau {
    let neue: Vec<String> = markierte
        .iter()
        .enumerate()
        .map(|(lauf, name)| regel.anwenden(name, lauf as u32))
        .collect();
    let gruende = kollision::pruefen(markierte, &neue, bestand);

    let zeilen = markierte
        .iter()
        .zip(neue)
        .zip(gruende)
        .map(|((alt, neu), kollision)| Vorschauzeile {
            alt: alt.clone(),
            neu,
            kollision,
        })
        .collect();
    Vorschau { zeilen }
}

#[cfg(test)]
mod tests {
    use super::super::regel::Nummerierung;
    use super::*;

    fn namen(liste: &[&str]) -> Vec<String> {
        liste.iter().map(|name| (*name).to_owned()).collect()
    }

    #[test]
    fn die_nummer_zaehlt_in_sichtreihenfolge_hoch() {
        let regel = Regel {
            suchen: "foto".to_owned(),
            ersetzen: "Urlaub".to_owned(),
            nummerierung: Some(Nummerierung::neu(1, 2)),
        };
        let markierte = namen(&["fotoA.jpg", "fotoB.jpg", "fotoC.jpg"]);
        let ergebnis = vorschau(&regel, &markierte, &markierte);
        let neue: Vec<&str> = ergebnis
            .zeilen()
            .iter()
            .map(|zeile| zeile.neu.as_str())
            .collect();
        assert_eq!(neue, ["UrlaubA01.jpg", "UrlaubB02.jpg", "UrlaubC03.jpg"]);
        assert_eq!(ergebnis.kollisionen(), 0);
    }

    #[test]
    fn eine_wirkungslose_regel_faesst_nichts_an() {
        let markierte = namen(&["a.txt", "b.txt"]);
        let ergebnis = vorschau(&Regel::default(), &markierte, &markierte);
        assert_eq!(ergebnis.auszufuehren().count(), 0);
        assert_eq!(ergebnis.kollisionen(), 0);
    }
}
