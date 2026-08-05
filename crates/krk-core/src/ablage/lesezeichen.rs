//! Die Lesezeichen aus C5: frei benannte Verweise auf Ordner.
//!
//! Die Reihenfolge der Liste ist die Reihenfolge in der Leiste; ein eigenes
//! Ordnungsfeld gibt es nicht, weil zwei Ordnungen zwei Wahrheiten waeren.
//! Genau deshalb sind Anlegen, Umbenennen, Loeschen und Verschieben Aenderungen
//! **an dieser Liste** und stehen hier: sie verschieben Eintraege in einem
//! `Vec`, und ein zweites Lesezeichenmodul daneben haette denselben Bestand ein
//! zweites Mal gefuehrt.
//!
//! Was hier **nicht** steht: die Auswahl, die Ueberschriften und die Geraete
//! des unteren Leistenteils. Das ist Ansichtszustand und wohnt in
//! `krk-ui`; die Geraete kommen ohnehin vom System und werden nicht abgelegt.
//!
//! # Gueltig heisst: der Ordner steht noch da
//!
//! C5 sagt zu, dass ein Lesezeichen auf einen verschwundenen Ordner als
//! ungueltig markiert ist und bei der Auswahl den Grund nennt, statt
//! kommentarlos nichts zu tun. Die Regel dafuer ist [`Lesezeichen::gueltig`],
//! und sie steht hier und nicht in der Leiste: sie ist eine Aussage ueber das
//! Lesezeichen und ohne Fenster pruefbar. **Wann** gefragt wird, entscheidet
//! die Leiste — bei jedem Neuaufbau ihrer Liste und nach jedem Ein- und
//! Aushaengen eines Datentraegers, nicht bei jedem Zeichendurchgang.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Ein Lesezeichen: ein Name und der Ordner, auf den er zeigt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lesezeichen {
    /// Der Name, den der Nutzer vergeben hat.
    pub name: String,
    /// Der Ordner, den die Auswahl im aktiven Dateifenster oeffnet.
    pub ordner: PathBuf,
}

impl Lesezeichen {
    /// Ein Lesezeichen aus Name und Ordner.
    pub fn neu(name: impl Into<String>, ordner: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            ordner: ordner.into(),
        }
    }

    /// Ob der Ordner, auf den es zeigt, noch da ist (C5).
    ///
    /// Gefragt wird nach einem **Ordner** und nicht nach irgendeinem Eintrag:
    /// ein Lesezeichen, an dessen Stelle inzwischen eine Datei liegt, laesst
    /// sich so wenig oeffnen wie eines auf nichts.
    pub fn gueltig(&self) -> bool {
        self.ordner.is_dir()
    }
}

/// Was ein Name taugt, den der Nutzer einem Lesezeichen geben will (C5).
///
/// Die eine Regel, und sie ist bewusst duenner als die fuer einen Dateinamen
/// aus [`crate::operation::name_pruefen`]: ein Lesezeichenname ist eine
/// Beschriftung und kein Eintrag im Dateisystem. Ein Schraegstrich darin ist
/// erlaubt, weil "Projekte/2026" eine sinnvolle Beschriftung ist und kein
/// Pfad. Leer darf er nicht sein, denn eine Zeile ohne Text waere in der
/// Leiste nicht zu treffen.
pub fn name_pruefen(name: &str) -> Result<(), Namenshinweis> {
    if name.trim().is_empty() {
        return Err(Namenshinweis::Leer);
    }
    Ok(())
}

/// Warum ein Name fuer ein Lesezeichen nicht taugt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namenshinweis {
    /// Der Name ist leer oder besteht nur aus Leerzeichen.
    Leer,
}

impl Namenshinweis {
    /// Der Grund im Klartext, so wie ihn die Statuszeile zeigt.
    pub fn grund(self) -> &'static str {
        match self {
            Namenshinweis::Leer => "der Name ist leer",
        }
    }
}

/// Wohin ein Lesezeichen ruecken soll (C5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verschiebung {
    /// Einen Platz nach oben.
    Hoch,
    /// Einen Platz nach unten.
    Runter,
}

/// Alle Lesezeichen in ihrer Reihenfolge, wie sie in `bookmarks.toml` stehen.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Lesezeichenliste {
    /// Die Lesezeichen von oben nach unten.
    pub eintraege: Vec<Lesezeichen>,
}

impl Lesezeichenliste {
    /// Eine Liste aus vorhandenen Lesezeichen.
    pub fn aus(eintraege: Vec<Lesezeichen>) -> Self {
        Self { eintraege }
    }

    /// Wie viele Lesezeichen die Liste fuehrt.
    pub fn zahl(&self) -> usize {
        self.eintraege.len()
    }

    /// Das Lesezeichen an dieser Stelle.
    pub fn eintrag(&self, stelle: usize) -> Option<&Lesezeichen> {
        self.eintraege.get(stelle)
    }

    /// Haengt ein Lesezeichen unten an und liefert seine Stelle (C5).
    ///
    /// Unten und nicht oben: die Reihenfolge gehoert dem Nutzer, und ein neuer
    /// Eintrag, der sich vor seine gesetzten schiebt, nimmt ihm die Ordnung ab,
    /// die er mit `lesezeichen_hoch` und `lesezeichen_runter` hergestellt hat.
    /// Der Name kommt getrimmt herein; gepruefte Namen liefert
    /// [`name_pruefen`].
    pub fn anlegen(&mut self, name: &str, ordner: &Path) -> usize {
        self.eintraege.push(Lesezeichen::neu(name.trim(), ordner));
        self.eintraege.len() - 1
    }

    /// Benennt das Lesezeichen an dieser Stelle um (C5).
    ///
    /// Liefert, ob sich dadurch etwas geaendert hat. Eine Stelle, die es nicht
    /// gibt, ist keine Aenderung und kein Fehler: die Leiste kann eine
    /// Ueberschrift oder ein Geraet ausgewaehlt haben.
    pub fn umbenennen(&mut self, stelle: usize, name: &str) -> bool {
        let Some(eintrag) = self.eintraege.get_mut(stelle) else {
            return false;
        };
        let name = name.trim();
        if eintrag.name == name {
            return false;
        }
        eintrag.name = name.to_owned();
        true
    }

    /// Loescht das Lesezeichen an dieser Stelle (C5).
    ///
    /// Liefert, ob es eines gab. Ohne Rueckfrage: ein Lesezeichen ist ein
    /// Verweis und keine Datei, und `lesezeichen_anlegen` stellt es in einem
    /// Tastendruck wieder her.
    pub fn loeschen(&mut self, stelle: usize) -> bool {
        if stelle >= self.eintraege.len() {
            return false;
        }
        self.eintraege.remove(stelle);
        true
    }

    /// Schiebt das Lesezeichen an dieser Stelle einen Platz weiter (C5).
    ///
    /// Liefert die neue Stelle, oder `None`, wenn es dort keines gibt oder es
    /// schon am Rand steht. Am Rand geschieht nichts und wird nichts gemeldet:
    /// das ist dieselbe Antwort, die die Auswahl in der Dateiliste am Listenende
    /// gibt.
    pub fn verschieben(&mut self, stelle: usize, richtung: Verschiebung) -> Option<usize> {
        let ziel = match richtung {
            Verschiebung::Hoch => stelle.checked_sub(1)?,
            Verschiebung::Runter => stelle + 1,
        };
        if stelle >= self.eintraege.len() || ziel >= self.eintraege.len() {
            return None;
        }
        self.eintraege.swap(stelle, ziel);
        Some(ziel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn liste() -> Lesezeichenliste {
        Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Eins", "/eins"),
            Lesezeichen::neu("Zwei", "/zwei"),
            Lesezeichen::neu("Drei", "/drei"),
        ])
    }

    fn namen(liste: &Lesezeichenliste) -> Vec<&str> {
        liste
            .eintraege
            .iter()
            .map(|eintrag| eintrag.name.as_str())
            .collect()
    }

    #[test]
    fn ein_neues_lesezeichen_haengt_unten_an() {
        let mut liste = liste();
        assert_eq!(liste.anlegen("  Vier  ", Path::new("/vier")), 3);
        assert_eq!(namen(&liste), ["Eins", "Zwei", "Drei", "Vier"]);
    }

    #[test]
    fn umbenennen_trifft_die_genannte_stelle_und_sonst_keine() {
        let mut liste = liste();
        assert!(liste.umbenennen(1, "Neu"));
        assert_eq!(namen(&liste), ["Eins", "Neu", "Drei"]);
        assert!(
            !liste.umbenennen(1, "Neu"),
            "derselbe Name ist keine Aenderung"
        );
        assert!(!liste.umbenennen(9, "Weg"), "die Stelle gibt es nicht");
    }

    #[test]
    fn loeschen_nimmt_genau_einen_eintrag() {
        let mut liste = liste();
        assert!(liste.loeschen(0));
        assert_eq!(namen(&liste), ["Zwei", "Drei"]);
        assert!(!liste.loeschen(2));
    }

    #[test]
    fn verschieben_tauscht_mit_dem_nachbarn_und_haelt_am_rand_an() {
        let mut liste = liste();
        assert_eq!(liste.verschieben(2, Verschiebung::Hoch), Some(1));
        assert_eq!(namen(&liste), ["Eins", "Drei", "Zwei"]);
        assert_eq!(liste.verschieben(0, Verschiebung::Hoch), None);
        assert_eq!(liste.verschieben(2, Verschiebung::Runter), None);
        assert_eq!(namen(&liste), ["Eins", "Drei", "Zwei"]);
    }

    #[test]
    fn ein_name_ohne_zeichen_taugt_nicht_ein_schraegstrich_schon() {
        assert_eq!(name_pruefen("   "), Err(Namenshinweis::Leer));
        assert_eq!(name_pruefen("Projekte/2026"), Ok(()));
    }

    #[test]
    fn gueltig_ist_ein_lesezeichen_auf_einen_vorhandenen_ordner() {
        let ordner = std::env::temp_dir();
        assert!(Lesezeichen::neu("Temp", &ordner).gueltig());
        assert!(!Lesezeichen::neu("Weg", ordner.join("krk-gibt-es-nicht")).gueltig());
    }
}
