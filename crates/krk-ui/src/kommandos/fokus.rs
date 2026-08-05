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

use krk_core::tasten::Wirkungsbereich;

/// Wo der Eingabefokus steht.
///
/// Die Antwort der Oberflaeche auf den [`Wirkungsbereich`] des Kerns. Drei
/// Werte, und sie decken das Fenster vollstaendig ab: die beiden Dateilisten,
/// die Leiste, und alles uebrige.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fokus {
    /// In einer der beiden Dateilisten.
    Dateifenster,
    /// In der Lesezeichen- und Geraeteleiste (C5).
    Leiste,
    /// Irgendwo sonst: in einem Blatt oder in einem Textfeld.
    ///
    /// Ein Kommando, das einen Bereich braucht, wirkt hier nicht. Der Fall ist
    /// nicht theoretisch: vor der Rueckfrage des endgueltigen Loeschens ist das
    /// Panel des Blattes das Schluesselfenster, und ohne diesen Wert loeschte
    /// ein Delete davor in dem Ordner dahinter.
    Anderswo,
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
    }
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::Kommando;

    use super::*;

    const JEDER_FOKUS: [Fokus; 3] = [Fokus::Dateifenster, Fokus::Leiste, Fokus::Anderswo];

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

    /// Der Blick voraus auf Schritt 18c: der Terminal-Befehl aus C11 braucht
    /// keinen eigenen Mechanismus.
    ///
    /// Er traegt seinen Fokusvorbehalt, sobald er `Wirkungsbereich::Dateifenster`
    /// nennt, und diese Pruefung zeigt an einem beliebigen Befehl desselben
    /// Bereichs, dass die Zuleitung ihn ohne Zusatz abweist. Ein Kommando gibt
    /// es fuer C11 noch nicht; sobald es eines gibt, faellt es unter dieselbe
    /// Zeile.
    #[test]
    fn ein_befehl_mit_dem_bereich_dateifenster_wird_in_der_leiste_stumm_abgewiesen() {
        assert!(!wirkt(Wirkungsbereich::Dateifenster, Fokus::Leiste));
    }
}
