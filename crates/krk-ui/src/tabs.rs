//! Das Tabmodell eines Dateifensters: welche Tabs es gibt, was in ihnen steht,
//! welcher sichtbar ist und in welcher Reihenfolge gelesen wird.
//!
//! **Keine Zeile AppKit.** In dieser Datei steht keine `use objc2`-Zeile, und
//! das ist nachpruefbar, nicht nur gemeint. Sie haelt das Modell; die Ansicht
//! dazu ist [`crate::appkit::tabelle`], die den Inhalt des sichtbaren Tabs in
//! eine `NSTableView` stellt.
//!
//! # Ein Tab traegt seinen eigenen Ordnerinhalt
//!
//! Jeder Tab haelt sein eigenes [`Ordnermodell`] und seinen eigenen
//! [`Lesevorgang`]. Das ist die Bedingung fuer die Zusage aus C8, dass ein Tab
//! im Hintergrund bereitsteht, bevor der Nutzer ihn ansteuert: L5 gibt dem
//! Tabwechsel 50 ms, und ein Ordner, der erst beim Hinwechseln gelesen wird,
//! haelt das nicht. Ein Wechsel auf einen gelesenen Tab setzt deshalb nur die
//! Stelle um und stoesst keinen Lesevorgang an.
//!
//! # Die Lesereihenfolge beim Start
//!
//! ```text
//! Start ──> sichtbarer Tab jedes Dateifensters   (L4 endet hier)
//!             │
//!             └──> sobald der sichtbare Tab bedienbar ist:
//!                  die verdeckten Tabs desselben Fensters
//! ```
//!
//! [`Tabliste::nachzuegler_faellig`] beantwortet die Frage "ist der sichtbare
//! Tab bedienbar", [`Tabliste::nachzuegler_starten`] loest die zweite Stufe
//! aus. Beide Stufen laufen je Dateifenster fuer sich; eine Absprache zwischen
//! den beiden Fenstern braucht es nicht, weil die erste Stufe in beiden
//! gleichzeitig beginnt.
//!
//! Trifft ein Wechsel doch einen ungelesenen Tab, liest
//! [`Tabliste::waehlen`] ihn nach. Der Fall ist die Ausnahme, und C8 deckt ihn:
//! L5 deckt den Wechsel, die erste Bildschirmseite faellt unter L2.

use std::path::{Path, PathBuf};

use krk_core::ablage::{Dateifenster as Fensterzustand, Tab as Tabzustand};
use krk_core::verzeichnis::{Abschluss, Lesevorgang, Meldung, Ordnermodell};

/// Die Generation, mit der ein noch nicht gelesener Tab anfaengt.
const GENERATION_LEER: u64 = 0;

/// Ein Tab: sein Ordner, sein Inhalt und sein Lesevorgang.
pub struct Tabinhalt {
    ordner: PathBuf,
    modell: Ordnermodell,
    lesevorgang: Option<Lesevorgang>,
    /// Der Name, auf den die Auswahl springt, sobald der Ordner gelesen ist.
    ///
    /// Er kommt aus `session.toml` und lebt genau bis zum Abschluss des
    /// Lesevorgangs. Ein Name und keine Zeilennummer: zwischen Beenden und
    /// Neustart kann sich der Ordnerinhalt geaendert haben.
    wunschauswahl: Option<String>,
    bildlauf: f64,
    /// Ob die Ansicht die gemerkte Bildlaufposition noch herstellen muss.
    ///
    /// Sie steht in der Sitzung, die Liste dazu ist beim Start aber noch leer:
    /// eine Position in einer leeren Liste gibt es nicht. Bis die Ansicht sie
    /// hergestellt hat, uebergeht [`Tabinhalt::bildlauf_setzen`] jede Meldung
    /// aus der Ansicht, sonst schriebe der erste Sitzungsabgleich die gemerkte
    /// Position mit der Null einer leeren Liste zu.
    bildlauf_offen: bool,
    /// Was die Statuszeile fuer diesen Tab zeigt, falls etwas zu sagen ist.
    meldung: Option<String>,
    /// Ob dieser Tab schon einmal gelesen wurde.
    ///
    /// Ein leerer Ordner ist gelesen und hat trotzdem null Zeilen; ohne dieses
    /// Kennzeichen liesse er sich von einem ungelesenen nicht unterscheiden,
    /// und jeder Wechsel auf ihn stiesse einen neuen Lesevorgang an.
    gelesen: bool,
}

impl Tabinhalt {
    /// Ein ungelesener Tab aus seinem gespeicherten Zustand.
    fn aus_zustand(zustand: &Tabzustand) -> Self {
        let mut modell = Ordnermodell::neu(GENERATION_LEER);
        modell.verstecke_ausblenden_setzen(zustand.verstecke_ausgeblendet);
        modell.sortierung_setzen(zustand.sortierung);
        Self {
            ordner: zustand.ordner.clone(),
            modell,
            lesevorgang: None,
            wunschauswahl: zustand.auswahl.clone(),
            bildlauf: zustand.bildlauf,
            bildlauf_offen: zustand.bildlauf > 0.0,
            meldung: None,
            gelesen: false,
        }
    }

    /// Der Ordner, den dieser Tab zeigt.
    pub fn ordner(&self) -> &Path {
        &self.ordner
    }

    /// Der Inhalt des Ordners.
    pub fn modell(&self) -> &Ordnermodell {
        &self.modell
    }

    /// Der Inhalt des Ordners, veraenderlich.
    ///
    /// Die Ansicht setzt darueber die Auswahl, die Sortierung und den Filter
    /// fuer versteckte Eintraege.
    pub fn modell_mut(&mut self) -> &mut Ordnermodell {
        &mut self.modell
    }

    /// Ob gerade ein Lesevorgang laeuft.
    pub fn liest(&self) -> bool {
        self.lesevorgang.is_some()
    }

    /// Ob der Tab bedienbar ist: er zeigt Zeilen oder ist fertig gelesen.
    ///
    /// Das ist die Bedingung aus L4 und L2, an einem einzelnen Tab gemessen.
    fn ist_bedienbar(&self) -> bool {
        self.gelesen || self.modell.zeilenzahl() > 0
    }

    /// Die Bildlaufposition in Punkten, vom oberen Rand der Liste aus gezaehlt.
    ///
    /// **0 heisst "ganz oben".** Der rohe Ursprung der Bildlaufansicht ist dort
    /// nicht null, sondern liegt um die Hoehe der Spaltenueberschriften
    /// darueber; `crate::appkit::tabelle` rechnet ihn beim Merken heraus und
    /// beim Herstellen wieder hinein. Die Zahl steht in `session.toml`, die der
    /// Nutzer lesen und von Hand aendern koennen soll, und eine negative Zahl
    /// fuer "ganz oben" waere dort eine Stolperstelle
    /// (`issues/260804-1040_*_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28.md`).
    pub fn bildlauf(&self) -> f64 {
        self.bildlauf
    }

    /// Merkt sich die Bildlaufposition, die die Ansicht gerade zeigt.
    ///
    /// Uebergangen, solange die Ansicht die gemerkte Position noch nicht
    /// hergestellt hat: sie zeigt dann nicht den Stand des Tabs, sondern den
    /// einer noch leeren Liste.
    pub fn bildlauf_setzen(&mut self, bildlauf: f64) {
        if !self.bildlauf_offen {
            self.bildlauf = bildlauf;
        }
    }

    /// Ob die Ansicht die gemerkte Bildlaufposition noch herstellen muss.
    pub fn bildlauf_ausstehend(&self) -> bool {
        self.bildlauf_offen
    }

    /// Nimmt zur Kenntnis, dass die Ansicht die gemerkte Position hergestellt
    /// hat.
    pub fn bildlauf_hergestellt(&mut self) {
        self.bildlauf_offen = false;
    }

    /// Was die Statuszeile fuer diesen Tab zeigt.
    pub fn meldung(&self) -> Option<&str> {
        self.meldung.as_deref()
    }

    /// Die Beschriftung dieses Tabs in der Tableiste.
    ///
    /// Der letzte Namensteil des Ordners. Fuer die Wurzel gibt es keinen, und
    /// dort steht der Pfad selbst.
    pub fn titel(&self) -> String {
        match self.ordner.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => self.ordner.to_string_lossy().into_owned(),
        }
    }

    /// Der gespeicherte Zustand dieses Tabs, wie er in `session.toml` gehoert.
    fn zustand(&self) -> Tabzustand {
        Tabzustand {
            ordner: self.ordner.clone(),
            auswahl: self.auswahlname(),
            verstecke_ausgeblendet: self.modell.verstecke_ausgeblendet(),
            sortierung: self.modell.sortierung(),
            bildlauf: self.bildlauf,
        }
    }

    /// Der Name des ausgewaehlten Eintrags, so weit er tragfaehig ist.
    ///
    /// Solange der Tab noch nicht gelesen ist, steht in `wunschauswahl` der
    /// Name aus der letzten Sitzung. Ihn stehen zu lassen ist der Unterschied
    /// zwischen "die Auswahl ueberlebt einen Neustart" und "die Auswahl
    /// ueberlebt einen Neustart, sofern der Nutzer den Tab vorher angesehen
    /// hat".
    ///
    /// **Die `wunschauswahl` steht vor der Auswahl des Modells und nicht
    /// dahinter.** Eine Fallunterscheidung braucht das nicht, denn sie ist
    /// genau dann gesetzt, wenn die Auswahl des Modells nicht tragfaehig ist:
    /// gefuellt wird sie von einem Aufrufer, der einen Namen vormerkt, und
    /// herausgenommen von [`Tabinhalt::wunschauswahl_anwenden`] mit dem
    /// Abschluss des Lesevorgangs. Steht sie, steht also ein Lesevorgang aus —
    /// und dessen erster Stapel raeumt `modell.auswahl()` weg, weil der
    /// Eintragsindex dem vorigen Lauf gehoert.
    ///
    /// Bis zum 260807 stand sie hinten, und das war dasselbe: `leeren` hatte
    /// das Modell beim Start des Lesevorgangs geraeumt, `auswahl()` war `None`,
    /// und der Wunsch kam von selbst zum Zug. Seit `5f2e45d` bleibt der alte
    /// Bestand stehen, und die alte Reihenfolge schrieb einen kurz zuvor
    /// vorgemerkten Namen mit dem veralteten zu
    /// (`issues/260807-0800_*_auswahlname-haelt-die-veraltete-modellauswahl-fuer-gueltig.md`).
    fn auswahlname(&self) -> Option<String> {
        if let Some(name) = &self.wunschauswahl {
            return Some(name.clone());
        }
        let index = self.modell.auswahl()?;
        self.modell
            .eintraege()
            .get(index as usize)
            .map(|eintrag| eintrag.name.clone())
    }

    /// Setzt die Auswahl auf den Eintrag mit dem gemerkten Namen.
    ///
    /// Gerufen, sobald der Lesevorgang abgeschlossen und damit sortiert ist.
    /// Findet sich der Name nicht mehr, bleibt die Auswahl leer; der Ordner
    /// kann sich seit der letzten Sitzung geaendert haben.
    fn wunschauswahl_anwenden(&mut self) {
        let Some(name) = self.wunschauswahl.take() else {
            return;
        };
        let gefunden = self
            .modell
            .eintraege()
            .iter()
            .position(|eintrag| eintrag.name == name);
        if let Some(index) = gefunden {
            self.modell.auswahl_setzen(Some(index as u32));
        }
    }
}

/// Was daraus geworden ist, die Auswahl auf einen Namen zu setzen.
///
/// Erteilt wird der Auftrag von [`Tabliste::auswahl_auf_namen`], der einen
/// Stelle, die einen Namen zur Auswahl macht.
///
/// **Die Zeile reist bei `Gewaehlt` mit.** Sie steht hier nicht, weil dieser
/// Wert sie beschriebe, sondern weil die Ansicht sie braucht, um die
/// `NSTableView` nachzuziehen; sie ein zweites Mal auszurechnen hiesse,
/// dieselbe Frage zweimal zu stellen. Wer sie nicht braucht, uebergeht sie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Auswahlversuch {
    /// Der Eintrag stand sichtbar in der gelesenen Liste, in dieser Zeile.
    Gewaehlt(usize),
    /// Es steht ein Lesevorgang aus. Der Name ist vorgemerkt; die Auswahl
    /// springt auf den Eintrag, sobald der Lesevorgang abgeschlossen ist.
    Vorgemerkt,
    /// Die Liste ist gelesen und kennt den Namen nicht.
    Unbekannt,
}

/// Was ein Einzug am sichtbaren Tab veraendert hat.
///
/// Die Ansicht liest daran ab, was sie AppKit melden muss. Ein Stapel, der in
/// einem verdeckten Tab angekommen ist, taucht hier nicht auf: er aendert
/// nichts an dem, was auf dem Schirm steht.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Einzug {
    /// Der sichtbare Tab hat Zeilen dazubekommen.
    pub angehaengt: bool,
    /// Der erste Stapel hat die Liste des vorigen Lesevorgangs abgeloest.
    ///
    /// Neben `angehaengt`, weil die Ansicht darauf anders antwortet: eine blosse
    /// neue Zeilenzahl liesse die Auswahl der Tabelle auf einer Zeile stehen,
    /// die es nach dem Ersatz nicht mehr gibt. Kommt hoechstens einmal je
    /// Lesevorgang und nur, wenn wirklich Zeilen gefallen sind.
    pub ersetzt: bool,
    /// Der sichtbare Tab ist fertig gelesen und sortiert.
    pub fertig: bool,
    /// Die Statuszeile des sichtbaren Tabs hat einen neuen Text.
    pub meldung_neu: bool,
}

/// Was die Lesereihenfolge von einem Dateifenster wissen muss.
///
/// Zwei Zahlen statt der ganzen Liste: [`crate::fenstermodell`] entscheidet
/// ueber die Reihenfolge und braucht dafuer nicht den Inhalt der Tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tabuebersicht {
    /// Wie viele Tabs das Dateifenster hat.
    pub zahl: usize,
    /// Die Stelle des sichtbaren Tabs.
    pub sichtbar: usize,
}

/// Die Tabs eines Dateifensters.
pub struct Tabliste {
    tabs: Vec<Tabinhalt>,
    aktiv: usize,
    letzte_generation: u64,
    /// Ob die verdeckten Tabs noch auf ihre Lesevorgaenge warten.
    nachzuegler_offen: bool,
}

impl Tabliste {
    /// Die Tabs eines Dateifensters aus seinem gespeicherten Zustand.
    ///
    /// Liest nichts. Den ersten Lesevorgang stoesst
    /// [`Tabliste::sichtbaren_lesen`] an, die verdeckten folgen ueber
    /// [`Tabliste::nachzuegler_starten`].
    pub fn aus_zustand(zustand: &Fensterzustand) -> Self {
        let mut tabs: Vec<Tabinhalt> = zustand.tabs.iter().map(Tabinhalt::aus_zustand).collect();
        if tabs.is_empty() {
            // C1 verlangt mindestens einen Tab je Dateifenster. Eine
            // `session.toml` von Hand geleert zu bekommen ist moeglich, und ein
            // Dateifenster ohne Tab waere danach unbedienbar.
            tabs.push(Tabinhalt::aus_zustand(&Tabzustand::default()));
        }
        let aktiv = zustand.aktiver_tab.min(tabs.len() - 1);
        Self {
            tabs,
            aktiv,
            letzte_generation: GENERATION_LEER,
            nachzuegler_offen: true,
        }
    }

    /// Der gespeicherte Zustand dieses Dateifensters.
    pub fn zustand(&self) -> Fensterzustand {
        Fensterzustand {
            aktiver_tab: self.aktiv,
            tabs: self.tabs.iter().map(Tabinhalt::zustand).collect(),
        }
    }

    /// Wie viele Tabs es gibt. Nie null.
    pub fn zahl(&self) -> usize {
        self.tabs.len()
    }

    /// Die Stelle des sichtbaren Tabs.
    pub fn aktive_stelle(&self) -> usize {
        self.aktiv
    }

    /// Der sichtbare Tab.
    pub fn aktiver(&self) -> &Tabinhalt {
        &self.tabs[self.aktiv]
    }

    /// Der sichtbare Tab, veraenderlich.
    pub fn aktiver_mut(&mut self) -> &mut Tabinhalt {
        &mut self.tabs[self.aktiv]
    }

    /// Die Beschriftungen aller Tabs, in der Reihenfolge der Leiste.
    pub fn titel(&self) -> Vec<String> {
        self.tabs.iter().map(Tabinhalt::titel).collect()
    }

    /// Die Ordner aller Tabs, in der Reihenfolge der Leiste (C9).
    ///
    /// Neben [`Tabliste::aktiver`], weil der Auswurf eines Datentraegers jeden
    /// Tab trifft und nicht nur den sichtbaren.
    pub fn tabordner(&self) -> Vec<PathBuf> {
        self.tabs.iter().map(|tab| tab.ordner.clone()).collect()
    }

    /// Setzt einen **verdeckten** Tab auf einen anderen Ordner, ohne zu lesen
    /// (C9).
    ///
    /// Der Weg des Auswurfs aus [`crate::auffrischung::datentraeger_verloren`]
    /// fuer die Tabs, die niemand sieht. Der Tab wird ungelesen: sein bisheriger
    /// Inhalt gehoert einem Ordner, den es nicht mehr gibt. Gelesen wird er
    /// erst, wenn der Nutzer auf ihn wechselt, denn `waehlen` ruft
    /// `ungelesenen_aktiven_nachlesen`. Ein Lesevorgang fuer einen verdeckten
    /// Tab waere Arbeit fuer einen leeren Schirm; die zweite Stufe der
    /// Lesereihenfolge ist zu diesem Zeitpunkt laengst gelaufen.
    ///
    /// Eine Stelle ausserhalb der Liste und die des sichtbaren Tabs werden
    /// uebergangen: der sichtbare geht ueber [`Tabliste::ordner_setzen`], damit
    /// er liest und die Ansicht nachzieht.
    pub fn verdeckten_tab_setzen(&mut self, stelle: usize, ordner: impl Into<PathBuf>) {
        if stelle >= self.tabs.len() || stelle == self.aktiv {
            return;
        }
        let sortierung = self.tabs[stelle].modell.sortierung();
        let verstecke = self.tabs[stelle].modell.verstecke_ausgeblendet();
        let mut zustand = Tabzustand::auf(ordner);
        zustand.sortierung = sortierung;
        zustand.verstecke_ausgeblendet = verstecke;
        self.tabs[stelle] = Tabinhalt::aus_zustand(&zustand);
    }

    /// Was die Lesereihenfolge von diesem Dateifenster wissen muss.
    pub fn uebersicht(&self) -> Tabuebersicht {
        Tabuebersicht {
            zahl: self.zahl(),
            sichtbar: self.aktive_stelle(),
        }
    }

    /// Wechselt auf den Tab an der genannten Stelle.
    ///
    /// Eine Stelle ausserhalb der Liste wird uebergangen. Liefert, ob der
    /// sichtbare Tab dadurch ein anderer geworden ist; nur dann muss die
    /// Ansicht ihren Inhalt austauschen.
    pub fn waehlen(&mut self, stelle: usize) -> bool {
        if stelle >= self.tabs.len() || stelle == self.aktiv {
            return false;
        }
        self.aktiv = stelle;
        self.ungelesenen_aktiven_nachlesen();
        true
    }

    /// Wechselt zum naechsten Tab und laeuft am Ende auf den ersten um.
    pub fn naechster(&mut self) -> bool {
        let stelle = (self.aktiv + 1) % self.tabs.len();
        self.waehlen(stelle)
    }

    /// Wechselt zum vorigen Tab und laeuft am Anfang auf den letzten um.
    pub fn voriger(&mut self) -> bool {
        let stelle = (self.aktiv + self.tabs.len() - 1) % self.tabs.len();
        self.waehlen(stelle)
    }

    /// Oeffnet einen neuen Tab hinter dem sichtbaren und macht ihn sichtbar.
    ///
    /// Der neue Tab liest sofort: er ist der sichtbare, und ein sichtbarer Tab
    /// ohne Inhalt waere ein leeres Dateifenster.
    pub fn oeffnen(&mut self, ordner: impl Into<PathBuf>) {
        let zustand = Tabzustand::auf(ordner);
        let stelle = self.aktiv + 1;
        self.tabs.insert(stelle, Tabinhalt::aus_zustand(&zustand));
        self.aktiv = stelle;
        self.lesen_starten(stelle);
    }

    /// Schliesst den sichtbaren Tab.
    ///
    /// Beim letzten Tab bleibt das Dateifenster stehen und zeigt den
    /// Standardordner, wie C1 es verlangt. Liefert, ob danach ein anderer
    /// Ordner im Fenster steht.
    pub fn schliessen(&mut self) -> bool {
        if self.tabs.len() == 1 {
            let standard = Tabzustand::default();
            if self.tabs[0].ordner == standard.ordner {
                // Schon der Standardordner. Ihn ein zweites Mal zu lesen waere
                // Arbeit ohne sichtbare Wirkung.
                return false;
            }
            self.tabs[0] = Tabinhalt::aus_zustand(&standard);
            self.lesen_starten(0);
            return true;
        }
        self.tabs.remove(self.aktiv);
        if self.aktiv >= self.tabs.len() {
            self.aktiv = self.tabs.len() - 1;
        }
        self.ungelesenen_aktiven_nachlesen();
        true
    }

    /// Laesst den sichtbaren Tab einen anderen Ordner zeigen.
    ///
    /// Der Weg jeder Navigation: hinein, hinaus, ueber die Pfadeingabe, aus der
    /// Zwischenablage. Der bisherige Inhalt faellt, und der Lesevorgang beginnt
    /// sofort.
    ///
    /// `auswahl` ist der Name des Eintrags, auf den die Auswahl springt, sobald
    /// gelesen ist. Der Aufstieg aus C2 nennt hier den verlassenen Ordner, der
    /// Sprung aus C10 die genannte Datei. Getragen wird beides von derselben
    /// `wunschauswahl`, die schon die Sitzungswiederherstellung benutzt: der
    /// Name ueberlebt einen noch laufenden Lesevorgang, eine Zeilennummer
    /// nicht.
    pub fn ordner_setzen(&mut self, ordner: impl Into<PathBuf>, auswahl: Option<String>) {
        let stelle = self.aktiv;
        let sortierung = self.tabs[stelle].modell.sortierung();
        let verstecke = self.tabs[stelle].modell.verstecke_ausgeblendet();
        let mut zustand = Tabzustand::auf(ordner);
        zustand.sortierung = sortierung;
        zustand.verstecke_ausgeblendet = verstecke;
        zustand.auswahl = auswahl;
        self.tabs[stelle] = Tabinhalt::aus_zustand(&zustand);
        self.lesen_starten(stelle);
    }

    /// Liest den sichtbaren Tab noch einmal, ohne den Ordner zu wechseln (C9).
    ///
    /// Der Rumpf des einen Auffrischungspfads aus [`crate::auffrischung`].
    /// Gegenueber [`Tabliste::ordner_setzen`] mit demselben Ordner sind es zwei
    /// Unterschiede, und beide sind der Grund, aus dem diese Methode besteht:
    /// der Name des ausgewaehlten Eintrags und die Bildlaufposition gehen als
    /// Wunsch in den neuen Tab, statt verloren zu gehen.
    ///
    /// **Ein zweiter Mechanismus entsteht dabei nicht.** Getragen wird beides
    /// von den Feldern, die die Sitzungswiederherstellung ohnehin benutzt:
    /// `wunschauswahl` setzt die Auswahl, sobald gelesen und sortiert ist, und
    /// `bildlauf_offen` sagt der Ansicht, dass sie die gemerkte Position noch
    /// herstellen muss. Ein **Name** und keine Zeilennummer, weil eine
    /// Auffrischung genau dann stattfindet, wenn sich der Ordnerinhalt
    /// geaendert hat: die Zeile des Eintrags ist danach womoeglich eine andere.
    /// Ist der Eintrag verschwunden, bleibt die Auswahl leer, wie C9 es
    /// zulaesst ("soweit die Eintraege noch existieren").
    ///
    /// **Der Tab bleibt stehen, er entsteht nicht neu.** Bis zum 260807 setzte
    /// diese Methode einen frischen [`Tabinhalt`] an die Stelle des alten und
    /// warf damit den gelesenen Bestand weg, noch bevor der neue Lesevorgang
    /// etwas geliefert hatte. Genau daran hing die leere Liste waehrend eines
    /// Stapel-Umbenennens. Ordner, Sortierung und Filter aendern sich bei einer
    /// Auffrischung ohnehin nicht; zurueckzusetzen sind allein der Auswahlname
    /// und die offene Bildlaufposition, und beide stehen hier.
    pub fn aktiven_neu_lesen(&mut self) {
        let stelle = self.aktiv;
        let auswahlname = self.tabs[stelle].auswahlname();
        let tab = &mut self.tabs[stelle];
        // Der Name des ausgewaehlten Eintrags wird zum Wunsch, den der
        // Abschluss des neuen Lesevorgangs einloest — dieselben zwei Felder,
        // die die Sitzungswiederherstellung benutzt, und kein zweiter Weg.
        tab.wunschauswahl = auswahlname;
        tab.bildlauf_offen = tab.bildlauf > 0.0;
        self.lesen_starten(stelle);
    }

    /// Setzt die Auswahl des sichtbaren Tabs auf den Eintrag dieses Namens:
    /// jetzt, oder sobald er gelesen ist.
    ///
    /// **Die eine Stelle, die einen Namen zur Auswahl macht**, und die eine
    /// Stelle, die dafuer den laufenden Lesevorgang beruecksichtigt. Sie steht
    /// hier und nicht in der Ansicht, weil hier beides beieinander liegt: der
    /// Lesevorgang und die `wunschauswahl`, die den Namen ueber ihn hinweg
    /// traegt. Die Ansicht bekommt bei `Gewaehlt` die Zeile und setzt sie in
    /// der `NSTableView`; entschieden wird nichts mehr.
    ///
    /// **Der laufende Lesevorgang wird zuerst gefragt und nicht zuletzt.** Was
    /// er liefert, loest den angezeigten Bestand ab, und mit ihm faellt jeder
    /// Eintragsindex: der erste Stapel raeumt `modell.auswahl()`, und der
    /// Abschluss loest ohnehin die `wunschauswahl` ein. Eine jetzt gesetzte
    /// Zeile ueberlebt das nicht, ein Name schon.
    ///
    /// Bis zum 260807 stand die Frage hinten. Solange das Modell beim Start des
    /// Lesevorgangs geleert wurde, war das dasselbe: der Name war im Bestand
    /// nicht zu finden, und die Stelle fiel von selbst auf das Vormerken.
    /// Seit `5f2e45d` bleibt der alte Bestand stehen, die erste Frage gewann,
    /// und die Auswahl landete auf einer Zeile des vorigen Laufs, die der erste
    /// Stapel ersatzlos wegraeumte
    /// (`issues/260807-0800_*_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-und-die-auswahl-faellt-danach-ersatzlos.md`).
    ///
    /// **Kein zweiter Weg, eine Zeile anhand ihres Namens zu waehlen**, sondern
    /// derselbe: getragen wird der Name von der `wunschauswahl`, die schon die
    /// Sitzungswiederherstellung, der Aufstieg aus C2, der Sprung aus der
    /// Zwischenablage (C10) und die Auffrischung aus C9 benutzen.
    pub fn auswahl_auf_namen(&mut self, name: &str) -> Auswahlversuch {
        let tab = &mut self.tabs[self.aktiv];
        if tab.liest() {
            tab.wunschauswahl = Some(name.to_owned());
            return Auswahlversuch::Vorgemerkt;
        }
        match tab
            .modell
            .index_von_namen(name)
            .and_then(|index| tab.modell.zeile_von(index))
        {
            Some(zeile) => Auswahlversuch::Gewaehlt(zeile),
            None => Auswahlversuch::Unbekannt,
        }
    }

    /// Startet den Lesevorgang des sichtbaren Tabs.
    ///
    /// Die erste Stufe der Lesereihenfolge aus dem Modulkopf.
    pub fn sichtbaren_lesen(&mut self) {
        let stelle = self.aktiv;
        self.lesen_starten(stelle);
    }

    /// Ob die verdeckten Tabs jetzt an der Reihe sind.
    ///
    /// Wahr, sobald der sichtbare Tab bedienbar ist und noch verdeckte Tabs
    /// ungelesen sind.
    pub fn nachzuegler_faellig(&self) -> bool {
        self.nachzuegler_offen && self.aktiver().ist_bedienbar()
    }

    /// Startet die Lesevorgaenge aller verdeckten Tabs.
    ///
    /// Die zweite Stufe der Lesereihenfolge. Sie laeuft genau einmal je
    /// Dateifenster; ein Tab, den der Nutzer spaeter oeffnet, liest fuer sich.
    pub fn nachzuegler_starten(&mut self) {
        self.nachzuegler_offen = false;
        for stelle in 0..self.tabs.len() {
            if stelle != self.aktiv && !self.tabs[stelle].gelesen && !self.tabs[stelle].liest() {
                self.lesen_starten(stelle);
            }
        }
    }

    /// Ob irgendein Tab dieses Fensters gerade liest.
    pub fn liest_noch(&self) -> bool {
        self.tabs.iter().any(Tabinhalt::liest)
    }

    /// Bricht jeden laufenden Lesevorgang ab und schliesst die Modelle ab.
    ///
    /// Gerufen beim Schliessen des Fensters. Ohne diesen Aufruf liefe der
    /// Arbeitsfaden eines Ordners mit 100.000 Eintraegen gegen eine Tabelle
    /// weiter, die niemand mehr sieht.
    pub fn abbrechen(&mut self) {
        for tab in &mut self.tabs {
            if let Some(vorgang) = tab.lesevorgang.take() {
                vorgang.abbrechen();
            }
            tab.modell.abschliessen();
            tab.gelesen = true;
        }
    }

    /// Holt alle wartenden Meldungen aus allen Kanaelen.
    ///
    /// Liefert, was sich am sichtbaren Tab geaendert hat. Die verdeckten Tabs
    /// fuellen sich dabei still: was in ihnen ankommt, steht auf keinem Schirm.
    pub fn einziehen(&mut self) -> Einzug {
        let aktiv = self.aktiv;
        let mut einzug = Einzug::default();
        for (stelle, tab) in self.tabs.iter_mut().enumerate() {
            let veraendert = einzug_je_tab(tab);
            if stelle == aktiv {
                einzug = veraendert;
            }
        }
        einzug
    }

    /// Startet den Lesevorgang eines Tabs.
    ///
    /// **Das Modell wird nicht vorab geleert.** Es bekommt die neue Generation
    /// und merkt den Ersatz vor; der bisherige Inhalt faellt erst mit dem ersten
    /// gelieferten Stapel, spaetestens mit dem Abschluss. Was das behebt, steht
    /// im Modulkopf von [`Ordnermodell`]: bis zum 260807 setzte jede
    /// Aenderungsmeldung waehrend eines Stapel-Umbenennens den Lesevorgang neu
    /// auf, bevor sein erster Stapel angehaengt war, und die Liste kam fuer die
    /// ganze Laufzeit nicht mehr zum Fuellen.
    fn lesen_starten(&mut self, stelle: usize) {
        self.letzte_generation += 1;
        let generation = self.letzte_generation;
        let tab = &mut self.tabs[stelle];
        // Der bisherige Lesevorgang faellt hier. Sein Arbeitsfaden bemerkt den
        // Abbruch und endet von selbst; auf ihn zu warten hiesse, eine
        // Navigation an den verlassenen Ordner zu haengen. Zugleich ist das der
        // Grund, aus dem kein Stapel des alten Laufs mehr ankommen kann: sein
        // Empfaenger ist weg, und der Bestand mischt sich nicht.
        tab.lesevorgang = None;
        tab.modell.lesevorgang_beginnen(generation);
        tab.meldung = None;
        tab.gelesen = false;
        tab.lesevorgang = Some(Lesevorgang::starten(&tab.ordner, generation));
    }

    /// Liest den sichtbaren Tab nach, falls er noch nie gelesen wurde.
    ///
    /// Der Ausnahmefall aus dem Modulkopf: die zweite Stufe der Lesereihenfolge
    /// hat ihn noch nicht erreicht, und der Nutzer ist schneller.
    fn ungelesenen_aktiven_nachlesen(&mut self) {
        let stelle = self.aktiv;
        if !self.tabs[stelle].gelesen && !self.tabs[stelle].liest() {
            self.lesen_starten(stelle);
        }
    }
}

/// Holt die wartenden Meldungen eines einzelnen Tabs ab.
fn einzug_je_tab(tab: &mut Tabinhalt) -> Einzug {
    let Some(vorgang) = tab.lesevorgang.as_ref() else {
        return Einzug::default();
    };
    let mut einzug = Einzug::default();
    // Ohne Pruefung der Generation, und das ist Absicht: gelesen wird allein
    // aus dem Kanal des Lesevorgangs, den dieser Tab gerade haelt, und dessen
    // Generation ist die seines Modells. Was einen Ordnerwechsel mitten im
    // Lesen traegt, steht im Modulkopf von `crate::appkit::tabelle`.
    for meldung in vorgang.meldungen().try_iter() {
        match meldung {
            Meldung::Stapel { eintraege, .. } => {
                // Vor dem Anhaengen gefragt: danach ist der Ersatz eingeloest
                // und die Antwort waere immer "nein".
                einzug.ersetzt |= tab.modell.ersetzt_beim_naechsten_stapel();
                tab.modell.anhaengen(eintraege);
                einzug.angehaengt = true;
            }
            Meldung::Fertig { abschluss, .. } => {
                if let Abschluss::Fehler(fehler) = &abschluss {
                    tab.meldung = Some(format!(
                        "{} liess sich nicht vollstaendig lesen: {fehler}",
                        tab.ordner.display()
                    ));
                    einzug.meldung_neu = true;
                }
                tab.modell.abschliessen();
                tab.wunschauswahl_anwenden();
                tab.gelesen = true;
                einzug.fertig = true;
                break;
            }
        }
    }
    if einzug.fertig {
        tab.lesevorgang = None;
    }
    einzug
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ein Fensterzustand mit den genannten Ordnern, der erste ist sichtbar.
    fn zustand(ordner: &[&str]) -> Fensterzustand {
        Fensterzustand {
            aktiver_tab: 0,
            tabs: ordner.iter().map(Tabzustand::auf).collect(),
        }
    }

    /// Eine Tabliste, die nie liest.
    ///
    /// Die Proben unten pruefen die Verwaltung der Tabs und nicht das Lesen;
    /// ein Lesevorgang je Probe startete einen Arbeitsfaden gegen einen Ordner,
    /// den es nicht gibt.
    fn liste(ordner: &[&str]) -> Tabliste {
        Tabliste::aus_zustand(&zustand(ordner))
    }

    fn ordnernamen(liste: &Tabliste) -> Vec<String> {
        liste
            .tabs
            .iter()
            .map(|tab| tab.ordner.display().to_string())
            .collect()
    }

    #[test]
    fn ein_fenster_ohne_tabs_bekommt_einen_standardtab() {
        let leer = Fensterzustand {
            aktiver_tab: 7,
            tabs: Vec::new(),
        };
        let liste = Tabliste::aus_zustand(&leer);
        assert_eq!(liste.zahl(), 1, "C1 verlangt mindestens einen Tab");
        assert_eq!(liste.aktive_stelle(), 0);
    }

    #[test]
    fn eine_stelle_jenseits_der_liste_faellt_auf_den_letzten_tab() {
        let mut zustand = zustand(&["/a", "/b"]);
        zustand.aktiver_tab = 9;
        let liste = Tabliste::aus_zustand(&zustand);
        assert_eq!(liste.aktive_stelle(), 1);
    }

    #[test]
    fn der_naechste_und_der_vorige_tab_laufen_um() {
        let mut liste = liste(&["/a", "/b", "/c"]);
        assert!(liste.naechster());
        assert_eq!(liste.aktive_stelle(), 1);
        assert!(liste.naechster());
        assert!(liste.naechster());
        assert_eq!(liste.aktive_stelle(), 0, "der letzte laeuft auf den ersten");
        assert!(liste.voriger());
        assert_eq!(liste.aktive_stelle(), 2, "der erste laeuft auf den letzten");
    }

    #[test]
    fn ein_einziger_tab_wechselt_auf_sich_selbst_nicht() {
        let mut liste = liste(&["/a"]);
        assert!(!liste.naechster());
        assert!(!liste.voriger());
        assert!(!liste.waehlen(0));
        assert!(!liste.waehlen(5), "eine Stelle, die es nicht gibt");
    }

    #[test]
    fn das_schliessen_des_letzten_tabs_laesst_das_fenster_stehen() {
        let mut liste = liste(&["/a"]);
        assert!(liste.schliessen());
        assert_eq!(liste.zahl(), 1, "das Dateifenster bleibt bestehen");
        assert_eq!(
            liste.aktiver().ordner(),
            Tabzustand::default().ordner,
            "und zeigt den Standardordner"
        );
        // Der zweite Aufruf steht schon auf dem Standardordner und liest ihn
        // nicht ein zweites Mal.
        assert!(!liste.schliessen());
        assert_eq!(liste.zahl(), 1);
    }

    #[test]
    fn das_schliessen_ruecht_die_sichtbare_stelle_nach() {
        let mut liste = liste(&["/a", "/b", "/c"]);
        liste.waehlen(2);
        liste.schliessen();
        assert_eq!(ordnernamen(&liste), ["/a", "/b"]);
        assert_eq!(liste.aktive_stelle(), 1, "der letzte Tab war sichtbar");

        liste.waehlen(0);
        liste.schliessen();
        assert_eq!(ordnernamen(&liste), ["/b"]);
        assert_eq!(liste.aktive_stelle(), 0);
    }

    #[test]
    fn der_zustand_ueberlebt_den_rundlauf() {
        let mut vorher = zustand(&["/a", "/b"]);
        vorher.aktiver_tab = 1;
        vorher.tabs[1].auswahl = Some("bild.jpg".to_owned());
        vorher.tabs[1].bildlauf = 240.0;
        vorher.tabs[0].verstecke_ausgeblendet = false;

        let liste = Tabliste::aus_zustand(&vorher);
        let nachher = liste.zustand();

        assert_eq!(nachher, vorher);
    }

    /// Die Auswahl aus der letzten Sitzung geht nicht verloren, solange der Tab
    /// verdeckt bleibt.
    #[test]
    fn ein_ungelesener_tab_behaelt_seinen_auswahlnamen() {
        let mut vorher = zustand(&["/a"]);
        vorher.tabs[0].auswahl = Some("urlaub.jpg".to_owned());
        let liste = Tabliste::aus_zustand(&vorher);
        assert_eq!(
            liste.zustand().tabs[0].auswahl,
            Some("urlaub.jpg".to_owned())
        );
    }

    /// Die Zusage aus C9: Auswahl und Bildlaufposition ueberleben eine
    /// Auffrischung.
    #[test]
    fn eine_auffrischung_nimmt_ordner_auswahl_und_bildlauf_mit() {
        // Ein Ordner, den es gibt: `aktiven_neu_lesen` startet einen
        // Lesevorgang, und der soll nicht gegen ein Nichts laufen.
        let vorhanden = std::env::temp_dir().display().to_string();
        let mut liste = liste(&[&vorhanden]);
        liste.aktiver_mut().wunschauswahl = Some("bild.jpg".to_owned());
        liste.aktiver_mut().bildlauf_setzen(240.0);

        liste.aktiven_neu_lesen();

        assert_eq!(liste.aktiver().ordner(), Path::new(&vorhanden));
        assert_eq!(
            liste.aktiver().auswahlname().as_deref(),
            Some("bild.jpg"),
            "der Name des ausgewaehlten Eintrags geht in den neuen Tab"
        );
        assert_eq!(liste.aktiver().bildlauf(), 240.0);
        assert!(
            liste.aktiver().bildlauf_ausstehend(),
            "die Ansicht muss die gemerkte Position noch herstellen"
        );
        assert_eq!(liste.zahl(), 1, "es entsteht kein zweiter Tab");
    }

    /// Der Defekt vom 260805-1337, am Tab statt am Modell: eine Auffrischung
    /// leert die Liste nicht, und auch die fuenfte hintereinander nicht.
    #[test]
    fn eine_auffrischung_laesst_die_liste_stehen_bis_ihr_erster_stapel_da_ist() {
        use krk_core::verzeichnis::{Eintrag, Typ};

        let vorhanden = std::env::temp_dir().display().to_string();
        let mut liste = liste(&[&vorhanden]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.anhaengen([
            Eintrag::neu(
                "a.txt".to_owned(),
                0,
                std::time::SystemTime::UNIX_EPOCH,
                Typ::Datei,
            ),
            Eintrag::neu(
                "b.txt".to_owned(),
                0,
                std::time::SystemTime::UNIX_EPOCH,
                Typ::Datei,
            ),
        ]);
        modell.abschliessen();

        for runde in 1..=5 {
            liste.aktiven_neu_lesen();
            assert_eq!(
                liste.aktiver().modell().zeilenzahl(),
                2,
                "die Auffrischung {runde} hat die Liste geleert, bevor sie etwas geliefert hat"
            );
        }
    }

    /// Ein Eintrag fuer die drei Proben unten.
    fn datei(name: &str) -> krk_core::verzeichnis::Eintrag {
        use krk_core::verzeichnis::{Eintrag, Typ};
        Eintrag::neu(
            name.to_owned(),
            0,
            std::time::SystemTime::UNIX_EPOCH,
            Typ::Datei,
        )
    }

    /// Eine Tabliste mit einem gelesenen Tab auf einem Ordner, den es gibt.
    ///
    /// Ein vorhandener Ordner, weil `aktiven_neu_lesen` einen Lesevorgang
    /// startet und der nicht gegen ein Nichts laufen soll. Geliefert hat er in
    /// den Proben nie etwas: `einziehen` wird nicht gerufen, der Ersatz steht
    /// also fuer die ganze Probe aus. Genau das ist die Spanne, um die es geht.
    fn gelesene_liste(namen: &[&str]) -> Tabliste {
        let vorhanden = std::env::temp_dir().display().to_string();
        let mut liste = liste(&[&vorhanden]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.anhaengen(namen.iter().map(|name| datei(name)));
        modell.abschliessen();
        liste
    }

    /// Der deterministische Fall aus
    /// `issues/260807-0800_*_eintrag-waehlen-trifft-den-noch-nicht-abgeloesten-bestand-…`:
    /// ein Stapel-Umbenennen mit Umnummerierung nach oben.
    ///
    /// `IMG_1.jpg, IMG_2.jpg` wird zu `IMG_2.jpg, IMG_3.jpg`. Der erste neue
    /// Name stand schon im alten Bestand, und die Auffrischung laeuft im selben
    /// synchronen Aufruf wie die Auswahl — der Ersatz steht also garantiert
    /// noch aus. Waehlte die Stelle jetzt die alte Zeile, raeumte der erste
    /// Stapel sie ersatzlos weg.
    ///
    /// Geprueft wird die Ebene unter der Aufrufstelle: die Entscheidung sitzt
    /// seit dem 260807 in `auswahl_auf_namen` und ist damit ohne Fenster
    /// erreichbar. Was die Ebene darueber noch tut — die Zeile in der
    /// `NSTableView` setzen —, faellt bei `Vorgemerkt` ohnehin weg.
    #[test]
    fn der_erste_neue_name_eines_stapel_umbenennens_wird_vorgemerkt() {
        let mut liste = gelesene_liste(&["IMG_1.jpg", "IMG_2.jpg"]);
        let index = liste
            .aktiver()
            .modell()
            .index_von_namen("IMG_1.jpg")
            .expect("IMG_1.jpg steht in der Liste");
        liste.aktiver_mut().modell_mut().auswahl_setzen(Some(index));

        // Was `abschluss_verarbeiten` nach einem Stapel-Umbenennen tut: erst
        // auffrischen, dann den ersten neuen Namen waehlen.
        liste.aktiven_neu_lesen();
        let versuch = liste.auswahl_auf_namen("IMG_2.jpg");

        assert_eq!(
            versuch,
            Auswahlversuch::Vorgemerkt,
            "der alte Bestand kennt IMG_2.jpg zwar, aber der Ersatz raeumt ihn weg"
        );
        assert_eq!(
            liste.aktiver().auswahlname().as_deref(),
            Some("IMG_2.jpg"),
            "der Name traegt die Auswahl ueber den Lesevorgang"
        );
    }

    /// Der Fall aus
    /// `issues/260807-0800_*_auswahlname-haelt-die-veraltete-modellauswahl-fuer-gueltig.md`:
    /// zwei Auffrischungen vor dem ersten Stapel, mit einem Vormerken dazwischen.
    #[test]
    fn eine_zweite_auffrischung_laesst_den_vorgemerkten_namen_stehen() {
        let mut liste = gelesene_liste(&["alt.txt"]);
        let index = liste
            .aktiver()
            .modell()
            .index_von_namen("alt.txt")
            .expect("alt.txt steht in der Liste");
        liste.aktiver_mut().modell_mut().auswahl_setzen(Some(index));

        liste.aktiven_neu_lesen();
        liste.auswahl_auf_namen("neu.txt");
        liste.aktiven_neu_lesen();

        assert_eq!(
            liste.aktiver().auswahlname().as_deref(),
            Some("neu.txt"),
            "die veraltete Auswahl des Modells hat den vorgemerkten Namen zugeschrieben"
        );
    }

    /// Ohne laufenden Lesevorgang bleibt es beim alten Weg: der Sprung aus C10
    /// waehlt die Zeile sofort, und ein Name, den die fertige Liste nicht
    /// kennt, ist eine Auskunft an den Nutzer.
    #[test]
    fn ohne_lesevorgang_waehlt_der_name_seine_zeile() {
        let mut liste = gelesene_liste(&["b.txt", "a.txt"]);

        assert_eq!(
            liste.auswahl_auf_namen("b.txt"),
            Auswahlversuch::Gewaehlt(1),
            "abschliessen sortiert: a.txt steht in Zeile 0, b.txt in Zeile 1"
        );
        assert_eq!(liste.auswahl_auf_namen("c.txt"), Auswahlversuch::Unbekannt);
    }

    #[test]
    fn die_nachzuegler_sind_erst_faellig_wenn_der_sichtbare_tab_steht() {
        let mut liste = liste(&["/a", "/b"]);
        assert!(
            !liste.nachzuegler_faellig(),
            "der sichtbare Tab hat noch nichts"
        );
        liste.tabs[0].gelesen = true;
        assert!(liste.nachzuegler_faellig());
        liste.nachzuegler_offen = false;
        assert!(
            !liste.nachzuegler_faellig(),
            "die zweite Stufe laeuft genau einmal"
        );
    }
}
