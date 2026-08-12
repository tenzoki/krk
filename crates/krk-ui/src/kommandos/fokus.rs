//! Der eine Fokusvorbehalt: wirkt dieses Kommando dort, wo der Nutzer gerade
//! steht (C5)?
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Wo der Fokus steht, liest
//! `Anwendungsdelegierter::fokus`; die Regel, was daraus folgt, steht hier und
//! ist ohne Fenster pruefbar.
//!
//! ```text
//!  Kommando ──> Wirkungsbereich (krk-core, je Befehl)  ─┐
//!                                                       ├──> wirkt()
//!  Fenster ───> Fokus            (krk-ui, je Augenblick)┘        │
//!                                            ausfuehren oder nicht
//! ```
//!
//! # Warum eine Regel und nicht fuenf Abfragen
//!
//! Bis Schritt 18 gab es genau einen fokussierbaren Bereich, das Dateifenster,
//! und genau eine Stelle, die danach fragte: die Loeschtasten aus C4 pruegen an
//! ihrer Aufrufstelle, ob der Fokus im Dateifenster steht. Mit der Leiste aus
//! C5 kommt ein zweiter Bereich, und die Frage wird fuer **jedes** Kommando
//! faellig. Vier oder fuenf handgeschriebene Abfragen an vier oder fuenf
//! Aufrufstellen waeren das Dickicht aus Sonderregeln, das die Maxime
//! "supersimpel" ausschliesst; die Antwort ist stattdessen eine Eigenschaft je
//! Kommando ([`Wirkungsbereich`]) und **eine** Abfrage in der Zuleitung. Die
//! Abfrage aus Schritt 16 ist darin aufgegangen und steht nicht daneben.
//!
//! # Was ein abgewiesenes Kommando tut
//!
//! Nichts, und es meldet nichts. Der Tastendruck geht unveraendert an AppKit
//! weiter, wie jeder, den die Belegung nicht kennt. Die drei Abnahmekriterien
//! aus C5 verlangen von `delete`, `right` und `lesezeichen_loeschen`
//! ausdruecklich nur, dass sie nichts tun; eine Meldung waere eine Sonderregel
//! mit eigenem Text, und sie muesste fuer jeden der rund fuenfzig Befehle
//! entscheiden, wann sie zu laut wird.
//!
//! # Die Gegenrichtung: was ein Fokusbefehl selbst tut
//!
//! Die drei Fokusbefehle aus C5 und C2 sind die Ausnahme von nichts, aber sie
//! stellen die Frage andersherum: nicht "wirkt dieser Befehl hier", sondern
//! "wohin fuehrt er, und steht der Bereich dort ueberhaupt auf dem Schirm".
//! [`holt_hervor`] beantwortet die zweite Haelfte, und zwar fuer alle drei mit
//! derselben Zeile.
//!
//! # Vier Zuordnungen zwischen Fokus und Bereich, und keine davon doppelt
//!
//! Seit C9 den Fokus sichtbar macht, wird die Beziehung zwischen
//! [`Fokus`] und [`Bereich`](crate::fenstermodell::Bereich) in beide
//! Richtungen gebraucht. Sie steht deshalb hier vollstaendig und nicht
//! verstreut bei den Aufrufern:
//!
//! ```text
//!  Fokus ──holt_hervor──────> Bereich?   was ein Fokusbefehl hervorholt
//!  Fokus ──bereich_mit_fokus─> Bereich?   wo dieser Fokuswert wohnt (mit aktiver Seite)
//!  Bereich ──in_bereich──────> Fokus      was gilt, wenn der Ersthelfer darin liegt
//!  Bereich ──rahmenrolle─────> Rahmenrolle  wie der Kasten aus C9 gefaerbt wird
//! ```
//!
//! Die vierte stuetzt sich auf die zweite, die zweite auf die erste; allein
//! die dritte steht fuer sich, und die Probe
//! `das_enthaltensein_und_das_hervorholen_kehren_einander_um` haelt sie gegen
//! die erste.

use krk_core::ablage::Fensterseite;
use krk_core::tasten::Wirkungsbereich;

use crate::fenstermodell::Bereich;

/// Wo der Eingabefokus steht.
///
/// Die Antwort der Oberflaeche auf den [`Wirkungsbereich`] des Kerns. Fuenf
/// Werte, und sie decken das Fenster vollstaendig ab: die beiden Dateilisten,
/// die Leiste, das Vorschaufenster, der eingebaute Editor, und alles uebrige.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fokus {
    /// In einer der beiden Dateilisten.
    Dateifenster,
    /// In der Lesezeichen- und Geraeteleiste (C5).
    Leiste,
    /// Im Vorschaufenster (C6), dem dritten fokussierbaren Bereich seit S19.
    ///
    /// Hierhin kommt der Fokus per Mausklick in die Inhaltsflaeche der
    /// Vorschau und seit dem Nutzerentscheid vom 260807 ueber den Tastenbefehl
    /// `fokus_vorschau`. Mit dem Fokus hier bedienen die vier Tabbefehle aus
    /// C1 die Vorschau-Tabs — drei von ihnen ueber
    /// [`Wirkungsbereich::Tabbereich`], `tab_schliessen` seit C4 der Runde 4
    /// ueber [`Wirkungsbereich::Ueberall`] und die Verzweigung beim
    /// Anwendungsdelegierten —, und die beiden Zwischenablage-Befehle aus C10
    /// loesen nichts aus, wie ihr Abnahmekriterium es verlangt.
    Vorschau,
    /// Im eingebauten Editor (C1 der Editor-Runde), dem fuenften
    /// fokussierbaren Bereich.
    ///
    /// Hierhin kommt der Fokus per Mausklick in die Textflaeche, ueber den
    /// Tastenbefehl `fokus_editor` und auf den beiden Einstiegswegen, die eine
    /// Datei oeffnen: F4 im Dateifenster und der Uebergang aus der Vorschau.
    ///
    /// **Der Wert steht neben [`Fokus::Anderswo`] und nicht darin**, obwohl der
    /// Ersthelfer in beiden Faellen ein Textsystem sein kann. Der Unterschied
    /// ist die Naemlichkeit und nicht die Art: das Textfeld eines Blattes gibt
    /// seinen Ersthelferrang an den Feldeditor ab, und der ist dieselbe Art wie
    /// die Textflaeche des Editors, aber nicht dasselbe Objekt.
    Editor,
    /// Irgendwo sonst: in einem Blatt oder in einem Textfeld.
    ///
    /// Ein Kommando, das einen Bereich braucht, wirkt hier nicht. Der Fall ist
    /// nicht theoretisch: vor der Rueckfrage des endgueltigen Loeschens ist das
    /// Panel des Blattes das Schluesselfenster, und ohne diesen Wert loeschte
    /// ein Delete davor in dem Ordner dahinter.
    Anderswo,
}

impl Fokus {
    /// Alle fuenf Fokuswerte, in einer festen Reihenfolge.
    ///
    /// **Die eine Aufzaehlung, und seit S43 nur noch fuer die Proben.** Sie
    /// entstand mit S17, weil die Fokusabfrage sie durchlief: statt drei
    /// Vergleiche von Hand zu reihen und alles uebrige auf
    /// [`Fokus::Dateifenster`] fallen zu lassen, ging sie ueber diese Liste.
    /// S43 hat die Abfrage auf die Enthaltensfrage umgestellt, und die laeuft
    /// ueber [`Bereich::ALLE`] statt hierueber: gefragt ist, in welchem der
    /// fuenf Teilbaeume der Ersthelfer liegt, und die Antwort darauf ist ein
    /// Bereich. Das Programm zaehlt die Fokuswerte damit nirgends mehr auf,
    /// und deshalb steht `#[cfg(test)]` daran statt eines `#[allow(dead_code)]`
    /// mit einer Ankuendigung.
    ///
    /// **Die Aufgabe, die geblieben ist, ist keine kleine.** Vier Proben gehen
    /// ueber diese Liste — die Tafel des Fokusvorbehalts, die fuenfzig Paare
    /// der Rahmenrolle und zwei weitere —, und ohne sie fuehrte jede von ihnen
    /// eine eigene Liste derselben fuenf Werte. Die Tafel pruefte dann
    /// womoeglich eine andere Menge als die, ueber die das Programm laeuft.
    ///
    /// **Die Feldbreite steht in der Typangabe.** Ein sechster Wert haelt
    /// damit den Bau der Proben an, wie die Feldbreite von
    /// [`Kommando::KENNUNGEN`](krk_core::tasten::Kommando::KENNUNGEN) es fuer
    /// die Befehle tut; die Aufzaehlung selbst erzwingt der Uebersetzer nicht.
    /// [`Fokus::Anderswo`] steht darin wie die uebrigen, denn genau bei ihm
    /// haben die Proben etwas festzuhalten: kein Bereich traegt dann die
    /// Anzeige.
    #[cfg(test)]
    pub const ALLE: [Fokus; 5] = [
        Fokus::Dateifenster,
        Fokus::Leiste,
        Fokus::Vorschau,
        Fokus::Editor,
        Fokus::Anderswo,
    ];
}

/// Wo der Eingabefokus nach dem Aufbau der Oberflaeche steht.
///
/// **Immer derselbe Wert, und er wird nicht gespeichert.** C7 zaehlt auf, was
/// Beenden und Neustart ueberstehen soll: Tabs, Ordner, Auswahl, Breiten,
/// Sichtbarkeit und Sortierung. Der Fokus steht nicht darin, und das ist die
/// richtige Wahl: er ist keine Einstellung, die der Nutzer trifft, sondern der
/// Ort, an dem er zuletzt getippt hat. Ein Start in die Leiste, weil die letzte
/// Handlung vor dem Beenden ein `shift+cmd+l` war, waere fuer den Nutzer nicht
/// vorhersagbar.
///
/// Welches der beiden Dateifenster den Fokus bekommt, sagt das aktive aus dem
/// Fenstermodell; **das** ueberlebt den Neustart, weil C7 es zusagt.
///
/// Bis zum 260805 setzte niemand den Fokus beim Start. Der Ersthelfer stand
/// dann auf der ersten Ansicht der Schluesselansichtskette, seit S18 also auf
/// der Leiste, und **kein** Befehl mit [`Wirkungsbereich::Dateifenster`] wirkte
/// bis zum ersten `shift+cmd+d`
/// (`issues/260805-1845_*_beim-start-liegt-der-fokus-in-der-leiste-und-nicht-im-dateifenster.md`).
pub const BEIM_START: Fokus = Fokus::Dateifenster;

/// Welchen Bereich ein Fokusbefehl hervorholt, bevor er den Fokus setzt.
///
/// `None` heisst: es ist nichts hervorzuholen. Der Nutzer hat am 260807
/// entschieden, dass `fokus_leiste` eine ausgeblendete Leiste **einblendet**,
/// statt stumm abzuweisen
/// (`decisions/260805-1730_*_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md`);
/// ausgeblendet bleibt der Fokus in einem Bereich, den niemand sieht, ohne
/// Rueckmeldung haengen, und der Nutzer haelt den Befehl fuer kaputt.
/// Ausblenden tut kein Fokusbefehl, dafuer bleiben die Befehle aus C7 — die
/// Asymmetrie selbst steht in [`Fenstermodell::einblenden`](crate::fenstermodell::Fenstermodell::einblenden)
/// und traegt dort schon `shift+f3` aus C10.
///
/// **Die Vorschau steht hier neben der Leiste, und das ist die Antwort, die
/// aus beiden Nutzerentscheiden vom 260807 zusammen folgt.** Gefragt worden
/// ist der Nutzer nur zur Leiste; die Regel, die er gewaehlt hat, redet aber
/// nicht von einer Taste, sondern davon, dass ein Fokusbefehl seinen Bereich
/// holt. Fuer die Vorschau davon abzuweichen hiesse, `shift+cmd+y` stumm
/// abzuweisen, waehrend `shift+f3` dasselbe Fenster hervorholt und
/// `shift+cmd+l` seine Leiste — drei Befehle auf denselben Randbereichen mit
/// zwei Antworten. Das ist der Sonderfall, den "supersimpel" ausschliesst.
///
/// **Der Editor steht aus demselben Grund daneben.** C1 der Editor-Runde sagt
/// zu, dass der Fokusbefehl einen ausgeblendeten Editor hervorholt, sofern er
/// eine Datei haelt. Die Bedingung "sofern er eine Datei haelt" steht **nicht**
/// hier, sondern beim Aufrufer: [`holt_hervor`] ist eine reine Zuordnung von
/// einem Fokusziel auf einen Bereich und kennt keinen Zustand.
///
/// Das aktive Dateifenster ist nie ausgeblendet: eines der beiden bleibt
/// stehen, und wird das aktive ausgeblendet, wandert die Aktivitaet auf das
/// andere
/// ([`Fenstermodell::umschalten`](crate::fenstermodell::Fenstermodell::umschalten)).
/// Beides gilt seit der Bereichsleisten-Runde fuer beide Dateifenster; bis
/// dahin liess sich das linke gar nicht ausblenden.
/// Fuer [`Fokus::Dateifenster`] ist deshalb nichts hervorzuholen, und
/// [`Fokus::Anderswo`] ist kein Ziel eines Befehls, sondern ein Befund.
pub const fn holt_hervor(ziel: Fokus) -> Option<Bereich> {
    match ziel {
        Fokus::Leiste => Some(Bereich::Lesezeichen),
        Fokus::Vorschau => Some(Bereich::Vorschau),
        Fokus::Editor => Some(Bereich::Editor),
        Fokus::Dateifenster | Fokus::Anderswo => None,
    }
}

/// Welcher Fokuswert gilt, wenn der Ersthelfer **in** diesem Bereich liegt.
///
/// Die Zuordnung, die `Anwendungsdelegierter::fokus` seit S43 liest. Sie
/// beantwortet die Enthaltensfrage und nicht die Naemlichkeitsfrage: gefragt
/// ist nicht, welche eine Ansicht den Ersthelferrang traegt, sondern in
/// welchem der fuenf Teilbaeume der Rang ueberhaupt liegt. Bis zum 260809
/// stand statt dessen ein Vergleich gegen fuenf genannte Ansichten, und jeder
/// Ersthelfer innerhalb eines Randbereichs, der nicht dessen genannte Ansicht
/// war — eine Bildlaufleiste etwa —, galt als Dateifenster
/// (`issues/260809-1738_*_der-rueckfall-in-fokus-antwortet-dateifenster-fuer-jede-unteransicht-eines-randbereichs.md`).
///
/// **Zwei Bereiche auf einen Wert, und das ist keine Ungenauigkeit.** Es gibt
/// fuenf Bereiche und vier fokussierbare Orte: die beiden Dateifenster teilen
/// sich [`Fokus::Dateifenster`], weil das Fenstermodell sagt, welches der
/// beiden gemeint ist, und weil jeder Befehl mit
/// [`Wirkungsbereich::Dateifenster`] fuer beide dieselbe Regel traegt.
///
/// **Vollstaendig und ohne Auffangzweig**, wie die uebrigen
/// Fallunterscheidungen ueber [`Bereich`]: ein sechster Bereich haelt hier den
/// Bau an und erzwingt seine Einordnung.
pub const fn in_bereich(bereich: Bereich) -> Fokus {
    match bereich {
        Bereich::Lesezeichen => Fokus::Leiste,
        Bereich::Links | Bereich::Rechts => Fokus::Dateifenster,
        Bereich::Vorschau => Fokus::Vorschau,
        Bereich::Editor => Fokus::Editor,
    }
}

/// In welchem Bereich ein Fokuswert wohnt.
///
/// **Die eine Zuordnung von einem Fokuswert auf seinen Bereich**, und drei
/// Aufrufer lesen sie: die Fokusanzeige aus C9 ueber [`rahmenrolle`], das
/// Hervorholen und die Sichtbarkeitssperre der Fokusbefehle, und die
/// Breitenaenderung aus C7. Bis zum 260809 rechnete jeder von ihnen
/// `holt_hervor(...).unwrap_or_else(|| Bereich::von_seite(aktiv))` fuer sich;
/// das war dieselbe Rechnung dreimal, und die Anzeige und die
/// Breitenaenderung haetten auseinanderlaufen koennen.
///
/// Die drei Randbereiche kommen aus [`holt_hervor`] und nicht aus einer
/// zweiten Aufzaehlung daneben. [`Fokus::Dateifenster`] liefert das **aktive**
/// Dateifenster: es gibt zwei Listen und einen Fokuswert, und welche der
/// beiden gemeint ist, sagt allein das Fenstermodell. [`Fokus::Anderswo`]
/// liefert `None`, denn ein stehendes Blatt gehoert keinem Bereich; was die
/// Aufrufer daraus machen, ist ihre Sache und verschieden: die Anzeige laesst
/// dann alles stehen, die Breitenaenderung faellt auf das aktive Dateifenster.
///
/// **Vollstaendig und ohne Auffangzweig.**
pub const fn bereich_mit_fokus(fokus: Fokus, aktiv: Fensterseite) -> Option<Bereich> {
    match fokus {
        Fokus::Dateifenster => Some(Bereich::von_seite(aktiv)),
        Fokus::Leiste | Fokus::Vorschau | Fokus::Editor => holt_hervor(fokus),
        Fokus::Anderswo => None,
    }
}

/// Was der Rahmen eines Bereichs im Augenblick aussagt (C9).
///
/// Drei Zustaende und nicht zwei, und der Grund steht im Spec unter C9: der
/// Akzentrahmen bekommt eine zweite Bedeutung. Bis zur Runde 2 hiess er "dies
/// ist das aktive Dateifenster", kuenftig heisst er "hier kommen deine Tasten
/// an". Beide Aussagen sind zu treffen, denn der Nutzer muss auch mit dem
/// Fokus im Editor sehen, aus welchem Dateifenster F5 kopiert.
///
/// **Die Vorbelegung des Specs, und eine andere Antwort aendert einen
/// Funktionsrumpf und keinen Aufbau.** Der Datensatz
/// `decisions/260809-2043_*_bedeutet-der-akzentrahmen-kuenftig-den-fokus-oder-das-aktive-dateifenster.md`
/// ist offen und haelt diesen Bau nicht auf. Waehlt der Nutzer die dritte
/// Moeglichkeit, den Rahmen allein fuer den Fokus, entfaellt [`Self::AktivOhneFokus`]
/// aus [`rahmenrolle`] und wird [`Self::Ruhig`]; die fuenf Kaesten, der
/// Ausloesepunkt und [`bereich_mit_fokus`] bleiben unberuehrt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rahmenrolle {
    /// Hier kommen die Tasten an.
    Fokussiert,
    /// Aus diesem Dateifenster kopiert F5, aber die Tasten kommen woanders an.
    AktivOhneFokus,
    /// Weder noch.
    Ruhig,
}

/// Welche Rolle der Rahmen dieses Bereichs traegt (C9).
///
/// **Die eine Stelle, die entscheidet, welcher Bereich wie eingerahmt wird**,
/// und sie ist reine Rechnung ohne AppKit. Welche Farbe eine Rolle bekommt,
/// steht in `super::super::appkit::aufteilung`; diese Funktion kennt keine
/// Farbe, und die Probe darunter deckt sie ohne Fenster ab.
///
/// Drei Zusagen, die die Probe `die_fuenfzig_paare_der_rahmenrolle_gehen_auf`
/// festhaelt: bei jedem Fokuswert ausser [`Fokus::Anderswo`] traegt genau ein
/// Bereich [`Rahmenrolle::Fokussiert`]; das aktive Dateifenster traegt nie
/// [`Rahmenrolle::Ruhig`]; und bei [`Fokus::Anderswo`], also bei einem
/// stehenden Blatt, traegt kein Bereich [`Rahmenrolle::Fokussiert`].
///
/// **Das siebte Abnahmekriterium von C9 faellt daraus an und wird nicht eigens
/// gebaut:** ein Blatt nimmt keinem Bereich seine Anzeige, weil der Aufrufer
/// bei [`Fokus::Anderswo`] gar nicht erst schreibt. Die Regel dazu steht beim
/// Aufrufer, nicht hier; diese Funktion beantwortet die Frage auch fuer
/// `Anderswo`, damit die Probe sie stellen kann.
///
/// Verglichen wird ueber [`Bereich::index`] und nicht mit `==`: `PartialEq`
/// ist nicht `const`, und diese Zuordnung soll zur Uebersetzungszeit
/// nachrechenbar bleiben wie ihre Nachbarn.
pub const fn rahmenrolle(bereich: Bereich, fokus: Fokus, aktiv: Fensterseite) -> Rahmenrolle {
    if let Some(mit_fokus) = bereich_mit_fokus(fokus, aktiv)
        && mit_fokus.index() == bereich.index()
    {
        return Rahmenrolle::Fokussiert;
    }
    if Bereich::von_seite(aktiv).index() == bereich.index() {
        return Rahmenrolle::AktivOhneFokus;
    }
    Rahmenrolle::Ruhig
}

/// Ob ein Kommando mit diesem Wirkungsbereich hier wirken darf.
///
/// Die eine Regel, und die eine Stelle, an der die beiden Halbwahrheiten
/// zusammenkommen: der Kern weiss, welchen Bereich ein Befehl braucht, die
/// Oberflaeche, welcher ihn gerade hat.
pub fn wirkt(bereich: Wirkungsbereich, fokus: Fokus) -> bool {
    match bereich {
        Wirkungsbereich::Ueberall => true,
        Wirkungsbereich::Dateifenster => fokus == Fokus::Dateifenster,
        Wirkungsbereich::Leiste => fokus == Fokus::Leiste,
        Wirkungsbereich::Vorschau => fokus == Fokus::Vorschau,
        Wirkungsbereich::Editor => fokus == Fokus::Editor,
        Wirkungsbereich::Tabbereich => {
            matches!(fokus, Fokus::Dateifenster | Fokus::Vorschau)
        }
        // Positiv aufgezaehlt und **nicht** als `fokus != Fokus::Editor`. Die
        // Verneinung liesse `Fokus::Anderswo` durch, und ein `up` vor der
        // Rueckfrage des endgueltigen Loeschens bewegte die Auswahl im Ordner
        // dahinter. Die Begruendung im Langen steht an
        // [`Wirkungsbereich::Navigator`].
        Wirkungsbereich::Navigator => {
            matches!(fokus, Fokus::Dateifenster | Fokus::Leiste | Fokus::Vorschau)
        }
    }
}

#[cfg(test)]
mod tests {
    use krk_core::ablage::Sitzung;
    use krk_core::tasten::Kommando;

    use crate::fenstermodell::{Fenstermodell, Zeilenmass};

    use super::*;

    /// Die Aufzaehlung der Pruefungen ist die des Programms.
    ///
    /// Bis zum 260809 stand hier eine zweite Liste derselben fuenf Werte.
    /// Seit [`Fokus::ALLE`] die Fokusabfrage traegt, waere sie eine zweite
    /// Wahrheit darueber, welche Werte es gibt, und die Tafel unten pruefte
    /// womoeglich eine andere Menge als die, ueber die das Programm laeuft.
    const JEDER_FOKUS: [Fokus; 5] = Fokus::ALLE;

    /// Die ganze Regel auf einen Blick: sieben Wirkungsbereiche mal fuenf
    /// Fokuswerte, fuenfunddreissig Paare.
    ///
    /// Die Pruefungen darunter zeigen jeweils eine Zeile dieser Tafel mit ihrer
    /// Begruendung; die Tafel zeigt, dass keine Zeile und keine Spalte fehlt.
    /// Sie ist die Stelle, an der ein sechster Fokuswert oder ein achter
    /// Wirkungsbereich auffaellt: beide Feldbreiten stehen in der Typangabe,
    /// und eine vergessene Zeile haelt den Bau an.
    #[test]
    fn die_tafel_aus_sieben_wirkungsbereichen_und_fuenf_fokuswerten_geht_auf() {
        // Eine Zeile je Wirkungsbereich; die Spalten stehen in der Reihenfolge
        // von JEDER_FOKUS: Dateifenster, Leiste, Vorschau, Editor, Anderswo.
        const TAFEL: [(Wirkungsbereich, [bool; 5]); 7] = [
            (
                Wirkungsbereich::Dateifenster,
                [true, false, false, false, false],
            ),
            (Wirkungsbereich::Leiste, [false, true, false, false, false]),
            (
                Wirkungsbereich::Vorschau,
                [false, false, true, false, false],
            ),
            (Wirkungsbereich::Editor, [false, false, false, true, false]),
            (
                Wirkungsbereich::Tabbereich,
                [true, false, true, false, false],
            ),
            (Wirkungsbereich::Navigator, [true, true, true, false, false]),
            (Wirkungsbereich::Ueberall, [true, true, true, true, true]),
        ];

        for (bereich, zeile) in TAFEL {
            for (fokus, erwartet) in JEDER_FOKUS.into_iter().zip(zeile) {
                assert_eq!(wirkt(bereich, fokus), erwartet, "{bereich:?} in {fokus:?}");
            }
        }
    }

    /// Der Unterschied zwischen `Navigator` und `Ueberall`, an dem Fokuswert,
    /// fuer den es ihn ueberhaupt gibt.
    ///
    /// Ohne [`Wirkungsbereich::Navigator`] traegt `auswahl_hoch` weiter
    /// [`Wirkungsbereich::Ueberall`], und ein `up` mit dem Fokus im Editor
    /// bewegte die Auswahl im Dateifenster statt der Schreibmarke. Der Umzug
    /// der drei betroffenen Befehle steht in S5; diese Pruefung sichert die
    /// Regel, auf die er sich stuetzt.
    #[test]
    fn der_navigator_endet_am_editor_und_ueberall_nicht() {
        assert!(!wirkt(Wirkungsbereich::Navigator, Fokus::Editor));
        assert!(wirkt(Wirkungsbereich::Ueberall, Fokus::Editor));
    }

    /// `Navigator` ist positiv aufgezaehlt und schliesst deshalb auch das
    /// stehende Blatt aus.
    ///
    /// Die Gegenprobe zur Verneinung: waere der Wert als "ueberall ausser im
    /// Editor" geschrieben, kaeme hier `true` heraus, und ein `up` vor der
    /// Rueckfrage des endgueltigen Loeschens bewegte die Auswahl im Ordner
    /// dahinter.
    #[test]
    fn der_navigator_schliesst_auch_das_stehende_blatt_aus() {
        assert!(!wirkt(Wirkungsbereich::Navigator, Fokus::Anderswo));
    }

    #[test]
    fn ein_befehl_ohne_vorbehalt_wirkt_in_jedem_bereich() {
        for fokus in JEDER_FOKUS {
            assert!(wirkt(Wirkungsbereich::Ueberall, fokus));
        }
    }

    #[test]
    fn ein_bereichsbefehl_wirkt_in_genau_einem_bereich() {
        for fokus in JEDER_FOKUS {
            assert_eq!(
                wirkt(Wirkungsbereich::Dateifenster, fokus),
                fokus == Fokus::Dateifenster
            );
            assert_eq!(
                wirkt(Wirkungsbereich::Leiste, fokus),
                fokus == Fokus::Leiste
            );
        }
    }

    /// Drei der vier Tabbefehle bedienen nach C6 auch die Vorschau-Tabs.
    ///
    /// Sie wirken in beiden Bereichen mit Tabs und nirgends sonst: nicht in
    /// der Leiste, die keine Tabs traegt, und nicht in einem Blatt oder
    /// Textfeld.
    ///
    /// **`tab_schliessen` steht seit C4 der Runde 4 nicht mehr bei ihnen.**
    /// Die vier sind damit keine Gruppe eines Wirkungsbereichs mehr; die Probe
    /// darunter haelt seinen getrennt fest, und diese hier haelt fest, dass
    /// die drei uebrigen den ihren behalten haben.
    #[test]
    fn ein_tabbefehl_wirkt_in_beiden_bereichen_mit_tabs() {
        for kommando in [
            Kommando::TabNeu,
            Kommando::TabNaechster,
            Kommando::TabVoriger,
        ] {
            assert_eq!(
                kommando.wirkungsbereich(),
                Wirkungsbereich::Tabbereich,
                "{kommando:?} ist kein Tabbefehl mehr"
            );
        }
        for fokus in JEDER_FOKUS {
            assert_eq!(
                wirkt(Wirkungsbereich::Tabbereich, fokus),
                matches!(fokus, Fokus::Dateifenster | Fokus::Vorschau)
            );
        }
    }

    /// `tab_schliessen` wirkt aus jedem Fokus (C4 der Runde 4).
    ///
    /// Die Zusage, die den Befehl aus dem Zweig der vier Tabbefehle geholt
    /// hat: er schliesst einen Tab und setzt deshalb keinen Bereich mit Tabs
    /// im Fokus voraus, sondern eine aktive Fensterseite, und die gibt es
    /// immer. [`Fokus::Leiste`] und [`Fokus::Editor`] sind die beiden Werte,
    /// um derentwillen der Umzug geschehen ist (Nutzerantwort vom
    /// 260811-1505); die Schleife ueber [`JEDER_FOKUS`] deckt sie mit ab und
    /// sagt daneben zu, dass keiner der uebrigen drei verlorengegangen ist.
    ///
    /// **Was diese Probe nicht zusagt, ist das stehende Blatt.** Der
    /// Fokusvorbehalt ist nicht die Stelle, die es anhaelt — das tut
    /// `Anwendungsdelegierter::kommando_ausfuehren` ueber
    /// `waehrend_blatt_erlaubt`, und dort bleibt `cmd+w` aussen vor.
    #[test]
    fn das_tab_schliessen_wirkt_aus_jedem_fokus() {
        assert_eq!(
            Kommando::TabSchliessen.wirkungsbereich(),
            Wirkungsbereich::Ueberall,
            "cmd+w soll den aktiven Tab aus jedem Fokus schliessen"
        );
        for fokus in JEDER_FOKUS {
            assert!(
                wirkt(Kommando::TabSchliessen.wirkungsbereich(), fokus),
                "cmd+w wirkt in {fokus:?} nicht"
            );
        }
    }

    /// Die beiden Zwischenablage-Befehle aus C10 brauchen den Fokus im
    /// Dateifenster.
    ///
    /// Das Abnahmekriterium von C10 nennt beide Gegenproben ausdruecklich: in
    /// der Leiste und im Vorschaufenster loesen sie nichts aus
    /// (Nutzerentscheid vom 260805-0000).
    #[test]
    fn die_zwischenablage_befehle_wirken_nur_im_dateifenster() {
        for kommando in [
            Kommando::ZwischenablageAnsehen,
            Kommando::ZwischenablageSpringen,
        ] {
            assert_eq!(
                kommando.wirkungsbereich(),
                Wirkungsbereich::Dateifenster,
                "{kommando:?} traegt nicht den entschiedenen Wirkungsbereich"
            );
            assert!(!wirkt(kommando.wirkungsbereich(), Fokus::Leiste));
            assert!(!wirkt(kommando.wirkungsbereich(), Fokus::Vorschau));
            assert!(wirkt(kommando.wirkungsbereich(), Fokus::Dateifenster));
        }
    }

    /// Die drei Faelle, die das Abnahmekriterium von C5 namentlich nennt,
    /// diesmal am Zusammenspiel beider Haelften.
    #[test]
    fn die_drei_faelle_aus_c5_gehen_auf() {
        assert!(!wirkt(
            Kommando::InPapierkorb.wirkungsbereich(),
            Fokus::Leiste
        ));
        assert!(!wirkt(Kommando::Oeffnen.wirkungsbereich(), Fokus::Leiste));
        assert!(!wirkt(
            Kommando::LesezeichenLoeschen.wirkungsbereich(),
            Fokus::Dateifenster
        ));
        // Und die Gegenprobe: in ihrem eigenen Bereich wirken sie.
        assert!(wirkt(
            Kommando::InPapierkorb.wirkungsbereich(),
            Fokus::Dateifenster
        ));
        assert!(wirkt(
            Kommando::LesezeichenLoeschen.wirkungsbereich(),
            Fokus::Leiste
        ));
    }

    /// Nach dem Start wirkt jeder Befehl des Dateifensters.
    ///
    /// Die Pruefung zu dem Defekt vom 260805-1845, und sie zaehlt die Befehle
    /// nicht auf, sondern geht ueber [`Kommando::KENNUNGEN`]: betroffen waren
    /// **alle** Befehle mit [`Wirkungsbereich::Dateifenster`], und ein Befehl,
    /// der spaeter dazukommt, ist es dann auch.
    #[test]
    fn nach_dem_start_wirkt_jeder_befehl_des_dateifensters() {
        let mut gezaehlt = 0;
        for (kommando, kennung) in Kommando::KENNUNGEN {
            if kommando.wirkungsbereich() != Wirkungsbereich::Dateifenster {
                continue;
            }
            gezaehlt += 1;
            assert!(
                wirkt(kommando.wirkungsbereich(), BEIM_START),
                "„{kennung}“ wirkt beim Start nicht"
            );
        }
        assert!(
            gezaehlt > 0,
            "kein Befehl traegt Wirkungsbereich::Dateifenster; die Pruefung liefe leer"
        );
    }

    /// Der Fokusbefehl in die Vorschau ist gebaut und aus jedem Bereich
    /// erreichbar (C2, C6).
    ///
    /// Die Lueckenschliessung aus dem Nutzerentscheid vom 260807: ohne ihn
    /// waeren die vier Tabbefehle aus C1 in den Vorschau-Tabs allein per Maus
    /// erreichbar. Geprueft wird beides, was ohne Fenster pruefbar ist — dass
    /// die Kennung zu einem Kommando fuehrt, und dass der Befehl aus jedem
    /// Fokus heraus wirkt.
    #[test]
    fn der_fokusbefehl_in_die_vorschau_wirkt_aus_jedem_bereich() {
        assert_eq!(
            Kommando::aus_kennung("fokus_vorschau"),
            Some(Kommando::FokusVorschau),
            "die Kennung aus der Belegungsdatei fuehrt nicht zum Kommando"
        );
        assert_eq!(
            Kommando::FokusVorschau.wirkungsbereich(),
            Wirkungsbereich::Ueberall
        );
        for fokus in JEDER_FOKUS {
            assert!(wirkt(Kommando::FokusVorschau.wirkungsbereich(), fokus));
        }
    }

    /// Jeder Fokusbefehl holt genau den Bereich hervor, in den er fuehrt.
    ///
    /// Der Nutzerentscheid vom 260807 zur Leiste, und dieselbe Zeile fuer die
    /// Vorschau: ohne das Hervorholen wiese der Befehl auf einen
    /// ausgeblendeten Bereich stumm ab, und der Nutzer hielte ihn fuer kaputt.
    /// Das Dateifenster hat nichts hervorzuholen, weil das aktive nie
    /// ausgeblendet ist.
    #[test]
    fn jeder_fokusbefehl_holt_seinen_bereich_hervor() {
        assert_eq!(holt_hervor(Fokus::Leiste), Some(Bereich::Lesezeichen));
        assert_eq!(holt_hervor(Fokus::Vorschau), Some(Bereich::Vorschau));
        assert_eq!(holt_hervor(Fokus::Editor), Some(Bereich::Editor));
        assert_eq!(holt_hervor(Fokus::Dateifenster), None);
        assert_eq!(holt_hervor(Fokus::Anderswo), None);
    }

    /// Der Fokusbefehl auf einen ausgeblendeten Bereich blendet ihn ein,
    /// statt stumm abzuweisen.
    ///
    /// Die beiden Haelften des Nutzerentscheids vom 260807 an einem Stueck,
    /// soweit sie ohne Fenster pruefbar sind: [`holt_hervor`] nennt den
    /// Bereich, [`Fenstermodell::einblenden`] holt ihn hervor, und danach ist
    /// die Bedingung erfuellt, unter der
    /// `Anwendungsdelegierter::fokus_setzen` den Fokus hineinlaesst. Das
    /// Setzen selbst braucht ein Fenster und steht deshalb nicht hier.
    #[test]
    fn ein_fokusbefehl_auf_einen_ausgeblendeten_bereich_blendet_ihn_ein() {
        for (ziel, bereich) in [
            (Fokus::Leiste, Bereich::Lesezeichen),
            (Fokus::Vorschau, Bereich::Vorschau),
        ] {
            // Eine Zeile, in die jede Menge sichtbarer Bereiche passt: die
            // Abweisung an den Mindestbreiten ist hier nicht der Gegenstand,
            // sie steht in `fenstermodell.rs` unter Probe.
            let weit = Zeilenmass {
                gesamt: 4000.0,
                trennerbreite: 0.0,
            };
            let mut modell = Fenstermodell::aus_sitzung(&Sitzung::default());
            assert!(
                modell.umschalten(bereich, weit),
                "{bereich:?} liess sich nicht ausblenden"
            );
            assert!(!modell.sichtbar(bereich), "die Probe beginnt ausgeblendet");

            let hervorzuholen = holt_hervor(ziel).expect("dieser Befehl holt einen Bereich hervor");
            assert_eq!(hervorzuholen, bereich);
            assert!(
                modell.einblenden(hervorzuholen, weit),
                "{ziel:?} weist den ausgeblendeten Bereich stumm ab, statt ihn hervorzuholen"
            );
            assert!(
                modell.sichtbar(bereich),
                "erst der sichtbare Bereich laesst den Fokus hinein"
            );
        }
    }

    /// [`Fokus::ALLE`] fuehrt jeden Wert genau einmal.
    ///
    /// Die Aufzaehlung traegt seit S17 die Fokusabfrage in
    /// `Anwendungsdelegierter::fokus`, und die laeuft sie einmal durch. Ein
    /// fehlender Wert machte den zugehoerigen Bereich wieder unsichtbar fuer
    /// die Abfrage — genau der Zustand, aus dem der Defekt vom 260809-1640
    /// bestand. Ein doppelter Wert kostete nur einen Vergleich, faellt hier
    /// aber mit auf.
    ///
    /// Die Feldbreite `[Fokus; 5]` haelt den Bau an, wenn ein sechster Wert
    /// dazukommt; diese Pruefung deckt die andere Haelfte ab, dass die fuenf
    /// Plaetze mit fuenf **verschiedenen** Werten belegt sind.
    #[test]
    fn die_aufzaehlung_der_fokuswerte_ist_vollstaendig_und_doppelt_keinen() {
        for wert in [
            Fokus::Dateifenster,
            Fokus::Leiste,
            Fokus::Vorschau,
            Fokus::Editor,
            Fokus::Anderswo,
        ] {
            assert_eq!(
                Fokus::ALLE.iter().filter(|&&x| x == wert).count(),
                1,
                "{wert:?} steht nicht genau einmal in Fokus::ALLE"
            );
        }
    }

    /// Das Abnahmekriterium von S17: mit dem Fokus im Editor wirkt kein
    /// Befehl des Dateifensters, und die Befehle des Fensters wirken weiter.
    ///
    /// Die Regel selbst steht seit S3 in [`wirkt`]; was bis zum 260809 fehlte,
    /// war die Gegenseite — `Anwendungsdelegierter::fokus` lieferte niemals
    /// [`Fokus::Editor`], und deshalb kam diese Zeile der Tafel im laufenden
    /// Programm nie zur Anwendung. Die Pruefung haelt fest, was S17 damit
    /// erreicht.
    ///
    /// Zwei Durchgaenge. Der erste nennt die Befehle, die das
    /// Abnahmekriterium namentlich fuehrt, mit ihrer Gegenprobe im
    /// Dateifenster. Der zweite geht ueber [`Kommando::KENNUNGEN`] statt ueber
    /// eine Liste und deckt damit auch jeden Befehl ab, der spaeter
    /// dazukommt.
    #[test]
    fn im_editor_wirkt_kein_befehl_des_dateifensters_und_jeder_des_fensters() {
        for kommando in [
            // Die Dateioperationen aus C4.
            Kommando::Kopieren,
            Kommando::Verschieben,
            Kommando::InPapierkorb,
            Kommando::EndgueltigLoeschen,
            Kommando::OrdnerAnlegen,
            Kommando::DateiAnlegen,
            Kommando::Umbenennen,
            Kommando::UmbenennenStapel,
            // Die Ordnernavigation aus C2.
            Kommando::Oeffnen,
            Kommando::OrdnerAufwaerts,
            Kommando::Pfadeingabe,
            Kommando::SeiteHoch,
            Kommando::SeiteRunter,
            Kommando::Listenanfang,
            Kommando::Listenende,
            // Die beiden Zwischenablage-Befehle aus C10.
            Kommando::ZwischenablageAnsehen,
            Kommando::ZwischenablageSpringen,
        ] {
            assert!(
                !wirkt(kommando.wirkungsbereich(), Fokus::Editor),
                "{kommando:?} wirkt mit der Schreibmarke im Editor"
            );
            assert!(
                wirkt(kommando.wirkungsbereich(), Fokus::Dateifenster),
                "{kommando:?} wirkt nicht einmal im Dateifenster; die Gegenprobe liefe leer"
            );
        }

        let mut abgewiesen = 0;
        let mut durchgelassen = 0;
        for (kommando, kennung) in Kommando::KENNUNGEN {
            match kommando.wirkungsbereich() {
                // Die drei Bereichsbefehle und der Navigator enden am Editor.
                Wirkungsbereich::Dateifenster => {
                    abgewiesen += 1;
                    assert!(
                        !wirkt(Wirkungsbereich::Dateifenster, Fokus::Editor),
                        "„{kennung}“ wirkt mit der Schreibmarke im Editor"
                    );
                }
                // Was das Fenster als ganzes betrifft, wirkt im Editor wie
                // ueberall sonst: das Umschalten der Bereiche, die Breiten,
                // das Schliessen des Fensters, das Beenden — und seit C4 der
                // Runde 4 das Schliessen des aktiven Tabs, das dabei die Datei
                // des Editors nicht anfasst.
                Wirkungsbereich::Ueberall => {
                    durchgelassen += 1;
                    assert!(
                        wirkt(Wirkungsbereich::Ueberall, Fokus::Editor),
                        "„{kennung}“ wirkt im Editor nicht"
                    );
                }
                Wirkungsbereich::Leiste
                | Wirkungsbereich::Vorschau
                | Wirkungsbereich::Tabbereich
                | Wirkungsbereich::Navigator => {
                    assert!(
                        !wirkt(kommando.wirkungsbereich(), Fokus::Editor),
                        "„{kennung}“ gehoert einem anderen Bereich und wirkt trotzdem im Editor"
                    );
                }
                // Die neun Befehle des Editors sind der Sinn der Uebung.
                Wirkungsbereich::Editor => {
                    assert!(
                        wirkt(Wirkungsbereich::Editor, Fokus::Editor),
                        "„{kennung}“ wirkt in seinem eigenen Bereich nicht"
                    );
                }
            }
        }
        assert!(
            abgewiesen > 0 && durchgelassen > 0,
            "keine Befehle in einer der beiden Gruppen; die Pruefung liefe leer"
        );
    }

    /// [`in_bereich`] und [`holt_hervor`] kehren einander um (S43).
    ///
    /// Die beiden Zuordnungen stehen an entgegengesetzten Enden desselben
    /// Weges: [`holt_hervor`] sagt, welchen Bereich ein Fokusbefehl aufsucht,
    /// [`in_bereich`] sagt, welcher Fokuswert gilt, wenn der Ersthelfer darin
    /// liegt. Laufen sie auseinander, fuehrt ein Fokusbefehl in einen Bereich,
    /// den die Fokusabfrage danach anders benennt — und die Anzeige aus C9
    /// staende auf einem anderen Kasten als der Ersthelfer.
    ///
    /// Die beiden Dateifensterbereiche stehen daneben, weil sie sich einen
    /// Fokuswert teilen und [`holt_hervor`] fuer ihn `None` liefert.
    #[test]
    fn das_enthaltensein_und_das_hervorholen_kehren_einander_um() {
        for ziel in [Fokus::Leiste, Fokus::Vorschau, Fokus::Editor] {
            let bereich = holt_hervor(ziel).expect("die drei Randbereiche holen einen Bereich");
            assert_eq!(
                in_bereich(bereich),
                ziel,
                "{ziel:?} fuehrt in {bereich:?}, und der meldet sich als etwas anderes zurueck"
            );
        }
        for bereich in [Bereich::Links, Bereich::Rechts] {
            assert_eq!(in_bereich(bereich), Fokus::Dateifenster);
        }
    }

    /// Jeder der fuenf Bereiche traegt genau einen Fokuswert (S43).
    ///
    /// Die Gegenprobe zur Zusage, auf der die Fokusabfrage steht: die fuenf
    /// Teilbaeume der Aufteilung sind zueinander fremd, ein Ersthelfer liegt in
    /// hoechstens einem, und deshalb genuegt der erste Treffer.
    #[test]
    fn jeder_bereich_traegt_genau_einen_fokuswert() {
        for bereich in Bereich::ALLE {
            let fokus = in_bereich(bereich);
            assert_ne!(
                fokus,
                Fokus::Anderswo,
                "{bereich:?} liegt im Fenster und ist damit nie Anderswo"
            );
        }
    }

    /// Die ganze Rahmenregel auf einen Blick: fuenf Bereiche mal fuenf
    /// Fokuswerte mal zwei aktive Seiten, fuenfzig Paare (S44).
    ///
    /// Drei Zusagen stehen darin, und jede von ihnen traegt ein
    /// Abnahmekriterium von C9. Die zweite ist die, die den offenen Datensatz
    /// `260809-2043` betrifft: unter der Vorbelegung des Specs tragen zwei
    /// Bereiche eine Markierung, und "die Anzeige" im zweiten
    /// Abnahmekriterium meint die volle Akzentfarbe.
    #[test]
    fn die_fuenfzig_paare_der_rahmenrolle_gehen_auf() {
        for aktiv in Fensterseite::ALLE {
            for fokus in Fokus::ALLE {
                let fokussierte = Bereich::ALLE
                    .into_iter()
                    .filter(|bereich| {
                        rahmenrolle(*bereich, fokus, aktiv) == Rahmenrolle::Fokussiert
                    })
                    .count();
                let erwartet = usize::from(fokus != Fokus::Anderswo);
                assert_eq!(
                    fokussierte, erwartet,
                    "{fokus:?} bei aktivem {aktiv:?}: {fokussierte} Bereiche tragen die volle \
                     Akzentfarbe, erwartet waren {erwartet}"
                );

                let aktiver = rahmenrolle(Bereich::von_seite(aktiv), fokus, aktiv);
                assert_ne!(
                    aktiver,
                    Rahmenrolle::Ruhig,
                    "das aktive Dateifenster ist bei {fokus:?} nicht mehr zu erkennen"
                );
            }
        }
    }

    /// Der Fokus faerbt genau den Bereich, in dem er wohnt (S44).
    ///
    /// Die Feinprobe neben der Zaehlung darueber: nicht nur, dass genau einer
    /// die volle Akzentfarbe traegt, sondern dass es der richtige ist.
    #[test]
    fn die_volle_akzentfarbe_traegt_der_bereich_mit_dem_fokus() {
        for aktiv in Fensterseite::ALLE {
            for fokus in Fokus::ALLE {
                let Some(erwartet) = bereich_mit_fokus(fokus, aktiv) else {
                    continue;
                };
                assert_eq!(rahmenrolle(erwartet, fokus, aktiv), Rahmenrolle::Fokussiert);
            }
        }
    }

    /// [`bereich_mit_fokus`] liefert fuer das Dateifenster das **aktive**.
    ///
    /// Die Zeile, wegen der [`holt_hervor`] allein nicht genuegt: es kennt das
    /// Fenstermodell nicht und antwortet fuer [`Fokus::Dateifenster`] mit
    /// `None`.
    #[test]
    fn der_fokus_im_dateifenster_meint_das_aktive() {
        assert_eq!(
            bereich_mit_fokus(Fokus::Dateifenster, Fensterseite::Links),
            Some(Bereich::Links)
        );
        assert_eq!(
            bereich_mit_fokus(Fokus::Dateifenster, Fensterseite::Rechts),
            Some(Bereich::Rechts)
        );
        assert_eq!(
            bereich_mit_fokus(Fokus::Anderswo, Fensterseite::Links),
            None,
            "ein stehendes Blatt gehoert keinem Bereich"
        );
    }

    /// Der Terminal-Befehl aus C11 braucht keinen eigenen Mechanismus.
    ///
    /// Die Vorgaengerin dieser Pruefung stand hier als Blick voraus auf Schritt
    /// 18c und zeigte die Regel an einem beliebigen Befehl desselben Bereichs,
    /// weil es fuer C11 noch kein Kommando gab. Seit 18c gibt es eines, und die
    /// Pruefung nennt es: [`Kommando::TerminalOeffnen`] faellt unter dieselbe
    /// Zeile wie jeder andere Befehl des Dateifensters, ohne Zusatz und ohne
    /// Meldung.
    #[test]
    fn der_terminal_befehl_wird_in_der_leiste_stumm_abgewiesen() {
        assert_eq!(
            Kommando::TerminalOeffnen.wirkungsbereich(),
            Wirkungsbereich::Dateifenster
        );
        assert!(!wirkt(
            Kommando::TerminalOeffnen.wirkungsbereich(),
            Fokus::Leiste
        ));
        assert!(wirkt(
            Kommando::TerminalOeffnen.wirkungsbereich(),
            Fokus::Dateifenster
        ));
    }
}
