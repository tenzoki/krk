//! Die Abschlussliste der uebersprungenen Eintraege mit ihrem Grund (C4).
//!
//! "Scheitert eine Operation an einem einzelnen Eintrag, etwa wegen fehlender
//! Rechte, laeuft sie mit den uebrigen weiter und meldet am Ende eine Liste der
//! uebersprungenen Eintraege mit Grund." Die Liste selbst setzt
//! [`crate::kommandos::operationen::uebersprungenliste`] zusammen, samt der
//! Kuerzung langer Listen; hier steht allein, was AppKit betrifft.
//!
//! **Ohne uebersprungene Eintraege gibt es kein Blatt.** Ein Blatt, das nach
//! jeder gelungenen Kopie "nichts uebersprungen" meldete, waere ein Tastendruck
//! ohne Auskunft; die gelungene Operation meldet sich in der Statuszeile.
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

use super::{Blatt, Blattgriff, Schaltflaeche, Taste, Wirkung};

/// Die eine Schaltflaeche der Abschlussliste.
///
/// **Als reine Funktion herausgezogen**, damit der Bauplan dieses Blattes ohne
/// AppKit und ohne Hauptfaden pruefbar ist; dieselbe Bauform wie
/// [`super::loeschbestaetigung::schaltflaechen`](super::loeschbestaetigung) und
/// [`super::standardschaltflaechen`](super). An einem gebauten `NSAlert` ist
/// nicht mehr abzulesen, welche seiner Schaltflaechen alles liegen laesst, an
/// dieser Liste schon.
///
/// Sie laesst liegen, obwohl sie die einzige ist: das Blatt fragt nach keinem
/// Vorgang, sondern meldet einen abgeschlossenen, und das Schliessen ist
/// derselbe Ausgang, den die Escape-Taste naehme.
#[must_use]
fn schaltflaechen() -> [Schaltflaeche<'static>; 1] {
    [Schaltflaeche::neu(
        "Schließen",
        Taste::Eingabe,
        Wirkung::Liegenlassen,
    )]
}

/// Zeigt die Abschlussliste am Fenster.
///
/// `frage` und `liste` kommen aus
/// [`crate::kommandos::operationen::uebersprungenliste`].
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    liste: &str,
    fertig: impl Fn() + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(mtm, frage, &schaltflaechen());
    blatt.erlaeuterung_setzen(liste);
    blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| fertig())
}

#[cfg(test)]
mod tests {
    use crate::appkit::blaetter::abbruchstelle;

    use super::{Taste, Wirkung, schaltflaechen};

    /// Der Bauplan traegt genau eine Schaltflaeche, und sie laesst liegen.
    ///
    /// Ohne AppKit und ohne Hauptfaden: [`super::Schaltflaeche`] traegt nur eine
    /// Beschriftung, eine Taste und eine [`Wirkung`]. Eine zweite Schaltflaeche
    /// hier waere eine Antwort auf eine Frage, die dieses Blatt nicht stellt;
    /// die Probe wird rot, sobald jemand eine anlegt.
    #[test]
    fn der_bauplan_traegt_die_eine_schliessende_schaltflaeche() {
        let schaltflaechen = schaltflaechen();
        assert_eq!(
            schaltflaechen.len(),
            1,
            "die Abschlussliste bietet mehr als das Schließen an"
        );
        assert_eq!(schaltflaechen[0].titel, "Schließen");
        assert_eq!(schaltflaechen[0].taste, Taste::Eingabe);
        assert_eq!(
            schaltflaechen[0].wirkung,
            Wirkung::Liegenlassen,
            "das Schließen der Abschlussliste fuehrt etwas aus"
        );
        assert_eq!(
            abbruchstelle(&schaltflaechen),
            0,
            "der ungefaehrliche Ausgang liegt nicht auf der einen Schaltflaeche"
        );
    }
}
