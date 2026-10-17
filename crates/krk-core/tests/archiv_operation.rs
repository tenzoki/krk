//! Abnahme der Schritte 4, 6 und 7 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`: das
//! Herauskopieren aus einem Archiv in ein lokales oder fernes Ziel, ueber die
//! Maschine fuer ferne Vorgaenge mit dem Archiveintrag als Lesequelle (E11);
//! das Kopieren und Verschieben hinein und hinaus (E10, E13); Umbenennen,
//! Anlegen und endgueltiges Loeschen im Archiv (E17).
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
use krk_core::verzeichnis::Typ;
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

// ---------------------------------------------------------------------------
// Schritt 6: hinein und hinaus, Kopieren und Verschieben mit einem Archiv als
// Ziel oder als verschobener Quelle (E10, E13)
// ---------------------------------------------------------------------------

/// Was ein Archiv enthaelt, je Innenpfad die Art und bei einer Datei ihr
/// Inhalt, gelesen ueber den Leser des Kerns auf einem Faden mit Pfand.
fn inhalt(archiv: &Path) -> std::collections::BTreeMap<String, (Typ, Vec<u8>)> {
    let format = Archivformat::aus_namen(archiv.file_name().expect("Name")).expect("Endung");
    let zugang = Arc::new(Archivzugang::neu(archiv.to_path_buf(), format));
    faden("krk-probe-inhalt".to_owned(), move |pfand| {
        let mut leser = Archivleser::oeffnen(pfand, zugang, Arc::new(AtomicBool::new(false)))
            .expect("das Verzeichnis entsteht");
        let eintraege: Vec<_> = leser.verzeichnis().eintraege().to_vec();
        let mut inhalt = std::collections::BTreeMap::new();
        for eintrag in eintraege {
            let mut daten = Vec::new();
            if eintrag.typ == Typ::Datei {
                leser
                    .lesen(pfand, &eintrag.innen, |block| {
                        daten.extend_from_slice(block);
                        std::ops::ControlFlow::Continue(())
                    })
                    .expect("der Eintrag laesst sich lesen");
            }
            inhalt.insert(eintrag.innen.als_text().to_owned(), (eintrag.typ, daten));
        }
        inhalt
    })
    .expect("Faden")
    .join()
    .expect("Lesen")
}

/// Die Namen der Zwischendateien, die im Ordner `ordner` liegen geblieben
/// sind.
fn zwischennamen(ordner: &Path) -> Vec<String> {
    fs::read_dir(ordner)
        .expect("Ordner lesen")
        .map(|eintrag| {
            eintrag
                .expect("Eintrag")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.starts_with(".krk-"))
        .collect()
}

/// Ein Packer aus `gemeinsam::archivbau`: Baum unter dem ersten Pfad, Archiv
/// unter dem zweiten.
type Packer = fn(&Path, &Path);

/// Die drei beschreibbaren Formate mit ihrem Packer.
const BESCHREIBBAR: [(&str, Packer); 3] = [
    ("zip", zip_packen),
    ("tar", tar_packen),
    ("tgz", tar_gz_packen),
];

/// Ein Archiv mit dem einen Eintrag `vorher.txt`, im Format der Endung.
fn kleines_archiv(ordner: &Pruefordner, name: &str, packen: fn(&Path, &Path)) -> PathBuf {
    let quelle = ordner.ordner(&format!("{name}-quelle"));
    fs::write(quelle.join("vorher.txt"), b"vorher").expect("vorher.txt");
    let archiv = ordner.unter(name);
    packen(&quelle, &archiv);
    archiv
}

/// Ein Ordnerbaum kommt lokal und von einem Server in ein Zip, ein tar und
/// ein tar.gz und kommt wieder heraus, Byte fuer Byte und mit Rechten,
/// Aenderungszeit und der Verknuepfung als Verknuepfung; das System liest das
/// Ergebnis ohne Fehler.
#[test]
fn ein_ordnerbaum_kommt_lokal_und_fern_ins_archiv_und_wieder_heraus_byte_fuer_byte() {
    let ordner = Pruefordner::neu("archiv-hinein-baum");
    let quelle = ordner.ordner("quelle");
    baum_anlegen(&quelle);
    let erwartet = bild(&quelle);
    let server = zugang("rechner");
    for (endung, packen) in BESCHREIBBAR {
        for fern in [false, true] {
            let name = format!("baum-{fern}.{endung}");
            let archiv = kleines_archiv(&ordner, &name, packen);
            let quellen: Vec<Ort> = OBERSTE
                .iter()
                .map(|name| {
                    if fern {
                        Ort::Fern(FernerOrt {
                            zugang: Arc::clone(&server),
                            pfad: Fernpfad::aus_text(quelle.join(name).to_str().expect("UTF-8"))
                                .expect("absolut"),
                        })
                    } else {
                        Ort::Lokal(quelle.join(name))
                    }
                })
                .collect();

            let hinein = fahren(
                Fernauftrag::kopieren(quellen, Ort::Archiv(archivort(&archiv, ""))),
                Konfliktantwort::Abbrechen,
            );
            assert_eq!(hinein.bericht.abschluss, Abschluss::Fertig, "{name}");
            ohne_uebersprungene(&hinein.bericht);
            assert!(
                hinein.konflikte.is_empty(),
                "{name}: {:?}",
                hinein.konflikte
            );
            assert!(zwischennamen(ordner.pfad()).is_empty(), "{name}");
            assert!(inhalt(&archiv).contains_key("vorher.txt"), "{name}");

            let ziel = ordner.ordner(&format!("ziel-{name}"));
            let heraus = fahren(
                Fernauftrag::kopieren(im_archiv(&archiv, &OBERSTE), Ort::Lokal(ziel.clone())),
                Konfliktantwort::Abbrechen,
            );
            ohne_uebersprungene(&heraus.bericht);
            assert_eq!(bild(&ziel), erwartet, "{name}: der Baum weicht ab");

            let pruefung = if endung == "zip" {
                Command::new("/usr/bin/unzip")
                    .args(["-tq"])
                    .arg(&archiv)
                    .output()
            } else {
                Command::new("/usr/bin/tar")
                    .arg("-tf")
                    .arg(&archiv)
                    .output()
            }
            .expect("das Werkzeug des Systems laesst sich starten");
            assert!(
                pruefung.status.success(),
                "{name}: {}",
                String::from_utf8_lossy(&pruefung.stderr)
            );
        }
    }
}

/// Ein Name, der im Archiv schon steht, stellt die Konfliktfrage, und jede
/// Antwort wirkt: „Ueberschreiben“ ersetzt, „Umbenennen“ legt unter dem
/// freien Namen an, „Ueberspringen“ laesst den alten Eintrag; ein Ordner auf
/// einen Ordner fuehrt ohne Frage zusammen.
#[test]
fn ein_vorhandener_name_im_archiv_stellt_die_konfliktfrage_und_jede_antwort_wirkt() {
    let ordner = Pruefordner::neu("archiv-hinein-konflikt");
    let alt = ordner.ordner("alt");
    fs::write(alt.join("a.txt"), b"alt").expect("a.txt");
    fs::create_dir(alt.join("ordner")).expect("ordner");
    fs::write(alt.join("ordner/alt.txt"), b"alt").expect("alt.txt");
    let neu = ordner.ordner("neu");
    fs::write(neu.join("a.txt"), b"neu").expect("a.txt");
    fs::create_dir(neu.join("ordner")).expect("ordner");
    fs::write(neu.join("ordner/neu.txt"), b"neu").expect("neu.txt");

    for (endung, packen) in BESCHREIBBAR {
        for (antwort, a, b) in [
            (Konfliktantwort::Ueberschreiben, &b"neu"[..], None),
            (
                Konfliktantwort::UmbenennenIn("b.txt".to_owned()),
                &b"alt"[..],
                Some(&b"neu"[..]),
            ),
            (Konfliktantwort::Ueberspringen, &b"alt"[..], None),
        ] {
            let archiv = ordner.unter(&format!("konflikt-{antwort:?}.{endung}"));
            packen(&alt, &archiv);

            let ausgang = fahren(
                Fernauftrag::kopieren(
                    vec![
                        Ort::Lokal(neu.join("a.txt")),
                        Ort::Lokal(neu.join("ordner")),
                    ],
                    Ort::Archiv(archivort(&archiv, "")),
                ),
                antwort.clone(),
            );

            assert_eq!(
                ausgang.konflikte,
                [(
                    neu.join("a.txt"),
                    PathBuf::from(format!("{}:/a.txt", archiv.display())),
                )],
                "{endung} {antwort:?}"
            );
            let steht = inhalt(&archiv);
            assert_eq!(steht["a.txt"].1, a, "{endung} {antwort:?}");
            assert_eq!(steht.get("b.txt").map(|(_, daten)| &daten[..]), b);
            assert_eq!(steht["ordner/alt.txt"].1, b"alt");
            assert_eq!(steht["ordner/neu.txt"].1, b"neu");
        }
    }
}

/// Jede Konfliktfrage eines Hineinkopierens kommt, bevor der Umbau ein Byte
/// des neuen Archivs schreibt: die Folge der Meldungen zeigt keine Frage nach
/// dem ersten Fortschritt mit Bytes.
#[test]
fn alle_konfliktfragen_kommen_vor_dem_ersten_geschriebenen_byte() {
    let ordner = Pruefordner::neu("archiv-hinein-fragen");
    let alt = ordner.ordner("alt");
    fs::write(alt.join("eins.txt"), b"alt").expect("eins");
    fs::write(alt.join("zwei.txt"), b"alt").expect("zwei");
    fs::write(alt.join("gross.bin"), muster(4 * 1024 * 1024)).expect("gross");
    let neu = ordner.ordner("neu");
    fs::write(neu.join("eins.txt"), b"neu").expect("eins");
    fs::write(neu.join("zwei.txt"), b"neu").expect("zwei");
    fs::write(neu.join("drei.bin"), muster(4 * 1024 * 1024)).expect("drei");
    let archiv = ordner.unter("fragen.tar");
    tar_packen(&alt, &archiv);

    let lauf = starten(
        Fernauftrag::kopieren(
            ["drei.bin", "eins.txt", "zwei.txt"]
                .iter()
                .map(|name| Ort::Lokal(neu.join(name)))
                .collect(),
            Ort::Archiv(archivort(&archiv, "")),
        ),
        Arc::new(OhnePapierkorb),
    );
    let mut folge = Vec::new();
    loop {
        match lauf
            .meldungen()
            .recv_timeout(Duration::from_secs(60))
            .expect("der Lauf meldet sich nicht")
        {
            Meldung::Fortschritt(stand) if stand.bytes > 0 => folge.push("byte"),
            Meldung::Fortschritt(_) | Meldung::Uebersprungen(_) => {}
            Meldung::Konflikt { antwort, .. } => {
                folge.push("frage");
                antwort
                    .send(Konfliktentscheid::einmal(Konfliktantwort::Ueberschreiben))
                    .expect("der Lauf wartet nicht mehr");
            }
            Meldung::Fertig(bericht) => {
                ohne_uebersprungene(&bericht);
                break;
            }
        }
    }
    lauf.warten();
    let fragen = folge.iter().filter(|meldung| **meldung == "frage").count();
    assert_eq!(fragen, 2, "{folge:?}");
    let erste_bytes = folge.iter().position(|meldung| *meldung == "byte");
    let letzte_frage = folge.iter().rposition(|meldung| *meldung == "frage");
    assert!(
        erste_bytes.is_none_or(|bytes| letzte_frage.is_some_and(|frage| frage < bytes)),
        "eine Frage kam nach dem ersten Byte: {folge:?}"
    );
    assert_eq!(inhalt(&archiv)["eins.txt"].1, b"neu");
}

/// Ein Verschieben ins Archiv entfernt allein die Quellen, die vollstaendig
/// angekommen sind, und erst, nachdem das Archiv ersetzt ist: eine
/// uebersprungene Datei und ein Ordner, dessen Kind uebersprungen wurde,
/// bleiben stehen, und ein gescheiterter Umbau entfernt gar nichts.
#[test]
fn verschieben_ins_archiv_entfernt_allein_vollstaendig_angekommene_quellen_und_erst_nach_dem_ersetzen()
 {
    let ordner = Pruefordner::neu("archiv-hinein-verschieben");
    let alt = ordner.ordner("alt");
    fs::write(alt.join("x.txt"), b"alt").expect("x");
    fs::create_dir(alt.join("teil")).expect("teil");
    fs::write(alt.join("teil/z.txt"), b"alt").expect("z");

    for (endung, packen) in BESCHREIBBAR {
        let quelle = ordner.ordner(&format!("quelle-{endung}"));
        fs::write(quelle.join("x.txt"), b"neu").expect("x");
        fs::write(quelle.join("y.txt"), b"neu").expect("y");
        fs::create_dir_all(quelle.join("ganz/unter")).expect("ganz");
        fs::write(quelle.join("ganz/unter/w.txt"), b"neu").expect("w");
        fs::create_dir(quelle.join("teil")).expect("teil");
        fs::write(quelle.join("teil/z.txt"), b"neu").expect("z");
        fs::write(quelle.join("teil/v.txt"), b"neu").expect("v");
        let ablage = ordner.ordner(&format!("ablage-{endung}"));
        let archiv = ablage.join(format!("ziel.{endung}"));
        packen(&alt, &archiv);
        let quellen = || -> Vec<Ort> {
            ["x.txt", "y.txt", "ganz", "teil"]
                .iter()
                .map(|name| Ort::Lokal(quelle.join(name)))
                .collect()
        };

        // Ein Umbau, der nicht ersetzen kann, entfernt nichts.
        fs::set_permissions(&ablage, fs::Permissions::from_mode(0o555)).expect("sperren");
        let gescheitert = fahren(
            Fernauftrag::verschieben(quellen(), Ort::Archiv(archivort(&archiv, ""))),
            Konfliktantwort::Ueberspringen,
        );
        fs::set_permissions(&ablage, fs::Permissions::from_mode(0o755)).expect("entsperren");
        assert!(!gescheitert.bericht.uebersprungen.is_empty(), "{endung}");
        for name in ["x.txt", "y.txt", "ganz/unter/w.txt", "teil/v.txt"] {
            assert!(quelle.join(name).exists(), "{endung}: {name} ist fort");
        }

        let ausgang = fahren(
            Fernauftrag::verschieben(quellen(), Ort::Archiv(archivort(&archiv, ""))),
            Konfliktantwort::Ueberspringen,
        );
        assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig, "{endung}");
        let steht = inhalt(&archiv);
        assert_eq!(steht["x.txt"].1, b"alt", "{endung}");
        assert_eq!(steht["y.txt"].1, b"neu", "{endung}");
        assert_eq!(steht["ganz/unter/w.txt"].1, b"neu", "{endung}");
        assert_eq!(steht["teil/z.txt"].1, b"alt", "{endung}");
        assert_eq!(steht["teil/v.txt"].1, b"neu", "{endung}");
        assert!(quelle.join("x.txt").exists(), "{endung}: x.txt ist fort");
        assert!(!quelle.join("y.txt").exists(), "{endung}: y.txt blieb");
        assert!(!quelle.join("ganz").exists(), "{endung}: ganz blieb");
        assert!(
            quelle.join("teil/v.txt").exists() && quelle.join("teil/z.txt").exists(),
            "{endung}: teil ist angefasst"
        );
        assert!(
            ausgang
                .bericht
                .uebersprungen
                .iter()
                .any(|eintrag| eintrag.pfad == quelle.join("teil")
                    && eintrag.grund == text(Text::VorgangNichtVollstaendigKopiert)),
            "{endung}: {:?}",
            ausgang.bericht.uebersprungen
        );
    }
}

/// Ein Verschieben aus einem Archiv entfernt dort allein die Eintraege, die
/// vollstaendig am Ziel angekommen sind, ueber einen Umbau des Archivs nach
/// dem Kopieren.
#[test]
fn verschieben_aus_dem_archiv_entfernt_dort_allein_vollstaendig_angekommene_eintraege() {
    let ordner = Pruefordner::neu("archiv-heraus-verschieben");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("a.txt"), b"a").expect("a");
    fs::write(quelle.join("b.txt"), b"aus dem Archiv").expect("b");
    fs::create_dir_all(quelle.join("ordner/unter")).expect("ordner");
    fs::write(quelle.join("ordner/unter/c.txt"), b"c").expect("c");
    fs::write(quelle.join("bleibt.txt"), b"bleibt").expect("bleibt");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("heraus.{endung}"));
        packen(&quelle, &archiv);
        let ziel = ordner.ordner(&format!("ziel-{endung}"));
        fs::write(ziel.join("b.txt"), b"am Ziel").expect("b am Ziel");

        let ausgang = fahren(
            Fernauftrag::verschieben(
                im_archiv(&archiv, &["a.txt", "b.txt", "ordner"]),
                Ort::Lokal(ziel.clone()),
            ),
            Konfliktantwort::Ueberspringen,
        );

        assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert_eq!(fs::read(ziel.join("a.txt")).expect("a"), b"a");
        assert_eq!(fs::read(ziel.join("b.txt")).expect("b"), b"am Ziel");
        assert_eq!(fs::read(ziel.join("ordner/unter/c.txt")).expect("c"), b"c");
        let steht = inhalt(&archiv);
        let namen: Vec<&str> = steht.keys().map(String::as_str).collect();
        assert_eq!(namen, ["b.txt", "bleibt.txt"], "{endung}");
        assert!(zwischennamen(ordner.pfad()).is_empty(), "{endung}");
    }
}

/// Ein Verschieben im selben Archiv haengt Namen um und kopiert keine Daten:
/// der Bericht nennt keine geschriebenen Bytes, und der Zip-Eintrag behaelt
/// seine gepackten Bytes und seine Pruefsumme.
#[test]
fn verschieben_im_selben_archiv_haengt_namen_um_ohne_datenkopie() {
    let ordner = Pruefordner::neu("archiv-umhaengen");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("gross.bin"), muster(2 * 1024 * 1024)).expect("gross");
    fs::create_dir_all(quelle.join("ziel/tief")).expect("ziel");
    fs::write(quelle.join("ziel/tief/da.txt"), b"da").expect("da");
    let archiv = ordner.unter("umhaengen.zip");
    zip_packen(&quelle, &archiv);
    let roh = |name: &str| {
        let mut kiste = zip::ZipArchive::new(File::open(&archiv).expect("Zip")).expect("lesbar");
        let stelle = kiste.index_for_name(name).expect("Eintrag");
        let eintrag = kiste.by_index_raw(stelle).expect("Eintrag");
        (
            eintrag.crc32(),
            eintrag.compressed_size(),
            eintrag.compression(),
        )
    };
    let vorher = roh("gross.bin");

    let ausgang = fahren(
        Fernauftrag::verschieben(
            im_archiv(&archiv, &["gross.bin"]),
            Ort::Archiv(archivort(&archiv, "ziel/tief")),
        ),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    assert_eq!(ausgang.bericht.eintraege, 1);
    assert_eq!(ausgang.bericht.bytes, 0, "es wurden Daten kopiert");
    assert_eq!(roh("ziel/tief/gross.bin"), vorher);
    let steht = inhalt(&archiv);
    assert!(!steht.contains_key("gross.bin"));
    assert_eq!(steht["ziel/tief/gross.bin"].1, muster(2 * 1024 * 1024));
}

/// Ein Verschieben zwischen zwei Archiven schreibt erst das Ziel und dann die
/// Quelle neu; scheitert der zweite Umbau, stehen die Eintraege in beiden
/// Archiven und der Bericht nennt sie, und gelingt er, nur im Ziel.
#[test]
fn verschieben_zwischen_zwei_archiven_laesst_bei_scheitern_des_zweiten_umbaus_die_eintraege_doppelt_und_nicht_nirgends()
 {
    let ordner = Pruefordner::neu("archiv-zwischen-zweien");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("eins.txt"), b"eins").expect("eins");
    fs::write(quelle.join("zwei.txt"), b"zwei").expect("zwei");
    let leer = ordner.ordner("leer");
    fs::write(leer.join("da.txt"), b"da").expect("da");

    for gelingt in [false, true] {
        let ablage = ordner.ordner(&format!("ablage-{gelingt}"));
        let von = ablage.join("von.zip");
        zip_packen(&quelle, &von);
        let nach = ordner.unter(&format!("nach-{gelingt}.tgz"));
        tar_gz_packen(&leer, &nach);
        let vorher = fs::read(&von).expect("von");

        if !gelingt {
            fs::set_permissions(&ablage, fs::Permissions::from_mode(0o555)).expect("sperren");
        }
        let ausgang = fahren(
            Fernauftrag::verschieben(
                im_archiv(&von, &["eins.txt"]),
                Ort::Archiv(archivort(&nach, "")),
            ),
            Konfliktantwort::Abbrechen,
        );
        fs::set_permissions(&ablage, fs::Permissions::from_mode(0o755)).expect("entsperren");

        assert_eq!(inhalt(&nach)["eins.txt"].1, b"eins", "{gelingt}");
        let bezeichnung = PathBuf::from(format!("{}:/eins.txt", von.display()));
        if gelingt {
            ohne_uebersprungene(&ausgang.bericht);
            let steht = inhalt(&von);
            assert!(!steht.contains_key("eins.txt"));
            assert!(steht.contains_key("zwei.txt"));
        } else {
            assert_eq!(
                fs::read(&von).expect("von"),
                vorher,
                "die Quelle ist angefasst"
            );
            assert!(
                ausgang.bericht.uebersprungen.iter().any(|eintrag| {
                    eintrag.pfad == bezeichnung
                        && krk_core::operation::fehlerart(&eintrag.grund).is_none()
                }),
                "{:?}",
                ausgang.bericht.uebersprungen
            );
            assert!(zwischennamen(&ablage).is_empty());
        }
    }
}

/// Ein Abbruch mitten im Hineinkopieren einer grossen Datei laesst das
/// Archiv Byte fuer Byte stehen und ohne Zwischennamen daneben, und beim
/// Verschieben steht die Quelle noch.
#[test]
fn ein_abbruch_beim_hineinkopieren_laesst_das_archiv_unveraendert_und_die_quellen_stehen() {
    let ordner = Pruefordner::neu("archiv-hinein-abbruch");
    let quelle = ordner.ordner("quelle");
    let gross = quelle.join("gross.bin");
    fs::write(&gross, muster(24 * 1024 * 1024)).expect("gross");
    for (endung, packen) in [("zip", zip_packen as fn(&Path, &Path)), ("tar", tar_packen)] {
        let ablage = ordner.ordner(&format!("ablage-{endung}"));
        let archiv = kleines_archiv(&ordner, &format!("abbruch.{endung}"), packen);
        let archiv = {
            let verschoben = ablage.join(archiv.file_name().expect("Name"));
            fs::rename(&archiv, &verschoben).expect("umlegen");
            verschoben
        };
        let vorher = fs::read(&archiv).expect("Archiv");

        let bericht = mitten_abbrechen(Fernauftrag::verschieben(
            vec![Ort::Lokal(gross.clone())],
            Ort::Archiv(archivort(&archiv, "")),
        ));

        assert_eq!(bericht.abschluss, Abschluss::Abgebrochen, "{endung}");
        assert_eq!(fs::read(&archiv).expect("Archiv"), vorher, "{endung}");
        assert!(zwischennamen(&ablage).is_empty(), "{endung}");
        assert!(gross.exists(), "{endung}: die Quelle ist fort");
    }
}

/// Ein gz-Archiv als Ziel nimmt nichts an: jede Quelle steht mit
/// `Text::ArchivNurLesbar` im Bericht, und die Datei bleibt, wie sie war. Die
/// Oberflaeche weist denselben Auftrag schon vor dem Start ab
/// (`Anwendungsdelegierter::uebertragen`).
#[test]
fn ein_gz_archiv_als_ziel_wird_vor_dem_start_abgewiesen() {
    let ordner = Pruefordner::neu("archiv-hinein-gz");
    let gepackt = gzip_glied(b"inhalt");
    let archiv = ordner.datei("bericht.txt.gz", &gepackt);
    let quelle = ordner.datei("neu.txt", b"neu");

    let ausgang = fahren(
        Fernauftrag::kopieren(
            vec![Ort::Lokal(quelle.clone())],
            Ort::Archiv(archivort(&archiv, "")),
        ),
        Konfliktantwort::Abbrechen,
    );

    let gemeldet: Vec<(PathBuf, String)> = ausgang
        .bericht
        .uebersprungen
        .iter()
        .map(|eintrag| (eintrag.pfad.clone(), eintrag.grund.clone()))
        .collect();
    assert_eq!(gemeldet, [(quelle, text(Text::ArchivNurLesbar).to_owned())]);
    assert_eq!(fs::read(&archiv).expect("gz"), gepackt);
}

// ---------------------------------------------------------------------------
// Schritt 7: Umbenennen, neuer Ordner, neue Datei und endgueltiges Loeschen
// im Archiv (E10, E17)
// ---------------------------------------------------------------------------

/// Ein Name im Archiv wird umbenannt, eine Datei wie ein Ordner samt
/// Unterbaum; ein vergebener Name wird nicht ueberschrieben, scheitert mit
/// dem Grund, den lokal `AlreadyExists` traegt, und stellt keine Frage.
#[test]
fn ein_name_im_archiv_wird_umbenannt_und_ein_vergebener_nicht_ueberschrieben() {
    let ordner = Pruefordner::neu("archiv-umbenennen");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("datei.txt"), b"datei").expect("datei");
    fs::write(quelle.join("belegt.txt"), b"belegt").expect("belegt");
    fs::create_dir_all(quelle.join("ordner/unter")).expect("ordner");
    fs::write(quelle.join("ordner/unter/x.txt"), b"x").expect("x");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("umbenennen.{endung}"));
        packen(&quelle, &archiv);
        let ort = |innen: &str| Ort::Archiv(archivort(&archiv, innen));

        let datei = fahren(
            Fernauftrag::umbenennen(ort("datei.txt"), "neu.txt"),
            Konfliktantwort::Abbrechen,
        );
        let ordnername = fahren(
            Fernauftrag::umbenennen(ort("ordner"), "anders"),
            Konfliktantwort::Abbrechen,
        );
        for ausgang in [&datei, &ordnername] {
            assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig, "{endung}");
            assert_eq!(ausgang.bericht.eintraege, 1, "{endung}");
            ohne_uebersprungene(&ausgang.bericht);
        }
        let steht = inhalt(&archiv);
        let namen: Vec<&str> = steht.keys().map(String::as_str).collect();
        assert_eq!(
            namen,
            [
                "anders",
                "anders/unter",
                "anders/unter/x.txt",
                "belegt.txt",
                "neu.txt"
            ],
            "{endung}"
        );
        assert_eq!(steht["neu.txt"].1, b"datei");

        let vorher = fs::read(&archiv).expect("Archiv");
        let vergeben = fahren(
            Fernauftrag::umbenennen(ort("neu.txt"), "belegt.txt"),
            Konfliktantwort::Ueberschreiben,
        );
        assert!(vergeben.konflikte.is_empty(), "{endung}: eine Frage kam");
        assert_eq!(vergeben.bericht.eintraege, 0);
        let grund = &vergeben
            .bericht
            .uebersprungen
            .first()
            .expect("ein Grund")
            .grund;
        assert_eq!(
            krk_core::operation::fehlerart(grund),
            Some(std::io::ErrorKind::AlreadyExists),
            "{endung}: {grund}"
        );
        assert_eq!(fs::read(&archiv).expect("Archiv"), vorher, "{endung}");
    }
}

/// Ein neuer Ordner und eine neue, leere Datei entstehen im Archiv, an der
/// Wurzel wie in einem Unterordner; ein vergebener Name scheitert mit dem
/// Grund von `AlreadyExists` und laesst das Archiv stehen.
#[test]
fn ein_ordner_und_eine_leere_datei_entstehen_im_archiv_und_ein_vergebener_name_scheitert() {
    let ordner = Pruefordner::neu("archiv-anlegen");
    let quelle = ordner.ordner("quelle");
    fs::create_dir(quelle.join("ordner")).expect("ordner");
    fs::write(quelle.join("ordner/da.txt"), b"da").expect("da");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("anlegen.{endung}"));
        packen(&quelle, &archiv);
        let ort = |innen: &str| Ort::Archiv(archivort(&archiv, innen));

        for (wo, name, typ) in [
            ("", "neu", krk_core::operation::Anlegetyp::Ordner),
            ("ordner", "leer.txt", krk_core::operation::Anlegetyp::Datei),
        ] {
            let ausgang = fahren(
                Fernauftrag::anlegen(ort(wo), name, typ),
                Konfliktantwort::Abbrechen,
            );
            assert_eq!(ausgang.bericht.eintraege, 1, "{endung} {name}");
            ohne_uebersprungene(&ausgang.bericht);
        }
        let steht = inhalt(&archiv);
        assert_eq!(steht["neu"].0, Typ::Ordner, "{endung}");
        assert_eq!(
            steht["ordner/leer.txt"],
            (Typ::Datei, Vec::new()),
            "{endung}"
        );
        assert_eq!(steht["ordner/da.txt"].1, b"da");

        let vorher = fs::read(&archiv).expect("Archiv");
        for (wo, name, typ) in [
            ("", "neu", krk_core::operation::Anlegetyp::Datei),
            ("ordner", "da.txt", krk_core::operation::Anlegetyp::Ordner),
        ] {
            let vergeben = fahren(
                Fernauftrag::anlegen(ort(wo), name, typ),
                Konfliktantwort::Ueberschreiben,
            );
            assert!(vergeben.konflikte.is_empty(), "{endung} {name}");
            let grund = &vergeben
                .bericht
                .uebersprungen
                .first()
                .expect("ein Grund")
                .grund;
            assert_eq!(
                krk_core::operation::fehlerart(grund),
                Some(std::io::ErrorKind::AlreadyExists),
                "{endung} {name}: {grund}"
            );
        }
        assert_eq!(fs::read(&archiv).expect("Archiv"), vorher, "{endung}");
    }
}

/// Ein Baum wird im Archiv endgueltig entfernt, samt Unterbaum; ein Abbruch
/// mitten im Umbau laesst alles stehen, Byte fuer Byte und ohne Zwischennamen.
#[test]
fn ein_baum_wird_im_archiv_endgueltig_entfernt_und_ein_abbruch_laesst_alles_stehen() {
    let ordner = Pruefordner::neu("archiv-loeschen");
    let quelle = ordner.ordner("quelle");
    fs::create_dir_all(quelle.join("weg/unter")).expect("weg");
    fs::write(quelle.join("weg/unter/x.txt"), b"x").expect("x");
    fs::write(quelle.join("auch.txt"), b"auch").expect("auch");
    fs::write(quelle.join("bleibt.txt"), b"bleibt").expect("bleibt");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("loeschen.{endung}"));
        packen(&quelle, &archiv);
        let ausgang = fahren(
            Fernauftrag::loeschen(im_archiv(&archiv, &["weg", "auch.txt"])),
            Konfliktantwort::Abbrechen,
        );
        assert_eq!(ausgang.bericht.abschluss, Abschluss::Fertig, "{endung}");
        assert_eq!(ausgang.bericht.eintraege, 2, "{endung}");
        ohne_uebersprungene(&ausgang.bericht);
        let namen: Vec<String> = inhalt(&archiv).into_keys().collect();
        assert_eq!(namen, ["bleibt.txt"], "{endung}");
    }

    // Der Abbruch braucht ein Archiv, dessen Neuschreiben dauert; ein Zip mit
    // gespeichertem Eintrag und ein tar schreiben ihre Bytes ungepackt.
    let gross = ordner.ordner("gross");
    fs::write(gross.join("gross.bin"), muster(24 * 1024 * 1024)).expect("gross");
    fs::write(gross.join("weg.txt"), b"weg").expect("weg");
    for (endung, packen) in [
        ("zip", gespeichert_packen as fn(&Path, &Path)),
        ("tar", tar_packen),
    ] {
        let ablage = ordner.ordner(&format!("ablage-{endung}"));
        let archiv = ablage.join(format!("gross.{endung}"));
        packen(&gross, &archiv);
        let vorher = fs::read(&archiv).expect("Archiv");
        let bericht = mitten_abbrechen(Fernauftrag::loeschen(im_archiv(&archiv, &["weg.txt"])));
        assert_eq!(bericht.abschluss, Abschluss::Abgebrochen, "{endung}");
        assert_eq!(fs::read(&archiv).expect("Archiv"), vorher, "{endung}");
        assert!(zwischennamen(&ablage).is_empty(), "{endung}");
    }
}

/// Packt die Dateien unmittelbar unter `wurzel` als Zip mit gespeicherten,
/// also ungepackten Eintraegen: ein Umbau kopiert dann so viele Bytes, wie die
/// Dateien gross sind, und ein Abbruch trifft ihn mittendrin.
fn gespeichert_packen(wurzel: &Path, ziel: &Path) {
    let mut schreiber = ZipWriter::new(File::create(ziel).expect("Zip anlegen"));
    for (name, pfad) in gemeinsam::archivbau::baum(wurzel) {
        schreiber
            .start_file(
                name,
                SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
            )
            .expect("Eintrag");
        schreiber
            .write_all(&fs::read(&pfad).expect("Datei lesen"))
            .expect("Inhalt");
    }
    schreiber.finish().expect("Zip abschliessen");
}

/// Eine Datei ueber 4 GiB kommt in ein Zip: der Umbau gibt dem neuen Eintrag
/// die ZIP64-Form, `/usr/bin/unzip -t` liest das Ergebnis, und das
/// Inhaltsverzeichnis nennt die volle Groesse. Ohne `large_file` blieb das
/// Archiv mit „Large file option has not been set“ unveraendert (Defekt
/// `261017-1923-eine-datei-ueber-4-gib-laesst-sich-in-kein-zip-kopieren.md`).
#[test]
#[ignore = "packt 4 GiB Nullen und dauert im Profil dev Minuten"]
fn eine_datei_ueber_vier_gib_kommt_in_ein_zip() {
    let ordner = Pruefordner::neu("archiv-gross");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("klein.txt"), b"k").expect("klein");
    let archiv = ordner.unter("gross.zip");
    zip_packen(&quelle, &archiv);
    let gross = ordner.unter("gross.bin");
    File::create(&gross)
        .expect("anlegen")
        .set_len((1 << 32) + 4096)
        .expect("Groesse");

    let ausgang = fahren(
        Fernauftrag::kopieren(vec![Ort::Lokal(gross)], Ort::Archiv(archivort(&archiv, ""))),
        Konfliktantwort::Abbrechen,
    );

    ohne_uebersprungene(&ausgang.bericht);
    let pruefung = Command::new("/usr/bin/unzip")
        .arg("-tqq")
        .arg(&archiv)
        .status()
        .expect("unzip");
    assert!(pruefung.success(), "unzip -t liest das Zip nicht");
    let mut kiste = zip::ZipArchive::new(File::open(&archiv).expect("Zip")).expect("lesbar");
    let stelle = kiste.index_for_name("gross.bin").expect("Eintrag");
    assert_eq!(
        kiste.by_index(stelle).expect("Eintrag").size(),
        (1 << 32) + 4096
    );
}

/// Wie [`fahren`], und `bei_frage` beantwortet jede Konfliktfrage; was es
/// dabei tut, geschieht, waehrend der Lauf auf die Antwort wartet.
fn fahren_mit(auftrag: Fernauftrag, mut bei_frage: impl FnMut() -> Konfliktantwort) -> Bericht {
    let lauf = starten(auftrag, Arc::new(OhnePapierkorb));
    let bericht = loop {
        match lauf
            .meldungen()
            .recv_timeout(Duration::from_secs(60))
            .expect("der Lauf meldet sich nicht")
        {
            Meldung::Fortschritt(_) | Meldung::Uebersprungen(_) => {}
            Meldung::Konflikt { antwort: weg, .. } => {
                weg.send(Konfliktentscheid::einmal(bei_frage()))
                    .expect("der Lauf wartet nicht mehr");
            }
            Meldung::Fertig(bericht) => break bericht,
        }
    };
    lauf.warten();
    bericht
}

/// Ob der Bericht `pfad` mit genau dem Grund `grund` nennt.
fn nennt(bericht: &Bericht, pfad: &Path, grund: &str) -> bool {
    bericht
        .uebersprungen
        .iter()
        .any(|eintrag| eintrag.pfad == pfad && eintrag.grund == grund)
}

/// Eine lokale Quelle, die das Zielarchiv selbst ist oder es enthaelt, wird
/// vor dem Umbau abgewiesen, beim Verschieben wie beim Kopieren: danach
/// stehen Archiv und Ordner Byte fuer Byte, und der Bericht nennt die Quelle
/// mit Grund. Bis dahin ersetzte der Umbau das Archiv, und das Verschieben
/// entfernte danach die „angekommene“ Quelle, also das neue Archiv oder den
/// Ordner, der es traegt (Defekt
/// `261017-1923-verschieben-einer-lokalen-quelle-die-das-zielarchiv-ist-oder-enthaelt-loescht-das-archiv.md`).
#[test]
fn eine_lokale_quelle_die_das_zielarchiv_ist_oder_enthaelt_wird_abgewiesen() {
    let ordner = Pruefordner::neu("archiv-in-sich-selbst");
    let inhalt_wurzel = ordner.ordner("inhalt");
    fs::write(inhalt_wurzel.join("wert.txt"), b"wertvoll").expect("wert");

    for (endung, packen) in BESCHREIBBAR {
        let projekt = ordner.ordner(&format!("projekt-{endung}"));
        fs::write(projekt.join("x.txt"), b"x").expect("x");
        let archiv = projekt.join(format!("a.{endung}"));
        packen(&inhalt_wurzel, &archiv);
        let vorher = fs::read(&archiv).expect("vorher");

        for verschieben in [true, false] {
            for quelle in [archiv.clone(), projekt.clone()] {
                let quellen = vec![Ort::Lokal(quelle.clone())];
                let ziel = Ort::Archiv(archivort(&archiv, ""));
                let auftrag = if verschieben {
                    Fernauftrag::verschieben(quellen, ziel)
                } else {
                    Fernauftrag::kopieren(quellen, ziel)
                };
                let ausgang = fahren(auftrag, Konfliktantwort::Abbrechen);
                let fall = format!("{endung}, verschieben {verschieben}, {}", quelle.display());
                assert_eq!(fs::read(&archiv).expect("nachher"), vorher, "{fall}");
                assert_eq!(fs::read(projekt.join("x.txt")).expect("x"), b"x", "{fall}");
                assert!(
                    nennt(
                        &ausgang.bericht,
                        &quelle,
                        text(Text::VorgangZielLiegtInDerQuelle)
                    ),
                    "{fall}: {:?}",
                    ausgang.bericht.uebersprungen
                );
                assert!(zwischennamen(&projekt).is_empty(), "{fall}");
            }
        }
    }
}

/// Zwei Schreibweisen derselben Archivdatei, eine davon ueber eine
/// Verknuepfung, sind dasselbe Archiv: ein Ordner, der in seinen eigenen
/// Unterordner verschoben wird, wird abgewiesen wie im selben Archiv, und
/// kein Eintrag fehlt. Bis dahin entschied der Pfadtext, kopierte `a` nach
/// `a/b/a` und entfernte danach `a` samt der Kopie.
#[test]
fn zwei_schreibweisen_desselben_archivs_sind_ein_archiv() {
    let ordner = Pruefordner::neu("archiv-zwei-schreibweisen");
    let inhalt_wurzel = ordner.ordner("inhalt");
    fs::create_dir_all(inhalt_wurzel.join("a/b")).expect("a/b");
    fs::write(inhalt_wurzel.join("a/datei.txt"), b"datei").expect("datei");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("echt.{endung}"));
        packen(&inhalt_wurzel, &archiv);
        let verweis = ordner.verknuepfung(&format!("verweis.{endung}"), &archiv);
        let vorher = fs::read(&archiv).expect("vorher");

        let ausgang = fahren(
            Fernauftrag::verschieben(
                vec![Ort::Archiv(archivort(&verweis, "a"))],
                Ort::Archiv(archivort(&archiv, "a/b")),
            ),
            Konfliktantwort::Abbrechen,
        );

        assert_eq!(fs::read(&archiv).expect("nachher"), vorher, "{endung}");
        let steht = inhalt(&archiv);
        assert_eq!(steht["a/datei.txt"].1, b"datei", "{endung}");
        assert!(steht.contains_key("a/b"), "{endung}");
        assert!(
            !ausgang.bericht.uebersprungen.is_empty(),
            "{endung}: die Abweisung steht nicht im Bericht"
        );
    }
}

/// Das Entfernen nach einem Verschieben aus einem Archiv geschieht allein an
/// der Datei, aus der kopiert wurde: ersetzt ein anderes Programm das Archiv
/// waehrend der Konfliktfrage eines zweiten Eintrags, bleibt das ersetzende
/// Archiv Byte fuer Byte stehen, und der Bericht nennt den kopierten Eintrag
/// als „kopiert, aber in der Quelle geblieben“ (Defekt
/// `261017-1923-das-entfernen-nach-dem-verschieben-aus-einem-archiv-prueft-nicht-ob-das-archiv-seit-dem-kopieren-dasselbe-ist.md`).
#[test]
fn das_entfernen_nach_dem_verschieben_trifft_allein_das_archiv_aus_dem_kopiert_wurde() {
    let ordner = Pruefordner::neu("archiv-ersetzt-vor-dem-entfernen");
    let quelle = ordner.ordner("quelle");
    fs::write(quelle.join("a.txt"), b"alt-a").expect("a");
    fs::write(quelle.join("b.txt"), b"alt-b").expect("b");
    let ersatz_wurzel = ordner.ordner("ersatz");
    fs::write(ersatz_wurzel.join("a.txt"), b"ersatz-a").expect("a");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("heraus.{endung}"));
        packen(&quelle, &archiv);
        let ersatz = ordner.unter(&format!("ersatz.{endung}"));
        packen(&ersatz_wurzel, &ersatz);
        let ersatz_bytes = fs::read(&ersatz).expect("ersatz");
        let ziel = ordner.ordner(&format!("ziel-{endung}"));
        fs::write(ziel.join("b.txt"), b"am Ziel").expect("b am Ziel");

        let bericht = fahren_mit(
            Fernauftrag::verschieben(
                im_archiv(&archiv, &["a.txt", "b.txt"]),
                Ort::Lokal(ziel.clone()),
            ),
            || {
                // Ein anderes Programm ersetzt das Archiv, wie `ersetzen` es
                // selbst tut: neben dem Original schreiben, dann `rename`.
                let daneben = ordner.unter(&format!("daneben.{endung}"));
                fs::copy(&ersatz, &daneben).expect("daneben");
                fs::rename(&daneben, &archiv).expect("ersetzen");
                Konfliktantwort::Ueberspringen
            },
        );

        assert_eq!(
            fs::read(ziel.join("a.txt")).expect("a"),
            b"alt-a",
            "{endung}"
        );
        assert_eq!(
            fs::read(&archiv).expect("nachher"),
            ersatz_bytes,
            "{endung}: das ersetzende Archiv ist angefasst"
        );
        let a_im_archiv = PathBuf::from(format!("{}:/a.txt", archiv.display()));
        assert!(
            nennt(
                &bericht,
                &a_im_archiv,
                &satz(
                    Text::VorgangKopiertAberInQuelleGeblieben,
                    &[("grund", &text(Text::ArchivVonAussenVeraendert))]
                )
            ),
            "{endung}: {:?}",
            bericht.uebersprungen
        );
    }
}

/// Ein Verschieben ins Archiv entfernt aus einem lokalen Quellordner allein,
/// was beim Zusammenstellen darin stand: eine Datei, die waehrend einer
/// Konfliktfrage hinzukommt, bleibt stehen, mit ihr der Ordner, und der
/// Bericht nennt ihn; was angekommen ist, ist fort. Bis dahin entfernte
/// `baum_entfernen` den ganzen Quellordner samt der neuen Datei (Defekt
/// `261017-1923-verschieben-ins-archiv-entfernt-den-ganzen-quellbaum-auch-was-waehrend-konfliktfragen-und-umbau-hinzukam.md`).
#[test]
fn verschieben_ins_archiv_laesst_stehen_was_waehrend_der_konfliktfrage_hinzukam() {
    let ordner = Pruefordner::neu("archiv-hinein-hinzugekommen");
    let alt = ordner.ordner("alt");
    fs::create_dir(alt.join("quelle")).expect("quelle");
    fs::write(alt.join("quelle/x.txt"), b"alt-x").expect("x");

    for (endung, packen) in BESCHREIBBAR {
        let archiv = ordner.unter(&format!("ziel.{endung}"));
        packen(&alt, &archiv);
        let herkunft = ordner.ordner(&format!("herkunft-{endung}"));
        let quelle = herkunft.join("quelle");
        fs::create_dir_all(quelle.join("unter")).expect("unter");
        fs::write(quelle.join("x.txt"), b"neu-x").expect("x");
        fs::write(quelle.join("y.txt"), b"neu-y").expect("y");
        fs::write(quelle.join("unter/z.txt"), b"neu-z").expect("z");
        let hinzu = quelle.join("hinzu.txt");

        let bericht = fahren_mit(
            Fernauftrag::verschieben(
                vec![Ort::Lokal(quelle.clone())],
                Ort::Archiv(archivort(&archiv, "")),
            ),
            || {
                fs::write(&hinzu, b"waehrend der Frage").expect("hinzu");
                Konfliktantwort::Ueberschreiben
            },
        );

        assert_eq!(bericht.abschluss, Abschluss::Fertig, "{endung}");
        let steht = inhalt(&archiv);
        assert_eq!(steht["quelle/x.txt"].1, b"neu-x", "{endung}");
        assert_eq!(steht["quelle/unter/z.txt"].1, b"neu-z", "{endung}");
        assert!(!steht.contains_key("quelle/hinzu.txt"), "{endung}");
        assert_eq!(
            fs::read(&hinzu).expect("die hinzugekommene Datei ist fort"),
            b"waehrend der Frage",
            "{endung}"
        );
        for name in ["x.txt", "y.txt", "unter"] {
            assert!(!quelle.join(name).exists(), "{endung}: {name} blieb");
        }
        let vorsatz = satz(Text::VorgangKopiertAberInQuelleGeblieben, &[("grund", &"")]);
        assert!(
            bericht
                .uebersprungen
                .iter()
                .any(|eintrag| eintrag.pfad == quelle && eintrag.grund.starts_with(&vorsatz)),
            "{endung}: {:?}",
            bericht.uebersprungen
        );
    }
}
