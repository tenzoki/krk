//! Abnahme des Verzeichnislesers und des Ordnermodells (Schritt 2 des Plans).
//!
//! Alle Tests laufen ohne Fenster und ohne AppKit. Ihre Pruefordner kommen aus
//! `tests/gemeinsam/`, der einen Fassung fuer alle Abnahmeproben des Kerns; sie
//! tragen Prozesskennung und Laufnummer und raeumen sich in `Drop` selbst ab.

use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::{Arc, mpsc};
use std::time::{Duration, SystemTime};

use krk_core::verzeichnis::durchlauf::{Auftrag, Auftragsart, Befundmeldung, Durchlauf};
use krk_core::verzeichnis::inhalt::{Inhaltsbefund, traegt_der_inhalt};
use krk_core::verzeichnis::leser::{
    Abschluss, Lesevorgang, Meldung, STAPELGROESSE, lesen, lesen_hoechstens,
};
use krk_core::verzeichnis::modell::{Befund, Ordnermodell};
use krk_core::verzeichnis::sortierung::{Richtung, Schluessel, Sortierung};
use krk_core::verzeichnis::sys::{Schwungleser, ist_deskriptormangel};
use krk_core::verzeichnis::verweisziel::{self, Verweisziel};
use krk_core::verzeichnis::{Eintrag, Typ};

mod gemeinsam;
use gemeinsam::{Pruefordner, kind_mit_deskriptorgrenze, kindauftrag, mit_zeitschranke};

/// Ein flacher Ordner mit `anzahl` Dateien, deren Namen fest zugeordnet sind.
fn ordner_mit_dateien(zweck: &str, anzahl: usize) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);
    for nummer in 0..anzahl {
        ordner.fuelldatei(&format!("eintrag-{nummer:06}.txt"), nummer % 17);
    }
    ordner
}

/// Die Namen einer Eintragsliste, sortiert.
///
/// Zwei Laeufe ueber demselben Ordner vergleichbar zu machen, geht nur ueber
/// den Bestand: `Eintrag` traegt kein `PartialEq`, und eine Lesereihenfolge
/// sagt der Leser niemandem zu.
fn sortierte_namen(eintraege: &[Eintrag]) -> Vec<&str> {
    let mut gefunden: Vec<&str> = eintraege
        .iter()
        .map(|eintrag| eintrag.name.as_str())
        .collect();
    gefunden.sort_unstable();
    gefunden
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
// Der gedeckelte Leser
// ---------------------------------------------------------------------------

/// Der Deckel schneidet ab, und der Lesestand sagt, dass er es getan hat.
///
/// Die Zahl der Eintraege haengt am Deckel, `abgeschnitten` haengt daran, dass
/// mindestens ein Eintrag **nicht** aufgenommen wurde. Welche drei
/// zurueckkommen, sagt die Lesereihenfolge des Dateisystems und nicht diese
/// Probe; geprueft wird deshalb, dass jeder gelieferte Name aus dem angelegten
/// Bestand stammt, und nicht, welcher.
#[test]
fn ein_deckel_unter_dem_bestand_liefert_den_deckel_und_meldet_das_abschneiden() {
    let ordner = ordner_mit_dateien("deckel-darunter", 5);

    let stand = lesen_hoechstens(ordner.pfad(), 3).expect("Lesen gescheitert");

    assert_eq!(stand.eintraege.len(), 3, "der Deckel haelt nicht");
    assert!(
        stand.abgeschnitten,
        "der weggelassene Eintrag wird nicht gemeldet"
    );
    for eintrag in &stand.eintraege {
        assert!(
            eintrag.name.starts_with("eintrag-"),
            "{} stammt nicht aus dem angelegten Bestand",
            eintrag.name
        );
    }
}

/// Ein Deckel genau auf dem Bestand laesst nichts weg, und sagt genau das.
///
/// Das ist die Lage, fuer die der Leser einen `getattrlistbulk(2)` mehr ausgibt:
/// nach fuenf aufgenommenen Eintraegen unter dem Deckel fuenf steht noch nicht
/// fest, ob dahinter etwas kommt. Faellt `abgeschnitten` hier auf wahr, ist die
/// schwaechere Lesart "der Deckel wurde erreicht" zurueckgekommen, und jeder
/// Aufrufer, der darauf eine negative Antwort stuetzt, gibt sie zu Unrecht.
#[test]
fn ein_deckel_genau_auf_dem_bestand_meldet_kein_abschneiden() {
    let ordner = ordner_mit_dateien("deckel-genau", 5);

    let stand = lesen_hoechstens(ordner.pfad(), 5).expect("Lesen gescheitert");

    assert_eq!(stand.eintraege.len(), 5);
    assert!(
        !stand.abgeschnitten,
        "ein vollstaendig gelesener Ordner gilt als abgeschnitten"
    );
}

/// `lesen` ist `lesen_hoechstens` ohne Deckel, und das steht nicht nur im
/// Doc-Kommentar.
///
/// Die Aussage der Probe ist der Bestand: seit der Runde 16 hat `lesen` keine
/// eigene Leserschleife mehr, und was daran zu pruefen ist, ist dass der Umbau
/// nichts weggelassen hat.
#[test]
fn lesen_liefert_denselben_bestand_wie_der_hoechste_deckel() {
    let ordner = ordner_mit_dateien("deckel-ohne", 5);

    let ohne_deckel = lesen(ordner.pfad()).expect("Lesen gescheitert");
    let stand = lesen_hoechstens(ordner.pfad(), usize::MAX).expect("Lesen gescheitert");

    assert_eq!(ohne_deckel.len(), 5);
    assert!(
        !stand.abgeschnitten,
        "der hoechste Deckel meldet ein Abschneiden"
    );
    assert_eq!(
        sortierte_namen(&ohne_deckel),
        sortierte_namen(&stand.eintraege)
    );
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

/// Ein gelesenes Modell mit stehendem Filtertext und **flacher** Suche.
///
/// Die tiefe Suche wird ausdruecklich abgeschaltet: die Vorbelegung von
/// [`Ordnermodell::neu`] ist seit dem 260826 "ein", und die Proben darunter
/// messen den flachen Zweig. Wer den tiefen misst, ruft danach
/// `tief_setzen(true)`. Die Vorbelegung selbst haelt
/// `die_tiefe_suche_ist_die_vorbelegung`.
fn gefiltert(pfad: &Path, filtertext: &str) -> Ordnermodell {
    let mut modell = geladenes_modell(pfad);
    modell.tief_setzen(false);
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

    assert!(
        !modell.tief(),
        "diese Probe faehrt flach; `gefiltert` schaltet ab"
    );
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

/// Die tiefe Suche ist die Vorbelegung, der Inhaltsfilter nicht.
///
/// **Die Probe haelt die Vorbelegung selbst und nicht ihre Wirkung.** Jede
/// andere Filterprobe dieser Datei setzt den Stand ausdruecklich, damit sie
/// den Zweig misst, den ihr Name nennt; keine von ihnen sagt deshalb noch
/// etwas darueber, womit ein frisches Modell beginnt. Genau das steht hier.
///
/// Mitgehalten wird die Folge, die sich niemand gewuenscht hat: die Schwelle
/// des Inhaltsfilters haengt am Stand der tiefen Suche
/// ([`krk_core::verzeichnis::filter::inhaltsschwelle`]), und ab Werk gilt
/// damit die tiefe Fuenf und nicht mehr die flache Drei. Wer die Vorbelegung
/// zurueckdreht, macht diese Zeile rot und liest den Grund hier.
#[test]
fn die_tiefe_suche_ist_die_vorbelegung() {
    let frisch = Ordnermodell::neu(1);

    assert!(frisch.tief(), "\"Deep\" ist ab Werk eingeschaltet");
    assert!(!frisch.inhalt(), "\"Content\" ist ab Werk ausgeschaltet");

    let mut modell = handmodell([
        handeintrag("bbbaaaccc.rs", Typ::Datei),
        handeintrag("ohne.txt", Typ::Datei),
    ]);
    // `handmodell` faehrt flach; hier wird die Vorbelegung wiederhergestellt,
    // weil es um sie geht.
    modell.tief_setzen(true);
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaaa");

    assert!(
        !modell.inhalt_wirkt(),
        "ab Werk gilt die Schwelle der tiefen Suche: vier Zeichen reichen nicht"
    );

    modell.zeichen_anhaengen('a');

    assert!(
        modell.inhalt_wirkt(),
        "das fuenfte Zeichen erreicht die tiefe Schwelle"
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
    modell.tief_setzen(false);
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

/// Ein Befund gilt nur zu der Frage, die ihn erzeugt hat — und die Frage ist
/// der Filtertext und die Angabe, ob der Inhalt dabei zaehlt.
///
/// **Die dritte Behauptung ist die eigentliche.** Der Stand der tiefen Suche
/// entscheidet, ob die Frage fuer einen Ordner ueberhaupt gestellt wird, und
/// nicht, wie sie ausgeht: derselbe Unterbaum wird immer gleich abgeschritten.
/// Ein Umlegen von „Deep", das die Schwelle des Inhaltsfilters nicht kreuzt,
/// darf deshalb keine Antwort wegwerfen. Bis zum 260816 warf das Einschalten
/// jede weg.
#[test]
fn ein_befund_gilt_nur_zu_seiner_frage() {
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
        Befund::Treffer,
        "\"Deep\" allein aendert die Frage nicht; die Antwort gilt weiter"
    );

    modell.inhalt_setzen(true);
    assert!(
        !modell.inhalt_wirkt(),
        "vier Zeichen liegen unter der Schwelle der tiefen Suche; der Schalter \
         steht und wirkt nicht"
    );
    assert_eq!(
        modell.befund(still),
        Befund::Treffer,
        "ein Schalter, der nichts bewirkt, aendert die Frage nicht"
    );

    modell.zeichen_anhaengen('x');
    modell.befunde_setzen([(still, Befund::Treffer)]);
    assert!(
        modell.inhalt_wirkt(),
        "fuenf Zeichen erreichen die Schwelle"
    );
    modell.inhalt_setzen(false);
    assert_eq!(
        modell.befund(still),
        Befund::Unentschieden,
        "jetzt zaehlt der Inhalt nicht mehr mit, und das ist eine andere Frage"
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
// Der Inhaltsfilter aus C1, C2 und C5: derselbe Pruefschritt, ein Zweig mehr
// ---------------------------------------------------------------------------

/// Ein Eintrag fuer die Proben unten, ohne Platte.
///
/// Die Sortierschluessel entstehen dabei so, wie sie es beim Lesen tun.
fn handeintrag(name: &str, typ: Typ) -> Eintrag {
    Eintrag::neu(name.to_owned(), 0, SystemTime::UNIX_EPOCH, typ)
}

/// Ein fertiges Ordnermodell aus Eintraegen von Hand.
///
/// **Diese Proben brauchen keine Platte**, und das ist kein Sparen, sondern die
/// Aussage: der Pruefschritt entscheidet ueber den **Befund**, nicht ueber eine
/// Datei. Wer den Befund von Hand setzt, misst genau den Zweig, um den es geht,
/// und nicht nebenbei den Leseweg. Wer eine Datei wirklich liest, misst
/// [`krk_core::verzeichnis::inhalt`], und das steht anderswo.
fn handmodell(eintraege: impl IntoIterator<Item = Eintrag>) -> Ordnermodell {
    let mut modell = Ordnermodell::neu(1);
    // Flach, aus demselben Grund wie bei `gefiltert` darueber: die Schwelle des
    // Inhaltsfilters haengt am Stand der tiefen Suche, und die Proben darunter
    // messen die flache Drei und die tiefe Fuenf einzeln.
    modell.tief_setzen(false);
    modell.anhaengen(eintraege);
    modell.abschliessen();
    modell
}

/// Der Ordner der Inhaltsproben, von Hand gebaut.
///
/// | Name             | Typ    | traegt `aaa` im Namen |
/// |---|---|---|
/// | `bbbaaaccc.rs`   | Datei  | ja |
/// | `ohne.txt`       | Datei  | nein — der Inhaltskandidat |
/// | `zweite.txt`     | Datei  | nein — bleibt unentschieden |
/// | `stiller-ordner` | Ordner | nein |
fn inhaltsmodell() -> Ordnermodell {
    handmodell([
        handeintrag("bbbaaaccc.rs", Typ::Datei),
        handeintrag("ohne.txt", Typ::Datei),
        handeintrag("zweite.txt", Typ::Datei),
        handeintrag("stiller-ordner", Typ::Ordner),
    ])
}

/// C1.1, C1.2, C1.10: die Schwelle von drei Zeichen ohne tiefe Suche, und was
/// unterhalb von ihr geschieht.
///
/// **Der Befund wird nach dem Filtertext gesetzt und nicht davor.** Jede
/// Aenderung des Filtertexts setzt die Befunde zurueck, weil sie Auskuenfte
/// ueber eine frueher gestellte Frage waeren; in der Anwendung liefert der
/// Durchlauf sie danach neu. Die Probe schreibt denselben Ablauf ab.
///
/// C1.10 steht in derselben Probe: `zweite.txt` bleibt `Unentschieden` und
/// steht deshalb nicht. Die Liste beginnt bei den Namenstreffern und waechst
/// waehrend des Lesens.
#[test]
fn der_inhaltsfilter_wirkt_ab_drei_zeichen_und_darunter_nicht() {
    let mut modell = inhaltsmodell();
    modell.filtertext_setzen("aaa");
    let kandidat = index_von(&modell, "ohne.txt");
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert!(!modell.inhalt(), "\"Content\" ist aus die Vorbelegung");
    assert!(!modell.inhalt_wirkt());
    assert_eq!(
        namen(&modell),
        vec!["stiller-ordner", "bbbaaaccc.rs"],
        "ohne \"Content\" entscheidet ueber eine Datei allein ihr Name"
    );

    modell.inhalt_setzen(true);
    // Das Einschalten setzt die Befunde zurueck, wie das Einschalten der
    // tiefen Suche es tut; der Durchlauf liefert sie danach neu.
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert!(modell.inhalt_wirkt(), "drei Zeichen ohne tiefe Suche");
    assert_eq!(
        namen(&modell),
        vec!["stiller-ordner", "bbbaaaccc.rs", "ohne.txt"],
        "die Datei mit dem Inhaltstreffer fehlt"
    );
    assert!(
        !namen(&modell).contains(&"zweite.txt"),
        "eine noch nicht gelesene Datei steht nicht (C1.10)"
    );

    modell.filtertext_setzen("aa");
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert!(
        !modell.inhalt_wirkt(),
        "zwei Zeichen bleiben unter der Schwelle"
    );
    assert_eq!(
        namen(&modell),
        vec!["stiller-ordner", "bbbaaaccc.rs"],
        "unterhalb der Schwelle zeigt die Liste dasselbe wie ohne \"Content\""
    );
}

/// C2.10: mit eingeschalteter tiefer Suche steigt die Schwelle auf fuenf. Vier
/// Zeichen nehmen die Inhaltstreffer weg, ein fuenftes holt sie zurueck.
///
/// Der Ordner steht in dieser Probe nicht mehr: bei tiefer Suche entscheidet
/// ueber ihn sein Befund, und der ist `Unentschieden`. Das ist die Regel der
/// Runde 10 und hier nur der Hintergrund.
#[test]
fn die_tiefe_suche_hebt_die_schwelle_auf_fuenf_zeichen() {
    let mut modell = inhaltsmodell();
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaaa");
    let kandidat = index_von(&modell, "ohne.txt");
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert!(modell.inhalt_wirkt(), "vier Zeichen ohne tiefe Suche");
    assert!(namen(&modell).contains(&"ohne.txt"));

    modell.tief_setzen(true);
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert!(
        !modell.inhalt_wirkt(),
        "vier Zeichen liegen unter der Schwelle der tiefen Suche"
    );
    assert!(
        !namen(&modell).contains(&"ohne.txt"),
        "die gestiegene Schwelle nimmt den Inhaltstreffer weg"
    );

    modell.zeichen_anhaengen('a');
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert_eq!(modell.filtertext(), "aaaaa");
    assert!(
        modell.inhalt_wirkt(),
        "fuenf Zeichen erreichen die Schwelle"
    );
    assert!(
        namen(&modell).contains(&"ohne.txt"),
        "das fuenfte Zeichen holt den Inhaltstreffer zurueck"
    );
}

/// Die Schwelle zaehlt **Zeichen und keine Bytes**. Ein getipptes `äöü` sind
/// drei Zeichen und sechs Bytes; gerechnet wird mit den drei.
#[test]
fn die_schwelle_zaehlt_zeichen_und_keine_bytes() {
    let mut modell = inhaltsmodell();
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("äöü");

    assert_eq!(modell.filtertext().len(), 6, "sechs Bytes");
    assert!(
        modell.inhalt_wirkt(),
        "drei Zeichen erreichen die flache Schwelle"
    );
}

/// C2.6: ohne stehenden Filtertext aendert "Content" nichts an der Liste.
///
/// Dieselbe Aussage wie `ohne_filtertext_aendert_die_tiefe_suche_nichts` fuer
/// die tiefe Suche, und aus demselben Grund **auch** mit einem gesetzten
/// Befund: ohne Filtertext wird er gar nicht erst gefragt.
#[test]
fn ohne_filtertext_aendert_der_inhaltsfilter_nichts() {
    let mut modell = inhaltsmodell();
    let vorher: Vec<String> = namen(&modell).into_iter().map(str::to_owned).collect();
    assert!(!modell.filter_steht(), "diese Probe faehrt ohne Filtertext");

    let kandidat = index_von(&modell, "ohne.txt");
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);
    modell.inhalt_setzen(true);

    assert!(
        modell.inhalt(),
        "das Kennzeichen steht, auch ohne Filtertext"
    );
    assert!(
        !modell.inhalt_wirkt(),
        "ein leerer Filtertext bleibt unter jeder Schwelle"
    );
    assert_eq!(
        namen(&modell),
        vorher,
        "ohne Filtertext entscheidet der Befund ueber keine Zeile"
    );
}

/// C2.9: das Ausschalten nimmt die Zeilen weg, die allein wegen ihres Inhalts
/// standen — und setzt den Befundvektor zurueck.
///
/// **Bis zum 260816 blieb der Vektor stehen**, mit der Begruendung, ihn lese
/// beim Ausschalten fuer eine Datei niemand. Fuer eine Datei stimmte das; die
/// Probe darunter zeigt, was es fuer einen Ordner hiess. Nachgeprueft wird am
/// Wert selbst und nicht an der Liste: an der Liste waere ein zurueckgesetzter
/// Vektor hier nicht von einem stehenden zu unterscheiden.
#[test]
fn das_ausschalten_nimmt_die_inhaltszeilen_weg_und_setzt_den_befund_zurueck() {
    let mut modell = inhaltsmodell();
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaa");
    let kandidat = index_von(&modell, "ohne.txt");
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);
    assert!(namen(&modell).contains(&"ohne.txt"));

    modell.inhalt_setzen(false);

    assert!(
        !namen(&modell).contains(&"ohne.txt"),
        "die Zeile stand allein wegen ihres Inhalts"
    );
    assert_eq!(
        modell.befund(kandidat),
        Befund::Unentschieden,
        "der Befund beantwortete eine Frage, die nicht mehr gestellt ist"
    );
}

/// C2.9 fuer einen **Ordner**: das Ausschalten von „Content" nimmt auch seine
/// Zeile sofort weg, ohne auf einen neuen Unterbaumlauf zu warten.
///
/// **Der Befund, um den es geht.** Bei eingeschaltetem „Deep" haengt die Zeile
/// eines Ordners ohne Namenstreffer am Befundvektor, und der hing bis zum
/// 260816 nicht daran, ob „Content" steht. Ein Ordner, den allein ein
/// **gelesener Dateiinhalt** unter ihm ins Bild gebracht hatte, blieb nach dem
/// Ausschalten stehen, bis der neue Lauf ihn einholte — bei einem grossen
/// Unterbaum minutenlang, und C2.9 verlangt „sofort"
/// (`issues/260816-1930_*_content-ausschalten-laesst-ordnerzeilen-auf-einem-veralteten-inhaltsbefund-stehen.md`).
///
/// **Gemessen wird an der Zeile und nicht am Vektor**, denn die Zeile ist die
/// Zusage. Der Befund wird von Hand gesetzt, wie in jeder Probe dieses
/// Abschnitts: welcher Lauf ihn erzeugt haette, ist hier ohne Belang — er
/// haette ihn nur mit gesetztem „Content" erzeugen koennen.
#[test]
fn das_ausschalten_des_inhaltsfilters_nimmt_auch_die_ordnerzeile_sofort_weg() {
    let mut modell = inhaltsmodell();
    modell.tief_setzen(true);
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaaaa");
    assert!(
        modell.inhalt_wirkt(),
        "fuenf Zeichen liegen ueber der tiefen Schwelle"
    );

    let ordner = index_von(&modell, "stiller-ordner");
    modell.befunde_setzen([(ordner, Befund::Treffer)]);
    assert!(
        namen(&modell).contains(&"stiller-ordner"),
        "mit beiden Schaltern steht die Ordnerzeile auf ihrem Befund"
    );

    modell.inhalt_setzen(false);

    assert!(
        !namen(&modell).contains(&"stiller-ordner"),
        "die Ordnerzeile stand auf einem Befund, den nur der Inhaltsfilter \
         erzeugt haben kann"
    );
    assert!(
        modell.tief(),
        "\"Deep\" steht weiter; es ist \"Content\", das gefallen ist"
    );
}

/// Was das Ausschalten kostet: eine Ordnerzeile, die auf einem **Namen** unter
/// sich stand, faellt mit und kommt mit dem neuen Lauf wieder.
///
/// Die Kehrseite der Probe darueber, und sie steht hier, damit der Preis
/// gemessen ist und nicht bloss behauptet. Der Befundvektor sagt, **dass**
/// etwas unter einem Ordner liegt, und nicht **warum**; ihn nach dem Grund zu
/// fragen hiesse, den Grund ueber den Befundkanal zu melden.
#[test]
fn das_ausschalten_nimmt_auch_eine_namentlich_begruendete_ordnerzeile_mit() {
    let mut modell = inhaltsmodell();
    modell.tief_setzen(true);
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaaaa");

    let ordner = index_von(&modell, "stiller-ordner");
    modell.befunde_setzen([(ordner, Befund::Treffer)]);

    modell.inhalt_setzen(false);

    assert!(!namen(&modell).contains(&"stiller-ordner"));
    assert_eq!(
        modell.befund(ordner),
        Befund::Unentschieden,
        "der neue Lauf entscheidet ihn noch einmal"
    );
}

/// Ein ausgeblendeter Eintrag bekommt keinen Auftrag, und das Einblenden gibt
/// ihm einen.
///
/// **Der Befund, um den es geht.** Die Auftragsliste stand bis zum 260816 in
/// `krk-ui` und war eine zweite Fassung des Pruefschritts; sie kannte dessen
/// ersten Zweig nicht und erteilte deshalb Auftraege fuer Eintraege, deren
/// Zeile gar nicht stehen kann. Solange ein Auftrag einen Metadatengang kostete
/// und nur Ordner traf, war das ein Vorrat fuer den Fall, dass der Nutzer die
/// Verstecke einblendet. Seit der Runde 11 kostet er je verstecktem Eintrag ein
/// `open(2)` und bis zu 1 MB gelesene Bytes — ein Quellbaum mit „Deep" und
/// „Content" las damit sein ganzes `.git` mit
/// (`issues/260816-1931_*_der-inhaltsfilter-liest-versteckte-dateien-und-steigt-in-versteckte-ordner-ab.md`).
///
/// **Der Handel ist umgedreht und nicht abgeschafft:** wer nie einblendet,
/// zahlt nichts mehr; wer einblendet, bekommt die Auftraege in demselben
/// Augenblick und braucht dafuer einen neuen Lauf. Die zweite Haelfte der Probe
/// misst genau das.
///
/// **Vom Abstieg handelt diese Probe nicht.** Ein Treffer unter einem
/// versteckten Ordner ist ein Treffer unter dem sichtbaren Ordner darueber; ihn
/// zu uebergehen waere eine neue Regel und keine Ersparnis, und der Durchlauf
/// steigt deshalb unveraendert ab.
#[test]
fn ein_ausgeblendeter_eintrag_bekommt_keinen_auftrag() {
    let mut modell = handmodell([
        handeintrag("ohne.txt", Typ::Datei),
        handeintrag(".geheim.txt", Typ::Datei),
        handeintrag("stiller-ordner", Typ::Ordner),
        handeintrag(".verborgen", Typ::Ordner),
    ]);
    modell.tief_setzen(true);
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaaaa");
    assert!(
        modell.inhalt_wirkt(),
        "fuenf Zeichen ueber der tiefen Schwelle"
    );

    let auftragsnamen = |modell: &Ordnermodell| -> Vec<String> {
        modell
            .auftraege()
            .iter()
            .map(|auftrag| modell.eintraege()[auftrag.index as usize].name.clone())
            .collect()
    };

    assert_eq!(
        auftragsnamen(&modell),
        vec!["ohne.txt".to_owned(), "stiller-ordner".to_owned()],
        "die beiden versteckten Eintraege koennen keine Zeile bekommen und \
         werden deshalb nicht gelesen"
    );

    modell.verstecke_ausblenden_setzen(false);

    assert_eq!(
        auftragsnamen(&modell),
        vec![
            "ohne.txt".to_owned(),
            ".geheim.txt".to_owned(),
            "stiller-ordner".to_owned(),
            ".verborgen".to_owned(),
        ],
        "eingeblendet stehen sie unter demselben Vorbehalt wie jeder andere"
    );
}

/// Die Auftragsarten bleiben, was sie waren, auch wenn die Liste jetzt aus dem
/// Ordnermodell kommt.
///
/// Der Schnitt „Ordner oder Verknuepfung gegen gewoehnliche Datei" ist derselbe,
/// den der Pruefschritt zieht — er ist es jetzt buchstaeblich und nicht mehr
/// nur der Absicht nach.
#[test]
fn die_auftragsliste_traegt_je_typ_die_richtige_art() {
    let mut modell = handmodell([
        handeintrag("ohne.txt", Typ::Datei),
        handeintrag("stiller-ordner", Typ::Ordner),
        handeintrag("verweis", Typ::Verknuepfung),
    ]);
    modell.tief_setzen(true);
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaaaa");

    let arten: Vec<(u32, Auftragsart)> = modell
        .auftraege()
        .iter()
        .map(|auftrag| (auftrag.index, auftrag.art))
        .collect();

    assert_eq!(
        arten,
        vec![
            (0, Auftragsart::Inhalt),
            (1, Auftragsart::Unterbaum),
            (2, Auftragsart::Unterbaum),
        ],
        "eine Verknuepfung zaehlt zu den Ordnern, weil der Nutzer in sie \
         hineinnavigiert"
    );
}

/// C1.3, Sichtbarkeitshaelfte: traegt der Name die Folge, steht die Zeile ohne
/// jeden Befund.
///
/// Die andere Haelfte des Kriteriums — dass die Datei dabei ungelesen bleibt —
/// haengt am Durchlauf und an der Auftragsliste des Tabs und steht dort.
#[test]
fn ein_namentlicher_treffer_steht_ohne_jeden_befund() {
    let mut modell = inhaltsmodell();
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaa");

    let namenstreffer = index_von(&modell, "bbbaaaccc.rs");
    assert_eq!(
        modell.befund(namenstreffer),
        Befund::Unentschieden,
        "fuer diese Datei ist nichts gelesen worden"
    );
    assert!(
        namen(&modell).contains(&"bbbaaaccc.rs"),
        "der Namenstreffer steht, und der Inhaltszweig liegt hinter ihm"
    );
}

/// C5.4, C5.5: die Frage der Dateizelle hat alle Vorbedingungen vor sich und
/// gibt fuer jeden anderen Fall `false`.
///
/// Sie ist derselbe Rumpf wie der Dateizweig des Pruefschritts; gemessen wird
/// hier, dass die Vorbedingungen davor vollstaendig sind. **C5.4 steht im
/// Namenstreffer**: er steht in der Liste, aber nicht wegen seines Inhalts, und
/// damit ueberschneiden sich die beiden Treffergruende nicht.
#[test]
fn steht_wegen_des_inhalts_antwortet_nur_fuer_die_eine_lage() {
    let mut modell = inhaltsmodell();
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaa");
    let kandidat = index_von(&modell, "ohne.txt");
    let namenstreffer = index_von(&modell, "bbbaaaccc.rs");
    let ordner = index_von(&modell, "stiller-ordner");
    let unentschieden = index_von(&modell, "zweite.txt");
    modell.befunde_setzen([
        (kandidat, Befund::Treffer),
        (namenstreffer, Befund::Treffer),
        (ordner, Befund::Treffer),
    ]);

    assert!(
        modell.steht_wegen_des_inhalts(kandidat),
        "die eine Lage, fuer die die Frage ja heisst"
    );
    assert!(
        !modell.steht_wegen_des_inhalts(namenstreffer),
        "ueber diese Zeile entscheidet ihr Name (C5.4)"
    );
    assert!(
        !modell.steht_wegen_des_inhalts(ordner),
        "fuer einen Ordner trifft die Kennzeichnung keine Aussage (C5.5)"
    );
    assert!(
        !modell.steht_wegen_des_inhalts(unentschieden),
        "ungelesen heisst nicht: steht wegen des Inhalts"
    );
    assert!(
        !modell.steht_wegen_des_inhalts(9_999),
        "ueber einen Eintrag, den es nicht gibt, ist nichts bekannt"
    );

    modell.filter_leeren();
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);
    assert!(
        !modell.steht_wegen_des_inhalts(kandidat),
        "ohne Filtertext steht jede Zeile ohnehin"
    );
}

/// Eine symbolische Verknuepfung steht nie wegen ihres Inhalts, und zwar aus
/// demselben Schnitt, den der Pruefschritt zieht: fuer die Sichtbarkeit zaehlt
/// sie als Ordner.
#[test]
fn eine_verknuepfung_steht_nie_wegen_ihres_inhalts() {
    let mut modell = handmodell([
        handeintrag("ohne.txt", Typ::Datei),
        handeintrag("verweis", Typ::Verknuepfung),
    ]);
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aaa");
    let verweis = index_von(&modell, "verweis");
    modell.befunde_setzen([(verweis, Befund::Treffer)]);

    assert!(!modell.steht_wegen_des_inhalts(verweis));
}

/// Unterhalb der Schwelle antwortet auch die Frage der Zelle mit `false`. Sie
/// rechnet die Schwelle nicht nach, sondern geht durch dieselbe eine Stelle.
#[test]
fn unter_der_schwelle_steht_keine_zeile_wegen_ihres_inhalts() {
    let mut modell = inhaltsmodell();
    modell.inhalt_setzen(true);
    modell.filtertext_setzen("aa");
    let kandidat = index_von(&modell, "ohne.txt");
    modell.befunde_setzen([(kandidat, Befund::Treffer)]);

    assert!(!modell.inhalt_wirkt());
    assert!(!modell.steht_wegen_des_inhalts(kandidat));
}

// ---------------------------------------------------------------------------
// Der Inhaltsbefund: traegt der Text dieser Datei die Folge? (C1.4 bis C1.6, C6.9)
// ---------------------------------------------------------------------------

/// Fragt den Inhaltsbefund auf einem Arbeitsfaden und bricht ab, wenn nach
/// `schranke` keine Antwort da ist.
///
/// Dieselbe Bauart wie `bis_zur_grenze_mit_zeitschranke` in `tests/text.rs`, und
/// aus demselben Grund: eine benannte Roehre ohne Schreiber laesst ein
/// gewoehnliches `open(2)` warten, und eine haengende Probe unterscheidet sich
/// von einer laufenden durch nichts. Der Faden bleibt im Fehlerfall stehen; das
/// Testziel endet ohnehin gleich danach.
fn inhalt_mit_zeitschranke(
    pfad: &Path,
    filter_klein: &str,
    grenze: u64,
    schranke: Duration,
) -> Inhaltsbefund {
    let (sender, empfaenger) = mpsc::channel();
    let pfad = pfad.to_path_buf();
    let filter_klein = filter_klein.to_owned();
    std::thread::spawn(move || {
        let _ = sender.send(traegt_der_inhalt(&pfad, &filter_klein, grenze));
    });
    empfaenger.recv_timeout(schranke).unwrap_or_else(|_| {
        panic!("traegt_der_inhalt ist nach {schranke:?} nicht zurueckgekommen; das Oeffnen haengt")
    })
}

/// C1.4: Der Text traegt die Folge, oder er traegt sie nicht.
///
/// Beide Male ist die Frage entschieden und die Antwort sagt etwas ueber die
/// Datei. Die Schreibung spielt dabei so wenig eine Rolle wie beim Namen, denn
/// der Vergleich ist derselbe.
#[test]
fn ein_text_mit_der_folge_traegt_sie_und_einer_ohne_nicht() {
    let ordner = Pruefordner::neu("inhalt-folge");

    let mit = ordner.datei("mit.txt", b"erste Zeile\nzweite mit gesuchtem Wort\n");
    assert_eq!(
        traegt_der_inhalt(&mit, "gesuchtem", 1024),
        Inhaltsbefund::Traegt
    );

    let ohne = ordner.datei("ohne.txt", b"erste Zeile\nzweite Zeile\n");
    assert_eq!(
        traegt_der_inhalt(&ohne, "gesuchtem", 1024),
        Inhaltsbefund::TraegtNicht
    );

    let gross = ordner.datei("gross.txt", b"GESUCHTES WORT\n");
    assert_eq!(
        traegt_der_inhalt(&gross, "gesuchtes", 1024),
        Inhaltsbefund::Traegt,
        "die Schreibung des Textes zaehlt so wenig wie die des Namens"
    );
}

/// C1.6: Eine Datei ohne gueltiges UTF-8 ist kein Text und traegt nichts —
/// auch dann nicht, wenn die gesuchten Bytes in ihr stehen.
///
/// Die Folge steht hier als reines ASCII in der Datei, und eine Suche ueber die
/// **Bytes** faende sie. Gefragt ist aber, ob der **Text** sie traegt, und diese
/// Datei hat keinen. Genau darum liest der Weg die Datei ganz und nicht
/// streifenweise: die Typfrage ist erst am Ende beantwortet.
#[test]
fn eine_datei_ohne_gueltiges_utf8_traegt_nichts() {
    let ordner = Pruefordner::neu("inhalt-kein-text");
    let binaer = ordner.datei("binaer.bin", b"gesuchtes\xff\xfe\x00Wort");

    assert_eq!(
        traegt_der_inhalt(&binaer, "gesuchtes", 1024),
        Inhaltsbefund::TraegtNicht
    );
}

/// Eine Datei ueber der Grenze bleibt ungelesen, und `ZuGross` ist kein
/// `TraegtNicht`.
///
/// Sie traegt die Folge; die Antwort ist trotzdem nicht `Traegt`, weil gar nicht
/// gelesen wurde. Und sie ist auch nicht `TraegtNicht`, denn ueber die Datei ist
/// nichts entschieden — die Zahl der ungelesenen wandert spaeter in die
/// Statuszeile und nicht an die Zeile.
#[test]
fn eine_datei_ueber_der_grenze_bleibt_ungelesen() {
    let ordner = Pruefordner::neu("inhalt-zu-gross");
    let gross = ordner.datei("gross.txt", b"gesuchtes Wort\n");

    assert_eq!(
        traegt_der_inhalt(&gross, "gesuchtes", 8),
        Inhaltsbefund::ZuGross
    );
    assert_eq!(
        traegt_der_inhalt(&gross, "gesuchtes", 15),
        Inhaltsbefund::Traegt,
        "genau auf der Grenze wird gelesen"
    );
}

/// C1.5: Die Folge steht in den letzten Bytes vor der Grenze und wird gefunden.
///
/// Gelesen wird die ganze Datei und nicht ihr Anfang; ein Weg, der nach einem
/// ersten Streifen aufhoerte, faende hier nichts. Die Datei liegt genau auf der
/// Grenze, damit die Probe zugleich sagt, dass die Grenze selbst nichts
/// abschneidet.
#[test]
fn die_folge_in_den_letzten_bytes_vor_der_grenze_wird_gefunden() {
    let ordner = Pruefordner::neu("inhalt-am-ende");
    let mut inhalt = vec![b'a'; 4096 - 9];
    inhalt.extend_from_slice(b"gesuchtes");
    let grenze = inhalt.len() as u64;
    assert_eq!(grenze, 4096, "die Datei liegt genau auf der Grenze");
    let lang = ordner.datei("lang.txt", &inhalt);

    assert_eq!(
        traegt_der_inhalt(&lang, "gesuchtes", grenze),
        Inhaltsbefund::Traegt
    );
}

/// Was keine gewoehnliche Datei ist, traegt nichts — und die Frage danach
/// haengt nicht.
///
/// Eine benannte Roehre ohne Schreiber liesse ein gewoehnliches `open(2)`
/// warten. Der Weg dorthin geht ueber `ohne_warten_oeffnen`, also kommt die
/// Antwort; die Zeitschranke ist der Unterschied zwischen dieser Zusage und
/// einer Behauptung. Ein Ordner steht daneben, weil er dieselbe Antwort ueber
/// einen anderen Zweig bekommt: das `fstat` am Deskriptor.
#[test]
fn was_keine_gewoehnliche_datei_ist_traegt_nichts() {
    let ordner = Pruefordner::neu("inhalt-keine-datei");

    let roehre = ordner.roehre("roehre");
    assert_eq!(
        inhalt_mit_zeitschranke(&roehre, "gesuchtes", 1024, Duration::from_secs(5)),
        Inhaltsbefund::TraegtNicht
    );

    let unterordner = ordner.ordner("unterordner");
    assert_eq!(
        traegt_der_inhalt(&unterordner, "gesuchtes", 1024),
        Inhaltsbefund::TraegtNicht
    );
}

/// Eine Datei ohne Leserecht traegt nichts und ist ausdruecklich nicht
/// `Unentschieden`.
///
/// `EACCES` sagt etwas ueber **diese** Datei, `EMFILE` und `ENFILE` sagen etwas
/// ueber den Prozess. Nur die zweite Lage laesst den Auftrag offen, und diese
/// Probe zieht die Grenze von der einen Seite; die andere haengt an C3.6 und
/// steht beim Durchlauf.
#[test]
fn eine_datei_ohne_leserecht_traegt_nichts() {
    let ordner = Pruefordner::neu("inhalt-gesperrt");
    let gesperrt = ordner.datei("verschlossen.txt", b"gesuchtes Wort\n");
    fs::set_permissions(&gesperrt, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht setzen");
    if fs::read(&gesperrt).is_ok() {
        // Unter root liest sich auch eine gesperrte Datei. Dann sagt die Probe
        // nichts aus, und eine Probe, die nichts aussagt, behauptet hier auch
        // nichts.
        eprintln!("uebersprungen: die Rechtesperre wirkt auf dieser Kennung nicht");
        return;
    }

    assert_eq!(
        traegt_der_inhalt(&gesperrt, "gesuchtes", 1024),
        Inhaltsbefund::TraegtNicht
    );
}

/// C6.9: Dieselbe Folge gibt am Namen und am Inhalt dieselbe Antwort.
///
/// **Gemessen wird ueber beide Wege und nicht ueber die eine Regel, die sie
/// sich teilen.** Links steht [`Ordnermodell::name_traegt_den_filter`], rechts
/// der Inhaltsbefund an einer Datei, deren ganzer Inhalt derselbe Text ist. Eine
/// Probe gegen die geteilte Regel selbst sagte nur, dass sie sich verhaelt wie
/// sie selbst; diese hier faellt, sobald einer der beiden Wege eine zweite
/// Fassung des Vergleichs bekommt.
///
/// Die Reihe deckt die drei Eigenschaften des Vergleichs ab: die Folge zaehlt an
/// jeder Stelle, die Schreibung spielt keine Rolle, und gefaltet wird nichts.
#[test]
fn der_name_und_der_inhalt_geben_dieselbe_antwort() {
    let ordner = Pruefordner::neu("inhalt-neben-namen");
    let gegenstaende = ["Banane", "Äpfel", "LIESMICH", "Cafe", "Café", "bbbaaaccc"];
    let folgen = [
        "nan", "äpfel", "apfel", "liesmich", "café", "cafe", "aaa", "xyz",
    ];

    for (nummer, gegenstand) in gegenstaende.iter().enumerate() {
        let datei = ordner.datei(&format!("stueck-{nummer}.txt"), gegenstand.as_bytes());
        for folge in folgen {
            let mut modell = handmodell([handeintrag(gegenstand, Typ::Datei)]);
            modell.filtertext_setzen(folge);
            let am_namen = modell.name_traegt_den_filter(0);
            let am_inhalt =
                traegt_der_inhalt(&datei, modell.filter_klein(), 1024) == Inhaltsbefund::Traegt;

            assert_eq!(
                am_namen, am_inhalt,
                "{gegenstand:?} gegen {folge:?}: der Name sagt {am_namen}, der Inhalt {am_inhalt}"
            );
        }
    }
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

/// Ein Bestand fuer den Durchlauf, in dem die genannten Namen an den genannten
/// Stellen stehen.
///
/// Seit dem 260816 traegt ein [`Auftrag`] nur noch seinen Index, und der
/// Durchlauf schlaegt den Namen im Bestand des Ordnermodells nach; eine Probe,
/// die einen Auftrag von Hand baut, reicht ihm deshalb einen Bestand mit. Die
/// Stellen davor tragen Fuellnamen, die kein Auftrag nennt — sie halten den
/// Index dort, wo die Probe ihn haben will.
fn bestand_aus(stellen: &[(u32, &str)]) -> Arc<Vec<Eintrag>> {
    let laenge = stellen
        .iter()
        .map(|(index, _)| *index as usize + 1)
        .max()
        .unwrap_or_default();
    let mut eintraege: Vec<Eintrag> = (0..laenge)
        .map(|nummer| handeintrag(&format!("fuellstelle-{nummer}"), Typ::Datei))
        .collect();
    for (index, name) in stellen {
        eintraege[*index as usize] = handeintrag(name, Typ::Datei);
    }
    Arc::new(eintraege)
}

/// Startet einen Durchlauf ueber einen einzigen Auftrag und wartet ihn ab.
///
/// Liefert die Befunde **und** den Stand des Zaehlers der ungelesenen Dateien.
/// Beides erst nach dem Einsammeln: der Kanal schliesst, wenn der Arbeitsfaden
/// geendet hat, und danach steht der Zaehler still.
fn einen_auftrag_entscheiden(
    wurzel: &Path,
    name: &str,
    art: Auftragsart,
    filter_klein: &str,
    inhaltsgrenze: Option<u64>,
) -> (Vec<Befundmeldung>, u64) {
    let auftraege = vec![Auftrag { index: 7, art }];
    let durchlauf = Durchlauf::starten(
        bestand_aus(&[(7, name)]),
        auftraege,
        wurzel.to_path_buf(),
        filter_klein.to_owned(),
        inhaltsgrenze,
        1,
    );
    let befunde = befunde_einsammeln(&durchlauf);
    (befunde, durchlauf.zu_gross())
}

/// Ein einzelner Unterbaumauftrag ohne Inhaltsfilter — der Durchlauf, wie er
/// vor der Runde 11 war.
fn einen_ordner_entscheiden(wurzel: &Path, name: &str, filter_klein: &str) -> Vec<Befundmeldung> {
    einen_auftrag_entscheiden(wurzel, name, Auftragsart::Unterbaum, filter_klein, None).0
}

/// Ein einzelner Unterbaumauftrag **mit** Inhaltsfilter.
fn einen_ordner_mit_inhalt_entscheiden(
    wurzel: &Path,
    name: &str,
    filter_klein: &str,
    grenze: u64,
) -> Vec<Befundmeldung> {
    einen_auftrag_entscheiden(
        wurzel,
        name,
        Auftragsart::Unterbaum,
        filter_klein,
        Some(grenze),
    )
    .0
}

/// Ein einzelner Inhaltsauftrag ueber eine gewoehnliche Datei.
fn eine_datei_entscheiden(
    wurzel: &Path,
    name: &str,
    filter_klein: &str,
    grenze: u64,
) -> Vec<Befundmeldung> {
    einen_auftrag_entscheiden(
        wurzel,
        name,
        Auftragsart::Inhalt,
        filter_klein,
        Some(grenze),
    )
    .0
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
            art: Auftragsart::Unterbaum,
        }]
    };
    let bestand = || bestand_aus(&[(7, "flach")]);

    // Kontrollauf: ohne Abbruch entscheidet derselbe Ordner.
    let ungestoert = Durchlauf::starten(
        bestand(),
        auftrag(),
        ordner.pfad().to_path_buf(),
        "gibt-es-hier-nicht".to_owned(),
        None,
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
        bestand(),
        auftrag(),
        ordner.pfad().to_path_buf(),
        "gibt-es-hier-nicht".to_owned(),
        None,
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
    let stellen: Vec<(u32, &str)> = auftraege
        .iter()
        .enumerate()
        .map(|(stelle, (name, _))| (stelle as u32, *name))
        .collect();
    let durchlauf = Durchlauf::starten(
        bestand_aus(&stellen),
        auftraege
            .iter()
            .enumerate()
            .map(|(stelle, _)| Auftrag {
                index: stelle as u32,
                art: Auftragsart::Unterbaum,
            })
            .collect(),
        ordner.pfad().to_path_buf(),
        "gesuchtes".to_owned(),
        None,
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

// ---------------------------------------------------------------------------
// Der Inhalt als zweite Auftragsart (C3, C4)
// ---------------------------------------------------------------------------

/// Die Grenze, unter der die Inhaltsproben lesen.
///
/// Klein genug, dass eine Datei von wenigen Kilobyte sicher darueber liegt, und
/// gross genug fuer jeden Text, den diese Proben schreiben. Die 1 MB der
/// Oberflaeche wohnen in `krk-ui` und haben im Kern nichts zu suchen; hier
/// steht die Zahl, die diese Proben brauchen, und keine zweite Fassung von
/// jener.
const PROBENGRENZE: u64 = 1_024;

/// C4.1 und C4.2: Ein flacher Inhaltsauftrag entscheidet die Datei an ihrem
/// Text.
///
/// Beide Namen tragen die Folge nicht — sonst waere die Datei ohne diesen
/// Auftrag entschieden und die Probe maesse den Kurzschluss statt des Lesens.
#[test]
fn ein_flacher_inhaltsauftrag_liest_die_datei_und_entscheidet_sie() {
    let ordner = Pruefordner::neu("inhalt-flach");
    ordner.datei("mit.txt", b"oben steht gesuchtes und darunter nichts");
    ordner.datei("ohne.txt", b"hier steht nichts davon");

    let (befunde, ungelesen) = einen_auftrag_entscheiden(
        ordner.pfad(),
        "mit.txt",
        Auftragsart::Inhalt,
        "gesuchtes",
        Some(PROBENGRENZE),
    );
    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "der Text traegt die Folge, obwohl der Name sie nicht traegt"
    );
    assert_eq!(
        ungelesen, 0,
        "eine gelesene Datei ist keine ungelesene, und der Zaehler sagt das"
    );

    assert_eq!(
        eine_datei_entscheiden(ordner.pfad(), "ohne.txt", "gesuchtes", PROBENGRENZE),
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "gelesen und nichts gefunden ist ein Befund und kein Schweigen"
    );
}

/// C3.1: Ein Treffer, der allein im **Text** einer Datei tief unten liegt,
/// entscheidet den Ordner.
///
/// Derselbe Baum, zwei Laeufe: ohne Grenze bleibt er unentschieden — kein Name
/// traegt die Folge —, mit Grenze faellt der Treffer. Der Unterschied ist genau
/// die Zeile, um die es geht.
#[test]
fn ein_treffer_allein_im_text_entscheidet_den_unterbaum() {
    let ordner = Pruefordner::neu("inhalt-unterbaum");
    let tief = ordner.unter("aussen").join("a").join("b");
    fs::create_dir_all(&tief).expect("Kette laesst sich nicht anlegen");
    fs::write(tief.join("notiz.txt"), b"ganz unten steht gesuchtes").expect("Blatt");
    fs::write(ordner.unter("aussen").join("liesmich.md"), b"nichts davon").expect("Beiwerk");

    assert_eq!(
        einen_ordner_entscheiden(ordner.pfad(), "aussen", "gesuchtes"),
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "ohne Grenze wird keine Datei geoeffnet, und kein Name traegt die Folge"
    );
    assert_eq!(
        einen_ordner_mit_inhalt_entscheiden(ordner.pfad(), "aussen", "gesuchtes", PROBENGRENZE),
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "mit Grenze entscheidet der Text drei Ebenen tiefer den Ordner (C3.1)"
    );
}

/// C3.4: Der Kurzschluss des Namens spart im Unterbaum auch das **Lesen**.
///
/// Die eine Datei traegt die Folge im Namen und ist nicht lesbar. Wer sie
/// laese, bekaeme `TraegtNicht` und schriebe den Ordner ab; wer den Namen
/// zuerst fragt, entscheidet ihn ohne einen einzigen Lesevorgang. `treffer:
/// true` ist damit der Beleg, dass nicht gelesen wurde — an einem Zaehler ist
/// er nicht abzulesen, denn ein gescheitertes Lesen zaehlt nirgends mit.
#[test]
fn ein_namenstreffer_im_unterbaum_bleibt_ungelesen() {
    let ordner = Pruefordner::neu("inhalt-kurzschluss");
    let aussen = ordner.ordner("aussen");
    let blatt = aussen.join("gesuchtes-blatt.txt");
    fs::write(&blatt, b"").expect("Blatt laesst sich nicht schreiben");
    fs::set_permissions(&blatt, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht entziehen");

    assert_eq!(
        einen_ordner_mit_inhalt_entscheiden(ordner.pfad(), "aussen", "gesuchtes", PROBENGRENZE),
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "der Name entscheidet vor dem Lesen; eine unlesbare Datei mit passendem Namen \
         zeigt, dass gar nicht erst geoeffnet wurde (C3.4)"
    );
}

/// C3.7: In eine symbolische Verknuepfung wird weder abgestiegen noch
/// hineingelesen.
///
/// Die Verknuepfung im Unterbaum zeigt auf eine Datei, deren Text die Folge
/// traegt und die **ausserhalb** des Unterbaums liegt. Wuerde in sie
/// hineingelesen, faende der Durchlauf den Treffer; er traegt nichts bei, und
/// das ist genau die Regel, die der Durchlauf schon fuer Ordner hat.
#[test]
fn eine_verknuepfung_im_unterbaum_wird_nicht_gelesen() {
    let ordner = Pruefordner::neu("inhalt-verknuepfung");
    let ziel = ordner.datei("ziel.txt", b"hier steht gesuchtes");
    let aussen = ordner.ordner("aussen");
    std::os::unix::fs::symlink(&ziel, aussen.join("verweis.txt"))
        .expect("Verknuepfung laesst sich nicht anlegen");

    assert_eq!(
        einen_ordner_mit_inhalt_entscheiden(ordner.pfad(), "aussen", "gesuchtes", PROBENGRENZE),
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "eine Verknuepfung traegt zum Befund nichts bei, auch nicht ueber ihren Inhalt (C3.7)"
    );
    // Gegenprobe: ueber ihren echten Ort ist derselbe Text sehr wohl ein
    // Treffer. Ohne sie hiesse `treffer: false` vielleicht nur, dass der Text
    // gar nicht dasteht.
    assert_eq!(
        eine_datei_entscheiden(ordner.pfad(), "ziel.txt", "gesuchtes", PROBENGRENZE),
        vec![Befundmeldung {
            index: 7,
            treffer: true
        }],
        "derselbe Text ueber seinen echten Ort"
    );
}

/// Ein Lauf mit `inhaltsgrenze: None` verhaelt sich wie der Durchlauf vor
/// dieser Runde.
#[test]
fn ohne_grenze_wird_keine_einzige_datei_geoeffnet() {
    let ordner = Pruefordner::neu("inhalt-ohne-grenze");
    let aussen = ordner.ordner("aussen");
    fs::write(aussen.join("notiz.txt"), b"hier steht gesuchtes").expect("Datei");
    ordner.datei("flach.txt", b"hier steht gesuchtes");

    let (befunde, ungelesen) = einen_auftrag_entscheiden(
        ordner.pfad(),
        "aussen",
        Auftragsart::Unterbaum,
        "gesuchtes",
        None,
    );
    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "ohne Grenze zaehlt allein der Name, und keiner traegt die Folge"
    );
    assert_eq!(
        ungelesen, 0,
        "ohne Grenze wird nichts gelesen und nichts gezaehlt"
    );

    // Ein Inhaltsauftrag ohne Grenze ist von diesem Lauf nicht zu beantworten,
    // und ungelesen heisst unentschieden. Die Paarung entsteht in KRK nicht —
    // Auftragsart und Grenze kommen aus derselben Frage —, und diese Erwartung
    // haelt fest, dass sie nicht still negativ entschieden wird.
    assert_eq!(
        einen_auftrag_entscheiden(
            ordner.pfad(),
            "flach.txt",
            Auftragsart::Inhalt,
            "gesuchtes",
            None,
        )
        .0,
        Vec::new(),
        "ein Inhaltsauftrag ohne Grenze bleibt unentschieden, statt als Nichttreffer zu gelten"
    );
}

/// C4.6: Eine Datei ueber der Grenze bleibt ungelesen, traegt nichts bei und
/// **zaehlt**.
///
/// Der Zaehler ist die eine Stelle, an der eine ungelesene Datei sichtbar
/// bleibt; ihre Zeile sagt darueber nichts, und das ist der Grund, aus dem
/// `Befund` keine vierte Variante bekommen hat.
#[test]
fn eine_zu_grosse_datei_bleibt_ungelesen_und_zaehlt() {
    let ordner = Pruefordner::neu("inhalt-zu-gross");
    ordner.datei("gross.txt", vec![b'x'; 4 * 1024]);

    let (befunde, ungelesen) = einen_auftrag_entscheiden(
        ordner.pfad(),
        "gross.txt",
        Auftragsart::Inhalt,
        "gesuchtes",
        Some(64),
    );
    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "eine ungelesene Datei steht nicht in der Liste"
    );
    assert_eq!(
        ungelesen, 1,
        "und sie zaehlt in den Satzteil der Statuszeile"
    );

    // Dieselbe Datei im Unterbaum: sie zaehlt auch dort, und der Ordner ist
    // entschieden, ohne dass sie angesehen wurde.
    let aussen = ordner.ordner("aussen");
    fs::write(aussen.join("gross.txt"), vec![b'x'; 4 * 1024]).expect("Datei");
    let (befunde, ungelesen) = einen_auftrag_entscheiden(
        ordner.pfad(),
        "aussen",
        Auftragsart::Unterbaum,
        "gesuchtes",
        Some(64),
    );
    assert_eq!(
        befunde,
        vec![Befundmeldung {
            index: 7,
            treffer: false
        }],
        "ein Unterbaum, in dem nur Ungelesenes liegt, ist entschieden und nicht offen"
    );
    assert_eq!(
        ungelesen, 1,
        "auch tief unten zaehlt eine ungelesene Datei mit"
    );
}

/// C4.7, soweit an einer Probe abzulesen: die Abbruchgrenze steht an **zwei**
/// Stellen im Unterbaum und an einer im flachen Zweig.
///
/// **Was diese Probe nicht entscheidet, und der Satz gehoert dazu:** die Spanne
/// zwischen dem Setzen des Kennzeichens und dem Ende des Fadens. Sie zu messen
/// brauchte eine Uhr, und in diesem Weg steht keine — derselbe Vorbehalt, den
/// `der_abbruch_greift_in_einem_ordner_ohne_unterordner` schon festhaelt.
/// Abzulesen ist stattdessen, **wo** geprueft wird, und das ist die Aussage der
/// Regel: vor jeder Einheit, die dauern kann, und das sind seit dieser Runde
/// zwei.
///
/// Gezaehlt wird ueber die Code-Zeilen ohne Kommentare, damit der Modulkopf,
/// der die Regel im Klartext nennt, nicht mitzaehlt.
#[test]
fn die_abbruchgrenze_steht_vor_jedem_stapel_und_vor_jeder_datei() {
    let quelle = quelltext_von("krk-core/src/verzeichnis/durchlauf.rs");
    let code = code_zeilen(&quelle).join("\n");
    let zaehlen = |stueck: &str| stueck.matches("abbruch.load(").count();

    let (vor_dem_unterbaum, rest) = code
        .split_once("fn unterbaum_entscheiden")
        .expect("der Durchlauf traegt die Funktion `unterbaum_entscheiden`");
    let (im_unterbaum, nach_dem_unterbaum) = rest
        .split_once("\nfn ")
        .expect("nach `unterbaum_entscheiden` folgt eine weitere Funktion");

    assert_eq!(
        zaehlen(im_unterbaum),
        2,
        "`unterbaum_entscheiden` traegt genau zwei Abbruchgrenzen: vor dem naechsten \
         Stapel und vor der naechsten gelesenen Datei"
    );
    assert_eq!(
        zaehlen(vor_dem_unterbaum),
        1,
        "der flache Zweig traegt genau eine, vor dem Lesen der Datei"
    );
    assert_eq!(
        zaehlen(nach_dem_unterbaum),
        0,
        "hinter `unterbaum_entscheiden` steht keine weitere Abbruchgrenze"
    );
}

/// C3.6: Ein Deskriptormangel **beim Lesen einer Datei** laesst sie
/// unentschieden.
///
/// Dieselbe Regel wie beim Oeffnen eines Ordners, und derselbe Grund: `EMFILE`
/// und `ENFILE` sind ein Zustand des Prozesses und kein Befund ueber die Datei.
/// Der Faden endet ohne Meldung, statt die Zeile still und dauerhaft aus der
/// Liste zu nehmen; die naechste Frage — ein weiteres Zeichen, ein Umschalten,
/// ein Ordnerwechsel — stellt sie neu.
///
/// **Gemessen wird im Kindprozess unter `ulimit -n 64`.** `cargo test` erbt die
/// angehobene Grenze der Anmeldesitzung; ohne den Kindprozess behauptete die
/// Zusage sich selbst. Die Form steht seit der Runde 10 in dieser Datei, und
/// diese Probe schreibt sie ab.
///
/// Angelegt und abgeraeumt wird der Baum vom **Elternteil**, aus demselben
/// Grund wie bei den beiden Proben darueber.
#[test]
fn ein_deskriptormangel_beim_lesen_laesst_die_datei_unentschieden() {
    let ordner = Pruefordner::neu("inhalt-mangel");
    let ziel = ordner.datei("ziel.txt", b"hier steht gesuchtes");
    ordner.datei("zweiter.txt", b"auch hier steht gesuchtes");
    // Der erste Auftrag ist eine Verknuepfung und damit ohne ein einziges
    // Oeffnen entschieden (C3.7). Nur so ist zu sehen, dass der Mangel den
    // Durchlauf **ab** dem ersten Oeffnen anhaelt und nicht schon davor.
    ordner.verknuepfung("verweis", &ziel);

    let ergebnis = kind_mit_deskriptorgrenze(
        DESKRIPTORGRENZE,
        "kind_meldet_bei_deskriptormangel_ueber_einer_datei_nichts",
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "ein Deskriptormangel des Prozesses wird zu einer Aussage ueber eine Datei\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_meldet_bei_deskriptormangel_ueber_einer_datei_nichts() {
    let Some(ordner) = kindauftrag() else {
        return;
    };
    let bestand = || bestand_aus(&[(5, "verweis"), (7, "ziel.txt"), (8, "zweiter.txt")]);
    let auftraege = || {
        vec![
            Auftrag {
                index: 5,
                art: Auftragsart::Unterbaum,
            },
            Auftrag {
                index: 7,
                art: Auftragsart::Inhalt,
            },
            Auftrag {
                index: 8,
                art: Auftragsart::Inhalt,
            },
        ]
    };

    // Erster Durchgang, mit freiem Vorrat, und er ist die Gegenprobe: ohne ihn
    // saehe der zweite auch dann so aus, wenn die Dateien gar nicht stuenden
    // oder der Filtertext nirgends traefe.
    let mit_vorrat = befunde_einsammeln(&Durchlauf::starten(
        bestand(),
        auftraege(),
        ordner.clone(),
        "gesuchtes".to_owned(),
        Some(PROBENGRENZE),
        1,
    ));

    // Jetzt den Mangel herstellen: nehmen, bis keiner mehr kommt, und halten.
    let mut gehalten = Vec::new();
    let mut abweisung = None;
    while gehalten.len() < 4 * DESKRIPTORSCHRANKE {
        match fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(fehler) => {
                abweisung = Some(fehler);
                break;
            }
        }
    }
    let vorrat = gehalten.len();

    // Der Durchlauf laeuft, waehrend `gehalten` steht: sein erstes Oeffnen
    // einer Datei trifft auf eine volle Deskriptortabelle.
    let ohne_vorrat = befunde_einsammeln(&Durchlauf::starten(
        bestand(),
        auftraege(),
        ordner.clone(),
        "gesuchtes".to_owned(),
        Some(PROBENGRENZE),
        2,
    ));

    // Erst zurueckgeben, dann pruefen: eine gescheiterte Behauptung soll ihre
    // Meldung noch schreiben koennen.
    drop(gehalten);

    let abweisung = abweisung.expect(
        "der Vorrat an Deskriptoren ist nicht ausgegangen; die Grenze des Kindes ist nicht \
         abgesenkt, und die Probe wuerde nichts messen",
    );
    assert!(
        ist_deskriptormangel(&abweisung),
        "das Oeffnen ist nicht am Vorrat gescheitert, sondern an etwas anderem: {abweisung}"
    );
    assert!(
        vorrat < DESKRIPTORSCHRANKE,
        "die Deskriptorgrenze des Kindes ist nicht abgesenkt: {vorrat} Deskriptoren zugleich frei"
    );

    assert_eq!(
        mit_vorrat,
        vec![
            Befundmeldung {
                index: 5,
                treffer: false
            },
            Befundmeldung {
                index: 7,
                treffer: true
            },
            Befundmeldung {
                index: 8,
                treffer: true
            },
        ],
        "die Gegenprobe mit freiem Vorrat entscheidet die drei Auftraege nicht; der zweite \
         Durchgang maesse dann etwas anderes als den Mangel"
    );

    // Die Verknuepfung ist entschieden, weil sie kein Oeffnen braucht. Die
    // Datei danach ist es nicht, und der dritte Auftrag bleibt es ebenfalls.
    assert_eq!(
        ohne_vorrat,
        vec![Befundmeldung {
            index: 5,
            treffer: false
        }],
        "unter einer vollen Deskriptortabelle wird eine Datei entschieden, statt sie und die \
         Auftraege nach ihr unentschieden zu lassen (C3.6)"
    );
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

/// Wie tief die Kette der Deskriptorprobe ist.
///
/// Deutlich mehr als die 64 Deskriptoren, unter denen das Kind laeuft, und
/// deutlich weniger als `PATH_MAX / 2`: bei zwei Bytes je Ebene und einem
/// Temporaerpfad von rund 60 Bytes bleibt die tiefste Stelle unter 500 Bytes.
const KETTENTIEFE: usize = 200;

/// Wie viele Deskriptoren das Kind hoechstens haben darf, damit die Probe misst
/// und nicht behauptet.
const DESKRIPTORSCHRANKE: usize = 100;

/// Die Grenze, unter der die Kindproben dieser Datei laufen.
///
/// 64, weil `launchctl limit maxfiles` 256 als Voreinstellung fuehrt und ein aus
/// dem Finder gestartetes Buendel ungefaehr in dieser Groessenordnung liegt. Die
/// Zahl stand bis zum 260817 im Rumpf des Starters; seit der in
/// `tests/gemeinsam/` steht und `tests/umfang.rs` eine tiefere braucht, reist sie
/// als Argument.
const DESKRIPTORGRENZE: usize = 64;

/// C3.8 und C3.15 **unter der Deskriptorgrenze, die ein Buendel bekommt**.
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

    let ergebnis = kind_mit_deskriptorgrenze(
        DESKRIPTORGRENZE,
        "kind_entscheidet_die_tiefe_kette",
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

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_entscheidet_die_tiefe_kette() {
    let Some(ordner) = kindauftrag() else {
        return;
    };

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

/// C3.15 in der **Vorwaertsrichtung**: ein von aussen herbeigefuehrter Mangel
/// fuehrt zu keinem Befund.
///
/// `die_tiefe_kette_wird_auch_mit_vierundsechzig_deskriptoren_entschieden`
/// darueber misst die Rueckrichtung, naemlich dass der Durchlauf keinen eigenen
/// Mangel erzeugt. Erst beide zusammen decken C3.15 ab: ohne diese Probe hier
/// waere der Zweig `Err(fehler) if ist_deskriptormangel(&fehler) => return
/// None` in [`krk_core::verzeichnis::durchlauf`] von keiner Pruefung erreicht.
///
/// **Der Mangel wird hergestellt und nicht abgewartet.** Das Kind nimmt unter
/// `ulimit -n 64` Deskriptoren, bis keiner mehr kommt, und **haelt sie**,
/// waehrend der Durchlauf laeuft. Dessen erstes `File::open` kann dann nur noch
/// `EMFILE` liefern; der Mangel schlaegt also gleich beim ersten Oeffnen zu und
/// nicht irgendwo in der Tiefe. Darin liegt der Unterschied zur Probe darueber:
/// dort ist der Baum tief und der Durchlauf soll trotzdem durchkommen, hier ist
/// er flach und der Durchlauf soll gar nicht erst anfangen.
///
/// **Gemessen ist dabei auch der zweite Halbsatz von C3.15**, dass die noch
/// offenen Auftraege ebenfalls unentschieden bleiben. Der erste Auftrag ist
/// eine symbolische Verknuepfung und damit ohne Oeffnen entschieden (C3.9);
/// von den beiden Ordnern danach kommt kein Befund mehr, obwohl der Kanal
/// danach schliesst und nicht abgebrochen wurde.
///
/// Angelegt und abgeraeumt wird der Baum vom **Elternteil**, aus demselben
/// Grund wie bei der Probe darueber.
#[test]
fn ein_deskriptormangel_von_aussen_laesst_die_ordner_unentschieden() {
    let ordner = Pruefordner::neu("durchlauf-mangel");
    for name in ["aussen", "zweiter"] {
        let unterordner = ordner.unter(name).join("a");
        fs::create_dir_all(&unterordner).expect("Unterordner laesst sich nicht anlegen");
        fs::write(unterordner.join("gesuchtes-blatt.txt"), b"x").expect("Blatt");
    }
    // Der erste Auftrag ist eine Verknuepfung, und der ist ohne ein einziges
    // Oeffnen entschieden (C3.9). Nur so ist zu sehen, dass der Mangel den
    // Durchlauf **ab** dem ersten Oeffnen anhaelt und nicht schon davor.
    ordner.verknuepfung("verweis", ordner.unter("aussen"));

    let ergebnis = kind_mit_deskriptorgrenze(
        DESKRIPTORGRENZE,
        "kind_meldet_bei_deskriptormangel_nichts",
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "ein Deskriptormangel des Prozesses wird zu einer Aussage ueber einen Ordner\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_meldet_bei_deskriptormangel_nichts() {
    let Some(ordner) = kindauftrag() else {
        return;
    };
    let auftraege = || {
        vec![
            Auftrag {
                index: 5,
                art: Auftragsart::Unterbaum,
            },
            Auftrag {
                index: 7,
                art: Auftragsart::Unterbaum,
            },
            Auftrag {
                index: 8,
                art: Auftragsart::Unterbaum,
            },
        ]
    };
    let bestand = || bestand_aus(&[(5, "verweis"), (7, "aussen"), (8, "zweiter")]);

    // Erster Durchgang, mit freiem Vorrat, und er ist die Gegenprobe: ohne ihn
    // saehe der zweite auch dann so aus, wenn der Baum gar nicht stuende oder
    // der Filtertext nirgends traefe. Die Probe sagte dann mehr zu, als sie
    // haelt.
    let mit_vorrat = befunde_einsammeln(&Durchlauf::starten(
        bestand(),
        auftraege(),
        ordner.clone(),
        "gesuchtes".to_owned(),
        None,
        1,
    ));

    // Jetzt den Mangel herstellen: nehmen, bis keiner mehr kommt, und halten.
    // Die Schranke daneben faengt den Fall ab, dass `ulimit` nicht gegriffen
    // hat; dann bleibt `abweisung` leer und die Probe sagt es.
    let mut gehalten = Vec::new();
    let mut abweisung = None;
    while gehalten.len() < 4 * DESKRIPTORSCHRANKE {
        match fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(fehler) => {
                abweisung = Some(fehler);
                break;
            }
        }
    }
    let vorrat = gehalten.len();

    // Der Durchlauf laeuft, waehrend `gehalten` steht: sein erstes Oeffnen
    // trifft auf eine volle Deskriptortabelle. Ein leerer Kanal kann hier nur
    // zwei Ursachen haben, und die zweite ist ausgeschlossen — der `Durchlauf`
    // lebt bis zum Ende des Einsammelns, also hat niemand abgebrochen.
    let ohne_vorrat = befunde_einsammeln(&Durchlauf::starten(
        bestand(),
        auftraege(),
        ordner.clone(),
        "gesuchtes".to_owned(),
        None,
        2,
    ));

    // Erst zurueckgeben, dann pruefen: eine gescheiterte Behauptung soll ihre
    // Meldung noch schreiben koennen.
    drop(gehalten);

    let abweisung = abweisung.expect(
        "der Vorrat an Deskriptoren ist nicht ausgegangen; die Grenze des Kindes ist nicht \
         abgesenkt, und die Probe wuerde nichts messen",
    );
    assert!(
        ist_deskriptormangel(&abweisung),
        "das Oeffnen ist nicht am Vorrat gescheitert, sondern an etwas anderem: {abweisung}"
    );
    assert!(
        vorrat < DESKRIPTORSCHRANKE,
        "die Deskriptorgrenze des Kindes ist nicht abgesenkt: {vorrat} Deskriptoren zugleich frei"
    );

    assert_eq!(
        mit_vorrat,
        vec![
            Befundmeldung {
                index: 5,
                treffer: false
            },
            Befundmeldung {
                index: 7,
                treffer: true
            },
            Befundmeldung {
                index: 8,
                treffer: true
            },
        ],
        "die Gegenprobe mit freiem Vorrat entscheidet die drei Auftraege nicht; der zweite \
         Durchgang maesse dann etwas anderes als den Mangel"
    );

    // Die Verknuepfung ist entschieden, weil sie kein Oeffnen braucht. Der
    // Ordner danach ist es nicht, und der dritte Auftrag bleibt es ebenfalls:
    // beides sagt C3.15 zu, und beides steht in dieser einen Erwartung.
    assert_eq!(
        ohne_vorrat,
        vec![Befundmeldung {
            index: 5,
            treffer: false
        }],
        "unter einer vollen Deskriptortabelle wird ein Ordner entschieden, statt ihn und die \
         Auftraege nach ihm unentschieden zu lassen (C3.15)"
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
/// zuruecksetzte, waere in genau diesen sieben Dateien zu sehen: den vier
/// Modulen des Kerns, die den Filter tragen, dem Leseweg, ueber den der
/// Inhaltsfilter an die Bytes kommt, der Senke in `krk-ui`, in die das getippte
/// Zeichen laeuft, und der Tabliste. Die Sekundenregel der Sprungmarke aus C2
/// der Runde 1 stand in der ersten dieser Dateien, als sie noch `sprungmarke.rs`
/// hiess.
///
/// **Zwei Dateien sind mit der Runde 11 dazugekommen**, und beide liegen auf dem
/// Weg des Inhaltsfilters: `krk-core/src/verzeichnis/inhalt.rs` stellt die Frage
/// je Datei, `krk-core/src/text/datei.rs` holt die Bytes dafuer. Eine Wartezeit
/// oder eine Frist liesse sich in beiden ebenso gut unterbringen wie in den
/// fuenf anderen. **`krk-core/src/verzeichnis/sys.rs` tritt der Liste
/// ausdruecklich nicht bei**, obwohl der Filter darueber oeffnet: die Datei
/// fuehrt `Duration` viermal zur Umrechnung der Aenderungszeit, und die Nadel
/// kann eine Umrechnung nicht von einer Messung trennen
/// (`issues/260816-1359_*_die-probe-gegen-zeitmessung-im-filter-erreicht-zwei-dateien-des-filterwegs-nicht.md`).
///
/// **`krk-ui/src/tabs.rs` war nicht von Anfang an dabei.** Seit
/// Schritt F2 traegt sie den Filtertext ueber den Ordnerwechsel, haelt den
/// `Durchlauf` je Tab, entscheidet, wann einer beginnt und vergeht, und zieht
/// die Befunde ein; ein Zeitgeber liesse sich dort ebenso gut unterbringen wie
/// in den uebrigen. Bis zum 260815 fehlte sie in der Liste, waehrend der
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
/// Diese Datei liest zwar nur die benannten und nicht sich selbst, aber
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
        "krk-core/src/verzeichnis/inhalt.rs",
        "krk-core/src/text/datei.rs",
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
/// **Der letzte Traeger des Namens hiess `Nachschlag::Sprungmarke` und heisst
/// seit dem 260816 `Nachschlag::Tippen`.** Er behielt den Namen ueber die
/// Runde 10 hinweg, weil er weiter zutraf — „eine Taste ohne Zusatztaste, die
/// keiner Funktion gehoert" —, und verlor ihn, als der Nutzerentscheid vom
/// 260816-1105 `shift` und `opt` ebenfalls dorthin fallen liess. Die vier
/// Nadeln haetten ihn ohnehin nie gefunden, weil vor jeder ein `::` oder ein
/// `struct` steht oder sie ueberhaupt anders heisst; die Probe misst dasselbe
/// wie zuvor.
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

/// C1.4 und C6.3: Die eine Zeichenregel steht einmal und hat genau zwei
/// Aufrufer, der eine Vergleich steht einmal und hat genau drei.
///
/// Die Zeichenregel wird in `krk-core/src/verzeichnis/filter.rs` erklaert und
/// von der Senke des Tippens in der Dateiliste und von der Tippsuche der
/// Belegungsansicht aus der Runde 7 gerufen. **Gezaehlt werden Dateien und
/// nicht Aufrufe**: welche Datei fragt, ist die Aussage des Kriteriums; wie oft
/// sie innerhalb ihrer selbst fragt, ist es nicht. Der Inhaltsfilter aendert an
/// ihr nichts — welche Zeichen in den Filtertext kommen, ist dieselbe Frage
/// geblieben, und ihre Zahl bleibt deshalb bei zwei (C6.4).
///
/// Der Vergleich hat dieselbe Bauart und steht ebenfalls einmal in `filter.rs`.
/// Bis zum 260815 stand er zweimal da, einmal je Rufer; seither hat er zwei,
/// den Pruefschritt des Ordnermodells und den Durchlauf. **Mit der Runde 11
/// werden es drei**, und der dritte ist `krk-core/src/verzeichnis/inhalt.rs`.
/// Die Zahl steigt hier bewusst und nicht versehentlich: „lies eine Datei und
/// vergleiche ihren Text" ist eine andere Aufgabe als „schreite ein Verzeichnis
/// ab", also bekommt sie eine eigene Datei. In `durchlauf.rs` geschrieben
/// bliebe die Zahl bei zwei und mischte zwei Zustaendigkeiten — die Zahl waere
/// dann die falsche Auskunft und nicht die richtige.
///
/// **Die Probe behaelt ihre namentliche Liste und wird nicht durch eine blosse
/// Zahl ersetzt.** Eine Zahl sagte, dass es drei sind; die Liste sagt, welche
/// drei, und nur die zweite Auskunft faengt einen Rufer, der an die Stelle
/// eines anderen getreten ist.
#[test]
fn die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei() {
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
            "krk-core/src/verzeichnis/inhalt.rs".to_owned(),
            "krk-core/src/verzeichnis/modell.rs".to_owned(),
        ],
        "der Vergleich hat andere Rufer als der Pruefschritt, der Durchlauf und \
         der Inhaltsbefund"
    );
}

/// C6, gezaehlt: die Namensfrage des Filters wird an genau einer Stelle
/// gestellt.
///
/// `Ordnermodell::name_traegt_den_filter` ist der herausgegebene Zweig
/// `Name traegt die Folge?` des Pruefschritts. Er schreibt je Aufruf einen Namen
/// klein, kostet also je Gang ueber den Bestand so viele Zeichenketten, wie der
/// Ordner Eintraege hat — bei 100.000 Eintraegen 100.000 je Gang.
///
/// **Bis zum 260816 gab es drei Gaenge**, und zwei davon liefen ungezaehlt:
/// der Neuaufbau der Sicht, die Auftragsliste in `krk-ui` und, je gezeichneter
/// Zelle, die Frage `steht_wegen_des_inhalts`. Der Neuaufbau lief dabei auch
/// bei **jedem** eintreffenden Befund, waehrend eines Durchlaufs also bis zu
/// sechzigmal in der Sekunde, alles auf dem Hauptfaden
/// (`issues/260816-1933_*_die-auftragsliste-legt-je-tastendruck-einen-namen-je-datei-an-auf-dem-hauptfaden.md`).
/// Seither steht die Antwort im Zeilengrund, und die Frage hat einen Rufer.
///
/// **Gezaehlt werden die Dateien unter `src/`.** Die Probe zu C6.9 nennt die
/// Funktion ebenfalls und misst sie gegen den Inhaltsbefund; ein Rufer im Sinne
/// dieser Zaehlung ist sie nicht.
///
/// **Was diese Probe nicht entscheidet:** wie oft `modell.rs` innerhalb seiner
/// selbst fragt. Sie faengt den zweiten Ort, und der zweite Ort war der Befund.
#[test]
fn die_namensfrage_des_filters_hat_einen_rufer() {
    let nadel = concat!("name_traegt", "_den_filter");
    let heimat = "krk-core/src/verzeichnis/modell.rs";

    let mut rufer = Vec::new();
    for (name, inhalt) in gemeinsam::quelldateien() {
        if !name.contains("/src/") {
            continue;
        }
        if code_zeilen(&inhalt)
            .iter()
            .any(|zeile| zeile.contains(nadel))
        {
            rufer.push(name);
        }
    }

    assert_eq!(
        rufer,
        vec![heimat.to_owned()],
        "die Namensfrage wird ausserhalb des Pruefschritts gestellt"
    );
}

// ---------------------------------------------------------------------------
// Worauf eine Verknuepfung zeigt (Defekt 260814-1612, Befund 260815-1713)
// ---------------------------------------------------------------------------

/// Der Fall, den der Nutzer gemeldet hat: eine Verknuepfung auf ein
/// Verzeichnis.
///
/// Der Verzeichnisleser meldet sie als [`Typ::Verknuepfung`] und `ist_ordner`
/// antwortet fuer sie mit `false`; genau daran endete der Einstieg. Aufgeloest
/// wird sie erst hier, am Namen, und erst dann, wenn jemand hineingehen will.
#[test]
fn eine_verknuepfung_auf_ein_verzeichnis_ist_ein_ordner() {
    let ordner = Pruefordner::neu("verweisziel-ordner");
    ordner.ordner("ziel");
    let verweis = ordner.verknuepfung("verweis", ordner.unter("ziel"));

    assert_eq!(verweisziel::bestimmen(&verweis), Verweisziel::Ordner);
}

/// Eine Verknuepfung auf eine Datei bleibt eine Datei.
///
/// Damit geschieht mit ihr, was heute mit einer Datei geschieht: der Einstieg
/// findet nicht statt, und der Doppelklick gibt sie an das System.
#[test]
fn eine_verknuepfung_auf_eine_datei_ist_kein_ordner() {
    let ordner = Pruefordner::neu("verweisziel-datei");
    ordner.fuelldatei("ziel.txt", 3);
    let verweis = ordner.verknuepfung("verweis.txt", ordner.unter("ziel.txt"));

    assert_eq!(verweisziel::bestimmen(&verweis), Verweisziel::KeinOrdner);
}

/// Eine Verknuepfung auf eine Datei ohne Leserecht ist trotzdem eine Datei.
///
/// Der praktisch haeufigste Fehlfall des Befunds `260815-1713`. Am Deskriptor
/// gefragt scheiterte `open` hier mit `EACCES`, und der Doppelklick bekam
/// "laesst sich nicht oeffnen: Permission denied" in die Statuszeile, statt wie
/// jede andere Datei an das Standardprogramm zu gehen. Am Namen gefragt kommt
/// die Antwort heraus, die der Wert benennt: es ist kein Verzeichnis. Ob sich
/// das Ziel oeffnen laesst, entscheidet das Programm, das es oeffnet.
///
/// **Als `root` liefe die Probe nicht anders**, und deshalb steht kein
/// `#[ignore]` daran: `stat(2)` beantwortet die Typfrage fuer jede Kennung
/// gleich, und die Behauptung stimmt damit unter jeder. Was unter `root`
/// verloren ginge, ist allein die Faehigkeit dieser Probe, einen Rueckfall auf
/// den Deskriptorweg zu fangen — dort duerfte `root` die Datei oeffnen. Ein
/// Schnitt ueber das tatsaechliche Ergebnis waere deshalb kein Gewinn: er
/// machte aus einer immer richtigen Behauptung eine, die sich selbst bestaetigt.
#[test]
fn eine_verknuepfung_auf_eine_datei_ohne_leserecht_ist_kein_ordner() {
    let ordner = Pruefordner::neu("verweisziel-datei-gesperrt");
    let ziel = ordner.fuelldatei("ziel.txt", 3);
    fs::set_permissions(&ziel, fs::Permissions::from_mode(0o000))
        .expect("Rechte lassen sich nicht entziehen");
    let verweis = ordner.verknuepfung("verweis.txt", &ziel);

    assert_eq!(verweisziel::bestimmen(&verweis), Verweisziel::KeinOrdner);
}

/// Eine Verknuepfung auf ein Verzeichnis ohne Leserecht ist trotzdem ein Ordner.
///
/// Modus `0111` laesst durchschreiten und nicht lesen. `open` scheitert daran
/// mit `EACCES`, `stat` nicht; der zweite gemessene Fehlfall des Befunds
/// `260815-1713` kam deshalb als `Unerreichbar` statt als `Ordner` zurueck.
///
/// **Das Leserecht prueft [`Verweisziel::Ordner`] bewusst nicht.** Der Einstieg
/// landet danach in einer leeren Liste — genau wie heute schon der Einstieg in
/// einen gewoehnlichen [`Typ::Ordner`] ohne Leserecht, und das ist eine Regel
/// statt zweier. Zum Verhalten als `root` siehe die Probe darueber.
#[test]
fn eine_verknuepfung_auf_ein_verzeichnis_ohne_leserecht_ist_ein_ordner() {
    let ordner = Pruefordner::neu("verweisziel-ordner-gesperrt");
    let ziel = ordner.ordner("ziel");
    fs::set_permissions(&ziel, fs::Permissions::from_mode(0o111))
        .expect("Rechte lassen sich nicht entziehen");
    let verweis = ordner.verknuepfung("verweis", &ziel);

    assert_eq!(verweisziel::bestimmen(&verweis), Verweisziel::Ordner);
}

/// Eine Verknuepfung auf einen Unix-Socket ist kein Ordner.
///
/// Der dritte am Referenzgeraet gemessene Fehlfall des Befunds `260815-1713`:
/// `open(O_RDONLY|O_NONBLOCK)` scheitert an einem Socket mit `EOPNOTSUPP`, und
/// der Doppelklick bekam deshalb „laesst sich nicht oeffnen" in die
/// Statuszeile, statt den Eintrag wie jede andere Nicht-Ordner-Zeile zu
/// behandeln. `stat(2)` sagt dagegen ohne Umstaende, was dasteht.
///
/// **Von den drei Proben zu diesem Befund ist dies die einzige, die unter jeder
/// Kennung misst, was sie zu messen vorgibt.** Die beiden darueber haengen an
/// entzogenen Rechten, und unter `root` duerfte `open` die entsperrten Ziele
/// oeffnen: ihre Behauptung bliebe richtig, ihre Faehigkeit, einen Rueckfall auf
/// den Deskriptorweg zu fangen, waere weg. `EOPNOTSUPP` haengt an der Art des
/// Eintrags und nicht an Rechten und faellt fuer `root` genauso an.
#[test]
fn eine_verknuepfung_auf_einen_socket_ist_kein_ordner() {
    let ordner = Pruefordner::neu("verweisziel-socket");
    let ziel = ordner.socket("s");
    let verweis = ordner.verknuepfung("verweis", &ziel);

    assert_eq!(verweisziel::bestimmen(&verweis), Verweisziel::KeinOrdner);
}

/// Eine Verknuepfung ins Leere ist unerreichbar, und der Grund kommt mit.
///
/// Der Grund ist die Meldung des Systems und keine eigene Formulierung; den
/// Satz darum herum baut die Oberflaeche, die auch den Pfad hat. Ohne ihn
/// bliebe der Doppelklick auf eine Verknuepfung, deren Ziel geloescht wurde,
/// wirkungslos und stumm.
#[test]
fn eine_verknuepfung_ins_leere_ist_unerreichbar() {
    let ordner = Pruefordner::neu("verweisziel-leer");
    let verweis = ordner.verknuepfung("verweis", ordner.unter("gibtsnicht"));

    let Verweisziel::Unerreichbar { grund } = verweisziel::bestimmen(&verweis) else {
        panic!("die Verknuepfung ins Leere kam nicht als unerreichbar zurueck");
    };
    assert!(!grund.is_empty(), "der Grund ist leer");
}

/// Ein Ring aus zwei Verknuepfungen ist unerreichbar und haelt nichts an.
///
/// `ELOOP` ist einer der Fehlschlaege, mit denen `stat(2)` den Namen nicht
/// aufloest, und er kommt aus demselben Aufruf wie das fehlende Ziel; eine
/// eigene Regel braucht er deshalb nicht. Ein Ring ist dabei nicht der einzige
/// Weg dorthin: macOS meldet `ELOOP` ab `SYMLOOP_MAX` aufgeloesten
/// Verknuepfungen, also auch fuer eine lange Kette ohne Ring.
#[test]
fn ein_ring_aus_verknuepfungen_ist_unerreichbar() {
    let ordner = Pruefordner::neu("verweisziel-ring");
    ordner.verknuepfung("hin", ordner.unter("her"));
    let her = ordner.verknuepfung("her", ordner.unter("hin"));

    assert!(
        matches!(
            verweisziel::bestimmen(&her),
            Verweisziel::Unerreichbar { .. }
        ),
        "der Ring kam nicht als unerreichbar zurueck"
    );
}

/// Ein Verzeichnis und eine Datei ohne jede Verknuepfung kommen richtig zurueck.
///
/// Die Funktion fragt nicht, ob der Name eine Verknuepfung ist; sie sagt, was
/// hinter ihm steht. Gerufen wird sie im Einstiegsweg nur fuer eine
/// Verknuepfung, und diese Probe haelt fest, dass die Einschraenkung beim
/// Aufrufer liegt und nicht in der Funktion.
#[test]
fn ohne_verknuepfung_gilt_der_name_selbst() {
    let ordner = Pruefordner::neu("verweisziel-ohne");
    let unten = ordner.ordner("unten");
    let datei = ordner.fuelldatei("datei.txt", 1);

    assert_eq!(verweisziel::bestimmen(&unten), Verweisziel::Ordner);
    assert_eq!(verweisziel::bestimmen(&datei), Verweisziel::KeinOrdner);
}

/// Eine benannte Roehre ohne Schreiber haelt die Frage nicht an.
///
/// Die Zusage ist echt und diese Probe misst sie: ein Doppelklick darf nicht
/// haengen bleiben. **Ihr Gegenstueck ist ein blockierendes `open(2)` und nicht
/// `File::open`**, wie hier bis zum 260815 stand. `File::open` war nie die
/// Alternative; zur Wahl standen das Fragen am Deskriptor und das Fragen am
/// Namen, und seit dem Befund `260815-1713` fragt `bestimmen` am Namen. Ein
/// `stat(2)` fasst die Roehre gar nicht erst an, wartet also auch nicht auf
/// einen Schreiber.
///
/// Die Zeitschranke macht aus dem Stillstand einen Fehlschlag mit Namen; sie
/// ist dieselbe Bauart wie `oeffnen_mit_zeitschranke` in `tests/text.rs`, und
/// eine gemeinsame Fassung gaebe es nur um den Preis, den Pruefling durch die
/// Hilfsfunktion zu reichen.
#[test]
fn eine_roehre_haelt_die_frage_nach_dem_verweisziel_nicht_an() {
    let ordner = Pruefordner::neu("verweisziel-roehre");
    let roehre = ordner.roehre("ohne-schreiber");

    let (sender, empfaenger) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sender.send(verweisziel::bestimmen(&roehre));
    });
    let schranke = Duration::from_secs(5);
    let ergebnis = empfaenger.recv_timeout(schranke).unwrap_or_else(|_| {
        panic!("bestimmen ist nach {schranke:?} nicht zurueckgekommen; die Frage haengt")
    });

    assert_eq!(ergebnis, Verweisziel::KeinOrdner);
}

/// Eine benannte Roehre ohne Schreiber haelt den Schwungleser nicht an.
///
/// `Schwungleser::oeffnen` ist der Eingang jedes Lesens, Durchlaufs und
/// Zaehlens; bis zum Defekt `260826-1221` oeffnete er mit `File::open`, und ein
/// Pfad, der in diesem Augenblick auf eine Roehre ohne Schreiber zeigt, liess
/// den Faden fuer immer stehen — vor dem Abbruchkennzeichen, das erst nach dem
/// Oeffnen gelesen wird. Seither geht er ueber `ohne_warten_oeffnen` wie die
/// Textwege und die Archivwege, und die Antwort kommt vom `fstat` am
/// Deskriptor: kein Verzeichnis, ohne Betriebssystemnummer.
///
/// Die Zeitschranke macht aus dem Stillstand einen Fehlschlag mit Namen; ohne
/// Behebung endet die Probe nach fuenf Sekunden in ihrer Panik.
#[test]
fn eine_benannte_roehre_ohne_schreiber_haelt_den_schwungleser_nicht_an() {
    let ordner = Pruefordner::neu("schwungleser-roehre");
    let roehre = ordner.roehre("ohne-schreiber");

    let ergebnis = mit_zeitschranke("Schwungleser::oeffnen", Duration::from_secs(5), move || {
        Schwungleser::oeffnen(&roehre).map(|_| ())
    });

    let fehler = ergebnis.expect_err("eine Roehre ist kein Verzeichnis und gehoert abgewiesen");
    assert_eq!(fehler.kind(), std::io::ErrorKind::NotADirectory);
    assert!(
        fehler.raw_os_error().is_none(),
        "die Antwort kommt vom Aufrufer und traegt keine Betriebssystemnummer: {fehler:?}"
    );
}
