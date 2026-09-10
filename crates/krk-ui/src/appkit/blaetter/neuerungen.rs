//! Das Blatt auf Abruf: was diese Fassung an den von Hand gepflegten
//! Ablagedateien mitbringt.
//!
//! ```text
//! Kommando::NeuerungenZeigen ──> Anwendungsdelegierter::neuerungen_zeigen
//!                                        │  Bestand vom Start, sonst auf
//!                                        │  Verlangen nachgetragen
//!                                        ▼
//!                                zeigen(fenster, bestand)
//!                                        │
//!            krk_core::ablage::neuerungen::blatttext(bestand)
//! ```
//!
//! **Der Text kommt fertig aus dem Kern** und wird hier nicht zusammengesetzt.
//! [`krk_core::ablage::neuerungen::blatttext`] fuehrt je verglichener Datei
//! ihren vollen Pfad, den Unterschied in beide Richtungen und den Satz
//! darueber, was ein Unterschied bei ihr kostet; diese Datei traegt allein,
//! was AppKit betrifft. Dieselbe Arbeitsteilung wie bei
//! [`super::uebersprungen`], wo `kommandos::operationen::uebersprungenliste`
//! die Liste stellt.
//!
//! # Ohne einen einzigen Unterschied steht das Blatt trotzdem
//!
//! **Das ist der Unterschied zu [`super::uebersprungen`]**, wo ohne Eintraege
//! kein Blatt aufgeht. Dort meldet KRK ungefragt, und ein Blatt „nichts
//! uebersprungen" waere ein Tastendruck ohne Auskunft. Hier hat der Nutzer
//! gefragt: „an Ihren Dateien ist nichts" ist auf seine Frage eine Antwort,
//! und die drei vollen Pfade sind der Grund, aus dem er sie gestellt hat.
//!
//! Aus demselben Grund steht die Frage als eine Zeichenkette da und nicht in
//! zwei Fassungen. Ob es einen Unterschied gibt, sagt der Text darunter Datei
//! fuer Datei; eine Ueberschrift, die es vorwegnaehme, muesste beide Faelle
//! treffen und traefe keinen genau.
//!
//! # Ab welchem macOS die angesprochenen Klassen stehen
//!
//! Eine einzige AppKit-Klasse, `NSWindow`, und die Datei reicht sie nur weiter;
//! sie steht seit macOS 10.0 zur Verfuegung. `MainThreadMarker` gehoert `objc2`
//! und nicht AppKit. Das Buendel zielt auf 15.0 (`.cargo/config.toml`), und
//! nichts hier ist nach macOS 15 hinzugekommen; `objc2` fuehrt keine
//! Verfuegbarkeitsangaben mit sich, und die Nennung ist die Gegenmassnahme.
//! Alles, was `NSAlert` betrifft, steht im Kopf von [`Blatt`].

use objc2_app_kit::NSWindow;
use objc2_foundation::MainThreadMarker;

use krk_core::ablage::neuerungen::{Bestand, blatttext};

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Die Kopfzeile des Blattes.
///
/// Sie nennt den Gegenstand und nicht das Ergebnis: der Modulkopf sagt, warum
/// keine zweite Fassung fuer den Fall ohne Unterschied danebensteht.
pub const FRAGE: &str = "Ihre Ablagedateien und was diese Fassung mitbringt";

/// Die eine Schaltflaeche des Blattes auf Abruf.
///
/// **Als reine Funktion herausgezogen**, aus demselben Grund wie bei
/// [`super::uebersprungen`] und [`super::startmeldungen`]: an einem gebauten
/// `NSAlert` ist nicht mehr abzulesen, welche seiner Schaltflaechen alles
/// liegen laesst, an dieser Liste schon.
///
/// Sie laesst liegen, obwohl sie die einzige ist: das Blatt fragt nach nichts,
/// sondern zeigt, und das Schliessen ist derselbe Ausgang, den die
/// Escape-Taste naehme. KRK schreibt keine der gezeigten Dateien — eine
/// zweite Schaltflaeche „uebernehmen" waere die Zusage der Runde in ihr
/// Gegenteil verkehrt.
#[must_use]
fn schaltflaechen() -> [Schaltflaeche<'static>; 1] {
    [Schaltflaeche::neu(
        "Schließen",
        Taste::Eingabe,
        Wirkung::Liegenlassen,
    )]
}

/// Zeigt den erhobenen Bestand am Fenster.
///
/// Welchen Bestand sie zeigt, entscheidet der eine Rufer,
/// `Anwendungsdelegierter::neuerungen_zeigen`: den vom Start, wenn der Start
/// erhoben hat, und sonst einen auf Verlangen nachgetragenen. Diese Datei
/// kennt den Unterschied nicht und soll ihn nicht kennenlernen; was der
/// gezeigte Stand bedeutet, sagt der Schlusssatz von [`blatttext`].
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    bestand: &Bestand,
    fertig: impl Fn() + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(mtm, FRAGE, &schaltflaechen());
    blatt.erlaeuterung_setzen(&blatttext(bestand));
    blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| fertig())
}

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::abbruchstelle;

    use super::{FRAGE, Taste, Wirkung, schaltflaechen};

    /// Der Bauplan traegt genau eine Schaltflaeche, und sie laesst liegen.
    ///
    /// Dieselbe Probe wie bei [`super::super::uebersprungen`], aus einem
    /// staerkeren Grund: dieses Blatt zeigt Dateien, die KRK ausdruecklich
    /// nicht schreibt. Eine zweite Schaltflaeche waere hier nicht nur eine
    /// Antwort auf eine ungestellte Frage, sondern ein Schreibweg.
    #[test]
    fn der_bauplan_traegt_die_eine_schliessende_schaltflaeche() {
        let schaltflaechen = schaltflaechen();
        assert_eq!(
            schaltflaechen.len(),
            1,
            "das Blatt der Neuerungen bietet mehr als das Schließen an"
        );
        assert_eq!(schaltflaechen[0].titel, "Schließen");
        assert_eq!(schaltflaechen[0].taste, Taste::Eingabe);
        assert_eq!(
            schaltflaechen[0].wirkung,
            Wirkung::Liegenlassen,
            "das Schließen der Neuerungen fuehrt etwas aus"
        );
        assert_eq!(
            abbruchstelle(&schaltflaechen),
            0,
            "der ungefaehrliche Ausgang liegt nicht auf der einen Schaltflaeche"
        );
    }

    /// Die Kopfzeile traegt Umlaute und nennt keinen Befund.
    ///
    /// Der Wortlaut steht unter Probe, weil er die eine Zeile ist, die dieses
    /// Blatt selbst beisteuert; alles darunter kommt aus dem Kern und ist
    /// dort gemessen. „Keinen Befund" ist die Zusage aus dem Modulkopf: eine
    /// Kopfzeile, die von Neuerungen spraeche, waere falsch, sobald es keine
    /// gibt — und genau dann geht dieses Blatt trotzdem auf.
    #[test]
    fn die_kopfzeile_nennt_den_gegenstand_und_nicht_das_ergebnis() {
        assert_eq!(FRAGE, "Ihre Ablagedateien und was diese Fassung mitbringt");
        assert!(
            !FRAGE.contains("Neu"),
            "die Kopfzeile nimmt einen Befund vorweg, den es nicht geben muss"
        );
    }
}
