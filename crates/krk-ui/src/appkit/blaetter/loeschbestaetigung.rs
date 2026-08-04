//! Die Rueckfrage vor dem endgueltigen Loeschen (C4).
//!
//! Genau einmal je Vorgang, unabhaengig von der Zahl der betroffenen Eintraege.
//! Sie nennt die Zahl der Eintraege und, falls Ordner darunter sind, deren Zahl
//! gesondert. Die beiden Texte rechnet
//! [`crate::kommandos::operationen::loeschfrage`]; hier steht allein, was
//! AppKit betrifft.
//!
//! # Vorbelegt ist Abbrechen
//!
//! C4 verlangt es woertlich: "Vorbelegt ist Abbrechen, sodass ein reflexhaftes
//! Bestaetigen mit der Return-Taste nichts loescht." Das ist der Grund, aus dem
//! die Huelle die Taste je Schaltflaeche entgegennimmt: `NSAlert` gaebe die
//! Eingabetaste sonst der ersten, und die erste soll hier "Abbrechen" sein,
//! damit sie zugleich die hervorgehobene ist.
//!
//! Der zweite Weg zum Abbruch, die Escape-Taste, laeuft nicht ueber eine
//! Tastenentsprechung dieses Blattes, sondern ueber den Befehl `abbrechen` aus
//! `resources/default-keymap.toml`: der Ereignisabgriff sieht die Taste vor dem
//! Blatt, und der Anwendungsdelegierte schliesst das offene Blatt. Eine zweite
//! Tastenentsprechung waere hier auch gar nicht moeglich, weil ein `NSButton`
//! genau eine traegt.
//!
//! Der Weg dahin ist bindend: das Loeschen faengt erst an, wenn diese Frage mit
//! Ja beantwortet ist. Der Kern bekommt seinen Auftrag danach, siehe
//! `shared/decisions/260802-0842_a_loeschen-papierkorb-oder-endgueltig.md`.

use objc2_app_kit::NSWindow;
use objc2_foundation::MainThreadMarker;

use super::{Blatt, Blattgriff, Schaltflaeche, Taste};

/// Zeigt die Rueckfrage und meldet, ob der Nutzer das Loeschen bestaetigt hat.
///
/// `fertig` laeuft auf dem Hauptfaden und genau einmal. `false` heisst
/// abgebrochen, und dann geschieht nichts.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    erlaeuterung: &str,
    fertig: impl Fn(bool) + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        frage,
        &[
            Schaltflaeche::neu("Abbrechen", Taste::Eingabe),
            Schaltflaeche::neu("Endgültig löschen", Taste::EingabeMitBefehl),
        ],
    );
    blatt.erlaeuterung_setzen(&format!(
        "{erlaeuterung}\n\nReturn und Esc brechen ab. Zum Löschen Cmd+Return."
    ));
    blatt.als_warnung();
    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| fertig(stelle == 1))
}
