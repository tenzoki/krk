//! Abnahme von `Archivformat::Sqlite` und dem Inhaltsverzeichnis einer
//! Datenbank, Schritt 3 des Plans
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
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use gemeinsam::Pruefordner;
use krk_core::archiv::umbau::Umbaufehler;
use krk_core::archiv::{
    Archivformat, Archivzugang, Fundstelle, Inhaltsverzeichnis, Innenpfad, Lesbarkeit,
    Verzeichnisfehler,
};
use krk_core::fern::faden;
use krk_core::operation::{Bericht, Fernauftrag, Meldung, OhnePapierkorb, starten};
use krk_core::ort::{Archivort, Ort, nicht_verfuegbar_text};
use krk_core::sprache::{Text, text};
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
