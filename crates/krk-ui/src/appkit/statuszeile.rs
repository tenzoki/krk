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

    /// Zeigt eine Meldung an, oder leert die Zeile bei `None`.
    ///
    /// Eine Meldung faerbt die Zeile: ein Ordner ohne Leserecht ist ein Fehler
    /// und kein Hinweis, und eine graue Zeile am Fuss uebersieht der Nutzer
    /// neben einer leeren Liste genauso, wie er die Standardfehlerausgabe
    /// uebersehen hat.
    pub fn zeigen(&self, meldung: Option<&str>) {
        match meldung {
            Some(text) => {
                self.feld.setStringValue(&NSString::from_str(text));
                self.feld.setTextColor(Some(&NSColor::systemRedColor()));
            }
            None => {
                self.feld.setStringValue(ns_string!(""));
                self.feld
                    .setTextColor(Some(&NSColor::secondaryLabelColor()));
            }
        }
    }
}
