//! Die acht Sortierungen des Ordnermodells.
//!
//! Vier Schluessel mal zwei Richtungen. Ordner stehen immer vor Dateien, auch
//! absteigend: die Richtung dreht die Reihenfolge innerhalb der beiden Gruppen
//! um, nicht die Gruppen selbst.
//!
//! Die Sortierung nach Typ ordnet nach der **Dateiendung**. Nach der
//! Aufzaehlung Ordner/Datei/Verknuepfung zu ordnen, wie es bis zum
//! Nutzerentscheid vom 260806 geschah, taete im Alltag fast nichts: Ordner
//! stehen ohnehin vorn, und innerhalb der Dateien bliebe allein der
//! Unterschied zwischen Datei und Verknuepfung, den die meisten Ordner gar
//! nicht kennen. Der Datensatz dazu ist
//! `decisions/260802-1810_*_sortierung-ohne-sprachsensitive-kollation.md`.

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
    /// Nach der Dateiendung, ueber den vorberechneten Endungsschluessel.
    ///
    /// Die Variante heisst `Typ` und nicht `Endung`, weil ihr Name in
    /// `session.toml` steht: die Sortierung je Tab ueberlebt das Beenden der
    /// Anwendung, und eine Umbenennung liesse jede bereits geschriebene
    /// Sitzung auf die Vorbelegung zurueckfallen. In der Oberflaeche heisst
    /// die Sortierung weiterhin "nach Typ"; die Endung ist das, wonach sie
    /// ordnet.
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
    ///
    /// Jeder der vier Faelle vergleicht nur vorberechnete Werte. Die
    /// sprachsensitive Kollation laeuft beim Lesen, nicht hier; siehe
    /// [`super::kollation`].
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
            Schluessel::Typ => links
                .endungsschluessel
                .cmp(&rechts.endungsschluessel)
                .then_with(nach_name),
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
    use std::time::SystemTime;

    use super::*;

    fn datei(name: &str) -> Eintrag {
        Eintrag::neu(name.to_owned(), 0, SystemTime::UNIX_EPOCH, Typ::Datei)
    }

    fn ordner(name: &str) -> Eintrag {
        Eintrag::neu(name.to_owned(), 0, SystemTime::UNIX_EPOCH, Typ::Ordner)
    }

    /// Die Sortierung nach Typ, aufsteigend.
    fn nach_typ() -> Sortierung {
        Sortierung::neu(Schluessel::Typ, Richtung::Aufsteigend)
    }

    /// Sortiert die Namen wie das Ordnermodell: eine Ordnung ohne Stabilitaets-
    /// zusage, damit ein Gleichstand hier auffiele und nicht von `sort` still
    /// ueberdeckt wuerde.
    fn sortiert(sortierung: Sortierung, eintraege: &[Eintrag]) -> Vec<&str> {
        let mut reihe: Vec<&Eintrag> = eintraege.iter().collect();
        reihe.sort_unstable_by(|links, rechts| sortierung.vergleiche(links, rechts));
        reihe.iter().map(|eintrag| eintrag.name.as_str()).collect()
    }

    #[test]
    fn nach_namen_stehen_umlaute_beim_grundbuchstaben() {
        let eintraege = [datei("Zebra"), datei("Äpfel"), datei("Bäume")];
        assert_eq!(
            sortiert(Sortierung::default(), &eintraege),
            ["Äpfel", "Bäume", "Zebra"]
        );
    }

    #[test]
    fn nach_typ_wird_nach_der_endung_geordnet() {
        let eintraege = [
            datei("zebra.txt"),
            datei("alpha.zip"),
            datei("beta.md"),
            datei("gamma.txt"),
        ];
        assert_eq!(
            sortiert(nach_typ(), &eintraege),
            ["beta.md", "gamma.txt", "zebra.txt", "alpha.zip"]
        );
    }

    #[test]
    fn gleiche_endung_wird_nach_dem_namen_geordnet() {
        // Der Gleichstand im Schluessel faellt auf den Namen zurueck, nicht auf
        // die Lesereihenfolge. Das ist die Zusage, die zwei Laeufe dieselbe
        // Reihenfolge ergeben laesst.
        let eintraege = [datei("zebra.txt"), datei("Äpfel.txt"), datei("beta.txt")];
        assert_eq!(
            sortiert(nach_typ(), &eintraege),
            ["Äpfel.txt", "beta.txt", "zebra.txt"]
        );
    }

    #[test]
    fn gleiche_groesse_und_gleiches_datum_fallen_auf_den_namen_zurueck() {
        // Alle drei tragen Groesse 0 und denselben Zeitpunkt.
        let eintraege = [datei("zebra"), datei("Äpfel"), datei("beta")];
        for schluessel in [Schluessel::Groesse, Schluessel::Geaendert] {
            let sortierung = Sortierung::neu(schluessel, Richtung::Aufsteigend);
            assert_eq!(
                sortiert(sortierung, &eintraege),
                ["Äpfel", "beta", "zebra"],
                "{schluessel:?}"
            );
        }
    }

    #[test]
    fn dateien_ohne_endung_stehen_vor_den_uebrigen() {
        let eintraege = [datei("bericht.txt"), datei("Makefile"), datei("LICENSE")];
        assert_eq!(
            sortiert(nach_typ(), &eintraege),
            ["LICENSE", "Makefile", "bericht.txt"]
        );
    }

    #[test]
    fn ordner_stehen_auch_in_der_sortierung_nach_endung_vorn() {
        // Auch dann, wenn ihr Name eine Endung traegt, die spaeter einsortieren
        // wuerde als die der Datei.
        let eintraege = [datei("alpha.aaa"), ordner("sicherung.zzz")];
        for richtung in Richtung::ALLE {
            assert_eq!(
                sortiert(Sortierung::neu(Schluessel::Typ, richtung), &eintraege)[0],
                "sicherung.zzz",
                "{richtung:?}"
            );
        }
    }

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
