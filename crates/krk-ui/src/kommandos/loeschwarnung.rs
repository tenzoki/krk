//! Die eine Rueckfrage vor dem Raeumen in den Papierkorb: **ob sie erscheint**,
//! und **was in ihr steht** (C2, C3, C4).
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Das Blatt selbst baut
//! `crate::appkit::blaetter::loeschbestaetigung`; ob es ueberhaupt erscheint und
//! was darin *steht*, entsteht hier und ist ohne Fenster pruefbar.
//!
//! ```text
//!  vorgang_laeuft ─┐
//!  auswahl_leer ───┼──> vor_der_rueckfrage() ──> Vorstufe
//!  papierkorb ─────┘                              ├─ Rueckfrage ──> das Blatt
//!                                                 └─ sonst ──> eine Meldung
//!
//!  Loeschziel ──> warngruende() ──> [Warngrund], gerangt
//!
//!  auswahl ──┐
//!  ordner ───┼──> frage_und_erlaeuterung() ──> (Frage, Erlaeuterung)
//!  gruende ──┘
//!
//!            ohne_papierkorb() ──> die Meldung der dritten Stufe
//! ```
//!
//! # Warum die Stufenregel hier steht und nicht im Rumpf, der sie ausfuehrt
//!
//! Bis zum 260817 stand die Reihenfolge der Stufen als Kette von `if`-Zweigen im
//! Rumpf von `Anwendungsdelegierter::loeschen_nach_rueckfrage`, und **keine
//! Probe erreichte sie**: `krk-ui` hat kein Bibliotheksziel, und ein Blatt laesst
//! sich unter `libtest` nicht bedienen. Damit war ausgerechnet die Mechanik
//! ungedeckt, um derentwillen diese Runde laeuft
//! (`issues/260817-1107_*_der-rumpf-der-schutzschwelle-traegt-keine-probe.md`).
//!
//! Die Reihenfolge ist aber eine Regel ueber Wahrheitswerte und keine
//! AppKit-Sache: **welche** Stufe ein Loeschbefehl erreicht, haengt an drei
//! Tatsachen und an nichts, was ein Fenster wuesste. Sie steht deshalb als reine
//! Funktion mit ausgeschriebener Tafel da, so wie [`super::rueckschritt`] die
//! Schwesterregel dieses Loeschwegs traegt, und der Rumpf beschafft die
//! Tatsachen und fuehrt aus, was hier entschieden ist.
//!
//! **Sie steht in diesem Modul und nicht in einem neunten daneben**, weil „ob
//! die Rueckfrage erscheint" und „was in ihr steht" dieselbe Sache sind: drei der
//! vier Ausgaenge sind Meldungen, und eine davon entsteht zwei Funktionen
//! weiter unten in [`ohne_papierkorb`]. Ein eigenes Modul trennte die
//! Entscheidung von ihrem Wortlaut, und wer den einen Loeschweg lesen will,
//! muesste beide Dateien nebeneinanderlegen.
//!
//! # Drei Tatsachen, fuenf Stufen
//!
//! Die Kette vor dem Auftrag hat fuenf Stufen — laufender Vorgang, leere
//! Auswahl, Papierkorb am Ziel, das Blatt, die Bestaetigung —, und die Regel
//! hier traegt die **drei**, die vor dem Blatt entschieden sind. Die vierte ist
//! das Blatt selbst, und die fuenfte, die Bestaetigung, bleibt bei ihm: ob
//! Cmd+Return oder Esc gedrueckt wurde, weiss allein AppKit, und dass ein
//! Abbruch keinen Auftrag stellt, ist am Rueckruf des Blattes zu pruefen und
//! damit im Vordergrund. Der Zuschnitt ist keine Bequemlichkeit, sondern die
//! Grenze der Kiste: was hier steht, ist ohne Fenster pruefbar, und was am Blatt
//! haengt, ist es nicht.
//!
//! **Die dritte Stufe kommt aus dem Buendel B dieser Runde** und ist der Grund,
//! aus dem die Regel jetzt umzieht und nicht spaeter: sie setzt eine weitere
//! Stufe in dieselbe Kette, und ein Umzug danach aenderte dieselbe Stelle
//! zweimal.
//!
//! # Warum die Frage nach dem Papierkorb hier auf [`Loeschzielbefund::Ja`] prueft
//!
//! Weil sie auf der anderen Polaritaet liegt als die Ausloeser der lauten Form:
//! bei ihr ist [`Loeschzielbefund::Ja`] die **Erlaubnis** und nicht der Warngrund, und
//! [`Loeschzielbefund::Unentschieden`] gehoert deshalb zu [`Loeschzielbefund::Nein`].
//! [`Loeschzielbefund::ist_warnwuerdig`] kommt in dieser Datei nicht vor, und das ist
//! Absicht: es fasst `Ja` und `Unentschieden` zusammen und machte hier aus „wir
//! wissen nichts" die Erlaubnis zu loeschen. Die beiden Polaritaeten stehen im
//! Modulkopf von [`krk_core::verzeichnis::Loeschzielbefund`] auseinandergehalten.
//!
//! # Die Tafel der sechs Ausloeser, und warum die Aufzaehlung selbst die
//! Rangfolge ist
//!
//! [`warngruende`] ist die zweite reine Regel dieses Moduls: sie bekommt fuenf
//! Tatsachen ueber das Ziel und liefert die Warngruende, gerangt. Ist die Liste
//! leer, bleibt die Rueckfrage ruhig; sonst traegt sie das Warnzeichen, ihre
//! erste Zeile nennt den Wortlaut des **ersten** Grundes, und die Erlaeuterung
//! fuehrt die uebrigen auf (C3).
//!
//! ```text
//!  Rang  Warngrund                 haengt an              Wortlaut
//!  ────  ────────────────────────  ─────────────────────  ─────────────────────────
//!   1    Unentscheidbar            einer der fuenf        von einem Ziel
//!                                  Eingaenge ist nicht    unbekannter Einordnung
//!                                  beantwortet
//!   2    Netzlaufwerk              netzlaufwerk == Ja     von einem Netzlaufwerk
//!   3    Cloudort                  ordner unter           aus einem Cloud-Ordner
//!                                  ~/Library/CloudStorage
//!                                  oder ~/Library/Mobile
//!                                  Documents
//!   4    AusserhalbBenutzerordner  ordner nicht unter     ausserhalb des
//!                                  dem Benutzerordner     Benutzerordners
//!   5    ImBenutzerordner          ordner ist der         unmittelbar im
//!                                  Benutzerordner selbst  Benutzerordner
//!   6    Arbeitsbaum               arbeitsbaum == Ja      aus einem
//!                                                         Git-Arbeitsbaum
//!   7    Umfang                    umfang erreicht die    mit 25 Eintraegen /
//!                                  Schwelle               mit mehr als 25
//!                                                         Eintraegen
//! ```
//!
//! Die Rangfolge steht so im Spec unter C3 und ist danach geordnet, **wie
//! schwer der Weg zurueck ist**. Sie steht hier an genau einer Stelle, naemlich
//! als Reihenfolge der Werte von [`Warngrund`]; eine zweite Liste daneben
//! koennte von ihr weglaufen, und ein neuer Ausloeser bekommt seinen Rang,
//! indem er an die richtige Stelle der Aufzaehlung geschrieben wird.
//!
//! **Fuenf Eingaenge, sechs Ausloeser, sieben Gruende.** Die Zahlen gehen
//! auseinander, und das ist keine Unstimmigkeit: die Ausloeser 1, 2 und 4
//! rechnet [`warngruende`] selbst aus den beiden Pfaden, die sie hereinbekommt,
//! also tragen fuenf Eingaenge sechs Ausloeser. Und `Unentscheidbar` ist kein
//! siebter Ausloeser, sondern der Ausgang, den **jeder** der fuenf Eingaenge
//! nehmen kann, wenn er nicht beantwortet ist; er steht auf Rang 1, weil ein
//! Ziel, das KRK nicht einordnen konnte, die unguenstigste Auskunft ist, die es
//! ueber einen Loeschbefehl geben kann.
//!
//! **Ein unentschiedener Eingang nennt seinen eigenen Ausloeser nicht mit.**
//! Ein `Unentschieden` am Netzlaufwerk liefert `Unentscheidbar` und **nicht**
//! zusaetzlich [`Warngrund::Netzlaufwerk`]: KRK weiss nicht, ob der
//! Datentraeger einer ist, und ein Wortlaut „von einem Netzlaufwerk" in der
//! Erlaeuterung waere eine Behauptung, fuer die es keine Messung gibt. Die
//! Zusage „Unentschieden gilt als laut" ist damit vollstaendig erfuellt — laut
//! wird die Rueckfrage, und der genannte Grund ist der, der zutrifft.
//!
//! Daraus folgt, dass [`Loeschzielbefund::ist_warnwuerdig`] in dieser Datei
//! auch fuer die Ausloeser der ersten Polaritaet nicht vorkommt, obwohl es fuer
//! sie die richtige Frage waere: es fasst `Ja` und `Unentschieden` zusammen,
//! und genau die beiden muessen hier auseinandergehalten werden, weil sie zu
//! **verschiedenen** Warngruenden fuehren. Die Fallunterscheidungen in
//! [`warngruende`] schreiben deshalb alle drei Antworten aus.
//!
//! # Warum die Texte der Loeschfrage eigens dastehen
//!
//! Nach dieser Runde kennt KRK genau einen Loeschweg, und er fragt vorher
//! genau einmal nach. Ein Wortlaut, der an zwei Stellen entstuende, waere zwei
//! Wahrheiten ueber dieselbe Frage; deshalb steht er hier und nicht im Blatt,
//! das ihn zeigt, und nicht in [`super::operationen`], das die Texte aller
//! uebrigen Dateioperationen traegt. `operationen::loeschfrage`, der Wortlaut
//! des endgueltigen Loeschens, faellt mit diesem Loeschweg weg.
//!
//! # Warum der Pfad ungekuerzt dasteht
//!
//! Die Erlaeuterung nennt den vollen Pfad des Ordners, aus dem geraeumt wird,
//! und **nicht** die gekuerzte Form aus
//! `krk_core::ablage::pfade::gekuerzt_fuer_anzeige`, die aus dem
//! Benutzerverzeichnis eine Tilde macht.
//!
//! Der Anlass dieser Runde ist ein Schadensfall: KRK hat am 260817-0344 auf
//! einen einzigen Tastendruck 189 verfolgte Dateien des eigenen Projekts in den
//! Papierkorb geraeumt, ohne Rueckfrage und vier Stunden unbemerkt. Die
//! Erlaeuterung ist die eine Stelle, an der der Nutzer erkennen soll, **welchen**
//! Ordner er gerade leert. Ein `~` an der Stelle des Benutzerverzeichnisses
//! nimmt ihm genau diese Auskunft: es macht zwei verschiedene Orte gleich
//! aussehend und spart dabei die Zeichen, die den Unterschied tragen. Die
//! Kuerzung bleibt der Erfolgsmeldung der Tastenbelegung vorbehalten, wo eine
//! Verwechslung nichts kostet.
//!
//! Gebaut wird der Pfad ueber [`super::operationen::pfadtext`], das dieselbe
//! Entscheidung schon fuer die beiden Pfadkopierer getroffen und begruendet
//! hat. Ein zweiter Pfadformatierer daneben waere die erste Abweichung, die
//! niemand prueft.
//!
//! # Der eine Aufrufer, dreimal
//!
//! Jedes der drei Stuecke dieses Moduls hat genau einen Aufrufer, und alle drei
//! sitzen im Kommandoweg von `crate::appkit::anwendung`:
//! `in_den_papierkorb` ruft [`frage_und_erlaeuterung`], der gemeinsame Rumpf
//! `loeschen_nach_rueckfrage` ruft [`vor_der_rueckfrage`] und, im dritten ihrer
//! vier Zweige, [`ohne_papierkorb`]. Die Aufruferzaehlung
//! `die_stufenregel_hat_genau_einen_aufrufer` haelt die Zahl fuer die Regel
//! fest; sie steht in der Form von `die_regel_hat_genau_einen_aufrufer` in
//! [`super::rueckschritt`], und die Zusage, die sie traegt, ist die dieser
//! Runde: **die Stufenfolge gibt es einmal.** Ein zweiter Aufrufer waere ein
//! zweiter Loeschweg mit einer eigenen Reihenfolge seiner Pruefungen.
//!
//! `Anwendungsdelegierter::in_den_papierkorb` (`crate::appkit::anwendung`) ist
//! fuer die beiden Texte der einzige, und er ist es fuer jeden Weg in den
//! Papierkorb: die beiden Tasten `delete` und `cmd+delete` und der Menueeintrag
//! "In den Papierkorb raeumen" laufen durch ihn hindurch, und er reicht die
//! beiden Texte an den gemeinsamen Rumpf `loeschen_nach_rueckfrage` weiter. Ein zweiter Aufrufer
//! waere ein zweiter Loeschweg, und genau den schafft diese Runde ab.
//!
//! **Seit der Tafel der Ausloeser sind es vier Stuecke und zwei Zaehlungen.**
//! [`warngruende`] bekommt eine eigene, `die_ausloesertafel_hat_genau_einen_aufrufer`,
//! und die Zusage, die sie traegt, ist eine andere als die der Stufenregel: die
//! **Einordnung des Ziels geschieht einmal**. Ein zweiter Aufrufer waere eine
//! zweite Stelle, an der entschieden wird, ob und warum die Rueckfrage laut
//! ist, und die beiden liefen auseinander, ohne dass eine Uebersetzung etwas
//! dazu sagt. Der eine Aufrufer entsteht mit dem elften Schritt dieser Runde,
//! der die fuenf Tatsachen beschafft; bis dahin traegt [`warngruende`] ein
//! `expect(dead_code)` und die Zaehlung erwartet null.
//!
//! **Die Bereichsleiste ist keiner dieser Wege**, obwohl auch sie Kommandos
//! meldet: `crate::appkit::bereichsleiste` schickt zehn, und alle zehn sind
//! Umschalter — fuenf Bereiche, drei Spalten, die tiefe Suche und der
//! Inhaltsfilter. `Kommando::InPapierkorb` ist keines davon, und wer sie in
//! dieser Aufzaehlung mitzaehlt, sucht einen Loeschweg, der nie bestand.
//!
//! **`f8` kommt erst mit Buendel D dazu.** Heute traegt es
//! `Kommando::EndgueltigLoeschen` und erreicht diese Texte nicht; erst mit dem
//! Wegfall jenes Befehls wird es die dritte Taste dieser Aufzaehlung.
//!
//! **Das `expect(dead_code)` an [`frage_und_erlaeuterung`] ist mit dem
//! Aufrufer gefallen, und an [`warngruende`] steht es jetzt.** Es war und ist
//! `expect` und nicht `allow`, damit der Bau unter `-D warnings` anhaelt,
//! sobald die Erwartung unerfuellt wird: eine Ausnahme mit Ablaufdatum statt
//! einer, die stehen bleibt und niemandem mehr sagt, warum. Dieselbe Bauform
//! hat [`super::rueckschritt`] in der Runde 10 getragen.

use std::path::{Path, PathBuf};

use krk_core::verzeichnis::{Loeschzielbefund, Umfang, umfang::SCHWELLE};

use super::operationen::{Auswahl, ordner_text, pfadtext, zahl};

/// Welche Stufe ein Loeschbefehl erreicht, bevor die Rueckfrage erscheint (C2,
/// C4).
///
/// Vier Ausgaenge und kein Wahrheitswert mit Beipackzettel: drei von ihnen
/// halten den Befehl an und nennen dem Nutzer den Grund, der vierte zeigt das
/// Blatt. Sie stehen als Aufzaehlung da, weil die drei Meldungen **nicht
/// dieselbe** sind: „es laeuft schon etwas", „es ist nichts ausgewaehlt" und
/// „hier gibt es keinen Papierkorb" sagen dem Nutzer drei verschiedene Dinge,
/// und ein gemeinsames „nichts geschehen" verschwiege zwei davon.
///
/// Die Fallunterscheidungen darueber tragen keinen Auffangzweig. Eine fuenfte
/// Stufe haelt damit den Bau an und erzwingt eine bewusste Einordnung in die
/// Reihenfolge, statt still in einen bestehenden Zweig zu fallen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vorstufe {
    /// KRK haelt genau einen Vorgang, und es laeuft schon einer. Kein Blatt,
    /// kein Auftrag; die Meldung baut `operationen::schon_ein_vorgang`, denn
    /// sie nennt die Art des **laufenden** Vorgangs.
    VorgangLaeuft,
    /// Es ist nichts ausgewaehlt. Kein Blatt, kein Auftrag; die Rueckfrage
    /// haette keinen Gegenstand.
    NichtsAusgewaehlt,
    /// Das Ziel fuehrt keinen Papierkorb, oder es liess sich nicht feststellen,
    /// ob es einen fuehrt. Kein Blatt, kein Auftrag, und die Meldung steht in
    /// [`ohne_papierkorb`] (C4).
    OhnePapierkorb,
    /// Alle drei Pruefungen sind bestanden: das Blatt erscheint, mit
    /// „Abbrechen" vorbelegt. Erst seine Bestaetigung stellt einen Auftrag.
    Rueckfrage,
}

/// Die eine Reihenfolge der Stufen vor der Rueckfrage (C2, C4).
///
/// Der Rumpf ist diese Tafel, und sie steht ausgeschrieben und nicht gerechnet:
///
/// | `vorgang_laeuft` | `auswahl_leer` | `papierkorb` | Ausgang |
/// |---|---|---|---|
/// | ja | gleichgueltig | gleichgueltig | [`Vorstufe::VorgangLaeuft`] |
/// | nein | ja | gleichgueltig | [`Vorstufe::NichtsAusgewaehlt`] |
/// | nein | nein | [`Loeschzielbefund::Ja`] | [`Vorstufe::Rueckfrage`] |
/// | nein | nein | [`Loeschzielbefund::Nein`] | [`Vorstufe::OhnePapierkorb`] |
/// | nein | nein | [`Loeschzielbefund::Unentschieden`] | [`Vorstufe::OhnePapierkorb`] |
///
/// **Die fuenf Zeilen decken alle zwoelf Kombinationen ab** — zwei mal zwei mal
/// drei —, und die Fallunterscheidung ist damit ueberschneidungsfrei und
/// vollstaendig; einen Auffangzweig gibt es nicht, und der Uebersetzer haelt die
/// Vollstaendigkeit. Die Probe `die_tafel_aus_zwoelf_faellen_geht_auf` schreibt
/// alle zwoelf aus, aus demselben Grund, aus dem die Tafeln in
/// [`super::rueckschritt`] und [`Loeschzielbefund::oder`] ausgeschrieben dastehen: eine
/// gerechnete Erwartung waere die Umsetzung ein zweites Mal.
///
/// # Warum die Reihenfolge selbst die Zusage ist
///
/// Jede der drei Stufen haelt den Befehl an, und **welche** von ihnen zuerst
/// fragt, ist am Ergebnis nur an einer Stelle abzulesen: an der Meldung, die der
/// Nutzer liest. Zwei Zeilen der Tafel tragen deshalb Gewicht ueber ihren
/// eigenen Fall hinaus:
///
/// - **Der laufende Vorgang steht vor dem Blatt.** Bis zum 260817 stand die
///   Frage hinter der Rueckfrage: KRK zeigte ein Blatt, liess bestaetigen und
///   meldete erst danach, dass bereits eine Operation laeuft. Eine Rueckfrage,
///   deren Ja folgenlos bleibt, gewoehnt den Nutzer daran, sie wegzudruecken,
///   und genau diese Gewoehnung ist der Gegner dieser Runde.
/// - **Der Papierkorbtest steht ebenfalls vor dem Blatt** (C4). Er entscheidet,
///   ob es einen Rueckweg gibt; erst danach ist die Rueckfrage die Frage, die
///   sie zu sein behauptet. Danach gefragt, haette der Nutzer einem Raeumen
///   zugestimmt, das nicht raeumen kann.
///
/// Ein `papierkorb`, der [`Loeschzielbefund::Unentschieden`] traegt, faellt mit
/// [`Loeschzielbefund::Nein`] zusammen: der Modulkopf sagt, warum, und `Ja` ist hier die
/// Erlaubnis. Der Aufrufer, der den Ordnerpfad nicht aufloesen kann, reicht
/// deshalb `Unentschieden` herein und loescht damit ebenfalls nicht.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: die Funktion ist rein, aendert also nichts, und wer ihre Antwort
/// nicht nimmt, hat keine der drei Pruefungen gefahren. Verlorenginge dabei die
/// Schutzschwelle dieser Runde, und der Uebersetzer sagt dazu von sich aus
/// nichts, auch nicht unter `-D warnings`.
#[must_use = "die Stufe entscheidet, ob ueberhaupt gefragt wird; fallengelassen laeuft der Loeschbefehl ungeprueft weiter"]
pub fn vor_der_rueckfrage(
    vorgang_laeuft: bool,
    auswahl_leer: bool,
    papierkorb: Loeschzielbefund,
) -> Vorstufe {
    match (vorgang_laeuft, auswahl_leer, papierkorb) {
        // Die erste Stufe. Sie fragt vor allen anderen, weil ein zweiter
        // Vorgang gar nicht anfangen kann und der Nutzer das erfahren soll,
        // bevor er eine Rueckfrage beantwortet.
        (true, _, _) => Vorstufe::VorgangLaeuft,
        // Die zweite. Ohne Auswahl hat die Rueckfrage keinen Gegenstand.
        (false, true, _) => Vorstufe::NichtsAusgewaehlt,
        // Die dritte, und `Ja` ist ihre Erlaubnis: erst jetzt steht fest, dass
        // es einen Rueckweg gibt (C4).
        (false, false, Loeschzielbefund::Ja) => Vorstufe::Rueckfrage,
        // Kein Papierkorb, oder keine Auskunft darueber — beides haelt an. Die
        // beiden Werte stehen ausgeschrieben und nicht als `_` da: ein vierter
        // Befund haelt so den Bau an, statt still hierher zu fallen.
        (false, false, Loeschzielbefund::Nein | Loeschzielbefund::Unentschieden) => {
            Vorstufe::OhnePapierkorb
        }
    }
}

/// Die Meldung, wenn das Ziel keinen Papierkorb fuehrt (C4).
///
/// Drei Auskuenfte in einer Zeile, und jede ist noetig: **der Befund** (das Ziel
/// fuehrt keinen Papierkorb), **die Folge** (es wurde nichts geloescht) und
/// **der Ausweg** (im Finder). Ohne den Befund raetselt der Nutzer, warum der
/// Befehl nichts tat; ohne die Folge muss er nachsehen, ob doch etwas weg ist;
/// ohne den Ausweg bleibt er mit einem Eintrag stehen, den KRK nicht loeschen
/// wird, und einen zweiten Weg dorthin fuehrt KRK seit dieser Runde nicht.
///
/// Die Zeile nennt „das Ziel" und nicht den Datentraeger: gefragt wurde der
/// angezeigte Ordner, und ein Datentraeger ist ein Begriff, den der Nutzer erst
/// auf den Ordner vor sich zurueckrechnen muesste.
///
/// **Der Wortlaut sagt nicht, dass es zwei Wege hierher gibt**, und das ist
/// Absicht. [`Vorstufe::OhnePapierkorb`] entsteht aus [`Loeschzielbefund::Nein`] wie aus
/// [`Loeschzielbefund::Unentschieden`], und die beiden unterscheiden sich darin, ob KRK
/// das Ziel gefragt hat oder nicht fragen konnte. Fuer den Nutzer ist die Folge
/// dieselbe, und die Statuszeile traegt eine Zeile; die Unterscheidung bleibt
/// dort, wo sie etwas entscheidet, naemlich am Befund.
///
/// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: verloren ginge
/// die einzige Auskunft ueber einen Befehl, der nichts getan hat, und die
/// Statuszeile behielte den Text davor.
#[must_use = "der Text ist die einzige Auskunft ueber einen Loeschbefehl, der nichts getan hat"]
pub fn ohne_papierkorb() -> &'static str {
    "das Ziel führt keinen Papierkorb, es wurde nichts gelöscht; im Finder löschen"
}

// ---------------------------------------------------------------------------
// Die Tafel der sechs Ausloeser (C3)
// ---------------------------------------------------------------------------

/// Welcher der beiden Wortlaute des sechsten Ausloesers gilt.
///
/// Zwei Werte und keine Zahl, und das ist der ganze Unterschied zu
/// [`Umfang`]: dort steht, **wie viele** Eintraege an der Auswahl haengen, hier
/// steht, welchen der beiden Saetze aus C3 die Frage traegt. Der Weg von dort
/// nach hier ist die Aussage des Ausloesers, und er faellt drei von fuenf
/// Faellen von [`Umfang`] heraus: eine Zahl unter der Schwelle loest gar nicht
/// aus, und [`Umfang::Unentschieden`] loest [`Warngrund::Unentscheidbar`] aus.
///
/// **Eine Zahl steht hier bewusst nicht.** Sie waere in jedem Fall
/// [`SCHWELLE`]: [`krk_core::verzeichnis::umfang::zaehlen`] deckelt bei
/// `SCHWELLE + 1`, also ist die einzige genaue Zahl, die diesen Ausloeser
/// erreicht, die Schwelle selbst. Eine mitgefuehrte Zahl waere damit ein
/// zweiter Ort fuer denselben Wert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Umfangsgrund {
    /// Der Unterbaum umfasst genau [`SCHWELLE`] Eintraege.
    GenauDieSchwelle,
    /// Er umfasst mehr. Wie viele, ist nicht ermittelt und war nicht gefragt.
    MehrAlsDieSchwelle,
}

/// Warum die Rueckfrage laut ist, und in welchem Rang (C3).
///
/// **Bei diesem Typ ist die Reihenfolge der Aufzaehlung die Aussage.** Sie ist
/// die Rangfolge aus C3 des Specs, [`Ord`] ist deshalb abgeleitet, und
/// [`warngruende`] sortiert seine Liste damit. Wer einen Wert verschiebt,
/// aendert, welchen Grund die Frage nennt; wer einen hinzufuegt, gibt ihm seinen
/// Rang durch die Stelle, an die er ihn schreibt.
///
/// **Das ist der Unterschied zu [`Loeschzielbefund`], bei dem [`Ord`]
/// ausdruecklich nicht abgeleitet ist**, und die beiden zusammen sind keine
/// Inkonsequenz: dort waere eine Ordnung eine Behauptung ohne Gegenstand — `Ja`
/// ist nicht groesser als `Nein`, und `Unentschieden` liegt zwischen keinem von
/// beiden —, hier ist sie der Inhalt. Ein abgeleitetes [`Ord`] sagt in diesem
/// Projekt also nicht „die Werte sind vergleichbar", sondern „ihre Reihenfolge
/// traegt eine Zusage", und deshalb steht es an genau einem der beiden Typen.
///
/// Die Fallunterscheidungen darueber tragen keinen Auffangzweig. Ein siebter
/// Ausloeser haelt damit den Bau an und erzwingt zwei bewusste Entscheidungen:
/// seinen Rang und seinen Wortlaut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Warngrund {
    /// Einer der fuenf Eingaenge des [`Loeschziel`] ist nicht beantwortet.
    ///
    /// **Kein Ausloeser, sondern der Ausgang, den jeder der fuenf nehmen
    /// kann**, und er steht auf Rang 1: ein Ziel, das KRK nicht einordnen
    /// konnte, ist die unguenstigste Auskunft, die es ueber einen Loeschbefehl
    /// geben kann. Er steht genau einmal in der Liste, gleich wie viele
    /// Eingaenge unentschieden sind — welcher es war, aendert am Wortlaut
    /// nichts, und eine Aufzaehlung „dreimal unentschieden" waere fuer den
    /// Nutzer keine Auskunft.
    Unentscheidbar,
    /// Der Datentraeger des Ordners ist kein lokaler.
    ///
    /// Beantwortet von `crate::appkit::volumes::liegt_auf_netzlaufwerk`, das
    /// seit dem 260817 die Antwort dieses Ausloesers liefert und nicht ihre
    /// Umkehrung.
    Netzlaufwerk,
    /// Der Ordner liegt an einem der beiden benannten Cloud-Orte oder darunter.
    Cloudort,
    /// Der Ordner liegt nicht unter dem Benutzerverzeichnis.
    AusserhalbBenutzerordner,
    /// Der Ordner **ist** das Benutzerverzeichnis.
    ImBenutzerordner,
    /// Der Ordner, eine Ebene darueber oder ein ausgewaehlter Ordner traegt
    /// einen Eintrag `.git`.
    Arbeitsbaum,
    /// Der Unterbaum des Vorgangs erreicht die Schwelle aus C3.
    ///
    /// Der einzige Grund mit einem Wert daran, weil er der einzige mit zwei
    /// Wortlauten ist; siehe [`Umfangsgrund`].
    Umfang(Umfangsgrund),
}

/// Die beiden Wortlaute des sechsten Ausloesers schreiben die Schwelle aus.
///
/// Steigt oder sinkt [`SCHWELLE`], haelt diese Zusicherung den Bau an, statt
/// die Rueckfrage eine Zahl nennen zu lassen, die nicht mehr gilt. Dieselbe
/// Bauform bindet das Stapelbudget des Editors an die Editorgrenze
/// (`crate::appkit::editor`).
const _: () = assert!(
    SCHWELLE == 25,
    "die Wortlaute des sechsten Ausloesers nennen die 25 ausgeschrieben"
);

impl Warngrund {
    /// Der Wortlaut, mit dem dieser Grund in der Rueckfrage steht (C3).
    ///
    /// **Sechs der sieben stehen woertlich so im Spec**, in der Spalte
    /// „Wortlaut in der Frage" seiner Tafel unter C3. Der siebte,
    /// [`Warngrund::Unentscheidbar`], ist dort nur der Sache nach festgelegt —
    /// „nennt als Grund, dass das Ziel sich nicht einordnen liess" —, und die
    /// Form ist hier gewaehlt: **eine Fuegung ohne eingeschobenen Nebensatz**.
    /// Jeder Wortlaut steht in der Frage an derselben Stelle, zwischen der Zahl
    /// der Eintraege und „in den Papierkorb raeumen", und ein Relativsatz
    /// verlangte dort ein zweites Komma, das die uebrigen sechs nicht haben.
    ///
    /// Die Wortlaute sind Fuegungen und keine Saetze, damit sie in beide Texte
    /// passen: in die Frage als Einschub und in die Erlaeuterung als Glied einer
    /// Aufzaehlung. Ein Satz koennte nur eines von beiden.
    ///
    /// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: die
    /// Funktion ist rein, und ohne ihren Rueckgabewert nennt die Rueckfrage
    /// ihren Grund nicht.
    #[must_use = "der Wortlaut ist der einzige Ertrag des Aufrufs; fallengelassen nennt die Rueckfrage ihren Grund nicht"]
    pub fn wortlaut(self) -> &'static str {
        match self {
            Self::Unentscheidbar => "von einem Ziel unbekannter Einordnung",
            Self::Netzlaufwerk => "von einem Netzlaufwerk",
            Self::Cloudort => "aus einem Cloud-Ordner",
            Self::AusserhalbBenutzerordner => "außerhalb des Benutzerordners",
            Self::ImBenutzerordner => "unmittelbar im Benutzerordner",
            Self::Arbeitsbaum => "aus einem Git-Arbeitsbaum",
            Self::Umfang(Umfangsgrund::GenauDieSchwelle) => "mit 25 Einträgen",
            Self::Umfang(Umfangsgrund::MehrAlsDieSchwelle) => "mit mehr als 25 Einträgen",
        }
    }
}

/// Die fuenf Tatsachen ueber ein Loeschziel, aus denen [`warngruende`] urteilt.
///
/// **Fuenf Felder und kein Fenster.** Beschafft werden sie vom
/// Anwendungsdelegierten, jede aus genau einer Quelle: die beiden Pfade aus
/// `std::fs::canonicalize` und `krk_core::ablage::pfade::benutzerverzeichnis`,
/// das Netzlaufwerk aus `crate::appkit::volumes::liegt_auf_netzlaufwerk`, der
/// Arbeitsbaum aus `krk_core::verzeichnis::arbeitsbaum::beruehrt_einen_arbeitsbaum`
/// und der Umfang aus `krk_core::verzeichnis::umfang::zaehlen`. Hier stehen sie
/// nur als Werte, und deshalb ist die Regel darunter ohne Fenster, ohne
/// Dateisystem und ohne Netzlaufwerk pruefbar.
///
/// **Die beiden Pfade kommen aufgeloest herein.** Getestet wird der aufgeloeste
/// Ordnerpfad, damit `/tmp` und `/private/tmp` dieselbe Antwort bekommen (C3);
/// die Eintraege des Vorgangs selbst werden nicht aufgeloest. Ein `None` heisst
/// „liess sich nicht aufloesen" und fuehrt auf
/// [`Warngrund::Unentscheidbar`] — nicht auf ein stilles „dann eben nicht
/// warnen".
pub struct Loeschziel {
    /// Der angezeigte Ordner, aufgeloest. `None` heisst nicht aufloesbar.
    pub ordner: Option<PathBuf>,
    /// Das Benutzerverzeichnis, aufgeloest. `None` heisst, das System hat keines
    /// genannt oder es liess sich nicht aufloesen.
    ///
    /// Es reist als Argument herein, damit es genau einen Frager hat und die
    /// Regel ohne Zugriff auf das echte pruefbar ist; dieselbe Bauform traegt
    /// `krk_core::ablage::pfade::gekuerzt_fuer_anzeige`.
    pub benutzerverzeichnis: Option<PathBuf>,
    /// Liegt der Ordner auf einem Netzlaufwerk? Erste Polaritaet: `Ja` warnt.
    pub netzlaufwerk: Loeschzielbefund,
    /// Beruehrt der Vorgang einen Git-Arbeitsbaum? Erste Polaritaet: `Ja` warnt.
    pub arbeitsbaum: Loeschzielbefund,
    /// Wie viele Eintraege haengen an der Auswahl, gedeckelt?
    pub umfang: Umfang,
}

/// Die beiden benannten Cloud-Orte, relativ zum Benutzerverzeichnis (C3,
/// Ausloeser 4).
///
/// **Benannte Orte und keine Klasse „Clouddrive".** Der Spec hat hier den
/// Mechanismus gewechselt statt zu naehern: welcher Ordner von einem
/// Cloud-Dienst verwaltet wird, ist nicht entscheidbar, welcher unter diesen
/// beiden Pfaden liegt, ist es. `CloudStorage` traegt die Anbieter, die den
/// Dateianbieter des Systems benutzen, `Mobile Documents` traegt iCloud Drive.
///
/// Verglichen wird ueber `Path::starts_with`, also **Namensteil fuer
/// Namensteil**: ein Ordner `~/Library/CloudStorageAlt` faellt damit nicht
/// hinein, `~/Library/CloudStorage` selbst und alles darunter schon.
const CLOUDORTE: [&str; 2] = ["Library/CloudStorage", "Library/Mobile Documents"];

/// Ob dieser aufgeloeste Ordner an einem der benannten Cloud-Orte liegt.
///
/// Eine Zeile, und sie steht eigens da, weil sie die einzige Stelle ist, die
/// [`CLOUDORTE`] liest; der Rumpf von [`warngruende`] bleibt damit eine Folge
/// von Fallunterscheidungen ohne Pfadarithmetik dazwischen.
fn liegt_an_einem_cloudort(ordner: &Path, benutzerverzeichnis: &Path) -> bool {
    CLOUDORTE
        .iter()
        .any(|zweig| ordner.starts_with(benutzerverzeichnis.join(zweig)))
}

/// Welche der sechs Ausloeser an diesem Ziel zutreffen, in der Rangfolge aus C3.
///
/// Eine leere Liste heisst: die Rueckfrage bleibt ruhig. Sonst ist sie laut, ihr
/// **erster** Eintrag ist der Grund, den die Frage nennt, und die uebrigen
/// stehen in der Erlaeuterung; beides baut [`frage_und_erlaeuterung`]. Die Tafel
/// der sieben Gruende, ihre Raenge und ihre Wortlaute stehen im Modulkopf
/// ausgeschrieben.
///
/// # Wie die fuenf Eingaenge auf die sieben Gruende fallen
///
/// | Eingang | `Ja` beziehungsweise erreicht | `Nein` beziehungsweise darunter | nicht beantwortet |
/// |---|---|---|---|
/// | `ordner` und `benutzerverzeichnis` | [`Warngrund::Cloudort`], [`Warngrund::ImBenutzerordner`], [`Warngrund::AusserhalbBenutzerordner`] | nichts | [`Warngrund::Unentscheidbar`] |
/// | `netzlaufwerk` | [`Warngrund::Netzlaufwerk`] | nichts | [`Warngrund::Unentscheidbar`] |
/// | `arbeitsbaum` | [`Warngrund::Arbeitsbaum`] | nichts | [`Warngrund::Unentscheidbar`] |
/// | `umfang` | [`Warngrund::Umfang`] | nichts | [`Warngrund::Unentscheidbar`] |
///
/// **Ein unentschiedener Eingang nennt seinen eigenen Ausloeser nicht mit**, und
/// der Modulkopf sagt, warum: KRK weiss dann nicht, ob der Ausloeser zutrifft,
/// und ein Wortlaut in der Erlaeuterung waere eine Behauptung ohne Messung. Die
/// Zusage „Unentschieden gilt als laut" bleibt dabei erfuellt, denn
/// [`Warngrund::Unentscheidbar`] steht auf Rang 1 und macht die Rueckfrage laut.
///
/// **Die drei Ausloeser aus den Pfaden haengen an einem Eingangspaar.** Fehlt
/// einer der beiden Pfade, ist keiner der drei zu beantworten, und es entsteht
/// ein einzelnes [`Warngrund::Unentscheidbar`]. `ImBenutzerordner` und
/// `AusserhalbBenutzerordner` schliessen sich aus — der Ordner ist das
/// Benutzerverzeichnis, oder er liegt darunter, oder er liegt nicht darunter —,
/// und `Cloudort` steht neben beiden, weil ein Cloud-Ort unter dem
/// Benutzerverzeichnis liegt und trotzdem eigens zu nennen ist.
///
/// # Was mit den Doppelungen geschieht
///
/// Sortiert wird nach dem abgeleiteten [`Ord`] von [`Warngrund`], also nach der
/// Rangfolge, und danach werden benachbarte Gleiche entfernt. Das ist der Grund,
/// aus dem drei unentschiedene Eingaenge ein `Unentscheidbar` ergeben und nicht
/// drei; `dedup` genuegt dafuer, weil die Liste sortiert ist.
///
/// `#[must_use]`, weil das stille Fallenlassen unbemerkt bliebe: die Funktion
/// ist rein, und wer ihre Liste nicht nimmt, hat die Einordnung des Ziels nicht
/// gefahren — die Rueckfrage erschiene ruhig ueber einem Ziel, das jeden der
/// sechs Ausloeser trifft, und der Uebersetzer sagt dazu von sich aus nichts.
#[must_use = "die Liste entscheidet, ob die Rueckfrage laut wird und welchen Grund sie nennt; fallengelassen bleibt sie ruhig"]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "der Aufrufer entsteht mit dem elften Schritt dieser Runde, der die fuenf Tatsachen beschafft"
    )
)]
pub fn warngruende(ziel: &Loeschziel) -> Vec<Warngrund> {
    let mut gruende: Vec<Warngrund> = Vec::new();

    // Die Ausloeser 1, 2 und 4 rechnet diese Funktion selbst, und sie haengen
    // an beiden Pfaden zugleich: ohne das Benutzerverzeichnis ist „ausserhalb
    // des Benutzerordners" nicht zu beantworten, und ohne den Ordner keiner der
    // drei.
    match (&ziel.ordner, &ziel.benutzerverzeichnis) {
        (Some(ordner), Some(zuhause)) => {
            if liegt_an_einem_cloudort(ordner, zuhause) {
                gruende.push(Warngrund::Cloudort);
            }
            if ordner == zuhause {
                gruende.push(Warngrund::ImBenutzerordner);
            } else if !ordner.starts_with(zuhause) {
                gruende.push(Warngrund::AusserhalbBenutzerordner);
            }
        }
        // Die drei Faelle mit einem fehlenden Pfad stehen zusammen, weil sie
        // dasselbe bedeuten: KRK kann den Ort des Vorgangs nicht einordnen.
        (None, _) | (_, None) => gruende.push(Warngrund::Unentscheidbar),
    }

    // Der dritte Ausloeser. Alle drei Antworten stehen ausgeschrieben da: `Ja`
    // und `Unentschieden` fuehren zu **verschiedenen** Gruenden, und
    // `Loeschzielbefund::ist_warnwuerdig` fasst genau die beiden zusammen und
    // ist hier deshalb nicht zu gebrauchen.
    match ziel.netzlaufwerk {
        Loeschzielbefund::Ja => gruende.push(Warngrund::Netzlaufwerk),
        Loeschzielbefund::Unentschieden => gruende.push(Warngrund::Unentscheidbar),
        Loeschzielbefund::Nein => {}
    }

    // Der fuenfte, mit derselben Aufteilung.
    match ziel.arbeitsbaum {
        Loeschzielbefund::Ja => gruende.push(Warngrund::Arbeitsbaum),
        Loeschzielbefund::Unentschieden => gruende.push(Warngrund::Unentscheidbar),
        Loeschzielbefund::Nein => {}
    }

    // Der sechste. `Genau` oberhalb der Schwelle kann `zaehlen` nicht liefern,
    // weil es bei `SCHWELLE + 1` deckelt; der Zweig steht trotzdem da und
    // nennt den staerkeren der beiden Wortlaute, statt eine Zahl zu behaupten,
    // die der Wortlaut nicht traegt.
    match ziel.umfang {
        Umfang::Genau(gezaehlt) if gezaehlt == SCHWELLE => {
            gruende.push(Warngrund::Umfang(Umfangsgrund::GenauDieSchwelle));
        }
        Umfang::Genau(gezaehlt) if gezaehlt > SCHWELLE => {
            gruende.push(Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle));
        }
        // Unter der Schwelle, also kein Warngrund.
        Umfang::Genau(_) => {}
        Umfang::MehrAls(_) => {
            gruende.push(Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle));
        }
        Umfang::Unentschieden => gruende.push(Warngrund::Unentscheidbar),
    }

    // Erst die Rangfolge, dann die Doppelungen: `dedup` sieht nur Nachbarn, und
    // nach dem Sortieren stehen Gleiche nebeneinander.
    gruende.sort_unstable();
    gruende.dedup();
    gruende
}

/// Die beiden Zeilen der Rueckfrage vor dem Raeumen in den Papierkorb (C2, C3).
///
/// Genau einmal je Vorgang, unabhaengig von der Zahl der Eintraege und
/// unabhaengig davon, welche Taste den Befehl ausgeloest hat.
///
/// **Die Frage** nennt in ihrer ersten Zeile, wie viele Eintraege betroffen
/// sind, und bei einem einzelnen steht dort die Einzahl. Sie nennt das Ziel des
/// Vorgangs beim Namen — „in den Papierkorb raeumen" und nicht „loeschen" —,
/// weil der Rueckweg ueber den Papierkorb der Unterschied zum Weg ist, den
/// diese Runde abschafft.
///
/// **Die Erlaeuterung** nennt den vollen Pfad des Ordners, aus dem geraeumt
/// wird, und, falls Ordner unter der Auswahl sind, deren Zahl gesondert. Der
/// Vorgang betrifft genau einen Ordner, also nennt sie genau einen Pfad: jeder
/// Pfad der Auswahl entsteht in [`super::operationen::betroffene`] aus dem
/// angezeigten Ordner und dem Namen einer sichtbaren Zeile, auch bei
/// eingeschalteter tiefer Suche.
///
/// Warum der Pfad ungekuerzt dasteht, sagt der Modulkopf.
///
/// # Wo die Warngruende landen (C3)
///
/// `gruende` kommt aus [`warngruende`] und ist gerangt. Eine leere Liste ist die
/// **ruhige** Form: beide Texte stehen dann Wort fuer Wort so da wie vor dieser
/// Stufe, und keine Zeichenkette wird um einen leeren Einschub laenger.
///
/// Sonst geht der Wortlaut des **ersten** Grundes in die Frage, an die Stelle
/// zwischen der Zahl der Eintraege und „in den Papierkorb raeumen", und die
/// **uebrigen** stehen als eigener Absatz in der Erlaeuterung. Der erste steht
/// dort ausdruecklich nicht ein zweites Mal; er ist eine Zeile darueber zu
/// lesen, und C3 verlangt „die Frage nennt einen davon, und die Erlaeuterung
/// fuehrt die uebrigen auf".
///
/// **Das Warnzeichen setzt diese Funktion nicht.** Ob das Blatt laut erscheint,
/// entscheidet der Aufrufer daran, ob die Liste leer ist, und reicht es als
/// eigenen Wahrheitswert an
/// `crate::appkit::blaetter::loeschbestaetigung::zeigen`; ein Text kann kein
/// Warnzeichen tragen.
///
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: die Funktion ist rein, also ist ein Aufruf ohne Verwendung ihrer
/// beiden Zeichenketten ein Aufruf ohne jede Wirkung, und der Uebersetzer sagt
/// dazu von sich aus nichts, auch nicht unter `-D warnings`. Verlorenginge
/// dabei die Rueckfrage selbst — ein Blatt ohne Text oder gar keines —, und
/// damit die eine Zusage dieser Runde. Dieselbe Bauform traegt
/// [`super::rueckschritt::rueckschritt`], die Schwesterregel dieses Loeschwegs.
#[must_use]
pub fn frage_und_erlaeuterung(
    auswahl: &Auswahl,
    ordner: &Path,
    gruende: &[Warngrund],
) -> (String, String) {
    // Der genannte Grund, mit dem Abstand dahinter, den er in der Frage
    // braucht. Ohne Grund bleibt die Fuegung leer, und die ruhige Frage steht
    // damit unveraendert da, statt einen doppelten Abstand zu tragen.
    let genannt = match gruende.first() {
        Some(grund) => format!("{} ", grund.wortlaut()),
        None => String::new(),
    };
    let frage = match auswahl.zahl() {
        1 => format!("Diesen Eintrag {genannt}in den Papierkorb räumen?"),
        anzahl => format!(
            "Diese {} Einträge {genannt}in den Papierkorb räumen?",
            zahl(anzahl)
        ),
    };
    let mut erlaeuterung = format!("Geräumt wird aus {}.", pfadtext(ordner));
    // Die uebrigen Gruende, in der Rangfolge, in der sie hereinkamen. Der
    // erste fehlt hier, weil er in der Frage steht.
    if let Some(uebrige) = gruende.get(1..).filter(|rest| !rest.is_empty()) {
        let aufzaehlung: Vec<&str> = uebrige.iter().map(|grund| grund.wortlaut()).collect();
        erlaeuterung.push_str(&format!("\n\nAußerdem: {}.", aufzaehlung.join(", ")));
    }
    if auswahl.ordner > 0 {
        erlaeuterung.push_str(&format!(
            "\n\nDarunter {}, jeweils mit ihrem gesamten Inhalt.",
            ordner_text(auswahl.ordner)
        ));
    }
    (frage, erlaeuterung)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;
    // Die Tafeln darunter stehen in der Form der Tafel aus dem Doc-Kommentar von
    // [`vor_der_rueckfrage`], und die kurzen Namen halten jede Zeile lesbar auf
    // einer Zeile. Es ist eine Einfuhr der drei Werte und keine pauschale.
    use Loeschzielbefund::{Ja, Nein, Unentschieden};

    /// Alle drei Befunde, einmal als Daten.
    ///
    /// Sie stehen hier, weil drei der Stufenproben sie durchfahren, und nicht
    /// damit eine Erwartung daraus gerechnet wuerde: die Erwartungen stehen in
    /// ihren Proben Fall fuer Fall da.
    const BEFUNDE: [Loeschzielbefund; 3] = [Ja, Nein, Unentschieden];

    /// Genau eine Stelle im Baum ruft die Stufenregel.
    ///
    /// **Eine Aufruferzaehlung in der Form von
    /// `die_regel_hat_genau_einen_aufrufer` in [`super::super::rueckschritt`]**,
    /// und sie steht hier aus demselben Grund: die Zusage dieser Runde handelt
    /// davon, dass es **eine** Reihenfolge der Pruefungen gibt. Ein zweiter
    /// Aufrufer waere ein zweiter Loeschweg, der seine Stufen in eigener
    /// Reihenfolge fragt, und genau den schafft diese Runde ab.
    ///
    /// Der eine Aufrufer ist `Anwendungsdelegierter::loeschen_nach_rueckfrage`
    /// in `crate::appkit::anwendung`, der gemeinsame Rumpf beider Loeschbefehle.
    /// Rot wird die Probe, wenn ein zweiter hinzukommt; die richtige Antwort
    /// darauf ist die Frage, warum es ihn gibt, und nicht die Zahl hier.
    ///
    /// **Diese Datei bleibt aussen vor**, wie bei der Vorlage: die Tafel der
    /// Proben darunter ruft die Regel vielfach, und das sind keine Aufrufer im
    /// Sinne der Zusage. Was eine Aufruferzaehlung leistet und was nicht, steht
    /// in [`crate::quellbaum`].
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt, den
    /// sie liest.
    #[test]
    fn die_stufenregel_hat_genau_einen_aufrufer() {
        let zuhause = "krk-ui/src/kommandos/loeschwarnung.rs";
        let name = concat!("vor_der_", "rueckfrage");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 1,
            "die Stufenregel des Loeschwegs hat nicht genau einen Aufrufer"
        );
    }

    /// Die ganze Regel auf einen Blick: zwei mal zwei mal drei, also zwoelf
    /// Faelle.
    ///
    /// Die Tafel steht in der Form der Tafel aus [`super::super::rueckschritt`]
    /// und schreibt aus, was die fuenf Zeilen der Dokumentation von
    /// [`vor_der_rueckfrage`] mit „gleichgueltig" zusammenfassen. Sie zeigt, dass
    /// keine Kombination fehlt und keine zweimal beantwortet wird; die Proben
    /// darunter zeigen einzelne Felder mit ihrer Begruendung.
    #[test]
    fn die_tafel_aus_zwoelf_faellen_geht_auf() {
        // vorgang_laeuft, auswahl_leer, papierkorb, Ausgang.
        const TAFEL: [(bool, bool, Loeschzielbefund, Vorstufe); 12] = [
            (true, true, Ja, Vorstufe::VorgangLaeuft),
            (true, true, Nein, Vorstufe::VorgangLaeuft),
            (true, true, Unentschieden, Vorstufe::VorgangLaeuft),
            (true, false, Ja, Vorstufe::VorgangLaeuft),
            (true, false, Nein, Vorstufe::VorgangLaeuft),
            (true, false, Unentschieden, Vorstufe::VorgangLaeuft),
            (false, true, Ja, Vorstufe::NichtsAusgewaehlt),
            (false, true, Nein, Vorstufe::NichtsAusgewaehlt),
            (false, true, Unentschieden, Vorstufe::NichtsAusgewaehlt),
            (false, false, Ja, Vorstufe::Rueckfrage),
            (false, false, Nein, Vorstufe::OhnePapierkorb),
            (false, false, Unentschieden, Vorstufe::OhnePapierkorb),
        ];

        for (vorgang_laeuft, auswahl_leer, papierkorb, ausgang) in TAFEL {
            assert_eq!(
                vor_der_rueckfrage(vorgang_laeuft, auswahl_leer, papierkorb),
                ausgang,
                "vorgang_laeuft={vorgang_laeuft}, auswahl_leer={auswahl_leer}, \
                 papierkorb={papierkorb:?}"
            );
        }
    }

    /// Ein laufender Vorgang wird **vor** dem Blatt gemeldet.
    ///
    /// Die erste der vier Eigenschaften, die der Befund 2 der Durchsicht als
    /// ungedeckt aufgeschrieben hat. Bis zum 260817 stand die Frage hinter der
    /// Rueckfrage, und der Nutzer bestaetigte ein Raeumen, das nicht anfing.
    ///
    /// **Der Vorrang ist die Aussage**, nicht der einzelne Ausgang: die Probe
    /// faehrt alle sechs Kombinationen der beiden anderen Tatsachen durch, auch
    /// die, in der Auswahl und Papierkorb in Ordnung sind. Kaeme die Stufe erst
    /// nach einer von ihnen, waere genau eine dieser sechs Zeilen rot.
    #[test]
    fn ein_laufender_vorgang_kommt_nicht_bis_zum_blatt() {
        for auswahl_leer in [false, true] {
            for papierkorb in BEFUNDE {
                assert_eq!(
                    vor_der_rueckfrage(true, auswahl_leer, papierkorb),
                    Vorstufe::VorgangLaeuft,
                    "ein laufender Vorgang haelt den Befehl nicht an: \
                     auswahl_leer={auswahl_leer}, papierkorb={papierkorb:?}"
                );
            }
        }
    }

    /// Die leere Auswahl kommt nicht bis zum Blatt.
    ///
    /// Die zweite Eigenschaft aus dem Befund 2. Sie steht hinter dem laufenden
    /// Vorgang und vor dem Papierkorbtest: die Probe faehrt deshalb alle drei
    /// Befunde durch, denn ohne Gegenstand ist die Frage nach dem Rueckweg
    /// gleichgueltig.
    #[test]
    fn eine_leere_auswahl_kommt_nicht_bis_zum_blatt() {
        for papierkorb in BEFUNDE {
            assert_eq!(
                vor_der_rueckfrage(false, true, papierkorb),
                Vorstufe::NichtsAusgewaehlt,
                "die leere Auswahl erreicht das Blatt: papierkorb={papierkorb:?}"
            );
        }
    }

    /// Ohne Papierkorb erscheint kein Blatt, und der unentschiedene Befund
    /// zaehlt dabei wie das Nein (C4).
    ///
    /// **Die zweite Zusicherung ist die eigentliche.** Hier liegt die Frage auf
    /// der Polaritaet, auf der [`Loeschzielbefund::Ja`] die Erlaubnis ist; wer aus
    /// Gewohnheit [`Loeschzielbefund::ist_warnwuerdig`] nimmt, macht aus „wir wissen
    /// nichts" die Erlaubnis zu loeschen, und dann waere die zweite Zeile dieser
    /// Probe rot und die Runde um ihre Zusage aus C4 herum.
    #[test]
    fn ohne_papierkorb_erscheint_kein_blatt() {
        assert_eq!(
            vor_der_rueckfrage(false, false, Nein),
            Vorstufe::OhnePapierkorb,
            "ein Ziel ohne Papierkorb fuehrt trotzdem zur Rueckfrage"
        );
        assert_eq!(
            vor_der_rueckfrage(false, false, Unentschieden),
            Vorstufe::OhnePapierkorb,
            "ein unentschiedener Befund fuehrt zur Rueckfrage, obwohl C4 ihn \
             wie das Nein behandelt"
        );
    }

    /// Genau einer der zwoelf Faelle erreicht das Blatt.
    ///
    /// Die Zaehlung zur Tafel, und sie sagt etwas, das keine einzelne Zeile
    /// sagt: die Rueckfrage ist der **eine** Ausgang mit drei bestandenen
    /// Pruefungen. Faende sich ein zweiter, waere eine der drei Stufen
    /// durchlaessig geworden.
    #[test]
    fn genau_ein_fall_erreicht_das_blatt() {
        let bis_zum_blatt: Vec<(bool, bool, Loeschzielbefund)> = [false, true]
            .into_iter()
            .flat_map(|vorgang_laeuft| {
                [false, true].into_iter().flat_map(move |auswahl_leer| {
                    BEFUNDE
                        .into_iter()
                        .map(move |papierkorb| (vorgang_laeuft, auswahl_leer, papierkorb))
                })
            })
            .filter(|(vorgang_laeuft, auswahl_leer, papierkorb)| {
                vor_der_rueckfrage(*vorgang_laeuft, *auswahl_leer, *papierkorb)
                    == Vorstufe::Rueckfrage
            })
            .collect();
        assert_eq!(
            bis_zum_blatt,
            vec![(false, false, Ja)],
            "nicht genau eine der zwoelf Kombinationen erreicht das Blatt"
        );
    }

    /// Die Meldung ohne Papierkorb nennt Befund, Folge und Ausweg (C4).
    ///
    /// Drei Zusicherungen fuer drei Auskuenfte, und keine Gleichheitsprobe auf
    /// den ganzen Satz: geprueft ist, was die Zeile leisten muss, nicht ihre
    /// Zeichensetzung. Rot wird sie, wenn eine der drei Auskuenfte beim
    /// Umformulieren verlorengeht — und die Folge „nichts geloescht" ist die,
    /// die am ehesten als selbstverstaendlich gestrichen wuerde.
    #[test]
    fn die_meldung_ohne_papierkorb_nennt_befund_folge_und_ausweg() {
        let meldung = ohne_papierkorb();
        assert!(
            meldung.contains("keinen Papierkorb"),
            "die Meldung nennt den Befund nicht: {meldung}"
        );
        assert!(
            meldung.contains("nichts gelöscht"),
            "die Meldung sagt nicht, dass nichts geloescht wurde: {meldung}"
        );
        assert!(
            meldung.contains("Finder"),
            "die Meldung nennt den Ausweg nicht: {meldung}"
        );
    }

    /// Eine Auswahl aus `anzahl` Eintraegen, davon `ordner` Ordner.
    ///
    /// Die Pfade stehen nur fuer ihre Zahl; welche es sind, sagt die Frage
    /// nicht, und die Erlaeuterung nennt den Ordner und nicht die Eintraege.
    fn auswahl(anzahl: usize, ordner: usize) -> Auswahl {
        Auswahl {
            pfade: (0..anzahl)
                .map(|nummer| PathBuf::from(format!("/Users/k1/Notizen/eintrag-{nummer}")))
                .collect(),
            ordner,
        }
    }

    /// Ein einzelner Eintrag bekommt die Einzahl (C2).
    ///
    /// Das Abnahmekriterium nennt sie eigens, weil die Rueckfrage auch bei
    /// einem Eintrag erscheint und ein „Diese 1 Einträge" den Nutzer zweimal
    /// hinsehen liesse.
    #[test]
    fn ein_eintrag_steht_in_der_einzahl() {
        let (frage, _) =
            frage_und_erlaeuterung(&auswahl(1, 0), Path::new("/Users/k1/Notizen"), &[]);
        assert_eq!(frage, "Diesen Eintrag in den Papierkorb räumen?");
    }

    /// Mehrere Eintraege nennen ihre Zahl, mit der Tausendertrennung der
    /// Oberflaeche (C2).
    ///
    /// Die 1.234 stehen da, weil sie den Punkt zeigen: die Zahl entsteht ueber
    /// [`super::super::operationen::zahl`] und nicht ueber `{}`, damit ein
    /// grosser Vorgang dieselbe Schreibweise traegt wie die laufende
    /// Vorgangsanzeige daneben.
    #[test]
    fn mehrere_eintraege_nennen_ihre_zahl() {
        let (frage, _) =
            frage_und_erlaeuterung(&auswahl(2, 0), Path::new("/Users/k1/Notizen"), &[]);
        assert_eq!(frage, "Diese 2 Einträge in den Papierkorb räumen?");

        let (viele, _) =
            frage_und_erlaeuterung(&auswahl(1234, 0), Path::new("/Users/k1/Notizen"), &[]);
        assert_eq!(viele, "Diese 1.234 Einträge in den Papierkorb räumen?");
    }

    /// Die Erlaeuterung nennt den vollen Pfad, und zwar ungekuerzt (C2).
    ///
    /// **Die zweite Zusicherung ist die eigentliche.** Der Pfad liegt unter dem
    /// Benutzerverzeichnis, also waere er der Fall, in dem
    /// `gekuerzt_fuer_anzeige` eine Tilde setzte. Steht hier je eine, ist die
    /// Festlegung des Modulkopfes gebrochen, und der Nutzer sieht nicht mehr,
    /// welchen Ordner er leert.
    #[test]
    fn die_erlaeuterung_nennt_den_vollen_pfad() {
        let (_, erlaeuterung) = frage_und_erlaeuterung(
            &auswahl(3, 0),
            Path::new("/Users/k1/Projects/productive/krk/fusion-workbench/shared"),
            &[],
        );
        assert_eq!(
            erlaeuterung,
            "Geräumt wird aus /Users/k1/Projects/productive/krk/fusion-workbench/shared."
        );
        assert!(
            !erlaeuterung.contains('~'),
            "der Pfad steht gekuerzt in der Erlaeuterung: {erlaeuterung}"
        );
    }

    /// Sind Ordner darunter, nennt die Erlaeuterung ihre Zahl gesondert (C2).
    ///
    /// Der Zusatz sagt, was die Zahl der Eintraege verschweigt: hinter einem
    /// der drei Eintraege haengt ein ganzer Baum. Die Zahl der Eintraege bleibt
    /// davon unberuehrt, ein Ordner zaehlt in der Frage eins.
    #[test]
    fn die_ordner_stehen_gesondert_in_der_erlaeuterung() {
        let (frage, erlaeuterung) =
            frage_und_erlaeuterung(&auswahl(3, 2), Path::new("/Users/k1/Notizen"), &[]);
        assert_eq!(frage, "Diese 3 Einträge in den Papierkorb räumen?");
        assert_eq!(
            erlaeuterung,
            "Geräumt wird aus /Users/k1/Notizen.\n\nDarunter 2 Ordner, \
             jeweils mit ihrem gesamten Inhalt."
        );
    }

    /// Ohne Ordner in der Auswahl bleibt der Zusatz weg (C2).
    ///
    /// Die Gegenprobe zur vorigen: der zweite Absatz entsteht nur, wenn er
    /// etwas zu sagen hat.
    #[test]
    fn ohne_ordner_bleibt_die_erlaeuterung_einzeilig() {
        let (_, erlaeuterung) =
            frage_und_erlaeuterung(&auswahl(3, 0), Path::new("/Users/k1/Notizen"), &[]);
        assert_eq!(erlaeuterung, "Geräumt wird aus /Users/k1/Notizen.");
    }

    // -----------------------------------------------------------------------
    // Die Tafel der sechs Ausloeser (C3)
    // -----------------------------------------------------------------------

    /// Die sieben Gruende in der Rangfolge des Specs, einmal als Daten.
    ///
    /// Achtzeilig und nicht siebenzeilig: [`Warngrund::Umfang`] traegt zwei
    /// Wortlaute, und beide gehoeren in die Reihe, damit die Ordnung ueber die
    /// ganze Aufzaehlung gemessen wird und nicht ueber sechs Siebtel davon.
    ///
    /// Sie steht hier, weil zwei Proben sie durchfahren, und nicht damit eine
    /// Erwartung daraus gerechnet wuerde: die Erwartungen stehen in ihren Proben
    /// Fall fuer Fall da.
    const RANGFOLGE: [Warngrund; 8] = [
        Warngrund::Unentscheidbar,
        Warngrund::Netzlaufwerk,
        Warngrund::Cloudort,
        Warngrund::AusserhalbBenutzerordner,
        Warngrund::ImBenutzerordner,
        Warngrund::Arbeitsbaum,
        Warngrund::Umfang(Umfangsgrund::GenauDieSchwelle),
        Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle),
    ];

    /// Ein Ziel, an dem kein Ausloeser zutrifft und jeder entscheidbar ist.
    ///
    /// Die Grundlage der Ausloeserproben: jede aendert **ein** Feld und liest
    /// ab, was daraus folgt. Der Ordner liegt unter dem Benutzerverzeichnis und
    /// ist nicht es selbst, der Datentraeger ist lokal, kein Arbeitsbaum ist
    /// beruehrt, und drei Eintraege liegen weit unter der Schwelle.
    ///
    /// Dass dieses Ziel wirklich ruhig ist, prueft
    /// [`ein_ruhiges_ziel_hat_keinen_warngrund`] und nicht diese Funktion; sonst
    /// haetten die Ausloeserproben eine Grundlage, die niemand gemessen hat.
    fn ruhiges_ziel() -> Loeschziel {
        Loeschziel {
            ordner: Some(PathBuf::from("/Users/k1/Notizen")),
            benutzerverzeichnis: Some(PathBuf::from("/Users/k1")),
            netzlaufwerk: Nein,
            arbeitsbaum: Nein,
            umfang: Umfang::Genau(3),
        }
    }

    /// Genau eine Stelle im Baum fragt die Ausloesertafel — heute keine.
    ///
    /// **Die zweite Aufruferzaehlung dieses Moduls**, und sie traegt eine andere
    /// Zusage als `die_stufenregel_hat_genau_einen_aufrufer`: die **Einordnung
    /// des Ziels geschieht einmal**. Ein zweiter Aufrufer waere eine zweite
    /// Stelle, an der entschieden wird, ob und warum die Rueckfrage laut ist,
    /// und die beiden liefen auseinander, ohne dass eine Uebersetzung etwas dazu
    /// sagt.
    ///
    /// **Die Erwartung ist heute null und nicht eins, und der Name sagt das.**
    /// Der eine Aufrufer entsteht mit dem elften Schritt dieser Runde, der die
    /// fuenf Tatsachen beschafft. Eine Probe, die heute schon eins erwartet,
    /// waere rot; eine, die „hoechstens eins" erwartet, waere fuer immer gruen
    /// und wuerde nie etwas messen. Der elfte Schritt setzt die Erwartung auf
    /// eins und den Namen auf `die_ausloesertafel_hat_genau_einen_aufrufer`,
    /// zugleich mit dem `expect(dead_code)`, das dann unerfuellt wird.
    ///
    /// **Diese Datei bleibt aussen vor**, wie bei der Vorlage: die Proben
    /// darunter rufen die Regel vielfach, und das sind keine Aufrufer im Sinne
    /// der Zusage. Was eine Aufruferzaehlung leistet und was nicht, steht in
    /// [`crate::quellbaum`].
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt, den
    /// sie liest.
    #[test]
    fn die_ausloesertafel_hat_noch_keinen_aufrufer() {
        let zuhause = "krk-ui/src/kommandos/loeschwarnung.rs";
        let name = concat!("warn", "gruende");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 0,
            "die Ausloesertafel hat einen Aufrufer; ist der elfte Schritt gelaufen, gehoert \
             die Erwartung auf 1 und der Name dieser Probe auf \
             die_ausloesertafel_hat_genau_einen_aufrufer"
        );
    }

    /// Die Rangfolge der Aufzaehlung ist die des Specs unter C3.
    ///
    /// Die Reihenfolge der Werte **ist** die Zusage, und [`Ord`] ist deshalb
    /// abgeleitet; diese Probe liest sie ab, statt sie zu behaupten. Rot wird
    /// sie, sobald jemand einen Wert verschiebt — und das ist die einzige Art,
    /// die Rangfolge zu aendern, weil es keine zweite Liste gibt.
    ///
    /// Geprueft wird **streng** aufsteigend und nicht bloss aufsteigend: zwei
    /// Gruende auf demselben Rang machten den genannten Grund von der
    /// Sortierung abhaengig, und `sort_unstable` sagt zu gleichen Werten nichts
    /// zu.
    #[test]
    fn die_rangfolge_der_aufzaehlung_ist_die_des_specs() {
        for paar in RANGFOLGE.windows(2) {
            assert!(
                paar[0] < paar[1],
                "{:?} steht nicht vor {:?}; die Rangfolge aus C3 ist verschoben",
                paar[0],
                paar[1]
            );
        }
    }

    /// Jeder der acht Gruende traegt seinen Wortlaut, ausgeschrieben.
    ///
    /// Sechs stehen woertlich so im Spec, in der Spalte „Wortlaut in der Frage"
    /// seiner Tafel unter C3; die beiden Wortlaute des Umfangs sind der siebte
    /// und der achte, und [`Warngrund::Unentscheidbar`] traegt die hier
    /// gewaehlte Fuegung. Die Erwartungen stehen als Zeichenketten da und werden
    /// nicht gerechnet, aus demselben Grund wie in [`super::super::rueckschritt`].
    #[test]
    fn jeder_grund_traegt_seinen_wortlaut() {
        const TAFEL: [(Warngrund, &str); 8] = [
            (
                Warngrund::Unentscheidbar,
                "von einem Ziel unbekannter Einordnung",
            ),
            (Warngrund::Netzlaufwerk, "von einem Netzlaufwerk"),
            (Warngrund::Cloudort, "aus einem Cloud-Ordner"),
            (
                Warngrund::AusserhalbBenutzerordner,
                "außerhalb des Benutzerordners",
            ),
            (Warngrund::ImBenutzerordner, "unmittelbar im Benutzerordner"),
            (Warngrund::Arbeitsbaum, "aus einem Git-Arbeitsbaum"),
            (
                Warngrund::Umfang(Umfangsgrund::GenauDieSchwelle),
                "mit 25 Einträgen",
            ),
            (
                Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle),
                "mit mehr als 25 Einträgen",
            ),
        ];

        for (grund, wortlaut) in TAFEL {
            assert_eq!(grund.wortlaut(), wortlaut, "{grund:?}");
        }
    }

    /// Das ruhige Ziel traegt keinen Warngrund.
    ///
    /// Die Grundlage jeder Ausloeserprobe darunter, gemessen und nicht
    /// angenommen. Waere sie schon laut, messten die uebrigen Proben nicht, was
    /// ihr Name sagt.
    #[test]
    fn ein_ruhiges_ziel_hat_keinen_warngrund() {
        assert_eq!(warngruende(&ruhiges_ziel()), Vec::new());
    }

    /// Die drei Ausloeser aus den beiden Pfaden, Fall fuer Fall (C3, 1, 2, 4).
    ///
    /// Die Tafel schreibt die Ordnerpfade und ihre Gruende aus. Das
    /// Benutzerverzeichnis ist in jeder Zeile `/Users/k1`, und die drei uebrigen
    /// Eingaenge stehen ruhig — gemessen wird allein, was aus dem Ordnerpfad
    /// folgt.
    ///
    /// **Die vierte und die fuenfte Zeile sind die eigentlichen.**
    /// `~/Library/CloudStorage` und `~/Library/Mobile Documents` liegen **unter**
    /// dem Benutzerverzeichnis, also loest dort nur der Cloud-Ort aus und nicht
    /// zusaetzlich einer der beiden Ordnerausloeser; und `CloudStorageAlt` ist
    /// keiner, weil `Path::starts_with` Namensteil fuer Namensteil vergleicht
    /// und nicht Zeichen fuer Zeichen.
    #[test]
    fn die_drei_ausloeser_aus_den_pfaden_stehen_einzeln_da() {
        let faelle: [(&str, Vec<Warngrund>); 9] = [
            // Unter dem Benutzerverzeichnis, kein Sonderort: ruhig.
            ("/Users/k1/Notizen", Vec::new()),
            // Tief darunter, ebenfalls ruhig.
            ("/Users/k1/Projects/productive/krk", Vec::new()),
            // Das Benutzerverzeichnis selbst.
            ("/Users/k1", vec![Warngrund::ImBenutzerordner]),
            // Ausserhalb.
            (
                "/Volumes/Extern/Sicherung",
                vec![Warngrund::AusserhalbBenutzerordner],
            ),
            ("/", vec![Warngrund::AusserhalbBenutzerordner]),
            // Ein anderer Benutzer ist ebenfalls ausserhalb.
            (
                "/Users/gast/Notizen",
                vec![Warngrund::AusserhalbBenutzerordner],
            ),
            // Die beiden Cloud-Orte, samt allem darunter.
            ("/Users/k1/Library/CloudStorage", vec![Warngrund::Cloudort]),
            (
                "/Users/k1/Library/Mobile Documents/com~apple~CloudDocs/Notizen",
                vec![Warngrund::Cloudort],
            ),
            // Aehnlich benannt und trotzdem keiner.
            ("/Users/k1/Library/CloudStorageAlt", Vec::new()),
        ];

        for (ordner, erwartet) in faelle {
            let ziel = Loeschziel {
                ordner: Some(PathBuf::from(ordner)),
                ..ruhiges_ziel()
            };
            assert_eq!(warngruende(&ziel), erwartet, "Ordner {ordner}");
        }
    }

    /// Fehlt einer der beiden Pfade, ist das Ziel unentscheidbar (C3).
    ///
    /// Drei Zeilen fuer die drei Kombinationen mit einem fehlenden Pfad, und
    /// jede liefert **einen** Grund: die drei Ausloeser aus den Pfaden haengen
    /// an beiden zugleich, also ist keiner von ihnen zu beantworten, und
    /// „ausserhalb des Benutzerordners" waere ohne Benutzerordner eine
    /// Behauptung ohne Messung.
    #[test]
    fn ein_fehlender_pfad_macht_das_ziel_unentscheidbar() {
        let faelle: [(Option<&str>, Option<&str>); 3] = [
            (None, Some("/Users/k1")),
            (Some("/Users/k1/Notizen"), None),
            (None, None),
        ];

        for (ordner, zuhause) in faelle {
            let ziel = Loeschziel {
                ordner: ordner.map(PathBuf::from),
                benutzerverzeichnis: zuhause.map(PathBuf::from),
                ..ruhiges_ziel()
            };
            assert_eq!(
                warngruende(&ziel),
                vec![Warngrund::Unentscheidbar],
                "ordner={ordner:?}, benutzerverzeichnis={zuhause:?}"
            );
        }
    }

    /// Netzlaufwerk und Arbeitsbaum warnen auf `Ja` (C3, 3 und 5).
    ///
    /// Beide liegen auf der ersten Polaritaet, und beide bekommen ihre Zeile
    /// einzeln: sie tragen denselben Typ und **verschiedene** Gruende, und eine
    /// gemeinsame Zeile liesse eine Vertauschung der beiden Felder gruen
    /// durchgehen.
    #[test]
    fn das_netzlaufwerk_und_der_arbeitsbaum_warnen_auf_ja() {
        let netz = Loeschziel {
            netzlaufwerk: Ja,
            ..ruhiges_ziel()
        };
        assert_eq!(warngruende(&netz), vec![Warngrund::Netzlaufwerk]);

        let baum = Loeschziel {
            arbeitsbaum: Ja,
            ..ruhiges_ziel()
        };
        assert_eq!(warngruende(&baum), vec![Warngrund::Arbeitsbaum]);
    }

    /// Ein unentschiedener Eingang wird unentscheidbar und nennt seinen eigenen
    /// Ausloeser **nicht** mit (C3).
    ///
    /// **Die zweite Zusicherung jeder Zeile ist die eigentliche.** Die Zusage
    /// „Unentschieden gilt als laut" ist erfuellt, sobald die Liste nicht leer
    /// ist. Ein zusaetzliches [`Warngrund::Netzlaufwerk`] daneben waere aber
    /// eine Behauptung ohne Messung: KRK weiss nicht, ob der Datentraeger einer
    /// ist. Genau diese Zeile wuerde rot, wenn jemand die drei Antworten mit
    /// [`Loeschzielbefund::ist_warnwuerdig`] zusammenfasste — es zieht `Ja` und
    /// `Unentschieden` in denselben Zweig, und die beiden fuehren hier zu
    /// verschiedenen Gruenden.
    #[test]
    fn ein_unentschiedener_eingang_nennt_seinen_ausloeser_nicht_mit() {
        let netz = Loeschziel {
            netzlaufwerk: Unentschieden,
            ..ruhiges_ziel()
        };
        assert_eq!(warngruende(&netz), vec![Warngrund::Unentscheidbar]);

        let baum = Loeschziel {
            arbeitsbaum: Unentschieden,
            ..ruhiges_ziel()
        };
        assert_eq!(warngruende(&baum), vec![Warngrund::Unentscheidbar]);

        let umfang = Loeschziel {
            umfang: Umfang::Unentschieden,
            ..ruhiges_ziel()
        };
        assert_eq!(warngruende(&umfang), vec![Warngrund::Unentscheidbar]);
    }

    /// Drei unentschiedene Eingaenge ergeben **einen** Grund.
    ///
    /// Die Zaehlung zur vorigen Probe: `Unentscheidbar` steht genau einmal in
    /// der Liste, gleich wie viele Eingaenge unentschieden sind. Welcher es war,
    /// aendert am Wortlaut nichts, und ein dreifaches „von einem Ziel
    /// unbekannter Einordnung" in der Erlaeuterung waere fuer den Nutzer keine
    /// Auskunft. Rot wird die Probe, wenn das `dedup` faellt.
    #[test]
    fn mehrere_unentschiedene_eingaenge_ergeben_einen_grund() {
        let ziel = Loeschziel {
            ordner: None,
            netzlaufwerk: Unentschieden,
            arbeitsbaum: Unentschieden,
            umfang: Umfang::Unentschieden,
            ..ruhiges_ziel()
        };
        assert_eq!(warngruende(&ziel), vec![Warngrund::Unentscheidbar]);
    }

    /// Der sechste Ausloeser, Zeile fuer Zeile (C3, 6).
    ///
    /// Die Schwelle traegt die Zahl, alles darueber das „mehr als", und
    /// darunter loest gar nichts aus. Die drei Zeilen mit `Genau` stehen
    /// ausgeschrieben da, weil an ihnen die Zahl in der Frage haengt: bei 24
    /// bleibt die Rueckfrage ruhig, bei 25 nennt sie die 25, und `MehrAls` nennt
    /// sie „mehr als 25".
    ///
    /// Die letzte Zeile, `Genau(26)`, kann `zaehlen` nicht liefern — es deckelt
    /// bei `SCHWELLE + 1`. Sie steht trotzdem da, weil der Zweig existiert und
    /// weil ein Wortlaut „mit 26 Eintraegen" nirgends vorgesehen ist: die Frage
    /// nennt dann „mehr als 25" und behauptet keine Zahl, die sie nicht traegt.
    #[test]
    fn der_umfang_loest_ab_der_schwelle_aus() {
        let genau = Warngrund::Umfang(Umfangsgrund::GenauDieSchwelle);
        let mehr = Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle);
        let faelle: [(Umfang, Vec<Warngrund>); 6] = [
            (Umfang::Genau(0), Vec::new()),
            (Umfang::Genau(1), Vec::new()),
            (Umfang::Genau(24), Vec::new()),
            (Umfang::Genau(25), vec![genau]),
            (Umfang::Genau(26), vec![mehr]),
            (Umfang::MehrAls(25), vec![mehr]),
        ];

        for (umfang, erwartet) in faelle {
            let ziel = Loeschziel {
                umfang,
                ..ruhiges_ziel()
            };
            assert_eq!(warngruende(&ziel), erwartet, "{umfang:?}");
        }
    }

    /// Treffen mehrere zu, stehen sie in der Rangfolge, und der erste ist der
    /// genannte (C3).
    ///
    /// Vier Gruende an einem Ziel, und keiner davon unentschieden: ein
    /// Cloud-Ort auf einem Netzlaufwerk, in einem Arbeitsbaum, mit mehr als 25
    /// Eintraegen. Die Reihenfolge, in der die Rumpfzweige sie einsammeln, ist
    /// eine andere als die Rangfolge — der Cloud-Ort kommt zuerst —, und die
    /// Probe misst deshalb wirklich die Sortierung.
    ///
    /// **Vier ist das Hoechste, was ohne einen unentschiedenen Eingang
    /// zusammenkommt.** `Cloudort` liegt unter dem Benutzerverzeichnis, also
    /// schliesst er die beiden Ordnerausloeser aus, und die schliessen sich
    /// gegenseitig aus.
    #[test]
    fn treffen_vier_zu_stehen_sie_in_der_rangfolge() {
        let ziel = Loeschziel {
            ordner: Some(PathBuf::from(
                "/Users/k1/Library/CloudStorage/Dienst/Projekt",
            )),
            benutzerverzeichnis: Some(PathBuf::from("/Users/k1")),
            netzlaufwerk: Ja,
            arbeitsbaum: Ja,
            umfang: Umfang::MehrAls(25),
        };
        assert_eq!(
            warngruende(&ziel),
            vec![
                Warngrund::Netzlaufwerk,
                Warngrund::Cloudort,
                Warngrund::Arbeitsbaum,
                Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle),
            ]
        );
    }

    /// Ein unentschiedener Eingang steht vor allen zutreffenden Gruenden.
    ///
    /// Die Gegenprobe zur vorigen: derselbe Ordner, aber der Datentraeger liess
    /// sich nicht einordnen. `Unentscheidbar` steht auf Rang 1 und ist damit der
    /// **genannte** Grund, und `Netzlaufwerk` ist aus der Liste verschwunden,
    /// weil es nicht mehr gemessen ist.
    #[test]
    fn der_unentscheidbare_grund_steht_vor_den_zutreffenden() {
        let ziel = Loeschziel {
            ordner: Some(PathBuf::from(
                "/Users/k1/Library/CloudStorage/Dienst/Projekt",
            )),
            benutzerverzeichnis: Some(PathBuf::from("/Users/k1")),
            netzlaufwerk: Unentschieden,
            arbeitsbaum: Ja,
            umfang: Umfang::MehrAls(25),
        };
        assert_eq!(
            warngruende(&ziel),
            vec![
                Warngrund::Unentscheidbar,
                Warngrund::Cloudort,
                Warngrund::Arbeitsbaum,
                Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle),
            ]
        );
    }

    /// Die Frage nennt den ersten Grund, die Erlaeuterung die uebrigen (C3).
    ///
    /// Beide Texte stehen als ganze Zeichenketten da, denn hier haengt der
    /// Wortlaut der Rueckfrage daran und nicht bloss, dass etwas vorkommt. Der
    /// erste Grund steht in der Erlaeuterung **nicht** ein zweites Mal; die
    /// zweite Zusicherung haelt das fest.
    #[test]
    fn die_frage_nennt_den_ersten_grund_und_die_erlaeuterung_die_uebrigen() {
        let gruende = [
            Warngrund::Netzlaufwerk,
            Warngrund::Arbeitsbaum,
            Warngrund::Umfang(Umfangsgrund::MehrAlsDieSchwelle),
        ];
        let (frage, erlaeuterung) =
            frage_und_erlaeuterung(&auswahl(3, 1), Path::new("/Volumes/Netz/Projekt"), &gruende);
        assert_eq!(
            frage,
            "Diese 3 Einträge von einem Netzlaufwerk in den Papierkorb räumen?"
        );
        assert_eq!(
            erlaeuterung,
            "Geräumt wird aus /Volumes/Netz/Projekt.\n\nAußerdem: aus einem Git-Arbeitsbaum, \
             mit mehr als 25 Einträgen.\n\nDarunter ein Ordner, jeweils mit ihrem gesamten \
             Inhalt."
        );
        assert!(
            !erlaeuterung.contains("von einem Netzlaufwerk"),
            "der genannte Grund steht ein zweites Mal in der Erlaeuterung: {erlaeuterung}"
        );
    }

    /// Ein einzelner Grund kommt ohne den Absatz „Außerdem" aus (C3).
    ///
    /// Die Gegenprobe: der Absatz entsteht nur, wenn er etwas zu sagen hat,
    /// genau wie der ueber die Ordner. Genommen ist
    /// [`Warngrund::Unentscheidbar`], weil seine Fuegung die laengste ist und
    /// zeigt, dass sie ohne ein zweites Komma in die Frage passt.
    #[test]
    fn ein_einzelner_grund_kommt_ohne_den_absatz_aus() {
        let (frage, erlaeuterung) = frage_und_erlaeuterung(
            &auswahl(1, 0),
            Path::new("/Users/k1/Notizen"),
            &[Warngrund::Unentscheidbar],
        );
        assert_eq!(
            frage,
            "Diesen Eintrag von einem Ziel unbekannter Einordnung in den Papierkorb räumen?"
        );
        assert_eq!(erlaeuterung, "Geräumt wird aus /Users/k1/Notizen.");
    }

    /// Ohne Grund stehen beide Texte Wort fuer Wort wie vor dieser Stufe (C2).
    ///
    /// **Die Zusicherung, dass die ruhige Form nichts von der lauten
    /// mitbekommt.** Ein Einschub, der bei leerer Liste einen zweiten Abstand
    /// oder ein Komma hinterliesse, waere hier zu sehen; die Probe steht
    /// deshalb neben den fuenf Textproben aus dem ersten Schritt und nicht in
    /// ihnen.
    #[test]
    fn ohne_grund_bleibt_die_ruhige_form_unveraendert() {
        let (frage, erlaeuterung) =
            frage_und_erlaeuterung(&auswahl(2, 1), Path::new("/Users/k1/Notizen"), &[]);
        assert_eq!(frage, "Diese 2 Einträge in den Papierkorb räumen?");
        assert_eq!(
            erlaeuterung,
            "Geräumt wird aus /Users/k1/Notizen.\n\nDarunter ein Ordner, jeweils mit ihrem \
             gesamten Inhalt."
        );
    }
}
