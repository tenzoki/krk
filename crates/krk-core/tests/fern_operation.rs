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
            Ort::Archiv(_) => unreachable!("die Tafel darueber nennt kein Archiv"),
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

/// Ob im Ordner ein Zwischenname des Ueberschreibens liegen geblieben ist,
/// der eines Ersatzes oder der eines beiseitegelegten Ziels.
fn zwischennamen_in(ordner: &Path) -> Vec<String> {
    namen_mit(ordner, ".krk-ersatz-")
        .into_iter()
        .chain(namen_mit(ordner, ".krk-alt-"))
        .collect()
}

/// Die Namen im Ordner, die mit `anfang` beginnen.
fn namen_mit(ordner: &Path, anfang: &str) -> Vec<String> {
    fs::read_dir(ordner)
        .expect("der Ordner laesst sich nicht lesen")
        .filter_map(|eintrag| eintrag.ok())
        .map(|eintrag| eintrag.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with(anfang))
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

/// „Ueberschreiben“ entfernt einen Zielordner nie zur Haelfte, bevor sein
/// Ersatz steht (Befund
/// `261017-0113-ueberschreiben-entfernt-einen-zielordner-zur-haelfte-…`).
///
/// Am Ziel steht ein Ordner `d/` mit `a.txt` und `zu/b.txt`, `zu/` mit den
/// Rechten `0555`; kopiert wird eine Datei `d`. Bis zur Behebung entfernte
/// der Tausch zuerst das Ziel: `a.txt` war fort, `zu/b.txt` liess sich nicht
/// entfernen, und danach fiel der vollstaendige Ersatz weg. Jetzt geht das
/// Ziel per `RENAME` beiseite, der Ersatz bekommt seinen Namen, und erst dann
/// wird das alte Ziel entfernt; was davon blieb, liegt unter seinem
/// Zwischennamen, und der Bericht nennt ihn.
#[test]
fn ueberschreiben_entfernt_einen_zielordner_nie_zur_haelfte() {
    let ordner = Pruefordner::neu("fern-ersatz-zielordner");
    let _ = ordner.ordner("quelle");
    let quelle = ordner.datei("quelle/d", b"neue datei");
    let fernordner = ordner.ordner("server");
    let d = ordner.ordner("server/d");
    fs::write(d.join("a.txt"), b"a").expect("a.txt");
    let zu = ordner.ordner("server/d/zu");
    fs::write(zu.join("b.txt"), b"b").expect("b.txt");
    fs::set_permissions(&zu, fs::Permissions::from_mode(0o555)).expect("Rechte");
    gemeinsam::rechtesperre_haelt_oder_abbruch(
        "ein Kind von zu/ laesst sich nicht entfernen",
        fs::remove_file(zu.join("b.txt")).is_err(),
    );
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::kopieren(vec![lokal(&quelle)], fern(&server, &fernordner)),
        Konfliktantwort::Ueberschreiben,
    );
    let alte = namen_mit(&fernordner, ".krk-alt-");
    for alt in &alte {
        let _ = fs::set_permissions(
            fernordner.join(alt).join("zu"),
            fs::Permissions::from_mode(0o755),
        );
    }
    let _ = fs::set_permissions(&zu, fs::Permissions::from_mode(0o755));

    assert_eq!(ausgang.konflikte.len(), 1, "{:?}", ausgang.konflikte);
    assert_eq!(
        fs::read(fernordner.join("d")).ok().as_deref(),
        Some(&b"neue datei"[..]),
        "der Ersatz steht nicht am Ziel; am Ziel steht: {:?}",
        fs::read_dir(fernordner.join("d")).map(|liste| liste.count())
    );
    assert_eq!(ausgang.bericht.eintraege, 1, "{:?}", ausgang.bericht);
    assert!(namen_mit(&fernordner, ".krk-ersatz-").is_empty());
    assert_eq!(alte.len(), 1, "{alte:?}");
    let alt = fernordner.join(&alte[0]);
    assert_eq!(
        fs::read(alt.join("zu/b.txt")).ok().as_deref(),
        Some(&b"b"[..]),
        "der Rest des alten Ziels fehlt"
    );
    let ziel = format!("kai@server.test:{}", fernordner.join("d").display());
    assert_eq!(
        ausgang.bericht.uebersprungen.len(),
        1,
        "{:?}",
        ausgang.bericht
    );
    let gemeldet = &ausgang.bericht.uebersprungen[0];
    assert_eq!(gemeldet.pfad, PathBuf::from(&ziel));
    assert!(
        gemeldet.grund.contains(alte[0].as_str()),
        "der Bericht nennt den Zwischennamen nicht: {}",
        gemeldet.grund
    );
}

/// Ein Zugang zu einem Server, der `SETSTAT` ablehnt.
fn zugang_ohne_setstat(rechner: &str) -> Arc<Fernzugang> {
    let kennung = Serverkennung::neu(rechner, None, Some("kai"));
    let plan = Kanalplan {
        argumente: vec!["-P".into(), "setstat,fsetstat,lsetstat".into()],
        ..Kanalplan::nur(SFTP_SERVER)
    };
    faden("krk-probe-fern".to_owned(), move |pfand| {
        Fernzugang::aufbauen(pfand, kennung, "Probe", &plan)
    })
    .expect("kein Faden fuer die Probe")
    .join()
    .expect("der Aufbau ist abgestuerzt")
    .expect("sftp-server laesst sich nicht ansprechen")
}

/// Lehnt der Server allein das Setzen von Rechten und Zeit ab, ersetzt
/// „Ueberschreiben“ das Ziel trotzdem, und der Hinweis steht unter dem Namen
/// des Ziels (Befund
/// `261017-0113-ueberschreiben-verwirft-einen-vollstaendigen-ersatz-…`).
///
/// Bis zur Behebung zaehlte der Hinweis als Luecke im Ersatz: der Ersatz fiel
/// weg, das Ziel blieb, und der Bericht fuehrte den Eintrag als fertig und als
/// uebersprungen, den Hinweis unter dem Zwischennamen. Eine Datei und ein
/// Ordner mit einer Datei darin, je ueber eine Datei gleichen Namens.
#[test]
fn ueberschreiben_gelingt_auch_wenn_der_server_rechte_und_zeit_ablehnt() {
    let ordner = Pruefordner::neu("fern-ersatz-ohne-setstat");
    let _ = ordner.ordner("quelle");
    let datei = ordner.datei("quelle/a.txt", b"neu");
    let baum = ordner.ordner("quelle/d");
    fs::write(baum.join("x.txt"), b"x").expect("x.txt");
    let fernordner = ordner.ordner("server");
    fs::write(fernordner.join("a.txt"), b"alt").expect("a.txt");
    fs::write(fernordner.join("d"), b"alte datei").expect("d");
    let server = zugang_ohne_setstat("server.test");

    let ausgang = fahren(
        Fernauftrag::kopieren(
            vec![lokal(&datei), lokal(&baum)],
            fern(&server, &fernordner),
        ),
        Konfliktantwort::Ueberschreiben,
    );

    assert_eq!(ausgang.konflikte.len(), 2, "{:?}", ausgang.konflikte);
    assert_eq!(fs::read(fernordner.join("a.txt")).expect("a.txt"), b"neu");
    assert_eq!(fs::read(fernordner.join("d/x.txt")).expect("d/x.txt"), b"x");
    assert!(
        zwischennamen_in(&fernordner).is_empty(),
        "{:?}",
        zwischennamen_in(&fernordner)
    );
    let am_ziel = |name: &str| {
        PathBuf::from(format!(
            "kai@server.test:{}",
            fernordner.join(name).display()
        ))
    };
    let gemeldet: Vec<&Path> = ausgang
        .bericht
        .uebersprungen
        .iter()
        .map(|eintrag| eintrag.pfad.as_path())
        .collect();
    assert_eq!(
        gemeldet,
        vec![am_ziel("a.txt"), am_ziel("d/x.txt"), am_ziel("d")],
        "{:?}",
        ausgang.bericht
    );
    for eintrag in &ausgang.bericht.uebersprungen {
        assert!(
            eintrag
                .grund
                .starts_with("Inhalt kopiert, Rechte und Datum"),
            "{}",
            eintrag.grund
        );
    }
    for quelle in [&datei, &baum] {
        assert!(
            !gemeldet.contains(&quelle.as_path()),
            "die Quelle steht als uebersprungen im Bericht"
        );
    }
    // Zwei Quellen und das Kind im Ordner, jedes einmal.
    assert_eq!(ausgang.bericht.eintraege, 3, "{:?}", ausgang.bericht);
}

/// Verschieben ueber eine Grenze entfernt die Quelle, wenn allein Hinweise zu
/// angekommenen Eintraegen im Bericht stehen, und nennt sie nie als „nicht
/// vollstaendig kopiert“ (Befund
/// `261017-0145-verschieben-ueber-eine-grenze-nennt-einen-vollstaendigen-ersatz-…`).
///
/// Drei Faelle: eine Datei ueber eine Datei gleichen Namens und eine ohne
/// Konflikt, je auf einen Server, der Rechte und Zeit ablehnt; dazu eine
/// Datei ueber einen Ordner, von dem nach dem Tausch ein Rest bleibt. Bis zur
/// Behebung zaehlte jeder dieser Hinweise als Luecke: die Quelle blieb und
/// stand als fertig und als „nicht vollstaendig kopiert, in der Quelle
/// geblieben“ im Bericht.
#[test]
fn verschieben_ueber_eine_grenze_entfernt_die_quelle_auch_wenn_allein_hinweise_bleiben() {
    let ordner = Pruefordner::neu("fern-verschieben-hinweise");
    let _ = ordner.ordner("quelle");
    let ersetzend = ordner.datei("quelle/a.txt", b"neu");
    let frei = ordner.datei("quelle/b.txt", b"b");
    let fernordner = ordner.ordner("server");
    fs::write(fernordner.join("a.txt"), b"alt").expect("a.txt");
    let ohne_setstat = zugang_ohne_setstat("server.test");
    let am_ziel = |ordner: &Path, name: &str| {
        PathBuf::from(format!("kai@server.test:{}", ordner.join(name).display()))
    };

    let ausgang = fahren(
        Fernauftrag::verschieben(
            vec![lokal(&ersetzend), lokal(&frei)],
            fern(&ohne_setstat, &fernordner),
        ),
        Konfliktantwort::Ueberschreiben,
    );
    assert_eq!(ausgang.konflikte.len(), 1, "{:?}", ausgang.konflikte);
    assert_eq!(fs::read(fernordner.join("a.txt")).expect("a.txt"), b"neu");
    assert_eq!(fs::read(fernordner.join("b.txt")).expect("b.txt"), b"b");
    assert!(!ersetzend.exists(), "die Quelle des Ersatzes steht noch");
    assert!(!frei.exists(), "die Quelle ohne Konflikt steht noch");
    let gemeldet: Vec<&Path> = ausgang
        .bericht
        .uebersprungen
        .iter()
        .map(|eintrag| eintrag.pfad.as_path())
        .collect();
    assert_eq!(
        gemeldet,
        vec![am_ziel(&fernordner, "a.txt"), am_ziel(&fernordner, "b.txt")],
        "{:?}",
        ausgang.bericht
    );
    assert_eq!(ausgang.bericht.eintraege, 2, "{:?}", ausgang.bericht);

    // Eine Datei ueber einen Ordner, dessen Rest nach dem Tausch bleibt.
    let _ = ordner.ordner("quelle2");
    let quelle = ordner.datei("quelle2/d", b"neue datei");
    let zweiter = ordner.ordner("server2");
    let _ = ordner.ordner("server2/d");
    let zu = ordner.ordner("server2/d/zu");
    fs::write(zu.join("b.txt"), b"b").expect("b.txt");
    fs::set_permissions(&zu, fs::Permissions::from_mode(0o555)).expect("Rechte");
    gemeinsam::rechtesperre_haelt_oder_abbruch(
        "ein Kind von zu/ laesst sich nicht entfernen",
        fs::remove_file(zu.join("b.txt")).is_err(),
    );
    let server = zugang("server.test");

    let ausgang = fahren(
        Fernauftrag::verschieben(vec![lokal(&quelle)], fern(&server, &zweiter)),
        Konfliktantwort::Ueberschreiben,
    );
    let alte = namen_mit(&zweiter, ".krk-alt-");
    for alt in &alte {
        let _ = fs::set_permissions(
            zweiter.join(alt).join("zu"),
            fs::Permissions::from_mode(0o755),
        );
    }
    let _ = fs::set_permissions(&zu, fs::Permissions::from_mode(0o755));

    assert_eq!(
        fs::read(zweiter.join("d")).ok().as_deref(),
        Some(&b"neue datei"[..]),
        "der Ersatz steht nicht am Ziel"
    );
    assert!(!quelle.exists(), "die Quelle steht noch");
    assert_eq!(alte.len(), 1, "{alte:?}");
    assert_eq!(
        ausgang.bericht.uebersprungen.len(),
        1,
        "{:?}",
        ausgang.bericht
    );
    let gemeldet = &ausgang.bericht.uebersprungen[0];
    assert_eq!(gemeldet.pfad, am_ziel(&zweiter, "d"));
    assert!(
        gemeldet.grund.contains(alte[0].as_str()),
        "{}",
        gemeldet.grund
    );
    assert_eq!(ausgang.bericht.eintraege, 1, "{:?}", ausgang.bericht);
}

/// Verschieben mit „Ueberschreiben“ verliert nie die einzige Kopie, wenn
/// Quelle und Ziel ueber zwei Wege derselbe Eintrag sind (Befund
/// `261017-0113-verschieben-mit-ueberschreiben-loescht-die-einzige-kopie-…`).
///
/// Zwei Wege zu demselben Ordner: lokal und ueber den Server, und ueber zwei
/// Kennungen auf denselben Server. Bis zur Behebung entfernte der Tausch das
/// Ziel, also die Quelle, setzte den Ersatz an ihren Namen, und danach
/// entfernte das Verschieben die Quelle, unter deren Namen nun der Ersatz
/// stand. Jetzt faellt beim Beiseitelegen des Ziels auf, dass auch die Quelle
/// fort ist; das Ziel kommt zurueck, und der Eintrag steht im Bericht.
#[test]
fn verschieben_mit_ueberschreiben_verliert_keine_quelle_die_ueber_zwei_wege_das_ziel_ist() {
    let ordner = Pruefordner::neu("fern-zwei-wege");
    let hier = ordner.ordner("hier");
    let datei = ordner.datei("hier/x.txt", b"inhalt");
    let server = zugang("server.test");
    let zweiter = zugang("zweiter.test");

    for (art, auftrag) in [
        (
            "lokal nach fern, verschieben",
            Fernauftrag::verschieben(vec![lokal(&datei)], fern(&server, &hier)),
        ),
        (
            "fern nach lokal, verschieben",
            Fernauftrag::verschieben(vec![fern(&server, &datei)], lokal(&hier)),
        ),
        (
            "zwei Kennungen, verschieben",
            Fernauftrag::verschieben(vec![fern(&server, &datei)], fern(&zweiter, &hier)),
        ),
        (
            "zwei Kennungen, kopieren",
            Fernauftrag::kopieren(vec![fern(&server, &datei)], fern(&zweiter, &hier)),
        ),
    ] {
        let ausgang = fahren(auftrag, Konfliktantwort::Ueberschreiben);
        assert_eq!(
            fs::read(&datei).ok().as_deref(),
            Some(&b"inhalt"[..]),
            "{art}: die Datei ist verloren"
        );
        assert_eq!(ausgang.konflikte.len(), 1, "{art}: {:?}", ausgang.konflikte);
        assert_eq!(
            ausgang.bericht.uebersprungen.len(),
            1,
            "{art}: {:?}",
            ausgang.bericht
        );
        assert!(
            zwischennamen_in(&hier).is_empty(),
            "{art}: {:?}",
            zwischennamen_in(&hier)
        );
    }
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

/// Ein Abbild, das unter einem Ordner eingehaengt ist und beim Fallen wieder
/// ausgehaengt wird: ein zweites Dateisystem fuer den Server des Systems.
///
/// Haelt keinen Pruefordner: das Abbild und der Einhaengepunkt liegen in dem
/// der Probe, und dieser Wert faellt vor ihm, weil er nach ihm entsteht.
struct Einhaengung {
    punkt: PathBuf,
}

impl Einhaengung {
    /// Legt unter `ordner` ein Abbild von einem Megabyte mit HFS+ an und
    /// haengt es unter `ordner/volume` ein, ohne es im Finder zu zeigen.
    fn neu(ordner: &Pruefordner) -> Self {
        let abbild = ordner.unter("volume.dmg");
        let punkt = ordner.ordner("volume");
        let hdiutil = |argumente: &[&std::ffi::OsStr]| {
            let stand = std::process::Command::new("/usr/bin/hdiutil")
                .args(argumente)
                .status()
                .expect("hdiutil laesst sich nicht starten");
            assert!(stand.success(), "hdiutil {argumente:?} ist gescheitert");
        };
        hdiutil(&[
            "create".as_ref(),
            "-quiet".as_ref(),
            "-size".as_ref(),
            "1m".as_ref(),
            "-fs".as_ref(),
            "HFS+".as_ref(),
            "-layout".as_ref(),
            "NONE".as_ref(),
            "-volname".as_ref(),
            "krkprobe".as_ref(),
            abbild.as_os_str(),
        ]);
        hdiutil(&[
            "attach".as_ref(),
            "-quiet".as_ref(),
            "-nobrowse".as_ref(),
            "-mountpoint".as_ref(),
            punkt.as_os_str(),
            abbild.as_os_str(),
        ]);
        Einhaengung { punkt }
    }
}

impl Drop for Einhaengung {
    fn drop(&mut self) {
        // Scheitert das Aushaengen, bleibt das Abbild eingehaengt, und der
        // Pruefordner darueber laesst sich nicht ganz abraeumen; eine Probe
        // wird davon nicht rot, aber `hdiutil info` zeigt den Rest.
        let _ = std::process::Command::new("/usr/bin/hdiutil")
            .args(["detach", "-quiet", "-force"])
            .arg(&self.punkt)
            .status();
    }
}

/// Verschieben auf demselben Server ueber die Grenze zweier Dateisysteme dort
/// kommt an, und die Quelle ist danach fort (Befund
/// `261017-0004-verschieben-auf-demselben-server-ueber-eine-mount-grenze-des-servers-scheitert-ohne-ausweichweg`).
///
/// Der Server des Systems beantwortet `RENAME` von der Platte in das
/// eingehaengte Abbild mit `FAILURE`, denn `link(2)` und `rename(2)` scheitern
/// dort an `EXDEV`. Bis zur Behebung stand der Eintrag danach mit diesem
/// Grund im Bericht, und die Quelle blieb, wo sie war; jetzt wird kopiert und
/// die Quelle entfernt wie ueber eine Grenze zwischen zwei Servern, eine
/// Datei wie ein Ordner samt Inhalt.
#[test]
fn verschieben_ueber_eine_grenze_auf_dem_server_kopiert_und_entfernt_die_quelle() {
    let ordner = Pruefordner::neu("fern-verschieben-grenze");
    let datei = ordner.datei("f.txt", b"f");
    let baum = ordner.ordner("baum");
    fs::write(baum.join("a.txt"), b"a").expect("a.txt");
    let volume = Einhaengung::neu(&ordner);
    assert_ne!(
        fs::metadata(&datei).expect("f.txt").dev(),
        fs::metadata(&volume.punkt).expect("volume").dev(),
        "das Abbild ist kein eigenes Dateisystem"
    );

    let server = zugang("server.test");
    let ausgang = fahren(
        Fernauftrag::verschieben(
            vec![fern(&server, &datei), fern(&server, &baum)],
            fern(&server, &volume.punkt),
        ),
        Konfliktantwort::Abbrechen,
    );
    ohne_uebersprungene(&ausgang.bericht);
    assert!(ausgang.konflikte.is_empty(), "{:?}", ausgang.konflikte);
    assert_eq!(
        fs::read(volume.punkt.join("f.txt")).expect("f.txt am Ziel"),
        b"f"
    );
    assert_eq!(
        fs::read(volume.punkt.join("baum/a.txt")).expect("a.txt am Ziel"),
        b"a"
    );
    assert!(!datei.exists(), "die Datei steht noch in der Quelle");
    assert!(!baum.exists(), "der Ordner steht noch in der Quelle");
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
