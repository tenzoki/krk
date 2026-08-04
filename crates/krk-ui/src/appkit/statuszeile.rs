//! Die Statuszeile am Fuss eines Dateifensters.
//!
//! Sie ist der einzige Weg, auf dem KRK dem Nutzer eine laufende Meldung zeigt.
//! Der Nutzer hat am 260804-0830 Moeglichkeit 1 aus
//! `decisions/260803-2025_a_wie-zeigt-krk-dem-nutzer-fehler.md` gewaehlt: der
//! Ordner ohne Leserecht meldet sich hier, und allein der fehlende
//! Tastenabgriff bricht mit einem Hinweisfenster ab, weil er die Anwendung als
//! ganze betrifft.
//!
//! Vorher lief beides ueber `eprintln!`. Eine ueber den Finder gestartete
//! Anwendung hat keine Standardfehlerausgabe, LaunchServices haengt sie ins
//! Leere; in der einzigen Betriebsart, die die Abnahme zulaesst, war die
//! Fehlerbehandlung damit still. C1 verlangt seit dem 260804-0830
//! ausdruecklich, dass KRK keine Meldung an den Nutzer ueber die
//! Standardfehlerausgabe gibt.
//!
//! Seit dem 260804-1832 traegt sie eine zweite Art von Meldung: den Stand einer
//! laufenden Dateioperation aus C4. Der Nutzer hat den Fortschritt aus dem
//! Blatt hierher verlegt
//! (`decisions/260804-1832_a_traegt-der-fortschritt-ein-blatt-oder-die-statuszeile.md`),
//! weil ein Blatt das Fenster sperrt, das C4 bedienbar zusagt, und auf dem
//! Referenzgeraet 354 bis 403 ms zum Aufgehen braucht, waehrend L8 200 ms
//! zusagt.
//!
//! **Die Art steht in der Signatur und nicht in einer zweiten Funktion.** Ein
//! Fortschritt ist kein Fehler und wird nicht rot; eine zweite Funktion neben
//! [`Statuszeile::zeigen`] waeren zwei Wahrheiten darueber, was in der Zeile
//! steht.
//!
//! Was diese Zeile in dieser Runde **nicht** traegt: den Lesefortschritt und die
//! Zahl der Eintraege. C1 sagt beides nicht zu; sie kommen in einer spaeteren
//! Runde in dieselbe Zeile und nicht in eine zweite daneben.

use objc2::rc::Retained;
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSColor, NSFont, NSTextAlignment, NSTextField, NSView,
};
use objc2_foundation::{MainThreadMarker, NSPoint, NSRect, NSSize, NSString, ns_string};

/// Die Hoehe der Zeile in Punkten.
///
/// Eine Zeile in der kleinen Systemschrift mit etwas Luft darum.
pub const HOEHE: f64 = 18.0;

/// Der Abstand vom linken Rand, damit der Text nicht an der Trennlinie klebt.
pub const EINZUG: f64 = 6.0;

/// Was fuer eine Meldung gerade in der Zeile steht.
///
/// Zwei Werte, weil die Zeile seit dem 260804-1832 zwei Sorten traegt. Sie
/// unterscheiden sich allein in der Farbe: ein Fehler ist rot, damit ihn der
/// Nutzer neben einer leeren Liste nicht uebersieht, und ein Fortschritt ist es
/// nicht, weil er keiner ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    /// Ein Fehler oder ein Hinweis, den der Nutzer bemerken soll.
    Fehler,
    /// Der Stand einer laufenden Dateioperation (C4).
    Vorgang,
}

/// Die Textzeile am Fuss eines Dateifensters.
pub struct Statuszeile {
    feld: Retained<NSTextField>,
}

impl Statuszeile {
    /// Baut eine leere Statuszeile.
    ///
    /// Sie entsteht ohne Groesse und bekommt ihre erste beim Einhaengen; die
    /// Autogroesse haelt sie danach am unteren Rand ueber die volle Breite.
    pub fn bauen(mtm: MainThreadMarker) -> Self {
        let feld = NSTextField::labelWithString(ns_string!(""), mtm);
        feld.setFrame(NSRect::new(NSPoint::ZERO, NSSize::new(0.0, HOEHE)));
        feld.setFont(Some(&NSFont::systemFontOfSize(
            NSFont::smallSystemFontSize(),
        )));
        feld.setTextColor(Some(&NSColor::secondaryLabelColor()));
        feld.setAlignment(NSTextAlignment::Left);
        feld.setMaximumNumberOfLines(1);
        // Am unteren Rand festgemacht, in der Breite mitwachsend: der Abstand
        // nach oben ist beweglich, der nach unten nicht.
        feld.setAutoresizingMask(
            NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewMaxYMargin,
        );
        Self { feld }
    }

    /// Die Ansicht, die in das Dateifenster gehaengt wird.
    pub fn sicht(&self) -> &NSView {
        &self.feld
    }

    /// Zeigt eine Meldung der genannten Art an, oder leert die Zeile bei
    /// `None`.
    ///
    /// Ein Fehler faerbt die Zeile rot: ein Ordner ohne Leserecht ist ein
    /// Fehler und kein Hinweis, und eine graue Zeile am Fuss uebersieht der
    /// Nutzer neben einer leeren Liste genauso, wie er die
    /// Standardfehlerausgabe uebersehen hat. Ein Fortschritt bekommt die
    /// gewoehnliche Textfarbe; auffindbar ist der Abbruch bei ihm nicht ueber
    /// die Farbe, sondern weil die Zeile ihn benennt ("Esc bricht ab").
    pub fn zeigen(&self, meldung: Option<(&str, Art)>) {
        match meldung {
            Some((text, art)) => {
                self.feld.setStringValue(&NSString::from_str(text));
                let farbe = match art {
                    Art::Fehler => NSColor::systemRedColor(),
                    Art::Vorgang => NSColor::labelColor(),
                };
                self.feld.setTextColor(Some(&farbe));
            }
            None => {
                self.feld.setStringValue(ns_string!(""));
                self.feld
                    .setTextColor(Some(&NSColor::secondaryLabelColor()));
            }
        }
    }
}
