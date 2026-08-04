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

use objc2_app_kit::NSWindow;
use objc2_foundation::MainThreadMarker;

use super::{Blatt, Blattgriff, Schaltflaeche, Taste};

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
        &[Schaltflaeche::neu("Schließen", Taste::Eingabe)],
    );
    blatt.erlaeuterung_setzen(liste);
    blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| fertig())
}
