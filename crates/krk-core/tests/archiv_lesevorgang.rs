//! Abnahme von Schritt 2 des Plans
//! `261017-1036-plan-zip-und-gz-wie-ordner-im-dateifenster.md`, soweit sie
//! den Kern betrifft: der Lesevorgang eines Ordners in einem Archiv.
//!
//! Neben `tests/archiv.rs` und nicht darin, weil jene Datei die Abnahme des
//! Inhaltsverzeichnisses (Schritt 1) und des Umbaus (Schritt 5) traegt; der
//! Lesevorgang ist eine andere Naht, die des Dateifensters.
//!
//! Jede Probe baut ihre Archive selbst in einem Pruefordner
//! (`gemeinsam::archivbau`) und vergleicht, was [`Lesevorgang::archiv_starten`]
//! liefert, mit dem, was [`Lesevorgang::starten`] ueber den Ordner liefert, aus
//! dem das Archiv gepackt ist: dieselbe Gleichheit, die die fernen Proben
//! gegen den Server des Systems ziehen.

mod gemeinsam;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use gemeinsam::Pruefordner;
use gemeinsam::archivbau::{gzip_glied, tar_gz_packen, tar_packen, zip_packen};
use krk_core::archiv::{Archivformat, Archivzugang, Innenpfad, Verzeichnisfehler};
use krk_core::ort::{Archivort, Ort, Ortsart};
use krk_core::verzeichnis::{
    Abschluss, Archivfehler, Eintrag, Lesevorgang, Meldung, STAPELGROESSE, Typ,
};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Holt jede Meldung eines Lesevorgangs ab, bis sein Kanal schliesst: die
/// Eintraege, der Abschluss, wie oft `Fertig` kam, und die Zahl der
/// ausgelassenen Namen.
fn auslesen(vorgang: Lesevorgang) -> (Vec<Eintrag>, Abschluss, usize, u64) {
    let mut eintraege = Vec::new();
    let mut abschluss = None;
    let mut fertig = 0;
    while let Ok(meldung) = vorgang.meldungen().recv() {
        match meldung {
            Meldung::Stapel {
                eintraege: stapel, ..
            } => {
                assert!(stapel.len() <= STAPELGROESSE, "ein Stapel ist zu gross");
                eintraege.extend(stapel);
            }
            Meldung::Fertig {
                abschluss: ende, ..
            } => {
                fertig += 1;
                abschluss = Some(ende);
            }
        }
    }
    let uebersprungen = vorgang.uebersprungen();
    vorgang.warten();
    (
        eintraege,
        abschluss.expect("der Lauf meldet keinen Abschluss"),
        fertig,
        uebersprungen,
    )
}

/// Der Ort `innen` im Archiv `datei`, mit einem eigenen Zugang.
fn archivort(datei: &Path, innen: &str) -> Archivort {
    let format = Archivformat::aus_namen(datei.file_name().expect("Name"))
        .expect("die Probe gibt ihrem Archiv eine Endung aus der Tafel");
    Archivort {
        zugang: Arc::new(Archivzugang::neu(datei.to_path_buf(), format)),
        innen: Innenpfad::aus_archivname(innen).expect("ein sicherer Name"),
    }
}

/// Was an einem Eintrag verglichen wird: Name, Typ, Versteckt, die Groesse
/// einer Datei und das Aenderungsdatum auf die Sekunde. Die Groesse einer
/// Verknuepfung bleibt draussen: lokal ist sie die Laenge des Verweistexts,
/// im tar null.
fn vergleichbar(eintraege: &[Eintrag]) -> Vec<(String, Typ, bool, Option<u64>, u64)> {
    let mut zeilen: Vec<_> = eintraege
        .iter()
        .map(|eintrag| {
            (
                eintrag.name.clone(),
                eintrag.typ,
                eintrag.versteckt,
                (eintrag.typ == Typ::Datei).then_some(eintrag.groesse),
                eintrag
                    .geaendert
                    .duration_since(UNIX_EPOCH)
                    .expect("nach 1970")
                    .as_secs(),
            )
        })
        .collect();
    zeilen.sort_by(|eine, andere| eine.0.cmp(&andere.0));
    zeilen
}

/// Ein kleiner Baum mit Unterordnern, einer versteckten Datei, einer
/// Verknuepfung und festen Zeiten.
fn baum_anlegen(ordner: &Pruefordner) -> std::path::PathBuf {
    let wurzel = ordner.ordner("baum");
    fs::create_dir_all(wurzel.join("a/b")).unwrap();
    fs::create_dir_all(wurzel.join("leer")).unwrap();
    fs::write(wurzel.join("eins.txt"), b"eins").unwrap();
    fs::write(wurzel.join(".versteckt"), b"v").unwrap();
    fs::write(wurzel.join("a/zwei.txt"), vec![b'z'; 5000]).unwrap();
    fs::write(wurzel.join("a/b/drei.txt"), b"drei").unwrap();
    std::os::unix::fs::symlink("eins.txt", wurzel.join("verweis")).unwrap();
    let zeiten = [
        ("eins.txt", 1_500_000_000),
        (".versteckt", 1_510_000_000),
        ("a/zwei.txt", 1_600_000_000),
        ("a/b/drei.txt", 1_700_000_000),
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

/// Je Format und je Ordner liefert der Lesevorgang im Archiv dieselben
/// Eintraege wie der lokale Lesevorgang ueber den Ordner, aus dem das Archiv
/// gepackt ist; eine gz-Datei liefert ihren einen Eintrag.
#[test]
fn ein_archiv_und_sein_ausgepackter_ordner_liefern_ueber_den_lesevorgang_dieselben_eintraege() {
    let ordner = Pruefordner::neu("archiv-lesevorgang");
    let wurzel = baum_anlegen(&ordner);
    let archive = [
        ordner.unter("baum.zip"),
        ordner.unter("baum.tar"),
        ordner.unter("baum.tgz"),
    ];
    zip_packen(&wurzel, &archive[0]);
    tar_packen(&wurzel, &archive[1]);
    tar_gz_packen(&wurzel, &archive[2]);

    for archiv in &archive {
        for innen in ["", "a", "a/b", "leer"] {
            let (lokal, lokal_ende, _, _) = auslesen(Lesevorgang::starten(wurzel.join(innen), 1));
            assert!(lokal_ende.ist_vollstaendig());
            let (gelesen, ende, fertig, ausgelassen) =
                auslesen(Lesevorgang::archiv_starten(archivort(archiv, innen), 2));
            let wo = format!("{}:/{innen}", archiv.display());
            assert_eq!(fertig, 1, "{wo}");
            assert!(ende.ist_vollstaendig(), "{wo}: {ende:?}");
            assert_eq!(ausgelassen, 0, "{wo}");
            assert_eq!(vergleichbar(&gelesen), vergleichbar(&lokal), "{wo}");
        }
    }

    // Eine gz-Datei zeigt einen Eintrag: den Namen ohne `.gz`, die entpackte
    // Groesse und die Zeit der Archivdatei.
    let gz = ordner.unter("bericht.txt.gz");
    fs::write(&gz, gzip_glied(&[b'x'; 3000])).unwrap();
    let (gelesen, ende, fertig, _) = auslesen(Lesevorgang::archiv_starten(archivort(&gz, ""), 3));
    assert_eq!(fertig, 1);
    assert!(ende.ist_vollstaendig(), "{ende:?}");
    assert_eq!(gelesen.len(), 1);
    assert_eq!(gelesen[0].name, "bericht.txt");
    assert_eq!(gelesen[0].typ, Typ::Datei);
    assert_eq!(gelesen[0].groesse, 3000);
}

/// Ein Lesevorgang, der vor dem ersten Abholen abgebrochen wird, schickt
/// genau ein `Fertig` mit `Abgebrochen` und nicht alle Eintraege.
#[test]
fn ein_abgebrochener_archivlesevorgang_schickt_genau_ein_fertig() {
    let ordner = Pruefordner::neu("archiv-abbruch");
    let datei = ordner.unter("viele.zip");
    let mut schreiber = ZipWriter::new(File::create(&datei).unwrap());
    let anzahl = 6 * STAPELGROESSE;
    for nummer in 0..anzahl {
        schreiber
            .start_file(format!("d{nummer:05}.txt"), SimpleFileOptions::default())
            .unwrap();
        schreiber.write_all(b"x").unwrap();
    }
    schreiber.finish().unwrap();

    let vorgang = Lesevorgang::archiv_starten(archivort(&datei, ""), 4);
    vorgang.abbrechen();
    let (gelesen, ende, fertig, _) = auslesen(vorgang);
    assert_eq!(fertig, 1, "genau ein Fertig");
    assert!(matches!(ende, Abschluss::Abgebrochen), "{ende:?}");
    assert!(gelesen.len() < anzahl, "der Abbruch griff nicht");
}

/// Ein Archiv, das keines ist, endet mit dem Grund des Lesers, und ein
/// Innenpfad, der kein Ordner ist, mit `KeinOrdner`; beide ohne Stapel.
/// `KeinOrdner` nennt den naechsten Vorfahren, der im Archiv ein Ordner ist,
/// notfalls die Wurzel: dorthin weicht der Tab aus.
#[test]
fn ein_ungueltiges_archiv_und_ein_fehlender_ordner_enden_mit_grund() {
    let ordner = Pruefordner::neu("archiv-fehler");
    let falsch = ordner.datei("kein.zip", b"das ist kein zip");
    let (gelesen, ende, fertig, _) =
        auslesen(Lesevorgang::archiv_starten(archivort(&falsch, ""), 5));
    assert!(gelesen.is_empty());
    assert_eq!(fertig, 1);
    assert!(
        matches!(
            ende,
            Abschluss::Archiv(Archivfehler::Verzeichnis(Verzeichnisfehler::Ungueltig(_)))
        ),
        "{ende:?}"
    );

    let wurzel = baum_anlegen(&ordner);
    let zip = ordner.unter("baum.zip");
    zip_packen(&wurzel, &zip);
    for (innen, vorhanden) in [
        ("eins.txt", ""),
        ("gibt/es/nicht", ""),
        ("a/b/fort/tiefer", "a/b"),
        ("a/zwei.txt", "a"),
    ] {
        let (gelesen, ende, _, _) =
            auslesen(Lesevorgang::archiv_starten(archivort(&zip, innen), 6));
        assert!(gelesen.is_empty(), "{innen}");
        let Abschluss::Archiv(Archivfehler::KeinOrdner {
            vorhanden: gefunden,
        }) = ende
        else {
            panic!("{innen}: {ende:?}");
        };
        assert_eq!(gefunden.als_text(), vorhanden, "{innen}");
    }
}

/// Der Archivort rechnet lexikalisch, nennt sich mit dem Doppelpunkt und
/// sagt seine Art am Wert; gleich sind zwei Orte ueber Datei und Innenpfad,
/// nicht ueber den Zeiger auf den Zugang.
#[test]
fn ein_archivort_ist_ein_wert() {
    let datei = Path::new("/gibt/es/nicht/paket.tar.gz");
    let wurzel = archivort(datei, "");
    assert_eq!(wurzel.to_string(), "/gibt/es/nicht/paket.tar.gz:/");
    let unten = wurzel.kind("a").and_then(|a| a.kind("b")).expect("Namen");
    assert_eq!(unten.to_string(), "/gibt/es/nicht/paket.tar.gz:/a/b");
    assert!(wurzel.kind("..").is_none());
    let (oben, verlassen) = unten.aufwaerts().expect("unter der Wurzel");
    assert_eq!(verlassen, "b");
    assert_eq!(
        oben,
        archivort(datei, "a"),
        "ein anderer Zugang, derselbe Ort"
    );
    assert!(wurzel.aufwaerts().is_none());

    assert_eq!(
        Ort::Archiv(wurzel.clone()).art(),
        Ortsart::Archiv { beschreibbar: true }
    );
    assert_eq!(
        Ort::Archiv(archivort(Path::new("/x/a.txt.gz"), "")).art(),
        Ortsart::Archiv {
            beschreibbar: false
        }
    );
    assert!(Ort::Archiv(wurzel.clone()).lokal().is_none());
    assert!(Ort::Archiv(wurzel.clone()).fern().is_none());
    assert!(Ort::Archiv(wurzel).archiv().is_some());
    assert_eq!(
        krk_core::ort::nicht_verfuegbar_text(&Ort::Lokal("/".into())),
        None
    );
}
