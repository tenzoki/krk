//! Die Form eines Feldes und die eine Regel, nach der ein Wert geschrieben
//! wird (E3, E5).
//!
//! # Drei Formen, und jedes Feld hat genau eine
//!
//! - **Glatt**: das Feld traegt kein `"`. Sein Wert ist sein Text.
//! - **Gequotet**: das Feld steht vollstaendig nach RFC 4180 in `"`, ein `"`
//!   darin als `""`. Sein Wert ist der Text dazwischen mit `""` als `"`.
//! - **Nachsichtig**: alles andere, was der Leser trotzdem liest, naemlich ein
//!   `"` mitten in einem ungequoteten Feld (`ab"c`), Text hinter dem
//!   schliessenden `"` (`"ab"c`) und ein `"`, das bis zum Dateiende nicht
//!   schliesst. Der Wert ist das Feld, wie es dasteht: ein `"` mitten im Feld
//!   ist ein Zeichen, der Text hinter dem schliessenden `"` wird angehaengt.
//!
//! **Ein nachsichtiges Feld bleibt Byte fuer Byte stehen, bis seine Zelle
//! geaendert wird**; erst dann schreibt [`feld_kodieren`] es in kanonischer
//! Form. Der Leser meldet die Form, damit eine Probe genau das halten kann.

use std::borrow::Cow;
use std::ops::Range;

use super::dialekt::Trennzeichen;

/// Die Form, in der ein Feld in der Datei steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feldform {
    /// Ohne `"`.
    Glatt,
    /// Vollstaendig nach RFC 4180 in `"`.
    Gequotet,
    /// Gelesen mit Nachsicht; der Modulkopf nennt die Faelle.
    Nachsichtig,
}

/// Ein Feld eines Satzes: wo es steht, was es meint und in welcher Form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feld<'a> {
    /// Der Byte-Bereich des Feldes im gelesenen Text, samt Anfuehrungszeichen
    /// und ohne Trennzeichen.
    pub roh: Range<usize>,
    /// Der Wert der Zelle.
    pub wert: Cow<'a, str>,
    /// Die Form, in der das Feld dasteht.
    pub form: Feldform,
}

/// Der Wert eines Feldes aus seinem rohen Text und seiner Form.
pub(crate) fn wert_von(roh: &str, form: Feldform) -> Cow<'_, str> {
    match form {
        Feldform::Glatt => Cow::Borrowed(roh),
        Feldform::Gequotet => {
            // Der Leser meldet `Gequotet` allein fuer ein Feld, das mit `"`
            // beginnt und mit dem schliessenden `"` endet; beide sind ASCII,
            // der Schnitt liegt also an Zeichengrenzen.
            let innen = &roh[1..roh.len() - 1];
            if innen.contains("\"\"") {
                Cow::Owned(innen.replace("\"\"", "\""))
            } else {
                Cow::Borrowed(innen)
            }
        }
        Feldform::Nachsichtig => {
            let Some(rest) = roh.strip_prefix('"') else {
                return Cow::Borrowed(roh);
            };
            // Bis zum schliessenden `"` gilt `""` als `"`, danach steht der
            // Text, wie er dasteht. Schliesst das `"` nie, ist alles danach
            // der Wert.
            let mut wert = String::with_capacity(rest.len());
            let mut zeichen = rest.char_indices().peekable();
            while let Some((stelle, c)) = zeichen.next() {
                if c == '"' {
                    if zeichen.peek().is_some_and(|(_, folgend)| *folgend == '"') {
                        zeichen.next();
                        wert.push('"');
                        continue;
                    }
                    wert.push_str(&rest[stelle + 1..]);
                    return Cow::Owned(wert);
                }
                wert.push(c);
            }
            Cow::Owned(wert)
        }
    }
}

/// Ein Wert, wie er als Feld geschrieben wird: in Anfuehrungszeichen genau dann,
/// wenn er das Trennzeichen, `"`, `\n` oder `\r` traegt, und `"` darin
/// verdoppelt. Ein leerer Wert ist ein leeres Feld.
///
/// Das ist die eine Kodierregel; jede Handlung und die Ausgabe als TSV
/// schreiben ueber sie.
#[must_use]
pub fn feld_kodieren(wert: &str, trennzeichen: Trennzeichen) -> Cow<'_, str> {
    let trenn = trennzeichen.byte();
    let braucht_quote = wert
        .bytes()
        .any(|b| b == trenn || b == b'"' || b == b'\n' || b == b'\r');
    if !braucht_quote {
        return Cow::Borrowed(wert);
    }
    let mut kodiert = String::with_capacity(wert.len() + 2);
    kodiert.push('"');
    kodiert.push_str(&wert.replace('"', "\"\""));
    kodiert.push('"');
    Cow::Owned(kodiert)
}
