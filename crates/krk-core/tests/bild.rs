//! Abnahme des Lesers des Aufnahmedatums (Schritt 4 des Plans der Bildfolge).
//!
//! Die Pruefbilder unter `tests/bilder/` sind 64 × 48 Pixel gross und tragen,
//! wo sie eines tragen, das `DateTimeOriginal` `2008:03:14 15:09:26`. Wie sie
//! entstanden sind, beschreibt die Klaerung
//! `260929-1441-klaerung-aufnahmedatum-ohne-c.md` des Arbeitspakets
//! `260929-1415-vorschau-blaettert-fotos-nach-aufnahmedatum` unter (e): ein
//! Swift-Werkzeug ueber ImageIO und Byteumbauten in Python. Ins Repository
//! gehoeren die Dateien und nicht ihre Erzeuger.
//!
//! Die Grenzfaelle brauchen keine Pruefdatei: sie entstehen zur Laufzeit im
//! Pruefordner oder im Speicher.

mod gemeinsam;

use std::io::{self, Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::Duration;

use gemeinsam::{Pruefordner, mit_zeitschranke};
use krk_core::bild::aufnahmedatum::{Aufnahmezeit, aufnahmedatum, aus_quelle};
use krk_core::leseprofil::HOECHSTENS_BYTES_JE_FOTO;

/// Das Datum, das jedes Pruefbild mit Datum traegt.
fn erwartet() -> Aufnahmezeit {
    Aufnahmezeit::neu(2008, 3, 14, 15, 9, 26).expect("das Datum ist gueltig")
}

fn bild(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/bilder")
        .join(name)
}

#[test]
fn jedes_pruefbild_mit_datum_liefert_es() {
    for name in [
        "klein-mit-datum.jpg",
        "klein-mit-datum.heic",
        "klein-exif-am-ende.heic",
        "klein-meta-hinter-mdat.heic",
        "klein-mit-datum-ohne-xmp.png",
        // Unter der Grenze gelesen, obwohl `eXIf` hinter den Bilddaten steht.
        "klein-exif-hinter-idat-ohne-xmp.png",
        // Unter der Grenze gelesen, obwohl die Kiste TIFF ganz liest.
        "klein-mit-datum.tif",
    ] {
        assert_eq!(aufnahmedatum(&bild(name)), Some(erwartet()), "{name}");
    }
}

#[test]
fn ein_pruefbild_ohne_datum_liefert_keines() {
    for name in [
        "klein-ohne-datum.jpg",
        "klein-ohne-exif.jpg",
        "klein-ohne-datum.heic",
        "klein-ohne-datum.png",
        "klein-ohne-datum.tif",
    ] {
        assert_eq!(aufnahmedatum(&bild(name)), None, "{name}");
    }
}

/// `gif`, `bmp` und `icns` werden nicht geoeffnet. Belegt an Dateien, die
/// unter diesen Endungen ein JPEG mit Datum tragen: wuerden sie geoeffnet,
/// kaeme das Datum heraus.
#[test]
fn gif_bmp_und_icns_oeffnen_keine_datei() {
    let ordner = Pruefordner::neu("bild-ohne-exif-endung");
    let jpeg = std::fs::read(bild("klein-mit-datum.jpg")).expect("das Pruefbild liegt da");
    assert_eq!(
        aufnahmedatum(&ordner.datei("als.jpg", &jpeg)),
        Some(erwartet())
    );
    for name in ["verkleidet.gif", "verkleidet.bmp", "verkleidet.ICNS"] {
        assert_eq!(aufnahmedatum(&ordner.datei(name, &jpeg)), None, "{name}");
    }
    for name in [
        "klein-mit-datum.gif",
        "klein-mit-datum.bmp",
        "klein-ohne-datum.icns",
    ] {
        assert_eq!(aufnahmedatum(&bild(name)), None, "{name}");
    }
}

#[test]
fn die_endung_zaehlt_ohne_ruecksicht_auf_die_schreibung() {
    let ordner = Pruefordner::neu("bild-grosse-endung");
    let jpeg = std::fs::read(bild("klein-mit-datum.jpg")).expect("das Pruefbild liegt da");
    assert_eq!(
        aufnahmedatum(&ordner.datei("IMG_0970.JPG", &jpeg)),
        Some(erwartet())
    );
}

/// Eine abgeschnittene, eine leere Datei, ein Ordner und eine benannte Roehre
/// liefern `None` und kehren zurueck (C5.5).
#[test]
fn was_sich_nicht_lesen_laesst_liefert_none_und_kehrt_zurueck() {
    let ordner = Pruefordner::neu("bild-unlesbar");
    let jpeg = std::fs::read(bild("klein-mit-datum.jpg")).expect("das Pruefbild liegt da");
    // Das Datum steht im Pruefbild ab Byte 98; jede Laenge hier schneidet
    // davor oder mitten hinein.
    for laenge in [0, 2, 20, 60, 100] {
        let name = format!("abgeschnitten-{laenge}.jpg");
        assert_eq!(
            aufnahmedatum(&ordner.datei(&name, &jpeg[..laenge])),
            None,
            "{name}"
        );
    }
    assert_eq!(aufnahmedatum(&ordner.ordner("ordner.jpg")), None);
    assert_eq!(aufnahmedatum(&ordner.unter("gibt-es-nicht.jpg")), None);

    let roehre = ordner.roehre("roehre.jpg");
    let antwort = mit_zeitschranke("das Lesen der Roehre", Duration::from_secs(5), move || {
        aufnahmedatum(&roehre)
    });
    assert_eq!(antwort, None);
}

/// Ein Leser, der mitzaehlt, wie viele Bytes durch ihn gehen.
struct Zaehler<R> {
    innen: R,
    gelesen: u64,
}

impl<R: Read> Read for Zaehler<R> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        let geliefert = self.innen.read(puffer)?;
        self.gelesen += geliefert as u64;
        Ok(geliefert)
    }
}

impl<R: Seek> Seek for Zaehler<R> {
    fn seek(&mut self, stelle: SeekFrom) -> io::Result<u64> {
        self.innen.seek(stelle)
    }
}

fn gezaehlt(bytes: Vec<u8>) -> (Option<Aufnahmezeit>, u64) {
    let mut zaehler = Zaehler {
        innen: Cursor::new(bytes),
        gelesen: 0,
    };
    let antwort = aus_quelle(&mut zaehler);
    (antwort, zaehler.gelesen)
}

/// Ein JPEG ohne Exif laesst die Kiste bis ans Dateiende suchen; der
/// Begrenzer haelt sie an der Grenze an.
#[test]
fn ein_leser_der_ueber_die_grenze_greifen_muesste_liefert_none() {
    let mut ohne_ende = vec![0xFF, 0xD8];
    ohne_ende.resize(1024 * 1024 + 2, 0);
    let (antwort, gelesen) = gezaehlt(ohne_ende);
    assert_eq!(antwort, None);
    assert!(
        gelesen <= HOECHSTENS_BYTES_JE_FOTO,
        "{gelesen} Bytes gelesen, die Grenze ist {HOECHSTENS_BYTES_JE_FOTO}"
    );
}

/// Eine TIFF-Datei ueber der Grenze ordnet nach dem Aenderungsdatum, weil die
/// Kiste TIFF ganz liest (Nutzerentscheid vom 260929).
#[test]
fn eine_tiff_ueber_der_grenze_liefert_kein_datum() {
    let mut tiff = std::fs::read(bild("klein-mit-datum.tif")).expect("das Pruefbild liegt da");
    let (unter, _) = gezaehlt(tiff.clone());
    assert_eq!(unter, Some(erwartet()), "unter der Grenze liest sie");

    tiff.resize(
        usize::try_from(HOECHSTENS_BYTES_JE_FOTO).expect("passt") + 4096,
        0,
    );
    let (ueber, gelesen) = gezaehlt(tiff);
    assert_eq!(ueber, None);
    assert!(gelesen <= HOECHSTENS_BYTES_JE_FOTO);
}

/// Ein JPEG mit Exif kommt mit einem Bruchteil der Grenze aus.
#[test]
fn ein_jpeg_mit_exif_liest_wenig() {
    let mut jpeg = std::fs::read(bild("klein-mit-datum.jpg")).expect("das Pruefbild liegt da");
    jpeg.resize(4 * 1024 * 1024, 0);
    let (antwort, gelesen) = gezaehlt(jpeg);
    assert_eq!(antwort, Some(erwartet()));
    assert!(gelesen <= 16 * 1024, "{gelesen} Bytes gelesen");
}

#[test]
fn das_aenderungsdatum_kommt_in_dieselbe_form() {
    let ordner = Pruefordner::neu("bild-aenderungsdatum");
    let pfad = ordner.datei("x.jpg", b"");
    let geaendert = std::fs::metadata(&pfad)
        .and_then(|angaben| angaben.modified())
        .expect("das Aenderungsdatum ist lesbar");
    let zeit = Aufnahmezeit::aus_zeitpunkt(geaendert).expect("ein heutiger Zeitpunkt");
    assert!(zeit.jahr() >= 2024);
}
