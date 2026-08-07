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

use krk_core::tasten::Wirkungsbereich;

use crate::fenstermodell::Bereich;

/// Wo der Eingabefokus steht.
///
/// Die Antwort der Oberflaeche auf den [`Wirkungsbereich`] des Kerns. Vier
/// Werte, und sie decken das Fenster vollstaendig ab: die beiden Dateilisten,
/// die Leiste, das Vorschaufenster, und alles uebrige.
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
    /// C1 die Vorschau-Tabs ([`Wirkungsbereich::Tabbereich`]), und die beiden
    /// Zwischenablage-Befehle aus C10 loesen nichts aus, wie ihr
    /// Abnahmekriterium es verlangt.
    Vorschau,
    /// Irgendwo sonst: in einem Blatt oder in einem Textfeld.
    ///
    /// Ein Kommando, das einen Bereich braucht, wirkt hier nicht. Der Fall ist
    /// nicht theoretisch: vor der Rueckfrage des endgueltigen Loeschens ist das
    /// Panel des Blattes das Schluesselfenster, und ohne diesen Wert loeschte
    /// ein Delete davor in dem Ordner dahinter.
    Anderswo,
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
/// Das aktive Dateifenster ist nie ausgeblendet: das linke laesst C7 gar nicht
/// ausblenden, und mit dem rechten wandert die Aktivitaet auf das linke
/// ([`Fenstermodell::umschalten`](crate::fenstermodell::Fenstermodell::umschalten)).
/// Fuer [`Fokus::Dateifenster`] ist deshalb nichts hervorzuholen, und
/// [`Fokus::Anderswo`] ist kein Ziel eines Befehls, sondern ein Befund.
pub const fn holt_hervor(ziel: Fokus) -> Option<Bereich> {
    match ziel {
        Fokus::Leiste => Some(Bereich::Lesezeichen),
        Fokus::Vorschau => Some(Bereich::Vorschau),
        Fokus::Dateifenster | Fokus::Anderswo => None,
    }
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
        Wirkungsbereich::Tabbereich => {
            matches!(fokus, Fokus::Dateifenster | Fokus::Vorschau)
        }
    }
}

#[cfg(test)]
mod tests {
    use krk_core::ablage::Sitzung;
    use krk_core::tasten::Kommando;

    use crate::fenstermodell::Fenstermodell;

    use super::*;

    const JEDER_FOKUS: [Fokus; 4] = [
        Fokus::Dateifenster,
        Fokus::Leiste,
        Fokus::Vorschau,
        Fokus::Anderswo,
    ];

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

    /// Die vier Tabbefehle bedienen nach C6 auch die Vorschau-Tabs.
    ///
    /// Sie wirken in beiden Bereichen mit Tabs und nirgends sonst: nicht in
    /// der Leiste, die keine Tabs traegt, und nicht in einem Blatt oder
    /// Textfeld.
    #[test]
    fn ein_tabbefehl_wirkt_in_beiden_bereichen_mit_tabs() {
        for kommando in [
            Kommando::TabNeu,
            Kommando::TabSchliessen,
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
            let mut modell = Fenstermodell::aus_sitzung(&Sitzung::default());
            modell.umschalten(bereich);
            assert!(!modell.sichtbar(bereich), "die Probe beginnt ausgeblendet");

            let hervorzuholen = holt_hervor(ziel).expect("dieser Befehl holt einen Bereich hervor");
            assert_eq!(hervorzuholen, bereich);
            assert!(
                modell.einblenden(hervorzuholen),
                "{ziel:?} weist den ausgeblendeten Bereich stumm ab, statt ihn hervorzuholen"
            );
            assert!(
                modell.sichtbar(bereich),
                "erst der sichtbare Bereich laesst den Fokus hinein"
            );
        }
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
