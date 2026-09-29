//! Abnahme des Zuruecksetzens auf Werkseinstellungen im Kern (C2 des Spec
//! `260929-0759_*_spec-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md`,
//! Schritt 3 des Plans `260929-1025_*_plan-werkseinstellungen-zuruecksetzen-und-neu-einlesen.md`).
//!
//! Jede Probe legt ihren eigenen Ablageordner unter dem Temporaerverzeichnis an
//! und geht durch denselben Durchgang wie der Betrieb. Der Zeitpunkt ist fest;
//! die Uhr liest keine Probe dieser Datei.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use krk_core::ablage::neuerungen::Vergleichsform;
use krk_core::ablage::werkszustand::{self, Werkshindernis, Werkszustand, Zurueckgesetzt};
use krk_core::ablage::{Ablage, Ablageort, Datei, einstellungen, leseprofile};
use krk_core::verzeichnis::sys;

mod gemeinsam;
use gemeinsam::{Pruefordner, rechtesperre_haelt_oder_abbruch};

/// Ein fester Zeitpunkt, der 21. September 2026.
fn zeitpunkt() -> SystemTime {
    UNIX_EPOCH + Duration::from_secs(1_790_000_000)
}

/// Der Stempel, den ein Vorgang zum festen Zeitpunkt traegt.
fn stempel() -> String {
    let zeit = sys::ortszeit(zeitpunkt()).expect("der feste Zeitpunkt hat eine Ortszeit");
    format!(
        "{:02}{:02}{:02}-{:02}{:02}",
        zeit.jahr.rem_euclid(100),
        zeit.monat,
        zeit.tag,
        zeit.stunde,
        zeit.minute
    )
}

const EIGENE_EINSTELLUNGEN: &str = "# meine\nterminal = \"com.googlecode.iterm2\"\n";
const EIGENE_PROFILE: &str = "# keine Profile, mit Absicht\n";
const EIGENE_BELEGUNG: &str = "# meine Belegung\n";

/// Ein Ablageordner mit den drei eigenen Fassungen, soweit verlangt.
fn ablage_mit(ordner: &Pruefordner, dateien: &[(Datei, &str)]) -> (Ablage, PathBuf) {
    let wurzel = ordner.pfad().join("KRK");
    let ablage = Ablage::oeffnen(Ablageort::an(&wurzel)).expect("Ablage laesst sich nicht oeffnen");
    for (welche, inhalt) in dateien {
        fs::write(ablage.pfad(*welche), inhalt).expect("schreiben gescheitert");
    }
    (ablage, wurzel)
}

/// Alle drei eigenen Fassungen.
fn alle_drei() -> [(Datei, &'static str); 3] {
    [
        (Datei::Einstellungen, EIGENE_EINSTELLUNGEN),
        (Datei::Leser, EIGENE_PROFILE),
        (Datei::Belegung, EIGENE_BELEGUNG),
    ]
}

/// Setzt unter einem Durchgang zurueck, zum festen Zeitpunkt.
fn zuruecksetzen(ablage: &Ablage) -> Result<Zurueckgesetzt, Werkshindernis> {
    ablage
        .durchgang(|zugang| werkszustand::zuruecksetzen(zugang, zeitpunkt()))
        .expect("die Schreibsperre laesst sich nicht nehmen")
}

/// Die Namen im Ablageordner, sortiert, ohne die Sperrdateien der Ablage.
fn namen(wurzel: &Path) -> Vec<String> {
    let mut namen: Vec<String> = fs::read_dir(wurzel)
        .expect("Ablageordner nicht lesbar")
        .map(|eintrag| {
            eintrag
                .expect("Eintrag nicht lesbar")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| !name.ends_with(".lock"))
        .collect();
    namen.sort();
    namen
}

fn lesen(pfad: &Path) -> Vec<u8> {
    fs::read(pfad).unwrap_or_else(|fehler| panic!("{} nicht lesbar: {fehler}", pfad.display()))
}

/// C2.1: danach sind `readers.toml` und `settings.toml` bytegleich mit ihrer
/// Auslieferungsfassung, mit vorher eigenen Dateien und mit vorher fehlenden.
#[test]
fn c2_1_beide_dateien_stehen_woertlich_als_auslieferungsfassung_da() {
    for vorher in [&alle_drei()[..], &[]] {
        let ordner = Pruefordner::neu("werk-c2-1");
        let (ablage, _) = ablage_mit(&ordner, vorher);
        let ausgang = zuruecksetzen(&ablage).expect("das Zuruecksetzen scheitert");
        assert!(ausgang.vollstaendig(), "{ausgang:?}");
        assert_eq!(
            lesen(&ablage.pfad(Datei::Einstellungen)),
            einstellungen::AUSLIEFERUNGSTEXT.as_bytes()
        );
        assert_eq!(
            lesen(&ablage.pfad(Datei::Leser)),
            leseprofile::AUSLIEFERUNGSTEXT.as_bytes()
        );
    }
}

/// C2.2 und C2.3: `keymap.toml` fehlt danach, und jede vorher vorhandene Datei
/// liegt bytegleich unter `<name>.<JJMMTT-HHMM>`; die Meldung nennt jeden
/// Sicherungspfad voll (C2.8).
#[test]
fn c2_2_und_c2_3_die_alten_fassungen_liegen_unter_ihrem_stempel() {
    let ordner = Pruefordner::neu("werk-c2-3");
    let (ablage, wurzel) = ablage_mit(&ordner, &alle_drei());
    let ausgang = zuruecksetzen(&ablage).expect("das Zuruecksetzen scheitert");

    assert!(
        !ablage.pfad(Datei::Belegung).exists(),
        "keymap.toml steht noch"
    );
    let stempel = stempel();
    for (welche, inhalt) in alle_drei() {
        let sicherung = wurzel.join(format!("{}.{stempel}", welche.dateiname()));
        assert_eq!(
            lesen(&sicherung),
            inhalt.as_bytes(),
            "{}",
            sicherung.display()
        );
        assert!(
            ausgang.meldung().contains(&sicherung.display().to_string()),
            "die Meldung nennt {} nicht: {}",
            sicherung.display(),
            ausgang.meldung()
        );
    }
    assert!(
        ausgang
            .meldung()
            .starts_with("Auf Werkseinstellungen zurückgesetzt.")
    );
}

/// C2.4: eine vorher fehlende Datei hinterlaesst keine Sicherung, und der
/// Ausgang sagt es.
#[test]
fn c2_4_eine_fehlende_datei_hinterlaesst_keine_sicherung() {
    let ordner = Pruefordner::neu("werk-c2-4");
    let (ablage, wurzel) = ablage_mit(&ordner, &[(Datei::Einstellungen, EIGENE_EINSTELLUNGEN)]);
    let ausgang = zuruecksetzen(&ablage).expect("das Zuruecksetzen scheitert");

    let stempel = stempel();
    assert_eq!(
        namen(&wurzel),
        vec![
            "readers.toml".to_owned(),
            "settings.toml".to_owned(),
            format!("settings.toml.{stempel}"),
        ]
    );
    let meldung = ausgang.meldung();
    for name in ["keymap.toml", "readers.toml"] {
        assert!(
            meldung.contains(&format!(
                "{name} stand nicht da, für sie ist nichts beiseitegelegt."
            )),
            "{meldung}"
        );
    }
    assert!(
        !meldung.contains("settings.toml stand nicht da"),
        "{meldung}"
    );
}

/// C2.5: zwei Vorgaenge mit demselben Zeitpunkt legen `<name>.<stempel>` und
/// `<name>.<stempel>-2` an, beide mit ihrer jeweils alten Fassung.
#[test]
fn c2_5_ein_zweiter_vorgang_in_derselben_minute_ueberschreibt_nichts() {
    let ordner = Pruefordner::neu("werk-c2-5");
    let (ablage, wurzel) = ablage_mit(&ordner, &alle_drei());
    let _ = zuruecksetzen(&ablage).expect("der erste Vorgang scheitert");
    fs::write(ablage.pfad(Datei::Belegung), "# zweite\n").expect("schreiben gescheitert");
    let _ = zuruecksetzen(&ablage).expect("der zweite Vorgang scheitert");

    let stempel = stempel();
    assert_eq!(
        lesen(&wurzel.join(format!("settings.toml.{stempel}"))),
        EIGENE_EINSTELLUNGEN.as_bytes()
    );
    assert_eq!(
        lesen(&wurzel.join(format!("settings.toml.{stempel}-2"))),
        einstellungen::AUSLIEFERUNGSTEXT.as_bytes()
    );
    assert_eq!(
        lesen(&wurzel.join(format!("keymap.toml.{stempel}"))),
        EIGENE_BELEGUNG.as_bytes()
    );
    assert_eq!(
        lesen(&wurzel.join(format!("keymap.toml.{stempel}-2"))),
        b"# zweite\n"
    );
}

/// C2.6: `bookmarks.toml`, `session.toml`, `reported.toml` und eine
/// vorhandene `*.beschaedigt` sind danach bytegleich.
#[test]
fn c2_6_die_uebrigen_ablagedateien_bleiben_bytegleich() {
    let ordner = Pruefordner::neu("werk-c2-6");
    let (ablage, wurzel) = ablage_mit(
        &ordner,
        &[
            (Datei::Lesezeichen, "eintraege = []\n# L\n"),
            (Datei::Sitzung, "aktiv = 1\n# S\n"),
            (Datei::Merker, "gemeldete_fassung = \"2.1.0\"\n"),
        ],
    );
    let beschaedigt = wurzel.join("bookmarks.toml.beschaedigt");
    fs::write(&beschaedigt, "kaputt = [\n").expect("schreiben gescheitert");
    let vorher: Vec<(PathBuf, Vec<u8>)> = [
        ablage.pfad(Datei::Lesezeichen),
        ablage.pfad(Datei::Sitzung),
        ablage.pfad(Datei::Merker),
        beschaedigt,
    ]
    .into_iter()
    .map(|pfad| {
        let inhalt = lesen(&pfad);
        (pfad, inhalt)
    })
    .collect();

    let _ = zuruecksetzen(&ablage).expect("das Zuruecksetzen scheitert");

    for (pfad, inhalt) in vorher {
        assert_eq!(
            lesen(&pfad),
            inhalt,
            "{} hat sich geaendert",
            pfad.display()
        );
    }
}

/// C2.7, zweite Stufe: ein Ablageordner ohne Schreibrecht laesst `link`
/// scheitern; danach sind alle drei Dateien unveraendert, und kein
/// Sicherungsname steht.
///
/// **Unter root belegt die Probe nichts**; die Regel steht an
/// `gemeinsam::rechtesperre_haelt_oder_abbruch`.
#[test]
fn c2_7_scheitert_das_beiseitelegen_bleibt_alles_wie_es_war() {
    let ordner = Pruefordner::neu("werk-c2-7a");
    let (ablage, wurzel) = ablage_mit(&ordner, &alle_drei());
    let namen_vorher = namen(&wurzel);
    fs::set_permissions(&wurzel, fs::Permissions::from_mode(0o555))
        .expect("die Rechte lassen sich nicht entziehen");
    rechtesperre_haelt_oder_abbruch(
        "ein Ablageordner ohne Schreibrecht laesst das Beiseitelegen scheitern (C2.7)",
        fs::write(wurzel.join("probe"), "x").is_err(),
    );

    let ergebnis = zuruecksetzen(&ablage);
    fs::set_permissions(&wurzel, fs::Permissions::from_mode(0o755))
        .expect("die Rechte lassen sich nicht zurueckgeben");

    let hindernis = ergebnis.expect_err("das Zuruecksetzen ist trotz Sperre gelungen");
    assert!(
        matches!(
            hindernis,
            Werkshindernis::NichtBeiseitegelegt(Datei::Belegung, _)
        ),
        "{hindernis:?}"
    );
    assert!(
        hindernis.meldung().contains("keymap.toml"),
        "{}",
        hindernis.meldung()
    );
    assert_eq!(namen(&wurzel), namen_vorher);
    for (welche, inhalt) in alle_drei() {
        assert_eq!(lesen(&ablage.pfad(welche)), inhalt.as_bytes());
    }
}

/// C2.7, dritte Stufe: ein Verzeichnis an der Stelle der Nachbardatei von
/// `settings.toml` laesst `vorbereiten` scheitern; danach sind alle drei
/// Dateien unveraendert, und die schon angelegten Sicherungen sind entfernt.
#[test]
fn c2_7_scheitert_die_nachbardatei_werden_die_sicherungen_entfernt() {
    let ordner = Pruefordner::neu("werk-c2-7b");
    let (ablage, wurzel) = ablage_mit(&ordner, &alle_drei());
    fs::create_dir(wurzel.join("settings.toml.neu")).expect("Verzeichnis nicht anlegbar");
    let namen_vorher = namen(&wurzel);

    let hindernis = zuruecksetzen(&ablage).expect_err("das Zuruecksetzen ist gelungen");
    assert!(
        matches!(
            hindernis,
            Werkshindernis::NichtVorbereitet(Datei::Einstellungen, _)
        ),
        "{hindernis:?}"
    );
    assert!(
        hindernis.meldung().contains("settings.toml"),
        "{}",
        hindernis.meldung()
    );
    assert_eq!(namen(&wurzel), namen_vorher);
    for (welche, inhalt) in alle_drei() {
        assert_eq!(lesen(&ablage.pfad(welche)), inhalt.as_bytes());
    }
}

/// Ist eine der drei ein symbolischer Verweis, auch ein verwaister, bricht der
/// Vorgang mit `Verweis` ab, nichts ist geaendert, und die Zieldatei des
/// Verweises ist bytegleich.
#[test]
fn ein_verweis_bricht_ab_bevor_etwas_geschieht() {
    let ordner = Pruefordner::neu("werk-verweis");
    let ziel = ordner.datei("fremd.toml", EIGENE_PROFILE);
    for verweisziel in [ziel.clone(), ordner.unter("gibt-es-nicht.toml")] {
        let unterordner = Pruefordner::neu("werk-verweis-ablage");
        let (ablage, wurzel) = ablage_mit(
            &unterordner,
            &[
                (Datei::Einstellungen, EIGENE_EINSTELLUNGEN),
                (Datei::Belegung, EIGENE_BELEGUNG),
            ],
        );
        std::os::unix::fs::symlink(&verweisziel, ablage.pfad(Datei::Leser))
            .expect("Verweis nicht anlegbar");
        let namen_vorher = namen(&wurzel);

        let vorab = werkszustand::lage(|welche| ablage.pfad(welche));
        assert_eq!(vorab, Err(Werkshindernis::Verweis(Datei::Leser)));
        let hindernis = zuruecksetzen(&ablage).expect_err("das Zuruecksetzen ist gelungen");
        assert_eq!(hindernis, Werkshindernis::Verweis(Datei::Leser));
        assert!(
            hindernis
                .meldung()
                .starts_with("readers.toml ist ein symbolischer Verweis")
        );

        assert_eq!(namen(&wurzel), namen_vorher);
        assert!(
            fs::symlink_metadata(ablage.pfad(Datei::Leser))
                .expect("der Verweis ist fort")
                .file_type()
                .is_symlink()
        );
        assert_eq!(
            lesen(&ablage.pfad(Datei::Einstellungen)),
            EIGENE_EINSTELLUNGEN.as_bytes()
        );
        assert_eq!(
            lesen(&ablage.pfad(Datei::Belegung)),
            EIGENE_BELEGUNG.as_bytes()
        );
    }
    assert_eq!(lesen(&ziel), EIGENE_PROFILE.as_bytes());
}

/// Die Lage nennt, welche der drei steht; die Oberflaeche fragt so vor der
/// Rueckfrage, ob eine eigene `keymap.toml` da ist.
#[test]
fn die_lage_nennt_welche_datei_steht() {
    let ordner = Pruefordner::neu("werk-lage");
    let (ablage, _) = ablage_mit(&ordner, &[(Datei::Belegung, EIGENE_BELEGUNG)]);
    let lage = werkszustand::lage(|welche| ablage.pfad(welche)).expect("die Lage scheitert");
    assert!(lage.steht(Datei::Belegung));
    assert!(!lage.steht(Datei::Einstellungen));
    assert!(!lage.steht(Datei::Leser));
    assert!(
        !lage.steht(Datei::Sitzung),
        "eine unberuehrte Datei hat keine Lage"
    );
}

/// Die Menge der Dateien, die das Zuruecksetzen anfasst, ist die Menge, die
/// die Neuerungen vergleichen: beide sind die von Hand gepflegten.
#[test]
fn die_zurueckgesetzten_dateien_sind_die_verglichenen() {
    let beruehrt: Vec<Datei> = Datei::ALLE
        .into_iter()
        .filter(|&welche| Werkszustand::fuer(welche) != Werkszustand::Unberuehrt)
        .collect();
    let verglichen: Vec<Datei> = Datei::ALLE
        .into_iter()
        .filter(|&welche| Vergleichsform::fuer(welche) != Vergleichsform::Nicht)
        .collect();
    assert_eq!(beruehrt, verglichen);
}
