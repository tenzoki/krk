//! Abnahme der Bindung an `libsqlite3` in `krk_core::verzeichnis::sys`,
//! Schritt 2 des Plans `261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md`
//! im Arbeitspaket `261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen`.
//!
//! Jede Probe legt ihre Datenbank selbst im Pruefordner an, mit
//! `/usr/bin/sqlite3`. **Das Kommandozeilenprogramm traegt SQLite eingebaut
//! und laedt `/usr/lib/libsqlite3.dylib` nicht**
//! (`messungen/261023-1556-sqlite-nur-lesend-shm.txt`, Versuch a); es dient
//! hier allein als Schreiber der Pruefdaten und belegt nichts ueber die
//! Bibliothek. Was die Bindung tut, belegt allein der Leser ueber
//! [`Datenbank`]. Keine Probe liest eine Datenbank ausserhalb des
//! Pruefordners.

mod gemeinsam;

use std::ffi::OsStr;
use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use gemeinsam::Pruefordner;
use krk_core::verzeichnis::sys::{
    Datenbank, SQLITE_INTERRUPT, SQLITE_NOTADB, SQLITE_READONLY, Sqlitefehler, Wert, sqlite_uri,
};

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

/// Fuehrt `sql` ueber eine nur lesende Verbindung aus und liefert alle Zeilen
/// oder den ersten Fehler, gleich ob er beim Oeffnen, beim Vorbereiten oder
/// beim Schritt kommt.
fn alle_zeilen(
    pfad: &Path,
    sql: &str,
    abbruch: &AtomicBool,
) -> Result<Vec<Vec<Wert>>, Sqlitefehler> {
    let uri = sqlite_uri(pfad, false).expect("Pfad ohne Nullbyte");
    let datenbank = Datenbank::oeffnen(&uri, abbruch)?;
    let mut abfrage = datenbank.vorbereiten(sql)?;
    let mut zeilen = Vec::new();
    while let Some(zeile) = abfrage.naechste_zeile()? {
        zeilen.push(zeile);
    }
    Ok(zeilen)
}

/// Die Bytes jeder Datei im Ordner, nach Namen.
fn ordnerstand(ordner: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut stand: Vec<_> = fs::read_dir(ordner)
        .expect("Pruefordner ist lesbar")
        .map(|eintrag| {
            let pfad = eintrag.expect("Eintrag ist lesbar").path();
            let bytes = fs::read(&pfad).expect("Datei ist lesbar");
            (pfad, bytes)
        })
        .collect();
    stand.sort();
    stand
}

#[test]
fn eine_datenbank_wird_nur_lesend_geoeffnet_und_liest_jede_wertart() {
    let ordner = Pruefordner::neu("sqlite-wertarten");
    let pfad = ordner.unter("werte.db");
    anlegen(
        &pfad,
        "CREATE TABLE t(leer, ganz, komma, text, roh); \
         INSERT INTO t VALUES (NULL, 42, 3.5, 'Grüße', x'00ff10');",
    );
    let vorher = ordnerstand(ordner.pfad());

    let abbruch = AtomicBool::new(false);
    let uri = sqlite_uri(&pfad, false).expect("Pfad ohne Nullbyte");
    let datenbank = Datenbank::oeffnen(&uri, &abbruch).expect("oeffnet");
    let mut abfrage = datenbank
        .vorbereiten("SELECT * FROM t")
        .expect("bereitet vor");
    assert_eq!(
        abfrage.spaltennamen().expect("Spaltennamen"),
        ["leer", "ganz", "komma", "text", "roh"]
    );
    let zeile = abfrage
        .naechste_zeile()
        .expect("liest")
        .expect("eine Zeile");
    assert_eq!(
        zeile,
        vec![
            Wert::Null,
            Wert::Ganzzahl("42".to_owned()),
            Wert::Kommazahl("3.5".to_owned()),
            Wert::Text("Grüße".to_owned()),
            Wert::Blob(3),
        ]
    );
    assert_eq!(abfrage.naechste_zeile().expect("liest"), None);
    drop(abfrage);
    drop(datenbank);

    assert_eq!(
        ordnerstand(ordner.pfad()),
        vorher,
        "das Lesen hat im Ordner der Datenbank etwas angelegt oder geaendert"
    );
}

#[test]
fn ein_schreibversuch_ueber_eine_nur_lesende_verbindung_scheitert() {
    let ordner = Pruefordner::neu("sqlite-schreibversuch");
    let pfad = ordner.unter("t.db");
    anlegen(&pfad, "CREATE TABLE t(a); INSERT INTO t VALUES (1);");
    let vorher = ordnerstand(ordner.pfad());

    let abbruch = AtomicBool::new(false);
    let fehler = alle_zeilen(&pfad, "INSERT INTO t VALUES (2)", &abbruch)
        .expect_err("ein Schreibversuch muss scheitern");
    assert_eq!(fehler.grundcode(), SQLITE_READONLY, "{fehler}");

    let zeilen = alle_zeilen(&pfad, "SELECT count(*) FROM t", &abbruch).expect("liest");
    assert_eq!(zeilen, vec![vec![Wert::Ganzzahl("1".to_owned())]]);
    assert_eq!(ordnerstand(ordner.pfad()), vorher);
}

#[test]
fn eine_gesetzte_abbruchmarke_bricht_eine_laufende_anweisung_ab() {
    let ordner = Pruefordner::neu("sqlite-abbruch");
    let pfad = ordner.unter("z.db");
    anlegen(
        &pfad,
        "CREATE TABLE z(n); \
         WITH RECURSIVE r(n) AS (SELECT 1 UNION ALL SELECT n + 1 FROM r WHERE n < 100000) \
         INSERT INTO z SELECT n FROM r;",
    );
    // Die Bedingung haelt die Bibliothek davon ab, die Zeilen mit einem
    // einzigen Befehl zu zaehlen; sie geht jede Zeile durch.
    let sql = "SELECT count(*) FROM z WHERE n >= 0";

    let offen = AtomicBool::new(false);
    assert_eq!(
        alle_zeilen(&pfad, sql, &offen).expect("liest ohne Marke"),
        vec![vec![Wert::Ganzzahl("100000".to_owned())]]
    );

    let abbruch = AtomicBool::new(false);
    let uri = sqlite_uri(&pfad, false).expect("Pfad ohne Nullbyte");
    let datenbank = Datenbank::oeffnen(&uri, &abbruch).expect("oeffnet");
    let mut abfrage = datenbank.vorbereiten(sql).expect("bereitet vor");
    abbruch.store(true, Ordering::Relaxed);
    let fehler = abfrage
        .naechste_zeile()
        .expect_err("die gesetzte Marke muss die Anweisung abbrechen");
    assert_eq!(fehler.grundcode(), SQLITE_INTERRUPT, "{fehler}");
}

#[test]
fn die_uri_traegt_jeden_pfad_unveraendert() {
    let ordner = Pruefordner::neu("sqlite-uri");
    let namen: [&[u8]; 6] = [
        b"mit leerzeichen",
        b"prozent%41",
        b"frage?mode=rw",
        b"raute#teil",
        "Grüße".as_bytes(),
        b"kein-utf8-\xff",
    ];
    let mut geoeffnet = 0;
    for (nummer, name) in namen.iter().enumerate() {
        let unterordner = ordner.pfad().join(OsStr::from_bytes(name));
        match fs::create_dir(&unterordner) {
            Ok(()) => {}
            // APFS weist einen Namen ab, der kein UTF-8 ist (`EILSEQ`). Fuer
            // dieses Byte prueft die Probe deshalb allein die Form der URI
            // unten; eine Datei mit diesem Namen kann es auf dem Geraet nicht
            // geben.
            Err(fehler) if fehler.raw_os_error() == Some(92) => continue,
            Err(fehler) => panic!("{}: {fehler}", unterordner.display()),
        }
        let pfad = unterordner.join(OsStr::from_bytes(name));
        anlegen(
            &pfad,
            &format!("CREATE TABLE t(n); INSERT INTO t VALUES ({nummer});"),
        );
        let abbruch = AtomicBool::new(false);
        let zeilen = alle_zeilen(&pfad, "SELECT n FROM t", &abbruch)
            .unwrap_or_else(|fehler| panic!("{}: {fehler}", pfad.display()));
        assert_eq!(
            zeilen,
            vec![vec![Wert::Ganzzahl(nummer.to_string())]],
            "{} oeffnet eine andere Datei",
            pfad.display()
        );
        geoeffnet += 1;
    }
    assert!(geoeffnet >= 5, "nur {geoeffnet} Pfade geoeffnet");
}

#[test]
fn die_uri_bringt_jedes_byte_ausser_den_unreservierten_in_prozentform() {
    let pfad = Path::new(OsStr::from_bytes(b"/a b/%?#/Gr\xc3\xbc\xff-._~"));
    assert_eq!(
        sqlite_uri(pfad, false)
            .expect("Pfad ohne Nullbyte")
            .to_str()
            .expect("ASCII"),
        "file:///a%20b/%25%3F%23/Gr%C3%BC%FF-._~?mode=ro"
    );
    assert_eq!(
        sqlite_uri(Path::new("rel/x.db"), true)
            .expect("Pfad ohne Nullbyte")
            .to_str()
            .expect("ASCII"),
        "file:rel/x.db?mode=ro&immutable=1"
    );
    // Ein Pfad, der mit zwei Schraegstrichen beginnt, wird nicht zum
    // Rechnernamen.
    assert_eq!(
        sqlite_uri(Path::new("//srv/x.db"), false)
            .expect("Pfad ohne Nullbyte")
            .to_str()
            .expect("ASCII"),
        "file:////srv/x.db?mode=ro"
    );
}

/// Ein Pfad mit einem Nullbyte wird abgewiesen und nicht als `%00` kodiert:
/// die Bibliothek laese ihn sonst bis vor `%00` und oeffnete `x.db`
/// (Issue `261024-2116-sqlite-uri-schreibt-ein-nullbyte-als-prozent-00-und-sqlite-kuerzt-dort-den-pfad.md`).
#[test]
fn ein_pfad_mit_nullbyte_wird_abgewiesen_statt_gekuerzt_geoeffnet() {
    let ordner = Pruefordner::neu("sqlite-nullbyte");
    let pfad = ordner.unter("x.db");
    anlegen(&pfad, "CREATE TABLE t(n); INSERT INTO t VALUES (7);");
    let mut mit_null = pfad.as_os_str().as_bytes().to_vec();
    mit_null.extend_from_slice(b"\0rest");
    let mit_null = Path::new(OsStr::from_bytes(&mit_null));

    for unveraenderlich in [false, true] {
        let fehler = sqlite_uri(mit_null, unveraenderlich)
            .expect_err("ein Pfad mit Nullbyte muss abgewiesen werden");
        assert_eq!(fehler.kind(), std::io::ErrorKind::InvalidInput, "{fehler}");
    }
    // Derselbe Pfad ohne den Rest oeffnet `x.db`: abgewiesen wird das
    // Nullbyte und nicht die Datei.
    let abbruch = AtomicBool::new(false);
    assert_eq!(
        alle_zeilen(&pfad, "SELECT n FROM t", &abbruch).expect("liest"),
        vec![vec![Wert::Ganzzahl("7".to_owned())]]
    );
}

#[test]
fn eine_datei_die_keine_datenbank_ist_meldet_notadb() {
    let ordner = Pruefordner::neu("sqlite-notadb");
    let pfad = ordner.unter("text.db");
    fs::write(&pfad, "Dies ist keine Datenbank.\n".repeat(200)).expect("schreibt");

    let abbruch = AtomicBool::new(false);
    let fehler = alle_zeilen(&pfad, "SELECT count(*) FROM sqlite_master", &abbruch)
        .expect_err("eine Textdatei ist keine Datenbank");
    assert_eq!(fehler.grundcode(), SQLITE_NOTADB, "{fehler}");
}
