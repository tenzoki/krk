//! Abnahme des Verzeichnislesers und des Ordnermodells (Schritt 2 des Plans).
//!
//! Alle Tests laufen ohne Fenster und ohne AppKit. Ihre Pruefordner kommen aus
//! `tests/gemeinsam/`, der einen Fassung fuer alle Abnahmeproben des Kerns; sie
//! tragen Prozesskennung und Laufnummer und raeumen sich in `Drop` selbst ab.

use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::{Duration, SystemTime};

use krk_core::verzeichnis::durchlauf::{Auftrag, Befundmeldung, Durchlauf};
use krk_core::verzeichnis::leser::{Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen};
use krk_core::verzeichnis::modell::{Befund, Ordnermodell};
use krk_core::verzeichnis::sortierung::{Richtung, Schluessel, Sortierung};
use krk_core::verzeichnis::{Eintrag, Typ};

mod gemeinsam;
use gemeinsam::Pruefordner;

/// Ein flacher Ordner mit `anzahl` Dateien, deren Namen fest zugeordnet sind.
fn ordner_mit_dateien(zweck: &str, anzahl: usize) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);
    for nummer in 0..anzahl {
        ordner.fuelldatei(&format!("eintrag-{nummer:06}.txt"), nummer % 17);
    }
    ordner
}

fn namen(modell: &Ordnermodell) -> Vec<&str> {
    modell
        .zeilen()
        .map(|eintrag| eintrag.name.as_str())
        .collect()
}

/// Liest den Ordner vollstaendig ueber den gestueckelten Weg und liefert die
/// Stapel einzeln, damit ein Test ihre Zahl pruefen kann.
fn stapelweise_lesen(pfad: &Path, generation: u64) -> (Vec<Vec<Eintrag>>, Abschluss) {
    let vorgang = Lesevorgang::starten(pfad.to_path_buf(), generation);
    let mut stapel = Vec::new();
    loop {
        match vorgang
            .meldungen()
            .recv()
            .expect("Kanal vorzeitig geschlossen")
        {
            Meldung::Stapel {
                generation: gemeldet,
                eintraege,
            } => {
                assert_eq!(gemeldet, generation, "Stapel traegt die falsche Generation");
                stapel.push(eintraege);
            }
            Meldung::Fertig {
                generation: gemeldet,
                abschluss,
            } => {
                assert_eq!(
                    gemeldet, generation,
                    "Abschluss traegt die falsche Generation"
                );
                return (stapel, abschluss);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Der Leser
// ---------------------------------------------------------------------------

#[test]
fn fuenftausend_eintraege_kommen_in_mindestens_fuenf_stapeln() {
    let ordner = ordner_mit_dateien("fuenftausend", 5_000);

    let (stapel, abschluss) = stapelweise_lesen(ordner.pfad(), 7);

    let gelesen: usize = stapel.iter().map(Vec::len).sum();
    assert_eq!(
        gelesen, 5_000,
        "der Leser hat nicht alle Eintraege geliefert"
    );
    assert!(
        stapel.len() >= 5,
        "erwartet mindestens 5 Stapel, geliefert {}",
        stapel.len()
    );
    assert!(
        abschluss.ist_vollstaendig(),
        "erwartet einen vollstaendigen Lauf, geliefert {abschluss:?}"
    );
    for (nummer, einzelner) in stapel.iter().enumerate() {
        if nummer + 1 < stapel.len() {
            assert_eq!(
                einzelner.len(),
                STAPELGROESSE,
                "Stapel {nummer} ist nicht voll"
            );
        }
    }
}

#[test]
fn abgebrochener_leser_liefert_teilbestand_und_meldet_den_abbruch() {
    let ordner = ordner_mit_dateien("abbruch", 5_000);

    let vorgang = Lesevorgang::starten(ordner.pfad().to_path_buf(), 1);
    let mut gelesen = 0usize;
    let mut abschluss = None;

    // Den ersten Stapel abwarten, dann mitten im Lauf abbrechen.
    match vorgang.meldungen().recv().expect("kein erster Stapel") {
        Meldung::Stapel { eintraege, .. } => gelesen += eintraege.len(),
        Meldung::Fertig { .. } => panic!("der Lauf war fertig, bevor ein Stapel kam"),
    }
    vorgang.abbrechen();

    while let Ok(meldung) = vorgang.meldungen().recv() {
        match meldung {
            Meldung::Stapel { eintraege, .. } => gelesen += eintraege.len(),
            Meldung::Fertig {
                abschluss: gemeldet,
                ..
            } => {
                abschluss = Some(gemeldet);
                break;
            }
        }
    }

    let abschluss = abschluss.expect("kein Abschluss gemeldet");
    assert!(
        abschluss.ist_abgebrochen(),
        "erwartet einen gemeldeten Abbruch, geliefert {abschluss:?}"
    );
    assert!(
        gelesen < 5_000,
        "der Abbruch kam zu spaet: {gelesen} von 5.000 Eintraegen gelesen"
    );
    assert!(gelesen > 0, "der Teilbestand ist leer");
}

#[test]
fn punkt_und_punktpunkt_kommen_nicht_vor() {
    let ordner = Pruefordner::neu("punkte");
    ordner.fuelldatei("a.txt", 1);
    ordner.ordner("unterordner");

    let eintraege = lesen(ordner.pfad()).expect("Lesen gescheitert");

    let gelesene: Vec<&str> = eintraege.iter().map(|e| e.name.as_str()).collect();
    assert!(!gelesene.contains(&"."), "der Leser meldet .");
    assert!(!gelesene.contains(&".."), "der Leser meldet ..");
    assert_eq!(gelesene.len(), 2);
}

#[test]
fn gelesene_werte_stimmen_mit_dem_dateisystem_ueberein() {
    let ordner = Pruefordner::neu("querprobe");
    ordner.fuelldatei("klein.txt", 7);
    ordner.fuelldatei("gross.bin", 40_000);
    ordner.fuelldatei("leer.dat", 0);
    ordner.ordner("ein-ordner");
    ordner.verknuepfung("verweis", "klein.txt");
    ordner.fuelldatei(".versteckt", 3);
    ordner.fuelldatei("still.txt", 5);
    ordner.verstecken("still.txt");

    let eintraege = lesen(ordner.pfad()).expect("Lesen gescheitert");
    assert_eq!(eintraege.len(), 7);

    let nach_namen: HashMap<&str, &Eintrag> = eintraege
        .iter()
        .map(|eintrag| (eintrag.name.as_str(), eintrag))
        .collect();

    for (name, eintrag) in &nach_namen {
        let angaben = fs::symlink_metadata(ordner.pfad().join(name))
            .unwrap_or_else(|_| panic!("{name} gibt es im Dateisystem nicht"));

        let erwarteter_typ = if angaben.is_symlink() {
            Typ::Verknuepfung
        } else if angaben.is_dir() {
            Typ::Ordner
        } else {
            Typ::Datei
        };
        assert_eq!(eintrag.typ, erwarteter_typ, "falscher Typ bei {name}");

        if erwarteter_typ != Typ::Ordner {
            assert_eq!(eintrag.groesse, angaben.len(), "falsche Groesse bei {name}");
        }

        let erwartete_zeit = angaben.modified().expect("kein Aenderungsdatum");
        let abstand = erwartete_zeit
            .duration_since(eintrag.geaendert)
            .or_else(|_| eintrag.geaendert.duration_since(erwartete_zeit))
            .expect("Zeitvergleich gescheitert");
        assert!(
            abstand < Duration::from_millis(1),
            "Aenderungsdatum von {name} weicht um {abstand:?} ab"
        );
        assert!(
            eintrag.geaendert > SystemTime::UNIX_EPOCH,
            "Aenderungsdatum von {name} steht auf dem Nullpunkt"
        );
    }

    assert!(
        nach_namen[".versteckt"].versteckt,
        "der fuehrende Punkt ist nicht als versteckt erkannt"
    );
    assert!(
        nach_namen["still.txt"].versteckt,
        "das Kennzeichen UF_HIDDEN des Dateisystems ist nicht erkannt"
    );
    assert!(!nach_namen["klein.txt"].versteckt);
    assert!(nach_namen["ein-ordner"].ist_ordner());
    assert!(nach_namen["verweis"].ist_verknuepfung());
}

#[test]
fn ein_pfad_der_kein_verzeichnis_ist_scheitert() {
    let ordner = Pruefordner::neu("keinordner");
    ordner.fuelldatei("datei.txt", 1);

    let fehler = lesen(&ordner.pfad().join("datei.txt")).expect_err("das haette scheitern muessen");
    assert_eq!(fehler.kind(), std::io::ErrorKind::NotADirectory);
}

// ---------------------------------------------------------------------------
// Das Ordnermodell: Sortierung, Gruppierung, Filter
// ---------------------------------------------------------------------------

/// Der Ordner fuer die Sortierproben.
///
/// Angelegt in dieser Reihenfolge, mit Pausen dazwischen, damit die
/// Aenderungsdaten streng aufsteigen:
///
/// | Reihenfolge | Name        | Typ    | Groesse |
/// |---|---|---|---|
/// | 1 | `dir-b`     | Ordner |     0 |
/// | 2 | `Alpha.txt` | Datei  |   300 |
/// | 3 | `dir-a`     | Ordner |     0 |
/// | 4 | `zeta.txt`  | Datei  |   100 |
/// | 5 | `Beta.txt`  | Datei  |   200 |
fn sortierordner() -> Pruefordner {
    let ordner = Pruefordner::neu("sortierung");
    let pause = Duration::from_millis(5);
    ordner.ordner("dir-b");
    std::thread::sleep(pause);
    ordner.fuelldatei("Alpha.txt", 300);
    std::thread::sleep(pause);
    ordner.ordner("dir-a");
    std::thread::sleep(pause);
    ordner.fuelldatei("zeta.txt", 100);
    std::thread::sleep(pause);
    ordner.fuelldatei("Beta.txt", 200);
    ordner
}

fn geladenes_modell(pfad: &Path) -> Ordnermodell {
    let mut modell = Ordnermodell::neu(1);
    modell.anhaengen(lesen(pfad).expect("Lesen gescheitert"));
    modell.abschliessen();
    modell
}

#[test]
fn alle_acht_sortierungen_liefern_die_erwartete_reihenfolge() {
    let ordner = sortierordner();
    let mut modell = geladenes_modell(ordner.pfad());

    let erwartet: [(Schluessel, Richtung, [&str; 5]); 8] = [
        (
            Schluessel::Name,
            Richtung::Aufsteigend,
            ["dir-a", "dir-b", "Alpha.txt", "Beta.txt", "zeta.txt"],
        ),
        (
            Schluessel::Name,
            Richtung::Absteigend,
            ["dir-b", "dir-a", "zeta.txt", "Beta.txt", "Alpha.txt"],
        ),
        (
            Schluessel::Groesse,
            Richtung::Aufsteigend,
            ["dir-a", "dir-b", "zeta.txt", "Beta.txt", "Alpha.txt"],
        ),
        (
            Schluessel::Groesse,
            Richtung::Absteigend,
            ["dir-b", "dir-a", "Alpha.txt", "Beta.txt", "zeta.txt"],
        ),
        (
            Schluessel::Geaendert,
            Richtung::Aufsteigend,
            ["dir-b", "dir-a", "Alpha.txt", "zeta.txt", "Beta.txt"],
        ),
        (
            Schluessel::Geaendert,
            Richtung::Absteigend,
            ["dir-a", "dir-b", "Beta.txt", "zeta.txt", "Alpha.txt"],
        ),
        (
            Schluessel::Typ,
            Richtung::Aufsteigend,
            ["dir-a", "dir-b", "Alpha.txt", "Beta.txt", "zeta.txt"],
        ),
        (
            Schluessel::Typ,
            Richtung::Absteigend,
            ["dir-b", "dir-a", "zeta.txt", "Beta.txt", "Alpha.txt"],
        ),
    ];

    assert_eq!(erwartet.len(), Sortierung::alle().count());

    for (schluessel, richtung, reihenfolge) in erwartet {
        modell.sortierung_setzen(Sortierung::neu(schluessel, richtung));
        assert_eq!(
            namen(&modell),
            reihenfolge.to_vec(),
            "falsche Reihenfolge bei {schluessel:?} {richtung:?}"
        );
    }
}

#[test]
fn ordner_stehen_vor_dateien_in_jeder_sortierung() {
    let ordner = sortierordner();
    let mut modell = geladenes_modell(ordner.pfad());

    for sortierung in Sortierung::alle() {
        modell.sortierung_setzen(sortierung);
        let typen: Vec<Typ> = modell.zeilen().map(|eintrag| eintrag.typ).collect();
        let erster_dateiplatz = typen
            .iter()
            .position(|typ| *typ != Typ::Ordner)
            .unwrap_or(typen.len());
        assert!(
            typen[erster_dateiplatz..]
                .iter()
                .all(|typ| *typ != Typ::Ordner),
            "bei {sortierung:?} steht ein Ordner hinter einer Datei: {typen:?}"
        );
        assert_eq!(erster_dateiplatz, 2, "erwartet zwei Ordner am Anfang");
    }
}

/// Ein Ordner mit Umlauten und mit vier verschiedenen Endungen.
///
/// Der Ordner aus [`sortierordner`] taugt fuer die beiden Fragen nicht: alle
/// seine Dateien heissen `.txt`, und keiner seiner Namen traegt einen Umlaut.
/// Eine Sortierung nach der Endung saehe dort genauso aus wie eine nach dem
/// Namen, und eine falsche Kollation fiele nicht auf.
fn kollationsordner() -> Pruefordner {
    let ordner = Pruefordner::neu("kollation");
    ordner.ordner("Ähren");
    ordner.fuelldatei("Zebra.md", 10);
    ordner.fuelldatei("Äpfel.zip", 10);
    ordner.fuelldatei("Bäume.md", 10);
    ordner.fuelldatei("LIESMICH", 10);
    ordner.fuelldatei("Übersicht.txt", 10);
    ordner
}

/// Die Kollation traegt bis durch den echten Leser hindurch.
///
/// Die Pruefungen in `verzeichnis::kollation` bauen ihre Namen selbst. Diese
/// hier liest sie ueber `getattrlistbulk` aus dem Dateisystem, und damit auch
/// in der Normalform, in der APFS sie zurueckgibt: `Ä` kommt von dort als `A`
/// mit Kombinationszeichen und nicht als das eine Zeichen, das dieser
/// Quelltext schreibt.
#[test]
fn umlaute_sortieren_beim_grundbuchstaben_und_nicht_hinter_z() {
    let ordner = kollationsordner();
    let modell = geladenes_modell(ordner.pfad());

    assert_eq!(
        namen(&modell),
        vec![
            "Ähren",
            "Äpfel.zip",
            "Bäume.md",
            "LIESMICH",
            "Übersicht.txt",
            "Zebra.md",
        ],
        "Umlaute muessen beim Grundbuchstaben stehen, nicht hinter Z"
    );
}

/// Die Sortierung nach Typ ordnet nach der Endung, und Ordner bleiben vorn.
#[test]
fn nach_typ_ordnet_die_endung_und_ordner_stehen_weiter_vorn() {
    let ordner = kollationsordner();
    let mut modell = geladenes_modell(ordner.pfad());
    modell.sortierung_setzen(Sortierung::neu(Schluessel::Typ, Richtung::Aufsteigend));

    assert_eq!(
        namen(&modell),
        vec![
            // Der Ordner zuerst, wie in jeder Sortierung.
            "Ähren",
            // Dann die Datei ohne Endung, denn der leere Schluessel steht vorn.
            "LIESMICH",
            // Dann md, txt, zip — und bei gleicher Endung entscheidet der Name.
            "Bäume.md",
            "Zebra.md",
            "Übersicht.txt",
            "Äpfel.zip",
        ]
    );
}

#[test]
fn vorbelegung_ist_name_aufsteigend() {
    let ordner = sortierordner();
    let modell = geladenes_modell(ordner.pfad());

    assert_eq!(modell.sortierung(), Sortierung::default());
    assert_eq!(
        namen(&modell),
        vec!["dir-a", "dir-b", "Alpha.txt", "Beta.txt", "zeta.txt"]
    );
}

#[test]
fn der_filter_blendet_namen_mit_fuehrendem_punkt_aus() {
    let ordner = Pruefordner::neu("filter");
    ordner.fuelldatei(".punkt", 1);
    ordner.fuelldatei(".noch-einer", 1);
    ordner.fuelldatei("sichtbar.txt", 1);
    ordner.ordner("Ordner");
    ordner.ordner(".stiller-ordner");

    let mut modell = geladenes_modell(ordner.pfad());

    assert!(modell.verstecke_ausgeblendet(), "Vorbelegung blendet aus");
    assert_eq!(namen(&modell), vec!["Ordner", "sichtbar.txt"]);
    assert_eq!(
        modell.eintraege().len(),
        5,
        "der Filter darf keine Eintraege wegwerfen, nur ausblenden"
    );

    modell.verstecke_umschalten();
    assert_eq!(
        namen(&modell),
        vec![
            ".stiller-ordner",
            "Ordner",
            ".noch-einer",
            ".punkt",
            "sichtbar.txt"
        ]
    );

    modell.verstecke_umschalten();
    assert_eq!(namen(&modell), vec!["Ordner", "sichtbar.txt"]);
}

#[test]
fn der_filter_wirkt_auch_beim_anhaengen() {
    let ordner = Pruefordner::neu("filter-anhaengen");
    ordner.fuelldatei(".punkt", 1);
    ordner.fuelldatei("sichtbar.txt", 1);

    let mut modell = Ordnermodell::neu(1);
    modell.anhaengen(lesen(ordner.pfad()).expect("Lesen gescheitert"));

    // Noch vor dem Abschluss: die Sicht traegt nur den sichtbaren Eintrag.
    assert_eq!(namen(&modell), vec!["sichtbar.txt"]);
}

#[test]
fn die_auswahl_ueberlebt_einen_sortierwechsel() {
    let ordner = sortierordner();
    let mut modell = geladenes_modell(ordner.pfad());

    let zeile_vorher = modell
        .zeilen()
        .position(|eintrag| eintrag.name == "zeta.txt")
        .expect("zeta.txt fehlt");
    let ausgewaehlt = modell.eintragsindex(zeile_vorher).expect("keine Zeile");

    modell.sortierung_setzen(Sortierung::neu(Schluessel::Groesse, Richtung::Aufsteigend));

    let zeile_nachher = modell.zeile_von(ausgewaehlt).expect("Auswahl verloren");
    assert_eq!(modell.zeile(zeile_nachher).unwrap().name, "zeta.txt");
    assert_ne!(
        zeile_vorher, zeile_nachher,
        "die Probe traegt nur, wenn sich die Zeile aendert"
    );
}

#[test]
fn nach_schluessel_sortieren_schaltet_die_richtung_um() {
    let ordner = sortierordner();
    let mut modell = geladenes_modell(ordner.pfad());

    modell.nach_schluessel_sortieren(Schluessel::Groesse);
    assert_eq!(
        modell.sortierung(),
        Sortierung::neu(Schluessel::Groesse, Richtung::Aufsteigend)
    );

    modell.nach_schluessel_sortieren(Schluessel::Groesse);
    assert_eq!(
        modell.sortierung(),
        Sortierung::neu(Schluessel::Groesse, Richtung::Absteigend)
    );

    modell.nach_schluessel_sortieren(Schluessel::Name);
    assert_eq!(
        modell.sortierung(),
        Sortierung::neu(Schluessel::Name, Richtung::Aufsteigend)
    );
}

#[test]
fn das_modell_verwirft_stapel_einer_alten_generation() {
    let ordner = ordner_mit_dateien("generation", 10);
    let modell = Ordnermodell::neu(4);

    assert!(modell.gehoert_dazu(4));
    assert!(!modell.gehoert_dazu(3));

    let (stapel, _) = stapelweise_lesen(ordner.pfad(), 3);
    let veraltet: usize = stapel
        .iter()
        .filter(|_| !modell.gehoert_dazu(3))
        .map(|s| s.len())
        .sum();
    assert_eq!(veraltet, 10, "die Generationspruefung greift nicht");
}

#[test]
fn ein_grosser_ordner_laeuft_stapelweise_ins_modell() {
    let ordner = ordner_mit_dateien("modell-gross", 5_000);

    let mut modell = Ordnermodell::neu(2);
    let vorgang = Lesevorgang::starten(ordner.pfad().to_path_buf(), 2);
    let mut zeilen_nach_erstem_stapel = 0usize;

    loop {
        match vorgang
            .meldungen()
            .recv()
            .expect("Kanal vorzeitig geschlossen")
        {
            Meldung::Stapel {
                generation,
                eintraege,
            } => {
                if modell.gehoert_dazu(generation) {
                    modell.anhaengen(eintraege);
                }
                if zeilen_nach_erstem_stapel == 0 {
                    zeilen_nach_erstem_stapel = modell.zeilenzahl();
                }
            }
            Meldung::Fertig { abschluss, .. } => {
                assert!(matches!(abschluss, Abschluss::Vollstaendig));
                modell.abschliessen();
                break;
            }
        }
    }

    assert_eq!(
        zeilen_nach_erstem_stapel, STAPELGROESSE,
        "der erste Stapel steht nicht sofort in der Sicht"
    );
    assert_eq!(modell.zeilenzahl(), 5_000);
    assert_eq!(modell.zeile(0).unwrap().name, "eintrag-000000.txt");
    assert_eq!(modell.zeile(4_999).unwrap().name, "eintrag-004999.txt");
}

// ---------------------------------------------------------------------------
// Der Filter aus C1 und C2: ein Pruefschritt, zwei Frager
// ---------------------------------------------------------------------------

/// Der Ordner fuer die Filterproben.
///
/// | Name              | Typ    | traegt `aaa` |
/// |---|---|---|
/// | `bbbaaaccc.rs`    | Datei  | ja, in der Mitte |
/// | `AAA-gross.txt`   | Datei  | ja, gross geschrieben |
/// | `ohne.txt`        | Datei  | nein |
/// | `aaa-ordner`      | Ordner | ja |
/// | `stiller-ordner`  | Ordner | nein |
fn filterordner() -> Pruefordner {
    let ordner = Pruefordner::neu("filtertext");
    ordner.fuelldatei("bbbaaaccc.rs", 1);
    ordner.fuelldatei("AAA-gross.txt", 1);
    ordner.fuelldatei("ohne.txt", 1);
    ordner.ordner("aaa-ordner");
    ordner.ordner("stiller-ordner");
    ordner
}

fn gefiltert(pfad: &Path, filtertext: &str) -> Ordnermodell {
    let mut modell = geladenes_modell(pfad);
    modell.filtertext_setzen(filtertext);
    modell
}

fn index_von(modell: &Ordnermodell, name: &str) -> u32 {
    modell
        .index_von_namen(name)
        .unwrap_or_else(|| panic!("den Eintrag {name} gibt es nicht"))
}

/// C1.2: die Teilzeichenfolge zaehlt an jeder Stelle, und die Schreibung
/// entscheidet nicht mit.
#[test]
fn der_filter_nimmt_die_teilzeichenfolge_an_jeder_stelle_und_in_jeder_schreibung() {
    let ordner = filterordner();

    let modell = gefiltert(ordner.pfad(), "aaa");

    assert!(
        namen(&modell).contains(&"bbbaaaccc.rs"),
        "der Treffer in der Mitte des Namens fehlt"
    );
    assert!(
        namen(&modell).contains(&"AAA-gross.txt"),
        "der grossgeschriebene Treffer fehlt"
    );
    assert!(
        !namen(&modell).contains(&"ohne.txt"),
        "eine Datei ohne Treffer steht in der Liste"
    );
    assert_eq!(
        modell.eintraege().len(),
        5,
        "der Filter darf keine Eintraege wegwerfen, nur ausblenden"
    );
}

/// C1.3: der Vergleich faltet keine Umlaute. `apfel` findet `Aepfel` mit
/// Umlaut nicht.
#[test]
fn der_filter_faltet_keine_umlaute() {
    let ordner = Pruefordner::neu("filter-umlaut");
    ordner.fuelldatei("Äpfel.txt", 1);
    ordner.fuelldatei("apfelkuchen.txt", 1);

    let modell = gefiltert(ordner.pfad(), "apfel");

    assert_eq!(namen(&modell), vec!["apfelkuchen.txt"]);
}

/// C1.6: bei flacher Suche bleibt jeder Ordner stehen, damit die Navigation
/// bei stehendem Filter nicht abbricht. Gefiltert werden allein die Dateien.
#[test]
fn bei_flacher_suche_bleibt_jeder_ordner_stehen() {
    let ordner = filterordner();

    let modell = gefiltert(ordner.pfad(), "aaa");

    assert!(!modell.tief(), "die flache Suche ist die Vorbelegung");
    assert_eq!(
        namen(&modell),
        vec![
            "aaa-ordner",
            "stiller-ordner",
            "AAA-gross.txt",
            "bbbaaaccc.rs"
        ],
        "bei flacher Suche steht auch der Ordner ohne Treffer im Namen"
    );
}

/// C2.4: steht kein Filtertext, aendert "Deep" nichts an der Liste.
///
/// Der Befehl kommt trotzdem durch und kippt das Kennzeichen; ueber seine
/// Zulaessigkeit entscheidet der Wirkungsbereich und nicht, ob er etwas
/// findet. Das haelt `jeder_schalter_wirkt_aus_jedem_fokus` in
/// `crates/krk-ui/src/appkit/bereichsleiste.rs`. Hier steht die andere Haelfte:
/// dass die Liste dabei stehen bleibt, und zwar **auch** mit einem Befund
/// `KeinTreffer` an einem Ordner — ohne Filtertext wird er gar nicht erst
/// gefragt.
#[test]
fn ohne_filtertext_aendert_die_tiefe_suche_nichts() {
    let ordner = filterordner();
    let mut modell = geladenes_modell(ordner.pfad());
    // Eigene Zeichenketten: `namen` leiht aus dem Modell, und die Ausleihe
    // ueberstuende die Aenderungen darunter nicht.
    let vorher: Vec<String> = namen(&modell).into_iter().map(str::to_owned).collect();
    assert!(!modell.filter_steht(), "diese Probe faehrt ohne Filtertext");

    let still = index_von(&modell, "stiller-ordner");
    modell.befunde_setzen([(still, Befund::KeinTreffer)]);
    modell.tief_setzen(true);

    assert!(modell.tief(), "das Kennzeichen steht, auch ohne Filtertext");
    assert_eq!(
        namen(&modell),
        vorher,
        "ohne Filtertext entscheidet der Befund ueber keine Zeile"
    );
}

/// C2.5, C2.6: bei tiefer Suche entscheidet ueber einen Ordner sein Name oder
/// der Befund ueber seinen Unterbaum, und `Unentschieden` haelt ihn draussen.
#[test]
fn bei_tiefer_suche_entscheidet_name_oder_befund() {
    let ordner = filterordner();
    let mut modell = gefiltert(ordner.pfad(), "aaa");

    modell.tief_setzen(true);

    assert_eq!(
        namen(&modell),
        vec!["aaa-ordner", "AAA-gross.txt", "bbbaaaccc.rs"],
        "solange nichts entschieden ist, steht kein Ordner ohne passenden Namen"
    );

    let still = index_von(&modell, "stiller-ordner");
    modell.befunde_setzen([(still, Befund::KeinTreffer)]);
    assert!(
        !namen(&modell).contains(&"stiller-ordner"),
        "ein Ordner ohne Treffer darunter faellt weg"
    );

    modell.befunde_setzen([(still, Befund::Treffer)]);
    assert_eq!(
        namen(&modell),
        vec![
            "aaa-ordner",
            "stiller-ordner",
            "AAA-gross.txt",
            "bbbaaaccc.rs"
        ],
        "mit einem Treffer darunter steht der Ordner in der Liste"
    );
}

/// C2.5, C3.14: ein namentlich passender Ordner steht auch dann, wenn unter ihm
/// nichts liegt — sein Befund aendert daran nichts, und deshalb muss fuer ihn
/// nichts gelesen werden.
#[test]
fn ein_namentlich_passender_ordner_steht_auch_ohne_treffer_darunter() {
    let ordner = filterordner();
    let mut modell = gefiltert(ordner.pfad(), "aaa");
    modell.tief_setzen(true);

    let passend = index_von(&modell, "aaa-ordner");
    modell.befunde_setzen([(passend, Befund::KeinTreffer)]);

    assert!(
        namen(&modell).contains(&"aaa-ordner"),
        "der Name entscheidet ihn, und der Befund kann das nicht umstossen"
    );
}

/// C1.6, C2.13: eine symbolische Verknuepfung ist fuer die Sichtbarkeit ein
/// Ordner. Flach bleibt sie stehen, tief entscheidet ihr Name; der Durchlauf
/// steigt nicht in sie hinab und meldet deshalb `KeinTreffer`.
#[test]
fn eine_verknuepfung_zaehlt_fuer_die_sichtbarkeit_als_ordner() {
    let ordner = Pruefordner::neu("filter-verknuepfung");
    ordner.ordner("ziel");
    ordner.verknuepfung("verweis", ordner.unter("ziel"));
    ordner.verknuepfung("aaa-verweis", ordner.unter("ziel"));
    ordner.fuelldatei("ohne.txt", 1);

    let mut modell = gefiltert(ordner.pfad(), "aaa");

    assert!(
        namen(&modell).contains(&"verweis"),
        "flach bleibt die Verknuepfung stehen wie jeder Ordner"
    );

    modell.tief_setzen(true);
    let verweis = index_von(&modell, "verweis");
    modell.befunde_setzen([(verweis, Befund::KeinTreffer)]);

    assert_eq!(
        namen(&modell),
        vec!["aaa-verweis"],
        "tief steht allein die Verknuepfung, deren eigener Name traegt"
    );
}

/// C6.8: Filter und Verstecke sind zwei Zweige desselben Pruefschritts und
/// nicht zwei Regeln. Ein versteckter Treffer bleibt ausgeblendet, bis der
/// Nutzer die Verstecke einblendet.
#[test]
fn filter_und_verstecke_gehen_durch_denselben_pruefschritt() {
    let ordner = Pruefordner::neu("filter-und-verstecke");
    ordner.fuelldatei(".aaa-versteckt.txt", 1);
    ordner.fuelldatei("aaa-sichtbar.txt", 1);

    let mut modell = gefiltert(ordner.pfad(), "aaa");

    assert_eq!(namen(&modell), vec!["aaa-sichtbar.txt"]);

    modell.verstecke_umschalten();
    assert_eq!(
        namen(&modell),
        vec![".aaa-versteckt.txt", "aaa-sichtbar.txt"],
        "der eingeblendete Treffer kommt dazu"
    );
}

/// Beide Frager stellen dieselbe Frage: der Filter wirkt schon beim Anhaengen
/// eines Stapels und nicht erst beim Abschluss.
#[test]
fn der_filter_wirkt_schon_beim_anhaengen_eines_stapels() {
    let ordner = filterordner();

    let mut modell = Ordnermodell::neu(1);
    modell.filtertext_setzen("aaa");
    modell.anhaengen(lesen(ordner.pfad()).expect("Lesen gescheitert"));

    // Noch vor dem Abschluss, also in Lesereihenfolge und ungeordnet.
    let mut gesehen = namen(&modell);
    gesehen.sort_unstable();
    assert_eq!(
        gesehen,
        vec![
            "AAA-gross.txt",
            "aaa-ordner",
            "bbbaaaccc.rs",
            "stiller-ordner"
        ]
    );
}

/// C2.12: der Filter ist ein Pruefschritt vor dem Sortieren und kein Vergleich.
/// Die eingestellte Ordnung bleibt die Ordnung der gefilterten Liste, und
/// Ordner stehen weiter vorn.
#[test]
fn die_eingestellte_sortierung_bleibt_die_ordnung_der_gefilterten_liste() {
    let ordner = filterordner();
    let mut modell = gefiltert(ordner.pfad(), "aaa");

    modell.sortierung_setzen(Sortierung::neu(Schluessel::Name, Richtung::Absteigend));

    assert_eq!(
        namen(&modell),
        vec![
            "stiller-ordner",
            "aaa-ordner",
            "bbbaaaccc.rs",
            "AAA-gross.txt"
        ],
        "absteigend, Ordner weiter vorn, und keine Ordnung nach Passgenauigkeit"
    );
}

/// C1.11: faellt die Zeile weg, auf der die Auswahl stand, gibt es keine Zeile
/// mehr — der gemerkte Eintrag bleibt aber stehen und ist wieder da, sobald der
/// Filter faellt. Dasselbe Verhalten wie beim Ausblenden versteckter Dateien.
#[test]
fn eine_ausgefilterte_auswahl_kommt_beim_leeren_des_filters_zurueck() {
    let ordner = filterordner();
    let mut modell = geladenes_modell(ordner.pfad());
    let index = index_von(&modell, "ohne.txt");
    modell.auswahl_setzen(Some(index));

    modell.filtertext_setzen("aaa");
    assert_eq!(modell.auswahl_zeile(), None, "die Zeile ist weggefallen");
    assert_eq!(modell.auswahl(), Some(index), "der Eintrag bleibt gemerkt");

    modell.filter_leeren();
    let zeile = modell
        .auswahl_zeile()
        .expect("die Auswahl ist verloren gegangen");
    assert_eq!(
        modell.zeile(zeile).map(|e| e.name.as_str()),
        Some("ohne.txt")
    );
}

/// C6.2, C6.5: eine ausgeblendete Markierung besteht fort, zaehlt im
/// Markierungsstand mit und wirkt wieder, sobald der Filter faellt.
#[test]
fn die_markierung_besteht_unter_dem_filter_fort_und_wirkt_wieder() {
    let ordner = filterordner();
    let mut modell = geladenes_modell(ordner.pfad());
    let index = index_von(&modell, "ohne.txt");
    modell.markierung_umschalten(index);

    modell.filtertext_setzen("aaa");

    assert!(
        !namen(&modell).contains(&"ohne.txt"),
        "der markierte Eintrag ist ausgeblendet"
    );
    assert_eq!(
        modell.markierungsstand().zahl,
        1,
        "der Markierungsstand zaehlt ueber alle Eintraege"
    );
    assert!(modell.ist_markiert(index), "die Markierung besteht fort");

    modell.filter_leeren();
    assert!(modell.ist_markiert(index), "und sie wirkt wieder");
}

/// C6.3, C6.4: `alle_markieren` und `markierung_umkehren` wirken auf die
/// sichtbaren Eintraege, `markierung_aufheben` auf jeden.
#[test]
fn die_markierbefehle_behalten_ihren_zuschnitt_unter_dem_filter() {
    let ordner = filterordner();
    let mut modell = gefiltert(ordner.pfad(), "aaa");

    modell.alle_markieren();
    assert_eq!(
        modell.markierungsstand().zahl,
        4,
        "markiert werden die vier sichtbaren und nicht der ausgefilterte"
    );
    assert!(
        !modell.ist_markiert(index_von(&modell, "ohne.txt")),
        "der ausgefilterte Eintrag bleibt unberuehrt"
    );

    modell.markierung_umkehren();
    assert!(
        modell.markierungsstand().ist_leer(),
        "das Umkehren erreicht dieselben vier"
    );

    modell.alle_markieren();
    modell.markierung_umschalten(index_von(&modell, "ohne.txt"));
    assert_eq!(modell.markierungsstand().zahl, 5);
    modell.markierung_aufheben();
    assert!(
        modell.markierungsstand().ist_leer(),
        "jede Markierung aufheben heisst jede, auch die ausgeblendete"
    );
}

/// C1.14: die Ruecknahme eines Zeichens laesst die Liste wieder wachsen, und
/// bei leerem Filtertext ist nichts wegzunehmen.
#[test]
fn ein_zeichen_zurueck_laesst_die_liste_wieder_wachsen() {
    let ordner = filterordner();
    let mut modell = geladenes_modell(ordner.pfad());

    modell.zeichen_anhaengen('a');
    modell.zeichen_anhaengen('a');
    modell.zeichen_anhaengen('a');
    assert_eq!(modell.filtertext(), "aaa");
    let eng = modell.zeilenzahl();

    assert!(modell.letztes_zeichen_weg(), "es war etwas wegzunehmen");
    assert_eq!(modell.filtertext(), "aa");
    assert!(
        modell.zeilenzahl() >= eng,
        "die Liste waechst um die Eintraege, die wieder passen"
    );

    assert!(modell.letztes_zeichen_weg());
    assert!(modell.letztes_zeichen_weg());
    assert_eq!(
        modell.zeilenzahl(),
        5,
        "ohne Filtertext steht alles wieder da"
    );
    assert!(
        !modell.letztes_zeichen_weg(),
        "bei leerem Filtertext ist nichts wegzunehmen"
    );
}

/// Jede Aenderung des Filtertexts und jedes Einschalten der tiefen Suche setzt
/// die Befunde zurueck: sie waeren sonst Auskuenfte ueber einen frueheren
/// Filtertext.
#[test]
fn der_befund_faellt_bei_jeder_aenderung_der_frage_zurueck() {
    let ordner = filterordner();
    let mut modell = gefiltert(ordner.pfad(), "aaa");
    modell.tief_setzen(true);

    let still = index_von(&modell, "stiller-ordner");
    modell.befunde_setzen([(still, Befund::Treffer)]);
    assert_eq!(modell.befund(still), Befund::Treffer);

    modell.zeichen_anhaengen('x');
    assert_eq!(
        modell.befund(still),
        Befund::Unentschieden,
        "ein weiteres Zeichen stellt eine andere Frage"
    );

    modell.befunde_setzen([(still, Befund::Treffer)]);
    modell.tief_setzen(false);
    modell.tief_setzen(true);
    assert_eq!(
        modell.befund(still),
        Befund::Unentschieden,
        "das Einschalten der tiefen Suche fragt neu"
    );
}

/// Der kleingeschriebene Filtertext entsteht einmal je Aenderung und ist der
/// Wert, mit dem auch der Durchlauf vergleicht.
#[test]
fn der_kleingeschriebene_filtertext_laeuft_mit() {
    let mut modell = Ordnermodell::neu(1);

    assert!(!modell.filter_steht());
    modell.filtertext_setzen("AaA");
    assert_eq!(modell.filtertext(), "AaA");
    assert_eq!(modell.filter_klein(), "aaa");
    assert!(modell.filter_steht());

    modell.filter_leeren();
    assert_eq!(modell.filter_klein(), "");
    assert!(!modell.filter_steht());
}

// ---------------------------------------------------------------------------
// Der Durchlauf ueber die Unterbaeume (C3)
// ---------------------------------------------------------------------------

/// Nimmt alle Befunde entgegen, bis der Kanal schliesst.
///
/// Der Kanal schliesst, wenn der Arbeitsfaden geendet hat; ein Warten auf das
/// Fadenstueck braucht es dafuer nicht. **Was hier nicht ankommt, ist nicht
/// entschieden** — genau das ist der Unterschied zwischen „kein Treffer
/// darunter" und „noch nicht entschieden" (C3.13).
fn befunde_einsammeln(durchlauf: &Durchlauf) -> Vec<Befundmeldung> {
    let mut gesammelt = Vec::new();
    while let Ok(meldung) = durchlauf.befunde().recv() {
        gesammelt.push(meldung);
    }
    gesammelt
}

/// Startet einen Durchlauf ueber einen einzigen Auftrag und wartet seinen
/// Befund ab.
fn einen_ordner_entscheiden(wurzel: &Path, name: &str, filter_klein: &str) -> Vec<Befundmeldung> {
    let auftraege = vec![Auftrag {
        index: 7,
        name: name.to_owned(),
    }];
    let durchlauf = Durchlauf::starten(auftraege, wurzel.to_path_buf(), filter_klein.to_owned(), 1);
    befunde_einsammeln(&durchlauf)
}

#[test]
fn ein_treffer_tief_unten_entscheidet_den_ordner() {
    let ordner = Pruefordner::neu("durchlauf-tief");
    let tief = ordner
        .unter("aussen")
        .join("a")
        .join("b")
        .join("c")
        .join("d");
    fs::create_dir_all(&tief).expect("Kette laesst sich nicht anlegen");
    fs::write(tief.join("gesuchtes-blatt.txt"), b"x").expect("Blatt laesst sich nicht schreiben");
    // Beiwerk auf dem Weg, das den Filtertext nicht traegt.
    fs::write(ordner.unter("aussen").join("liesmich.md"), b"x").expect("Beiwerk");

    let befunde = einen_ordner_entscheiden(ordner.pfad(), "aussen", "gesuchtes");

    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "der Treffer liegt fuenf Ebenen tiefer und entscheidet den Ordner trotzdem"
    );
}

#[test]
fn ein_ordner_ohne_treffer_meldet_den_negativen_befund() {
    let ordner = Pruefordner::neu("durchlauf-ohne-treffer");
    let aussen = ordner.ordner("aussen");
    fs::create_dir_all(aussen.join("unten")).expect("Unterordner");
    fs::write(aussen.join("liesmich.md"), b"x").expect("Datei");
    fs::write(aussen.join("unten").join("notiz.txt"), b"x").expect("Datei");

    let befunde = einen_ordner_entscheiden(ordner.pfad(), "aussen", "gesuchtes");

    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "abgeschritten und nichts gefunden ist ein Befund und kein Schweigen"
    );
}

#[test]
fn ein_nicht_lesbarer_ordner_gilt_als_kein_treffer() {
    let ordner = Pruefordner::neu("durchlauf-gesperrt");
    let gesperrt = ordner.ordner("aussen");
    fs::write(gesperrt.join("gesuchtes-blatt.txt"), b"x").expect("Datei");
    fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht entziehen");

    let befunde = einen_ordner_entscheiden(ordner.pfad(), "aussen", "gesuchtes");

    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "was sich nicht oeffnen laesst, haelt den Durchlauf nicht an und meldet keinen Fehler"
    );
}

#[test]
fn eine_verknuepfung_auf_einen_ordner_meldet_kein_treffer() {
    let ordner = Pruefordner::neu("durchlauf-verknuepfung");
    let ziel = ordner.ordner("ziel");
    fs::write(ziel.join("gesuchtes-blatt.txt"), b"x").expect("Datei");
    ordner.verknuepfung("verweis", &ziel);

    // Ueber den echten Ordner ist der Treffer da: die Verknuepfung verdeckt ihn
    // nicht, es wird nur nicht in sie hinabgestiegen.
    assert_eq!(
        einen_ordner_entscheiden(ordner.pfad(), "ziel", "gesuchtes"),
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "derselbe Baum ueber seinen echten Namen"
    );

    assert_eq!(
        einen_ordner_entscheiden(ordner.pfad(), "verweis", "gesuchtes"),
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "in eine Verknuepfung wird nicht abgestiegen (C3.9)"
    );
}

/// C3.4, zweite Haelfte: Der Abbruch greift auch dort, wo kein einziges Mal
/// abgestiegen wird.
///
/// Der Pruefordner traegt 5.000 gewoehnliche Eintraege und **keinen einzigen
/// Unterordner**; er passiert die Stapelgrenze viermal und das Absteigen nie.
/// Eine Pruefung des Abbruchkennzeichens beim Absteigen bliebe hier wirkungslos.
///
/// **Gemessen wird an zwei Laeufen ueber denselben Ordner** und nicht an einem.
/// Der Kontrollauf ohne Abbruch muss `treffer: false` melden; erst dadurch
/// heisst das Schweigen des zweiten Laufs „der Abbruch hat gegriffen" und nicht
/// „der Durchlauf meldet fuer diesen Ordner ohnehin nichts". Ohne den
/// Kontrollauf bestuende die Probe auch bei einem vollstaendig kaputten
/// Durchlauf.
///
/// **Was diese Probe nicht entscheidet, und der Satz gehoert dazu:** die erste
/// Haelfte von C3.4, also die Zahl **zwei**. Das Kennzeichen steht hier, bevor
/// der Arbeitsfaden den ersten Stapel geholt hat, und wie viele Stapel zwischen
/// dem Setzen und dem Ende des Fadens liegen, ist von aussen an nichts
/// abzulesen: der Durchlauf meldet je Auftrag genau einen Befund und sonst
/// nichts, und eine Probe ueber eine Zeitspanne waere eine ueber den Planer des
/// Betriebssystems. Wer die Zahl messen will, braucht zuerst eine Groesse am
/// Durchlauf, an der die geleistete Arbeit abzulesen ist
/// (`issues/260815-0211_*_die-abbruchprobe-bricht-vor-dem-ersten-stapel-ab-…`).
/// Die 5.000 Eintraege stehen deshalb nicht als Beleg fuer die Zahl zwei da,
/// sondern als Ordner, der sicher mehr als einen Stapel braucht.
#[test]
fn der_abbruch_greift_in_einem_ordner_ohne_unterordner() {
    let ordner = Pruefordner::neu("durchlauf-abbruch");
    let flach = ordner.ordner("flach");
    for nummer in 0..5_000 {
        fs::write(flach.join(format!("eintrag-{nummer:06}.txt")), b"").expect("Datei");
    }
    assert!(
        lesen(&flach).expect("der flache Ordner ist lesbar").len() > 2 * STAPELGROESSE,
        "der Ordner soll mehr als zwei Stapel brauchen"
    );

    let auftrag = || {
        vec![Auftrag {
            index: 7,
            name: "flach".to_owned(),
        }]
    };

    // Kontrollauf: ohne Abbruch entscheidet derselbe Ordner.
    let ungestoert = Durchlauf::starten(
        auftrag(),
        ordner.pfad().to_path_buf(),
        "gibt-es-hier-nicht".to_owned(),
        1,
    );
    assert_eq!(
        befunde_einsammeln(&ungestoert),
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "ohne Abbruch wird der Ordner abgeschritten und entschieden"
    );

    // Derselbe Ordner mit gesetztem Abbruchkennzeichen.
    let durchlauf = Durchlauf::starten(
        auftrag(),
        ordner.pfad().to_path_buf(),
        "gibt-es-hier-nicht".to_owned(),
        2,
    );
    durchlauf.abbrechen();

    assert_eq!(
        befunde_einsammeln(&durchlauf),
        Vec::new(),
        "ein abgebrochener Durchlauf laesst den Ordner unentschieden, statt ihn zu entscheiden"
    );
}

/// C3.13: Jeder Auftrag bekommt genau einen Befund, und der negative kommt auf
/// drei Wegen.
#[test]
fn jeder_auftrag_bekommt_genau_einen_befund() {
    let ordner = Pruefordner::neu("durchlauf-drei-wege");

    let leer = ordner.ordner("ohne-fund");
    fs::write(leer.join("liesmich.md"), b"x").expect("Datei");

    let gesperrt = ordner.ordner("gesperrt");
    fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000)).expect("Rechte");

    let ziel = ordner.ordner("ziel");
    fs::write(ziel.join("gesuchtes-blatt.txt"), b"x").expect("Datei");
    ordner.verknuepfung("verweis", &ziel);

    let auftraege = [
        ("ohne-fund", false),
        ("gesperrt", false),
        ("verweis", false),
        ("ziel", true),
    ];
    let durchlauf = Durchlauf::starten(
        auftraege
            .iter()
            .enumerate()
            .map(|(stelle, (name, _))| Auftrag {
                index: stelle as u32,
                name: (*name).to_owned(),
            })
            .collect(),
        ordner.pfad().to_path_buf(),
        "gesuchtes".to_owned(),
        1,
    );

    let erwartet: Vec<Befundmeldung> = auftraege
        .iter()
        .enumerate()
        .map(|(stelle, (_, treffer))| Befundmeldung {
            index: stelle as u32,
            treffer: *treffer,
        })
        .collect();
    assert_eq!(befunde_einsammeln(&durchlauf), erwartet);
}

/// C3.8: Es gibt keine Tiefengrenze.
///
/// Zweihundert Ebenen, und der Treffer liegt ganz unten. Die Zahl ist nicht
/// beliebig gross gewaehlt: der Pfad waechst mit jeder Ebene, und
/// `PATH_MAX` liegt auf macOS bei 1.024 Bytes.
#[test]
fn der_durchlauf_kennt_keine_tiefengrenze() {
    let ordner = Pruefordner::neu("durchlauf-tiefe");
    let mut tief = ordner.unter("kette");
    for _ in 0..200 {
        tief = tief.join("e");
    }
    fs::create_dir_all(&tief).expect("Kette laesst sich nicht anlegen");
    fs::write(tief.join("gesuchtes-blatt.txt"), b"x").expect("Blatt");

    assert_eq!(
        einen_ordner_entscheiden(ordner.pfad(), "kette", "gesuchtes"),
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "zweihundert Ebenen tief, und der Abstieg laeuft ueber einen eigenen Stapel"
    );
}

/// Die Umgebungsvariable, die die Deskriptor-Kindprobe beauftragt. Ihr Wert ist
/// der Ordner, unter dem die tiefe Kette schon steht.
const AUFTRAG_DESKRIPTOREN: &str = "KRK_PROBE_DESKRIPTOREN";

/// Wie tief die Kette der Deskriptorprobe ist.
///
/// Deutlich mehr als die 64 Deskriptoren, unter denen das Kind laeuft, und
/// deutlich weniger als `PATH_MAX / 2`: bei zwei Bytes je Ebene und einem
/// Temporaerpfad von rund 60 Bytes bleibt die tiefste Stelle unter 500 Bytes.
const KETTENTIEFE: usize = 200;

/// Wie viele Deskriptoren das Kind hoechstens haben darf, damit die Probe misst
/// und nicht behauptet.
const DESKRIPTORSCHRANKE: usize = 100;

/// C3.8 und C3.10 **unter der Deskriptorgrenze, die ein Buendel bekommt**.
///
/// `der_durchlauf_kennt_keine_tiefengrenze` darueber legt zweihundert Ebenen an
/// und ist gruen — aber `cargo test` erbt die angehobene Grenze der
/// Anmeldesitzung, waehrend `launchctl limit maxfiles` 256 als Voreinstellung
/// fuehrt. Die Probe lief damit unter Bedingungen, die der Nutzer nicht hat,
/// und der Defekt `260815-0211` ist genau darin gewachsen: der Durchlauf hielt
/// einen Deskriptor je Ebene, erzeugte seinen eigenen `EMFILE` und meldete
/// darauf „kein Treffer darunter" — dieselbe Kette, dieselbe Frage, zwei
/// Antworten.
///
/// **Diese Probe misst den Fall.** Der Elternteil legt die Kette an; das Kind
/// laeuft ueber `/bin/sh` mit `ulimit -n 64` und entscheidet sie. Das Kind
/// misst seine Grenze zuerst selbst, indem es Deskriptoren nimmt, bis keiner
/// mehr kommt: ohne diese Zusicherung bestuende die Probe auch dann, wenn
/// `ulimit` nicht gegriffen haette, und waere wieder eine Behauptung.
///
/// **Ohne die Behebung meldet sie `treffer: false`.** Der alte Abstieg braucht
/// [`KETTENTIEFE`] Deskriptoren gleichzeitig, bekommt bei rund 55 keinen mehr
/// und uebergeht den Rest der Kette stillschweigend; der Treffer ganz unten
/// wird nie gelesen.
///
/// Angelegt und abgeraeumt wird der Baum vom **Elternteil**, und das ist kein
/// Zierrat: `remove_dir_all` haelt selbst einen Deskriptor je Ebene, koennte
/// die Kette unter der abgesenkten Grenze also nicht abraeumen.
#[test]
fn die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden() {
    let ordner = Pruefordner::neu("durchlauf-deskriptoren");
    let mut tief = ordner.unter("kette");
    for _ in 0..KETTENTIEFE {
        tief = tief.join("e");
    }
    fs::create_dir_all(&tief).expect("Kette laesst sich nicht anlegen");
    fs::write(tief.join("gesuchtes-blatt.txt"), b"x").expect("Blatt");

    let ergebnis = kind_mit_wenigen_deskriptoren(
        "kind_entscheidet_die_tiefe_kette",
        AUFTRAG_DESKRIPTOREN,
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "unter einer knappen Deskriptorgrenze faellt der Treffer aus der Antwort\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

/// Startet dieselbe Testdatei noch einmal, mit abgesenkter Deskriptorgrenze.
///
/// Der Umweg ueber `/bin/sh` ist der einzige ohne `setrlimit(2)`, und
/// `setrlimit(2)` waere eine sechste Bindung in [`krk_core::verzeichnis::sys`]
/// fuer etwas, das KRK selbst nicht braucht. `$0` ist die Testdatei, `$1` der
/// Name der Kindprobe.
fn kind_mit_wenigen_deskriptoren(name: &str, auftrag: &str, wert: &Path) -> std::process::Output {
    let selbst = std::env::current_exe().expect("die Testdatei kennt ihren Pfad nicht");
    std::process::Command::new("/bin/sh")
        .arg("-c")
        .arg("ulimit -n 64 && exec \"$0\" --exact --ignored --nocapture --test-threads 1 \"$1\"")
        .arg(&selbst)
        .arg(name)
        .env(auftrag, wert)
        .output()
        .expect("die Kindprobe laesst sich nicht starten")
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_PROBE_DESKRIPTOREN gestartet"]
fn kind_entscheidet_die_tiefe_kette() {
    let Some(ordner) = std::env::var_os(AUFTRAG_DESKRIPTOREN) else {
        return;
    };
    let ordner = std::path::PathBuf::from(ordner);

    // Erst die Grenze messen, dann die Frage stellen. Genommen wird, bis nichts
    // mehr kommt; was dabei zusammenkommt, ist die Zahl der Deskriptoren, die
    // dieses Kind ueberhaupt hat.
    let mut gehalten = Vec::new();
    while gehalten.len() < 4 * DESKRIPTORSCHRANKE {
        match fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(_) => break,
        }
    }
    let vorrat = gehalten.len();
    drop(gehalten);
    assert!(
        vorrat < DESKRIPTORSCHRANKE,
        "die Deskriptorgrenze des Kindes ist nicht abgesenkt: {vorrat} Deskriptoren \
         zugleich frei, die Probe wuerde nichts messen"
    );
    assert!(
        vorrat < KETTENTIEFE,
        "der Vorrat von {vorrat} Deskriptoren reicht fuer {KETTENTIEFE} Ebenen; \
         ein Abstieg mit einem Deskriptor je Ebene liefe hier durch"
    );

    assert_eq!(
        einen_ordner_entscheiden(&ordner, "kette", "gesuchtes"),
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "der Treffer liegt {KETTENTIEFE} Ebenen tief und faellt unter {vorrat} \
         freien Deskriptoren aus der Antwort"
    );
}

/// C3.1 und C3.8 als Aussagen ueber den Baum.
///
/// Beide sagen etwas ueber das **Fehlen** zu: keine zweite Lesemechanik, keine
/// Konstante fuer eine Tiefe. An keinem Rueckgabewert ist abzulesen, dass es
/// keine gibt.
///
/// Gelesen werden nur Code-Zeilen. Die Doc-Kommentare des Moduls nennen jede
/// Nadel im Klartext, damit ein Leser weiss, wonach gesucht wird; ein
/// `contains` ueber den ganzen Text faende sie dort. Die Hilfsfunktion steht
/// hier und nicht in `tests/gemeinsam/`, weil `tests/baum.rs` dieselbe fuehrt
/// und beide Ziele eigene Kisten sind — dieselbe Lage wie beim Pruefordner.
#[test]
fn der_durchlauf_liest_ueber_den_schwungleser_und_setzt_keine_grenze() {
    let (_, quelle) = gemeinsam::quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/verzeichnis/durchlauf.rs")
        .expect("das Modul des Durchlaufs steht im Baum");
    let code = code_zeilen(&quelle);

    assert!(
        code.iter().any(|zeile| zeile.contains("Schwungleser")),
        "der Durchlauf liest ueber dieselbe Huelle wie der Leser"
    );
    for fremde_mechanik in ["read_dir", "WalkDir", "getattrlistbulk"] {
        assert!(
            !code.iter().any(|zeile| zeile.contains(fremde_mechanik)),
            "neben dem Schwungleser steht eine zweite Lesemechanik: {fremde_mechanik}"
        );
    }
    let konstanten: Vec<&&str> = code
        .iter()
        .filter(|zeile| {
            let ohne_rand = zeile.trim_start();
            ohne_rand.starts_with("const ") || ohne_rand.starts_with("pub const ")
        })
        .collect();
    assert!(
        konstanten.is_empty(),
        "das Modul erklaert eine Konstante; eine Tiefengrenze faellt genau so an: {konstanten:?}"
    );
}

// ---------------------------------------------------------------------------
// Was der Filter nicht mehr hat: die Sprungmarke und jede Zeitmessung
// ---------------------------------------------------------------------------

/// Die Code-Zeilen einer Datei, ohne ihre Kommentare.
///
/// Die Doc-Kommentare dieser Runde nennen jede Nadel im Klartext, damit ein
/// Leser weiss, wonach gesucht wird; ein `contains` ueber den ganzen Text faende
/// sie dort. Gefragt ist aber, wer eine Sache **tut**, und das steht nie hinter
/// `//`. Dieselbe Hilfsfunktion fuehrt `der_durchlauf_liest_ueber_den_
/// schwungleser_und_setzt_keine_grenze` weiter oben in dieser Datei; sie steht
/// nicht in `tests/gemeinsam/`, weil `tests/baum.rs` eine gleichnamige fuehrt
/// und beide Ziele eigene Kisten sind — dieselbe Lage wie beim Pruefordner.
fn code_zeilen(inhalt: &str) -> Vec<&str> {
    inhalt
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
        .collect()
}

/// Die Code-Zeilen einer Datei **bis zu ihrem Pruefmodul**.
///
/// [`code_zeilen`] streicht Kommentarzeilen und sonst nichts; ein Pruefmodul
/// bleibt darin stehen. Fuer eine Nadel wie `Duration` ist das der Unterschied
/// zwischen einer Aussage ueber das Programm und einer ueber seine Proben: eine
/// Probe, die zwischen zwei Takten schlaeft, ist keine Uhr im Filter. Der
/// Schnitt steht am ersten Pruefmodul-Vermerk; eine Datei ohne ihn kommt ganz
/// zurueck.
fn code_zeilen_vor_dem_pruefmodul(inhalt: &str) -> Vec<&str> {
    let vermerk = concat!("#[cfg(", "test)]");
    code_zeilen(inhalt.split(vermerk).next().unwrap_or(inhalt))
}

/// Der Inhalt einer benannten Quelldatei, oder ein Fehlschlag.
///
/// Fehlt die Datei, schlaegt die Probe fehl statt still nichts zu zaehlen: eine
/// Zaehlung ueber eine leere Zeichenkette bestaetigt alles.
fn quelltext_von(name: &str) -> String {
    gemeinsam::quelldateien()
        .into_iter()
        .find(|(datei, _)| datei == name)
        .unwrap_or_else(|| panic!("{name} steht nicht im Baum"))
        .1
}

/// C1.5: Im Filter steht keine Zeitmessung, also laeuft der Filtertext nicht ab.
///
/// **Geprueft wird ein Fehlen, und an keinem Rueckgabewert ist ein Fehlen
/// abzulesen.** Ein Zeitgeber, der den Filtertext nach einer Pause
/// zuruecksetzte, waere in genau diesen fuenf Dateien zu sehen: den drei
/// Modulen des Kerns, die den Filter tragen, der Senke in `krk-ui`, in die das
/// getippte Zeichen laeuft, und der Tabliste. Die Sekundenregel der Sprungmarke
/// aus C2 der Runde 1 stand in der ersten dieser Dateien, als sie noch
/// `sprungmarke.rs` hiess.
///
/// **`krk-ui/src/tabs.rs` ist die fuenfte und war es nicht immer.** Seit
/// Schritt F2 traegt sie den Filtertext ueber den Ordnerwechsel, haelt den
/// `Durchlauf` je Tab, entscheidet, wann einer beginnt und vergeht, und zieht
/// die Befunde ein; ein Zeitgeber liesse sich dort ebenso gut unterbringen wie
/// in den vier anderen. Bis zum 260815 fehlte sie in der Liste, waehrend der
/// Doc-Kommentar von „den Dateien, die den Filter tragen" sprach
/// (`issues/260815-0211_*_die-probe-gegen-eine-zeitmessung-liest-vier-dateien-…`).
/// Aufnehmen liess sie sich erst mit [`code_zeilen_vor_dem_pruefmodul`], denn
/// ihr Pruefmodul schlaeft zwischen zwei Einzugstakten.
///
/// **Was diese Probe nicht entscheidet**, und der Satz gehoert dazu: der Weg
/// eines getippten Zeichens fuehrt vorher durch
/// `krk-ui/src/appkit/anwendung.rs`, und diese Datei fuehrt eine Uhr — fuer den
/// Anzeigeverzug der Dateioperationen, der mit dem Filter nichts zu tun hat.
/// Sie steht deshalb nicht in der Liste, und damit deckt keine Nadel den ganzen
/// Weg. Gedeckt ist der Filter selbst.
///
/// **`SystemTime` ist ausdruecklich keine Nadel**, und das ist keine Nachsicht,
/// sondern der Unterschied zwischen einer Uhr und einem Datum: ein
/// [`Eintrag`] traegt seine Aenderungszeit als `SystemTime`, und das
/// Pruefmodul von `modell.rs` baut damit seine Eintraege. Eine Uhr liest man
/// ab; ein Datum steht am Eintrag. Was eine Pause braucht, faellt trotzdem
/// unter die Nadeln: eine Spanne (`Duration`), ein monotoner Zeitpunkt
/// (`Instant`) oder das Ablesen einer beliebigen Uhr (`::now(`).
///
/// **Die Nadeln stehen zusammengesetzt da**, wie bei jeder Zaehlprobe dieses
/// Baums: als ein Stueck geschrieben faende jede sich in dieser Datei selbst.
/// Diese Datei liest zwar nur die vier benannten und nicht sich selbst, aber
/// die Bauform bleibt dieselbe, damit eine spaeter erweiterte Liste nicht
/// unbemerkt zur Selbstfundstelle wird.
#[test]
fn im_filter_steht_keine_zeitmessung() {
    let uhr = concat!("Inst", "ant");
    let dauer = concat!("Dura", "tion");
    let ablesen = concat!("::no", "w(");
    for datei in [
        "krk-core/src/verzeichnis/filter.rs",
        "krk-core/src/verzeichnis/modell.rs",
        "krk-core/src/verzeichnis/durchlauf.rs",
        "krk-ui/src/appkit/tabelle.rs",
        "krk-ui/src/tabs.rs",
    ] {
        let quelle = quelltext_von(datei);
        let code = code_zeilen_vor_dem_pruefmodul(&quelle);
        for nadel in [uhr, dauer, ablesen] {
            let treffer: Vec<&&str> = code.iter().filter(|zeile| zeile.contains(nadel)).collect();
            assert!(
                treffer.is_empty(),
                "{datei} misst die Zeit ({nadel}): {treffer:?}"
            );
        }
    }
}

/// C1.12: Die Sprungmarke ist restlos gefallen.
///
/// Gesucht wird im **ganzen** Baum und nicht in einer Datei: eine
/// stehengebliebene Aufrufstelle irgendwo waere genau der Befund, den diese
/// Probe holen soll. Vier Nadeln, und jede muss null Fundstellen haben — der
/// Typ, seine beiden Methoden, die Zeilensuche und die Konstante der Pause.
///
/// **`Nachschlag::Sprungmarke` behaelt seinen Namen und ist deshalb keine
/// Nadel.** Der Wert benennt „eine Taste ohne Zusatztaste, die keiner Funktion
/// gehoert", und das trifft nach der Runde 10 weiter zu; keine der vier Nadeln
/// findet ihn, weil vor jeder ein `::` oder ein `struct` steht oder sie
/// ueberhaupt anders heisst.
///
/// **Was diese Probe nicht entscheidet:** ob dieselbe Sache unter einem anderen
/// Namen wieder aufgebaut wird. Der Kopf von `tests/baum.rs` schreibt aus,
/// warum keine Suche im Quelltext das leisten kann.
#[test]
fn die_sprungmarke_steht_nirgends_mehr_im_baum() {
    let typ = concat!("struct Sprung", "marke");
    let tippen = concat!("Sprungmarke::", "tippen");
    let zeilensuche = concat!("erste_zeile", "_mit");
    let pause = concat!("PAU", "SE");
    for (name, inhalt) in gemeinsam::quelldateien() {
        let code = code_zeilen(&inhalt);
        for nadel in [typ, tippen, zeilensuche, pause] {
            let treffer: Vec<&&str> = code.iter().filter(|zeile| zeile.contains(nadel)).collect();
            assert!(
                treffer.is_empty(),
                "{name} traegt noch ein Stueck der Sprungmarke ({nadel}): {treffer:?}"
            );
        }
    }
}

/// C1.4: Die eine Zeichenregel steht einmal und hat genau zwei Aufrufer.
///
/// Erklaert wird sie in `krk-core/src/verzeichnis/filter.rs`, gerufen von der
/// Senke des Tippens in der Dateiliste und von der Tippsuche der
/// Belegungsansicht aus der Runde 7. **Gezaehlt werden Dateien und nicht
/// Aufrufe**: welche Datei fragt, ist die Aussage des Kriteriums; wie oft sie
/// innerhalb ihrer selbst fragt, ist es nicht.
///
/// Der Vergleich hat dieselbe Bauart: er steht ebenfalls einmal in `filter.rs`
/// und wird vom Pruefschritt des Ordnermodells und vom Durchlauf gerufen. Bis
/// zum 260815 stand er zweimal da, einmal je Rufer.
#[test]
fn die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer() {
    let zeichenregel = concat!("traegt_ein_", "dateiname");
    let vergleich = concat!("traegt_die", "_folge");
    let heimat = "krk-core/src/verzeichnis/filter.rs";

    let mut zeichenrufer = Vec::new();
    let mut vergleichsrufer = Vec::new();
    for (name, inhalt) in gemeinsam::quelldateien() {
        let code = code_zeilen(&inhalt);
        // Die Probenmodule der Heimatdatei rufen beide Regeln ebenfalls; sie
        // sind kein Rufer im Sinne des Kriteriums, und die Heimat faellt
        // deshalb aus der Zaehlung.
        if name == heimat {
            for (regel, nadel) in [("Zeichenregel", zeichenregel), ("Vergleich", vergleich)] {
                assert!(
                    code.iter()
                        .any(|zeile| zeile.contains(&format!("pub fn {nadel}("))),
                    "{heimat} erklaert die {regel} nicht mehr"
                );
            }
            continue;
        }
        if code.iter().any(|zeile| zeile.contains(zeichenregel)) {
            zeichenrufer.push(name.clone());
        }
        if code.iter().any(|zeile| zeile.contains(vergleich)) {
            vergleichsrufer.push(name);
        }
    }

    assert_eq!(
        zeichenrufer,
        vec![
            "krk-ui/src/appkit/tabelle.rs".to_owned(),
            "krk-ui/src/belegungsmodell.rs".to_owned(),
        ],
        "die Zeichenregel hat andere Rufer als den Filter und die Tippsuche"
    );
    assert_eq!(
        vergleichsrufer,
        vec![
            "krk-core/src/verzeichnis/durchlauf.rs".to_owned(),
            "krk-core/src/verzeichnis/modell.rs".to_owned(),
        ],
        "der Vergleich hat andere Rufer als der Pruefschritt und der Durchlauf"
    );
}
