//! Die eine Rueckfrage vor dem Raeumen in den Papierkorb (C2, C3).
//!
//! Genau einmal je Vorgang, unabhaengig von der Zahl der betroffenen Eintraege
//! und unabhaengig davon, welcher der Loeschbefehle ihn ausgeloest hat. Die
//! beiden Texte rechnet
//! [`crate::kommandos::loeschwarnung::frage_und_erlaeuterung`]; hier steht
//! allein, was AppKit betrifft.
//!
//! # Ruhig und laut sind dasselbe Blatt
//!
//! Ein zweites Blatt entsteht nicht: zwei Blaetter waeren zwei Wahrheiten ueber
//! dieselbe Frage. Die laute Form unterscheidet sich von der ruhigen in genau
//! drei Dingen, und nur eines davon steht in dieser Datei. Der Grund in der
//! Frage und die Folgen in der Erlaeuterung kommen als Text herein; das
//! Warnzeichen setzt [`Blatt::als_warnung`], und zwar nur bei `laut`. Die
//! Schaltflaechen, ihre Reihenfolge und ihre Tasten sind in beiden Formen
//! dieselben (C3).
//!
//! ```text
//!  frage, erlaeuterung ─┬──> ruhig: Frage, Erlaeuterung, zwei Schaltflaechen
//!  laut ────────────────┘    laut:  dasselbe, dazu das Warnzeichen
//! ```
//!
//! **Die Beschriftung der zweiten Schaltflaeche kommt als Argument herein**,
//! weil der Wortlaut des Vorgangs [`crate::kommandos::loeschwarnung`] gehoert
//! und nicht dieser Datei. "Abbrechen" bleibt als einziger Wortlaut hier
//! stehen, denn es benennt keinen Vorgang, sondern die Sicherheitseigenschaft
//! des Blattes, und es haengt untrennbar an der Tastenzuordnung darunter.
//!
//! # Vorbelegt ist Abbrechen
//!
//! C4 verlangt es woertlich: "Vorbelegt ist Abbrechen, sodass ein reflexhaftes
//! Bestaetigen mit der Return-Taste nichts loescht." Das ist der Grund, aus dem
//! die Huelle die Taste je Schaltflaeche entgegennimmt: `NSAlert` gaebe die
//! Eingabetaste sonst der ersten, und die erste soll hier "Abbrechen" sein,
//! damit sie zugleich die hervorgehobene ist.
//!
//! Dieselbe Forderung steht als Abnahmekriterium in C2 dieser Runde, und sie
//! gilt dort fuer **beide** Formen: die ruhige Rueckfrage vor dem alltaeglichen
//! Raeumen ist genauso vorbelegt wie die laute.
//!
//! Der zweite Weg zum Abbruch, die Escape-Taste, laeuft nicht ueber eine
//! Tastenentsprechung dieses Blattes, sondern ueber den Befehl `abbrechen` aus
//! `resources/default-keymap.toml`: der Ereignisabgriff sieht die Taste vor dem
//! Blatt, und der Anwendungsdelegierte schliesst das offene Blatt. Eine zweite
//! Tastenentsprechung waere hier auch gar nicht moeglich, weil ein `NSButton`
//! genau eine traegt.
//!
//! Der Weg dahin ist bindend: das Raeumen faengt erst an, wenn diese Frage mit
//! Ja beantwortet ist. Der Kern bekommt seinen Auftrag danach, siehe
//! `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.
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

use super::{Blatt, Blattgriff, Schaltflaeche, Taste};

/// Zeigt die Rueckfrage und meldet, ob der Nutzer den Vorgang bestaetigt hat.
///
/// `schaltflaeche` ist die Beschriftung der **zweiten** Schaltflaeche, also
/// derjenigen, die den Vorgang ausloest; die erste bleibt "Abbrechen" und
/// traegt die Eingabetaste. `laut` setzt das Warnzeichen des Systems und sonst
/// nichts: Text, Reihenfolge und Tasten sind in beiden Formen dieselben.
///
/// `fertig` laeuft auf dem Hauptfaden und genau einmal. `false` heisst
/// abgebrochen, und dann geschieht nichts.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    frage: &str,
    erlaeuterung: &str,
    schaltflaeche: &str,
    laut: bool,
    fertig: impl Fn(bool) + 'static,
) -> Blattgriff {
    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        frage,
        &[
            Schaltflaeche::neu("Abbrechen", Taste::Eingabe),
            Schaltflaeche::neu(schaltflaeche, Taste::EingabeMitBefehl),
        ],
    );
    blatt.erlaeuterung_setzen(&format!(
        "{erlaeuterung}\n\nReturn und Esc brechen ab. Zum Löschen Cmd+Return."
    ));
    if laut {
        blatt.als_warnung();
    }
    blatt.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| fertig(stelle == 1))
}
