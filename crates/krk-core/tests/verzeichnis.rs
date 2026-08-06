//! Abnahme des Verzeichnislesers und des Ordnermodells (Schritt 2 des Plans).
//!
//! Alle Tests laufen ohne Fenster und ohne AppKit. Sie legen ihre Pruefordner
//! selbst an und raeumen sie wieder ab; einen Pruefordner-Erzeuger gibt es
//! bewusst noch nicht, der ist Schritt 3.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use krk_core::verzeichnis::leser::{Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen};
use krk_core::verzeichnis::modell::Ordnermodell;
use krk_core::verzeichnis::sortierung::{Richtung, Schluessel, Sortierung};
use krk_core::verzeichnis::{Eintrag, Typ};

// ---------------------------------------------------------------------------
// Pruefordner
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
            "krk-test-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&pfad);
        fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
        Self { pfad }
    }

    fn pfad(&self) -> &Path {
        &self.pfad
    }

    fn datei(&self, name: &str, bytes: usize) {
        let mut datei =
            fs::File::create(self.pfad.join(name)).expect("Datei laesst sich nicht anlegen");
        datei
            .write_all(&vec![b'x'; bytes])
            .expect("Datei laesst sich nicht schreiben");
    }

    fn ordner(&self, name: &str) {
        fs::create_dir(self.pfad.join(name)).expect("Ordner laesst sich nicht anlegen");
    }

    fn verknuepfung(&self, name: &str, ziel: &str) {
        std::os::unix::fs::symlink(ziel, self.pfad.join(name))
            .expect("Verknuepfung laesst sich nicht anlegen");
    }

    /// Setzt `UF_HIDDEN`. Das Kennzeichen des Dateisystems ist der zweite Weg,
    /// auf dem ein Eintrag versteckt sein kann; der erste ist der fuehrende
    /// Punkt im Namen.
    fn verstecken(&self, name: &str) {
        let ergebnis = std::process::Command::new("/usr/bin/chflags")
            .arg("hidden")
            .arg(self.pfad.join(name))
            .status()
            .expect("chflags laesst sich nicht aufrufen");
        assert!(ergebnis.success(), "chflags hidden ist gescheitert");
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.pfad);
    }
}

/// Ein flacher Ordner mit `anzahl` Dateien, deren Namen fest zugeordnet sind.
fn ordner_mit_dateien(zweck: &str, anzahl: usize) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);
    for nummer in 0..anzahl {
        ordner.datei(&format!("eintrag-{nummer:06}.txt"), nummer % 17);
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
    ordner.datei("a.txt", 1);
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
    ordner.datei("klein.txt", 7);
    ordner.datei("gross.bin", 40_000);
    ordner.datei("leer.dat", 0);
    ordner.ordner("ein-ordner");
    ordner.verknuepfung("verweis", "klein.txt");
    ordner.datei(".versteckt", 3);
    ordner.datei("still.txt", 5);
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
    ordner.datei("datei.txt", 1);

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
    ordner.datei("Alpha.txt", 300);
    std::thread::sleep(pause);
    ordner.ordner("dir-a");
    std::thread::sleep(pause);
    ordner.datei("zeta.txt", 100);
    std::thread::sleep(pause);
    ordner.datei("Beta.txt", 200);
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
    ordner.datei("Zebra.md", 10);
    ordner.datei("Äpfel.zip", 10);
    ordner.datei("Bäume.md", 10);
    ordner.datei("LIESMICH", 10);
    ordner.datei("Übersicht.txt", 10);
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
    ordner.datei(".punkt", 1);
    ordner.datei(".noch-einer", 1);
    ordner.datei("sichtbar.txt", 1);
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
    ordner.datei(".punkt", 1);
    ordner.datei("sichtbar.txt", 1);

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
