//! Der Fotoordner fuer die Messwege der Bildfolge (Schritt 12 des Plans der
//! Bildfolge).
//!
//! Ein Aufruf legt unter `PFAD/Fotos/2008/01` bis `12` eine Zahl kleiner, aber
//! gueltiger JPEG-Dateien an, jede mit einem APP1-Segment, das
//! `DateTimeOriginal` traegt. Daneben steht `PFAD/Fotos/2007` leer: die
//! Messung im Buendel waehlt vor jeder Wiederholung eine andere Zeile als
//! `2008` aus, und ohne eine zweite Zeile in `Fotos` gaebe es keine.
//!
//! # Was an den Fotos gewollt ist
//!
//! - **Die Aufnahmedaten laufen gegen die Namen.** Innerhalb eines Monats
//!   traegt das Foto mit dem kleinsten Namen das spaeteste Datum. Eine Folge,
//!   die nach Namen statt nach Datum ordnete, stuende damit genau verkehrt
//!   herum, und die Messung ordnet wirklich, statt eine schon geordnete Liste
//!   zu bestaetigen.
//! - **Die Groesse besteht aus echten Bytes.** Hinter dem Ende des Bildes
//!   (`FFD9`) steht Fuellung aus dem Zufallsstrom bis `--groesse`, und sie
//!   liegt auf der Platte. Anders als im Pruefordner aus [`crate::fixture`]
//!   gibt es kein Loch: der Leser des Aufnahmedatums liest die ersten Bytes
//!   einer Datei, und die Vorschau liest das ganze Foto, und beides soll so
//!   viel kosten wie an einem wirklichen Foto dieser Groesse. Ein Bildleser
//!   uebergeht, was hinter `FFD9` steht.
//! - **Die Verteilung auf die Monate haengt am Startwert** und sonst an
//!   nichts; derselbe Aufruf ergibt denselben Ordner. Welche Zahl in welchem
//!   Monat liegt, schreibt der Steckbrief.
//!
//! # Woher das Bild kommt
//!
//! Das Bild selbst ist die Pruefdatei `klein-ohne-exif.jpg` aus den Proben
//! des Kerns, 64 × 48 Bildpunkte, woertlich eingebunden. Geschrieben wird hier
//! allein das APP1-Segment, hinter das APP0-Segment der Vorlage gesetzt; ein
//! JPEG-Kodierer entsteht nicht, und eine fremde Kiste fuer das Schreiben
//! ebenso wenig.

use std::fs::{self, File, FileTimes};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use serde::Deserialize;

use crate::fixture::{self, Zufall};

/// Das Bild, in das jedes Foto sein APP1-Segment bekommt.
const VORLAGE: &[u8] = include_bytes!("../../krk-core/tests/bilder/klein-ohne-exif.jpg");

/// Der Name des Ordners, unter dem die Jahre liegen; das Muster der zwei
/// ausgelieferten Fotoprofile verlangt ihn.
pub const FOTOS: &str = "Fotos";

/// Das Jahr, dessen Monate die Fotos tragen, und die Zeile, die die Messung im
/// Buendel auswaehlt.
pub const JAHR: &str = "2008";

/// Die Nachbarzeile neben [`JAHR`], leer.
pub const NACHBAR: &str = "2007";

/// Was ein Aufruf angelegt hat.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fotoordner {
    /// Der Jahresordner, `PFAD/Fotos/2008`.
    pub jahr: PathBuf,
    /// Wie viele Fotos je Monat, Januar zuerst.
    pub je_monat: [usize; 12],
    /// Die Groesse jedes Fotos in Bytes.
    pub groesse: u64,
    /// Der Pfad des Steckbriefs.
    pub steckbrief: PathBuf,
}

/// Was neben einem Fotoordner ueber ihn festgehalten ist.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Fotosteckbrief {
    /// Wie viele Fotos der Ordner traegt.
    pub fotos: usize,
    /// Die Groesse jedes Fotos in Bytes.
    pub groesse: u64,
    /// Der Startwert, aus dem die Verteilung stammt.
    pub startwert: u64,
    /// Wie viele Fotos je Monat, Januar zuerst.
    pub je_monat: Vec<usize>,
}

/// Legt den Fotoordner unter `ziel` an.
///
/// `ziel` muss fehlen oder leer sein; der Erzeuger ueberschreibt nichts, aus
/// demselben Grund wie [`crate::fixture::erzeugen`]. Ein Foto wird nie kleiner
/// als die Vorlage samt APP1-Segment; eine kleinere `groesse` ergibt Fotos in
/// genau dieser Mindestgroesse.
pub fn erzeugen(ziel: &Path, fotos: usize, groesse: u64, startwert: u64) -> io::Result<Fotoordner> {
    fixture::pruefen_dass_leer(ziel)?;
    let jahr = ziel.join(FOTOS).join(JAHR);
    fs::create_dir_all(ziel.join(FOTOS).join(NACHBAR))?;
    for monat in 1..=12 {
        fs::create_dir_all(jahr.join(format!("{monat:02}")))?;
    }

    let mut zufall = Zufall::neu(startwert);
    // Je Foto sein Monat; die Nummer im Namen zaehlt ueber das ganze Jahr.
    let monate: Vec<usize> = (0..fotos)
        .map(|_| usize::try_from(zufall.unter(12)).unwrap_or(0))
        .collect();
    let mut je_monat = [0usize; 12];
    for monat in &monate {
        je_monat[*monat] += 1;
    }

    let mut schon = [0usize; 12];
    for (nummer, monat) in monate.iter().enumerate() {
        // Die Stelle im Monat nach Namen: die Nummer steigt mit dem Namen, also
        // ist das n-te Foto dieses Monats auch das n-te nach Namen.
        let stelle = schon[*monat];
        schon[*monat] += 1;
        let zeit = aufnahmezeit(*monat + 1, stelle, je_monat[*monat]);
        let pfad = jahr
            .join(format!("{:02}", monat + 1))
            .join(format!("IMG_{:05}.jpg", nummer + 1));
        let bytes = foto(&zeit, groesse, &mut zufall)?;
        let mut datei = File::create(&pfad)?;
        datei.write_all(&bytes)?;
        // Ein festes Aenderungsdatum, damit derselbe Aufruf denselben Ordner
        // ergibt; es liegt vor jedem Aufnahmedatum und entscheidet nichts,
        // solange die Bilddaten ihr Datum hergeben.
        let geaendert = UNIX_EPOCH + Duration::from_secs(1_199_145_600 + nummer as u64);
        datei.set_times(FileTimes::new().set_modified(geaendert))?;
    }

    let steckbrief = fixture::nebenpfad(ziel, "fotoordner.toml")?;
    let fotoordner = Fotoordner {
        jahr,
        je_monat,
        groesse: groesse.max(mindestgroesse()),
        steckbrief,
    };
    steckbrief_schreiben(&fotoordner, ziel, fotos, startwert)?;
    Ok(fotoordner)
}

/// Das Aufnahmedatum des Fotos an `stelle` (nach Namen, von null) in einem
/// Monat mit `anzahl` Fotos, als Text der Form von `DateTimeOriginal`.
///
/// Das Foto mit dem kleinsten Namen traegt das spaeteste Datum; je Stelle
/// liegt eine Minute dazwischen, beginnend am Ersten des Monats um acht Uhr.
/// Auch 7.500 Fotos in einem Monat bleiben damit innerhalb seiner ersten
/// sechs Tage.
pub fn aufnahmezeit(monat: usize, stelle: usize, anzahl: usize) -> String {
    let minuten = 8 * 60 + (anzahl - 1 - stelle);
    let tag = 1 + minuten / (24 * 60);
    let stunde = (minuten % (24 * 60)) / 60;
    let minute = minuten % 60;
    format!("{JAHR}:{monat:02}:{tag:02} {stunde:02}:{minute:02}:00")
}

/// Die kleinste Groesse eines Fotos: die Vorlage samt APP1-Segment.
fn mindestgroesse() -> u64 {
    (VORLAGE.len() + app1(&aufnahmezeit(1, 0, 1)).len()) as u64
}

/// Ein Foto mit dem genannten Aufnahmedatum, aufgefuellt auf `groesse`.
fn foto(zeit: &str, groesse: u64, zufall: &mut Zufall) -> io::Result<Vec<u8>> {
    // Die Vorlage beginnt mit SOI und APP0 (JFIF); das APP1-Segment kommt
    // unmittelbar dahinter, wie Kameras es schreiben.
    let ungueltig = || io::Error::other("die eingebundene Vorlage ist kein JPEG mit APP0-Segment");
    if VORLAGE.get(0..4) != Some(&[0xFF, 0xD8, 0xFF, 0xE0][..]) {
        return Err(ungueltig());
    }
    let app0 = usize::from(u16::from_be_bytes([VORLAGE[4], VORLAGE[5]]));
    let hinter_app0 = 4 + app0;
    if hinter_app0 > VORLAGE.len() {
        return Err(ungueltig());
    }

    let segment = app1(zeit);
    let mut bytes = Vec::with_capacity(usize::try_from(groesse).unwrap_or(0));
    bytes.extend_from_slice(&VORLAGE[..hinter_app0]);
    bytes.extend_from_slice(&segment);
    bytes.extend_from_slice(&VORLAGE[hinter_app0..]);
    let ziel = usize::try_from(groesse).unwrap_or(usize::MAX);
    while bytes.len() < ziel {
        let wort = zufall.naechste().to_le_bytes();
        let rest = (ziel - bytes.len()).min(wort.len());
        bytes.extend_from_slice(&wort[..rest]);
    }
    Ok(bytes)
}

/// Das APP1-Segment mit einem TIFF-Kopf, einer IFD0 mit dem Zeiger auf die
/// Exif-IFD und darin allein `DateTimeOriginal` (Kennung `0x9003`).
///
/// Die Versaetze zaehlen vom Anfang des TIFF-Kopfes: IFD0 bei 8, die
/// Exif-IFD bei 26, der Text des Datums bei 44. Grosse Endung (`MM`).
fn app1(zeit: &str) -> Vec<u8> {
    let mut tiff: Vec<u8> = Vec::with_capacity(64);
    tiff.extend_from_slice(b"MM\x00\x2a");
    tiff.extend_from_slice(&8u32.to_be_bytes());
    // IFD0: ein Eintrag, ExifIFDPointer (LONG) auf 26.
    tiff.extend_from_slice(&1u16.to_be_bytes());
    tiff.extend_from_slice(&0x8769u16.to_be_bytes());
    tiff.extend_from_slice(&4u16.to_be_bytes());
    tiff.extend_from_slice(&1u32.to_be_bytes());
    tiff.extend_from_slice(&26u32.to_be_bytes());
    tiff.extend_from_slice(&0u32.to_be_bytes());
    // Exif-IFD: ein Eintrag, DateTimeOriginal (ASCII, 20 Zeichen) bei 44.
    tiff.extend_from_slice(&1u16.to_be_bytes());
    tiff.extend_from_slice(&0x9003u16.to_be_bytes());
    tiff.extend_from_slice(&2u16.to_be_bytes());
    tiff.extend_from_slice(&20u32.to_be_bytes());
    tiff.extend_from_slice(&44u32.to_be_bytes());
    tiff.extend_from_slice(&0u32.to_be_bytes());
    let mut text = zeit.as_bytes().to_vec();
    text.resize(19, b' ');
    tiff.extend_from_slice(&text);
    tiff.push(0);

    let laenge = u16::try_from(2 + 6 + tiff.len()).unwrap_or(u16::MAX);
    let mut segment = Vec::with_capacity(4 + 6 + tiff.len());
    segment.extend_from_slice(&[0xFF, 0xE1]);
    segment.extend_from_slice(&laenge.to_be_bytes());
    segment.extend_from_slice(b"Exif\x00\x00");
    segment.extend_from_slice(&tiff);
    segment
}

/// Schreibt den Steckbrief neben `ziel`.
///
/// **Neben und nicht in den Ordner**, aus dem Grund, den der Steckbrief aus
/// [`crate::fixture`] nennt: in `Fotos` stuende er als dritte Zeile, und die
/// Messung waehlte ihn womoeglich als Nachbarn aus.
fn steckbrief_schreiben(
    fotoordner: &Fotoordner,
    ziel: &Path,
    fotos: usize,
    startwert: u64,
) -> io::Result<()> {
    let je_monat: Vec<String> = fotoordner.je_monat.iter().map(usize::to_string).collect();
    let inhalt = format!(
        "# Steckbrief eines KRK-Fotoordners, geschrieben von krk-bench {version}.\n\
         # Der Ordner ist aus Fotozahl, Groesse und Startwert reproduzierbar:\n\
         #   cargo run -p krk-bench -- fotoordner --fotos {fotos} --groesse {groesse} --seed {startwert} --out {ziel}\n\
         ordner = \"{ziel}\"\n\
         jahr = \"{jahr}\"\n\
         fotos = {fotos}\n\
         groesse = {groesse}\n\
         startwert = {startwert}\n\
         je_monat = [{je_monat}]\n",
        version = env!("CARGO_PKG_VERSION"),
        ziel = ziel.display(),
        jahr = fotoordner.jahr.display(),
        groesse = fotoordner.groesse,
        je_monat = je_monat.join(", "),
    );
    fs::write(&fotoordner.steckbrief, inhalt)
}

/// Liest den Steckbrief zu einem Jahresordner `PFAD/Fotos/2008`, falls einer
/// neben `PFAD` liegt.
///
/// `None` ist kein Fehler: gemessen werden darf auch ein Fotoordner des
/// Nutzers, den dieses Werkzeug nicht angelegt hat.
pub fn steckbrief_zum_jahr(jahr: &Path) -> Option<Fotosteckbrief> {
    let ziel = jahr.parent()?.parent()?;
    let pfad = fixture::nebenpfad(ziel, "fotoordner.toml").ok()?;
    toml::from_str(&fs::read_to_string(pfad).ok()?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wegwerfordner::Wegwerfordner;

    /// Die Monatsordner eines erzeugten Jahres, je mit ihren Fotos nach Namen.
    fn monate(jahr: &Path) -> Vec<Vec<PathBuf>> {
        (1..=12)
            .map(|monat| {
                let mut fotos: Vec<PathBuf> = fs::read_dir(jahr.join(format!("{monat:02}")))
                    .expect("der Monatsordner steht")
                    .map(|eintrag| eintrag.expect("der Eintrag laesst sich lesen").path())
                    .collect();
                fotos.sort();
                fotos
            })
            .collect()
    }

    /// Der Ordner liefert die zugesagte Zahl, ihre Verteilung steht im
    /// Steckbrief, und derselbe Startwert ergibt dieselbe Verteilung.
    #[test]
    fn der_fotoordner_liefert_die_zugesagte_zahl_und_verteilung() {
        let erster = Wegwerfordner::neu("fotoordner-zahl");
        let zweiter = Wegwerfordner::neu("fotoordner-zahl-wieder");
        let angelegt = erzeugen(&erster.pfad().join("platz"), 60, 4096, 7).expect("erzeugt");
        let wieder = erzeugen(&zweiter.pfad().join("platz"), 60, 4096, 7).expect("erzeugt");

        assert_eq!(angelegt.je_monat.iter().sum::<usize>(), 60);
        assert_eq!(angelegt.je_monat, wieder.je_monat, "derselbe Startwert");
        let gezaehlt: Vec<usize> = monate(&angelegt.jahr).iter().map(Vec::len).collect();
        assert_eq!(gezaehlt, angelegt.je_monat.to_vec());

        let brief = steckbrief_zum_jahr(&angelegt.jahr).expect("der Steckbrief liegt da");
        assert_eq!(brief.fotos, 60);
        assert_eq!(brief.startwert, 7);
        assert_eq!(brief.groesse, 4096);
        assert_eq!(brief.je_monat, angelegt.je_monat.to_vec());

        let fotos = erster.pfad().join("platz").join(FOTOS);
        let mut zeilen: Vec<String> = fs::read_dir(&fotos)
            .expect("Fotos steht")
            .map(|eintrag| {
                eintrag
                    .expect("lesbar")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        zeilen.sort();
        assert_eq!(
            zeilen,
            [NACHBAR, JAHR],
            "Fotos traegt genau Nachbar und Jahr"
        );
        assert_eq!(
            fs::read_dir(fotos.join(NACHBAR))
                .expect("der Nachbar steht")
                .count(),
            0,
            "der Nachbar ist leer"
        );
    }

    /// Der Leser des Aufnahmedatums aus dem Kern liest aus jedem erzeugten
    /// Foto das geschriebene Datum, und innerhalb eines Monats laufen die
    /// Daten gegen die Namen.
    #[test]
    fn der_leser_des_kerns_liest_aus_jedem_foto_das_geschriebene_datum() {
        let ordner = Wegwerfordner::neu("fotoordner-datum");
        let angelegt = erzeugen(&ordner.pfad().join("platz"), 40, 8192, 3).expect("erzeugt");

        for (index, fotos) in monate(&angelegt.jahr).iter().enumerate() {
            let mut vorher = None;
            for (stelle, pfad) in fotos.iter().enumerate() {
                assert_eq!(
                    fs::metadata(pfad).expect("das Foto steht").len(),
                    8192,
                    "{}",
                    pfad.display()
                );
                let gelesen = krk_core::bild::aufnahmedatum(pfad)
                    .unwrap_or_else(|| panic!("{} traegt kein lesbares Datum", pfad.display()));
                let text = format!(
                    "{:04}:{:02}:{:02} {:02}:{:02}:{:02}",
                    gelesen.jahr(),
                    gelesen.monat(),
                    gelesen.tag(),
                    gelesen.stunde(),
                    gelesen.minute(),
                    gelesen.sekunde()
                );
                assert_eq!(text, aufnahmezeit(index + 1, stelle, fotos.len()));
                if let Some(frueher) = vorher {
                    assert!(gelesen < frueher, "die Daten laufen nicht gegen die Namen");
                }
                vorher = Some(gelesen);
            }
        }
    }

    /// Ein belegter Zielordner wird nicht angefasst.
    #[test]
    fn ein_belegter_zielordner_wird_abgewiesen() {
        let ordner = Wegwerfordner::neu("fotoordner-belegt");
        fs::create_dir_all(ordner.pfad()).expect("angelegt");
        fs::write(ordner.pfad().join("fremd.txt"), "x").expect("geschrieben");
        assert!(erzeugen(ordner.pfad(), 3, 0, 1).is_err());
        assert!(!ordner.pfad().join(FOTOS).exists());
    }
}
