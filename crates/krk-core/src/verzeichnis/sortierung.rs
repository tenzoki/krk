//! Die acht Sortierungen des Ordnermodells.
//!
//! Vier Schluessel mal zwei Richtungen. Ordner stehen immer vor Dateien, auch
//! absteigend: die Richtung dreht die Reihenfolge innerhalb der beiden Gruppen
//! um, nicht die Gruppen selbst.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use super::eintrag::{Eintrag, Typ};

/// Wonach sortiert wird.
///
/// Die Ableitungen von `serde` gehoeren hierher und nicht in die Ablage: der
/// Sitzungszustand aus C7 haelt die Sortierung je Tab, und eine eigene
/// Aufzaehlung daneben waere eine zweite Wahrheit darueber, wonach KRK
/// sortieren kann.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Schluessel {
    /// Nach dem Namen, ueber den vorberechneten Sortierschluessel.
    Name,
    /// Nach der Groesse der Daten.
    Groesse,
    /// Nach dem Zeitpunkt der letzten Aenderung.
    Geaendert,
    /// Nach der Art des Eintrags.
    Typ,
}

impl Schluessel {
    /// Alle vier Schluessel, fuer Tests und fuer die Belegungsansicht.
    pub const ALLE: [Schluessel; 4] = [
        Schluessel::Name,
        Schluessel::Groesse,
        Schluessel::Geaendert,
        Schluessel::Typ,
    ];
}

/// In welche Richtung sortiert wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Richtung {
    /// Aufsteigend: klein vor gross, frueh vor spaet.
    Aufsteigend,
    /// Absteigend: die umgekehrte Reihenfolge, Gruppen bleiben unberuehrt.
    Absteigend,
}

impl Richtung {
    /// Beide Richtungen.
    pub const ALLE: [Richtung; 2] = [Richtung::Aufsteigend, Richtung::Absteigend];

    /// Die jeweils andere Richtung.
    pub fn umgekehrt(self) -> Self {
        match self {
            Richtung::Aufsteigend => Richtung::Absteigend,
            Richtung::Absteigend => Richtung::Aufsteigend,
        }
    }
}

/// Eine der acht Sortierungen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct Sortierung {
    /// Wonach sortiert wird.
    pub schluessel: Schluessel,
    /// In welche Richtung.
    pub richtung: Richtung,
}

impl Default for Sortierung {
    /// Die Vorbelegung: Name aufsteigend.
    fn default() -> Self {
        Self {
            schluessel: Schluessel::Name,
            richtung: Richtung::Aufsteigend,
        }
    }
}

impl Sortierung {
    /// Baut eine Sortierung aus Schluessel und Richtung.
    pub fn neu(schluessel: Schluessel, richtung: Richtung) -> Self {
        Self {
            schluessel,
            richtung,
        }
    }

    /// Alle acht Sortierungen, in fester Reihenfolge.
    pub fn alle() -> impl Iterator<Item = Sortierung> {
        Schluessel::ALLE.into_iter().flat_map(|schluessel| {
            Richtung::ALLE.map(|richtung| Sortierung::neu(schluessel, richtung))
        })
    }

    /// Vergleicht zwei Eintraege nach dieser Sortierung.
    ///
    /// Ordner gewinnen immer gegen Dateien. Innerhalb einer Gruppe entscheidet
    /// der Schluessel, bei Gleichstand der Name, damit die Ordnung total ist
    /// und zwei Laeufe dieselbe Reihenfolge ergeben.
    pub fn vergleiche(&self, links: &Eintrag, rechts: &Eintrag) -> Ordering {
        let gruppen = gruppe(links).cmp(&gruppe(rechts));
        if gruppen != Ordering::Equal {
            return gruppen;
        }
        let nach_name = || links.sortierschluessel.cmp(&rechts.sortierschluessel);
        let innerhalb = match self.schluessel {
            Schluessel::Name => nach_name(),
            Schluessel::Groesse => links.groesse.cmp(&rechts.groesse).then_with(nach_name),
            Schluessel::Geaendert => links.geaendert.cmp(&rechts.geaendert).then_with(nach_name),
            Schluessel::Typ => links.typ.cmp(&rechts.typ).then_with(nach_name),
        };
        match self.richtung {
            Richtung::Aufsteigend => innerhalb,
            Richtung::Absteigend => innerhalb.reverse(),
        }
    }
}

/// Die Gruppe, die vor dem Schluessel entscheidet: Ordner vor allem anderen.
fn gruppe(eintrag: &Eintrag) -> u8 {
    if eintrag.typ == Typ::Ordner { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn es_gibt_genau_acht_sortierungen() {
        let alle: Vec<Sortierung> = Sortierung::alle().collect();
        assert_eq!(alle.len(), 8);
        let einmalig: std::collections::HashSet<Sortierung> = alle.into_iter().collect();
        assert_eq!(einmalig.len(), 8);
    }

    #[test]
    fn vorbelegung_ist_name_aufsteigend() {
        assert_eq!(
            Sortierung::default(),
            Sortierung::neu(Schluessel::Name, Richtung::Aufsteigend)
        );
    }
}
