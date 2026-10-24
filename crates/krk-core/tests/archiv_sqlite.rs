//! Abnahme von `Archivformat::Sqlite`, dem Inhaltsverzeichnis einer
//! Datenbank und dem Lesen einer Tabelle als TSV und einer Sicht als SQL-Text,
//! Schritte 3 und 4 des Plans, und der Zusage „nur lesen“ neben einem
//! laufenden Schreiber, Schritt 5 desselben Plans
//! `261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md`
//! im Arbeitspaket
//! `261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen`.
//!
//! Jede Probe legt ihre Datenbank selbst im Pruefordner an, mit
//! `/usr/bin/sqlite3`, das hier allein als Schreiber der Pruefdaten dient
//! (Modulkopf von `tests/sqlite_bindung.rs`). Gelesen wird allein ueber die
//! Wege des Kerns. Keine Probe liest eine Datenbank ausserhalb des
//! Pruefordners.

mod gemeinsam;

use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use gemeinsam::Pruefordner;
use krk_core::archiv::umbau::Umbaufehler;
use krk_core::archiv::{
    Archivformat, Archivleser, Archivzugang, Fundstelle, Inhaltsverzeichnis, Innenpfad, Lesbarkeit,
    Lesefehler, Verzeichnisfehler,
};
use krk_core::fern::faden;
use krk_core::operation::{Bericht, Fernauftrag, Meldung, OhnePapierkorb, starten};
use krk_core::ort::{Archivort, Ort, nicht_verfuegbar_text};
use krk_core::sprache::{Sprache, Text, Zahlwort, text};
use krk_core::trenntext::{Dialekt, indexieren};
use krk_core::verzeichnis::Typ;

/// Legt mit `/usr/bin/sqlite3` eine Datenbank unter `pfad` an und fuehrt `sql`
/// darin aus.
fn anlegen(pfad: &Path, sql: &str) {
    let ausgabe = Command::new("/usr/bin/sqlite3")
        .arg(pfad)
        .arg(sql)
        .output()
        .expect("/usr/bin/sqlite3 laesst sich nicht starten");
    assert!(
        ausgabe.status.success(),
        "sqlite3 scheitert an {}: {}",
        pfad.display(),
        String::from_utf8_lossy(&ausgabe.stderr)
    );
}

/// Fragt mit `/usr/bin/sqlite3` eine Zahl ab, als Erwartung fuer eine Tabelle,
/// deren Zeilen die Bibliothek selbst schreibt.
fn zahl_abfragen(pfad: &Path, sql: &str) -> u64 {
    let ausgabe = Command::new("/usr/bin/sqlite3")
        .arg(pfad)
        .arg(sql)
        .output()
        .expect("/usr/bin/sqlite3 laesst sich nicht starten");
    String::from_utf8_lossy(&ausgabe.stdout)
        .trim()
        .parse()
        .expect("eine Zahl")
}

/// Ein `/usr/bin/sqlite3`, das offen bleibt, bis die Probe es freigibt; seine
/// Befehle bekommt es ueber die Standardeingabe.
struct Schreiber {
    kind: Child,
    eingabe: Option<ChildStdin>,
}

impl Schreiber {
    fn starten(pfad: &Path, sql: &str) -> Self {
        let mut kind = Command::new("/usr/bin/sqlite3")
            .arg(pfad)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("/usr/bin/sqlite3 laesst sich nicht starten");
        let mut eingabe = kind.stdin.take().expect("Standardeingabe");
        eingabe.write_all(sql.as_bytes()).expect("schreiben");
        eingabe.flush().expect("leeren");
        Schreiber {
            kind,
            eingabe: Some(eingabe),
        }
    }
}

impl Drop for Schreiber {
    fn drop(&mut self) {
        let _ = self.kind.kill();
        drop(self.eingabe.take());
        let _ = self.kind.wait();
    }
}

fn zugang(pfad: &Path) -> Arc<Archivzugang> {
    let format = Archivformat::aus_namen(pfad.file_name().expect("Name"))
        .expect("die Probe gibt ihrer Datenbank eine Endung aus der Tafel");
    Arc::new(Archivzugang::neu(pfad.to_path_buf(), format))
}

/// Liest das Verzeichnis eines Zugangs in einem Faden mit Pfand.
fn lesen_mit(zugang: &Arc<Archivzugang>) -> Result<Arc<Inhaltsverzeichnis>, Verzeichnisfehler> {
    let zugang = Arc::clone(zugang);
    faden("krk-sqliteprobe".into(), move |pfand| {
        zugang.verzeichnis(pfand, &AtomicBool::new(false))
    })
    .expect("ein Faden laesst sich starten")
    .join()
    .expect("der Lesefaden geraet nicht in Panik")
}

fn lesen(pfad: &Path) -> Result<Arc<Inhaltsverzeichnis>, Verzeichnisfehler> {
    lesen_mit(&zugang(pfad))
}

fn innen(name: &str) -> Innenpfad {
    Innenpfad::aus_einzelnamen(name).expect("ein sicherer Name")
}

/// Die Namen an der Wurzel, geordnet.
fn namen(verzeichnis: &Inhaltsverzeichnis) -> Vec<String> {
    let mut namen: Vec<String> = verzeichnis
        .kinder(&Innenpfad::wurzel())
        .expect("die Wurzel steht im Verzeichnis")
        .map(|eintrag| eintrag.innen.als_text().to_owned())
        .collect();
    namen.sort();
    namen
}

fn neben(pfad: &Path, endung: &str) -> PathBuf {
    let mut name = pfad.as_os_str().to_owned();
    name.push(endung);
    PathBuf::from(name)
}

#[test]
fn die_vier_endungen_ergeben_sqlite_und_keine_andere() {
    let tafel = [
        ("a.sqlite", Some(Archivformat::Sqlite)),
        ("A.SQLITE", Some(Archivformat::Sqlite)),
        ("a.sqlite3", Some(Archivformat::Sqlite)),
        ("a.SqLiTe3", Some(Archivformat::Sqlite)),
        ("wordstore.db", Some(Archivformat::Sqlite)),
        ("Thumbs.DB", Some(Archivformat::Sqlite)),
        ("a.db3", Some(Archivformat::Sqlite)),
        ("a.DB3", Some(Archivformat::Sqlite)),
        (".db", Some(Archivformat::Sqlite)),
        ("a.db.gz", Some(Archivformat::Gz)),
        ("a.sqlite.zip", Some(Archivformat::Zip)),
        ("a.db-wal", None),
        ("a.db-shm", None),
        ("a.sqlite-journal", None),
        ("a.dbx", None),
        ("a.db4", None),
        ("a.sqlite2", None),
        ("db", None),
        ("adb", None),
    ];
    for (name, erwartet) in tafel {
        assert_eq!(
            Archivformat::aus_namen(OsStr::new(name)),
            erwartet,
            "die Endung von {name}"
        );
    }
    assert!(!Archivformat::Sqlite.beschreibbar());
}

#[test]
fn tabellen_und_sichten_stehen_an_der_wurzel_mit_zeilenzahl() {
    let ordner = Pruefordner::neu("sqlite-wurzel");
    let datei = ordner.unter("wordstore.db");
    anlegen(
        &datei,
        "CREATE TABLE woerter(id INTEGER PRIMARY KEY AUTOINCREMENT, wort TEXT);
         WITH RECURSIVE n(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM n WHERE i < 250)
           INSERT INTO woerter(wort) SELECT 'wort' || i FROM n;
         CREATE TABLE quellen(id INTEGER PRIMARY KEY, name TEXT NOT NULL) STRICT;
         INSERT INTO quellen VALUES (1, 'a'), (2, 'b'), (3, 'c');
         CREATE TABLE paare(links TEXT, rechts TEXT, PRIMARY KEY(links, rechts)) WITHOUT ROWID;
         INSERT INTO paare VALUES ('a', 'b'), ('b', 'c');
         CREATE TABLE streng_ohne(k TEXT PRIMARY KEY, v INTEGER) STRICT, WITHOUT ROWID;
         INSERT INTO streng_ohne VALUES ('x', 1);
         CREATE TABLE leer(x);
         CREATE TABLE kategorien(id INTEGER PRIMARY KEY, name TEXT);
         INSERT INTO kategorien(name) VALUES ('Nomen'), ('Verb');
         CREATE TABLE zuordnung(wort INTEGER, kategorie INTEGER);
         INSERT INTO zuordnung VALUES (1, 1), (2, 2), (3, 1), (4, 1);
         CREATE TABLE verlauf(zeit TEXT, wort INTEGER);
         INSERT INTO verlauf VALUES ('heute', 1);
         CREATE TABLE \"Mit Leerzeichen und Ä\"(x);
         INSERT INTO \"Mit Leerzeichen und Ä\" VALUES (1), (2);
         CREATE TABLE notizen(text TEXT);
         CREATE TABLE \"a\"\"b\"(x);
         INSERT INTO \"a\"\"b\" VALUES (7);
         CREATE INDEX woerter_wort ON woerter(wort);
         CREATE UNIQUE INDEX kategorien_name ON kategorien(name);
         CREATE TRIGGER verlauf_nach AFTER INSERT ON woerter BEGIN
           INSERT INTO verlauf VALUES ('jetzt', new.id);
         END;
         CREATE VIEW nomen AS SELECT wort FROM woerter JOIN zuordnung ON woerter.id = zuordnung.wort WHERE kategorie = 1;
         CREATE VIEW anzahl AS SELECT count(*) FROM woerter;
         CREATE VIEW kaputt AS SELECT 1 / 0;
         CREATE VIEW leer_sicht AS SELECT * FROM leer;
         ANALYZE;",
    );

    let verzeichnis = lesen(&datei).expect("die Datenbank wird gelesen");
    let tabellen = [
        ("woerter", 250),
        ("sqlite_sequence", 1),
        ("quellen", 3),
        ("paare", 2),
        ("streng_ohne", 1),
        ("leer", 0),
        ("kategorien", 2),
        ("zuordnung", 4),
        ("verlauf", 1),
        ("Mit Leerzeichen und Ä", 2),
        ("notizen", 0),
        ("a\"b", 1),
        // ANALYZE legt `sqlite_stat1` an; wie viele Zeilen die Bibliothek
        // dort schreibt, fragt die Probe sie selbst.
        (
            "sqlite_stat1",
            zahl_abfragen(&datei, "SELECT count(*) FROM sqlite_stat1"),
        ),
    ];
    let sichten = ["nomen", "anzahl", "kaputt", "leer_sicht"];

    let mut erwartet: Vec<String> = tabellen
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .chain(sichten.iter().map(|name| (*name).to_owned()))
        .collect();
    erwartet.sort();
    assert_eq!(namen(&verzeichnis), erwartet, "Indizes und Trigger fehlen");
    assert_eq!(verzeichnis.ausgelassen(), 0);

    let dateizeit = fs::metadata(&datei).unwrap().modified().unwrap();
    for (name, zeilen) in tabellen {
        let eintrag = verzeichnis.eintrag(&innen(name)).expect(name);
        assert_eq!(eintrag.typ, Typ::Datei, "{name}");
        assert_eq!(
            eintrag.fundstelle,
            Fundstelle::SqliteTabelle {
                zeilen: Some(zeilen)
            },
            "{name}"
        );
        assert_eq!(eintrag.groesse, zeilen, "{name}");
        assert_eq!(verzeichnis.zeilen(&innen(name)), Some(zeilen), "{name}");
        assert_eq!(eintrag.geaendert, dateizeit, "{name}");
        assert_eq!(eintrag.lesbarkeit, Lesbarkeit::Lesbar, "{name}");
    }
    for name in sichten {
        let eintrag = verzeichnis.eintrag(&innen(name)).expect(name);
        assert_eq!(eintrag.typ, Typ::Datei, "{name}");
        assert_eq!(eintrag.fundstelle, Fundstelle::SqliteSicht, "{name}");
        assert_eq!(eintrag.groesse, 0, "{name}");
        assert_eq!(verzeichnis.zeilen(&innen(name)), None, "{name}");
        assert_eq!(eintrag.geaendert, dateizeit, "{name}");
    }
    assert_eq!(verzeichnis.zeilen(&innen("woerter_wort")), None);
    assert!(verzeichnis.eintrag(&innen("verlauf_nach")).is_none());
}

#[test]
fn ein_tabellenname_mit_schraegstrich_wird_ausgelassen_und_gezaehlt() {
    let ordner = Pruefordner::neu("sqlite-namen");
    let datei = ordner.unter("namen.sqlite");
    anlegen(
        &datei,
        "CREATE TABLE \"a/b\"(x);
         CREATE TABLE \"..\"(x);
         CREATE TABLE \"\"(x);
         CREATE TABLE \".\"(x);
         CREATE VIEW \"/etc\" AS SELECT 1;
         CREATE TABLE gut(x);
         INSERT INTO gut VALUES (1), (2);",
    );
    let verzeichnis = lesen(&datei).expect("die Datenbank wird gelesen");
    assert_eq!(namen(&verzeichnis), ["gut"]);
    assert_eq!(verzeichnis.ausgelassen(), 5);
    assert_eq!(verzeichnis.zeilen(&innen("gut")), Some(2));
}

#[test]
fn eine_virtuelle_tabelle_ohne_modul_steht_mit_unbekannter_zeilenzahl() {
    let ordner = Pruefordner::neu("sqlite-virtuell");
    let datei = ordner.unter("virtuell.db");
    // `/usr/bin/sqlite3` legt den Eintrag einer virtuellen Tabelle mit einem
    // Modul an, das keine Bibliothek kennt, ueber das beschreibbare Schema.
    anlegen(
        &datei,
        "CREATE TABLE echt(x);
         INSERT INTO echt VALUES (1);
         PRAGMA writable_schema = ON;
         INSERT INTO sqlite_master(type, name, tbl_name, rootpage, sql)
           VALUES ('table', 'vt', 'vt', 0, 'CREATE VIRTUAL TABLE vt USING gibtesnicht(x)');",
    );
    let verzeichnis = lesen(&datei).expect("die Datenbank wird gelesen");
    assert_eq!(namen(&verzeichnis), ["echt", "vt"]);
    let vt = verzeichnis.eintrag(&innen("vt")).expect("vt");
    assert_eq!(vt.fundstelle, Fundstelle::SqliteTabelle { zeilen: None });
    assert_eq!(vt.groesse, 0);
    assert_eq!(verzeichnis.zeilen(&innen("vt")), None);
    assert_eq!(verzeichnis.zeilen(&innen("echt")), Some(1));
}

#[test]
fn eine_ruhende_wal_datenbank_ohne_wal_datei_wird_unveraenderlich_gelesen() {
    let ordner = Pruefordner::neu("sqlite-ruhend");
    let datei = ordner.unter("ruhend.db");
    anlegen(
        &datei,
        "PRAGMA journal_mode = WAL;
         CREATE TABLE t(x);
         INSERT INTO t VALUES (1), (2), (3);
         PRAGMA wal_checkpoint(TRUNCATE);",
    );
    // Das `/usr/bin/sqlite3` von macOS laesst `-wal` und `-shm` beim Beenden
    // stehen; nach dem Checkpoint steht alles in der Hauptdatei, und die Probe
    // nimmt beide fort, wie eine Kopie ohne Begleitdateien.
    for endung in ["-wal", "-shm"] {
        let _ = fs::remove_file(neben(&datei, endung));
    }
    let kopf = fs::read(&datei).unwrap();
    assert_eq!(
        (kopf[18], kopf[19]),
        (2, 2),
        "der Kopf traegt die WAL-Kennung"
    );
    assert!(!neben(&datei, "-wal").exists());
    assert!(!neben(&datei, "-shm").exists());
    let vorher = fs::read(&datei).unwrap();

    let verzeichnis = lesen(&datei).expect("die ruhende Datenbank wird gelesen");
    assert_eq!(namen(&verzeichnis), ["t"]);
    assert_eq!(verzeichnis.zeilen(&innen("t")), Some(3));

    assert_eq!(fs::read(&datei).unwrap(), vorher, "die Hauptdatei bleibt");
    let mut daneben: Vec<String> = fs::read_dir(ordner.pfad())
        .unwrap()
        .map(|eintrag| eintrag.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    daneben.sort();
    assert_eq!(daneben, ["ruhend.db"], "neben der Datei entsteht nichts");
}

#[test]
fn eine_datei_mit_endung_db_die_keine_datenbank_ist_meldet_keine_sqlite_datenbank() {
    let ordner = Pruefordner::neu("sqlite-keine");
    let datei = ordner.datei(
        "Thumbs.db",
        b"das ist keine Datenbank, sondern ein Satz. ".repeat(100),
    );
    match lesen(&datei) {
        Err(Verzeichnisfehler::Ungueltig(grund)) => {
            assert_eq!(grund, text(Text::SqliteKeineDatenbank));
        }
        andere => panic!("erwartet keine SQLite-Datenbank, gelesen {andere:?}"),
    }
}

#[test]
fn ein_gesperrter_schreiber_ergibt_belegt() {
    let ordner = Pruefordner::neu("sqlite-belegt");
    let datei = ordner.unter("belegt.db");
    anlegen(&datei, "CREATE TABLE t(x); INSERT INTO t VALUES (1);");
    let _schreiber = Schreiber::starten(&datei, "BEGIN EXCLUSIVE;\nINSERT INTO t VALUES (2);\n");

    // Wann der Schreiber seine Sperre hat, sagt er nicht; gefragt wird mit je
    // einem frischen Zugang, damit kein gespeichertes Verzeichnis antwortet.
    let frist = Instant::now() + Duration::from_secs(10);
    loop {
        match lesen(&datei) {
            Err(Verzeichnisfehler::Belegt) => break,
            Ok(_) if Instant::now() < frist => std::thread::sleep(Duration::from_millis(20)),
            andere => panic!("erwartet Belegt, gelesen {andere:?}"),
        }
    }
}

#[test]
fn eine_neue_tabelle_hinter_wal_aendert_die_kennung() {
    let ordner = Pruefordner::neu("sqlite-wal-kennung");
    let datei = ordner.unter("wal.sqlite3");
    anlegen(
        &datei,
        "PRAGMA journal_mode = WAL; CREATE TABLE alt(x); INSERT INTO alt VALUES (1);",
    );
    let zugang = zugang(&datei);
    let erstes = lesen_mit(&zugang).expect("erstes Lesen");
    assert_eq!(namen(&erstes), ["alt"]);
    let zweites = lesen_mit(&zugang).expect("zweites Lesen");
    assert!(
        Arc::ptr_eq(&erstes, &zweites),
        "eine unveraenderte Datenbank wird nicht zweimal gelesen"
    );
    let hauptdatei_vorher = fs::read(&datei).unwrap();

    let _schreiber = Schreiber::starten(
        &datei,
        "PRAGMA wal_autocheckpoint = 0;\nCREATE TABLE neu(x);\nINSERT INTO neu VALUES (1), (2);\n",
    );
    let frist = Instant::now() + Duration::from_secs(10);
    let drittes = loop {
        let verzeichnis = lesen_mit(&zugang).expect("Lesen neben dem Schreiber");
        if verzeichnis.zeilen(&innen("neu")) == Some(2) {
            break verzeichnis;
        }
        assert!(
            Instant::now() < frist,
            "die Tabelle hinter -wal erscheint nicht: {:?}",
            namen(&verzeichnis)
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(namen(&drittes), ["alt", "neu"]);
    assert!(!Arc::ptr_eq(&erstes, &drittes));
    assert!(neben(&datei, "-wal").exists());
    assert_eq!(
        fs::read(&datei).unwrap(),
        hauptdatei_vorher,
        "der Schreiber hat allein in -wal uebergeben; die Kennung hat es an -wal gesehen"
    );
}

/// Faehrt einen Auftrag und liefert den Bericht.
fn fahren(auftrag: Fernauftrag) -> Bericht {
    let lauf = starten(auftrag, Arc::new(OhnePapierkorb));
    let bericht = loop {
        match lauf
            .meldungen()
            .recv_timeout(Duration::from_secs(60))
            .expect("der Lauf meldet sich nicht")
        {
            Meldung::Fortschritt(_) | Meldung::Uebersprungen(_) => {}
            Meldung::Konflikt { .. } => panic!("unerwartete Konfliktfrage"),
            Meldung::Fertig(bericht) => break bericht,
        }
    };
    lauf.warten();
    bericht
}

fn gruende(bericht: &Bericht) -> Vec<String> {
    bericht
        .uebersprungen
        .iter()
        .map(|eintrag| eintrag.grund.clone())
        .collect()
}

#[test]
fn in_einer_datenbank_meldet_jeder_melder_den_text_der_datenbank() {
    let ordner = Pruefordner::neu("sqlite-melder");
    let datei = ordner.unter("melder.db");
    anlegen(&datei, "CREATE TABLE t(x); INSERT INTO t VALUES (1);");
    let vorher = fs::read(&datei).unwrap();
    let wurzel = Archivort::wurzel(zugang(&datei));

    // Die zwei Funktionen am Format, fuer jedes Format.
    for (format, nicht_verfuegbar, nur_lesbar) in [
        (
            Archivformat::Sqlite,
            Text::InDatenbankNichtVerfuegbar,
            Text::DatenbankNurLesbar,
        ),
        (
            Archivformat::Zip,
            Text::ImArchivNichtVerfuegbar,
            Text::ArchivNurLesbar,
        ),
        (
            Archivformat::Tar,
            Text::ImArchivNichtVerfuegbar,
            Text::ArchivNurLesbar,
        ),
        (
            Archivformat::TarGz,
            Text::ImArchivNichtVerfuegbar,
            Text::ArchivNurLesbar,
        ),
        (
            Archivformat::Gz,
            Text::ImArchivNichtVerfuegbar,
            Text::ArchivNurLesbar,
        ),
    ] {
        assert_eq!(
            format.nicht_verfuegbar_text(),
            nicht_verfuegbar,
            "{format:?}"
        );
        assert_eq!(format.nur_lesbar_text(), nur_lesbar, "{format:?}");
        assert_eq!(
            Umbaufehler::NurLesbar(format).grund(),
            text(nur_lesbar),
            "{format:?}"
        );
    }

    // `ort::nicht_verfuegbar_text` am Ort.
    assert_eq!(
        nicht_verfuegbar_text(&Ort::Archiv(wurzel.clone())),
        Some(Text::InDatenbankNichtVerfuegbar)
    );
    for name in ["a.zip", "a.tar", "a.tgz", "a.txt.gz"] {
        let archiv = Archivort::wurzel(zugang(&ordner.unter(name)));
        assert_eq!(
            nicht_verfuegbar_text(&Ort::Archiv(archiv)),
            Some(Text::ImArchivNichtVerfuegbar),
            "{name}"
        );
    }

    // Die Melder der Vorgangsmaschine: hineinkopieren, herausverschieben und
    // loeschen in einer Datenbank.
    let quelle = ordner.datei("neu.txt", b"neu");
    let tabelle = Ort::Archiv(wurzel.kind("t").expect("ein einzelner Name"));
    let ziel = ordner.ordner("ziel");
    let datenbanktext = text(Text::DatenbankNurLesbar);

    let hinein = fahren(Fernauftrag::kopieren(
        vec![Ort::Lokal(quelle.clone())],
        Ort::Archiv(wurzel.clone()),
    ));
    assert_eq!(gruende(&hinein), [datenbanktext], "hineinkopieren");

    let heraus = fahren(Fernauftrag::verschieben(
        vec![tabelle.clone()],
        Ort::Lokal(ziel.clone()),
    ));
    assert_eq!(gruende(&heraus), [datenbanktext], "herausverschieben");

    let geloescht = fahren(Fernauftrag::loeschen(vec![tabelle]));
    assert_eq!(gruende(&geloescht), [datenbanktext], "loeschen");

    assert_eq!(fs::read(&datei).unwrap(), vorher, "die Datenbank bleibt");
    assert!(quelle.exists());
    assert_eq!(fs::read_dir(&ziel).unwrap().count(), 0);
}

// ---------------------------------------------------------------------------
// Schritt 4: eine Tabelle als TSV, eine Sicht als SQL-Text
// ---------------------------------------------------------------------------

/// Liest den Eintrag `name` der Datenbank hinter `zugang` ueber den
/// `Archivleser`, auf einem Faden mit Pfand, und reicht jeden Block an
/// `empfang`; zurueck kommen das Ergebnis von `lesen` und die gesammelten
/// Bytes.
fn eintrag_lesen_mit(
    zugang: &Arc<Archivzugang>,
    name: &str,
    mut empfang: impl FnMut(&[u8]) -> ControlFlow<()> + Send + 'static,
) -> (Result<u64, Lesefehler>, Vec<u8>) {
    let zugang = Arc::clone(zugang);
    let innen = innen(name);
    faden("krk-sqliteprobe-lesen".into(), move |pfand| {
        let mut leser = Archivleser::oeffnen(pfand, zugang, Arc::new(AtomicBool::new(false)))
            .expect("das Verzeichnis entsteht");
        let mut daten = Vec::new();
        let ergebnis = leser.lesen(pfand, &innen, |block| {
            daten.extend_from_slice(block);
            empfang(block)
        });
        (ergebnis, daten)
    })
    .expect("ein Faden laesst sich starten")
    .join()
    .expect("der Lesefaden geraet nicht in Panik")
}

/// Die Daten des Eintrags `name`, ganz gelesen.
fn eintrag_lesen(datei: &Path, name: &str) -> Vec<u8> {
    let (ergebnis, daten) = eintrag_lesen_mit(&zugang(datei), name, |_| ControlFlow::Continue(()));
    let gelesen = ergebnis.expect("der Eintrag wird gelesen");
    assert_eq!(gelesen, daten.len() as u64, "die gemeldete Zahl der Bytes");
    daten
}

/// Die Tabelle der Bindungsprobe `eine_datenbank_wird_nur_lesend_geoeffnet_und_liest_jede_wertart`
/// (`tests/sqlite_bindung.rs`), um Zeilen mit Tabulator, `"` und
/// Zeilenumbruch in einem Text, einem leeren Text, einem leeren, einem
/// einbytigen und einem BLOB ueber tausend Bytes erweitert, und eine Zeile,
/// deren Text woertlich `NULL` lautet.
const WERTE: &str = "CREATE TABLE t(leer, ganz, komma, text, roh);
     INSERT INTO t VALUES (NULL, 42, 3.5, 'Grüße', x'00ff10');
     INSERT INTO t VALUES ('', -7, 0.25, 'Tab' || char(9) || 'und \"Zitat\"' || char(10) || 'neue Zeile', x'');
     INSERT INTO t VALUES ('NULL', 0, -1.5, NULL, x'01');
     INSERT INTO t VALUES (NULL, 1, 2.0, 'x', zeroblob(1234));";

/// Die Bytes, die der Entscheid
/// `261023-1507-wie-stehen-null-und-blob-in-der-tsv-einer-sqlite-tabelle.md`
/// fuer [`WERTE`] verlangt, in der Sprache der Proben (Deutsch): `NULL` als
/// `NULL`, ein BLOB als Platzhalter mit seiner Laenge, Felder nach
/// `trenntext::feld_kodieren`, Satzende `\n`.
const WERTE_ALS_TSV: &str = "leer\tganz\tkomma\ttext\troh\n\
     NULL\t42\t3.5\tGrüße\t[BLOB, 3 Bytes]\n\
     \t-7\t0.25\t\"Tab\tund \"\"Zitat\"\"\nneue Zeile\"\t[BLOB, 0 Bytes]\n\
     NULL\t0\t-1.5\tNULL\t[BLOB, 1 Byte]\n\
     NULL\t1\t2.0\tx\t[BLOB, 1.234 Bytes]\n";

#[test]
fn eine_tabelle_kommt_als_tsv_mit_spaltennamen_und_jeder_zeile() {
    let ordner = Pruefordner::neu("sqlite-tsv");
    let datei = ordner.unter("werte.db");
    anlegen(&datei, WERTE);

    let daten = eintrag_lesen(&datei, "t");
    let tsv = String::from_utf8(daten).expect("die TSV ist UTF-8");
    assert_eq!(tsv, WERTE_ALS_TSV);

    // Die Platzhalter stehen in jeder Sprache in der Tabelle.
    for sprache in [Sprache::De, Sprache::Fr, Sprache::En] {
        assert_eq!(sprache.text(Text::SqliteNull), "NULL", "{sprache:?}");
    }
    assert_eq!(
        Sprache::Fr.anzahl(Zahlwort::SqliteBlob, 1234, &[]),
        "[BLOB, 1\u{202F}234 octets]"
    );
    assert_eq!(
        Sprache::En.anzahl(Zahlwort::SqliteBlob, 1, &[]),
        "[BLOB, 1 byte]"
    );

    // Zurueckgelesen ueber den einen Leser fuer CSV und TSV ergeben sich
    // dieselben Zellen.
    let index = indexieren(&tsv, &Dialekt::tsv());
    let zellen: Vec<Vec<String>> = (0..index.zeilenzahl())
        .map(|satz| {
            index
                .felder(&tsv, satz)
                .expect("ein Satz")
                .into_iter()
                .map(|feld| feld.wert.into_owned())
                .collect()
        })
        .collect();
    let blob = |n: &str| format!("[BLOB, {n}]");
    assert_eq!(
        zellen,
        [
            vec!["leer", "ganz", "komma", "text", "roh"]
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            vec![
                "NULL".to_owned(),
                "42".to_owned(),
                "3.5".to_owned(),
                "Grüße".to_owned(),
                blob("3 Bytes"),
            ],
            vec![
                String::new(),
                "-7".to_owned(),
                "0.25".to_owned(),
                "Tab\tund \"Zitat\"\nneue Zeile".to_owned(),
                blob("0 Bytes"),
            ],
            vec![
                "NULL".to_owned(),
                "0".to_owned(),
                "-1.5".to_owned(),
                "NULL".to_owned(),
                blob("1 Byte"),
            ],
            vec![
                "NULL".to_owned(),
                "1".to_owned(),
                "2.0".to_owned(),
                "x".to_owned(),
                blob("1.234 Bytes"),
            ],
        ]
    );
}

/// Eine Tabelle, deren TSV mehrere Bloecke und mehr als eine Lesegrenze der
/// Vorschau (1 MiB) fuellt: `zeilen` Zeilen mit je hundert Zeichen Text.
fn grosse_tabelle(datei: &Path, zeilen: u32) {
    anlegen(
        datei,
        &format!(
            "CREATE TABLE gross(n INTEGER, text TEXT);
             WITH RECURSIVE z(i) AS (SELECT 1 UNION ALL SELECT i + 1 FROM z WHERE i < {zeilen})
               INSERT INTO gross SELECT i, printf('%0100d', i) FROM z;"
        ),
    );
}

#[test]
fn ein_abbruch_im_rueckruf_beendet_die_anweisung() {
    let ordner = Pruefordner::neu("sqlite-abbruch");
    let datei = ordner.unter("gross.db");
    grosse_tabelle(&datei, 40_000);

    let mut bloecke = 0u32;
    let (ergebnis, daten) = eintrag_lesen_mit(&zugang(&datei), "gross", move |_| {
        bloecke += 1;
        assert_eq!(bloecke, 1, "nach dem Abbruch kommt kein weiterer Block");
        ControlFlow::Break(())
    });
    assert!(
        matches!(ergebnis, Err(Lesefehler::Abgebrochen)),
        "erwartet abgebrochen, gelesen {ergebnis:?}"
    );
    assert!(!daten.is_empty(), "der erste Block ist angekommen");
    assert!(
        (daten.len() as u64) < 1 << 20,
        "der erste Block reicht nicht ueber die Lesegrenze: {}",
        daten.len()
    );

    // Ganz gelesen ist dieselbe Tabelle groesser als die Lesegrenze.
    assert!(eintrag_lesen(&datei, "gross").len() > 1 << 20);
}

#[test]
fn eine_sicht_kommt_als_ihr_sql_text_und_wird_nicht_ausgefuehrt() {
    let ordner = Pruefordner::neu("sqlite-sicht");
    let datei = ordner.unter("sicht.db");
    let sicht = "CREATE VIEW kaputt AS SELECT abs(-9223372036854775808) AS ueberlauf";
    anlegen(&datei, &format!("CREATE TABLE t(x); {sicht};"));
    // Ausgefuehrt scheitert die Sicht.
    let ausgefuehrt = Command::new("/usr/bin/sqlite3")
        .arg(&datei)
        .arg("SELECT * FROM kaputt")
        .output()
        .expect("/usr/bin/sqlite3 laesst sich nicht starten");
    assert!(
        !ausgefuehrt.status.success(),
        "die Sicht scheitert ausgefuehrt"
    );

    assert_eq!(eintrag_lesen(&datei, "kaputt"), sicht.as_bytes());
}

#[test]
fn ein_tabellenname_mit_anfuehrungszeichen_wird_als_bezeichner_gelesen() {
    let ordner = Pruefordner::neu("sqlite-bezeichner");
    let datei = ordner.unter("namen.db");
    anlegen(
        &datei,
        "CREATE TABLE \"a\"\"b\"(x);
         INSERT INTO \"a\"\"b\" VALUES (7);
         CREATE TABLE a(x);
         INSERT INTO a VALUES (1);
         CREATE TABLE \"x\"\"; SELECT 1; --\"(y);
         INSERT INTO \"x\"\"; SELECT 1; --\" VALUES ('drin');
         CREATE VIEW \"s'q\"\"\" AS SELECT 2;",
    );
    assert_eq!(eintrag_lesen(&datei, "a\"b"), b"x\n7\n");
    assert_eq!(eintrag_lesen(&datei, "a"), b"x\n1\n");
    assert_eq!(eintrag_lesen(&datei, "x\"; SELECT 1; --"), b"y\ndrin\n");
    assert_eq!(
        eintrag_lesen(&datei, "s'q\""),
        b"CREATE VIEW \"s'q\"\"\" AS SELECT 2"
    );
}

// ---------------------------------------------------------------------------
// Schritt 5: die Zusage „nur lesen“ neben einem laufenden Schreiber
// ---------------------------------------------------------------------------

/// Ein `/usr/bin/sqlite3` als Schreiber, der erst zurueckkehrt, wenn er seine
/// Befehle ausgefuehrt hat: nach `sql` druckt er `BEREIT`, und `.bail on`
/// beendet ihn bei einem Fehler, sodass das Warten dann am Ende seiner
/// Ausgabe scheitert statt ewig zu dauern. Danach bleibt er offen, bis die
/// Probe ihn abbricht oder der `Drop` von [`Schreiber`] ihn beendet, auch
/// nach einer Panik.
fn schreiber_bereit(pfad: &Path, sql: &str) -> Schreiber {
    let mut kind = Command::new("/usr/bin/sqlite3")
        .arg(pfad)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("/usr/bin/sqlite3 laesst sich nicht starten");
    let ausgabe = kind.stdout.take().expect("Standardausgabe");
    let mut schreiber = Schreiber {
        eingabe: kind.stdin.take(),
        kind,
    };
    let eingabe = schreiber.eingabe.as_mut().expect("Standardeingabe");
    eingabe
        .write_all(format!(".bail on\n{sql}\n.print BEREIT\n").as_bytes())
        .expect("schreiben");
    eingabe.flush().expect("leeren");
    let mut zeilen = std::io::BufReader::new(ausgabe);
    loop {
        let mut zeile = String::new();
        let gelesen = std::io::BufRead::read_line(&mut zeilen, &mut zeile).expect("lesen");
        assert!(gelesen > 0, "der Schreiber endet, bevor er bereit ist");
        if zeile.trim_end() == "BEREIT" {
            break;
        }
    }
    schreiber
}

impl Schreiber {
    /// Beendet den Schreiber mit `SIGKILL`, ohne dass er schliesst: kein
    /// Checkpoint, `-wal` und `-shm` bleiben stehen (Versuch c der Messung
    /// `messungen/261023-1556-sqlite-nur-lesend-shm.txt`).
    fn abbrechen(&mut self) {
        self.kind.kill().expect("der Schreiber laesst sich beenden");
        self.kind.wait().expect("auf den Schreiber warten");
    }
}

/// Was von einer Datei vor und nach dem Lesen verglichen wird: SHA-256 des
/// Inhalts, Inode und Aenderungszeit in Nanosekunden; `None`, wenn sie nicht
/// steht.
#[derive(Debug, PartialEq, Eq)]
struct Abdruck {
    sha256: String,
    inode: u64,
    geaendert: (i64, i64),
}

fn abdruck(pfad: &Path) -> Option<Abdruck> {
    use std::os::unix::fs::MetadataExt;
    let angaben = match fs::symlink_metadata(pfad) {
        Ok(angaben) => angaben,
        Err(fehler) if fehler.kind() == std::io::ErrorKind::NotFound => return None,
        Err(fehler) => panic!("{}: {fehler}", pfad.display()),
    };
    let ausgabe = Command::new("/usr/bin/shasum")
        .args(["-a", "256"])
        .arg(pfad)
        .output()
        .expect("/usr/bin/shasum laesst sich nicht starten");
    assert!(
        ausgabe.status.success(),
        "shasum scheitert an {}",
        pfad.display()
    );
    let sha256 = String::from_utf8_lossy(&ausgabe.stdout)
        .split_whitespace()
        .next()
        .expect("eine Pruefsumme")
        .to_owned();
    Some(Abdruck {
        sha256,
        inode: angaben.ino(),
        geaendert: (angaben.mtime(), angaben.mtime_nsec()),
    })
}

/// Die Namen eines Ordners, geordnet.
fn ordnernamen(ordner: &Path) -> Vec<String> {
    let mut namen: Vec<String> = fs::read_dir(ordner)
        .unwrap()
        .map(|eintrag| eintrag.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    namen.sort();
    namen
}

/// Faehrt die vier Wege des Kerns an der Datenbank `datei`, jeden mit einem
/// frischen Zugang, damit kein gespeichertes Verzeichnis antwortet:
/// Verzeichnisaufbau, Lesen der Tabelle `t`, Lesen der Sicht `v` und Kopieren
/// der Tabelle `t` in den Ordner `ziel`. Liefert die gelesene TSV der
/// Tabelle.
fn alle_wege_fahren(datei: &Path, ziel: &Path) -> Vec<u8> {
    let verzeichnis = lesen(datei).expect("das Verzeichnis entsteht neben dem Schreiber");
    assert_eq!(namen(&verzeichnis), ["t", "v"]);

    let tabelle = eintrag_lesen(datei, "t");
    assert_eq!(
        eintrag_lesen(datei, "v"),
        b"CREATE VIEW v AS SELECT a FROM t"
    );

    let wurzel = Archivort::wurzel(zugang(datei));
    let bericht = fahren(Fernauftrag::kopieren(
        vec![Ort::Archiv(wurzel.kind("t").expect("ein einzelner Name"))],
        Ort::Lokal(ziel.to_path_buf()),
    ));
    assert!(
        bericht.uebersprungen.is_empty(),
        "kopieren: {:?}",
        gruende(&bericht)
    );
    let kopie = ziel.join("t.tsv");
    assert_eq!(fs::read(&kopie).expect("t.tsv"), tabelle);
    fs::remove_file(&kopie).expect("die Kopie geht wieder weg");
    tabelle
}

/// Eine WAL-Datenbank mit Tabelle `t` und Sicht `v` im Ordner `db` und ein
/// Schreiber, der eine weitere Zeile allein nach `-wal` uebergeben hat und
/// offen bleibt. Das `/usr/bin/sqlite3` des Anlegens schliesst sauber, faehrt
/// dabei den Checkpoint und laesst ein leeres `-wal` und ein `-shm` stehen;
/// der Schreiber schreibt danach mit abgeschaltetem Checkpoint.
fn wal_mit_schreiber(ordner: &Pruefordner) -> (PathBuf, Schreiber) {
    let datei = ordner.ordner("db").join("wal.db");
    anlegen(
        &datei,
        "PRAGMA journal_mode = WAL;
         CREATE TABLE t(a, b);
         INSERT INTO t VALUES (1, 'eins'), (2, 'zwei');
         CREATE VIEW v AS SELECT a FROM t;",
    );
    let schreiber = schreiber_bereit(
        &datei,
        "PRAGMA wal_autocheckpoint = 0;\nINSERT INTO t VALUES (3, 'aus dem wal');",
    );
    let wal = neben(&datei, "-wal");
    assert!(
        fs::metadata(&wal).expect("-wal steht").len() > 0,
        "die Zeile des Schreibers steht in -wal"
    );
    assert!(neben(&datei, "-shm").exists(), "-shm steht");
    (datei, schreiber)
}

#[test]
fn verzeichnis_lesen_und_kopieren_lassen_hauptdatei_und_wal_byte_fuer_byte_stehen() {
    let ordner = Pruefordner::neu("sqlite-nur-lesen-wal");
    let ziel = ordner.ordner("ziel");
    let (datei, mut schreiber) = wal_mit_schreiber(&ordner);
    let wal = neben(&datei, "-wal");
    let db_ordner = datei
        .parent()
        .expect("der Ordner der Datenbank")
        .to_path_buf();
    let namen_vorher = ordnernamen(&db_ordner);
    assert_eq!(namen_vorher, ["wal.db", "wal.db-shm", "wal.db-wal"]);
    let haupt_vorher = abdruck(&datei);
    let wal_vorher = abdruck(&wal);

    // Bei laufendem Schreiber: die Zeile aus -wal ist zu lesen, und weder
    // die Hauptdatei noch -wal aendern sich.
    let tabelle = alle_wege_fahren(&datei, &ziel);
    assert_eq!(tabelle, b"a\tb\n1\teins\n2\tzwei\n3\taus dem wal\n");
    assert_eq!(abdruck(&datei), haupt_vorher, "Hauptdatei, Schreiber offen");
    assert_eq!(abdruck(&wal), wal_vorher, "-wal, Schreiber offen");
    assert_eq!(ordnernamen(&db_ordner), namen_vorher, "Schreiber offen");

    // Nach dem Abbruch des Schreibers ohne Checkpoint ist KRK die einzige
    // Verbindung; ihr Schliessen faehrt keinen Checkpoint und entfernt -wal
    // nicht (Urteil f der Messung).
    schreiber.abbrechen();
    assert_eq!(abdruck(&datei), haupt_vorher, "der Abbruch schreibt nichts");
    assert_eq!(abdruck(&wal), wal_vorher, "der Abbruch schreibt nichts");
    let tabelle = alle_wege_fahren(&datei, &ziel);
    assert_eq!(tabelle, b"a\tb\n1\teins\n2\tzwei\n3\taus dem wal\n");
    assert_eq!(abdruck(&datei), haupt_vorher, "Hauptdatei, Schreiber fort");
    assert_eq!(abdruck(&wal), wal_vorher, "-wal, Schreiber fort");
    assert_eq!(ordnernamen(&db_ordner), namen_vorher, "Schreiber fort");
}

#[test]
fn neben_einer_rollback_datenbank_entsteht_keine_datei() {
    let ordner = Pruefordner::neu("sqlite-nur-lesen-rollback");
    let ziel = ordner.ordner("ziel");
    let datei = ordner.ordner("db").join("rollback.db");
    anlegen(
        &datei,
        "PRAGMA journal_mode = DELETE;
         CREATE TABLE t(a, b);
         INSERT INTO t VALUES (1, 'eins');
         CREATE VIEW v AS SELECT a FROM t;",
    );
    let kopf = fs::read(&datei).unwrap();
    assert_eq!(
        (kopf[18], kopf[19]),
        (1, 1),
        "der Kopf traegt die Rollback-Kennung"
    );
    let _schreiber = schreiber_bereit(
        &datei,
        "BEGIN;\nINSERT INTO t VALUES (2, 'uebergeben');\nCOMMIT;",
    );
    let db_ordner = datei
        .parent()
        .expect("der Ordner der Datenbank")
        .to_path_buf();
    let namen_vorher = ordnernamen(&db_ordner);
    assert_eq!(
        namen_vorher,
        ["rollback.db"],
        "nach dem Uebergeben steht kein -journal"
    );
    let haupt_vorher = abdruck(&datei);

    let tabelle = alle_wege_fahren(&datei, &ziel);
    assert_eq!(tabelle, b"a\tb\n1\teins\n2\tuebergeben\n");

    assert_eq!(
        ordnernamen(&db_ordner),
        namen_vorher,
        "neben der Datenbank entsteht nichts"
    );
    assert_eq!(abdruck(&datei), haupt_vorher, "die Hauptdatei bleibt");
    assert_eq!(ordnernamen(&ziel), Vec::<String>::new());
}

/// Die Stellen, an denen sich zwei gleich lange Bytefolgen unterscheiden.
fn abweichende_stellen(vorher: &[u8], nachher: &[u8]) -> Vec<usize> {
    assert_eq!(vorher.len(), nachher.len(), "-shm behaelt seine Laenge");
    (0..vorher.len())
        .filter(|&stelle| vorher[stelle] != nachher[stelle])
        .collect()
}

/// Die fuenf Lesemarken `aReadMark[0..5]` in `-shm`, je vier Bytes. Nach dem
/// Aufbau des Wal-Index in `wal.c` der Bibliothek stehen am Anfang zwei
/// Kopien des Kopfs `WalIndexHdr` zu je 48 Bytes (0 bis 96), danach
/// `WalCkptInfo`: `nBackfill` (96), die Lesemarken (100 bis 120), `aLock`
/// (120 bis 128, nur fuer Sperren mit `fcntl`, nie beschrieben),
/// `nBackfillAttempted` (128) und ein freies Wort (132); ab 136 folgen die
/// Hashtabellen des Index.
const LESEMARKEN: std::ops::Range<usize> = 100..120;

/// Kopf und Checkpoint-Angaben von `-shm`, alles vor den Hashtabellen
/// (siehe [`LESEMARKEN`]).
const INDEXKOPF: std::ops::Range<usize> = 0..136;

/// Haelt, was sich an `-shm` belegbar halten laesst
/// (`messungen/261023-1556-sqlite-nur-lesend-shm.txt`, Versuche b und c;
/// Entscheid `261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md`):
/// `-shm` darf sich aendern, steht danach aber unter demselben Inode und mit
/// derselben Laenge da, Hauptdatei und `-wal` bleiben Byte fuer Byte, und
/// keine weitere Datei entsteht. Darueber hinaus haelt die Probe, welche
/// Bytes sich aendern duerfen: **solange ein anderes Programm die Datenbank
/// offen haelt, allein die Lesemarken.** Ist KRK nach dem Abbruch des
/// Schreibers die einzige Verbindung, baut die Bibliothek den Index neu auf;
/// dann aendern sich Kopf und Checkpoint-Angaben, die Hashtabellen nicht
/// (gemessen bei einem Rahmen in `-wal`; ob ein laengeres `-wal` beim
/// Neuaufbau auch die Hashtabellen neu schreibt, haelt die Probe nicht). Ob
/// sich ueberhaupt ein Byte aendert, haelt sie nicht: erlaubt ist die
/// Aenderung, verlangt ist sie nicht.
#[test]
fn die_lesemarke_ist_die_einzige_aenderung_an_shm() {
    let ordner = Pruefordner::neu("sqlite-nur-lesen-shm");
    let ziel = ordner.ordner("ziel");
    let (datei, mut schreiber) = wal_mit_schreiber(&ordner);
    let wal = neben(&datei, "-wal");
    let shm = neben(&datei, "-shm");
    let db_ordner = datei
        .parent()
        .expect("der Ordner der Datenbank")
        .to_path_buf();
    let namen_vorher = ordnernamen(&db_ordner);
    let haupt_vorher = abdruck(&datei);
    let wal_vorher = abdruck(&wal);
    let shm_inode = abdruck(&shm).expect("-shm steht").inode;

    // Der Schreiber haelt die Datenbank offen.
    let vorher = fs::read(&shm).unwrap();
    let _ = alle_wege_fahren(&datei, &ziel);
    let nachher = fs::read(&shm).expect("-shm steht nach dem Lesen noch");
    let stellen = abweichende_stellen(&vorher, &nachher);
    assert!(
        stellen.iter().all(|stelle| LESEMARKEN.contains(stelle)),
        "neben dem Schreiber aendern sich allein die Lesemarken, geaendert: {stellen:?}"
    );
    assert_eq!(
        abdruck(&shm).expect("-shm").inode,
        shm_inode,
        "-shm ist nicht ersetzt"
    );
    assert_eq!(abdruck(&datei), haupt_vorher, "Hauptdatei, Schreiber offen");
    assert_eq!(abdruck(&wal), wal_vorher, "-wal, Schreiber offen");
    assert_eq!(
        ordnernamen(&db_ordner),
        namen_vorher,
        "keine weitere Datei, Schreiber offen"
    );

    // Der Schreiber ist ohne Checkpoint fort, KRK die einzige Verbindung.
    schreiber.abbrechen();
    let vorher = fs::read(&shm).unwrap();
    let _ = alle_wege_fahren(&datei, &ziel);
    let nachher = fs::read(&shm).expect("-shm steht nach dem Lesen noch");
    let stellen = abweichende_stellen(&vorher, &nachher);
    assert!(
        stellen.iter().all(|stelle| INDEXKOPF.contains(stelle)),
        "der Neuaufbau aendert allein Kopf und Checkpoint-Angaben, geaendert: {stellen:?}"
    );
    assert_eq!(
        abdruck(&shm).expect("-shm").inode,
        shm_inode,
        "-shm ist nicht ersetzt"
    );
    assert_eq!(abdruck(&datei), haupt_vorher, "Hauptdatei, Schreiber fort");
    assert_eq!(abdruck(&wal), wal_vorher, "-wal, Schreiber fort");
    assert_eq!(
        ordnernamen(&db_ordner),
        namen_vorher,
        "keine weitere Datei, Schreiber fort"
    );
}
