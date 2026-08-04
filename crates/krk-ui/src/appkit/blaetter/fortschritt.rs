//! Das Fortschrittsblatt mit dem Abbruch (C4).
//!
//! Es geht auf, sobald die Operation [`crate::kommandos::operationen::BLATTVERZUG`]
//! gelaufen ist, zeigt den Stand und traegt genau eine Schaltflaeche. Wann es
//! aufgeht und was in seiner Zeile steht, entscheidet
//! [`crate::kommandos::operationen`]; hier steht allein, was AppKit betrifft.
//!
//! **Es hat keine bestaetigende Schaltflaeche.** Eine Operation, die laeuft,
//! laesst sich nicht bestaetigen, nur abbrechen. Die eine Schaltflaeche traegt
//! deshalb die Escape-Taste und nicht die Eingabetaste; die Eingabetaste bleibt
//! ohne Wirkung, und das ist richtig.
//!
//! **Die Standzeile ist eine eigene Beschriftung und nicht der erlaeuternde
//! Text der Warnung.** Ein `NSAlert` bemisst sich beim Aufgehen und waechst
//! danach nicht mehr; eine Zeile, die von "0 Einträge" auf "4.812 Einträge"
//! springt, waere im engeren Blatt abgeschnitten. Die Beschriftung hat eine
//! feste Breite und zwei feste Zeilen.

use std::cell::Cell;
use std::rc::Rc;

use objc2::rc::Retained;
use objc2_app_kit::{NSFont, NSTextField, NSWindow};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString, ns_string};

use super::{Blatt, Blattgriff, Schaltflaeche, Taste};

/// Die Breite der Standzeile in Punkten. Sie bestimmt die Breite des Blattes.
const ZEILENBREITE: f64 = 420.0;

/// Die Hoehe der Standzeile in Punkten: zwei Zeilen kleine Systemschrift.
const ZEILENHOEHE: f64 = 32.0;

/// Ein stehendes Fortschrittsblatt.
pub struct Fortschrittsblatt {
    griff: Blattgriff,
    anzeige: Retained<NSTextField>,
    /// Wahr, sobald [`Fortschrittsblatt::schliessen`] das Blatt selbst
    /// wegnimmt.
    ///
    /// Ohne dieses Kennzeichen loeste jedes programmatische Schliessen den
    /// Abbruchweg aus: AppKit ruft den Abschlussblock, und der kann von aussen
    /// nicht unterscheiden, ob der Nutzer gedrueckt hat oder der Vorgang zu
    /// Ende ist. Ein Konflikt mitten in einer Kopie nimmt das Blatt kurz weg
    /// und braechte die Kopie sonst um.
    still: Rc<Cell<bool>>,
}

/// Zeigt das Fortschrittsblatt am Fenster.
///
/// `abbrechen` laeuft auf dem Hauptfaden, sobald der Nutzer die Schaltflaeche
/// drueckt. Das Blatt bleibt danach stehen: der Abbruch ist ein Wunsch an den
/// Arbeitsfaden, und geschlossen wird erst, wenn dieser seinen Bericht schickt.
/// Erst so nennt die Abschlussmeldung die Zahl der bis dahin uebertragenen
/// Eintraege, die C4 verlangt.
pub fn zeigen(
    mtm: MainThreadMarker,
    fenster: &NSWindow,
    ueberschrift: &str,
    stand: &str,
    abbrechen: impl Fn() + 'static,
) -> Fortschrittsblatt {
    let anzeige = NSTextField::labelWithString(ns_string!(""), mtm);
    anzeige.setFrame(NSRect::new(
        NSPoint::ZERO,
        NSSize::new(ZEILENBREITE, ZEILENHOEHE),
    ));
    anzeige.setFont(Some(&NSFont::systemFontOfSize(
        NSFont::smallSystemFontSize(),
    )));
    anzeige.setMaximumNumberOfLines(2);
    anzeige.setStringValue(&NSString::from_str(stand));

    let blatt = Blatt::mit_schaltflaechen(
        mtm,
        ueberschrift,
        &[Schaltflaeche::neu("Abbrechen", Taste::Escape)],
    );
    blatt.beigabe_setzen(&anzeige);

    let still = Rc::new(Cell::new(false));
    let im_block = Rc::clone(&still);
    let griff = blatt.zeigen_mit_wahl(fenster, move |_stelle, _fuer_alle| {
        if !im_block.get() {
            abbrechen();
        }
    });
    Fortschrittsblatt {
        griff,
        anzeige,
        still,
    }
}

impl Fortschrittsblatt {
    /// Schreibt den neuen Stand in die Zeile.
    pub fn stand_setzen(&self, stand: &str) {
        self.anzeige.setStringValue(&NSString::from_str(stand));
    }

    /// Nimmt das Blatt weg, ohne den Abbruchweg auszuloesen.
    pub fn schliessen(&self) {
        self.still.set(true);
        self.griff.abbrechen();
    }
}
