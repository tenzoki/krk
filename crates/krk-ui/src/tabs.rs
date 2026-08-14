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
use krk_core::verzeichnis::modell::Befund;
use krk_core::verzeichnis::{Abschluss, Auftrag, Durchlauf, Lesevorgang, Meldung, Ordnermodell};

/// Die Generation, mit der ein noch nicht gelesener Tab anfaengt.
const GENERATION_LEER: u64 = 0;

/// Ein Tab: sein Ordner, sein Inhalt und sein Lesevorgang.
pub struct Tabinhalt {
    ordner: PathBuf,
    modell: Ordnermodell,
    lesevorgang: Option<Lesevorgang>,
    /// Der Durchlauf ueber die Unterbaeume, falls einer laeuft.
    ///
    /// **Hoechstens einer je Tab** (C3.6), und deshalb ein Feld und keine
    /// Sammlung: es gibt keine Schreibweise, in der zwei zugleich stuenden.
    /// Neben `lesevorgang` und nicht in ihm, weil die beiden verschiedene
    /// Fragen beantworten und verschieden lange leben; welcher Anlass ihn
    /// faellen laesst, steht bei [`Tabliste::durchlauf_nachziehen`].
    durchlauf: Option<Durchlauf>,
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
            durchlauf: None,
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
///
/// **Der Wert laesst sich nicht still fallenlassen**, und das ist eine
/// Erzwingung und keine Bitte. Das `#[must_use]` unten macht jeden Aufruf,
/// dessen Ergebnis wortlos faellt, zum Uebersetzerfehler, wie die
/// vollstaendigen Fallunterscheidungen dieses Programms es an anderen Stellen
/// tun. Wer die Auskunft wirklich nicht braucht, schreibt `let _ =` davor und
/// sagt damit ausdruecklich, dass er sie nicht braucht. Dieselbe Erzwingung
/// aus demselben Grund traegt
/// [`crate::editormodell::EditorModell::bearbeiten`] seit dem Defekt
/// `260810-0423`.
///
/// **Die fruehere Konvention gilt nicht mehr.** Bis zum Defekt `260810-1906`
/// hiess ein nackter Aufruf „`Unbekannt` kann hier nicht eintreten" und ein
/// begruendetes `let _ =` „kann eintreten und wird bewusst verworfen". Damit
/// standen zwei entgegengesetzte Bedeutungen von `let _ =` in derselben Kiste:
/// am `bearbeiten` hiess es „ich brauche den Wert nicht", hier hiess es „der
/// Wert kann `Unbekannt` sein und wird bewusst verworfen". Ein nackter Aufruf
/// baut jetzt gar nicht mehr, die Unterscheidung steht also vollstaendig im
/// Kommentar der Aufrufstelle, und `let _ =` heisst ueberall dasselbe. Der
/// Datensatz ist
/// `shared/issues/260810-1906_*_die-konvention-am-auswahlversuch-steht-in-kommentaren-und-wird-von-nichts-erzwungen.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use = "war der Versuch Unbekannt, steht der Name nicht in der gelesenen Liste"]
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
///
/// **Der Wert laesst sich nicht still fallenlassen**, aus demselben Grund und
/// in derselben Form wie beim [`Auswahlversuch`], wo die Begruendung
/// ausfuehrlich steht. Eigen ist hier die Folge des Versaeumnisses: faellt der
/// Wert wortlos, bleibt die Meldung an AppKit aus, und die `NSTableView` steht
/// weiter mit dem Bestand von vorhin da, waehrend das Modell laengst den neuen
/// fuehrt. Kein zweiter Weg meldet das nach, und der Bau waere dabei gruen.
/// Heute bindet der eine Aufrufer den Wert und wertet alle fuenf Felder aus
/// (`Dateitabelle::einziehen` in `crate::appkit::tabelle`); das `#[must_use]`
/// haelt das fuer den zweiten fest.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[must_use = "hat sich am sichtbaren Tab etwas geaendert, ist die NSTableView nachzuziehen"]
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
    /// Der Durchlauf des sichtbaren Tabs hat Befunde geliefert.
    ///
    /// Neben `angehaengt`, weil die Ansicht darauf anders antwortet: ein Befund
    /// stellt eine Zeile **mitten** in die sortierte Sicht und nicht an ihr
    /// Ende, also aendert er den Inhalt der Zeilen darunter.
    /// `noteNumberOfRowsChanged` liesse die alten Inhalte stehen; es braucht
    /// `reloadData` (C3.11).
    pub befunde_neu: bool,
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
    /// Die laufende Nummer des zuletzt gestarteten Durchlaufs.
    ///
    /// Eine eigene Zaehlung neben `letzte_generation` und nicht dieselbe: die
    /// Generation gehoert dem Bestand eines Modells, der Durchlauf laesst sie
    /// unberuehrt. Die Nummer benennt allein seinen Arbeitsfaden
    /// (`krk-durchlauf-<n>`), damit zwei Durchlaeufe desselben Tabs in einem
    /// Fadenprotokoll auseinanderzuhalten sind.
    letzter_durchlauf: u64,
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
            letzter_durchlauf: 0,
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
    ///
    /// # Die eine Stelle, an der ein Ordnerwechsel den Filter entscheidet
    ///
    /// Der bisherige [`Tabinhalt`] faellt hier, und mit ihm sein
    /// [`Ordnermodell`]; was der Tab ueber den Wechsel hinweg behaelt, steht
    /// deshalb genau hier und nirgends sonst. Bis zum 260815 waren das
    /// Sortierung und Verstecke; seither kommen der Filter der Tiefe und der
    /// Filtertext dazu, in derselben Bauart und aus demselben Grund.
    ///
    /// **Der Aufstieg braucht keine eigene Zeile.** Er geht wie der Einstieg
    /// durch diese Stelle, und damit gilt fuer ihn dieselbe Regel (C1.9).
    ///
    /// **Weder der Filtertext noch der Filter der Tiefe gehen in die
    /// Sitzung.** Beide werden hier vom alten Modell in das neue getragen und
    /// nicht ueber [`Tabzustand`], der `session.toml` schreibt: ein
    /// wiederhergestellter Filter der Tiefe ohne Filtertext waere ein Zustand,
    /// den nichts anzeigt und der nichts tut.
    pub fn ordner_setzen(&mut self, ordner: impl Into<PathBuf>, auswahl: Option<String>) {
        let stelle = self.aktiv;
        let sortierung = self.tabs[stelle].modell.sortierung();
        let verstecke = self.tabs[stelle].modell.verstecke_ausgeblendet();
        let tief = self.tabs[stelle].modell.tief();
        // **Diese eine Zeile traegt die Antwort auf
        // `decisions/260814-1830_o_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`.**
        // Der Plan faehrt auf "geleert" (C1.9); bei eingeschaltetem Filter der
        // Tiefe uebersteht der Text den Wechsel, weil das Modell der tiefen
        // Ansicht sonst auf jeder Ebene seinen Gegenstand verloere (C1.10).
        // Faellt die Antwort spaeter auf "stehen lassen", wird aus dieser Zeile
        // ein `true`, und sonst aendert sich nichts.
        let filtertext_ueberlebt = tief;
        let filtertext = if filtertext_ueberlebt {
            self.tabs[stelle].modell.filtertext().to_owned()
        } else {
            String::new()
        };
        let mut zustand = Tabzustand::auf(ordner);
        zustand.sortierung = sortierung;
        zustand.verstecke_ausgeblendet = verstecke;
        zustand.auswahl = auswahl;
        self.tabs[stelle] = Tabinhalt::aus_zustand(&zustand);
        let modell = &mut self.tabs[stelle].modell;
        // Unbedingt gesetzt und nicht nur, wenn sich etwas aendert: das frische
        // Modell hat null Eintraege, beide Setzer bauen also eine leere Sicht
        // neu auf, und ein Zweig davor waere eine zweite Stelle, an der die
        // Uebertragung anders ausfallen koennte.
        modell.tief_setzen(tief);
        modell.filtertext_setzen(&filtertext);
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
    ///
    /// **Der Filtertext bleibt deshalb stehen, und zwar ohne eine Zeile
    /// dafuer.** Eine Auffrischung wechselt den Ordner nicht, also greift die
    /// Regel aus [`Tabliste::ordner_setzen`] hier nicht: der Tab behaelt sein
    /// [`Ordnermodell`] und damit seinen Filtertext, gleich ob der Filter der
    /// Tiefe an ist. Was der neue Lesevorgang liefert, geht durch denselben
    /// Filter wie zuvor.
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

    /// Ob irgendein Tab dieses Fensters noch etwas einzuziehen hat.
    ///
    /// **Die eine Bedingung des Einzugstakts**, und sie zaehlt beide Kanaele:
    /// den des Lesevorgangs und den des Durchlaufs. Der Takt bedient beide, und
    /// eine Bedingung, die nur den ersten kennte, hielte ihn an, waehrend die
    /// Befunde des zweiten noch unterwegs sind — die Liste bliebe dann stehen
    /// und wuechse erst beim naechsten Anlass weiter.
    pub fn arbeitet_noch(&self) -> bool {
        self.liest_noch() || self.tabs.iter().any(|tab| tab.durchlauf.is_some())
    }

    /// Bricht den Durchlauf des sichtbaren Tabs ab und stoesst, wenn seine
    /// Bedingungen stehen, einen neuen an.
    ///
    /// **Die eine Stelle, an der ein Durchlauf entsteht und vergeht**, und
    /// damit die Antwort auf beide Haelften von C3.6 und C3.7 zugleich. Zu
    /// rufen ist sie von jedem Anlass, der eine seiner Eingaben aendert: von
    /// jeder Aenderung des Filtertexts, vom Umschalten des Filters der Tiefe
    /// und vom Einzugstakt, sobald ein Tab fertig gelesen ist. Die uebrigen
    /// Anlaesse brauchen keinen Ruf, weil der [`Tabinhalt`] mit dem Durchlauf
    /// dort ohnehin faellt: der Ordnerwechsel tauscht ihn aus, das Schliessen
    /// nimmt ihn weg, und [`Tabliste::lesen_starten`] setzt das Feld
    /// ausdruecklich zurueck, weil der Bestand gerade abgeloest wird und ein
    /// Eintragsindex des alten Bestands auf einen beliebigen Eintrag des neuen
    /// zeigte.
    ///
    /// **Ein Tabwechsel ruft hier nicht**, und das ist Absicht (C3.6 zaehlt je
    /// Tab): ein verdeckter Tab fuellt sich still weiter, wie er es beim
    /// Lesevorgang tut, und ein Abbruch beim Wegwechseln naehme dem Nutzer
    /// gerade die Arbeit weg, die er beim Zurueckwechseln braeuchte.
    ///
    /// **Liefert, ob jetzt ein Durchlauf laeuft.** Der Wert sagt dem Aufrufer,
    /// ob er den Einzugstakt anwerfen muss; faellt er still, kaeme kein Befund
    /// je an und die Zeilen erschienen nie.
    #[must_use = "laeuft jetzt ein Durchlauf, ist der Einzugstakt anzuwerfen"]
    pub fn durchlauf_nachziehen(&mut self) -> bool {
        let stelle = self.aktiv;
        self.durchlauf_nachziehen_an(stelle)
    }

    /// Der Rumpf von [`Tabliste::durchlauf_nachziehen`] fuer einen genannten
    /// Tab.
    ///
    /// Der bisherige Durchlauf faellt in jedem Fall zuerst; sein `Drop` setzt
    /// das Abbruchkennzeichen, und sein Empfaenger geht mit, also kann kein
    /// Befund des alten Laufs mehr ankommen und die Befunde zweier Filtertexte
    /// mischen sich nicht.
    fn durchlauf_nachziehen_an(&mut self, stelle: usize) -> bool {
        self.tabs[stelle].durchlauf = None;
        let tab = &self.tabs[stelle];
        // Der Bestand muss stehen. Waehrend eines Lesevorgangs zeigt das Modell
        // noch den **alten** Ordner — es wird nicht vorab geleert, sondern mit
        // dem ersten Stapel ersetzt —, und eine Auftragsliste daraus benennte
        // Ordner, die es hier gar nicht gibt, unter Indizes, die gleich einem
        // anderen Eintrag gehoeren werden.
        if !tab.gelesen || tab.liest() {
            return false;
        }
        if !tab.modell.filter_steht() || !tab.modell.tief() {
            return false;
        }
        let auftraege = auftraege(&tab.modell);
        // Kein Auftrag, kein Faden: die Zusage aus C3.14 zaehlt Durchlaeufe,
        // und ein Ordner, dessen saemtliche Unterordner den Filtertext im Namen
        // tragen, stoesst hier gar keinen an.
        if auftraege.is_empty() {
            return false;
        }
        self.letzter_durchlauf += 1;
        let nummer = self.letzter_durchlauf;
        let tab = &mut self.tabs[stelle];
        tab.durchlauf = Some(Durchlauf::starten(
            auftraege,
            tab.ordner.clone(),
            tab.modell.filter_klein().to_owned(),
            nummer,
        ));
        true
    }

    /// Bricht jeden laufenden Lesevorgang und jeden Durchlauf ab und schliesst
    /// die Modelle ab.
    ///
    /// Gerufen beim Schliessen des Fensters. Ohne diesen Aufruf liefe der
    /// Arbeitsfaden eines Ordners mit 100.000 Eintraegen gegen eine Tabelle
    /// weiter, die niemand mehr sieht. Fuer den Durchlauf gilt dasselbe, und
    /// sein Abbruch braucht keine eigene Zeile ausser dieser: das Feld
    /// zurueckzusetzen laesst ihn fallen, und sein `Drop` setzt das
    /// Abbruchkennzeichen.
    pub fn abbrechen(&mut self) {
        for tab in &mut self.tabs {
            if let Some(vorgang) = tab.lesevorgang.take() {
                vorgang.abbrechen();
            }
            tab.durchlauf = None;
            tab.modell.abschliessen();
            tab.gelesen = true;
        }
    }

    /// Holt alle wartenden Meldungen aus allen Kanaelen.
    ///
    /// Liefert, was sich am sichtbaren Tab geaendert hat. Die verdeckten Tabs
    /// fuellen sich dabei still: was in ihnen ankommt, steht auf keinem Schirm.
    ///
    /// **Der eine Anlass, an dem ein Durchlauf von selbst beginnt.** Ein Tab,
    /// der eben fertig gelesen hat, hat seinen Bestand zum ersten Mal
    /// vollstaendig; vorher ist keine Auftragsliste zu bilden. Gefragt wird je
    /// Tab und nicht nur am sichtbaren, und das trifft dieselbe Menge: einen
    /// Filtertext bekommt ein Tab allein ueber das Tippen und ueber
    /// [`Tabliste::ordner_setzen`], und beide fassen den sichtbaren an. Eine
    /// Bedingung auf die Stelle waere deshalb eine zweite Regel ohne zweiten
    /// Fall.
    pub fn einziehen(&mut self) -> Einzug {
        let aktiv = self.aktiv;
        let mut einzug = Einzug::default();
        for stelle in 0..self.tabs.len() {
            let veraendert = einzug_je_tab(&mut self.tabs[stelle]);
            if veraendert.fertig {
                // Der Wert sagt, ob der Einzugstakt anzuwerfen ist. Hier laeuft
                // er schon: dieser Aufruf **ist** sein Rumpf.
                let _ = self.durchlauf_nachziehen_an(stelle);
            }
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
        // Und mit ihm der Durchlauf: seine Auftraege nennen Eintragsindizes des
        // Bestands, der gerade abgeloest wird, und ein Befund darauf traefe nach
        // dem Ersatz einen beliebigen anderen Eintrag. Ein neuer Durchlauf
        // beginnt, sobald der Einzugstakt den Abschluss dieses Lesevorgangs
        // sieht.
        tab.durchlauf = None;
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

/// Die Ordner des angezeigten Ordners, ueber die der Durchlauf zu entscheiden
/// hat.
///
/// **Rein und ohne Fenster pruefbar**, und das ist der Grund fuer die Form: die
/// Zusammensetzung dieser Liste ist die Zusage C3.14, und `krk-ui` hat kein
/// Bibliotheksziel, an dem eine Probe von aussen ansetzen koennte.
///
/// **Zwei Bedingungen und keine dritte.**
///
/// Erstens `ist es ein Ordner?`, mit demselben Schnitt, den
/// `Ordnermodell::sichtbar` zieht: eine symbolische Verknuepfung zaehlt mit,
/// weil der Nutzer in sie hineinnavigiert. Die Verknuepfungsregel selbst wohnt
/// allein im Durchlauf, der fuer sie „kein Treffer darunter" meldet, ohne in
/// sie hinabzusteigen (C2.13). Ein zweiter Schnitt hier hiesse, dass eine
/// Verknuepfung nie einen Befund bekaeme und damit von „noch nicht
/// entschieden" nicht zu unterscheiden waere.
///
/// Zweitens `Name traegt die Folge?` mit **nein**, gefragt ueber
/// `Ordnermodell::name_traegt_den_filter` und nicht ueber einen eigenen
/// Vergleich: der Zweig gehoert dem Pruefschritt, und ein zweiter Vergleich
/// hier hiesse, dass die Auftragsliste etwas anderes fuer passend hielte als
/// die Liste, die der Nutzer sieht. Fuer einen Ordner, dessen
/// eigener Name den Filtertext traegt, laeuft kein Durchlauf: seine
/// Sichtbarkeit steht mit dem Namen fest, und ein Befund ueber seinen
/// Unterbaum aenderte sie nicht (C3.14, C2.5, C2.8). Das ist die
/// Zustaendigkeitsgrenze zwischen den ersten beiden Bildern des Spec, und sie
/// steht hier am Eingang und nicht als Sonderfall an einem Ausgang.
///
/// **Ein ausgeblendeter Ordner steht mit in der Liste.** Die Regel, die ihn
/// heute wegblendet, ist der erste Zweig von `Ordnermodell::sichtbar`, und der
/// gehoert dorthin: ein zweites Mal hier gefragt waere die zweite Fassung
/// derselben Regel, die diese Runde gerade abgeschafft hat. Der Befund ist
/// dabei nicht umsonst — blendet der Nutzer die versteckten Eintraege waehrend
/// des Durchlaufs ein, steht die Zeile sofort richtig da.
fn auftraege(modell: &Ordnermodell) -> Vec<Auftrag> {
    modell
        .eintraege()
        .iter()
        .enumerate()
        .filter(|(_, eintrag)| eintrag.ist_ordner() || eintrag.ist_verknuepfung())
        .filter(|(index, _)| !modell.name_traegt_den_filter(*index as u32))
        .map(|(index, eintrag)| Auftrag {
            index: index as u32,
            name: eintrag.name.clone(),
        })
        .collect()
}

/// Holt die wartenden Meldungen eines einzelnen Tabs ab.
///
/// Zwei Kanaele, in dieser Reihenfolge: erst die Stapel des Lesevorgangs, dann
/// die Befunde des Durchlaufs. Die Reihenfolge traegt nichts — die beiden
/// koennen nie zugleich laufen, weil ein Durchlauf erst nach dem Abschluss des
/// Lesevorgangs beginnt —, und sie steht so herum, weil der Lesevorgang den
/// Bestand liefert, auf den sich jeder Befund bezieht.
fn einzug_je_tab(tab: &mut Tabinhalt) -> Einzug {
    let mut einzug = lesemeldungen_einziehen(tab);
    einzug.befunde_neu = befunde_einziehen(tab);
    einzug
}

/// Traegt die wartenden Befunde des Durchlaufs in das Modell ein.
///
/// Liefert, ob etwas eingetroffen ist. Der ganze Schwung geht in einem Stueck
/// an `Ordnermodell::befunde_setzen`, das **einmal** neu aufbaut; ein Setzer je
/// Ordner haette den Neuaufbau samt Sortierlauf so oft auf den Hauptfaden
/// gelegt, wie der angezeigte Ordner Unterordner hat
/// (`issues/260814-2145_*_befund-setzen-baut-die-ganze-sicht-neu-auf-und-der-durchlauf-ruft-es-je-ordner.md`).
///
/// **Der geschlossene Kanal raeumt den Durchlauf weg.** Er sagt, dass der
/// Arbeitsfaden geendet hat; das Feld stehen zu lassen hielte den Einzugstakt
/// fuer immer am Laufen. Was danach an Befunden fehlt, ist damit **nicht**
/// „kein Treffer darunter", sondern „nicht entschieden" — genau der
/// Unterschied, den `Befund` fuehrt (C3.13).
fn befunde_einziehen(tab: &mut Tabinhalt) -> bool {
    use std::sync::mpsc::TryRecvError;

    let Some(durchlauf) = tab.durchlauf.as_ref() else {
        return false;
    };
    let mut eingetroffen = Vec::new();
    let mut kanal_zu = false;
    loop {
        match durchlauf.befunde().try_recv() {
            Ok(meldung) => eingetroffen.push((
                meldung.index,
                if meldung.treffer {
                    Befund::Treffer
                } else {
                    Befund::KeinTreffer
                },
            )),
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => {
                kanal_zu = true;
                break;
            }
        }
    }
    if kanal_zu {
        tab.durchlauf = None;
    }
    if eingetroffen.is_empty() {
        return false;
    }
    tab.modell.befunde_setzen(eingetroffen);
    true
}

/// Holt die wartenden Stapel und den Abschluss des Lesevorgangs ab.
fn lesemeldungen_einziehen(tab: &mut Tabinhalt) -> Einzug {
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
        // Bewusst verworfen: geprueft wird der vorgemerkte Name nach der
        // zweiten Auffrischung, nicht die Antwort des Vormerkens selbst.
        let _ = liste.auswahl_auf_namen("neu.txt");
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

    // ------------------------------------------------------------------
    // Der Filtertext ueber einen Ordner-, Tab- und Auffrischungswechsel
    // ------------------------------------------------------------------

    /// Zwei Ordner, die es gibt.
    ///
    /// `ordner_setzen` und `waehlen` starten einen Lesevorgang, und der soll
    /// nicht gegen ein Nichts laufen. Geliefert hat er in diesen Proben nie
    /// etwas: `einziehen` wird nicht gerufen, der Bestand bleibt also der von
    /// Hand angehaengte.
    fn zwei_vorhandene_ordner() -> (String, String) {
        let einer = std::env::temp_dir().display().to_string();
        (einer, "/".to_owned())
    }

    /// C1.9: bei ausgeschalteter tiefer Suche faellt der Filtertext mit dem
    /// Ordner. Der Aufstieg geht durch dieselbe Stelle und zaehlt deshalb wie
    /// der Einstieg.
    #[test]
    fn ein_ordnerwechsel_leert_den_filtertext_wenn_die_tiefe_suche_aus_ist() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        liste.aktiver_mut().modell_mut().filtertext_setzen("rs");
        assert!(liste.aktiver().modell().filter_steht());

        liste.ordner_setzen(&dorthin, None);

        assert_eq!(
            liste.aktiver().modell().filtertext(),
            "",
            "ohne tiefe Suche faellt der Filtertext mit dem Ordner"
        );
        assert!(
            !liste.aktiver().modell().tief(),
            "der Schalter selbst bleibt, was er war"
        );
    }

    /// C1.10: bei eingeschalteter tiefer Suche uebersteht der Filtertext jeden
    /// Ordnerwechsel. Ohne diese Ausnahme haette das Modell der tiefen Ansicht
    /// auf der naechsten Ebene keinen Gegenstand mehr.
    #[test]
    fn mit_tiefer_suche_ueberlebt_der_filtertext_den_ordnerwechsel() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.filtertext_setzen("rs");
        modell.tief_setzen(true);

        liste.ordner_setzen(&dorthin, None);

        assert_eq!(liste.aktiver().modell().filtertext(), "rs");
        assert!(
            liste.aktiver().modell().tief(),
            "der Schalter geht mit dem Filtertext hinueber"
        );
        assert_eq!(
            liste.aktiver().ordner(),
            Path::new(&dorthin),
            "gewechselt wurde trotzdem"
        );
    }

    /// Der Filter der Tiefe geht auch ohne Filtertext hinueber: er ist ein
    /// Schalter des Tabs und keine Beigabe zum Text.
    #[test]
    fn die_tiefe_suche_geht_auch_ohne_filtertext_hinueber() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        liste.aktiver_mut().modell_mut().tief_setzen(true);

        liste.ordner_setzen(&dorthin, None);

        assert!(liste.aktiver().modell().tief());
        assert_eq!(liste.aktiver().modell().filtertext(), "");
    }

    /// Eine Auffrischung wechselt den Ordner nicht, also faellt der Filtertext
    /// auch bei ausgeschalteter tiefer Suche nicht.
    #[test]
    fn eine_auffrischung_laesst_den_filtertext_stehen() {
        let mut liste = gelesene_liste(&["a.rs", "b.txt"]);
        liste.aktiver_mut().modell_mut().filtertext_setzen("rs");
        assert_eq!(liste.aktiver().modell().zeilenzahl(), 1);

        liste.aktiven_neu_lesen();

        assert_eq!(
            liste.aktiver().modell().filtertext(),
            "rs",
            "eine Auffrischung wechselt den Ordner nicht"
        );
        assert_eq!(
            liste.aktiver().modell().zeilenzahl(),
            1,
            "und der stehende Bestand wird weiter gefiltert gezeigt"
        );
    }

    /// C1.8: der Filtertext gehoert dem Tab. Der Wechsel setzt nichts zurueck
    /// und traegt nichts hinueber — er zeigt schlicht das Modell des anderen
    /// Tabs.
    #[test]
    fn der_filtertext_gehoert_dem_tab_und_nicht_dem_fenster() {
        let vorhanden = std::env::temp_dir().display().to_string();
        let mut liste = liste(&[&vorhanden, &vorhanden]);
        liste.aktiver_mut().modell_mut().filtertext_setzen("rs");

        assert!(liste.waehlen(1));
        assert_eq!(
            liste.aktiver().modell().filtertext(),
            "",
            "der zweite Tab hat seinen eigenen, leeren Filtertext"
        );

        assert!(liste.waehlen(0));
        assert_eq!(
            liste.aktiver().modell().filtertext(),
            "rs",
            "und der erste hat den seinen behalten"
        );
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

    // ------------------------------------------------------------------
    // Der Durchlauf: Auftragsliste, Anlass, Abbruch
    // ------------------------------------------------------------------

    /// Ein Eintrag der genannten Art, ohne Groesse und ohne Zeitangabe.
    fn eintrag(name: &str, typ: krk_core::verzeichnis::Typ) -> krk_core::verzeichnis::Eintrag {
        krk_core::verzeichnis::Eintrag::neu(
            name.to_owned(),
            0,
            std::time::SystemTime::UNIX_EPOCH,
            typ,
        )
    }

    /// Ein fertig gelesenes Ordnermodell mit stehendem Filter.
    fn modell_mit(
        bestand: &[(&str, krk_core::verzeichnis::Typ)],
        filter: &str,
        tief: bool,
    ) -> Ordnermodell {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen(bestand.iter().map(|(name, typ)| eintrag(name, *typ)));
        modell.abschliessen();
        modell.tief_setzen(tief);
        modell.filtertext_setzen(filter);
        modell
    }

    /// Die Namen der Auftraege, in der Reihenfolge der Liste.
    fn auftragsnamen(modell: &Ordnermodell) -> Vec<String> {
        auftraege(modell)
            .into_iter()
            .map(|auftrag| auftrag.name)
            .collect()
    }

    /// Die Namen der Zeilen, die die Sicht gerade zeigt.
    fn zeilennamen(modell: &Ordnermodell) -> Vec<String> {
        modell
            .zeilen()
            .map(|eintrag| eintrag.name.clone())
            .collect()
    }

    /// C3.14: fuer einen Ordner, dessen eigener Name den Filtertext traegt,
    /// laeuft kein Durchlauf.
    #[test]
    fn die_auftragsliste_laesst_namentlich_passende_ordner_aus() {
        use krk_core::verzeichnis::Typ;

        let modell = modell_mit(
            &[
                ("src", Typ::Ordner),
                ("bilder", Typ::Ordner),
                ("liesmich.txt", Typ::Datei),
            ],
            "src",
            true,
        );
        assert_eq!(
            auftragsnamen(&modell),
            ["bilder"],
            "`src` traegt den Filtertext im Namen und ist damit ohne Durchlauf entschieden"
        );
    }

    /// C3.14, gezaehlt: ein Ordner, dessen saemtliche Unterordner den
    /// Filtertext im Namen tragen, liest keinen Unterbaum.
    #[test]
    fn ein_ordner_mit_lauter_passenden_unterordnern_stoesst_null_durchlaeufe_an() {
        use krk_core::verzeichnis::Typ;

        let modell = modell_mit(
            &[
                ("src", Typ::Ordner),
                ("src-alt", Typ::Ordner),
                ("meinsrc", Typ::Ordner),
                ("SRC-GROSS", Typ::Ordner),
                ("notiz.md", Typ::Datei),
            ],
            "src",
            true,
        );
        assert_eq!(
            auftraege(&modell).len(),
            0,
            "vier passende Ordner und eine Datei ergeben keinen einzigen Auftrag"
        );
    }

    /// C3.2: Dateien und namentlich passende Ordner stehen sofort und warten
    /// nicht auf den Durchlauf.
    ///
    /// Zwei Zaehlungen an einem Bestand: keine Datei steht in der
    /// Auftragsliste, und die Zeilen, die ohne jeden Befund schon stehen, sind
    /// genau die namentlich passenden.
    #[test]
    fn dateien_und_passende_ordner_warten_nicht_auf_den_durchlauf() {
        use krk_core::verzeichnis::Typ;

        let modell = modell_mit(
            &[
                ("notiz.txt", Typ::Datei),
                ("bild.png", Typ::Datei),
                ("notizen", Typ::Ordner),
                ("bilder", Typ::Ordner),
            ],
            "notiz",
            true,
        );
        assert_eq!(
            auftragsnamen(&modell),
            ["bilder"],
            "keine Datei bekommt einen Auftrag, und `notizen` ist am Namen entschieden"
        );
        assert_eq!(
            zeilennamen(&modell),
            ["notizen", "notiz.txt"],
            "beide stehen, obwohl noch kein Befund eingetroffen ist"
        );
    }

    /// C2.13: eine symbolische Verknuepfung auf einen Ordner bekommt einen
    /// Auftrag wie jeder Ordner; dass nicht in sie hinabgestiegen wird,
    /// entscheidet der Durchlauf und nicht diese Liste.
    #[test]
    fn eine_verknuepfung_bekommt_einen_auftrag_wie_jeder_ordner() {
        use krk_core::verzeichnis::Typ;

        let modell = modell_mit(
            &[
                ("anderswo", Typ::Verknuepfung),
                ("bilder", Typ::Ordner),
                ("a.txt", Typ::Datei),
            ],
            "zzz",
            true,
        );
        assert_eq!(auftragsnamen(&modell), ["anderswo", "bilder"]);
    }

    /// Der Eintragsindex und nicht die Zeile: die Auftraege ueberstehen damit
    /// jedes Umsortieren.
    #[test]
    fn ein_auftrag_traegt_den_eintragsindex_und_nicht_die_zeile() {
        use krk_core::verzeichnis::Typ;

        let modell = modell_mit(
            &[
                ("a.txt", Typ::Datei),
                ("bilder", Typ::Ordner),
                ("b.txt", Typ::Datei),
                ("daten", Typ::Ordner),
            ],
            "zzz",
            true,
        );
        let auftraege = auftraege(&modell);
        assert_eq!(auftraege.len(), 2);
        assert_eq!(
            (auftraege[0].index, auftraege[0].name.as_str()),
            (1, "bilder")
        );
        assert_eq!(
            (auftraege[1].index, auftraege[1].name.as_str()),
            (3, "daten")
        );
    }

    /// Eine Tabliste auf einem vorhandenen Ordner, mit stehendem Filter und
    /// eingeschaltetem "Deep", fertig gelesen.
    fn tiefe_liste(ordner: &Path, filter: &str) -> Tabliste {
        let mut liste = liste(&[&ordner.display().to_string()]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.tief_setzen(true);
        modell.filtertext_setzen(filter);
        liste.tabs[0].gelesen = true;
        liste
    }

    /// C3.6: je Tab laeuft nie mehr als einer, und ein zweiter Anstoss loest
    /// den ersten ab.
    #[test]
    fn je_tab_laeuft_nie_mehr_als_ein_durchlauf() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-einer");
        ordner.ordner("bilder");
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);

        assert!(liste.durchlauf_nachziehen(), "der erste Anstoss startet");
        assert!(liste.durchlauf_nachziehen(), "der zweite ebenso");
        assert_eq!(
            liste
                .tabs
                .iter()
                .filter(|tab| tab.durchlauf.is_some())
                .count(),
            1,
            "und danach steht genau einer da"
        );
    }

    /// C3.7 und C3.5: ohne Filtertext, ohne "Deep" und vor dem Abschluss des
    /// Lesevorgangs beginnt keiner.
    #[test]
    fn ohne_seine_drei_bedingungen_beginnt_kein_durchlauf() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-bedingungen");
        ordner.ordner("bilder");

        // Ohne "Deep".
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);
        liste.aktiver_mut().modell_mut().tief_setzen(false);
        assert!(!liste.durchlauf_nachziehen(), "\"Deep\" ist aus");
        assert!(!liste.arbeitet_noch());

        // Ohne Filtertext.
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);
        liste.aktiver_mut().modell_mut().filter_leeren();
        assert!(!liste.durchlauf_nachziehen(), "der Filtertext ist leer");
        assert!(!liste.arbeitet_noch());

        // Solange der Lesevorgang laeuft, steht der Bestand noch nicht.
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);
        liste.tabs[0].gelesen = false;
        assert!(!liste.durchlauf_nachziehen(), "der Ordner ist nicht fertig");
    }

    /// C3.7: das Ausschalten von "Deep" bricht den laufenden Durchlauf ab.
    #[test]
    fn das_ausschalten_von_deep_bricht_den_durchlauf_ab() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-deep-aus");
        ordner.ordner("bilder");
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);
        assert!(liste.durchlauf_nachziehen());
        assert!(liste.arbeitet_noch(), "der Einzugstakt hat noch zu tun");

        liste.aktiver_mut().modell_mut().tief_setzen(false);
        assert!(!liste.durchlauf_nachziehen());
        assert!(
            !liste.arbeitet_noch(),
            "mit dem Schalter faellt der Durchlauf, und der Takt darf enden"
        );
    }

    /// C3.6: eine Aenderung des Filtertexts bricht ab und stoesst neu an.
    #[test]
    fn ein_weiteres_zeichen_loest_den_laufenden_durchlauf_ab() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-zeichen");
        ordner.ordner("bilder");
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);
        assert!(liste.durchlauf_nachziehen());

        liste.aktiver_mut().modell_mut().zeichen_anhaengen('q');
        assert!(
            liste.durchlauf_nachziehen(),
            "der neue Filtertext bekommt seinen eigenen Durchlauf"
        );
        assert_eq!(
            liste
                .tabs
                .iter()
                .filter(|tab| tab.durchlauf.is_some())
                .count(),
            1
        );
    }

    /// Ein Tabwechsel bricht nicht ab: ein verdeckter Tab fuellt sich still
    /// weiter, wie er es beim Lesevorgang tut.
    #[test]
    fn ein_tabwechsel_laesst_den_durchlauf_stehen() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-tabwechsel");
        ordner.ordner("bilder");
        let vorhanden = ordner.pfad().display().to_string();
        let mut liste = liste(&[&vorhanden, "/b"]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.tief_setzen(true);
        modell.filtertext_setzen("zzz");
        modell.anhaengen([eintrag("bilder", Typ::Ordner)]);
        liste.tabs[0].gelesen = true;
        assert!(liste.durchlauf_nachziehen());

        assert!(liste.naechster(), "auf den zweiten Tab wechseln");
        assert!(
            liste.tabs[0].durchlauf.is_some(),
            "der verdeckte Tab arbeitet weiter"
        );
        assert!(liste.arbeitet_noch());
    }

    /// `Tabliste::abbrechen` nimmt den Durchlauf mit, wie es den Lesevorgang
    /// mitnimmt.
    #[test]
    fn das_abbrechen_des_fensters_nimmt_den_durchlauf_mit() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-abbrechen");
        ordner.ordner("bilder");
        let mut liste = tiefe_liste(ordner.pfad(), "zzz");
        liste
            .aktiver_mut()
            .modell_mut()
            .anhaengen([eintrag("bilder", Typ::Ordner)]);
        assert!(liste.durchlauf_nachziehen());

        liste.abbrechen();
        assert!(!liste.arbeitet_noch());
    }

    /// C3.3 und C3.11 am Modell: der Tab haelt den Durchlauf, zieht seine
    /// Befunde ein, und die Zeile des tiefen Treffers erscheint.
    ///
    /// Der ganze Weg von F2 ohne AppKit: Lesevorgang, `Einzug::fertig`,
    /// Auftragsliste, Arbeitsfaden, Befundkanal, `befunde_setzen`.
    #[test]
    fn der_tab_zieht_die_befunde_ein_und_die_zeile_des_tiefen_treffers_erscheint() {
        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-einzug");
        let daten = ordner.ordner("daten");
        std::fs::create_dir_all(daten.join("unten")).expect("Unterordner anlegen");
        std::fs::write(daten.join("unten").join("ziel-xyz.txt"), b"x").expect("Datei anlegen");
        ordner.ordner("leer");
        ordner.datei("oben.txt", b"x");

        let mut liste = liste(&[&ordner.pfad().display().to_string()]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.tief_setzen(true);
        modell.filtertext_setzen("xyz");
        liste.sichtbaren_lesen();

        // Der Einzugstakt, von Hand gedreht: erst der Lesevorgang, dann die
        // Befunde. Die Schranke ist grosszuegig und die Schleife endet an der
        // Sache und nicht an ihr.
        let mut takte = 0;
        while liste.arbeitet_noch() {
            let _ = liste.einziehen();
            takte += 1;
            assert!(takte < 2_000, "der Durchlauf ist nicht zum Ende gekommen");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        assert_eq!(
            zeilennamen(liste.aktiver().modell()),
            ["daten"],
            "`daten` traegt den Treffer unter sich, `leer` nicht, und `oben.txt` passt nicht"
        );
    }

    /// C2.10 und C6.1: eine Zeile, die allein wegen eines tiefen Treffers
    /// steht, liegt trotzdem im angezeigten Ordner.
    ///
    /// Gezaehlt und nicht behauptet: der Pfad, den
    /// `kommandos::operationen::betroffene` baut, hat unter dem angezeigten
    /// Ordner genau **einen** Bestandteil. Traege `Eintrag` einen Pfad oder
    /// naehme die Sicht Zeilen aus einem anderen Ordner auf, stuenden hier
    /// mehr.
    #[test]
    fn eine_zeile_aus_einem_tiefen_treffer_liegt_im_angezeigten_ordner() {
        use krk_core::verzeichnis::Typ;
        use krk_core::verzeichnis::modell::Befund;

        let angezeigt = Path::new("/Users/k1/Projekte");
        let mut modell = modell_mit(
            &[("daten", Typ::Ordner), ("oben.txt", Typ::Datei)],
            "xyz",
            true,
        );
        modell.befunde_setzen([(0, Befund::Treffer)]);
        assert_eq!(zeilennamen(&modell), ["daten"]);

        modell.auswahl_setzen(Some(0));
        let betroffene = crate::kommandos::operationen::betroffene(&modell, angezeigt);
        assert_eq!(betroffene.pfade, [angezeigt.join("daten")]);
        assert_eq!(
            betroffene.pfade[0]
                .strip_prefix(angezeigt)
                .expect("der Pfad liegt unter dem angezeigten Ordner")
                .components()
                .count(),
            1,
            "genau ein Bestandteil unter dem angezeigten Ordner"
        );
    }

    /// C2.9: die Dateiliste bleibt eine flache Tabelle mit vier Spalten.
    ///
    /// Gezaehlt an zwei Stellen statt behauptet: die Zahl der Spalten am
    /// Aufzaehlungstyp, der sie fuehrt, und das Vorkommen von `NSOutlineView`
    /// im Quelltext der Ansicht. Der Text ist ueber `include_str!` gebunden;
    /// wird die Datei verschoben, haelt der Bau an, und das ist der richtige
    /// Zeitpunkt, diese Probe nachzuziehen.
    #[test]
    fn die_dateiliste_bleibt_flach_und_hat_vier_spalten() {
        assert_eq!(crate::spalten::Spalte::ALLE.len(), 4);
        let quelltext = include_str!("appkit/tabelle.rs");
        assert_eq!(
            quelltext.matches("NSOutlineView").count(),
            0,
            "keine NSOutlineView, kein Aufklappzeichen"
        );
    }

    /// C2.11: `angezeigtedatei::welche` bleibt bei zwei Quellen.
    ///
    /// Gezaehlt am Quelltext des Moduls: jede Quelle ist genau ein `return
    /// Some(`, und eine dritte waere eine dritte. Die Fallunterscheidung selbst
    /// prueft das Modul in seinem eigenen Probenmodul ueber alle acht
    /// Kombinationen.
    #[test]
    fn die_angezeigte_datei_bleibt_bei_zwei_quellen() {
        let quelltext = include_str!("angezeigtedatei.rs");
        let rumpf = quelltext
            .split("#[cfg(test)]")
            .next()
            .expect("der Rumpf steht vor dem Probenmodul");
        assert_eq!(
            rumpf.matches("return Some(").count(),
            2,
            "die Vorschau und der Editor, und keine dritte Quelle"
        );
    }
}
