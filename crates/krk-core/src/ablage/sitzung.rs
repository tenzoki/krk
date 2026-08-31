//! Der Sitzungszustand: was Beenden und Neustart ueberlebt, und wie er
//! gebuendelt geschrieben wird.
//!
//! **Dies ist zugleich das Fenster- und Tabmodell, nicht nur seine
//! Serialisierung.** C7 verlangt, dass Tabs, Ordner, Auswahl, Breiten,
//! Sichtbarkeit und Sortierung einen Neustart ueberstehen; das gehaltene und
//! das geschriebene Modell sind damit derselbe Datenbestand. Schritt 12 laesst
//! dieses Modell wachsen und legt keine zweite Datei daneben. Jede Struktur
//! hier traegt deshalb `#[serde(default)]`: ein Feld, das Schritt 12
//! hinzufuegt, macht eine aeltere `session.toml` nicht ungueltig, sondern
//! nimmt seinen Auslieferungswert an.
//!
//! Was hier noch nicht steht, ist Absicht und nicht Auslassung. Die Markierung
//! mehrerer Eintraege und die Tabs des Vorschaufensters gehoeren zu
//! Faehigkeiten, die Schritt 13 und Schritt 19 erst bauen; sie kommen als
//! Felder in genau diese Strukturen. Die Bildlaufposition je Tab ist mit
//! Schritt 12 dazugekommen, als [`Tab::bildlauf`].
//!
//! **Was der Editor beitraegt, ist der Pfad seiner Datei und nicht ihr Stand.**
//! [`Sitzung::editor`] haelt fest, welche Datei offen ist, und sonst nichts;
//! der ungesicherte Stand bleibt draussen, und der Grund steht am Feld.
//!
//! [`Sitzungsschreiber`] haelt die zweite Zusage aus `### Frage 4`: der
//! Sitzungszustand wird gebuendelt geschrieben, hoechstens alle zwei Sekunden
//! und einmal beim Beenden.

use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use super::pfade;
use super::sperre::Sitzungsrecht;
use super::{Datei, Zugang};
use crate::verzeichnis::Sortierung;

/// Der Mindestabstand zwischen zwei Schreibvorgaengen des Sitzungszustands.
pub const SITZUNGSTAKT: Duration = Duration::from_secs(2);

/// Welches der beiden Dateifenster gemeint ist.
///
/// C1 kennt genau zwei, gleichrangig nebeneinander. Eine Seite statt einer
/// Zahl, damit ein Index nicht versehentlich zu drei Fenstern wird.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Fensterseite {
    /// Das linke Dateifenster.
    #[default]
    Links,
    /// Das rechte Dateifenster.
    Rechts,
}

impl Fensterseite {
    /// Beide Seiten, links zuerst.
    pub const ALLE: [Fensterseite; 2] = [Fensterseite::Links, Fensterseite::Rechts];

    /// Die Stelle im Feld der beiden Dateifenster.
    pub const fn index(self) -> usize {
        match self {
            Fensterseite::Links => 0,
            Fensterseite::Rechts => 1,
        }
    }

    /// Die jeweils andere Seite. Bei Dateioperationen ist sie das Ziel.
    pub const fn andere(self) -> Self {
        match self {
            Fensterseite::Links => Fensterseite::Rechts,
            Fensterseite::Rechts => Fensterseite::Links,
        }
    }
}

/// Ein Tab eines Dateifensters: ein Ordner mit seiner Sicht darauf.
///
/// Kein `Eq`, und der Grund ist [`Tab::bildlauf`]: eine Gleitkommazahl kennt
/// keine vollstaendige Gleichheit. Dieselbe Ueberlegung wie bei [`Breiten`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tab {
    /// Der Ordner, den dieser Tab zeigt.
    pub ordner: PathBuf,
    /// Der Name des ausgewaehlten Eintrags, falls einer ausgewaehlt ist.
    ///
    /// Der Name und nicht die Zeilennummer: zwischen Beenden und Neustart kann
    /// sich der Ordnerinhalt geaendert haben, und eine Zeilennummer zeigte
    /// dann auf einen anderen Eintrag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auswahl: Option<String>,
    /// Ob versteckte Eintraege ausgeblendet sind.
    pub verstecke_ausgeblendet: bool,
    /// Nach welchem Schluessel und in welche Richtung sortiert ist.
    pub sortierung: Sortierung,
    /// Die Bildlaufposition in Punkten, vom oberen Rand der Liste aus.
    ///
    /// **0 heisst "ganz oben", und das gilt seit dem 260805 auch geschrieben.**
    /// Bis dahin stand hier der rohe Ursprung der Bildlaufansicht, der um die
    /// Hoehe der Spaltenueberschriften darueber liegt, und ein ungescrollter Tab
    /// trug `-28.0`. Diese Datei soll der Nutzer lesen und von Hand aendern
    /// koennen; eine negative Zahl fuer den obersten Stand war dort eine
    /// Stolperstelle
    /// (`issues/260804-1040_*_die-bildlaufposition-in-der-session-toml-steht-am-oberen-rand-auf-minus-28.md`).
    /// Umgerechnet wird in `krk_ui::appkit::tabelle`, an der einen Stelle, die
    /// die Ansicht ueberhaupt liest und setzt.
    ///
    /// Sie gehoert zum Tab und nicht zum Dateifenster: C1 verlangt, dass zwei
    /// Dateifenster denselben Ordner zeigen koennen, ohne dass sich ihre
    /// Bildlaufposition gegenseitig beeinflusst, und dieselbe Trennung gilt
    /// zwischen zwei Tabs eines Fensters.
    pub bildlauf: f64,
}

impl Default for Tab {
    /// Der Auslieferungszustand: das Benutzerverzeichnis, nichts ausgewaehlt,
    /// versteckte Eintraege aus, Name aufsteigend, ganz oben.
    fn default() -> Self {
        Self {
            ordner: standardordner(),
            auswahl: None,
            verstecke_ausgeblendet: true,
            sortierung: Sortierung::default(),
            bildlauf: 0.0,
        }
    }
}

impl Tab {
    /// Ein Tab auf einem Ordner, sonst im Auslieferungszustand.
    pub fn auf(ordner: impl Into<PathBuf>) -> Self {
        Self {
            ordner: ordner.into(),
            ..Self::default()
        }
    }
}

/// Eines der beiden Dateifenster mit seinen Tabs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dateifenster {
    /// Die Stelle des sichtbaren Tabs in [`Dateifenster::tabs`].
    pub aktiver_tab: usize,
    /// Die Tabs, in der Reihenfolge der Leiste. Nie leer, siehe C1.
    pub tabs: Vec<Tab>,
}

impl Default for Dateifenster {
    /// Ein Fenster mit genau einem Tab. C1 verlangt mindestens einen.
    fn default() -> Self {
        Self {
            aktiver_tab: 0,
            tabs: vec![Tab::default()],
        }
    }
}

impl Dateifenster {
    /// Der sichtbare Tab, falls [`Dateifenster::aktiver_tab`] auf einen zeigt.
    ///
    /// Die Sitzung kommt von der Platte und kann eine Stelle nennen, die es
    /// nicht gibt. Die Ablage weist das nicht als beschaedigt zurueck, weil
    /// eine um eins verrutschte Zahl den Nutzer nicht seine Tabs kosten soll;
    /// wer den Tab braucht, bekommt hier `None` und entscheidet selbst.
    pub fn aktiver_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.aktiver_tab)
    }
}

/// Die Breiten der sechs Bereiche in Punkten, soweit KRK sie schon kennt.
///
/// `None` heisst "noch nie gesetzt": dann waehlt der Aufbau der Oberflaeche
/// die Breite. Eine gespeicherte Zahl gilt auch fuer einen ausgeblendeten
/// Bereich, weil C7 verlangt, dass das Wiedereinblenden die vorherige Breite
/// herstellt.
///
/// Die Reihenfolge der Felder ist die der Fensterzeile von links nach rechts
/// und damit die von `krk_ui::fenstermodell::Bereich::ALLE`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Breiten {
    /// Die Lesezeichen- und Geraeteleiste ganz links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesezeichen: Option<f64>,
    /// Das linke Dateifenster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<f64>,
    /// Das rechte Dateifenster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rechts: Option<f64>,
    /// Das Vorschaufenster ganz rechts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vorschau: Option<f64>,
    /// Der eingebaute Editor, der sich die Stelle ganz rechts mit dem
    /// Vorschaufenster teilt.
    ///
    /// Das Feld ist mit der Editor-Runde dazugekommen. Eine `session.toml` aus
    /// der Zeit davor bleibt lesbar, weil diese Struktur `#[serde(default)]`
    /// traegt; die Probe dazu steht in `tests/ablage.rs`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<f64>,
    /// Der Git-Bereich, der sich die Stelle ganz rechts mit dem
    /// Vorschaufenster und dem Editor teilt.
    ///
    /// Das Feld ist mit der Git-Runde dazugekommen und steht wie die uebrigen
    /// auf `None`, solange niemand eine Breite gesetzt hat. Eine
    /// `session.toml` aus der Zeit davor bleibt lesbar, weil diese Struktur
    /// `#[serde(default)]` traegt; die Probe dazu steht in `tests/ablage.rs`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git: Option<f64>,
}

/// Welche der sechs Bereiche sichtbar sind.
///
/// **Alle sechs tragen ein Feld, die beiden Dateifenster eingeschlossen.** Bis
/// zur Bereichsleisten-Runde fehlte das linke mit Absicht, weil es sich nicht
/// ausblenden liess; seit dem Nutzerentscheid vom 260812-0306
/// (`decisions/260811-1305_*_traegt-das-linke-dateifenster-einen-schalter.md`)
/// laesst es sich, solange das rechte steht.
///
/// **Die Regel heisst danach "eines bleibt" und nicht "das linke ist
/// besonders", und sie steht nicht hier.** Diese Struktur traegt die Angabe und
/// haelt keine Zusage darueber; eingeloest wird die Regel im Fenstermodell von
/// `krk-ui`: `Fenstermodell::umschalten` weist zur Laufzeit jeden Befehl ab,
/// der das letzte sichtbare Dateifenster ausblenden wuerde, und
/// `Fenstermodell::aus_sitzung` macht das linke sichtbar, wenn eine von Hand
/// geschriebene Datei beide ausblendet.
///
/// Die Reihenfolge der Felder ist die der Fensterzeile von links nach rechts,
/// dieselbe wie bei [`Breiten`] darueber und damit die von
/// `krk_ui::fenstermodell::Bereich::ALLE`. Sie ist keine Geschmacksfrage: `serde`
/// schreibt die Zeilen in dieser Reihenfolge, und `session.toml` ist nach C7
/// zum Lesen von Hand gedacht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sichtbarkeit {
    /// Die Lesezeichen- und Geraeteleiste.
    pub lesezeichen: bool,
    /// Das erste, linke Dateifenster.
    ///
    /// Mit der Bereichsleisten-Runde dazugekommen; eine `session.toml` aus der
    /// Zeit davor bleibt lesbar und nimmt hier den Vorgabewert `true` an, das
    /// fehlende Feld heisst also "sichtbar". Die Probe dazu steht in
    /// `tests/ablage.rs`.
    pub erstes_dateifenster: bool,
    /// Das zweite, rechte Dateifenster.
    pub zweites_dateifenster: bool,
    /// Das Vorschaufenster.
    pub vorschau: bool,
    /// Der eingebaute Editor.
    ///
    /// Mit der Editor-Runde dazugekommen; eine `session.toml` aus der Zeit
    /// davor bleibt lesbar und nimmt hier den Vorgabewert an.
    pub editor: bool,
    /// Der Git-Bereich.
    ///
    /// Mit der Git-Runde dazugekommen; eine `session.toml` aus der Zeit davor
    /// bleibt lesbar und nimmt hier den Vorgabewert `false` an, das fehlende
    /// Feld heisst also "ausgeblendet". Die Probe dazu steht in
    /// `tests/ablage.rs`.
    pub git: bool,
}

impl Default for Sichtbarkeit {
    /// Der Auslieferungszustand: die vier Bereiche der Runde 1 sichtbar, der
    /// Editor und der Git-Bereich ausgeblendet.
    ///
    /// **Die beiden Mitbewerber der Vorschau stehen auf `false`**, und das ist
    /// kein Versehen: beim allerersten Start haelt der Editor keine Datei, und
    /// ein sichtbarer leerer Editor naehme den Dateifenstern Platz fuer
    /// nichts; derselbe Grund gilt dem Git-Bereich, der sich dieselbe Flaeche
    /// teilt und ohne Repository nichts zu zeigen haette (A13). Beide kommen
    /// hervor, wenn sie jemand verlangt.
    fn default() -> Self {
        Self {
            lesezeichen: true,
            erstes_dateifenster: true,
            zweites_dateifenster: true,
            vorschau: true,
            editor: false,
            git: false,
        }
    }
}

/// Welche der schaltbaren Spalten der beiden Dateilisten stehen.
///
/// **Ein Feld weniger als die Aufzaehlung Werte hat: die Spalte Name traegt
/// keinen Schalter.** Eine
/// Dateiliste ohne sie zeigt nichts, was den Eintrag benennt, und ein Wert, den
/// niemand aendern kann, gehoert nicht in eine Datei, die der Nutzer nach C7
/// von Hand liest. **Das ist etwas anderes als die Luecke, die
/// [`Sichtbarkeit::erstes_dateifenster`] gerade geschlossen hat**: dort gab es
/// einen Wert, der wechseln kann, und er wurde nur nicht gespeichert; hier gibt
/// es keinen Schalter und damit keinen Wert. Die Zuordnung von einer Spalte auf
/// ihr Feld — die Namensspalte eingeschlossen, die immer steht — trifft
/// `krk_ui::fenstermodell::spalte_sichtbar_in`.
///
/// **Ein Feld gilt fuer beide Dateilisten.** Nutzerentscheid vom 260812-0306
/// (`decisions/260812-0306_*_gelten-die-spaltenschalter-fuer-beide-dateifenster-gemeinsam.md`):
/// die Bereichsleiste steht einmal ueber die volle Fensterbreite, und ein
/// Schalter darin, der nur eine Seite traefe, muesste anzeigen, welche. Eine
/// spaetere Runde, die es je Seite fuehren will, macht aus jedem Feld zwei.
///
/// **Das Wegschalten einer Spalte ruehrt die Sortierung nicht an** (Kriterium
/// C3.3 der Bereichsleisten-Runde, Nutzerentscheid vom 260812-0306). Der
/// Sortierschluessel steht in [`Tab::sortierung`] und bleibt, wo er steht; wer
/// nach Groesse sortiert und die Spalte Groesse wegschaltet, sieht dieselbe
/// Reihenfolge wie zuvor, nur ihren Schluessel nicht mehr.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Spaltensichtbarkeit {
    /// Die Spalte mit der Groesse der Daten.
    pub groesse: bool,
    /// Die Spalte mit dem Zeitpunkt der letzten Aenderung.
    pub geaendert: bool,
    /// Die Spalte mit der Dateiendung.
    pub typ: bool,
    /// Die Spalte mit der Git-Marke des Eintrags.
    ///
    /// Mit der Git-Runde dazugekommen; eine `session.toml` aus der Zeit davor
    /// bleibt lesbar und nimmt hier den Vorgabewert `true` an, das fehlende
    /// Feld heisst also "steht" (A13, C5.9). Die Probe dazu steht in
    /// `tests/ablage.rs`.
    pub marke: bool,
}

impl Default for Spaltensichtbarkeit {
    /// Der Auslieferungszustand: jede schaltbare Spalte steht.
    ///
    /// Damit heisst ein fehlender Abschnitt `[spalten]` dasselbe wie der
    /// bisherige Zustand, und eine `session.toml` aus der Zeit davor geht
    /// unveraendert auf. **Die Markenspalte folgt darin den drei aelteren**
    /// (A13, C5.10): sie steht ab Werk, waehrend der Git-Bereich ab Werk
    /// ausgeblendet ist. Der Unterschied hat einen Grund — eine Spalte in der
    /// Dateiliste nimmt dem Fenster ein paar Punkte Breite, ein Bereich der
    /// Fensterzeile nimmt ihm eine Flaeche.
    fn default() -> Self {
        Self {
            groesse: true,
            geaendert: true,
            typ: true,
            marke: true,
        }
    }
}

/// Der ganze Sitzungszustand, wie er in `session.toml` steht.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sitzung {
    /// Welches Dateifenster das aktive ist. Bei Dateioperationen die Quelle.
    pub aktiv: Fensterseite,
    /// Die Datei, die der eingebaute Editor haelt; `None`, wenn er keine haelt.
    ///
    /// **Der Pfad und sonst nichts.** Weder der bearbeitete Stand noch die
    /// Abweichungsmarke gehoeren hierher: die getaktete Sitzungssicherung fragt
    /// nichts und traegt den ungesicherten Stand nicht mit (siebtes
    /// Abnahmekriterium von C4 der Editor-Runde, Datensatz
    /// `260807-2147_*_wie-greift-die-nachfrage-bei-der-sitzungssicherung.md`).
    /// Der Preis steht dort und wird nicht verschwiegen: bei einem Absturz ist
    /// der ungesicherte Stand verloren. Beim naechsten Start kommt die Datei so
    /// herein, wie sie auf der Platte steht.
    ///
    /// **Das Feld steht vor den drei Tabellen und nicht hinter ihnen.** TOML
    /// verlangt, dass die Werte einer Tabelle vor ihren Untertabellen stehen;
    /// eine Zeile hinter `[breiten]` liesse das Schreiben scheitern. Die
    /// Reihenfolge der Felder ist damit keine Geschmacksfrage.
    ///
    /// Es ist mit der Editor-Runde dazugekommen. Eine `session.toml` aus der
    /// Zeit davor bleibt lesbar, weil diese Struktur `#[serde(default)]`
    /// traegt; die Probe dazu steht in `tests/ablage.rs`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<PathBuf>,
    /// Welcher der beiden Notizzettel zuletzt offen war.
    ///
    /// **Die Merkung und nicht der Text.** Was auf den Zetteln steht, liegt in
    /// `note-1.txt` und `note-2.txt` und kommt in diese Datei an keiner Stelle;
    /// der Zwei-Sekunden-Takt des [`Sitzungsschreiber`] traegt damit vom Zettel
    /// nichts als diese eine Angabe. C4 der Runde 9 sagt beides zu, und eine
    /// Probe in `tests/ablage.rs` haelt es fest. Es ist derselbe Zuschnitt wie
    /// bei [`Sitzung::editor`] weiter oben: der Ort und nicht der Stand.
    ///
    /// **Vor den drei Tabellen und vor `fenster`**, aus dem Grund, den
    /// [`Sitzung::editor`] ausschreibt: TOML verlangt, dass die Werte einer
    /// Tabelle vor ihren Untertabellen stehen.
    ///
    /// Mit der Runde 9 dazugekommen. Eine `session.toml` aus der Zeit davor
    /// bleibt lesbar, weil diese Struktur `#[serde(default)]` traegt, und
    /// ergibt den ersten Zettel.
    pub zettel: pfade::Zettel,
    /// Wie der Git-Bereich die Flaeche unter seinem Kopf teilt: der Anteil, den
    /// die Verlaufsliste davon bekommt.
    ///
    /// **Ein Anteil und keine Punktzahl, anders als bei [`Breiten`] daneben**,
    /// und das ist kein Bruch mit der Waehrung dieser Datei, sondern eine
    /// andere Groesse. Die Breiten teilen die Fensterzeile, deren Zahl von
    /// aussen kommt; hier wird eine Flaeche geteilt, deren Hoehe mit jeder
    /// Fenstergroesse eine andere ist. Eine Punktzahl gaelte nur fuer die Hoehe,
    /// bei der sie entstanden ist, und der Nutzer, der diese Datei nach C7 von
    /// Hand liest, kann `0.5` deuten und `181.0` nicht.
    ///
    /// `None` heisst "noch nie gesetzt", wie bei [`Breiten`]: dann teilt der
    /// Git-Bereich haelftig. Der Wert liegt zwischen 0 und 1; was davon uebrig
    /// bleibt, gehoert der Flaeche der Einzelheiten. Die Mindesthoehen der
    /// beiden Flaechen deckeln ihn beim Anwenden, und die kennt allein
    /// `krk_ui::appkit::git`.
    ///
    /// Es ist mit dem Nutzerbefund vom 260831 dazugekommen. Eine `session.toml`
    /// aus der Zeit davor bleibt lesbar, weil diese Struktur
    /// `#[serde(default)]` traegt; die Probe dazu steht in `tests/ablage.rs`.
    ///
    /// **Vor den drei Tabellen**, aus dem Grund, den [`Sitzung::editor`]
    /// ausschreibt: TOML verlangt, dass die Werte einer Tabelle vor ihren
    /// Untertabellen stehen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitanteil: Option<f64>,
    /// Die Breiten der sechs Bereiche.
    pub breiten: Breiten,
    /// Welche Bereiche sichtbar sind.
    pub sichtbar: Sichtbarkeit,
    /// Welche der schaltbaren Spalten beide Dateilisten zeigen.
    ///
    /// **Vor `fenster` und nicht dahinter**, aus demselben Grund, den
    /// [`Sitzung::editor`] weiter oben ausschreibt: TOML verlangt, dass eine
    /// Tabelle vor einer Tabellenfolge steht, und `[spalten]` hinter den beiden
    /// `[[fenster]]` liesse das Schreiben scheitern.
    ///
    /// Mit der Bereichsleisten-Runde dazugekommen. Eine `session.toml` aus der
    /// Zeit davor bleibt lesbar, weil diese Struktur `#[serde(default)]`
    /// traegt; die Probe dazu steht in `tests/ablage.rs`.
    pub spalten: Spaltensichtbarkeit,
    /// Die beiden Dateifenster, links zuerst.
    ///
    /// Zwei, weil C1 zwei verlangt. Ein `session.toml` mit einer anderen Zahl
    /// ist beschaedigt und fuehrt zum Auslieferungszustand.
    pub fenster: [Dateifenster; 2],
}

impl Default for Sitzung {
    /// Der Auslieferungszustand: zwei Fenster mit je einem Tab auf dem
    /// Benutzerverzeichnis, die vier Bereiche der Runde 1 sichtbar, Editor und
    /// Git-Bereich ausgeblendet und der Editor ohne Datei, alle fuenf Spalten
    /// sichtbar, links aktiv, der erste Notizzettel offen, der Git-Bereich
    /// ungeteilt gelassen.
    fn default() -> Self {
        Self {
            aktiv: Fensterseite::default(),
            editor: None,
            zettel: pfade::Zettel::default(),
            gitanteil: None,
            breiten: Breiten::default(),
            sichtbar: Sichtbarkeit::default(),
            spalten: Spaltensichtbarkeit::default(),
            fenster: [Dateifenster::default(), Dateifenster::default()],
        }
    }
}

impl Sitzung {
    /// Eines der beiden Dateifenster.
    pub fn fenster(&self, seite: Fensterseite) -> &Dateifenster {
        &self.fenster[seite.index()]
    }

    /// Eines der beiden Dateifenster, veraenderlich.
    pub fn fenster_mut(&mut self, seite: Fensterseite) -> &mut Dateifenster {
        &mut self.fenster[seite.index()]
    }
}

/// Der Ordner, den ein Tab ohne eigene Angabe zeigt.
///
/// Das Benutzerverzeichnis, und ohne eines die Wurzel. Der Rueckfall ist kein
/// zweiter Weg, sondern die Antwort auf eine Lage, in der KRK ohnehin schon
/// gemeldet hat, dass es keinen Ablageordner anlegen kann: ein Dateifenster
/// muss trotzdem einen Ordner zeigen, und `/` gibt es immer.
fn standardordner() -> PathBuf {
    pfade::benutzerverzeichnis().unwrap_or_else(|| PathBuf::from("/"))
}

/// Schreibt den Sitzungszustand gebuendelt: hoechstens alle
/// [`SITZUNGSTAKT`] und einmal beim Beenden.
///
/// **Ein Schreibweg, zwei Ausloeser.** [`Sitzungsschreiber::vormerken`] meldet
/// eine Aenderung, [`Sitzungsschreiber::abgleichen`] ist der Takt, der einen
/// liegengebliebenen Stand nachtraegt, wenn keine weitere Aenderung mehr
/// kommt. Beide laufen in dieselbe private Schreibfunktion; einen zweiten Weg
/// auf die Platte gibt es nicht.
///
/// Die Zeit kommt von aussen und nicht aus [`Instant::now`]. Damit ist die
/// Buendelung ohne Warten pruefbar, und der Aufrufer bestimmt, welche Uhr
/// gilt.
///
/// **Den Pfad traegt der Schreiber seit der Runde 7 nicht mehr, sondern der
/// [`Zugang`], den jede der drei Schreibmethoden entgegennimmt.** Er ist damit
/// der vierte Schreiber hinter derselben Grenze wie die drei anderen: es gibt
/// keinen Weg von hier auf die Platte, der nicht unter der Schreibsperre
/// laeuft. Der Schreiber selbst haelt weiter allein den vorgemerkten Stand und
/// die Uhr, denn beide ueberdauern einen Durchgang.
///
/// **Er entsteht nur, wenn dieser Prozess das Sitzungsrecht haelt**, und seit
/// der Runde 7 haelt der Uebersetzer das fest: beide Erzeuger verlangen ein
/// [`Sitzungsrecht`] und liefern `None`, wenn eine andere Instanz es haelt.
///
/// Bis dahin stand die Regel an einem fehlenden Wert — wer das Recht nicht
/// bekam, baute eben keinen Schreiber —, und genau daran ist sie einmal
/// vorbeigelaufen: der Messmodus baute sich einen ohne zu fragen
/// (`issues/260813-0540_*_der-messmodus-schreibt-die-sitzung-ohne-sitzungsrecht.md`).
/// Ein fehlender Wert ist eine Abmachung; ein verlangtes Argument ist eine
/// Eigenschaft der Typen.
#[derive(Debug)]
pub struct Sitzungsschreiber {
    takt: Duration,
    /// Der vorgemerkte, noch nicht geschriebene Stand.
    offen: Option<Sitzung>,
    /// Wann zuletzt geschrieben wurde. `None` heisst: noch nie.
    zuletzt: Option<Instant>,
}

impl Sitzungsschreiber {
    /// Ein Schreiber mit dem Takt aus [`SITZUNGSTAKT`], sofern dieser Prozess
    /// das Sitzungsrecht haelt.
    ///
    /// `None` heisst „eine andere Instanz schreibt die Sitzung". Das Recht wird
    /// hier weder genommen noch gehalten: es kommt von
    /// [`Sitzungsrecht::nehmen`], wird einmal beim Start vergeben und lebt
    /// laenger als jeder Schreiber.
    pub fn neu(recht: &Sitzungsrecht) -> Option<Self> {
        Self::mit_takt(recht, SITZUNGSTAKT)
    }

    /// Ein Schreiber mit abweichendem Takt, sofern dieser Prozess das
    /// Sitzungsrecht haelt.
    pub fn mit_takt(recht: &Sitzungsrecht, takt: Duration) -> Option<Self> {
        if !recht.gehalten() {
            return None;
        }
        Some(Self {
            takt,
            offen: None,
            zuletzt: None,
        })
    }

    /// Ob ein Stand vorgemerkt und noch nicht geschrieben ist.
    pub fn steht_aus(&self) -> bool {
        self.offen.is_some()
    }

    /// Merkt einen neuen Stand vor und schreibt ihn, falls der Takt es
    /// zulaesst.
    ///
    /// Liefert `true`, wenn dabei geschrieben wurde.
    pub fn vormerken(
        &mut self,
        sitzung: Sitzung,
        jetzt: Instant,
        zugang: &Zugang<'_>,
    ) -> io::Result<bool> {
        self.offen = Some(sitzung);
        self.abgleichen(jetzt, zugang)
    }

    /// Schreibt einen vorgemerkten Stand, sobald der Takt abgelaufen ist.
    ///
    /// Ohne vorgemerkten Stand oder vor Ablauf des Takts passiert nichts.
    /// Liefert `true`, wenn geschrieben wurde.
    pub fn abgleichen(&mut self, jetzt: Instant, zugang: &Zugang<'_>) -> io::Result<bool> {
        if self.offen.is_none() {
            return Ok(false);
        }
        if let Some(zuletzt) = self.zuletzt
            && jetzt.saturating_duration_since(zuletzt) < self.takt
        {
            return Ok(false);
        }
        self.schreiben(jetzt, zugang)
    }

    /// Der eine Schreibvorgang beim Beenden: schreibt einen vorgemerkten
    /// Stand ohne Ruecksicht auf den Takt.
    ///
    /// Liefert `true`, wenn geschrieben wurde.
    pub fn beenden(&mut self, jetzt: Instant, zugang: &Zugang<'_>) -> io::Result<bool> {
        self.schreiben(jetzt, zugang)
    }

    fn schreiben(&mut self, jetzt: Instant, zugang: &Zugang<'_>) -> io::Result<bool> {
        let Some(sitzung) = self.offen.as_ref() else {
            return Ok(false);
        };
        zugang.sichern(Datei::Sitzung, sitzung)?;
        self.offen = None;
        self.zuletzt = Some(jetzt);
        Ok(true)
    }
}
