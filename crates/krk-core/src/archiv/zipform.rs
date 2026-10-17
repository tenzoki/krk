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

use std::fs::{self, File};
use std::io::{BufReader, Read, Seek, SeekFrom};
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
    mut wahl: FullFileOptions<'static>,
    angaben: Option<&fs::Metadata>,
) -> FullFileOptions<'static> {
    let Some(geaendert) = angaben.and_then(|angaben| angaben.modified().ok()) else {
        return wahl;
    };
    // Die Zugriffszeit ist die Zugabe und nicht der Gegenstand: gibt sie nichts
    // her, steht das Aenderungsdatum an beiden Stellen, so wie `ditto(1)` es
    // auch haelt. Sie gibt in zwei Lagen nichts her, und beide sind hier
    // gleich behandelt: sie fehlt, oder sie liegt ausserhalb der vier Byte, die
    // die zwei Zusatzfelder dafuer vorsehen. Haenge die Felder an die Zugabe,
    // verhinderte eine Zugriffszeit von 1969 das Zusatzfeld eines tadellosen
    // Aenderungsdatums, und zwar ohne eine Zeile in der Abschlussliste.
    let gelesen = angaben.and_then(|angaben| angaben.accessed().ok());

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
fn zipfehler(fehler: ZipError) -> Verzeichnisfehler {
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
