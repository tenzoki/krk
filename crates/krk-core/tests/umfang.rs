//! Abnahme der gedeckelten Zaehlung eines Unterbaums (Schritt 7 der Loeschrunde).
//!
//! Alle Proben laufen ohne Fenster und ohne AppKit. Ihre Pruefordner kommen aus
//! `tests/gemeinsam/`, der einen Fassung fuer alle Abnahmeproben des Kerns; sie
//! tragen Prozesskennung und Laufnummer und raeumen sich in `Drop` selbst ab.
//!
//! # Warum diese Proben hier stehen und nicht in `tests/verzeichnis.rs`
//!
//! Weil sie einen echten Baum brauchen und `Pruefordner` unter `tests/gemeinsam/`
//! liegt: eine Probe in `#[cfg(test)]` neben dem Modul erreicht ihn nicht. Ein
//! eigenes Ziel und nicht ein Anhang an `tests/verzeichnis.rs`, weil die Frage
//! eine andere ist — dort steht die Abnahme des Verzeichnislesers, des
//! Ordnermodells und des Durchlaufs, hier die einer Zaehlung mit Deckel.
//!
//! # Die zwei Zusagen ueber Deskriptoren, und welche davon messbar ist
//!
//! `krk_core::verzeichnis::umfang` sagt zwei verschiedene Dinge zu:
//!
//! 1. **Ein Mangel von aussen laesst die Zaehlung unentschieden.** Gemessen von
//!    `ein_deskriptormangel_von_aussen_laesst_den_umfang_unentschieden`.
//! 2. **Die Zaehlung erzeugt keinen Mangel selbst**, denn sie haelt einen
//!    Deskriptor und nicht einen je Ebene. Gemessen von
//!    `die_tiefe_kette_kostet_einen_deskriptor_und_nicht_einen_je_ebene`.
//!
//! **Die zweite braucht eine tiefere Grenze als die Proben des Durchlaufs**, und
//! zwar aus einem Grund, der zur Sache gehoert: der Deckel der Zaehlung begrenzt
//! die Zahl der geoeffneten Verzeichnisse ohnehin auf `SCHWELLE + 1`, also 26.
//! Unter `ulimit -n 64` liefe deshalb auch ein Abstieg durch, der einen
//! Deskriptor je Ebene haelt — die Probe bestaetigte dann eine Bauform, die sie
//! gar nicht geprueft hat. [`GRENZE`] liegt darum unter 26, und das Kind rechnet
//! nach, dass es wirklich weniger als 26 Deskriptoren bekommt.
//!
//! Der Deckel macht den Fehler also seltener und nicht falsch, und genau das
//! haelt diese Datei fest.

use std::fs;
use std::path::PathBuf;

use krk_core::verzeichnis::umfang::{SCHWELLE, Umfang, zaehlen};

mod gemeinsam;
use gemeinsam::{Pruefordner, kind_mit_deskriptorgrenze, kindauftrag};

/// Der Deckel, gegen den [`zaehlen`] zaehlt, in der Sprache der Proben.
///
/// `umfang::DECKEL` ist privat, und das soll er bleiben: kein Aufrufer hat mit
/// ihm zu rechnen. Die Proben rechnen ihn aus [`SCHWELLE`] nach, statt die 26
/// auszuschreiben.
const DECKEL: u32 = SCHWELLE + 1;

/// Legt `anzahl` Dateien unmittelbar in den Ordner.
fn dateien_anlegen(ordner: &Pruefordner, anzahl: u32) -> Vec<PathBuf> {
    (0..anzahl)
        .map(|nummer| ordner.datei(&format!("d-{nummer:03}.txt"), b"x"))
        .collect()
}

/// Legt unter `ordner` eine Kette aus `tiefe` einstufigen Unterordnern an und
/// liefert deren obersten.
fn kette_anlegen(ordner: &Pruefordner, tiefe: usize) -> PathBuf {
    let oben = ordner.unter("kette");
    let mut tief = oben.clone();
    for _ in 0..tiefe {
        tief = tief.join("e");
    }
    fs::create_dir_all(&tief).expect("Kette laesst sich nicht anlegen");
    oben
}

// ---------------------------------------------------------------------------
// Die genaue Zahl, solange sie unter der Schwelle liegt
// ---------------------------------------------------------------------------

/// Ein flacher Ordner unter der Schwelle liefert die genaue Zahl.
///
/// Zwei Blickwinkel auf denselben Ordner, und beide gehoeren zur Zusage: fuenf
/// **ausgewaehlte Dateien** sind fuenf, und der **ausgewaehlte Ordner darueber**
/// ist sechs, weil er selbst mitzaehlt. Wer nur den ersten Fall prueft, sieht
/// den Abstieg nicht; wer nur den zweiten prueft, sieht die Eins je Auswahl
/// nicht.
#[test]
fn ein_flacher_ordner_unter_der_schwelle_wird_genau_gezaehlt() {
    let ordner = Pruefordner::neu("umfang-flach");
    let dateien = dateien_anlegen(&ordner, 5);

    assert_eq!(
        zaehlen(&dateien),
        Umfang::Genau(5),
        "fuenf ausgewaehlte Dateien sind fuenf Eintraege"
    );
    assert_eq!(
        zaehlen(&[ordner.pfad().to_path_buf()]),
        Umfang::Genau(6),
        "der ausgewaehlte Ordner zaehlt selbst mit, seine fuenf Dateien dazu"
    );
}

/// Ein ausgewaehlter Pfad, den es nicht mehr gibt, zaehlt eins.
///
/// Er steht in der Auswahl, und der Loeschauftrag nimmt ihn mit; ihn wegzulassen
/// hiesse, ueber einen Eintrag zu schweigen, den der Nutzer ausgewaehlt hat. Ein
/// Abstieg findet nicht statt, und unentschieden wird die Zaehlung davon nicht:
/// `lstat(2)` hat geantwortet, nur eben mit einem Fehler ueber den Pfad.
#[test]
fn ein_ausgewaehlter_pfad_ohne_eintrag_zaehlt_eins() {
    let ordner = Pruefordner::neu("umfang-fehlend");
    assert_eq!(
        zaehlen(&[ordner.unter("gibt-es-nicht")]),
        Umfang::Genau(1),
        "ein Pfad ohne Eintrag zaehlt nicht eins"
    );
}

/// Genau [`SCHWELLE`] Eintraege sind noch eine genaue Zahl.
///
/// Der eine Rand des Deckels, ausgeschrieben: der Ordner selbst und
/// `SCHWELLE - 1` Dateien darin. „Mehr als [`SCHWELLE`]" ist hier noch nicht
/// wahr, und die Rueckfrage bleibt aus diesem Grund ruhig.
#[test]
fn genau_die_schwelle_bleibt_eine_genaue_zahl() {
    let ordner = Pruefordner::neu("umfang-fuenfundzwanzig");
    dateien_anlegen(&ordner, SCHWELLE - 1);

    assert_eq!(
        zaehlen(&[ordner.pfad().to_path_buf()]),
        Umfang::Genau(SCHWELLE),
        "genau die Schwelle ist noch keine Ueberschreitung"
    );
}

/// Ein Eintrag mehr, und die Antwort kippt.
///
/// Der andere Rand des Deckels. Die Zahl in `MehrAls` ist die ueberschrittene
/// Schwelle und nicht die Zahl der gezaehlten Eintraege — gezaehlt wurde einer
/// mehr, und wie viele es wirklich sind, war nicht gefragt.
#[test]
fn ein_eintrag_ueber_der_schwelle_kippt_die_antwort() {
    let ordner = Pruefordner::neu("umfang-sechsundzwanzig");
    dateien_anlegen(&ordner, SCHWELLE);

    assert_eq!(
        zaehlen(&[ordner.pfad().to_path_buf()]),
        Umfang::MehrAls(SCHWELLE),
        "einer mehr als die Schwelle kippt die Antwort nicht"
    );
}

/// Eine tiefe Kette wird gezaehlt wie ein flacher Ordner: erst genau, dann
/// gedeckelt.
///
/// Beide Faelle an derselben Bauform, weil die Tiefe fuer die Antwort nichts
/// bedeutet: zehn Ebenen sind elf Eintraege, dreissig Ebenen sind mehr als die
/// Schwelle. Die Probe darunter misst, was sie kosten.
#[test]
fn eine_tiefe_kette_wird_bis_zum_deckel_gezaehlt() {
    let flach = Pruefordner::neu("umfang-kette-kurz");
    let kurz = kette_anlegen(&flach, 10);
    assert_eq!(
        zaehlen(&[kurz]),
        Umfang::Genau(11),
        "zehn Ebenen und der Ordner darueber sind elf Eintraege"
    );

    let tief = Pruefordner::neu("umfang-kette-lang");
    let lang = kette_anlegen(&tief, 30);
    assert_eq!(
        zaehlen(&[lang]),
        Umfang::MehrAls(SCHWELLE),
        "dreissig Ebenen liegen ueber der Schwelle"
    );
}

// ---------------------------------------------------------------------------
// Verknuepfungen zaehlen eins und werden nicht verfolgt
// ---------------------------------------------------------------------------

/// Eine Verknuepfung auf einen grossen Baum zaehlt eins, an beiden Stellen.
///
/// Entschieden wird das mit zwei verschiedenen Werkzeugen, und die Probe prueft
/// beide: an der **obersten Ebene** ueber `symlink_metadata`, **unterhalb eines
/// ausgewaehlten Ordners** am `Typ` des gelesenen Eintrags. Eine Fassung, die nur
/// eine der beiden Stellen traegt, faellt hier auf.
///
/// Die Gegenprobe in der Mitte gehoert dazu: ohne sie saehe die Probe genauso
/// aus, wenn der Baum hinter der Verknuepfung leer waere.
#[test]
fn eine_verknuepfung_auf_einen_grossen_baum_zaehlt_eins() {
    let ordner = Pruefordner::neu("umfang-verweis");
    let gross = ordner.ordner("gross");
    for nummer in 0..(SCHWELLE * 2) {
        fs::write(gross.join(format!("d-{nummer:03}.txt")), b"x").expect("Datei");
    }
    let verweis = ordner.verknuepfung("verweis", &gross);

    assert_eq!(
        zaehlen(std::slice::from_ref(&gross)),
        Umfang::MehrAls(SCHWELLE),
        "der Baum hinter der Verknuepfung ist nicht gross; die Probe sagte nichts aus"
    );
    assert_eq!(
        zaehlen(&[verweis]),
        Umfang::Genau(1),
        "die ausgewaehlte Verknuepfung ist verfolgt worden"
    );

    let huelle = ordner.ordner("huelle");
    std::os::unix::fs::symlink(&gross, huelle.join("verweis")).expect("Verknuepfung");
    assert_eq!(
        zaehlen(&[huelle]),
        Umfang::Genau(2),
        "eine Verknuepfung unterhalb des ausgewaehlten Ordners ist verfolgt worden"
    );
}

// ---------------------------------------------------------------------------
// Die zwei Zusagen ueber den Vorrat an Deskriptoren
// ---------------------------------------------------------------------------

/// Die Grenze, unter der die Kindproben dieser Datei laufen.
///
/// Unter 26, denn genau darin liegt der Unterschied zu den Proben des
/// Durchlaufs; der Modulkopf rechnet es aus. Empirisch traegt das Testgeruest
/// diese Grenze: das Kind bekommt darunter noch genug Deskriptoren, um zu
/// melden, und weniger als 26, um zu messen. Sinkt der Vorrat auf null, sagen
/// die Zusicherungen im Kind es und behaupten nichts.
const GRENZE: usize = 24;

/// Wie tief die Kette der Kindprobe ist.
///
/// Mehr als der Deckel, sonst waere die Kette schon vor der Deskriptorgrenze zu
/// Ende und die Probe messte nichts. Zugleich wenig genug, dass der Pfad
/// unterhalb des Temporaerverzeichnisses `PATH_MAX` nicht ausschoepft.
const KETTENTIEFE: usize = 30;

/// Ein Mangel an Deskriptoren macht die Zaehlung unentschieden und keine Zahl.
///
/// `EMFILE` und `ENFILE` sind ein Zustand des Prozesses und keine Aussage ueber
/// die Auswahl. Waere hier `Genau(1)` die Antwort — der ausgewaehlte Ordner,
/// dessen Inhalt niemand lesen konnte —, dann bliebe die Rueckfrage ueber einem
/// Baum mit Tausenden Eintraegen ruhig, weil KRK gerade keinen Deskriptor frei
/// hatte.
///
/// **Der Mangel wird hergestellt und nicht abgewartet.** Das Kind nimmt
/// Deskriptoren, bis keiner mehr kommt, und **haelt sie**, waehrend die Zaehlung
/// laeuft. Ihr erstes `File::open` kann dann nur noch `EMFILE` liefern.
///
/// Angelegt und abgeraeumt wird der Baum vom **Elternteil**: `remove_dir_all`
/// haelt selbst Deskriptoren und koennte unter der abgesenkten Grenze nicht
/// aufraeumen.
#[test]
fn ein_deskriptormangel_von_aussen_laesst_den_umfang_unentschieden() {
    let ordner = Pruefordner::neu("umfang-mangel");
    let aussen = ordner.ordner("aussen");
    for nummer in 0..3 {
        fs::write(aussen.join(format!("d-{nummer}.txt")), b"x").expect("Datei");
    }

    let ergebnis = kind_mit_deskriptorgrenze(
        GRENZE,
        "kind_laesst_den_umfang_bei_deskriptormangel_unentschieden",
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "ein Deskriptormangel des Prozesses wird zu einer Zahl ueber eine Auswahl\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_laesst_den_umfang_bei_deskriptormangel_unentschieden() {
    let Some(ordner) = kindauftrag() else {
        return;
    };
    let auswahl = vec![ordner.join("aussen")];

    // Erster Durchgang, mit freiem Vorrat, und er ist die Gegenprobe: ohne ihn
    // saehe der zweite auch dann so aus, wenn der Baum gar nicht stuende.
    assert_eq!(
        zaehlen(&auswahl),
        Umfang::Genau(4),
        "mit freiem Vorrat liefert die Zaehlung nicht die genaue Zahl; \
         der zweite Durchgang saegte nichts aus"
    );

    // Jetzt den Mangel herstellen: nehmen, bis keiner mehr kommt, und halten.
    let mut gehalten = Vec::new();
    let mut abweisung = None;
    while gehalten.len() < 4 * GRENZE {
        match fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(fehler) => {
                abweisung = Some(fehler);
                break;
            }
        }
    }

    // Die Zaehlung laeuft, waehrend `gehalten` steht: ihr erstes Oeffnen trifft
    // auf eine volle Deskriptortabelle.
    let ohne_vorrat = zaehlen(&auswahl);

    // Erst zurueckgeben, dann pruefen: eine gescheiterte Behauptung soll ihre
    // Meldung noch schreiben koennen.
    drop(gehalten);

    let abweisung = abweisung.expect(
        "der Vorrat an Deskriptoren ist nicht ausgegangen; die Grenze des Kindes ist nicht \
         abgesenkt, und die Probe wuerde nichts messen",
    );
    assert!(
        krk_core::verzeichnis::sys::ist_deskriptormangel(&abweisung),
        "der Vorrat ist aus einem anderen Grund ausgegangen: {abweisung}"
    );
    assert_eq!(
        ohne_vorrat,
        Umfang::Unentschieden,
        "ein Mangel an Deskriptoren wird zu einer Aussage ueber die Auswahl"
    );
}

/// Die tiefe Kette kostet einen Deskriptor und nicht einen je Ebene.
///
/// Der Deckel begrenzt die Zahl der geoeffneten Verzeichnisse auf 26. Ein
/// Abstieg, der den Leser der uebergeordneten Ebene offen haelt, braucht sie
/// deshalb **gleichzeitig**, und unter [`GRENZE`] bekommt er sie nicht: er
/// erzeugte seinen eigenen `EMFILE` und meldete darauf `Unentschieden`, wo die
/// Antwort `MehrAls` ist. Das ist der Defekt `260815-0211` in der Gestalt, die
/// dieses Modul haette bekommen koennen.
///
/// **Ohne die Bauform mit dem Stapel von Pfaden meldet diese Probe
/// `Unentschieden`.** Mit ihr steht zu jedem Zeitpunkt genau ein
/// Verzeichnisdeskriptor offen, und die Tiefe der Kette ist ohne Bedeutung.
///
/// Angelegt und abgeraeumt wird die Kette vom **Elternteil**, aus demselben
/// Grund wie bei der Probe darueber.
#[test]
fn die_tiefe_kette_kostet_einen_deskriptor_und_nicht_einen_je_ebene() {
    let ordner = Pruefordner::neu("umfang-kette-deskriptoren");
    kette_anlegen(&ordner, KETTENTIEFE);

    let ergebnis = kind_mit_deskriptorgrenze(
        GRENZE,
        "kind_zaehlt_die_tiefe_kette_mit_einem_deskriptor",
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "unter einer knappen Deskriptorgrenze faellt die Zaehlung der tiefen Kette aus\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_zaehlt_die_tiefe_kette_mit_einem_deskriptor() {
    let Some(ordner) = kindauftrag() else {
        return;
    };
    let auswahl = vec![ordner.join("kette")];

    // Erst die Grenze messen, dann die Frage stellen. Genommen wird, bis nichts
    // mehr kommt; was dabei zusammenkommt, ist der Vorrat dieses Kindes.
    let mut gehalten = Vec::new();
    while gehalten.len() < 4 * GRENZE {
        match fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(_) => break,
        }
    }
    let vorrat = gehalten.len();
    drop(gehalten);

    assert!(
        (vorrat as u32) < DECKEL,
        "der Vorrat von {vorrat} Deskriptoren reicht fuer {DECKEL} gleichzeitig offene \
         Verzeichnisse; ein Abstieg mit einem Deskriptor je Ebene liefe hier durch und \
         die Probe messte nichts"
    );
    assert!(
        vorrat > 0,
        "das Kind bekommt gar keinen Deskriptor mehr; die Grenze {GRENZE} ist zu tief, \
         und die Probe messte den Mangel statt der Bauform"
    );

    assert_eq!(
        zaehlen(&auswahl),
        Umfang::MehrAls(SCHWELLE),
        "die Kette aus {KETTENTIEFE} Ebenen ist unter {vorrat} freien Deskriptoren nicht \
         bis zum Deckel gezaehlt worden"
    );
}
