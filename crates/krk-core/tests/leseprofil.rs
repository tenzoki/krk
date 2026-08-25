//! Abnahme der Gestalt von `readers.toml`, des Pruefschritts dahinter
//! (Schritt 3 der Runde 16), der Ortserkennung (Schritt 5) und der vier
//! Bausteine (Schritt 6).
//!
//! Alle Proben hier laufen **ohne Fenster**, wie C6.8 es verlangt, und sie
//! zerfallen in die ersten zwei der drei Pruefformen aus dem Abschnitt
//! `## Testing Strategy` des Plans:
//!
//! - **Reine Rechnung, ohne Dateisystem**: der Pruefschritt, seine vier
//!   Abweisungen, die Erkennung in ihren zwei Durchgaengen und `als_text`. Sie
//!   lesen einen TOML-Text aus dem Quelltext, und die Eintraege eines Ordners
//!   kommen als von Hand gebaute Liste herein.
//! - **Gegen einen Pruefordner** in der Gestalt einer Werkbank: die vier
//!   Bausteine (C3.1 bis C3.13) und die eine Regel ueber die Teillesung.
//!
//! - **Zaehlend, gegen den Haushalt eines Laufs**: die neun Kriterien aus C6
//!   und die eine Haelfte von C2.8, die ohne Fenster zu belegen ist. Sie lesen
//!   [`krk_core::leseprofil::Haushalt`] **nach** dem Lauf aus, statt eine
//!   zweite Zaehlstelle neben die eine zu stellen, die der Lauf ohnehin
//!   fuehrt; C6.8 verlangt gezaehlte Aufrufe und keine Millisekunden.
//!
//! Zwei der neun stehen nicht in dieser Reihe, weil sie schon woanders belegt
//! sind und eine zweite Probe dieselbe Frage ein zweites Mal stellte: C6.3
//! haengt an
//! [`eine_anzahl_ueber_der_grenze_wird_gekappt_und_nicht_abgewiesen`], C6.5 an
//! [`eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet`].
//!
//! # Warum die Zahlen dieser Werkbank in keiner Probe stehen
//!
//! Der Bestand der echten Werkbank aendert sich mit jeder Sitzung: 54 offene
//! Defekte und 82 Datensaetze sind Staende vom 260824, und eine Probe darauf
//! waere morgen rot, ohne dass jemand etwas kaputtgemacht haette. Die Proben
//! bauen deshalb einen Ordner derselben **Gestalt** mit einem Bestand
//! bekannter Groesse, siehe [`werkbankgestalt`]. Nachgezaehlt wird am echten
//! Bestand einmal bei der Abnahme.
//!
//! # Die erste Probe ist die, die laufen muss
//!
//! [`eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte`]
//! nimmt die Verbindung aus `#[serde(flatten)]` und `#[serde(untagged)]` ab.
//! Die Vorlage `ablage::lesezeichen::Ziel` traegt zwei Varianten, diese
//! Auswahl vier, und ob `toml` so weit traegt, war am Papier nicht zu
//! entscheiden. Faellt sie, ist der Ausweg im Kopf von
//! `krk_core::leseprofil` benannt und nicht zu suchen.

use std::cell::Cell;
use std::fs::FileTimes;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use krk_core::ablage::leseprofile::AUSLIEFERUNGSTEXT;
use krk_core::leseprofil::datei::{Profildatei, pruefen};
use krk_core::leseprofil::erkennung::erkennen;
use krk_core::leseprofil::{
    Anzeige, Baustein, HOECHSTENS_BYTES, HOECHSTENS_EINTRAEGE, HOECHSTENS_JUENGSTE,
    HOECHSTENS_LESELAEUFE, HOECHSTENS_OEFFNUNGEN, Haushalt, Profil, Profile, Wert, Zusammenfassung,
    Zusammenfassungszeile, zusammenfassen, zusammenfassen_gezaehlt,
};
use krk_core::verzeichnis::sys::ortszeit;
use krk_core::verzeichnis::{Eintrag, Typ};

use gemeinsam::{Pruefordner, kind_mit_deskriptorgrenze};

mod gemeinsam;

/// Liest einen TOML-Text und prueft ihn, so wie die Ablage es beim Start tut.
///
/// Ein Text, der sich nicht lesen laesst, ist in diesen Proben ein Fehler der
/// Probe: der Fall der beschaedigten Datei gehoert der Ablage und wird in
/// Schritt 8 abgenommen.
fn gepruefte(text: &str) -> (Profile, Vec<String>) {
    let datei: Profildatei = toml::from_str(text).unwrap_or_else(|fehler| {
        panic!("der Probentext ist kein TOML, das der Leser versteht: {fehler}")
    });
    pruefen(datei)
}

/// Die Beschriftungen des Profils an dieser Stelle, in der Reihenfolge der
/// Datei.
fn beschriftungen(profile: &Profile, stelle: usize) -> Vec<&str> {
    profile
        .iter()
        .nth(stelle)
        .expect("das Profil steht nicht in der Liste")
        .zeilen()
        .iter()
        .map(|zeile| zeile.beschriftung())
        .collect()
}

// ---------------------------------------------------------------------------
// Die Rundreise ueber alle vier Bausteine
// ---------------------------------------------------------------------------

/// Eine Datei mit je einer Zeile jeder Sorte kommt als geprueftes Profil an.
///
/// Geprueft wird beides: dass die Zerlegung **die richtige Sorte** waehlt und
/// dass die Angaben darin ankommen — der Unterordner, das wahlfreie Muster,
/// seine Abwesenheit und die Zahl.
#[test]
fn eine_rundreise_ueber_alle_vier_bausteine_liefert_die_erwarteten_werte() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein Speicher"
pfad = 'fusion-workbench/shared/analyses$'
kennzeichen = '^\.fusion-setup$'

  [[profil.zeile]]
  beschriftung = "Datensaetze"
  zaehlung = { muster = '\.md$' }

  [[profil.zeile]]
  beschriftung = "Die juengsten zehn"
  juengste = { ordner = "history", anzahl = 10 }

  [[profil.zeile]]
  beschriftung = "Fassung"
  feld = { datei = '^\.fusion-setup$', feldmuster = '"plugin_version":"([^"]*)"' }

  [[profil.zeile]]
  beschriftung = "Spec liegt vor"
  vorhandensein = { ordner = "planning", muster = '_._spec-' }
"#,
    );

    assert!(meldungen.is_empty(), "unerwartete Meldungen: {meldungen:?}");
    assert_eq!(profile.zahl(), 1);

    let profil = profile.iter().next().expect("das eine Profil fehlt");
    assert_eq!(profil.name(), "Ein Speicher");
    assert!(
        profil
            .pfad()
            .is_some_and(|muster| muster.is_match("/Users/k/krk/fusion-workbench/shared/analyses"))
    );
    assert!(
        profil
            .kennzeichen()
            .is_some_and(|muster| muster.is_match(".fusion-setup"))
    );
    assert_eq!(
        beschriftungen(&profile, 0),
        [
            "Datensaetze",
            "Die juengsten zehn",
            "Fassung",
            "Spec liegt vor"
        ]
    );

    let zeilen = profil.zeilen();

    match zeilen[0].baustein().expect("die Zaehlung fehlt") {
        Baustein::Zaehlung { ort, muster } => {
            assert!(
                ort.teile().is_empty(),
                "ohne Angabe gilt der erkannte Ordner"
            );
            let muster = muster.as_ref().expect("das Muster fehlt");
            assert!(muster.is_match("260824-0613_o_spec.md"));
            assert!(!muster.is_match("notiz.txt"));
        }
        anderer => panic!("die erste Zeile traegt keine Zaehlung: {anderer:?}"),
    }

    match zeilen[1].baustein().expect("die juengsten fehlen") {
        Baustein::Juengste {
            ort,
            muster,
            anzahl,
            zeigt,
        } => {
            assert_eq!(ort.teile(), ["history"]);
            assert!(muster.is_none(), "ohne Muster zaehlen alle Eintraege");
            assert_eq!(*anzahl, 10);
            assert_eq!(
                *zeigt,
                Anzeige::Titel,
                "ohne den Schluessel `zeigt` stehen Titel da"
            );
        }
        anderer => panic!("die zweite Zeile traegt nicht die juengsten: {anderer:?}"),
    }

    match zeilen[2].baustein().expect("das Feld fehlt") {
        Baustein::Feld {
            ort,
            datei,
            feldmuster,
        } => {
            assert!(ort.teile().is_empty());
            assert!(datei.is_match(".fusion-setup"));
            let treffer = feldmuster
                .captures(r#"{"plugin_version":"5.3.1","setup_at":"260801"}"#)
                .expect("das Feldmuster greift nicht");
            assert_eq!(&treffer[1], "5.3.1");
        }
        anderer => panic!("die dritte Zeile traegt kein Feld: {anderer:?}"),
    }

    match zeilen[3].baustein().expect("das Vorhandensein fehlt") {
        Baustein::Vorhandensein { ort, muster } => {
            assert_eq!(ort.teile(), ["planning"]);
            assert!(muster.is_match("260824-0613_o_spec-vorschau.md"));
            assert!(!muster.is_match("260824-0640_o_plan-vorschau.md"));
        }
        anderer => panic!("die vierte Zeile traegt kein Vorhandensein: {anderer:?}"),
    }
}

// ---------------------------------------------------------------------------
// Die Abweisungen des Pruefschritts
// ---------------------------------------------------------------------------

/// C2.7: Ein unuebersetzbares Pfadmuster schaltet nur sein eigenes Profil ab.
#[test]
fn ein_unuebersetzbares_pfadmuster_nimmt_nur_sein_eigenes_profil_weg() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Das gute Profil davor"
pfad = 'davor$'

[[profil]]
name = "Das kaputte"
pfad = '(unvollstaendig'

  [[profil.zeile]]
  beschriftung = "Datensaetze"
  zaehlung = { }

[[profil]]
name = "Das gute Profil danach"
kennzeichen = 'danach'
"#,
    );

    let namen: Vec<&str> = profile.iter().map(|profil| profil.name()).collect();
    assert_eq!(namen, ["Das gute Profil davor", "Das gute Profil danach"]);
    assert_eq!(meldungen.len(), 1, "genau eine Meldung: {meldungen:?}");
    let meldung = &meldungen[0];
    assert!(meldung.contains("Das kaputte"), "{meldung}");
    assert!(meldung.contains("Pfadmuster"), "{meldung}");
    assert!(
        !meldung.contains('\n'),
        "die Meldung geht in eine Statuszeile: {meldung}"
    );
}

/// Ein Profil ohne Pfadmuster und ohne Kennzeichen koennte nie treffen und
/// faellt weg. Die uebrigen bleiben stehen.
#[test]
fn ein_profil_ohne_erkennung_faellt_weg_und_die_uebrigen_bleiben() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ohne Erkennung"

  [[profil.zeile]]
  beschriftung = "Datensaetze"
  zaehlung = { }

[[profil]]
name = "Mit Erkennung"
kennzeichen = '^_._circle\.md$'
"#,
    );

    let namen: Vec<&str> = profile.iter().map(|profil| profil.name()).collect();
    assert_eq!(namen, ["Mit Erkennung"]);
    assert_eq!(meldungen.len(), 1, "{meldungen:?}");
    assert!(
        meldungen[0].contains("Ohne Erkennung"),
        "{:?}",
        meldungen[0]
    );
}

/// C3.10: Ein Feldmuster mit mehr als einer Fanggruppe nimmt der Zeile ihren
/// Baustein und laesst ihre Beschriftung stehen.
///
/// Geprueft werden beide Abweichungen von der einen Gruppe: zwei Gruppen und
/// keine. Die Zeilen darum bleiben unberuehrt (C3.12).
#[test]
fn ein_feldmuster_ohne_genau_eine_fanggruppe_nimmt_der_zeile_ihren_baustein() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Die Wurzel"
kennzeichen = '^\.fusion-setup$'

  [[profil.zeile]]
  beschriftung = "Davor"
  zaehlung = { }

  [[profil.zeile]]
  beschriftung = "Zwei Gruppen"
  feld = { datei = 'x', feldmuster = '(a)(b)' }

  [[profil.zeile]]
  beschriftung = "Keine Gruppe"
  feld = { datei = 'x', feldmuster = 'a(?:b)' }

  [[profil.zeile]]
  beschriftung = "Danach"
  zaehlung = { }
"#,
    );

    assert_eq!(
        beschriftungen(&profile, 0),
        ["Davor", "Zwei Gruppen", "Keine Gruppe", "Danach"],
        "jede Beschriftung bleibt stehen"
    );
    let zeilen = profile.iter().next().expect("das Profil fehlt").zeilen();
    assert!(zeilen[0].baustein().is_some(), "die Zeile davor bleibt");
    assert!(zeilen[1].baustein().is_none(), "zwei Gruppen sind zu viele");
    assert!(zeilen[2].baustein().is_none(), "keine Gruppe ist zu wenig");
    assert!(zeilen[3].baustein().is_some(), "die Zeile danach bleibt");

    assert_eq!(meldungen.len(), 2, "{meldungen:?}");
    for (meldung, beschriftung) in meldungen.iter().zip(["Zwei Gruppen", "Keine Gruppe"]) {
        assert!(meldung.contains("Die Wurzel"), "{meldung}");
        assert!(meldung.contains(beschriftung), "{meldung}");
        assert!(meldung.contains("Fanggruppen"), "{meldung}");
    }
}

/// C3.13, textliche Haelfte: Eine Ortsangabe, die schon am Text aus dem
/// erkannten Ordner herausfuehrt, nimmt der Zeile ihren Baustein.
#[test]
fn eine_ortsangabe_die_herausfuehrt_nimmt_der_zeile_ihren_baustein() {
    for angabe in [
        "/etc",
        "..",
        "planning/../..",
        "planning/",
        "planning//x",
        ".",
    ] {
        let (profile, meldungen) = gepruefte(&format!(
            r#"
[[profil]]
name = "Ein Circle"
kennzeichen = '^_._circle\.md$'

  [[profil.zeile]]
  beschriftung = "Woanders"
  zaehlung = {{ ordner = "{angabe}" }}

  [[profil.zeile]]
  beschriftung = "Hier"
  zaehlung = {{ }}
"#
        ));

        let zeilen = profile.iter().next().expect("das Profil fehlt").zeilen();
        assert_eq!(
            beschriftungen(&profile, 0),
            ["Woanders", "Hier"],
            "die Angabe {angabe:?} nimmt eine Beschriftung weg"
        );
        assert!(
            zeilen[0].baustein().is_none(),
            "die Angabe {angabe:?} kommt durch"
        );
        assert!(
            zeilen[1].baustein().is_some(),
            "die Zeile daneben bleibt unberuehrt"
        );
        assert_eq!(meldungen.len(), 1, "{meldungen:?}");
        assert!(meldungen[0].contains("Ortsangabe"), "{:?}", meldungen[0]);
        assert!(meldungen[0].contains("Woanders"), "{:?}", meldungen[0]);
    }
}

/// Ein **zweiter** Platzhalter nimmt der Zeile ihren Baustein.
///
/// Einer laesst die Form der Kosten aus dem Profil ablesen: ein Lauf ueber den
/// Ordner vor ihm, dann einer je Treffer. Ein zweiter vervielfachte sie um eine
/// Zahl, die erst am Bestand feststuende.
#[test]
fn eine_ortsangabe_mit_zwei_platzhaltern_nimmt_der_zeile_ihren_baustein() {
    for angabe in ["*/*", "*/issues/*", "circles/*/*/planning"] {
        let (profile, meldungen) = gepruefte(&format!(
            r#"
[[profil]]
name = "Ein Speicher"
kennzeichen = '^_._circle\.md$'

  [[profil.zeile]]
  beschriftung = "Zweimal offen"
  zaehlung = {{ ordner = "{angabe}" }}

  [[profil.zeile]]
  beschriftung = "Einmal offen"
  zaehlung = {{ ordner = "*" }}
"#
        ));

        let zeilen = profile.iter().next().expect("das Profil fehlt").zeilen();
        assert_eq!(
            beschriftungen(&profile, 0),
            ["Zweimal offen", "Einmal offen"],
            "die Angabe {angabe:?} nimmt eine Beschriftung weg"
        );
        assert!(
            zeilen[0].baustein().is_none(),
            "die Angabe {angabe:?} kommt durch"
        );
        assert!(
            zeilen[1].baustein().is_some(),
            "ein einzelner Platzhalter ist zulaessig und die Zeile daneben bleibt unberuehrt"
        );
        assert_eq!(meldungen.len(), 1, "{meldungen:?}");
        let meldung = &meldungen[0];
        assert!(meldung.contains("Ein Speicher"), "{meldung:?}");
        assert!(meldung.contains("Zweimal offen"), "{meldung:?}");
        assert!(meldung.contains("Platzhalter"), "{meldung:?}");
    }
}

/// `juengste` und `feld` nehmen keinen Platzhalter an, je mit eigener Meldung.
///
/// Die Grenze liegt auf der Naht, die der Modulkopf von `leseprofil::bausteine`
/// ohnehin zieht: zwei Bausteine sehen auf Namen, zwei **lesen** Dateien. Wer
/// eine Datei liest, braucht ihren Pfad, und den traegt ein Lesestand nicht
/// mehr, in dem die Eintraege mehrerer Ordner zusammenliegen.
#[test]
fn juengste_und_feld_nehmen_keinen_platzhalter_an() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein Speicher"
kennzeichen = '^_._circle\.md$'

  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { ordner = "*/history", anzahl = 3 }

  [[profil.zeile]]
  beschriftung = "Fassung"
  feld = { ordner = "*", datei = '^\.fusion-setup$', feldmuster = '"plugin_version":"([^"]*)"' }

  [[profil.zeile]]
  beschriftung = "Offene Defekte"
  zaehlung = { ordner = "*/issues", muster = '_o_' }

  [[profil.zeile]]
  beschriftung = "Ein Spec darunter"
  vorhandensein = { ordner = "*/planning", muster = '_._spec-' }
"#,
    );

    let zeilen = profile.iter().next().expect("das Profil fehlt").zeilen();
    assert_eq!(
        beschriftungen(&profile, 0),
        [
            "Die juengsten drei",
            "Fassung",
            "Offene Defekte",
            "Ein Spec darunter"
        ],
        "eine abgewiesene Zeile behaelt ihre Beschriftung"
    );
    assert!(zeilen[0].baustein().is_none(), "juengste kommt durch");
    assert!(zeilen[1].baustein().is_none(), "feld kommt durch");
    assert!(
        zeilen[2].baustein().is_some() && zeilen[3].baustein().is_some(),
        "die zwei Bausteine, die auf Namen sehen, nehmen den Platzhalter an"
    );

    // Der Tischname steht in Anfuehrungszeichen, sonst traefe „juengste" auch
    // auf die Beschriftung „Die juengsten drei" und die Probe belegte nichts.
    assert_eq!(meldungen.len(), 2, "{meldungen:?}");
    assert!(
        meldungen[0].contains("\u{201e}juengste\u{201c}"),
        "{:?}",
        meldungen[0]
    );
    assert!(
        meldungen[0].contains("Die juengsten drei"),
        "{:?}",
        meldungen[0]
    );
    assert!(
        meldungen[1].contains("\u{201e}feld\u{201c}"),
        "{:?}",
        meldungen[1]
    );
    assert!(meldungen[1].contains("Fassung"), "{:?}", meldungen[1]);
}

// ---------------------------------------------------------------------------
// Kappen statt Abweisen, und die leere Datei
// ---------------------------------------------------------------------------

/// C6.3: Eine Zahl ueber der Grenze wird gekappt und **nicht** abgewiesen.
///
/// Ohne Meldung: die Angabe ist nicht falsch, sie verlangt nur mehr, als die
/// Zusammenfassung hergibt.
#[test]
fn eine_anzahl_ueber_der_grenze_wird_gekappt_und_nicht_abgewiesen() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein Speicher"
pfad = 'history$'

  [[profil.zeile]]
  beschriftung = "Die juengsten"
  juengste = { anzahl = 25 }

  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { anzahl = 3 }
"#,
    );

    assert!(
        meldungen.is_empty(),
        "das Kappen meldet nichts: {meldungen:?}"
    );
    let zeilen = profile.iter().next().expect("das Profil fehlt").zeilen();
    let anzahlen: Vec<u8> = zeilen
        .iter()
        .map(
            |zeile| match zeile.baustein().expect("der Baustein fehlt") {
                Baustein::Juengste { anzahl, .. } => *anzahl,
                anderer => panic!("die Zeile traegt nicht die juengsten: {anderer:?}"),
            },
        )
        .collect();
    assert_eq!(anzahlen, [HOECHSTENS_JUENGSTE, 3]);
}

/// C3: Eine Zeile traegt genau einen Baustein, und keiner wie zwei sind ein
/// Grund mit Meldung.
///
/// **Zwei Tische wurden bis zum 260824 schweigend angenommen**, wobei der in
/// der Aufzaehlung obere gewann und der untere wegfiel
/// (`issues/260824-1216_*_zwei-bausteintische-…`). Geprueft wird deshalb
/// beides: dass die Zeile ihren Baustein verliert und dass die Meldung sagt,
/// welche Tische sie gefunden hat.
#[test]
fn eine_zeile_mit_zwei_bausteinen_oder_ohne_einen_verliert_ihren_baustein() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein Speicher"
pfad = 'analyses$'

  [[profil.zeile]]
  beschriftung = "Beides"
  zaehlung = { }
  vorhandensein = { muster = 'y' }

  [[profil.zeile]]
  beschriftung = "Gar nichts"

  [[profil.zeile]]
  beschriftung = "Nur eines"
  zaehlung = { }
"#,
    );

    assert_eq!(profile.zahl(), 1, "das Profil bleibt stehen");
    let zeilen = profile.iter().next().expect("das Profil fehlt").zeilen();
    assert_eq!(
        beschriftungen(&profile, 0),
        ["Beides", "Gar nichts", "Nur eines"],
        "jede abgewiesene Zeile behaelt ihre Beschriftung (C3.12)"
    );
    assert!(zeilen[0].baustein().is_none(), "zwei Tische sind kein Wert");
    assert!(zeilen[1].baustein().is_none(), "kein Tisch ist kein Wert");
    assert!(zeilen[2].baustein().is_some(), "genau einer traegt");

    assert_eq!(meldungen.len(), 2, "{meldungen:?}");
    for (meldung, erwartet) in meldungen.iter().zip(["Beides", "Gar nichts"]) {
        assert!(
            meldung.contains("Ein Speicher") && meldung.contains(erwartet),
            "die Meldung nennt Profil und Beschriftung nicht: {meldung}"
        );
    }
    assert!(
        meldungen[0].contains("zaehlung, vorhandensein"),
        "die Meldung nennt die zwei gefundenen Tische nicht: {}",
        meldungen[0]
    );
    assert!(
        meldungen[1].contains("zaehlung, juengste, feld, vorhandensein"),
        "die Meldung zaehlt die vier moeglichen Tische nicht auf: {}",
        meldungen[1]
    );
}

/// Ein verschriebener Schluessel kostet die ganze Datei, und die Meldung nennt
/// ihn.
///
/// **Die Reichweite ist die weiteste der drei** und nach C1.6 zulaessig: die
/// Datei gilt als beschaedigt, wird beiseitegelegt, und KRK arbeitet ohne
/// Profile weiter. Was bis zum 260824 daneben fehlte, war der Gegenstand: die
/// unmarkierte Auswahl verwarf die Meldung des Tisches und sagte allein, dass
/// keine Variante gepasst habe
/// (`issues/260824-1217_*_ein-tippfehler-in-einem-bausteintisch-…`). Diese
/// Probe haelt fest, dass jede der vier Eingaben ihren eigenen Schluessel
/// nennt: ein verschriebenes Feld im Bausteintisch, ein verschriebener
/// Tischname, ein zusaetzlicher Schluessel neben der Beschriftung und einer auf
/// der obersten Ebene. Bis zum 260824 nannte der Satz vier und die Schleife
/// trug drei; der vierte ist nachgetragen und nicht die Zahl gesenkt worden,
/// denn die oberste Ebene ist die vierte der sechs Stellen mit
/// `deny_unknown_fields` und war ungemessen.
#[test]
fn ein_verschriebener_schluessel_nennt_sich_in_der_meldung() {
    let vorspann = "[[profil]]\nname = \"Ein Speicher\"\npfad = 'analyses$'\n\n[[profil.zeile]]\n  beschriftung = \"Eine Zeile\"\n";
    let ganz = format!("{vorspann}  zaehlung = {{ }}\n");
    for (text, gesucht) in [
        // Ein verschriebenes Feld *im* Bausteintisch.
        (
            format!("{vorspann}  zaehlung = {{ mustre = 'y' }}\n"),
            "mustre",
        ),
        // Ein verschriebener Tischname.
        (format!("{vorspann}  zaehlungg = {{ }}\n"), "zaehlungg"),
        // Ein zusaetzlicher Schluessel *neben* der Beschriftung.
        (
            format!("{vorspann}  zaehlung = {{ }}\n  beschreibung = \"zu viel\"\n"),
            "beschreibung",
        ),
        // Ein unbekannter Schluessel auf der obersten Ebene, neben
        // `[[profil]]`. Er faellt an `Profildatei`s `deny_unknown_fields` und
        // nicht an einer der drei Ebenen darunter.
        (format!("fassung = 2\n{ganz}"), "fassung"),
    ] {
        let fehler = toml::from_str::<Profildatei>(&text)
            .expect_err("der Text kommt durch, obwohl er einen falschen Schluessel traegt");
        assert!(
            fehler.to_string().contains(gesucht),
            "die Meldung nennt {gesucht:?} nicht: {fehler}"
        );
    }
}

/// Ein dritter Wert fuer `zeigt` kostet die ganze Datei, und die Meldung nennt
/// den Schluessel und die zwei erwarteten Namen.
///
/// **Dieselbe Reichweite wie ein verschriebener Bausteintisch**, und aus
/// demselben Grund: `zeigt = "titelchen"` ist keine Angabe, die mehr verlangt,
/// als die Zusammenfassung hergibt — dann wuerde gekappt wie bei `anzahl` —,
/// sondern ein Vertipper. Ihn still auf „titel" zu bringen hiesse, dem Nutzer
/// etwas anderes zu zeigen, als er geschrieben hat.
///
/// Die zwei erwarteten Namen stehen in der Meldung, weil `serde` sie aus der
/// Aufzaehlung nimmt; der Schluessel steht darin, weil `toml` die Quellzeile
/// mitliefert. Beides ist die Auskunft, die der Nutzer braucht, um die eine
/// Stelle zu finden — die Datei liegt danach beiseite, und KRK arbeitet ohne
/// jedes Profil weiter (C1.6).
#[test]
fn ein_dritter_wert_fuer_zeigt_kostet_die_ganze_datei() {
    let vorspann = "[[profil]]\nname = \"Ein Speicher\"\npfad = 'analyses$'\n\n[[profil.zeile]]\n  beschriftung = \"Eine Zeile\"\n";
    for wert in ["titelchen", "Datum", ""] {
        let text = format!("{vorspann}  juengste = {{ anzahl = 1, zeigt = \"{wert}\" }}\n");
        let fehler = toml::from_str::<Profildatei>(&text)
            .expect_err("der Wert {wert:?} kommt durch, obwohl es ihn nicht gibt");
        let meldung = fehler.to_string();
        for gesucht in ["zeigt", "titel", "datum"] {
            assert!(
                meldung.contains(gesucht),
                "die Meldung zu {wert:?} nennt {gesucht:?} nicht: {meldung}"
            );
        }
    }

    // Die zwei, die es gibt, kommen durch, und der Schluessel darf fehlen.
    for zeile in [
        "  juengste = { anzahl = 1, zeigt = \"titel\" }\n",
        "  juengste = { anzahl = 1, zeigt = \"datum\" }\n",
        "  juengste = { anzahl = 1 }\n",
    ] {
        let (profile, meldungen) = gepruefte(&format!("{vorspann}{zeile}"));
        assert!(meldungen.is_empty(), "{zeile:?} meldet: {meldungen:?}");
        assert_eq!(profile.zahl(), 1, "{zeile:?} laesst kein Profil uebrig");
    }
}

/// C1.5: Eine Datei ohne einen einzigen `[[profil]]`-Block ist gueltig und
/// heisst „keine Profile".
///
/// Weder Meldung noch Profil, und das gilt auch fuer eine Datei, die nur noch
/// aus ihren Kommentarzeilen besteht: der Nutzer, der sie leerraeumt, meint
/// „keine Profile" und keinen Schaden (C1.4).
#[test]
fn eine_datei_ohne_profilblock_liefert_keine_profile_und_keine_meldung() {
    for text in ["", "# nur ein Kommentar\n\n# und noch einer\n"] {
        let (profile, meldungen) = gepruefte(text);
        assert_eq!(profile.zahl(), 0, "der Text {text:?} liefert ein Profil");
        assert!(
            meldungen.is_empty(),
            "der Text {text:?} meldet: {meldungen:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Die Erkennung, in zwei Durchgaengen
// ---------------------------------------------------------------------------

/// Eintraege eines Ordners, von Hand gebaut.
///
/// Die Erkennung sieht allein auf [`Eintrag::name`]; Groesse, Zeitpunkt und Typ
/// stehen deshalb fest und tragen keine Aussage. Genau das ist der Grund, aus
/// dem diese Proben ohne Dateisystem auskommen.
fn bestand(namen: &[&str]) -> Vec<Eintrag> {
    namen
        .iter()
        .map(|name| Eintrag::neu((*name).to_owned(), 0, SystemTime::UNIX_EPOCH, Typ::Datei))
        .collect()
}

/// Ein Abschluss ueber einen festen Bestand, der seine Rufe mitzaehlt.
///
/// Die Zaehlung ist die Probe selbst und kein Beiwerk: der erste Durchgang
/// darf den Abschluss nicht rufen, und dass er es nicht tut, ist an nichts
/// anderem abzulesen.
fn abschluss<'e>(
    eintraege: &'e [Eintrag],
    gerufen: &'e Cell<u32>,
) -> impl Fn() -> Option<&'e [Eintrag]> + 'e {
    move || {
        gerufen.set(gerufen.get() + 1);
        Some(eintraege)
    }
}

/// Der Name des erkannten Profils, oder `None`.
fn erkannt(
    profile: &Profile,
    pfad: &str,
    eintraege: &[Eintrag],
    gerufen: &Cell<u32>,
) -> Option<String> {
    erkennen(profile, Path::new(pfad), &abschluss(eintraege, gerufen))
        .map(|profil| profil.name().to_owned())
}

/// C2.1: Ein Pfadmuster trifft seinen Ordner und den daneben nicht.
///
/// Der zweite Ordner liegt im selben Speicher und unterscheidet sich allein im
/// letzten Namensbestandteil; das Muster ist am Ende verankert und trennt die
/// zwei deshalb.
#[test]
fn ein_pfadmuster_trifft_seinen_ordner_und_den_daneben_nicht() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein Speicher"
pfad = 'fusion-workbench/shared/analyses$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    let gerufen = Cell::new(0);
    assert_eq!(
        erkannt(
            &profile,
            "/Users/k/krk/fusion-workbench/shared/analyses",
            &[],
            &gerufen
        )
        .as_deref(),
        Some("Ein Speicher")
    );
    assert_eq!(
        erkannt(
            &profile,
            "/Users/k/krk/fusion-workbench/shared/history",
            &[],
            &gerufen
        ),
        None,
        "ohne Treffer bleibt es bei der Metadatenanzeige"
    );
}

/// C2.2: Von zwei passenden Pfadmustern gewinnt das obere. Vertauscht der
/// Nutzer die Bloecke, gewinnt das andere.
///
/// Beide Muster treffen denselben Pfad, also entscheidet allein die
/// Reihenfolge der Datei (Festlegung A1).
#[test]
fn von_zwei_passenden_pfadmustern_gewinnt_das_obere() {
    let weit = "[[profil]]\nname = \"Weit\"\npfad = 'analyses$'\n";
    let eng = "[[profil]]\nname = \"Eng\"\npfad = 'fusion-workbench/shared/analyses$'\n";
    let pfad = "/Users/k/krk/fusion-workbench/shared/analyses";

    for (datei, erwartet) in [
        (format!("{weit}\n{eng}"), "Weit"),
        (format!("{eng}\n{weit}"), "Eng"),
    ] {
        let (profile, meldungen) = gepruefte(&datei);
        assert!(meldungen.is_empty(), "{meldungen:?}");
        assert_eq!(profile.zahl(), 2);

        let gerufen = Cell::new(0);
        assert_eq!(
            erkannt(&profile, pfad, &[], &gerufen).as_deref(),
            Some(erwartet),
            "die Reihenfolge der Datei entscheidet nicht"
        );
    }
}

/// C2.3: Das Pfadmuster eines **spaeteren** Profils schlaegt die
/// Kennzeichendatei eines **frueheren**.
///
/// Das ist keine Sonderregel, sondern die Folge davon, dass der erste
/// Durchgang ganz vorbei ist, bevor der zweite beginnt.
#[test]
fn ein_spaeteres_pfadmuster_schlaegt_ein_frueheres_kennzeichen() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Das fruehere Kennzeichen"
kennzeichen = '^_._circle\.md$'

[[profil]]
name = "Das spaetere Pfadmuster"
pfad = 'circles/[^/]+$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    let eintraege = bestand(&["_t_circle.md", "planning", "history"]);
    let gerufen = Cell::new(0);
    assert_eq!(
        erkannt(
            &profile,
            "/Users/k/krk/fusion-workbench/circles/260823-2208-vorschau",
            &eintraege,
            &gerufen
        )
        .as_deref(),
        Some("Das spaetere Pfadmuster")
    );
    assert_eq!(
        gerufen.get(),
        0,
        "das getroffene Pfadmuster kostet keinen Verzeichnisleselauf"
    );
}

/// C2.4: Die Kennzeichendatei `^_._circle\.md$` trifft bei jedem der sechs
/// Zustandsmarker des Vokabulars.
///
/// Kein Pfadmuster steht davor, also entscheidet der zweite Durchgang, und die
/// Erkennung des einzelnen Circles haengt nicht am Zustand seiner Runde.
#[test]
fn das_kennzeichen_eines_circles_trifft_bei_jedem_der_sechs_marker() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein einzelner Circle"
kennzeichen = '^_._circle\.md$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    for marker in ['a', 't', 'c', 'b', 's', 'd'] {
        let eintraege = bestand(&[&format!("_{marker}_circle.md"), "planning"]);
        let gerufen = Cell::new(0);
        assert_eq!(
            erkannt(
                &profile,
                "/Users/k/krk/fusion-workbench/circles/260823-2208-vorschau",
                &eintraege,
                &gerufen
            )
            .as_deref(),
            Some("Ein einzelner Circle"),
            "der Marker {marker} wird nicht erkannt"
        );
        assert_eq!(gerufen.get(), 1, "der Bestand wird genau einmal geholt");
    }
}

/// Der erste Durchgang ruft den Abschluss nicht.
///
/// Weder wenn ein Pfadmuster trifft, noch wenn keines der Profile eine
/// Kennzeichendatei nennt: der Verzeichnisleselauf faellt erst am ersten
/// Profil mit Kennzeichendatei an, und die Zahlen aus C6.7 fallen aus dieser
/// Bauart.
#[test]
fn der_erste_durchgang_ruft_den_abschluss_nicht() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Mit Kennzeichen"
kennzeichen = '^\.fusion-setup$'

[[profil]]
name = "Nur Pfad"
pfad = 'shared/analyses$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    let eintraege = bestand(&[".fusion-setup"]);
    let gerufen = Cell::new(0);
    assert_eq!(
        erkannt(
            &profile,
            "/Users/k/krk/fusion-workbench/shared/analyses",
            &eintraege,
            &gerufen
        )
        .as_deref(),
        Some("Nur Pfad")
    );
    assert_eq!(
        gerufen.get(),
        0,
        "ein Pfadmustertreffer holt die Eintraege nicht"
    );

    let (nur_pfade, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Nur Pfad"
pfad = 'nirgends$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");
    let gerufen = Cell::new(0);
    assert_eq!(
        erkannt(&nur_pfade, "/Users/k/krk", &eintraege, &gerufen),
        None
    );
    assert_eq!(
        gerufen.get(),
        0,
        "ohne Kennzeichendatei in der Datei wird nichts gelesen"
    );
}

/// Ein Profil, das beides nennt, nimmt an beiden Durchgaengen teil.
///
/// Der erste Durchgang uebergeht es nicht, weil es eine Kennzeichendatei hat,
/// und der zweite nicht, weil es ein Pfadmuster hat.
#[test]
fn ein_profil_mit_beidem_nimmt_an_beiden_durchgaengen_teil() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Beides"
pfad = 'shared/analyses$'
kennzeichen = '^\.fusion-setup$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    let gerufen = Cell::new(0);
    assert_eq!(
        erkannt(
            &profile,
            "/Users/k/krk/fusion-workbench/shared/analyses",
            &[],
            &gerufen
        )
        .as_deref(),
        Some("Beides"),
        "der erste Durchgang uebergeht es nicht"
    );
    assert_eq!(gerufen.get(), 0);

    let eintraege = bestand(&[".fusion-setup"]);
    let gerufen = Cell::new(0);
    assert_eq!(
        erkannt(
            &profile,
            "/Users/k/krk/fusion-workbench",
            &eintraege,
            &gerufen
        )
        .as_deref(),
        Some("Beides"),
        "der zweite Durchgang uebergeht es nicht"
    );
    assert_eq!(gerufen.get(), 1);
}

/// Ohne Eintraege trifft keine Kennzeichendatei, und die Antwort ist
/// unentschieden und nicht negativ.
///
/// `None` aus dem Abschluss heisst „die Eintraege stehen nicht zur
/// Verfuegung", nicht „der Ordner ist leer": ein leerer Ordner liefert einen
/// leeren Ausschnitt und wird hier danebengestellt.
#[test]
fn ohne_eintraege_trifft_keine_kennzeichendatei() {
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Ein einzelner Circle"
kennzeichen = '^_._circle\.md$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    let gerufen = Cell::new(0);
    let ohne = || {
        gerufen.set(gerufen.get() + 1);
        None
    };
    assert!(erkennen(&profile, Path::new("/Users/k/krk"), &ohne).is_none());
    assert_eq!(gerufen.get(), 1);

    let leer = Cell::new(0);
    assert_eq!(erkannt(&profile, "/Users/k/krk", &[], &leer), None);
    assert_eq!(leer.get(), 1, "auch der leere Ordner wird einmal geholt");
}

// ---------------------------------------------------------------------------
// Die vier Bausteine am Pruefordner
// ---------------------------------------------------------------------------

/// Der Bestand, gegen den die Bausteinproben rechnen.
///
/// **Ausdruecklich nicht die echte Werkbank.** Ihre Zahlen aendern sich mit
/// jeder Sitzung, und eine Probe darauf waere morgen rot, ohne dass jemand
/// etwas kaputtgemacht haette; der Plan sagt es unter `## Testing Strategy`. Der
/// Ordner hier traegt dieselbe **Gestalt** und einen Bestand bekannter Groesse:
///
/// ```text
/// <wurzel>/            _t_circle.md, .fusion-setup, README (kein Text)
///   planning/          zwei Datensaetze, einer davon ein Spec
///   decisions/         drei Datensaetze
///   issues/            zwei offene, ein geschlossener
///   history/           vier Verlaeufe mit gestaffeltem Aenderungsdatum
///   leer/              nichts
/// ```
fn werkbankgestalt(zweck: &str) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);

    ordner.datei(
        "_t_circle.md",
        "# Circle: eine Runde\n\n## Directive\n\nDas Vorschaufenster beantwortet,\nwas an einem Ort liegt.\n\n## Grounding\n",
    );
    ordner.datei(
        ".fusion-setup",
        r#"{"setup_at":"260801-0900","setup_pwd":"/Users/k/krk","plugin_version":"5.3.1"}"#,
    );
    // Kein Text: die Bausteine, die Dateien lesen, muessen daran vorbeikommen.
    ordner.datei("README", [0xff, 0xfe, 0x00, 0x01]);

    let planning = ordner.ordner("planning");
    schreiben(
        &planning,
        "260824-0613_o_spec-vorschau.md",
        "# Spec: die Vorschau\n",
    );
    schreiben(
        &planning,
        "260824-0640_o_plan-vorschau.md",
        "# Plan: die Vorschau\n",
    );

    let decisions = ordner.ordner("decisions");
    for (name, inhalt) in [
        ("260823-2208_a_erste-frage.md", "#   Erste Frage?\n\nText\n"),
        ("260824-0541_a_zweite-frage.md", "# Zweite Frage?\n"),
        ("260824-0600_a_dritte-frage.md", "# Dritte Frage?\n"),
    ] {
        schreiben(&decisions, name, inhalt);
    }

    let issues = ordner.ordner("issues");
    schreiben(&issues, "260824-0955_o_ein offener Defekt.md", "");
    schreiben(
        &issues,
        "260824-1014_o_ein zweiter offener Defekt.md",
        "260824-1014 ein Defekt ohne Doppelkreuz\n---\nBeschreibung\n",
    );
    schreiben(
        &issues,
        "260824-0600_c_ein geschlossener.md",
        "geschlossen\n",
    );

    let history = ordner.ordner("history");
    for (nummer, name) in [
        "260824-0530-die-aelteste.md",
        "260824-0919-die-zweitaelteste.md",
        "260824-1042-die-zweitjuengste.md",
        "260824-1101-die-juengste.md",
    ]
    .into_iter()
    .enumerate()
    {
        let pfad = schreiben(&history, name, &format!("# Verlauf {nummer}\n"));
        geaendert_setzen(&pfad, 1_700_000_000 + nummer as u64 * 60);
    }

    ordner.ordner("leer");
    ordner
}

/// Legt eine Datei in einem Unterordner an und liefert ihren Pfad.
fn schreiben(ordner: &Path, name: &str, inhalt: &str) -> PathBuf {
    let pfad = ordner.join(name);
    std::fs::write(&pfad, inhalt).expect("die Probendatei laesst sich nicht schreiben");
    pfad
}

/// Setzt den Aenderungszeitpunkt einer Datei auf eine feste Sekunde.
///
/// Ohne ihn haengt die Reihenfolge der juengsten N daran, wie schnell die Probe
/// laeuft: vier Dateien nacheinander geschrieben tragen auf einem schnellen
/// Dateisystem denselben Zeitpunkt, und die Probe pruefte dann den
/// Zweitschluessel statt der Sortierung.
fn geaendert_setzen(pfad: &Path, seit_epoche: u64) {
    let zeitpunkt = SystemTime::UNIX_EPOCH + Duration::from_secs(seit_epoche);
    let datei = std::fs::File::options()
        .write(true)
        .open(pfad)
        .expect("die Probendatei laesst sich nicht oeffnen");
    datei
        .set_times(FileTimes::new().set_modified(zeitpunkt))
        .expect("der Aenderungszeitpunkt laesst sich nicht setzen");
}

/// Die Zusammenfassung eines Ordners gegen ein von Hand geschriebenes Profil.
fn zusammengefasst(text: &str, ordner: &Path) -> Zusammenfassung {
    gezaehlt(text, ordner).0
}

/// Dieselbe Zusammenfassung, mit dem verbrauchten [`Haushalt`] daneben.
///
/// **Der Haushalt ist der Zaehler, den der Lauf ohnehin fuehrt**, und C6.8
/// verlangt genau das: die Zahlen aus C6.1 bis C6.7 sind gezaehlte Aufrufe und
/// keine Millisekunden. Eine Probe, die selbst mitzaehlte — etwa ueber die
/// Zahl der Ordner, die sie angelegt hat —, zaehlte ihre eigene Erwartung und
/// nicht den Lauf.
///
/// Die zwei Einstiege sind kein zweiter Rechenweg: `zusammenfassen` ist
/// `zusammenfassen_gezaehlt` ohne die zweite Haelfte seines Paares.
fn gezaehlt(text: &str, ordner: &Path) -> (Zusammenfassung, Haushalt) {
    let (profile, meldungen) = gepruefte(text);
    assert!(meldungen.is_empty(), "unerwartete Meldungen: {meldungen:?}");
    zusammenfassen_gezaehlt(&profile, ordner).expect("kein Profil greift auf den Pruefordner")
}

/// Die Werte der Zusammenfassung, zu ihren Beschriftungen.
fn werte(zusammenfassung: &Zusammenfassung) -> Vec<(&str, &Wert)> {
    zusammenfassung
        .zeilen()
        .iter()
        .map(|zeile| (zeile.beschriftung(), zeile.wert()))
        .collect()
}

/// Ein Profil, das seinen Ort ueber die Kennzeichendatei eines Circles findet.
fn circleprofil(zeilen: &str) -> String {
    format!("[[profil]]\nname = \"Ein Circle\"\nkennzeichen = '^_._circle\\.md$'\n{zeilen}")
}

/// C3.1 und C3.2: die Zaehlung zaehlt, was ihr Muster erfuellt, und flach.
///
/// Der Unterbaum traegt mehr Datensaetze als die eine Ebene; zaehlte sie tief,
/// stuende hier eine andere Zahl.
#[test]
fn die_zaehlung_zaehlt_mit_muster_ohne_muster_und_flach() {
    let ordner = werkbankgestalt("zaehlung");
    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Offene Defekte"
  zaehlung = { ordner = "issues", muster = '_o_' }

  [[profil.zeile]]
  beschriftung = "Defekte insgesamt"
  zaehlung = { ordner = "issues" }

  [[profil.zeile]]
  beschriftung = "Eintraege der Runde"
  zaehlung = { }

  [[profil.zeile]]
  beschriftung = "Der leere Ordner"
  zaehlung = { ordner = "leer" }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung),
        [
            ("Offene Defekte", &Wert::Zahl(2)),
            ("Defekte insgesamt", &Wert::Zahl(3)),
            // Drei Dateien und fuenf Unterordner, und kein Datensatz darunter.
            ("Eintraege der Runde", &Wert::Zahl(8)),
            // Eine Null ist eine Antwort und kein Fehlschlag.
            ("Der leere Ordner", &Wert::Zahl(0)),
        ]
    );
}

/// C3.3 bis C3.6: die juengsten N, ihre Reihenfolge und ihre Titel.
///
/// Vier Kriterien in einer Probe, weil sie an einem Lauf haengen: die
/// Reihenfolge ist die des Aenderungsdatums (C3.3), ein Datensatz ohne
/// Doppelkreuz liefert trotzdem einen Satz (C3.4), ein Doppelkreuz und die
/// Leerzeichen dahinter fallen weg (C3.5), und eine leere Datei liefert ihren
/// Namen (C3.6).
#[test]
fn die_juengsten_stehen_nach_aenderungsdatum_und_tragen_ihre_titel() {
    let ordner = werkbankgestalt("juengste");
    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { ordner = "history", anzahl = 3 }

  [[profil.zeile]]
  beschriftung = "Die Defekte"
  juengste = { ordner = "issues", muster = '_o_', anzahl = 10 }

  [[profil.zeile]]
  beschriftung = "Die Entscheidungen"
  juengste = { ordner = "decisions", muster = '\.md$', anzahl = 1 }

  [[profil.zeile]]
  beschriftung = "Im leeren Ordner"
  juengste = { ordner = "leer", anzahl = 10 }
"#,
        ),
        ordner.pfad(),
    );
    let werte = werte(&zusammenfassung);

    assert_eq!(
        werte[0].1,
        &Wert::Titel(vec![
            "Verlauf 3".to_owned(),
            "Verlauf 2".to_owned(),
            "Verlauf 1".to_owned(),
        ]),
        "die Reihenfolge ist die des Aenderungsdatums, absteigend"
    );

    let Wert::Titel(defekte) = werte[1].1 else {
        panic!("die Defekte tragen keine Titel: {:?}", werte[1].1)
    };
    assert!(
        defekte.contains(&"260824-1014 ein Defekt ohne Doppelkreuz".to_owned()),
        "ein Datensatz ohne Doppelkreuz liefert seinen Satz: {defekte:?}"
    );
    assert!(
        defekte.contains(&"260824-0955_o_ein offener Defekt.md".to_owned()),
        "die leere Datei liefert ihren Dateinamen: {defekte:?}"
    );

    assert_eq!(
        werte[2].1,
        &Wert::Titel(vec!["Dritte Frage?".to_owned()]),
        "das Doppelkreuz und die Leerzeichen dahinter fallen weg"
    );
    assert_eq!(
        werte[3].1,
        &Wert::Nicht,
        "ein Ordner ohne Kandidaten setzt den Platzhalter"
    );
}

/// `zeigt = "datum"` liefert ein Kalenderdatum und oeffnet dabei keine Datei.
///
/// **Die Zahl ist das erste der Abnahmekriterien**, und sie ist eine Null: der
/// Zeitpunkt steht in `Eintrag::geaendert`, das der Verzeichnisleselauf
/// ohnehin liefert. Die Datumsform ist damit billiger als die Titelform, die
/// fuer dieselbe eine Zeile eine Oeffnung braucht — die zweite Haelfte der
/// Probe zaehlt sie daneben, denn eine Null ohne die Eins daneben belegte
/// nicht, dass hier ueberhaupt etwas zu sparen war.
///
/// **Die erwartete Zeichenkette kommt aus [`ortszeit`] und steht nicht im
/// Quelltext.** Eine feste Zahl darin waere die Zeitzone des Geraets, auf dem
/// die Probe geschrieben wurde; dieselbe Wahl trifft
/// `tests/operation.rs::das_msdos_feld_traegt_die_ortszeit_des_quelldatums`.
/// Die **Form** dagegen steht hier ausgeschrieben, denn sie ist die Zusage.
#[test]
fn zeigt_datum_liefert_ein_kalenderdatum_und_oeffnet_keine_datei() {
    let ordner = werkbankgestalt("zeigt-datum");
    let juengster = 1_700_000_000 + 3 * 60;

    let (zusammenfassung, haushalt) = gezaehlt(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Zuletzt geschrieben"
  juengste = { ordner = "history", anzahl = 1, zeigt = "datum" }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung)[0].1,
        &Wert::Text(kalendertext(juengster)),
        "das Datum des juengsten Eintrags steht nicht da"
    );
    assert_eq!(
        haushalt.oeffnungen(),
        0,
        "die Datumsform oeffnet eine Datei"
    );

    // Die Titelform daneben, einmal ausgeschrieben und einmal weggelassen. Eine
    // Null ohne die Eins daneben belegte nicht, dass hier ueberhaupt etwas zu
    // sparen war, und ein ausgeschriebenes `zeigt = "titel"` muss dasselbe
    // liefern wie eine `readers.toml`, die den Schluessel nicht kennt.
    let titelzeile = |zeigt: &str| {
        gezaehlt(
            &circleprofil(&format!(
                r#"
  [[profil.zeile]]
  beschriftung = "Zuletzt geschrieben"
  juengste = {{ ordner = "history", anzahl = 1{zeigt} }}
"#
            )),
            ordner.pfad(),
        )
    };
    let (ausgeschrieben, mit_titel) = titelzeile(r#", zeigt = "titel""#);
    let (weggelassen, ohne_zeigt) = titelzeile("");

    assert_eq!(
        mit_titel.oeffnungen(),
        1,
        "dieselbe Zeile kostet als Titelform eine Oeffnung"
    );
    assert_eq!(
        werte(&ausgeschrieben),
        werte(&weggelassen),
        "`zeigt = \"titel\"` liefert etwas anderes als eine Zeile ohne den Schluessel"
    );
    assert_eq!(
        mit_titel, ohne_zeigt,
        "`zeigt = \"titel\"` kostet etwas anderes als eine Zeile ohne den Schluessel"
    );
}

/// Die Form ist `JJJJ-MM-TT HH:MM`, und sie haengt an keiner
/// Spracheinstellung.
///
/// Sechzehn Zeichen, vier Trenner an festen Stellen, sonst nur Ziffern. Der
/// Nutzerentscheid vom 260825-1740 nennt genau diese Form: sie ist eindeutig,
/// sortiert sich von selbst und entsteht ohne AppKit. Ohne diese Probe
/// belegte die Probe darueber allein, dass zwei Wege dieselbe Zeichenkette
/// bauen, und nicht, wie sie aussieht.
#[test]
fn die_datumsform_traegt_vier_zahlen_an_festen_stellen() {
    let ordner = werkbankgestalt("datumsform");
    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Zuletzt geschrieben"
  juengste = { ordner = "history", anzahl = 1, zeigt = "datum" }
"#,
        ),
        ordner.pfad(),
    );
    let Wert::Text(datum) = werte(&zusammenfassung)[0].1 else {
        panic!(
            "die Zeile traegt keinen Text: {:?}",
            werte(&zusammenfassung)[0].1
        )
    };

    let zeichen: Vec<char> = datum.chars().collect();
    assert_eq!(
        zeichen.len(),
        16,
        "{datum:?} ist nicht sechzehn Zeichen lang"
    );
    for (stelle, erwartet) in [(4, '-'), (7, '-'), (10, ' '), (13, ':')] {
        assert_eq!(
            zeichen[stelle], erwartet,
            "an Stelle {stelle} steht in {datum:?} nicht {erwartet:?}"
        );
    }
    for stelle in [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15] {
        assert!(
            zeichen[stelle].is_ascii_digit(),
            "an Stelle {stelle} steht in {datum:?} keine Ziffer"
        );
    }
}

/// Ein Ordner, der **nur Ordner** enthaelt, liefert ein Datum und nicht den
/// Platzhalter.
///
/// Das ist die zweite der drei Festlegungen des Nutzerentscheids und die
/// Bedingung dafuer, dass `fusion-workbench/archive` ueberhaupt antworten
/// kann: dort liegen die Archivlaeufe als Ordner. Die Titelform sieht
/// denselben Ordner leer, und das bleibt richtig — sie liest Dateien.
#[test]
fn ein_ordner_aus_lauter_ordnern_liefert_ein_datum_und_keinen_titel() {
    let ordner = Pruefordner::neu("nur-ordner");
    ordner.datei("_t_circle.md", "# Eine Runde\n");
    let archive = ordner.ordner("archive");
    for name in [
        "260819-1613-safe-cleanup-tier-1",
        "260820-2115-safe-cleanup-tier-1",
    ] {
        std::fs::create_dir(archive.join(name)).expect("der Archivlauf laesst sich nicht anlegen");
    }

    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Zuletzt archiviert"
  juengste = { ordner = "archive", anzahl = 1, zeigt = "datum" }

  [[profil.zeile]]
  beschriftung = "Der juengste Lauf"
  juengste = { ordner = "archive", anzahl = 1 }
"#,
        ),
        ordner.pfad(),
    );
    let werte = werte(&zusammenfassung);

    assert!(
        matches!(werte[0].1, Wert::Text(_)),
        "die Datumsform sieht die Ordner nicht: {:?}",
        werte[0].1
    );
    assert_eq!(
        werte[1].1,
        &Wert::Nicht,
        "die Titelform nimmt weiter allein Eintraege vom Typ Datei"
    );
}

/// Mehrere Daten stehen untereinander unter ihrer Beschriftung, ein einzelnes
/// daneben.
///
/// **Ohne eine neue Regel in `als_text`**: die vorhandene entscheidet am
/// Zeilenumbruch, und deshalb ist der Wert [`Wert::Text`] und kein siebter
/// Wert. Die Probe nimmt beide Lagen an einem Lauf ab, denn es ist eine Regel
/// und nicht zwei.
#[test]
fn drei_daten_stehen_untereinander_und_eines_daneben() {
    let ordner = werkbankgestalt("drei-daten");
    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { ordner = "history", anzahl = 3, zeigt = "datum" }

  [[profil.zeile]]
  beschriftung = "Zuletzt geschrieben"
  juengste = { ordner = "history", anzahl = 1, zeigt = "datum" }
"#,
        ),
        ordner.pfad(),
    );

    let drei = [3, 2, 1].map(|nummer| kalendertext(1_700_000_000 + nummer * 60));
    assert_eq!(
        werte(&zusammenfassung)[0].1,
        &Wert::Text(drei.join("\n")),
        "die drei Daten stehen nicht in der Reihenfolge des Aenderungsdatums"
    );

    let text = zusammenfassung.als_text();
    assert!(
        text.contains(&format!(
            "Die juengsten drei:\n    {}\n    {}\n    {}",
            drei[0], drei[1], drei[2]
        )),
        "die drei Daten stehen nicht eingerueckt unter ihrer Beschriftung: {text}"
    );
    assert!(
        text.contains(&format!("Zuletzt geschrieben: {}", drei[0])),
        "das einzelne Datum steht nicht neben seiner Beschriftung: {text}"
    );
}

/// Ein Zeitpunkt, wie ihn die Datumsform schreibt.
///
/// Sie rechnet ueber [`ortszeit`] und nicht ueber eine feste Zahl: die Probe
/// soll in jeder Zone dieselbe Aussage machen wie die Auswertung.
fn kalendertext(seit_epoche: u64) -> String {
    let zeit = ortszeit(SystemTime::UNIX_EPOCH + Duration::from_secs(seit_epoche))
        .expect("der Pruefzeitpunkt laesst sich nicht in Ortszeit umrechnen");
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        zeit.jahr, zeit.monat, zeit.tag, zeit.stunde, zeit.minute
    )
}

/// C3.7 bis C3.9: der Feldbaustein zieht drei Felder aus einer JSON-Zeile und
/// einen Absatz ueber mehrere Zeilen aus einem Markdown-Datensatz.
#[test]
fn das_feld_zieht_die_erste_fanggruppe_des_ersten_treffers() {
    let ordner = werkbankgestalt("feld");
    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Fassung"
  feld = { datei = '^\.fusion-setup$', feldmuster = '"plugin_version":"([^"]*)"' }

  [[profil.zeile]]
  beschriftung = "Projekt"
  feld = { datei = '^\.fusion-setup$', feldmuster = '"setup_pwd":"[^"]*/([^"/]+)"' }

  [[profil.zeile]]
  beschriftung = "Directive"
  # Mit `m`, sonst verankerte `^` am Anfang der ganzen Datei und nicht an dem
  # einer Zeile; der Defekt dazu ist 260824-1136.
  feld = { datei = '^_._circle\.md$', feldmuster = '(?sm)^## Directive\s*\n+(.+?)\n\n' }

  [[profil.zeile]]
  beschriftung = "Es gibt die Datei nicht"
  feld = { datei = '^nirgends$', feldmuster = '(.+)' }

  [[profil.zeile]]
  beschriftung = "Das Muster greift nicht"
  feld = { datei = '^\.fusion-setup$', feldmuster = '"kein_feld":"([^"]*)"' }

  [[profil.zeile]]
  beschriftung = "Die Datei ist kein Text"
  feld = { datei = '^README$', feldmuster = '(.+)' }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung),
        [
            ("Fassung", &Wert::Text("5.3.1".to_owned())),
            ("Projekt", &Wert::Text("krk".to_owned())),
            (
                "Directive",
                &Wert::Text("Das Vorschaufenster beantwortet,\nwas an einem Ort liegt.".to_owned())
            ),
            ("Es gibt die Datei nicht", &Wert::Nicht),
            ("Das Muster greift nicht", &Wert::Nicht),
            ("Die Datei ist kein Text", &Wert::Nicht),
        ]
    );
}

/// C3.11 und C3.12: das Vorhandensein antwortet ja und nein, und eine beim
/// Laden abgewiesene Zeile behaelt ihre Beschriftung.
#[test]
fn das_vorhandensein_antwortet_ja_und_nein_und_die_abgewiesene_zeile_bleibt() {
    let ordner = werkbankgestalt("vorhandensein");
    let (profile, meldungen) = gepruefte(&circleprofil(
        r#"
  [[profil.zeile]]
  beschriftung = "Spec liegt vor"
  vorhandensein = { ordner = "planning", muster = '_._spec-' }

  [[profil.zeile]]
  beschriftung = "Durchsicht liegt vor"
  vorhandensein = { ordner = "planning", muster = '_._review-' }

  [[profil.zeile]]
  beschriftung = "Es gibt den Ordner nicht"
  vorhandensein = { ordner = "reviews", muster = '\.md$' }

  [[profil.zeile]]
  beschriftung = "Beim Laden abgewiesen"
  feld = { datei = 'x', feldmuster = '(a)(b)' }

  [[profil.zeile]]
  beschriftung = "Aktive Runde"
  vorhandensein = { muster = '^_t_circle\.md$' }
"#,
    ));
    assert_eq!(meldungen.len(), 1, "{meldungen:?}");

    let zusammenfassung =
        zusammenfassen(&profile, ordner.pfad()).expect("kein Profil greift auf den Pruefordner");
    assert_eq!(
        werte(&zusammenfassung),
        [
            ("Spec liegt vor", &Wert::Vorhanden(true)),
            ("Durchsicht liegt vor", &Wert::Vorhanden(false)),
            ("Es gibt den Ordner nicht", &Wert::Nicht),
            ("Beim Laden abgewiesen", &Wert::Nicht),
            ("Aktive Runde", &Wert::Vorhanden(true)),
        ],
        "die Zeilen um eine abgewiesene bleiben unberuehrt"
    );
}

/// C3.13, aufgeloeste Haelfte: eine Verknuepfung, die aus dem erkannten Ordner
/// herausfuehrt, wird abgewiesen.
///
/// Die textliche Haelfte sieht dem Namen `hinaus` nichts an; erst der
/// aufgeloeste Pfad entscheidet. Daneben steht eine Verknuepfung, die **im**
/// Ordner bleibt: ohne sie pruefte die Probe nur, dass Verknuepfungen nicht
/// gehen.
#[test]
fn eine_verknuepfung_aus_dem_ordner_heraus_wird_erst_aufgeloest_abgewiesen() {
    let draussen = Pruefordner::neu("draussen");
    schreiben(draussen.pfad(), "fremd.md", "# Fremd\n");

    let ordner = werkbankgestalt("aufgeloest");
    ordner.verknuepfung("hinaus", draussen.pfad());
    ordner.verknuepfung("drinnen", ordner.unter("decisions"));

    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Hinaus"
  zaehlung = { ordner = "hinaus" }

  [[profil.zeile]]
  beschriftung = "Drinnen"
  zaehlung = { ordner = "drinnen" }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung),
        [("Hinaus", &Wert::Nicht), ("Drinnen", &Wert::Zahl(3)),],
        "eine Zusammenfassung liest nie ausserhalb des Ordners, den sie beschreibt"
    );
}

// ---------------------------------------------------------------------------
// Der Platzhalter in der Ortsangabe
// ---------------------------------------------------------------------------

/// Ein Speicher gleichartiger Unterordner, wie ihn `circles/` einer Werkbank
/// darstellt.
///
/// Die zwei Auskuenfte, um derentwillen der Platzhalter entstanden ist, liegen
/// hier je eine Ebene tiefer: der Zustandsmarker einer Runde in
/// `<runde>/_X_circle.md`, ihre offenen Defekte in `<runde>/issues/*_o_*.md`.
///
/// ```text
/// <wurzel>/          README.md, hinaus -> <fremd>, drinnen -> 260801-erste
///   260801-erste/    _c_circle.md ; issues/ zwei offene, einer geschlossen
///   260802-zweite/   _b_circle.md ; issues/ ein offener
///   260803-dritte/   _t_circle.md ; kein issues-Ordner
/// ```
///
/// **Vier Fallen stecken mit Absicht darin.** `260803-dritte` hat keinen
/// Defektspeicher: ein Ordner, den es hinter dem Platzhalter nicht gibt, wird
/// uebergangen. `README.md` ist eine Datei: der Platzhalter greift allein
/// Eintraege vom Typ Ordner.
///
/// Und die zwei Verknuepfungen sind zwei und nicht eine, weil sie **zwei
/// verschiedene Stellen** messen. `hinaus` fuehrt auf einen Ordner derselben
/// Gestalt ausserhalb der Wurzel und wird von der aufgeloesten Pruefung
/// abgewiesen (C3.13, zweite Haelfte). `drinnen` fuehrt auf einen Ordner
/// **innerhalb** der Wurzel; jene Pruefung laesst es durch, und was es
/// uebergeht, ist allein die Bauart des Platzhalters: gegriffen wird, was vom
/// Typ Ordner ist. Ohne diese zweite Verknuepfung bliebe die Probe gruen, wenn
/// jemand die Typfrage streicht.
fn circlespeicher(zweck: &str) -> (Pruefordner, Pruefordner) {
    let fremd = Pruefordner::neu(&format!("{zweck}-fremd"));
    schreiben(fremd.pfad(), "_a_circle.md", "# Eine fremde Runde\n");
    schreiben(
        &fremd.ordner("issues"),
        "260824-0100_o_ein fremder Defekt.md",
        "",
    );

    let ordner = Pruefordner::neu(zweck);
    ordner.datei("README.md", "# Der Speicher\n");
    ordner.verknuepfung("hinaus", fremd.pfad());

    for (name, marker, offene) in [
        ("260801-erste", "_c_", 2),
        ("260802-zweite", "_b_", 1),
        ("260803-dritte", "_t_", 0),
    ] {
        let runde = ordner.ordner(name);
        schreiben(&runde, &format!("{marker}circle.md"), "# Eine Runde\n");
        if offene == 0 {
            // Diese Runde bekommt bewusst keinen Defektspeicher.
            continue;
        }
        let issues = runde.join("issues");
        std::fs::create_dir_all(&issues).expect("der Defektspeicher laesst sich nicht anlegen");
        for nummer in 0..offene {
            schreiben(&issues, &format!("260824-{nummer:02}00_o_offen.md"), "");
        }
        schreiben(&issues, "260824-0900_c_geschlossen.md", "");
    }

    // Erst hier, denn sie zeigt auf einen der Rundenordner darueber.
    ordner.verknuepfung("drinnen", ordner.unter("260801-erste"));

    (ordner, fremd)
}

/// Ein Profil auf dem Speicher, das seinen Ort ueber den Pfad erkennt.
///
/// Ueber den Pfad und nicht ueber eine Kennzeichendatei, damit die Erkennung
/// keinen Leselauf kostet und die gezaehlten Laeufe allein von den Zeilen
/// stammen.
fn speicherprofil(zeilen: &str) -> String {
    format!("[[profil]]\nname = \"Ein Speicher\"\npfad = '.'\n{zeilen}")
}

/// Der Platzhalter legt die Eintraege aller Unterordner zu **einem** Stand
/// zusammen und bucht dafuer **einen** Leselauf.
///
/// Drei Aussagen an einem Lauf, weil sie sich einen Bestand teilen:
///
/// - `ordner = "*"` erreicht die Zustandsmarker, die je eine Ebene tiefer
///   liegen, und `ordner = "*/issues"` die Defekte zwei Ebenen tiefer.
/// - Eine Runde ohne Defektspeicher wird uebergangen und macht die Zeile nicht
///   zum Platzhalterwert: die Antwort ist eine Zahl und nicht `--`.
/// - Der Ordner vor dem Platzhalter wird genau **einmal** gelesen, obwohl ihn
///   drei Zeilen nennen — die zwei Platzhalterzeilen und die Zeile ohne
///   Ortsangabe. Drei Laeufe insgesamt: einer fuer ihn, einer je Sammlung.
#[test]
fn der_platzhalter_legt_die_eintraege_aller_unterordner_zu_einem_stand_zusammen() {
    let (ordner, _fremd) = circlespeicher("platzhalter");

    let (zusammenfassung, haushalt) = gezaehlt(
        &speicherprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Eintraege"
  zaehlung = { }

  [[profil.zeile]]
  beschriftung = "Runden"
  zaehlung = { ordner = "*", muster = '^_._circle\.md$' }

  [[profil.zeile]]
  beschriftung = "Beschraenkt geschlossen"
  zaehlung = { ordner = "*", muster = '^_b_circle\.md$' }

  [[profil.zeile]]
  beschriftung = "Offene Defekte"
  zaehlung = { ordner = "*/issues", muster = '_o_' }

  [[profil.zeile]]
  beschriftung = "Ein Defekt darunter"
  vorhandensein = { ordner = "*/issues", muster = '_o_' }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung),
        [
            // Drei Rundenordner, eine Datei, zwei Verknuepfungen. Die Zaehlung
            // ohne Ortsangabe sieht auf Namen und zaehlt jeden Eintrag mit.
            ("Eintraege", &Wert::Zahl(6)),
            ("Runden", &Wert::Zahl(3)),
            ("Beschraenkt geschlossen", &Wert::Zahl(1)),
            // Zwei aus der ersten Runde, einer aus der zweiten; die dritte hat
            // keinen Defektspeicher und wird uebergangen, ohne die Zeile zum
            // Platzhalterwert zu machen.
            ("Offene Defekte", &Wert::Zahl(3)),
            ("Ein Defekt darunter", &Wert::Vorhanden(true)),
        ]
    );

    assert_eq!(
        haushalt.leselaeufe(),
        3,
        "erwartet sind drei Laeufe: der Speicher selbst, den drei Zeilen nennen, und je einer \
         fuer die zwei verschiedenen Sammlungen"
    );
    assert_eq!(
        haushalt.oeffnungen(),
        0,
        "die zwei Bausteine, die den Platzhalter annehmen, sehen auf Namen und oeffnen nichts"
    );
}

/// Eine Verknuepfung an der Stelle des Platzhalters wird uebergangen (C3.13).
///
/// **Zwei Verknuepfungen, zwei verschiedene Stellen, die es halten.**
///
/// `hinaus` fuehrt aus dem erkannten Ordner heraus, auf einen fremden Ordner
/// derselben Gestalt — einen Zustandsmarker und einen offenen Defekt. Sie faellt
/// an der aufgeloesten Pruefung, die jede Ortsangabe gegen die Wurzel haelt.
///
/// `drinnen` fuehrt auf einen der Rundenordner **innerhalb** der Wurzel. Jene
/// Pruefung laesst sie durch, denn aufgeloest bleibt sie im Ordner; was sie
/// uebergeht, ist allein die Bauart des Platzhalters, der nimmt, was vom Typ
/// Ordner ist. Wer diese Frage streicht, zaehlt die erste Runde ein zweites Mal
/// mit, und dann steht hier 5 statt 3.
///
/// Ohne die zweite Verknuepfung maesse die Probe die Bauart nicht: sie bliebe
/// gruen, weil schon die Aufloesung die erste abweist. Genau so ist es bei der
/// ersten Gegenprobe am 260825 gewesen.
#[test]
fn eine_verknuepfung_an_der_stelle_des_platzhalters_wird_uebergangen() {
    let (ordner, fremd) = circlespeicher("platzhalter-verknuepfung");

    // Ohne diese drei Zeilen belegte die Probe nichts: eine Verknuepfung ins
    // Leere waere auch dann uebergangen, wenn der Platzhalter ihr folgte.
    assert!(
        ordner.pfad().join("hinaus/_a_circle.md").is_file(),
        "die Verknuepfung fuehrt nicht auf den fremden Ordner"
    );
    assert!(
        fremd.pfad().join("issues").is_dir(),
        "der fremde Ordner traegt nicht die Gestalt einer Runde"
    );
    assert!(
        ordner.pfad().join("drinnen/_c_circle.md").is_file(),
        "die zweite Verknuepfung fuehrt nicht auf einen Rundenordner"
    );

    let zusammenfassung = zusammengefasst(
        &speicherprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Runden"
  zaehlung = { ordner = "*", muster = '^_._circle\.md$' }

  [[profil.zeile]]
  beschriftung = "Offene Defekte"
  zaehlung = { ordner = "*/issues", muster = '_o_' }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung),
        [
            ("Runden", &Wert::Zahl(3)),
            ("Offene Defekte", &Wert::Zahl(3))
        ],
        "der Platzhalter ist der Verknuepfung gefolgt und liest ausserhalb des Ordners, ueber \
         den er spricht"
    );
}

/// Eine Sammlung ueber der Eintragsschranke nennt die Zahl der **Treffer** und
/// nicht die der Grenze.
///
/// Das ist derselbe Satz, den die Runde 16 fuer eine abgeschnittene Lesung
/// geschrieben hat, an der Sammlung nachgemessen: „mindestens n, und die Lesung
/// wurde abgebrochen". Naehme er die Grenze statt der Treffer, stuende hier
/// 2.000 statt 1.200 und damit eine falsche Aussage.
///
/// # Warum drei gleich zusammengesetzte Ordner
///
/// In welcher Reihenfolge das Dateisystem die drei Unterordner liefert, ist
/// nicht zugesagt. Traegt jeder von ihnen gleich viele Eintraege und gleich
/// viele Treffer, haengt die Antwort daran nicht: zwei beliebige von ihnen
/// fuellen die Schranke genau aus, der dritte kommt nicht mehr dran, und die
/// Zahl der Treffer ist in jeder Reihenfolge dieselbe.
#[test]
fn eine_sammlung_ueber_der_grenze_nennt_die_treffer_und_nicht_die_grenze() {
    const JE_ORDNER: usize = HOECHSTENS_EINTRAEGE / 2;
    const TREFFER_JE_ORDNER: usize = JE_ORDNER * 3 / 5;

    let ordner = Pruefordner::neu("platzhalter-ueber-der-grenze");
    for name in ["eins", "zwei", "drei"] {
        let runde = ordner.ordner(name);
        for nummer in 0..JE_ORDNER {
            let marker = if nummer < TREFFER_JE_ORDNER {
                "_o_"
            } else {
                "_c_"
            };
            schreiben(&runde, &format!("2608{nummer:04}{marker}datensatz.md"), "");
        }
    }

    let (zusammenfassung, haushalt) = gezaehlt(
        &speicherprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Offene Defekte"
  zaehlung = { ordner = "*", muster = '_o_' }
"#,
        ),
        ordner.pfad(),
    );

    assert_eq!(
        werte(&zusammenfassung),
        [(
            "Offene Defekte",
            &Wert::UeberGrenze((TREFFER_JE_ORDNER * 2) as u64)
        )],
        "die Sammlung nennt nicht die Treffer der zwei gelesenen Ordner"
    );
    assert_eq!(
        haushalt.leselaeufe(),
        2,
        "auch eine Sammlung ueber der Schranke bucht einen Leselauf und keinen je Ordner"
    );
}

/// Die Kopfzeile traegt Name und vollen Pfad des **ausgewaehlten** Ordners
/// (Festlegung A6, C4.2).
///
/// Ausgewaehlt und nicht aufgeloest: unter macOS liegt das Temporaerverzeichnis
/// hinter einer Verknuepfung, und der Nutzer, der `/tmp/...` gewaehlt hat, will
/// `/tmp/...` lesen und nicht `/private/tmp/...`.
#[test]
fn die_kopfzeile_traegt_den_ausgewaehlten_pfad_und_nicht_den_aufgeloesten() {
    let ordner = werkbankgestalt("kopfzeile");
    let zusammenfassung = zusammengefasst(&circleprofil(""), ordner.pfad());

    assert_eq!(
        zusammenfassung.name(),
        ordner
            .pfad()
            .file_name()
            .expect("der Pruefordner hat einen Namen")
            .to_string_lossy()
    );
    assert_eq!(zusammenfassung.pfad(), ordner.pfad());
    assert_eq!(
        zusammenfassung.als_text(),
        format!(
            "Name: {}\nPfad: {}",
            zusammenfassung.name(),
            ordner.pfad().display()
        ),
        "ein Profil ohne Zeilen zeigt allein die Kopfzeile"
    );
}

/// C4.3: der Text setzt einzeilige Werte hinter die Beschriftung und
/// mehrzeilige darunter.
#[test]
fn der_text_setzt_einzeilige_werte_hinter_und_mehrzeilige_unter_die_beschriftung() {
    let zusammenfassung = Zusammenfassung::neu(
        "analyses".to_owned(),
        PathBuf::from("/Users/k/krk/fusion-workbench/shared/analyses"),
        vec![
            Zusammenfassungszeile::neu("Datensaetze".to_owned(), Wert::Zahl(54)),
            // Eine **kleine** Zahl, und darin liegt der Punkt: bei einem Wert
            // nahe der Grenze erriete der Nutzer den Abbruch noch, bei „1"
            // nicht mehr. Der Satz nennt ihn deshalb ausdruecklich, und
            // „mindestens" statt „ueber", weil ein zweiter Treffer hinter dem
            // Abbruch moeglich und nicht gesichert ist.
            Zusammenfassungszeile::neu("Eintraege".to_owned(), Wert::UeberGrenze(1)),
            Zusammenfassungszeile::neu("Spec liegt vor".to_owned(), Wert::Vorhanden(true)),
            Zusammenfassungszeile::neu("Plan liegt vor".to_owned(), Wert::Vorhanden(false)),
            Zusammenfassungszeile::neu("Fassung".to_owned(), Wert::Text("5.3.1".to_owned())),
            Zusammenfassungszeile::neu(
                "Directive".to_owned(),
                Wert::Text("Erste Zeile\nzweite Zeile".to_owned()),
            ),
            Zusammenfassungszeile::neu(
                "Die juengsten zwei".to_owned(),
                Wert::Titel(vec!["Erster Titel".to_owned(), "Zweiter Titel".to_owned()]),
            ),
            Zusammenfassungszeile::neu("Nichts dazu".to_owned(), Wert::Nicht),
        ],
    );

    assert_eq!(
        zusammenfassung.als_text(),
        "Name: analyses\n\
         Pfad: /Users/k/krk/fusion-workbench/shared/analyses\n\
         Datensaetze: 54\n\
         Eintraege: mindestens 1 (Lesung bei 2000 Einträgen abgebrochen)\n\
         Spec liegt vor: ja\n\
         Plan liegt vor: nein\n\
         Fassung: 5.3.1\n\
         Directive:\n\
         \x20   Erste Zeile\n\
         \x20   zweite Zeile\n\
         Die juengsten zwei:\n\
         \x20   Erster Titel\n\
         \x20   Zweiter Titel\n\
         Nichts dazu: --"
    );
}

/// Die drei Anwendungen der einen Regel ueber die Teillesung.
///
/// Ein Ordner ueber [`HOECHSTENS_EINTRAEGE`] wird abgeschnitten gelesen, und
/// dann sagt jeder Baustein nur noch, was die Teilliste entscheidet: die
/// Zaehlung „mindestens n", das Vorhandensein sein „ja" bei Treffer und sonst
/// den Platzhalter, die juengsten N den Platzhalter.
///
/// Alle drei stehen in **einer** Probe, weil sie sich einen Ordner mit gut
/// zweitausend Eintraegen teilen; drei Proben legten ihn dreimal an. Die
/// juengsten N stehen darin zweimal, einmal je Form: die Abbruchregel haengt
/// an der Liste und nicht daran, was der Baustein ueber sie zeigt.
///
/// # Warum das Muster des Treffers auf fast jeden Eintrag passt
///
/// Ob ein **einzelner** Treffer innerhalb der ersten [`HOECHSTENS_EINTRAEGE`]
/// gelesenen Eintraege liegt, entscheidet die Reihenfolge, in der das
/// Dateisystem sie liefert, und die ist nicht zugesagt. Bis zum 260824 liess
/// die Probe deshalb beide Ausgaenge zu und belegte damit nichts
/// (`issues/260824-1218_*_die-probe-zur-teillesung-…`). Ein Muster auf alle
/// 2.001 Dateien haengt dagegen an keiner Reihenfolge: jede Auswahl von 2.000
/// aus 2.001 Eintraegen enthaelt mindestens 1.999 davon.
#[test]
fn eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet() {
    let ordner = Pruefordner::neu("teillesung");
    ordner.datei("_t_circle.md", "# Eine Runde\n");
    let viele = ordner.ordner("viele");
    for nummer in 0..=HOECHSTENS_EINTRAEGE {
        schreiben(&viele, &format!("{nummer:05}.md"), "# Ein Datensatz\n");
    }

    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Eintraege"
  zaehlung = { ordner = "viele" }

  [[profil.zeile]]
  beschriftung = "Ein Treffer darunter"
  vorhandensein = { ordner = "viele", muster = '\.md$' }

  [[profil.zeile]]
  beschriftung = "Etwas, das es nicht gibt"
  vorhandensein = { ordner = "viele", muster = '^nirgends$' }

  [[profil.zeile]]
  beschriftung = "Die juengsten zehn"
  juengste = { ordner = "viele", anzahl = 10 }

  [[profil.zeile]]
  beschriftung = "Die juengsten zehn, als Datum"
  juengste = { ordner = "viele", anzahl = 10, zeigt = "datum" }
"#,
        ),
        ordner.pfad(),
    );
    let werte = werte(&zusammenfassung);

    assert_eq!(
        werte[0].1,
        &Wert::UeberGrenze(HOECHSTENS_EINTRAEGE as u64),
        "die Zaehlung sagt, dass es mehr sind, und keine Zahl"
    );
    assert_eq!(
        werte[1].1,
        &Wert::Vorhanden(true),
        "ein Treffer entscheidet auch in einer Teilliste"
    );
    assert_eq!(
        werte[2].1,
        &Wert::Nicht,
        "ein Nichtfund in einer Teilliste ist kein Nichtvorhandensein"
    );
    assert_eq!(
        werte[3].1,
        &Wert::Nicht,
        "die juengsten zehn einer Teilliste sind nicht die juengsten zehn"
    );
    assert_eq!(
        werte[4].1,
        &Wert::Nicht,
        "auch als Datum sind die juengsten zehn einer Teilliste nicht die juengsten zehn"
    );
}

/// C2.6: Auf eine Datei greift kein Profil, auch nicht bei passendem
/// Pfadmuster.
///
/// **Die Zusage haengt an dieser Stelle und nicht am Aufrufer.** Der erste
/// Erkennungsdurchgang sieht allein auf den Pfadtext und braucht keine
/// Eintraege; ein Profil mit Pfadmuster traefe eine Datei deshalb genauso wie
/// den Ordner daneben, und bis zum 260824 tat es das auch
/// (`issues/260824-1214_*_zusammenfassen-nimmt-auch-eine-datei-an-…`). Ein
/// Profil, das seinen Ort ueber eine Kennzeichendatei findet, konnte eine
/// Datei nie treffen; die Probe nimmt beide Wege.
///
/// Die Verknuepfung steht daneben, weil die Frage am **aufgeloesten** Pfad
/// entschieden wird: eine Verknuepfung auf eine Datei ist eine Datei.
#[test]
fn auf_eine_datei_greift_kein_profil_auch_bei_passendem_pfadmuster() {
    let ordner = werkbankgestalt("datei-statt-ordner");
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Trifft jeden Pfad"
pfad = '.'
kennzeichen = '^_._circle\.md$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    assert!(
        zusammenfassen(&profile, ordner.pfad()).is_some(),
        "der Ordner selbst bekommt seine Zusammenfassung"
    );
    for name in ["_t_circle.md", ".fusion-setup", "README"] {
        assert!(
            zusammenfassen(&profile, &ordner.unter(name)).is_none(),
            "die Datei {name} hat eine Zusammenfassung bekommen"
        );
    }

    let verknuepfung = ordner.unter("zeigt-auf-eine-datei");
    std::os::unix::fs::symlink(ordner.unter("_t_circle.md"), &verknuepfung)
        .expect("die Verknuepfung laesst sich nicht anlegen");
    assert!(
        zusammenfassen(&profile, &verknuepfung).is_none(),
        "eine Verknuepfung auf eine Datei ist eine Datei"
    );
}

/// Ohne Profiltreffer entsteht keine Zusammenfassung, und die Vorschau bleibt
/// bei ihrer Metadatenanzeige (C2.5).
#[test]
fn ohne_profiltreffer_entsteht_keine_zusammenfassung() {
    let ordner = werkbankgestalt("ohne-treffer");
    let (profile, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Trifft hier nicht"
kennzeichen = '^\.fusion-nicht-da$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");

    assert!(zusammenfassen(&profile, ordner.pfad()).is_none());
    assert!(
        zusammenfassen(&profile, &ordner.unter("gibt-es-nicht")).is_none(),
        "ein Ordner, der sich nicht aufloesen laesst, liefert keine Zusammenfassung"
    );
}

// ---------------------------------------------------------------------------
// Die abzaehlbaren Grenzen aus C6, gezaehlt am Haushalt eines Laufs
// ---------------------------------------------------------------------------

/// Ein Circle-Verzeichnis in der Gestalt dieser Werkbank, mit vollem Bestand.
///
/// Der Unterschied zu [`werkbankgestalt`] ist zweierlei, und beides braucht die
/// Messung zu C6.7. **Hier steht keine `.fusion-setup`**: das erste
/// mitgelieferte Profil erkennt die Wurzel der Werkbank daran, und in einem
/// Ordner mit beidem gewaenne es den zweiten Erkennungsdurchgang, sodass die
/// Messung das falsche Profil naehme. Und `history` traegt **zwoelf** Dateien
/// statt vier, damit der Baustein „juengste zehn" seine zehn Oeffnungen
/// wirklich braucht; mit vier Dateien maesse die Probe vier und nicht die
/// Zusage.
fn runde(zweck: &str) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);

    ordner.datei(
        "_t_circle.md",
        "# Circle: eine Runde\n\n## Directive\n\nDas Vorschaufenster beantwortet, was an einem Ort liegt.\n\n## Grounding\n",
    );

    let planning = ordner.ordner("planning");
    schreiben(&planning, "260824-0613_o_spec-vorschau.md", "# Spec\n");
    schreiben(&planning, "260824-0640_o_plan-vorschau.md", "# Plan\n");

    let decisions = ordner.ordner("decisions");
    for nummer in 0..3 {
        schreiben(
            &decisions,
            &format!("260824-06{nummer:02}_a_eine-frage.md"),
            "# Eine Frage?\n",
        );
    }

    let history = ordner.ordner("history");
    for nummer in 0..12 {
        let pfad = schreiben(
            &history,
            &format!("260824-{nummer:02}00-ein-verlauf.md"),
            &format!("# Verlauf {nummer}\n"),
        );
        geaendert_setzen(&pfad, 1_700_000_000 + nummer as u64 * 60);
    }

    ordner
}

/// Der Bestand einer Werkbankwurzel, unter einen beliebigen Ordner geschrieben.
///
/// **Zwei Pruefordner brauchen ihn, und sie unterscheiden sich allein darin,
/// wo er liegt.** [`werkbankwurzel`] schreibt ihn an die Wurzel des
/// Pruefordners, [`projektwurzel`] eine Ebene tiefer unter `fusion-workbench`.
/// Die sieben Zeilen des Projektwurzelprofils sind die des Wurzelprofils, jede
/// mit `fusion-workbench/` vor der Ortsangabe; ein zweiter, von Hand
/// gepflegter Bestand daneben liefe von diesem weg, und die zwei Messungen
/// verglichen dann nicht mehr dieselbe Gestalt.
fn werkbankbestand(wurzel: &Path) {
    std::fs::create_dir_all(wurzel).expect("die Werkbankwurzel laesst sich nicht anlegen");

    schreiben(
        wurzel,
        ".fusion-setup",
        r#"{"setup_at":"260801-0900","setup_pwd":"/Users/k/krk","plugin_version":"5.3.1"}"#,
    );
    schreiben(wurzel, ".active-circle", "circles/260823-2208-vorschau\n");
    schreiben(
        wurzel,
        "orchestrator-live.md",
        "# Live\n\n## Current\n\nSchritt 12, die Zaehlproben\n\n## Next\n",
    );

    let circles = wurzel.join("circles");
    for nummer in 0..3 {
        std::fs::create_dir_all(circles.join(format!("2608{nummer:02}-eine-runde")))
            .expect("das Circle-Verzeichnis laesst sich nicht anlegen");
    }

    let issues = wurzel.join("shared/issues");
    std::fs::create_dir_all(&issues).expect("der Defektspeicher laesst sich nicht anlegen");
    schreiben(&issues, "260824-0955_o_ein offener.md", "ein offener\n");
    schreiben(
        &issues,
        "260824-1014_o_ein zweiter offener.md",
        "noch einer\n",
    );
    schreiben(&issues, "260824-0600_c_ein geschlossener.md", "erledigt\n");
}

/// Die Wurzel einer Werkbank in der Gestalt, die das erste mitgelieferte
/// Profil erwartet.
fn werkbankwurzel(zweck: &str) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);
    werkbankbestand(ordner.pfad());
    ordner
}

/// Eine Projektwurzel in der Gestalt, die das achte mitgelieferte Profil
/// erwartet: ein Ordner, der eine Werkbank **enthaelt**.
///
/// **Der Pruefordner ist hier eine Ebene hoeher als bei [`werkbankwurzel`]**,
/// denn das Profil erkennt ueber das Kennzeichen `^fusion-workbench$`, also
/// ueber einen Eintrag im ausgewaehlten Ordner und nicht ueber dessen Inhalt.
///
/// **An der Wurzel steht sonst nichts**, und das ist kein Sparen: der zweite
/// Erkennungsdurchgang nimmt das erste Profil mit Treffer, und
/// `^\.fusion-setup$` wie `^_._circle\.md$` stehen vor `^fusion-workbench$`.
/// Ein `.fusion-setup` daneben naehme dem Profil seinen Ordner.
fn projektwurzel(zweck: &str) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);
    werkbankbestand(&ordner.unter("fusion-workbench"));
    ordner
}

/// Die geprueften Profile der eingebetteten Auslieferungsfassung.
///
/// **Gemessen wird gegen `resources/default-readers.toml` und nicht gegen ein
/// nachgebautes Profil im Quelltext der Probe.** C6.7 spricht ueber die
/// mitgelieferten Profile; ein Nachbau maesse, was die Probe schreibt, und
/// bliebe gruen, wenn jemand der Auslieferungsfassung eine Zeile hinzufuegt.
fn ausgelieferte() -> Profile {
    let (profile, meldungen) = gepruefte(AUSLIEFERUNGSTEXT);
    assert!(
        meldungen.is_empty(),
        "die Auslieferungsfassung wird beanstandet: {meldungen:?}"
    );
    assert_eq!(profile.zahl(), 12, "es sind nicht die zwoelf Profile");
    profile
}

/// C6.1: Ein Baustein kostet hoechstens einen Leselauf, und im erkannten Ordner
/// keinen eigenen.
///
/// **Das ist die am 260824-1224 berichtigte Fassung des Kriteriums**
/// (`issues/260824-0634_*_c6-1-sagt-der-feldbaustein-lese-kein-verzeichnis-…`).
/// „Der Feldbaustein loest keinen Leselauf aus" war in dieser Allgemeinheit
/// falsch: wer eine Datei ueber ein Muster auf ihrem Namen benennt, liest damit
/// das Verzeichnis, in dem sie liegt. Was gilt, steht in den zwei letzten
/// Zeilen der Tabelle: im erkannten Ordner kostet er nichts, weil er die eine
/// Lesung benutzt, die es dort ohnehin gibt; in einem Unterordner kostet er
/// genau einen, und auch den nur, wenn diese Zusammenfassung den Unterordner
/// noch nicht gelesen hat.
///
/// **Der letzte Fall ist der, den die Runde 18 umgedreht hat.** Bis dahin
/// lasen zwei Bausteine auf demselben Unterordner ihn zweimal, weil ein
/// Unterordner nicht gemerkt wurde; seit
/// `shared/decisions/260825-1725_*_liest-eine-zusammenfassung-denselben-…`
/// gilt ohne Ausnahme: ein Ort wird je Zusammenfassung hoechstens einmal
/// gelesen. Die Zahl der Laeufe ist damit die Zahl der **verschiedenen**
/// genannten Orte und nicht die der Zeilen mit Ortsangabe.
///
/// **Der Erkennungslauf steckt in jeder Zahl**, denn er ist ein Leselauf dieser
/// Zusammenfassung und keiner daneben; das Profil hier findet seinen Ort ueber
/// eine Kennzeichendatei und braucht die Eintraege des erkannten Ordners
/// deshalb schon vor der ersten Zeile.
///
/// Ohne diese Probe stuende die Zusage allein im Modulkopf von
/// `leseprofil::bausteine`. Ein weggefallener gemeinsamer Leselauf bliebe dann
/// unbemerkt, bis jemand die Vorschau am laufenden Buendel langsam findet — und
/// dort ist sie nicht mehr abzuzaehlen, sondern nur noch zu spueren.
#[test]
fn ein_baustein_kostet_hoechstens_einen_leselauf_und_im_erkannten_ordner_keinen() {
    let ordner = werkbankgestalt("leselaeufe-je-baustein");

    let faelle: [(&str, &str, u32); 11] = [
        ("kein Baustein, nur die Erkennung", "", 1),
        (
            "Zaehlung im erkannten Ordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Eintraege"
  zaehlung = { }
"#,
            1,
        ),
        (
            "Zaehlung in einem Unterordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Entscheidungen"
  zaehlung = { ordner = "decisions" }
"#,
            2,
        ),
        (
            "juengste im erkannten Ordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { anzahl = 3 }
"#,
            1,
        ),
        (
            "juengste in einem Unterordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { ordner = "history", anzahl = 3 }
"#,
            2,
        ),
        (
            "Feld im erkannten Ordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Directive"
  feld = { datei = '^_._circle\.md$', feldmuster = '^# (.+)' }
"#,
            1,
        ),
        (
            "Feld in einem Unterordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Spec"
  feld = { ordner = "planning", datei = '_o_spec-', feldmuster = '^# (.+)' }
"#,
            2,
        ),
        (
            "Vorhandensein im erkannten Ordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Aktiv"
  vorhandensein = { muster = '^_t_circle\.md$' }
"#,
            1,
        ),
        (
            "Vorhandensein in einem Unterordner",
            r#"
  [[profil.zeile]]
  beschriftung = "Plan"
  vorhandensein = { ordner = "planning", muster = '_._plan-' }
"#,
            2,
        ),
        (
            "alle vier im erkannten Ordner teilen sich eine Lesung",
            r#"
  [[profil.zeile]]
  beschriftung = "Eintraege"
  zaehlung = { }

  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { anzahl = 3 }

  [[profil.zeile]]
  beschriftung = "Directive"
  feld = { datei = '^_._circle\.md$', feldmuster = '^# (.+)' }

  [[profil.zeile]]
  beschriftung = "Aktiv"
  vorhandensein = { muster = '^_t_circle\.md$' }
"#,
            1,
        ),
        (
            "zwei Bausteine auf demselben Unterordner teilen sich eine Lesung",
            r#"
  [[profil.zeile]]
  beschriftung = "Spec"
  vorhandensein = { ordner = "planning", muster = '_._spec-' }

  [[profil.zeile]]
  beschriftung = "Plan"
  vorhandensein = { ordner = "planning", muster = '_._plan-' }
"#,
            2,
        ),
    ];

    for (fall, zeilen, erwartet) in faelle {
        let (_, haushalt) = gezaehlt(&circleprofil(zeilen), ordner.pfad());
        assert_eq!(
            haushalt.leselaeufe(),
            erwartet,
            "{fall}: {} Leselaeufe statt {erwartet}",
            haushalt.leselaeufe()
        );
    }
}

/// C6.1, die zweite Haelfte: der eine Leselauf faellt erst an, wenn ihn jemand
/// braucht.
///
/// Trifft ein **Pfadmuster**, kostet die Erkennung nichts, denn sie sieht allein
/// auf den Pfadtext. Ein Profil, dessen Zeilen alle in Unterordnern arbeiten,
/// liest den erkannten Ordner dann ueberhaupt nicht: zwei Bausteine, zwei
/// Leselaeufe, und kein dritter fuer einen Ordner, den niemand befragt hat.
///
/// Die Probe steht neben der Tabelle darueber und nicht in ihr, weil sie eine
/// andere Aussage traegt: dort geht es um den Preis **eines** Bausteins, hier
/// darum, dass der gemerkte Leselauf traege ist. Faellt die Traegheit weg,
/// bleibt die Tabelle gruen und diese Probe wird rot.
#[test]
fn ohne_einen_rufer_wird_der_erkannte_ordner_gar_nicht_gelesen() {
    let ordner = werkbankgestalt("traeger-leselauf");

    let (zusammenfassung, haushalt) = gezaehlt(
        r#"
[[profil]]
name = "Trifft ueber den Pfad"
pfad = '.'

  [[profil.zeile]]
  beschriftung = "Entscheidungen"
  zaehlung = { ordner = "decisions" }

  [[profil.zeile]]
  beschriftung = "Verlaeufe"
  zaehlung = { ordner = "history" }
"#,
        ordner.pfad(),
    );

    assert_eq!(
        haushalt.leselaeufe(),
        2,
        "der erkannte Ordner ist gelesen worden, obwohl ihn keine Zeile nennt"
    );
    assert_eq!(
        werte(&zusammenfassung)
            .into_iter()
            .map(|(_, wert)| wert.clone())
            .collect::<Vec<_>>(),
        [Wert::Zahl(3), Wert::Zahl(4)],
        "die zwei Unterordner sind trotzdem gelesen worden"
    );
}

/// Der Merker ueber die gelesenen Orte lebt genau so lange wie **eine**
/// Zusammenfassung.
///
/// Die dritte Haelfte derselben Zusage. Die Tabelle darueber misst, dass ein
/// Ort **innerhalb** eines Laufs hoechstens einmal gelesen wird; hier steht die
/// Gegenprobe, dass er es in jedem Lauf wieder wird. Ein Merker, der laenger
/// lebte, zeigte dem Nutzer beim zweiten Blick auf denselben Ordner den Stand
/// vom ersten — und zwar ohne jedes Anzeichen, dass er alt ist.
///
/// Gemessen wird beides, was ein zu langlebiger Merker verraten wuerde: die
/// Zahl der Leselaeufe des zweiten Laufes und sein Ergebnis, nachdem sich der
/// Ordner zwischen den Laeufen geaendert hat. Die erste Haelfte allein bliebe
/// gruen, wenn jemand den Merker global haelt und die Buchung trotzdem vornimmt.
#[test]
fn zwei_zusammenfassungen_desselben_ordners_lesen_zweimal() {
    let ordner = werkbankgestalt("merker-ueberlebt-die-zusammenfassung-nicht");
    let profil = circleprofil(
        r#"
  [[profil.zeile]]
  beschriftung = "Spec"
  vorhandensein = { ordner = "planning", muster = '_._spec-' }

  [[profil.zeile]]
  beschriftung = "Plan"
  vorhandensein = { ordner = "planning", muster = '_._plan-' }

  [[profil.zeile]]
  beschriftung = "Entscheidungen"
  zaehlung = { ordner = "decisions" }
"#,
    );

    // Drei verschiedene Orte auf vier Zeilen: der erkannte Ordner fuer die
    // Kennzeichendatei, `planning` fuer die zwei Vorhandenseinszeilen zusammen,
    // `decisions` fuer die Zaehlung.
    let (erste, erster_haushalt) = gezaehlt(&profil, ordner.pfad());
    assert_eq!(
        erster_haushalt.leselaeufe(),
        3,
        "der erste Lauf liest nicht die drei verschiedenen Orte des Profils"
    );
    assert_eq!(
        werte(&erste)[2].1,
        &Wert::Zahl(3),
        "der erste Lauf zaehlt nicht die drei Entscheidungsdatensaetze"
    );

    schreiben(
        &ordner.ordner("decisions"),
        "260825-1725_o_vierte-frage.md",
        "# Vierte Frage?\n",
    );

    let (zweite, zweiter_haushalt) = gezaehlt(&profil, ordner.pfad());
    assert_eq!(
        zweiter_haushalt.leselaeufe(),
        3,
        "der zweite Lauf hat Staende des ersten benutzt, statt selbst zu lesen"
    );
    assert_eq!(
        werte(&zweite)[2].1,
        &Wert::Zahl(4),
        "der zweite Lauf zeigt den Stand von vorhin"
    );
}

/// C6.2: Wie viele Dateien eine Bausteinsorte oeffnet.
///
/// Die Zaehlung und das Vorhandensein sehen auf Namen und oeffnen nichts; der
/// Feldbaustein oeffnet eine Datei, und auch das nur, wenn sein Dateimuster
/// einen Eintrag trifft; die juengsten N oeffnen so viele Dateien, wie es
/// Kandidaten gibt, hoechstens aber N.
///
/// **Die letzte Zeile der Tabelle ist die, die eine Zahl aus dem Profil
/// ablesbar haelt.** Zwei Feldbausteine auf **derselben** Datei kosten zwei
/// Oeffnungen und nicht eine: die Auswertung fuehrt bewusst keinen
/// Zwischenspeicher ueber gelesene Dateien, damit die Zahl der Oeffnungen aus
/// dem Profil folgt und nicht aus dessen Inhalt. Faellt ein Zwischenspeicher
/// spaeter doch hinein, wird diese Zeile rot und nicht die Zusage in C6.7.
#[test]
fn die_zahl_der_oeffnungen_folgt_der_bausteinsorte() {
    let ordner = werkbankgestalt("oeffnungen-je-baustein");

    let faelle: [(&str, &str, u32); 8] = [
        (
            "die Zaehlung oeffnet nichts",
            r#"
  [[profil.zeile]]
  beschriftung = "Entscheidungen"
  zaehlung = { ordner = "decisions" }
"#,
            0,
        ),
        (
            "das Vorhandensein oeffnet nichts",
            r#"
  [[profil.zeile]]
  beschriftung = "Spec"
  vorhandensein = { ordner = "planning", muster = '_._spec-' }
"#,
            0,
        ),
        (
            "das Feld oeffnet eine Datei",
            r#"
  [[profil.zeile]]
  beschriftung = "Directive"
  feld = { datei = '^_._circle\.md$', feldmuster = '^# (.+)' }
"#,
            1,
        ),
        (
            "ein Feld ohne passende Datei oeffnet keine",
            r#"
  [[profil.zeile]]
  beschriftung = "Gibt es nicht"
  feld = { datei = '^steht-hier-nicht$', feldmuster = '^(.+)' }
"#,
            0,
        ),
        (
            "die juengsten drei oeffnen drei",
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten drei"
  juengste = { ordner = "history", anzahl = 3 }
"#,
            3,
        ),
        (
            "die juengsten zehn oeffnen nur die vier, die es gibt",
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten zehn"
  juengste = { ordner = "history", anzahl = 10 }
"#,
            4,
        ),
        (
            "ohne Kandidaten wird nichts geoeffnet",
            r#"
  [[profil.zeile]]
  beschriftung = "Die juengsten zehn"
  juengste = { ordner = "leer", anzahl = 10 }
"#,
            0,
        ),
        (
            "zwei Felder auf derselben Datei oeffnen sie zweimal",
            r#"
  [[profil.zeile]]
  beschriftung = "Directive"
  feld = { datei = '^_._circle\.md$', feldmuster = '^# (.+)' }

  [[profil.zeile]]
  beschriftung = "Noch einmal dieselbe Datei"
  feld = { datei = '^_._circle\.md$', feldmuster = '(?m)^## (.+)' }
"#,
            2,
        ),
    ];

    for (fall, zeilen, erwartet) in faelle {
        let (_, haushalt) = gezaehlt(&circleprofil(zeilen), ordner.pfad());
        assert_eq!(
            haushalt.oeffnungen(),
            erwartet,
            "{fall}: {} Oeffnungen statt {erwartet}",
            haushalt.oeffnungen()
        );
    }
}

/// C6.4, erste Haelfte: mehr Bausteine als Leselaeufe, und die uebrigen Zeilen
/// tragen ihren Platzhalter.
///
/// Das Profil traegt einen Zaehlbaustein mehr, als der Haushalt hergibt. Der
/// Erkennungslauf nimmt den ersten der [`HOECHSTENS_LESELAEUFE`], also rechnen
/// genau einer weniger als die Grenze; die zwei uebrigen Zeilen behalten ihre
/// Beschriftung und bekommen [`Wert::Nicht`].
///
/// **Der Zaehler bleibt bei der Grenze stehen und laeuft nicht darueber
/// hinaus.** Er zaehlt die stattgefundenen Leselaeufe und nicht die versuchten;
/// ohne diese Zusage saehe eine Zusammenfassung, die dreizehnmal liest, genauso
/// aus wie eine, die zwoelfmal liest und zweimal absagt.
///
/// **Dreizehn verschiedene Orte und nicht dreizehn Zeilen auf einem.** Seit ein
/// Ort je Zusammenfassung hoechstens einmal gelesen wird, kosten dreizehn
/// Zeilen auf demselben Unterordner einen einzigen Leselauf; das Profil
/// erreichte die Grenze nicht mehr und diese Probe maesse nichts. Der Haushalt
/// begrenzt die Arbeit auf der Platte, und Arbeit auf der Platte macht ein
/// weiterer **Ort**.
#[test]
fn dreizehn_zaehlbausteine_erreichen_die_grenze_und_der_rest_traegt_den_platzhalter() {
    let ordner = werkbankgestalt("leselaufgrenze");

    // Einer mehr, als nach dem Erkennungslauf noch hineinpasst. Jede Zeile
    // nennt ihren eigenen Ort, und jeder traegt dieselben drei Datensaetze wie
    // `decisions`, damit die gerechneten Zeilen alle dieselbe Zahl tragen.
    let wie_viele = HOECHSTENS_LESELAEUFE + 1;
    let mut zeilen = String::new();
    for nummer in 0..wie_viele {
        let ort = format!("grenze-{nummer:02}");
        let angelegt = ordner.ordner(&ort);
        for lfd in 0..3 {
            schreiben(
                &angelegt,
                &format!("260825-{lfd:02}00_a_eine-frage.md"),
                "# Eine Frage?\n",
            );
        }
        zeilen.push_str(&format!(
            "\n  [[profil.zeile]]\n  beschriftung = \"Entscheidungen {nummer}\"\n  \
             zaehlung = {{ ordner = \"{ort}\" }}\n"
        ));
    }

    let (zusammenfassung, haushalt) = gezaehlt(&circleprofil(&zeilen), ordner.pfad());
    let werte = werte(&zusammenfassung);

    assert_eq!(
        haushalt.leselaeufe(),
        HOECHSTENS_LESELAEUFE,
        "der Haushalt ist ueber seine Grenze hinaus verbraucht worden"
    );
    assert_eq!(
        haushalt.oeffnungen(),
        0,
        "eine Zaehlung oeffnet keine Datei"
    );

    let gerechnet = werte
        .iter()
        .take_while(|(_, wert)| !matches!(wert, Wert::Nicht))
        .count();
    assert_eq!(
        u32::try_from(gerechnet).expect("die Zahl der Zeilen passt in u32"),
        HOECHSTENS_LESELAEUFE - 1,
        "es haben nicht genau die Zeilen gerechnet, fuer die der Haushalt reichte"
    );
    assert!(
        werte[..gerechnet]
            .iter()
            .all(|(_, wert)| matches!(wert, Wert::Zahl(3))),
        "eine der gerechneten Zeilen traegt nicht die drei Entscheidungen: {werte:?}"
    );
    assert!(
        werte[gerechnet..]
            .iter()
            .all(|(_, wert)| matches!(wert, Wert::Nicht)),
        "eine Zeile hinter der Grenze traegt keinen Platzhalter: {werte:?}"
    );
    assert_eq!(
        werte.last().map(|(beschriftung, _)| *beschriftung),
        Some(format!("Entscheidungen {}", wie_viele - 1)).as_deref(),
        "die Beschriftung faellt mit dem Wert weg"
    );
}

/// C6.4, zweite Haelfte: dieselbe Regel fuer die Dateioeffnungen, und „ganz oder
/// gar nicht".
///
/// Der Verlauf des Profils ist mit Absicht so gelegt, dass er beide Aussagen
/// trennt:
///
/// ```text
/// zwei juengste zu je zehn      20 von 24    beide rechnen
/// eine dritte juengste zu zehn  passt nicht  Platzhalter, und es wird keine
///                                            einzige Datei geoeffnet
/// vier Felder zu je einer       24 von 24    die Grenze ist erreicht
/// ein fuenftes Feld             passt nicht  Platzhalter
/// ```
///
/// **Die dritte Zeile ist die eigentliche Zusage.** Vier ihrer zehn Oeffnungen
/// haetten noch hineingepasst, und einzeln gebucht haette sie diese vier
/// verbraucht und ihren Wert am Ende doch fallen lassen — die vier fehlten dann
/// den Zeilen darunter. Eine Liste aus vier von zehn Titeln unter der
/// Beschriftung „die juengsten zehn" laese sich ausserdem als „es sind nur
/// vier".
#[test]
fn die_oeffnungen_gehen_ganz_oder_gar_nicht_und_enden_an_der_grenze() {
    let ordner = werkbankgestalt("oeffnungsgrenze");
    let viele = ordner.ordner("viele");
    for nummer in 0..12 {
        schreiben(&viele, &format!("{nummer:02}.md"), "# Ein Datensatz\n");
    }

    let mut zeilen = String::new();
    for nummer in 0..3 {
        zeilen.push_str(&format!(
            "\n  [[profil.zeile]]\n  beschriftung = \"Die juengsten zehn {nummer}\"\n  \
             juengste = {{ ordner = \"viele\", anzahl = 10 }}\n"
        ));
    }
    for nummer in 0..5 {
        zeilen.push_str(&format!(
            "\n  [[profil.zeile]]\n  beschriftung = \"Directive {nummer}\"\n  \
             feld = {{ datei = '^_._circle\\.md$', feldmuster = '^# (.+)' }}\n"
        ));
    }

    let (zusammenfassung, haushalt) = gezaehlt(&circleprofil(&zeilen), ordner.pfad());
    let werte = werte(&zusammenfassung);

    assert_eq!(
        haushalt.oeffnungen(),
        HOECHSTENS_OEFFNUNGEN,
        "die Grenze ist nicht genau erreicht worden"
    );
    assert!(
        matches!(werte[0].1, Wert::Titel(titel) if titel.len() == 10),
        "die erste Zeile hat ihre zehn Titel nicht bekommen: {:?}",
        werte[0].1
    );
    assert!(
        matches!(werte[1].1, Wert::Titel(titel) if titel.len() == 10),
        "die zweite Zeile hat ihre zehn Titel nicht bekommen: {:?}",
        werte[1].1
    );
    assert_eq!(
        werte[2].1,
        &Wert::Nicht,
        "die dritte Zeile hat eine halbe Antwort bekommen"
    );
    for (stelle, (_, wert)) in werte.iter().enumerate().take(7).skip(3) {
        assert_eq!(
            *wert,
            &Wert::Text("Circle: eine Runde".to_owned()),
            "die Zeile an Stelle {stelle} hat ihr Feld nicht bekommen"
        );
    }
    assert_eq!(
        werte[7].1,
        &Wert::Nicht,
        "hinter der erreichten Grenze steht kein Platzhalter"
    );
}

/// C6.6: Eine Datei wird bis [`HOECHSTENS_BYTES`] gelesen und keinen Schritt
/// weiter.
///
/// **Geprueft wird an einem Feldmuster, das nur hinter der Grenze trifft.** Eine
/// Probe, die allein die Laufzeit misst, saehe den Unterschied zwischen 64 KB
/// und 100 KB nicht; eine, die die gelesenen Bytes zaehlte, brauchte eine zweite
/// Zaehlstelle in der Auswertung. Der Text hinter der Grenze steht in der Datei,
/// und die Antwort darauf ist der Platzhalter: was nicht gelesen wurde, ist
/// nicht da.
///
/// Die dritte Zeile nimmt dieselbe Zusage fuer den **Titel** ab, denn C6.6
/// nennt beide: die Datei in `spaet` traegt vor ihrer ersten nicht leeren Zeile
/// mehr Leerzeilen, als der Deckel fasst, und faellt deshalb auf ihren
/// Dateinamen zurueck.
#[test]
fn eine_datei_wird_bis_zur_grenze_gelesen_und_nicht_weiter() {
    let ordner = werkbankgestalt("bytegrenze");

    let deckel = usize::try_from(HOECHSTENS_BYTES).expect("der Deckel passt in usize");
    let ganze_groesse = 100 * 1024;
    let mut inhalt = String::from("KOPF: vorn\n");
    inhalt.push_str(&"x".repeat(deckel + 4096 - inhalt.len()));
    inhalt.push_str("\nHINTEN: dahinter\n");
    inhalt.push_str(&"y".repeat(ganze_groesse - inhalt.len()));
    let hinter_der_grenze = inhalt
        .find("HINTEN:")
        .expect("die Marke steht in der Datei");
    assert!(
        hinter_der_grenze > deckel,
        "die Marke steht bei Byte {hinter_der_grenze} und damit noch vor dem Deckel"
    );
    let gross = ordner.datei("gross.md", &inhalt);
    assert_eq!(
        std::fs::metadata(&gross)
            .expect("die grosse Datei steht nicht da")
            .len(),
        ganze_groesse as u64,
        "die Datei ist nicht 100 KB gross"
    );

    let spaet = ordner.ordner("spaet");
    schreiben(
        &spaet,
        "der-titel-kommt-zu-spaet.md",
        &format!("{}Der Titel\n", "\n".repeat(deckel + 1024)),
    );

    let zusammenfassung = zusammengefasst(
        &circleprofil(
            r#"
  [[profil.zeile]]
  beschriftung = "Vorn"
  feld = { datei = '^gross\.md$', feldmuster = '(?m)^KOPF: (.+)$' }

  [[profil.zeile]]
  beschriftung = "Dahinter"
  feld = { datei = '^gross\.md$', feldmuster = 'HINTEN: (.+)' }

  [[profil.zeile]]
  beschriftung = "Die juengste"
  juengste = { ordner = "spaet", anzahl = 1 }
"#,
        ),
        ordner.pfad(),
    );
    let werte = werte(&zusammenfassung);

    assert_eq!(
        werte[0].1,
        &Wert::Text("vorn".to_owned()),
        "das Feld vor der Grenze ist nicht gelesen worden"
    );
    assert_eq!(
        werte[1].1,
        &Wert::Nicht,
        "das Feld hinter der Grenze ist gelesen worden; die Datei wird weiter gelesen, \
         als C6.6 zusagt"
    );
    assert_eq!(
        werte[2].1,
        &Wert::Titel(vec!["der-titel-kommt-zu-spaet.md".to_owned()]),
        "der Titel ist hinter der Grenze gefunden worden"
    );
}

/// Das mitgelieferte Profil dieses Namens.
///
/// **Der Name ist der Ausweis, unter dem die Probe ein Profil aus der
/// Auslieferungsfassung greift**, und nicht seine Nummer in der Datei: wer
/// einen Block verschiebt, verschiebt keine Messung mit.
fn profil_der_auslieferung<'a>(profile: &'a Profile, name: &str) -> &'a Profil {
    profile
        .iter()
        .find(|profil| profil.name() == name)
        .unwrap_or_else(|| panic!("die Auslieferungsfassung fuehrt kein Profil namens {name:?}"))
}

/// Das mitgelieferte Profil des gemeinsamen Speichers, `fusion-Werkbank: der
/// gemeinsame Speicher`.
fn speicherprofil_der_auslieferung(profile: &Profile) -> &Profil {
    profil_der_auslieferung(profile, "fusion-Werkbank: der gemeinsame Speicher")
}

/// Die verschiedenen Orte, die die Zeilen eines Profils nennen, in der
/// Reihenfolge des ersten Auftretens und ohne Wiederholung.
///
/// **Die Zahl der Unterspeicher kommt aus der Profildatei und nicht aus der
/// Probe.** Wer dem Profil einen Unterspeicher hinzufuegt, aendert damit den
/// Pruefordner mit; die Probe misst dann elf Orte und sagt es, statt an einem
/// Ordner mit zehn zu messen und zu schweigen. Ein Ort mit Platzhalter waere
/// hier keine einzelne Lesung mehr; das Speicherprofil fuehrt keinen, und
/// die Probe haelt den Bau an, sobald es einen fuehrt.
fn genannte_orte(profil: &Profil) -> Vec<String> {
    let mut orte: Vec<String> = Vec::new();
    for zeile in profil.zeilen() {
        let ort = match zeile.baustein().expect("eine Zeile ohne Baustein") {
            Baustein::Zaehlung { ort, .. }
            | Baustein::Juengste { ort, .. }
            | Baustein::Feld { ort, .. }
            | Baustein::Vorhandensein { ort, .. } => ort,
        };
        assert!(
            !ort.traegt_platzhalter(),
            "ein Ort des Speicherprofils traegt einen Platzhalter; die Rechnung \
             „ein Ort, ein Leselauf\" gilt fuer ihn nicht mehr"
        );
        let name = ort.teile().join("/");
        if !orte.contains(&name) {
            orte.push(name);
        }
    }
    orte
}

/// Ein gemeinsamer Speicher in der Gestalt, die das Profil erwartet: der
/// Ordner heisst `fusion-workbench/shared`, weil das Pfadmuster des Profils
/// darauf trifft, und traegt genau die Unterordner, die das Profil nennt,
/// mit je einem Datensatz darin.
///
/// Ein leerer Unterordner kostete dieselbe Lesung; der eine Datensatz steht
/// da, damit die Zaehlung `1` und nicht `0` liefert und die Messung sich von
/// einem Lauf unterscheidet, der die Ordner gar nicht findet.
fn gemeinsamer_speicher(zweck: &str, orte: &[String]) -> (Pruefordner, PathBuf) {
    let ordner = Pruefordner::neu(zweck);
    let shared = ordner.ordner("fusion-workbench/shared");
    for ort in orte {
        let unterordner = ordner.ordner(&format!("fusion-workbench/shared/{ort}"));
        schreiben(&unterordner, "260825-2127_o_ein-datensatz.md", "# Einer\n");
    }
    (ordner, shared)
}

/// C6.7: Die drei groessten mitgelieferten Profile bleiben unter den Zahlen,
/// die der Spec ihnen zusagt.
///
/// Gemessen an der eingebetteten Auslieferungsfassung und an je einem
/// Pruefordner in der Gestalt, die das Profil erwartet. Die Zahlen stehen hier
/// **genau** und nicht als „unter der Grenze": eine Probe, die allein
/// `<= 7` prueft, bliebe gruen, wenn ein Profil von vier auf sieben
/// Leselaeufe steigt, und genau der Schritt waere die Nachricht.
///
/// **Welches Profil das groesste ist, haengt an der Frage.** Nach Oeffnungen
/// ist es das der einzelnen Runde mit elf; nach Leselaeufen ist es seit der
/// Runde 18 das des gemeinsamen Speichers mit zehn von zwoelf, und das ist
/// zugleich das mit dem kleinsten Abstand zu seiner Schranke. Die Zahlen
/// sind die der Kostenmessung vom 260825-2107 an der wirklichen Werkbank
/// (`shared/analyses/260825-2107-was-die-zwoelf-leseprofile-…`).
///
/// **Vier und nicht mehr fuenf** bei der Runde, seit ein Ort je
/// Zusammenfassung hoechstens einmal gelesen wird: die zwei Zeilen des
/// Circle-Profils auf `planning` teilen sich seither eine Lesung. Aus
/// demselben Grund kostet der gemeinsame Speicher zehn Laeufe fuer zwanzig
/// Zeilen.
///
/// ```text
/// eine Runde       4 Leselaeufe   11 Oeffnungen   C6.7: hoechstens 7 und 11
///   erkannter Ordner, planning, decisions, history
///   Circle-Datensatz, zehn Verlaeufe
/// die Wurzel       3 Leselaeufe    5 Oeffnungen   C6.4: hoechstens 12 und 24
///   erkannter Ordner, circles, shared/issues
///   .fusion-setup dreimal, .active-circle, orchestrator-live.md
/// der Speicher    10 Leselaeufe    0 Oeffnungen   C6.4: hoechstens 12 und 24
///   die zehn Unterspeicher, die das Profil nennt; keiner doppelt
///   `zeigt = "datum"` oeffnet keine Datei
/// die Projektwz.   4 Leselaeufe    5 Oeffnungen   C6.4: hoechstens 12 und 24
///   erkannter Ordner, fusion-workbench, dessen circles, dessen shared/issues
///   .fusion-setup dreimal, .active-circle, orchestrator-live.md
/// ```
///
/// **Der vierte Fall ist nicht der eines der groessten Profile.** Er steht
/// hier, weil `default-readers.toml` seine Leselaufregel an zwei Zahlen
/// vorfuehrt und bis zum 260825 nur die erste eine Probe hatte
/// (`shared/issues/260825-2233_*_die-beispielzahl-vier-des-…`): das
/// Wurzelprofil kostet drei Laeufe, das Projektwurzelprofil mit **denselben
/// sieben Zeilen** vier. Die Vier ist die eine Zahl der Datei, die den
/// Halbsatz „plus einen Lauf fuer die Erkennung" belegt, und sie leitet sich
/// hier her und wird nicht uebernommen: das Profil nennt drei verschiedene
/// Orte, keiner davon ist der erkannte Ordner selbst, also liest den allein
/// die Erkennung ueber `kennzeichen = '^fusion-workbench$'`. Die Probe haelt
/// beide Haelften — die Vier als Zahl und die Vier als `orte.len() + 1`.
///
/// **Fuenf Oeffnungen und nicht vier**, obwohl die Kostenmessung vom
/// 260825-2107 an der wirklichen Werkbank vier zaehlt: dort fehlt
/// `.active-circle`, und eine Zeile, die ihre Datei nicht findet, oeffnet
/// nichts. Nachgemessen am 260826 an einem Pruefordner ohne diese eine Datei:
/// vier Laeufe, vier Oeffnungen. Der Pruefordner hier traegt den vollen
/// Bestand, also faellt die fuenfte an, und die sieben Werte darunter sind der
/// Nachweis, dass jede Oeffnung etwas gefunden hat.
///
/// **Der Bestand unter `fusion-workbench` ist nicht Beiwerk.** Ein leeres
/// `fusion-workbench` kostet zwei Laeufe und keine Oeffnung, gemessen am
/// selben Tag: ein Ort, den es nicht gibt, wird nicht gelesen. Die Vier steht
/// also nur an einer eingerichteten Werkbank, und genau die baut
/// [`projektwurzel`].
///
/// **Geprueft wird auch, welches Profil gegriffen hat.** Die Erkennung nimmt
/// das erste Profil mit Treffer, und ein Pruefordner, auf den ein anderes
/// passt, maesse dessen Zahlen unter dieser Ueberschrift; die Beschriftungen
/// der Zusammenfassung sind der Ausweis dafuer, welches es war.
///
/// **Beim vierten Fall traegt die Beschriftungsliste allein diesen Ausweis
/// nicht.** Das Wurzelprofil fuehrt dieselben sieben Beschriftungen, es sind
/// dieselben sieben Zeilen; erst die Werte trennen die zwei. An einer
/// Projektwurzel sieht das Wurzelprofil in den ausgewaehlten Ordner selbst und
/// findet dort nichts als den Eintrag `fusion-workbench`, liefert also
/// siebenmal [`Wert::Nicht`]. Die Werteliste steht deshalb hier ausgeschrieben
/// und nicht als Vergleich gegen `wurzelwerte`: was die zwei Profile
/// aneinanderhaelte, waere eine Zusage, die `default-readers.toml` fuer sich
/// ausdruecklich nicht gibt.
#[test]
fn die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen() {
    let profile = ausgelieferte();

    let eine_runde = runde("haushalt-eine-runde");
    let (zusammenfassung, haushalt) =
        zusammenfassen_gezaehlt(&profile, eine_runde.pfad()).expect("kein Profil greift");
    let rundenwerte = werte(&zusammenfassung);

    assert_eq!(
        rundenwerte
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>(),
        [
            "Vorgesehen",
            "Aktiv",
            "Geschlossen",
            "Abgelegt",
            "Directive",
            "Spec",
            "Plan",
            "Entscheidungen",
            "Die jüngsten zehn Verläufe"
        ],
        "gemessen wurde nicht das Profil des einzelnen Circles"
    );
    assert_eq!(
        (haushalt.leselaeufe(), haushalt.oeffnungen()),
        (4, 11),
        "das groesste mitgelieferte Profil kostet nicht mehr die gemessenen vier \
         Leselaeufe und elf Oeffnungen"
    );
    assert!(
        haushalt.leselaeufe() <= 7 && haushalt.oeffnungen() <= 11,
        "C6.7 ist gebrochen: {} Leselaeufe und {} Oeffnungen",
        haushalt.leselaeufe(),
        haushalt.oeffnungen()
    );
    assert_eq!(
        rundenwerte[4].1,
        &Wert::Text("Das Vorschaufenster beantwortet, was an einem Ort liegt.".to_owned()),
        "das Profil hat seine Directive nicht gezogen; gemessen waere dann ein Lauf, \
         der gar nichts findet"
    );
    assert!(
        matches!(rundenwerte[8].1, Wert::Titel(titel) if titel.len() == 10),
        "die zehn juengsten Verlaeufe fehlen: {:?}",
        rundenwerte[8].1
    );

    let wurzel = werkbankwurzel("haushalt-wurzel");
    let (zusammenfassung, haushalt) =
        zusammenfassen_gezaehlt(&profile, wurzel.pfad()).expect("kein Profil greift");
    let wurzelwerte = werte(&zusammenfassung);

    assert_eq!(
        wurzelwerte
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>(),
        [
            "Projekt",
            "Eingerichtet",
            "fusion-Fassung",
            "Aktive Runde",
            "Sitzung",
            "Runden",
            "Offene Defekte, gemeinsam"
        ],
        "gemessen wurde nicht das Profil der Werkbankwurzel"
    );
    assert_eq!(
        (haushalt.leselaeufe(), haushalt.oeffnungen()),
        (3, 5),
        "die Wurzelzusammenfassung kostet nicht mehr die gemessenen drei Leselaeufe \
         und fuenf Oeffnungen"
    );
    assert!(
        haushalt.leselaeufe() <= HOECHSTENS_LESELAEUFE
            && haushalt.oeffnungen() <= HOECHSTENS_OEFFNUNGEN,
        "C6.4 ist gebrochen: {} Leselaeufe und {} Oeffnungen",
        haushalt.leselaeufe(),
        haushalt.oeffnungen()
    );
    assert_eq!(
        wurzelwerte
            .iter()
            .map(|(_, wert)| (*wert).clone())
            .collect::<Vec<_>>(),
        [
            Wert::Text("krk".to_owned()),
            Wert::Text("260801-0900".to_owned()),
            Wert::Text("5.3.1".to_owned()),
            Wert::Text("circles/260823-2208-vorschau".to_owned()),
            Wert::Text("Schritt 12, die Zaehlproben".to_owned()),
            Wert::Zahl(3),
            Wert::Zahl(2),
        ],
        "die Wurzelzusammenfassung liefert nicht die Werte, fuer die sie gelesen hat"
    );

    let speicherprofil = speicherprofil_der_auslieferung(&profile);
    let orte = genannte_orte(speicherprofil);
    assert_eq!(
        orte.len(),
        10,
        "das Speicherprofil nennt nicht mehr zehn Unterspeicher: {orte:?}"
    );
    let (_speicher, shared) = gemeinsamer_speicher("haushalt-speicher", &orte);
    let (zusammenfassung, haushalt) =
        zusammenfassen_gezaehlt(&profile, &shared).expect("kein Profil greift");
    let speicherwerte = werte(&zusammenfassung);

    assert_eq!(
        speicherwerte
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>(),
        speicherprofil
            .zeilen()
            .iter()
            .map(|zeile| zeile.beschriftung())
            .collect::<Vec<_>>(),
        "gemessen wurde nicht das Profil des gemeinsamen Speichers"
    );
    assert_eq!(
        (haushalt.leselaeufe(), haushalt.oeffnungen()),
        (10, 0),
        "der gemeinsame Speicher kostet nicht mehr die gemessenen zehn Leselaeufe \
         und null Oeffnungen"
    );
    assert_eq!(
        haushalt.leselaeufe() as usize,
        orte.len(),
        "ein Ort, ein Leselauf: die Laeufe folgen nicht mehr der Zahl der Orte"
    );
    assert_eq!(
        HOECHSTENS_LESELAEUFE - haushalt.leselaeufe(),
        2,
        "der Abstand des Speicherprofils zur Schranke ist nicht mehr zwei Laeufe"
    );
    assert!(
        haushalt.leselaeufe() <= HOECHSTENS_LESELAEUFE
            && haushalt.oeffnungen() <= HOECHSTENS_OEFFNUNGEN,
        "C6.4 ist gebrochen: {} Leselaeufe und {} Oeffnungen",
        haushalt.leselaeufe(),
        haushalt.oeffnungen()
    );
    assert!(
        speicherwerte
            .iter()
            .all(|(_, wert)| !matches!(wert, Wert::Nicht)),
        "eine Zeile des Speicherprofils ist nicht drangekommen: {speicherwerte:?}"
    );
    assert!(
        speicherwerte
            .iter()
            .step_by(2)
            .all(|(_, wert)| **wert == Wert::Zahl(1)),
        "die Zaehlungen sehen nicht je den einen Datensatz: {speicherwerte:?}"
    );

    let projektwurzelprofil =
        profil_der_auslieferung(&profile, "Projektwurzel mit fusion-Werkbank");
    let projektorte = genannte_orte(projektwurzelprofil);
    assert_eq!(
        projektorte,
        [
            "fusion-workbench",
            "fusion-workbench/circles",
            "fusion-workbench/shared/issues"
        ],
        "das Projektwurzelprofil nennt nicht mehr diese drei Orte"
    );
    assert!(
        !projektorte.iter().any(String::is_empty),
        "eine Zeile des Projektwurzelprofils nennt den erkannten Ordner selbst; dann \
         teilt sie sich dessen Lesung mit der Erkennung, und der Erkennungslauf \
         kommt nicht mehr obendrauf: {projektorte:?}"
    );

    let projekt = projektwurzel("haushalt-projektwurzel");
    let (zusammenfassung, haushalt) =
        zusammenfassen_gezaehlt(&profile, projekt.pfad()).expect("kein Profil greift");
    let projektwerte = werte(&zusammenfassung);

    assert_eq!(
        projektwerte
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>(),
        projektwurzelprofil
            .zeilen()
            .iter()
            .map(|zeile| zeile.beschriftung())
            .collect::<Vec<_>>(),
        "gemessen wurde nicht das Profil der Projektwurzel"
    );
    assert_eq!(
        (haushalt.leselaeufe(), haushalt.oeffnungen()),
        (4, 5),
        "die Projektwurzel kostet nicht mehr die vier Leselaeufe und fuenf \
         Oeffnungen, mit denen `default-readers.toml` die Leselaufregel belegt"
    );
    assert_eq!(
        haushalt.leselaeufe() as usize,
        projektorte.len() + 1,
        "die Vier ist nicht mehr die drei genannten Orte plus den einen \
         Erkennungslauf"
    );
    assert!(
        haushalt.leselaeufe() <= HOECHSTENS_LESELAEUFE
            && haushalt.oeffnungen() <= HOECHSTENS_OEFFNUNGEN,
        "C6.4 ist gebrochen: {} Leselaeufe und {} Oeffnungen",
        haushalt.leselaeufe(),
        haushalt.oeffnungen()
    );
    assert_eq!(
        projektwerte
            .iter()
            .map(|(_, wert)| (*wert).clone())
            .collect::<Vec<_>>(),
        [
            Wert::Text("krk".to_owned()),
            Wert::Text("260801-0900".to_owned()),
            Wert::Text("5.3.1".to_owned()),
            Wert::Text("circles/260823-2208-vorschau".to_owned()),
            Wert::Text("Schritt 12, die Zaehlproben".to_owned()),
            Wert::Zahl(3),
            Wert::Zahl(2),
        ],
        "die Projektwurzelzusammenfassung liefert nicht die Werte, fuer die sie \
         gelesen hat; eine Zeile, die nichts findet, oeffnet auch nichts, und die \
         fuenf Oeffnungen darueber waeren dann keine fuenf Treffer"
    );
}

/// Die Gegenprobe zu C6.7 am Speicherprofil: ein elfter Unterspeicher kostet
/// einen elften Leselauf, und der Abstand zur Schranke faellt auf einen.
///
/// Gemessen wird an einer **Kopie** der Auslieferungsfassung, der das
/// Speicherprofil um eine Zeile auf einen elften Ort erweitert ist; die
/// Datei unter `resources/` bleibt, wie sie ist. Die Probe belegt, dass die
/// Messung darueber die Aenderung sieht: waere der Ort schon mitgelesen oder
/// die Zaehlung an den Zeilen statt an den Orten, bliebe die Zahl bei zehn,
/// und die Probe davor hielte etwas, das sie nicht misst.
#[test]
fn ein_elfter_unterspeicher_kostet_einen_elften_leselauf() {
    let ankerzeile = "pfad = 'fusion-workbench/shared$'\n";
    assert_eq!(
        AUSLIEFERUNGSTEXT.matches(ankerzeile).count(),
        1,
        "das Pfadmuster des Speicherprofils steht nicht genau einmal in der \
         Auslieferungsfassung"
    );
    let erweitert = AUSLIEFERUNGSTEXT.replacen(
        ankerzeile,
        "pfad = 'fusion-workbench/shared$'\n\n  [[profil.zeile]]\n  beschriftung = \"Elfter\"\n  zaehlung = { ordner = \"elfter\", muster = '\\.md$' }\n",
        1,
    );
    let (profile, meldungen) = gepruefte(&erweitert);
    assert!(
        meldungen.is_empty(),
        "die erweiterte Fassung wird beanstandet: {meldungen:?}"
    );

    let speicherprofil = speicherprofil_der_auslieferung(&profile);
    let orte = genannte_orte(speicherprofil);
    assert_eq!(orte.len(), 11, "die Kopie nennt nicht elf Orte: {orte:?}");
    let (_speicher, shared) = gemeinsamer_speicher("haushalt-elfter-speicher", &orte);
    let (zusammenfassung, haushalt) =
        zusammenfassen_gezaehlt(&profile, &shared).expect("kein Profil greift");

    assert_eq!(
        werte(&zusammenfassung).first().map(|(name, _)| *name),
        Some("Elfter"),
        "gemessen wurde nicht das erweiterte Speicherprofil"
    );
    assert_eq!(
        (haushalt.leselaeufe(), haushalt.oeffnungen()),
        (11, 0),
        "der elfte Unterspeicher kostet nicht den elften Leselauf"
    );
    assert_eq!(
        HOECHSTENS_LESELAEUFE - haushalt.leselaeufe(),
        1,
        "mit elf Orten bleibt nicht genau ein Lauf Abstand"
    );
}

/// C2.8, die Haelfte ohne Fenster: ein bloesartiges Muster haelt die Auswertung
/// nicht an.
///
/// `(a+)+$` gegen vierzig `a` und ein `b` ist der Schulfall der exponentiellen
/// Rueckverfolgung: eine rueckverfolgende Maschine probiert jede Zerlegung der
/// vierzig `a` und kommt in der Lebenszeit dieses Laufs nicht zurueck. Die
/// Kiste `regex` hat kein Rueckverfolgen und laeuft linear; **die Zusage
/// besteht deshalb darin, dass der Aufruf zurueckkehrt**, und die Zeitschranke
/// darunter steht nur da, damit ein Fehlschlag als Fehlschlag erscheint und
/// nicht als haengender Testlauf.
///
/// Geprueft werden alle vier Stellen, an denen ein Muster aus der
/// `readers.toml` auf Text trifft: das Pfadmuster auf dem vollen Pfad, die
/// Kennzeichendatei und das Eintragsmuster auf Namen, das Feldmuster auf dem
/// Inhalt. Eine fuenfte gibt es nicht.
///
/// Die sichtbare Haelfte der Zusage — die Zusammenfassung erscheint, das
/// Fenster bleibt bedienbar — steht unter `## Nutzerarbeit` des Plans und ist
/// hier nicht zu belegen.
#[test]
fn ein_boesartiges_muster_haelt_die_auswertung_nicht_an() {
    let boesartig = format!("{}b", "a".repeat(40));
    let ordner = Pruefordner::neu("boesartiges-muster");
    let tief = ordner.ordner(&boesartig);
    schreiben(&tief, &boesartig, &boesartig);

    let beginn = std::time::Instant::now();

    // Das Pfadmuster, auf dem vollen Pfad des ausgewaehlten Ordners.
    let (nur_pfad, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Boesartiges Pfadmuster"
pfad = '(a+)+$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");
    assert!(
        zusammenfassen(&nur_pfad, &tief).is_none(),
        "der Pfad endet auf b und darf nicht treffen"
    );

    // Die Kennzeichendatei, auf den Namen der Eintraege.
    let (nur_kennzeichen, meldungen) = gepruefte(
        r#"
[[profil]]
name = "Boesartiges Kennzeichen"
kennzeichen = '(a+)+$'
"#,
    );
    assert!(meldungen.is_empty(), "{meldungen:?}");
    assert!(
        zusammenfassen(&nur_kennzeichen, &tief).is_none(),
        "der eine Eintrag endet auf b und darf nicht treffen"
    );

    // Das Eintragsmuster und das Feldmuster, in einem Profil, das trifft.
    let zusammenfassung = zusammengefasst(
        r#"
[[profil]]
name = "Boesartige Zeilen"
pfad = '.'

  [[profil.zeile]]
  beschriftung = "Zaehlung"
  zaehlung = { muster = '(a+)+$' }

  [[profil.zeile]]
  beschriftung = "Vorhandensein"
  vorhandensein = { muster = '(a+)+$' }

  [[profil.zeile]]
  beschriftung = "Feld"
  feld = { datei = '(a+)+b$', feldmuster = '(a+)+$' }
"#,
        &tief,
    );

    let gebraucht = beginn.elapsed();

    assert_eq!(
        werte(&zusammenfassung)
            .into_iter()
            .map(|(_, wert)| wert.clone())
            .collect::<Vec<_>>(),
        [Wert::Zahl(0), Wert::Vorhanden(false), Wert::Nicht],
        "die drei Zeilen liefern nicht, was ein Muster ohne Treffer liefert"
    );
    assert!(
        gebraucht < Duration::from_secs(10),
        "die vier Muster haben {gebraucht:?} gebraucht; eine rueckverfolgende \
         Maschine ist in die Auswertung geraten"
    );
}

// ---------------------------------------------------------------------------
// C6.9: der Deskriptorhaushalt, gemessen in einer Kindprobe
// ---------------------------------------------------------------------------

/// Die abgesenkte Deskriptorgrenze der Kindprobe.
///
/// Dieselbe Zahl wie in `tests/umfang.rs`, und aus demselben Grund: sie muss
/// tief genug liegen, dass das Kind den Vorrat in wenigen Schritten
/// aufgebraucht hat, und hoch genug, dass `libtest` selbst noch starten kann.
/// Behauptet wird sie nicht — das Kind misst zuerst, was es bekommt.
const GRENZE_DESKRIPTOREN: usize = 24;

/// Die Umgebungsvariable, die die Kindprobe beauftragt. Ihr Wert ist der
/// Pruefordner, den das Elternteil angelegt hat.
const AUFTRAG_DESKRIPTOREN: &str = "KRK_PROBE_LESEPROFIL_DESKRIPTOREN";

/// Die Zeilen, mit denen die Kindprobe rechnet.
///
/// Alle drei Sorten, die etwas oeffnen oder lesen, und jede an einer anderen
/// Stelle: das Feld im erkannten Ordner, die Zaehlung in einem Unterordner, die
/// juengsten zehn in einem zweiten. Das sind vier Verzeichnisleselaeufe und elf
/// Oeffnungen nacheinander — und keine zwei zugleich.
const ZEILEN_DER_DESKRIPTORPROBE: &str = r#"
  [[profil.zeile]]
  beschriftung = "Directive"
  feld = { datei = '^_._circle\.md$', feldmuster = '(?sm)^## Directive\s*\n+(.+?)\n\n' }

  [[profil.zeile]]
  beschriftung = "Entscheidungen"
  zaehlung = { ordner = "decisions", muster = '\.md$' }

  [[profil.zeile]]
  beschriftung = "Die juengsten zehn"
  juengste = { ordner = "history", anzahl = 10 }
"#;

/// C6.9: Eine Zusammenfassung haelt nie mehr als einen Verzeichnis- und einen
/// Dateideskriptor zugleich.
///
/// **Die Probe laeuft im Kind, weil `cargo test` die angehobene Deskriptor-
/// grenze der Anmeldesitzung erbt.** Im selben Prozess gemessen behauptete sie
/// die Zusage, statt sie zu messen: bei tausend freien Deskriptoren liefe auch
/// eine Auswertung durch, die zehn Dateien gleichzeitig offen haelt. Die Form
/// ist die der Deskriptorproben aus der Runde 10 in `tests/verzeichnis.rs` und
/// `tests/umfang.rs`, und sie ist es ausdruecklich: eine zweite Bauart daneben
/// haette dieselbe Frage zweimal verschieden beantwortet.
///
/// Angelegt und abgeraeumt wird der Pruefordner vom **Elternteil**:
/// `remove_dir_all` haelt selbst Deskriptoren und koennte unter der abgesenkten
/// Grenze nicht aufraeumen.
#[test]
fn eine_zusammenfassung_haelt_nie_mehr_als_einen_deskriptor_zugleich() {
    let ordner = runde("deskriptorhaushalt");

    let ergebnis = kind_mit_deskriptorgrenze(
        GRENZE_DESKRIPTOREN,
        "kind_fasst_mit_einem_freien_deskriptor_zusammen",
        AUFTRAG_DESKRIPTOREN,
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "mit einem freien Deskriptor kommt die Zusammenfassung nicht zustande\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

/// Die Kindprobe zu C6.9: rechnen mit genau einem freien Deskriptor.
///
/// Der Vorrat wird **hergestellt und nicht abgewartet**: das Kind nimmt
/// Deskriptoren, bis keiner mehr kommt, und gibt genau einen zurueck. Wer dann
/// zwei zugleich braucht, bekommt beim zweiten `EMFILE`.
///
/// **Der erste Durchgang ohne einen einzigen freien Deskriptor ist die
/// Gegenprobe.** Ohne ihn saehe der zweite auch dann bestanden aus, wenn
/// `ulimit` nicht gegriffen haette und in Wahrheit tausend Deskriptoren frei
/// waeren.
///
/// **Gemessen wird an den Werten und nicht an einem Rueckgabewert `Some`.** Ein
/// Titel faellt bei einem Lesefehler still auf den Dateinamen zurueck, und ein
/// Feld auf den Platzhalter; die Titel der Verlaufsdateien lauten deshalb
/// „Verlauf n" und nicht wie ihre Dateien.
#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_PROBE_LESEPROFIL_DESKRIPTOREN gestartet"]
fn kind_fasst_mit_einem_freien_deskriptor_zusammen() {
    let Some(ordner) = std::env::var_os(AUFTRAG_DESKRIPTOREN) else {
        return;
    };
    let ordner = PathBuf::from(ordner);

    // Vor dem Mangel: das Uebersetzen der Muster braucht keinen Deskriptor,
    // und danach steht keiner mehr zur Verfuegung.
    let (profile, meldungen) = gepruefte(&circleprofil(ZEILEN_DER_DESKRIPTORPROBE));
    assert!(meldungen.is_empty(), "{meldungen:?}");

    let mut gehalten = Vec::new();
    while gehalten.len() < 4 * GRENZE_DESKRIPTOREN {
        match std::fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(_) => break,
        }
    }
    let vorrat = gehalten.len();

    // Erst ohne einen freien, dann mit genau einem.
    let ohne = zusammenfassen(&profile, &ordner);
    drop(gehalten.pop());
    let mit = zusammenfassen(&profile, &ordner);
    drop(gehalten);

    assert!(
        vorrat < 4 * GRENZE_DESKRIPTOREN,
        "das Kind bekommt {vorrat} Deskriptoren; die Grenze {GRENZE_DESKRIPTOREN} hat \
         nicht gegriffen, und die Probe messte nichts"
    );
    assert!(
        vorrat > 0,
        "das Kind bekommt gar keinen Deskriptor; gemessen waere der Mangel und nicht \
         die Bauart"
    );
    assert!(
        ohne.is_none(),
        "ohne einen freien Deskriptor entsteht eine Zusammenfassung; die Gegenprobe \
         belegt nichts mehr"
    );

    let zusammenfassung = mit.expect("mit einem freien Deskriptor entsteht keine Zusammenfassung");
    let werte = werte(&zusammenfassung);
    assert_eq!(
        werte[0].1,
        &Wert::Text("Das Vorschaufenster beantwortet, was an einem Ort liegt.".to_owned()),
        "das Feld ist nicht gelesen worden; ein Ordner blieb offen, waehrend die Datei \
         an der Reihe war"
    );
    assert_eq!(
        werte[1].1,
        &Wert::Zahl(3),
        "der Unterordner ist nicht gelesen worden"
    );
    let Wert::Titel(titel) = werte[2].1 else {
        panic!("die juengsten zehn fehlen: {:?}", werte[2].1);
    };
    assert_eq!(titel.len(), 10, "es sind nicht zehn Titel: {titel:?}");
    assert!(
        titel.iter().all(|zeile| zeile.starts_with("Verlauf ")),
        "ein Titel ist auf seinen Dateinamen zurueckgefallen: {titel:?}"
    );
}
