//! Die Sprungmarke aus C2: den Eintrag durch Tippen der Anfangsbuchstaben
//! finden.
//!
//! ```text
//! Taste ohne Zusatztaste ──> Nachschlag::Sprungmarke ──> Sprungmarke::tippen
//!                                                             │
//!                                    erste_zeile_mit(Modell, Praefix)
//! ```
//!
//! # Aufgenommen wird nur, was ein Dateiname tragen kann
//!
//! [`Nachschlag::Sprungmarke`](crate::tasten::Nachschlag::Sprungmarke)
//! antwortet auf **jede** Taste ohne Zusatztaste, die keiner Funktion gehoert,
//! nicht nur auf Buchstaben: der Kern kennt allein den Tastencode und weiss
//! nicht, welches Zeichen darauf liegt. Diese Datei traegt deshalb die eine
//! Regel, die daraus eine Sucheingabe macht, [`traegt_ein_dateiname`].
//!
//! Ohne sie schoebe die seit dem 260804 freie Eingabetaste ein
//! Wagenruecklaufzeichen in den Puffer, und die naechste getippte Suche liefe
//! ins Leere. Die Regel ist trotzdem **keine Sonderregel fuer die
//! Eingabetaste**: sie deckt jede unbelegte Funktionstaste ab, deren Zeichen
//! AppKit aus dem privaten Bereich `U+F700` bis `U+F8FF` meldet, und jede
//! andere Taste, die ein Steuerzeichen liefert.
//!
//! Ein abgewiesenes Zeichen laesst den Puffer unveraendert und startet die
//! Pause nicht neu. Beides zusammen ist die Zusage: eine begonnene Suche
//! uebersteht einen Tastendruck, der keine Suche sein kann.

use std::time::{Duration, Instant};

use super::modell::Ordnermodell;

/// Wie lange eine begonnene Eingabe gilt (C2: "Nach einer Pause beginnt die
/// Eingabe von vorn").
pub const PAUSE: Duration = Duration::from_secs(1);

/// Der erste Tastencode des privaten Bereichs, in dem AppKit die Pfeile und
/// die Funktionstasten meldet (`NSUpArrowFunctionKey` und die uebrigen).
const FUNKTIONSTASTEN_ANFANG: char = '\u{F700}';

/// Das letzte Zeichen dieses Bereichs.
const FUNKTIONSTASTEN_ENDE: char = '\u{F8FF}';

/// Ob ein Dateiname dieses Zeichen tragen kann.
///
/// Zwei Klassen fallen weg. Steuerzeichen, wozu der Wagenruecklauf der
/// Eingabetaste, der Tabulator und die Escape-Taste gehoeren; ein Dateiname
/// traegt sie nicht, und sie im Puffer zu fuehren hiesse, nach etwas zu suchen,
/// das kein Eintrag heissen kann. Und der Bereich `U+F700` bis `U+F8FF`, in dem
/// AppKit die Pfeile und die Funktionstasten meldet: diese Zeichen sind ein
/// Behelf der Oberflaeche und stehen fuer gar kein Schriftzeichen.
pub fn traegt_ein_dateiname(zeichen: char) -> bool {
    !zeichen.is_control() && !(FUNKTIONSTASTEN_ANFANG..=FUNKTIONSTASTEN_ENDE).contains(&zeichen)
}

/// Der Puffer der getippten Anfangsbuchstaben eines Dateifensters.
///
/// Er lebt je Dateifenster und nicht je Tab: gesucht wird in der Liste, die
/// gerade auf dem Schirm steht.
#[derive(Debug, Default)]
pub struct Sprungmarke {
    puffer: String,
    /// Wann zuletzt ein Zeichen aufgenommen wurde. `None` heisst: der Puffer
    /// ist leer und die naechste Eingabe faengt ohnehin von vorn an.
    zuletzt: Option<Instant>,
}

impl Sprungmarke {
    /// Eine leere Sprungmarke.
    pub fn neu() -> Self {
        Self::default()
    }

    /// Was gerade getippt ist.
    pub fn puffer(&self) -> &str {
        &self.puffer
    }

    /// Verwirft die begonnene Eingabe.
    ///
    /// Gerufen bei jedem Ordnerwechsel: der Puffer gehoert der Liste, die er
    /// durchsucht hat.
    pub fn zuruecksetzen(&mut self) {
        self.puffer.clear();
        self.zuletzt = None;
    }

    /// Nimmt ein getipptes Zeichen auf.
    ///
    /// Liefert das Praefix, nach dem zu suchen ist, oder `None` fuer ein
    /// Zeichen, das kein Dateiname tragen kann. Im zweiten Fall bleibt der
    /// Puffer unveraendert und die Pause laeuft weiter, statt neu zu beginnen.
    ///
    /// Liegt der letzte Tastendruck [`PAUSE`] oder laenger zurueck, faengt die
    /// Eingabe von vorn an, wie C2 es verlangt.
    pub fn tippen(&mut self, zeichen: char, jetzt: Instant) -> Option<&str> {
        if !traegt_ein_dateiname(zeichen) {
            return None;
        }
        let faengt_von_vorn = match self.zuletzt {
            Some(vorher) => jetzt.saturating_duration_since(vorher) >= PAUSE,
            None => true,
        };
        if faengt_von_vorn {
            self.puffer.clear();
        }
        self.puffer.push(zeichen);
        self.zuletzt = Some(jetzt);
        Some(&self.puffer)
    }
}

/// Die erste Zeile, deren Name mit dem Praefix beginnt.
///
/// Ohne Ruecksicht auf Gross- und Kleinschreibung, wie die Sortierung nach
/// Namen. Gesucht wird in der Sichtreihenfolge und nicht in der
/// Lesereihenfolge: C2 sagt "der erste Eintrag", und das ist der erste, den der
/// Nutzer sieht.
pub fn erste_zeile_mit(modell: &Ordnermodell, praefix: &str) -> Option<usize> {
    if praefix.is_empty() {
        return None;
    }
    let gesucht = praefix.to_lowercase();
    modell
        .zeilen()
        .position(|eintrag| eintrag.name.to_lowercase().starts_with(&gesucht))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ein_wagenruecklauf_und_eine_funktionstaste_tragen_kein_dateiname() {
        assert!(!traegt_ein_dateiname('\r'));
        assert!(!traegt_ein_dateiname('\n'));
        assert!(!traegt_ein_dateiname('\t'));
        assert!(!traegt_ein_dateiname('\u{1B}'), "die Escape-Taste");
        assert!(!traegt_ein_dateiname('\u{F701}'), "NSDownArrowFunctionKey");
        assert!(!traegt_ein_dateiname('\u{F704}'), "NSF1FunctionKey");
    }

    #[test]
    fn buchstaben_ziffern_und_satzzeichen_tragen_ein_dateiname() {
        for zeichen in ['a', 'Z', '7', '.', '-', ' ', 'ä', '中'] {
            assert!(
                traegt_ein_dateiname(zeichen),
                "{zeichen:?} gilt als nicht tragbar"
            );
        }
    }
}
