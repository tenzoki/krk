//! Abnahme von Kopieren und Verschieben mit einer fernen Seite (Schritt 5 des
//! Plans `261016-1956-plan-sftp-ordner-im-dateifenster-wie-lokal.md`,
//! Entscheidung 9).
//!
//! **Kein Netz, kein `ssh`.** Jeder ferne Ort steht auf einem Zugang, der
//! `/usr/libexec/sftp-server` unmittelbar als Kindprozess spricht, wie in
//! `tests/fern.rs`; der Server arbeitet auf diesem Geraet, also liegt jeder
//! ferne Pfad zugleich im [`Pruefordner`] und laesst sich von der Probe mit
//! `std::fs` nachsehen. Zwei Zugaenge mit verschiedener Kennung stehen fuer
//! zwei Server.
//!
//! Jeder Vorgang laeuft ueber `operation::starten` und damit ueber denselben
//! Faden, denselben Kanal und dieselben Meldungen wie im Betrieb.

mod gemeinsam;

use std::fs::{self, File, FileTimes};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use gemeinsam::{Pruefordner, betriebscode, quelldateien};
use krk_core::fern::{Fernpfad, Fernzugang, Kanalplan, Serverkennung, faden};
use krk_core::operation::{
    Abschluss, Anlegetyp, Bericht, Fernauftrag, Konfliktantwort, Konfliktentscheid, Konfliktregel,
    Meldung, OhnePapierkorb, fehlerart, starten,
};
use krk_core::ort::{FernerOrt, Ort};

/// Der Server des Systems, ohne `ssh` davor.
const SFTP_SERVER: &str = "/usr/libexec/sftp-server";

/// Ein Zugang zum Server des Systems unter der Kennung `kai@rechner`.
fn zugang(rechner: &str) -> Arc<Fernzugang> {
    let kennung = Serverkennung::neu(rechner, None, Some("kai"));
    faden("krk-probe-fern".to_owned(), move |pfand| {
        Fernzugang::aufbauen(pfand, kennung, "Probe", &Kanalplan::nur(SFTP_SERVER))
    })
    .expect("kein Faden fuer die Probe")
    .join()
    .expect("der Aufbau ist abgestuerzt")
    .expect("sftp-server laesst sich nicht ansprechen")
}

/// Der Eintrag oder Ordner `pfad` auf dem Zugang, als ferner Ort.
fn fern(zugang: &Arc<Fernzugang>, pfad: &Path) -> Ort {
    Ort::Fern(FernerOrt {
        zugang: Arc::clone(zugang),
        pfad: Fernpfad::aus_text(pfad.to_str().expect("Pruefpfade sind UTF-8"))
            .expect("Pruefpfade sind absolut"),
    })
}

fn lokal(pfad: &Path) -> Ort {
    Ort::Lokal(pfad.to_path_buf())
}

/// Was ein Lauf gemeldet hat: der Bericht und jede Konfliktfrage als Paar aus
/// Quelle und Ziel.
struct Ausgang {
    bericht: Bericht,
    konflikte: Vec<(PathBuf, PathBuf)>,
}

/// Faehrt einen Auftrag bis zu seinem Bericht und beantwortet jede
/// Konfliktfrage mit `antwort`.
fn fahren(auftrag: Fernauftrag, antwort: Konfliktantwort) -> Ausgang {
    let lauf = starten(auftrag, Arc::new(OhnePapierkorb));
    let mut konflikte = Vec::new();
    let bericht = loop {
        match lauf
            .meldungen()
            .recv_timeout(Duration::from_secs(60))
            .expect("der Lauf meldet sich nicht")
        {
            Meldung::Fortschritt(_) | Meldung::Uebersprungen(_) => {}
            Meldung::Konflikt {
                quelle,
                ziel,
                antwort: weg,
            } => {
                konflikte.push((quelle, ziel));
                weg.send(Konfliktentscheid::einmal(antwort.clone()))
                    .expect("der Lauf wartet nicht mehr");
            }
            Meldung::Fertig(bericht) => break bericht,
        }
    };
    lauf.warten();
    Ausgang { bericht, konflikte }
}

/// Wie [`fahren`], und sobald ein Fortschritt mit uebertragenen Bytes kommt,
/// wird abgebrochen.
fn mitten_abbrechen(auftrag: Fernauftrag) -> Bericht {
    let lauf = starten(auftrag, Arc::new(OhnePapierkorb));
    let bericht = loop {
        match lauf
            .meldungen()
            .recv_timeout(Duration::from_secs(60))
            .expect("der Lauf meldet sich nicht")
        {
            Meldung::Fortschritt(stand) if stand.bytes > 0 => lauf.abbrechen(),
            Meldung::Fortschritt(_) | Meldung::Uebersprungen(_) => {}
            Meldung::Konflikt { .. } => panic!("unerwartete Konfliktfrage"),
            Meldung::Fertig(bericht) => break bericht,
        }
    };
    lauf.warten();
    bericht
}

/// Ein Inhalt, der sich ueber Blockgrenzen nicht wiederholt.
fn muster(laenge: usize) -> Vec<u8> {
    (0..laenge).map(|stelle| (stelle % 251) as u8).collect()
}

fn ohne_uebersprungene(bericht: &Bericht) {
    assert!(
        bericht.uebersprungen.is_empty(),
        "uebersprungen: {:?}",
        bericht.uebersprungen
    );
}

/// Eine Datei ueber mehrere Bloecke, deren Laenge kein Vielfaches eines
/// Blocks ist, kommt von hier auf den Server und von dort zurueck, Byte fuer
/// Byte, mit ihren Rechten und ihrer Aenderungszeit.
#[test]
fn eine_datei_kommt_lokal_nach_fern_und_zurueck_byte_fuer_byte() {
    let ordner = Pruefordner::neu("fern-hin-und-zurueck");
    let quellordner = ordner.ordner("quelle");
    let fernordner = ordner.ordner("server");
    let rueckordner = ordner.ordner("zurueck");
    let inhalt = muster(300_001);
    let quelle = quellordner.join("daten.bin");
    fs::write(&quelle, &inhalt).expect("die Quelle laesst sich nicht schreiben");
    let zeit = UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    File::options()
        .write(true)
        .open(&quelle)
        .and_then(|datei| datei.set_times(FileTimes::new().set_modified(zeit)))
        .expect("die Zeit laesst sich nicht setzen");
    fs::set_permissions(&quelle, fs::Permissions::from_mode(0o640)).expect("Rechte");
    let server = zugang("server.test");

    let hin = fahren(
        Fernauftrag::kopieren(vec![lokal(&quelle)], fern(&server, &fernordner)),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&hin.bericht);
    assert_eq!(hin.bericht.abschluss, Abschluss::Fertig);
    assert_eq!(hin.bericht.bytes, inhalt.len() as u64);

    let zurueck = fahren(
        Fernauftrag::kopieren(
            vec![fern(&server, &fernordner.join("daten.bin"))],
            lokal(&rueckordner),
        ),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&zurueck.bericht);

    for ziel in [fernordner.join("daten.bin"), rueckordner.join("daten.bin")] {
        assert_eq!(fs::read(&ziel).expect("das Ziel fehlt"), inhalt, "{ziel:?}");
        let angaben = fs::metadata(&ziel).expect("Angaben");
        assert_eq!(angaben.mode() & 0o7777, 0o640, "Rechte an {ziel:?}");
        assert_eq!(angaben.mtime(), 1_700_000_000, "Aenderungszeit an {ziel:?}");
    }
}

/// Ein Ordnerbaum mit Unterordner, leerem Ordner und Dateien kommt von einem
/// Server auf einen zweiten vollstaendig an.
#[test]
fn ein_ordnerbaum_kommt_fern_nach_fern_vollstaendig_an() {
    let ordner = Pruefordner::neu("fern-baum");
    let baum = ordner.ordner("eins/baum");
    fs::write(baum.join("a.txt"), b"a").expect("a.txt");
    fs::create_dir_all(baum.join("unter/leer")).expect("Unterordner");
    fs::write(baum.join("unter/b.bin"), muster(70_000)).expect("b.bin");
    let ziel = ordner.ordner("zwei");
    let eins = zugang("eins.test");
    let zwei = zugang("zwei.test");

    let ausgang = fahren(
        Fernauftrag::kopieren(vec![fern(&eins, &baum)], fern(&zwei, &ziel)),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig);
    assert_eq!(fs::read(ziel.join("baum/a.txt")).expect("a.txt"), b"a");
    assert_eq!(
        fs::read(ziel.join("baum/unter/b.bin")).expect("b.bin"),
        muster(70_000)
    );
    assert!(
        ziel.join("baum/unter/leer").is_dir(),
        "der leere Ordner fehlt"
    );
    assert!(
        baum.join("a.txt").is_file(),
        "das Kopieren hat die Quelle angefasst"
    );
}

/// Ein Abbruch mitten in einer grossen Datei laesst in keiner der drei
/// Richtungen ein halbes Ziel stehen.
#[test]
fn ein_abbruch_mitten_in_der_datei_laesst_kein_halbes_ziel_stehen() {
    let ordner = Pruefordner::neu("fern-abbruch");
    let quellordner = ordner.ordner("quelle");
    let gross = ordner.luecke("quelle/gross.bin", 256 * 1024 * 1024);
    let server = zugang("server.test");
    let anderer = zugang("anderer.test");

    let richtungen = [
        (
            "lokal nach fern",
            lokal(&gross),
            fern(&server, &ordner.ordner("eins")),
        ),
        (
            "fern nach lokal",
            fern(&server, &gross),
            lokal(&ordner.ordner("zwei")),
        ),
        (
            "fern nach fern",
            fern(&server, &gross),
            fern(&anderer, &ordner.ordner("drei")),
        ),
    ];
    for (richtung, quelle, ziel) in richtungen {
        let zielordner = match &ziel {
            Ort::Lokal(pfad) => pfad.clone(),
            Ort::Fern(fern) => PathBuf::from(fern.pfad.als_text()),
        };
        let bericht = mitten_abbrechen(Fernauftrag::kopieren(vec![quelle], ziel));
        assert_eq!(bericht.abschluss, Abschluss::Abgebrochen, "{richtung}");
        assert_eq!(bericht.eintraege, 0, "{richtung}");
        assert!(
            !zielordner.join("gross.bin").exists(),
            "{richtung}: das halbe Ziel steht noch"
        );
        assert!(
            bericht.uebersprungen.is_empty(),
            "{richtung}: {:?}",
            bericht.uebersprungen
        );
    }
    assert!(quellordner.join("gross.bin").is_file(), "die Quelle fehlt");
}

/// Ein vorhandenes Ziel stellt die Konfliktfrage mit Quelle und Ziel, und
/// „Ueberschreiben“ ersetzt es; unter „automatisch umbenennen“ entsteht der
/// freie Name neben dem vorhandenen, gefragt auf dem Server.
#[test]
fn ein_vorhandenes_ziel_stellt_die_konfliktfrage_und_ueberschreiben_ersetzt_es() {
    let ordner = Pruefordner::neu("fern-konflikt");
    let _ = ordner.ordner("quelle");
    let quelle = ordner.datei("quelle/a.txt", b"neu");
    let fernordner = ordner.ordner("server");
    fs::write(fernordner.join("a.txt"), b"alt").expect("das Ziel laesst sich nicht anlegen");
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::kopieren(vec![lokal(&quelle)], fern(&server, &fernordner)),
        Konfliktantwort::Ueberschreiben,
    );

    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(
        ausgang.konflikte,
        vec![(
            quelle.clone(),
            PathBuf::from(format!(
                "kai@server.test:{}",
                fernordner.join("a.txt").display()
            ))
        )]
    );
    assert_eq!(fs::read(fernordner.join("a.txt")).expect("a.txt"), b"neu");

    let umbenannt = fahren(
        Fernauftrag::kopieren(vec![lokal(&quelle)], fern(&server, &fernordner))
            .mit_konfliktregel(Konfliktregel::AutomatischUmbenennen),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&umbenannt.bericht);
    assert!(umbenannt.konflikte.is_empty(), "die Regel fragt nicht");
    assert_eq!(
        fs::read(fernordner.join("a Kopie.txt")).expect("a Kopie.txt"),
        b"neu"
    );
}

/// Ob im Ordner ein Zwischenname des Ueberschreibens liegen geblieben ist.
fn zwischennamen_in(ordner: &Path) -> Vec<String> {
    fs::read_dir(ordner)
        .expect("der Ordner laesst sich nicht lesen")
        .filter_map(|eintrag| eintrag.ok())
        .map(|eintrag| eintrag.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with(".krk-ersatz-"))
        .collect()
}

/// Benennen Quelle und Ziel auf dem Server ueber eine Verknuepfung denselben
/// Eintrag, ueberschreibt weder das Kopieren noch das Verschieben die Quelle
/// (Befund `261017-0004-ueberschreiben-zerstoert-die-quelle-wenn-ziel-und-quelle-…`).
///
/// Im [`Pruefordner`] stehen `echt/x` und `alias -> echt`; kopiert und
/// verschoben wird `echt/x` nach `alias/`, und jede Konfliktfrage bekommt
/// „Ueberschreiben“. Bis zur Behebung entschied der Fernzweig am Pfadtext,
/// stellte die Konfliktfrage, und „Ueberschreiben“ entfernte die eine
/// gemeinsame Datei: das Kopieren hinterliess eine leere, das Verschieben
/// keine. Daneben ein Zielordner, der ueber eine Verknuepfung in der Quelle
/// liegt: kein Kopieren in sich selbst.
#[test]
fn ueberschreiben_ueber_eine_verknuepfung_laesst_die_quelle_stehen() {
    let ordner = Pruefordner::neu("fern-verknuepfte-quelle");
    let _ = ordner.ordner("echt");
    let quelle = ordner.datei("echt/x", b"inhalt");
    let alias = ordner.pfad().join("alias");
    std::os::unix::fs::symlink("echt", &alias).expect("die Verknuepfung laesst sich nicht anlegen");
    let server = zugang("server.test");

    for (art, auftrag) in [
        (
            "kopieren",
            Fernauftrag::kopieren(vec![fern(&server, &quelle)], fern(&server, &alias)),
        ),
        (
            "verschieben",
            Fernauftrag::verschieben(vec![fern(&server, &quelle)], fern(&server, &alias)),
        ),
    ] {
        let ausgang = fahren(auftrag, Konfliktantwort::Ueberschreiben);
        assert_eq!(
            fs::read(&quelle).ok().as_deref(),
            Some(&b"inhalt"[..]),
            "{art}: die Quelle ist zerstoert"
        );
        assert!(
            ausgang.konflikte.is_empty(),
            "{art}: {:?}",
            ausgang.konflikte
        );
        assert_eq!(
            ausgang.bericht.uebersprungen.len(),
            1,
            "{art}: {:?}",
            ausgang.bericht
        );
        assert!(
            zwischennamen_in(&ordner.pfad().join("echt")).is_empty(),
            "{art}"
        );
    }

    // Ein Ordner in einen Ordner, der ueber eine Verknuepfung in ihm liegt;
    // am Pfadtext liegt `aussen` nicht in `baum`.
    let baum = ordner.ordner("baum");
    fs::write(baum.join("a.txt"), b"a").expect("a.txt");
    let _ = ordner.ordner("baum/unten");
    let aussen = ordner.pfad().join("aussen");
    std::os::unix::fs::symlink("baum/unten", &aussen).expect("Verknuepfung");
    let ausgang = fahren(
        Fernauftrag::kopieren(vec![fern(&server, &baum)], fern(&server, &aussen)),
        Konfliktantwort::Abbrechen,
    );
    assert_eq!(
        ausgang.bericht.uebersprungen.len(),
        1,
        "{:?}",
        ausgang.bericht
    );
    assert!(
        !baum.join("unten/baum").exists(),
        "der Ordner wurde in sich selbst kopiert"
    );
}

/// „Ueberschreiben“ entfernt ein Ziel nie, ohne dass sein Ersatz vollstaendig
/// dasteht (Befund `261017-0004-jedes-scheitern-beim-fernen-anlegen-gilt-als-konflikt-…`).
///
/// Die Quelle laesst sich nicht lesen, eine Datei mit den Rechten `000` und
/// ein Ordner mit einer solchen Datei darin, und am Ziel auf dem Server steht
/// je eine Datei gleichen Namens. Bis zur Behebung entfernte „Ueberschreiben“
/// das Ziel zuerst; danach scheiterte das Lesen, und am Ende war das Ziel fort
/// und nichts an seiner Stelle. Jetzt entsteht der Ersatz unter einem
/// Zwischennamen, faellt weg, und das Ziel steht unveraendert. Daneben gelingt
/// dasselbe Ueberschreiben mit lesbarer Quelle, auch als Verschieben auf
/// demselben Server, und kein Zwischenname bleibt liegen.
#[test]
fn ueberschreiben_entfernt_das_ziel_erst_wenn_der_ersatz_vollstaendig_dasteht() {
    use std::os::unix::fs::PermissionsExt as _;

    let ordner = Pruefordner::neu("fern-ersatz");
    let fernordner = ordner.ordner("server");
    let _ = ordner.ordner("quelle");
    let verschlossen = ordner.datei("quelle/a.txt", b"neu");
    let teilbaum = ordner.ordner("quelle/d");
    fs::write(teilbaum.join("gut.txt"), b"gut").expect("gut.txt");
    let zu = ordner.datei("quelle/d/zu.txt", b"zu");
    for datei in [&verschlossen, &zu] {
        fs::set_permissions(datei, fs::Permissions::from_mode(0o000)).expect("Rechte");
    }
    fs::write(fernordner.join("a.txt"), b"alt").expect("a.txt");
    fs::write(fernordner.join("d"), b"alte datei").expect("d");
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::kopieren(
            vec![lokal(&verschlossen), lokal(&teilbaum)],
            fern(&server, &fernordner),
        ),
        Konfliktantwort::Ueberschreiben,
    );
    for datei in [&verschlossen, &zu] {
        fs::set_permissions(datei, fs::Permissions::from_mode(0o644)).expect("Rechte");
    }
    assert_eq!(ausgang.konflikte.len(), 2, "{:?}", ausgang.konflikte);
    assert_eq!(
        fs::read(fernordner.join("a.txt")).ok().as_deref(),
        Some(&b"alt"[..]),
        "die Datei am Ziel ist ohne Ersatz verschwunden"
    );
    assert_eq!(
        fs::read(fernordner.join("d")).ok().as_deref(),
        Some(&b"alte datei"[..]),
        "die Datei am Ziel ist einem halben Ordner gewichen"
    );
    assert!(!ausgang.bericht.uebersprungen.is_empty());
    assert!(
        zwischennamen_in(&fernordner).is_empty(),
        "{:?}",
        zwischennamen_in(&fernordner)
    );

    // Mit lesbarer Quelle wird ersetzt: Datei und Ordner ueber eine Datei.
    let ausgang = fahren(
        Fernauftrag::kopieren(
            vec![lokal(&verschlossen), lokal(&teilbaum)],
            fern(&server, &fernordner),
        ),
        Konfliktantwort::Ueberschreiben,
    );
    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(fs::read(fernordner.join("a.txt")).expect("a.txt"), b"neu");
    assert_eq!(
        fs::read(fernordner.join("d/zu.txt")).expect("d/zu.txt"),
        b"zu"
    );
    assert!(zwischennamen_in(&fernordner).is_empty());

    // Verschieben auf demselben Server ueber ein vorhandenes Ziel.
    let hier = ordner.ordner("server/hier");
    let wandernd = ordner.datei("server/hier/a.txt", b"wandernd");
    let ausgang = fahren(
        Fernauftrag::verschieben(vec![fern(&server, &wandernd)], fern(&server, &fernordner)),
        Konfliktantwort::Ueberschreiben,
    );
    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(ausgang.konflikte.len(), 1);
    assert!(!wandernd.exists(), "die Quelle steht noch");
    assert_eq!(
        fs::read(fernordner.join("a.txt")).expect("a.txt"),
        b"wandernd"
    );
    assert!(zwischennamen_in(&fernordner).is_empty());
    assert!(zwischennamen_in(&hier).is_empty());
}

/// Ein Ordner auf einen gleichnamigen Ordner ist kein Konflikt: der Inhalt
/// kommt hinzu, und gefragt wird nichts.
#[test]
fn ordner_auf_ordner_fuehrt_zusammen_ohne_frage() {
    let ordner = Pruefordner::neu("fern-zusammen");
    let quelle = ordner.ordner("quelle/d");
    fs::write(quelle.join("x.txt"), b"x").expect("x.txt");
    let fernordner = ordner.ordner("server");
    fs::create_dir(fernordner.join("d")).expect("d");
    fs::write(fernordner.join("d/y.txt"), b"y").expect("y.txt");
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::kopieren(vec![lokal(&quelle)], fern(&server, &fernordner)),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    assert!(ausgang.konflikte.is_empty(), "{:?}", ausgang.konflikte);
    assert_eq!(fs::read(fernordner.join("d/x.txt")).expect("x.txt"), b"x");
    assert_eq!(fs::read(fernordner.join("d/y.txt")).expect("y.txt"), b"y");
}

/// Auf demselben Server ist das Verschieben ein `RENAME`: der Eintrag behaelt
/// seinen Inode. Ueber die Grenze wird kopiert, und die Quelle faellt allein,
/// wenn nichts uebersprungen wurde.
#[test]
fn verschieben_auf_demselben_server_benennt_um_und_ueber_die_grenze_entfernt_es_die_quelle_nur_ohne_uebersprungene()
 {
    let ordner = Pruefordner::neu("fern-verschieben");
    let server = zugang("server.test");

    // Derselbe Server.
    let _ = ordner.ordner("hier");
    let datei = ordner.datei("hier/f.txt", b"f");
    let inode = fs::metadata(&datei).expect("f.txt").ino();
    let dort = ordner.ordner("dort");
    let ausgang = fahren(
        Fernauftrag::verschieben(vec![fern(&server, &datei)], fern(&server, &dort)),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&ausgang.bericht);
    assert!(!datei.exists(), "die Quelle steht noch");
    assert_eq!(
        fs::metadata(dort.join("f.txt"))
            .expect("f.txt am Ziel")
            .ino(),
        inode,
        "umbenannt und nicht kopiert"
    );

    // Ueber die Grenze, ohne Ausfall.
    let baum = ordner.ordner("server/baum");
    fs::write(baum.join("a.txt"), b"a").expect("a.txt");
    let lokalziel = ordner.ordner("lokal");
    let ausgang = fahren(
        Fernauftrag::verschieben(vec![fern(&server, &baum)], lokal(&lokalziel)),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&ausgang.bericht);
    assert!(!baum.exists(), "die Quelle steht noch");
    assert_eq!(fs::read(lokalziel.join("baum/a.txt")).expect("a.txt"), b"a");

    // Ueber die Grenze, mit einem uebersprungenen Kind.
    let zweiter = ordner.ordner("server/zweiter");
    fs::write(zweiter.join("a.txt"), b"a").expect("a.txt");
    fs::write(zweiter.join("b.txt"), b"b").expect("b.txt");
    fs::create_dir_all(lokalziel.join("zweiter/b.txt")).expect("der Stoerer");
    let ausgang = fahren(
        Fernauftrag::verschieben(vec![fern(&server, &zweiter)], lokal(&lokalziel))
            .mit_konfliktregel(Konfliktregel::Ueberspringen),
        Konfliktantwort::Abbrechen,
    );
    assert!(zweiter.join("a.txt").is_file(), "a.txt fehlt in der Quelle");
    assert!(zweiter.join("b.txt").is_file(), "b.txt fehlt in der Quelle");
    let pfade: Vec<&Path> = ausgang
        .bericht
        .uebersprungen
        .iter()
        .map(|eintrag| eintrag.pfad.as_path())
        .collect();
    let quelle = format!("kai@server.test:{}", zweiter.display());
    assert!(
        pfade.contains(&Path::new(&format!("{quelle}/b.txt"))),
        "das Kind fehlt im Bericht: {pfade:?}"
    );
    assert!(
        pfade.contains(&Path::new(&quelle)),
        "der Ordner fehlt im Bericht: {pfade:?}"
    );
}

/// Eine Verknuepfung kommt als Verknuepfung mit ihrem Verweis an, auf den
/// Server und von dort zurueck; ihrem Ziel folgt keine der zwei Richtungen.
#[test]
fn eine_verknuepfung_kommt_mit_ihrem_ziel_an() {
    let ordner = Pruefordner::neu("fern-verknuepfung");
    let zeiger = ordner.verknuepfung("zeiger", "irgendwo/datei.txt");
    let fernordner = ordner.ordner("server");
    let rueckordner = ordner.ordner("zurueck");
    let server = zugang("server.test");

    let hin = fahren(
        Fernauftrag::kopieren(vec![lokal(&zeiger)], fern(&server, &fernordner)),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&hin.bericht);
    let zurueck = fahren(
        Fernauftrag::kopieren(
            vec![fern(&server, &fernordner.join("zeiger"))],
            lokal(&rueckordner),
        ),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&zurueck.bericht);

    for ziel in [fernordner.join("zeiger"), rueckordner.join("zeiger")] {
        assert_eq!(
            fs::read_link(&ziel).expect("keine Verknuepfung"),
            Path::new("irgendwo/datei.txt"),
            "{ziel:?}"
        );
    }
}

/// Ein ferner Eintrag wird ueber `RENAME` umbenannt und behaelt dabei seinen
/// Inode; ein vergebener Name scheitert, laesst beide Eintraege stehen und
/// traegt den Grund, den lokal `AlreadyExists` traegt (Entscheidung 10).
#[test]
fn ein_ferner_name_wird_umbenannt_und_ein_vergebener_nicht_ueberschrieben() {
    let ordner = Pruefordner::neu("fern-umbenennen");
    let alt = ordner.datei("alt.txt", b"alt");
    let inode = fs::metadata(&alt).expect("alt.txt").ino();
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::umbenennen(fern(&server, &alt), "neu.txt"),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig);
    assert_eq!(ausgang.bericht.eintraege, 1);
    assert!(ausgang.konflikte.is_empty(), "das Umbenennen fragt nicht");
    assert!(!alt.exists(), "der alte Name steht noch");
    assert_eq!(
        fs::metadata(ordner.pfad().join("neu.txt"))
            .expect("neu.txt")
            .ino(),
        inode,
        "umbenannt und nicht kopiert"
    );

    let anderer = ordner.datei("anderer.txt", b"anderer");
    let ausgang = fahren(
        Fernauftrag::umbenennen(fern(&server, &anderer), "neu.txt"),
        Konfliktantwort::Ueberschreiben,
    );
    assert!(ausgang.konflikte.is_empty(), "das Umbenennen fragt nicht");
    assert_eq!(ausgang.bericht.eintraege, 0);
    let [eintrag] = ausgang.bericht.uebersprungen.as_slice() else {
        panic!(
            "nicht genau ein Eintrag im Bericht: {:?}",
            ausgang.bericht.uebersprungen
        );
    };
    assert_eq!(
        fehlerart(&eintrag.grund),
        Some(std::io::ErrorKind::AlreadyExists),
        "der Grund ist nicht der des vergebenen Namens: {}",
        eintrag.grund
    );
    assert_eq!(fs::read(&anderer).expect("anderer.txt"), b"anderer");
    assert_eq!(
        fs::read(ordner.pfad().join("neu.txt")).expect("neu.txt"),
        b"alt",
        "der vergebene Name wurde ueberschrieben"
    );
}

/// Ein ferner Ordner und eine ferne leere Datei entstehen; ein vergebener
/// Name laesst das Anlegen scheitern, ohne den vorhandenen Eintrag
/// anzufassen, mit dem Grund des vergebenen Namens (Entscheidung 11).
#[test]
fn ein_ferner_ordner_und_eine_ferne_datei_entstehen_und_ein_vergebener_name_scheitert() {
    let ordner = Pruefordner::neu("fern-anlegen");
    let server = zugang("server.test");
    let ort = fern(&server, ordner.pfad());

    for (name, typ) in [
        ("Neuer Ordner", Anlegetyp::Ordner),
        ("neu.txt", Anlegetyp::Datei),
    ] {
        let ausgang = fahren(
            Fernauftrag::anlegen(ort.clone(), name, typ),
            Konfliktantwort::Abbrechen,
        );
        ohne_uebersprungene(&ausgang.bericht);
        assert_eq!(ausgang.bericht.eintraege, 1, "{name}");
        let angaben = fs::symlink_metadata(ordner.pfad().join(name)).expect(name);
        match typ {
            Anlegetyp::Ordner => assert!(angaben.is_dir(), "{name} ist kein Ordner"),
            Anlegetyp::Datei => {
                assert!(angaben.is_file(), "{name} ist keine Datei");
                assert_eq!(angaben.len(), 0, "{name} ist nicht leer");
            }
        }
    }

    let vorhanden = ordner.datei("vorhanden.txt", b"inhalt");
    for typ in [Anlegetyp::Ordner, Anlegetyp::Datei] {
        let ausgang = fahren(
            Fernauftrag::anlegen(ort.clone(), "vorhanden.txt", typ),
            Konfliktantwort::Ueberschreiben,
        );
        assert!(ausgang.konflikte.is_empty(), "das Anlegen fragt nicht");
        assert_eq!(ausgang.bericht.eintraege, 0, "{typ:?}");
        let [eintrag] = ausgang.bericht.uebersprungen.as_slice() else {
            panic!(
                "{typ:?}: nicht genau ein Eintrag im Bericht: {:?}",
                ausgang.bericht.uebersprungen
            );
        };
        assert_eq!(
            fehlerart(&eintrag.grund),
            Some(std::io::ErrorKind::AlreadyExists),
            "{typ:?}: der Grund ist nicht der des vergebenen Namens: {}",
            eintrag.grund
        );
        assert_eq!(
            fs::read(&vorhanden).expect("vorhanden.txt"),
            b"inhalt",
            "{typ:?}"
        );
    }
}

/// Ein ferner Baum wird endgueltig entfernt, Ordner samt Inhalt, und eine
/// Quelle, die es nicht mehr gibt, steht mit ihrem Grund im Bericht, ohne die
/// naechste aufzuhalten (Entscheidung 12).
#[test]
fn ein_ferner_baum_wird_endgueltig_entfernt() {
    let ordner = Pruefordner::neu("fern-loeschen");
    let tief = ordner.ordner("baum/unter/leer");
    fs::write(ordner.pfad().join("baum/unter/a.bin"), muster(70_000)).expect("a.bin");
    let baum = ordner.pfad().join("baum");
    let datei = ordner.datei("einzeln.txt", b"x");
    let zeiger = ordner.verknuepfung("zeiger", &datei);
    let bleibt = ordner.datei("bleibt.txt", b"bleibt");
    let fehlt = ordner.pfad().join("fehlt.txt");
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::loeschen(vec![
            fern(&server, &baum),
            fern(&server, &fehlt),
            fern(&server, &zeiger),
            fern(&server, &datei),
        ]),
        Konfliktantwort::Abbrechen,
    );

    assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig);
    assert_eq!(ausgang.bericht.eintraege, 3);
    assert!(ausgang.konflikte.is_empty(), "das Loeschen fragt nicht");
    assert!(!tief.exists() && !baum.exists(), "der Baum steht noch");
    assert!(
        fs::symlink_metadata(&zeiger).is_err(),
        "die Verknuepfung steht noch"
    );
    assert!(!datei.exists(), "die Datei steht noch");
    assert!(bleibt.exists(), "ein nicht gewaehlter Eintrag ist fort");
    let [eintrag] = ausgang.bericht.uebersprungen.as_slice() else {
        panic!(
            "nicht genau ein Eintrag im Bericht: {:?}",
            ausgang.bericht.uebersprungen
        );
    };
    assert_eq!(
        eintrag.pfad,
        PathBuf::from(format!("kai@server.test:{}", fehlt.display()))
    );
}

/// Der lokale Auftrag ist unveraendert: `Auftrag`, `Art`, der Rumpf von
/// `einen_abarbeiten` und die Dateien des lokalen Kopierens und Verschiebens
/// nennen keinen fernen Ort.
///
/// Gesucht wird nach ganzen Bezeichnern im Code, Kommentare ausgenommen:
/// `Ort`, `FernerOrt`, `fern` und jeder Bezeichner, der mit `Fern` beginnt.
///
/// # Was diese Nadel nicht sieht
///
/// Einen fernen Ort, der unter einem anderen Namen hereinkommt, etwa ueber
/// einen Typalias oder eine Funktion, die ihn verbirgt; und einen lokalen
/// Weg, der sich aendert, ohne einen fernen Ort zu nennen. Gegen das Zweite
/// stehen die Proben der Operationsmaschine in `tests/operation.rs` mit ihren
/// Erwartungen von vorher.
#[test]
fn der_lokale_auftrag_ist_unveraendert() {
    let dateien = quelldateien();
    let inhalt = |name: &str| {
        dateien
            .iter()
            .find(|(datei, _)| datei == name)
            .map(|(_, inhalt)| inhalt.clone())
            .unwrap_or_else(|| panic!("{name} steht nicht im Baum"))
    };
    let mut stellen: Vec<(String, Vec<String>)> = Vec::new();
    for name in [
        "krk-core/src/operation/auftrag.rs",
        "krk-core/src/operation/kopieren.rs",
        "krk-core/src/operation/verschieben.rs",
    ] {
        let quelle = inhalt(name);
        stellen.push((
            name.to_owned(),
            betriebscode(&quelle)
                .into_iter()
                .map(str::to_owned)
                .collect(),
        ));
    }
    let maschine = inhalt("krk-core/src/operation/mod.rs");
    let rumpf: Vec<String> = maschine
        .lines()
        .skip_while(|zeile| !zeile.starts_with("fn einen_abarbeiten("))
        .take_while(|zeile| *zeile != "}")
        .map(str::to_owned)
        .collect();
    assert!(!rumpf.is_empty(), "einen_abarbeiten steht nicht in mod.rs");
    stellen.push(("einen_abarbeiten".to_owned(), rumpf));

    let mut funde = Vec::new();
    for (name, zeilen) in &stellen {
        for zeile in zeilen {
            if zeile.trim_start().starts_with("//") {
                continue;
            }
            for wort in zeile.split(|zeichen: char| !(zeichen.is_alphanumeric() || zeichen == '_'))
            {
                if wort == "Ort"
                    || wort == "FernerOrt"
                    || wort == "fern"
                    || wort.starts_with("Fern")
                {
                    funde.push(format!("{name}: {}", zeile.trim()));
                }
            }
        }
    }
    assert!(
        funde.is_empty(),
        "der lokale Weg nennt einen fernen Ort: {funde:?}"
    );
}
