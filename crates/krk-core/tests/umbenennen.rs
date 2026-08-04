//! Das Umbenennen im Stapel: Regel, Vorschau und die drei Kollisionsfaelle
//! (C4, Schritt 17).
//!
//! Die fuenf Faelle, die das Abnahmekriterium von S17 namentlich nennt:
//!
//! ```text
//! 1  Suchen und Ersetzen ueber 50 Namen
//! 2  Nummerierung mit drei Stellen ab 7
//! 3  Kollision mit einem bestehenden Eintrag
//! 4  Kollision zweier neuer Namen untereinander
//! 5  leerer neuer Name
//! ```
//!
//! **Kein Pruefordner.** Das Regelmodell rechnet auf Zeichenketten; die
//! Ausfuehrung, die das Dateisystem anfasst, ist
//! [`krk_core::operation::umbenennen`] aus S15 und hat ihre eigenen Pruefungen
//! in `tests/operation.rs`. Die Trennung ist der Grund, aus dem diese fuenf
//! Faelle ohne eine einzige angelegte Datei laufen.

use krk_core::operation::Namensfehler;
use krk_core::umbenennen::{Kollision, Nummerierung, Regel, vorschau};

/// Hilfe: aus Zeichenketten eine Namensliste.
fn namen<const N: usize>(liste: [&str; N]) -> Vec<String> {
    liste.iter().map(|name| (*name).to_owned()).collect()
}

/// Fall 1: Suchen und Ersetzen ueber 50 Namen.
///
/// Der Anwendungsfall aus dem Entscheidungsdatensatz: fuenfzig Fotos bekommen
/// eine gemeinsame Vorsilbe, statt fuenfzigmal getippt zu werden.
#[test]
fn suchen_und_ersetzen_ueber_fuenfzig_namen() {
    let markierte: Vec<String> = (1..=50).map(|nummer| format!("IMG_{nummer}.jpg")).collect();
    let regel = Regel {
        suchen: "IMG_".to_owned(),
        ersetzen: "Urlaub_".to_owned(),
        nummerierung: None,
    };

    let ergebnis = vorschau(&regel, &markierte, &markierte);
    assert_eq!(
        ergebnis.zeilen().len(),
        50,
        "je markiertem Eintrag eine Zeile"
    );
    assert_eq!(
        ergebnis.kollisionen(),
        0,
        "keiner der neuen Namen ist belegt"
    );
    assert_eq!(ergebnis.auszufuehren().count(), 50);

    let erste = &ergebnis.zeilen()[0];
    assert_eq!(erste.alt, "IMG_1.jpg");
    assert_eq!(erste.neu, "Urlaub_1.jpg");
    let letzte = &ergebnis.zeilen()[49];
    assert_eq!(letzte.alt, "IMG_50.jpg");
    assert_eq!(letzte.neu, "Urlaub_50.jpg");
}

/// Fall 2: Nummerierung mit drei Stellen ab 7.
///
/// Die Nummer haengt an den Stamm, damit die Endung eine Endung bleibt, und sie
/// zaehlt in Sichtreihenfolge hoch.
#[test]
fn nummerierung_mit_drei_stellen_ab_sieben() {
    let markierte = namen(["a.jpg", "b.jpg", "c.jpg", "d.jpg"]);
    let regel = Regel {
        suchen: String::new(),
        ersetzen: String::new(),
        nummerierung: Some(Nummerierung::neu(7, 3)),
    };

    let ergebnis = vorschau(&regel, &markierte, &markierte);
    let neue: Vec<&str> = ergebnis
        .zeilen()
        .iter()
        .map(|zeile| zeile.neu.as_str())
        .collect();
    assert_eq!(neue, ["a007.jpg", "b008.jpg", "c009.jpg", "d010.jpg"]);
    assert_eq!(ergebnis.kollisionen(), 0);

    // Dieselbe Regel, wie sie aus den Eingabefeldern des Blattes entsteht.
    let aus_eingabe = Regel::aus_eingabe("", "", "7", "3").expect("7 und 3 sind Zahlen");
    assert_eq!(aus_eingabe, regel);
}

/// Fall 3: Kollision mit einem bestehenden Eintrag.
///
/// Der bestehende Eintrag ist nicht markiert und wird deshalb nicht selbst
/// umbenannt; sein Name bleibt belegt.
#[test]
fn kollision_mit_einem_bestehenden_eintrag() {
    let markierte = namen(["entwurf.txt"]);
    let bestand = namen(["entwurf.txt", "bericht.txt", ".notiz"]);
    let regel = Regel {
        suchen: "entwurf".to_owned(),
        ersetzen: "bericht".to_owned(),
        nummerierung: None,
    };

    let ergebnis = vorschau(&regel, &markierte, &bestand);
    let zeile = &ergebnis.zeilen()[0];
    assert_eq!(zeile.neu, "bericht.txt");
    assert_eq!(zeile.kollision, Some(Kollision::Bestehender));
    assert!(
        !zeile.wird_umbenannt(),
        "ein kollidierender Eintrag bleibt stehen"
    );
    assert_eq!(ergebnis.kollisionen(), 1);
    assert_eq!(ergebnis.auszufuehren().count(), 0);

    // Ein ausgeblendeter Eintrag belegt seinen Namen ebenso: der Bestand ist
    // der ganze Ordner und nicht die sichtbare Liste.
    let versteckt = Regel {
        suchen: "entwurf.txt".to_owned(),
        ersetzen: ".notiz".to_owned(),
        nummerierung: None,
    };
    let ergebnis = vorschau(&versteckt, &markierte, &bestand);
    assert_eq!(ergebnis.zeilen()[0].kollision, Some(Kollision::Bestehender));
}

/// Fall 4: Kollision zweier neuer Namen untereinander.
///
/// Zwei markierte Eintraege bekommen denselben neuen Namen, und keiner der
/// beiden Namen steht vorher im Ordner. **Beide** Zeilen tragen den Grund und
/// nicht nur die zweite: der erste Eintrag ist so wenig zu vergeben wie der
/// zweite, und der Nutzer sieht nicht, welcher von beiden "zuerst" ist.
#[test]
fn kollision_zweier_neuer_namen_untereinander() {
    // Der Suchtext trifft in beiden Namen den unterscheidenden Teil und
    // streicht ihn; danach heissen beide gleich.
    let markierte = namen(["scan-links.pdf", "scan-rechts.pdf"]);
    let bestand = markierte.clone();
    let regel = Regel {
        suchen: "scan".to_owned(),
        ersetzen: "Beleg".to_owned(),
        nummerierung: None,
    };
    let ergebnis = vorschau(&regel, &markierte, &bestand);
    assert_eq!(ergebnis.zeilen()[0].neu, "Beleg-links.pdf");
    assert_eq!(ergebnis.zeilen()[1].neu, "Beleg-rechts.pdf");
    assert_eq!(ergebnis.kollisionen(), 0, "noch sind es zwei Namen");

    let markierte = namen(["a-x.txt", "b-x.txt"]);
    let bestand = markierte.clone();
    let regel = Regel {
        // Der Anfangsbuchstabe faellt weg; uebrig bleibt zweimal "-x.txt".
        suchen: "a".to_owned(),
        ersetzen: "b".to_owned(),
        nummerierung: None,
    };
    let ergebnis = vorschau(&regel, &markierte, &bestand);
    assert_eq!(ergebnis.zeilen()[0].neu, "b-x.txt");
    assert_eq!(ergebnis.zeilen()[1].neu, "b-x.txt");
    assert_eq!(
        ergebnis.zeilen()[1].kollision,
        Some(Kollision::Doppelt),
        "der zweite Eintrag behaelt seinen Namen und trifft den ersten"
    );
    assert_eq!(ergebnis.auszufuehren().count(), 0);

    // Und derselbe Fall ohne jeden Bestand daneben, damit allein die
    // Doppelvergabe uebrig bleibt und nicht der bestehende Eintrag.
    let markierte = namen(["a-x.txt", "c-x.txt"]);
    let regel = Regel {
        suchen: "c".to_owned(),
        ersetzen: "a".to_owned(),
        nummerierung: None,
    };
    let ergebnis = vorschau(&regel, &markierte, &namen(["c-x.txt"]));
    assert_eq!(ergebnis.zeilen()[0].neu, "a-x.txt");
    assert_eq!(ergebnis.zeilen()[1].neu, "a-x.txt");
    assert_eq!(
        ergebnis.zeilen()[0].kollision,
        Some(Kollision::Doppelt),
        "beide Zeilen tragen den Grund, nicht nur die zweite"
    );
    assert_eq!(ergebnis.zeilen()[1].kollision, Some(Kollision::Doppelt));
    assert_eq!(ergebnis.auszufuehren().count(), 0);
}

/// Fall 5: leerer neuer Name.
///
/// Eine Regel, die den ganzen Namen streicht, laesst nichts uebrig. Das ist der
/// Fall, den C4 neben den beiden Kollisionen ausdruecklich nennt.
#[test]
fn ein_leerer_neuer_name_wird_markiert() {
    let markierte = namen(["bericht"]);
    let bestand = namen(["bericht"]);
    let regel = Regel {
        suchen: "bericht".to_owned(),
        ersetzen: String::new(),
        nummerierung: None,
    };

    let ergebnis = vorschau(&regel, &markierte, &bestand);
    let zeile = &ergebnis.zeilen()[0];
    assert_eq!(zeile.neu, "");
    assert_eq!(
        zeile.kollision,
        Some(Kollision::Unzulaessig(Namensfehler::Leer))
    );
    assert_eq!(
        zeile.kollision.expect("gerade geprueft").grund(),
        "der Name ist leer"
    );
    assert_eq!(ergebnis.auszufuehren().count(), 0);

    // Ein Name, der nur noch aus Leerzeichen bestuende, zaehlt genauso: die
    // Pruefung ist `name_pruefen` aus S15 und keine zweite daneben.
    let regel = Regel {
        suchen: "bericht".to_owned(),
        ersetzen: "   ".to_owned(),
        nummerierung: None,
    };
    let ergebnis = vorschau(&regel, &markierte, &bestand);
    assert_eq!(
        ergebnis.zeilen()[0].kollision,
        Some(Kollision::Unzulaessig(Namensfehler::Leer))
    );
}

/// Ein Schraegstrich im Ersetzungstext macht aus dem Namen einen Pfad.
///
/// Nicht in der Aufzaehlung des Abnahmekriteriums, aber derselbe Weg: die
/// Vorschau haelt ihn zurueck, statt die Ausfuehrung an ihm scheitern zu
/// lassen.
#[test]
fn ein_schraegstrich_im_neuen_namen_haelt_den_eintrag_zurueck() {
    let markierte = namen(["a.txt"]);
    let regel = Regel {
        suchen: "a".to_owned(),
        ersetzen: "unten/a".to_owned(),
        nummerierung: None,
    };
    let ergebnis = vorschau(&regel, &markierte, &markierte);
    assert_eq!(
        ergebnis.zeilen()[0].kollision,
        Some(Kollision::Unzulaessig(Namensfehler::Schraegstrich))
    );
}

/// Ordner und Dateien gehen denselben Weg (C4).
///
/// Das Regelmodell kennt den Typ eines Eintrags nicht, und genau daran haengt
/// die Zusage: es gibt keine Stelle, an der ein Ordner anders behandelt wuerde.
#[test]
fn ordner_und_dateien_gehen_denselben_weg() {
    let markierte = namen(["Bilder", "bericht.txt"]);
    let regel = Regel {
        suchen: String::new(),
        ersetzen: String::new(),
        nummerierung: Some(Nummerierung::neu(1, 2)),
    };
    let ergebnis = vorschau(&regel, &markierte, &markierte);
    assert_eq!(ergebnis.zeilen()[0].neu, "Bilder01");
    assert_eq!(ergebnis.zeilen()[1].neu, "bericht02.txt");
}
