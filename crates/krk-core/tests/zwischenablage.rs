//! Abnahme der Zwischenablage-Deutung (Schritt 13 des Plans, Faehigkeit C10).
//!
//! Ohne Fenster und ohne AppKit. Geprueft ist allein die Auswertung: aus einer
//! Zeichenkette wird ein lokaler Pfad, eine Web-Adresse oder nichts
//! Verwertbares. Wo die Zeichenkette herkommt (`NSPasteboard`) und wer die
//! Adresse an den Systembrowser gibt (`NSWorkspace`), steht in
//! `krk-ui/src/appkit/zwischenablage.rs` und ist hier nicht pruefbar.
//!
//! Die sieben Faelle unten sind die, die das Abnahmekriterium von Schritt 13
//! einzeln nennt.

use std::path::PathBuf;

use krk_core::zwischenablage::{Ziel, deuten};

/// Fall 1: ein Ordnerpfad.
#[test]
fn ein_ordnerpfad_fuehrt_zum_sprung() {
    assert_eq!(
        deuten("/Users/k1/Projekte"),
        Ziel::Pfad(PathBuf::from("/Users/k1/Projekte"))
    );
}

/// Fall 2: ein Dateipfad.
///
/// Die Auswertung unterscheidet Ordner und Datei nicht: beide sind ein
/// absoluter lokaler Pfad. Was der Pfad zeigt, entscheidet erst die Pruefung in
/// `kommandos::pfadeingabe`, und die ist dieselbe wie fuer die Pfadeingabe von
/// Hand. Zwei Wahrheiten darueber, was ein Pfad ist, entstehen nicht.
#[test]
fn ein_dateipfad_fuehrt_ebenfalls_zum_sprung() {
    assert_eq!(
        deuten("/Users/k1/Projekte/idee.txt"),
        Ziel::Pfad(PathBuf::from("/Users/k1/Projekte/idee.txt"))
    );
}

/// Fall 3: ein `file:`-Verweis.
///
/// Das ist die Sorte, die der Finder bei Cmd+C auf einer Datei ablegt
/// (Nutzerentscheid 260804,
/// `decisions/260804-0830_*_was-die-zwischenablage-auswertung-liest.md`). Er
/// zaehlt als Pfad und nicht als Adresse: er benennt dasselbe und ist nur
/// anders geschrieben.
#[test]
fn ein_datei_verweis_zaehlt_als_pfad_und_nicht_als_adresse() {
    assert_eq!(
        deuten("file:///Users/k1/Projekte/idee.txt"),
        Ziel::Pfad(PathBuf::from("/Users/k1/Projekte/idee.txt"))
    );
}

/// Fall 4: eine `https:`-Adresse.
#[test]
fn eine_https_adresse_geht_an_den_systembrowser() {
    assert_eq!(
        deuten("https://www.anthropic.com/"),
        Ziel::Web("https://www.anthropic.com/".to_owned())
    );
    assert_eq!(
        deuten("http://example.org"),
        Ziel::Web("http://example.org".to_owned())
    );
}

/// Fall 5: eine `smb:`-Adresse.
///
/// Der Fall, an dem die Schemata-Grenze haengt. Sie an das System
/// weiterzureichen hiesse, ueber einen Umweg die Serververbindung aufzubauen,
/// die C9 ausschliesst.
#[test]
fn eine_smb_adresse_ist_nicht_verwertbar() {
    assert_eq!(deuten("smb://fileserver/freigabe"), Ziel::Nichts);
    assert_eq!(deuten("ftp://ftp.example.org/pub"), Ziel::Nichts);
    assert_eq!(deuten("sftp://host/pfad"), Ziel::Nichts);
    assert_eq!(deuten("s3://eimer/schluessel"), Ziel::Nichts);
    assert_eq!(deuten("davs://host/freigabe"), Ziel::Nichts);
}

/// Fall 6: ein relativer Pfad.
///
/// Die Regel "absolut" ist von C2 geerbt und keine eigene: die Pfadeingabe von
/// Hand verlangt sie schon.
#[test]
fn ein_relativer_pfad_ist_nicht_verwertbar() {
    assert_eq!(deuten("Projekte/idee.txt"), Ziel::Nichts);
    assert_eq!(deuten("./idee.txt"), Ziel::Nichts);
    assert_eq!(deuten("../idee.txt"), Ziel::Nichts);
    assert_eq!(deuten("idee.txt"), Ziel::Nichts);
    assert_eq!(
        deuten("~/Projekte"),
        Ziel::Nichts,
        "die Tilde ist eine Abkuerzung der Shell und kein absoluter Pfad"
    );
}

/// Fall 7: eine leere Zeichenkette.
#[test]
fn eine_leere_zwischenablage_ist_nicht_verwertbar() {
    assert_eq!(deuten(""), Ziel::Nichts);
    assert_eq!(deuten("   \n\t "), Ziel::Nichts);
}

/// Umgebende Leerzeichen sind kein Grund, einen Pfad abzuweisen.
///
/// Ein aus einem Terminal oder einer Textdatei kopierter Pfad traegt oft einen
/// Zeilenumbruch am Ende. Ihn als "nichts Verwertbares" zu melden waere die
/// Fehlermeldung im haeufigen Fall.
#[test]
fn umgebende_leerzeichen_werden_abgeschnitten() {
    assert_eq!(
        deuten("  /Users/k1/Projekte\n"),
        Ziel::Pfad(PathBuf::from("/Users/k1/Projekte"))
    );
}

/// Das Schema ist schreibungsunabhaengig.
#[test]
fn die_schreibung_des_schemas_entscheidet_nicht() {
    assert_eq!(
        deuten("HTTPS://example.org"),
        Ziel::Web("HTTPS://example.org".to_owned())
    );
    assert_eq!(
        deuten("File:///Users/k1"),
        Ziel::Pfad(PathBuf::from("/Users/k1"))
    );
}
