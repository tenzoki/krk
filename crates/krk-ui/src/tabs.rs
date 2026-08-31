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
use krk_core::git::Marke;
use krk_core::git::lauf::{Gitfrage, Gitlauf, Gitmeldung};
use krk_core::verzeichnis::modell::Befund;
use krk_core::verzeichnis::{Abschluss, Durchlauf, Lesevorgang, Meldung, Ordnermodell};

use crate::gitmodell::Gitmodell;

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
    /// Wie viele Dateien der letzte Durchlauf wegen ihrer Groesse **nicht**
    /// gelesen hat.
    ///
    /// Ein Feld am Tab und nicht die Frage an den Durchlauf, und der Grund ist
    /// die Lebensdauer: der [`Durchlauf`] faellt, sobald sein Kanal schliesst,
    /// die Zahl soll danach aber weiter dastehen. Sonst saehe der Nutzer sie
    /// bei einem kleinen Ordner nie — dort ist der Lauf durch, bevor die
    /// Statuszeile das naechste Mal rechnet —, und der Groessenhinweis waere
    /// eine Anzeige, die nur bei langen Laeufen aufblitzt.
    ///
    /// Sie gehoert dem Lauf und nicht dem Ordner: `Tabliste::lesen_starten`
    /// und `Tabliste::durchlauf_nachziehen_an` setzen sie auf null, wo ein
    /// Lauf faellt oder beginnt.
    zu_gross: u64,
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
    /// Der Gitlauf ueber den Ordner dieses Tabs, falls einer laeuft.
    ///
    /// **Hoechstens einer je Dateifenster** (A10, C7.11), und deshalb ein Feld
    /// und keine Sammlung: das Feld zu setzen laesst den alten fallen, und sein
    /// `Drop` bricht ihn ab. Zwei Laeufe koennen damit nicht nebeneinander
    /// **stehen**, ohne dass eine Zeile es verhinderte.
    ///
    /// **Zwei Faeden koennen sich trotzdem kurz ueberschneiden**, und das Feld
    /// sagt darueber nichts: `Drop` fordert den Abbruch an und wartet nicht.
    /// Wie lange der aeltere Faden noch laeuft, sagt `git/lauf.rs`; seit dem
    /// 260831 bricht er mitten im Statusstrom ab statt erst an dessen Ende.
    ///
    /// Neben `lesevorgang` und nicht in ihm, wie der Durchlauf: die beiden
    /// beantworten verschiedene Fragen und beginnen zwar zugleich, enden aber
    /// nicht zugleich. Wer ihn faellen laesst, steht bei
    /// [`Tabliste::gitlauf_nachziehen_an`].
    gitlauf: Option<Gitlauf>,
    /// Was der Git-Bereich fuer diesen Tab zeigt.
    gitmodell: Gitmodell,
    /// Die Generation des Lesevorgangs, zu dem der laufende Gitlauf gehoert.
    ///
    /// **Sie reist nicht im Kanal mit, und der Grund steht bei
    /// [`Gitlauf::starten`]:** jeder Tab liest allein aus dem Kanal des Laufs,
    /// den er selbst haelt. Gebraucht wird sie trotzdem, weil der Befund einen
    /// **Namen** traegt, den auch ein neuer Ordner fuehren kann;
    /// `Ordnermodell::gitmarken_setzen` haelt sie deshalb gegen die Generation
    /// des Bestands (C7.5). Gesetzt wird sie an der einen Stelle, an der ein
    /// Lauf entsteht, und dort aus dem Modell des Tabs.
    gitgeneration: u64,
    /// Die Markenmeldung, solange der Bestand sie noch nicht annehmen kann.
    ///
    /// **Warum sie hier liegt und nicht im Kanal.** Der Plan der Runde 23 sagt,
    /// die Meldung werde erst aus dem Kanal genommen, wenn der Tab gelesen ist.
    /// Wortwoertlich ist das mit `std::sync::mpsc::Receiver` nicht zu bauen: er
    /// kennt kein Vorausschauen, und die drei Meldungen teilen sich einen Kanal
    /// in fester Reihenfolge. Wer wartete, bis der Tab gelesen ist, hielte
    /// **Kopf und Verlauf** genauso lange zurueck — und genau das soll dieser
    /// Schritt vermeiden, denn der Branchname wartete dann in einem Ordner mit
    /// hunderttausend Eintraegen vier Sekunden auf einen Bestand, den er nicht
    /// braucht.
    ///
    /// Das Feld ist deshalb dieselbe Wartestelle, einen Schritt spaeter, mit
    /// derselben Zusage: **nichts erreicht das Ordnermodell, bevor der Bestand
    /// steht** (A8, C7.4). Sie eintreffend zu verwerfen waere die Alternative
    /// gewesen, und sie waere falsch: `gitmarken_setzen` weist ab, solange der
    /// Ersatz aussteht, und der Befund waere danach fuer immer weg.
    ///
    /// Sie faellt mit dem Lauf, zu dem sie gehoert; wo, steht bei
    /// [`Tabliste::gitlauf_nachziehen_an`].
    wartende_marken: Option<Vec<(String, Marke)>>,
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
            zu_gross: 0,
            wunschauswahl: zustand.auswahl.clone(),
            bildlauf: zustand.bildlauf,
            bildlauf_offen: zustand.bildlauf > 0.0,
            meldung: None,
            gelesen: false,
            gitlauf: None,
            gitmodell: Gitmodell::neu(),
            gitgeneration: GENERATION_LEER,
            wartende_marken: None,
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

    /// Was der Git-Bereich fuer diesen Tab zeigt.
    ///
    /// **Nur zu lesen, mit einer benannten Ausnahme.** Kopf, Verlauf und
    /// Zusammenfassung schreibt allein der Einzugstakt, und zurueckgesetzt wird
    /// das Modell allein aus [`Tabliste::gitlauf_nachziehen_an`]; ein zweiter
    /// Schreiber dieser Felder waere eine zweite Quelle fuer denselben Stand.
    ///
    /// **Die Ausnahme ist die Auswahl**, und sie geht ueber
    /// [`Self::gitauswahl_setzen`] darunter und ueber keinen anderen Weg. Der
    /// Nutzer hat sie am 260831 hierher gelegt statt in die Ansicht
    /// (`260831-0120_*_wo-wohnt-die-auswahl-der-verlaufsliste-im-gitfenster-oder-im-gitmodell.md`,
    /// Moeglichkeit 2): es gibt **ein** Gitfenster und **ein Gitmodell je
    /// Dateifenster und Tab**. **Wie weit sie damit reicht, hat der Nutzer am
    /// 260831 ein zweites Mal entschieden**
    /// (`260831-1815_*_faellt-die-auswahl-der-verlaufsliste-mit-dem-tabwechsel-oder-ueberlebt-sie-ihn-wie-am-260831-entschieden.md`,
    /// Moeglichkeit 2), und die Antwort hat zwei Haelften: sie **uebersteht
    /// den Wechsel des aktiven Dateifensters**, weil jede [`Tabliste`] ihr
    /// eigenes Gitmodell haelt — und sie **faellt mit dem Tabwechsel**, weil
    /// [`Tabliste::waehlen`] fuer den verlassenen Tab
    /// [`Tabliste::gitlauf_nachziehen_an`] ruft und das dessen Gitmodell
    /// unbedingt zuruecksetzt. Sie ist damit das eine Feld, das der
    /// Git-Bereich fuellt und der Einzugstakt nicht — kein zweiter
    /// Schreiber auf demselben Feld, sondern ein zweites Feld mit einem eigenen
    /// Schreiber. Genau daran ist Moeglichkeit 3 des Datensatzes gescheitert,
    /// die `zeigen` das Modell veraenderlich gegeben haette.
    pub fn gitmodell(&self) -> &Gitmodell {
        &self.gitmodell
    }

    /// Waehlt den Commit an dieser Stelle des Verlaufs aus (C3.4, C4.2).
    ///
    /// **Die benannte Ausnahme von der Zusage darueber**, und der einzige Weg,
    /// auf dem ein Schreiber von aussen an das Gitmodell kommt. Der Weg hinein
    /// ist der Auswahlmelder des Git-Bereichs: die Verlaufsliste bewegt ihre
    /// Auswahl, meldet sie nach oben, und der Anwendungsdelegierte traegt sie
    /// hier ein. Geschrieben wird eine Zahl im Arbeitsspeicher; das Repository
    /// bleibt unberuehrt (E8).
    ///
    /// Eine Stelle jenseits des Verlaufs raeumt die Auswahl, statt sie auf
    /// einen Commit zu setzen, den es nicht gibt; die Regel steht in
    /// [`Gitmodell::auswahl_setzen`] und nicht hier.
    pub fn gitauswahl_setzen(&mut self, stelle: Option<usize>) {
        self.gitmodell.auswahl_setzen(stelle);
    }

    /// Ob gerade ein Lesevorgang laeuft.
    pub fn liest(&self) -> bool {
        self.lesevorgang.is_some()
    }

    /// Wie viele Dateien der letzte Durchlauf wegen ihrer Groesse **nicht**
    /// gelesen hat.
    ///
    /// Null, wo kein Lauf war oder keine Datei zu gross war. Der Wert steht
    /// auch nach dem Ende des Laufs; warum, steht am Feld.
    ///
    /// Ihr einer Ableser ist der Groessenhinweis der Statuszeile: er faellt in
    /// `appkit::statuszeile::Filterstand::zu_gross` und von dort in den Satz
    /// des Filterstands.
    pub fn zu_gross(&self) -> u64 {
        self.zu_gross
    }

    /// Ob gerade ein Durchlauf laeuft, der Dateiinhalte liest.
    ///
    /// **Zwei Bedingungen, und die zweite ist noetig.** Ein Durchlauf allein
    /// genuegt nicht: ueber einen Unterbaum laeuft auch der Namensdurchlauf der
    /// Runde 10, und der liest keine Datei. Erst mit [`Ordnermodell::inhalt_wirkt`]
    /// steht fest, dass der Lauf Inhalte oeffnet, und nur dann ist der
    /// Lesehinweis der Statuszeile wahr. Ohne diese zweite Bedingung waere der
    /// Satz des Filterstands bei ausgeschaltetem "Content" nicht mehr
    /// zeichengleich mit dem der Runde 10.
    ///
    /// **Die Frage steht hier und nicht beim Ableser**, weil allein dieser Typ
    /// den [`Durchlauf`] haelt und weil sie hier ohne AppKit pruefbar ist. Ihr
    /// Ableser ist `appkit::statuszeile::Filterstand::liest_inhalt`.
    pub fn liest_inhalt(&self) -> bool {
        self.durchlauf.is_some() && self.modell.inhalt_wirkt()
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
/// Heute bindet der eine Aufrufer den Wert und wertet jedes Feld aus, das schon
/// eine Ansicht hat (`Dateitabelle::einziehen` in `crate::appkit::tabelle`); das
/// `#[must_use]` haelt das fuer den zweiten fest. Die eine Ausnahme ist
/// [`Einzug::gitkopf_neu`]: es zeichnet in der Tabelle nichts neu, sondern
/// reist als Meldung weiter, und die Begruendung steht an seinem Feld.
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
    /// Der Gitlauf des sichtbaren Tabs hat Kopf, Verlauf oder Zusammenfassung
    /// geliefert.
    ///
    /// **Das eine Feld, das die Tabelle nur weiterreicht.** Die Dateiliste hat
    /// damit nichts zu tun — Kopf und Verlauf stehen in keiner ihrer Spalten —,
    /// und sie zeichnet auf dieses Feld hin nichts neu; sie meldet es ueber
    /// ihren `gitwechsel`-Rueckruf an
    /// `Anwendungsdelegierter::gitanzeige_nachziehen`, der die drei Flaechen
    /// des Git-Bereichs schreibt. **Der Weg ueber die Tabelle ist der einzige,
    /// den es gibt**: der Einzugstakt haengt an ihr, und der
    /// Anwendungsdelegierte kaeme sonst nie zu dem Zeitpunkt, an dem die
    /// Antwort eintrifft.
    pub gitkopf_neu: bool,
    /// Der Gitlauf des sichtbaren Tabs hat Marken in sein Ordnermodell
    /// getragen.
    ///
    /// Neben `befunde_neu`, weil die Ansicht darauf **anders** antwortet: eine
    /// Marke entscheidet nicht, ob eine Zeile steht, sondern nur, was in einer
    /// ihrer Zellen steht. Die Sichtreihenfolge bleibt, die ausgewaehlte Zeile
    /// behaelt ihre Stelle, und `auswahl_anzeigen` bleibt deshalb aus.
    pub gitmarken_neu: bool,
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
    /// Ob ueberhaupt jemand den Gitbefund sehen will.
    ///
    /// Wahr, sobald der Git-Bereich steht **oder** die Markenspalte steht; die
    /// Oder-Verknuepfung rechnet der Anwendungsdelegierte, denn hier ist weder
    /// die Sichtbarkeit eines Bereichs noch die einer Spalte bekannt. Steht
    /// keines von beidem, entsteht kein Lauf: ein Statusabruf, dessen Antwort
    /// niemand anzeigt, kostete Faeden und Deskriptoren fuer nichts.
    ///
    /// **Ab Werk falsch, und das ist der Anfangswert und nicht der
    /// Auslieferungszustand.** Die Markenspalte steht ab Werk (A13), also wird
    /// der Wert beim Aufbau der Oberflaeche sofort auf wahr gezogen; ihn hier
    /// vorwegzunehmen hiesse, eine Sichtbarkeit zu behaupten, die diese Datei
    /// nicht kennt und die aus einer `session.toml` auch anders kommen kann.
    git_gefragt: bool,
    /// Die laufende Nummer des zuletzt gestarteten Gitlaufs.
    ///
    /// Eine eigene Zaehlung neben `letzte_generation` und `letzter_durchlauf`,
    /// aus demselben Grund wie dort: sie benennt allein den Arbeitsfaden
    /// (`krk-gitlauf-<n>`), damit zwei Laeufe desselben Tabs in einem
    /// Fadenprotokoll auseinanderzuhalten sind. Was der Befund gegen den
    /// Bestand haelt, ist die **Generation** und nicht diese Nummer; sie steht
    /// am Tab in `gitgeneration`.
    letzter_gitlauf: u64,
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
            git_gefragt: false,
            letzter_gitlauf: 0,
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
    ///
    /// **Der Durchlauf des verlassenen Tabs endet hier** (C4.5). Gerufen wird
    /// dafuer [`Tabliste::durchlauf_nachziehen_an`] auf der verlassenen Stelle,
    /// und zwar erst, nachdem `aktiv` schon umgesetzt ist: die Sichtbarkeit ist
    /// eine der Bedingungen jener Methode, also faellt der Lauf dort und
    /// beginnt nicht neu. Ein Zweig nach der Art des Laufs faellt hier nicht
    /// an; die Regel steht ganz in der einen Methode.
    pub fn waehlen(&mut self, stelle: usize) -> bool {
        if stelle >= self.tabs.len() || stelle == self.aktiv {
            return false;
        }
        let verlassen = self.aktiv;
        self.aktiv = stelle;
        // Der Wert sagt, ob jetzt ein Durchlauf laeuft. Auf der verlassenen
        // Stelle ist die Antwort seit dem Nutzerentscheid vom 260816-1410
        // immer "nein", und der Ruf steht hier des Abbruchs wegen.
        let _ = self.durchlauf_nachziehen_an(verlassen);
        // Und der Gitlauf des verlassenen Tabs faellt an derselben Stelle und
        // aus demselben Grund: die Sichtbarkeit ist eine seiner Bedingungen.
        let _ = self.gitlauf_nachziehen_an(verlassen);
        self.ungelesenen_aktiven_nachlesen();
        // Der Tabwechsel ist einer der vier Ausloeser aus A9, also bekommt der
        // neue sichtbare Tab seinen Lauf. Hat der Nachleser darueber schon
        // einen Lesevorgang gestartet, steht er bereits — `lesen_starten`
        // stoesst ihn zugleich mit dem Lesen an —, und ein zweiter waere
        // derselbe Lauf ein zweites Mal.
        if self.tabs[stelle].gitlauf.is_none() {
            let _ = self.gitlauf_nachziehen_an(stelle);
        }
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
    /// # Die eine Stelle, an der ein Ordnerwechsel den Filter traegt
    ///
    /// Der bisherige [`Tabinhalt`] faellt hier, und mit ihm sein
    /// [`Ordnermodell`]; was der Tab ueber den Wechsel hinweg behaelt, steht
    /// deshalb genau hier und nirgends sonst. Bis zum 260815 waren das
    /// Sortierung und Verstecke; seither kommen der Filter der Tiefe und der
    /// Filtertext dazu, in derselben Bauart und aus demselben Grund. Seit dem
    /// 260816 ist der Filter des Inhalts die fuenfte Uebertragung, und sie hat
    /// keine eigene Regel: der Stand von „Content" uebersteht den Wechsel
    /// unbedingt, wie der von „Deep" (C1.12, C2.4). Genau davon lebt die
    /// Zusage, dass der neue Ordner sofort anfaengt, seine Dateien zu lesen.
    ///
    /// **Eine Regel und keine Fallunterscheidung:** der Filtertext uebersteht
    /// jeden Ordnerwechsel, gleich ob der Filter der Tiefe an oder aus ist
    /// (C1.9, C1.10). Kein Ordnerwechsel und keine Auffrischung loescht ihn.
    /// Bis zum 260815-0955 leerte ein Wechsel bei ausgeschaltetem Filter der
    /// Tiefe den Text; der Nutzerentscheid zu
    /// `decisions/260814-1830_*_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`
    /// (Moeglichkeit 2) hat aus der Ausnahme die Regel gemacht.
    ///
    /// **Die Wege, auf denen der Filtertext verschwindet, sind nicht
    /// abschliessend aufgezaehlt.** Der Nutzer nimmt ihn mit `Esc` weg oder
    /// Zeichen fuer Zeichen ueber die Rueckschritt-Taste; daneben faellt er
    /// mit dem Tab, der ihn haelt, und mit der Sitzung. C1.9 sagt seit dem
    /// 260815 ausdruecklich, dass das keine Liste von zwei Tastenwegen ist:
    /// drei weitere Wege nehmen ihn weg, ohne dass der Nutzer den Filter
    /// anfasst. Es sind das Schliessen des
    /// letzten Tabs ([`Tabliste::schliessen`] baut dort einen frischen
    /// [`Tabinhalt`]), der Auswurf eines Datentraegers unter einem verdeckten
    /// Tab ([`Tabliste::verdeckten_tab_setzen`]) und der Neustart, weil
    /// `krk_core::ablage::sitzung::Tab` den Filtertext nicht fuehrt. Erhoben
    /// als
    /// `shared/issues/260815-1047_*_c1-9-und-der-doc-kommentar-nennen-zwei-loeschwege-des-filtertextes-der-baum-hat-fuenf.md`.
    ///
    /// **Die Sichtbarkeit des stehenden Filtertextes ist nicht zugesagt.** Der
    /// Nutzerentscheid haengt an ihr: die Statuszeile soll den Filtertext samt
    /// Trefferzahl nennen, sonst hielte der Nutzer den neuen Ordner fuer fast
    /// leer. Geprueft war dafuer allein `statuszeile::filterstand_text`, das
    /// den Satz **baut**. Ob er die Zeile erreicht, entscheidet
    /// `statuszeile::zeile` ueber die Rangfolge `Rang::ALLE`, und dort ist der
    /// Filterstand Rang 5 von 6. Die Ordnung ist erst der Rang und dann die
    /// aktive Seite, also stehen vier Raenge ueber ihm, drei davon auch dem
    /// anderen Dateifenster offen. Eine Fenstermeldung (Rang 3) des inaktiven
    /// Dateifensters verdraengt den Filterstand und wird allein vom Ordner-
    /// oder Tabwechsel **derselben** Seite geraeumt. Wer in dieser Lage
    /// filtert, sieht seinen Filtertext nicht, auch nicht im Augenblick des
    /// Tippens und ueber jeden folgenden Ordnerwechsel hinweg. Der Weg besteht
    /// seit der Runde 10 unveraendert; was der 260815 aendert, ist die
    /// Haeufigkeit: ein vergessener Filter war die Ausnahme und ist der
    /// Regelfall. Offen als
    /// `shared/issues/260815-1047_*_die-bedingung-der-moeglichkeit-2-ist-an-filterstand-text-geprueft-und-nicht-an-der-rangfolge.md`.
    ///
    /// **Der Aufstieg braucht keine eigene Zeile.** Er geht wie der Einstieg
    /// durch diese Stelle, und damit gilt fuer ihn dieselbe Regel (C1.9).
    ///
    /// **Weder der Filtertext noch die beiden Filterschalter gehen in die
    /// Sitzung** (C2.5). Alle drei werden hier vom alten Modell in das neue
    /// getragen und nicht ueber [`Tabzustand`], der `session.toml` schreibt:
    /// ein wiederhergestellter Filter der Tiefe oder des Inhalts ohne
    /// Filtertext waere ein Zustand, den nichts anzeigt und der nichts tut.
    pub fn ordner_setzen(&mut self, ordner: impl Into<PathBuf>, auswahl: Option<String>) {
        let stelle = self.aktiv;
        let sortierung = self.tabs[stelle].modell.sortierung();
        let verstecke = self.tabs[stelle].modell.verstecke_ausgeblendet();
        let tief = self.tabs[stelle].modell.tief();
        let inhalt = self.tabs[stelle].modell.inhalt();
        // Die vierte Uebertragung, in derselben Bauart wie die drei darueber
        // und ohne Bedingung: der Filtertext geht hinueber, gleich wie `tief`
        // steht (C1.9, C1.10). Bis zum Nutzerentscheid vom 260815-0955 zu
        // `decisions/260814-1830_*_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`
        // stand hier ein `filtertext_ueberlebt`, das die offene Antwort trug.
        // Es ist ersatzlos entfallen: mit der einen Regel truege es nur noch
        // ein `true` und liesse eine Fallunterscheidung vermuten, die es nicht
        // gibt. Der Doc-Kommentar darueber schreibt die Regel aus.
        let filtertext = self.tabs[stelle].modell.filtertext().to_owned();
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
        modell.inhalt_setzen(inhalt);
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
    /// dafuer.** Eine Auffrischung wechselt den Ordner nicht, also faellt das
    /// [`Ordnermodell`] hier gar nicht erst: der Tab behaelt es samt seinem
    /// Filtertext, ohne dass die Uebertragung aus
    /// [`Tabliste::ordner_setzen`] etwas zu tun haette. Was der neue
    /// Lesevorgang liefert, geht durch denselben Filter wie zuvor.
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
    /// **Die eine Bedingung des Einzugstakts**, und sie zaehlt alle drei
    /// Kanaele: den des Lesevorgangs, den des Durchlaufs und den des Gitlaufs.
    /// Der Takt bedient alle drei, und eine Bedingung, die einen ausliesse,
    /// hielte ihn an, waehrend dessen Befunde noch unterwegs sind — die Liste
    /// bliebe dann stehen und wuechse erst beim naechsten Anlass weiter.
    ///
    /// **Der dritte Kanal ist der laengste.** Ein Statuslauf kostet gemessen 12
    /// bis 164 ms und ueberlebt damit den Lesevorgang eines kleinen Ordners um
    /// ein Vielfaches; ohne ihn hier hielte der Takt an, bevor die Marken da
    /// sind, und die Spalte bliebe leer, bis irgendetwas anderes ihn wieder
    /// anwirft.
    pub fn arbeitet_noch(&self) -> bool {
        self.liest_noch()
            || self
                .tabs
                .iter()
                .any(|tab| tab.durchlauf.is_some() || tab.gitlauf.is_some())
    }

    /// Bricht den Durchlauf des sichtbaren Tabs ab und stoesst, wenn seine
    /// Bedingungen stehen, einen neuen an.
    ///
    /// **Die eine Stelle, an der ein Durchlauf entsteht und vergeht**, und
    /// damit die Antwort auf beide Haelften von C3.6 und C3.7 zugleich. Zu
    /// rufen ist sie von jedem Anlass, der eine seiner Eingaben aendert: von
    /// jeder Aenderung des Filtertexts, vom Umschalten eines der beiden Filter,
    /// seit dem 260816 vom Ein- und Ausblenden der versteckten Eintraege — sie
    /// entscheiden seither mit, wer einen Auftrag bekommt — und vom Einzugstakt,
    /// sobald ein Tab fertig gelesen ist. Die uebrigen
    /// Anlaesse brauchen keinen Ruf, weil der [`Tabinhalt`] mit dem Durchlauf
    /// dort ohnehin faellt: der Ordnerwechsel tauscht ihn aus, das Schliessen
    /// nimmt ihn weg, und [`Tabliste::lesen_starten`] setzt das Feld
    /// ausdruecklich zurueck, weil der Bestand gerade abgeloest wird und ein
    /// Eintragsindex des alten Bestands auf einen beliebigen Eintrag des neuen
    /// zeigte.
    ///
    /// **Ein Tabwechsel ruft hier, und ein verdeckter Tab haelt keinen
    /// Durchlauf** (C4.5). Bis zum 260816 stand hier das Gegenteil: die Runde
    /// 10 liess den verlassenen Tab weiterlaufen, weil ein Namensdurchlauf
    /// ueber Verzeichnismetadaten in Millisekunden durch ist und ein Abbruch
    /// dem Nutzer die Arbeit naehme, die er beim Zurueckwechseln braeuchte.
    /// Mit dem Inhaltsfilter wiegt das anders: ein Lauf oeffnet und liest
    /// Dateien bis 1 MB, ueber einen Unterbaum minutenlang, fuer einen Tab, den
    /// niemand ansieht, und er nimmt Deskriptoren aus einem Vorrat, den Editor,
    /// Vorschau, Kopiervorgaenge und beide Lesevorgaenge teilen. Der Nutzer hat
    /// am 260816-1410 Moeglichkeit 1 von
    /// `decisions/260816-1359_*_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md`
    /// gewaehlt: **eine Regel und kein Zweig nach der Art des Laufs.** Der Preis
    /// ist benannt und angenommen — wer mit stehendem Filtertext zwischen zwei
    /// Tabs hin und her wechselt, laesst den Unterbaum jedes Mal von vorn
    /// abschreiten.
    ///
    /// Getragen wird die Regel von der Sichtbarkeitsbedingung im Rumpf und
    /// nicht von einem Zweig in [`Tabliste::waehlen`]. Damit ist der Zuschnitt
    /// des Einzugstakts, der ueber **alle** Tabs fragt, nicht falsch geworden,
    /// sondern gegenstandslos: fuer einen verdeckten Tab faellt der Ruf hier
    /// von selbst.
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
    /// mischen sich nicht. Mit ihm faellt die Zahl der ungelesenen Dateien:
    /// sie gehoert dem Lauf, und ein neuer beginnt bei null.
    ///
    /// **Vier Bedingungen, und die erste ist die Sichtbarkeit.** Ein verdeckter
    /// Tab bekommt keinen Durchlauf; die Begruendung steht bei
    /// [`Tabliste::durchlauf_nachziehen`]. Sie steht vor den uebrigen drei, weil
    /// sie die einzige ist, die nicht am Modell des Tabs haengt.
    fn durchlauf_nachziehen_an(&mut self, stelle: usize) -> bool {
        self.tabs[stelle].durchlauf = None;
        self.tabs[stelle].zu_gross = 0;
        if stelle != self.aktiv {
            return false;
        }
        let tab = &self.tabs[stelle];
        // Der Bestand muss stehen. Waehrend eines Lesevorgangs zeigt das Modell
        // noch den **alten** Ordner — es wird nicht vorab geleert, sondern mit
        // dem ersten Stapel ersetzt —, und eine Auftragsliste daraus benennte
        // Ordner, die es hier gar nicht gibt, unter Indizes, die gleich einem
        // anderen Eintrag gehoeren werden.
        if !tab.gelesen || tab.liest() {
            return false;
        }
        // Ein Filtertext muss stehen, und mindestens einer der beiden Schalter
        // muss etwas zu tun geben. Ob der Inhaltsfilter wirkt, entscheidet
        // `inhalt_wirkt` und nicht diese Stelle: die Schwelle wird an einem Ort
        // geprueft (C2.10).
        if !tab.modell.filter_steht() || (!tab.modell.tief() && !tab.modell.inhalt_wirkt()) {
            return false;
        }
        // Die Liste kommt aus dem Ordnermodell und wird hier nicht
        // zusammengestellt: wessen Zeile an einem Befund haengt, ist dieselbe
        // Frage, die der Pruefschritt schon beantwortet hat, und sie steht
        // seit dem 260816 nur dort.
        let auftraege = tab.modell.auftraege();
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
            // Der Bestand wird geteilt und nicht kopiert; der Auftrag traegt
            // deshalb den blossen Index und keinen Namen.
            tab.modell.bestand(),
            auftraege,
            tab.ordner.clone(),
            tab.modell.muster().clone(),
            // **Die eine Stelle, an der die 1 MB in den Kern reisen**, und sie
            // liegt hier, weil die Zahl in `crate::vorschaumodell` wohnt:
            // `krk-core` bekommt keinen Bezug auf `krk-ui` (C1.7). `None` heisst
            // "bei diesem Lauf wird keine Datei geoeffnet", `Some(n)` heisst
            // "es wird gelesen, und n ist die Grenze" — zwei Aussagen in einem
            // Wert, damit sie nicht widerspruechlich gesetzt werden koennen.
            tab.modell
                .inhalt_wirkt()
                .then_some(crate::vorschaumodell::TEXTGRENZE),
            nummer,
        ));
        true
    }

    /// Sagt, ob der Gitbefund ueberhaupt angezeigt wird, und zieht den Lauf
    /// nach.
    ///
    /// Gerufen vom Anwendungsdelegierten, der als einziger weiss, ob der
    /// Git-Bereich steht oder die Markenspalte; die Oder-Verknuepfung rechnet
    /// er, hier steht die Antwort. Zwei der vier Ausloeser aus A9 kommen ueber
    /// diese Stelle herein, das Einschalten des Bereichs und das der Spalte.
    ///
    /// **Ein Wechsel und kein Wiederholen.** Steht der Wert schon, geschieht
    /// nichts: sonst stiesse jeder Nachzug der Aufteilung — und der laeuft bei
    /// jedem Wechsel des aktiven Dateifensters — einen weiteren Lauf an, ohne
    /// dass sich etwas geaendert haette.
    ///
    /// **Liefert, ob jetzt ein Gitlauf laeuft.** Wie bei
    /// [`Tabliste::durchlauf_nachziehen`] sagt der Wert dem Aufrufer, ob er den
    /// Einzugstakt anwerfen muss; faellt er still, bliebe der Befund im Kanal
    /// stehen und die Spalte leer.
    #[must_use = "laeuft jetzt ein Gitlauf, ist der Einzugstakt anzuwerfen"]
    pub fn git_gefragt_setzen(&mut self, gefragt: bool) -> bool {
        if self.git_gefragt == gefragt {
            return false;
        }
        self.git_gefragt = gefragt;
        let stelle = self.aktiv;
        self.gitlauf_nachziehen_an(stelle)
    }

    /// Holt die naechsten Commits hinter dem zuletzt gehaltenen (E12, C4.2).
    ///
    /// Der Rueckweg des Nachlademelders aus dem Git-Bereich: die Auswahl steht
    /// am letzten Eintrag der Liste, und der Nutzer drueckt weiter `down`.
    ///
    /// **Drei Fragen, und jede kann den Nachschlag abweisen.** Ist der Verlauf
    /// leer, gibt es keine Stelle, ab der nachzuladen waere. Ist er erschoepft,
    /// folgt nichts mehr (C4.3), und ein Lauf braechte eine leere Liste
    /// zurueck. Und **laeuft schon einer, faengt keiner an**: zwei Statuslaeufe
    /// fuer dasselbe Dateifenster stehen nie nebeneinander (A10, C7.11), und
    /// hier waere der Schaden ein besonderer — der laufende haelt womoeglich
    /// noch die Markenmeldung im Kanal, und ein Nachschlag, der ihn ersetzte,
    /// naehme sie mit.
    ///
    /// Der Verlauf **waechst** dabei und faengt nicht neu an; das Gitmodell
    /// wird deshalb nicht zurueckgesetzt.
    #[must_use = "laeuft jetzt ein Gitlauf, ist der Einzugstakt anzuwerfen"]
    pub fn verlauf_nachladen(&mut self) -> bool {
        let stelle = self.aktiv;
        let tab = &self.tabs[stelle];
        if tab.gitlauf.is_some() || tab.gitmodell.erschoepft() {
            return false;
        }
        let bereits = tab.gitmodell.verlaufslaenge();
        if bereits == 0 {
            return false;
        }
        self.letzter_gitlauf += 1;
        let nummer = self.letzter_gitlauf;
        let tab = &mut self.tabs[stelle];
        tab.gitlauf = Some(Gitlauf::starten(
            tab.ordner.clone(),
            Gitfrage::WeitererVerlauf { bereits },
            nummer,
        ));
        true
    }

    /// Traegt die Auswahl der Verlaufsliste in den sichtbaren Tab ein.
    ///
    /// Der Rueckweg des Auswahlmelders aus dem Git-Bereich. **Der sichtbare Tab
    /// und nicht ein genannter**: der Git-Bereich zeigt den Stand des
    /// sichtbaren Tabs im aktiven Dateifenster, also gehoert die Auswahl, die
    /// er meldet, ebendiesem Tab. Die Zusage und ihre eine Ausnahme stehen an
    /// [`Tabinhalt::gitmodell`].
    pub fn gitauswahl_setzen(&mut self, stelle: Option<usize>) {
        self.aktiver_mut().gitauswahl_setzen(stelle);
    }

    /// Bricht den Gitlauf eines Tabs ab und stoesst, wenn seine Bedingungen
    /// stehen, einen neuen an.
    ///
    /// **Die eine Stelle, an der ein Gitlauf entsteht und vergeht**, bis auf
    /// den Nachschlag in [`Tabliste::verlauf_nachladen`], der keinen Befund
    /// verwirft, sondern an ihn anhaengt. Der bisherige Lauf faellt in jedem
    /// Fall zuerst; sein `Drop` setzt das Abbruchkennzeichen, und sein
    /// Empfaenger geht mit, also kann kein Befund des alten Ordners mehr
    /// ankommen (A10). Mit ihm faellt die zurueckgehaltene Markenmeldung: sie
    /// gehoert dem Bestand, den es gleich nicht mehr gibt.
    ///
    /// **Und mit ihm faellt der Verlauf auf die ersten fuenfzig zurueck**
    /// (C4.6). Die Nachladehoehe gehoert dem Lauf und nicht dem Dateifenster;
    /// sie ueber zwei Ordner hinweg zu halten hiesse, den Verlauf eines
    /// Repositorys mit der Blaettertiefe eines anderen anzuzeigen.
    ///
    /// # Drei Bedingungen, und die dritte ist schwaecher als beim Durchlauf
    ///
    /// Sichtbar muss der Tab sein, gefragt muss der Befund sein, und der Ordner
    /// muss stehen. Die dritte heisst hier **nicht** „der Bestand ist gelesen":
    /// der Gitlauf braucht allein den **Pfad** — `gix::discover` sucht von dort
    /// aufwaerts, und die Pfadmuster des Status rechnen gegen ihn —, und er
    /// beginnt deshalb zugleich mit dem Lesevorgang. Mit der staerkeren
    /// Bedingung wartete der Branchname in einem Ordner mit hunderttausend
    /// Eintraegen vier Sekunden auf einen Bestand, den er nicht braucht (A8:
    /// Branch und Verlauf stehen schon, waehrend die Markenspalte noch leer
    /// ist).
    ///
    /// Was auf den gelesenen Bestand wartet, ist allein das **Eintragen** der
    /// Marken, und es wartet im Einzugstakt und nicht hier.
    fn gitlauf_nachziehen_an(&mut self, stelle: usize) -> bool {
        self.tabs[stelle].gitlauf = None;
        self.tabs[stelle].wartende_marken = None;
        self.tabs[stelle].gitmodell.zuruecksetzen();
        if stelle != self.aktiv || !self.git_gefragt {
            return false;
        }
        // Der Ordner steht. Ein leerer Pfad kommt aus einer von Hand
        // geaenderten `session.toml` und ist kein Ordner, den `gix::discover`
        // sinnvoll befragte; der Lauf faellt dann aus, statt aufwaerts vom
        // Arbeitsverzeichnis zu suchen.
        if self.tabs[stelle].ordner.as_os_str().is_empty() {
            return false;
        }
        self.letzter_gitlauf += 1;
        let nummer = self.letzter_gitlauf;
        let tab = &mut self.tabs[stelle];
        // Die Generation des Bestands, dem der Befund gelten wird. Sie steht
        // hier und nicht in der Meldung; der Grund steht am Feld.
        tab.gitgeneration = tab.modell.generation();
        tab.gitlauf = Some(Gitlauf::starten(tab.ordner.clone(), Gitfrage::Ganz, nummer));
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
    /// Abbruchkennzeichen. Fuer den Gitlauf gilt Zeile fuer Zeile dasselbe.
    pub fn abbrechen(&mut self) {
        for tab in &mut self.tabs {
            if let Some(vorgang) = tab.lesevorgang.take() {
                vorgang.abbrechen();
            }
            tab.durchlauf = None;
            tab.gitlauf = None;
            tab.wartende_marken = None;
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
        // Und mit ihm seine Zahl der ungelesenen Dateien: sie gehoert dem Lauf
        // und nicht dem Tab, und der neue Bestand hat noch keinen.
        tab.zu_gross = 0;
        tab.modell.lesevorgang_beginnen(generation);
        tab.meldung = None;
        tab.gelesen = false;
        tab.lesevorgang = Some(Lesevorgang::starten(&tab.ordner, generation));
        // **Zugleich mit dem Lesevorgang und nicht nach ihm**, und danach, weil
        // der neue Lauf die eben gesetzte Generation mitbekommen muss. Der
        // Gitlauf braucht allein den Pfad; darauf zu warten, dass hunderttausend
        // Eintraege gelesen sind, hiesse den Branchnamen vier Sekunden lang
        // zurueckzuhalten. Der Wert sagt, ob der Einzugstakt anzuwerfen ist —
        // er laeuft hier ohnehin, denn der Lesevorgang darueber braucht ihn.
        let _ = self.gitlauf_nachziehen_an(stelle);
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
///
/// Drei Kanaele, in dieser Reihenfolge: erst die Stapel des Lesevorgangs, dann
/// die Befunde des Durchlaufs, zuletzt die Meldungen des Gitlaufs. Die
/// Reihenfolge der ersten beiden traegt nichts — sie koennen nie zugleich
/// laufen, weil ein Durchlauf erst nach dem Abschluss des Lesevorgangs
/// beginnt —, und sie steht so herum, weil der Lesevorgang den Bestand
/// liefert, auf den sich jeder Befund bezieht.
///
/// **Beim dritten traegt sie sehr wohl etwas**, und deshalb steht er hinten:
/// der Gitlauf laeuft im Gegensatz zum Durchlauf **zugleich** mit dem
/// Lesevorgang, und seine Marken duerfen erst in einen Bestand, der steht. Wer
/// ihn vor `lesemeldungen_einziehen` zoege, liesse den Abschluss desselben
/// Takts ungenutzt und traege die Marken erst einen Takt spaeter ein.
fn einzug_je_tab(tab: &mut Tabinhalt) -> Einzug {
    let mut einzug = lesemeldungen_einziehen(tab);
    einzug.befunde_neu = befunde_einziehen(tab);
    let gitzug = gitmeldungen_einziehen(tab);
    einzug.gitkopf_neu = gitzug.kopf_neu;
    einzug.gitmarken_neu = gitzug.marken_neu;
    einzug
}

/// Was ein Takt am Gitstand eines Tabs veraendert hat.
#[derive(Default)]
struct Gitzug {
    /// Der Git-Bereich zeigt etwas anderes als vorher.
    kopf_neu: bool,
    /// Das Ordnermodell hat Marken bekommen.
    marken_neu: bool,
}

/// Holt die wartenden Meldungen des Gitlaufs ab.
///
/// **Zwei Takte in einem, und sie haengen an verschiedenen Bedingungen.** Kopf
/// und Verlauf gehen sofort in das Gitmodell: sie brauchen den Bestand nicht,
/// und A8 verlangt ausdruecklich, dass sie schon dastehen, waehrend die
/// Markenspalte noch leer ist. Die Marken warten dagegen, bis der Bestand
/// steht — `tab.gelesen && !tab.liest()` —, denn sie werden ueber den **Namen**
/// zugeordnet, und waehrend eines Lesevorgangs zeigt das Modell noch den alten
/// Ordner.
///
/// Wo die wartende Meldung liegt und warum nicht im Kanal, steht am Feld
/// `wartende_marken`.
///
/// **Der geschlossene Kanal raeumt den Lauf weg**, wie beim Durchlauf: er sagt,
/// dass der Arbeitsfaden geendet hat, und das Feld stehen zu lassen hielte den
/// Einzugstakt fuer immer am Laufen. Was danach an Meldungen fehlt, heisst
/// **nicht** „dieser Ordner hat keine Marken", sondern „der Befund steht aus";
/// der Modulkopf von [`krk_core::git::lauf`] schreibt die Regel aus.
///
/// **Die zurueckgehaltene Meldung haelt den Lauf nicht am Leben.** Der Kanal
/// ist drei tief, der Arbeitsfaden blockiert also an keiner der drei Meldungen
/// und endet auch dann, wenn niemand sie holt.
///
/// **Der Kanalschluss nimmt `wartende_marken` nicht mit**, und das ist die
/// Zusage und kein Versehen: er raeumt allein `tab.gitlauf` weg, wie der Absatz
/// darueber sagt. In einem grossen Ordner schliesst der Kanal regelmaessig,
/// **bevor** der Bestand gelesen ist; fiele der Befund mit ihm, waere er fuer
/// immer weg — genau die Alternative, die das Feld `wartende_marken` als
/// verworfen ausschreibt. Er faellt stattdessen mit dem Lauf, zu dem er gehoert,
/// und das sind [`Tabliste::gitlauf_nachziehen_an`] und
/// [`Tabliste::abbrechen`]. Und der Tab, dessen Ordner sich nicht lesen laesst,
/// ist nach `abschliessen` trotzdem gelesen, sodass die Marken ihren Weg
/// finden.
fn gitmeldungen_einziehen(tab: &mut Tabinhalt) -> Gitzug {
    use std::sync::mpsc::TryRecvError;

    let mut zug = Gitzug::default();
    // Erst holen, dann eintragen. Der Lauf wird dafuer ausgeliehen, und das
    // Eintragen fasst den Tab veraenderlich an; beides in einer Schleife hielte
    // zwei einander ausschliessende Ausleihen zugleich. Dieselbe Zweiteilung
    // steht in `befunde_einziehen` darueber.
    let mut eingetroffen = Vec::new();
    let mut kanal_zu = false;
    if let Some(lauf) = tab.gitlauf.as_ref() {
        loop {
            match lauf.meldungen().try_recv() {
                Ok(meldung) => eingetroffen.push(meldung),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    kanal_zu = true;
                    break;
                }
            }
        }
    }
    if kanal_zu {
        tab.gitlauf = None;
    }
    for meldung in eingetroffen {
        match meldung {
            Gitmeldung::Kopf(kopf) => {
                tab.gitmodell.kopf_setzen(kopf);
                zug.kopf_neu = true;
            }
            Gitmeldung::Verlauf(commits) => {
                tab.gitmodell.verlauf_anhaengen(commits);
                zug.kopf_neu = true;
            }
            Gitmeldung::Marken(marken) => tab.wartende_marken = Some(marken),
        }
    }
    if !tab.gelesen || tab.liest() {
        return zug;
    }
    let Some(marken) = tab.wartende_marken.take() else {
        return zug;
    };
    // Die Zusammenfassung des Git-Bereichs und die Buchstaben der Spalte kommen
    // aus **einer** Meldung; sie ein zweites Mal zu holen hiesse, den Status ein
    // zweites Mal zu fragen.
    tab.gitmodell.marken_setzen(&marken);
    zug.kopf_neu = true;
    // Die Generation entscheidet, ob der Befund noch zu diesem Bestand gehoert
    // (C7.5). Weist das Modell ab, bleibt die Spalte leer, und die Ansicht hat
    // nichts nachzuziehen.
    zug.marken_neu = tab.modell.gitmarken_setzen(tab.gitgeneration, &marken);
    zug
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
    // Der Stand der wegen ihrer Groesse ungelesenen Dateien, bei **jedem** Takt
    // und auch bei dem, der den geschlossenen Kanal sieht. Er wird hier
    // abgeschrieben und nicht spaeter am `Durchlauf` gefragt, weil der gleich
    // darunter faellt; danach traegt ihn allein der Tab, und die Statuszeile
    // findet die Zahl auch bei einem Ordner, dessen Lauf laengst durch ist.
    tab.zu_gross = durchlauf.zu_gross();
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
    use krk_core::verzeichnis::{Auftrag, Auftragsart};

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

    /// C1.9: der Filtertext uebersteht den Ordnerwechsel auch bei
    /// ausgeschalteter tiefer Suche.
    ///
    /// Die Richtung, die der Nutzerentscheid vom 260815-0955 umgekehrt hat
    /// (`decisions/260814-1830_*_bleibt-der-filtertext-bei-einem-ordnerwechsel-stehen-wenn-deep-aus-ist.md`,
    /// Moeglichkeit 2). Bis dahin pruefte diese Probe das Gegenteil.
    #[test]
    fn ein_ordnerwechsel_laesst_den_filtertext_stehen_wenn_die_tiefe_suche_aus_ist() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        // Ausdruecklich abgeschaltet: die Vorbelegung von `Ordnermodell::neu`
        // ist seit dem 260826 "ein", und diese Probe misst den anderen Stand.
        liste.aktiver_mut().modell_mut().tief_setzen(false);
        liste.aktiver_mut().modell_mut().filtertext_setzen("rs");
        assert!(liste.aktiver().modell().filter_steht());

        liste.ordner_setzen(&dorthin, None);

        assert_eq!(
            liste.aktiver().modell().filtertext(),
            "rs",
            "der Filtertext haengt nicht am Filter der Tiefe"
        );
        assert!(
            !liste.aktiver().modell().tief(),
            "der Schalter selbst bleibt, was er war"
        );
        assert_eq!(
            liste.aktiver().ordner(),
            Path::new(&dorthin),
            "gewechselt wurde trotzdem"
        );
    }

    /// C1.9: der Aufstieg zaehlt wie der Einstieg.
    ///
    /// Er geht durch dieselbe Stelle, nimmt aber den verlassenen Ordner als
    /// Wunschauswahl mit. Die Probe faehrt ihn so, wie
    /// `DateifensterQuelle::ordner_aufwaerts` ihn faehrt: mit
    /// [`krk_core::verzeichnis::aufwaerts`] gerechnet und mit dem Namen des
    /// verlassenen Ordners als `auswahl`.
    #[test]
    fn der_aufstieg_laesst_den_filtertext_stehen_wie_der_einstieg() {
        let (hier, _) = zwei_vorhandene_ordner();
        let (eltern, verlassen) = krk_core::verzeichnis::aufwaerts(Path::new(&hier))
            .expect("das Temporaerverzeichnis hat einen uebergeordneten Ordner");
        let mut liste = liste(&[&hier]);
        liste.aktiver_mut().modell_mut().filtertext_setzen("rs");

        liste.ordner_setzen(&eltern, Some(verlassen.clone()));

        assert_eq!(
            liste.aktiver().modell().filtertext(),
            "rs",
            "der Aufstieg loescht den Filtertext so wenig wie der Einstieg"
        );
        assert_eq!(liste.aktiver().ordner(), eltern.as_path());
        assert_eq!(
            liste.aktiver().auswahlname().as_deref(),
            Some(verlassen.as_str()),
            "die Auswahl geht weiterhin auf den verlassenen Ordner"
        );
    }

    /// C1.10: bei eingeschalteter tiefer Suche uebersteht der Filtertext den
    /// Ordnerwechsel ebenfalls. Seit dem 260815 ist das kein eigener Fall mehr,
    /// sondern derselbe wie in C1.9; die Probe steht daneben, weil das Modell
    /// der tiefen Ansicht auf der naechsten Ebene sonst seinen Gegenstand
    /// verloere.
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

    /// C2.4 und C1.12: der Stand von „Content" uebersteht den Ordnerwechsel,
    /// wie der Filtertext und der Stand von „Deep".
    ///
    /// Die fuenfte Uebertragung in `ordner_setzen`, unbedingt und ohne Zweig.
    /// Daran haengt die Zusage, dass der neue Ordner sofort anfaengt, seine
    /// Dateien zu lesen: faellt der Schalter beim Wechsel, wirkt der
    /// Inhaltsfilter dort gar nicht.
    #[test]
    fn ein_ordnerwechsel_traegt_den_stand_von_content() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.filtertext_setzen("notiz");
        modell.inhalt_setzen(true);
        assert!(liste.aktiver().modell().inhalt_wirkt());

        liste.ordner_setzen(&dorthin, None);

        assert!(
            liste.aktiver().modell().inhalt(),
            "der Schalter geht mit dem Filtertext hinueber"
        );
        assert!(
            liste.aktiver().modell().inhalt_wirkt(),
            "und mit ihm die Wirkung, denn der Filtertext steht auch noch"
        );
        assert_eq!(
            liste.aktiver().ordner(),
            Path::new(&dorthin),
            "gewechselt wurde trotzdem"
        );
    }

    /// Der Filter des Inhalts geht auch ohne Filtertext hinueber, wie der
    /// Filter der Tiefe: er ist ein Schalter des Tabs und keine Beigabe zum
    /// Text.
    #[test]
    fn der_inhaltsfilter_geht_auch_ohne_filtertext_hinueber() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        liste.aktiver_mut().modell_mut().inhalt_setzen(true);

        liste.ordner_setzen(&dorthin, None);

        assert!(liste.aktiver().modell().inhalt());
        assert!(
            !liste.aktiver().modell().inhalt_wirkt(),
            "ohne Filtertext steht der Schalter und tut nichts"
        );
        assert_eq!(liste.aktiver().modell().filtertext(), "");
    }

    /// Ein neu geoeffneter Tab traegt die Vorbelegung und nicht den Stand des
    /// Geschwistertabs.
    ///
    /// **Das ist die Antwort des Baumes und keine Antwort auf
    /// `decisions/260814-1830_*_gilt-das-ankreuzfeld-deep-je-tab-oder-je-fenster.md`**,
    /// die offen bleibt: jeder Tab kommt ueber `Tabinhalt::aus_zustand` und
    /// damit ueber `Ordnermodell::neu`, und `Tabzustand` fuehrt den Stand
    /// nicht mit. Faellt die offene Frage einmal auf "je Fenster", wird diese
    /// Probe rot und ist dann zu Recht rot.
    #[test]
    fn ein_neuer_tab_traegt_die_vorbelegung_der_tiefen_suche() {
        let (hier, dorthin) = zwei_vorhandene_ordner();
        let mut liste = liste(&[&hier]);
        liste.aktiver_mut().modell_mut().tief_setzen(false);

        liste.oeffnen(&dorthin);

        assert!(
            liste.aktiver().modell().tief(),
            "der neue Tab beginnt bei der Vorbelegung, nicht beim Nachbarn"
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

    /// Der Name, den ein Auftrag benennt.
    ///
    /// Der Auftrag traegt seit dem 260816 nur den Index; nachgeschlagen wird im
    /// Bestand, genau wie der Durchlauf es tut.
    fn auftragsname(modell: &Ordnermodell, auftrag: &Auftrag) -> String {
        modell.eintraege()[auftrag.index as usize].name.clone()
    }

    /// Die Namen der Auftraege, in der Reihenfolge der Liste.
    fn auftragsnamen(modell: &Ordnermodell) -> Vec<String> {
        modell
            .auftraege()
            .iter()
            .map(|auftrag| auftragsname(modell, auftrag))
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
            modell.auftraege().len(),
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
        let auftraege = modell.auftraege();
        assert_eq!(auftraege.len(), 2);
        assert_eq!(
            (auftraege[0].index, auftragsname(&modell, &auftraege[0])),
            (1, "bilder".to_owned())
        );
        assert_eq!(
            (auftraege[1].index, auftragsname(&modell, &auftraege[1])),
            (3, "daten".to_owned())
        );
    }

    /// Ein fertig gelesenes Ordnermodell mit stehendem Filter und beiden
    /// Schaltern.
    fn modell_mit_schaltern(
        bestand: &[(&str, krk_core::verzeichnis::Typ)],
        filter: &str,
        tief: bool,
        inhalt: bool,
    ) -> Ordnermodell {
        let mut modell = modell_mit(bestand, filter, tief);
        modell.inhalt_setzen(inhalt);
        modell
    }

    /// Die Auftraege als Paare aus Name und Art, in der Reihenfolge der Liste.
    fn auftragstafel(modell: &Ordnermodell) -> Vec<(String, Auftragsart)> {
        modell
            .auftraege()
            .iter()
            .map(|auftrag| (auftragsname(modell, auftrag), auftrag.art))
            .collect()
    }

    /// Die Tafel der vier Auftragslagen, in einer Probe.
    ///
    /// Ein Bestand, vier Schalterstellungen. Der Kurzschluss am Eingang nimmt
    /// in jeder von ihnen `notiz.txt` und `notizen` heraus, weil ihr Name die
    /// Folge schon traegt; was uebrig ist, entscheidet der Typ mit dem
    /// zugehoerigen Schalter.
    ///
    /// Fuenf Zeichen sind es mit Absicht: sie liegen ueber beiden Schwellen,
    /// also haengt das Ergebnis allein an den Schaltern und nicht daran, wie
    /// `inhaltsschwelle` gerade steht.
    #[test]
    fn die_auftragsliste_stellt_die_tafel_der_vier_auftragslagen() {
        use krk_core::verzeichnis::Typ;

        let bestand = [
            ("notiz.txt", Typ::Datei),
            ("bild.png", Typ::Datei),
            ("notizen", Typ::Ordner),
            ("bilder", Typ::Ordner),
        ];

        assert!(
            auftragstafel(&modell_mit_schaltern(&bestand, "notiz", false, false)).is_empty(),
            "ohne beide Schalter gibt es nichts zu entscheiden"
        );
        assert_eq!(
            auftragstafel(&modell_mit_schaltern(&bestand, "notiz", true, false)),
            [("bilder".to_owned(), Auftragsart::Unterbaum)],
            "allein \"Deep\": der Ordner ohne Namenstreffer bekommt seinen Unterbaum"
        );
        assert_eq!(
            auftragstafel(&modell_mit_schaltern(&bestand, "notiz", false, true)),
            [("bild.png".to_owned(), Auftragsart::Inhalt)],
            "allein \"Content\": die Datei ohne Namenstreffer wird gelesen"
        );
        assert_eq!(
            auftragstafel(&modell_mit_schaltern(&bestand, "notiz", true, true)),
            [
                ("bild.png".to_owned(), Auftragsart::Inhalt),
                ("bilder".to_owned(), Auftragsart::Unterbaum),
            ],
            "beide Schalter: beide Arten nebeneinander, in der Reihenfolge des Bestands"
        );
    }

    /// C3.2: bei vier getippten Zeichen und gesetztem "Deep" entscheidet allein
    /// der Name, auch wenn "Content" steht.
    ///
    /// Die Schwelle steigt mit der tiefen Suche von drei auf fuenf, und sie
    /// wird bei jeder Bewertung neu gefragt. Ein fuenftes Zeichen holt die
    /// Inhaltsauftraege zurueck.
    #[test]
    fn bei_vier_zeichen_und_deep_traegt_die_auftragsliste_keinen_inhaltsauftrag() {
        use krk_core::verzeichnis::Typ;

        let bestand = [("bild.png", Typ::Datei), ("bilder", Typ::Ordner)];

        assert_eq!(
            auftragstafel(&modell_mit_schaltern(&bestand, "noti", true, true)),
            [("bilder".to_owned(), Auftragsart::Unterbaum)],
            "vier Zeichen liegen unter der Schwelle der tiefen Suche"
        );
        assert_eq!(
            auftragstafel(&modell_mit_schaltern(&bestand, "notiz", true, true)),
            [
                ("bild.png".to_owned(), Auftragsart::Inhalt),
                ("bilder".to_owned(), Auftragsart::Unterbaum),
            ],
            "das fuenfte Zeichen holt den Inhaltsauftrag zurueck"
        );
    }

    /// C3.4: eine Datei, deren Name die Folge traegt, bekommt keinen Auftrag
    /// und bleibt damit ungelesen.
    ///
    /// Der Kurzschluss steht am Eingang der Liste und gilt fuer beide Arten.
    /// Ohne ihn oeffnete der Durchlauf Dateien, deren Zeile ohnehin schon
    /// feststeht.
    #[test]
    fn eine_datei_mit_namenstreffer_bleibt_ungelesen() {
        use krk_core::verzeichnis::Typ;

        let modell = modell_mit_schaltern(
            &[("notiz-gross.txt", Typ::Datei), ("bild.png", Typ::Datei)],
            "notiz",
            false,
            true,
        );
        assert_eq!(
            auftragstafel(&modell),
            [("bild.png".to_owned(), Auftragsart::Inhalt)],
            "`notiz-gross.txt` steht am Namen und wird nicht geoeffnet"
        );
    }

    /// C3.8 und die Sperre: ohne wirkenden Schalter beginnt kein Durchlauf,
    /// mit gesetztem "Content" schon — auch wenn "Deep" aus ist.
    #[test]
    fn allein_content_stoesst_einen_durchlauf_an() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-content");
        ordner.datei("bild.png", b"x");
        let mut liste = liste(&[&ordner.pfad().display().to_string()]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.filtertext_setzen("notiz");
        modell.anhaengen([eintrag("bild.png", Typ::Datei)]);
        liste.tabs[0].gelesen = true;

        assert!(
            !liste.durchlauf_nachziehen(),
            "ohne beide Schalter gibt es nichts zu tun"
        );
        liste.aktiver_mut().modell_mut().inhalt_setzen(true);
        assert!(
            liste.durchlauf_nachziehen(),
            "\"Content\" allein reicht, \"Deep\" ist dafuer nicht noetig"
        );

        // Unter die Schwelle: der Lauf faellt, ohne dass jemand den Schalter
        // angefasst haette.
        liste.aktiver_mut().modell_mut().filtertext_setzen("no");
        assert!(!liste.durchlauf_nachziehen());
        assert!(!liste.arbeitet_noch());
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

    /// C4.5: ein Tabwechsel beendet den Durchlauf des verlassenen Tabs, gleich
    /// welcher Art.
    ///
    /// Bis zum 260816 stand hier die Gegenprobe: der verdeckte Tab lief weiter.
    /// Der Nutzerentscheid vom 260816-1410 zu
    /// `decisions/260816-1359_*_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md`
    /// hat das umgedreht, und der Preis steht am Doc-Kommentar von
    /// `Tabliste::durchlauf_nachziehen`.
    #[test]
    fn ein_tabwechsel_beendet_den_durchlauf_des_verlassenen_tabs() {
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
        // Auch der zweite Tab gilt als gelesen, sonst stiesse der Wechsel dort
        // einen Lesevorgang an und `arbeitet_noch` bliebe deswegen wahr.
        liste.tabs[1].gelesen = true;
        assert!(liste.durchlauf_nachziehen());

        assert!(liste.naechster(), "auf den zweiten Tab wechseln");
        assert!(
            liste.tabs[0].durchlauf.is_none(),
            "der verlassene Tab laeuft nicht weiter"
        );
        assert!(
            !liste.arbeitet_noch(),
            "und der Einzugstakt hat nichts mehr zu tun"
        );
    }

    /// Ein verdeckter Tab bekommt auch sonst keinen Durchlauf: die Bedingung
    /// steht im Rumpf und nicht als Zweig in `waehlen`.
    #[test]
    fn ein_verdeckter_tab_bekommt_keinen_durchlauf() {
        use krk_core::verzeichnis::Typ;

        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-verdeckt");
        ordner.ordner("bilder");
        let vorhanden = ordner.pfad().display().to_string();
        let mut liste = liste(&["/b", &vorhanden]);
        let modell = liste.tabs[1].modell_mut();
        modell.tief_setzen(true);
        modell.filtertext_setzen("zzz");
        modell.anhaengen([eintrag("bilder", Typ::Ordner)]);
        liste.tabs[1].gelesen = true;

        assert!(
            !liste.durchlauf_nachziehen_an(1),
            "Tab 1 ist verdeckt, und der sichtbare ist Tab 0"
        );
        assert!(liste.tabs[1].durchlauf.is_none());
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

    /// Der ganze Weg des Inhaltsfilters ohne AppKit, und die Zahl der wegen
    /// ihrer Groesse ungelesenen Dateien steht **nach** dem Ende des Laufs
    /// noch da.
    ///
    /// Die Datei ueber der Grenze ist mit Absicht echt und nicht behauptet: die
    /// Grenze wird gehalten und nicht vorhergesagt, und was `zu_gross` zaehlt,
    /// entsteht erst beim Lesen. Der Lauf ueber diesen kleinen Ordner ist durch,
    /// bevor die Statuszeile das naechste Mal rechnet — genau deshalb traegt der
    /// Tab die Zahl und nicht der `Durchlauf`.
    #[test]
    fn die_zahl_der_zu_grossen_dateien_steht_auch_nach_dem_ende_des_laufs() {
        let ordner = crate::pruefordner::Pruefordner::neu("durchlauf-zu-gross");
        ordner.datei("klein.txt", b"hier steht notiz drin");
        ordner.datei(
            "gross.bin",
            "a".repeat((crate::vorschaumodell::TEXTGRENZE + 1) as usize),
        );
        ordner.datei("leer.txt", b"nichts davon");

        let mut liste = liste(&[&ordner.pfad().display().to_string()]);
        let modell = liste.aktiver_mut().modell_mut();
        modell.inhalt_setzen(true);
        modell.filtertext_setzen("notiz");
        liste.sichtbaren_lesen();

        let mut takte = 0;
        while liste.arbeitet_noch() {
            let _ = liste.einziehen();
            takte += 1;
            assert!(takte < 2_000, "der Durchlauf ist nicht zum Ende gekommen");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        assert_eq!(
            zeilennamen(liste.aktiver().modell()),
            ["klein.txt"],
            "der Inhaltstreffer steht, die anderen beiden nicht"
        );
        assert!(
            liste.aktiver().durchlauf.is_none(),
            "der Lauf ist durch, und sein `Durchlauf` ist weg"
        );
        assert_eq!(
            liste.aktiver().zu_gross(),
            1,
            "die eine ungelesene Datei steht danach immer noch zu Buche"
        );
        assert!(
            !liste.aktiver().liest_inhalt(),
            "ohne laufenden Durchlauf ist der Lesehinweis falsch, auch wenn \u{201e}Content\u{201c} steht"
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

    /// C2.9: die Dateiliste bleibt eine flache Tabelle. Die Zahl ihrer Spalten
    /// steht seit der Git-Runde bei fuenf (C5.1) und war bis dahin vier; die
    /// Zusage von C2.9 ist die **Flachheit** und nicht die Zahl, und deshalb
    /// zieht diese Probe die Zahl nach, statt sie zu verteidigen.
    ///
    /// Gezaehlt an zwei Stellen statt behauptet: die Zahl der Spalten am
    /// Aufzaehlungstyp, der sie fuehrt, und das Vorkommen von `NSOutlineView`
    /// **im ganzen Baum**.
    ///
    /// Bis zum 260815 las die Nadel ueber `include_str!` genau
    /// `appkit/tabelle.rs`. Damit war die dritte Zusage von C2.9, „keine zweite
    /// Tabellenklasse", gar nicht geprueft: eine zweite Klasse stuende ja
    /// gerade **nicht** in dieser Datei
    /// (`issues/260815-0211_*_die-probe-fuer-die-flache-dateiliste-liest-eine-datei-…`).
    /// [`crate::quellbaum::quelldateien`] steht fuer genau diesen Fall bereit,
    /// und der Kopf dieses Moduls schreibt aus, warum eine Zaehlprobe den Baum
    /// liest und nicht eine Datei.
    ///
    /// **`NSTableView` ist ausdruecklich keine Nadel.** KRK hat mehrere
    /// Tabellen — die Belegungsansicht und das Blatt zum Stapelumbenennen —,
    /// und eine Zaehlung darueber saehe sie als Fundstellen. Gefragt ist die
    /// Aufklappansicht, und die heisst in AppKit `NSOutlineView`.
    ///
    /// **Die verbleibende Blindheit:** eine Aufklappansicht, die niemand so
    /// nennt, weil sie von Hand aus Zeilen mit Einzug gebaut waere, faende
    /// diese Zaehlung nicht. Der Bezug auf `spalten::Spalte::ALLE` haengt
    /// dagegen am Aufzaehlungstyp und nicht an seinem Namen im Text.
    #[test]
    fn die_dateiliste_bleibt_flach_und_hat_fuenf_spalten() {
        assert_eq!(crate::spalten::Spalte::ALLE.len(), 5);
        let aufklappansicht = concat!("NSOutline", "View");
        for (name, inhalt) in crate::quellbaum::quelldateien() {
            let treffer: Vec<&str> = inhalt
                .lines()
                .filter(|zeile| !zeile.trim_start().starts_with("//"))
                .filter(|zeile| zeile.contains(aufklappansicht))
                .collect();
            assert!(
                treffer.is_empty(),
                "{name} baut eine Aufklappansicht: {treffer:?}"
            );
        }
    }

    /// C2.11: `angezeigtedatei::welche` bleibt bei zwei Quellen.
    ///
    /// **Gefragt ist der Gegenstand und nicht die Schreibweise.** Bis zum
    /// 260815 zaehlte diese Probe die Zeichenfolge `return Some(` im Rumpf des
    /// Moduls und erklaerte damit die heutige Schreibweise zur Regel: ein Wert
    /// am Ende der Funktion, ein `.or_else`, ein `match` mit `Some(…)` als
    /// Armwert — jede dritte Quelle in einer dieser Formen waere durchgelaufen
    /// (`issues/260815-0211_*_die-probe-fuer-die-angezeigte-datei-zaehlt-return-some-…`).
    ///
    /// An ihre Stelle treten zwei Zusicherungen, die keine Zeile Quelltext
    /// lesen:
    ///
    /// 1. **Die Signatur.** `welche` wird an einen Funktionszeiger mit genau
    ///    diesen vier Eingaben gebunden. Eine dritte Quelle, die eine fuenfte
    ///    Eingabe braucht — die Auswahl der Dateiliste, der angezeigte Ordner
    ///    —, haelt damit den Bau an, statt still dazuzukommen.
    /// 2. **Die Antwort.** Ueber alle sechzehn Kombinationen der vier Eingaben
    ///    ist das Ergebnis entweder `None` oder **genau einer der beiden
    ///    uebergebenen Pfade**. Eine Quelle ausserhalb der Eingaben — ein
    ///    Ivar, eine Umgebungsvariable, ein Blick ins Dateisystem — laege
    ///    ausserhalb dieser Menge und faellt hier heraus.
    ///
    /// **Die verbleibende Blindheit:** eine dritte Quelle, die genau einen der
    /// beiden uebergebenen Pfade liefert, saehe diese Probe nicht. Sie waere
    /// von den beiden aber auch nicht zu unterscheiden, und C2.11 spricht ueber
    /// die Antwort. Welche der beiden Quellen in welcher Lage gewinnt, prueft
    /// `angezeigtedatei.rs` in seinem eigenen Probenmodul ueber die volle Tafel.
    #[test]
    fn die_angezeigte_datei_bleibt_bei_zwei_quellen() {
        use crate::angezeigtedatei::welche;

        let gebunden: fn(bool, Option<PathBuf>, bool, Option<PathBuf>) -> Option<PathBuf> = welche;

        let vorschau = PathBuf::from("/Users/k1/Bilder/schirm.png");
        let editor = PathBuf::from("/Users/k1/Projekte/krk/README.md");

        for vorschau_sichtbar in [false, true] {
            for editor_sichtbar in [false, true] {
                for vorschau_pfad in [None, Some(vorschau.clone())] {
                    for editor_pfad in [None, Some(editor.clone())] {
                        let antwort = gebunden(
                            vorschau_sichtbar,
                            vorschau_pfad.clone(),
                            editor_sichtbar,
                            editor_pfad.clone(),
                        );
                        let erlaubt = antwort.is_none()
                            || antwort.as_ref() == Some(&vorschau)
                            || antwort.as_ref() == Some(&editor);
                        assert!(
                            erlaubt,
                            "eine dritte Quelle: {antwort:?} ist weder die Vorschau noch der \
                             Editor (sichtbar {vorschau_sichtbar}/{editor_sichtbar}, Pfade \
                             {vorschau_pfad:?}/{editor_pfad:?})"
                        );
                    }
                }
            }
        }
    }

    // ── Der Gitlauf am Tab (Runde 23, Schritt 6) ────────────────────────────

    /// Ein Commit fuer die Proben unten; der Objektname entsteht aus der
    /// Nummer.
    fn gitcommit(nummer: u8) -> krk_core::git::Commit {
        let hex = format!("{nummer:02x}").repeat(20);
        krk_core::git::Commit {
            id: krk_core::git::ObjectId::from_hex(hex.as_bytes())
                .expect("vierzig Hexziffern sind ein Objektname"),
            kurzbeschreibung: format!("Commit {nummer}"),
            nachricht: format!("Commit {nummer}"),
            autor: "Wer".to_owned(),
            email: "wer@example.invalid".to_owned(),
            zeit: std::time::SystemTime::UNIX_EPOCH,
        }
    }

    /// Wie viele Tabs der Liste gerade einen Gitlauf halten.
    fn stehende_gitlaeufe(liste: &Tabliste) -> usize {
        liste
            .tabs
            .iter()
            .filter(|tab| tab.gitlauf.is_some())
            .count()
    }

    /// Der Kern des Schritts: der Gitlauf beginnt **zugleich** mit dem
    /// Lesevorgang und wartet nicht auf den gelesenen Bestand.
    ///
    /// Die dritte Bedingung ist damit schwaecher als beim Durchlauf, der auf
    /// `gelesen && !liest()` wartet. Ohne diesen Unterschied wartete der
    /// Branchname in einem Ordner mit hunderttausend Eintraegen vier Sekunden
    /// auf etwas, das er nicht braucht (A8).
    #[test]
    fn der_gitlauf_beginnt_zugleich_mit_dem_lesevorgang() {
        let ordner = crate::pruefordner::Pruefordner::neu("gitlauf-zugleich");
        let mut liste = liste(&[&ordner.pfad().display().to_string()]);
        let _ = liste.git_gefragt_setzen(true);

        liste.sichtbaren_lesen();

        assert!(
            liste.tabs[0].gitlauf.is_some(),
            "der Gitlauf steht, obwohl der Bestand noch nicht gelesen ist"
        );
        assert!(
            !liste.tabs[0].gelesen,
            "der Tab ist zu diesem Zeitpunkt gerade nicht gelesen"
        );
        assert!(
            liste.tabs[0].durchlauf.is_none(),
            "der Durchlauf wartet dagegen auf den Bestand"
        );
        assert!(liste.arbeitet_noch(), "der Einzugstakt hat zu tun");
    }

    /// Ohne seine drei Bedingungen beginnt kein Gitlauf.
    #[test]
    fn ohne_seine_drei_bedingungen_beginnt_kein_gitlauf() {
        let ordner = crate::pruefordner::Pruefordner::neu("gitlauf-bedingungen");
        let pfad = ordner.pfad().display().to_string();

        // Niemand fragt nach dem Befund: kein Bereich, keine Spalte.
        let mut ungefragt = liste(&[&pfad, &pfad]);
        ungefragt.sichtbaren_lesen();
        assert_eq!(
            stehende_gitlaeufe(&ungefragt),
            0,
            "ohne Bereich und ohne Spalte laeuft nichts"
        );

        // Gefragt, aber der Tab ist verdeckt.
        let mut verdeckt = liste(&[&pfad, &pfad]);
        let _ = verdeckt.git_gefragt_setzen(true);
        assert!(
            !verdeckt.gitlauf_nachziehen_an(1),
            "Tab 1 ist verdeckt, und der sichtbare ist Tab 0"
        );
        assert!(
            verdeckt.tabs[1].gitlauf.is_none(),
            "und er haelt danach auch keinen"
        );

        // Gefragt und sichtbar, aber ohne Ordner.
        let mut ohne_ordner = liste(&[""]);
        let _ = ohne_ordner.git_gefragt_setzen(true);
        assert!(
            !ohne_ordner.gitlauf_nachziehen_an(0),
            "ein leerer Pfad ist kein Ordner, den gix::discover befragte"
        );
        assert_eq!(stehende_gitlaeufe(&ohne_ordner), 0);
    }

    /// C7.11: zwei schnell aufeinanderfolgende Ordnerwechsel lassen nie zwei
    /// Laeufe stehen.
    #[test]
    fn zwei_schnelle_ordnerwechsel_lassen_nie_zwei_gitlaeufe_stehen() {
        let erster = crate::pruefordner::Pruefordner::neu("gitlauf-erster");
        let zweiter = crate::pruefordner::Pruefordner::neu("gitlauf-zweiter");
        let mut liste = liste(&[&erster.pfad().display().to_string()]);
        let _ = liste.git_gefragt_setzen(true);

        liste.ordner_setzen(erster.pfad(), None);
        liste.ordner_setzen(zweiter.pfad(), None);

        assert_eq!(
            stehende_gitlaeufe(&liste),
            1,
            "der zweite Wechsel laesst den ersten Lauf fallen"
        );
        assert_eq!(
            liste.aktiver().ordner(),
            zweiter.pfad(),
            "und der stehende gilt dem angezeigten Ordner"
        );
    }

    /// C4.6: ein Ordnerwechsel setzt den Verlauf auf die ersten fuenfzig
    /// zurueck; die Nachladehoehe wird nicht ueber zwei Ordner hinweg gehalten.
    #[test]
    fn ein_ordnerwechsel_setzt_den_verlauf_auf_die_ersten_fuenfzig_zurueck() {
        use krk_core::git::Kopf;
        use krk_core::git::lauf::VERLAUFSSCHRITT;

        let erster = crate::pruefordner::Pruefordner::neu("gitverlauf-erster");
        let zweiter = crate::pruefordner::Pruefordner::neu("gitverlauf-zweiter");
        let mut liste = liste(&[&erster.pfad().display().to_string()]);
        let _ = liste.git_gefragt_setzen(true);

        // Zwei volle Schwuenge nachgeladen, dazu Kopf und Zusammenfassung.
        let gitmodell = &mut liste.tabs[0].gitmodell;
        gitmodell.kopf_setzen(Kopf::Branch("main".to_owned()));
        for schwung in 0..2 {
            gitmodell.verlauf_anhaengen(
                (0..VERLAUFSSCHRITT)
                    .map(|nummer| {
                        gitcommit(u8::try_from((schwung * 7 + nummer) % 256).expect("unter 256"))
                    })
                    .collect(),
            );
        }
        gitmodell.marken_setzen(&[("a.txt".to_owned(), Marke::Geaendert)]);
        assert_eq!(
            liste.aktiver().gitmodell().verlaufslaenge(),
            2 * VERLAUFSSCHRITT
        );

        liste.ordner_setzen(zweiter.pfad(), None);

        let gitmodell = liste.aktiver().gitmodell();
        assert_eq!(
            gitmodell.verlaufslaenge(),
            0,
            "der neue Ordner faengt bei den ersten fuenfzig an"
        );
        assert_eq!(
            gitmodell.kopfzeile(),
            "",
            "und traegt den Kopf des alten nicht"
        );
        assert_eq!(gitmodell.zusammenfassung(), "");
        assert!(!gitmodell.erschoepft());
    }

    /// C7.5, Tabhaelfte: ein verspaeteter Befund schreibt keine Marke in den
    /// neuen Bestand.
    ///
    /// Die Zuordnung laeuft ueber den **Namen**, und derselbe Name kann im
    /// neuen Ordner ebenso stehen; der Schutz ist deshalb die Generation und
    /// nicht der Bestand.
    #[test]
    fn ein_verspaeteter_gitbefund_schreibt_nichts_in_den_neuen_bestand() {
        let marken = vec![("a.txt".to_owned(), Marke::Geaendert)];

        // Fremde Generation: der Befund gehoert zum vorigen Lesevorgang.
        let mut liste = gelesene_liste(&["a.txt"]);
        liste.tabs[0].gelesen = true;
        liste.tabs[0].gitgeneration = liste.aktiver().modell().generation() + 1;
        liste.tabs[0].wartende_marken = Some(marken.clone());
        let einzug = liste.einziehen();
        assert!(
            !einzug.gitmarken_neu,
            "ein Befund fremder Generation traegt nichts ein"
        );
        assert_eq!(liste.aktiver().modell().gitmarke(0), None);

        // Gegenprobe: dieselbe Meldung mit der eigenen Generation kommt an.
        let mut liste = gelesene_liste(&["a.txt"]);
        liste.tabs[0].gelesen = true;
        liste.tabs[0].gitgeneration = liste.aktiver().modell().generation();
        liste.tabs[0].wartende_marken = Some(marken);
        let einzug = liste.einziehen();
        assert!(
            einzug.gitmarken_neu,
            "sonst belegt die Probe darueber nichts"
        );
        assert_eq!(
            liste.aktiver().modell().gitmarke(0),
            Some(Marke::Geaendert),
            "und der Buchstabe steht am Eintrag"
        );
    }

    /// A8 und C7.3: die Marken warten, bis der Bestand steht, und gehen dabei
    /// nicht verloren.
    ///
    /// Kopf und Verlauf stehen in dieser Spanne schon; das haelt die Probe
    /// `der_gitlauf_beginnt_zugleich_mit_dem_lesevorgang` darueber fest.
    #[test]
    fn die_marken_warten_auf_den_bestand_und_gehen_dabei_nicht_verloren() {
        let mut liste = gelesene_liste(&["a.txt"]);
        liste.tabs[0].gitgeneration = liste.aktiver().modell().generation();
        liste.tabs[0].wartende_marken = Some(vec![("a.txt".to_owned(), Marke::Neu)]);

        // Solange der Lesevorgang laeuft, wird nichts eingetragen — und nichts
        // weggeworfen.
        liste.tabs[0].gelesen = false;
        let einzug = liste.einziehen();
        assert!(!einzug.gitmarken_neu, "der Bestand steht noch nicht");
        assert_eq!(liste.aktiver().modell().gitmarke(0), None);
        assert!(
            liste.tabs[0].wartende_marken.is_some(),
            "die Meldung liegt weiter bereit und ist nicht verworfen"
        );

        // Sobald er steht, kommt der Befund an.
        liste.tabs[0].lesevorgang = None;
        liste.tabs[0].gelesen = true;
        let einzug = liste.einziehen();
        assert!(einzug.gitmarken_neu);
        assert_eq!(liste.aktiver().modell().gitmarke(0), Some(Marke::Neu));
        assert!(liste.tabs[0].wartende_marken.is_none());
    }

    /// E12 und C7.11: der Nachschlag haengt an drei Fragen, und jede kann ihn
    /// abweisen.
    ///
    /// Die dritte ist die tragende: **laeuft schon ein Lauf, faengt keiner
    /// an.** Sonst naehme der Nachschlag dem laufenden die Markenmeldung mit,
    /// die womoeglich noch auf den gelesenen Bestand wartet.
    #[test]
    fn ein_nachschlag_faengt_nur_an_wenn_kein_lauf_steht() {
        use krk_core::git::lauf::VERLAUFSSCHRITT;

        let ordner = crate::pruefordner::Pruefordner::neu("gitverlauf-nachschlag");
        let pfad = ordner.pfad().display().to_string();

        // Ohne Verlauf gibt es keine Stelle, ab der nachzuladen waere.
        let mut leer = liste(&[&pfad]);
        assert!(!leer.verlauf_nachladen(), "der Verlauf ist leer");

        // Ein erschoepfter Verlauf laedt nicht nach (C4.3).
        let mut erschoepft = liste(&[&pfad]);
        erschoepft.tabs[0]
            .gitmodell
            .verlauf_anhaengen(vec![gitcommit(1)]);
        assert!(
            !erschoepft.verlauf_nachladen(),
            "der Rest ist da, es folgt nichts mehr"
        );

        // Ein voller Schwung laesst offen: hier faengt der Nachschlag an.
        let mut offen = liste(&[&pfad]);
        offen.tabs[0].gitmodell.verlauf_anhaengen(
            (0..VERLAUFSSCHRITT)
                .map(|nummer| gitcommit(u8::try_from(nummer % 256).expect("unter 256")))
                .collect(),
        );
        assert!(
            offen.verlauf_nachladen(),
            "hinter dem letzten geht es weiter"
        );
        assert_eq!(stehende_gitlaeufe(&offen), 1);
        assert!(
            !offen.verlauf_nachladen(),
            "und ein zweiter stellt sich nicht daneben"
        );
        assert_eq!(stehende_gitlaeufe(&offen), 1);
    }

    /// C7.10: der Gitlauf wird an genau den Stellen angestossen, die A9 nennt,
    /// und an keiner weiteren.
    ///
    /// **Eine Aufruferzaehlung und ausdruecklich die richtige Form:** C7.10
    /// sagt die Zahl der Stellen selbst zu ("ueber keinen zweiten Weg"). Der
    /// Kopf von [`crate::quellbaum`] sagt, wann eine solche Zaehlung gehoert
    /// und wann nicht.
    ///
    /// Zwei Zaehlungen. Die erste haelt fest, dass ein Lauf im ganzen Baum
    /// allein in dieser Datei entsteht. Die zweite nennt die vier Anlaesse
    /// namentlich, aus denen der Nachzug gerufen wird: `lesen_starten` (jedes
    /// Neulesen eines Ordners, und damit der eine Auffrischungspfad und jede
    /// Navigation), `waehlen` zweimal (der Abbruch am verlassenen Tab und der
    /// Anstoss am neuen) und `git_gefragt_setzen` (das Einschalten des
    /// Bereichs oder der Spalte).
    ///
    /// **Ihre Blindheit** gehoert dazu: ein Aufruf unter anderem Namen — ein
    /// `use … as anders;` — entgeht ihr, wie jeder Suche im Quelltext.
    #[test]
    fn der_gitlauf_wird_an_genau_den_stellen_aus_a9_angestossen() {
        use crate::quellbaum::{aufrufstellen, quelldateien};

        let start = concat!("Gitlauf::star", "ten");
        let nachzug = concat!("gitlauf_nachziehen", "_an");

        let rufer: Vec<(String, usize)> = quelldateien()
            .into_iter()
            .filter(|(name, _)| name.contains("/src/"))
            .map(|(name, inhalt)| {
                let zahl = aufrufstellen(&inhalt, start);
                (name, zahl)
            })
            .filter(|(_, zahl)| *zahl > 0)
            .collect();
        assert_eq!(
            rufer,
            vec![("krk-ui/src/tabs.rs".to_owned(), 2)],
            "ein Gitlauf entsteht ausserhalb von tabs.rs oder oefter als in \
             Tabliste::gitlauf_nachziehen_an und Tabliste::verlauf_nachladen"
        );

        let diese_datei = quelldateien()
            .into_iter()
            .find(|(name, _)| name == "krk-ui/src/tabs.rs")
            .map(|(_, inhalt)| inhalt)
            .expect("krk-ui/src/tabs.rs steht nicht mehr im Baum");
        let code = diese_datei
            .split(concat!("#[cfg(", "test)]"))
            .next()
            .unwrap_or(&diese_datei);
        assert_eq!(
            aufrufstellen(code, nachzug),
            4,
            "der Nachzug hat andere Anlaesse als die vier aus A9: lesen_starten, \
             waehlen zweimal und git_gefragt_setzen"
        );
    }
}
