//! Die eine Zulaessigkeitsfrage: darf dieser Befehl hier gerade wirken (C2)?
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die vier Eingaben liest der
//! Anwendungsdelegierte und stellt sie als [`Lage`] zusammen; die Regel selbst
//! steht hier und ist ohne Fenster pruefbar.
//!
//! ```text
//!  zulaessig(Kommando) ──> Anspruch::Kommando ─┐
//!  dateiablage_zulaessig() ─> Anspruch::Dateiablage ┤   (copy:, cut:, paste:)
//!                                               ├─> Wirkungsbereich ─┐
//!                                               └─> Ausnahmeliste ───┤
//!                                                                    ├──> gestattet()
//!  Lage ─────┬─> blatt_steht ────────────────────────────────────────┤
//!            ├─> Ersthelferbefund ──────────────────────────────────┤
//!            ├─> Fokus ──────────────────────────────────────────────┤
//!            └─> Schluesselfenster ──────────────────────────────────┘
//! ```
//!
//! # Eine Frage, zwei Frager
//!
//! Der Ereignisabgriff fragt ueber `Anwendungsdelegierter::kommando_ausfuehren`,
//! ob er den Tastendruck schlucken darf; das Hauptmenue fragt ueber
//! `validateMenuItem:`, ob es seinen Eintrag freigeben darf. Beide fragen
//! **dieselbe** Funktion auf **derselben** [`Lage`], und ihre Antworten koennen
//! deshalb nicht auseinanderlaufen. Genau das ist der Grund, aus dem die Regel
//! eine Funktion ist und nicht zwei Abfragen an zwei Stellen: ein freigegebener
//! Menueeintrag zu einem abgewiesenen Tastendruck fuehrte den Befehl aus, den
//! der Abgriff eben verweigert hat.
//!
//! # Ein Rumpf, zwei Eingaenge (Runde 22)
//!
//! Seit der Runde 22 hat die Regel **einen** Rumpf, [`gestattet`], und zwei
//! benannte Eingaenge davor. [`zulaessig`] nimmt ein [`Kommando`] und ist der
//! Eingang der zwei Frager oben; [`dateiablage_zulaessig`] nimmt allein die
//! [`Lage`] und ist der Eingang fuer die Dateiablage, also fuer die drei
//! Selektoren des Menues „Bearbeiten", die der Delegierte am Dateifenster
//! beantwortet: `copy:` und `cut:` seit der Runde 22, `paste:` seit der
//! Runde 21. Keiner der drei ist ein Kommando, und keiner bekommt eines
//! (Constraint 3 der Runde 22, Constraint 3 und 5 der Runde 21). Was der
//! Rumpf vom Befehl wissen will, sind
//! drei Antworten, und die gibt [`Anspruch`] fuer beide Eingaenge vollstaendig
//! und ohne Auffangzweig: den Wirkungsbereich, ob der Befehl waehrend eines
//! Blattes erlaubt ist, ob er immer erreichbar ist. Die Dateiablage antwortet
//! `Dateifenster`, nein, nein (A11 der Runde 22), und die Ausnahmeliste
//! waechst nicht.
//!
//! **`paste:` nimmt seit der Runde 21 denselben Eingang und bekommt keinen
//! dritten Wert von [`Anspruch`].** Das Einfuegen in den Filtertext stellt
//! byteweise denselben Anspruch wie das Kopieren und das Ausschneiden:
//! `Wirkungsbereich::Dateifenster`, nicht waehrend eines Blattes, nicht immer
//! erreichbar (A9 der Runde 21). Ein Wert `Anspruch::Einfuegen` mit denselben
//! drei Antworten waere eine zweite Kopie der drei Antworten, die sich allein
//! im Namen unterschiede; darum liest sich `Dateiablage` seit der Runde 21 als
//! „der Ablage-Einhaengepunkt des Dateifensters" und nicht mehr als „die zwei
//! Selektoren, die Verweise ablegen". Ob der Name umbenannt gehoert, ist eine
//! offene Frage des Plans der Runde 21 und keine dieser Datei.
//!
//! **Der zweite Eingang hat seine zwei eigenen Frager, und es sind dieselben
//! zwei Stellen**: `validateMenuItem:` fuer die Ausgrauung von „Kopieren",
//! „Ausschneiden" und „Einfuegen", und
//! `Anwendungsdelegierter::bearbeiten_am_dateifenster` fuer die Antwort auf
//! den Selektor, seit der Runde 21 der eine Vorspann, durch den alle drei
//! Selektoren gehen (bis dahin hiess er `dateiablage_ausfuehren` und bediente
//! zwei). Beide fragen [`dateiablage_zulaessig`] auf derselben [`Lage`]; die
//! Zaehlprobe `die_zwei_frager_der_dateiablage_rufen_dieselbe_regel` haelt
//! die Zahl, wie `beide_frager_rufen_die_eine_regel` sie fuer den
//! Kommando-Eingang haelt. Dass die Zahl mit dem dritten Selektor nicht
//! gewachsen ist, ist der Zweck des einen Vorspanns.
//! Ein generisches `zulaessig(impl Into<Anspruch>, Lage)` waere die andere Form
//! gewesen; sie haette den Kommando-Frager auf drei gehoben und die Tafel aus
//! 280 Faellen an einen Trait gebunden. Zwei benannte Huellen um einen
//! privaten Rumpf lassen beides, wie es ist.
//!
//! # Die vier Bestandteile
//!
//! 1. **Es steht kein Blatt**, oder der Befehl ist waehrend eines Blattes
//!    erlaubt. Die zweite Haelfte beantwortet
//!    [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt)
//!    und keine zweite Fassung daneben.
//! 2. **Der Ersthelfer des Schluesselfensters gehoert nicht AppKit.** Dieselbe
//!    Frage, die der Fokusvorbehalt bis zum 260813 im Ereignisabgriff selbst
//!    gestellt hat; sie ist hierher gewandert und steht nicht mehr daneben.
//! 3. **[`fokus::wirkt`](super::fokus::wirkt) sagt ja** zum Wirkungsbereich des
//!    Befehls und zum Fokus des Augenblicks.
//! 4. **Das Schluesselfenster gehoert KRK.** Es ist das Hauptfenster oder ein
//!    Blatt, das daran haengt; steht ein fremdes Fenster vorn, wirkt kein
//!    Befehl.
//!
//! **Der vierte Bestandteil ist der juengste, und er schliesst die Luecke, die
//! der Ueber-Dialog aufreisst.** Ein freistehendes Panel ist kein Blatt, also
//! sagt (1) nichts dazu, und welchen Ersthelfer AppKit darin einsetzt, ist
//! nicht zugesagt, also traegt (2) den Fall nicht. Bestandteil (3) weist vor
//! einem fremden Fenster schon heute jeden Befehl ab, dessen Wirkungsbereich
//! ein Bereich ist, denn `Anwendungsdelegierter::fokus` antwortet dann
//! `Anderswo`; durch kommt genau, was `Wirkungsbereich::Ueberall` traegt, und
//! das ist der Rest, den (4) aufhaelt. Entschieden hat das der Nutzer am
//! 260813-1055 mit Moeglichkeit 2 aus
//! `decisions/260813-1037_*_wirken-krks-tastenbefehle-weiter-waehrend-der-ueber-dialog-steht.md`.
//!
//! **Ein anhaengendes Blatt braucht dafuer keinen Sonderfall.** Es **ist** das
//! Schluesselfenster, also sagt (4) fuer ein stehendes Blatt ja, und ueber das
//! Blatt entscheidet allein (1) wie bisher. Der Abbruch aus dem Blatt heraus
//! bleibt damit erreichbar, ohne dass eine Ausnahme dafuer geschrieben waere.
//!
//! **(4) steht neben (1) und (2) und nicht ueber ihnen.** Alle drei fragen nach
//! der Lage, und die Ausnahmeliste hebt sie deshalb gemeinsam auf; nur (3)
//! fragt nach dem Wirkungsbereich und bleibt unberuehrt. Der naechste Abschnitt
//! fuehrt es aus.
//!
//! **Der zweite Bestandteil ist der, den man weglaesst, und er traegt den
//! gefaehrlichsten Fall.** Beim Umbenennen direkt in der Dateiliste haelt der
//! Feldeditor eines `NSTextField` den Ersthelferrang. Es steht dabei **kein**
//! Blatt, und `Anwendungsdelegierter::fokus` antwortet fuer diesen Feldeditor
//! `Dateifenster`, weil er eine Unteransicht des Dateifensters ist. Die beiden
//! anderen Bestandteile sagen also beide freundlich ja, und ohne den zweiten
//! waere jeder Befehl des Dateifensters freigegeben — `up`, `down`, `return`,
//! `space` und `tab` liegen in `resources/default-keymap.toml` ohne Zusatztaste.
//! Der Nutzer benennt um, drueckt `up`, und die Auswahl in der Liste springt.
//!
//! # Die Ausnahmeliste
//!
//! [`immer_erreichbar`] hebt die Bestandteile (1), (2) und (4) auf, den
//! dritten **nicht**. In einem Satz: sie hebt jede Sperre auf, die nach der
//! **Lage** fragt, und keine, die nach dem **Wirkungsbereich** fragt.
//!
//! Sie ist aus „kein Verlust gegenueber heute" abgeleitet: `beenden`
//! und `fenster_schliessen` sind heute waehrend einer Umbenennung in der Liste
//! und waehrend eines stehenden Blattes allein ueber ihren Menueeintrag
//! erreichbar, weil der Abgriff den Tastendruck an AppKit weiterreicht und das
//! Hauptmenue ihn aufnimmt. Sobald jeder Eintrag des Menues seine Zulaessigkeit
//! von hier bezieht, naehme die Regel ohne Ausnahme genau diesen Weg weg.
//!
//! **`fenster_einblenden` steht aus derselben Randbedingung darauf, und an ihm
//! trifft (4) am haertesten.** Ein geschlossenes Fenster gibt den
//! Schluesselrang ab, KRK haelt genau eines, also meldet die Lage danach
//! dasselbe `schluesselfenster_gehoert_krk == false` wie vor einem fremden
//! Fenster. Cmd+N ist der eine Rueckweg aus dieser Lage; ohne den Eintrag
//! wiese (4) ihn ab und uebrig bliebe der Klick auf das Dock-Symbol
//! (`issues/260813-1258_*_fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar.md`).
//!
//! **Dass sie auch (4) aufhebt, ist eine Wahl und keine Ableitung.** Der
//! Wortlaut des Entscheids sagt „wirkt kein Befehl", und unter dieser Fassung
//! wirken `beenden` und `fenster_schliessen` doch. Der Grund ist dieselbe
//! Randbedingung: vor dem Freigabewaehler der Runde 6 beendet Cmd+Q die
//! Anwendung heute, und die strenge Lesart naehme diesen Weg weg, ohne dass
//! ihn jemand genannt haette. Der Nutzer hat es am 260813 mit Moeglichkeit 1
//! aus
//! `decisions/260813-1110_*_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`
//! so festgelegt.
//!
//! Dass sie den dritten Bestandteil nicht aufhebt, faellt heute nicht auf:
//! jeder Eintrag der Liste traegt `Wirkungsbereich::Ueberall`, und
//! [`fokus::wirkt`](super::fokus::wirkt) sagt dafuer in jedem Fokus ja. Sobald
//! ein Eintrag der Liste einen Bereich braucht, faellt der Unterschied an, und
//! die Probe `die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf` haelt ihn
//! fest. Eine Ordnungszahl steht hier bewusst nicht: sie altert mit jedem
//! neuen Eintrag, die Regel darueber nicht.

use krk_core::tasten::{Kommando, Wirkungsbereich};

use super::fokus::{self, Fokus};
use super::operationen;

/// Was die Oberflaeche im Augenblick der Frage ueber sich weiss.
///
/// Die vier Eingaben der Zulaessigkeitsfrage an **einer** Stelle, damit die
/// Frage rein bleibt und die Tafel aus 280 Faellen sie ohne Fenster stellen
/// kann. Erhoben werden sie von `Anwendungsdelegierter::lage`, und zwar einmal
/// je Eingabe: der Kommandozweig gibt die `Lage` an [`zulaessig`], der
/// Zeichenzweig liest drei davon heraus. Zwei Erhebungen
/// desselben Augenblicks koennten auseinanderlaufen, eine kann es nicht.
///
/// `Copy`, weil der Wert vier kleine Felder traegt und die Tafel ihn
/// zweihundertachtzigmal durchreicht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lage {
    /// Ob am Hauptfenster gerade ein Blatt haengt.
    pub blatt_steht: bool,
    /// Ob der Ersthelfer des Schluesselfensters seine AppKit-Bedeutung behaelt.
    ///
    /// Die eigenen Textflaechen von KRK sind die Ausnahme davon und melden
    /// hier `false`. Es sind seit der Runde 14 zwei, die Textflaeche des
    /// eingebauten Editors und die Textanzeige der Vorschau; die
    /// Naemlichkeitsfrage dahinter beantwortet der Anwendungsdelegierte, der
    /// beide Flaechen haelt. Die Flaeche eines Blattes gehoert ausdruecklich
    /// nicht dazu und meldet `true`, denn nur so bleibt `Abbrechen` dort
    /// unzulaessig und schliesst `Esc` den Notizzettel.
    pub ersthelfer_gehoert_appkit: bool,
    /// Ob das Schluesselfenster KRKs Hauptfenster oder ein daran haengendes
    /// Blatt ist.
    ///
    /// **Ein anhaengendes Blatt meldet hier `true`**, denn es ist selbst das
    /// Schluesselfenster; ueber das Blatt entscheidet allein
    /// [`blatt_steht`](Self::blatt_steht) zusammen mit
    /// [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt).
    /// Die beiden Felder sind unabhaengig: steht ein Blatt und oeffnet der
    /// Nutzer den Ueber-Dialog, ist `blatt_steht` wahr und dieses Feld `false`.
    ///
    /// `false` meldet jedes fremde Fenster, das freistehende Panel des
    /// Ueber-Dialogs so gut wie das Fenster einer anderen Anwendung, und
    /// ebenso ein KRK ohne Schluesselfenster, also im Hintergrund.
    pub schluesselfenster_gehoert_krk: bool,
    /// Wo der Eingabefokus steht.
    pub fokus: Fokus,
}

/// Ob dieser Befehl in dieser Lage wirken darf.
///
/// **Die eine Stelle, an der die Frage beantwortet wird**, und die eine
/// Antwort, die der Ereignisabgriff und die Ausgrauung des Hauptmenues
/// gemeinsam bekommen. Die vier Bestandteile und ihre Herleitung stehen im
/// Modulkopf.
///
/// Eine Sonderbehandlung fuer ein einzelnes Kommando gibt es hier nicht:
/// welcher Bereich noetig ist, sagt der [`Wirkungsbereich`] des Kerns, was
/// waehrend eines Blattes durchkommt, sagt
/// [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt),
/// und was ohne Ruecksicht auf beides erreichbar bleibt, sagt
/// [`immer_erreichbar`].
///
/// [`Wirkungsbereich`]: krk_core::tasten::Wirkungsbereich
pub fn zulaessig(kommando: Kommando, lage: Lage) -> bool {
    gestattet(Anspruch::Kommando(kommando), lage)
}

/// Ob die Dateiablage in dieser Lage wirken darf (A11 der Runde 22, A9 der
/// Runde 21): die drei Selektoren des Menues „Bearbeiten", die der Delegierte
/// am Dateifenster beantwortet, `copy:` und `cut:` seit der Runde 22 und
/// `paste:` seit der Runde 21.
///
/// **Der zweite Eingang zur einen Regel, und kein zweiter Rumpf.** Die
/// Dateiablage ist kein [`Kommando`]: sie haengt an keiner Taste der
/// Belegung und an keinem `krkKommando:`-Eintrag, sondern an den drei
/// Aktionsselektoren, die AppKit dem Anwendungsdelegierten am Ende der
/// Antwortkette zustellt. Ein Kommando dafuer anzulegen hiesse, `cmd+c` oder
/// `cmd+v` in `resources/default-keymap.toml` zu binden, und das Ereignis
/// kaeme im Editor nie mehr beim Textsystem an. Also fragt sie die Regel ohne
/// Kommando, mit dem [`Anspruch`], den ein Kommando mit
/// `Wirkungsbereich::Dateifenster` stellte: kein stehendes Blatt, ein
/// Ersthelfer, der nicht AppKit gehoert, der Fokus im Dateifenster und KRKs
/// eigenes Schluesselfenster.
///
/// Das Einfuegen stellt denselben Anspruch wie das Kopieren und das
/// Ausschneiden, Byte fuer Byte, und bekommt deshalb weder einen eigenen
/// Eingang noch einen dritten Wert von [`Anspruch`]; der Grund steht im
/// Modulkopf unter „Ein Rumpf, zwei Eingaenge".
///
/// Warum eine zweite benannte Huelle und nicht eine generische Signatur an
/// [`zulaessig`], steht ebenfalls dort.
///
/// `#[must_use]`, weil ein Rufer, der die Antwort fallen liesse, den Befehl
/// ausfuehrte, den die Regel eben verweigert hat.
#[must_use]
pub fn dateiablage_zulaessig(lage: Lage) -> bool {
    gestattet(Anspruch::Dateiablage, lage)
}

/// Was die Regel vom Befehl wissen will, fuer jeden ihrer Eingaenge.
///
/// Drei Antworten braucht [`gestattet`], und jede Methode gibt sie als
/// vollstaendiges `match` ueber die zwei Varianten, ohne Auffangzweig: ein
/// dritter Eingang haelt den Bau an und bekommt seine Einordnung bewusst.
/// Fuer ein Kommando kommen die Antworten aus dem Kern und aus
/// [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt);
/// die Dateiablage antwortet fest, denn sie hat kein Kommando, das man fragen
/// koennte.
///
/// Privat: die Aufzaehlung ist die Innenseite der Regel, und die Rufer kennen
/// allein die zwei benannten Eingaenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Anspruch {
    /// Ein Tastenbefehl oder ein Eintrag des Hauptmenues.
    Kommando(Kommando),
    /// Der Ablage-Einhaengepunkt des Dateifensters: `copy:` und `cut:` seit
    /// der Runde 22, `paste:` seit der Runde 21. Ein eigener Wert fuer das
    /// Einfuegen entsteht nicht, weil er dieselben drei Antworten gaebe
    /// (A9 der Runde 21).
    Dateiablage,
}

impl Anspruch {
    /// Welchen Bereich der Befehl braucht.
    fn wirkungsbereich(self) -> Wirkungsbereich {
        match self {
            Anspruch::Kommando(kommando) => kommando.wirkungsbereich(),
            Anspruch::Dateiablage => Wirkungsbereich::Dateifenster,
        }
    }

    /// Ob der Befehl waehrend eines stehenden Blattes durchkommt.
    fn waehrend_blatt_erlaubt(self) -> bool {
        match self {
            Anspruch::Kommando(kommando) => operationen::waehrend_blatt_erlaubt(kommando),
            Anspruch::Dateiablage => false,
        }
    }

    /// Ob der Befehl auf der Ausnahmeliste steht.
    ///
    /// Die Dateiablage steht nicht darauf, und die Liste waechst mit ihr
    /// nicht (C4.2 der Runde 22).
    fn immer_erreichbar(self) -> bool {
        match self {
            Anspruch::Kommando(kommando) => immer_erreichbar(kommando),
            Anspruch::Dateiablage => false,
        }
    }
}

/// Der eine Rumpf der Regel: die vier Bestandteile aus dem Modulkopf.
///
/// Bis zur Runde 22 war das der Rumpf von [`zulaessig`]; er ist unveraendert
/// hierher gewandert und fragt seit dem den [`Anspruch`] statt das Kommando.
/// Beide oeffentlichen Eingaenge sind Einzeiler auf diese Funktion, und die
/// Zaehlprobe `die_zulaessigkeitsregel_ist_genau_einmal_erklaert` haelt fest,
/// dass es sie genau einmal gibt.
fn gestattet(anspruch: Anspruch, lage: Lage) -> bool {
    let kein_blatt_oder_erlaubt = !lage.blatt_steht || anspruch.waehrend_blatt_erlaubt();
    let durchgelassen = anspruch.immer_erreichbar()
        || (lage.schluesselfenster_gehoert_krk
            && kein_blatt_oder_erlaubt
            && !lage.ersthelfer_gehoert_appkit);

    durchgelassen && fokus::wirkt(anspruch.wirkungsbereich(), lage.fokus)
}

/// Die benannte Liste der Befehle, die ein Blatt, ein Textfeld und ein fremdes
/// oder fehlendes Schluesselfenster nicht aufhalten.
///
/// **Bewusst keine vollstaendige Fallunterscheidung.** Die uebrigen
/// Fallunterscheidungen dieses Projekts zaehlen jedes Kommando auf, damit ein
/// neues den Bau anhaelt und eine bewusste Einordnung erzwingt. Hier waere das
/// falsch herum: die Liste soll **nicht** mit jedem neuen Befehl wachsen,
/// sondern nur mit einem genannten Grund. Der Vorgabewert ist „gehoert nicht
/// dazu", und ein neues Kommando bekommt ihn stillschweigend und richtig.
///
/// Alle drei Eintraege stammen aus „kein Verlust gegenueber heute"; die
/// Herleitung steht im Modulkopf. Die Liste hebt die Bestandteile (1), (2) und (4) auf und
/// den dritten nicht: sie hebt jede Sperre auf, die nach der Lage fragt, und
/// keine, die nach dem Wirkungsbereich fragt.
pub fn immer_erreichbar(kommando: Kommando) -> bool {
    matches!(
        kommando,
        Kommando::Beenden | Kommando::FensterSchliessen | Kommando::FensterEinblenden
    )
}

#[cfg(test)]
mod tests {
    use crate::quellbaum::quelldateien;

    use super::*;

    /// Die Zulaessigkeitsregel ist im Baum genau einmal erklaert (C2.16, erste
    /// Haelfte).
    ///
    /// **Eine Erklaerungszaehlung und keine Aufruferzaehlung.** Sie haelt die
    /// eine Haelfte: eine zweite Fassung derselben Regel laesst sie rot werden.
    /// Die andere Haelfte, dass beide Frager diese eine Stelle rufen, zaehlt
    /// [`beide_frager_rufen_die_eine_regel`] daneben. Die Begruendung fuer die
    /// Unterscheidung steht in [`crate::quellbaum`].
    ///
    /// Die Nadel steht zusammengesetzt da, wie bei
    /// `es_gibt_genau_einen_menuebauer` in
    /// [`crate::appkit::teilen`]: als ein Stueck
    /// geschrieben faende sie sich selbst und zaehlte eine Fundstelle zu viel.
    ///
    /// **Seit der Runde 22 sind es zwei Nadeln**: der oeffentliche Eingang
    /// [`zulaessig`] und der private Rumpf [`gestattet`], jeder genau einmal
    /// erklaert. Die Erklaerung von [`dateiablage_zulaessig`] trifft die erste
    /// Nadel nicht, weil die Nadel mit dem Schluesselwort und einem
    /// Leerzeichen beginnt. Beide Nadeln stehen zusammengesetzt da und werden
    /// in dieser Prosa nicht ausgeschrieben, aus dem Grund im Absatz darueber.
    #[test]
    fn die_zulaessigkeitsregel_ist_genau_einmal_erklaert() {
        for regel in [concat!("fn ", "zulaessig("), concat!("fn ", "gestattet(")] {
            let erklaerungen: usize = quelldateien()
                .iter()
                .map(|(_, inhalt)| inhalt.matches(regel).count())
                .sum();
            assert_eq!(erklaerungen, 1, "`{regel}` ist nicht genau einmal erklaert");
        }
    }

    /// Genau zwei Stellen rufen die Regel, und es sind der Abgriff und die
    /// Ausgrauung (C2.16, zweite Haelfte).
    ///
    /// **Eine Aufruferzaehlung, und sie steht hier, weil C2.16 die Zahl selbst
    /// zusagt.** Der eine Frager ist `Anwendungsdelegierter::kommando_ausfuehren`
    /// und entscheidet, ob der Ereignisabgriff den Tastendruck schluckt; der
    /// andere ist `validateMenuItem:` und entscheidet, ob der Menueeintrag
    /// bedienbar ist. Beide fragen dieselbe Funktion auf derselben [`Lage`], und
    /// **daran** haengt die Runde: gaeben sie verschiedene Antworten, fuehrte
    /// ein freigegebener Eintrag den Befehl aus, den der Abgriff eben verweigert
    /// hat — mit dem Fokus im Editor bewegte ein Auf-Pfeil die Dateiliste.
    ///
    /// Rot wird die Probe, wenn ein dritter berechtigter Frager hinzukommt. Die
    /// richtige Antwort darauf ist die Zahl hier und nicht das Streichen eines
    /// Fragers; was eine Aufruferzaehlung leistet und was nicht, steht in
    /// [`crate::quellbaum`].
    ///
    /// **Gezaehlt wird der Aufruf und nicht seine Schreibweise.** Die Nadel war
    /// bis zur Runde 7 `zulaessigkeit::zulaessig(`, und ein dritter Frager mit
    /// einem `use` und einem unqualifizierten Aufruf waere ihr entgangen — also
    /// genau der Fall, fuer den die Probe steht
    /// (`issues/260813-0540_*_zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs.md`).
    /// [`crate::quellbaum::aufrufstellen`] zaehlt jetzt jede Empfaengerform und
    /// jeden Pfad.
    ///
    /// **Diese Datei bleibt aussen vor**, so wie
    /// `das_menue_wird_an_zwei_anlaessen_gebaut` `menue.rs` aussen vor laesst:
    /// hier stehen die Erklaerung und die Tafel aus 280 Faellen, die
    /// [`zulaessig`] zweihundertachtzigmal ruft. Ein dritter Frager **in** dieser
    /// Datei waere Teil der Regel und nicht ein zweiter Weg an ihr vorbei.
    #[test]
    fn beide_frager_rufen_die_eine_regel() {
        let zuhause = "krk-ui/src/kommandos/zulaessigkeit.rs";
        let name = concat!("zulaes", "sig");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| crate::quellbaum::aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 2,
            "die Regel hat nicht die zwei Frager Ereignisabgriff und Ausgrauung"
        );
    }

    /// Der Dateiablage-Eingang hat genau zwei Frager, und es sind dieselben
    /// zwei Stellen wie beim Kommando-Eingang (C4.5 der Runde 22, Baumhaelfte;
    /// C3.6 der Runde 21).
    ///
    /// `validateMenuItem:` fragt fuer die Ausgrauung von „Kopieren",
    /// „Ausschneiden" und „Einfuegen",
    /// `Anwendungsdelegierter::bearbeiten_am_dateifenster` fuer die Antwort auf
    /// `copy:`, `cut:` und `paste:`; seit der Runde 21 ist dieser eine
    /// Vorspann der Rumpf aller drei Selektoren, und deshalb bleibt die Zahl
    /// beim dritten Selektor bei zwei. Beide rufen
    /// [`dateiablage_zulaessig`] auf derselben [`Lage`], aus demselben Grund
    /// wie in [`beide_frager_rufen_die_eine_regel`]: ein freigegebener Eintrag
    /// zu einer abgewiesenen Antwort legte Verweise ab oder fuellte den
    /// Filtertext, was die Regel eben verweigert hat.
    ///
    /// Die Nadel `dateiablage_zulaessig` zaehlt die Nachbarin nicht mit: vor
    /// deren `zulaessig(` steht ein Unterstrich, und
    /// [`crate::quellbaum::aufrufstellen`] laesst eine Fundstelle mitten in
    /// einem Namen heraus. Diese Datei bleibt aussen vor, wie dort.
    #[test]
    fn die_zwei_frager_der_dateiablage_rufen_dieselbe_regel() {
        let zuhause = "krk-ui/src/kommandos/zulaessigkeit.rs";
        let name = concat!("dateiablage_", "zulaessig");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| crate::quellbaum::aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 2,
            "die Dateiablage hat nicht die zwei Frager Ausgrauung und Antwortkette"
        );
    }

    /// Die Dateiablage wirkt genau mit dem Fokus im Dateifenster, und jedes
    /// Hindernis der Lage weist sie ab (C4.1 bis C4.4 der Runde 22,
    /// Probenhaelften).
    ///
    /// Ueber [`Fokus::ALLE`] und nicht ueber eine zweite Liste, aus dem Grund
    /// bei [`JEDER_FOKUS`]. Die Hindernisse sind die sieben aus
    /// [`HINDERNISSE`]: jede Kombination, in der ein Blatt steht, der
    /// Ersthelfer AppKit gehoert oder das Schluesselfenster fremd ist, weist
    /// ab, in jedem Fokus, denn die Dateiablage steht auf keiner
    /// Ausnahmeliste.
    ///
    /// Seit der Runde 21 haelt die Tafel auch das Einfuegen (C3.2, C3.4, C3.5,
    /// Probenhaelften): `paste:` fragt denselben Eingang mit demselben
    /// Anspruch, also ist jede Zeile hier zugleich seine.
    #[test]
    fn die_dateiablage_wirkt_genau_mit_dem_fokus_im_dateifenster() {
        let (blatt, appkit, krk) = OHNE_HINDERNIS;
        for fokus in JEDER_FOKUS {
            assert_eq!(
                dateiablage_zulaessig(lage(blatt, appkit, krk, fokus)),
                fokus == Fokus::Dateifenster,
                "die Dateiablage antwortet mit dem Fokus {fokus:?} falsch"
            );
        }
        for (blatt, appkit, krk) in HINDERNISSE {
            for fokus in JEDER_FOKUS {
                assert!(
                    !dateiablage_zulaessig(lage(blatt, appkit, krk, fokus)),
                    "die Dateiablage kommt bei Blatt={blatt}, AppKit={appkit}, \
                     KRK={krk} mit dem Fokus {fokus:?} durch"
                );
            }
        }
    }

    /// Die Aufzaehlung der Pruefungen ist die des Programms.
    ///
    /// Dieselbe Begruendung wie in [`super::super::fokus`]: eine zweite Liste
    /// derselben Werte pruefte womoeglich eine andere Menge als die, ueber
    /// die das Programm laeuft.
    const JEDER_FOKUS: [Fokus; 6] = Fokus::ALLE;

    /// Ein Befehl je Wirkungsbereich, und keiner von ihnen steht auf der
    /// Ausnahmeliste oder kommt waehrend eines Blattes durch.
    ///
    /// Die Tafel unten braucht zu jedem der acht Wirkungsbereiche ein
    /// Kommando, denn [`zulaessig`] fragt nach einem Kommando und nicht nach
    /// einem Bereich. Die Paarung ist nicht behauptet: die Probe
    /// `jeder_stellvertreter_traegt_den_bereich_den_er_vertritt` haelt sie gegen
    /// [`Kommando::wirkungsbereich`], und sie haelt daneben fest, dass keiner
    /// der acht eine der beiden Ausnahmen traegt. Ohne das zweite koennte ein
    /// Stellvertreter die drei abweisenden Viertel der Tafel gruen faerben,
    /// ohne dass die Regel sie traegt.
    ///
    /// **Die Feldbreite haelt den Bau nicht an, wenn ein Wert dazukommt**: hier
    /// steht kein `match`, und ein neunter Wirkungsbereich ohne Zeile bliebe
    /// von der Tafel unbemerkt. Was ihn faengt, ist die Tafel in
    /// [`super::super::fokus`], die als `match` ohne Auffangzweig uebersetzt
    /// wird, und die Probe `jeder_wirkungsbereich_hat_einen_stellvertreter`
    /// darunter, die die Zahl der Zeilen gegen die Aufzaehlung im Quelltext
    /// haelt.
    const STELLVERTRETER: [(Wirkungsbereich, Kommando); 8] = [
        (Wirkungsbereich::Dateifenster, Kommando::Oeffnen),
        (Wirkungsbereich::Leiste, Kommando::LesezeichenLoeschen),
        (Wirkungsbereich::Dateibereiche, Kommando::EditorRundweg),
        (Wirkungsbereich::Editor, Kommando::EditorSichern),
        (Wirkungsbereich::Tabbereich, Kommando::TabNeu),
        (Wirkungsbereich::Navigator, Kommando::AuswahlHoch),
        (Wirkungsbereich::Vorschau, Kommando::VorschauVergroessern),
        (Wirkungsbereich::Ueberall, Kommando::LeisteUmschalten),
    ];

    /// Jede Variante von [`Wirkungsbereich`] hat genau einen Stellvertreter.
    ///
    /// Die Varianten kommen aus dem Quelltext der Aufzaehlung und nicht aus
    /// [`STELLVERTRETER`]: eine Probe, die ueber das Feld laeuft, kann die
    /// Vollstaendigkeit des Feldes nicht halten. Gelesen wird der Block
    /// `pub enum Wirkungsbereich` in `belegung.rs` ueber [`quelldateien`], nach
    /// der Lesart von `varianten_der_aufzaehlung` in
    /// `krk-core/tests/gemeinsam`; jene Fassung erreicht diese Kiste nicht,
    /// weil `krk-ui` kein Bibliotheksziel hat, und die Aufzaehlung traegt
    /// keine Variante mit Daten, also genuegt die Zeile bis zum Komma.
    #[test]
    fn jeder_wirkungsbereich_hat_einen_stellvertreter() {
        let quellen = quelldateien();
        let (_, inhalt) = quellen
            .iter()
            .find(|(pfad, _)| pfad == "krk-core/src/tasten/belegung.rs")
            .expect("unter crates/ steht keine belegung.rs");
        let varianten: Vec<&str> = inhalt
            .lines()
            .skip_while(|zeile| *zeile != "pub enum Wirkungsbereich {")
            .skip(1)
            .take_while(|zeile| *zeile != "}")
            .map(str::trim)
            .filter(|zeile| {
                !zeile.is_empty() && !zeile.starts_with("//") && !zeile.starts_with("#[")
            })
            .map(|zeile| zeile.trim_end_matches(','))
            .collect();
        assert!(
            !varianten.is_empty(),
            "die Aufzaehlung ist nicht gefunden worden"
        );
        for name in &varianten {
            let zeilen = STELLVERTRETER
                .iter()
                .filter(|(bereich, _)| format!("{bereich:?}") == *name)
                .count();
            assert_eq!(
                zeilen, 1,
                "Wirkungsbereich::{name} hat {zeilen} Stellvertreter und nicht genau einen"
            );
        }
        assert_eq!(
            varianten.len(),
            STELLVERTRETER.len(),
            "die Stellvertreter zaehlen anders als die Aufzaehlung"
        );
    }

    /// Die Lage aus vier Werten, kurz geschrieben.
    ///
    /// Die Reihenfolge ist die der Felder: Blattstand, Ersthelferbefund,
    /// Schluesselfenster, Fokus.
    fn lage(
        blatt_steht: bool,
        ersthelfer_gehoert_appkit: bool,
        schluesselfenster_gehoert_krk: bool,
        fokus: Fokus,
    ) -> Lage {
        Lage {
            blatt_steht,
            ersthelfer_gehoert_appkit,
            schluesselfenster_gehoert_krk,
            fokus,
        }
    }

    /// Die eine Lage ohne jedes Hindernis: kein Blatt, ein Ersthelfer, der
    /// nicht AppKit gehoert, und KRKs eigenes Schluesselfenster.
    ///
    /// Die Reihenfolge ist die von [`lage`], ohne den Fokus.
    const OHNE_HINDERNIS: (bool, bool, bool) = (false, false, true);

    /// Die sieben Lagen, in denen mindestens eine der drei aufhebbaren
    /// Bedingungen einen Befehl aufhaelt.
    ///
    /// Alle acht Wahrheitskombinationen aus Blattstand, Ersthelferbefund und
    /// Schluesselfenster **ausser** [`OHNE_HINDERNIS`]. Die Liste steht
    /// ausgeschrieben und nicht gerechnet, aus demselben Grund wie die Tafel:
    /// eine gerechnete Menge waere die Umsetzung ein zweites Mal.
    const HINDERNISSE: [(bool, bool, bool); 7] = [
        (false, false, false),
        (false, true, false),
        (false, true, true),
        (true, false, false),
        (true, false, true),
        (true, true, false),
        (true, true, true),
    ];

    #[test]
    fn jeder_stellvertreter_traegt_den_bereich_den_er_vertritt() {
        for (bereich, kommando) in STELLVERTRETER {
            assert_eq!(
                kommando.wirkungsbereich(),
                bereich,
                "{kommando:?} vertritt {bereich:?} nicht"
            );
            assert!(
                !immer_erreichbar(kommando),
                "{kommando:?} steht auf der Ausnahmeliste und taugt nicht als Stellvertreter"
            );
            assert!(
                !operationen::waehrend_blatt_erlaubt(kommando),
                "{kommando:?} kommt waehrend eines Blattes durch und taugt nicht als \
                 Stellvertreter"
            );
        }
    }

    /// Die ganze Regel auf einen Blick: acht Wirkungsbereiche mal sechs
    /// Fokuswerte mal zwei Blattstaende mal zwei Ersthelferbefunde mal zwei
    /// Schluesselfensterbefunde.
    ///
    /// **Die Zahl der Faelle steht nicht mehr im Namen, sondern wird
    /// gerechnet.** Sie stand bis zur Git-Runde auf 320 und ist mit dem
    /// sechsten Fokuswert falsch geworden; die Zusicherung am Ende haelt die
    /// gezaehlten Faelle gegen das Produkt der drei Aufzaehlungen, statt gegen
    /// eine Zahl im Quelltext.
    ///
    /// Die Tafel steht in der Form der Tafel aus [`super::super::fokus`], nur um
    /// drei Wahrheitswerte erweitert. Ein Achtel traegt die Zeilen des
    /// Fokusvorbehalts, die sieben uebrigen weisen jeden Befehl ab: ein Blatt
    /// haelt alles auf, was nicht waehrend eines Blattes erlaubt ist, ein
    /// Ersthelfer, der AppKit gehoert, ebenso, und ein fremdes
    /// Schluesselfenster ebenso. Dass die sieben Achtel wirklich leer sind und
    /// nicht bloss so aussehen, haengt an den Stellvertretern; die Probe
    /// darueber haelt ihre beiden Voraussetzungen fest.
    ///
    /// Die Pruefungen darunter zeigen einzelne Felder dieser Tafel mit ihrer
    /// Begruendung; die Tafel zeigt, dass keine Zeile und keine Spalte fehlt.
    #[test]
    fn die_tafel_aus_allen_faellen_geht_auf() {
        // Eine Zeile je Wirkungsbereich; die Spalten stehen in der Reihenfolge
        // von JEDER_FOKUS: Dateifenster, Leiste, Vorschau, Editor, Git,
        // Anderswo. Es ist dieselbe Tafel, die `fokus::wirkt` traegt, und sie
        // steht hier ausgeschrieben und nicht gerechnet: eine gerechnete
        // Erwartung waere die Umsetzung ein zweites Mal.
        //
        // **Die sechste Spalte steht von Hand da und faellt nicht aus `zip`
        // an**: `JEDER_FOKUS.into_iter().zip(zeile)` bricht bei der kuerzeren
        // Seite ab, und fuenfspaltige Zeilen liessen den sechsten Fokuswert
        // ungeprueft. Die Zusicherung unter der Tafel haelt die Spaltenzahl
        // gegen `Fokus::ALLE`.
        const OHNE_SPERRE: [[bool; 6]; 8] = [
            [true, false, false, false, false, false],
            [false, true, false, false, false, false],
            [true, false, true, true, false, false],
            [false, false, false, true, false, false],
            [true, false, true, false, false, false],
            [true, true, true, false, true, false],
            [false, false, true, false, false, false],
            [true, true, true, true, true, true],
        ];
        const ALLES_ABGEWIESEN: [[bool; 6]; 8] = [[false; 6]; 8];

        // blatt_steht, ersthelfer_gehoert_appkit, schluesselfenster_gehoert_krk,
        // und welches Achtel gilt.
        let achtel: [(bool, bool, bool, [[bool; 6]; 8]); 8] = [
            (false, false, true, OHNE_SPERRE),
            (false, false, false, ALLES_ABGEWIESEN),
            (false, true, true, ALLES_ABGEWIESEN),
            (false, true, false, ALLES_ABGEWIESEN),
            (true, false, true, ALLES_ABGEWIESEN),
            (true, false, false, ALLES_ABGEWIESEN),
            (true, true, true, ALLES_ABGEWIESEN),
            (true, true, false, ALLES_ABGEWIESEN),
        ];

        let mut geprueft = 0usize;
        for (blatt, ersthelfer, schluessel, tafel) in achtel {
            for ((_, kommando), zeile) in STELLVERTRETER.into_iter().zip(tafel) {
                assert_eq!(
                    zeile.len(),
                    Fokus::ALLE.len(),
                    "die Zeile {kommando:?} hat nicht so viele Spalten wie Fokus::ALLE Werte \
                     fuehrt"
                );
                for (fokus, erwartet) in JEDER_FOKUS.into_iter().zip(zeile) {
                    assert_eq!(
                        zulaessig(kommando, lage(blatt, ersthelfer, schluessel, fokus)),
                        erwartet,
                        "{kommando:?} bei blatt={blatt} ersthelfer={ersthelfer} \
                         schluessel={schluessel} in {fokus:?}"
                    );
                    geprueft += 1;
                }
            }
        }
        assert_eq!(
            geprueft,
            achtel.len() * STELLVERTRETER.len() * Fokus::ALLE.len(),
            "die Tafel deckt nicht jeden Fall ab"
        );
    }

    /// Mit dem Fokus im Editor ist ein Befehl des Dateifensters unzulaessig.
    ///
    /// Der Bestandteil (3) allein, ohne Blatt und ohne Textfeld: `return` liegt
    /// auf [`Kommando::Oeffnen`], und mit der Schreibmarke im Editor soll es
    /// einen Zeilenumbruch setzen und keine Datei an das Standardprogramm
    /// uebergeben.
    #[test]
    fn ein_befehl_des_dateifensters_wirkt_im_editor_nicht() {
        assert!(!zulaessig(
            Kommando::Oeffnen,
            lage(false, false, true, Fokus::Editor)
        ));
        assert!(zulaessig(
            Kommando::Oeffnen,
            lage(false, false, true, Fokus::Dateifenster)
        ));
    }

    /// Beim Umbenennen direkt in der Liste ist derselbe Befehl unzulaessig,
    /// obwohl kein Blatt steht und der Fokus im Dateifenster liegt.
    ///
    /// **Der Fall, um dessentwillen die Regel drei Bestandteile hat.** Der
    /// Feldeditor der Namensspalte haelt den Ersthelferrang, es steht kein
    /// Blatt, und `Anwendungsdelegierter::fokus` antwortet `Dateifenster`. Ohne
    /// Bestandteil (2) waere hier jeder Befehl des Dateifensters freigegeben.
    #[test]
    fn beim_umbenennen_in_der_liste_wirkt_kein_befehl_des_dateifensters() {
        let umbenennung = lage(false, true, true, Fokus::Dateifenster);
        for kommando in [
            Kommando::Oeffnen,
            Kommando::AuswahlHoch,
            Kommando::AuswahlRunter,
            Kommando::MarkierungUmschalten,
            Kommando::FensterWechseln,
        ] {
            assert!(
                !zulaessig(kommando, umbenennung),
                "{kommando:?} kommt waehrend einer Umbenennung in der Liste durch"
            );
        }
    }

    /// Vor einem fremden Schluesselfenster wirkt auch ein fensterweiter Befehl
    /// nicht (C5.6).
    ///
    /// **Die eine Probe, die den Unterschied zwischen der Regel der Runde 7 und
    /// der dieser Runde zeigt.** Vor einem fremden Fenster antwortet
    /// `Anwendungsdelegierter::fokus` schon vorher [`Fokus::Anderswo`], und
    /// damit weist Bestandteil (3) jeden Befehl ab, dessen Wirkungsbereich ein
    /// Bereich ist. Uebrig bleibt allein die Zeile `Ueberall`, fuer die
    /// [`fokus::wirkt`](super::super::fokus::wirkt) in jedem Fokus ja sagt;
    /// dort und nur dort faellt der vierte Bestandteil ueberhaupt an. Ohne
    /// diese Probe zeigte keine der uebrigen ihn.
    ///
    /// Der Stellvertreter ist [`Kommando::LeisteUmschalten`], derselbe, den die
    /// Tafel fuer `Ueberall` fuehrt: er traegt den Bereich und steht nicht auf
    /// der Ausnahmeliste. Die zweite Zusicherung haelt fest, dass die neue
    /// Bedingung nichts anderes wegnimmt: vor KRKs eigenem Schluesselfenster
    /// kommt derselbe Befehl in jedem Fokus durch.
    #[test]
    fn vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl() {
        let kommando = Kommando::LeisteUmschalten;
        assert_eq!(kommando.wirkungsbereich(), Wirkungsbereich::Ueberall);
        assert!(!immer_erreichbar(kommando));

        for fokus in JEDER_FOKUS {
            assert!(
                !zulaessig(kommando, lage(false, false, false, fokus)),
                "{kommando:?} kommt vor einem fremden Schluesselfenster in {fokus:?} durch"
            );
            assert!(
                zulaessig(kommando, lage(false, false, true, fokus)),
                "{kommando:?} kommt vor KRKs eigenem Schluesselfenster in {fokus:?} nicht durch"
            );
        }
    }

    /// Ohne Schluesselfenster kommt `fenster_einblenden` durch.
    ///
    /// **Der Rueckweg, und das Gegenstueck zu
    /// [`vor_einem_fremden_schluesselfenster_wirkt_kein_fensterweiter_befehl`].**
    /// Nach Shift+Cmd+W ist das Fenster ausgeordnet, `keyWindow()` liefert
    /// nichts, und die Lage traegt darum denselben Wert wie vor einem fremden
    /// Fenster. Bestandteil (4) wiese Cmd+N damit ab, und der Nutzer kaeme an
    /// sein Fenster nur noch ueber das Dock-Symbol
    /// (`issues/260813-1258_*_fenster-einblenden-ist-nach-dem-schliessen-des-fensters-nicht-mehr-erreichbar.md`).
    ///
    /// Die erste Zusicherung nennt den Weg, ueber den der Befehl durchkommt:
    /// ohne die Ausnahmeliste bliebe die Schleife darunter rot, und mit einem
    /// engeren Wirkungsbereich waere sie es ebenfalls.
    #[test]
    fn ohne_schluesselfenster_kommt_fenster_einblenden_durch() {
        let kommando = Kommando::FensterEinblenden;
        assert!(immer_erreichbar(kommando));
        assert_eq!(kommando.wirkungsbereich(), Wirkungsbereich::Ueberall);

        for fokus in JEDER_FOKUS {
            assert!(
                zulaessig(kommando, lage(false, false, false, fokus)),
                "{kommando:?} kommt ohne Schluesselfenster in {fokus:?} nicht durch"
            );
        }
    }

    /// `beenden` und `fenster_schliessen` bleiben in jeder Lage erreichbar.
    ///
    /// Die Pruefung der Ausnahmeliste und zugleich ihre Herleitung: heute
    /// erreichen beide Befehle waehrend einer Umbenennung und waehrend eines
    /// Blattes ihr Ziel ueber den Menueeintrag, und die neue Regel naehme ihnen
    /// das ohne Ausnahme weg.
    ///
    /// **Der vierte Bestandteil steht seit der Runde 8 mit in der Schleife**,
    /// und die Probe haelt damit die Wahl aus
    /// `decisions/260813-1110_*_hebt-die-ausnahmeliste-auch-die-neue-schluesselfensterfrage-auf.md`
    /// fest: `beenden` kommt auch vor einem fremden Schluesselfenster durch,
    /// also beendet Cmd+Q die Anwendung, waehrend der Ueber-Dialog steht.
    /// Ohne diese Zeile waere der Weg lautlos weg.
    #[test]
    fn die_ausnahmeliste_kommt_durch_blatt_und_textfeld() {
        for kommando in [Kommando::Beenden, Kommando::FensterSchliessen] {
            assert!(immer_erreichbar(kommando));
            for fokus in JEDER_FOKUS {
                for (blatt, ersthelfer, schluessel) in HINDERNISSE {
                    assert!(
                        zulaessig(kommando, lage(blatt, ersthelfer, schluessel, fokus)),
                        "{kommando:?} kommt bei blatt={blatt} ersthelfer={ersthelfer} \
                         schluessel={schluessel} in {fokus:?} nicht durch"
                    );
                }
            }
        }
    }

    /// Die Ausnahmeliste hebt den Fokusvorbehalt **nicht** auf.
    ///
    /// **Was die Zusage genau sagt:** fuer einen Befehl auf der Liste
    /// entscheidet allein der dritte Bestandteil, und zwar in jeder Lage. Die
    /// Erwartung steht deshalb als [`fokus::wirkt`](super::super::fokus::wirkt)
    /// da und nicht als `true`.
    ///
    /// **Warum sie so und nicht mit einem Gegenbeispiel geprueft ist:** die
    /// Liste geht ueber [`Kommando`], und alle drei heutigen Eintraege tragen
    /// `Wirkungsbereich::Ueberall`, fuer das `wirkt` in jedem Fokus ja sagt. Ein
    /// Befehl mit einem engeren Bereich laesst sich nicht dazuerfinden. Traegt
    /// ein kuenftiger dritter Eintrag einen engeren Bereich, faellt der
    /// Unterschied an, und diese Probe misst ihn dann; heute haelt sie fest,
    /// dass die Rechnung ihn ueberhaupt stellt.
    #[test]
    fn die_ausnahmeliste_hebt_den_fokusvorbehalt_nicht_auf() {
        for kommando in Kommando::KENNUNGEN.map(|(kommando, _)| kommando) {
            if !immer_erreichbar(kommando) {
                continue;
            }
            for fokus in JEDER_FOKUS {
                for (blatt, ersthelfer, schluessel) in
                    HINDERNISSE.into_iter().chain([OHNE_HINDERNIS])
                {
                    assert_eq!(
                        zulaessig(kommando, lage(blatt, ersthelfer, schluessel, fokus)),
                        fokus::wirkt(kommando.wirkungsbereich(), fokus),
                        "{kommando:?} bei blatt={blatt} ersthelfer={ersthelfer} \
                         schluessel={schluessel} in {fokus:?} haengt nicht allein am \
                         Wirkungsbereich"
                    );
                }
            }
        }
    }

    /// Waehrend eines Blattes kommt der Abbruch durch, und sonst nur die
    /// Ausnahmeliste.
    ///
    /// Der Durchgang geht ueber **alle** Kommandos und nicht ueber eine zweite
    /// Liste: die Zusage aus C2.7 handelt von jedem Eintrag des Menues.
    #[test]
    fn waehrend_eines_blattes_kommt_allein_der_abbruch_und_die_ausnahmeliste_durch() {
        let blatt = lage(true, false, true, Fokus::Anderswo);
        for kommando in Kommando::KENNUNGEN.map(|(kommando, _)| kommando) {
            let erwartet =
                operationen::waehrend_blatt_erlaubt(kommando) || immer_erreichbar(kommando);
            assert_eq!(
                zulaessig(kommando, blatt),
                erwartet,
                "{kommando:?} verhaelt sich waehrend eines Blattes anders als zugesagt"
            );
        }
    }

    /// Waehrend eines Blattes kommen genau diese vier durch, ausgeschrieben.
    ///
    /// **Die Zahl vier ist in diesem Baum bisher Prosa gewesen, und sie ist
    /// vier Prosastellen falsch geraten.** Die Nachbarin darueber prueft
    /// `zulaessig` gegen `waehrend_blatt_erlaubt(k) || immer_erreichbar(k)`,
    /// also gegen die beiden Quellen selbst; sie haelt, dass die Zusammenrechnung
    /// stimmt, und sagt nichts darueber, **wie viele** Kommandos das sind. Wer
    /// einen fuenften auf die Ausnahmeliste setzt, sieht sie gruen bleiben.
    ///
    /// Diese Probe schreibt die Liste aus. Sie ist damit die eine Stelle, an
    /// der die Zahl gemessen und nicht behauptet steht, und jede Prosastelle,
    /// die von „vier" spricht, hat hier ihren Beleg
    /// (`issues/260817-1302_*_zwei-weitere-stellen-tragen-die-verkuerzte-blattsperre-*.md`,
    /// `issues/260817-1419_*_ein-vierter-traeger-der-verkuerzten-blattsperre-*.md`).
    ///
    /// **Sie wird rot, wenn die Liste waechst oder schrumpft**, und der
    /// Fehlschlag nennt die Kommandos beim Namen statt einer Zahl. Sie ist
    /// ausdruecklich **keine** Aufruferzaehlung, die man durch Streichen eines
    /// Fragers wieder gruen bekommt: ein neuer Eintrag ist eine Erweiterung der
    /// Zulaessigkeit, und der Weg ins Gruene ist, ihn hier einzutragen und die
    /// Prosastellen mitzuziehen, die die Zahl nennen. Genau dieses Mitziehen ist
    /// viermal unterblieben.
    ///
    /// Der Fokus steht auf [`Fokus::Anderswo`], damit allein die Lage
    /// entscheidet: alle vier tragen `Wirkungsbereich::Ueberall`, und ein
    /// Kommando, das das eines Tages nicht mehr taete, faellt hier heraus und
    /// meldet sich.
    ///
    /// Die Zahl steht als eigene Zusicherung vor der Mitgliedschaft, in der
    /// Bauform von [`die_ausnahmeliste_fuehrt_dieselben_drei_befehle_wie_vor_dieser_runde`]
    /// darunter: die Laenge schliesst einen fuenften aus, die vier
    /// Zusicherungen darunter das Verschwinden eines der bekannten. Verglichen
    /// wird nicht mit einer festen Reihenfolge, denn die waere die von
    /// `Kommando::KENNUNGEN` und sagt ueber die Zulaessigkeit nichts.
    ///
    /// **Seit der Runde 22 steht der zweite Eingang daneben**: die Dateiablage
    /// ist kein Kommando und kommt in `KENNUNGEN` nicht vor, also haelt die
    /// Zaehlung sie nicht. Die letzte Zusicherung fragt sie deshalb eigens und
    /// erwartet die Abweisung (C4.2): die Liste bleibt bei vier. Das Einfuegen
    /// der Runde 21 erweitert sie nicht (C3.2): es geht durch denselben
    /// Eingang, und die Abweisung gilt ihm mit.
    #[test]
    fn waehrend_eines_blattes_kommen_genau_diese_vier_durch() {
        let blatt = lage(true, false, true, Fokus::Anderswo);
        assert!(
            !dateiablage_zulaessig(blatt),
            "die Dateiablage kommt waehrend eines Blattes durch"
        );
        assert!(
            !dateiablage_zulaessig(lage(true, false, true, Fokus::Dateifenster)),
            "die Dateiablage kommt waehrend eines Blattes mit dem Fokus im Dateifenster durch"
        );
        let durchgelassen: Vec<Kommando> = Kommando::KENNUNGEN
            .into_iter()
            .map(|(kommando, _)| kommando)
            .filter(|kommando| zulaessig(*kommando, blatt))
            .collect();

        assert_eq!(
            durchgelassen.len(),
            4,
            "waehrend eines Blattes kommen nicht mehr vier Kommandos durch, sondern \
             {durchgelassen:?}; wer die Liste aendert, zieht die Prosastellen mit, \
             die ihre Laenge nennen"
        );
        for kommando in [
            Kommando::Abbrechen,
            Kommando::Beenden,
            Kommando::FensterSchliessen,
            Kommando::FensterEinblenden,
        ] {
            assert!(
                durchgelassen.contains(&kommando),
                "{kommando:?} kommt waehrend eines Blattes nicht mehr durch"
            );
        }
        // C2.11 der Runde 23: die beiden Befehle des Git-Bereichs sind bei
        // stehendem Blatt abgewiesen. Sie tragen `Wirkungsbereich::Ueberall`
        // wie die fuenf anderen Bereichsumschalter und die vier anderen
        // Fokusbefehle, und `immer_erreichbar` ist fuer sie ausdruecklich
        // **nicht** gewachsen: ein Bereich, der sich hinter einer stehenden
        // Rueckfrage ein- und ausblenden liesse, waere die Ausnahme ohne Grund.
        for kommando in [Kommando::GitBereichUmschalten, Kommando::FokusGit] {
            assert!(
                !durchgelassen.contains(&kommando),
                "{kommando:?} kommt waehrend eines Blattes durch"
            );
        }
    }

    /// Steht die Schreibmarke im Textfeld eines Blattes, ist auch der Abbruch
    /// abgewiesen.
    ///
    /// Die zweite Haelfte von C2.7: `esc` erreicht dann AppKit wie heute und
    /// schliesst das Blatt ueber dessen eigene Abbruchschaltflaeche.
    ///
    /// **An dieser Abweisung haengt der Notizzettel.** Seine Textflaeche haelt
    /// den Ersthelferrang und ist in `ersthelfer_gehoert_appkit` **nicht** als
    /// Ausnahme angemeldet, also meldet die Lage hier `true`, der Abbruch ist
    /// abgewiesen, und `esc` geht unveraendert an AppKit. Erst dadurch kommt
    /// `cancelOperation:` beim Waechter des Zettels an und schliesst ihn. Waere
    /// diese zweite Zusicherung eines Tages `true`, schluckte KRK die Taste, und
    /// der Zettel haette keinen Weg zurueck — die Probe haelt damit eine
    /// Vorbedingung der Notizzettel-Runde und nicht nur eine Aussage ueber den
    /// Abbruch.
    #[test]
    fn im_textfeld_eines_blattes_ist_auch_der_abbruch_abgewiesen() {
        assert!(zulaessig(
            Kommando::Abbrechen,
            lage(true, false, true, Fokus::Anderswo)
        ));
        assert!(!zulaessig(
            Kommando::Abbrechen,
            lage(true, true, true, Fokus::Anderswo)
        ));
    }

    /// Steht der Zettel, kommt der Befehl nicht durch, der ihn geoeffnet hat.
    ///
    /// **Keine Luecke, sondern die Regel.** Der Notizzettelbefehl steht weder
    /// auf der Ausnahmeliste noch unter
    /// [`operationen::waehrend_blatt_erlaubt`](super::operationen::waehrend_blatt_erlaubt),
    /// und beides bleibt in dieser Runde ausdruecklich so. Die Folge ist die
    /// Zusage aus C1: ein zweiter Druck auf `f2` oder `cmd+k` schliesst den
    /// Zettel nicht und tut nichts; der Weg zurueck ist `esc`.
    ///
    /// Die drei Zusicherungen ueber der Schleife nennen die Herleitung, damit
    /// ein Fehlschlag sagt, **welche** der drei Voraussetzungen gewichen ist.
    /// Die Schleife selbst haelt daneben die Gegenrichtung fest: ohne Blatt
    /// wirkt der Befehl aus jedem Fokuswert, und genau dafuer traegt
    /// er [`Wirkungsbereich::Ueberall`].
    #[test]
    fn der_notizzettel_kommt_bei_stehendem_blatt_nicht_durch() {
        let kommando = Kommando::Notizzettel;
        assert_eq!(kommando.wirkungsbereich(), Wirkungsbereich::Ueberall);
        assert!(!immer_erreichbar(kommando));
        assert!(!operationen::waehrend_blatt_erlaubt(kommando));

        for fokus in JEDER_FOKUS {
            assert!(
                zulaessig(kommando, lage(false, false, true, fokus)),
                "der Notizzettel kommt ohne Blatt in {fokus:?} nicht durch"
            );
            assert!(
                !zulaessig(kommando, lage(true, false, true, fokus)),
                "der Notizzettel kommt bei stehendem Blatt in {fokus:?} durch"
            );
        }
    }

    /// Die Ausnahmeliste fuehrt nach der Notizzettel-Runde dieselben drei
    /// Befehle wie davor.
    ///
    /// **Der Durchgang geht ueber alle Kommandos und nicht ueber eine zweite
    /// Liste.** Eine ausgeschriebene Erwartung neben [`immer_erreichbar`] waere
    /// die Umsetzung ein zweites Mal; gezaehlt wird deshalb, welche Kommandos
    /// die Funktion selbst bejaht.
    ///
    /// Die Zahl steht als eigene Zusicherung vor der Mitgliedschaft, weil erst
    /// beides zusammen „genau diese drei" sagt: die Laenge schliesst einen
    /// vierten Eintrag aus, die drei Zusicherungen darunter das Verschwinden
    /// eines der bekannten.
    #[test]
    fn die_ausnahmeliste_fuehrt_dieselben_drei_befehle_wie_vor_dieser_runde() {
        let auf_der_liste: Vec<Kommando> = Kommando::KENNUNGEN
            .into_iter()
            .map(|(kommando, _)| kommando)
            .filter(|kommando| immer_erreichbar(*kommando))
            .collect();

        assert_eq!(
            auf_der_liste.len(),
            3,
            "die Ausnahmeliste fuehrt nicht mehr drei Befehle, sondern {auf_der_liste:?}"
        );
        for kommando in [
            Kommando::Beenden,
            Kommando::FensterSchliessen,
            Kommando::FensterEinblenden,
        ] {
            assert!(
                auf_der_liste.contains(&kommando),
                "{kommando:?} steht nicht mehr auf der Ausnahmeliste"
            );
        }
    }

    /// Die vier Tabbefehle aus C1 der Runde 2 wirken mit dem Fokus in der
    /// Vorschau (C1.6, Probenhaelfte).
    ///
    /// **Die Zulaessigkeitshaelfte der Anmeldung im Ereignisabgriff.** Seit die
    /// Textanzeige der Vorschau auswaehlbar ist, nimmt sie den Ersthelferrang;
    /// gehoerte er AppKit, waere `ersthelfer_gehoert_appkit` wahr, Bestandteil
    /// (2) wiese ab, und mit dem Fokus in der Vorschau wirkte keiner der vier.
    /// Die Probe rechnet genau die Lage, die die Anmeldung herstellt: kein
    /// Blatt, KRKs eigenes Schluesselfenster, [`Fokus::Vorschau`] und ein
    /// Ersthelfer, der nicht AppKit gehoert.
    ///
    /// **Die zweite Zusicherung ist die eigentliche Aussage.** Ohne sie bliebe
    /// offen, ob die vier auch ohne die Anmeldung durchkaemen; mit ihr zeigt
    /// die Probe, dass allein der Ersthelferbefund den Unterschied macht. Sie
    /// ist damit die Probe, die rot wird, wenn die Anmeldung wieder faellt.
    ///
    /// Die Tastendruecke selbst misst keine Probe, sondern der Buendellauf;
    /// `krk-ui` hat kein Bibliotheksziel, und eine Probe, die den Hauptfaden
    /// behauptet, ist der bekannte Defekt `issues/260810-1001_*`.
    #[test]
    fn die_vier_tabbefehle_wirken_mit_dem_fokus_in_der_vorschau() {
        let vorschau = lage(false, false, true, Fokus::Vorschau);
        let ohne_anmeldung = lage(false, true, true, Fokus::Vorschau);
        for kommando in [
            Kommando::TabNeu,
            Kommando::TabSchliessen,
            Kommando::TabNaechster,
            Kommando::TabVoriger,
        ] {
            assert!(
                zulaessig(kommando, vorschau),
                "{kommando:?} wirkt mit dem Fokus in der Vorschau nicht"
            );
            assert!(
                !zulaessig(kommando, ohne_anmeldung),
                "{kommando:?} kaeme auch ohne die Anmeldung der Textflaeche durch"
            );
        }
    }

    /// Pfeil hoch und Pfeil runter bleiben mit dem Fokus in der Vorschau
    /// zulaessig (C1.10, Probenhaelfte fuer die Zulaessigkeit).
    ///
    /// **Zulaessig heisst hier ausdruecklich nicht „bewegt etwas".** Beide
    /// tragen [`Wirkungsbereich::Navigator`], der die Vorschau seit der Runde 1
    /// mitfuehrt; sie werden von KRK entgegengenommen, von der Vorschau nicht
    /// ausgefuehrt und erreichen AppKit deshalb nicht. Genau daran haengt die
    /// Zusage: waeren sie unzulaessig, liefen sie an AppKit weiter, und die
    /// Schreibmarke der auswaehlbaren Textanzeige begaenne zu wandern. Der
    /// Verbrauch ist die andere Haelfte und wird am Buendel abgenommen.
    ///
    /// Der Nutzerentscheid dazu ist
    /// `shared/decisions/260819-2216_*_was-tun-pfeil-hoch-und-runter-in-der-auswaehlbaren-vorschau.md`.
    #[test]
    fn die_beiden_pfeiltasten_bleiben_in_der_vorschau_zulaessig() {
        let vorschau = lage(false, false, true, Fokus::Vorschau);
        for kommando in [Kommando::AuswahlHoch, Kommando::AuswahlRunter] {
            assert!(
                zulaessig(kommando, vorschau),
                "{kommando:?} ist mit dem Fokus in der Vorschau nicht mehr zulaessig"
            );
        }
    }

    /// Die drei Zoombefehle des PDF-Betrachters, kurz geschrieben.
    const ZOOMBEFEHLE: [Kommando; 3] = [
        Kommando::VorschauVergroessern,
        Kommando::VorschauVerkleinern,
        Kommando::VorschauAusgangsgroesse,
    ];

    /// Mit dem Fokus in der Vorschau sind die drei Zoombefehle zulaessig
    /// (C3.7 der Runde 20, Probenhaelfte).
    ///
    /// **Zulaessig heisst auch hier nicht „zoomt etwas".** Die Lage kennt
    /// keinen Inhalt, und die Regel fragt nicht danach (A6): mit dem Fokus in
    /// der Vorschau und ohne angezeigtes PDF werden die drei entgegengenommen
    /// und tun nichts. Daran haengt dieselbe Zusage wie bei den Pfeiltasten
    /// darueber: waeren sie unzulaessig, liefen `cmd+plus` und `cmd+minus` an
    /// AppKit weiter. Ohne die Anmeldung der Textflaeche kommen sie nicht
    /// durch, wie jeder andere Befehl.
    #[test]
    fn die_drei_zoombefehle_wirken_mit_dem_fokus_in_der_vorschau() {
        let vorschau = lage(false, false, true, Fokus::Vorschau);
        let ohne_anmeldung = lage(false, true, true, Fokus::Vorschau);
        for kommando in ZOOMBEFEHLE {
            assert!(
                zulaessig(kommando, vorschau),
                "{kommando:?} wirkt mit dem Fokus in der Vorschau nicht"
            );
            assert!(
                !zulaessig(kommando, ohne_anmeldung),
                "{kommando:?} kaeme auch ohne die Anmeldung der Textflaeche durch"
            );
        }
    }

    /// Mit dem Fokus im Dateifenster, in der Leiste oder im Editor sind die
    /// drei Zoombefehle unzulaessig (C3.5 der Runde 20, Probenhaelfte).
    ///
    /// Das ist die Zeile `Vorschau` der Tafel, einzeln und mit Begruendung:
    /// der Betrachter steht allein im Vorschaufenster, und ein Zoom mit dem
    /// Fokus anderswo haette keinen Gegenstand. Weil `validateMenuItem:`
    /// dieselbe Regel ruft, sind die drei Menueeintraege dort ausgegraut. Die
    /// Schriftgroesse von Editor und Textvorschau ruehrt keiner der drei an;
    /// das ist keine Frage der Zulaessigkeit, sondern der Ausfuehrung, und die
    /// gibt es fuer die drei allein am Betrachter.
    #[test]
    fn die_drei_zoombefehle_wirken_ausserhalb_der_vorschau_nicht() {
        for fokus in [
            Fokus::Dateifenster,
            Fokus::Leiste,
            Fokus::Editor,
            Fokus::Git,
            Fokus::Anderswo,
        ] {
            let anderswo = lage(false, false, true, fokus);
            for kommando in ZOOMBEFEHLE {
                assert!(
                    !zulaessig(kommando, anderswo),
                    "{kommando:?} kaeme mit dem Fokus in {fokus:?} durch"
                );
            }
        }
    }
}
