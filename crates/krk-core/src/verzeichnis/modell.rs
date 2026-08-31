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
//!
//! # Ein Lesevorgang ersetzt, er leert nicht vorab
//!
//! ```text
//! lesevorgang_beginnen(g)   Generation g, Ersatz vorgemerkt
//!        │                  der alte Bestand steht weiter auf dem Schirm
//!        ├── erster Stapel ──> Ersatz eingeloest, dann angehaengt
//!        └── kein Stapel  ────> abschliessen loest den Ersatz ein
//! ```
//!
//! Bis zum 260807 leerte eine Methode `leeren` das Modell beim **Start** eines
//! Lesevorgangs. Traf die naechste Aenderungsmeldung ein, bevor der erste
//! Stapel angehaengt war, setzte sie den Lesevorgang neu auf, und der Nutzer sah
//! fuer die ganze Laufzeit eine leere Liste
//! (`issues/260805-1337_*_die-dateiliste-ist-waehrend-eines-stapel-umbenennens-im-angezeigten-ordner-leer.md`).
//! [`Ordnermodell::lesevorgang_beginnen`] merkt den Ersatz stattdessen nur vor;
//! eingeloest wird er von dem, was als Erstes kommt, und danach nie ein zweites
//! Mal. Der Bestand ist damit zu keinem Zeitpunkt aus zwei Ordnern gemischt, und
//! eine leere Zwischenzeit gibt es nur noch dort, wo der neue Ordner wirklich
//! leer ist.
//!
//! # Ob eine Zeile steht, entscheidet ein Pruefschritt und kein zweiter
//!
//! Der Pruefschritt ist die eine Stelle, an der ueber die Sichtbarkeit eines
//! Eintrags entschieden wird. Er steht in zwei Stuecken, und die Naht dazwischen
//! traegt: `zeilengrund_von` beantwortet alles, was das Modell selbst weiss,
//! und [`Ordnermodell::sichtbar`] haengt den Blick in den Befund daran. Beide
//! Aufbauwege der Sicht rufen `sichtbar`: [`Ordnermodell::anhaengen`] je neuem
//! Eintrag und [`Ordnermodell::sicht_neu_aufbauen`] je Eintrag des Bestands.
//!
//! ```text
//! versteckt und ausgeblendet? ── ja ──> faellt weg
//!            │ nein
//! steht ein Filtertext?       ── nein ─> steht in der Liste
//!            │ ja
//! Name traegt die Folge?      ── ja ──> steht in der Liste
//!            │ nein
//! ist es ein Ordner?          ── nein ─> wirkt "Content"? ── nein ─> faellt weg
//!            │ ja                            │ ja
//!            │                        traegt der Inhalt? ── Treffer ──> steht
//!            │                                           └ sonst ────> faellt weg
//! ist "Deep" eingeschaltet?   ── nein ─> steht in der Liste
//!            │ ja
//! liegt unter ihm ein Treffer? ─ ja ──> steht in der Liste
//!                              └ sonst > faellt weg
//! ```
//!
//! Die beiden Fragen an den Befund — „traegt der Inhalt?" und „liegt unter ihm
//! ein Treffer?" — sind der letzte Schritt und stehen in
//! [`Ordnermodell::sichtbar`]; alles davor rechnet `zeilengrund_von` und legt
//! es als `Zeilengrund` ab.
//!
//! **Bis zur Runde 10 stand die Regel zweimal wortgleich da**, einmal in
//! `anhaengen` und einmal in `sicht_neu_aufbauen`, und trug damals nur ihren
//! ersten Zweig
//! (`issues/260814-2102_*_der-pruefschritt-fuer-die-sichtbarkeit-steht-im-ordnermodell-zweimal-wortgleich-da.md`).
//! Eine Regel mit sechs Eingaben an zwei Stellen zu fuehren waere eine zweite
//! Wahrheit ueber dieselbe Sache; der Inhaltsfilter der Runde 11 ist wieder ein
//! Zweig mehr in demselben Pruefschritt und keine zweite Sicht daneben.
//!
//! **Fuenf der sechs Eingaben wohnen hier, die sechste kommt von aussen.**
//! `verstecke_ausblenden`, `filtertext`, `tief`, `inhalt` und die Zeichenzahl
//! des Filtertexts sind Groessen dieses Modells; wer den Unterbaum abschreitet
//! oder eine Datei liest und den Befund liefert, ist nicht Sache dieser Datei.
//! Sie nimmt ihn ueber [`Ordnermodell::befunde_setzen`] entgegen und baut die
//! Sicht damit neu auf.
//!
//! **Genau daran liegt die Naht.** Die fuenf eigenen Eingaben aendern sich, wenn
//! der Nutzer tippt oder einen Schalter umlegt — selten also, gemessen an dem,
//! was dazwischen geschieht. Die sechste trifft waehrend eines Durchlaufs
//! sechzigmal in der Sekunde ein. Der Zeilengrund haelt das Ergebnis der fuenf
//! fest, und ein eintreffender Befund baut damit die Sicht neu auf, ohne die
//! Namensfrage noch einmal an 100.000 Eintraege zu stellen.
//!
//! # Ein Befund gilt nur zu der Frage, die ihn erzeugt hat
//!
//! Der Befundvektor ist eine Sammlung von Antworten, und die Frage steht nicht
//! bei jeder Antwort dabei. Sie lautet fuer jeden Eintrag gleich — traegt er
//! den Filtertext unter sich oder in sich, und zaehlt sein Inhalt dabei mit? —,
//! und sie steckt in zwei Groessen: dem Muster aus dem Filtertext
//! ([`filter::Muster`]) und dem, was [`Ordnermodell::inhalt_wirkt`] sagt. Aendert sich eine von beiden, faellt
//! der ganze Vektor auf `Unentschieden`; das besorgen `filter_uebernehmen` und
//! `schalter_setzen`, und sonst niemand.
//!
//! **Der Stand der tiefen Suche gehoert nicht dazu.** Er entscheidet, ob die
//! Frage fuer einen Ordner ueberhaupt gestellt wird, und nicht, wie sie
//! ausgeht: derselbe Unterbaum wird immer gleich abgeschritten. Ihn trotzdem
//! mitzuzaehlen hiesse, beim Umlegen von „Deep" Antworten wegzuwerfen, die
//! weiter gelten.
//!
//! **Die Verstecke gehoeren erst recht nicht dazu.** Sie aendern den
//! Zeilengrund und damit die Auftragsliste, aber keine einzige Antwort.
//!
//! # Zwei Befundvektoren, zwei Ungueltigkeitsregeln
//!
//! Neben `befund` steht `gitmarke`, und die beiden fallen zu **verschiedenen**
//! Anlaessen. Der Filterbefund faellt mit der **Frage**: aendert sich das
//! Muster oder das, was [`Ordnermodell::inhalt_wirkt`] sagt, ist jede Antwort
//! im Vektor eine Auskunft ueber einen Filtertext, den es nicht mehr gibt
//! (`befund_zuruecksetzen`, gerufen von `filter_uebernehmen` und
//! [`Ordnermodell::schalter_setzen`]). Die Gitmarke faellt mit dem
//! **Bestand**: sie sagt etwas ueber eine Datei auf der Platte, und diese
//! Auskunft ueberlebt jedes Tippen, aber keinen Ordnerwechsel
//! (`ersatz_einloesen`).
//!
//! ```text
//!                        befund      gitmarke
//! Filtertext getippt      faellt      bleibt
//! "Content" umgelegt      faellt      bleibt
//! Verstecke umgelegt      bleibt      bleibt
//! Ordnerwechsel           faellt      faellt
//! ```
//!
//! **Ein gemeinsamer Vektor waere die eine Frage mit der anderen
//! weggeworfen.** Ein getipptes Zeichen wuerde den Statuslauf entwerten, der
//! Sekunden gekostet hat und nichts mit dem Filter zu tun hat; ein
//! Ordnerwechsel wuerde umgekehrt zu wenig wegwerfen, wenn die Marke am
//! Filterrhythmus haengt. `befund_zuruecksetzen` fasst `gitmarke` deshalb
//! ausdruecklich nicht an, und `ersatz_einloesen` leert beide.
//!
//! # Die beiden Treffergruende ueberschneiden sich nicht
//!
//! Eine Zeile steht entweder, weil ihr **Name** die Folge traegt, oder weil ihr
//! **Inhalt** sie traegt, und nie aus beiden Gruenden zugleich. Das leistet
//! keine zusaetzliche Regel, sondern der Kurzschluss des Namens: der Zweig
//! `Name traegt die Folge? ── ja` verlaesst den Pruefschritt, und der
//! Inhaltszweig liegt hinter ihm. Daran haengen zwei Dinge auf einmal — die
//! Ersparnis, denn eine namentlich passende Datei wird nie gelesen, und die
//! Ausschliesslichkeit, an der die abgesetzte Darstellung der Zelle haengt.
//!
//! [`Ordnermodell::steht_wegen_des_inhalts`] ist deshalb kein zweiter
//! Pruefschritt, sondern derselbe Rumpf mit den Vorbedingungen davor, die der
//! Pruefschritt an dieser Stelle schon hinter sich hat.

use std::collections::HashMap;
use std::sync::Arc;

use super::durchlauf::{Auftrag, Auftragsart};
use super::eintrag::Eintrag;
use super::filter::{self, Muster, traegt_die_folge};
use super::kollation;
use super::sortierung::{Richtung, Schluessel, Sortierung};
use crate::git::Marke;

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

/// Was ueber einen Eintrag von der Platte her bekannt ist: bei einem Ordner
/// ueber seinen Unterbaum, bei einer Datei ueber ihren Text.
///
/// Die sechste Eingabe des Pruefschritts aus dem Modulkopf, und die einzige,
/// die dieses Modell nicht selbst ermittelt: sie kommt von dem, der den
/// Unterbaum abschreitet oder die Datei liest, und geht ueber
/// [`Ordnermodell::befunde_setzen`] herein.
///
/// **Drei Werte und kein Auffangzweig.** `Unentschieden` ist etwas anderes als
/// `KeinTreffer`: der erste heisst "es ist noch nicht gelesen", der zweite "es
/// ist gelesen und es liegt nichts darunter". Wer die beiden zusammenzoege,
/// koennte einen laufenden Durchlauf nicht von einem abgeschlossenen ohne Fund
/// unterscheiden. Eine vierte Variante soll den Bau anhalten und nicht still in
/// einen Sammelzweig fallen.
///
/// **Eine vierte Variante fuer "zu gross" gibt es ausdruecklich nicht.** Sie
/// waere ein dritter Trefferzustand, und zu gross ist kein Treffer: die Zeile
/// steht nicht, und wie viele Dateien wegen ihrer Groesse ungelesen blieben,
/// sagt die Statuszeile und nicht die Zeile selbst. Die drei Werte tragen fuer
/// eine Datei damit genau dasselbe wie fuer einen Ordner.
///
/// **Der Wert haengt am Eintragsindex und nicht an der Zeile**, aus demselben
/// Grund wie die Markierung; er uebersteht damit jedes Umsortieren.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Befund {
    /// Noch nicht entschieden. Der Anfangswert jedes Eintrags und der Stand,
    /// den [`Ordnermodell::befund_zuruecksetzen`] herstellt.
    #[default]
    Unentschieden,
    /// Der Eintrag traegt die Folge: unter einem Ordner liegt mindestens ein
    /// Eintrag, dessen Name sie traegt, oder der Text einer Datei enthaelt sie.
    /// Der erste Fund entscheidet ihn.
    Treffer,
    /// Der Eintrag traegt die Folge nicht. Bei einem Ordner entsteht der
    /// negative Befund auf drei Wegen: abgeschritten ohne Fund, nicht zu
    /// oeffnen, oder eine symbolische Verknuepfung, in die nicht hinabgestiegen
    /// wird. Bei einer Datei heisst er, dass ihr gelesener Text die Folge nicht
    /// enthaelt, dass sie kein Text ist oder dass sie zu gross war.
    KeinTreffer,
}

/// Woran die Zeile eines Eintrags haengt: an nichts weiter, oder an einem
/// Befund von der Platte.
///
/// **Die eine Regel des Filters, ohne ihren letzten Schritt.** Der
/// Pruefschritt aus dem Modulkopf zerfaellt in zwei Teile: alles, was das
/// Modell selbst weiss, und die Frage an den Befund. Dieser Wert ist der erste
/// Teil, einmal je Eintrag und je Frage gerechnet und in `grund` aufbewahrt;
/// der zweite ist ein Blick in `befund` und kostet nichts.
///
/// **Drei Werte, vollstaendig und ohne Auffangzweig.** Sie sind die drei
/// Ausgaenge des Bildes im Modulkopf; eine vierte Lage soll den Bau anhalten.
///
/// **Dieser Wert beantwortet zwei Fragen auf einmal, und das ist der Grund,
/// warum es ihn gibt.** Wessen Zeile an einem Befund haengt, ist genau der,
/// der einen [`Auftrag`] verdient — es ist dieselbe Frage, und bis zum 260816
/// stand sie zweimal da, einmal hier und einmal in der Auftragsliste von
/// `krk-ui`. Die zweite Fassung kannte den ersten Zweig des Pruefschritts
/// nicht und erteilte deshalb Auftraege fuer ausgeblendete Eintraege
/// (`issues/260816-1931_*_der-inhaltsfilter-liest-versteckte-dateien-und-steigt-in-versteckte-ordner-ab.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Zeilengrund {
    /// Ohne Befund entschieden: die Zeile steht.
    Steht,
    /// Ohne Befund entschieden: die Zeile faellt weg.
    FaelltWeg,
    /// Die Zeile steht unter dem Vorbehalt eines Befunds, und der Auftrag
    /// fragt nach dem Genannten.
    UnterVorbehalt(Auftragsart),
}

/// Der Inhalt eines Ordners, wie ihn ein Dateifenster anzeigt.
#[derive(Debug)]
pub struct Ordnermodell {
    /// Der gelesene Bestand, geteilt statt kopiert.
    ///
    /// **Der `Arc` steht hier, damit der Durchlauf denselben Bestand ansieht
    /// und keine zweite Fassung davon bekommt.** Bis zum 260816 trug jeder
    /// [`Auftrag`] eine Kopie des Namens, den dieses Modell ohnehin schon
    /// hielt; bei 100.000 Eintraegen waren das 100.000 Zeichenketten je
    /// getipptem Zeichen, auf dem Hauptfaden
    /// (`issues/260816-1933_*_die-auftragsliste-legt-je-tastendruck-einen-namen-je-datei-an-auf-dem-hauptfaden.md`).
    /// Der Auftrag traegt seither den blossen Index, und
    /// [`Ordnermodell::bestand`] reicht den Bestand mit — ein Zaehlerschritt
    /// statt einer Kopie.
    ///
    /// **Geschrieben wird ueber [`Arc::make_mut`], und tief kopiert wird dabei
    /// nie.** Ein Durchlauf entsteht erst, wenn der Lesevorgang durch ist, und
    /// `ersatz_einloesen` setzt einen **frischen** `Arc` ein, statt den alten
    /// zu leeren; der Bestand, an dem [`Ordnermodell::anhaengen`] arbeitet,
    /// gehoert damit immer diesem Modell allein.
    eintraege: Arc<Vec<Eintrag>>,
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
    /// Der Filtertext, so wie der Nutzer ihn getippt hat.
    ///
    /// Er wohnt hier und nicht in der Ansicht, weil ein `Ordnermodell` einem
    /// Tab gehoert: damit gehoert der Filtertext dem Tab, ohne dass irgendwo
    /// ein zweiter Wert desselben Namens stuende. Er laeuft nicht ab; es gibt
    /// keinen Zeitgeber und keine Pause, nach der die Eingabe von vorn begaenne.
    filtertext: String,
    /// Der einmal je Aenderung kleingeschriebene und an `*` zerlegte
    /// Filtertext.
    ///
    /// Der Vergleich in [`Ordnermodell::sichtbar`] laeuft ueber diesen Wert und
    /// schreibt und zerlegt nicht je Zeile. Bei 100.000 Eintraegen waere das
    /// 100.000 Umschreibungen desselben kurzen Texts.
    muster: Muster,
    /// Ob der Filter auch den Unterbaum meint ("Deep").
    ///
    /// Aus heisst: der Name entscheidet jede Datei, und jeder Ordner bleibt
    /// stehen, damit die Navigation bei stehendem Filter nicht abbricht. An
    /// heisst: ein Ordner, dessen Name nicht passt, braucht einen Treffer unter
    /// sich.
    ///
    /// Die Vorbelegung steht bei [`Ordnermodell::neu`] und nirgends sonst.
    tief: bool,
    /// Ob der Filter auch den Text einer Datei meint ("Content").
    ///
    /// Aus heisst: ueber eine Datei entscheidet allein ihr Name. An heisst:
    /// eine Datei, deren Name die Folge nicht traegt, bleibt stehen, wenn ihr
    /// Text sie traegt — aber erst ab der Schwelle aus
    /// [`super::filter::inhaltsschwelle`]. Das Kennzeichen allein sagt deshalb
    /// nicht, ob der Inhaltsfilter wirkt; das sagt
    /// [`Ordnermodell::inhalt_wirkt`].
    inhalt: bool,
    /// Was von der Platte her je Eintrag bekannt ist.
    ///
    /// Parallel zu `eintraege` und nicht zur Sichtreihenfolge, in derselben
    /// Bauart und aus demselben Grund wie `markiert`. Gelesen wird der Wert in
    /// den beiden letzten Zweigen von [`Ordnermodell::sichtbar`]: fuer einen
    /// Ordner, dessen eigener Name den Filtertext nicht traegt, sagt er etwas
    /// ueber seinen Unterbaum; fuer eine gewoehnliche Datei, deren Name ihn
    /// nicht traegt, etwas ueber ihren Text. **Zwei Fragen, ein Vektor**, denn
    /// ein Eintrag ist entweder das eine oder das andere, und beide Fragen
    /// haben dieselben drei Antworten.
    befund: Vec<Befund>,
    /// Woran die Zeile je Eintrag haengt, einmal je Frage gerechnet.
    ///
    /// Parallel zu `eintraege`, in derselben Bauart wie `markiert` und
    /// `befund`. **Er ist kein zweiter Wahrheitsort, sondern das Ergebnis des
    /// einen Pruefschritts**: `zeilengrund_von` rechnet ihn,
    /// `grund_neu_rechnen` fuellt ihn, und jeder Frager liest ihn, statt die
    /// Frage noch einmal zu stellen.
    ///
    /// **Aufbewahrt wird er, weil die Frage teuer ist und selten wechselt.**
    /// Sie kostet je Eintrag eine kleingeschriebene Fassung seines Namens, bei
    /// 100.000 Eintraegen also 100.000 Zeichenketten; sie aendert sich aber nur
    /// mit dem Filtertext, mit einem der beiden Schalter oder mit dem Aus- und
    /// Einblenden der Verstecke. Ein eintreffender Befund und ein
    /// Sortierwechsel bauen die Sicht neu auf und ruehren diesen Vektor nicht
    /// an — vor dem 260816 rechnete der Neuaufbau die Namensfrage jedes Mal
    /// mit, also bis zu sechzigmal in der Sekunde, solange ein Durchlauf lief.
    grund: Vec<Zeilengrund>,
    /// Die Gitmarke je Eintrag, oder `None`, wenn keine dasteht.
    ///
    /// Parallel zu `eintraege`, in derselben Bauart wie `markiert`, `befund`
    /// und `grund`, und aus demselben Grund: die Marke haengt am Eintragsindex
    /// und ueberlebt damit jedes Umsortieren und jedes Ein- und Ausblenden der
    /// versteckten Eintraege.
    ///
    /// **`None` ist keine sechste Marke fuer "unveraendert", sondern die
    /// Aussage, dass hier nichts steht** (A11 der Runde 23): ein Eintrag ohne
    /// Befund traegt eine leere Zelle. Er traegt sie in zwei Lagen, die die
    /// Zelle nicht auseinanderhaelt und die Zelle auch nicht auseinanderhalten
    /// soll — der Eintrag ist gegenueber dem Index unveraendert, oder der
    /// Befund steht noch aus. Der Unterschied gehoert dem Kanal des Gitlaufs,
    /// nicht dieser Spalte.
    ///
    /// **Der zweite Vektor ist kein Doppelbau, sondern die zweite
    /// Ungueltigkeitsregel**; der Modulkopf schreibt sie unter
    /// `# Zwei Befundvektoren, zwei Ungueltigkeitsregeln` aus.
    gitmarke: Vec<Option<Marke>>,
    /// Ob der begonnene Lesevorgang seinen Bestand noch abloesen muss.
    ///
    /// Gesetzt von [`Ordnermodell::lesevorgang_beginnen`], eingeloest von
    /// [`Ordnermodell::ersatz_einloesen`]. Solange er aussteht, gehoert der
    /// Inhalt noch dem vorigen Lauf, die Generation aber schon dem neuen; der
    /// Modulkopf schreibt aus, warum das die richtige Reihenfolge ist.
    ersatz_ausstehend: bool,
}

impl Ordnermodell {
    /// Ein leeres Modell fuer die genannte Generation.
    ///
    /// # Die Vorgaben der beiden Suchschalter
    ///
    /// **Die tiefe Suche steht ab Werk auf "ein", der Inhaltsfilter auf
    /// "aus".** Das ist die einzige Stelle, an der beide Vorgaben stehen: die
    /// Sitzung fuehrt keinen der zwei Staende mit (`krk_core::ablage::sitzung`
    /// kennt sie nicht), und ein neuer Tab erbt nichts vom Geschwistertab,
    /// sondern kommt ueber diesen Weg. Wer die Vorgabe aendert, aendert sie
    /// hier und nirgends sonst.
    ///
    /// **Sichtbar wird die Vorgabe erst, wenn ein Filtertext steht.** Ohne ihn
    /// verlaesst `Ordnermodell::zeilengrund_von` den Pruefschritt, bevor die
    /// Frage nach der Tiefe faellt, und `Tabliste::durchlauf_nachziehen`
    /// stoesst keinen Durchlauf an. Ein frisch gestartetes KRK liest deshalb
    /// genau so viel wie zuvor.
    pub fn neu(generation: u64) -> Self {
        Self {
            eintraege: Arc::default(),
            sichtreihenfolge: Vec::new(),
            sortierung: Sortierung::default(),
            verstecke_ausblenden: true,
            generation,
            auswahl: None,
            markiert: Vec::new(),
            filtertext: String::new(),
            muster: Muster::aus(""),
            // Ab Werk eingeschaltet, siehe den Abschnitt darueber. Damit haengt
            // an dieser Zeile auch die Schwelle des Inhaltsfilters: sie fragt
            // `super::filter::inhaltsschwelle` nach dem Stand der tiefen Suche,
            // und der ist ab Werk `true`.
            tief: true,
            inhalt: false,
            befund: Vec::new(),
            grund: Vec::new(),
            gitmarke: Vec::new(),
            ersatz_ausstehend: false,
        }
    }

    /// Die Generation, zu der dieses Modell gehoert.
    ///
    /// Sie sagt, zu welchem Lesevorgang das Modell gehoert. Steht ein Ersatz
    /// aus, stammt der **Inhalt** noch aus dem vorigen Lauf; das ist die
    /// Zwischenzeit aus dem Modulkopf und dauert bis zum ersten Stapel.
    ///
    /// Die Oberflaeche prueft die Nummer **nicht** je Stapel: sie haelt immer
    /// nur einen Lesevorgang und liest allein aus dessen Kanal. Der Modulkopf
    /// von `krk-ui/src/appkit/tabelle.rs` schreibt aus, was einen Ordnerwechsel
    /// mitten im Lesen stattdessen traegt.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Wahr, wenn der Stapel zu diesem Modell gehoert.
    pub fn gehoert_dazu(&self, generation: u64) -> bool {
        generation == self.generation
    }

    /// Beginnt einen Lesevorgang: neue Generation, Ersatz vorgemerkt.
    ///
    /// **Der bisherige Inhalt bleibt stehen.** Er verschwindet erst, wenn der
    /// neue Lauf liefert: mit seinem ersten Stapel, und wenn er keinen hat, mit
    /// [`Ordnermodell::abschliessen`]. Ein Ordner, der leer ist oder sich nicht
    /// lesen laesst, raeumt die alte Liste damit ebenso zuverlaessig wie ein
    /// voller, nur eine Spur spaeter.
    ///
    /// Zweimal hintereinander gerufen — die Meldelawine des Defekts
    /// `260805-1337` — bleibt es bei einem vorgemerkten Ersatz. Genau daran
    /// haengt, dass die Liste waehrend eines Stapel-Umbenennens nicht mehr leer
    /// laeuft.
    pub fn lesevorgang_beginnen(&mut self, generation: u64) {
        self.generation = generation;
        self.ersatz_ausstehend = true;
    }

    /// Ob der naechste Stapel den angezeigten Bestand abloesen wird.
    ///
    /// Wahr, solange ein begonnener Lesevorgang noch nichts geliefert hat und
    /// noch Zeilen des vorigen stehen. Die Ansicht fragt danach, weil sie diesen
    /// einen Stapel nicht als blosse neue Zeilenzahl melden darf: die Auswahl
    /// der Tabelle zeigte sonst auf eine Zeile, die es nach dem Ersatz nicht
    /// mehr gibt.
    pub fn ersetzt_beim_naechsten_stapel(&self) -> bool {
        self.ersatz_ausstehend && !self.sichtreihenfolge.is_empty()
    }

    /// Loest einen vorgemerkten Ersatz ein: der alte Bestand faellt.
    ///
    /// Auswahl, Markierung, Filterbefund und Gitmarke fallen mit, und zwar hier
    /// und nicht schon beim Beginn des Lesevorgangs. Alle vier haengen am
    /// Eintragsindex; ein Index, der den Ersatz uebersteht, zeigte danach auf
    /// einen beliebigen Eintrag des neuen Ordners. Was die Auswahl ueber einen
    /// Lesevorgang hinweg traegt, ist der **Name** in `krk-ui`s
    /// `Tabinhalt::wunschauswahl`.
    ///
    /// **Dies ist die ganze Ungueltigkeitsregel der Gitmarke**: sie faellt mit
    /// dem Bestand, dem sie gilt, und mit nichts sonst. Der Filterbefund faellt
    /// daneben ein zweites Mal, naemlich wenn die Frage sich aendert; der
    /// Modulkopf haelt die beiden Regeln unter
    /// `# Zwei Befundvektoren, zwei Ungueltigkeitsregeln` auseinander.
    ///
    /// **Der Filtertext faellt hier nicht mit.** Er gehoert dem Tab und nicht
    /// dem Bestand; ob ein Ordnerwechsel ihn stehen laesst, entscheidet der
    /// Tab, und diese Stelle wuerde ihm die Entscheidung wegnehmen.
    fn ersatz_einloesen(&mut self) {
        if !self.ersatz_ausstehend {
            return;
        }
        self.ersatz_ausstehend = false;
        // Ein **frischer** `Arc` und kein `clear` am alten: der Faden eines
        // eben abgebrochenen Durchlaufs kann den bisherigen Bestand noch
        // halten, und `Arc::make_mut` kopierte ihn dann Eintrag fuer Eintrag,
        // nur um ihn gleich darauf zu leeren.
        self.eintraege = Arc::default();
        self.sichtreihenfolge.clear();
        self.markiert.clear();
        self.befund.clear();
        self.grund.clear();
        self.gitmarke.clear();
        self.auswahl = None;
    }

    /// Haengt einen gelesenen Stapel an.
    ///
    /// Loest zuvor einen vorgemerkten Ersatz ein: dieser Stapel ist der erste
    /// des neuen Laufs, und ab ihm gehoert der Bestand dem neuen Ordner.
    ///
    /// Die neuen Eintraege stehen zunaechst in Lesereihenfolge am Ende der
    /// Sicht. Das ist Absicht: der erste Stapel soll sofort sichtbar sein
    /// (L2), und ein vollstaendiges Sortieren je Stapel waere bei 100.000
    /// Eintraegen hundertmal dieselbe Arbeit. Die Reihenfolge steht mit
    /// [`Ordnermodell::abschliessen`].
    pub fn anhaengen(&mut self, neue: impl IntoIterator<Item = Eintrag>) {
        self.ersatz_einloesen();
        for eintrag in neue {
            let index = self.eintraege.len();
            Arc::make_mut(&mut self.eintraege).push(eintrag);
            self.markiert.push(false);
            self.befund.push(Befund::Unentschieden);
            self.gitmarke.push(None);
            // Erst anhaengen, dann fragen: `zeilengrund_von` liest den Eintrag
            // aus dem Bestand, und es soll dieselbe Frage sein, die
            // `grund_neu_rechnen` stellt, und nicht eine zweite Fassung.
            let grund = self.zeilengrund_von(index);
            self.grund.push(grund);
            if self.sichtbar(index) {
                self.sichtreihenfolge.push(index as u32);
            }
        }
    }

    /// Stellt die endgueltige Reihenfolge her.
    ///
    /// Ruft der Hauptfaden, sobald der Leser seinen Abschluss gemeldet hat,
    /// gleich ob vollstaendig, abgebrochen oder gescheitert.
    ///
    /// **Der Auffangfall des Ersatzes.** Ein leerer Ordner und ein Ordner, der
    /// sich nicht oeffnen laesst, liefern keinen einzigen Stapel; ohne diese
    /// Zeile bliebe die Liste des vorigen Ordners stehen. Der Leser meldet
    /// seinen Abschluss in jedem dieser Faelle, also gibt es keinen Ausgang, der
    /// den Ersatz schuldig bleibt.
    pub fn abschliessen(&mut self) {
        self.ersatz_einloesen();
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
    ///
    /// **Der Befundvektor bleibt stehen**, denn die Frage ist dieselbe
    /// geblieben: ob unter einem Ordner ein Treffer liegt, haengt nicht daran,
    /// ob der Nutzer Verstecke sieht. Der **Zeilengrund** aendert sich sehr
    /// wohl, und mit ihm die Auftragsliste — ein eben eingeblendeter Eintrag
    /// steht seit dem 260816 unter einem Vorbehalt, unter dem er vorher nicht
    /// stand, und braucht dafuer einen neuen Lauf. Ihn anzustossen ist Sache
    /// des Aufrufers, wie bei jeder anderen Aenderung dieser Liste auch.
    pub fn verstecke_ausblenden_setzen(&mut self, ausblenden: bool) {
        self.verstecke_ausblenden = ausblenden;
        self.grund_neu_rechnen();
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

    // ------------------------------------------------------------------
    // Der Filter: ein Pruefschritt, zwei Frager
    // ------------------------------------------------------------------

    /// Woran die Zeile dieses Eintrags haengt.
    ///
    /// **Der eine Pruefschritt.** Er traegt das Bild aus dem Modulkopf Zweig
    /// fuer Zweig und ist die einzige Stelle, an der ueber die Sichtbarkeit
    /// entschieden wird. Was er **nicht** tut, ist den Befund zu lesen: das ist
    /// der letzte Schritt, er kostet nichts, und er steht in
    /// [`Ordnermodell::sichtbar`]. Genau an dieser Naht haengt, dass sich die
    /// Frage aufbewahren laesst, ohne die Antwort mit aufzubewahren.
    ///
    /// Ein Index ausserhalb des Bestands faellt weg. Das ist kein Zweig des
    /// Bildes, sondern die Antwort auf eine Frage nach einem Eintrag, den es
    /// nicht gibt.
    fn zeilengrund_von(&self, index: usize) -> Zeilengrund {
        let Some(eintrag) = self.eintraege.get(index) else {
            return Zeilengrund::FaelltWeg;
        };

        // versteckt und Verstecke ausgeblendet?
        if self.verstecke_ausblenden && eintrag.versteckt {
            return Zeilengrund::FaelltWeg;
        }

        // steht ein Filtertext?
        if self.filtertext.is_empty() {
            return Zeilengrund::Steht;
        }

        // Name traegt die Teilzeichenfolge? Der eine Vergleich des Filters, und
        // derselbe, den der Durchlauf auf jeden Namen im Unterbaum zieht:
        // kleingeschrieben und als Teilzeichenfolge, ohne jede Faltung von
        // Umlauten und Akzenten. `apfel` findet `Aepfel` mit Umlaut nicht, und
        // das ist so gewollt.
        if self.name_traegt_den_filter(index as u32) {
            return Zeilengrund::Steht;
        }

        // ist es ein Ordner? Eine symbolische Verknuepfung zaehlt hier mit: bei
        // ausgeschaltetem Filter der Tiefe bleibt sie sichtbar wie jeder Ordner,
        // bei eingeschaltetem entscheidet ihr Befund, und den meldet der
        // Durchlauf als "kein Treffer darunter", weil er nicht in sie
        // hinabsteigt. Das ist der eine Schnitt fuer "Ordner"; die
        // Verknuepfungsregel selbst wohnt allein im Durchlauf.
        //
        // Eine gewoehnliche Datei faellt hier nicht mehr weg, sondern geht in
        // den Inhaltszweig. Er steht hinter dem Kurzschluss des Namens, und
        // genau daran haengt, dass die beiden Treffergruende sich nicht
        // ueberschneiden.
        if !(eintrag.ist_ordner() || eintrag.ist_verknuepfung()) {
            return if self.inhalt_wirkt() {
                Zeilengrund::UnterVorbehalt(Auftragsart::Inhalt)
            } else {
                Zeilengrund::FaelltWeg
            };
        }

        // ist der Filter der Tiefe eingeschaltet?
        if !self.tief {
            return Zeilengrund::Steht;
        }

        // liegt unter ihm ein Treffer? Das entscheidet der Befund, und ihn
        // liest `sichtbar`.
        Zeilengrund::UnterVorbehalt(Auftragsart::Unterbaum)
    }

    /// Woran die Zeile dieses Eintrags haengt, aus dem aufbewahrten Vektor.
    ///
    /// Ein Index ausserhalb des Bestands faellt weg, aus demselben Grund wie in
    /// [`Ordnermodell::zeilengrund_von`].
    fn grund(&self, index: usize) -> Zeilengrund {
        self.grund
            .get(index)
            .copied()
            .unwrap_or(Zeilengrund::FaelltWeg)
    }

    /// Rechnet den Zeilengrund jedes Eintrags neu.
    ///
    /// **Zu rufen, wann immer sich eine Eingabe des Pruefschritts aendert**,
    /// und das sind genau drei Anlaesse: der Filtertext, einer der beiden
    /// Schalter, und das Aus- und Einblenden der versteckten Eintraege. Ein
    /// eintreffender Befund gehoert ausdruecklich **nicht** dazu — er ist die
    /// Antwort und nicht die Frage —, und ein Sortierwechsel ebenso wenig.
    fn grund_neu_rechnen(&mut self) {
        // Herausgenommen und zurueckgegeben, damit `zeilengrund_von` sich
        // `self` ausleihen kann und die Zuteilung trotzdem stehen bleibt.
        let mut grund = std::mem::take(&mut self.grund);
        grund.clear();
        grund.extend((0..self.eintraege.len()).map(|index| self.zeilengrund_von(index)));
        self.grund = grund;
    }

    /// Ob der Eintrag mit diesem Index in der Liste steht.
    ///
    /// **Der letzte Schritt des Pruefschritts**, und der einzige, der den
    /// Befund liest. Seine beiden Frager sind [`Ordnermodell::anhaengen`] und
    /// [`Ordnermodell::sicht_neu_aufbauen`].
    ///
    /// Beide Fallunterscheidungen sind vollstaendig und ohne Auffangzweig:
    /// `Unentschieden` heisst, dass der Durchlauf diesen Eintrag noch nicht
    /// erreicht hat, und bis dahin steht seine Zeile nicht.
    fn sichtbar(&self, index: usize) -> bool {
        match self.grund(index) {
            Zeilengrund::Steht => true,
            Zeilengrund::FaelltWeg => false,
            Zeilengrund::UnterVorbehalt(_) => match self.befund(index as u32) {
                Befund::Treffer => true,
                Befund::Unentschieden | Befund::KeinTreffer => false,
            },
        }
    }

    /// Ob der Name dieses Eintrags den stehenden Filtertext traegt.
    ///
    /// **Der Zweig `Name traegt die Folge?` des Pruefschritts, herausgegeben.**
    /// Sein einziger Rufer in diesem Baum ist [`Ordnermodell::zeilengrund_von`]
    /// — einmal je Eintrag und je Frage, und nicht einmal je Frager. Bis zum
    /// 260816 rief ihn daneben die Auftragsliste in `krk-ui`, und der Neuaufbau
    /// der Sicht rief ihn ein drittes Mal, sooft ein Befund eintraf.
    ///
    /// Herausgegeben ist er trotzdem, denn die Probe zu C6.9 misst ihn gegen
    /// den Inhaltsbefund: dieselbe Folge soll am Namen und am Inhalt dieselbe
    /// Antwort geben. Der Vergleich selbst, [`traegt_die_folge`], hat weiterhin
    /// keinen Rufer in `krk-ui`; seine drei Rufer stehen alle im Kern, diese
    /// Datei, der Durchlauf und seit der Runde 11 [`super::inhalt`] fuer den
    /// Text einer Datei.
    ///
    /// Ein Index ausserhalb des Bestands traegt nichts. **Ohne Filtertext ist
    /// die Frage gegenstandslos**: der Rufer stellt sie erst hinter dem Zweig
    /// „steht ein Filtertext?", und der leere Text steckt der Sache nach in
    /// jedem Namen.
    pub fn name_traegt_den_filter(&self, eintragsindex: u32) -> bool {
        self.eintraege
            .get(eintragsindex as usize)
            .is_some_and(|eintrag| traegt_die_folge(&eintrag.name, &self.muster))
    }

    /// Ob dieser Eintrag allein wegen seines Inhalts in der Liste steht.
    ///
    /// **Die Frage der Dateizelle**, die eine so stehende Zeile abgesetzt
    /// schreibt. Sie stellt keine zweite Frage neben dem Pruefschritt, sondern
    /// liest dessen aufbewahrtes Ergebnis: unter dem Vorbehalt eines
    /// Inhaltsbefunds zu stehen und ihn als Treffer beantwortet zu haben, ist
    /// genau das, was der Dateizweig von [`Ordnermodell::zeilengrund_von`]
    /// bedeutet.
    ///
    /// **Damit sind die beiden Treffergruende ueberschneidungsfrei**: keine
    /// Zeile steht aus beiden zugleich. Das leistet der Kurzschluss des Namens
    /// im Pruefschritt, denn wessen Name die Folge traegt, kommt gar nicht erst
    /// unter einen Vorbehalt.
    ///
    /// **Bis zum 260816 stand hier eine zweite Fassung der Vorbedingungen**,
    /// fuenf Zweige lang, und einer davon rief je gezeichneter Zelle
    /// [`Ordnermodell::name_traegt_den_filter`] und schrieb den Namen dafuer
    /// klein. Beides ist mit dem aufbewahrten Zeilengrund weg.
    #[must_use]
    pub fn steht_wegen_des_inhalts(&self, eintragsindex: u32) -> bool {
        self.grund(eintragsindex as usize) == Zeilengrund::UnterVorbehalt(Auftragsart::Inhalt)
            && matches!(self.befund(eintragsindex), Befund::Treffer)
    }

    /// Die Auftraege, die dieser Stand des Filters offen laesst.
    ///
    /// **Wessen Zeile an einem Befund haengt, verdient einen Auftrag, und das
    /// ist keine zweite Regel, sondern dieselbe.** Die Liste ist deshalb ein
    /// Gang ueber den aufbewahrten Zeilengrund und stellt keine einzige Frage
    /// neu. Wer den Bestand zum Beantworten braucht, bekommt ihn ueber
    /// [`Ordnermodell::bestand`]; der Auftrag selbst traegt nur den Index.
    ///
    /// Die Reihenfolge ist die des Bestands, und keine Zusage haengt an ihr.
    ///
    /// **Bis zum 260816 stand diese Liste in `krk-ui` und war die zweite
    /// Fassung des Pruefschritts.** Sie kannte dessen ersten Zweig nicht und
    /// erteilte Auftraege fuer ausgeblendete Eintraege, die keine Zeile
    /// bekommen koennen; seit der Runde 11 kostet ein solcher Auftrag ein
    /// `open(2)` und bis zu 1 MB gelesene Bytes
    /// (`issues/260816-1931_*_der-inhaltsfilter-liest-versteckte-dateien-und-steigt-in-versteckte-ordner-ab.md`).
    ///
    /// **Der Abstieg in einen versteckten Ordner ist davon unberuehrt**: ein
    /// Treffer unter ihm ist ein Treffer unter dem sichtbaren Ordner darueber,
    /// und ihn zu uebergehen waere eine neue Regel und keine Ersparnis. Die
    /// Verstecke sind hier eine Frage an die **Zeile** und keine an den
    /// Unterbaum.
    ///
    /// **Ein Eintrag, den das Einblenden der Verstecke sichtbar macht, braucht
    /// deshalb einen neuen Lauf.** Der Aufrufer stoesst ihn an, wie er ihn nach
    /// jeder anderen Aenderung dieser Liste auch anstoesst.
    #[must_use]
    pub fn auftraege(&self) -> Vec<Auftrag> {
        self.grund
            .iter()
            .enumerate()
            .filter_map(|(index, grund)| match grund {
                Zeilengrund::UnterVorbehalt(art) => Some(Auftrag {
                    index: index as u32,
                    art: *art,
                }),
                Zeilengrund::Steht | Zeilengrund::FaelltWeg => None,
            })
            .collect()
    }

    /// Der gelesene Bestand, zum Mitgeben an einen Durchlauf.
    ///
    /// Ein Zaehlerschritt und keine Kopie; was daran haengt, steht am Feld
    /// `eintraege`.
    #[must_use]
    pub fn bestand(&self) -> Arc<Vec<Eintrag>> {
        Arc::clone(&self.eintraege)
    }

    /// Der Filtertext, so wie der Nutzer ihn getippt hat.
    pub fn filtertext(&self) -> &str {
        &self.filtertext
    }

    /// Das Muster aus dem Filtertext, wie der Vergleich es braucht.
    ///
    /// Wer den Unterbaum abschreitet, vergleicht mit demselben Wert wie
    /// [`Ordnermodell::sichtbar`] und zerlegt ihn nicht ein zweites Mal.
    pub fn muster(&self) -> &Muster {
        &self.muster
    }

    /// Ob ein Filtertext steht.
    pub fn filter_steht(&self) -> bool {
        !self.filtertext.is_empty()
    }

    /// Setzt den Filtertext und baut die Sicht neu auf.
    pub fn filtertext_setzen(&mut self, text: &str) {
        self.filtertext.clear();
        self.filtertext.push_str(text);
        self.filter_uebernehmen();
    }

    /// Haengt ein getipptes Zeichen an den Filtertext an.
    ///
    /// **Welche Zeichen der Filter aufnimmt, entscheidet der Aufrufer** ueber
    /// [`super::filter::traegt_ein_dateiname`]. Diese Stelle
    /// nimmt jedes Zeichen: sie hat keinen Rueckgabewert, und ein still
    /// verworfenes Zeichen waere ein Ausgang, den niemand sieht. Der Aufrufer
    /// muss die Frage ohnehin stellen, denn er meldet der Ereignisbehandlung,
    /// ob KRK den Tastendruck verbraucht hat.
    pub fn zeichen_anhaengen(&mut self, zeichen: char) {
        self.filtertext.push(zeichen);
        self.filter_uebernehmen();
    }

    /// Haengt einen ganzen Text an den Filtertext an, in einem Zug.
    ///
    /// Der Weg des Einfuegens aus der Zwischenablage (Runde 21). Derselbe
    /// Vertrag wie bei [`Ordnermodell::zeichen_anhaengen`]: welche Zeichen
    /// hineinduerfen, hat der Rufer entschieden, hier die Reinigung
    /// [`crate::zwischenablage::filtertext_aus`], und diese Stelle nimmt
    /// jedes. **Eine Schleife ueber `zeichen_anhaengen` ist bewusst nicht der
    /// Weg**: sie riefe `filter_uebernehmen` je Zeichen und baute die Sicht je
    /// Zeichen ueber den ganzen Bestand neu auf; bei 100.000 Eintraegen und
    /// zwoelf Zeichen sind das elf Gaenge zu viel. Hier wird einmal
    /// angehaengt und einmal uebernommen, und fuer die Sicht ist das Einfuegen
    /// damit ein einzelner Anschlag mit vielen Zeichen.
    pub fn text_anhaengen(&mut self, text: &str) {
        self.filtertext.push_str(text);
        self.filter_uebernehmen();
    }

    /// Nimmt das letzte Zeichen des Filtertexts zurueck.
    ///
    /// Liefert, ob etwas wegzunehmen war; bei leerem Filtertext geschieht
    /// nichts. **Dieselbe Form wie `Suchlage::letztes_zeichen_weg` in
    /// `krk-ui`**, und aus demselben Grund mit `#[must_use]`: am Wert haengt,
    /// ob der Tastendruck verbraucht ist oder weiterzureichen, und sein stiller
    /// Verlust bliebe unbemerkt.
    #[must_use]
    pub fn letztes_zeichen_weg(&mut self) -> bool {
        if self.filtertext.pop().is_none() {
            return false;
        }
        self.filter_uebernehmen();
        true
    }

    /// Loescht den Filtertext und baut die Sicht neu auf.
    pub fn filter_leeren(&mut self) {
        self.filtertext.clear();
        self.filter_uebernehmen();
    }

    /// Ob der Filter auch den Unterbaum meint ("Deep").
    pub fn tief(&self) -> bool {
        self.tief
    }

    /// Schaltet den Filter der Tiefe ein oder aus.
    ///
    /// Ob dabei ein Befund verfaellt, entscheidet
    /// [`Ordnermodell::schalter_setzen`] und nicht diese Stelle. **Der Stand
    /// der tiefen Suche gehoert nicht zu der Frage, die ein Befund
    /// beantwortet**, sondern nur dazu, ob sie ueberhaupt gestellt wird: der
    /// Durchlauf schreitet einen Unterbaum immer gleich ab, gleich wie der
    /// Schalter steht. Er aendert die Frage trotzdem, wenn er die Schwelle des
    /// Inhaltsfilters ueber- oder unterschreitet — und genau das misst
    /// `schalter_setzen`, statt es hier nachzurechnen.
    pub fn tief_setzen(&mut self, tief: bool) {
        self.schalter_setzen(|modell| modell.tief = tief);
    }

    /// Ob der Filter auch den Text einer Datei meint ("Content").
    ///
    /// Das blosse Kennzeichen. Ob der Inhaltsfilter **wirkt**, sagt
    /// [`Ordnermodell::inhalt_wirkt`]; unterhalb der Schwelle steht das
    /// Kennzeichen und tut nichts.
    #[must_use]
    pub fn inhalt(&self) -> bool {
        self.inhalt
    }

    /// Schaltet den Filter des Inhalts ein oder aus.
    ///
    /// **Dieselbe Form wie [`Ordnermodell::tief_setzen`]**, Zeile fuer Zeile,
    /// und aus demselben Grund: was verfaellt, entscheidet
    /// [`Ordnermodell::schalter_setzen`] an einer Stelle fuer beide Schalter.
    pub fn inhalt_setzen(&mut self, inhalt: bool) {
        self.schalter_setzen(|modell| modell.inhalt = inhalt);
    }

    /// Legt einen der beiden Schalter um und traegt die Folgen nach.
    ///
    /// **Ein Befund ist die Antwort auf eine Frage, und diese Stelle huetet die
    /// Frage.** Sie lautet: traegt dieser Eintrag den Filtertext unter sich
    /// oder in sich, und zaehlt sein Inhalt dabei mit? Beides sind Angaben, mit
    /// denen der Durchlauf startet — der kleingeschriebene Filtertext und die
    /// Inhaltsgrenze —, und aendert sich eine von beiden, sind die aufbewahrten
    /// Antworten Auskuenfte ueber eine frueher gestellte Frage. Der Filtertext
    /// geht seinen eigenen Weg ueber `filter_uebernehmen`, das ohnehin jedes
    /// Mal zuruecksetzt; hier bleibt die zweite Haelfte.
    ///
    /// **Bis zum 260816 stand die Regel zweimal da und war beide Male
    /// unsymmetrisch**: eingeschaltet wurde zurueckgesetzt, ausgeschaltet
    /// nicht, „weil ihn dann niemand liest". Fuer eine Datei stimmte das, fuer
    /// einen Ordner nie: dessen Zweig liest den Befund, solange die tiefe Suche
    /// steht. Eine Ordnerzeile blieb nach dem Ausschalten von „Content" auf
    /// einem Befund stehen, den erst das Lesen von Dateien erzeugt hatte, und
    /// zwar so lange, wie der neue Lauf bis zu ihr brauchte — nach dem eigenen
    /// Text der Runde bis zu Minuten
    /// (`issues/260816-1930_*_content-ausschalten-laesst-ordnerzeilen-auf-einem-veralteten-inhaltsbefund-stehen.md`).
    /// C2.9 verlangt „sofort", und sofort heisst hier: der Befund faellt mit dem
    /// Schalter.
    ///
    /// **Der Preis ist benannt und angenommen.** Ein Ordner, der wegen eines
    /// **Namens** unter sich stand, verschwindet beim Umlegen ebenfalls und
    /// kommt mit dem neuen Lauf wieder. Der Vektor sagt, **dass** etwas
    /// darunter lag, und nicht **warum**; ihn nach dem Grund zu fragen hiesse,
    /// den Grund ueber den Befundkanal zu melden und aus einem Wahrheitswert je
    /// Auftrag zwei zu machen. Es ist derselbe Handel, den das Einschalten der
    /// tiefen Suche seit der Runde 10 schon eingeht.
    fn schalter_setzen(&mut self, umlegen: impl FnOnce(&mut Self)) {
        let inhalt_zaehlte = self.inhalt_wirkt();
        umlegen(self);
        if self.inhalt_wirkt() != inhalt_zaehlte {
            self.befund_zuruecksetzen();
        }
        self.grund_neu_rechnen();
        self.sicht_neu_aufbauen();
    }

    /// Ob der Inhaltsfilter bei diesem Stand wirkt: „Content" steht **und** der
    /// Filtertext ist lang genug.
    ///
    /// **Die eine Stelle, an der die Schwelle geprueft wird.** Ihre Frager
    /// stellen alle dieselbe Frage und rechnen sie nicht nach: der Dateizweig
    /// des Pruefschritts (steht diese Zeile?), die Auftragsliste des Tabs
    /// (bekommt diese Datei einen Auftrag?), die Entscheidung, ob ueberhaupt
    /// ein Durchlauf laeuft, und die Statuszeile (ist der Lesehinweis
    /// faellig?). Ein zweiter Rechenweg an einer dieser Stellen waere die
    /// Gelegenheit, verschieden zu antworten.
    ///
    /// Die Schwelle selbst wohnt in [`super::filter::inhaltsschwelle`] und
    /// haengt am Stand der tiefen Suche. **Gezaehlt werden Zeichen und keine
    /// Bytes**: ein getipptes `äöü` sind drei Zeichen und sechs Bytes. **Das
    /// `*` zaehlt seit der Runde 21 nicht mit**: der Platzhalter sagt nichts
    /// ueber den Gegenstand aus, `ab*cd` sind vier Zeichen, `*****` sind null,
    /// und ein Filtertext aus lauter `*` liest nie eine Datei. Gezaehlt wird
    /// der Filtertext und nicht das Muster, weil die Kleinschreibung die
    /// Zeichenzahl aendern kann (`İ` wird zu zwei Zeichen) und die Schwelle von
    /// getippten Zeichen spricht; die Zaehlung steht hier und nur hier.
    ///
    /// Gefragt wird bei jeder Bewertung neu. Wer bei vier Zeichen ohne tiefe
    /// Suche Inhaltstreffer vor sich hat und die tiefe Suche einschaltet,
    /// verliert sie an der gestiegenen Schwelle; ein fuenftes Zeichen holt sie
    /// zurueck.
    #[must_use]
    pub fn inhalt_wirkt(&self) -> bool {
        self.inhalt
            && self
                .filtertext
                .chars()
                .filter(|zeichen| *zeichen != '*')
                .count()
                >= filter::inhaltsschwelle(self.tief)
    }

    /// Was ueber den Unterbaum dieses Eintrags bekannt ist.
    ///
    /// `Unentschieden` fuer jeden Index ausserhalb des Bestands: ueber einen
    /// Eintrag, den es nicht gibt, ist nichts bekannt.
    pub fn befund(&self, eintragsindex: u32) -> Befund {
        self.befund
            .get(eintragsindex as usize)
            .copied()
            .unwrap_or_default()
    }

    /// Traegt eine Reihe von Befunden ein und baut die Sicht **einmal** neu
    /// auf.
    ///
    /// Der Weg, auf dem das Ergebnis des Durchlaufs hereinkommt. Ein Index
    /// ausserhalb des Bestands wird verworfen, ohne die Sicht anzufassen: er
    /// stammt dann aus einem Lauf, dessen Bestand schon abgeloest ist. Kommt
    /// kein einziger brauchbarer Index herein, bleibt die Sicht unangetastet.
    ///
    /// **Eine Reihe und nicht ein einzelner Befund, und der Grund ist gezaehlt
    /// und nicht vermutet.** Der Neuaufbau ist ein Gang ueber alle Eintraege
    /// samt `sort_unstable_by`; ein Setzer je Ordner hiesse bei einem
    /// angezeigten Ordner mit `k` Unterordnern und `n` Eintraegen `k` Laeufe zu
    /// je `O(n log n)` auf dem **Hauptfaden**. Der Einzugstakt raeumt den
    /// Befundkanal ohnehin in einem Zug leer und hat die Reihe damit schon in
    /// der Hand; er reicht sie hier in einem Stueck herein und baut einmal auf
    /// (`issues/260814-2145_*_befund-setzen-baut-die-ganze-sicht-neu-auf-und-der-durchlauf-ruft-es-je-ordner.md`).
    pub fn befunde_setzen(&mut self, befunde: impl IntoIterator<Item = (u32, Befund)>) {
        let mut eingetragen = false;
        for (eintragsindex, befund) in befunde {
            if let Some(stelle) = self.befund.get_mut(eintragsindex as usize) {
                *stelle = befund;
                eingetragen = true;
            }
        }
        if eingetragen {
            self.sicht_neu_aufbauen();
        }
    }

    /// Die Gitmarke des genannten Eintrags, oder `None`.
    ///
    /// `None` fuer jeden Index ausserhalb des Bestands, wie es
    /// [`Ordnermodell::befund`] fuer einen solchen Index `Unentschieden`
    /// liefert: ueber einen Eintrag, den es nicht gibt, ist nichts bekannt.
    #[must_use]
    pub fn gitmarke(&self, eintragsindex: u32) -> Option<Marke> {
        self.gitmarke.get(eintragsindex as usize).copied().flatten()
    }

    /// Traegt die Marken eines Gitlaufs ein, ueber den **Namen** zugeordnet.
    ///
    /// Liefert, ob ueberhaupt etwas eingetragen wurde. Der Weg, auf dem der
    /// Befund des Gitlaufs hereinkommt; ein Name, den der Bestand nicht fuehrt,
    /// wird verworfen, ohne die uebrigen zu verhindern.
    ///
    /// # Zwei Unterschiede zu [`Ordnermodell::befunde_setzen`]
    ///
    /// Die beiden Setzer stehen nebeneinander und sehen gleich aus; sie
    /// unterscheiden sich in zwei Punkten, und beide sind tragend.
    ///
    /// **Erstens: die Zuordnung laeuft ueber den Namen und nicht ueber den
    /// Eintragsindex, und deshalb wird die Generation gegengehalten.**
    /// [`Ordnermodell::lesevorgang_beginnen`] leert den Bestand nicht vorab,
    /// sondern merkt den Ersatz vor; in dieser Spanne steht noch der **alte**
    /// Ordner da. Ein Filterbefund kommt darueber von selbst hinweg, weil sein
    /// Eintragsindex am Bestandsende durchfaellt, sobald der neue Ordner
    /// kuerzer ist — und wo er nicht durchfaellt, faellt er mit dem Ersatz.
    /// Ein Name hat diesen Schutz nicht: derselbe Name kann im neuen Ordner
    /// ebenso stehen. Die Pruefung auf `generation` und `ersatz_ausstehend` ist
    /// deshalb keine Doppelung der Kanalzusage, sondern der Ersatz fuer einen
    /// Schutz, den es hier nicht gibt (C7.4 und C7.5 der Runde 23).
    ///
    /// **Zweitens: die Sicht wird nicht neu aufgebaut.** Eine Marke
    /// entscheidet nicht, **ob** eine Zeile steht, sondern nur, **was** in
    /// einer ihrer Zellen steht; `sichtbar` fragt sie nicht. `sicht_neu_aufbauen`
    /// liefe ueber alle Eintraege samt `sort_unstable_by` und ordnete die Liste
    /// fuer nichts — bei 100.000 Eintraegen auf dem Hauptfaden. Die Ansicht
    /// antwortet stattdessen mit `reloadData` und **ohne** `auswahl_anzeigen`:
    /// die Sichtreihenfolge bleibt, und die ausgewaehlte Zeile behaelt ihre
    /// Stelle.
    ///
    /// # Einmal ein Nachschlagewerk und nicht einmal je Name
    ///
    /// Die `HashMap` ueber den Bestand entsteht einmal je Aufruf. Der Gitlauf
    /// liefert seine Marken deshalb in **einem** Stueck und nicht Eintrag fuer
    /// Eintrag; der Modulkopf von [`crate::git::lauf`] schreibt den zweiten
    /// Grund dazu.
    ///
    /// # Zwei Nachschlagewerke, und das zweite entsteht nur, wenn es gebraucht wird
    ///
    /// **Die beiden Seiten stammen aus verschiedenen Quellen**, und deshalb
    /// genuegt der Bytevergleich nicht: der Bestand kommt unveraendert aus
    /// `readdir`, der Befund kommt aus `gix`, das `core.precomposeUnicode`
    /// anwendet und **vorkomponierte** Namen liefert. Eine Datei, die auf der
    /// Platte zerlegt benannt ist — nach einem Entpacken, nach einer
    /// Uebertragung von einem aelteren Dateisystem —, traegt damit auf den zwei
    /// Seiten verschiedene Bytes und bekaeme keine Marke.
    ///
    /// Gefragt wird deshalb zweimal: erst bytegenau, und erst bei einem
    /// Fehlschlag ueber [`kollation::namensschluessel`], unter dem zwei
    /// kanonisch gleiche Namen derselbe Name sind.
    ///
    /// **Die Reihenfolge ist tragend und keine Sparmassnahme.** Ein Ordner
    /// kann beide Schreibweisen desselben Namens zugleich tragen; APFS haelt
    /// sie auseinander. Der bytegenaue Treffer geht deshalb vor, und das
    /// zweite Werk entscheidet allein die Faelle, die er nicht trifft. Wo es
    /// zwei kanonisch gleiche Eintraege gibt, gewinnt darin der erste des
    /// Bestands.
    ///
    /// **Und es entsteht erst beim ersten Fehlschlag**: ein Ordner, dessen
    /// Namen alle bytegenau treffen, zahlt keinen einzigen Kollationsschluessel.
    #[must_use = "die Antwort sagt, ob der Befund noch zu diesem Bestand gehoert hat"]
    pub fn gitmarken_setzen(&mut self, generation: u64, marken: &[(String, Marke)]) -> bool {
        if generation != self.generation || self.ersatz_ausstehend {
            return false;
        }
        let stellen: HashMap<&str, usize> = self
            .eintraege
            .iter()
            .enumerate()
            .map(|(index, eintrag)| (eintrag.name.as_str(), index))
            .collect();
        let eintraege = &self.eintraege;
        let mut kanonische: Option<HashMap<Box<[u8]>, usize>> = None;
        let mut eingetragen = false;
        for (name, marke) in marken {
            let index = match stellen.get(name.as_str()) {
                Some(index) => *index,
                None => {
                    let werk = kanonische.get_or_insert_with(|| {
                        let mut werk: HashMap<Box<[u8]>, usize> =
                            HashMap::with_capacity(eintraege.len());
                        for (index, eintrag) in eintraege.iter().enumerate() {
                            werk.entry(kollation::namensschluessel(&eintrag.name))
                                .or_insert(index);
                        }
                        werk
                    });
                    let Some(index) = werk.get(kollation::namensschluessel(name).as_ref()) else {
                        continue;
                    };
                    *index
                }
            };
            if let Some(stelle) = self.gitmarke.get_mut(index) {
                *stelle = Some(*marke);
                eingetragen = true;
            }
        }
        eingetragen
    }

    /// Setzt jeden Befund auf `Unentschieden` zurueck.
    ///
    /// Zu rufen, wann immer die Frage eine andere wird, und **nur** dann. Die
    /// Frage ist der kleingeschriebene Filtertext und die Angabe, ob der Inhalt
    /// dabei zaehlt; wer sie aendert, sind `filter_uebernehmen` und
    /// [`Ordnermodell::schalter_setzen`], dazu der Abbruch eines Durchlaufs von
    /// aussen. Die Sicht baut diese Methode **nicht** neu auf; ihre Rufer tun es
    /// unmittelbar danach, und zweimal zu bauen waere zweimal dieselbe Arbeit.
    ///
    /// **`gitmarke` wird hier nicht angefasst** (C7.6 der Runde 23). Die Marke
    /// ist keine Antwort auf den Filtertext, sondern eine Auskunft ueber die
    /// Datei auf der Platte; sie beim Tippen wegzuwerfen hiesse, einen
    /// Statuslauf zu entwerten, der Sekunden gekostet hat und mit der
    /// geaenderten Frage nichts zu tun hat. Ihr Anlass ist `ersatz_einloesen`.
    pub fn befund_zuruecksetzen(&mut self) {
        self.befund.fill(Befund::Unentschieden);
    }

    /// Zieht die abgeleiteten Groessen des Filters nach und baut die Sicht neu
    /// auf.
    ///
    /// Die eine Stelle, an der das Muster entsteht — einmal je Aenderung
    /// des Filtertexts und nicht einmal je Zeile — und an der die Befunde
    /// zurueckfallen, weil sie Auskuenfte ueber den vorigen Filtertext waeren.
    fn filter_uebernehmen(&mut self) {
        self.muster = Muster::aus(&self.filtertext);
        self.befund_zuruecksetzen();
        self.grund_neu_rechnen();
        self.sicht_neu_aufbauen();
    }

    /// Alle sichtbaren Eintraege in Sichtreihenfolge.
    pub fn zeilen(&self) -> impl Iterator<Item = &Eintrag> {
        self.sichtreihenfolge
            .iter()
            .filter_map(|index| self.eintraege.get(*index as usize))
    }

    /// Filtert und sortiert die Sicht von Grund auf neu.
    ///
    /// Gefiltert wird ueber [`Ordnermodell::sichtbar`], denselben Pruefschritt,
    /// den [`Ordnermodell::anhaengen`] je neuem Eintrag stellt.
    ///
    /// **Sortiert wird nach dem Filtern und nicht danach, wie gut ein Name
    /// passt.** Der Sortierschluessel entsteht einmal beim Lesen und traegt die
    /// Kollation als Bytefolge; der Filter ist ein Pruefschritt davor und kein
    /// Vergleich. Nur deshalb bleibt das Sortieren, was es war.
    fn sicht_neu_aufbauen(&mut self) {
        // Die Liste wird herausgenommen und zurueckgegeben, damit `sichtbar`
        // sich `self` ausleihen kann und die Zuteilung trotzdem stehen bleibt.
        let mut sicht = std::mem::take(&mut self.sichtreihenfolge);
        sicht.clear();
        sicht.extend(
            (0..self.eintraege.len())
                .filter(|index| self.sichtbar(*index))
                .map(|index| index as u32),
        );
        let sortierung = self.sortierung;
        let eintraege: &[Eintrag] = &self.eintraege;
        sicht.sort_unstable_by(|links, rechts| {
            sortierung.vergleiche(&eintraege[*links as usize], &eintraege[*rechts as usize])
        });
        self.sichtreihenfolge = sicht;
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::super::eintrag::Typ;
    use super::*;

    /// Ein Eintrag fuer die Proben unten.
    ///
    /// Die Sortierschluessel entstehen dabei so, wie sie es beim Lesen tun.
    fn eintrag(name: &str, typ: Typ) -> Eintrag {
        Eintrag::neu(name.to_owned(), 0, SystemTime::UNIX_EPOCH, typ)
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

    /// Der Defekt vom 260805-1337, an der Stelle, an der er entsteht: der
    /// zweite Lesevorgang darf die Liste nicht leeren, bevor er liefert.
    #[test]
    fn ein_zweiter_lesevorgang_laesst_die_alte_liste_stehen() {
        let mut modell = gelesen();
        modell.abschliessen();
        let vorher: Vec<String> = modell.zeilen().map(|e| e.name.clone()).collect();

        modell.lesevorgang_beginnen(2);

        assert_eq!(modell.zeilenzahl(), vorher.len(), "die Liste ist leer");
        let jetzt: Vec<String> = modell.zeilen().map(|e| e.name.clone()).collect();
        assert_eq!(jetzt, vorher, "es steht etwas anderes da als vorher");
        assert!(
            modell.gehoert_dazu(2),
            "die Generation gehoert dem neuen Lauf"
        );
    }

    /// Die Meldelawine: mehrere Lesevorgaenge kurz hintereinander, keiner
    /// liefert. Genau hier lief die Liste bis zum 260807 leer.
    #[test]
    fn auch_der_fuenfte_neu_aufgesetzte_lesevorgang_leert_nicht() {
        let mut modell = gelesen();
        modell.abschliessen();

        for generation in 2..=6 {
            modell.lesevorgang_beginnen(generation);
            assert_eq!(
                modell.zeilenzahl(),
                2,
                "der Lesevorgang {generation} hat die Liste geleert"
            );
        }
    }

    #[test]
    fn der_erste_stapel_loest_den_alten_bestand_ab() {
        let mut modell = gelesen();
        modell.abschliessen();

        modell.lesevorgang_beginnen(2);
        modell.anhaengen([eintrag("neu.txt", Typ::Datei)]);

        assert_eq!(modell.zeilenzahl(), 1);
        assert_eq!(name_in_zeile(&modell, 0), Some("neu.txt"));
        assert_eq!(
            modell.eintraege().len(),
            1,
            "der alte Bestand steht noch da"
        );
    }

    /// Der zweite Stapel desselben Laufs haengt an, statt ein zweites Mal zu
    /// ersetzen.
    #[test]
    fn der_zweite_stapel_ersetzt_nicht_noch_einmal() {
        let mut modell = gelesen();
        modell.abschliessen();

        modell.lesevorgang_beginnen(2);
        modell.anhaengen([eintrag("eins.txt", Typ::Datei)]);
        modell.anhaengen([eintrag("zwei.txt", Typ::Datei)]);

        assert_eq!(modell.zeilenzahl(), 2);
    }

    /// Der Ordner, der nie einen Stapel liefert: leer oder nicht lesbar. Ohne
    /// diesen Auffangfall bliebe die alte Liste fuer immer stehen.
    #[test]
    fn ein_ordner_ohne_stapel_raeumt_die_alte_liste_beim_abschluss() {
        let mut modell = gelesen();
        modell.abschliessen();
        auswaehlen(&mut modell, "zzz.txt");

        modell.lesevorgang_beginnen(2);
        modell.abschliessen();

        assert_eq!(modell.zeilenzahl(), 0);
        assert!(modell.eintraege().is_empty());
        assert_eq!(modell.auswahl(), None);
    }

    #[test]
    fn ein_neuer_ordner_hebt_die_auswahl_auf() {
        let mut modell = gelesen();
        modell.abschliessen();
        auswaehlen(&mut modell, "zzz.txt");

        modell.lesevorgang_beginnen(2);
        assert!(
            modell.auswahl().is_some(),
            "solange die alten Zeilen stehen, steht auch die Auswahl darauf"
        );

        modell.anhaengen([eintrag("neu.txt", Typ::Datei)]);

        assert_eq!(modell.auswahl(), None, "die Auswahl faellt mit dem Ersatz");
        assert_eq!(modell.auswahl_zeile(), None);
    }

    #[test]
    fn die_markierung_faellt_mit_dem_ersatz_und_nicht_frueher() {
        let mut modell = gelesen();
        modell.abschliessen();
        modell.alle_markieren();
        assert_eq!(modell.markierungsstand().zahl, 2);

        modell.lesevorgang_beginnen(2);
        assert_eq!(
            modell.markierungsstand().zahl,
            2,
            "die markierten Eintraege stehen noch auf dem Schirm"
        );

        modell.anhaengen([eintrag("neu.txt", Typ::Datei)]);
        assert!(modell.markierungsstand().ist_leer());
    }

    /// Woran die Ansicht ablesen soll, dass sie die Tabelle neu holen muss
    /// statt nur eine neue Zeilenzahl zu melden.
    #[test]
    fn der_ersatz_wird_nur_angekuendigt_wenn_zeilen_fallen() {
        let mut leer = Ordnermodell::neu(1);
        leer.lesevorgang_beginnen(2);
        assert!(
            !leer.ersetzt_beim_naechsten_stapel(),
            "ein leeres Modell hat nichts abzuloesen"
        );

        let mut modell = gelesen();
        modell.abschliessen();
        assert!(
            !modell.ersetzt_beim_naechsten_stapel(),
            "ohne begonnenen Lesevorgang steht kein Ersatz aus"
        );

        modell.lesevorgang_beginnen(2);
        assert!(modell.ersetzt_beim_naechsten_stapel());

        modell.anhaengen([eintrag("neu.txt", Typ::Datei)]);
        assert!(
            !modell.ersetzt_beim_naechsten_stapel(),
            "eingeloest wird genau einmal"
        );
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

    // ---- Die Gitmarke: ein zweiter Befundvektor mit eigener Ungueltigkeitsregel
    //
    // Die Proben zu Schritt 5 der Runde 23. Keine von ihnen ruft `git`: sie
    // reichen die Marken von Hand herein, so wie der Gitlauf sie ueber seinen
    // Kanal liefert. Die Laeufe gegen ein angelegtes Repository stehen in
    // `crates/krk-core/tests/git.rs`.

    /// Ein Modell mit fuenf benannten Eintraegen, abgeschlossen und sortiert.
    fn fuenf_eintraege() -> Ordnermodell {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen([
            eintrag("geaendert.txt", Typ::Datei),
            eintrag("vorgemerkt.txt", Typ::Datei),
            eintrag("neu.txt", Typ::Datei),
            eintrag("konflikt.txt", Typ::Datei),
            eintrag("umbenannt.txt", Typ::Datei),
            eintrag("unveraendert.txt", Typ::Datei),
        ]);
        modell.abschliessen();
        modell
    }

    /// Die fuenf Marken in der Reihenfolge von [`Marke::ALLE`], an den fuenf
    /// gleichnamigen Eintraegen.
    fn fuenf_marken() -> Vec<(String, Marke)> {
        [
            ("geaendert.txt", Marke::Geaendert),
            ("vorgemerkt.txt", Marke::Vorgemerkt),
            ("neu.txt", Marke::Neu),
            ("konflikt.txt", Marke::Konflikt),
            ("umbenannt.txt", Marke::Umbenannt),
        ]
        .into_iter()
        .map(|(name, marke)| (name.to_owned(), marke))
        .collect()
    }

    /// Die Marke des Eintrags mit dem genannten Namen.
    fn marke_von(modell: &Ordnermodell, name: &str) -> Option<Marke> {
        let index = modell
            .bestand()
            .iter()
            .position(|eintrag| eintrag.name == name)
            .expect("den Eintrag gibt es nicht");
        modell.gitmarke(index as u32)
    }

    /// Ein zerlegt benannter Eintrag bekommt den vorkomponiert gemeldeten
    /// Befund.
    ///
    /// Der Lauf gegen ein angelegtes Repository steht in
    /// `crates/krk-core/tests/git.rs`; hier wird die Zuordnung allein geprueft,
    /// mit den zwei Schreibweisen von Hand hereingereicht.
    #[test]
    fn ein_zerlegt_benannter_eintrag_bekommt_den_vorkomponierten_befund() {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen([eintrag("U\u{308}bung.txt", Typ::Datei)]);
        modell.abschliessen();

        assert!(
            modell.gitmarken_setzen(1, &[("\u{dc}bung.txt".to_owned(), Marke::Geaendert)]),
            "der vorkomponiert gemeldete Befund findet den zerlegten Eintrag nicht"
        );
        assert_eq!(
            marke_von(&modell, "U\u{308}bung.txt"),
            Some(Marke::Geaendert)
        );
    }

    /// Traegt der Ordner beide Schreibweisen, gewinnt der bytegenaue Treffer.
    ///
    /// APFS haelt die zwei auseinander, also sind es zwei Zeilen, und der
    /// Befund gehoert der, deren Bytes er traegt. Ohne den Vorrang des
    /// bytegenauen Nachschlags landete die Marke an der falschen Zeile.
    #[test]
    fn bei_zwei_schreibweisen_gewinnt_der_bytegenaue_treffer() {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen([
            eintrag("U\u{308}bung.txt", Typ::Datei),
            eintrag("\u{dc}bung.txt", Typ::Datei),
        ]);
        modell.abschliessen();

        assert!(modell.gitmarken_setzen(1, &[("\u{dc}bung.txt".to_owned(), Marke::Neu)]));
        assert_eq!(marke_von(&modell, "\u{dc}bung.txt"), Some(Marke::Neu));
        assert_eq!(
            marke_von(&modell, "U\u{308}bung.txt"),
            None,
            "der Befund ist an der falschen der zwei Zeilen gelandet"
        );
    }

    /// C5.3 (Modellhaelfte) und A11: die fuenf Zustaende stehen an den fuenf
    /// Eintraegen, und der unveraenderte traegt **keine** Marke.
    #[test]
    fn die_fuenf_marken_stehen_an_ihren_eintraegen_und_der_unveraenderte_traegt_keine() {
        let mut modell = fuenf_eintraege();

        assert!(
            modell.gitmarken_setzen(1, &fuenf_marken()),
            "der Befund gehoert zu diesem Bestand und muss eingetragen werden"
        );

        assert_eq!(marke_von(&modell, "geaendert.txt"), Some(Marke::Geaendert));
        assert_eq!(
            marke_von(&modell, "vorgemerkt.txt"),
            Some(Marke::Vorgemerkt)
        );
        assert_eq!(marke_von(&modell, "neu.txt"), Some(Marke::Neu));
        assert_eq!(marke_von(&modell, "konflikt.txt"), Some(Marke::Konflikt));
        assert_eq!(marke_von(&modell, "umbenannt.txt"), Some(Marke::Umbenannt));
        assert_eq!(
            marke_von(&modell, "unveraendert.txt"),
            None,
            "ein Eintrag ohne Befund traegt eine leere Zelle und keine sechste Marke"
        );

        let buchstaben: Vec<char> = Marke::ALLE
            .into_iter()
            .map(|marke| marke.buchstabe())
            .collect();
        assert_eq!(
            buchstaben.len(),
            Marke::ALLE.len(),
            "die Buchstaben stehen fuer alle fuenf Werte"
        );
    }

    /// C7.5: ein Befund, dessen Ordner nicht mehr angezeigt wird, schreibt
    /// nichts in den neuen Ordner. Hier die Modellhaelfte: eine fremde
    /// Generation traegt nichts ein.
    #[test]
    fn ein_befund_mit_fremder_generation_traegt_keine_marke_ein() {
        let mut modell = fuenf_eintraege();

        assert!(
            !modell.gitmarken_setzen(2, &fuenf_marken()),
            "die Generation 2 gehoert einem anderen Lesevorgang"
        );

        for (name, _) in fuenf_marken() {
            assert_eq!(
                marke_von(&modell, &name),
                None,
                "der verspaetete Befund hat {name} markiert"
            );
        }
    }

    /// C7.4: ein Befund, der eintrifft, waehrend `lesevorgang_beginnen` den
    /// Ersatz noch vormerkt, schreibt keine Marke in den alten Bestand — auch
    /// dann nicht, wenn er die neue Generation nennt und der neue Ordner
    /// dieselben Namen fuehrt.
    #[test]
    fn ein_befund_waehrend_des_vorgemerkten_ersatzes_schreibt_nichts_in_den_alten_bestand() {
        let mut modell = fuenf_eintraege();
        modell.lesevorgang_beginnen(2);

        assert!(
            modell.ersetzt_beim_naechsten_stapel(),
            "die Probe traegt nur, solange der alte Bestand noch dasteht"
        );
        assert!(
            modell.gehoert_dazu(2),
            "die Generation gehoert schon dem neuen Lauf, der Inhalt noch dem alten"
        );

        assert!(
            !modell.gitmarken_setzen(2, &fuenf_marken()),
            "der Befund darf den alten Bestand nicht markieren, obwohl die \
             Generation stimmt"
        );
        for (name, _) in fuenf_marken() {
            assert_eq!(
                marke_von(&modell, &name),
                None,
                "der Befund hat {name} im alten Bestand markiert"
            );
        }

        // Nach dem eingeloesten Ersatz nimmt dasselbe Modell denselben Befund an.
        modell.anhaengen([eintrag("geaendert.txt", Typ::Datei)]);
        modell.abschliessen();
        assert!(
            modell.gitmarken_setzen(2, &fuenf_marken()),
            "nach dem Ersatz gehoert der Befund zum Bestand"
        );
        assert_eq!(marke_von(&modell, "geaendert.txt"), Some(Marke::Geaendert));
    }

    /// C7.6: zwei Vektoren, zwei Ungueltigkeitsregeln. Ein Tippen im Filter
    /// wirft allein den Filterbefund weg, ein Ordnerwechsel beide.
    #[test]
    fn ein_tippen_wirft_nur_den_filterbefund_weg_ein_ordnerwechsel_beide() {
        let mut modell = fuenf_eintraege();
        assert!(modell.gitmarken_setzen(1, &fuenf_marken()));
        let index = modell
            .bestand()
            .iter()
            .position(|eintrag| eintrag.name == "geaendert.txt")
            .expect("den Eintrag gibt es nicht") as u32;
        modell.befunde_setzen([(index, Befund::Treffer)]);
        assert_eq!(modell.befund(index), Befund::Treffer);

        // Getippt: die Frage des Filters ist eine andere, die Datei auf der
        // Platte ist dieselbe.
        modell.zeichen_anhaengen('g');

        assert_eq!(
            modell.befund(index),
            Befund::Unentschieden,
            "der Filterbefund gilt der alten Frage und muss fallen"
        );
        assert_eq!(
            modell.gitmarke(index),
            Some(Marke::Geaendert),
            "die Gitmarke haengt am Bestand und nicht am Filtertext"
        );

        // Ordnerwechsel: beide fallen.
        modell.lesevorgang_beginnen(2);
        modell.anhaengen([eintrag("geaendert.txt", Typ::Datei)]);
        modell.abschliessen();

        assert_eq!(modell.befund(0), Befund::Unentschieden);
        assert_eq!(
            modell.gitmarke(0),
            None,
            "der Ordnerwechsel wirft die Gitmarken mit dem Bestand weg"
        );
    }

    /// Ein Name, den der Bestand nicht fuehrt, wird verworfen, ohne die
    /// uebrigen zu verhindern.
    #[test]
    fn ein_unbekannter_name_wird_verworfen_ohne_die_uebrigen_zu_verhindern() {
        let mut modell = fuenf_eintraege();
        let mut marken = vec![("gibtesnicht.txt".to_owned(), Marke::Konflikt)];
        marken.extend(fuenf_marken());
        marken.push(("auchnicht.txt".to_owned(), Marke::Neu));

        assert!(
            modell.gitmarken_setzen(1, &marken),
            "die fuenf bekannten Namen sind einzutragen"
        );

        assert_eq!(marke_von(&modell, "geaendert.txt"), Some(Marke::Geaendert));
        assert_eq!(marke_von(&modell, "umbenannt.txt"), Some(Marke::Umbenannt));
        assert_eq!(marke_von(&modell, "unveraendert.txt"), None);

        // Und ein Befund, der **nur** unbekannte Namen nennt, traegt nichts ein.
        let mut leeres = Ordnermodell::neu(1);
        leeres.anhaengen([eintrag("a.txt", Typ::Datei)]);
        leeres.abschliessen();
        assert!(
            !leeres.gitmarken_setzen(1, &[("b.txt".to_owned(), Marke::Neu)]),
            "kein einziger Name passt, also ist nichts eingetragen"
        );
        assert_eq!(leeres.gitmarke(0), None);
    }

    /// Der zweite Unterschied zu `befunde_setzen`, an der einen Lage gemessen,
    /// in der er sichtbar ist: waehrend eines Lesevorgangs steht die Sicht in
    /// **Lesereihenfolge**, und erst `abschliessen` sortiert sie. Ein Setzer,
    /// der die Sicht neu aufbaute, sortierte sie hier vorzeitig um.
    #[test]
    fn gitmarken_setzen_baut_die_sicht_nicht_neu_auf() {
        let mut modell = Ordnermodell::neu(1);
        modell.anhaengen([
            eintrag("zzz.txt", Typ::Datei),
            eintrag("aaa.txt", Typ::Datei),
        ]);
        assert_eq!(
            name_in_zeile(&modell, 0),
            Some("zzz.txt"),
            "die Probe traegt nur, solange die Sicht noch in Lesereihenfolge steht"
        );

        assert!(modell.gitmarken_setzen(1, &[("aaa.txt".to_owned(), Marke::Neu)]));

        assert_eq!(
            name_in_zeile(&modell, 0),
            Some("zzz.txt"),
            "die Sicht ist umsortiert worden, also wurde sie neu aufgebaut"
        );

        // Die Gegenprobe an demselben Modell: `befunde_setzen` baut auf und
        // sortiert damit um. Ohne sie hinge diese Probe an der Annahme, dass
        // ein Neuaufbau hier ueberhaupt sichtbar waere.
        modell.befunde_setzen([(0, Befund::Treffer)]);
        assert_eq!(
            name_in_zeile(&modell, 0),
            Some("aaa.txt"),
            "befunde_setzen baut die Sicht neu auf und sortiert dabei"
        );
    }
}
