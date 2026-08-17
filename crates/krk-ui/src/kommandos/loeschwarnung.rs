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
//!  auswahl ─┬──> frage_und_erlaeuterung() ──> (Frage, Erlaeuterung)
//!  ordner ──┘
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
//! # Warum die Frage nach dem Papierkorb hier auf [`Befund::Ja`] prueft
//!
//! Weil sie auf der anderen Polaritaet liegt als die Ausloeser der lauten Form:
//! bei ihr ist [`Befund::Ja`] die **Erlaubnis** und nicht der Warngrund, und
//! [`Befund::Unentschieden`] gehoert deshalb zu [`Befund::Nein`].
//! [`Befund::ist_warnwuerdig`] kommt in dieser Datei nicht vor, und das ist
//! Absicht: es fasst `Ja` und `Unentschieden` zusammen und machte hier aus „wir
//! wissen nichts" die Erlaubnis zu loeschen. Die beiden Polaritaeten stehen im
//! Modulkopf von [`krk_core::verzeichnis::Befund`] auseinandergehalten.
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
//! waere ein zweiter Loeschweg, und genau den schafft diese Runde ab. Eine
//! eigene Zaehlung dafuer kommt mit der Tafel der Ausloeser aus dem Buendel C,
//! weil erst diese die Zusage traegt, dass die Einordnung des Ziels einmal
//! geschieht.
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
//! **Das `expect(dead_code)` ist mit dem Aufrufer gefallen.** Es stand an
//! [`frage_und_erlaeuterung`], solange die Funktion nur von ihren eigenen
//! Proben erreicht wurde, und es war
//! `expect` und nicht `allow`, damit der Bau unter `-D warnings` anhaelt,
//! sobald die Erwartung unerfuellt wird. Eine Ausnahme mit Ablaufdatum statt
//! einer, die stehen bleibt und niemandem mehr sagt, warum; dieselbe Bauform
//! hat [`super::rueckschritt`] in der Runde 10 getragen.

use std::path::Path;

use krk_core::verzeichnis::Befund;

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
/// | nein | nein | [`Befund::Ja`] | [`Vorstufe::Rueckfrage`] |
/// | nein | nein | [`Befund::Nein`] | [`Vorstufe::OhnePapierkorb`] |
/// | nein | nein | [`Befund::Unentschieden`] | [`Vorstufe::OhnePapierkorb`] |
///
/// **Die fuenf Zeilen decken alle zwoelf Kombinationen ab** — zwei mal zwei mal
/// drei —, und die Fallunterscheidung ist damit ueberschneidungsfrei und
/// vollstaendig; einen Auffangzweig gibt es nicht, und der Uebersetzer haelt die
/// Vollstaendigkeit. Die Probe `die_tafel_aus_zwoelf_faellen_geht_auf` schreibt
/// alle zwoelf aus, aus demselben Grund, aus dem die Tafeln in
/// [`super::rueckschritt`] und [`Befund::oder`] ausgeschrieben dastehen: eine
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
/// Ein `papierkorb`, der [`Befund::Unentschieden`] traegt, faellt mit
/// [`Befund::Nein`] zusammen: der Modulkopf sagt, warum, und `Ja` ist hier die
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
    papierkorb: Befund,
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
        (false, false, Befund::Ja) => Vorstufe::Rueckfrage,
        // Kein Papierkorb, oder keine Auskunft darueber — beides haelt an. Die
        // beiden Werte stehen ausgeschrieben und nicht als `_` da: ein vierter
        // Befund haelt so den Bau an, statt still hierher zu fallen.
        (false, false, Befund::Nein | Befund::Unentschieden) => Vorstufe::OhnePapierkorb,
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
/// Absicht. [`Vorstufe::OhnePapierkorb`] entsteht aus [`Befund::Nein`] wie aus
/// [`Befund::Unentschieden`], und die beiden unterscheiden sich darin, ob KRK
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

/// Die beiden Zeilen der Rueckfrage vor dem Raeumen in den Papierkorb (C2).
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
/// `#[must_use]`, weil das stille Fallenlassen des Rueckgabewerts unbemerkt
/// bliebe: die Funktion ist rein, also ist ein Aufruf ohne Verwendung ihrer
/// beiden Zeichenketten ein Aufruf ohne jede Wirkung, und der Uebersetzer sagt
/// dazu von sich aus nichts, auch nicht unter `-D warnings`. Verlorenginge
/// dabei die Rueckfrage selbst — ein Blatt ohne Text oder gar keines —, und
/// damit die eine Zusage dieser Runde. Dieselbe Bauform traegt
/// [`super::rueckschritt::rueckschritt`], die Schwesterregel dieses Loeschwegs.
#[must_use]
pub fn frage_und_erlaeuterung(auswahl: &Auswahl, ordner: &Path) -> (String, String) {
    let frage = match auswahl.zahl() {
        1 => "Diesen Eintrag in den Papierkorb räumen?".to_owned(),
        anzahl => format!("Diese {} Einträge in den Papierkorb räumen?", zahl(anzahl)),
    };
    let mut erlaeuterung = format!("Geräumt wird aus {}.", pfadtext(ordner));
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
    use Befund::{Ja, Nein, Unentschieden};

    /// Alle drei Befunde, einmal als Daten.
    ///
    /// Sie stehen hier, weil drei der Stufenproben sie durchfahren, und nicht
    /// damit eine Erwartung daraus gerechnet wuerde: die Erwartungen stehen in
    /// ihren Proben Fall fuer Fall da.
    const BEFUNDE: [Befund; 3] = [Ja, Nein, Unentschieden];

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
        const TAFEL: [(bool, bool, Befund, Vorstufe); 12] = [
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
    /// der Polaritaet, auf der [`Befund::Ja`] die Erlaubnis ist; wer aus
    /// Gewohnheit [`Befund::ist_warnwuerdig`] nimmt, macht aus „wir wissen
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
        let bis_zum_blatt: Vec<(bool, bool, Befund)> = [false, true]
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
        let (frage, _) = frage_und_erlaeuterung(&auswahl(1, 0), Path::new("/Users/k1/Notizen"));
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
        let (frage, _) = frage_und_erlaeuterung(&auswahl(2, 0), Path::new("/Users/k1/Notizen"));
        assert_eq!(frage, "Diese 2 Einträge in den Papierkorb räumen?");

        let (viele, _) = frage_und_erlaeuterung(&auswahl(1234, 0), Path::new("/Users/k1/Notizen"));
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
            frage_und_erlaeuterung(&auswahl(3, 2), Path::new("/Users/k1/Notizen"));
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
            frage_und_erlaeuterung(&auswahl(3, 0), Path::new("/Users/k1/Notizen"));
        assert_eq!(erlaeuterung, "Geräumt wird aus /Users/k1/Notizen.");
    }
}
