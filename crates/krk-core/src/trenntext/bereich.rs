//! Ein Zellbereich als TSV hinaus und TSV als Block herein (E11, E12), und die
//! Kuerzung eines angelesenen Textes am letzten vollstaendigen Satz (E17).
//!
//! # Warum TSV
//!
//! Numbers und Excel nehmen TSV beim Einfuegen in Zellen auf, und KRK liest
//! beim Einfuegen dasselbe. Ausgegeben wird jede Zelle ueber
//! [`feld_kodieren`] mit dem Tabulator als Trennzeichen; eine einzelne Zelle
//! ohne Tabulator, Zeilenumbruch und `"` ist damit ihr blosser Wert.
//!
//! # TSV lesen
//!
//! [`tsv_lesen`] liest ueber denselben Leser wie jede Datei, Tabulator fest
//! und ohne `sep=`-Zeile; ein abschliessendes Zeilenende eroeffnet keine Zeile.
//! **Schliesst ein Anfuehrungszeichen nicht, liest es den Text ohne
//! Anfuehrungszeichen**, je Zeile an `\n` oder `\r\n` und je Zelle am
//! Tabulator. Ein Text aus der Zwischenablage, der mit `"` beginnt, ohne als
//! gequotetes Feld gemeint zu sein, ergaebe sonst eine einzige Zelle bis zum
//! Ende; der Rueckfall fuegt ihn so ein, wie er dasteht.

use super::Zellbereich;
use super::dialekt::{Dialekt, Trennzeichen};
use super::feld::feld_kodieren;
use super::index::{Satzindex, indexieren, satz_abtasten};

/// Die Zellen des Bereichs als TSV: Zeilen durch `\n`, Zellen durch
/// Tabulator, ohne Zeilenende am Schluss. Eine Zelle hinter dem Ende ihres
/// Satzes ist leer; Zeilen hinter der letzten fallen weg.
///
/// # Panics
///
/// Wenn `text` nicht der Text ist, aus dem der Index entstand.
#[must_use]
pub fn bereich_als_tsv(text: &str, index: &Satzindex, bereich: &Zellbereich) -> String {
    let mut zeilen = Vec::new();
    for satz in bereich.zeilen.clone() {
        let Some(felder) = index.felder(text, satz) else {
            break;
        };
        let zellen: Vec<_> = bereich
            .spalten
            .clone()
            .map(|spalte| {
                let wert = felder.get(spalte).map_or("", |feld| feld.wert.as_ref());
                feld_kodieren(wert, Trennzeichen::Tabulator).into_owned()
            })
            .collect();
        zeilen.push(zellen.join("\t"));
    }
    zeilen.join("\n")
}

/// Ein TSV-Text als Block von Zeilen aus Werten; der Modulkopf nennt den
/// Rueckfall bei einem offenen Anfuehrungszeichen.
#[must_use]
pub fn tsv_lesen(text: &str) -> Vec<Vec<String>> {
    let index = indexieren(text, &Dialekt::tsv());
    if index.unterminiert().is_some() {
        let ohne_ende = text.strip_suffix('\n').unwrap_or(text);
        let ohne_ende = ohne_ende.strip_suffix('\r').unwrap_or(ohne_ende);
        return ohne_ende
            .split('\n')
            .map(|zeile| {
                let zeile = zeile.strip_suffix('\r').unwrap_or(zeile);
                zeile.split('\t').map(str::to_owned).collect()
            })
            .collect();
    }
    (0..index.zeilenzahl())
        .map(|satz| {
            index
                .felder(text, satz)
                .unwrap_or_default()
                .into_iter()
                .map(|feld| feld.wert.into_owned())
                .collect()
        })
        .collect()
}

/// Wie viele Bytes vom Anfang von `bytes` vollstaendige Saetze tragen: das
/// Ende hinter dem Zeilenende des letzten Satzes, der vor dem Ende von `bytes`
/// schliesst, mit Ruecksicht auf Zeilenumbrueche in Anfuehrungszeichen.
///
/// Fuer einen angelesenen Text, von dem mehr da ist, als gelesen wurde (E17):
/// was dahinter steht, ist ein angeschnittener Satz. Gefragt wird an Bytes,
/// weil der Schnitt mitten in einem Zeichen liegen kann; das Ergebnis liegt
/// immer hinter einem `\n` und damit an einer Zeichengrenze. Ein Text ohne
/// vollstaendigen Satz ergibt 0. Eine `sep=`-Zeile zaehlt dabei wie ein Satz;
/// sie endet ohnehin an ihrem Zeilenende.
#[must_use]
pub fn vollstaendig_bis(bytes: &[u8], trennzeichen: Trennzeichen) -> usize {
    let mut stelle = 0;
    let mut vollstaendig = 0;
    while stelle < bytes.len() {
        let lauf = satz_abtasten(bytes, stelle, trennzeichen.byte(), |_, _| {});
        if lauf.naechster == lauf.inhalt_ende {
            // Am Ende ohne Zeilenende, oder ein offenes Anfuehrungszeichen.
            break;
        }
        vollstaendig = lauf.naechster;
        stelle = lauf.naechster;
    }
    vollstaendig
}
