//! Die Lesezeichen aus C5 und die Textmarken aus C6: frei benannte Verweise
//! auf einen Ordner oder auf eine Zeile in einer Datei.
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
//! # Eine Liste mit zwei Sorten und keine zweite Liste
//!
//! Derselbe Grund, aus dem die Ordnung kein eigenes Feld bekommt, gilt fuer den
//! Bestand: zwei Listen waeren zwei Wahrheiten. C6 sagt zu, dass beide Sorten
//! in **einer** Datei und **einer** Ordnung stehen, und die Sorte ist deshalb
//! eine Eigenschaft des einzelnen Eintrags ([`Ziel`]) und keine Eigenschaft der
//! Liste. [`Lesezeichenliste`] kennt sie nicht: `anlegen`, `umbenennen`,
//! `loeschen` und `verschieben` schieben Eintraege in einem `Vec` und fragen an
//! keiner Stelle nach der Sorte. Damit wirken die vier Befehle aus C5 auf eine
//! Textmarke wie auf eine Ordnermarke, ohne dass dafuer etwas gebaut wurde.
//!
//! # Gueltig heisst: das Ziel steht noch da
//!
//! C5 sagt zu, dass ein Lesezeichen auf einen verschwundenen Ordner als
//! ungueltig markiert ist und bei der Auswahl den Grund nennt, statt
//! kommentarlos nichts zu tun. Die Regel dafuer ist [`Lesezeichen::gueltig`],
//! und sie steht hier und nicht in der Leiste: sie ist eine Aussage ueber das
//! Lesezeichen und ohne Fenster pruefbar. **Wann** gefragt wird, entscheidet
//! die Leiste — bei jedem Neuaufbau ihrer Liste und nach jedem Ein- und
//! Aushaengen eines Datentraegers, nicht bei jedem Zeichendurchgang.
//!
//! Fuer eine Textmarke heisst ungueltig **allein, dass die Datei fehlt**. Ob
//! der gemerkte Zeileninhalt noch auf der gemerkten Nummer steht, entscheidet
//! sich erst beim Sprung und nur dort. Das ist keine Sparsamkeit, sondern der
//! tragende Grund der Antwort vom 260808-0017
//! (`decisions/260807-2147_*_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`):
//! die Leiste stellt diese Frage bei jedem Neuaufbau ihrer Liste fuer jede
//! Marke, und eine Antwort, die dafuer jede gemerkte Datei oeffnen und lesen
//! muesste, machte aus einer Frage an das Dateisystem einen Lesevorgang je
//! Marke. [`Lesezeichen::gueltig`] stellt deshalb in beiden Faellen genau eine
//! Frage und liest keine Datei.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Ein Lesezeichen: ein Name und das Ziel, auf das er zeigt.
///
/// `#[serde(default)]` steht hier aus demselben Grund, aus dem jede Struktur in
/// [`super::sitzung`] ihn traegt: ein Feld, das eine spaetere Runde
/// hinzufuegt, macht eine aeltere `bookmarks.toml` nicht ungueltig, sondern
/// nimmt seinen Auslieferungswert an. Bis zu dieser Runde war `Lesezeichen` die
/// einzige serde-Struktur der Ablage ohne diese Vorsorge.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Lesezeichen {
    /// Der Name, den der Nutzer vergeben hat.
    pub name: String,
    /// Worauf der Name zeigt: ein Ordner oder eine Stelle in einer Datei.
    ///
    /// Eingebettet und nicht geschachtelt: in `bookmarks.toml` stehen die
    /// Felder der gewaehlten Sorte unmittelbar neben `name`, damit die Datei
    /// von Hand lesbar bleibt.
    #[serde(flatten)]
    pub ziel: Ziel,
}

/// Worauf ein Lesezeichen zeigt: die beiden Sorten aus C5 und C6.
///
/// Eine **unmarkierte** Auswahl: in `bookmarks.toml` steht keine Sortenkennung,
/// sondern allein das Feld `ordner` oder das Feldtrio `datei`, `zeile` und
/// `zeileninhalt`. Drei Eigenschaften machen diese Form zur richtigen, und alle
/// drei sind Zusagen und keine Bequemlichkeit:
///
/// - **Eine bestehende Datei bleibt gueltig.** Ein Eintrag mit `name` und
///   `ordner` trifft [`Ziel::Ordner`] und wird unverandert gelesen. Das ist das
///   dreizehnte Abnahmekriterium von C6.
/// - **Die Sorte ist eine Eigenschaft des Typs und keine Pruefung zur
///   Laufzeit.** Genau eine der beiden Sorten liegt vor, nie beide und nie
///   keine; mit zwei wahlfreien Feldern nebeneinander waere das eine Regel, an
///   die sich jemand halten muesste. Es gibt deshalb keinen Konstruktor, der
///   beide Sorten zugleich annimmt.
/// - **Die Datei bleibt von Hand lesbar**, wie C7 und C11 der Runde 1 es fuer
///   alle vier Ablagedateien zusagen: keine Sortenkennung, kein
///   `typ = "textstelle"`, keine geschachtelte Tabelle.
///
/// # Der Vorbehalt zu `flatten` und der Ausweg dazu
///
/// `#[serde(flatten)]` zwingt den Deserialisierer ueber einen
/// zwischenspeichernden Weg, und ob `toml` die Verbindung aus `flatten` und
/// `untagged` traegt, war am Papier nicht zu entscheiden. Die Abnahme ist
/// deshalb eine Rundreise durch beide Sorten
/// (`ablage.rs::eine_rundreise_ueber_beide_sorten_liefert_dieselbe_datei`).
/// Sollte sie eines Tages fallen, ist der Ausweg benannt und nicht zu suchen:
/// `Lesezeichen` wird selbst zur unmarkierten Auswahl mit zwei
/// Strukturvarianten, die beide ein Feld `name` tragen, und `flatten` entfaellt.
/// Der Preis dafuer ist, dass `name` von einem Feld zu einer Methode wird und
/// die Leserstellen mitziehen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ziel {
    /// Ein Ordner, den die Auswahl im aktiven Dateifenster oeffnet (C5).
    Ordner {
        /// Der Ordner.
        ordner: PathBuf,
    },
    /// Eine Stelle in einer Datei, auf die die Auswahl den Editor setzt (C6).
    ///
    /// Eine **Stelle** und kein Bereich, festgelegt am 260808-0017
    /// (`decisions/260807-2147_*_traegt-eine-textmarke-auch-einen-bereich-oder-nur-eine-stelle.md`).
    /// Der tragende Grund war nicht der Aufwand, sondern eine unbeantwortete
    /// Folgefrage: ein Bereich hat zwei Anker, und was gilt, wenn nach einer
    /// Aenderung von aussen nur einer wiedergefunden wird, ist zu entscheiden
    /// und nicht abzuleiten.
    Textstelle {
        /// Die Datei, die der Editor oeffnet.
        datei: PathBuf,
        /// Die gemerkte Zeilennummer, von 1 an gezaehlt.
        zeile: u32,
        /// Der Textinhalt jener Zeile, als Probe beim Sprung.
        ///
        /// **Keine eindeutige Kennung.** Eine Marke auf einer Zeile, die in der
        /// Datei mehrfach steht, etwa auf einer schliessenden Klammer oder
        /// einer Leerzeile, ist nach einer Aenderung von aussen nicht
        /// zuverlaessig wiederzufinden. Das ist eine Grenze der gewaehlten
        /// Regel und keine Luecke der Umsetzung; der Spec haelt sie in C6
        /// ausdruecklich fest.
        zeileninhalt: String,
    },
}

impl Default for Ziel {
    /// Der Auslieferungswert einer Lesezeichensorte: ein Ordner ohne Pfad.
    ///
    /// Er steht nie in einer geschriebenen Datei; er ist der Wert, den
    /// `#[serde(default)]` auf [`Lesezeichen`] braucht.
    fn default() -> Self {
        Ziel::Ordner {
            ordner: PathBuf::new(),
        }
    }
}

impl Lesezeichen {
    /// Ein Lesezeichen auf einen Ordner (C5).
    pub fn neu(name: impl Into<String>, ordner: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            ziel: Ziel::Ordner {
                ordner: ordner.into(),
            },
        }
    }

    /// Ein Lesezeichen auf eine Stelle in einer Datei (C6).
    ///
    /// Kein Gegenstueck zu [`Lesezeichen::neu`], das beides annimmt: die Sorte
    /// ist eine Eigenschaft des Typs, siehe [`Ziel`].
    pub fn textstelle(
        name: impl Into<String>,
        datei: impl Into<PathBuf>,
        zeile: u32,
        zeileninhalt: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            ziel: Ziel::Textstelle {
                datei: datei.into(),
                zeile,
                zeileninhalt: zeileninhalt.into(),
            },
        }
    }

    /// Ob das Ziel, auf das es zeigt, noch da ist (C5, C6).
    ///
    /// Beide Zweige stellen **genau eine Frage an das Dateisystem und lesen
    /// keine Datei**; das elfte Abnahmekriterium von C6 verlangt es, und der
    /// tragende Grund steht im Modulkopf unter "Gueltig heisst: das Ziel steht
    /// noch da".
    ///
    /// Gefragt wird nach der Art des Eintrags und nicht nach irgendeinem: ein
    /// Ordner-Lesezeichen, an dessen Stelle inzwischen eine Datei liegt, laesst
    /// sich so wenig oeffnen wie eines auf nichts, und eine Textmarke, an deren
    /// Stelle ein Ordner steht, ebenso wenig.
    ///
    /// Was hier **nicht** gefragt wird: ob der gemerkte Zeileninhalt noch auf
    /// der gemerkten Nummer steht. Eine Marke, deren Zeile sich geaendert hat
    /// oder gar nicht mehr auffindbar ist, bleibt gueltig; das entscheidet sich
    /// beim Sprung und nur dort.
    pub fn gueltig(&self) -> bool {
        match &self.ziel {
            Ziel::Ordner { ordner } => ordner.is_dir(),
            Ziel::Textstelle { datei, .. } => datei.is_file(),
        }
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

    /// Haengt ein Lesezeichen unten an und liefert seine Stelle (C5, C6).
    ///
    /// Unten und nicht oben: die Reihenfolge gehoert dem Nutzer, und ein neuer
    /// Eintrag, der sich vor seine gesetzten schiebt, nimmt ihm die Ordnung ab,
    /// die er mit `lesezeichen_hoch` und `lesezeichen_runter` hergestellt hat.
    /// Der Name kommt getrimmt herein; gepruefte Namen liefert
    /// [`name_pruefen`].
    ///
    /// **Eine Tuer fuer beide Sorten.** Die Liste fragt nicht nach der Sorte,
    /// sie nimmt das fertige [`Ziel`] entgegen und haengt an. Ein zweiter
    /// Anlegeweg fuer Textmarken daneben waere der zweite Mechanismus fuer
    /// dieselbe Aufgabe, und die eine Liste mit zwei Sorten haette zwei Tueren.
    pub fn anlegen(&mut self, name: &str, ziel: Ziel) -> usize {
        self.eintraege.push(Lesezeichen {
            name: name.trim().to_owned(),
            ziel,
        });
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

    /// Ein Ziel aus einem Pfad, damit die Proben unten kurz bleiben.
    fn ordner(pfad: &str) -> Ziel {
        Ziel::Ordner {
            ordner: PathBuf::from(pfad),
        }
    }

    #[test]
    fn ein_neues_lesezeichen_haengt_unten_an() {
        let mut liste = liste();
        assert_eq!(liste.anlegen("  Vier  ", ordner("/vier")), 3);
        assert_eq!(namen(&liste), ["Eins", "Zwei", "Drei", "Vier"]);
    }

    /// Anlegen fragt nicht nach der Sorte: beide gehen durch dieselbe Tuer und
    /// landen in derselben Liste, in der Reihenfolge des Anlegens (C6).
    #[test]
    fn beide_sorten_gehen_durch_dieselbe_tuer_und_in_dieselbe_ordnung() {
        let mut liste = Lesezeichenliste::default();
        assert_eq!(liste.anlegen("Projekte", ordner("/p")), 0);
        assert_eq!(
            liste.anlegen(
                "Die Lesestelle",
                Ziel::Textstelle {
                    datei: PathBuf::from("/p/leser.rs"),
                    zeile: 118,
                    zeileninhalt: "    let mut puffer = vec![];".to_owned(),
                },
            ),
            1
        );
        assert_eq!(liste.anlegen("Sicherung", ordner("/s")), 2);
        assert_eq!(namen(&liste), ["Projekte", "Die Lesestelle", "Sicherung"]);
    }

    /// Die drei uebrigen Listenaenderungen wirken auf eine Textmarke wie auf
    /// eine Ordnermarke — das vierte Abnahmekriterium von C6, ohne eigenen Bau.
    #[test]
    fn umbenennen_loeschen_und_verschieben_sind_sortenblind() {
        let mut liste = Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Eins", "/eins"),
            Lesezeichen::textstelle("Zwei", "/zwei.rs", 7, "fn zwei() {}"),
            Lesezeichen::textstelle("Drei", "/drei.rs", 9, "fn drei() {}"),
        ]);
        assert!(liste.umbenennen(1, "Zweite Stelle"));
        assert_eq!(liste.verschieben(2, Verschiebung::Hoch), Some(1));
        assert_eq!(namen(&liste), ["Eins", "Drei", "Zweite Stelle"]);
        assert!(liste.loeschen(2));
        assert_eq!(namen(&liste), ["Eins", "Drei"]);
    }

    /// Ein Lesezeichen traegt genau eine Sorte, nie beide und nie keine.
    ///
    /// Der Typ erzwingt es: [`Ziel`] hat zwei Werte, und es gibt **keinen
    /// Konstruktor**, der Ordner und Textstelle zugleich annimmt. Die
    /// Fallunterscheidung unten ist deshalb vollstaendig und hat keinen
    /// Auffangzweig; ein dritter Wert hielte den Bau an.
    #[test]
    fn ein_lesezeichen_traegt_genau_eine_sorte() {
        let marken = [
            (Lesezeichen::neu("Projekte", "/p"), true),
            (
                Lesezeichen::textstelle("Stelle", "/p/leser.rs", 118, "let x = 1;"),
                false,
            ),
        ];
        for (marke, ist_ordner) in &marken {
            match &marke.ziel {
                Ziel::Ordner { .. } => assert!(*ist_ordner, "{} ist keine Ordnermarke", marke.name),
                Ziel::Textstelle { .. } => {
                    assert!(!*ist_ordner, "{} ist keine Textmarke", marke.name)
                }
            }
        }
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
