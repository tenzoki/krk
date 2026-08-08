//! Der Zeilenindex: aus einem Text der Anfangsversatz jeder Zeile, aus einer
//! Zeilennummer der Versatz und aus einem Versatz die Zeilennummer (C5, C6).
//!
//! # Eine Zeile faengt am Textanfang an und hinter jedem `\n`
//!
//! Daraus folgt, was auf den ersten Blick nach Zaehlkunst aussieht: ein Text,
//! der auf `\n` endet, hat danach eine **leere letzte Zeile**. Sie ist die
//! Stelle, an der die Schreibmarke steht, wenn der Nutzer am Ende einer
//! solchen Datei weiterschreibt; die `NSTextView` zeigt sie, und ein Index,
//! der sie nicht kennte, wiese den Zeilensprung an das Ende der vorletzten
//! Zeile. Der leere Text hat genau eine Zeile, naemlich die leere erste.
//!
//! # Nur `\n`, und das ist eine Zusage von anderswo
//!
//! Der Index kennt ein einziges Zeilenende. Er darf das, weil der gehaltene
//! Stand des Editors keines anderen traegt: das Einlesen macht `\r\n` und
//! einzelne `\r` zu `\n`, entschieden am 260808-0021
//! (`decisions/260808-0021_*_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`).
//! Ein Index, der beide Formen selbst behandelte, waere die zweite Stelle mit
//! einer Meinung darueber, was eine Zeile beendet.
//!
//! # Zeilennummern zaehlen ab 1
//!
//! Der Nutzer gibt sie ein und liest sie, und keine Zeile heisst dort 0. Die
//! beiden Faelle daneben, die Nummer 0 und eine Nummer ueber der Zeilenzahl,
//! sind deshalb keine Fehler, sondern zwei benannte Lagen in
//! [`Zeilenlage`]: der Sprung fuehrt trotzdem irgendwohin, und der Aufrufer
//! erfaehrt am Kennzeichen, dass er zu melden hat.
//!
//! **Die Regel fuer eine zu grosse Nummer steht hier und nur hier.** C5 sagt
//! sie fuer den Zeilensprung zu, C6 benutzt sie fuer eine Textmarke, deren
//! gemerkte Zeile in einer inzwischen gekuerzten Datei nicht mehr existiert.
//! Der Spec sagt ausdruecklich, dass daneben kein zweiter Weg entsteht.

/// Wo die Zeilennummer lag, nach der gefragt wurde.
///
/// Die Aufzaehlung ist vollstaendig und hat keinen Auffangzweig: eine
/// Zeilennummer liegt unter der ersten Zeile, auf einer Zeile oder ueber der
/// letzten, und ein vierter Fall entstuende nur mit einer vierten Zaehlweise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zeilenlage {
    /// Die Nummer bezeichnet eine Zeile des Textes.
    Getroffen,
    /// Die Nummer war 0 und liegt damit unter der ersten Zeile. Der Sprung
    /// fuehrt an den Textanfang.
    VorDerErsten,
    /// Die Nummer lag ueber der Zeilenzahl. Der Sprung fuehrt an das
    /// Textende, und C5 verlangt, dass der Aufrufer den Grund meldet, statt
    /// kommentarlos nichts zu tun.
    HinterDerLetzten,
}

/// Wohin ein Zeilensprung fuehrt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zeilensprung {
    /// Der Byteversatz im Text, auf einer Zeichengrenze.
    pub versatz: usize,
    /// Ob die gefragte Nummer eine Zeile bezeichnete.
    pub lage: Zeilenlage,
}

/// Der Anfangsversatz jeder Zeile eines Textes.
///
/// Er entsteht einmal ueber den ganzen Text und beantwortet danach jede Frage
/// ohne einen weiteren Durchlauf: [`Zeilenindex::anfang_der_zeile`] greift
/// zu, [`Zeilenindex::zeile_am_versatz`] sucht binaer. Wer den Text aendert,
/// baut einen neuen Index; ein Index, der sich selbst nachfuehrte, muesste
/// wissen, was sich geaendert hat, und das weiss die Textflaeche und nicht er.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeilenindex {
    /// Der Byteversatz, an dem jede Zeile beginnt. Nie leer: die erste Zeile
    /// beginnt bei 0, auch im leeren Text.
    anfaenge: Vec<usize>,
    /// Die Laenge des Textes in Bytes, also das Textende.
    laenge: usize,
}

impl Zeilenindex {
    /// Baut den Index ueber den ganzen Text.
    pub fn neu(text: &str) -> Self {
        let mut anfaenge = vec![0];
        anfaenge.extend(text.match_indices('\n').map(|(stelle, _)| stelle + 1));
        Self {
            anfaenge,
            laenge: text.len(),
        }
    }

    /// Wie viele Zeilen der Text hat, die leere letzte mitgezaehlt.
    pub fn zeilenzahl(&self) -> usize {
        self.anfaenge.len()
    }

    /// Der Versatz, an dem die Zeile `nummer` beginnt, samt der Lage der
    /// Nummer.
    ///
    /// `nummer` zaehlt ab 1. Die 0 fuehrt an den Textanfang, eine Nummer ueber
    /// der Zeilenzahl an das Textende; beide Faelle stehen im Kennzeichen
    /// [`Zeilensprung::lage`], damit der Aufrufer sie melden kann.
    pub fn anfang_der_zeile(&self, nummer: usize) -> Zeilensprung {
        if nummer == 0 {
            return Zeilensprung {
                versatz: 0,
                lage: Zeilenlage::VorDerErsten,
            };
        }
        match self.anfaenge.get(nummer - 1) {
            Some(versatz) => Zeilensprung {
                versatz: *versatz,
                lage: Zeilenlage::Getroffen,
            },
            // Das Textende und nicht der Anfang der letzten Zeile: bei einem
            // Text ohne abschliessenden Umbruch sind beide verschieden, und
            // C5 sagt "springt an das Dateiende" zu.
            None => Zeilensprung {
                versatz: self.laenge,
                lage: Zeilenlage::HinterDerLetzten,
            },
        }
    }

    /// Die Nummer der Zeile, in der `versatz` liegt, ab 1 gezaehlt.
    ///
    /// Ein Versatz hinter dem Textende liefert die letzte Zeile: er entsteht
    /// nur aus einem ueberholten Stand, und die letzte Zeile ist die
    /// nachweisbar naechstgelegene Antwort.
    pub fn zeile_am_versatz(&self, versatz: usize) -> usize {
        // `partition_point` zaehlt die Anfaenge bis einschliesslich des
        // eigenen, und das ist bereits die ab 1 gezaehlte Nummer. Sie ist nie
        // 0, weil die erste Zeile bei 0 beginnt und damit jeden Versatz
        // mitzaehlt.
        self.anfaenge.partition_point(|anfang| *anfang <= versatz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn der_leere_text_hat_eine_zeile() {
        let index = Zeilenindex::neu("");
        assert_eq!(index.zeilenzahl(), 1);
        assert_eq!(
            index.anfang_der_zeile(1),
            Zeilensprung {
                versatz: 0,
                lage: Zeilenlage::Getroffen,
            }
        );
    }

    #[test]
    fn ein_abschliessender_umbruch_oeffnet_eine_leere_letzte_zeile() {
        let index = Zeilenindex::neu("eins\nzwei\n");
        assert_eq!(index.zeilenzahl(), 3);
        assert_eq!(index.anfang_der_zeile(3).versatz, 10);
        assert_eq!(index.anfang_der_zeile(3).lage, Zeilenlage::Getroffen);
    }

    #[test]
    fn ohne_abschliessenden_umbruch_endet_der_text_in_der_letzten_zeile() {
        let index = Zeilenindex::neu("eins\nzwei");
        assert_eq!(index.zeilenzahl(), 2);
        // Der Sprung hinter die letzte Zeile fuehrt an das Textende und nicht
        // an den Anfang der zweiten Zeile.
        assert_eq!(
            index.anfang_der_zeile(3),
            Zeilensprung {
                versatz: 9,
                lage: Zeilenlage::HinterDerLetzten,
            }
        );
    }

    #[test]
    fn ein_versatz_findet_seine_zeile_und_zurueck() {
        let text = "eins\nzwei\ndrei";
        let index = Zeilenindex::neu(text);
        assert_eq!(index.zeile_am_versatz(0), 1);
        assert_eq!(
            index.zeile_am_versatz(4),
            1,
            "der Umbruch gehoert zu Zeile 1"
        );
        assert_eq!(index.zeile_am_versatz(5), 2);
        assert_eq!(index.zeile_am_versatz(text.len()), 3);
        assert_eq!(
            index.zeile_am_versatz(text.len() + 100),
            3,
            "ein ueberholter Versatz landet in der letzten Zeile"
        );
    }
}
