//! Abnahme von Schritt 1 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`: Archive lesen
//! im Kern, ohne Oberflaeche.
//!
//! Jede Probe baut ihre Archive selbst in einem Pruefordner, mit den Kisten,
//! mit denen KRK sie liest (`gemeinsam::archivbau`); das verschluesselte Zip
//! kommt von `/usr/bin/zip -P`, weil die Kiste ohne ihr Merkmal fuer
//! Verschluesselung keines schreibt. Gelesen wird ueber
//! [`Archivzugang::verzeichnis`] in einem Faden mit Pfand, wie im Betrieb.

mod gemeinsam;

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, UNIX_EPOCH};

use gemeinsam::archivbau::{baum, gzip_glied, tar_gz_packen, tar_packen, zip_packen};
use gemeinsam::{Pruefordner, mit_zeitschranke};
use krk_core::archiv::{
    Archiveintrag, Archivformat, Archivzugang, Fundstelle, Inhaltsverzeichnis, Innenpfad,
    Lesbarkeit, Verzeichnisfehler,
};
use krk_core::fern::faden;
use krk_core::verzeichnis::Typ;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Liest das Verzeichnis eines Zugangs in einem Faden mit Pfand.
fn lesen_mit(
    zugang: &Arc<Archivzugang>,
    abbruch: &Arc<AtomicBool>,
) -> Result<Arc<Inhaltsverzeichnis>, Verzeichnisfehler> {
    let zugang = Arc::clone(zugang);
    let abbruch = Arc::clone(abbruch);
    faden("krk-archivprobe".into(), move |pfand| {
        zugang.verzeichnis(pfand, &abbruch)
    })
    .expect("ein Faden laesst sich starten")
    .join()
    .expect("der Lesefaden geraet nicht in Panik")
}

/// Ein Zugang zu `pfad` im Format seiner Endung.
fn zugang(pfad: &Path) -> Arc<Archivzugang> {
    let format = Archivformat::aus_namen(pfad.file_name().expect("Name"))
        .expect("die Probe gibt ihrem Archiv eine Endung aus der Tafel");
    Arc::new(Archivzugang::neu(pfad.to_path_buf(), format))
}

/// Liest das Verzeichnis eines Archivs ohne Abbruch.
fn lesen(pfad: &Path) -> Result<Arc<Inhaltsverzeichnis>, Verzeichnisfehler> {
    lesen_mit(&zugang(pfad), &Arc::new(AtomicBool::new(false)))
}

fn innen(text: &str) -> Innenpfad {
    Innenpfad::aus_archivname(text).expect("ein sicherer Name")
}

/// Die Namen unter einem Ordner, geordnet.
fn namen_unter(verzeichnis: &Inhaltsverzeichnis, ordner: &str) -> Vec<String> {
    let ordner = if ordner.is_empty() {
        Innenpfad::wurzel()
    } else {
        innen(ordner)
    };
    let mut namen: Vec<String> = verzeichnis
        .kinder(&ordner)
        .expect("der Ordner steht im Verzeichnis")
        .map(|eintrag| {
            eintrag
                .innen
                .name()
                .expect("kein Eintrag ist die Wurzel")
                .to_owned()
        })
        .collect();
    namen.sort();
    namen
}

/// Alle Pfade des Verzeichnisses, geordnet.
fn alle_pfade(verzeichnis: &Inhaltsverzeichnis) -> Vec<String> {
    let mut pfade: Vec<String> = verzeichnis
        .eintraege()
        .iter()
        .map(|eintrag| eintrag.innen.als_text().to_owned())
        .collect();
    pfade.sort();
    pfade
}

#[test]
fn jede_endung_aus_der_tafel_ergibt_ihr_format_und_keine_andere_ein_archiv() {
    let tafel = [
        ("a.zip", Some(Archivformat::Zip)),
        ("A.ZIP", Some(Archivformat::Zip)),
        ("a.tar", Some(Archivformat::Tar)),
        ("a.TaR", Some(Archivformat::Tar)),
        ("a.tar.gz", Some(Archivformat::TarGz)),
        ("a.TAR.GZ", Some(Archivformat::TarGz)),
        ("a.tgz", Some(Archivformat::TarGz)),
        ("a.TGZ", Some(Archivformat::TarGz)),
        ("bericht.txt.gz", Some(Archivformat::Gz)),
        ("a.Gz", Some(Archivformat::Gz)),
        (".gz", Some(Archivformat::Gz)),
        ("a.sqlite", Some(Archivformat::Sqlite)),
        ("a.SQLite3", Some(Archivformat::Sqlite)),
        ("a.db", Some(Archivformat::Sqlite)),
        ("a.DB3", Some(Archivformat::Sqlite)),
        ("a.db.gz", Some(Archivformat::Gz)),
        ("a.db-wal", None),
        ("a.jar", None),
        ("a.docx", None),
        ("a.epub", None),
        ("a.tar.bz2", None),
        ("a.tar.xz", None),
        ("a.7z", None),
        ("a.rar", None),
        ("a.zip.txt", None),
        ("zip", None),
        ("agz", None),
        ("a", None),
    ];
    for (name, erwartet) in tafel {
        assert_eq!(
            Archivformat::aus_namen(OsStr::new(name)),
            erwartet,
            "die Endung von {name}"
        );
    }
    assert!(Archivformat::Zip.beschreibbar());
    assert!(Archivformat::Tar.beschreibbar());
    assert!(Archivformat::TarGz.beschreibbar());
    assert!(!Archivformat::Gz.beschreibbar());
    assert!(!Archivformat::Sqlite.beschreibbar());
}

#[test]
fn der_innenpfad_rechnet_lexikalisch_und_kennt_die_wurzel() {
    let pfad = innen("a/b/c.txt");
    assert_eq!(pfad.name(), Some("c.txt"));
    assert_eq!(pfad.aufwaerts(), Some(innen("a/b")));
    assert_eq!(innen("a").aufwaerts(), Some(Innenpfad::wurzel()));
    assert_eq!(Innenpfad::wurzel().aufwaerts(), None);
    assert_eq!(Innenpfad::wurzel().verbinden("x"), Some(innen("x")));
    assert_eq!(innen("a").verbinden("x"), Some(innen("a/x")));
    assert_eq!(innen("a").verbinden(".."), None);
    assert_eq!(innen("a").verbinden("x/y"), None);
    assert!(pfad.liegt_unter(&innen("a")));
    assert!(pfad.liegt_unter(&Innenpfad::wurzel()));
    assert!(!innen("ab/c").liegt_unter(&innen("a")));
    assert!(!innen("a").liegt_unter(&innen("a")));
    assert_eq!(pfad.to_string(), "/a/b/c.txt");
    assert_eq!(Innenpfad::wurzel().to_string(), "/");

    // Was abgestreift wird und was eine Wurzel ist.
    assert_eq!(
        Innenpfad::aus_archivname("/etc/passwd"),
        Some(innen("etc/passwd"))
    );
    assert_eq!(Innenpfad::aus_archivname("./a/b/"), Some(innen("a/b")));
    assert_eq!(Innenpfad::aus_archivname("./"), Some(Innenpfad::wurzel()));
    assert_eq!(Innenpfad::aus_archivname("/"), Some(Innenpfad::wurzel()));
    assert_eq!(Innenpfad::aus_archivname("a\\b"), Some(innen("a\\b")));
    for unsicher in ["../x", "a/../../x", "a/./b", "a//b", "a\0b", ".."] {
        assert_eq!(Innenpfad::aus_archivname(unsicher), None, "{unsicher:?}");
    }
}

/// Legt den Baum der Gleichheitsprobe an: Ordner, Unterordner, Dateien mit
/// eigenen Rechten und Zeiten, ein leerer Ordner und eine Verknuepfung.
fn baum_anlegen(ordner: &Pruefordner) -> std::path::PathBuf {
    let wurzel = ordner.ordner("baum");
    fs::create_dir_all(wurzel.join("a/b")).unwrap();
    fs::create_dir_all(wurzel.join("leer")).unwrap();
    fs::write(wurzel.join("eins.txt"), b"eins").unwrap();
    fs::write(wurzel.join("a/zwei.txt"), vec![b'z'; 5000]).unwrap();
    fs::write(wurzel.join("a/b/skript.sh"), b"#!/bin/sh\n").unwrap();
    fs::set_permissions(
        wurzel.join("a/b/skript.sh"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    fs::set_permissions(wurzel.join("eins.txt"), fs::Permissions::from_mode(0o600)).unwrap();
    std::os::unix::fs::symlink("eins.txt", wurzel.join("verweis")).unwrap();
    let zeiten = [
        ("eins.txt", 1_500_000_000),
        ("a/zwei.txt", 1_600_000_000),
        ("a/b/skript.sh", 1_700_000_000),
        ("a/b", 1_650_000_000),
        ("a", 1_550_000_000),
        ("leer", 1_450_000_000),
    ];
    for (name, sekunden) in zeiten {
        let zeit = UNIX_EPOCH + Duration::from_secs(sekunden);
        File::open(wurzel.join(name))
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(zeit).set_accessed(zeit))
            .unwrap();
    }
    wurzel
}

#[test]
fn zip_tar_und_tgz_zeigen_denselben_baum_wie_der_ordner_aus_dem_sie_gepackt_sind() {
    let ordner = Pruefordner::neu("archiv-gleich");
    let wurzel = baum_anlegen(&ordner);
    let archive = [
        ordner.unter("baum.zip"),
        ordner.unter("baum.tar"),
        ordner.unter("baum.tar.gz"),
    ];
    zip_packen(&wurzel, &archive[0]);
    tar_packen(&wurzel, &archive[1]);
    tar_gz_packen(&wurzel, &archive[2]);

    let quelle = baum(&wurzel);
    for archiv in &archive {
        let verzeichnis = lesen(archiv).expect("das Archiv laesst sich lesen");
        assert_eq!(verzeichnis.ausgelassen(), 0, "{}", archiv.display());
        assert!(!verzeichnis.verschluesselt());
        assert_eq!(
            verzeichnis.eintraege().len(),
            quelle.len(),
            "{}: {:?}",
            archiv.display(),
            alle_pfade(&verzeichnis)
        );
        for (name, pfad) in &quelle {
            let angaben = fs::symlink_metadata(pfad).unwrap();
            let eintrag = verzeichnis
                .eintrag(&innen(name))
                .unwrap_or_else(|| panic!("{} fehlt in {}", name, archiv.display()));
            let typ = if angaben.is_symlink() {
                Typ::Verknuepfung
            } else if angaben.is_dir() {
                Typ::Ordner
            } else {
                Typ::Datei
            };
            let wo = format!("{name} in {}", archiv.display());
            assert_eq!(eintrag.typ, typ, "{wo}");
            if typ == Typ::Datei {
                assert_eq!(eintrag.groesse, angaben.len(), "{wo}");
            }
            if typ != Typ::Verknuepfung {
                assert_eq!(
                    eintrag.rechte,
                    Some(angaben.permissions().mode() & 0o7777),
                    "{wo}"
                );
            }
            let erwartet = angaben
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let gelesen = eintrag
                .geaendert
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            assert_eq!(gelesen, erwartet, "{wo}");
            assert_eq!(eintrag.lesbarkeit, Lesbarkeit::Lesbar, "{wo}");
            assert!(!eintrag.ist_erschlossen(), "{wo}");
        }
        assert_eq!(
            namen_unter(&verzeichnis, ""),
            ["a", "eins.txt", "leer", "verweis"],
            "{}",
            archiv.display()
        );
        assert_eq!(namen_unter(&verzeichnis, "a"), ["b", "zwei.txt"]);
        assert_eq!(namen_unter(&verzeichnis, "a/b"), ["skript.sh"]);
        assert!(namen_unter(&verzeichnis, "leer").is_empty());
        assert!(
            verzeichnis.kinder(&innen("eins.txt")).is_none(),
            "eine Datei hat keine Kinder"
        );
    }
}

#[test]
fn eine_gz_datei_zeigt_einen_eintrag_ohne_endung_mit_der_gezaehlten_groesse() {
    let ordner = Pruefordner::neu("archiv-gz");
    let eins = vec![b'a'; 70_000];
    let zwei = vec![b'b'; 30_000];

    let einfach = ordner.datei("bericht.txt.gz", gzip_glied(&eins));
    let verzeichnis = lesen(&einfach).expect("eine gz-Datei laesst sich lesen");
    assert_eq!(alle_pfade(&verzeichnis), ["bericht.txt"]);
    let eintrag = verzeichnis.eintrag(&innen("bericht.txt")).unwrap();
    assert_eq!(eintrag.typ, Typ::Datei);
    assert_eq!(eintrag.groesse, 70_000);
    assert_eq!(eintrag.fundstelle, Fundstelle::GzGanz);
    assert_eq!(
        eintrag.geaendert,
        fs::metadata(&einfach).unwrap().modified().unwrap(),
        "der Eintrag traegt das Datum der Archivdatei"
    );

    // Zwei Glieder hintereinander: gezaehlt wird ueber beide, nicht das Feld
    // ISIZE des letzten.
    let mut zweifach = gzip_glied(&eins);
    zweifach.extend(gzip_glied(&zwei));
    let zweifach = ordner.datei("DOPPELT.GZ", zweifach);
    let verzeichnis = lesen(&zweifach).unwrap();
    assert_eq!(alle_pfade(&verzeichnis), ["DOPPELT"]);
    assert_eq!(verzeichnis.eintraege()[0].groesse, 100_000);

    // Ein leerer Rest heisst `inhalt`.
    let ohne_stamm = ordner.datei(".gz", gzip_glied(b"x"));
    assert_eq!(alle_pfade(&lesen(&ohne_stamm).unwrap()), ["inhalt"]);
}

#[test]
fn ordner_ohne_eigenen_eintrag_werden_erschlossen() {
    let ordner = Pruefordner::neu("archiv-erschlossen");
    let archiv = ordner.unter("flach.zip");
    let mut schreiber = ZipWriter::new(File::create(&archiv).unwrap());
    schreiber
        .start_file("a/b/c.txt", SimpleFileOptions::default())
        .unwrap();
    schreiber.write_all(b"c").unwrap();
    schreiber
        .start_file("a/d.txt", SimpleFileOptions::default())
        .unwrap();
    schreiber.finish().unwrap();

    let verzeichnis = lesen(&archiv).unwrap();
    assert_eq!(
        alle_pfade(&verzeichnis),
        ["a", "a/b", "a/b/c.txt", "a/d.txt"]
    );
    let archivzeit = fs::metadata(&archiv).unwrap().modified().unwrap();
    for erschlossen in ["a", "a/b"] {
        let eintrag = verzeichnis.eintrag(&innen(erschlossen)).unwrap();
        assert_eq!(eintrag.typ, Typ::Ordner);
        assert!(eintrag.ist_erschlossen(), "{erschlossen}");
        assert_eq!(eintrag.geaendert, archivzeit, "{erschlossen}");
    }
    assert!(
        !verzeichnis
            .eintrag(&innen("a/d.txt"))
            .unwrap()
            .ist_erschlossen()
    );
    assert_eq!(namen_unter(&verzeichnis, ""), ["a"]);
    assert_eq!(namen_unter(&verzeichnis, "a"), ["b", "d.txt"]);
    assert_eq!(verzeichnis.ausgelassen(), 0);
}

/// Schreibt einen tar-Kopf mit rohem Namen, an jeder Pruefung der Kiste
/// vorbei, wie ein fremdes Werkzeug ihn schreiben koennte.
fn roher_tar_eintrag<W: Write>(
    bauer: &mut tar::Builder<W>,
    name: &[u8],
    art: tar::EntryType,
    verweis: Option<&[u8]>,
    inhalt: &[u8],
) {
    let mut kopf = tar::Header::new_old();
    kopf.as_old_mut().name[..name.len()].copy_from_slice(name);
    if let Some(verweis) = verweis {
        kopf.as_old_mut().linkname[..verweis.len()].copy_from_slice(verweis);
    }
    kopf.set_entry_type(art);
    kopf.set_mode(0o644);
    kopf.set_mtime(1_600_000_000);
    kopf.set_size(inhalt.len() as u64);
    kopf.set_cksum();
    bauer.append(&kopf, inhalt).unwrap();
}

#[test]
fn unsichere_namen_erscheinen_nicht_und_werden_gezaehlt() {
    let ordner = Pruefordner::neu("archiv-unsicher");

    // Zip: Namen, die aus dem Ordner herausfuehren, und einer mit NUL.
    let zip = ordner.unter("unsicher.zip");
    let mut schreiber = ZipWriter::new(File::create(&zip).unwrap());
    for name in ["../x", "a/../../x", "/etc/passwd", "a\0b", "gut.txt"] {
        schreiber
            .start_file(name, SimpleFileOptions::default())
            .unwrap();
        schreiber.write_all(b"x").unwrap();
    }
    schreiber.finish().unwrap();
    let verzeichnis = lesen(&zip).unwrap();
    assert_eq!(alle_pfade(&verzeichnis), ["etc", "etc/passwd", "gut.txt"]);
    assert_eq!(verzeichnis.ausgelassen(), 3);

    // tar: dieselben Wege, ein Name, der kein UTF-8 ist, und der Weg ueber
    // eine Verknuepfung im selben Archiv.
    let tar = ordner.unter("unsicher.tar");
    let mut bauer = tar::Builder::new(File::create(&tar).unwrap());
    roher_tar_eintrag(&mut bauer, b"../x", tar::EntryType::Regular, None, b"x");
    roher_tar_eintrag(
        &mut bauer,
        b"a/../../x",
        tar::EntryType::Regular,
        None,
        b"x",
    );
    roher_tar_eintrag(
        &mut bauer,
        b"/etc/passwd",
        tar::EntryType::Regular,
        None,
        b"x",
    );
    roher_tar_eintrag(
        &mut bauer,
        b"kaputt\xff",
        tar::EntryType::Regular,
        None,
        b"x",
    );
    roher_tar_eintrag(
        &mut bauer,
        b"hinaus",
        tar::EntryType::Symlink,
        Some(b"/etc"),
        b"",
    );
    roher_tar_eintrag(
        &mut bauer,
        b"hinaus/passwd",
        tar::EntryType::Regular,
        None,
        b"x",
    );
    roher_tar_eintrag(
        &mut bauer,
        b"./gut.txt",
        tar::EntryType::Regular,
        None,
        b"x",
    );
    bauer.into_inner().unwrap();
    let verzeichnis = lesen(&tar).unwrap();
    assert_eq!(
        alle_pfade(&verzeichnis),
        ["etc", "etc/passwd", "gut.txt", "hinaus"]
    );
    assert_eq!(
        verzeichnis.eintrag(&innen("hinaus")).unwrap().typ,
        Typ::Verknuepfung,
        "die Verknuepfung selbst bleibt sichtbar"
    );
    assert_eq!(verzeichnis.ausgelassen(), 4);
}

#[test]
fn doppelte_namen_zeigen_den_spaeteren() {
    let ordner = Pruefordner::neu("archiv-doppelt");

    // tar: zweimal derselbe Name, und ein Name, der zugleich Datei und Ordner
    // ist.
    let tar = ordner.unter("doppelt.tar");
    let mut bauer = tar::Builder::new(File::create(&tar).unwrap());
    roher_tar_eintrag(
        &mut bauer,
        b"a.txt",
        tar::EntryType::Regular,
        None,
        b"frueh",
    );
    roher_tar_eintrag(
        &mut bauer,
        b"a.txt",
        tar::EntryType::Regular,
        None,
        b"spaeter!",
    );
    roher_tar_eintrag(&mut bauer, b"d/", tar::EntryType::Directory, None, b"");
    roher_tar_eintrag(&mut bauer, b"d", tar::EntryType::Regular, None, b"datei");
    roher_tar_eintrag(&mut bauer, b"e", tar::EntryType::Regular, None, b"datei");
    roher_tar_eintrag(&mut bauer, b"e/f.txt", tar::EntryType::Regular, None, b"f");
    bauer.into_inner().unwrap();
    let verzeichnis = lesen(&tar).unwrap();
    assert_eq!(alle_pfade(&verzeichnis), ["a.txt", "d", "e", "e/f.txt"]);
    assert_eq!(verzeichnis.eintrag(&innen("a.txt")).unwrap().groesse, 8);
    assert_eq!(verzeichnis.eintrag(&innen("d")).unwrap().typ, Typ::Ordner);
    assert_eq!(verzeichnis.eintrag(&innen("e")).unwrap().typ, Typ::Ordner);
    assert_eq!(
        verzeichnis.ausgelassen(),
        3,
        "das fruehere a.txt und die zwei Dateien, die einem Ordner weichen"
    );

    // Zip: die Kiste schreibt keinen Namen zweimal. Die Probe schreibt zwei
    // gleich lange Namen und macht danach in beiden Koepfen aus dem zweiten den
    // ersten; die Pruefsumme gilt den Daten und bleibt gueltig.
    let zip = ordner.unter("doppelt.zip");
    let mut schreiber = ZipWriter::new(File::create(&zip).unwrap());
    schreiber
        .start_file("x.txt", SimpleFileOptions::default())
        .unwrap();
    schreiber.write_all(b"frueh").unwrap();
    schreiber
        .start_file("y.txt", SimpleFileOptions::default())
        .unwrap();
    schreiber.write_all(b"spaeter!").unwrap();
    schreiber.finish().unwrap();
    let mut bytes = fs::read(&zip).unwrap();
    let mut ersetzt = 0;
    for stelle in 0..bytes.len() - 5 {
        if &bytes[stelle..stelle + 5] == b"y.txt" {
            bytes[stelle] = b'x';
            ersetzt += 1;
        }
    }
    assert_eq!(ersetzt, 2, "lokaler und zentraler Kopf");
    fs::write(&zip, bytes).unwrap();
    let verzeichnis = lesen(&zip).unwrap();
    assert_eq!(alle_pfade(&verzeichnis), ["x.txt"]);
    assert_eq!(verzeichnis.eintrag(&innen("x.txt")).unwrap().groesse, 8);
    assert_eq!(verzeichnis.ausgelassen(), 1);
}

#[test]
fn ein_verschluesselter_eintrag_wird_gezeigt_und_als_verschluesselt_gekennzeichnet() {
    let ordner = Pruefordner::neu("archiv-geheim");
    ordner.datei("geheim.txt", b"streng geheim");
    ordner.datei("offen.txt", b"offen");
    let stand = Command::new("/usr/bin/zip")
        .current_dir(ordner.pfad())
        .args(["-q", "-P", "kennwort", "geheim.zip", "geheim.txt"])
        .status()
        .expect("/usr/bin/zip laesst sich starten");
    assert!(stand.success());
    let stand = Command::new("/usr/bin/zip")
        .current_dir(ordner.pfad())
        .args(["-q", "geheim.zip", "offen.txt"])
        .status()
        .expect("/usr/bin/zip laesst sich starten");
    assert!(stand.success());

    let verzeichnis = lesen(&ordner.unter("geheim.zip")).unwrap();
    assert_eq!(alle_pfade(&verzeichnis), ["geheim.txt", "offen.txt"]);
    let geheim = verzeichnis.eintrag(&innen("geheim.txt")).unwrap();
    assert_eq!(geheim.lesbarkeit, Lesbarkeit::Verschluesselt);
    assert_eq!(geheim.groesse, 13);
    assert_eq!(
        verzeichnis.eintrag(&innen("offen.txt")).unwrap().lesbarkeit,
        Lesbarkeit::Lesbar
    );
    assert!(verzeichnis.verschluesselt());
}

#[test]
fn ein_unbekanntes_verfahren_und_sonderdateien_im_tar_werden_gekennzeichnet() {
    let ordner = Pruefordner::neu("archiv-lesbarkeit");
    let tar = ordner.unter("sonder.tar");
    let mut bauer = tar::Builder::new(File::create(&tar).unwrap());
    roher_tar_eintrag(&mut bauer, b"ziel.txt", tar::EntryType::Regular, None, b"z");
    roher_tar_eintrag(
        &mut bauer,
        b"hart",
        tar::EntryType::Link,
        Some(b"ziel.txt"),
        b"",
    );
    roher_tar_eintrag(&mut bauer, b"roehre", tar::EntryType::Fifo, None, b"");
    bauer.into_inner().unwrap();
    let verzeichnis = lesen(&tar).unwrap();
    let lesbarkeit = |name: &str| {
        verzeichnis
            .eintrag(&innen(name))
            .unwrap()
            .lesbarkeit
            .clone()
    };
    assert_eq!(lesbarkeit("ziel.txt"), Lesbarkeit::Lesbar);
    assert_eq!(lesbarkeit("hart"), Lesbarkeit::HarterVerweis);
    assert_eq!(lesbarkeit("roehre"), Lesbarkeit::Sonderdatei);
    assert!(!verzeichnis.verschluesselt());

    // Ein Zip-Eintrag, gepackt mit einem Verfahren, das `deflate-flate2` nicht
    // entpackt: die Probe schreibt ihn gespeichert und setzt danach in beiden
    // Koepfen die Nummer des Verfahrens auf bzip2 (12).
    let zip = ordner.unter("bzip.zip");
    let mut schreiber = ZipWriter::new(File::create(&zip).unwrap());
    schreiber
        .start_file(
            "b.txt",
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored),
        )
        .unwrap();
    schreiber.write_all(b"b").unwrap();
    schreiber.finish().unwrap();
    let mut bytes = fs::read(&zip).unwrap();
    for (kennung, versatz) in [
        ([0x50, 0x4b, 0x03, 0x04], 8),
        ([0x50, 0x4b, 0x01, 0x02], 10),
    ] {
        let stelle = bytes
            .windows(4)
            .position(|fenster| fenster == kennung)
            .expect("der Kopf steht im Archiv");
        bytes[stelle + versatz] = 12;
        bytes[stelle + versatz + 1] = 0;
    }
    fs::write(&zip, bytes).unwrap();
    let verzeichnis = lesen(&zip).unwrap();
    match &verzeichnis.eintrag(&innen("b.txt")).unwrap().lesbarkeit {
        Lesbarkeit::Verfahren(name) => assert_eq!(name, "bzip2"),
        andere => panic!("erwartet ein unbekanntes Verfahren, gelesen {andere:?}"),
    }
}

#[test]
fn ein_zip_mit_mehr_als_65535_eintraegen_wird_gelesen() {
    let ordner = Pruefordner::neu("archiv-zip64");
    let archiv = ordner.unter("viele.zip");
    let mut schreiber = ZipWriter::new(std::io::BufWriter::new(File::create(&archiv).unwrap()));
    let wahl = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let zahl = 65_540;
    for nummer in 0..zahl {
        schreiber.start_file(format!("d/{nummer}"), wahl).unwrap();
    }
    schreiber.finish().unwrap();

    let verzeichnis = lesen(&archiv).unwrap();
    assert_eq!(
        verzeichnis.eintraege().len(),
        zahl + 1,
        "die Dateien und `d`"
    );
    assert_eq!(
        verzeichnis.kinder(&innen("d")).unwrap().count(),
        zahl,
        "alle stehen unter d"
    );
    assert_eq!(verzeichnis.ausgelassen(), 0);
}

#[test]
fn eine_datei_mit_falscher_endung_scheitert_mit_grund_und_nicht_still() {
    let ordner = Pruefordner::neu("archiv-falsch");
    let text = b"das ist kein Archiv, sondern ein Satz.".repeat(30);
    for name in ["text.zip", "text.tar", "text.tar.gz", "text.gz", "leer.tgz"] {
        let inhalt: &[u8] = if name.starts_with("leer") { b"" } else { &text };
        let pfad = ordner.datei(name, inhalt);
        match lesen(&pfad) {
            Err(Verzeichnisfehler::Ungueltig(grund)) => {
                assert!(!grund.is_empty(), "{name}: der Grund ist leer");
            }
            andere => panic!("{name}: erwartet ungueltig, gelesen {andere:?}"),
        }
    }

    // Ein Zip unter der Endung eines tar.
    let echt = ordner.unter("echt.zip");
    let mut schreiber = ZipWriter::new(File::create(&echt).unwrap());
    schreiber
        .start_file("x.txt", SimpleFileOptions::default())
        .unwrap();
    schreiber.write_all(&text).unwrap();
    schreiber.finish().unwrap();
    let falsch = ordner.unter("zip-als.tar");
    fs::copy(&echt, &falsch).unwrap();
    assert!(
        matches!(lesen(&falsch), Err(Verzeichnisfehler::Ungueltig(_))),
        "ein Zip wird nicht still als tar gelesen"
    );

    // Ein Ordner und eine Roehre unter einer Archivendung: keine Datei, und
    // die Roehre haelt den Leser nicht an.
    let als_ordner = ordner.ordner("ordner.zip");
    assert!(matches!(
        lesen(&als_ordner),
        Err(Verzeichnisfehler::KeineDatei)
    ));
    let roehre = ordner.roehre("roehre.tar.gz");
    let ergebnis = mit_zeitschranke(
        "das Lesen einer Roehre",
        Duration::from_secs(5),
        move || matches!(lesen(&roehre), Err(Verzeichnisfehler::KeineDatei)),
    );
    assert!(ergebnis, "eine Roehre ist keine Datei");

    // Und eine fehlende Datei ist ein Lesefehler mit dem Grund des Systems.
    assert!(matches!(
        lesen(&ordner.unter("fehlt.zip")),
        Err(Verzeichnisfehler::Lesen(_))
    ));
}

#[test]
fn das_verzeichnis_entsteht_nach_einer_aenderung_der_datei_neu_und_sonst_nicht() {
    let ordner = Pruefordner::neu("archiv-kennung");
    let wurzel = ordner.ordner("quelle");
    fs::write(wurzel.join("a.txt"), b"a").unwrap();
    let archiv = ordner.unter("k.tar.gz");
    tar_gz_packen(&wurzel, &archiv);

    let zugang = zugang(&archiv);
    let ohne_abbruch = Arc::new(AtomicBool::new(false));
    assert!(
        zugang.gespeichertes().is_none(),
        "ein neuer Zugang hat nichts gelesen"
    );
    let erstes = lesen_mit(&zugang, &ohne_abbruch).unwrap();
    let zweites = lesen_mit(&zugang, &ohne_abbruch).unwrap();
    assert!(
        Arc::ptr_eq(&erstes, &zweites),
        "eine unveraenderte Datei wird nicht zweimal gelesen"
    );

    fs::write(wurzel.join("b.txt"), b"b").unwrap();
    // Neu geschrieben unter einem anderen Namen und dann umbenannt: ein neuer
    // Inode, so wie ein Umbau die Datei ersetzt.
    let neu = ordner.unter("k-neu.tar.gz");
    tar_gz_packen(&wurzel, &neu);
    fs::rename(&neu, &archiv).unwrap();
    let drittes = lesen_mit(&zugang, &ohne_abbruch).unwrap();
    assert!(!Arc::ptr_eq(&erstes, &drittes));
    assert_eq!(alle_pfade(&drittes), ["a.txt", "b.txt"]);

    // Und an Ort und Stelle beschrieben, mit derselben Groesse: die
    // Aenderungszeit in Nanosekunden traegt die Unterscheidung.
    let mut bytes = fs::read(&archiv).unwrap();
    let laenge = bytes.len();
    bytes[laenge - 1] ^= 0;
    std::thread::sleep(Duration::from_millis(5));
    fs::write(&archiv, &bytes).unwrap();
    let viertes = lesen_mit(&zugang, &ohne_abbruch).unwrap();
    assert!(!Arc::ptr_eq(&drittes, &viertes));
    assert!(Arc::ptr_eq(
        &viertes,
        &zugang
            .gespeichertes()
            .expect("das Verzeichnis ist gespeichert")
    ));
}

#[test]
fn ein_abgebrochener_aufbau_hinterlaesst_kein_verzeichnis() {
    let ordner = Pruefordner::neu("archiv-abbruch");

    // Vor dem Beginn gesetzt: kein Byte wird gelesen.
    let klein = ordner.datei("klein.gz", gzip_glied(b"klein"));
    let zugang_klein = zugang(&klein);
    let gesetzt = Arc::new(AtomicBool::new(true));
    assert!(matches!(
        lesen_mit(&zugang_klein, &gesetzt),
        Err(Verzeichnisfehler::Abgebrochen)
    ));
    assert!(zugang_klein.gespeichertes().is_none());

    // Mitten im Lauf gesetzt: 512 Glieder zu je 1 MiB Nullen, entpackt also
    // 512 MiB. Gefragt wird je MiB, also endet der Lauf, lange bevor er durch
    // ist, und hinterlaesst kein Verzeichnis.
    let glied = gzip_glied(&vec![0u8; 1 << 20]);
    let mut gross = Vec::with_capacity(glied.len() * 512);
    for _ in 0..512 {
        gross.extend_from_slice(&glied);
    }
    let gross = ordner.datei("gross.gz", gross);
    let zugang_gross = zugang(&gross);
    let abbruch = Arc::new(AtomicBool::new(false));
    let setzer = {
        let abbruch = Arc::clone(&abbruch);
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(30));
            abbruch.store(true, Ordering::Release);
        })
    };
    let beginn = Instant::now();
    let ergebnis = lesen_mit(&zugang_gross, &abbruch);
    let dauer = beginn.elapsed();
    setzer.join().unwrap();
    assert!(
        matches!(ergebnis, Err(Verzeichnisfehler::Abgebrochen)),
        "erwartet Abbruch nach {dauer:?}, gelesen {ergebnis:?}"
    );
    assert!(zugang_gross.gespeichertes().is_none());

    // Ohne Abbruch entsteht es danach vollstaendig.
    let verzeichnis =
        lesen_mit(&zugang_klein, &Arc::new(AtomicBool::new(false))).expect("ohne Abbruch");
    assert_eq!(alle_pfade(&verzeichnis), ["klein"]);
}

/// Misst das Lesen eines Zip mit 100 000 Eintraegen und schaetzt den Speicher
/// je Eintrag. Ignoriert, weil sie misst und nichts zusagt; gefahren mit
/// `cargo test --release -p krk-core --test archiv -- --ignored --nocapture`.
#[test]
#[ignore = "Messung fuer den Bericht von Schritt 1, keine Zusage"]
fn hunderttausend_zip_eintraege_messen() {
    let ordner = Pruefordner::neu("archiv-messung");
    let archiv = ordner.unter("hunderttausend.zip");
    let mut schreiber = ZipWriter::new(std::io::BufWriter::new(File::create(&archiv).unwrap()));
    let wahl = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for nummer in 0..100_000 {
        schreiber
            .start_file(
                format!("ordner-{}/datei-{nummer:06}.txt", nummer % 100),
                wahl,
            )
            .unwrap();
        schreiber.write_all(b"x").unwrap();
    }
    schreiber.finish().unwrap();

    let mut zeiten = Vec::new();
    for _ in 0..3 {
        let beginn = Instant::now();
        let verzeichnis = lesen(&archiv).unwrap();
        zeiten.push(beginn.elapsed());
        assert_eq!(verzeichnis.eintraege().len(), 100_100);
    }
    let verzeichnis = lesen(&archiv).unwrap();
    let namen: usize = verzeichnis
        .eintraege()
        .iter()
        .map(|eintrag| eintrag.innen.als_text().len())
        .sum();
    let je_eintrag = std::mem::size_of::<Archiveintrag>()
        // Name im Eintrag und seine Abschrift als Schluessel der Namenstafel.
        + 2 * namen / verzeichnis.eintraege().len()
        // Schluessel, Wert und Steuerbyte der Namenstafel.
        + std::mem::size_of::<Innenpfad>() + std::mem::size_of::<usize>() + 1
        // Platz in der Kinderliste.
        + std::mem::size_of::<usize>();
    println!(
        "100 000 Zip-Eintraege: {zeiten:?}; Archiveintrag {} Bytes, \
         mittlerer Name {} Bytes, geschaetzt {je_eintrag} Bytes je Eintrag",
        std::mem::size_of::<Archiveintrag>(),
        namen / verzeichnis.eintraege().len()
    );
}

/// Wie gross der lange Name der Bombe ist: weit ueber der Grenze von 16 MiB,
/// und so gross, dass die Kiste ohne Grenze ein Gigabyte zuteilte.
const BOMBENNAME: u64 = 1 << 30;

/// Der Kopf eines GNU-Langnamens mit `groesse` Bytes Daten.
fn langnamenkopf(groesse: u64) -> tar::Header {
    let mut kopf = tar::Header::new_gnu();
    kopf.as_old_mut().name[..13].copy_from_slice(b"././@LongLink");
    kopf.set_entry_type(tar::EntryType::GNULongName);
    kopf.set_mode(0o644);
    kopf.set_size(groesse);
    kopf.set_cksum();
    kopf
}

/// Der Rest eines tar nach dem langen Namen: ein gewoehnlicher Eintrag und
/// das Ende.
fn tar_rest() -> Vec<u8> {
    let mut rest = Vec::new();
    {
        let mut bauer = tar::Builder::new(&mut rest);
        let mut kopf = tar::Header::new_gnu();
        kopf.set_size(4);
        kopf.set_mode(0o644);
        kopf.set_cksum();
        bauer
            .append_data(&mut kopf, "da.txt", &b"inhalt"[..4])
            .expect("Eintrag");
        bauer.finish().expect("Ende");
    }
    rest
}

/// Ein tar und ein tar.gz, deren erster Kopf einen langen Namen von
/// [`BOMBENNAME`] Nullen ankuendigt. Das tar steht duenn auf der Platte, das
/// tar.gz besteht aus vielen gzip-Gliedern zu je 1 MiB Nullen und ist rund ein
/// Megabyte gross.
fn bomben_bauen(ordner: &Path) {
    let tar_pfad = ordner.join("bombe.tar");
    let mut datei = File::create(&tar_pfad).expect("tar anlegen");
    datei
        .write_all(langnamenkopf(BOMBENNAME).as_bytes())
        .expect("Kopf");
    datei.set_len(512 + BOMBENNAME).expect("Luecke");
    use std::io::{Seek, SeekFrom};
    datei.seek(SeekFrom::End(0)).expect("ans Ende");
    datei.write_all(&tar_rest()).expect("Rest");
    drop(datei);

    let mut tgz = File::create(ordner.join("bombe.tgz")).expect("tgz anlegen");
    tgz.write_all(&gzip_glied(langnamenkopf(BOMBENNAME).as_bytes()))
        .expect("Kopf");
    let mebibyte = gzip_glied(&vec![0u8; 1 << 20]);
    for _ in 0..(BOMBENNAME >> 20) {
        tgz.write_all(&mebibyte).expect("Nullen");
    }
    tgz.write_all(&gzip_glied(&tar_rest())).expect("Rest");
}

/// Die Bomben lesen und halten, dass jede als ungueltig endet, mit dem
/// Wortlaut der Grenze.
fn bomben_lesen(ordner: &Path) {
    for name in ["bombe.tar", "bombe.tgz"] {
        match lesen(&ordner.join(name)) {
            Err(Verzeichnisfehler::Ungueltig(wortlaut)) => {
                assert_eq!(wortlaut, "tar: extension header too large", "{name}")
            }
            anders => panic!("{name}: der lange Name ueber der Grenze ergibt {anders:?}"),
        }
    }
}

/// Ein langer Name ueber der Grenze macht ein tar und ein tar.gz ungueltig,
/// statt in den Speicher gelesen zu werden, und zwar ohne mehr als einige
/// Dutzend MiB zu belegen (Defekt
/// `261017-1923-lange-namen-und-pax-koepfe-im-tar-werden-beim-lesen-ohne-obergrenze-in-den-speicher-gelesen.md`).
///
/// Gemessen wird der hoechste Stand des Speichers im Kind ueber
/// `/usr/bin/time -l`, damit die Probe nicht nur das Ergebnis, sondern auch
/// das Zuteilen haelt: ohne die Grenze lese die Kiste das Gigabyte Nullen ganz
/// in einen `Vec` und laese danach den Eintrag dahinter, und das Verzeichnis
/// entstuende mit einem ausgelassenen Namen.
#[test]
fn ein_langer_name_ueber_der_grenze_macht_das_tar_ungueltig_ohne_ihn_zu_lesen() {
    let ordner = Pruefordner::neu("archiv-langnamenbombe");
    bomben_bauen(ordner.pfad());

    let selbst = std::env::current_exe().expect("die Testdatei kennt ihren Pfad nicht");
    let name = "kind_liest_die_langnamenbomben";
    let ergebnis = Command::new("/usr/bin/time")
        .arg("-l")
        .arg(&selbst)
        .args([
            "--exact",
            "--ignored",
            "--nocapture",
            "--test-threads",
            "1",
            name,
        ])
        .env(gemeinsam::KINDAUFTRAG, ordner.pfad())
        .output()
        .expect("das Kind laesst sich starten");
    let stdout = String::from_utf8_lossy(&ergebnis.stdout);
    let stderr = String::from_utf8_lossy(&ergebnis.stderr);
    assert!(
        ergebnis.status.success() && stdout.contains("test result: ok. 1 passed;"),
        "das Kind `{name}` ist nicht als genau ein Kind gelaufen\n{stdout}\n{stderr}"
    );
    let spitze: u64 = stderr
        .lines()
        .find(|zeile| zeile.contains("maximum resident set size"))
        .and_then(|zeile| zeile.split_whitespace().next())
        .and_then(|zahl| zahl.parse().ok())
        .unwrap_or_else(|| panic!("`time -l` nennt keinen Hoechststand:\n{stderr}"));
    assert!(
        spitze < 160 * 1024 * 1024,
        "das Lesen der Bomben belegte {spitze} Bytes"
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_liest_die_langnamenbomben() {
    let Some(ordner) = gemeinsam::kindauftrag() else {
        return;
    };
    bomben_lesen(&ordner);
}
