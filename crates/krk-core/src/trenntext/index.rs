//! Der eine Leser: RFC 4180 mit benannter Nachsicht, und jeder Satz als
//! Byte-Bereich (E3).
//!
//! # Die Leseregel
//!
//! - Ein Satz endet an `\n` oder `\r\n` ausserhalb von Anfuehrungszeichen. Ein
//!   einzelnes `\r` ist ein Zeichen des Feldes.
//! - Ein Feld, das mit `"` beginnt, darf Trennzeichen, `""` fuer `"` und
//!   Zeilenumbrueche tragen. Ein `"` an anderer Stelle eroeffnet nichts; die
//!   Formen stehen in [`super::feld`].
//! - Das letzte Zeilenende des Textes eroeffnet keinen leeren Satz; eine
//!   Leerzeile mitten im Text ist ein Satz mit einem leeren Feld. Ein leerer
//!   Text hat keinen Satz.
//! - **Ein Anfuehrungszeichen, das bis zum Ende nicht schliesst, wird gemeldet
//!   und nicht still gelesen** ([`Satzindex::unterminiert`]); der Satz reicht
//!   dann bis zum Ende des Textes, und keine Handlung rechnet auf einem solchen
//!   Index.
//!
//! # Der Vorspann
//!
//! Vor dem ersten Satz steht der Vorspann, der keine Zeile der Tabelle ist:
//! eine Bytefolgenmarke am Anfang und, wenn der Dialekt es sagt, eine Zeile
//! `sep=X` samt ihrem Zeilenende. Beide bleiben Byte fuer Byte stehen, weil
//! keine Handlung in den Vorspann schreibt. Die Vorschau zerlegt die Bytes der
//! Datei samt Marke und `\r\n`, der Editor seinen gehaltenen Stand ohne Marke
//! und mit `\n`; es ist derselbe Leser.
//!
//! # Warum Bytes und kein Zeichendurchlauf
//!
//! Alle Zeichen, an denen die Zerlegung entscheidet (`"`, `\n`, `\r` und die
//! drei Trennzeichen), sind ASCII, und in UTF-8 kommt kein ASCII-Byte als Teil
//! eines laengeren Zeichens vor. Jeder Bereich, den der Leser meldet, liegt
//! deshalb an Zeichengrenzen, und der Durchlauf ist eine lineare Folge von
//! Bytevergleichen.

use std::ops::Range;

use super::dialekt::{Dialekt, Trennzeichen};
use super::feld::{Feld, Feldform, wert_von};

/// Die Bytefolgenmarke, die der Leser als Vorspann uebergeht.
pub(crate) const BOM: char = '\u{feff}';

/// Was ein Durchlauf ueber einen Satz ergibt.
pub(crate) struct Satzlauf {
    /// Wo der Inhalt des Satzes endet, vor seinem Zeilenende.
    pub(crate) inhalt_ende: usize,
    /// Wo der naechste Satz beginnt, hinter dem Zeilenende; am Ende des Textes
    /// dessen Laenge.
    pub(crate) naechster: usize,
    /// Ob ein Anfuehrungszeichen bis zum Ende des Textes nicht schliesst.
    pub(crate) offen: bool,
}

/// Liest einen Satz ab `start` und meldet jedes Feld mit Bereich und Form an
/// `je_feld`.
///
/// Das ist die eine Zerlegung; [`indexieren`], [`Satzindex::felder`], die Wahl
/// des Trennzeichens und die Kuerzung der Vorschau laufen alle hierueber.
pub(crate) fn satz_abtasten(
    bytes: &[u8],
    start: usize,
    trenn: u8,
    mut je_feld: impl FnMut(Range<usize>, Feldform),
) -> Satzlauf {
    let mut i = start;
    loop {
        let feldanfang = i;
        let mut form = Feldform::Glatt;
        if bytes.get(i) == Some(&b'"') {
            form = Feldform::Gequotet;
            i += 1;
            loop {
                match bytes.get(i) {
                    None => {
                        je_feld(feldanfang..i, Feldform::Nachsichtig);
                        return Satzlauf {
                            inhalt_ende: i,
                            naechster: i,
                            offen: true,
                        };
                    }
                    Some(b'"') if bytes.get(i + 1) == Some(&b'"') => i += 2,
                    Some(b'"') => {
                        i += 1;
                        break;
                    }
                    Some(_) => i += 1,
                }
            }
        }
        // Der ungequotete Teil: das ganze Feld, wenn es glatt beginnt, sonst
        // was hinter dem schliessenden `"` steht.
        loop {
            let Some(&c) = bytes.get(i) else {
                je_feld(feldanfang..i, form);
                return Satzlauf {
                    inhalt_ende: i,
                    naechster: i,
                    offen: false,
                };
            };
            if c == trenn {
                je_feld(feldanfang..i, form);
                i += 1;
                break;
            }
            if c == b'\n' {
                je_feld(feldanfang..i, form);
                return Satzlauf {
                    inhalt_ende: i,
                    naechster: i + 1,
                    offen: false,
                };
            }
            if c == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
                je_feld(feldanfang..i, form);
                return Satzlauf {
                    inhalt_ende: i,
                    naechster: i + 2,
                    offen: false,
                };
            }
            if form == Feldform::Gequotet || c == b'"' {
                form = Feldform::Nachsichtig;
            }
            i += 1;
        }
    }
}

/// Ob ab `start` eine Zeile genau `sep=X` steht, mit `X` aus den drei
/// Trennzeichen und danach `\n`, `\r\n` oder das Ende; dann das Zeichen und
/// das Ende der Zeile samt Zeilenende.
pub(crate) fn sep_zeile(bytes: &[u8], start: usize) -> Option<(Trennzeichen, usize)> {
    let rest = bytes.get(start..)?;
    let nach_sep = rest.strip_prefix(b"sep=")?;
    let (&zeichen, danach) = nach_sep.split_first()?;
    let trennzeichen = Trennzeichen::aus_byte(zeichen)?;
    let ende = start + 5;
    let laenge = if danach.is_empty() {
        0
    } else if danach.starts_with(b"\n") {
        1
    } else if danach.starts_with(b"\r\n") {
        2
    } else {
        return None;
    };
    Some((trennzeichen, ende + laenge))
}

/// Ein gelesener Trenntext: Vorspann, Saetze als Byte-Bereiche, Spaltenzahl
/// und ein unterminierter Satz.
///
/// Er entsteht allein ueber [`indexieren`] und gilt fuer genau den Text, aus
/// dem er entstand. Jede Handlung prueft das an der Laenge, der einzigen
/// Angabe, die sich ohne zweiten Durchlauf pruefen laesst; ein veralteter
/// Index gleicher Laenge entgeht ihr, und der Rufer haelt deshalb Text und
/// Index zusammen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Satzindex {
    vorspann: Range<usize>,
    saetze: Vec<Range<usize>>,
    spaltenzahl: usize,
    unterminiert: Option<usize>,
    trennzeichen: Trennzeichen,
    laenge: usize,
}

/// Liest `text` im `dialekt` nach der Regel des Modulkopfs.
///
/// Linear in der Laenge des Textes; je Satz bleibt ein Bereich, die Felder
/// liest [`Satzindex::felder`] erst auf Anfrage.
#[must_use]
pub fn indexieren(text: &str, dialekt: &Dialekt) -> Satzindex {
    let bytes = text.as_bytes();
    let trenn = dialekt.trennzeichen.byte();
    let mut stelle = if text.starts_with(BOM) {
        BOM.len_utf8()
    } else {
        0
    };
    if dialekt.sep_zeile
        && let Some((_, ende)) = sep_zeile(bytes, stelle)
    {
        stelle = ende;
    }
    let vorspann = 0..stelle;

    let mut saetze = Vec::new();
    let mut spaltenzahl = 1;
    let mut unterminiert = None;
    while stelle < bytes.len() {
        let mut felder = 0;
        let lauf = satz_abtasten(bytes, stelle, trenn, |_, _| felder += 1);
        saetze.push(stelle..lauf.inhalt_ende);
        spaltenzahl = spaltenzahl.max(felder);
        if lauf.offen && unterminiert.is_none() {
            unterminiert = Some(saetze.len() - 1);
        }
        stelle = lauf.naechster;
    }

    Satzindex {
        vorspann,
        saetze,
        spaltenzahl,
        unterminiert,
        trennzeichen: dialekt.trennzeichen,
        laenge: text.len(),
    }
}

impl Satzindex {
    /// Der Vorspann: Bytefolgenmarke und `sep=`-Zeile samt Zeilenende, sonst
    /// leer.
    #[must_use]
    pub fn vorspann(&self) -> Range<usize> {
        self.vorspann.clone()
    }

    /// Die Saetze als Byte-Bereiche, je ohne Zeilenende.
    #[must_use]
    pub fn saetze(&self) -> &[Range<usize>] {
        &self.saetze
    }

    /// Wie viele Saetze, also Zeilen der Tabelle, der Text traegt.
    #[must_use]
    pub fn zeilenzahl(&self) -> usize {
        self.saetze.len()
    }

    /// Die groesste Feldzahl aller Saetze, mindestens 1 (E7).
    #[must_use]
    pub fn spaltenzahl(&self) -> usize {
        self.spaltenzahl
    }

    /// Der erste Satz, in dem ein Anfuehrungszeichen bis zum Ende des Textes
    /// nicht schliesst.
    #[must_use]
    pub fn unterminiert(&self) -> Option<usize> {
        self.unterminiert
    }

    /// Das Trennzeichen, in dem gelesen wurde.
    #[must_use]
    pub fn trennzeichen(&self) -> Trennzeichen {
        self.trennzeichen
    }

    /// Die Felder des Satzes `satz`, oder `None`, wenn es ihn nicht gibt.
    ///
    /// # Panics
    ///
    /// Wenn `text` nicht der Text ist, aus dem der Index entstand (an der
    /// Laenge geprueft).
    #[must_use]
    pub fn felder<'a>(&self, text: &'a str, satz: usize) -> Option<Vec<Feld<'a>>> {
        self.pruefen(text);
        let bereich = self.saetze.get(satz)?;
        let mut felder = Vec::new();
        // Der Satz ist schon begrenzt; der Durchlauf liest ihn auf dem
        // Ausschnitt, damit er am Satzende und nicht am Textende anhaelt, und
        // verschiebt die Bereiche zurueck in den ganzen Text.
        let ausschnitt = &text.as_bytes()[..bereich.end];
        let _ = satz_abtasten(
            ausschnitt,
            bereich.start,
            self.trennzeichen.byte(),
            |roh, form| {
                felder.push(Feld {
                    wert: wert_von(&text[roh.clone()], form),
                    roh,
                    form,
                });
            },
        );
        Some(felder)
    }

    /// Die rohen Bereiche der Felder des Satzes `satz`, ohne ihre Werte zu
    /// bilden; fuer die Handlungen, die allein Stellen brauchen.
    pub(crate) fn feldbereiche(&self, text: &str, satz: usize) -> Vec<Range<usize>> {
        self.pruefen(text);
        let Some(bereich) = self.saetze.get(satz) else {
            return Vec::new();
        };
        let mut felder = Vec::new();
        let ausschnitt = &text.as_bytes()[..bereich.end];
        let _ = satz_abtasten(
            ausschnitt,
            bereich.start,
            self.trennzeichen.byte(),
            |roh, _| {
                felder.push(roh);
            },
        );
        felder
    }

    /// Das Zeilenende hinter dem Satz `satz`: `\n`, `\r\n` oder leer am Ende
    /// des Textes.
    pub(crate) fn zeilenende(&self, satz: usize) -> Range<usize> {
        let ende = self.saetze[satz].end;
        let naechster = self
            .saetze
            .get(satz + 1)
            .map_or(self.laenge, |folgend| folgend.start);
        ende..naechster
    }

    /// Haelt an, wenn `text` erkennbar nicht der Text dieses Index ist.
    pub(crate) fn pruefen(&self, text: &str) {
        assert_eq!(
            self.laenge,
            text.len(),
            "der Satzindex gehoert zu einem anderen Text"
        );
    }
}
