//! Abnahme der Ablage unter Application Support (Schritt 10 des Plans).
//!
//! Alle Pruefungen laufen ohne Fenster und ohne AppKit, und keine fasst das
//! echte Benutzerverzeichnis an: jede legt ihren eigenen Ablageordner unter
//! dem Temporaerverzeichnis an. Die einzige Ausnahme ist
//! [`der_ablageordner_liegt_unter_application_support`], und die liest nur
//! einen Pfad, ohne etwas anzulegen.
//!
//! Die Pruefungen des Fenster- und Tabmodells sind mit Schritt 12 in diese
//! Datei hineingewachsen; deshalb waehlt das Abnahmekommando das Testprogramm
//! mit `--test ablage` und filtert nicht ueber Pruefungsnamen.
//!
//! # Eine Pruefung laeuft in einem eigenen Prozess
//!
//! Der Abbruch zwischen Schreiben und Umbenennen ist im laufenden Testprozess
//! nicht feststellbar: er verlangt einen Prozess, der wirklich stirbt. Er
//! startet deshalb dieselbe Testdatei ein zweites Mal, mit einer
//! Umgebungsvariablen als Auftrag. Die Kindprobe traegt `#[ignore]`, damit ein
//! gewoehnlicher Lauf sie nicht anfasst, und kehrt ohne ihre Umgebungsvariable
//! sofort zurueck.
//!
//! Die zweite Kindprobe ist mit Schritt 12 entfallen. Sie las die
//! Standardfehlerausgabe mit, weil `ablage::melden` dorthin schrieb; seit der
//! Kern nichts mehr ausgibt, ist die Zusage ohne zweiten Prozess pruefbar.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use krk_core::ablage::sitzung::SITZUNGSTAKT;
use krk_core::ablage::{
    Ablage, Ablageort, Breiten, Datei, Dateifenster, Ersetzung, Fensterseite, Geladen, Grund,
    Lesezeichen, Lesezeichenliste, Sichtbarkeit, Sitzung, Tab, Verschiebung, atomar, pfade,
};
use krk_core::verzeichnis::{Richtung, Schluessel, Sortierung};

// ---------------------------------------------------------------------------
// Pruefordner und Stellvertreter
// ---------------------------------------------------------------------------

static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst wieder abraeumt.
struct Pruefordner {
    pfad: PathBuf,
}

impl Pruefordner {
    fn neu(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let mut pfad = std::env::temp_dir();
        pfad.push(format!(
            "krk-ablage-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&pfad);
        fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
        Self { pfad }
    }

    fn pfad(&self) -> &Path {
        &self.pfad
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.pfad);
    }
}

/// Eine Ablage in einem frischen Pruefordner.
fn ablage(zweck: &str) -> (Pruefordner, Ablage) {
    let ordner = Pruefordner::neu(zweck);
    let ablage =
        Ablage::oeffnen(Ablageort::an(ordner.pfad())).expect("Ablage laesst sich nicht oeffnen");
    (ordner, ablage)
}

/// Stellvertreter fuer den Inhalt von `keymap.toml`.
///
/// Die Belegung selbst entsteht mit Schritt 11 und liegt nicht in diesem
/// Modul; die Ablage kennt von der dritten Datei nur ihren Namen und ihren
/// Weg. Damit die Zusage "alle drei Dateien" trotzdem an drei Dateien geprueft
/// wird und nicht an zweien, laeuft der dritte Weg hier mit diesem
/// Stellvertreter, ueber dieselbe Ablage und denselben Pfad.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct BelegungStellvertreter {
    zurueckgesetzt: bool,
    zeilen: Vec<BelegungZeile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct BelegungZeile {
    id: String,
    tasten: Vec<String>,
}

/// Eine Sitzung, die sich in jedem Feld vom Auslieferungszustand unterscheidet.
///
/// Nur so traegt der Rundlauf: eine Sitzung im Auslieferungszustand liefe auch
/// dann unveraendert zurueck, wenn das Wiedereinlesen gar nichts laese.
fn beispielsitzung() -> Sitzung {
    Sitzung {
        aktiv: Fensterseite::Rechts,
        breiten: Breiten {
            lesezeichen: Some(180.0),
            links: Some(520.5),
            rechts: Some(520.5),
            vorschau: None,
        },
        sichtbar: Sichtbarkeit {
            lesezeichen: false,
            zweites_dateifenster: true,
            vorschau: false,
        },
        fenster: [
            Dateifenster {
                aktiver_tab: 1,
                tabs: vec![
                    Tab::auf("/Users/pruefung/Projekte"),
                    Tab {
                        ordner: PathBuf::from("/Users/pruefung/Bilder"),
                        auswahl: Some("urlaub.jpg".to_string()),
                        verstecke_ausgeblendet: false,
                        sortierung: Sortierung::neu(Schluessel::Geaendert, Richtung::Absteigend),
                        bildlauf: 640.0,
                    },
                ],
            },
            Dateifenster {
                aktiver_tab: 0,
                tabs: vec![Tab {
                    ordner: PathBuf::from("/Volumes/Sicherung"),
                    auswahl: Some("2026-08".to_string()),
                    verstecke_ausgeblendet: true,
                    sortierung: Sortierung::neu(Schluessel::Groesse, Richtung::Aufsteigend),
                    bildlauf: 0.0,
                }],
            },
        ],
    }
}

fn beispielbelegung() -> BelegungStellvertreter {
    BelegungStellvertreter {
        zurueckgesetzt: true,
        zeilen: vec![
            BelegungZeile {
                id: "vorschau-anzeigen".to_string(),
                tasten: vec!["f3".to_string(), "cmd+y".to_string()],
            },
            BelegungZeile {
                id: "in-papierkorb".to_string(),
                tasten: vec!["delete".to_string(), "cmd+delete".to_string()],
            },
        ],
    }
}

fn beispiellesezeichen() -> Lesezeichenliste {
    Lesezeichenliste::aus(vec![
        Lesezeichen::neu("Projekte", "/Users/pruefung/Projekte"),
        Lesezeichen::neu("Sicherung", "/Volumes/Sicherung"),
        Lesezeichen::neu("Wurzel", "/"),
    ])
}

// ---------------------------------------------------------------------------
// Ort und erster Start
// ---------------------------------------------------------------------------

#[test]
fn der_ablageordner_liegt_unter_application_support() {
    let zuhause = pfade::benutzerverzeichnis().expect("kein Benutzerverzeichnis");
    let ort = Ablageort::im_benutzerverzeichnis().expect("der Ort laesst sich nicht aufloesen");

    assert_eq!(
        ort.wurzel(),
        zuhause
            .join("Library")
            .join("Application Support")
            .join("KRK")
    );

    let namen: Vec<String> = Datei::ALLE
        .iter()
        .map(|welche| {
            let pfad = ort.datei(*welche);
            assert_eq!(
                pfad.parent(),
                Some(ort.wurzel()),
                "{} liegt nicht im Ablageordner",
                welche.dateiname()
            );
            welche.dateiname().to_string()
        })
        .collect();
    assert_eq!(namen, ["keymap.toml", "bookmarks.toml", "session.toml"]);
}

#[test]
fn der_erste_start_legt_den_ordner_an_und_liefert_den_auslieferungszustand() {
    let ordner = Pruefordner::neu("erststart");
    let wurzel = ordner.pfad().join("Application Support").join("KRK");
    assert!(!wurzel.exists(), "der Ablageordner steht schon vorher");

    let ablage = Ablage::oeffnen(Ablageort::an(&wurzel)).expect("erster Start scheitert");
    assert!(wurzel.is_dir(), "der erste Start hat nichts angelegt");

    let belegung: Geladen<BelegungStellvertreter> = ablage.laden(Datei::Belegung);
    let lesezeichen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    let sitzung: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert_eq!(belegung.wert, BelegungStellvertreter::default());
    assert_eq!(lesezeichen.wert, Lesezeichenliste::default());
    assert_eq!(sitzung.wert, Sitzung::default());

    // Eine fehlende Datei ist der erste Start und keine Meldung wert.
    assert!(!belegung.ist_ersetzt());
    assert!(!lesezeichen.ist_ersetzt());
    assert!(!sitzung.ist_ersetzt());

    // Und das Laden legt nichts an: geschrieben wird nur beim Schreiben.
    for welche in Datei::ALLE {
        assert!(
            !ablage.pfad(welche).exists(),
            "{} ist beim Laden entstanden",
            welche.dateiname()
        );
    }

    // Der zweite Start findet den Ordner vor und stolpert nicht darueber.
    Ablage::oeffnen(Ablageort::an(&wurzel)).expect("zweiter Start scheitert");
}

#[test]
fn der_auslieferungszustand_der_sitzung_erfuellt_c1() {
    let sitzung = Sitzung::default();
    for seite in Fensterseite::ALLE {
        let fenster = sitzung.fenster(seite);
        assert!(
            !fenster.tabs.is_empty(),
            "{seite:?} hat keinen Tab; C1 verlangt mindestens einen"
        );
        assert!(
            fenster.aktiver_tab().is_some(),
            "{seite:?} hat keinen sichtbaren Tab"
        );
    }
    assert!(sitzung.sichtbar.lesezeichen);
    assert!(sitzung.sichtbar.zweites_dateifenster);
    assert!(sitzung.sichtbar.vorschau);
}

// ---------------------------------------------------------------------------
// Rundlauf: schreiben und wiedereinlesen
// ---------------------------------------------------------------------------

#[test]
fn alle_drei_dateien_ueberstehen_schreiben_und_wiedereinlesen() {
    let (_ordner, ablage) = ablage("rundlauf");

    let belegung = beispielbelegung();
    let lesezeichen = beispiellesezeichen();
    let sitzung = beispielsitzung();

    ablage
        .sichern(Datei::Belegung, &belegung)
        .expect("keymap.toml laesst sich nicht schreiben");
    ablage
        .sichern(Datei::Lesezeichen, &lesezeichen)
        .expect("bookmarks.toml laesst sich nicht schreiben");
    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("session.toml laesst sich nicht schreiben");

    for welche in Datei::ALLE {
        assert!(
            ablage.pfad(welche).is_file(),
            "{} liegt nicht im Ablageordner",
            welche.dateiname()
        );
    }

    let zurueck_belegung: Geladen<BelegungStellvertreter> = ablage.laden(Datei::Belegung);
    let zurueck_lesezeichen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    let zurueck_sitzung: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert!(!zurueck_belegung.ist_ersetzt());
    assert!(!zurueck_lesezeichen.ist_ersetzt());
    assert!(!zurueck_sitzung.ist_ersetzt());

    assert_eq!(zurueck_belegung.wert, belegung);
    assert_eq!(zurueck_lesezeichen.wert, lesezeichen);
    assert_eq!(zurueck_sitzung.wert, sitzung);
}

#[test]
fn die_geschriebene_sitzung_ist_lesbares_toml() {
    let (_ordner, ablage) = ablage("lesbar");
    ablage
        .sichern(Datei::Sitzung, &beispielsitzung())
        .expect("schreiben gescheitert");

    let text = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");

    // Der Nutzer soll die Datei aufmachen und verstehen koennen; das ist der
    // Grund, aus dem `### Frage 4` TOML waehlt und keine Property-Liste.
    assert!(text.contains("aktiv = \"rechts\""), "{text}");
    assert!(text.contains("[[fenster]]"), "{text}");
    assert!(text.contains("schluessel = \"geaendert\""), "{text}");
    // Die Breite der Vorschau ist nicht gesetzt und steht deshalb gar nicht
    // da; die eine verbleibende Zeile ist ihre Sichtbarkeit.
    let vorschauzeilen: Vec<&str> = text
        .lines()
        .filter(|zeile| zeile.starts_with("vorschau ="))
        .collect();
    assert_eq!(vorschauzeilen, ["vorschau = false"], "{text}");
}

// ---------------------------------------------------------------------------
// Das Fenster- und Tabmodell (Schritt 12)
// ---------------------------------------------------------------------------

/// Jedes Feld des Fenster- und Tabmodells kommt so zurueck, wie es hineinging.
///
/// Die Pruefung nennt die Felder einzeln, statt sich auf den Vergleich der
/// ganzen Sitzung zu verlassen: ein Feld, das beim Schreiben oder beim Lesen
/// verloren geht, faellt hier mit seinem Namen auf.
#[test]
fn das_fenster_und_tabmodell_ueberlebt_schreiben_und_wiedereinlesen() {
    let (_ordner, ablage) = ablage("tabmodell");
    let vorher = beispielsitzung();
    ablage
        .sichern(Datei::Sitzung, &vorher)
        .expect("schreiben gescheitert");

    let nachher = gelesene_sitzung(&ablage);

    assert_eq!(nachher.aktiv, vorher.aktiv, "das aktive Dateifenster");
    assert_eq!(nachher.sichtbar, vorher.sichtbar, "die Sichtbarkeit");
    assert_eq!(nachher.breiten, vorher.breiten, "die Breiten");
    for seite in Fensterseite::ALLE {
        let da = nachher.fenster(seite);
        let war = vorher.fenster(seite);
        assert_eq!(da.aktiver_tab, war.aktiver_tab, "{seite:?}: sichtbarer Tab");
        assert_eq!(da.tabs.len(), war.tabs.len(), "{seite:?}: Zahl der Tabs");
        for (stelle, (jetzt, damals)) in da.tabs.iter().zip(&war.tabs).enumerate() {
            assert_eq!(
                jetzt.ordner, damals.ordner,
                "{seite:?}, Tab {stelle}: Ordner"
            );
            assert_eq!(
                jetzt.auswahl, damals.auswahl,
                "{seite:?}, Tab {stelle}: Auswahl"
            );
            assert_eq!(
                jetzt.verstecke_ausgeblendet, damals.verstecke_ausgeblendet,
                "{seite:?}, Tab {stelle}: versteckte Eintraege"
            );
            assert_eq!(
                jetzt.sortierung, damals.sortierung,
                "{seite:?}, Tab {stelle}: Sortierung"
            );
            assert_eq!(
                jetzt.bildlauf, damals.bildlauf,
                "{seite:?}, Tab {stelle}: Bildlaufposition"
            );
        }
    }
}

/// Eine `session.toml` aus der Zeit vor der Bildlaufposition bleibt lesbar.
///
/// Jede Struktur des Sitzungszustands traegt `#[serde(default)]`, damit ein
/// neues Feld eine aeltere Datei nicht ungueltig macht. Schritt 12 ist der
/// erste, der ein Feld hinzufuegt; ohne diese Pruefung stuende die Zusage nur
/// im Modulkopf.
#[test]
fn eine_sitzung_ohne_bildlaufposition_bleibt_lesbar() {
    let (_ordner, ablage) = ablage("altbestand");
    let alt = "\
aktiv = \"rechts\"

[[fenster]]
aktiver_tab = 0

[[fenster.tabs]]
ordner = \"/Users/pruefung/Projekte\"
verstecke_ausgeblendet = true

[[fenster]]
aktiver_tab = 0
";
    fs::write(ablage.pfad(Datei::Sitzung), alt).expect("schreiben gescheitert");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert!(
        !geladen.ist_ersetzt(),
        "die alte Datei gilt als beschaedigt: {:?}",
        geladen.ersetzung
    );
    let tab = &geladen.wert.fenster(Fensterseite::Links).tabs[0];
    assert_eq!(tab.ordner, PathBuf::from("/Users/pruefung/Projekte"));
    assert_eq!(
        tab.bildlauf, 0.0,
        "das neue Feld nimmt seinen Vorgabewert an"
    );
    assert_eq!(geladen.wert.aktiv, Fensterseite::Rechts);
}

/// Ein Tab mehr im Fenster, und die Datei traegt ihn.
///
/// C1 laesst beliebig viele Tabs je Dateifenster zu; die Serialisierung darf
/// nicht bei einem stehen bleiben.
#[test]
fn ein_dateifenster_traegt_beliebig_viele_tabs() {
    let (_ordner, ablage) = ablage("viele-tabs");
    let mut sitzung = Sitzung::default();
    sitzung.fenster_mut(Fensterseite::Links).tabs = vec![
        Tab::auf("/eins"),
        Tab::auf("/zwei"),
        Tab::auf("/drei"),
        Tab::auf("/vier"),
    ];
    sitzung.fenster_mut(Fensterseite::Links).aktiver_tab = 2;
    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("schreiben gescheitert");

    let zurueck = gelesene_sitzung(&ablage);
    let fenster = zurueck.fenster(Fensterseite::Links);
    assert_eq!(fenster.tabs.len(), 4);
    assert_eq!(fenster.aktiver_tab, 2);
    assert_eq!(
        fenster.aktiver_tab().map(|tab| tab.ordner.clone()),
        Some(PathBuf::from("/drei"))
    );
}

// ---------------------------------------------------------------------------
// Beschaedigt und nicht lesbar
// ---------------------------------------------------------------------------

/// Weder gueltiges TOML noch etwas, das sich reparieren liesse.
const KAPUTT: &str = "dies = ist [kein gueltiges TOML\n";

#[test]
fn eine_kaputte_datei_fuehrt_zum_auslieferungszustand_und_zu_einer_meldung() {
    let (_ordner, ablage) = ablage("kaputt");
    for welche in Datei::ALLE {
        fs::write(ablage.pfad(welche), KAPUTT).expect("schreiben gescheitert");
    }

    let belegung: Geladen<BelegungStellvertreter> = ablage.laden(Datei::Belegung);
    let lesezeichen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    let sitzung: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert_eq!(belegung.wert, BelegungStellvertreter::default());
    assert_eq!(lesezeichen.wert, Lesezeichenliste::default());
    assert_eq!(sitzung.wert, Sitzung::default());

    for (welche, ersetzung) in [
        (Datei::Belegung, belegung.ersetzung),
        (Datei::Lesezeichen, lesezeichen.ersetzung),
        (Datei::Sitzung, sitzung.ersetzung),
    ] {
        pruefe_meldung(&ablage, welche, ersetzung, true);

        // Die kaputte Datei bleibt liegen. `keymap.toml` aendert der Nutzer von
        // Hand; ein Tippfehler darf seine Arbeit nicht loeschen.
        assert_eq!(
            fs::read_to_string(ablage.pfad(welche)).expect("lesen gescheitert"),
            KAPUTT,
            "{} wurde ueberschrieben",
            welche.dateiname()
        );
    }
}

#[test]
fn gueltiges_toml_mit_falscher_gestalt_gilt_ebenfalls_als_beschaedigt() {
    let (_ordner, ablage) = ablage("gestalt");

    // Gueltiges TOML, aber nur ein Dateifenster. C1 kennt zwei.
    fs::write(
        ablage.pfad(Datei::Sitzung),
        "aktiv = \"links\"\n\n[[fenster]]\naktiver_tab = 0\n",
    )
    .expect("schreiben gescheitert");

    let sitzung: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    assert_eq!(sitzung.wert, Sitzung::default());
    pruefe_meldung(&ablage, Datei::Sitzung, sitzung.ersetzung, true);
}

#[test]
fn eine_nicht_lesbare_datei_fuehrt_ebenso_zum_auslieferungszustand() {
    let (_ordner, ablage) = ablage("nichtlesbar");

    // Ein Ordner an der Stelle der Datei. Das Lesen scheitert damit mit einem
    // anderen Fehler als "nicht vorhanden", und zwar unabhaengig davon, unter
    // welchem Benutzer die Pruefung laeuft; ein entzogenes Leserecht taete
    // dasselbe, aber nicht fuer root.
    fs::create_dir(ablage.pfad(Datei::Lesezeichen)).expect("Ordner laesst sich nicht anlegen");

    let lesezeichen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    assert_eq!(lesezeichen.wert, Lesezeichenliste::default());
    pruefe_meldung(&ablage, Datei::Lesezeichen, lesezeichen.ersetzung, false);
}

/// Prueft, dass eine Ersetzung gemeldet wird und die Datei benennt.
fn pruefe_meldung(ablage: &Ablage, welche: Datei, ersetzung: Option<Ersetzung>, beschaedigt: bool) {
    let ersetzung =
        ersetzung.unwrap_or_else(|| panic!("{} wurde ohne Meldung ersetzt", welche.dateiname()));
    assert_eq!(ersetzung.datei, ablage.pfad(welche));
    if beschaedigt {
        assert!(
            matches!(ersetzung.grund, Grund::Beschaedigt(_)),
            "{ersetzung:?}"
        );
    } else {
        assert!(
            matches!(ersetzung.grund, Grund::NichtLesbar(_)),
            "{ersetzung:?}"
        );
    }
    assert!(
        !ersetzung.grund.einzelheit().is_empty(),
        "die Meldung nennt keinen Grund"
    );

    let text = ersetzung.to_string();
    assert!(
        text.contains(welche.dateiname()),
        "die Meldung benennt die Datei nicht: {text}"
    );
    assert!(
        text.contains("Auslieferungszustand"),
        "die Meldung nennt die Ersetzung nicht: {text}"
    );
    assert!(!text.contains('\n'), "die Meldung ist mehrzeilig: {text}");
}

// ---------------------------------------------------------------------------
// Gebuendeltes Schreiben des Sitzungszustands
// ---------------------------------------------------------------------------

fn gelesene_sitzung(ablage: &Ablage) -> Sitzung {
    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    assert!(!geladen.ist_ersetzt(), "session.toml ist beschaedigt");
    geladen.wert
}

#[test]
fn der_takt_ist_zwei_sekunden() {
    assert_eq!(SITZUNGSTAKT, Duration::from_secs(2));
}

#[test]
fn der_sitzungsschreiber_buendelt_auf_hoechstens_zwei_sekunden() {
    let (_ordner, ablage) = ablage("takt");
    let mut schreiber = ablage.sitzungsschreiber();
    let start = Instant::now();

    let erste = beispielsitzung();
    assert!(
        schreiber
            .vormerken(erste.clone(), start)
            .expect("schreiben gescheitert"),
        "der erste Stand soll sofort auf die Platte"
    );
    assert_eq!(gelesene_sitzung(&ablage), erste);
    assert!(!schreiber.steht_aus());

    let mut zweite = erste.clone();
    zweite.aktiv = Fensterseite::Links;
    let mut dritte = zweite.clone();
    dritte.breiten.vorschau = Some(240.0);

    for (stand, versatz) in [(&zweite, 500u64), (&dritte, 1_999)] {
        assert!(
            !schreiber
                .vormerken(stand.clone(), start + Duration::from_millis(versatz))
                .expect("schreiben gescheitert"),
            "nach {versatz} ms darf noch nicht geschrieben werden"
        );
    }
    assert!(schreiber.steht_aus(), "der Stand ist nicht vorgemerkt");
    assert_eq!(
        gelesene_sitzung(&ablage),
        erste,
        "auf der Platte steht nicht mehr der erste Stand"
    );

    // Nach dem Takt geht der letzte vorgemerkte Stand raus, und nur er: die
    // beiden Zwischenstaende sind gebuendelt und nicht einzeln geschrieben.
    assert!(
        schreiber
            .vormerken(dritte.clone(), start + Duration::from_millis(2_000))
            .expect("schreiben gescheitert")
    );
    assert_eq!(gelesene_sitzung(&ablage), dritte);
    assert!(!schreiber.steht_aus());
}

#[test]
fn ein_liegengebliebener_stand_geht_ueber_den_takt_hinaus() {
    let (_ordner, ablage) = ablage("abgleich");
    let mut schreiber = ablage.sitzungsschreiber();
    let start = Instant::now();

    let erste = beispielsitzung();
    schreiber
        .vormerken(erste.clone(), start)
        .expect("schreiben gescheitert");

    let mut zweite = erste.clone();
    zweite.aktiv = Fensterseite::Links;
    schreiber
        .vormerken(zweite.clone(), start + Duration::from_millis(10))
        .expect("schreiben gescheitert");

    // Ohne weitere Aenderung: der Takt allein traegt den Stand nach.
    assert!(
        !schreiber
            .abgleichen(start + Duration::from_millis(1_000))
            .expect("schreiben gescheitert")
    );
    assert!(
        schreiber
            .abgleichen(start + Duration::from_secs(3))
            .expect("schreiben gescheitert")
    );
    assert_eq!(gelesene_sitzung(&ablage), zweite);

    // Und ohne vorgemerkten Stand tut der Takt nichts.
    assert!(
        !schreiber
            .abgleichen(start + Duration::from_secs(9))
            .expect("schreiben gescheitert")
    );
}

#[test]
fn beim_beenden_wird_der_letzte_stand_genau_einmal_geschrieben() {
    let (_ordner, ablage) = ablage("beenden");
    let mut schreiber = ablage.sitzungsschreiber();
    let start = Instant::now();

    let erste = beispielsitzung();
    schreiber
        .vormerken(erste.clone(), start)
        .expect("schreiben gescheitert");

    let mut letzte = erste.clone();
    letzte.fenster_mut(Fensterseite::Links).aktiver_tab = 0;
    assert!(
        !schreiber
            .vormerken(letzte.clone(), start + Duration::from_millis(100))
            .expect("schreiben gescheitert"),
        "der Takt ist noch nicht abgelaufen"
    );

    // Das Beenden schreibt ohne Ruecksicht auf den Takt.
    assert!(
        schreiber
            .beenden(start + Duration::from_millis(101))
            .expect("schreiben gescheitert")
    );
    assert_eq!(gelesene_sitzung(&ablage), letzte);

    // Und ein zweites Mal beenden schreibt nicht noch einmal.
    assert!(
        !schreiber
            .beenden(start + Duration::from_millis(102))
            .expect("schreiben gescheitert")
    );
}

// ---------------------------------------------------------------------------
// Atomares Schreiben
// ---------------------------------------------------------------------------

#[test]
fn die_nachbardatei_liegt_neben_dem_ziel_und_verschwindet_nach_dem_umbenennen() {
    let ordner = Pruefordner::neu("nachbar");
    let ziel = ordner.pfad().join("session.toml");
    fs::write(&ziel, "alt = true\n").expect("schreiben gescheitert");

    let nachbar = atomar::nachbarpfad(&ziel).expect("kein Nachbarpfad");
    assert_eq!(nachbar.parent(), ziel.parent());
    assert_eq!(
        nachbar.file_name().and_then(|name| name.to_str()),
        Some("session.toml.neu")
    );

    let vorbereitet = atomar::vorbereiten(&ziel, "neu = true\n").expect("vorbereiten gescheitert");
    assert!(vorbereitet.nachbarpfad().is_file());
    assert_eq!(
        fs::read_to_string(&ziel).expect("lesen gescheitert"),
        "alt = true\n",
        "das Ziel ist vor dem Umbenennen schon veraendert"
    );

    vorbereitet.umbenennen().expect("umbenennen gescheitert");
    assert_eq!(
        fs::read_to_string(&ziel).expect("lesen gescheitert"),
        "neu = true\n"
    );
    assert!(!nachbar.exists(), "die Nachbardatei liegt noch da");
}

#[test]
fn eine_fallengelassene_nachbardatei_raeumt_sich_ab() {
    let ordner = Pruefordner::neu("fallen");
    let ziel = ordner.pfad().join("session.toml");
    fs::write(&ziel, "alt = true\n").expect("schreiben gescheitert");
    let nachbar = atomar::nachbarpfad(&ziel).expect("kein Nachbarpfad");

    drop(atomar::vorbereiten(&ziel, "neu = true\n").expect("vorbereiten gescheitert"));

    assert!(!nachbar.exists(), "die Nachbardatei liegt noch da");
    assert_eq!(
        fs::read_to_string(&ziel).expect("lesen gescheitert"),
        "alt = true\n"
    );
}

// ---------------------------------------------------------------------------
// Die Pruefung mit eigenem Prozess
// ---------------------------------------------------------------------------

/// Die Umgebungsvariable, die die Abbruch-Kindprobe beauftragt. Ihr Wert ist
/// die Zieldatei.
const AUFTRAG_ABBRUCH: &str = "KRK_PROBE_ABBRUCH";

/// Der Inhalt, den das sterbende Kind schreibt.
const KINDINHALT: &str = "# vom Kind geschrieben, nie umbenannt\naktiv = \"rechts\"\n";

/// Das Signal, mit dem `std::process::abort` den Prozess beendet.
const SIGABRT: i32 = 6;

/// Startet dieselbe Testdatei noch einmal und laesst genau eine Kindprobe
/// laufen.
fn kindprobe(name: &str, auftrag: &str, wert: &Path) -> Output {
    let selbst = std::env::current_exe().expect("die Testdatei kennt ihren Pfad nicht");
    Command::new(selbst)
        .args(["--exact", "--ignored", "--nocapture", "--test-threads", "1"])
        .arg(name)
        .env(auftrag, wert)
        .output()
        .expect("die Kindprobe laesst sich nicht starten")
}

#[test]
fn ein_abbruch_zwischen_schreiben_und_umbenennen_laesst_die_alte_datei_unveraendert() {
    let (_ordner, ablage) = ablage("abbruch");
    let ziel = ablage.pfad(Datei::Sitzung);
    let alt = beispielsitzung();
    ablage
        .sichern(Datei::Sitzung, &alt)
        .expect("schreiben gescheitert");
    let alter_text = fs::read_to_string(&ziel).expect("lesen gescheitert");

    let ergebnis = kindprobe(
        "kind_stirbt_zwischen_schreiben_und_umbenennen",
        AUFTRAG_ABBRUCH,
        &ziel,
    );

    // Das Kind ist wirklich gestorben, und zwar mitten im Lauf.
    use std::os::unix::process::ExitStatusExt;
    assert_eq!(
        ergebnis.status.signal(),
        Some(SIGABRT),
        "das Kind ist nicht abgestuerzt, sondern zurueckgekehrt: {:?}\n{}",
        ergebnis.status,
        String::from_utf8_lossy(&ergebnis.stderr)
    );

    // Es war ueber das Schreiben hinaus: die Nachbardatei traegt seinen Inhalt,
    // und `Drop` hat sie nicht abgeraeumt, weil ein Absturz kein `Drop` kennt.
    // Ohne diese Zusicherung traege die Pruefung nur den Namen des Falls: sie
    // koennte auch bestehen, wenn das Kind vor dem Schreiben gestorben waere.
    let nachbar = atomar::nachbarpfad(&ziel).expect("kein Nachbarpfad");
    assert!(
        nachbar.is_file(),
        "das Kind ist gestorben, bevor es geschrieben hat"
    );
    assert_eq!(
        fs::read_to_string(&nachbar).expect("lesen gescheitert"),
        KINDINHALT
    );

    // Und das Ziel ist Byte fuer Byte das alte.
    assert_eq!(
        fs::read_to_string(&ziel).expect("lesen gescheitert"),
        alter_text
    );
    assert_eq!(gelesene_sitzung(&ablage), alt);

    // Die liegengebliebene Nachbardatei stoert den naechsten Lauf nicht.
    let mut neu = alt.clone();
    neu.aktiv = Fensterseite::Links;
    ablage
        .sichern(Datei::Sitzung, &neu)
        .expect("schreiben gescheitert");
    assert_eq!(gelesene_sitzung(&ablage), neu);
    assert!(
        !nachbar.exists(),
        "der naechste Schreibvorgang hat die Nachbardatei nicht mitgenommen"
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_PROBE_ABBRUCH gestartet"]
fn kind_stirbt_zwischen_schreiben_und_umbenennen() {
    let Some(ziel) = std::env::var_os(AUFTRAG_ABBRUCH) else {
        return;
    };
    let vorbereitet =
        atomar::vorbereiten(Path::new(&ziel), KINDINHALT).expect("vorbereiten gescheitert");
    assert!(vorbereitet.nachbarpfad().is_file());

    // Genau hier liegt die Luecke: geschrieben ist, umbenannt ist nicht.
    // `abort` fuehrt kein `Drop` aus und laesst dem Prozess keine Gelegenheit
    // aufzuraeumen; das ist der Absturz, den die Zusage meint.
    std::process::abort();
}

/// Der Kern gibt nichts aus; er liefert den Satz und laesst den Aufrufer
/// entscheiden.
///
/// Die Vorgaengerin dieser Pruefung startete einen Kindprozess und las seine
/// Standardfehlerausgabe mit, weil `melden` dorthin schrieb. Seit Schritt 12
/// tut es das nicht mehr: der Nutzer hat am 260804-0830 die Statuszeile
/// gewaehlt, und ein ueber den Finder gestartetes Buendel hat ohnehin keine
/// Standardfehlerausgabe. Die Zusage lautet jetzt umgekehrt, und sie ist ohne
/// Kindprozess pruefbar.
#[test]
fn die_ersetzung_kommt_als_text_zurueck_und_landet_auf_keinem_kanal() {
    let (_ordner, ablage) = ablage("meldung");
    fs::write(ablage.pfad(Datei::Sitzung), KAPUTT).expect("schreiben gescheitert");

    let (sitzung, meldung) = ablage.laden::<Sitzung>(Datei::Sitzung).mit_meldung();

    assert_eq!(sitzung, Sitzung::default());
    let meldung = meldung.expect("eine beschaedigte Datei muss eine Meldung tragen");
    assert!(meldung.contains("session.toml"), "{meldung}");
    assert!(meldung.contains("ist beschaedigt"), "{meldung}");
    assert!(meldung.contains("Auslieferungszustand"), "{meldung}");
    assert!(
        !meldung.starts_with("krk: "),
        "der Programmname gehoert in ein Terminal und nicht in die Statuszeile: {meldung}"
    );

    // Eine heile Datei liefert keinen Satz, den jemand anzeigen muesste.
    ablage
        .sichern(Datei::Sitzung, &beispielsitzung())
        .expect("schreiben gescheitert");
    let (_, keine) = ablage.laden::<Sitzung>(Datei::Sitzung).mit_meldung();
    assert_eq!(keine, None);
}

// ---------------------------------------------------------------------------
// Die Lesezeichen aus C5 (Schritt 18)
// ---------------------------------------------------------------------------

/// Anlegen, Umbenennen, Loeschen und Reihenfolge ueberleben Schreiben und
/// Wiedereinlesen.
///
/// Das ist die Zusage "Die Lesezeichen ueberleben Beenden und Neustart" aus C5,
/// gemessen an derselben Datei, die das laufende Programm schreibt: die vier
/// Aenderungen laufen ueber [`Lesezeichenliste`], und was danach in
/// `bookmarks.toml` steht, wird zurueckgelesen und verglichen.
#[test]
fn die_vier_aenderungen_an_den_lesezeichen_ueberleben_einen_neustart() {
    let (_ordner, ablage) = ablage("lesezeichen");

    let mut liste = Lesezeichenliste::default();
    assert_eq!(
        liste.anlegen("Projekte", Path::new("/Users/pruefung/Projekte")),
        0
    );
    assert_eq!(
        liste.anlegen("Sicherung", Path::new("/Volumes/Sicherung")),
        1
    );
    assert_eq!(liste.anlegen("Wurzel", Path::new("/")), 2);
    assert!(liste.umbenennen(1, "Sicherungsplatte"));
    assert!(liste.loeschen(0));
    assert_eq!(liste.verschieben(1, Verschiebung::Hoch), Some(0));

    ablage
        .sichern(Datei::Lesezeichen, &liste)
        .expect("bookmarks.toml laesst sich nicht schreiben");

    // Der Neustart: eine zweite Ablage auf demselben Ordner liest die Datei so,
    // wie das Programm sie beim naechsten Start liest.
    let wieder = Ablage::oeffnen(Ablageort::an(ablage.ort().wurzel()))
        .expect("die Ablage laesst sich nicht ein zweites Mal oeffnen");
    let gelesen: Geladen<Lesezeichenliste> = wieder.laden(Datei::Lesezeichen);

    assert!(!gelesen.ist_ersetzt());
    assert_eq!(gelesen.wert, liste);
    assert_eq!(
        gelesen
            .wert
            .eintraege
            .iter()
            .map(|eintrag| eintrag.name.as_str())
            .collect::<Vec<&str>>(),
        ["Wurzel", "Sicherungsplatte"],
        "die Reihenfolge der Leiste ist die Reihenfolge der Datei"
    );
}

/// Ein Lesezeichen auf einen verschwundenen Ordner ist ungueltig, eines auf
/// einen vorhandenen nicht (C5).
#[test]
fn ein_lesezeichen_kennt_den_zustand_seines_ordners() {
    let ordner = Pruefordner::neu("gueltigkeit");
    let datei = ordner.pfad().join("keine-ordner");
    fs::write(&datei, b"").expect("die Pruefdatei laesst sich nicht schreiben");

    assert!(Lesezeichen::neu("Da", ordner.pfad()).gueltig());
    assert!(!Lesezeichen::neu("Weg", ordner.pfad().join("fort")).gueltig());
    assert!(
        !Lesezeichen::neu("Datei", &datei).gueltig(),
        "eine Datei ist kein Ordner und kein Ziel fuer ein Lesezeichen"
    );
}
