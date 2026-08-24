//! Abnahme der Gestalt von `readers.toml`, des Pruefschritts dahinter
//! (Schritt 3 der Runde 16) und der Ortserkennung (Schritt 5).
//!
//! Alle Proben hier laufen ohne Fenster **und ohne Dateisystem**: sie lesen
//! einen TOML-Text aus dem Quelltext und halten die gepruefte Fassung gegen
//! erwartete Werte, und die Eintraege eines Ordners kommen als von Hand
//! gebaute Liste herein. Das ist die erste der drei Pruefformen aus dem
//! Abschnitt `## Testing Strategy` des Plans; die Bausteine am Pruefordner und
//! die Zaehlproben zu C6 kommen in spaeteren Schritten hinzu.
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
use std::path::Path;
use std::time::SystemTime;

use krk_core::leseprofil::datei::{Profildatei, pruefen};
use krk_core::leseprofil::erkennung::erkennen;
use krk_core::leseprofil::{Baustein, HOECHSTENS_JUENGSTE, Profile};
use krk_core::verzeichnis::{Eintrag, Typ};

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
/// Geprueft wird beides: dass die unmarkierte Auswahl **die richtige Sorte**
/// trifft und dass die Angaben darin ankommen — der Unterordner, das wahlfreie
/// Muster, seine Abwesenheit und die Zahl.
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
// Die vier Abweisungen
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
