//! Abnahme der Tastaturnavigation (Schritt 13 des Plans, Faehigkeit C2).
//!
//! Alle Pruefungen laufen ohne Fenster und ohne AppKit. Geprueft ist die reine
//! Logik: die Auswahl beim Aufstieg und die vier Markierungsbefehle auf einem
//! Ordnermodell mit 1.000 Eintraegen.
//!
//! **Die Sprungmarke aus C2 stand bis zur Runde 10 hier obenan.** Sie ist mit
//! ihrem Zeitablauf gefallen; der Filtertext des Tabs, der sie abgeloest hat,
//! wird in `tests/verzeichnis.rs` abgenommen, wo auch der Pruefschritt der
//! Sichtbarkeit und der Durchlauf stehen.
//!
//! Das Modell entsteht aus einem echten Pruefordner und nicht aus von Hand
//! gebauten Eintraegen. Der Grund ist der Sortierschluessel: er wird beim Lesen
//! einmal berechnet, und ihn hier nachzubauen hiesse, die Berechnung aus
//! `Eintrag::aus_roh` ein zweites Mal zu fuehren. Ohne ihn verglichen sich alle
//! Eintraege als gleich, und die Reihenfolge, gegen die diese Pruefungen
//! messen, waere beliebig.

use krk_core::verzeichnis::{Ordnermodell, aufwaerts, lesen};

mod gemeinsam;
use gemeinsam::Pruefordner;

/// Ein gelesenes und sortiertes Modell des Pruefordners.
fn modell_von(ordner: &Pruefordner) -> Ordnermodell {
    let eintraege = lesen(ordner.pfad()).expect("der Pruefordner laesst sich lesen");
    let mut modell = Ordnermodell::neu(1);
    modell.anhaengen(eintraege);
    modell.abschliessen();
    modell
}

/// Ein Modell mit 1.000 Dateien, `datei-0000.txt` bis `datei-0999.txt`.
fn tausend() -> (Pruefordner, Ordnermodell) {
    let ordner = Pruefordner::neu("tausend");
    for nummer in 0..1_000 {
        ordner.fuelldatei(&format!("datei-{nummer:04}.txt"), 1);
    }
    let modell = modell_von(&ordner);
    assert_eq!(
        modell.zeilenzahl(),
        1_000,
        "der Pruefordner ist unvollstaendig"
    );
    (ordner, modell)
}

/// Ein Modell mit 1.000 Eintraegen: jeder zehnte ein Ordner.
///
/// 100 Ordner und 900 Dateien, jede Datei so viele Bytes, wie ihre Nummer
/// sagt. Die erwartete Groessensumme steht damit fest und wird hier gerechnet
/// und nicht hingeschrieben: eine Summe im Programmtext waere beim naechsten
/// Zuschnitt des Pruefordners falsch.
fn tausend_gemischt() -> (Pruefordner, Ordnermodell, u64) {
    let ordner = Pruefordner::neu("tausend-gemischt");
    let mut groessensumme = 0u64;
    for nummer in 0..1_000usize {
        let name = format!("eintrag-{nummer:04}");
        if nummer.is_multiple_of(10) {
            ordner.ordner(&name);
        } else {
            // Eine Datei ohne Bytes waere kein Beleg dafuer, dass die Summe
            // wirklich summiert; deshalb wenigstens ein Byte.
            let bytes = nummer % 97 + 1;
            ordner.fuelldatei(&name, bytes);
            groessensumme += bytes as u64;
        }
    }
    let modell = modell_von(&ordner);
    assert_eq!(
        modell.zeilenzahl(),
        1_000,
        "der Pruefordner ist unvollstaendig"
    );
    (ordner, modell, groessensumme)
}

fn name_in_zeile(modell: &Ordnermodell, zeile: usize) -> &str {
    modell
        .zeile(zeile)
        .map(|eintrag| eintrag.name.as_str())
        .expect("die Zeile steht nicht in der Sicht")
}

// ---------------------------------------------------------------------------
// Der Aufstieg und seine Auswahl (C2)
// ---------------------------------------------------------------------------

#[test]
fn der_aufstieg_stellt_die_auswahl_auf_den_verlassenen_ordner() {
    let ordner = Pruefordner::neu("aufstieg");
    ordner.ordner("Unterordner");
    ordner.fuelldatei("nebenan.txt", 1);
    let modell = modell_von(&ordner);

    let unten = ordner.pfad().join("Unterordner");
    let (eltern, name) = aufwaerts(&unten).expect("der Unterordner hat einen uebergeordneten");

    assert_eq!(eltern, ordner.pfad());
    assert_eq!(name, "Unterordner");
    let index = modell
        .index_von_namen(&name)
        .expect("der verlassene Ordner steht im uebergeordneten");
    assert_eq!(
        modell
            .zeile_von(index)
            .map(|zeile| name_in_zeile(&modell, zeile)),
        Some("Unterordner"),
        "die Auswahl landet auf dem Ordner, aus dem der Nutzer kam"
    );
}

// ---------------------------------------------------------------------------
// Die vier Markierungsbefehle (C2), auf 1.000 Eintraegen
// ---------------------------------------------------------------------------

#[test]
fn ein_einzelner_eintrag_laesst_sich_markieren_und_wieder_freigeben() {
    let (_ordner, mut modell) = tausend();
    let index = modell
        .eintragsindex(0)
        .expect("die erste Zeile traegt einen Eintrag");

    assert!(!modell.ist_markiert(index));
    modell.markierung_umschalten(index);
    assert!(modell.ist_markiert(index));
    assert_eq!(modell.markierungsstand().zahl, 1);

    modell.markierung_umschalten(index);
    assert!(!modell.ist_markiert(index));
    assert_eq!(modell.markierungsstand().zahl, 0);
}

#[test]
fn alle_markieren_erfasst_jede_der_tausend_zeilen() {
    let (_ordner, mut modell) = tausend();

    modell.alle_markieren();

    assert_eq!(modell.markierungsstand().zahl, 1_000);
    for zeile in 0..modell.zeilenzahl() {
        let index = modell
            .eintragsindex(zeile)
            .expect("jede Zeile hat einen Index");
        assert!(modell.ist_markiert(index), "Zeile {zeile} blieb unmarkiert");
    }
}

#[test]
fn die_markierung_aufheben_raeumt_jede_der_tausend_wieder_ab() {
    let (_ordner, mut modell) = tausend();
    modell.alle_markieren();

    modell.markierung_aufheben();

    assert_eq!(modell.markierungsstand().zahl, 0);
}

#[test]
fn die_markierung_umkehren_tauscht_markierte_und_freie() {
    let (_ordner, mut modell) = tausend();
    // Jede zehnte Zeile markieren: 100 von 1.000.
    for zeile in (0..modell.zeilenzahl()).step_by(10) {
        let index = modell
            .eintragsindex(zeile)
            .expect("jede Zeile hat einen Index");
        modell.markierung_umschalten(index);
    }
    assert_eq!(modell.markierungsstand().zahl, 100);

    modell.markierung_umkehren();

    assert_eq!(modell.markierungsstand().zahl, 900);
    let erste = modell.eintragsindex(0).expect("die erste Zeile");
    let zweite = modell.eintragsindex(1).expect("die zweite Zeile");
    assert!(!modell.ist_markiert(erste), "die vorher markierte ist frei");
    assert!(modell.ist_markiert(zweite), "die vorher freie ist markiert");
}

/// Die Markierung haengt am Eintrag und nicht an der Zeile.
#[test]
fn die_markierung_ueberlebt_einen_sortierwechsel() {
    let ordner = Pruefordner::neu("sortierwechsel");
    for name in ["aaa.txt", "zzz.txt"] {
        ordner.fuelldatei(name, 1);
    }
    let mut modell = modell_von(&ordner);
    let index = modell
        .index_von_namen("aaa.txt")
        .expect("aaa.txt steht im Ordner");
    modell.markierung_umschalten(index);
    assert_eq!(name_in_zeile(&modell, 0), "aaa.txt");

    modell.sortierung_setzen(krk_core::verzeichnis::Sortierung::neu(
        krk_core::verzeichnis::Schluessel::Name,
        krk_core::verzeichnis::Richtung::Absteigend,
    ));

    assert_eq!(
        name_in_zeile(&modell, 0),
        "zzz.txt",
        "die Probe traegt nur, wenn sich die Zeilen wirklich vertauscht haben"
    );
    assert_eq!(modell.markierungsstand().zahl, 1);
    assert!(
        modell.ist_markiert(index),
        "die Markierung ist beim Sortieren verloren gegangen"
    );
}

/// Ein ausgeblendeter Eintrag wird nicht mitmarkiert, seine Markierung aber
/// auch nicht stillschweigend behalten, wenn der Nutzer alles abraeumt.
#[test]
fn alle_markieren_laesst_die_ausgeblendeten_aus_und_aufheben_erfasst_sie_doch() {
    let ordner = Pruefordner::neu("verstecke");
    ordner.fuelldatei("sichtbar.txt", 1);
    ordner.fuelldatei(".versteckt.txt", 1);
    let mut modell = modell_von(&ordner);

    // Zuerst sichtbar machen und markieren, dann ausblenden.
    modell.verstecke_ausblenden_setzen(false);
    let versteckt = modell
        .index_von_namen(".versteckt.txt")
        .expect("die versteckte Datei ist gelesen");
    modell.markierung_umschalten(versteckt);
    modell.verstecke_ausblenden_setzen(true);
    assert_eq!(modell.zeilenzahl(), 1, "nur die sichtbare Datei steht da");

    modell.markierung_aufheben();
    assert_eq!(
        modell.markierungsstand().zahl,
        0,
        "auch die ausgeblendete Markierung ist fort"
    );

    modell.alle_markieren();
    assert_eq!(
        modell.markierungsstand().zahl,
        1,
        "markiert ist allein die sichtbare Datei"
    );
    assert!(!modell.ist_markiert(versteckt));
}

// ---------------------------------------------------------------------------
// Der Markierungsstand in der Statuszeile (C1, C2), Schritt 16c
// ---------------------------------------------------------------------------

#[test]
fn ohne_markierung_sind_alle_drei_werte_null() {
    let (_ordner, modell, _summe) = tausend_gemischt();

    let stand = modell.markierungsstand();

    assert!(stand.ist_leer());
    assert_eq!(stand.zahl, 0);
    assert_eq!(stand.ordner, 0);
    assert_eq!(stand.groesse, 0);
}

#[test]
fn alle_markieren_zaehlt_ordner_gesondert_und_summiert_allein_die_dateien() {
    let (_ordner, mut modell, summe) = tausend_gemischt();

    modell.alle_markieren();
    let stand = modell.markierungsstand();

    assert_eq!(stand.zahl, 1_000, "gezaehlt werden Ordner und Dateien");
    assert_eq!(stand.ordner, 100, "jeder zehnte Eintrag ist ein Ordner");
    assert_eq!(
        stand.groesse, summe,
        "die Summe zaehlt genau die Bytes der 900 Dateien"
    );
}

/// Ein Ordner hat keine Groesse, die sich ohne Durchlauf ermitteln liesse.
#[test]
fn ein_markierter_ordner_erhoeht_die_groessensumme_nicht() {
    let ordner = Pruefordner::neu("markierter-ordner");
    ordner.ordner("Unterordner");
    ordner.fuelldatei("datei.txt", 42);
    let mut modell = modell_von(&ordner);
    let unterordner = modell
        .index_von_namen("Unterordner")
        .expect("der Unterordner ist gelesen");
    let datei = modell
        .index_von_namen("datei.txt")
        .expect("die Datei ist gelesen");

    modell.markierung_umschalten(unterordner);
    let nur_ordner = modell.markierungsstand();
    assert_eq!(nur_ordner.zahl, 1);
    assert_eq!(nur_ordner.ordner, 1);
    assert_eq!(nur_ordner.groesse, 0, "ein Ordner bringt keine Bytes mit");

    modell.markierung_umschalten(datei);
    let beide = modell.markierungsstand();
    assert_eq!(beide.zahl, 2);
    assert_eq!(beide.ordner, 1, "die Datei erhoeht die Ordnerzahl nicht");
    assert_eq!(beide.groesse, 42, "allein die Datei zaehlt in der Summe");
}
