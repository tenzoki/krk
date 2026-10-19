//! Trenntext: CSV und TSV, gelesen als Saetze, deren Felder ein Trennzeichen
//! scheidet, und bearbeitet als Ersetzung genau der Bytes, die eine Handlung
//! beruehrt.
//!
//! Plan `261018-0822_*_plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md`
//! im Arbeitspaket `261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor`,
//! Schritt 1. Die Nummern E1 bis E20 in den Koepfen dieses Moduls sind die
//! Entscheidungen jenes Plans.
//!
//! # Die Teile
//!
//! - [`Art::aus_namen`] entscheidet an der Endung, ob ein Name CSV oder TSV
//!   meint (E1), am Namen und ohne Systemaufruf.
//! - [`dialekt`] waehlt das Trennzeichen einer `.csv` nach einer festen Regel
//!   (E2). Eine Erkennung behauptet es nicht: `1,5;2` ist mit Komma wie mit
//!   Semikolon gueltig, und die Wahl ist deshalb eine Regel, die sich
//!   nachpruefen und wechseln laesst.
//! - [`index`] liest nach RFC 4180 mit benannter Nachsicht (E3) und haelt jeden
//!   Satz als Byte-Bereich des gelesenen Textes. Es ist **der eine Leser**; die
//!   Vorschau, der Editor, die Handlungen und das Einfuegen aus der
//!   Zwischenablage zerlegen ueber ihn und ueber keinen zweiten.
//! - [`feld`] traegt die Form eines Feldes und die eine Kodierregel
//!   [`feld_kodieren`] (E5).
//! - [`handlung`] rechnet jede Tabellenhandlung als reine Funktion vom alten
//!   Text auf einen [`Trenntextstand`] (E5, E7).
//! - [`bereich`] gibt einen Zellbereich als TSV aus, liest TSV zurueck (E11,
//!   E12) und sagt, wo ein gekuerzter Text am letzten vollstaendigen Satz
//!   endet (E17).
//!
//! # Was dieses Modul nicht tut
//!
//! Es oeffnet keine Datei und stellt keinen Systemaufruf; es rechnet allein auf
//! Text, der schon im Speicher steht. Es kennt allein UTF-8 (E4). Es bringt
//! keine fremde Kiste mit (E20): die Byte-Bereiche, die ein Spleiss braucht,
//! liefert die Kiste `csv` nicht in dieser Form, und der Leser ist eine
//! lineare Zerlegung mit drei Zustaenden.

pub mod bereich;
pub mod dialekt;
pub mod feld;
pub mod handlung;
pub mod index;

use std::ffi::OsStr;
use std::ops::Range;
use std::path::Path;

pub use bereich::{bereich_als_tsv, tsv_lesen, vollstaendig_bis};
pub use dialekt::{Dialekt, Herkunft, Trennzeichen, dialekt_erkennen};
pub use feld::{Feld, Feldform, feld_kodieren};
pub use handlung::{
    LetzteSpalte, bereich_einsetzen, spalte_einfuegen, spalten_loeschen, zeile_anhaengen,
    zeilen_einfuegen, zeilen_loeschen, zeilen_verschieben, zelle_setzen,
};
pub use index::{Satzindex, indexieren};

/// Hoechstens so viele Spalten zeigt die Tabellenform (E9); eine Datei mit
/// mehr bleibt in der Rohansicht, und die Vorschau zeigt sie als Text.
///
/// **256 und nicht die 1024, die der Plan vorschlug**, weil Schritt 3 die
/// Tabelle des Editors gemessen hat (`--release`, Referenzgeraet, 261018):
/// `NSTableView` legt Spalten nicht in linearer Zeit an, 128 Spalten kosten
/// 17 ms, 256 kosten 43 ms, 512 kosten 132 ms und 1024 kosten 466 ms, und
/// das Einrichten einer Datei mit 1024 Spalten und 1000 Zeilen dauerte samt
/// Anfangsbreiten 1,2 s. Mit 256 bleibt das Oeffnen der breitesten Tabelle
/// unter einer Zehntelsekunde fuer die Spalten. Die Messprobe ist
/// `messung_der_breitesten_tabelle` in `krk-ui/src/appkit/editor.rs`. Der
/// Leser selbst kennt keine Grenze; er liest jede Spaltenzahl und meldet sie
/// in [`Satzindex::spaltenzahl`].
pub const HOECHSTENS_SPALTEN: usize = 256;

/// Ob ein Trenntext `\r` als Satzende traegt: **er traegt kein `\n`, aber ein
/// `\r`** (Excel „CSV (Macintosh)“ und aeltere Mac-Programme).
///
/// **Die eine Regel fuer diesen Fall**, und Vorschau wie Editor fragen sie:
/// der Leser selbst kennt als Satzende allein `\n` und `\r\n` und liest ein
/// einzelnes `\r` als Zeichen des Feldes (E3). Ein solcher Text bekommt deshalb
/// vor dem Lesen jedes `\r` als `\n` ([`lesbar`]); der Editor haelt ihn so und
/// schreibt beim Sichern jedes `\n` wieder als `\r`
/// (`krk_core::text::datei::trenntext_oeffnen`). Bis zum 261019 zeigte die
/// Vorschau einen solchen Text als eine Zeile, der Editor als Tabelle
/// (`261019-0037-vorschau-und-editor-zerlegen-eine-csv-mit-einzelnem-cr-verschieden.md`).
/// Entschieden wird am Text und ohne Raten: ein Text mit einem einzigen `\n`
/// faellt nicht darunter, und seine `\r` bleiben Zeichen der Felder.
#[must_use]
pub fn cr_als_satzende(text: &str) -> bool {
    let bytes = text.as_bytes();
    !bytes.contains(&b'\n') && bytes.contains(&b'\r')
}

/// Der Text, den der Leser bekommt: unveraendert, oder bei
/// [`cr_als_satzende`] mit jedem `\r` als `\n`.
#[must_use]
pub fn lesbar(text: &str) -> std::borrow::Cow<'_, str> {
    if cr_als_satzende(text) {
        std::borrow::Cow::Owned(text.replace('\r', "\n"))
    } else {
        std::borrow::Cow::Borrowed(text)
    }
}

/// Welche Art Trenntext ein Name meint (E1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Art {
    /// `.csv`: das Trennzeichen waehlt [`dialekt_erkennen`].
    Csv,
    /// `.tsv` und `.tab`: das Trennzeichen ist fest der Tabulator.
    Tsv,
}

impl Art {
    /// Die Art, die die Endung von `name` meint, ohne Ruecksicht auf Gross- und
    /// Kleinschreibung, oder `None`.
    ///
    /// Gefragt wird allein die letzte Endung: `x.csv.gz` ist keine CSV, sondern
    /// ein gz, und erscheint erst als Eintrag `x.csv` eines Archivs als
    /// Trenntext. Ein Name, der nur aus der Endung besteht (`.csv`), traegt fuer
    /// [`Path::extension`] keine Endung und ist keine CSV.
    #[must_use]
    pub fn aus_namen(name: &OsStr) -> Option<Self> {
        let endung = Path::new(name).extension()?.to_str()?;
        if endung.eq_ignore_ascii_case("csv") {
            Some(Self::Csv)
        } else if endung.eq_ignore_ascii_case("tsv") || endung.eq_ignore_ascii_case("tab") {
            Some(Self::Tsv)
        } else {
            None
        }
    }
}

/// Ein Rechteck aus Zellen: Zeilen und Spalten der Tabelle, je als halboffener
/// Bereich. Zeile 0 ist der erste Satz nach dem Vorspann, Spalte 0 das erste
/// Feld; eine Nummernspalte der Oberflaeche zaehlt hier nicht mit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zellbereich {
    /// Die Zeilen des Rechtecks.
    pub zeilen: Range<usize>,
    /// Die Spalten des Rechtecks.
    pub spalten: Range<usize>,
}

impl Zellbereich {
    /// Das Rechteck aus genau einer Zelle.
    #[must_use]
    pub fn zelle(zeile: usize, spalte: usize) -> Self {
        Self {
            zeilen: zeile..zeile + 1,
            spalten: spalte..spalte + 1,
        }
    }
}

/// Der Stand nach einer Handlung: der ganze neue Text und die Auswahl danach,
/// wie `Neustand` bei den Eintraegen des Heimordners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trenntextstand {
    /// Der ganze neue Inhalt der Datei.
    pub text: String,
    /// Die Zellen, die nach der Handlung gewaehlt sind.
    pub auswahl: Zellbereich,
}
