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
//!
//! **Die Auswahl wohnt deshalb hier und nicht in der Tabelle der Oberflaeche.**
//! Das Modell fuehrt sie als Eintragsindex; die Oberflaeche fragt vor jedem
//! Zeichendurchgang mit [`Ordnermodell::auswahl_zeile`] nach der Zeile, in der
//! der ausgewaehlte Eintrag gerade steht. Laege sie als Zeilennummer in der
//! `NSTableView`, zeigte dieselbe Nummer nach jedem [`Ordnermodell::abschliessen`]
//! und nach jedem [`Ordnermodell::sortierung_setzen`] auf einen anderen Eintrag.
//!
//! **Die Markierung aus C2 wohnt aus demselben Grund hier.** Sie ist etwas
//! anderes als die Auswahl: die Auswahl ist der eine Eintrag unter dem
//! Cursor, die Markierung eine Menge von Eintraegen, auf die eine
//! Dateioperation gleich wirken soll. Auch sie haengt am Eintragsindex und
//! ueberlebt damit jedes Umsortieren und jedes Ein- und Ausblenden der
//! versteckten Eintraege.

use super::eintrag::Eintrag;
use super::sortierung::{Richtung, Schluessel, Sortierung};

/// Was gerade markiert ist, in einem Durchlauf gezaehlt.
///
/// Drei Werte und keine drei Methoden: die Statuszeile zeigt sie zusammen, und
/// drei Methoden waeren drei Durchlaeufe ueber dieselbe Liste und drei
/// Gelegenheiten, unterschiedlich zu filtern.
///
/// **Die Groessensumme zaehlt allein Dateien.** [`Eintrag::groesse`] ist fuer
/// einen Ordner ohne Aussage, und sie zu ermitteln hiesse, ihn zu durchlaufen;
/// genau diesen Vorabdurchlauf schliesst der Plan aus. Dieselbe Trennung zieht
/// die Groessenspalte des Dateifensters, die bei einem Ordner `--` zeigt.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Markierungsstand {
    /// Wie viele Eintraege markiert sind, Ordner eingerechnet.
    pub zahl: usize,
    /// Wie viele der markierten Eintraege Ordner sind.
    pub ordner: usize,
    /// Die Summe der Groessen der markierten **Dateien**, in Bytes.
    pub groesse: u64,
}

impl Markierungsstand {
    /// Ob ueberhaupt etwas markiert ist.
    pub fn ist_leer(&self) -> bool {
        self.zahl == 0
    }
}

/// Der Inhalt eines Ordners, wie ihn ein Dateifenster anzeigt.
#[derive(Debug)]
pub struct Ordnermodell {
    eintraege: Vec<Eintrag>,
    sichtreihenfolge: Vec<u32>,
    sortierung: Sortierung,
    verstecke_ausblenden: bool,
    generation: u64,
    /// Der ausgewaehlte Eintrag, als Index in `eintraege`.
    ///
    /// Ein Index und keine Zeilennummer: `sichtreihenfolge` wird bei jedem
    /// Sortierwechsel neu gebaut, `eintraege` nicht.
    auswahl: Option<u32>,
    /// Die Markierung aus C2, ein Wahrheitswert je Eintrag.
    ///
    /// Parallel zu `eintraege` und nicht zur Sichtreihenfolge, aus demselben
    /// Grund wie die Auswahl. Eine Liste statt einer Menge, weil "alle
    /// markieren" bei 100.000 Eintraegen sonst 100.000 Einfuegungen waere.
    markiert: Vec<bool>,
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
            auswahl: None,
            markiert: Vec::new(),
        }
    }

    /// Die Generation, zu der dieses Modell gehoert.
    ///
    /// Sie sagt, aus welchem Lesevorgang der Inhalt stammt. Die Oberflaeche
    /// prueft sie **nicht** je Stapel: sie haelt immer nur einen Lesevorgang
    /// und liest allein aus dessen Kanal. Der Modulkopf von
    /// `krk-ui/src/appkit/tabelle.rs` schreibt aus, was einen Ordnerwechsel
    /// mitten im Lesen stattdessen traegt.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Wahr, wenn der Stapel zu diesem Modell gehoert.
    pub fn gehoert_dazu(&self, generation: u64) -> bool {
        generation == self.generation
    }

    /// Leert das Modell und setzt es auf eine neue Generation.
    ///
    /// Auswahl und Markierung fallen mit: beide zeigen auf Eintraege des alten
    /// Ordners, und im neuen gibt es sie nicht.
    pub fn leeren(&mut self, generation: u64) {
        self.eintraege.clear();
        self.sichtreihenfolge.clear();
        self.markiert.clear();
        self.generation = generation;
        self.auswahl = None;
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
            self.markiert.push(false);
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

    /// Der ausgewaehlte Eintrag, als Index in [`Ordnermodell::eintraege`].
    ///
    /// Die Oberflaeche braucht ihn, wenn sie die Tabelle gleich neu laden
    /// laesst: waehrend des Neuladens gibt es keine tragfaehige Zeilennummer.
    pub fn auswahl(&self) -> Option<u32> {
        self.auswahl
    }

    /// Setzt die Auswahl auf den genannten Eintrag oder hebt sie auf.
    ///
    /// Genommen wird der Eintragsindex und nicht die Zeile. Wer eine Zeile
    /// hat, rechnet sie mit [`Ordnermodell::eintragsindex`] um; genau diese
    /// eine Umrechnung ist der Grund, aus dem die Auswahl ein Umsortieren
    /// uebersteht.
    pub fn auswahl_setzen(&mut self, eintragsindex: Option<u32>) {
        self.auswahl = eintragsindex;
    }

    /// Die Zeile, in der der ausgewaehlte Eintrag gerade steht.
    ///
    /// `None`, wenn nichts ausgewaehlt ist oder der ausgewaehlte Eintrag
    /// gerade ausgeblendet ist. Im zweiten Fall bleibt der gemerkte Eintrag
    /// stehen: blendet der Nutzer die versteckten Eintraege wieder ein, ist
    /// seine Auswahl wieder da, statt beim Umschalten verloren zu gehen.
    pub fn auswahl_zeile(&self) -> Option<usize> {
        self.zeile_von(self.auswahl?)
    }

    /// Der Eintragsindex des Eintrags mit diesem Namen.
    ///
    /// Der Weg vom Namen zum Eintrag, den der Sprung aus C10 braucht: die
    /// Zwischenablage nennt eine Datei, und gesucht ist ihre Zeile. Auch die
    /// ausgeblendeten Eintraege zaehlen; ob der gefundene sichtbar ist, sagt
    /// danach [`Ordnermodell::zeile_von`].
    pub fn index_von_namen(&self, name: &str) -> Option<u32> {
        self.eintraege
            .iter()
            .position(|eintrag| eintrag.name == name)
            .map(|index| index as u32)
    }

    // ------------------------------------------------------------------
    // Die Markierung aus C2
    // ------------------------------------------------------------------

    /// Ob dieser Eintrag markiert ist.
    pub fn ist_markiert(&self, eintragsindex: u32) -> bool {
        self.markiert
            .get(eintragsindex as usize)
            .copied()
            .unwrap_or(false)
    }

    /// Was markiert ist: Zahl, Ordnerzahl und Groessensumme.
    ///
    /// Ueber alle gelesenen Eintraege, auch die gerade ausgeblendeten: eine
    /// Markierung, die der Nutzer nicht sieht, besteht trotzdem.
    ///
    /// Ein Durchlauf fuer alle drei Werte, und deshalb eine Struktur statt
    /// dreier Methoden; die Begruendung steht bei [`Markierungsstand`].
    pub fn markierungsstand(&self) -> Markierungsstand {
        let mut stand = Markierungsstand::default();
        for (index, markiert) in self.markiert.iter().enumerate() {
            if !*markiert {
                continue;
            }
            let Some(eintrag) = self.eintraege.get(index) else {
                continue;
            };
            stand.zahl += 1;
            if eintrag.ist_ordner() {
                stand.ordner += 1;
            } else {
                stand.groesse = stand.groesse.saturating_add(eintrag.groesse);
            }
        }
        stand
    }

    /// Kehrt die Markierung eines einzelnen Eintrags um.
    ///
    /// Der erste der vier Markierungsbefehle aus C2, ohne das Weiterruecken der
    /// Auswahl: das ist Sache der Oberflaeche, die die Zeilen kennt.
    pub fn markierung_umschalten(&mut self, eintragsindex: u32) {
        if let Some(markiert) = self.markiert.get_mut(eintragsindex as usize) {
            *markiert = !*markiert;
        }
    }

    /// Markiert alle sichtbaren Eintraege (C2).
    ///
    /// Ausgeblendete bleiben unberuehrt. Sie mitzumarkieren hiesse, eine
    /// spaetere Dateioperation auf Eintraege zu richten, die der Nutzer beim
    /// Druecken der Taste nicht vor sich hatte.
    pub fn alle_markieren(&mut self) {
        for index in &self.sichtreihenfolge {
            if let Some(markiert) = self.markiert.get_mut(*index as usize) {
                *markiert = true;
            }
        }
    }

    /// Hebt jede Markierung auf (C2).
    ///
    /// Ueber alle Eintraege und nicht nur ueber die sichtbaren: "jede
    /// Markierung aufheben" heisst jede, und eine stehengebliebene, die der
    /// Nutzer nicht sieht, waere die Ueberraschung bei der naechsten Operation.
    pub fn markierung_aufheben(&mut self) {
        self.markiert.fill(false);
    }

    /// Kehrt die Markierung aller sichtbaren Eintraege um (C2).
    ///
    /// Derselbe Zuschnitt wie [`Ordnermodell::alle_markieren`] und aus
    /// demselben Grund.
    pub fn markierung_umkehren(&mut self) {
        for index in &self.sichtreihenfolge {
            if let Some(markiert) = self.markiert.get_mut(*index as usize) {
                *markiert = !*markiert;
            }
        }
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

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::super::eintrag::Typ;
    use super::*;

    /// Ein Eintrag fuer die Proben unten.
    ///
    /// Der Sortierschluessel bleibt leer, und das ist kein Versaeumnis: die
    /// beiden Eintraege jeder Probe stehen in verschiedenen Gruppen (Ordner vor
    /// Datei), also entscheidet allein die Gruppe. Den Schluessel hier
    /// nachzubauen hiesse, die Berechnung aus `Eintrag::aus_roh` ein zweites
    /// Mal zu fuehren.
    fn eintrag(name: &str, typ: Typ) -> Eintrag {
        Eintrag {
            name: name.to_owned(),
            sortierschluessel: Box::default(),
            groesse: 0,
            geaendert: SystemTime::UNIX_EPOCH,
            typ,
            versteckt: name.starts_with('.'),
        }
    }

    /// Ein Modell in Lesereihenfolge: erst die Datei, dann der Ordner.
    ///
    /// Genau diese Reihenfolge dreht [`Ordnermodell::abschliessen`] um.
    fn gelesen() -> Ordnermodell {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen([
            eintrag("zzz.txt", Typ::Datei),
            eintrag("Applications", Typ::Ordner),
        ]);
        modell
    }

    fn name_in_zeile(modell: &Ordnermodell, zeile: usize) -> Option<&str> {
        modell.zeile(zeile).map(|eintrag| eintrag.name.as_str())
    }

    fn auswaehlen(modell: &mut Ordnermodell, name: &str) -> usize {
        let zeile = modell
            .zeilen()
            .position(|eintrag| eintrag.name == name)
            .expect("der Eintrag steht nicht in der Sicht");
        let index = modell.eintragsindex(zeile);
        modell.auswahl_setzen(index);
        zeile
    }

    /// Der Fall aus dem Defekt: waehrend des Lesens ausgewaehlt, danach
    /// sortiert.
    #[test]
    fn die_auswahl_ueberlebt_das_sortieren_am_ende_des_lesevorgangs() {
        let mut modell = gelesen();
        let zeile_vorher = auswaehlen(&mut modell, "zzz.txt");

        modell.abschliessen();

        assert_ne!(
            name_in_zeile(&modell, zeile_vorher),
            Some("zzz.txt"),
            "die Probe traegt nur, wenn unter der alten Zeilennummer jetzt ein \
             anderer Eintrag steht"
        );
        let zeile_nachher = modell
            .auswahl_zeile()
            .expect("die Auswahl ist beim Sortieren verloren gegangen");
        assert_eq!(name_in_zeile(&modell, zeile_nachher), Some("zzz.txt"));
    }

    #[test]
    fn ein_neuer_ordner_hebt_die_auswahl_auf() {
        let mut modell = gelesen();
        auswaehlen(&mut modell, "zzz.txt");

        modell.leeren(2);

        assert_eq!(modell.auswahl(), None);
        assert_eq!(modell.auswahl_zeile(), None);
    }

    #[test]
    fn eine_ausgeblendete_auswahl_kommt_beim_einblenden_zurueck() {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen([
            eintrag(".versteckt.txt", Typ::Datei),
            eintrag("Applications", Typ::Ordner),
        ]);
        modell.verstecke_ausblenden_setzen(false);
        auswaehlen(&mut modell, ".versteckt.txt");

        modell.verstecke_ausblenden_setzen(true);
        assert_eq!(
            modell.auswahl_zeile(),
            None,
            "der Eintrag ist nicht sichtbar"
        );

        modell.verstecke_ausblenden_setzen(false);
        let zeile = modell
            .auswahl_zeile()
            .expect("die Auswahl ist verloren gegangen");
        assert_eq!(name_in_zeile(&modell, zeile), Some(".versteckt.txt"));
    }
}
