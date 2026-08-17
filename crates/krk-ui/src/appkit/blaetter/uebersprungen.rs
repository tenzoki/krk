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
    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        frage,
        &[Schaltflaeche::neu(
            "Schließen",
            Taste::Eingabe,
            Wirkung::Liegenlassen,
        )],
    );
    blatt.erlaeuterung_setzen(liste);
    blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| fertig())
}
