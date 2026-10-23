//! Eine SQLite-Datenbank als Ort: die eine Oeffnungsregel und das
//! Inhaltsverzeichnis aus `sqlite_master`.
//!
//! Eine Datenbank ist das fuenfte [`Archivformat`](super::Archivformat), allein lesbar (Zuschnitt
//! D1 des Berichts
//! `261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md` im
//! Arbeitspaket
//! `261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen`).
//! Einstieg, Register, Pfand, Rueckkehr und Auffrischung sind die des
//! Archivwegs; dieses Modul ist der einzige Teil des Kerns, der die Bindung
//! an `libsqlite3` in `verzeichnis::sys` anspricht.
//!
//! # Was an der Wurzel steht
//!
//! Jede Zeile von `sqlite_master` mit `type` gleich `table` oder `view`, auch
//! die inneren Tabellen mit dem Vorsatz `sqlite_`; Indizes und Trigger nicht
//! (P5 des Plans
//! `261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md`).
//! Ein Name geht ueber [`Innenpfad::aus_einzelnamen`] ins Verzeichnis, die
//! eine Namensregel des Archivwegs; ein Name mit `/`, `..` oder leer zaehlt in
//! die ausgelassenen (P4). Eine Tabelle traegt ihre Zeilenzahl aus
//! `SELECT count(*)`, gezaehlt beim Aufbau des Verzeichnisses (Entscheid
//! `261023-1340-was-zeigt-die-spalte-groesse-einer-tabelle.md`); scheitert das
//! Zaehlen, etwa an einer virtuellen Tabelle, deren Modul die Bibliothek nicht
//! kennt, steht die Tabelle trotzdem da, mit unbekannter Zeilenzahl (P6). Eine
//! Sicht wird nicht ausgefuehrt und hat keine Zeilenzahl (Entscheid
//! `261023-1340-zeigt-krk-sichten-einer-sqlite-datenbank-oder-nur-ihren-sql-text.md`).
//! Jeder Eintrag traegt die Aenderungszeit der Datei.
//!
//! Ein Name, der kein UTF-8 ist, kommt als Text mit U+FFFD an seiner Stelle;
//! sein Zaehlen scheitert dann am ersetzten Namen, und er steht mit
//! unbekannter Zeilenzahl da.
//!
//! # Die Oeffnungsregel
//!
//! **Ein Fall und keine Kette**, und [`oeffnen`] ist die einzige Stelle, die
//! eine Datenbank oeffnet. Geoeffnet wird ueber die URI
//! `file:<pfad>?mode=ro` mit `SQLITE_OPEN_READONLY | SQLITE_OPEN_URI`
//! (`verzeichnis::sys::sqlite_uri`, `Datenbank::oeffnen`), mit einer
//! Wartezeit von [`WARTEZEIT`] auf die Sperre eines Schreibers, und sofort
//! danach liest eine erste Anweisung das Schema. Scheitert das Oeffnen oder
//! dieses erste Lesen mit `SQLITE_CANTOPEN`, **und** traegt der Dateikopf in
//! den Bytes 18 und 19 die WAL-Kennung (je `2`), **und** steht kein
//! `<datei>-wal` daneben, wird dieselbe URI mit `&immutable=1` geoeffnet. In
//! jedem anderen Fall geht der Fehler an den Rufer. Das ist die ruhende
//! WAL-Datenbank, etwa eine Kopie ohne ihre Begleitdateien: `mode=ro` kann
//! dort den Index in `-shm` nicht anlegen und scheitert, `immutable=1` liest
//! (gemessen, Bericht Abschnitt 3, und
//! `messungen/261023-1556-sqlite-nur-lesend-shm.txt`, Versuch d). Die drei
//! Bedingungen ueberlappen nicht: entweder liest `mode=ro`, oder genau dieser
//! eine Fall liest unveraenderlich, oder es gibt eine Meldung.
//!
//! # Die Kennung ueber zwei Dateien
//!
//! Ein Schreiber im WAL-Betrieb uebergibt nach `<datei>-wal` und laesst die
//! Hauptdatei bis zum Checkpoint stehen. Die `Dateikennung` einer Datenbank
//! umfasst deshalb neben der Hauptdatei Geraet, Inode, Groesse und
//! Aenderungszeit von `-wal`, falls es steht ([`wal_angaben`]); gefragt wird
//! sie vor dem Lesen, und ein Schreiber, der zwischen dieser Frage und dem
//! Lesen beginnt, aendert sie, sodass das naechste `Archivzugang::verzeichnis`
//! neu liest. Auch der Schreiber, der nach einem unveraenderlichen Lesen ein
//! `-wal` anlegt, aendert sie. `-wal` steht neben der aufgeloesten Datei, wie
//! die Bibliothek es anlegt; aufgeloest wird mit `realpath(3)` und, wenn das
//! scheitert, der Pfad genommen, wie er ist.
//!
//! # Nur lesen
//!
//! **KRK veraendert weder die Hauptdatei noch ihr `-wal`, und es legt neben
//! der Datenbank keine Datei an**, weder `-wal` noch `-shm` noch `-journal`.
//! **Steht neben einer WAL-Datenbank schon ein `-shm`, darf die Bibliothek
//! dort eine Lesemarke setzen** oder, ist KRK die einzige Verbindung, den
//! Index darin neu aufbauen; das ist die eine Aenderung, die ein Lesen
//! hinterlaesst, und sie trifft allein `-shm`. So gemessen in
//! `messungen/261023-1556-sqlite-nur-lesend-shm.txt` (Urteile: Hauptdatei
//! unveraendert, `-wal` unveraendert, `-shm` veraendert; das Schliessen der
//! Verbindung faehrt keinen Checkpoint und entfernt `-wal` nicht) und vom
//! Nutzer so angenommen (Entscheid
//! `261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md`):
//! bei laufendem Schreiber liest KRK den neuesten Stand mit `mode=ro`, und
//! `immutable=1` bleibt allein der ruhenden WAL-Datenbank ohne `-wal`
//! vorbehalten, wo es nichts Neueres zu lesen gibt.
//!
//! Was die Bindung zusaetzlich zusagt, steht an `Datenbank`: es gibt keinen
//! Weg, eine Datenbank anders als nur lesend zu oeffnen, und Erweiterungen
//! laedt die Bibliothek des Systems nicht nach.
//!
//! # Was die Pruefung am Deskriptor nicht schliesst
//!
//! `Archivzugang::verzeichnis` prueft am Deskriptor, dass unter dem Namen eine
//! gewoehnliche Datei steht, bevor es hierher kommt; geoeffnet wird danach
//! von der Bibliothek, am Pfad. Wer den Namen zwischen beiden durch eine
//! benannte Roehre ersetzt, laesst das Oeffnen warten. Das ist ein
//! Wettlauf und kein gewoehnlicher Weg, und er haelt allein den Arbeitsfaden
//! an, nicht den Hauptfaden.
//!
//! # Fehler
//!
//! [`fehler_zuordnen`] macht aus einem Fehler der Bibliothek einen
//! [`Verzeichnisfehler`]: eine gesetzte Abbruchmarke und `SQLITE_INTERRUPT`
//! werden `Abgebrochen`, `SQLITE_BUSY` und `SQLITE_LOCKED` `Belegt` (P8),
//! `SQLITE_NOTADB` `Ungueltig` mit dem Text „keine SQLite-Datenbank“, ein
//! Ein- oder Ausgabefehler und ein gescheitertes Oeffnen mit einem Fehler des
//! Systems `Lesen`, jeder andere `Ungueltig` mit dem Wortlaut der Bibliothek.

use std::fs::{self, Metadata};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use crate::fern::Fernfaden;
use crate::sprache::{Text, text};
use crate::verzeichnis::Typ;
use crate::verzeichnis::sys::{
    Datenbank, SQLITE_BUSY, SQLITE_CANTOPEN, SQLITE_INTERRUPT, SQLITE_NOTADB, Sqlitefehler, Wert,
    ohne_warten_oeffnen, sqlite_uri,
};

use super::innenpfad::Innenpfad;
use super::verzeichnis::{
    Archiveintrag, Fundstelle, Lesbarkeit, Sammler, Verzeichnisfehler, abbruch_fragen,
};

/// Wie lange eine Anweisung auf die Sperre eines Schreibers wartet, bevor sie
/// aufgibt und der Ort „wird gerade geschrieben“ meldet (P8 des Plans). Kurz,
/// weil auf diese Antwort eine Auffrischung oder ein Einstieg wartet.
pub(crate) const WARTEZEIT: Duration = Duration::from_millis(200);

/// `SQLITE_LOCKED` aus `sqlite3.h`: eine Sperre innerhalb desselben Prozesses.
/// Die Bindung in `verzeichnis::sys` nennt den Wert nicht, weil keiner ihrer
/// Aufrufe ihn eigens meldet; zugeordnet wird er hier wie `SQLITE_BUSY`.
const SQLITE_LOCKED: i32 = 6;

/// `SQLITE_IOERR` aus `sqlite3.h`: ein Ein- oder Ausgabefehler. Nur er und
/// `SQLITE_CANTOPEN` werden mit ihrem Fehler des Systems zu einem Lesefehler,
/// denn `sqlite3_system_errno` nennt den letzten Fehler des Systems an der
/// Verbindung, auch einen alten, etwa das fehlende `-wal` beim Oeffnen.
const SQLITE_IOERR: i32 = 10;

/// Die Abfrage nach Tabellen und Sichten (P5 des Plans).
const SCHEMA: &str = "SELECT type, name FROM sqlite_master WHERE type IN ('table', 'view')";

/// Die erste Anweisung nach dem Oeffnen: sie liest das Schema und laesst
/// dabei ein `SQLITE_CANTOPEN` oder `SQLITE_NOTADB` hervortreten.
const SCHEMAPROBE: &str = "SELECT 1 FROM sqlite_master LIMIT 1";

/// Oeffnet die Datenbank `datei` nach der einen Oeffnungsregel (Modulkopf).
///
/// # Errors
///
/// Jeder Fehler der Bibliothek beim Oeffnen, beim Setzen der Wartezeit und
/// beim ersten Lesen des Schemas, ausser dem einen `SQLITE_CANTOPEN`, das die
/// Regel mit `immutable=1` beantwortet.
pub(crate) fn oeffnen<'a>(
    _pfand: &Fernfaden,
    datei: &Path,
    abbruch: &'a AtomicBool,
) -> Result<Datenbank<'a>, Sqlitefehler> {
    match nur_lesend_oeffnen(datei, false, abbruch) {
        Err(fehler)
            if fehler.grundcode() == SQLITE_CANTOPEN && ruhende_wal_datenbank_ohne_wal(datei) =>
        {
            nur_lesend_oeffnen(datei, true, abbruch)
        }
        ergebnis => ergebnis,
    }
}

/// Oeffnet ueber die URI aus `sqlite_uri`, setzt die Wartezeit und liest das
/// Schema ein erstes Mal.
fn nur_lesend_oeffnen<'a>(
    datei: &Path,
    unveraenderlich: bool,
    abbruch: &'a AtomicBool,
) -> Result<Datenbank<'a>, Sqlitefehler> {
    let datenbank = Datenbank::oeffnen(&sqlite_uri(datei, unveraenderlich), abbruch)?;
    datenbank.wartezeit_setzen(WARTEZEIT)?;
    datenbank.vorbereiten(SCHEMAPROBE)?.naechste_zeile()?;
    Ok(datenbank)
}

/// Ob der Kopf von `datei` die WAL-Kennung traegt (Bytes 18 und 19 je `2`)
/// und kein `<datei>-wal` daneben steht. Im Zweifel, also wenn sich der Kopf
/// nicht lesen oder `-wal` nicht befragen laesst, nein: dann bleibt es beim
/// Fehler des ersten Oeffnens.
fn ruhende_wal_datenbank_ohne_wal(datei: &Path) -> bool {
    let mut kopf = [0u8; 20];
    let kopf_gelesen = ohne_warten_oeffnen(datei)
        .and_then(|mut offen| offen.read_exact(&mut kopf))
        .is_ok();
    kopf_gelesen && kopf[18] == 2 && kopf[19] == 2 && matches!(wal_angaben(datei), Ok(None))
}

/// Der Pfad von `-wal` neben der aufgeloesten Datei (Modulkopf).
fn wal_pfad(datei: &Path) -> PathBuf {
    let aufgeloest = fs::canonicalize(datei).unwrap_or_else(|_| datei.to_path_buf());
    let mut name = aufgeloest.into_os_string();
    name.push("-wal");
    PathBuf::from(name)
}

/// Die Angaben von `<datei>-wal`, oder `None`, wenn es nicht steht. Fuer die
/// `Dateikennung` einer Datenbank (Modulkopf).
///
/// # Errors
///
/// Jeder Fehler von `stat(2)` ausser „nicht vorhanden“.
pub(crate) fn wal_angaben(datei: &Path) -> io::Result<Option<Metadata>> {
    match fs::metadata(wal_pfad(datei)) {
        Ok(angaben) => Ok(Some(angaben)),
        Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(fehler) => Err(fehler),
    }
}

/// Liest Tabellen und Sichten der Datenbank `datei` in den Sammler
/// (Modulkopf). `angaben` sind die der Hauptdatei, am Deskriptor gefragt.
///
/// # Errors
///
/// [`Verzeichnisfehler`] nach [`fehler_zuordnen`], und `ZuGross` aus dem
/// Sammler.
pub(crate) fn verzeichnis_lesen(
    pfand: &Fernfaden,
    datei: &Path,
    angaben: &Metadata,
    sammler: &mut Sammler,
    abbruch: &AtomicBool,
) -> Result<(), Verzeichnisfehler> {
    let zuordnen = |fehler| fehler_zuordnen(fehler, abbruch);
    let datenbank = oeffnen(pfand, datei, abbruch).map_err(zuordnen)?;

    let mut gefunden: Vec<(bool, Option<String>)> = Vec::new();
    {
        let mut abfrage = datenbank.vorbereiten(SCHEMA).map_err(zuordnen)?;
        while let Some(zeile) = abfrage.naechste_zeile().map_err(zuordnen)? {
            let sicht = matches!(zeile.first(), Some(Wert::Text(art)) if art == "view");
            let name = match zeile.into_iter().nth(1) {
                Some(Wert::Text(name)) => Some(name),
                _ => None,
            };
            gefunden.push((sicht, name));
        }
    }

    let geaendert = angaben.modified().unwrap_or(UNIX_EPOCH);
    for (sicht, name) in gefunden {
        abbruch_fragen(abbruch)?;
        let Some((name, innen)) = name.and_then(|name| {
            let innen = Innenpfad::aus_einzelnamen(&name)?;
            Some((name, innen))
        }) else {
            sammler.auslassen();
            continue;
        };
        let (fundstelle, groesse) = if sicht {
            (Fundstelle::SqliteSicht, 0)
        } else {
            let zeilen = zeilen_zaehlen(&datenbank, &name, abbruch)?;
            (Fundstelle::SqliteTabelle { zeilen }, zeilen.unwrap_or(0))
        };
        sammler.aufnehmen(Archiveintrag {
            innen,
            typ: Typ::Datei,
            groesse,
            geaendert,
            rechte: None,
            fundstelle,
            lesbarkeit: Lesbarkeit::Lesbar,
        })?;
    }
    Ok(())
}

/// Die Zeilen der Tabelle `name`, oder `None`, wenn das Zaehlen an der
/// Tabelle scheitert (P6). Ein Abbruch und die Sperre eines Schreibers gehen
/// an den Rufer: sie betreffen die Datenbank und nicht die Tabelle.
fn zeilen_zaehlen(
    datenbank: &Datenbank<'_>,
    name: &str,
    abbruch: &AtomicBool,
) -> Result<Option<u64>, Verzeichnisfehler> {
    let gezaehlt = datenbank
        .vorbereiten(&format!("SELECT count(*) FROM {}", bezeichner(name)))
        .and_then(|mut abfrage| abfrage.naechste_zeile());
    match gezaehlt {
        Ok(Some(zeile)) => Ok(match zeile.first() {
            Some(Wert::Ganzzahl(zahl)) => zahl.parse().ok(),
            _ => None,
        }),
        Ok(None) => Ok(None),
        Err(fehler) => match fehler_zuordnen(fehler, abbruch) {
            ernst @ (Verzeichnisfehler::Abgebrochen | Verzeichnisfehler::Belegt) => Err(ernst),
            Verzeichnisfehler::Lesen(_)
            | Verzeichnisfehler::KeineDatei
            | Verzeichnisfehler::Ungueltig(_)
            | Verzeichnisfehler::ZuGross => Ok(None),
        },
    }
}

/// Ein Name als Bezeichner in doppelten Anfuehrungszeichen, mit verdoppeltem
/// `"` darin (P3 des Plans). Ein Name geht nie als SQL-Text in eine Abfrage.
fn bezeichner(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Ordnet einen Fehler der Bibliothek einem [`Verzeichnisfehler`] zu
/// (Modulkopf). Eine gesetzte Abbruchmarke geht vor.
pub(crate) fn fehler_zuordnen(fehler: Sqlitefehler, abbruch: &AtomicBool) -> Verzeichnisfehler {
    let grundcode = fehler.grundcode();
    if abbruch.load(Ordering::Acquire) || grundcode == SQLITE_INTERRUPT {
        return Verzeichnisfehler::Abgebrochen;
    }
    match grundcode {
        SQLITE_BUSY | SQLITE_LOCKED => Verzeichnisfehler::Belegt,
        SQLITE_NOTADB => Verzeichnisfehler::Ungueltig(text(Text::SqliteKeineDatenbank).to_owned()),
        SQLITE_IOERR | SQLITE_CANTOPEN if fehler.systemfehler.is_some() => {
            Verzeichnisfehler::Lesen(io::Error::from_raw_os_error(
                fehler.systemfehler.unwrap_or_default(),
            ))
        }
        _ => Verzeichnisfehler::Ungueltig(fehler.to_string()),
    }
}
