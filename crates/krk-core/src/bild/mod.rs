//! Was als Bild gilt: die eine Liste der Bildendungen.
//!
//! Die Vorschau zeigt eine Datei mit einer dieser Endungen als Bild (C6 der
//! Runde 1), und die Bildfolge eines Leseprofils nimmt genau diese Dateien als
//! Fotos. **Die Liste speist beide und steht nirgends sonst**: bis zur Arbeit
//! an der Bildfolge stand sie als `BILDENDUNGEN` in `krk-ui` und war dort fuer
//! den Kern unerreichbar, und eine zweite Fassung im Kern waere die Stelle, an
//! der beide auseinanderlaufen. Wer eine Endung hinzufuegt, fuegt sie hier
//! hinzu, und die Vorschau zeigt sie und die Bildfolge nimmt sie im selben
//! Zug.
//!
//! Verglichen wird ohne Ruecksicht auf Gross- und Kleinschreibung. Die Liste
//! nennt, was `NSImage` auf jedem macOS dieser Anwendung liest; ein Format, das
//! die Dekodierung dann doch nicht nimmt, faellt in der Ansicht auf die
//! Metadaten zurueck.
//!
//! Daneben steht [`aufnahmedatum()`], der Leser des Aufnahmedatums, nach dem die
//! Bildfolge ihre Fotos ordnet.

use std::path::Path;

pub mod aufnahmedatum;

pub use aufnahmedatum::{Aufnahmezeit, Datumsleser, aufnahmedatum};

/// Die Dateiendungen, die als gaengige Bildformate gelten, klein geschrieben.
pub const ENDUNGEN: [&str; 10] = [
    "png", "jpg", "jpeg", "gif", "tif", "tiff", "heic", "heif", "bmp", "icns",
];

/// Ob der Name auf eine der [`ENDUNGEN`] endet, ohne Ruecksicht auf Gross- und
/// Kleinschreibung.
///
/// Die Endung ist, was [`Path::extension`] liefert: der Teil hinter dem
/// letzten Punkt, und ein Name, der allein aus einem Punkt und einer Endung
/// besteht (`.jpg`), traegt keine.
#[must_use]
pub fn ist_fotoname(name: &str) -> bool {
    Path::new(name).extension().is_some_and(|endung| {
        let klein = endung.to_string_lossy().to_ascii_lowercase();
        ENDUNGEN.contains(&klein.as_str())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die_endung_zaehlt_ohne_ruecksicht_auf_die_schreibung() {
        assert!(ist_fotoname("IMG_0970.JPG"));
        assert!(ist_fotoname("bild.jpeg"));
        assert!(ist_fotoname("a.b.HeIc"));
        assert!(!ist_fotoname("notiz.txt"));
        assert!(!ist_fotoname("jpg"));
        assert!(!ist_fotoname(".jpg"));
        assert!(!ist_fotoname("bild.jpg.txt"));
    }
}
