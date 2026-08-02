//! Das Ordnermodell: Daten und Sicht getrennt.
//!
//! `eintraege` haelt die gelesenen Daten in Lesereihenfolge und aendert sich
//! nach dem Lesen nicht mehr. `sichtreihenfolge` haelt Indizes in diese Liste
//! und bildet die aktuelle Sortierung samt Filter ab. Umsortieren heisst: die
//! Indexliste neu ordnen, nicht die Eintraege verschieben.
//!
//! Das zahlt auf zwei Zusagen. Das Umschalten der Sortierung bewegt keine
//! Nutzdaten, und die Auswahl des Nutzers bleibt ueber einen Sortierwechsel
//! hinweg stabil, weil sie am Eintragsindex haengt und nicht an der
//! Zeilennummer.

use super::eintrag::Eintrag;
use super::sortierung::{Richtung, Schluessel, Sortierung};

/// Der Inhalt eines Ordners, wie ihn ein Dateifenster anzeigt.
#[derive(Debug)]
pub struct Ordnermodell {
    eintraege: Vec<Eintrag>,
    sichtreihenfolge: Vec<u32>,
    sortierung: Sortierung,
    verstecke_ausblenden: bool,
    generation: u64,
}

impl Ordnermodell {
    /// Ein leeres Modell fuer die genannte Generation.
    pub fn neu(generation: u64) -> Self {
        Self {
            eintraege: Vec::new(),
            sichtreihenfolge: Vec::new(),
            sortierung: Sortierung::default(),
            verstecke_ausblenden: true,
            generation,
        }
    }

    /// Die Generation, zu der dieses Modell gehoert.
    ///
    /// Der Hauptfaden verwirft jeden Stapel, dessen Generation nicht mit dieser
    /// uebereinstimmt.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Wahr, wenn der Stapel zu diesem Modell gehoert.
    pub fn gehoert_dazu(&self, generation: u64) -> bool {
        generation == self.generation
    }

    /// Leert das Modell und setzt es auf eine neue Generation.
    pub fn leeren(&mut self, generation: u64) {
        self.eintraege.clear();
        self.sichtreihenfolge.clear();
        self.generation = generation;
    }

    /// Haengt einen gelesenen Stapel an.
    ///
    /// Die neuen Eintraege stehen zunaechst in Lesereihenfolge am Ende der
    /// Sicht. Das ist Absicht: der erste Stapel soll sofort sichtbar sein
    /// (L2), und ein vollstaendiges Sortieren je Stapel waere bei 100.000
    /// Eintraegen hundertmal dieselbe Arbeit. Die Reihenfolge steht mit
    /// [`Ordnermodell::abschliessen`].
    pub fn anhaengen(&mut self, neue: impl IntoIterator<Item = Eintrag>) {
        for eintrag in neue {
            let index = self.eintraege.len() as u32;
            let sichtbar = !(self.verstecke_ausblenden && eintrag.versteckt);
            self.eintraege.push(eintrag);
            if sichtbar {
                self.sichtreihenfolge.push(index);
            }
        }
    }

    /// Stellt die endgueltige Reihenfolge her.
    ///
    /// Ruft der Hauptfaden, sobald der Leser seinen Abschluss gemeldet hat,
    /// gleich ob vollstaendig oder abgebrochen.
    pub fn abschliessen(&mut self) {
        self.sicht_neu_aufbauen();
    }

    /// Die aktuelle Sortierung.
    pub fn sortierung(&self) -> Sortierung {
        self.sortierung
    }

    /// Setzt die Sortierung und ordnet die Sicht neu.
    pub fn sortierung_setzen(&mut self, sortierung: Sortierung) {
        self.sortierung = sortierung;
        self.sicht_neu_aufbauen();
    }

    /// Schaltet die Richtung um, wenn derselbe Schluessel erneut gewaehlt wird,
    /// und wechselt sonst auf den neuen Schluessel aufsteigend.
    pub fn nach_schluessel_sortieren(&mut self, schluessel: Schluessel) {
        let richtung = if self.sortierung.schluessel == schluessel {
            self.sortierung.richtung.umgekehrt()
        } else {
            Richtung::Aufsteigend
        };
        self.sortierung_setzen(Sortierung::neu(schluessel, richtung));
    }

    /// Wahr, wenn versteckte Eintraege ausgeblendet sind.
    pub fn verstecke_ausgeblendet(&self) -> bool {
        self.verstecke_ausblenden
    }

    /// Blendet versteckte Eintraege aus oder ein und baut die Sicht neu auf.
    pub fn verstecke_ausblenden_setzen(&mut self, ausblenden: bool) {
        self.verstecke_ausblenden = ausblenden;
        self.sicht_neu_aufbauen();
    }

    /// Kehrt die Sichtbarkeit versteckter Eintraege um.
    pub fn verstecke_umschalten(&mut self) {
        self.verstecke_ausblenden_setzen(!self.verstecke_ausblenden);
    }

    /// Alle gelesenen Eintraege in Lesereihenfolge, auch die ausgeblendeten.
    pub fn eintraege(&self) -> &[Eintrag] {
        &self.eintraege
    }

    /// Die Sichtreihenfolge als Indizes in [`Ordnermodell::eintraege`].
    pub fn sichtreihenfolge(&self) -> &[u32] {
        &self.sichtreihenfolge
    }

    /// Die Zahl der angezeigten Zeilen.
    pub fn zeilenzahl(&self) -> usize {
        self.sichtreihenfolge.len()
    }

    /// Der Eintrag in der genannten Zeile.
    pub fn zeile(&self, zeile: usize) -> Option<&Eintrag> {
        let index = *self.sichtreihenfolge.get(zeile)? as usize;
        self.eintraege.get(index)
    }

    /// Der Eintragsindex zur genannten Zeile.
    ///
    /// Die Auswahl haengt an diesem Index und nicht an der Zeilennummer; nur
    /// deshalb ueberlebt sie einen Sortierwechsel.
    pub fn eintragsindex(&self, zeile: usize) -> Option<u32> {
        self.sichtreihenfolge.get(zeile).copied()
    }

    /// Die Zeile, in der der genannte Eintrag steht, falls er sichtbar ist.
    pub fn zeile_von(&self, eintragsindex: u32) -> Option<usize> {
        self.sichtreihenfolge
            .iter()
            .position(|index| *index == eintragsindex)
    }

    /// Alle sichtbaren Eintraege in Sichtreihenfolge.
    pub fn zeilen(&self) -> impl Iterator<Item = &Eintrag> {
        self.sichtreihenfolge
            .iter()
            .filter_map(|index| self.eintraege.get(*index as usize))
    }

    /// Filtert und sortiert die Sicht von Grund auf neu.
    fn sicht_neu_aufbauen(&mut self) {
        let ausblenden = self.verstecke_ausblenden;
        let sortierung = self.sortierung;
        let eintraege = &self.eintraege;
        self.sichtreihenfolge.clear();
        self.sichtreihenfolge.extend(
            eintraege
                .iter()
                .enumerate()
                .filter(|(_, eintrag)| !(ausblenden && eintrag.versteckt))
                .map(|(index, _)| index as u32),
        );
        self.sichtreihenfolge.sort_unstable_by(|links, rechts| {
            sortierung.vergleiche(&eintraege[*links as usize], &eintraege[*rechts as usize])
        });
    }
}
