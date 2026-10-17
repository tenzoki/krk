//! Abnahme von Schritt 4 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`: das
//! Herauskopieren aus einem Archiv in ein lokales oder fernes Ziel, ueber die
//! Maschine fuer ferne Vorgaenge mit dem Archiveintrag als Lesequelle (E11).
//!
//! Jede Probe baut ihre Archive selbst in einem Pruefordner
//! (`gemeinsam::archivbau`), das verschluesselte Zip ueber `/usr/bin/zip -P`;
//! der ferne Ort steht auf `/usr/libexec/sftp-server` ohne `ssh`, wie in
//! `tests/fern_operation.rs`. Jeder Vorgang laeuft ueber `operation::starten`
//! und damit ueber denselben Faden, denselben Kanal und dieselben Meldungen
//! wie im Betrieb.

mod gemeinsam;

use std::fs::{self, File, FileTimes, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, UNIX_EPOCH};

use gemeinsam::Pruefordner;
use gemeinsam::archivbau::{gzip_glied, tar_gz_packen, tar_packen, zip_packen};
use krk_core::archiv::{Archivformat, Archivleser, Archivzugang, Innenpfad};
use krk_core::fern::{Fernpfad, Fernzugang, Kanalplan, Serverkennung, faden};
use krk_core::operation::{
    Abschluss, Bericht, Fernauftrag, Konfliktantwort, Konfliktentscheid, Meldung, OhnePapierkorb,
    starten,
};
use krk_core::ort::{Archivort, FernerOrt, Ort};
use krk_core::sprache::{Text, satz, text};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Der Server des Systems, ohne `ssh` davor.
const SFTP_SERVER: &str = "/usr/libexec/sftp-server";

/// Der Ort `innen` im Archiv `datei`, mit einem eigenen Zugang.
fn archivort(datei: &Path, innen: &str) -> Archivort {
    let format = Archivformat::aus_namen(datei.file_name().expect("Name"))
        .expect("die Probe gibt ihrem Archiv eine Endung aus der Tafel");
    Archivort {
        zugang: Arc::new(Archivzugang::neu(datei.to_path_buf(), format)),
        innen: Innenpfad::aus_archivname(innen).expect("ein sicherer Name"),
    }
}

/// Die Orte der Namen `namen` unter der Wurzel des Archivs, mit einem
/// geteilten Zugang wie im Dateifenster.
fn im_archiv(datei: &Path, namen: &[&str]) -> Vec<Ort> {
    let wurzel = archivort(datei, "");
    namen
        .iter()
        .map(|name| Ort::Archiv(wurzel.kind(name).expect("ein einzelner Name")))
        .collect()
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

fn ohne_uebersprungene(bericht: &Bericht) {
    assert!(
        bericht.uebersprungen.is_empty(),
        "uebersprungen: {:?}",
        bericht.uebersprungen
    );
}

/// Ein Inhalt, der sich ueber Blockgrenzen nicht wiederholt.
fn muster(laenge: usize) -> Vec<u8> {
    (0..laenge).map(|stelle| (stelle % 251) as u8).collect()
}

/// Setzt Rechte und Aenderungszeit eines Eintrags.
fn angaben_setzen(pfad: &Path, rechte: u32, sekunden: u64) {
    let zeit = UNIX_EPOCH + Duration::from_secs(sekunden);
    File::open(pfad)
        .expect("oeffnen")
        .set_times(FileTimes::new().set_modified(zeit).set_accessed(zeit))
        .expect("Zeit setzen");
    fs::set_permissions(pfad, fs::Permissions::from_mode(rechte)).expect("Rechte setzen");
}

/// Ein Baum mit zwei Ebenen, einer Datei ueber mehrere Bloecke, einer leeren
/// Datei, eigenen Rechten und Zeiten und einer Verknuepfung.
fn baum_anlegen(wurzel: &Path) {
    fs::create_dir_all(wurzel.join("ordner/unter")).expect("Ordner");
    fs::write(wurzel.join("gross.bin"), muster(700_001)).expect("gross.bin");
    fs::write(wurzel.join("leer.txt"), b"").expect("leer.txt");
    fs::write(wurzel.join("ordner/a.txt"), b"alpha").expect("a.txt");
    fs::write(wurzel.join("ordner/unter/b.txt"), b"beta").expect("b.txt");
    std::os::unix::fs::symlink("ordner/a.txt", wurzel.join("verweis")).expect("Verknuepfung");
    angaben_setzen(&wurzel.join("gross.bin"), 0o640, 1_600_000_000);
    angaben_setzen(&wurzel.join("leer.txt"), 0o600, 1_600_000_100);
    angaben_setzen(&wurzel.join("ordner/a.txt"), 0o644, 1_600_000_200);
    angaben_setzen(&wurzel.join("ordner/unter/b.txt"), 0o755, 1_600_000_300);
    // Ordner zuletzt, von innen nach aussen: jedes Anlegen darin setzte ihre
    // Zeit neu.
    angaben_setzen(&wurzel.join("ordner/unter"), 0o750, 1_600_000_400);
    angaben_setzen(&wurzel.join("ordner"), 0o755, 1_600_000_500);
}

/// Ein Eintrag des Baums, wie [`bild`] ihn vergleicht.
type Bildzeile = (String, String, Vec<u8>, Option<u32>, Option<i64>);

/// Was an einem Eintrag des Baums verglichen wird: Name, Art, Inhalt oder
/// Verweis, Rechte und Aenderungszeit auf die Sekunde. Eine Verknuepfung
/// traegt weder Rechte noch Zeit, die mitgehen.
fn bild(wurzel: &Path) -> Vec<Bildzeile> {
    let mut bild = Vec::new();
    for (name, pfad) in gemeinsam::archivbau::baum(wurzel) {
        let angaben = fs::symlink_metadata(&pfad).expect("Angaben");
        let (art, inhalt, rechte, zeit) = if angaben.is_symlink() {
            let verweis = fs::read_link(&pfad).expect("Verweis");
            (
                "verknuepfung",
                verweis.to_string_lossy().into_owned().into_bytes(),
                None,
                None,
            )
        } else if angaben.is_dir() {
            (
                "ordner",
                Vec::new(),
                Some(angaben.mode() & 0o7777),
                Some(angaben.mtime()),
            )
        } else {
            (
                "datei",
                fs::read(&pfad).expect("Inhalt"),
                Some(angaben.mode() & 0o7777),
                Some(angaben.mtime()),
            )
        };
        bild.push((name, art.to_owned(), inhalt, rechte, zeit));
    }
    bild
}

/// Die obersten Namen des Baums, wie die Auswahl eines Archivtabs sie nennt.
const OBERSTE: [&str; 4] = ["gross.bin", "leer.txt", "ordner", "verweis"];

/// Ein Ordnerbaum kommt aus einem Zip, einem tar und einem tar.gz Byte fuer
/// Byte heraus, mit Rechten und Aenderungszeit am Ziel und der Verknuepfung
/// als Verknuepfung.
#[test]
fn ein_ordnerbaum_kommt_aus_zip_tar_und_tgz_byte_fuer_byte_heraus() {
    let ordner = Pruefordner::neu("archiv-heraus-baum");
    let quelle = ordner.ordner("quelle");
    baum_anlegen(&quelle);
    let erwartet = bild(&quelle);
    for (endung, packen) in [
        ("zip", zip_packen as fn(&Path, &Path)),
        ("tar", tar_packen),
        ("tgz", tar_gz_packen),
    ] {
        let archiv = ordner.unter(&format!("baum.{endung}"));
        packen(&quelle, &archiv);
        let vorher = fs::read(&archiv).expect("Archiv");
        let ziel = ordner.ordner(&format!("ziel-{endung}"));

        let ausgang = fahren(
            Fernauftrag::kopieren(im_archiv(&archiv, &OBERSTE), Ort::Lokal(ziel.clone())),
            Konfliktantwort::Abbrechen,
        );

        assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig, "{endung}");
        ohne_uebersprungene(&ausgang.bericht);
        assert!(
            ausgang.konflikte.is_empty(),
            "{endung}: {:?}",
            ausgang.konflikte
        );
        assert_eq!(
            bild(&ziel),
            erwartet,
            "{endung}: der Baum am Ziel weicht ab"
        );
        assert_eq!(
            fs::read(&archiv).expect("Archiv"),
            vorher,
            "{endung}: das Archiv ist angefasst"
        );
    }
}

/// Eine gz-Datei kommt entpackt heraus, auch ein Strom aus zwei Gliedern, mit
/// den Rechten und der Aenderungszeit der Archivdatei.
#[test]
fn eine_gz_datei_kommt_entpackt_heraus() {
    let ordner = Pruefordner::neu("archiv-heraus-gz");
    let erstes = muster(300_000);
    let zweites = b"und ein zweites Glied".to_vec();
    let mut gepackt = gzip_glied(&erstes);
    gepackt.extend(gzip_glied(&zweites));
    let archiv = ordner.datei("bericht.txt.gz", &gepackt);
    angaben_setzen(&archiv, 0o640, 1_650_000_000);
    let ziel = ordner.ordner("ziel");

    let ausgang = fahren(
        Fernauftrag::kopieren(
            im_archiv(&archiv, &["bericht.txt"]),
            Ort::Lokal(ziel.clone()),
        ),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    let mut inhalt = erstes;
    inhalt.extend(zweites);
    let angekommen = ziel.join("bericht.txt");
    assert_eq!(fs::read(&angekommen).expect("bericht.txt"), inhalt);
    let angaben = fs::metadata(&angekommen).expect("Angaben");
    assert_eq!(angaben.mode() & 0o7777, 0o640);
    assert_eq!(angaben.mtime(), 1_650_000_000);
    assert_eq!(fs::read(&archiv).expect("Archiv"), gepackt);
}

/// Setzt das Verfahren eines Zip-Eintrags auf eines, das KRK nicht entpackt,
/// im lokalen Kopf und im zentralen Satz (wie in `tests/archiv_umbau.rs`).
fn verfahren_setzen(archiv: &Path, name: &str, verfahren: u16) {
    let (lokal, zentral) = {
        let mut kiste =
            zip::ZipArchive::new(File::open(archiv).expect("Zip oeffnen")).expect("Zip lesbar");
        let stelle = kiste.index_for_name(name).expect("Eintrag");
        let eintrag = kiste.by_index_raw(stelle).expect("Eintrag");
        (eintrag.header_start(), eintrag.central_header_start())
    };
    let mut datei = OpenOptions::new()
        .write(true)
        .open(archiv)
        .expect("Zip zum Patchen oeffnen");
    for stelle in [lokal + 8, zentral + 10] {
        datei.seek(SeekFrom::Start(stelle)).expect("springen");
        datei.write_all(&verfahren.to_le_bytes()).expect("patchen");
    }
}

/// Ein verschluesselter Eintrag und einer mit einem Verfahren, das KRK nicht
/// entpackt, kommen nicht heraus und stehen mit ihrem Grund im Bericht; am
/// Ziel entsteht fuer sie nichts, und die uebrigen kommen heraus.
#[test]
fn ein_verschluesselter_und_ein_unbekannt_gepackter_eintrag_stehen_mit_grund_im_bericht() {
    let ordner = Pruefordner::neu("archiv-heraus-unlesbar");
    ordner.datei("geheim.txt", b"streng geheim");
    ordner.datei("offen.txt", b"offen");
    for argumente in [
        ["-q", "-P", "kennwort", "geheim.zip", "geheim.txt"].as_slice(),
        ["-q", "geheim.zip", "offen.txt"].as_slice(),
    ] {
        let stand = Command::new("/usr/bin/zip")
            .current_dir(ordner.pfad())
            .args(argumente)
            .status()
            .expect("/usr/bin/zip laesst sich starten");
        assert!(stand.success());
    }
    let bzip = ordner.unter("bzip.zip");
    let mut schreiber = ZipWriter::new(File::create(&bzip).expect("Zip anlegen"));
    for name in ["b.txt", "c.txt"] {
        schreiber
            .start_file(
                name,
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
            )
            .expect("Eintrag");
        schreiber.write_all(name.as_bytes()).expect("Inhalt");
    }
    schreiber.finish().expect("Zip abschliessen");
    verfahren_setzen(&bzip, "b.txt", 12);
    let ziel = ordner.ordner("ziel");

    let geheim = fahren(
        Fernauftrag::kopieren(
            im_archiv(&ordner.unter("geheim.zip"), &["geheim.txt", "offen.txt"]),
            Ort::Lokal(ziel.clone()),
        ),
        Konfliktantwort::Abbrechen,
    );
    let unbekannt = fahren(
        Fernauftrag::kopieren(
            im_archiv(&bzip, &["b.txt", "c.txt"]),
            Ort::Lokal(ziel.clone()),
        ),
        Konfliktantwort::Abbrechen,
    );

    let gemeldet: Vec<(PathBuf, String)> = geheim
        .bericht
        .uebersprungen
        .iter()
        .chain(&unbekannt.bericht.uebersprungen)
        .map(|eintrag| (eintrag.pfad.clone(), eintrag.grund.clone()))
        .collect();
    assert_eq!(
        gemeldet,
        [
            (
                PathBuf::from(format!(
                    "{}:/geheim.txt",
                    ordner.unter("geheim.zip").display()
                )),
                text(Text::ArchivEintragVerschluesselt).to_owned(),
            ),
            (
                PathBuf::from(format!("{}:/b.txt", bzip.display())),
                satz(Text::ArchivVerfahrenUnbekannt, &[("verfahren", &"bzip2")]),
            ),
        ]
    );
    assert!(!ziel.join("geheim.txt").exists());
    assert!(!ziel.join("b.txt").exists());
    assert_eq!(
        fs::read(ziel.join("offen.txt")).expect("offen.txt"),
        b"offen"
    );
    assert_eq!(fs::read(ziel.join("c.txt")).expect("c.txt"), b"c.txt");
}

/// Ein vorhandener Name am Ziel stellt dieselbe Konfliktfrage wie lokal, mit
/// der Quelle in der Schreibweise des Fenstertitels; „Ueberschreiben“ ersetzt,
/// „Ueberspringen“ laesst das Ziel stehen, und Ordner auf Ordner fuehrt ohne
/// Frage zusammen.
#[test]
fn herauskopieren_auf_ein_vorhandenes_ziel_stellt_die_konfliktfrage_wie_lokal() {
    let ordner = Pruefordner::neu("archiv-heraus-konflikt");
    let quelle = ordner.ordner("quelle");
    fs::create_dir(quelle.join("ordner")).expect("ordner");
    fs::write(quelle.join("ordner/neu.txt"), b"neu").expect("neu.txt");
    fs::write(quelle.join("a.txt"), b"aus dem Archiv").expect("a.txt");
    let archiv = ordner.unter("konflikt.tgz");
    tar_gz_packen(&quelle, &archiv);

    for (antwort, erwartet) in [
        (Konfliktantwort::Ueberschreiben, &b"aus dem Archiv"[..]),
        (Konfliktantwort::Ueberspringen, &b"am Ziel"[..]),
    ] {
        let ziel = ordner.ordner(&format!("ziel-{antwort:?}"));
        fs::write(ziel.join("a.txt"), b"am Ziel").expect("a.txt am Ziel");
        fs::create_dir(ziel.join("ordner")).expect("ordner am Ziel");
        fs::write(ziel.join("ordner/alt.txt"), b"alt").expect("alt.txt");

        let ausgang = fahren(
            Fernauftrag::kopieren(
                im_archiv(&archiv, &["a.txt", "ordner"]),
                Ort::Lokal(ziel.clone()),
            ),
            antwort.clone(),
        );

        assert_eq!(
            ausgang.konflikte,
            [(
                PathBuf::from(format!("{}:/a.txt", archiv.display())),
                ziel.join("a.txt"),
            )],
            "{antwort:?}"
        );
        assert_eq!(fs::read(ziel.join("a.txt")).expect("a.txt"), erwartet);
        assert_eq!(fs::read(ziel.join("ordner/alt.txt")).expect("alt"), b"alt");
        assert_eq!(fs::read(ziel.join("ordner/neu.txt")).expect("neu"), b"neu");
        let reste: Vec<String> = fs::read_dir(&ziel)
            .expect("Ziel lesen")
            .map(|eintrag| {
                eintrag
                    .expect("Eintrag")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|name| name.starts_with(".krk-"))
            .collect();
        assert!(reste.is_empty(), "Zwischennamen blieben: {reste:?}");
    }
}

/// Ein Abbruch mitten in einer grossen Datei laesst am Ziel keine halbe Datei
/// stehen, und das Archiv gleicht Byte fuer Byte dem Stand davor; aus einem
/// Zip wie aus einem tar.
#[test]
fn ein_abbruch_beim_herauskopieren_laesst_kein_halbes_ziel_und_ein_unveraendertes_archiv() {
    let ordner = Pruefordner::neu("archiv-heraus-abbruch");
    let inhalt = muster(24 * 1024 * 1024);
    let zip = ordner.unter("gross.zip");
    let mut schreiber = ZipWriter::new(File::create(&zip).expect("Zip anlegen"));
    schreiber
        .start_file(
            "gross.bin",
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
        )
        .expect("Eintrag");
    schreiber.write_all(&inhalt).expect("Inhalt");
    schreiber.finish().expect("Zip abschliessen");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("gross.bin"), &inhalt).expect("gross.bin");
    let tar = ordner.unter("gross.tar");
    tar_packen(&quelle, &tar);

    for archiv in [zip, tar] {
        let vorher = fs::read(&archiv).expect("Archiv");
        let ziel = ordner.ordner(&format!(
            "ziel-{}",
            archiv.extension().and_then(|e| e.to_str()).expect("Endung")
        ));
        let bericht = mitten_abbrechen(Fernauftrag::kopieren(
            im_archiv(&archiv, &["gross.bin"]),
            Ort::Lokal(ziel.clone()),
        ));
        assert_eq!(bericht.abschluss, Abschluss::Abgebrochen, "{archiv:?}");
        let geblieben: Vec<_> = fs::read_dir(&ziel).expect("Ziel lesen").collect();
        assert!(geblieben.is_empty(), "am Ziel blieb etwas: {geblieben:?}");
        assert_eq!(fs::read(&archiv).expect("Archiv"), vorher);
    }
}

/// Ein Zugang zum Server des Systems unter der Kennung `kai@rechner`.
fn zugang(rechner: &str) -> Arc<Fernzugang> {
    let kennung = Serverkennung::neu(rechner, None, Some("kai"));
    faden("krk-probe-archiv-fern".to_owned(), move |pfand| {
        Fernzugang::aufbauen(pfand, kennung, "Probe", &Kanalplan::nur(SFTP_SERVER))
    })
    .expect("kein Faden fuer die Probe")
    .join()
    .expect("der Aufbau ist abgestuerzt")
    .expect("sftp-server laesst sich nicht ansprechen")
}

/// Herauskopieren auf einen fernen Ort geht durch dieselbe Uebertragung wie
/// jedes Kopieren auf einen Server.
#[test]
fn herauskopieren_auf_einen_fernen_ort_geht() {
    let ordner = Pruefordner::neu("archiv-heraus-fern");
    let quelle = ordner.ordner("quelle");
    baum_anlegen(&quelle);
    let archiv = ordner.unter("baum.tgz");
    tar_gz_packen(&quelle, &archiv);
    let server = ordner.ordner("server");
    let fern = Ort::Fern(FernerOrt {
        zugang: zugang("rechner"),
        pfad: Fernpfad::aus_text(server.to_str().expect("UTF-8")).expect("absolut"),
    });

    let ausgang = fahren(
        Fernauftrag::kopieren(im_archiv(&archiv, &OBERSTE), fern),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(bild(&server), bild(&quelle));
}

/// Kein Name eines Archivs bringt das Herauskopieren dazu, ausserhalb des
/// Zielordners zu schreiben: `..`, ein absoluter Name und der Weg durch eine
/// Verknuepfung im selben Archiv stehen nicht im Verzeichnis, also auch in
/// keiner Auswahl, und eine Verknuepfung entsteht am Ziel als Verknuepfung,
/// durch die nichts geschrieben wird.
///
/// **Was sie nicht sieht:** einen Auftrag, den jemand an der Auswahl vorbei
/// mit einem Pfad baut; das verhindert der Typ, denn ein `Innenpfad` traegt
/// weder `..` noch ein leeres Glied.
#[test]
fn herauskopieren_schreibt_nie_ausserhalb_des_ziels() {
    let ordner = Pruefordner::neu("archiv-heraus-slip");
    let archiv = ordner.unter("boese.zip");
    let mut schreiber = ZipWriter::new(File::create(&archiv).expect("Zip anlegen"));
    for name in [
        "../draussen.txt",
        "a/../../auch.txt",
        "/etc/absolut.txt",
        "gut.txt",
    ] {
        schreiber
            .start_file(name, SimpleFileOptions::default())
            .expect("Eintrag");
        schreiber.write_all(b"x").expect("Inhalt");
    }
    schreiber
        .add_symlink(
            "hinaus",
            ordner.pfad().to_str().expect("UTF-8"),
            SimpleFileOptions::default(),
        )
        .expect("Verknuepfung");
    schreiber
        .start_file("hinaus/durch.txt", SimpleFileOptions::default())
        .expect("Eintrag");
    schreiber.write_all(b"x").expect("Inhalt");
    schreiber.finish().expect("Zip abschliessen");
    let ziel = ordner.ordner("ziel");

    // Die Auswahl eines Archivtabs: jeder Name, den das Verzeichnis unter der
    // Wurzel zeigt.
    let zugang = Arc::new(Archivzugang::neu(archiv.clone(), Archivformat::Zip));
    let namen: Vec<String> = faden("krk-probe-slip".to_owned(), {
        let zugang = Arc::clone(&zugang);
        move |pfand| {
            let leser = Archivleser::oeffnen(pfand, zugang, Arc::new(AtomicBool::new(false)))
                .expect("das Verzeichnis entsteht");
            leser
                .verzeichnis()
                .kinder(&Innenpfad::wurzel())
                .expect("die Wurzel ist ein Ordner")
                .filter_map(|eintrag| eintrag.innen.name().map(str::to_owned))
                .collect()
        }
    })
    .expect("Faden")
    .join()
    .expect("Lesen");
    let mut gezeigt = namen.clone();
    gezeigt.sort();
    assert_eq!(gezeigt, ["etc", "gut.txt", "hinaus"]);
    let wurzel = Archivort::wurzel(zugang);
    let quellen = namen
        .iter()
        .map(|name| Ort::Archiv(wurzel.kind(name).expect("Name")))
        .collect();

    let ausgang = fahren(
        Fernauftrag::kopieren(quellen, Ort::Lokal(ziel.clone())),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    let mut draussen: Vec<String> = fs::read_dir(ordner.pfad())
        .expect("Pruefordner lesen")
        .map(|eintrag| {
            eintrag
                .expect("Eintrag")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    draussen.sort();
    assert_eq!(
        draussen,
        ["boese.zip", "ziel"],
        "ausserhalb des Ziels entstand etwas"
    );
    assert_eq!(
        fs::read(ziel.join("etc/absolut.txt")).expect("absolut"),
        b"x"
    );
    assert_eq!(fs::read(ziel.join("gut.txt")).expect("gut"), b"x");
    let hinaus = fs::symlink_metadata(ziel.join("hinaus")).expect("hinaus");
    assert!(hinaus.is_symlink(), "die Verknuepfung ist keine mehr");
    assert!(!ordner.unter("durch.txt").exists());
}

/// Der Lesezeiger eines tar.gz laeuft nur vorwaerts: zwei Eintraege in der
/// Folge ihrer Fundstelle kosten keinen Neustart, in umgekehrter Folge genau
/// einen. Wie die Maschine einen ganzen Baum ohne Neustart abarbeitet, haelt
/// `ein_baum_aus_einem_tgz_kostet_einen_durchlauf` in ihrem Pruefmodul
/// (`operation/fern.rs`).
#[test]
fn der_lesezeiger_eines_tgz_beginnt_nur_fuer_einen_eintrag_hinter_ihm_neu() {
    let ordner = Pruefordner::neu("archiv-lesezeiger");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("eins.txt"), b"eins").expect("eins");
    fs::write(quelle.join("zwei.txt"), b"zwei").expect("zwei");
    let archiv = ordner.unter("zwei.tgz");
    tar_gz_packen(&quelle, &archiv);
    let zugang = Arc::new(Archivzugang::neu(archiv, Archivformat::TarGz));

    let (vorwaerts, rueckwaerts) = faden("krk-probe-lesezeiger".to_owned(), move |pfand| {
        let zaehlen = |folge: [&str; 2]| {
            let mut leser =
                Archivleser::oeffnen(pfand, Arc::clone(&zugang), Arc::new(AtomicBool::new(false)))
                    .expect("das Verzeichnis entsteht");
            for name in folge {
                let mut gelesen = Vec::new();
                leser
                    .lesen(
                        pfand,
                        &Innenpfad::aus_archivname(name).expect("Name"),
                        |daten| {
                            gelesen.extend_from_slice(daten);
                            std::ops::ControlFlow::Continue(())
                        },
                    )
                    .expect("der Eintrag laesst sich lesen");
                assert_eq!(gelesen, name.trim_end_matches(".txt").as_bytes());
            }
            leser.neustarts()
        };
        (
            zaehlen(["eins.txt", "zwei.txt"]),
            zaehlen(["zwei.txt", "eins.txt"]),
        )
    })
    .expect("Faden")
    .join()
    .expect("Lesen");
    assert_eq!(vorwaerts, 0);
    assert_eq!(rueckwaerts, 1);
}
