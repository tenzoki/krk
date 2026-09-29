//! Das Aufnahmedatum eines Fotos, gelesen aus seinen Bilddaten.
//!
//! Die Bildfolge eines Leseprofils ordnet die Fotos eines Ordners nach dem
//! Zeitpunkt der Aufnahme (C2 des Spec
//! `260929-1313_*_spec-vorschau-blaettert-fotos-nach-aufnahmedatum.md`). Diesen
//! Zeitpunkt liefert [`aufnahmedatum`] aus dem EXIF-Feld `DateTimeOriginal`,
//! gelesen mit der Kiste `kamadak-exif`; warum diese und nicht ImageIO, steht
//! in der Wurzel-`Cargo.toml` und in der Klaerung
//! `260929-1441-klaerung-aufnahmedatum-ohne-c.md` des Arbeitspakets
//! `260929-1415-vorschau-blaettert-fotos-nach-aufnahmedatum`.
//!
//! # Der Begrenzer, und warum er unter dem Puffer liegt
//!
//! Gelesen wird hoechstens [`HOECHSTENS_BYTES_JE_FOTO`] je Foto. Die Kiste
//! kennt keine Grenze, sie sucht so weit, wie ein Format es verlangt: durch die
//! Scandaten eines JPEG ohne Exif, durch jeden `IDAT`-Block eines PNG, durch
//! eine ganze TIFF-Datei. **Ein [`Begrenzer`] um die Datei liefert nach der
//! Grenze einen Fehler**, und die Kiste gibt damit auf. Er liegt **unter** dem
//! `BufReader` und nicht darueber, damit das Nachlesen nach einem Sprung
//! mitzaehlt: gemessen wird, was von der Platte kommt.
//!
//! # Welche Endungen geoeffnet werden
//!
//! Allein die, deren Format ein Exif tragen kann ([`EXIF_ENDUNGEN`]). `gif`,
//! `bmp` und `icns` tragen keines im Sinn des Spec und kosten keine Oeffnung;
//! die Bildfolge ordnet sie nach dem Aenderungsdatum, das der Verzeichnis-
//! leselauf ohnehin liefert.
//!
//! **TIFF steht darunter, und die Grenze entscheidet dort ueber das Ergebnis.**
//! Die Kiste liest eine TIFF-Datei ganz; eine kleine liegt unter der Grenze und
//! liefert ihr Datum, eine Foto-TIFF liegt darueber und ordnet nach dem
//! Aenderungsdatum. So hat es der Nutzer am 260929 entschieden (Haltepunkt 2
//! des Spec, Bericht der Klaerung, Befund „Stop fuer dieses Format“). Dasselbe
//! gilt fuer ein PNG, dessen `eXIf`-Block hinter den Bilddaten steht.
//!
//! # Was `None` heisst
//!
//! Kein Datum aus den Bilddaten: die Endung traegt keines, die Datei laesst
//! sich nicht oeffnen oder ist keine gewoehnliche Datei, die Grenze ist
//! erreicht, das Feld fehlt oder traegt keinen gueltigen Zeitpunkt. **Jeder
//! Fehler ist `None` und nie eine Panik** (C5.5); die Bildfolge faellt dann auf
//! das Aenderungsdatum zurueck, und kein Foto faellt aus der Folge.
//!
//! # Warum die Zeit keine Zone traegt
//!
//! `DateTimeOriginal` ist buergerliche Ortszeit ohne Zone. Das
//! Aenderungsdatum bringt [`Aufnahmezeit::aus_zeitpunkt`] ueber
//! [`crate::verzeichnis::sys::ortszeit`] in dieselbe Form, und erst damit sind
//! die zwei vergleichbar. `OffsetTimeOriginal` und Sekundenbruchteile gehen
//! nicht ein (Entscheidung 6 des Plans).

use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use std::time::SystemTime;

use crate::leseprofil::HOECHSTENS_BYTES_JE_FOTO;
use crate::verzeichnis::sys::{ohne_warten_oeffnen, ortszeit};

/// Die Endungen, deren Format ein Aufnahmedatum tragen kann, klein
/// geschrieben. Jede steht auch in [`super::ENDUNGEN`].
pub const EXIF_ENDUNGEN: [&str; 7] = ["jpg", "jpeg", "tif", "tiff", "png", "heic", "heif"];

/// Ein Zeitpunkt in buergerlicher Ortszeit, bis zur Sekunde.
///
/// Die Ordnung ist die der Zeit: die Felder stehen vom Jahr bis zur Sekunde,
/// und die abgeleitete Ordnung vergleicht sie in dieser Reihenfolge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Aufnahmezeit {
    jahr: u16,
    monat: u8,
    tag: u8,
    stunde: u8,
    minute: u8,
    sekunde: u8,
}

impl Aufnahmezeit {
    /// Ein Zeitpunkt aus seinen sechs Feldern, oder `None`, wenn eines davon
    /// keinen Kalenderwert traegt.
    ///
    /// Das Jahr 0 ist kein Wert, sondern die Fuellung, die manche Kameras in
    /// ein leeres Feld schreiben (`0000:00:00 00:00:00`). Die Sekunde darf 60
    /// sein, die Schaltsekunde.
    #[must_use]
    pub fn neu(jahr: u16, monat: u8, tag: u8, stunde: u8, minute: u8, sekunde: u8) -> Option<Self> {
        let gueltig = jahr >= 1
            && (1..=12).contains(&monat)
            && (1..=31).contains(&tag)
            && stunde <= 23
            && minute <= 59
            && sekunde <= 60;
        gueltig.then_some(Self {
            jahr,
            monat,
            tag,
            stunde,
            minute,
            sekunde,
        })
    }

    /// Ein Zeitpunkt aus dem Dateisystem, in Ortszeit gerechnet.
    ///
    /// Der Zonenversatz ist der, der zum Zeitpunkt galt; das haelt
    /// [`crate::verzeichnis::sys::ortszeit`].
    #[must_use]
    pub fn aus_zeitpunkt(zeitpunkt: SystemTime) -> Option<Self> {
        let zeit = ortszeit(zeitpunkt)?;
        Self::neu(
            u16::try_from(zeit.jahr).ok()?,
            zeit.monat,
            zeit.tag,
            zeit.stunde,
            zeit.minute,
            zeit.sekunde,
        )
    }

    /// Das Jahr mit seinem Jahrhundert.
    pub fn jahr(self) -> u16 {
        self.jahr
    }

    /// Der Monat von 1 bis 12.
    pub fn monat(self) -> u8 {
        self.monat
    }

    /// Der Tag im Monat.
    pub fn tag(self) -> u8 {
        self.tag
    }

    /// Die Stunde von 0 bis 23.
    pub fn stunde(self) -> u8 {
        self.stunde
    }

    /// Die Minute.
    pub fn minute(self) -> u8 {
        self.minute
    }

    /// Die Sekunde.
    pub fn sekunde(self) -> u8 {
        self.sekunde
    }
}

/// Wie die Bildfolge an ein Aufnahmedatum kommt.
///
/// **Eingespritzt und nicht fest gerufen**, damit Ordnen, Grenzen und Abbruch
/// der Bildfolge ohne echte Bilddateien pruefbar sind: eine Probe reicht einen
/// Leser herein, der zaehlt oder fuer bestimmte Namen nichts liefert. Im
/// Betrieb ist es [`aufnahmedatum`]. `Sync`, weil der Leser auf dem Faden der
/// Vorschau laeuft. Die Lebensdauer laesst einen Leser zu, der Werte des
/// Rufers leiht, etwa einen Zaehler einer Probe.
pub type Datumsleser<'a> = dyn Fn(&Path) -> Option<Aufnahmezeit> + Sync + 'a;

/// Das Aufnahmedatum eines Fotos aus seinen Bilddaten, oder `None`.
///
/// Geoeffnet wird allein eine Datei mit einer der [`EXIF_ENDUNGEN`], ueber
/// [`ohne_warten_oeffnen`], und der Typ wird am Deskriptor gefragt: eine
/// benannte Roehre oder ein Ordner mit passendem Namen liefert `None`, ohne
/// zu warten. Was `None` sonst heisst, steht im Modulkopf.
#[must_use]
pub fn aufnahmedatum(pfad: &Path) -> Option<Aufnahmezeit> {
    if !traegt_exif_endung(pfad) {
        return None;
    }
    let datei = ohne_warten_oeffnen(pfad).ok()?;
    if !datei.metadata().ok()?.is_file() {
        return None;
    }
    aus_quelle(datei)
}

/// Das Aufnahmedatum aus einer beliebigen Quelle, hinter dem Begrenzer.
///
/// Der Teil von [`aufnahmedatum`] ohne Endung und ohne Datei. Er steht fuer
/// sich, damit eine Probe mit einem zaehlenden Leser nachmessen kann, dass
/// nie mehr als [`HOECHSTENS_BYTES_JE_FOTO`] gelesen werden.
#[must_use]
pub fn aus_quelle<R: Read + Seek>(quelle: R) -> Option<Aufnahmezeit> {
    let mut gepuffert = BufReader::new(Begrenzer::neu(quelle, HOECHSTENS_BYTES_JE_FOTO));
    let exif = exif::Reader::new()
        .read_from_container(&mut gepuffert)
        .ok()?;
    let feld = exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)?;
    let exif::Value::Ascii(ref teile) = feld.value else {
        return None;
    };
    let zeit = exif::DateTime::from_ascii(teile.first()?).ok()?;
    Aufnahmezeit::neu(
        zeit.year,
        zeit.month,
        zeit.day,
        zeit.hour,
        zeit.minute,
        zeit.second,
    )
}

/// Ob die Endung des Pfades ein Aufnahmedatum tragen kann.
fn traegt_exif_endung(pfad: &Path) -> bool {
    pfad.extension().is_some_and(|endung| {
        let klein = endung.to_string_lossy().to_ascii_lowercase();
        EXIF_ENDUNGEN.contains(&klein.as_str())
    })
}

/// Ein Leser, der nach `grenze` Bytes einen Fehler liefert.
///
/// Gezaehlt werden die gelesenen Bytes und nicht die Stelle: ein Sprung
/// kostet nichts, das Lesen danach zaehlt weiter. So misst er, was von der
/// Platte kommt.
struct Begrenzer<R> {
    innen: R,
    gelesen: u64,
    grenze: u64,
}

impl<R> Begrenzer<R> {
    fn neu(innen: R, grenze: u64) -> Self {
        Self {
            innen,
            gelesen: 0,
            grenze,
        }
    }
}

impl<R: Read> Read for Begrenzer<R> {
    fn read(&mut self, puffer: &mut [u8]) -> io::Result<usize> {
        let offen = self.grenze.saturating_sub(self.gelesen);
        if offen == 0 {
            return Err(io::Error::other("Lesegrenze erreicht"));
        }
        let hoechstens = usize::try_from(offen)
            .unwrap_or(usize::MAX)
            .min(puffer.len());
        let geliefert = self.innen.read(&mut puffer[..hoechstens])?;
        self.gelesen += geliefert as u64;
        Ok(geliefert)
    }
}

impl<R: Seek> Seek for Begrenzer<R> {
    fn seek(&mut self, stelle: SeekFrom) -> io::Result<u64> {
        self.innen.seek(stelle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jede_exif_endung_ist_eine_bildendung() {
        for endung in EXIF_ENDUNGEN {
            assert!(
                super::super::ENDUNGEN.contains(&endung),
                "{endung} ist keine Bildendung"
            );
        }
    }

    #[test]
    fn ein_zeitpunkt_ohne_kalenderwert_ist_keiner() {
        assert!(Aufnahmezeit::neu(0, 0, 0, 0, 0, 0).is_none());
        assert!(Aufnahmezeit::neu(2008, 13, 1, 0, 0, 0).is_none());
        assert!(Aufnahmezeit::neu(2008, 2, 30, 24, 0, 0).is_none());
        assert!(Aufnahmezeit::neu(2008, 3, 14, 15, 9, 26).is_some());
    }

    #[test]
    fn die_ordnung_ist_die_der_zeit() {
        let frueh = Aufnahmezeit::neu(2008, 1, 31, 23, 59, 59).expect("gueltig");
        let spaet = Aufnahmezeit::neu(2008, 2, 1, 0, 0, 0).expect("gueltig");
        assert!(frueh < spaet);
    }

    #[test]
    fn der_begrenzer_liefert_nach_der_grenze_einen_fehler() {
        let mut begrenzt = Begrenzer::neu(io::Cursor::new(vec![7u8; 100]), 10);
        let mut puffer = [0u8; 64];
        assert_eq!(begrenzt.read(&mut puffer).expect("unter der Grenze"), 10);
        assert!(begrenzt.read(&mut puffer).is_err());
    }
}
