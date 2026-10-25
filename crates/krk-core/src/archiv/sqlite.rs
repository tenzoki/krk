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
//! **Zwei Faelle, je an einer Frage entschieden**, und [`oeffnen`] ist die
//! einzige Stelle, die eine Datenbank oeffnet. Geoeffnet wird ueber die URI
//! `file:<pfad>?mode=ro` mit `SQLITE_OPEN_READONLY | SQLITE_OPEN_URI`
//! (`verzeichnis::sys::sqlite_uri`, `Datenbank::oeffnen`), mit einer
//! Wartezeit von [`WARTEZEIT`] auf die Sperre eines Schreibers, und sofort
//! danach liest eine erste Anweisung das Schema. Davon gibt es zwei
//! Ausnahmen, beide mit derselben URI und `&immutable=1`, und beide allein
//! fuer einen Dateikopf, der in den Bytes 18 und 19 die WAL-Kennung traegt
//! (je `2`):
//!
//! - **Vor dem ersten Oeffnen:** steht daneben ein `<datei>-wal` mit 0 Bytes
//!   **und** keine `<datei>-shm`, wird sofort unveraenderlich geoeffnet. Ein
//!   leeres `-wal` traegt nichts Neueres als die Hauptdatei, also liest
//!   `immutable=1` den vollen Stand, und `mode=ro` legte hier eine `-shm` an
//!   (gemessen in der Analyse
//!   `261024-2133-shm-neben-wal-ohne-shm-zusage-korrigieren-oder-anlegen-verhindern.md`,
//!   Abschnitt 1; Entscheid
//!   `261024-2215-wie-liest-krk-eine-wal-datenbank-mit-wal-und-ohne-shm.md`,
//!   Moeglichkeit 4). So stehen etwa die Datenbanken von Programmen im
//!   Sperrmodus `exclusive` da, die nie eine `-shm` anlegen. **Vorher fragt
//!   KRK mit `fcntl(F_GETLK)`, ob ein anderer Prozess im Sperrbereich von
//!   SQLite eine Sperre haelt** (`verzeichnis::sys::fremde_sperre`, ab
//!   `0x4000_0000` ueber 512 Bytes, an demselben Deskriptor, an dem der Kopf
//!   gelesen wird). Ein solches Programm haelt, solange es laeuft, dasselbe
//!   Bild aus leerem `-wal` ohne `-shm` und dazu seine Sperre; dann, und bei
//!   jedem Fehler der Frage, oeffnet `mode=ro`, das neben ihm mit
//!   `SQLITE_BUSY` endet und nichts anlegt (gemessen am 261025), und der Ort
//!   meldet „wird gerade geschrieben“ (Issue
//!   `261025-0030-die-vorfrage-zum-leeren-wal-liest-neben-einem-laufenden-exklusiven-schreiber-ohne-sperre-und-niemand-sagt-es.md`,
//!   Weg B).
//! - **Nach einem gescheiterten ersten Oeffnen:** scheitert `mode=ro` oder
//!   das erste Lesen mit `SQLITE_CANTOPEN`, **und** steht kein `<datei>-wal`
//!   daneben, wird unveraenderlich geoeffnet, gleich ob eine `-shm` steht.
//!   Das ist die ruhende WAL-Datenbank, etwa eine Kopie ohne ihre
//!   Begleitdateien: `mode=ro` kann dort den Index nicht anlegen oder nicht
//!   nutzen und scheitert, `immutable=1` liest (gemessen, Bericht Abschnitt 3,
//!   und `messungen/261023-1556-sqlite-nur-lesend-shm.txt`, Versuch d; dass
//!   `mode=ro` bei stehender `-shm` und fehlendem `-wal` ebenso mit
//!   `SQLITE_CANTOPEN` scheitert, gemessen am 261024).
//!
//! In jedem anderen Fall geht der Fehler an den Rufer. Die Faelle
//! ueberlappen nicht: die erste Ausnahme verlangt ein stehendes `-wal`, die
//! zweite ein fehlendes. **Ein nicht leeres `-wal` ohne `-shm`**, etwa nach
//! einem Absturz des letzten Schreibers oder in einer Kopie, oeffnet
//! `mode=ro` wie jede andere Datenbank, und dann legt die Bibliothek eine
//! `-shm` an (Entscheid
//! `261024-2215-wie-liest-krk-eine-wal-datenbank-mit-wal-und-ohne-shm.md`,
//! fuer den Rest Moeglichkeit 1): `immutable=1` laese dort die Zeilen im
//! `-wal` nicht.
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
//! der Datenbank weder `-wal` noch `-journal` an.** **Ein leeres `-wal` ohne
//! `-shm` liest KRK unveraenderlich und legt nichts an; ein nicht leeres
//! `-wal` ohne `-shm` (Absturz, Kopie) laesst SQLite eine `-shm` anlegen**
//! (Oeffnungsregel oben). **Steht neben einer WAL-Datenbank schon ein
//! `-shm`, darf die Bibliothek dort eine Lesemarke setzen** oder, ist KRK die
//! einzige Verbindung, den Index darin neu aufbauen; das ist die eine
//! Aenderung, die ein Lesen an einer stehenden Datei hinterlaesst, und sie
//! trifft allein `-shm`. So gemessen in
//! `messungen/261023-1556-sqlite-nur-lesend-shm.txt` (Urteile: Hauptdatei
//! unveraendert, `-wal` unveraendert, `-shm` veraendert; das Schliessen der
//! Verbindung faehrt keinen Checkpoint und entfernt `-wal` nicht) und vom
//! Nutzer so angenommen (Entscheid
//! `261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md`):
//! bei laufendem Schreiber liest KRK den neuesten Stand mit `mode=ro`, und
//! `immutable=1` bleibt den zwei Faellen der Oeffnungsregel vorbehalten, in
//! denen es nichts Neueres zu lesen gibt.
//!
//! **Im Augenblick der Vorfrage, und nicht darueber hinaus.** Die Sperrfrage
//! vor dem ersten Fall sieht ein Programm, das schon laeuft; eines, das
//! zwischen Vorfrage und Oeffnen oder waehrend des Lesens startet und
//! schreibt, sieht sie nicht. Das ist dieselbe Klasse von Wettlauf wie bei
//! der Rueckfallstufe, und sie ist nicht geschlossen: eine Tabelle schuetzt
//! die Kennungspruefung von `Archivleser::datenbank_lesen` vor und nach dem
//! Lesen, das Inhaltsverzeichnis mit seinen Zeilenzahlen prueft niemand nach.
//! **Ein Restfall bleibt:** haelt ein anderer Prozess allein eine geteilte
//! Sperre und steht trotzdem keine `-shm` daneben, was nur geschieht, wenn
//! jemand die `-shm` unter einem laufenden Programm geloescht hat, dann
//! oeffnet `mode=ro`, und das kann dort eine `-shm` anlegen.
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
//! # Eine Tabelle als TSV, eine Sicht als SQL-Text
//!
//! **Ein Leser fuer Vorschau und Ausfuhr, ohne Schalter** (P1 des Plans):
//! [`tabelle_lesen`] und [`sicht_lesen`] erreicht jeder Rufer allein ueber
//! `Archivleser::lesen`, und Vorschau wie Kopieren bekommen dieselben Bytes.
//! Eine Tabelle kommt als `SELECT * FROM "<name>"` ohne `ORDER BY`, der Name
//! als Bezeichner und nie als SQL-Text (P3). Die erste Zeile traegt die
//! Spaltennamen, jede weitere eine Zeile der Tabelle; jedes Feld geht ueber
//! `trenntext::feld_kodieren` mit dem Tabulator, das Satzende ist `\n` (P2).
//! Ganzzahl und Kommazahl stehen in der Textform der Bibliothek, Text, der
//! kein UTF-8 ist, mit U+FFFD. **`NULL` steht als `NULL`, ein BLOB als
//! Platzhalter mit seiner Laenge in Bytes**, beide aus der Sprachtabelle
//! (`Text::SqliteNull`, `Zahlwort::SqliteBlob`), in Vorschau und Ausfuhr
//! gleich (Entscheid
//! `261023-1507-wie-stehen-null-und-blob-in-der-tsv-einer-sqlite-tabelle.md`,
//! Moeglichkeit b): die Ausfuhr verliert damit jeden BLOB, ihr Platzhalter
//! wechselt mit der Sprache der Oberflaeche, und eine Zelle mit dem Text
//! `NULL` ist von einem echten `NULL` nicht zu unterscheiden. Gereicht wird
//! in Bloecken von [`BLOCK`] Bytes; zwischen zwei Zeilen fragt der Leser die
//! Abbruchmarke selbst, waehrend einer Anweisung fragt sie der
//! Fortschrittsrueckruf der Bindung (P7). Eine Sicht liefert den Text der
//! Spalte `sql` aus `sqlite_master` und wird nie ausgefuehrt.
//!
//! # Fehler
//!
//! [`fehler_zuordnen`] macht aus einem Fehler der Bibliothek einen
//! [`Verzeichnisfehler`]: eine gesetzte Abbruchmarke und `SQLITE_INTERRUPT`
//! werden `Abgebrochen`, `SQLITE_BUSY` und `SQLITE_LOCKED` `Belegt` (P8),
//! `SQLITE_NOTADB` `Ungueltig` mit dem Grund `Text::SqliteKeineDatenbank`, ein
//! Ein- oder Ausgabefehler und ein gescheitertes Oeffnen mit einem Fehler des
//! Systems `Lesen`, jeder andere `Ungueltig` mit dem Wortlaut der Bibliothek.

use std::borrow::Cow;
use std::fs::{self, File, Metadata};
use std::io::{self, Read};
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use crate::fern::Fernfaden;
use crate::sprache::{Text, Zahlwort, anzahl, text};
use crate::trenntext::{Trennzeichen, feld_kodieren};
use crate::verzeichnis::Typ;
use crate::verzeichnis::sys::{
    Datenbank, SQLITE_BUSY, SQLITE_CANTOPEN, SQLITE_INTERRUPT, SQLITE_MISUSE, SQLITE_NOTADB,
    Sqlitefehler, Wert, fremde_sperre, ohne_warten_oeffnen, sqlite_uri,
};

use super::format::Archivformat;
use super::innenpfad::Innenpfad;
use super::lesen::Lesefehler;
use super::verzeichnis::{
    Archiveintrag, Fundstelle, Lesbarkeit, Sammler, Verzeichnisfehler, abbruch_fragen,
};
use super::zugang::Dateikennung;

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

/// Der SQL-Text einer Sicht, ihr Name als gebundener Parameter (P3).
const SICHTTEXT: &str = "SELECT sql FROM sqlite_master WHERE type = 'view' AND name = ?1";

/// Ab wie vielen Bytes der TSV ein Block an den Rufer geht; so gross wie der
/// Puffer, in dem `archiv::lesen` die Daten der anderen Formate reicht.
const BLOCK: usize = 256 * 1024;

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
    if sofort_unveraenderlich(datei) {
        return nur_lesend_oeffnen(datei, true, abbruch);
    }
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
    // Ein Pfad mit Nullbyte kommt aus keinem Dateisystem; weist `sqlite_uri`
    // ihn trotzdem ab, geht er als Fehlbenutzung weiter, die keine zweite
    // Oeffnung mit `immutable=1` ausloest und `fehler_zuordnen` als ungueltig
    // meldet.
    let uri = sqlite_uri(datei, unveraenderlich).map_err(|fehler| Sqlitefehler {
        code: SQLITE_MISUSE,
        wortlaut: fehler.to_string(),
        systemfehler: None,
    })?;
    let datenbank = Datenbank::oeffnen(&uri, abbruch)?;
    datenbank.wartezeit_setzen(WARTEZEIT)?;
    datenbank.vorbereiten(SCHEMAPROBE)?.naechste_zeile()?;
    Ok(datenbank)
}

/// Wo der Sperrbereich von SQLite in der Hauptdatei beginnt: das Byte
/// `PENDING` bei `0x4000_0000` (`os_unix.c`), dahinter `RESERVED` und die 510
/// Bytes des geteilten Bereichs ab `0x4000_0002`.
const SPERRBEREICH_START: i64 = 0x4000_0000;

/// Wie lang der gefragte Sperrbereich ist: `PENDING`, `RESERVED` und der
/// geteilte Bereich zusammen. Ein Schreiber im Sperrmodus `exclusive` haelt
/// seine ausschliessende Sperre auf dem geteilten Bereich (`0x4000_0002`,
/// Laenge 510, gemessen am 261025); wer allein `PENDING` und `RESERVED`
/// fragt, uebersieht sie.
const SPERRBEREICH_LAENGE: i64 = 512;

/// Ob die Datenbank `datei` vor dem ersten Oeffnen sofort unveraenderlich
/// geoeffnet wird (Modulkopf, der Fall vor dem ersten Oeffnen): der Kopf
/// traegt die WAL-Kennung, daneben steht ein leeres `-wal` und keine `-shm`,
/// **und kein anderer Prozess haelt eine Sperre im Sperrbereich**. Im
/// Zweifel, also bei jedem Fehler einer dieser Fragen, nein: dann oeffnet
/// `mode=ro`.
///
/// Kopf und Sperre werden an **demselben** Deskriptor gefragt, der fuer die
/// ganze Vorfrage offen bleibt: POSIX hebt beim Schliessen jedes Deskriptors
/// auf die Datei alle `fcntl`-Sperren des Prozesses auf ihr auf, und ein
/// zweites Oeffnen und Schliessen nur fuer die Sperrfrage hiesse ein weiteres
/// Mal, an dem das geschehen kann.
fn sofort_unveraenderlich(datei: &Path) -> bool {
    let Ok(mut offen) = ohne_warten_oeffnen(datei) else {
        return false;
    };
    kopf_traegt_wal(&mut offen)
        && leeres_wal_ohne_shm(datei)
        && matches!(
            fremde_sperre(&offen, SPERRBEREICH_START, SPERRBEREICH_LAENGE),
            Ok(false)
        )
}

/// Ob der Kopf von `datei` die WAL-Kennung traegt (Bytes 18 und 19 je `2`).
/// Laesst sich der Kopf nicht lesen, nein.
fn wal_kopf(datei: &Path) -> bool {
    ohne_warten_oeffnen(datei).is_ok_and(|mut offen| kopf_traegt_wal(&mut offen))
}

/// Ob die ersten 20 Bytes der offenen Datei die WAL-Kennung tragen; gelesen
/// ab der Stelle des Deskriptors, also vom Anfang einer frisch geoeffneten
/// Datei. Laesst sich der Kopf nicht lesen, nein.
fn kopf_traegt_wal(offen: &mut File) -> bool {
    let mut kopf = [0u8; 20];
    offen.read_exact(&mut kopf).is_ok() && kopf[18] == 2 && kopf[19] == 2
}

/// Ob die Datenbank `datei` mit WAL-Kopf ruht und kein `<datei>-wal` daneben
/// steht (Modulkopf, die Rueckfallstufe). Im Zweifel, also wenn sich der Kopf
/// nicht lesen oder `-wal` nicht befragen laesst, nein: dann bleibt es beim
/// Fehler des ersten Oeffnens. Ob `-shm` steht, fragt sie nicht: steht `-shm`
/// und fehlt `-wal`, scheitert `mode=ro` ebenso mit `SQLITE_CANTOPEN`, und
/// diese Stufe muss greifen.
fn ruhende_wal_datenbank_ohne_wal(datei: &Path) -> bool {
    wal_kopf(datei) && matches!(wal_angaben(datei), Ok(None))
}

/// Ob neben `datei` ein leeres `-wal` (0 Bytes) und keine `-shm` steht
/// (Modulkopf, der Fall vor dem ersten Oeffnen). Im Zweifel, also wenn sich
/// eine der zwei Dateien nicht befragen laesst, nein: dann oeffnet `mode=ro`.
fn leeres_wal_ohne_shm(datei: &Path) -> bool {
    let wal_leer = matches!(wal_angaben(datei), Ok(Some(angaben)) if angaben.len() == 0);
    wal_leer
        && matches!(
            fs::symlink_metadata(begleitpfad(datei, "-shm")),
            Err(fehler) if fehler.kind() == io::ErrorKind::NotFound
        )
}

/// Der Pfad einer Begleitdatei (`-wal`, `-shm`) neben der aufgeloesten Datei
/// (Modulkopf).
fn begleitpfad(datei: &Path, endung: &str) -> PathBuf {
    let aufgeloest = fs::canonicalize(datei).unwrap_or_else(|_| datei.to_path_buf());
    let mut name = aufgeloest.into_os_string();
    name.push(endung);
    PathBuf::from(name)
}

/// Der Pfad von `-wal` neben der aufgeloesten Datei (Modulkopf).
fn wal_pfad(datei: &Path) -> PathBuf {
    begleitpfad(datei, "-wal")
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

/// Die Kennung der Datenbank `datei` am Pfad: die Hauptdatei und, falls es
/// steht, `-wal` (Modulkopf). Dieselbe Rechnung wie die des Zugangs, am Pfad
/// statt am Deskriptor, weil die Bibliothek die Datei am Pfad oeffnet.
///
/// # Errors
///
/// Jeder Fehler von `stat(2)` an der Hauptdatei, und an `-wal` jeder ausser
/// „nicht vorhanden“.
pub(crate) fn kennung(datei: &Path) -> io::Result<Dateikennung> {
    let angaben = fs::metadata(datei)?;
    let wal = wal_angaben(datei)?;
    Ok(Dateikennung::aus_angaben(&angaben).mit_wal(wal.as_ref()))
}

/// Liest die Tabelle `name` als TSV und reicht sie in Bloecken an `empfang`
/// (Modulkopf, „Eine Tabelle als TSV“); `Break` dort beendet die Anweisung.
/// Gibt die Zahl der Bytes zurueck.
///
/// # Errors
///
/// [`Lesefehler::Abgebrochen`] nach einer gesetzten Marke oder einem `Break`,
/// sonst der Fehler der Bibliothek nach [`fehler_zuordnen`].
pub(crate) fn tabelle_lesen(
    pfand: &Fernfaden,
    datei: &Path,
    name: &str,
    abbruch: &AtomicBool,
    empfang: &mut dyn FnMut(&[u8]) -> ControlFlow<()>,
) -> Result<u64, Lesefehler> {
    let zuordnen = |fehler| lesefehler(fehler_zuordnen(fehler, abbruch));
    let datenbank = oeffnen(pfand, datei, abbruch).map_err(zuordnen)?;
    let mut abfrage = datenbank
        .vorbereiten(&format!("SELECT * FROM {}", bezeichner(name)))
        .map_err(zuordnen)?;

    let mut block = Vec::with_capacity(BLOCK);
    let mut geliefert: u64 = 0;
    let kopf: Vec<Wert> = abfrage
        .spaltennamen()
        .map_err(zuordnen)?
        .into_iter()
        .map(Wert::Text)
        .collect();
    satz_anhaengen(&mut block, &kopf);
    loop {
        if abbruch.load(Ordering::Acquire) {
            return Err(Lesefehler::Abgebrochen);
        }
        let Some(zeile) = abfrage.naechste_zeile().map_err(zuordnen)? else {
            break;
        };
        satz_anhaengen(&mut block, &zeile);
        if block.len() >= BLOCK {
            geliefert += reichen(&mut block, empfang)?;
        }
    }
    geliefert += reichen(&mut block, empfang)?;
    Ok(geliefert)
}

/// Liest den SQL-Text der Sicht `name` aus `sqlite_master` und reicht ihn an
/// `empfang`. Die Sicht wird nicht ausgefuehrt (Entscheid
/// `261023-1340-zeigt-krk-sichten-einer-sqlite-datenbank-oder-nur-ihren-sql-text.md`),
/// und ihr Name geht als gebundener Parameter in die Abfrage (P3).
///
/// # Errors
///
/// Wie [`tabelle_lesen`], und [`Lesefehler::Fehlt`], wenn keine Sicht dieses
/// Namens steht.
pub(crate) fn sicht_lesen(
    pfand: &Fernfaden,
    datei: &Path,
    name: &str,
    abbruch: &AtomicBool,
    empfang: &mut dyn FnMut(&[u8]) -> ControlFlow<()>,
) -> Result<u64, Lesefehler> {
    let zuordnen = |fehler| lesefehler(fehler_zuordnen(fehler, abbruch));
    let datenbank = oeffnen(pfand, datei, abbruch).map_err(zuordnen)?;
    let mut abfrage = datenbank.vorbereiten(SICHTTEXT).map_err(zuordnen)?;
    abfrage.text_binden(1, name).map_err(zuordnen)?;
    // Die Spalte `sql` einer Sicht ist immer ein Text; ein anderer Wert hiesse
    // ein von Hand veraendertes Schema und kommt als „fehlt“ zurueck.
    let Some(Wert::Text(sql)) = abfrage
        .naechste_zeile()
        .map_err(zuordnen)?
        .and_then(|zeile| zeile.into_iter().next())
    else {
        return Err(Lesefehler::Fehlt(Archivformat::Sqlite));
    };
    let mut block = sql.into_bytes();
    reichen(&mut block, empfang)
}

/// Haengt einen Satz der TSV an `block`: jedes Feld ueber
/// `trenntext::feld_kodieren` mit dem Tabulator, das Satzende `\n` (P2).
fn satz_anhaengen(block: &mut Vec<u8>, werte: &[Wert]) {
    for (stelle, wert) in werte.iter().enumerate() {
        if stelle > 0 {
            block.push(b'\t');
        }
        let feld = feldtext(wert);
        block.extend_from_slice(feld_kodieren(&feld, Trennzeichen::Tabulator).as_bytes());
    }
    block.push(b'\n');
}

/// Der Text einer Zelle vor dem Kodieren: Zahlen in der Textform der
/// Bibliothek, `NULL` und ein BLOB als Platzhalter aus der Sprachtabelle
/// (Entscheid
/// `261023-1507-wie-stehen-null-und-blob-in-der-tsv-einer-sqlite-tabelle.md`).
fn feldtext(wert: &Wert) -> Cow<'_, str> {
    match wert {
        Wert::Null => Cow::Borrowed(text(Text::SqliteNull)),
        Wert::Ganzzahl(zahl) | Wert::Kommazahl(zahl) => Cow::Borrowed(zahl),
        Wert::Text(inhalt) => Cow::Borrowed(inhalt),
        Wert::Blob(laenge) => Cow::Owned(anzahl(Zahlwort::SqliteBlob, *laenge, &[])),
    }
}

/// Reicht `block` an `empfang`, wenn er nicht leer ist, und leert ihn; gibt
/// die Zahl der gereichten Bytes zurueck.
fn reichen(
    block: &mut Vec<u8>,
    empfang: &mut dyn FnMut(&[u8]) -> ControlFlow<()>,
) -> Result<u64, Lesefehler> {
    if block.is_empty() {
        return Ok(0);
    }
    let laenge = block.len() as u64;
    if empfang(block).is_break() {
        return Err(Lesefehler::Abgebrochen);
    }
    block.clear();
    Ok(laenge)
}

/// Ein Fehler beim Oeffnen oder Lesen der Datenbank als [`Lesefehler`].
/// Vollstaendig ueber [`Verzeichnisfehler`] und ohne Auffangzweig.
fn lesefehler(fehler: Verzeichnisfehler) -> Lesefehler {
    match fehler {
        Verzeichnisfehler::Abgebrochen => Lesefehler::Abgebrochen,
        Verzeichnisfehler::Lesen(fehler) => Lesefehler::Lesen(fehler),
        Verzeichnisfehler::Ungueltig(wortlaut) => Lesefehler::Ungueltig(wortlaut),
        andere @ (Verzeichnisfehler::KeineDatei
        | Verzeichnisfehler::ZuGross
        | Verzeichnisfehler::Belegt) => Lesefehler::Verzeichnis(andere),
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
