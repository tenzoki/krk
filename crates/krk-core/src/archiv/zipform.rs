//! Das Zip-Format an einer Stelle: die Zeitfelder und Rechte, die der Packlauf
//! schreibt und der Entpacklauf liest, und der Leser des Inhaltsverzeichnisses.
//!
//! **Die Helfer stehen hier und nicht mehr in `operation::zippen` und
//! `operation::entpacken`**, seit der Leser des Inhaltsverzeichnisses dieselben
//! zwei Zeitfelder liest wie das Entpacken und der Umbau eines Archivs
//! dieselben schreiben wird wie das Packen (Schritt 1 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`). Verschoben
//! sind sie ohne Aenderung am Verhalten; warum es zwei Zeitfelder sind und
//! warum das MS-DOS-Feld nicht gelesen wird, steht weiter in den Koepfen jener
//! zwei Module.
//!
//! # Das Inhaltsverzeichnis eines Zip
//!
//! `ZipArchive::new` liest das Zentralverzeichnis. Je Eintrag fragt
//! [`verzeichnis_lesen`] Name, Groesse, Rechte, Verschluesselung, Verfahren und
//! das Datum aus den Zeitfeldern; entpackt wird nichts. Der Zugang je Eintrag
//! geht ueber `by_index_raw`, und das kostet einen Sprung zum lokalen Kopf je
//! Eintrag, denn erst dort steht, wo die Daten beginnen. Gemessen ist das an
//! 100 000 Eintraegen in der Probe `hunderttausend_zip_eintraege_messen`
//! (`tests/archiv.rs`, ignoriert, mit `--ignored` zu fahren).
//!
//! **Doppelte Namen sieht die Kiste nicht mehr**: sie fuehrt ihre Eintraege in
//! einer Tabelle nach Namen, und ein spaeterer Eintrag gleichen Namens ersetzt
//! den frueheren dort. Dass der spaetere gilt, ist die Regel des Sammlers
//! ohnehin; damit die frueheren trotzdem in der Zahl der ausgelassenen Namen
//! stehen, zaehlt [`zentrale_koepfe_zaehlen`] die Koepfe des
//! Zentralverzeichnisses selbst nach.
//!
//! # Die rohe Form fuer den Umbau
//!
//! Der Umbau eines Zip (`super::umbau`) kopiert jeden unveraenderten Eintrag
//! **Byte fuer Byte**: lokaler Kopf, gepackte Daten und ein etwaiger
//! Datenbeschreiber gehen unveraendert hinueber, der Satz im
//! Zentralverzeichnis ebenso, bis auf den Versatz seines lokalen Kopfs. Die
//! Kiste bietet dafuer `raw_copy_file`, und das reicht nicht: es baut den
//! Eintrag aus `ZipFile::options` neu, und die tragen weder die Zusatzfelder
//! (also nicht die Zeitfelder `0x5455` und `0x5855`, ohne die KRK jeden
//! kopierten Eintrag auf 1970 zeigte) noch das Typkennzeichen einer
//! Verknuepfung, die danach eine gewoehnliche Datei waere (gelesen in der
//! Quelle von `zip-8.6.0`, `ZipFile::options` und `FileOptions::normalize`).
//! Die Saetze liest und schreibt deshalb [`Zentralsatz`] selbst, das Ende des
//! Archivs samt ZIP64 [`ende_lesen`] und [`ende_schreiben`]; neue Eintraege
//! packt weiter die Kiste.
//!
//! Ein Zip mit Daten vor seinem ersten Eintrag oder ueber mehrere Teile
//! verteilt meldet sich hier mit `io::ErrorKind::InvalidData`, und der Umbau
//! schreibt es nicht: die Versaetze waeren dort nicht die, die er setzt.

use std::fs::{self, File};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use zip::extra_fields::ExtraField;
use zip::read::ZipFile;
use zip::result::ZipError;
use zip::write::FullFileOptions;
use zip::{CompressionMethod, DateTime, ZipArchive};

use crate::verzeichnis::Typ;

use super::HOECHSTENS_EINTRAEGE;
use super::innenpfad::Innenpfad;
use super::verzeichnis::{
    Archiveintrag, Fundstelle, Lesbarkeit, Sammler, Verzeichnisfehler, abbruch_fragen,
};

/// Liest das Aenderungsdatum eines Archiveintrags, soweit es in Epochensekunden
/// dasteht.
///
/// Zwei Quellen, in dieser Reihenfolge: das erweiterte Zeitfeld `0x5455`, das
/// die Kiste selbst zerlegt, und das alte Info-ZIP-Unix-Feld `0x5855`, das sie
/// nicht zerlegt und das [`infozip_unix_zeit`] deshalb aus den Rohbytes holt.
/// `None` heisst: der Eintrag fuehrt keines von beiden, und dann bleibt die
/// entpackte Datei bei der Uhrzeit des Entpackens. Warum das MS-DOS-Pflichtfeld
/// hier nicht einspringt, steht im Kopf von `operation::entpacken`.
pub(crate) fn eintragszeit<R: Read>(eintrag: &ZipFile<'_, R>) -> Option<SystemTime> {
    let erweitert = eintrag.extra_data_fields().find_map(|feld| match feld {
        ExtraField::ExtendedTimestamp(zeit) => zeit.mod_time(),
        _ => None,
    });
    let sekunden = erweitert.or_else(|| eintrag.extra_data().and_then(infozip_unix_zeit))?;
    Some(UNIX_EPOCH + Duration::from_secs(u64::from(sekunden)))
}

/// Holt die Aenderungszeit aus dem alten Info-ZIP-Unix-Feld `0x5855`.
///
/// Der uebergebene Block ist die Folge der Zusatzfelder, jedes mit zwei Byte
/// Kennung, zwei Byte Laenge und seinem Rumpf. Gesucht wird
/// [`FELD_INFOZIP_UNIX`]; sein Rumpf traegt zuerst die
/// Zugriffszeit und dann die Aenderungszeit, je vier Byte Epochensekunden.
///
/// **Die Kennung steht neben ihrem Schreiber in dieser Datei**, damit sie im
/// Vorhaben genau einmal dasteht: dieselbe Zahl schreibt der Packlauf.
fn infozip_unix_zeit(zusatzfelder: &[u8]) -> Option<u32> {
    let mut rest = zusatzfelder;
    while rest.len() >= 4 {
        let kennung = u16::from_le_bytes([rest[0], rest[1]]);
        let laenge = usize::from(u16::from_le_bytes([rest[2], rest[3]]));
        let rumpf = rest.get(4..4 + laenge)?;
        if kennung == FELD_INFOZIP_UNIX
            && let Some(zeit) = rumpf.get(4..8)
        {
            return Some(u32::from_le_bytes([zeit[0], zeit[1], zeit[2], zeit[3]]));
        }
        rest = &rest[4 + laenge..];
    }
    None
}

/// Ab welcher Groesse ein neuer Zip-Eintrag die ZIP64-Form bekommt.
///
/// Die Kiste `zip` bricht einen Eintrag ohne `large_file(true)` ab, sobald
/// seine Daten `u32::MAX` Bytes ueberschreiten, ungepackt beim Schreiben und
/// gepackt beim Abschluss; `set_auto_large_file` aendert daran nichts, es
/// wirkt allein auf den Datenbeschreiber. Gepackte Daten koennen etwas
/// groesser sein als ungepackte (ein Block, der sich nicht packen laesst,
/// kostet fuenf Bytes je 64 KiB), deshalb liegt die Schwelle um ein
/// Zweihundertsechsundfuenfzigstel unter `u32::MAX` und nicht an ihm.
const ZIP64_AB: u64 = u32::MAX as u64 - (u32::MAX as u64 >> 8);

/// Ob ein neuer Eintrag mit `groesse` Bytes die ZIP64-Form braucht; eine
/// unbekannte Groesse braucht sie.
#[must_use]
pub(crate) fn braucht_zip64(groesse: Option<u64>) -> bool {
    groesse.is_none_or(|groesse| groesse >= ZIP64_AB)
}

/// Setzt die ZIP64-Form eines neuen Datei-Eintrags nach [`braucht_zip64`].
///
/// Die eine Stelle dieser Regel fuer das Packen (`operation::zippen`) und den
/// Umbau eines Archivs (`archiv::umbau`). Kleine Eintraege bekommen kein
/// ZIP64-Feld, damit ein Archiv aus kleinen Dateien so aussieht wie vorher.
#[must_use = "ohne die Wahl scheitert ein Eintrag ueber 4 GiB"]
pub(crate) fn grosse_form_waehlen(
    wahl: FullFileOptions<'static>,
    groesse: Option<u64>,
) -> FullFileOptions<'static> {
    wahl.large_file(braucht_zip64(groesse))
}

/// Uebernimmt die Rechte der Quelle in die Wahl, notfalls die genannte Vorgabe.
///
/// Ein Archiv, das die Rechte fortwirft, macht aus einem ausfuehrbaren Skript
/// beim Entpacken eine gewoehnliche Datei. Laesst sich die Quelle gerade nicht
/// befragen, steht die uebliche Vorgabe da; ein Eintrag ohne Rechte waere
/// schlechter als ein Eintrag mit den ueblichen.
#[must_use = "ohne die Wahl traegt der Eintrag keine Rechte"]
pub(crate) fn rechte_uebernehmen(
    wahl: FullFileOptions<'static>,
    angaben: Option<&fs::Metadata>,
    vorgabe: u32,
) -> FullFileOptions<'static> {
    use std::os::unix::fs::PermissionsExt;

    let rechte = angaben.map_or(vorgabe, |angaben| angaben.permissions().mode());
    wahl.unix_permissions(rechte)
}

/// Kennung des erweiterten Zeitfeldes (`Extended Timestamp`), das `unzip` liest.
///
/// Rumpf: ein Kennzeichenbyte, dann die genannten Zeiten als Epochensekunden in
/// der Reihenfolge Aenderung, Zugriff, Erzeugung. KRK setzt die ersten zwei
/// Kennzeichenbits, schreibt also acht Byte Zeit hinter das Kennzeichen.
pub(crate) const FELD_ERWEITERTE_ZEIT: u16 = 0x5455;

/// Kennung des alten Info-ZIP-Unix-Feldes, das `ditto(1)` liest.
///
/// Rumpf in der kurzen Form: Zugriffszeit, dann Aenderungszeit, je vier Byte
/// Epochensekunden. Die lange Form haengt Benutzer- und Gruppenkennung an; KRK
/// schreibt sie nicht, denn `ditto` setzt sie beim Auspacken um, und dieses
/// Vorhaben packt Zeitstempel und keine Eigentumsverhaeltnisse.
///
/// **Diese Kennung ist der einzige Grund fuer das Merkmal `unreserved`** in der
/// Wurzel-`Cargo.toml`; ohne es weist `add_extra_data` sie ab.
pub(crate) const FELD_INFOZIP_UNIX: u16 = 0x5855;

/// Traegt das Aenderungsdatum der Quelle in die Wahl ein, dreifach.
///
/// Einmal als MS-DOS-Feld, das jeder Leser versteht und das keine Zone kennt,
/// und zweimal als Zusatzfeld in Epochensekunden. Warum es zwei Zusatzfelder
/// sein muessen, steht als Messtabelle in der Wurzel-`Cargo.toml`.
///
/// **Ein Zeitpunkt, den keines der Felder fasst, weist den Eintrag nicht ab,
/// und er bleibt stumm.** Der Eintrag traegt dann das Vorgabedatum des
/// Formats; ein Archiv ohne diese Datei waere die schlechtere Antwort als eine
/// Datei mit einem Datum, das der Nutzer als falsch erkennt. Dasselbe gilt fuer
/// ein Aenderungsdatum, das nicht zu lesen war, und fuer ein Zusatzfeld, das
/// nicht in den Eintrag kam.
///
/// Stumm, weil die Abschlussliste die Liste der **nicht bearbeiteten**
/// Eintraege ist ([`crate::operation::fortschritt::Uebersprungen`]) und das Blatt sie dem
/// Nutzer so ausschreibt: die Datei steht vollstaendig im Archiv, und sie dort
/// als uebersprungen zu nennen, waere die falsche Auskunft. Das ist die Wahl,
/// die `operation::entpacken` fuer den Fehlschlag beim Rechte- und Datumsetzen
/// trifft, und bis zum 260825 traf das Packen fuer dieselbe Lage die
/// entgegengesetzte — mit bis zu drei Zeilen je Eintrag
/// (`shared/issues/260825-2127_*_ein-gepackter-eintrag-mit-ersatzdatum-steht-
/// in-der-liste-der-uebersprungenen.md`). Deshalb nimmt diese Funktion keine
/// [`crate::operation::fortschritt::Steuerung`] entgegen: sie hat nichts zu melden.
#[must_use = "ohne die Wahl traegt der Eintrag kein Datum"]
pub(crate) fn zeit_uebernehmen(
    wahl: FullFileOptions<'static>,
    angaben: Option<&fs::Metadata>,
) -> FullFileOptions<'static> {
    let Some(geaendert) = angaben.and_then(|angaben| angaben.modified().ok()) else {
        return wahl;
    };
    let gelesen = angaben.and_then(|angaben| angaben.accessed().ok());
    zeit_eintragen(wahl, geaendert, gelesen)
}

/// Traegt ein Aenderungsdatum und, wenn bekannt, eine Zugriffszeit in die
/// Wahl ein, wie [`zeit_uebernehmen`] es aus den Angaben einer Datei tut.
///
/// Der Umbau eines Archivs kennt fuer einen neuen Eintrag ein Datum und keine
/// `Metadata`: seine Quelle kann auf einem Server oder in einem anderen Archiv
/// liegen. Die Regel ist dieselbe und steht deshalb hier und nicht zweimal.
#[must_use = "ohne die Wahl traegt der Eintrag kein Datum"]
pub(crate) fn zeit_eintragen(
    mut wahl: FullFileOptions<'static>,
    geaendert: SystemTime,
    gelesen: Option<SystemTime>,
) -> FullFileOptions<'static> {
    // Die Zugriffszeit ist die Zugabe und nicht der Gegenstand: gibt sie nichts
    // her, steht das Aenderungsdatum an beiden Stellen, so wie `ditto(1)` es
    // auch haelt. Sie gibt in zwei Lagen nichts her, und beide sind hier
    // gleich behandelt: sie fehlt, oder sie liegt ausserhalb der vier Byte, die
    // die zwei Zusatzfelder dafuer vorsehen. Haenge die Felder an die Zugabe,
    // verhinderte eine Zugriffszeit von 1969 das Zusatzfeld eines tadellosen
    // Aenderungsdatums, und zwar ohne eine Zeile in der Abschlussliste.
    if let Some(zeitpunkt) = archivzeitpunkt(geaendert) {
        wahl = wahl.last_modified_time(zeitpunkt);
    }

    if let Some(geaendert) = epochensekunden(geaendert) {
        let gelesen = gelesen.and_then(epochensekunden).unwrap_or(geaendert);
        let mut erweitert = Vec::with_capacity(9);
        // Bit 0: die Aenderungszeit steht da. Bit 1: die Zugriffszeit steht da.
        erweitert.push(0b0000_0011);
        erweitert.extend_from_slice(&geaendert.to_le_bytes());
        erweitert.extend_from_slice(&gelesen.to_le_bytes());

        let mut infozip = Vec::with_capacity(8);
        infozip.extend_from_slice(&gelesen.to_le_bytes());
        infozip.extend_from_slice(&geaendert.to_le_bytes());

        // Beide Felder gehen in den lokalen Kopf; die Kiste wiederholt ihn im
        // Hauptverzeichnis, ein zweiter Eintrag als "nur zentral" stuende dort
        // also doppelt.
        for (kennung, rumpf) in [
            (FELD_ERWEITERTE_ZEIT, erweitert),
            (FELD_INFOZIP_UNIX, infozip),
        ] {
            // Ein Feld, das nicht hineinkam, laesst das MS-DOS-Feld stehen;
            // der Eintrag traegt dann ein Datum ohne Zone statt keines.
            let _ = wahl.add_extra_data(kennung, rumpf, false);
        }
    }

    wahl
}

/// Rechnet ein Aenderungsdatum in die MS-DOS-Zeitform des Zip-Formats um.
///
/// `None` heisst: der Zeitpunkt liegt ausserhalb von 1980 bis 2107, oder das
/// System konnte ihn nicht in einen Kalendertag uebersetzen. Die Sekunde faellt
/// dabei auf ein gerades Raster, denn das Format traegt fuer sie nur fuenf Bit.
fn archivzeitpunkt(zeitpunkt: SystemTime) -> Option<DateTime> {
    let ortszeit = crate::verzeichnis::sys::ortszeit(zeitpunkt)?;
    DateTime::from_date_and_time(
        u16::try_from(ortszeit.jahr).ok()?,
        ortszeit.monat,
        ortszeit.tag,
        ortszeit.stunde,
        ortszeit.minute,
        ortszeit.sekunde,
    )
    .ok()
}

/// Macht aus einem Zeitpunkt die Zahl der Sekunden seit 1970, wie die zwei
/// Zusatzfelder sie tragen.
///
/// `None` heisst: der Zeitpunkt liegt vor 1970 oder nach 2106 und passt damit
/// nicht in die vier Byte, die beide Felder dafuer vorsehen.
fn epochensekunden(zeitpunkt: SystemTime) -> Option<u32> {
    let seither = zeitpunkt.duration_since(UNIX_EPOCH).ok()?;
    u32::try_from(seither.as_secs()).ok()
}

/// Liest das Inhaltsverzeichnis eines Zip in den Sammler.
pub(crate) fn verzeichnis_lesen(
    datei: File,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    let zweitleser = datei.try_clone().map_err(Verzeichnisfehler::Lesen)?;
    let mut archiv = ZipArchive::new(BufReader::new(datei)).map_err(zipfehler)?;
    if archiv.len() > HOECHSTENS_EINTRAEGE {
        return Err(Verzeichnisfehler::ZuGross);
    }
    let koepfe = zentrale_koepfe_zaehlen(zweitleser, archiv.central_directory_start());
    for _ in archiv.len()..koepfe {
        sammler.auslassen();
    }

    for stelle in 0..archiv.len() {
        abbruch_fragen(abbruch)?;
        let eintrag = archiv.by_index_raw(stelle).map_err(zipfehler)?;
        let Some(innen) = eintragsname(&eintrag).and_then(Innenpfad::aus_archivname) else {
            sammler.auslassen();
            continue;
        };
        let modus = eintrag.unix_mode();
        let typ = if eintrag.is_dir() {
            Typ::Ordner
        } else if eintrag.is_symlink() {
            Typ::Verknuepfung
        } else {
            Typ::Datei
        };
        let lesbarkeit = if matches!(typ, Typ::Ordner) {
            Lesbarkeit::Lesbar
        } else if eintrag.encrypted() {
            Lesbarkeit::Verschluesselt
        } else if !matches!(
            eintrag.compression(),
            CompressionMethod::Stored | CompressionMethod::Deflated
        ) {
            Lesbarkeit::Verfahren(verfahrensname(eintrag.compression()))
        } else if modus.is_some_and(ist_sonderdatei) {
            Lesbarkeit::Sonderdatei
        } else {
            Lesbarkeit::Lesbar
        };
        let groesse = match typ {
            Typ::Ordner => 0,
            Typ::Datei | Typ::Verknuepfung => eintrag.size(),
        };
        let geaendert = eintragszeit(&eintrag).unwrap_or(UNIX_EPOCH);
        drop(eintrag);
        sammler.aufnehmen(Archiveintrag {
            innen,
            typ,
            groesse,
            geaendert,
            rechte: modus.map(|modus| modus & 0o7777),
            fundstelle: Fundstelle::Zip(stelle),
            lesbarkeit,
        })?;
    }
    Ok(())
}

/// Der Name eines Eintrags als Text, oder `None`, wenn er sich nicht treu als
/// Text fassen laesst.
///
/// Sind die rohen Bytes UTF-8, gelten sie. Sonst gilt der Name, den die Kiste
/// daraus entschluesselt hat, nach Codepage 437, wie ihn jeder Zip-Leser ohne
/// das UTF-8-Kennzeichen liest; traegt dieser ein Ersatzzeichen, hat die Kiste
/// ungueltiges UTF-8 ersetzt, und der Name zeigte auf keinen Eintrag mehr.
fn eintragsname<'a, R: Read>(eintrag: &'a ZipFile<'_, R>) -> Option<&'a str> {
    match std::str::from_utf8(eintrag.name_raw()) {
        Ok(name) => Some(name),
        Err(_) if eintrag.name().contains('\u{FFFD}') => None,
        Err(_) => Some(eintrag.name()),
    }
}

/// Der Name eines Verfahrens, das KRK nicht entpackt, wie der Bericht ihn
/// nennt.
///
/// Ohne ihre Merkmale fuer diese Verfahren fuehrt die Kiste jedes davon als
/// `Unsupported(<nummer>)`; die Namen hier sind die der Formatbeschreibung,
/// verglichen ueber die Konstanten der Kiste, die in beiden Faellen dieselben
/// Werte tragen. Ein Verfahren, das hier fehlt, heisst mit seiner Nummer.
fn verfahrensname(methode: CompressionMethod) -> String {
    let bekannt = [
        (CompressionMethod::SHRINK, "shrink"),
        (CompressionMethod::REDUCE_1, "reduce"),
        (CompressionMethod::REDUCE_2, "reduce"),
        (CompressionMethod::REDUCE_3, "reduce"),
        (CompressionMethod::REDUCE_4, "reduce"),
        (CompressionMethod::IMPLODE, "implode"),
        (CompressionMethod::DEFLATE64, "deflate64"),
        (CompressionMethod::BZIP2, "bzip2"),
        (CompressionMethod::LZMA, "lzma"),
        (CompressionMethod::ZSTD, "zstd"),
        (CompressionMethod::XZ, "xz"),
        (CompressionMethod::PPMD, "ppmd"),
        (CompressionMethod::AES, "aes"),
    ];
    bekannt
        .iter()
        .find(|(bekannte, _)| *bekannte == methode)
        .map_or_else(|| format!("{methode:?}"), |(_, name)| (*name).to_owned())
}

/// Ob die Typbits eines Unix-Modus eine Geraetedatei, eine Roehre oder ein
/// Sockel sind.
fn ist_sonderdatei(modus: u32) -> bool {
    const TYPBITS: u32 = 0o170_000;
    matches!(
        modus & TYPBITS,
        0o010_000 | 0o020_000 | 0o060_000 | 0o140_000
    )
}

/// Ordnet einen Fehler der Kiste zu, wie [`Verzeichnisfehler::aus_strom`]
/// einen Fehler beim Lesen eines Stroms.
pub(crate) fn zipfehler(fehler: ZipError) -> Verzeichnisfehler {
    match fehler {
        ZipError::Io(fehler) if fehler.raw_os_error().is_some() => Verzeichnisfehler::Lesen(fehler),
        andere => Verzeichnisfehler::Ungueltig(andere.to_string()),
    }
}

/// Zaehlt die Koepfe des Zentralverzeichnisses ab `anfang`, bis eine
/// Kennung nicht mehr die eines zentralen Kopfs ist.
///
/// Ein Kopf traegt 46 feste Bytes und danach Name, Zusatzfelder und Kommentar
/// mit den Laengen an den Stellen 28, 30 und 32. Ein Lesefehler beendet die
/// Zaehlung: die Kiste hat dasselbe Verzeichnis eben gelesen, und eine Zahl,
/// die zu klein ist, laesst allein frueher ausgelassene Namen ungezaehlt.
fn zentrale_koepfe_zaehlen(datei: File, anfang: u64) -> usize {
    const KENNUNG: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];
    let mut leser = BufReader::new(datei);
    if leser.seek(SeekFrom::Start(anfang)).is_err() {
        return 0;
    }
    let mut zahl = 0;
    let mut kopf = [0u8; 46];
    while leser.read_exact(&mut kopf).is_ok() && kopf[..4] == KENNUNG {
        zahl += 1;
        let laenge =
            |stelle: usize| i64::from(u16::from_le_bytes([kopf[stelle], kopf[stelle + 1]]));
        if leser
            .seek_relative(laenge(28) + laenge(30) + laenge(32))
            .is_err()
        {
            break;
        }
    }
    zahl
}

/// Kennung eines zentralen Kopfs.
const SIG_ZENTRAL: u32 = 0x0201_4b50;
/// Kennung eines lokalen Kopfs.
const SIG_LOKAL: u32 = 0x0403_4b50;
/// Kennung des Endes des Zentralverzeichnisses.
const SIG_ENDE: u32 = 0x0605_4b50;
/// Kennung des ZIP64-Endes.
const SIG_ENDE64: u32 = 0x0606_4b50;
/// Kennung des Zeigers auf das ZIP64-Ende.
const SIG_ZEIGER64: u32 = 0x0706_4b50;
/// Das Zusatzfeld mit den ZIP64-Groessen und dem ZIP64-Versatz.
const FELD_ZIP64: u16 = 0x0001;
/// Das Info-ZIP-Feld fuer einen Namen in Unicode. Es nennt den alten Namen und
/// faellt deshalb bei einem umbenannten Eintrag weg; ein Leser zoege es sonst
/// dem neuen Namen vor.
const FELD_UNICODE_NAME: u16 = 0x7075;
/// Das Kennzeichen „Name in UTF-8“ im Allzweckfeld.
const KENNZEICHEN_UTF8: u16 = 0x0800;
/// Die Version, ab der ein Leser ZIP64 kennen muss.
const VERSION_ZIP64: u16 = 45;
/// Der Platzhalter eines 32-Bit-Felds, dessen Wert im ZIP64-Feld steht.
const VOLL32: u32 = u32::MAX;
/// Der Platzhalter eines 16-Bit-Felds, dessen Wert im ZIP64-Ende steht.
const VOLL16: u16 = u16::MAX;

/// Ein Fehler der rohen Form: das Zip hat eine Gestalt, die der Umbau nicht
/// schreibt.
fn formfehler(was: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, was.to_owned())
}

fn u16_an(bytes: &[u8], stelle: usize) -> u16 {
    u16::from_le_bytes([bytes[stelle], bytes[stelle + 1]])
}

fn u32_an(bytes: &[u8], stelle: usize) -> u32 {
    u32::from_le_bytes([
        bytes[stelle],
        bytes[stelle + 1],
        bytes[stelle + 2],
        bytes[stelle + 3],
    ])
}

fn u64_an(bytes: &[u8], stelle: usize) -> u64 {
    let mut feld = [0u8; 8];
    feld.copy_from_slice(&bytes[stelle..stelle + 8]);
    u64::from_le_bytes(feld)
}

fn u16_setzen(bytes: &mut [u8], stelle: usize, wert: u16) {
    bytes[stelle..stelle + 2].copy_from_slice(&wert.to_le_bytes());
}

fn u32_setzen(bytes: &mut [u8], stelle: usize, wert: u32) {
    bytes[stelle..stelle + 4].copy_from_slice(&wert.to_le_bytes());
}

/// Das Ende eines Zip: wo das Zentralverzeichnis beginnt, wie viele Saetze es
/// fuehrt, und der Kommentar des Archivs als rohe Bytes.
#[derive(Debug)]
pub(crate) struct Zipende {
    /// Der Versatz des ersten zentralen Kopfs.
    pub(crate) anfang: u64,
    /// Die Zahl der zentralen Koepfe.
    pub(crate) anzahl: u64,
    /// Der Kommentar des Archivs, unveraendert.
    pub(crate) kommentar: Vec<u8>,
}

/// Liest das Ende eines Zip, ZIP64 eingeschlossen.
///
/// Gesucht wird die Kennung des Endes von hinten, im Bereich, den ein
/// Kommentar hoechstens einnimmt. Ein Zip ueber mehrere Teile und eines, an
/// dessen genanntem Anfang kein zentraler Kopf steht (Daten vor dem ersten
/// Eintrag), melden `InvalidData`.
pub(crate) fn ende_lesen<R: Read + Seek>(leser: &mut R) -> io::Result<Zipende> {
    let laenge = leser.seek(SeekFrom::End(0))?;
    let fenster = laenge.min(22 + u64::from(u16::MAX));
    leser.seek(SeekFrom::Start(laenge - fenster))?;
    let mut schwanz = vec![0u8; usize::try_from(fenster).map_err(io::Error::other)?];
    leser.read_exact(&mut schwanz)?;
    if schwanz.len() < 22 {
        return Err(formfehler("zip: no end of central directory"));
    }

    let mut gefunden = None;
    let mut stelle = schwanz.len().saturating_sub(22);
    loop {
        if u32_an(&schwanz, stelle) == SIG_ENDE {
            let kommentarlaenge = usize::from(u16_an(&schwanz, stelle + 20));
            if stelle + 22 + kommentarlaenge <= schwanz.len() {
                gefunden = Some(stelle);
                break;
            }
        }
        if stelle == 0 {
            break;
        }
        stelle -= 1;
    }
    let stelle = gefunden.ok_or_else(|| formfehler("zip: no end of central directory"))?;
    let ende = &schwanz[stelle..];
    let ende_versatz = laenge - fenster + stelle as u64;
    let teil = u16_an(ende, 4);
    let teil_des_verzeichnisses = u16_an(ende, 6);
    let mut anzahl = u64::from(u16_an(ende, 10));
    let mut anfang = u64::from(u32_an(ende, 16));
    let kommentar = ende[22..22 + usize::from(u16_an(ende, 20))].to_vec();

    let braucht_zip64 = anzahl == u64::from(VOLL16) || anfang == u64::from(VOLL32);
    if braucht_zip64 && ende_versatz >= 20 {
        let mut zeiger = [0u8; 20];
        leser.seek(SeekFrom::Start(ende_versatz - 20))?;
        leser.read_exact(&mut zeiger)?;
        if u32_an(&zeiger, 0) == SIG_ZEIGER64 {
            let mut ende64 = [0u8; 56];
            leser.seek(SeekFrom::Start(u64_an(&zeiger, 8)))?;
            leser.read_exact(&mut ende64)?;
            if u32_an(&ende64, 0) != SIG_ENDE64 {
                return Err(formfehler("zip: broken zip64 end record"));
            }
            if u32_an(&ende64, 16) != 0 || u32_an(&ende64, 20) != 0 {
                return Err(formfehler("zip: spans several disks"));
            }
            anzahl = u64_an(&ende64, 32);
            anfang = u64_an(&ende64, 48);
        }
    } else if (teil != 0 && teil != VOLL16)
        || (teil_des_verzeichnisses != 0 && teil_des_verzeichnisses != VOLL16)
    {
        return Err(formfehler("zip: spans several disks"));
    }

    if anzahl > 0 {
        let mut kennung = [0u8; 4];
        leser.seek(SeekFrom::Start(anfang))?;
        leser.read_exact(&mut kennung)?;
        if u32::from_le_bytes(kennung) != SIG_ZENTRAL {
            return Err(formfehler("zip: data before the first entry"));
        }
    }
    Ok(Zipende {
        anfang,
        anzahl,
        kommentar,
    })
}

/// Zerlegt eine Folge von Zusatzfeldern in Kennung und Rumpf.
fn felder(roh: &[u8]) -> io::Result<Vec<(u16, Vec<u8>)>> {
    let mut felder = Vec::new();
    let mut rest = roh;
    while !rest.is_empty() {
        if rest.len() < 4 {
            return Err(formfehler("zip: broken extra field"));
        }
        let kennung = u16_an(rest, 0);
        let laenge = usize::from(u16_an(rest, 2));
        let rumpf = rest
            .get(4..4 + laenge)
            .ok_or_else(|| formfehler("zip: broken extra field"))?;
        felder.push((kennung, rumpf.to_vec()));
        rest = &rest[4 + laenge..];
    }
    Ok(felder)
}

/// Setzt Zusatzfelder wieder zu einer Folge zusammen.
fn felder_zusammen(felder: &[(u16, Vec<u8>)]) -> io::Result<Vec<u8>> {
    let mut roh = Vec::new();
    for (kennung, rumpf) in felder {
        let laenge =
            u16::try_from(rumpf.len()).map_err(|_| formfehler("zip: extra field too long"))?;
        roh.extend_from_slice(&kennung.to_le_bytes());
        roh.extend_from_slice(&laenge.to_le_bytes());
        roh.extend_from_slice(rumpf);
    }
    Ok(roh)
}

/// Das Allzweckfeld mit dem Kennzeichen „Name in UTF-8“, gesetzt fuer einen
/// Namen ausserhalb von ASCII und sonst wie es war.
fn kennzeichen_fuer(kennzeichen: u16, name: &[u8]) -> u16 {
    if name.is_ascii() {
        kennzeichen
    } else {
        kennzeichen | KENNZEICHEN_UTF8
    }
}

/// Ein Satz des Zentralverzeichnisses, wie er dasteht.
#[derive(Debug, Clone)]
pub(crate) struct Zentralsatz {
    fest: [u8; 46],
    name: Vec<u8>,
    zusatz: Vec<u8>,
    kommentar: Vec<u8>,
    /// Wo der Satz im Archiv steht.
    stelle: u64,
    /// Der Versatz des lokalen Kopfs, aus dem ZIP64-Feld, wenn es ihn traegt.
    lokal: u64,
    /// Die entpackte Groesse, sofern sie im ZIP64-Feld steht.
    groesse64: Option<u64>,
    /// Die gepackte Groesse, sofern sie im ZIP64-Feld steht.
    gepackt64: Option<u64>,
}

impl Zentralsatz {
    /// Wo der Satz im Archiv steht, wie `ZipFile::central_header_start` es
    /// nennt.
    pub(crate) fn stelle(&self) -> u64 {
        self.stelle
    }

    /// Der Versatz des lokalen Kopfs.
    pub(crate) fn lokal(&self) -> u64 {
        self.lokal
    }

    /// Der Name, wie er im Satz steht.
    pub(crate) fn name(&self) -> &[u8] {
        &self.name
    }

    /// Ob der Eintrag verschluesselt ist.
    pub(crate) fn verschluesselt(&self) -> bool {
        u16_an(&self.fest, 8) & 1 != 0
    }

    /// Der Satz, wie er an der neuen Stelle `lokal` und gegebenenfalls unter
    /// dem neuen Namen `name` dasteht, als Bytes.
    ///
    /// Alles andere bleibt: Verfahren, Pruefsumme, Groessen, Zeiten, Rechte,
    /// Typkennzeichen und jedes Zusatzfeld ausser dem ZIP64-Feld, das neu
    /// entsteht, und bei einem neuen Namen dem Info-ZIP-Namensfeld, das den
    /// alten traegt.
    pub(crate) fn verlegt(&self, lokal: u64, name: Option<&[u8]>) -> io::Result<Vec<u8>> {
        let mut fest = self.fest;
        let name = match name {
            Some(neu) => {
                let kennzeichen = kennzeichen_fuer(u16_an(&fest, 8), neu);
                u16_setzen(&mut fest, 8, kennzeichen);
                neu
            }
            None => &self.name,
        };
        let mut felder = felder(&self.zusatz)?;
        felder.retain(|(kennung, _)| {
            *kennung != FELD_ZIP64
                && !(name != self.name.as_slice() && *kennung == FELD_UNICODE_NAME)
        });
        let mut zip64 = Vec::new();
        if let Some(groesse) = self.groesse64 {
            zip64.extend_from_slice(&groesse.to_le_bytes());
        }
        if let Some(gepackt) = self.gepackt64 {
            zip64.extend_from_slice(&gepackt.to_le_bytes());
        }
        match u32::try_from(lokal) {
            Ok(klein) if klein != VOLL32 => u32_setzen(&mut fest, 42, klein),
            _ => {
                u32_setzen(&mut fest, 42, VOLL32);
                zip64.extend_from_slice(&lokal.to_le_bytes());
            }
        }
        // Die Nummer des Teils: ein Archiv aus einem Teil fuehrt die 0.
        u16_setzen(&mut fest, 34, 0);
        if !zip64.is_empty() {
            felder.insert(0, (FELD_ZIP64, zip64));
            let noetig = u16_an(&fest, 6).max(VERSION_ZIP64);
            u16_setzen(&mut fest, 6, noetig);
        }
        let zusatz = felder_zusammen(&felder)?;
        let laenge = |teil: &[u8]| {
            u16::try_from(teil.len()).map_err(|_| formfehler("zip: name or field too long"))
        };
        u16_setzen(&mut fest, 28, laenge(name)?);
        u16_setzen(&mut fest, 30, laenge(&zusatz)?);
        let mut satz = Vec::with_capacity(46 + name.len() + zusatz.len() + self.kommentar.len());
        satz.extend_from_slice(&fest);
        satz.extend_from_slice(name);
        satz.extend_from_slice(&zusatz);
        satz.extend_from_slice(&self.kommentar);
        Ok(satz)
    }
}

/// Liest die zentralen Saetze, die `ende` nennt, in ihrer Folge.
pub(crate) fn zentralsaetze_lesen<R: Read + Seek>(
    leser: &mut R,
    ende: &Zipende,
) -> io::Result<Vec<Zentralsatz>> {
    leser.seek(SeekFrom::Start(ende.anfang))?;
    let mut leser = BufReader::new(leser);
    let mut stelle = ende.anfang;
    let mut saetze = Vec::new();
    for _ in 0..ende.anzahl {
        let mut fest = [0u8; 46];
        leser.read_exact(&mut fest)?;
        if u32_an(&fest, 0) != SIG_ZENTRAL {
            return Err(formfehler("zip: central directory ends early"));
        }
        let mut name = vec![0u8; usize::from(u16_an(&fest, 28))];
        let mut zusatz = vec![0u8; usize::from(u16_an(&fest, 30))];
        let mut kommentar = vec![0u8; usize::from(u16_an(&fest, 32))];
        leser.read_exact(&mut name)?;
        leser.read_exact(&mut zusatz)?;
        leser.read_exact(&mut kommentar)?;

        // Das ZIP64-Feld fuehrt in fester Folge genau die Werte, deren
        // 32-Bit-Feld den Platzhalter traegt.
        let mut groesse64 = None;
        let mut gepackt64 = None;
        let mut lokal = u64::from(u32_an(&fest, 42));
        let groesse_voll = u32_an(&fest, 24) == VOLL32;
        let gepackt_voll = u32_an(&fest, 20) == VOLL32;
        let lokal_voll = u32_an(&fest, 42) == VOLL32;
        if groesse_voll || gepackt_voll || lokal_voll {
            let feld = felder(&zusatz)?
                .into_iter()
                .find(|(kennung, _)| *kennung == FELD_ZIP64)
                .map(|(_, rumpf)| rumpf)
                .ok_or_else(|| formfehler("zip: zip64 field missing"))?;
            let mut lesestelle = 0;
            let mut naechster = || -> io::Result<u64> {
                let wert = feld
                    .get(lesestelle..lesestelle + 8)
                    .map(|teil| u64_an(teil, 0))
                    .ok_or_else(|| formfehler("zip: zip64 field too short"))?;
                lesestelle += 8;
                Ok(wert)
            };
            if groesse_voll {
                groesse64 = Some(naechster()?);
            }
            if gepackt_voll {
                gepackt64 = Some(naechster()?);
            }
            if lokal_voll {
                lokal = naechster()?;
            }
        }

        let laenge = 46 + name.len() + zusatz.len() + kommentar.len();
        saetze.push(Zentralsatz {
            fest,
            name,
            zusatz,
            kommentar,
            stelle,
            lokal,
            groesse64,
            gepackt64,
        });
        stelle += laenge as u64;
    }
    Ok(saetze)
}

/// Liest den lokalen Kopf an der Stelle des Lesers und gibt ihn unter dem
/// neuen Namen zurueck, zusammen mit der Laenge des alten Kopfs.
///
/// Das Info-ZIP-Namensfeld faellt weg, aus dem Grund bei
/// [`FELD_UNICODE_NAME`]; jedes andere Zusatzfeld bleibt, auch das lokale
/// ZIP64-Feld, denn es traegt Groessen und keinen Versatz.
pub(crate) fn lokalkopf_umbenannt<R: Read>(
    leser: &mut R,
    name: &[u8],
) -> io::Result<(Vec<u8>, u64)> {
    let mut fest = [0u8; 30];
    leser.read_exact(&mut fest)?;
    if u32_an(&fest, 0) != SIG_LOKAL {
        return Err(formfehler("zip: local header missing"));
    }
    let mut alter_name = vec![0u8; usize::from(u16_an(&fest, 26))];
    let mut zusatz = vec![0u8; usize::from(u16_an(&fest, 28))];
    leser.read_exact(&mut alter_name)?;
    leser.read_exact(&mut zusatz)?;
    let alte_laenge = (30 + alter_name.len() + zusatz.len()) as u64;

    let zusatz = match felder(&zusatz) {
        Ok(mut felder) => {
            felder.retain(|(kennung, _)| *kennung != FELD_UNICODE_NAME);
            felder_zusammen(&felder)?
        }
        // Ein lokales Zusatzfeld, das sich nicht zerlegen laesst, liest auch
        // kein anderer Leser; es bleibt, wie es war.
        Err(_) => zusatz,
    };
    let kennzeichen = kennzeichen_fuer(u16_an(&fest, 6), name);
    u16_setzen(&mut fest, 6, kennzeichen);
    let laenge = |teil: &[u8]| {
        u16::try_from(teil.len()).map_err(|_| formfehler("zip: name or field too long"))
    };
    u16_setzen(&mut fest, 26, laenge(name)?);
    u16_setzen(&mut fest, 28, laenge(&zusatz)?);
    let mut kopf = Vec::with_capacity(30 + name.len() + zusatz.len());
    kopf.extend_from_slice(&fest);
    kopf.extend_from_slice(name);
    kopf.extend_from_slice(&zusatz);
    Ok((kopf, alte_laenge))
}

/// Schreibt das Ende eines Zip hinter ein Zentralverzeichnis, das bei
/// `anfang` beginnt und `groesse` Bytes lang ist.
///
/// Mehr als 65 534 Saetze oder ein Versatz oder eine Groesse ueber 32 Bit
/// schreiben vorher das ZIP64-Ende und seinen Zeiger.
pub(crate) fn ende_schreiben<W: Write>(
    schreiber: &mut W,
    anzahl: u64,
    anfang: u64,
    groesse: u64,
    kommentar: &[u8],
) -> io::Result<()> {
    let anzahl16 = u16::try_from(anzahl).ok().filter(|&wert| wert != VOLL16);
    let anfang32 = u32::try_from(anfang).ok().filter(|&wert| wert != VOLL32);
    let groesse32 = u32::try_from(groesse).ok().filter(|&wert| wert != VOLL32);
    if anzahl16.is_none() || anfang32.is_none() || groesse32.is_none() {
        let stelle64 = anfang + groesse;
        let mut ende64 = Vec::with_capacity(56 + 20);
        ende64.extend_from_slice(&SIG_ENDE64.to_le_bytes());
        ende64.extend_from_slice(&44u64.to_le_bytes());
        ende64.extend_from_slice(&VERSION_ZIP64.to_le_bytes());
        ende64.extend_from_slice(&VERSION_ZIP64.to_le_bytes());
        ende64.extend_from_slice(&0u32.to_le_bytes());
        ende64.extend_from_slice(&0u32.to_le_bytes());
        ende64.extend_from_slice(&anzahl.to_le_bytes());
        ende64.extend_from_slice(&anzahl.to_le_bytes());
        ende64.extend_from_slice(&groesse.to_le_bytes());
        ende64.extend_from_slice(&anfang.to_le_bytes());
        ende64.extend_from_slice(&SIG_ZEIGER64.to_le_bytes());
        ende64.extend_from_slice(&0u32.to_le_bytes());
        ende64.extend_from_slice(&stelle64.to_le_bytes());
        ende64.extend_from_slice(&1u32.to_le_bytes());
        schreiber.write_all(&ende64)?;
    }
    let kommentarlaenge =
        u16::try_from(kommentar.len()).map_err(|_| formfehler("zip: comment too long"))?;
    let mut ende = Vec::with_capacity(22 + kommentar.len());
    ende.extend_from_slice(&SIG_ENDE.to_le_bytes());
    ende.extend_from_slice(&0u16.to_le_bytes());
    ende.extend_from_slice(&0u16.to_le_bytes());
    ende.extend_from_slice(&anzahl16.unwrap_or(VOLL16).to_le_bytes());
    ende.extend_from_slice(&anzahl16.unwrap_or(VOLL16).to_le_bytes());
    ende.extend_from_slice(&groesse32.unwrap_or(VOLL32).to_le_bytes());
    ende.extend_from_slice(&anfang32.unwrap_or(VOLL32).to_le_bytes());
    ende.extend_from_slice(&kommentarlaenge.to_le_bytes());
    ende.extend_from_slice(kommentar);
    schreiber.write_all(&ende)
}

#[cfg(test)]
mod pruefung {
    use std::io::{Cursor, Write};

    use zip::ZipWriter;
    use zip::write::FullFileOptions;

    use super::{ZIP64_AB, braucht_zip64, grosse_form_waehlen};

    /// Die Kennungen der Zusatzfelder im ersten lokalen Kopf eines Zip.
    fn felder_des_ersten_kopfs(daten: &[u8]) -> Vec<u16> {
        assert_eq!(&daten[..4], b"PK\x03\x04", "kein lokaler Kopf");
        let name = usize::from(u16::from_le_bytes([daten[26], daten[27]]));
        let zusatz = usize::from(u16::from_le_bytes([daten[28], daten[29]]));
        let mut rest = &daten[30 + name..30 + name + zusatz];
        let mut felder = Vec::new();
        while rest.len() >= 4 {
            felder.push(u16::from_le_bytes([rest[0], rest[1]]));
            let laenge = usize::from(u16::from_le_bytes([rest[2], rest[3]]));
            rest = &rest[(4 + laenge).min(rest.len())..];
        }
        felder
    }

    fn ein_eintrag(groesse: Option<u64>) -> Vec<u8> {
        let mut schreiber = ZipWriter::new(Cursor::new(Vec::new()));
        schreiber
            .start_file(
                "a.bin",
                grosse_form_waehlen(FullFileOptions::default(), groesse),
            )
            .expect("Eintrag beginnen");
        schreiber.write_all(b"abc").expect("schreiben");
        schreiber.finish().expect("abschliessen").into_inner()
    }

    /// Eine Datei ueber 4 GiB und eine unbekannter Groesse bekommen die
    /// ZIP64-Form, und zwar so, dass die Kiste sie in den lokalen Kopf
    /// schreibt; eine kleine Datei nicht. Ohne die Wahl schriebe die Kiste den
    /// Eintrag ohne ZIP64-Feld und braeche bei 4 GiB ab (Defekt
    /// `261017-1923-eine-datei-ueber-4-gib-laesst-sich-in-kein-zip-kopieren.md`).
    #[test]
    fn eine_datei_ab_vier_gib_und_eine_unbekannter_groesse_bekommen_zip64() {
        assert!(braucht_zip64(Some((1 << 32) + 4096)));
        assert!(braucht_zip64(Some(u64::from(u32::MAX))));
        assert!(braucht_zip64(Some(ZIP64_AB)));
        assert!(braucht_zip64(None));
        assert!(!braucht_zip64(Some(ZIP64_AB - 1)));
        assert!(!braucht_zip64(Some(0)));

        assert!(felder_des_ersten_kopfs(&ein_eintrag(Some((1 << 32) + 4096))).contains(&0x0001));
        assert!(felder_des_ersten_kopfs(&ein_eintrag(None)).contains(&0x0001));
        assert!(!felder_des_ersten_kopfs(&ein_eintrag(Some(3))).contains(&0x0001));
    }
}
