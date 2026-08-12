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
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use krk_core::ablage::sitzung::SITZUNGSTAKT;
use krk_core::ablage::{
    Ablage, Ablageort, Beiseite, Breiten, Datei, Dateifenster, Einstellungen, Ersetzung,
    Fensterseite, Geladen, Grund, Lesezeichen, Lesezeichenliste, Sichtbarkeit, Sitzung,
    Spaltensichtbarkeit, Tab, Verschiebung, Ziel, atomar, einstellungen, pfade,
};
use krk_core::verzeichnis::{Richtung, Schluessel, Sortierung};

mod gemeinsam;
use gemeinsam::Pruefordner;

// ---------------------------------------------------------------------------
// Stellvertreter
// ---------------------------------------------------------------------------

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
/// Modul; die Ablage kennt von dieser Datei nur ihren Namen und ihren Weg.
/// Damit die Zusage "alle vier Dateien" trotzdem an vier Dateien geprueft wird
/// und nicht an dreien, laeuft dieser Weg hier mit dem Stellvertreter, ueber
/// dieselbe Ablage und denselben Pfad.
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
        editor: Some(PathBuf::from("/Users/pruefung/Projekte/notiz.md")),
        breiten: Breiten {
            lesezeichen: Some(180.0),
            links: Some(520.5),
            rechts: Some(520.5),
            vorschau: None,
            editor: Some(480.0),
        },
        sichtbar: Sichtbarkeit {
            lesezeichen: false,
            erstes_dateifenster: false,
            zweites_dateifenster: true,
            vorschau: false,
            editor: true,
        },
        spalten: Spaltensichtbarkeit {
            groesse: false,
            geaendert: true,
            typ: false,
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
    assert_eq!(
        namen,
        [
            "keymap.toml",
            "bookmarks.toml",
            "session.toml",
            "settings.toml"
        ]
    );
}

/// Die Kuerzung fuer Meldungen, ueber ihre vier Faelle und den einen, den ein
/// Vergleich auf Zeichenketten falsch beantwortet.
///
/// Kein Fall fasst das echte Benutzerverzeichnis an: die Funktion nimmt es als
/// Argument, und genau dafuer nimmt sie es.
#[test]
fn die_kuerzung_fuer_meldungen_zieht_nur_ganze_pfadbestandteile_ab() {
    let zuhause = Path::new("/Users/kai");

    // Unter dem Benutzerverzeichnis: `~/` und der Rest.
    assert_eq!(
        pfade::gekuerzt_fuer_anzeige(
            Path::new("/Users/kai/Downloads/KRK-Tastenbelegung.md"),
            Some(zuhause)
        ),
        "~/Downloads/KRK-Tastenbelegung.md"
    );

    // Das Benutzerverzeichnis selbst wird zu `~`, ohne angehaengten Schraegstrich.
    assert_eq!(pfade::gekuerzt_fuer_anzeige(zuhause, Some(zuhause)), "~");

    // Ausserhalb: ausgeschrieben, Zeichen fuer Zeichen der Eingabe.
    assert_eq!(
        pfade::gekuerzt_fuer_anzeige(Path::new("/Volumes/Sicherung/Belegung.md"), Some(zuhause)),
        "/Volumes/Sicherung/Belegung.md"
    );

    // Ohne uebergebenes Benutzerverzeichnis: ausgeschrieben. Kein Fehler.
    assert_eq!(
        pfade::gekuerzt_fuer_anzeige(Path::new("/Users/kai/Downloads"), None),
        "/Users/kai/Downloads"
    );

    // Der Fall, den ein Vergleich auf Zeichenketten falsch beantwortet:
    // `/Users/kai` ist ein Praefix der Bytes von `/Users/kai-alt`, aber kein
    // Praefix seiner Pfadbestandteile. `~-alt/Downloads` waere die falsche
    // Antwort, und sie waere ein Pfad, den es nicht gibt.
    assert_eq!(
        pfade::gekuerzt_fuer_anzeige(Path::new("/Users/kai-alt/Downloads"), Some(zuhause)),
        "/Users/kai-alt/Downloads"
    );
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
    assert!(
        sitzung.sichtbar.erstes_dateifenster,
        "ab Werk stehen beide Dateifenster; C1 verlangt zwei"
    );
    assert!(sitzung.sichtbar.zweites_dateifenster);
    assert!(sitzung.sichtbar.vorschau);
    assert!(
        !sitzung.sichtbar.editor,
        "der Editor haelt beim allerersten Start keine Datei und ist ausgeblendet"
    );
}

// ---------------------------------------------------------------------------
// Rundlauf: schreiben und wiedereinlesen
// ---------------------------------------------------------------------------

/// Der Rundlauf jeder Ablagedatei, `settings.toml` eingeschlossen.
///
/// Die vierte geht als einzige nicht ueber [`Ablage::sichern`]: sie wird von
/// Hand gepflegt, und ihr Schreibweg ist das atomare Schreiben eines Textes.
/// Geprueft wird derselbe Rundlauf, allein die Nutzlast ist eine andere.
#[test]
fn alle_vier_dateien_ueberstehen_schreiben_und_wiedereinlesen() {
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
    atomar::schreiben(
        &ablage.pfad(Datei::Einstellungen),
        "terminal = \"com.mitchellh.ghostty\"\n",
    )
    .expect("settings.toml laesst sich nicht schreiben");

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

    let zurueck_einstellungen = einstellungen::laden(&ablage);
    assert!(!zurueck_einstellungen.ist_ersetzt());
    assert_eq!(
        zurueck_einstellungen.wert.terminal, "com.mitchellh.ghostty",
        "der Rundlauf hat den eingestellten Terminal-Eintrag veraendert"
    );
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
    assert_eq!(nachher.spalten, vorher.spalten, "die Spaltensichtbarkeit");
    assert_eq!(nachher.editor, vorher.editor, "die Datei des Editors");
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

/// Eine `session.toml` aus der Zeit vor dem Editor bleibt lesbar.
///
/// Die Datei tritt so auf, wie die Runde 1 sie geschrieben hat: mit
/// `[breiten]` und `[sichtbar]`, aber ohne die beiden Editorfelder. Sie gilt
/// nicht als beschaedigt, und der Nutzer verliert weder Breiten noch
/// Sichtbarkeit.
#[test]
fn eine_sitzung_ohne_die_editorfelder_bleibt_lesbar() {
    let (_ordner, ablage) = ablage("vor-dem-editor");
    let alt = "\
aktiv = \"links\"

[breiten]
lesezeichen = 180.0
links = 420.0
rechts = 420.0
vorschau = 260.0

[sichtbar]
lesezeichen = true
zweites_dateifenster = true
vorschau = true

[[fenster]]
aktiver_tab = 0

[[fenster]]
aktiver_tab = 0
";
    fs::write(ablage.pfad(Datei::Sitzung), alt).expect("schreiben gescheitert");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert!(
        !geladen.ist_ersetzt(),
        "die Datei der Runde 1 gilt als beschaedigt: {:?}",
        geladen.ersetzung
    );
    assert!(
        !geladen.wert.sichtbar.editor,
        "der Editor ist ohne eigenes Feld ausgeblendet und nicht sichtbar"
    );
    assert_eq!(
        geladen.wert.breiten.editor, None,
        "eine nie gesetzte Editorbreite bleibt ungesetzt"
    );
    assert_eq!(
        geladen.wert.editor, None,
        "ohne das Feld haelt der Editor keine Datei"
    );
    assert_eq!(geladen.wert.breiten.vorschau, Some(260.0));
    assert!(geladen.wert.sichtbar.vorschau);
    assert!(geladen.wert.sichtbar.lesezeichen);
    assert!(
        geladen.wert.sichtbar.erstes_dateifenster,
        "ohne eigenes Feld steht das linke Dateifenster"
    );
}

/// Eine `session.toml` aus der Zeit vor der Bereichsleisten-Runde bleibt
/// lesbar, und das fehlende Feld heisst "sichtbar".
///
/// Bis zu dieser Runde liess sich das linke Dateifenster nicht ausblenden, und
/// `[sichtbar]` trug gar keine Zeile dafuer. Eine Datei aus jener Zeit darf
/// weder als beschaedigt gelten noch mit einem ausgeblendeten linken
/// Dateifenster aufgehen; der Vorgabewert des neuen Feldes ist `true`.
#[test]
fn eine_sitzung_ohne_das_erste_dateifenster_bleibt_lesbar() {
    let (_ordner, ablage) = ablage("vor-der-bereichsleiste");
    let alt = "\
aktiv = \"rechts\"

[breiten]
lesezeichen = 180.0
links = 420.0
rechts = 420.0

[sichtbar]
lesezeichen = true
zweites_dateifenster = false
vorschau = true
editor = false

[[fenster]]
aktiver_tab = 0

[[fenster]]
aktiver_tab = 0
";
    fs::write(ablage.pfad(Datei::Sitzung), alt).expect("schreiben gescheitert");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert!(
        !geladen.ist_ersetzt(),
        "die Datei vor der Bereichsleisten-Runde gilt als beschaedigt: {:?}",
        geladen.ersetzung
    );
    assert!(
        geladen.wert.sichtbar.erstes_dateifenster,
        "das fehlende Feld heisst sichtbar und nicht ausgeblendet"
    );
    assert!(
        !geladen.wert.sichtbar.zweites_dateifenster,
        "das rechte war ausgeblendet und bleibt es"
    );
}

/// Das ausgeblendete linke Dateifenster uebersteht den Rundlauf byteweise.
///
/// Derselbe Weg wie bei den Editorfeldern: zwei Schreibvorgaenge statt eines
/// Strukturvergleichs. Verlore das Schreiben das neue Feld, kaeme es beim Lesen
/// als `true` zurueck, und die zweite Datei unterschiede sich von der ersten.
#[test]
fn das_ausgeblendete_erste_dateifenster_ueberlebt_den_rundlauf_byteweise() {
    let (_ordner, ablage) = ablage("erstes-dateifenster");
    let mut sitzung = Sitzung::default();
    sitzung.sichtbar.erstes_dateifenster = false;
    sitzung.aktiv = Fensterseite::Rechts;

    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("schreiben gescheitert");
    let zuerst = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert!(
        zuerst.contains("erstes_dateifenster = false"),
        "das Feld steht nicht in der Datei, die der Nutzer nach C7 von Hand liest: {zuerst}"
    );

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    assert!(!geladen.ist_ersetzt());
    assert!(!geladen.wert.sichtbar.erstes_dateifenster);
    assert!(geladen.wert.sichtbar.zweites_dateifenster);

    ablage
        .sichern(Datei::Sitzung, &geladen.wert)
        .expect("zweites Schreiben gescheitert");
    let danach = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert_eq!(zuerst, danach, "der Rundlauf hat die Datei veraendert");
}

/// Eine `session.toml` ohne den Abschnitt `[spalten]` bleibt lesbar, und die
/// fehlenden Felder heissen "sichtbar".
///
/// Bis zur Bereichsleisten-Runde liessen sich die Spalten nicht schalten, und
/// die Datei trug den Abschnitt gar nicht. Eine Datei aus jener Zeit darf weder
/// als beschaedigt gelten noch mit einer weggeschalteten Spalte aufgehen; der
/// Vorgabewert aller drei Felder ist `true`, also der bisherige Zustand
/// (Kriterium C7.4).
#[test]
fn eine_sitzung_ohne_den_spaltenabschnitt_bleibt_lesbar() {
    let (_ordner, ablage) = ablage("vor-den-spaltenschaltern");
    let alt = "\
aktiv = \"links\"

[breiten]
lesezeichen = 180.0
links = 420.0
rechts = 420.0

[sichtbar]
lesezeichen = true
erstes_dateifenster = true
zweites_dateifenster = true
vorschau = true
editor = false

[[fenster]]
aktiver_tab = 0

[[fenster]]
aktiver_tab = 0
";
    fs::write(ablage.pfad(Datei::Sitzung), alt).expect("schreiben gescheitert");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);

    assert!(
        !geladen.ist_ersetzt(),
        "die Datei vor den Spaltenschaltern gilt als beschaedigt: {:?}",
        geladen.ersetzung
    );
    assert_eq!(
        geladen.wert.spalten,
        Spaltensichtbarkeit::default(),
        "der fehlende Abschnitt heisst: alle drei stehen"
    );
    assert!(geladen.wert.spalten.groesse);
    assert!(geladen.wert.spalten.geaendert);
    assert!(geladen.wert.spalten.typ);
}

/// Die Spaltensichtbarkeit uebersteht den Rundlauf byteweise (Kriterium C7.2).
///
/// Derselbe Weg wie beim ausgeblendeten linken Dateifenster: zwei
/// Schreibvorgaenge statt eines Strukturvergleichs. Verlore das Schreiben ein
/// Feld, kaeme es beim Lesen als `true` zurueck, und die zweite Datei
/// unterschiede sich von der ersten.
#[test]
fn die_spaltensichtbarkeit_ueberlebt_den_rundlauf_byteweise() {
    let (_ordner, ablage) = ablage("spaltensichtbarkeit");
    let mut sitzung = Sitzung::default();
    sitzung.spalten.groesse = false;
    sitzung.spalten.typ = false;

    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("schreiben gescheitert");
    let zuerst = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert!(
        zuerst.contains("[spalten]"),
        "der Abschnitt steht nicht in der Datei, die der Nutzer nach C7 von Hand liest: {zuerst}"
    );
    assert!(zuerst.contains("groesse = false"), "{zuerst}");
    assert!(zuerst.contains("geaendert = true"), "{zuerst}");
    assert!(zuerst.contains("typ = false"), "{zuerst}");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    assert!(!geladen.ist_ersetzt());
    assert_eq!(geladen.wert.spalten, sitzung.spalten);

    ablage
        .sichern(Datei::Sitzung, &geladen.wert)
        .expect("zweites Schreiben gescheitert");
    let danach = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert_eq!(zuerst, danach, "der Rundlauf hat die Datei veraendert");
}

/// Breite und Sichtbarkeit des Editors ueberstehen den Rundlauf byteweise.
///
/// Der Rundlauf geht ueber zwei Schreibvorgaenge und nicht nur ueber einen
/// Vergleich der Strukturen: verlore das Schreiben ein Feld, kaeme es beim
/// Lesen als Vorgabewert zurueck, und die zweite Datei unterschiede sich von
/// der ersten.
#[test]
fn die_editorbreite_ueberlebt_den_rundlauf_byteweise() {
    let (_ordner, ablage) = ablage("editorbreite");
    let mut sitzung = Sitzung::default();
    sitzung.breiten.editor = Some(512.5);
    sitzung.sichtbar.editor = true;
    sitzung.sichtbar.vorschau = false;

    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("schreiben gescheitert");
    let zuerst = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    assert!(!geladen.ist_ersetzt());
    assert_eq!(geladen.wert.breiten.editor, Some(512.5));
    assert!(geladen.wert.sichtbar.editor);

    ablage
        .sichern(Datei::Sitzung, &geladen.wert)
        .expect("zweites Schreiben gescheitert");
    let danach = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert_eq!(zuerst, danach, "der Rundlauf hat die Datei veraendert");
}

/// Eine nicht gesetzte Editorbreite steht gar nicht in der Datei.
///
/// Dieselbe Zusage wie fuer die uebrigen vier Breiten: `None` heisst "noch nie
/// gesetzt", und eine Zeile mit einer erfundenen Zahl waere in einer Datei, die
/// der Nutzer nach C7 von Hand liest, eine Falschaussage.
#[test]
fn eine_nicht_gesetzte_editorbreite_steht_nicht_in_der_datei() {
    let (_ordner, ablage) = ablage("editorbreite-ungesetzt");
    ablage
        .sichern(Datei::Sitzung, &Sitzung::default())
        .expect("schreiben gescheitert");

    let text = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");

    // Die einzige verbleibende Zeile mit diesem Namen ist die Sichtbarkeit.
    let editorzeilen: Vec<&str> = text
        .lines()
        .filter(|zeile| zeile.starts_with("editor ="))
        .collect();
    assert_eq!(editorzeilen, ["editor = false"], "{text}");
}

/// Die geoeffnete Datei des Editors uebersteht den Rundlauf byteweise (C7).
///
/// Der Rundlauf geht ueber zwei Schreibvorgaenge und nicht nur ueber einen
/// Vergleich der Strukturen, aus demselben Grund wie bei der Editorbreite
/// darueber: verlore das Schreiben das Feld, kaeme es beim Lesen als `None`
/// zurueck, und die zweite Datei unterschiede sich von der ersten.
#[test]
fn die_geoeffnete_editordatei_ueberlebt_den_rundlauf_byteweise() {
    let (_ordner, ablage) = ablage("editordatei");
    let sitzung = Sitzung {
        editor: Some(PathBuf::from("/Users/pruefung/Projekte/notiz.md")),
        ..Sitzung::default()
    };

    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("schreiben gescheitert");
    let zuerst = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");

    let geladen: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    assert!(!geladen.ist_ersetzt());
    assert_eq!(
        geladen.wert.editor,
        Some(PathBuf::from("/Users/pruefung/Projekte/notiz.md"))
    );

    ablage
        .sichern(Datei::Sitzung, &geladen.wert)
        .expect("zweites Schreiben gescheitert");
    let danach = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert_eq!(zuerst, danach, "der Rundlauf hat die Datei veraendert");
}

/// Ein gesetzter Editorpfad steht in der Datei, ein nicht gesetzter nicht (C7).
///
/// Dieselbe Zusage wie fuer die fuenf Breiten: `None` heisst "der Editor haelt
/// keine Datei", und eine Zeile mit einem erfundenen Pfad waere in einer Datei,
/// die der Nutzer nach C7 von Hand liest, eine Falschaussage.
///
/// Geprueft wird die **Zeile** und nicht nur der eingelesene Wert: der Name
/// `editor` kommt in `session.toml` dreimal vor, als Breite, als Sichtbarkeit
/// und hier, und allein die Stelle im Text unterscheidet die drei.
#[test]
fn der_editorpfad_steht_nur_dann_in_der_datei_wenn_eine_datei_offen_ist() {
    let (_ordner, ablage) = ablage("editordatei-zeile");

    ablage
        .sichern(Datei::Sitzung, &Sitzung::default())
        .expect("schreiben gescheitert");
    let ohne = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    assert!(
        !ohne.lines().any(|zeile| zeile.starts_with("editor = \"")),
        "ohne geoeffnete Datei steht ein Pfad in session.toml: {ohne}"
    );

    let sitzung = Sitzung {
        editor: Some(PathBuf::from("/Users/pruefung/notiz.md")),
        ..Sitzung::default()
    };
    ablage
        .sichern(Datei::Sitzung, &sitzung)
        .expect("zweites Schreiben gescheitert");
    let mit = fs::read_to_string(ablage.pfad(Datei::Sitzung)).expect("lesen gescheitert");
    let pfadzeilen: Vec<&str> = mit
        .lines()
        .filter(|zeile| zeile.starts_with("editor = \""))
        .collect();
    assert_eq!(
        pfadzeilen,
        ["editor = \"/Users/pruefung/notiz.md\""],
        "{mit}"
    );
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
    let eingestellt = einstellungen::laden(&ablage);

    assert_eq!(belegung.wert, BelegungStellvertreter::default());
    assert_eq!(lesezeichen.wert, Lesezeichenliste::default());
    assert_eq!(sitzung.wert, Sitzung::default());
    assert_eq!(eingestellt.wert, Einstellungen::auslieferung());

    for (welche, ersetzung) in [
        (Datei::Belegung, belegung.ersetzung),
        (Datei::Lesezeichen, lesezeichen.ersetzung),
        (Datei::Sitzung, sitzung.ersetzung),
        (Datei::Einstellungen, eingestellt.ersetzung),
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
// Eine beschaedigte Datei wird zur Seite gelegt (C3 der Runde 6)
// ---------------------------------------------------------------------------

/// Ein zweiter kaputter Inhalt, der sich vom ersten unterscheiden laesst.
const KAPUTT_ZWEITER: &str = "auch = dies [ist kein gueltiges TOML\n";

/// Der Pfad, unter dem die Sicherung einer der vier Dateien zu erwarten ist.
fn beiseitepfad(ablage: &Ablage, welche: Datei) -> PathBuf {
    atomar::beiseitepfad(&ablage.pfad(welche)).expect("kein Beiseitepfad")
}

/// Laedt alle vier Dateien und liefert die vier Ersetzungen in der Reihenfolge
/// von [`Datei::ALLE`].
///
/// Die Belegung geht ueber ihren Stellvertreter, die Einstellungen ueber
/// `einstellungen::laden`; damit laufen alle vier durch denselben
/// `Ablage::laden` wie im Betrieb.
fn vier_ersetzungen(ablage: &Ablage) -> Vec<Option<Ersetzung>> {
    let belegung: Geladen<BelegungStellvertreter> = ablage.laden(Datei::Belegung);
    let lesezeichen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    let sitzung: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    let eingestellt = einstellungen::laden(ablage);
    vec![
        belegung.ersetzung,
        lesezeichen.ersetzung,
        sitzung.ersetzung,
        eingestellt.ersetzung,
    ]
}

/// Alle vier Dateien werden gesichert, und das Original bleibt liegen (C3.1,
/// C3.3, C3.4).
///
/// Die Regel hat in `Ablage::laden` keinen Zweig je Datei, und diese Probe
/// laeuft deshalb ueber `Datei::ALLE`: eine fuenfte Ablagedatei koennte sie
/// nicht vergessen.
#[test]
fn jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt() {
    let (_ordner, ablage) = ablage("beiseite-alle-vier");
    for welche in Datei::ALLE {
        fs::write(ablage.pfad(welche), KAPUTT).expect("schreiben gescheitert");
    }

    for (welche, ersetzung) in Datei::ALLE.into_iter().zip(vier_ersetzungen(&ablage)) {
        let ersetzung = ersetzung
            .unwrap_or_else(|| panic!("{} wurde ohne Meldung ersetzt", welche.dateiname()));
        let erwartet = beiseitepfad(&ablage, welche);
        assert_eq!(
            ersetzung.beiseite,
            Beiseite::Gesichert(erwartet.clone()),
            "{} wurde nicht zur Seite gelegt",
            welche.dateiname()
        );
        assert_eq!(
            fs::read_to_string(&erwartet).expect("die Sicherung fehlt"),
            KAPUTT,
            "die Sicherung von {} traegt einen anderen Inhalt",
            welche.dateiname()
        );

        // Kopiert und nicht verschoben: `keymap.toml` und `settings.toml` sind
        // von Hand aenderbar, und ein Tippfehler darf dem Nutzer die Datei nicht
        // unter der Hand wegnehmen.
        assert_eq!(
            fs::read_to_string(ablage.pfad(welche)).expect("das Original fehlt"),
            KAPUTT,
            "{} wurde verschoben statt kopiert",
            welche.dateiname()
        );
    }
}

/// Der abgeleitete Name haengt die Endung an und ist selbst keine Ablagedatei
/// (C3.1).
///
/// Die zweite Haelfte ist die Zusage aus dem Datensatz vom 260812-1105: KRK
/// darf eine Sicherung nicht selbst wieder als Ablagedatei lesen.
#[test]
fn der_name_der_sicherung_haengt_die_endung_an_und_ist_keine_ablagedatei() {
    let (_ordner, ablage) = ablage("beiseite-name");
    let namen_der_ablage: Vec<&str> = Datei::ALLE.iter().map(|w| w.dateiname()).collect();

    for welche in Datei::ALLE {
        let pfad = beiseitepfad(&ablage, welche);
        assert_eq!(
            pfad,
            ablage.ort().wurzel().join(format!(
                "{}.{}",
                welche.dateiname(),
                atomar::BESCHAEDIGTENDUNG
            )),
            "der abgeleitete Name stimmt nicht"
        );

        let name = pfad
            .file_name()
            .and_then(|name| name.to_str())
            .expect("der abgeleitete Pfad traegt keinen Dateinamen");
        assert!(
            !namen_der_ablage.contains(&name),
            "{name} wuerde von KRK als Ablagedatei gelesen"
        );
    }
}

/// Eine zweite Beschaedigung laesst die erste Sicherung unangetastet (C3.2).
///
/// Was zaehlt, ist die **erste** zur Seite gelegte Fassung: sie traegt die
/// Arbeit des Nutzers, waehrend die zweite aus dem Auslieferungszustand
/// entstanden ist, den KRK selbst geschrieben hat.
#[test]
fn eine_zweite_beschaedigung_laesst_die_erste_sicherung_unangetastet() {
    let (_ordner, ablage) = ablage("beiseite-zweimal");
    let pfad = ablage.pfad(Datei::Lesezeichen);
    let sicherung = beiseitepfad(&ablage, Datei::Lesezeichen);

    fs::write(&pfad, KAPUTT).expect("schreiben gescheitert");
    let erst: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    assert_eq!(
        erst.ersetzung.expect("keine Meldung").beiseite,
        Beiseite::Gesichert(sicherung.clone())
    );

    fs::write(&pfad, KAPUTT_ZWEITER).expect("schreiben gescheitert");
    let wieder: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    assert_eq!(
        wieder.ersetzung.expect("keine Meldung").beiseite,
        Beiseite::SchonVorhanden(sicherung.clone()),
        "die zweite Beschaedigung hat die Sicherung angefasst"
    );
    assert_eq!(
        fs::read_to_string(&sicherung).expect("die Sicherung fehlt"),
        KAPUTT,
        "die erste Sicherung wurde ueberschrieben"
    );

    // Keine durchnummerierte Reihe: neben der einen Sicherung entsteht keine
    // zweite.
    let sicherungen: Vec<String> = fs::read_dir(ablage.ort().wurzel())
        .expect("der Ablageordner laesst sich nicht lesen")
        .map(|eintrag| {
            eintrag
                .expect("Eintrag")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.contains(atomar::BESCHAEDIGTENDUNG))
        .collect();
    assert_eq!(
        sicherungen.len(),
        1,
        "es steht mehr als eine Sicherung da: {sicherungen:?}"
    );
}

/// Eine fehlende und eine nicht lesbare Datei werden nicht zur Seite gelegt
/// (C3.5).
///
/// Eine fehlende Datei ist der erste Start, und von einer, die sich nicht lesen
/// liess, gibt es keinen Inhalt zu sichern.
#[test]
fn eine_fehlende_und_eine_nicht_lesbare_datei_werden_nicht_zur_seite_gelegt() {
    let (_ordner, ablage) = ablage("beiseite-nichts-zu-sichern");

    let fehlt: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    assert!(fehlt.ersetzung.is_none(), "eine fehlende Datei meldet sich");
    assert!(
        !beiseitepfad(&ablage, Datei::Lesezeichen)
            .try_exists()
            .expect("try_exists gescheitert"),
        "eine fehlende Datei wurde zur Seite gelegt"
    );

    // Ein Ordner an der Stelle der Datei: das Lesen scheitert mit einem anderen
    // Fehler als "nicht vorhanden", und zwar unter jedem Benutzer.
    fs::create_dir(ablage.pfad(Datei::Sitzung)).expect("Ordner laesst sich nicht anlegen");
    let nicht_lesbar: Geladen<Sitzung> = ablage.laden(Datei::Sitzung);
    let ersetzung = nicht_lesbar.ersetzung.expect("keine Meldung");
    assert!(
        matches!(ersetzung.grund, Grund::NichtLesbar(_)),
        "{ersetzung:?}"
    );
    assert_eq!(ersetzung.beiseite, Beiseite::Nicht);
    assert!(
        !beiseitepfad(&ablage, Datei::Sitzung)
            .try_exists()
            .expect("try_exists gescheitert"),
        "eine nicht lesbare Datei wurde zur Seite gelegt"
    );
}

/// Scheitert das Zur-Seite-Legen, sagt die Meldung es und verspricht keine
/// Datei (C3.6, C3.8).
///
/// Der Weg wird an der Nachbardatei des atomaren Schreibens versperrt, und das
/// ist zugleich der Nachweis, dass er ueber `atomar::schreiben` fuehrt: laege
/// ein zweiter Schreibweg daneben, kaeme die Sicherung trotzdem zustande.
#[test]
fn ein_gescheitertes_zur_seite_legen_wird_gemeldet_und_verspricht_keine_datei() {
    let (_ordner, ablage) = ablage("beiseite-gescheitert");
    fs::write(ablage.pfad(Datei::Lesezeichen), KAPUTT).expect("schreiben gescheitert");

    let sicherung = beiseitepfad(&ablage, Datei::Lesezeichen);
    let nachbar = atomar::nachbarpfad(&sicherung).expect("kein Nachbarpfad");
    fs::create_dir(&nachbar).expect("der Sperrordner laesst sich nicht anlegen");

    let geladen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    let ersetzung = geladen.ersetzung.expect("keine Meldung");
    let Beiseite::Gescheitert(ref grund) = ersetzung.beiseite else {
        panic!("das Zur-Seite-Legen ist nicht gescheitert: {ersetzung:?}");
    };
    assert!(!grund.is_empty(), "die Meldung nennt keinen Grund");

    assert!(
        !sicherung.try_exists().expect("try_exists gescheitert"),
        "es steht doch eine Sicherung da"
    );
    let text = ersetzung.to_string();
    assert!(
        !text.contains(&sicherung.display().to_string()),
        "die Meldung verspricht eine Datei, die es nicht gibt: {text}"
    );
    assert!(!text.contains('\n'), "die Meldung ist mehrzeilig: {text}");
}

/// Jede der vier Lagen traegt ihren eigenen Satz, und keiner ist mehrzeilig
/// (C3.7, C3.8).
///
/// Die Saetze werden an gebauten Werten geprueft und nicht an einem Ablauf:
/// die Fallunterscheidung ist ueber `Beiseite` vollstaendig, und eine Probe
/// ueber die vier Werte prueft sie ebenso vollstaendig.
#[test]
fn die_meldung_unterscheidet_die_vier_lagen_und_bleibt_einzeilig() {
    let datei = PathBuf::from("/Users/pruefung/Library/Application Support/KRK/bookmarks.toml");
    let sicherung = atomar::beiseitepfad(&datei).expect("kein Beiseitepfad");
    let bau = |beiseite: Beiseite| Ersetzung {
        datei: datei.clone(),
        grund: Grund::Beschaedigt("Zeile 3, Spalte 7".to_owned()),
        beiseite,
    };

    // Ohne Sicherung bleibt der Satz Wort fuer Wort der von vor der Runde 6.
    assert_eq!(
        bau(Beiseite::Nicht).to_string(),
        format!(
            "{} ist beschaedigt und wird durch den Auslieferungszustand ersetzt: \
             Zeile 3, Spalte 7",
            datei.display()
        )
    );

    for (lage, beiseite) in [
        ("gesichert", Beiseite::Gesichert(sicherung.clone())),
        (
            "schon vorhanden",
            Beiseite::SchonVorhanden(sicherung.clone()),
        ),
    ] {
        let text = bau(beiseite).to_string();
        assert!(
            text.starts_with("Die bisherige Fassung liegt"),
            "der Satz sagt nicht zuerst, was der Nutzer tun kann ({lage}): {text}"
        );
        assert!(
            text.contains(&sicherung.display().to_string()),
            "der Satz nennt die Sicherung nicht ({lage}): {text}"
        );
        assert!(
            text.contains(&datei.display().to_string()),
            "der Satz nennt die beschaedigte Datei nicht ({lage}): {text}"
        );
    }

    // Die beiden Saetze sind verschieden: der zweite sagt, dass die Sicherung
    // von einem frueheren Start stammt und dort bleibt.
    assert_ne!(
        bau(Beiseite::Gesichert(sicherung.clone())).to_string(),
        bau(Beiseite::SchonVorhanden(sicherung.clone())).to_string()
    );

    let gescheitert = bau(Beiseite::Gescheitert("kein Platz mehr".to_owned())).to_string();
    assert!(
        gescheitert.contains("kein Platz mehr"),
        "der Satz nennt den Grund nicht: {gescheitert}"
    );
    assert!(
        !gescheitert.contains(&sicherung.display().to_string()),
        "der Satz verspricht eine Datei, die es nicht gibt: {gescheitert}"
    );

    for beiseite in [
        Beiseite::Nicht,
        Beiseite::Gesichert(sicherung.clone()),
        Beiseite::SchonVorhanden(sicherung.clone()),
        Beiseite::Gescheitert("kein Platz mehr".to_owned()),
    ] {
        let text = bau(beiseite).to_string();
        assert!(!text.contains('\n'), "die Meldung ist mehrzeilig: {text}");
        assert!(
            text.contains("Auslieferungszustand"),
            "die Meldung nennt die Ersetzung nicht: {text}"
        );
    }
}

// ---------------------------------------------------------------------------
// Die von Hand gepflegten Einstellungen (C11, Schritt 18c)
// ---------------------------------------------------------------------------

/// Der erste Start legt die Datei an, und zwar **mit** ihren Kommentaren.
///
/// Die Kommentare sind der Zweck dieser Datei: sie nennen das `mdls`-Kommando,
/// mit dem der Nutzer die Buendelkennung seiner eigenen Anwendung ausliest.
/// Ginge die Anlage ueber `Ablage::sichern`, stuende dort eine einzige Zeile,
/// denn `serde` kennt keine Kommentare.
#[test]
fn eine_fehlende_settings_toml_liefert_die_vorbelegung_und_entsteht_mit_kommentaren() {
    let (_ordner, ablage) = ablage("einstellungen-erststart");
    let pfad = ablage.pfad(Datei::Einstellungen);
    assert!(!pfad.exists(), "settings.toml steht schon vorher");

    let geladen = einstellungen::laden(&ablage);

    assert_eq!(geladen.wert.terminal, "com.apple.Terminal");
    assert!(
        !geladen.ist_ersetzt(),
        "eine fehlende Datei ist der erste Start und keine Meldung wert"
    );

    let geschrieben = fs::read_to_string(&pfad).expect("settings.toml ist nicht entstanden");
    assert!(
        geschrieben.contains("mdls -name kMDItemCFBundleIdentifier"),
        "die angelegte Datei traegt die Kommentare nicht: {geschrieben}"
    );
    let kommentarzeilen = geschrieben
        .lines()
        .filter(|zeile| zeile.starts_with('#'))
        .count();
    assert!(
        kommentarzeilen > 20,
        "die angelegte Datei traegt nur {kommentarzeilen} Kommentarzeilen"
    );

    // Der zweite Start findet sie vor und schreibt sie nicht noch einmal.
    fs::write(&pfad, "terminal = \"com.mitchellh.ghostty\"\n").expect("schreiben gescheitert");
    let wieder = einstellungen::laden(&ablage);
    assert_eq!(wieder.wert.terminal, "com.mitchellh.ghostty");
    assert!(!wieder.ist_ersetzt());
    assert_eq!(
        fs::read_to_string(&pfad).expect("lesen gescheitert"),
        "terminal = \"com.mitchellh.ghostty\"\n",
        "die vorhandene Datei wurde ueberschrieben"
    );
}

/// Eine kaputte Datei kostet den Inhalt und nicht die Datei.
#[test]
fn eine_kaputte_settings_toml_liefert_die_vorbelegung_und_bleibt_liegen() {
    let (_ordner, ablage) = ablage("einstellungen-kaputt");
    let pfad = ablage.pfad(Datei::Einstellungen);
    fs::write(&pfad, KAPUTT).expect("schreiben gescheitert");

    let geladen = einstellungen::laden(&ablage);

    assert_eq!(geladen.wert, Einstellungen::auslieferung());
    let ersetzung = geladen
        .ersetzung
        .clone()
        .expect("die kaputte Datei wurde ohne Meldung ersetzt");
    assert!(
        matches!(ersetzung.grund, Grund::Beschaedigt(_)),
        "{ersetzung:?}"
    );
    pruefe_meldung(&ablage, Datei::Einstellungen, geladen.ersetzung, true);
    assert_eq!(
        fs::read_to_string(&pfad).expect("lesen gescheitert"),
        KAPUTT,
        "die kaputte Datei wurde ueberschrieben"
    );
}

/// Ein Feld, das die Nutzerdatei nicht nennt, kommt aus der
/// Auslieferungsfassung. Das ist die eine Abweichung von `keymap.toml`.
#[test]
fn eine_settings_toml_ohne_terminal_liefert_die_vorbelegung() {
    let (_ordner, ablage) = ablage("einstellungen-leer");
    let pfad = ablage.pfad(Datei::Einstellungen);
    let inhalt = "# der Nutzer hat den Eintrag herausgenommen\n";
    fs::write(&pfad, inhalt).expect("schreiben gescheitert");

    let geladen = einstellungen::laden(&ablage);

    assert_eq!(geladen.wert.terminal, "com.apple.Terminal");
    assert!(
        !geladen.ist_ersetzt(),
        "ein fehlendes Feld ist keine beschaedigte Datei"
    );
    assert_eq!(
        fs::read_to_string(&pfad).expect("lesen gescheitert"),
        inhalt,
        "die Datei wurde ueberschrieben"
    );
}

/// Ein Feld, das KRK nicht kennt, ist in einer von Hand gepflegten Datei fast
/// immer ein Tippfehler und bekommt deshalb eine Meldung.
#[test]
fn ein_unbekanntes_feld_in_settings_toml_gilt_als_beschaedigt() {
    let (_ordner, ablage) = ablage("einstellungen-tippfehler");
    fs::write(
        ablage.pfad(Datei::Einstellungen),
        "termnal = \"com.apple.Terminal\"\n",
    )
    .expect("schreiben gescheitert");

    let geladen = einstellungen::laden(&ablage);

    assert_eq!(geladen.wert, Einstellungen::auslieferung());
    pruefe_meldung(&ablage, Datei::Einstellungen, geladen.ersetzung, true);
}

/// Laesst sich die Datei nicht anlegen, sagt KRK das, statt still weiterzulaufen.
///
/// Der Ablageordner verschwindet zwischen Oeffnen und Laden. Das ist der eine
/// Weg, der ohne entzogene Rechte auskommt und deshalb unabhaengig davon
/// laeuft, unter welchem Benutzer die Pruefung startet.
#[test]
fn eine_nicht_anlegbare_settings_toml_meldet_sich() {
    let ordner = Pruefordner::neu("einstellungen-nicht-anlegbar");
    let wurzel = ordner.pfad().join("KRK");
    let ablage = Ablage::oeffnen(Ablageort::an(&wurzel)).expect("Ablage laesst sich nicht oeffnen");
    fs::remove_dir_all(&wurzel).expect("der Ablageordner laesst sich nicht entfernen");

    let geladen = einstellungen::laden(&ablage);

    assert_eq!(geladen.wert, Einstellungen::auslieferung());
    let ersetzung = geladen
        .ersetzung
        .expect("die gescheiterte Anlage wurde nicht gemeldet");
    assert!(
        matches!(ersetzung.grund, Grund::NichtAnlegbar(_)),
        "{ersetzung:?}"
    );
    let text = ersetzung.to_string();
    assert!(
        text.contains("settings.toml"),
        "die Meldung benennt die Datei nicht: {text}"
    );
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
        liste.anlegen("Projekte", auf("/Users/pruefung/Projekte")),
        0
    );
    assert_eq!(liste.anlegen("Sicherung", auf("/Volumes/Sicherung")), 1);
    assert_eq!(liste.anlegen("Wurzel", auf("/")), 2);
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

// ---------------------------------------------------------------------------
// Die zweite Sorte: Textmarken in derselben Liste und Datei (C6, Schritt 11)
// ---------------------------------------------------------------------------

/// Ein Ordnerziel aus einem Pfad, damit die Proben unten kurz bleiben.
fn auf(pfad: &str) -> Ziel {
    Ziel::Ordner {
        ordner: PathBuf::from(pfad),
    }
}

/// Eine Beispielliste mit beiden Sorten, absichtlich abwechselnd.
///
/// Die Reihenfolge ist die der Leiste und wird nicht nach Sorte gruppiert; der
/// Modulkopf von `lesezeichen.rs` begruendet es damit, dass zwei Ordnungen zwei
/// Wahrheiten waeren.
fn gemischte_liste() -> Lesezeichenliste {
    Lesezeichenliste::aus(vec![
        Lesezeichen::neu("Projekte", "/Users/pruefung/Projekte"),
        Lesezeichen::textstelle(
            "Die Lesestelle",
            "/Users/pruefung/Projekte/krk/crates/krk-core/src/verzeichnis/leser.rs",
            118,
            "        let mut puffer = vec![0u8; PUFFERGROESSE];",
        ),
        Lesezeichen::neu("Sicherung", "/Volumes/Sicherung"),
        Lesezeichen::textstelle("Der Modulkopf", "/Users/pruefung/notiz.md", 1, "# Notiz"),
    ])
}

/// Eine `bookmarks.toml` aus der Zeit vor den Textmarken wird unveraendert
/// eingelesen (C6, dreizehntes Abnahmekriterium — und C3.9 der Runde 6).
///
/// Die Datei traegt allein `name` und `ordner` und kein weiteres Feld — genau
/// die Gestalt, die das laufende Programm bis zum 260807 geschrieben hat. Der
/// Nutzer verliert seine Lesezeichen nicht.
///
/// **Die Runde 6 haengt eine zweite Zusage an dieselbe Datei**, statt eine
/// zweite Probe daneben zu stellen: die alte Form gilt nicht als beschaedigt,
/// und deshalb wird auch nichts zur Seite gelegt. Beide Zusagen reden ueber
/// denselben Lesevorgang derselben Datei; zwei Proben davon waeren zweimal
/// dasselbe Ereignis. Der Nachweis, dass eine wirklich beschaedigte Datei
/// gesichert wird, steht weiter oben bei
/// [`jede_der_vier_dateien_wird_bei_beschaedigung_zur_seite_gelegt`].
#[test]
fn eine_bookmarks_toml_aus_der_zeit_vor_den_textmarken_bleibt_lesbar() {
    let (_ordner, ablage) = ablage("lesezeichen-altbestand");
    let alt = "\
[[eintraege]]
name = \"Projekte\"
ordner = \"/Users/pruefung/Projekte\"

[[eintraege]]
name = \"Sicherung\"
ordner = \"/Volumes/Sicherung\"

[[eintraege]]
name = \"Wurzel\"
ordner = \"/\"
";
    fs::write(ablage.pfad(Datei::Lesezeichen), alt).expect("schreiben gescheitert");

    let geladen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);

    assert!(
        !geladen.ist_ersetzt(),
        "die alte Datei gilt als beschaedigt: {:?}",
        geladen.ersetzung
    );
    assert_eq!(geladen.wert.zahl(), 3);
    assert_eq!(
        geladen.wert,
        Lesezeichenliste::aus(vec![
            Lesezeichen::neu("Projekte", "/Users/pruefung/Projekte"),
            Lesezeichen::neu("Sicherung", "/Volumes/Sicherung"),
            Lesezeichen::neu("Wurzel", "/"),
        ]),
        "drei Ordnermarken, in der Reihenfolge der Datei"
    );

    // Die Zusage der Runde 6: eine Datei, die gelesen werden kann, wird nicht
    // zur Seite gelegt. Ein `bookmarks.toml.beschaedigt` neben einer heilen
    // Datei waere eine Warnung ueber einen Schaden, den es nicht gibt.
    assert!(
        !beiseitepfad(&ablage, Datei::Lesezeichen)
            .try_exists()
            .expect("try_exists gescheitert"),
        "die alte Form wurde zur Seite gelegt"
    );
}

/// Eine Rundreise ueber beide Sorten liefert byteweise dieselbe Datei.
///
/// **Das ist die Abnahme des Vorbehalts zu `#[serde(flatten)]`**, den der
/// Modulkopf von [`Ziel`] benennt: ob `toml` die Verbindung aus `flatten` und
/// `untagged` traegt, war am Papier nicht zu entscheiden. Geprueft wird
/// deshalb der ganze Weg, den das laufende Programm geht — schreiben, wieder
/// einlesen, ein zweites Mal schreiben — und beide Male muss derselbe Text
/// herauskommen.
#[test]
fn eine_rundreise_ueber_beide_sorten_liefert_dieselbe_datei() {
    let (_ordner, ablage) = ablage("lesezeichen-rundreise");
    let liste = gemischte_liste();

    ablage
        .sichern(Datei::Lesezeichen, &liste)
        .expect("bookmarks.toml laesst sich nicht schreiben");
    let erster = fs::read(ablage.pfad(Datei::Lesezeichen)).expect("lesen gescheitert");

    let geladen: Geladen<Lesezeichenliste> = ablage.laden(Datei::Lesezeichen);
    assert!(
        !geladen.ist_ersetzt(),
        "die geschriebene Datei liest sich nicht zurueck: {:?}",
        geladen.ersetzung
    );
    assert_eq!(geladen.wert, liste, "der Wert ueberlebt die Rundreise");

    ablage
        .sichern(Datei::Lesezeichen, &geladen.wert)
        .expect("bookmarks.toml laesst sich nicht ein zweites Mal schreiben");
    let zweiter = fs::read(ablage.pfad(Datei::Lesezeichen)).expect("lesen gescheitert");

    assert_eq!(erster, zweiter, "die Datei ist byteweise dieselbe");
}

/// Die geschriebene Datei bleibt von Hand lesbar (C6, zwoelftes
/// Abnahmekriterium, und C7/C11 der Runde 1).
///
/// Festgemacht an dem, was **nicht** darin steht: keine geschachtelte Tabelle
/// unter einem Eintrag und keine Sortenkennung. Die unmarkierte Auswahl [`Ziel`]
/// legt die Felder der gewaehlten Sorte unmittelbar neben `name`.
#[test]
fn die_geschriebene_datei_traegt_weder_geschachtelte_tabelle_noch_sortenkennung() {
    let (_ordner, ablage) = ablage("lesezeichen-lesbar");
    ablage
        .sichern(Datei::Lesezeichen, &gemischte_liste())
        .expect("bookmarks.toml laesst sich nicht schreiben");
    let text = fs::read_to_string(ablage.pfad(Datei::Lesezeichen)).expect("lesen gescheitert");

    for kopf in text.lines().filter(|zeile| zeile.starts_with('[')) {
        assert_eq!(
            kopf.trim(),
            "[[eintraege]]",
            "die Datei traegt eine geschachtelte Tabelle:\n{text}"
        );
    }
    for kennung in ["typ", "sorte", "art", "ziel"] {
        assert!(
            !text.contains(&format!("{kennung} =")),
            "die Datei traegt eine Sortenkennung `{kennung}`:\n{text}"
        );
    }
    assert!(text.contains("ordner = "), "die Ordnermarke fehlt:\n{text}");
    assert!(
        text.contains("zeileninhalt = "),
        "die Textmarke fehlt:\n{text}"
    );
}

/// Gueltig heisst fuer eine Textmarke allein, dass die Datei da ist (C6).
///
/// Eine Frage an das Dateisystem und **kein Lesevorgang**: der gemerkte
/// Zeileninhalt kommt darin nicht vor. Die Probe haelt beides fest — die
/// vorhandene Datei ist gueltig, obwohl ihr Inhalt mit dem gemerkten nichts zu
/// tun hat, und ein Ordner an der Stelle der Datei ist es nicht.
#[test]
fn eine_textmarke_ist_gueltig_solange_ihre_datei_da_ist() {
    let ordner = Pruefordner::neu("textmarke-gueltigkeit");
    let datei = ordner.pfad().join("notiz.md");
    fs::write(&datei, b"ein ganz anderer Inhalt\n").expect("schreiben gescheitert");

    assert!(
        Lesezeichen::textstelle("Da", &datei, 7, "eine Zeile, die dort nicht steht").gueltig(),
        "ein abweichender Zeileninhalt macht die Marke nicht ungueltig"
    );
    assert!(!Lesezeichen::textstelle("Weg", ordner.pfad().join("fort.md"), 1, "x").gueltig());
    assert!(
        !Lesezeichen::textstelle("Ordner", ordner.pfad(), 1, "x").gueltig(),
        "ein Ordner ist kein Ziel fuer eine Textmarke"
    );
}

/// Die Gueltigkeitspruefung **oeffnet und liest die Datei nicht** (C6, Schritt
/// 12).
///
/// Die Zusage ist der tragende Grund der Antwort vom 260808-0017
/// (`decisions/260807-2147_*_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`):
/// die Leiste stellt diese Frage bei jedem Neuaufbau ihrer Liste fuer jede
/// Marke, und ein Lesevorgang je Marke waere etwas anderes als eine Frage an
/// das Dateisystem.
///
/// Die Probe nimmt der Datei jedes Leserecht. Ihre Groesse und ihr Typ bleiben
/// erfragbar, ihr Inhalt nicht — wer sie oeffnen wollte, bekaeme
/// `PermissionDenied`. Bleibt die Marke dabei gueltig, hat die Pruefung nicht
/// gelesen.
///
/// **Unter root belegt die Probe nichts**, weil Zugriffsrechte dann nicht
/// greifen; dieselbe Einschraenkung steht bei
/// [`eine_nicht_lesbare_datei_fuehrt_ebenso_zum_auslieferungszustand`]. Sie
/// bricht deshalb erkennbar ab, statt still durchzugehen und eine Zusage
/// vorzutaeuschen.
#[test]
fn die_gueltigkeitspruefung_kommt_ohne_lesen_der_datei_aus() {
    use std::os::unix::fs::PermissionsExt;

    let ordner = Pruefordner::neu("textmarke-ohne-lesen");
    let datei = ordner.pfad().join("verschlossen.md");
    fs::write(&datei, b"eine Zeile\n").expect("schreiben gescheitert");
    fs::set_permissions(&datei, fs::Permissions::from_mode(0o000))
        .expect("die Rechte lassen sich nicht entziehen");

    assert!(
        fs::read(&datei).is_err(),
        "die Pruefdatei ist trotz entzogener Rechte lesbar — laeuft der Lauf unter root?"
    );
    assert!(
        Lesezeichen::textstelle("Verschlossen", &datei, 1, "eine Zeile").gueltig(),
        "gueltig() hat die Datei zu oeffnen versucht, statt nur nach ihr zu fragen"
    );

    // Das Gegenstueck: verschwindet die Datei, ist die Marke ungueltig. Das ist
    // der einzige Grund, aus dem sie es wird.
    fs::set_permissions(&datei, fs::Permissions::from_mode(0o600))
        .expect("die Rechte lassen sich nicht zuruecksetzen");
    fs::remove_file(&datei).expect("die Pruefdatei laesst sich nicht loeschen");
    assert!(!Lesezeichen::textstelle("Verschlossen", &datei, 1, "eine Zeile").gueltig());
}
