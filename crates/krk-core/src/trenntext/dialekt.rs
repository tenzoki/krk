//! Das Trennzeichen einer Datei und woher die Wahl kommt (E2).
//!
//! # Die Regel fuer eine `.csv`
//!
//! In dieser Reihenfolge, und die erste Regel, die greift, entscheidet:
//!
//! 1. Ist die erste Zeile genau `sep=X` mit `X` aus Komma, Semikolon oder
//!    Tabulator (die Angabe, die Excel liest), gilt `X`. Die Zeile ist
//!    **Vorspann**: sie bleibt Byte fuer Byte stehen und ist keine Zeile der
//!    Tabelle ([`Dialekt::sep_zeile`]).
//! 2. Sonst wird der Anfang, hoechstens [`PROBE_BYTES`] und [`PROBE_SAETZE`],
//!    je Kandidat Komma, Semikolon, Tabulator zerlegt, mit Ruecksicht auf
//!    Anfuehrungszeichen. Gewaehlt wird der Kandidat, dessen haeufigste
//!    Feldzahl groesser als 1 ist und den groessten Anteil der Saetze traegt;
//!    bei Gleichstand gilt die Reihenfolge Komma, Semikolon, Tabulator. Sind
//!    zwei Feldzahlen eines Kandidaten gleich haeufig, gilt die groessere als
//!    seine haeufigste.
//! 3. Greift keine Regel, gilt Komma.
//!
//! Eine `.tsv` und eine `.tab` tragen fest den Tabulator und lesen keine
//! `sep=`-Zeile; ihre erste Zeile ist immer eine Zeile der Tabelle.
//!
//! # Warum die `sep=`-Zeile am Dialekt haengt und nicht am Text allein
//!
//! Ob die erste Zeile Vorspann ist, entscheidet das Oeffnen und nicht jede
//! spaetere Zerlegung: [`Dialekt::sep_zeile`] ist genau dann wahr, wenn die
//! Datei beim Erkennen mit einer solchen Zeile begann. Sonst machte eine
//! Handlung, die eine Zeile `sep=;` an den Anfang schiebt, aus einer
//! Datenzeile still einen Vorspann, und die Tabelle verloere eine Zeile. Wird
//! die Zeile in der Rohansicht entfernt, findet [`super::indexieren`] sie nicht
//! mehr und liest die erste Zeile wieder als Zeile.

use std::collections::BTreeMap;

use super::Art;
use super::index::{BOM, satz_abtasten, sep_zeile};

/// Hoechstens so viele Bytes vom Anfang liest die Wahl nach dem Gleichmass.
pub const PROBE_BYTES: usize = 64 * 1024;

/// Hoechstens so viele Saetze vom Anfang liest die Wahl nach dem Gleichmass.
pub const PROBE_SAETZE: usize = 200;

/// Ein Zeichen, das Felder scheidet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trennzeichen {
    /// `,`
    Komma,
    /// `;`
    Semikolon,
    /// `\t`
    Tabulator,
}

impl Trennzeichen {
    /// Die drei Trennzeichen in der Reihenfolge, die bei Gleichstand gilt und
    /// in der [`Trennzeichen::naechstes`] reihum geht.
    pub const ALLE: [Trennzeichen; 3] = [
        Trennzeichen::Komma,
        Trennzeichen::Semikolon,
        Trennzeichen::Tabulator,
    ];

    /// Das Byte des Zeichens; alle drei sind ASCII.
    #[must_use]
    pub fn byte(self) -> u8 {
        match self {
            Self::Komma => b',',
            Self::Semikolon => b';',
            Self::Tabulator => b'\t',
        }
    }

    /// Das Zeichen, das nach diesem reihum kommt: Komma, Semikolon, Tabulator,
    /// dann wieder Komma.
    #[must_use]
    pub fn naechstes(self) -> Self {
        match self {
            Self::Komma => Self::Semikolon,
            Self::Semikolon => Self::Tabulator,
            Self::Tabulator => Self::Komma,
        }
    }

    /// Das Trennzeichen zu einem Byte, oder `None`.
    #[must_use]
    pub fn aus_byte(byte: u8) -> Option<Self> {
        Self::ALLE.into_iter().find(|t| t.byte() == byte)
    }
}

/// Woher die Wahl des Trennzeichens kommt; die Oberflaeche nennt sie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herkunft {
    /// Die Endung legt es fest (`.tsv`, `.tab`).
    Endung,
    /// Die erste Zeile `sep=X` nennt es.
    SepZeile,
    /// Die Feldzahlen am Anfang der Datei sprechen dafuer.
    Gleichmass,
    /// Keine Regel greift, also gilt Komma.
    Vorgabe,
    /// Der Nutzer hat gewechselt.
    Gewechselt,
}

/// Wie ein Text zu lesen ist: Trennzeichen, Herkunft der Wahl, und ob eine
/// erste Zeile `sep=X` Vorspann ist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dialekt {
    /// Das Zeichen, das die Felder scheidet.
    pub trennzeichen: Trennzeichen,
    /// Woher die Wahl kommt.
    pub herkunft: Herkunft,
    /// Ob eine erste Zeile `sep=X` als Vorspann gelesen wird; der Modulkopf
    /// sagt, warum das am Dialekt haengt.
    pub sep_zeile: bool,
}

impl Dialekt {
    /// Der Dialekt einer `.tsv` und jedes TSV-Textes, etwa aus der
    /// Zwischenablage: Tabulator, keine `sep=`-Zeile.
    #[must_use]
    pub fn tsv() -> Self {
        Self {
            trennzeichen: Trennzeichen::Tabulator,
            herkunft: Herkunft::Endung,
            sep_zeile: false,
        }
    }

    /// Derselbe Dialekt mit dem naechsten Trennzeichen reihum (E2, Befehl
    /// „Trennzeichen wechseln“). Er aendert die Deutung und kein Byte; ein
    /// Vorspann bleibt Vorspann.
    #[must_use]
    pub fn gewechselt(self) -> Self {
        Self {
            trennzeichen: self.trennzeichen.naechstes(),
            herkunft: Herkunft::Gewechselt,
            sep_zeile: self.sep_zeile,
        }
    }
}

/// Der Dialekt, in dem `text` nach der Regel des Modulkopfs zu lesen ist.
///
/// `text` darf mit einer Bytefolgenmarke beginnen; sie wird uebergangen.
#[must_use]
pub fn dialekt_erkennen(art: Art, text: &str) -> Dialekt {
    match art {
        Art::Tsv => Dialekt::tsv(),
        Art::Csv => csv_dialekt(text),
    }
}

fn csv_dialekt(text: &str) -> Dialekt {
    let ohne_marke = text.strip_prefix(BOM).unwrap_or(text);
    let bytes = ohne_marke.as_bytes();
    if let Some((trennzeichen, _)) = sep_zeile(bytes, 0) {
        return Dialekt {
            trennzeichen,
            herkunft: Herkunft::SepZeile,
            sep_zeile: true,
        };
    }

    let mut probe_ende = bytes.len().min(PROBE_BYTES);
    while !ohne_marke.is_char_boundary(probe_ende) {
        probe_ende -= 1;
    }
    let probe = &bytes[..probe_ende];
    let gekuerzt = probe_ende < bytes.len();

    // (Kandidat, Saetze mit der haeufigsten Feldzahl, Saetze insgesamt)
    let mut bester: Option<(Trennzeichen, usize, usize)> = None;
    for kandidat in Trennzeichen::ALLE {
        let Some((treffer, gesamt)) = gleichmass(probe, gekuerzt, kandidat) else {
            continue;
        };
        let besser = match bester {
            None => true,
            // Anteile als Brueche vergleichen; streng groesser, damit bei
            // Gleichstand der fruehere Kandidat bleibt.
            Some((_, bester_treffer, bester_gesamt)) => {
                treffer * bester_gesamt > bester_treffer * gesamt
            }
        };
        if besser {
            bester = Some((kandidat, treffer, gesamt));
        }
    }

    match bester {
        Some((trennzeichen, _, _)) => Dialekt {
            trennzeichen,
            herkunft: Herkunft::Gleichmass,
            sep_zeile: false,
        },
        None => Dialekt {
            trennzeichen: Trennzeichen::Komma,
            herkunft: Herkunft::Vorgabe,
            sep_zeile: false,
        },
    }
}

/// Wie viele Saetze der Probe die haeufigste Feldzahl tragen und wie viele
/// Saetze die Probe insgesamt hat, oder `None`, wenn die haeufigste Feldzahl 1
/// ist oder die Probe keinen Satz traegt.
///
/// Ist die Probe gekuerzt, zaehlt ihr letzter Satz nicht, wenn er nicht an
/// einem Zeilenende endet: er waere ein abgeschnittener Satz.
fn gleichmass(probe: &[u8], gekuerzt: bool, kandidat: Trennzeichen) -> Option<(usize, usize)> {
    let mut haeufigkeit: BTreeMap<usize, usize> = BTreeMap::new();
    let mut gesamt = 0;
    let mut stelle = 0;
    while stelle < probe.len() && gesamt < PROBE_SAETZE {
        let mut felder = 0;
        let lauf = satz_abtasten(probe, stelle, kandidat.byte(), |_, _| felder += 1);
        let abgeschnitten = lauf.naechster == probe.len() && lauf.inhalt_ende == probe.len();
        if gekuerzt && abgeschnitten {
            break;
        }
        *haeufigkeit.entry(felder).or_default() += 1;
        gesamt += 1;
        stelle = lauf.naechster;
    }
    // Die groesste Haeufigkeit, bei gleicher die groessere Feldzahl: die
    // BTreeMap laeuft aufsteigend nach Feldzahl, und `max_by_key` behaelt bei
    // Gleichstand das letzte Element.
    let (feldzahl, treffer) = haeufigkeit.into_iter().max_by_key(|(_, anzahl)| *anzahl)?;
    (feldzahl > 1).then_some((treffer, gesamt))
}
