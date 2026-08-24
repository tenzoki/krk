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
//! Die Zaehlproben zu C6 kommen mit Schritt 12 hinzu.
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

use krk_core::leseprofil::datei::{Profildatei, pruefen};
use krk_core::leseprofil::erkennung::erkennen;
use krk_core::leseprofil::{
    Baustein, HOECHSTENS_EINTRAEGE, HOECHSTENS_JUENGSTE, Profile, Wert, Zusammenfassung,
    Zusammenfassungszeile, zusammenfassen,
};
use krk_core::verzeichnis::{Eintrag, Typ};

use gemeinsam::Pruefordner;

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
        } => {
            assert_eq!(ort.teile(), ["history"]);
            assert!(muster.is_none(), "ohne Muster zaehlen alle Eintraege");
            assert_eq!(*anzahl, 10);
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
/// nennt.
#[test]
fn ein_verschriebener_schluessel_nennt_sich_in_der_meldung() {
    let vorspann = "[[profil]]\nname = \"Ein Speicher\"\npfad = 'analyses$'\n\n                      [[profil.zeile]]\n  beschriftung = \"Eine Zeile\"\n";
    for (zeile, gesucht) in [
        ("  zaehlung = { mustre = 'y' }\n", "mustre"),
        ("  zaehlungg = { }\n", "zaehlungg"),
        (
            "  zaehlung = { }\n  beschreibung = \"zu viel\"\n",
            "beschreibung",
        ),
    ] {
        let fehler = toml::from_str::<Profildatei>(&format!("{vorspann}{zeile}"))
            .expect_err("der Text kommt durch, obwohl er einen falschen Schluessel traegt");
        assert!(
            fehler.to_string().contains(gesucht),
            "die Meldung nennt {gesucht:?} nicht: {fehler}"
        );
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
    let (profile, meldungen) = gepruefte(text);
    assert!(meldungen.is_empty(), "unerwartete Meldungen: {meldungen:?}");
    zusammenfassen(&profile, ordner).expect("kein Profil greift auf den Pruefordner")
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
/// zweitausend Eintraegen teilen; drei Proben legten ihn dreimal an.
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
