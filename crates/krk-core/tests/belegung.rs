//! Abnahme der Belegungsmaschine (Schritt 11 des Plans).
//!
//! Alle Pruefungen laufen ohne Fenster und ohne AppKit. Die, die eine
//! `keymap.toml` brauchen, legen ihren eigenen Ablageordner unter dem
//! Temporaerverzeichnis an und fassen das echte Benutzerverzeichnis nicht an.
//!
//! Schritt 20 laesst die Pruefungen der Belegungsansicht in diese Datei
//! hineinwachsen; deshalb waehlt das Abnahmekommando das Testprogramm mit
//! `--test belegung` und filtert nicht ueber Pruefungsnamen.

use std::collections::BTreeSet;
use std::fs;

use krk_core::ablage::{Ablage, Ablageort, Datei};
use krk_core::tasten::belegung::{self, Belegung, Belegungsfehler, Zuweisungsfehler};
use krk_core::tasten::normalisierung::roh;
use krk_core::tasten::parser::{self, Herkunft};
use krk_core::tasten::{Kombination, Kommando, ModMaske, Nachschlag, Tastendruck, Wirkungsbereich};

mod gemeinsam;
use gemeinsam::{Pruefordner, varianten_der_aufzaehlung};

// ---------------------------------------------------------------------------
// Hilfsmittel
// ---------------------------------------------------------------------------

/// Eine Ablage im genannten Pruefordner, mit dem gegebenen Inhalt von
/// `keymap.toml`.
///
/// Eine freie Funktion und keine Methode am [`Pruefordner`]: der gemeinsame
/// Pruefordner unter `tests/gemeinsam/` haelt Ordner und Dateien, und eine
/// [`Ablage`] ist keines von beiden, sondern ein Gegenstand des Kerns. Nur diese
/// Datei braucht ihn, also steht er hier.
///
/// **Der Anfangsinhalt geht unter der Schreibsperre auf die Platte.** Bis zur
/// Runde 7 stand hier `fs::write(ablage.pfad(...))`, also der eine Schreibweg
/// an der Sperre vorbei, den `krk_core::ablage` ausschliessen will; die naechste
/// Probe haette ihn fuer erlaubt gehalten
/// (`issues/260813-0540_*_kein-schreibweg-an-der-sperre-vorbei-ist-nicht-typgesichert-und-ungeprueft.md`).
/// Geschrieben wird ein roher Text und keine Serialisierung, weil die Proben
/// auch fehlerhafte `keymap.toml` brauchen; der Pfad kommt deshalb aus dem
/// [`Zugang`] und der Vorgang aus `atomar::schreiben`, wie bei `settings.toml`.
fn ablage_mit(ordner: &Pruefordner, keymap: &str) -> Ablage {
    let ablage =
        Ablage::oeffnen(Ablageort::an(ordner.pfad())).expect("die Ablage laesst sich oeffnen");
    ablage
        .durchgang(|zugang| fs::write(zugang.pfad(Datei::Belegung), keymap))
        .expect("die Schreibsperre laesst sich nicht nehmen")
        .expect("keymap.toml laesst sich schreiben");
    ablage
}

/// Laedt die Belegung so, wie der Betrieb es tut: unter der Schreibsperre.
///
/// Seit der Runde 7 fuehrt jeder Weg auf die Platte durch einen `Zugang`, und
/// den gibt es nur aus einem Durchgang. Die Proben halten sich daran, statt eine
/// Hintertuer zu bekommen; der Grund steht im Kopf von
/// `krk_core::ablage::sperre`.
fn geladene_belegung(ablage: &Ablage) -> krk_core::ablage::Geladen<Belegung> {
    ablage
        .durchgang(belegung::laden)
        .expect("die Schreibsperre laesst sich nicht nehmen")
}

/// Sichert die Belegung unter der Schreibsperre.
fn belegung_sichern(ablage: &Ablage, belegung: &Belegung) {
    ablage
        .durchgang(|zugang| belegung.sichern(zugang))
        .expect("die Schreibsperre laesst sich nicht nehmen")
        .expect("die Belegung laesst sich sichern");
}

/// Die Kennungen einer Belegung, sortiert: ihr Wortschatz ohne die Reihenfolge.
///
/// Die Reihenfolge bleibt aussen vor, weil eine Nutzerdatei die Funktionen, die
/// sie nennt, nach vorne holt und die uebrigen dahinter antreten laesst.
fn kennungen(belegung: &Belegung) -> Vec<&str> {
    let mut gefunden: Vec<&str> = belegung
        .funktionen()
        .iter()
        .map(|funktion| funktion.kennung())
        .collect();
    gefunden.sort_unstable();
    gefunden
}

/// Die Funktionen, die ab Werk ohne Kombination ausgeliefert werden, ohne
/// `reserviert_fuer` zu tragen.
///
/// Die drei Spaltenschalter der Bereichsleisten-Runde. Die Wahl ist eine
/// Nutzerantwort und keine Auslassung
/// (`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/
/// 260812-0306_*_bekommen-die-spaltenschalter-tastenbefehle.md`, Moeglichkeit 2:
/// in der Belegung gefuehrt, ohne ausgelieferte Kombination). Der Grund ist die
/// Knappheit der 39 frei gewaehlten Kombinationen; eine Spaltensichtbarkeit ist
/// eine Einstellung, die man einmal trifft, und kein Handgriff im Arbeitsfluss.
///
/// **Seit dem 260814 kommt eine vierte hinzu, und ihr Grund ist ein anderer.**
/// `tiefe_suche_umschalten` ist das Ankreuzfeld "Deep" der Filter-Runde. Es
/// geht nicht ohne Kombination, weil keine uebrig waere, sondern weil der
/// Nutzer am 260814-1610 keine der drei vorgeschlagenen Ebenen gewaehlt hat
/// (`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/
/// 260814-1552_*_welche-tastenkombination-schaltet-die-tiefe-suche.md`):
/// ausgeliefert wird keine, und wer eine will, vergibt sie in der
/// Belegungsansicht oder in seiner eigenen `keymap.toml`. Eine offen gelassene
/// Wahl ist nicht dasselbe wie eine bewusst gesparte Kombination, und der Grund
/// steht deshalb daneben und nicht in derselben Klammer.
///
/// **Seit dem 260901 tritt eine siebte hinzu, und ihr Grund ist ein anderer als
/// der der sechs davor.** `belegungsdatei_ansehen` stellt die Belegungsdatei des
/// Nutzers in die Vorschau, und der Nutzer hat sie im Auftrag desselben Tages
/// ausdruecklich ohne Kombination bestellt: erreichbar ueber das Hauptmenue, wie
/// die vier Spaltenschalter. Die sechs davor folgen einer offen gelassenen Wahl,
/// diese einer getroffenen; die Form ist dieselbe, die Herleitung nicht.
///
/// **Seit dem 260816 tritt eine fuenfte hinzu, und sie hat denselben Grund wie
/// die vierte.** `inhaltssuche_umschalten` ist das Ankreuzfeld "Content" der
/// Inhaltsfilter-Runde, der zweite Schalter derselben Art neben "Deep". Es
/// folgt der Nutzerantwort vom 260814-1610, die fuer den ersten Schalter dieser
/// Art keine Kombination gewaehlt hat; ein zweiter derselben Art belegt keine
/// der frei gehaltenen Kombinationen, sondern folgt derselben Form.
///
/// **Sie steht hier und nicht in `resources/default-keymap.toml`.** Das Feld
/// `reserviert_fuer` der Datei heisst "benannt, aber einer spaeteren Runde
/// vorbehalten", und diese Funktionen gibt es; es passt also nicht. Die
/// Ausnahme ist damit eine Aussage der Pruefungen ueber die Auslieferung, und
/// **zwei Pruefungen brauchen sie**, weshalb sie einmal hier steht und nicht
/// zweimal in je einem Rumpf: `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste`
/// liest sie von der Seite der Belegungsdatei her,
/// `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` von der Seite
/// der gebauten Kommandos. Wer eine weitere Funktion ohne Kombination
/// ausliefert, traegt sie mit ihrem Datensatz hier nach; die Runde 23 hat es
/// fuer `spalte_marke_umschalten` getan, den vierten Spaltenschalter, der der
/// Nutzerantwort vom 260812-0306 folgt wie die drei vor ihm.
///
/// **Eine dritte Pruefung fuehrt dieselbe Aufzaehlung ein zweites Mal**, als
/// Literal im Rumpf von `belegungsausgabe::tests::
/// jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
/// (`crates/krk-ui/src/belegungsausgabe.rs`). Sie erreicht diese Konstante
/// nicht: `krk-ui` hat kein Bibliotheksziel, und `crates/krk-core/tests/` ist
/// eine eigene Kiste. Wer hier nachtraegt, traegt dort mit nach; ob die beiden
/// Listen eine werden, ist die Frage `circles/
/// 260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/
/// 260814-2326_*_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`.
const OHNE_KOMBINATION_AB_WERK: [&str; 7] = [
    "spalte_groesse_umschalten",
    "spalte_datum_umschalten",
    "spalte_typ_umschalten",
    "spalte_marke_umschalten",
    "tiefe_suche_umschalten",
    "inhaltssuche_umschalten",
    "belegungsdatei_ansehen",
];

/// Die Kombination zu einer Zeichenkette, oder ein Abbruch mit klarer Meldung.
///
/// **Nur fuer Kombinationen, an denen die Zusage selbst haengt**, etwa die ab
/// Werk freie Eingabetaste. Wer eine Kombination braucht, weil eine bestimmte
/// Funktion sie traegt, nimmt [`ausgeliefert`]; wer eine braucht, weil keine
/// Funktion sie traegt, nimmt [`frei`]. Eine hier hingeschriebene Kombination
/// bindet die Pruefung sonst an eine Belegung, die der Nutzer jederzeit aendert:
/// `oeffnen` ist in zwei Tagen von `return` ueber `cmd+right` auf `right`
/// gewandert und hat dabei dreimal Pruefungen umgeworfen, an deren Zusage
/// nichts kaputt war.
fn kombi(text: &str) -> Kombination {
    match Kombination::lesen(text) {
        Ok(kombination) => kombination,
        Err(fehler) => panic!("\"{text}\" ist keine Kombination: {fehler}"),
    }
}

/// Eine Kombination, die die Auslieferungsbelegung dieser Funktion gibt.
///
/// Die Quelle ist `resources/default-keymap.toml` und nicht diese Datei. Welche
/// der Wege es ist, wenn die Funktion mehrere traegt, bleibt offen: die
/// Pruefungen, die den Aufruf machen, brauchen eine ihrer Kombinationen und
/// nicht eine bestimmte.
fn ausgeliefert(kennung: &str) -> Kombination {
    let belegung = Belegung::auslieferung();
    let Some(funktion) = belegung.funktion(kennung) else {
        panic!("die Auslieferungsbelegung kennt die Funktion {kennung} nicht");
    };
    match funktion.tasten().first() {
        Some(kombination) => *kombination,
        None => panic!("{kennung} traegt ab Werk keine Kombination"),
    }
}

/// Eine Kombination mit Zusatztaste, die die Auslieferungsbelegung keiner
/// Funktion gibt.
///
/// Gesucht statt hingeschrieben, und aus demselben Grund wie in
/// [`keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke`]:
/// jede hingeschriebene Kombination kann eines Tages belegt werden, und dann
/// faellt eine Pruefung um, die von der Belegung gar nicht handelt. `cmd+q` hat
/// genau das am 260805 getan
/// (`issues/260805-0820_*_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md`).
fn frei() -> Kombination {
    let belegung = Belegung::auslieferung();
    let vergeben: Vec<Tastendruck> = belegung
        .funktionen()
        .iter()
        .flat_map(|funktion| funktion.tasten())
        .map(|kombination| kombination.tastendruck())
        .collect();

    for taste in parser::TASTEN {
        for maske in masken_mit_zusatztaste() {
            let kombination = Kombination::neu(taste, maske);
            if !vergeben.contains(&kombination.tastendruck()) {
                return kombination;
            }
        }
    }
    panic!("die Auslieferungsbelegung laesst keine Kombination mit Zusatztaste frei");
}

// ---------------------------------------------------------------------------
// Die Auslieferungsbelegung
// ---------------------------------------------------------------------------

#[test]
fn die_auslieferungsbelegung_ist_konfliktfrei() {
    let belegung = Belegung::auslieferung();
    let konflikte = belegung.konflikte();

    assert!(
        konflikte.is_empty(),
        "die Auslieferungsbelegung ist nicht konfliktfrei: {}",
        konflikte
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("; ")
    );
}

/// Jede Funktion steht genau einmal, traegt eine Beschriftung und mindestens
/// eine Kombination.
///
/// **Die Ausnahme fuer reservierte Funktionen bleibt stehen, obwohl die
/// Auslieferungsbelegung keine mehr fuehrt.** `bearbeiten` war bis zur
/// Editor-Runde die einzige, und sie traegt seit S6 die Taste F4.
/// `reserviert_fuer` ist damit nicht weg: es ist ein Feld der Belegungsdatei,
/// eine `keymap.toml` aus einer aelteren Fassung kann es tragen, und die Regel
/// "reserviert heisst ohne Kombination" gilt fuer sie weiter.
///
/// **Seit dem 260812 gibt es eine zweite Ausnahme, und sie haengt nicht an
/// `reserviert_fuer`.** Sie steht als [`OHNE_KOMBINATION_AB_WERK`] am Kopf
/// dieser Datei, samt ihrer Begruendung und dem Datensatz dazu; eine zweite
/// Pruefung liest dieselbe Liste.
#[test]
fn jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste() {
    let belegung = Belegung::auslieferung();

    for (stelle, funktion) in belegung.funktionen().iter().enumerate() {
        for andere in belegung.funktionen().iter().skip(stelle + 1) {
            assert_ne!(
                funktion.kennung(),
                andere.kennung(),
                "{} steht zweimal",
                funktion.kennung()
            );
        }
        assert!(
            !funktion.name().is_empty(),
            "{} ohne Beschriftung",
            funktion.kennung()
        );
        // C3: jede Funktion ausser einer reservierten und den benannten
        // traegt mindestens eine Kombination.
        match funktion.reserviert_fuer() {
            Some(_) => assert!(
                funktion.tasten().is_empty(),
                "{} ist reserviert und traegt trotzdem eine Taste",
                funktion.kennung()
            ),
            None if OHNE_KOMBINATION_AB_WERK.contains(&funktion.kennung()) => {}
            None => assert!(
                !funktion.tasten().is_empty(),
                "{} traegt keine Kombination",
                funktion.kennung()
            ),
        }
    }
}

#[test]
fn die_ab_werk_freien_kombinationen_kommen_nicht_vor() {
    // Die Zusage: eine Kombination, die ein Leser belegt erwartete und die
    // ausdruecklich frei bleibt, steht in keiner Tastenliste. Der Name nennt
    // ihre Zahl nicht, denn die Liste waechst und schrumpft; eine Zahl im Namen
    // bindet die Pruefung an ihre Groesse statt an ihre Zusage und muesste bei
    // jeder Aenderung mit umbenannt werden.
    //
    // **Zwischen dem 260811 und dem 260818 stand hier eine einzige Kombination
    // und keine Schleife**, weil Clippy eine Schleife ueber ein Element abweist
    // (`single_element_loop`). Seit Opt+Cmd+Entf danebensteht, traegt die
    // Pruefung wieder eine Liste, und die Liste ist die Zusage: sie und der
    // Kopfkommentar von `resources/default-keymap.toml` (`:62`-`:67`) nennen
    // dieselben Kombinationen.
    //
    // Umschalt+Entf bleibt ab Werk frei. Bis zum 260817 stand der Grund im
    // Datensatz `shared/decisions/
    // 260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`, naemlich dass die
    // Kombination nichts endgueltig loesche; seit dem Wegfall des endgueltigen
    // Loeschens fuehrt der Kopfkommentar von `resources/default-keymap.toml`
    // sie ohne diesen Zusatz weiter.
    //
    // **Opt+Cmd+Entf ist am 260817 danebengetreten** (`82707ef`). Es trug bis
    // dahin das endgueltige Loeschen und ist mit ihm frei geworden. Neu
    // vergeben wird es nicht: im Finder bedeutet es "sofort loeschen", und
    // diese Bedeutung hat KRK nicht mehr (`shared/decisions/
    // 260817-0536_*_bekommt-f8-den-papierkorb-nachdem-das-endgueltige-loeschen-weggefallen-ist.md`).
    //
    // **Die Eingabetaste stand bis zum 260811 hier und steht es nicht mehr.**
    // Der Nutzer hatte sie am 260804 freigegeben, als der Einstieg in den
    // Ordner von ihr weggewandert ist (C2), und am 260811-1505 hat er sie fuer
    // `mit_standardprogramm_oeffnen` vergeben (`decisions/
    // 260811-1300_*_welche-vier-kombinationen-gelten-ab-werk.md`,
    // Moeglichkeit 1). Freihalten war nie das Ziel, sondern der Zwischenstand:
    // die Taste war fuer die Handlung reserviert, die sie jetzt traegt.
    //
    // Dass ein Blatt die Taste weiterhin an seine Vorgabeschaltflaeche
    // durchlaesst, ist keine Zusage dieser Pruefung und wird anderswo gehalten:
    // der Anwendungsdelegierte weist bei stehendem Blatt jeden Befehl ausser
    // dem Abbruch ab, und der Tastendruck laeuft danach unveraendert an AppKit
    // weiter.
    //
    // **Cmd+C und Cmd+V standen bis zum 260805 hier und stehen es nicht mehr.**
    // Seit S13b tragen sie die Textbefehle des Menues "Bearbeiten", und die
    // Zusage dieser Pruefung lautet "steht in keiner Tastenliste"; fuer die
    // beiden stimmt sie nicht mehr. Was sie heute zusagen, naemlich im
    // Dateifenster nichts auszuloesen, prueft
    // `der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege` weiter
    // unten, und dort steht es unter seinem richtigen Grund: nicht "unbelegt",
    // sondern "vom Menue zugestellt". Die Reservierung aus C3 ist damit
    // eingeloest und nicht gebrochen (Nutzerentscheid vom 260805-0000,
    // `decisions/
    // 260805-0000_*_menuekuerzel-in-die-konflikterkennung-oder-daneben.md`).
    //
    // ctrl+b und ctrl+s sind mit derselben Aenderung unbelegt geworden und
    // gehoeren trotzdem nicht hierher. Sie waren eine Behelfsbelegung fuer den
    // Auf- und Abstieg, deren Grund weggefallen ist; niemand erwartet sie in
    // einem Dateimanager. Sie hier zu fuehren hiesse zusagen, dass sie frei
    // bleiben, und wuerde ausgerechnet ctrl+s dem Editor spaeterer Runden
    // verstellen.
    let belegung = Belegung::auslieferung();
    for text in ["shift+delete", "opt+cmd+delete"] {
        let druck = kombi(text).tastendruck();
        assert!(
            matches!(
                belegung.nachschlag(druck),
                Nachschlag::Tippen | Nachschlag::Unbelegt
            ),
            "{text} ist ab Werk belegt"
        );
    }
}

// ---------------------------------------------------------------------------
// Der Zusteller (Schritt 13c)
// ---------------------------------------------------------------------------
//
// Die Regel, an der die fuenf Pruefungen dieses Abschnitts haengen: zwei
// Funktionen sind genau dann ein Konflikt, wenn sie dieselbe Kombination tragen
// und denselben Zusteller haben. Der Zusteller steht in `gehalten_von`: ohne das
// Feld stellt der Ereignisabgriff aus C2 zu, mit dem Wert "menue" das
// Hauptmenue. Nutzerentscheid vom 260805, `decisions/
// 260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md`.

#[test]
fn cmd_a_steht_bei_zwei_funktionen_und_ist_kein_konflikt() {
    // Der Fall, der keiner ist, und der einzige seiner Art in der
    // Auslieferungsbelegung: in der Liste markiert cmd+a alle Eintraege, im
    // Eingabefeld waehlt es den Text aus, wie auf dem Mac ueblich.
    let belegung = Belegung::auslieferung();
    let cmd_a = kombi("cmd+a");

    for kennung in ["alle_markieren", "text_alles_auswaehlen"] {
        let Some(funktion) = belegung.funktion(kennung) else {
            panic!("die Auslieferungsbelegung kennt {kennung} nicht");
        };
        assert!(
            funktion.tasten().contains(&cmd_a),
            "{kennung} traegt cmd+a nicht mehr; die Pruefung misst dann nichts"
        );
    }
    assert_eq!(
        belegung.funktion("alle_markieren").unwrap().gehalten_von(),
        None
    );
    assert_eq!(
        belegung
            .funktion("text_alles_auswaehlen")
            .unwrap()
            .gehalten_von(),
        Some("menue")
    );
    assert!(
        belegung.konflikte().is_empty(),
        "verschiedene Zusteller auf einer Kombination gelten als Konflikt"
    );
}

#[test]
fn zwei_funktionen_desselben_zustellers_auf_einer_kombination_bleiben_ein_konflikt() {
    // Beide Zusteller, damit die Regel nicht zu "kein Konflikt zwischen Menue
    // und Dateifenster" verwaessert: zwei Menueeintraege auf derselben
    // Kombination sind sehr wohl einer.
    let faelle = [
        (
            "abgriff",
            r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["ctrl+j"]

[[funktion]]
id = "verschieben"
name = "In das andere Fenster verschieben"
tasten = ["ctrl+j"]
"#,
            "In das andere Fenster kopieren",
            "In das andere Fenster verschieben",
        ),
        (
            "menue",
            r#"
[[funktion]]
id = "text_kopieren"
name = "Kopieren"
tasten = ["ctrl+j"]
gehalten_von = "menue"

[[funktion]]
id = "text_einfuegen"
name = "Einfügen"
tasten = ["ctrl+j"]
gehalten_von = "menue"
"#,
            "Kopieren",
            "Einfügen",
        ),
    ];

    for (zweck, keymap, andere, bewerber) in faelle {
        let ordner = Pruefordner::neu(zweck);
        let ablage = ablage_mit(&ordner, keymap);

        let geladen = geladene_belegung(&ablage);

        assert_eq!(
            geladen.wert,
            Belegung::auslieferung(),
            "der Konflikt unter dem Zusteller {zweck} blieb unbemerkt"
        );
        let Some(ersetzung) = geladen.ersetzung else {
            panic!("der Konflikt unter dem Zusteller {zweck} blieb unbemerkt");
        };
        let meldung = ersetzung.grund.einzelheit();
        assert!(
            meldung.contains(andere) && meldung.contains(bewerber),
            "die Meldung nennt nicht beide Funktionen: {meldung}"
        );
    }
}

#[test]
fn die_umbelegung_vergleicht_den_zusteller_ebenso() {
    // Dieselbe Regel auf dem zweiten Weg in eine Belegung, den die
    // Belegungsansicht aus C3 geht.
    let mut belegung = Belegung::auslieferung();
    let cmd_a = kombi("cmd+a");
    let cmd_x = kombi("cmd+x");

    // Verschiedene Zusteller: kein Konflikt. cmd+x haelt der Menueeintrag
    // "Ausschneiden"; das Markieren aller Eintraege stellt der Abgriff zu.
    assert_eq!(belegung.zuweisen("alle_markieren", cmd_x), Ok(()));
    assert!(belegung.konflikte().is_empty());

    // Derselbe Zusteller, zweimal Menue: Konflikt, mit beiden Namen.
    let Err(Zuweisungsfehler::Konflikt(konflikt)) = belegung.zuweisen("text_ausschneiden", cmd_a)
    else {
        panic!("cmd+a an einen zweiten Menueeintrag lieferte keinen Konflikt");
    };
    assert_eq!(konflikt.andere.kennung, "text_alles_auswaehlen");
    assert_eq!(konflikt.bewerber.kennung, "text_ausschneiden");

    // Derselbe Zusteller, zweimal Abgriff: ebenso.
    let Err(Zuweisungsfehler::Konflikt(konflikt)) = belegung.zuweisen("markierung_aufheben", cmd_a)
    else {
        panic!("cmd+a an eine zweite Funktion des Dateifensters lieferte keinen Konflikt");
    };
    assert_eq!(konflikt.andere.kennung, "alle_markieren");
}

#[test]
fn eine_unbelegte_menuefunktion_nimmt_ihre_kombination_ohne_konflikt_an() {
    // Der Satz aus dem Abnahmekriterium, an einer Belegung gemessen, in der die
    // Zuweisung wirklich etwas aendert: `text_alles_auswaehlen` steht ohne
    // Taste da, `alle_markieren` haelt cmd+a.
    let ordner = Pruefordner::neu("zuweisen-menue");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "alle_markieren"
name = "Alle Einträge markieren"
tasten = ["cmd+a"]

[[funktion]]
id = "text_alles_auswaehlen"
name = "Alles auswählen"
tasten = []
gehalten_von = "menue"
"#,
    );
    let mut belegung = geladene_belegung(&ablage).wert;

    assert_eq!(
        belegung.zuweisen("text_alles_auswaehlen", kombi("cmd+a")),
        Ok(())
    );

    assert!(belegung.konflikte().is_empty());
    assert_eq!(
        belegung
            .funktion("text_alles_auswaehlen")
            .unwrap()
            .tasten()
            .len(),
        1
    );
}

#[test]
fn der_nachschlag_haengt_nicht_an_der_reihenfolge_der_eintraege() {
    // Ohne das Ueberspringen im Nachschlag bestimmte die Reihenfolge der
    // Eintraege in der Datei des Nutzers, ob das Markieren aller Eintraege noch
    // wirkt. Diese Datei stellt den Textbefehl bewusst nach vorn.
    let ordner = Pruefordner::neu("reihenfolge");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "text_alles_auswaehlen"
name = "Alles auswählen"
tasten = ["cmd+a"]
gehalten_von = "menue"

[[funktion]]
id = "alle_markieren"
name = "Alle Einträge markieren"
tasten = ["cmd+a"]

[[funktion]]
id = "fenster_schliessen"
name = "Fenster schließen"
tasten = ["shift+cmd+w"]
gehalten_von = "menue"
"#,
    );
    let geladen = geladene_belegung(&ablage);
    assert!(!geladen.ist_ersetzt(), "die Datei ist gueltig");
    let belegung = geladen.wert;

    let Nachschlag::Funktion(funktion) = belegung.nachschlag(kombi("cmd+a").tastendruck()) else {
        panic!("cmd+a trifft keine Funktion, obwohl alle_markieren sie traegt");
    };
    assert_eq!(funktion.kennung(), "alle_markieren");
    assert_eq!(funktion.kommando(), Some(Kommando::AlleMarkieren));

    // Und eine Funktion, die es sonst gaebe, liefert kein Kommando, sobald das
    // Hauptmenue sie zustellt: die vierte Stelle der Regel. `fenster_schliessen`
    // ist der einzige Fall, an dem sich das ueberhaupt messen laesst, weil die
    // vier Textbefehle ohnehin in keiner Kennung eines Kommandos stehen.
    let geschlossen = belegung
        .funktion("fenster_schliessen")
        .expect("die Funktion steht in der Datei");
    assert_eq!(geschlossen.gehalten_von(), Some("menue"));
    assert_eq!(geschlossen.kommando(), None);
    assert!(matches!(
        belegung.nachschlag(kombi("shift+cmd+w").tastendruck()),
        Nachschlag::Unbelegt
    ));

    // Die drei Textbefehle der Auslieferungsbelegung loesen im Dateifenster
    // nichts aus. cmd+a fehlt hier, weil es dort das Markieren traegt.
    let ab_werk = Belegung::auslieferung();
    for text in ["cmd+x", "cmd+c", "cmd+v"] {
        assert_eq!(
            ab_werk.nachschlag(kombi(text).tastendruck()),
            Nachschlag::Unbelegt,
            "{text} loest im Dateifenster etwas aus"
        );
    }
}

#[test]
fn der_rueckweg_ueber_die_belegungsdatei_traegt_den_zusteller_mit() {
    // Fehlte `gehalten_von` im Rueckweg, schriebe KRK beim Sichern eine Datei,
    // die es beim naechsten Start als widerspruechlich abweist.
    let ordner = Pruefordner::neu("rueckweg");
    let ablage =
        Ablage::oeffnen(Ablageort::an(ordner.pfad())).expect("die Ablage laesst sich oeffnen");
    let belegung = Belegung::auslieferung();

    let text = toml::to_string(&krk_core::tasten::Belegungsdatei::from(&belegung))
        .expect("die Belegung laesst sich schreiben");
    assert!(
        text.contains("gehalten_von"),
        "der Rueckweg laesst den Zusteller fallen"
    );
    let wieder: krk_core::tasten::Belegungsdatei =
        toml::from_str(&text).expect("und wieder einlesen");
    assert_eq!(Belegung::vom_nutzer(&wieder), Ok(belegung.clone()));

    // Und derselbe Weg ueber die Platte, den `Belegung::sichern` wirklich geht.
    belegung_sichern(&ablage, &belegung);
    let geladen = geladene_belegung(&ablage);
    assert!(
        !geladen.ist_ersetzt(),
        "die selbst geschriebene keymap.toml wurde beim Einlesen abgewiesen"
    );
    assert_eq!(geladen.wert, belegung);
}

// ---------------------------------------------------------------------------
// Die Tastencodes und ihre Herkunft
// ---------------------------------------------------------------------------

#[test]
fn die_gemessenen_drei_sind_gemessen_und_die_dokumentierten_drei_dokumentiert() {
    // Der Kern der Belegkette: F3, F5 und F8 hat das Projekt am 260802-1137
    // selbst gedrueckt (`spikes/fn-tasten/messung-A.txt`, Ereignisse #03 bis
    // #05). F4, F6 und F7 nicht. Ihre Codes stammen allein aus der
    // Carbon-Tabelle, und die Tabelle sagt das ueber sich selbst.
    let gemessen = [("f3", 99u16), ("f5", 96), ("f8", 100)];
    let dokumentiert = [("f4", 118u16), ("f6", 97), ("f7", 98)];

    for (name, code) in gemessen {
        let taste = parser::taste_mit_namen(name).expect("die Tabelle kennt die Taste");
        assert_eq!(taste.code, code);
        assert!(
            taste.herkunft.ist_gemessen(),
            "{name} steht als dokumentiert, ist aber gemessen"
        );
        let Herkunft::Gemessen { beleg, .. } = taste.herkunft else {
            panic!("{name} traegt keinen Beleg");
        };
        assert!(
            beleg.contains("messung-A.txt"),
            "der Beleg von {name} nennt die Messung nicht: {beleg}"
        );
    }

    for (name, code) in dokumentiert {
        let taste = parser::taste_mit_namen(name).expect("die Tabelle kennt die Taste");
        assert_eq!(taste.code, code);
        assert!(
            !taste.herkunft.ist_gemessen(),
            "{name} ist als gemessen gekennzeichnet, gemessen wurde es nie"
        );
    }
}

#[test]
fn genau_die_drei_funktionstasten_der_messung_sind_gemessen() {
    let gemessen: Vec<&str> = parser::TASTEN
        .iter()
        .filter(|taste| taste.herkunft.ist_gemessen())
        .map(|taste| taste.name)
        .collect();

    assert_eq!(gemessen, vec!["f3", "f5", "f8"]);
}

#[test]
fn die_tastencodes_stimmen_mit_der_carbon_tabelle_ueberein() {
    // Die Gegenprobe zur Tabelle: einmal als Zahl. Die Werte stammen aus
    // `kVK_*` in `HIToolbox.framework/Headers/Events.h` des macOS-SDK,
    // nachgesehen am 260803 und fuer die acht Nachtraege am 260804. `objc2`
    // fuehrt die Tastencodes nicht, sonst
    // stuende hier ein Vergleich gegen die Kiste wie in `krk-ui` fuer die acht
    // Modifikatorbits.
    //
    // Ohne diese Probe kann die Pruefung der Belegung nur scheitern, wenn
    // jemand die Tabelle aendert; ob `down` wirklich 125 ist, pruefte dann
    // nichts.
    let erwartet = [
        ("return", 0x24u16),
        ("tab", 0x30),
        ("space", 0x31),
        ("delete", 0x33),
        ("esc", 0x35),
        ("home", 0x73),
        ("pageup", 0x74),
        ("end", 0x77),
        ("pagedown", 0x79),
        ("down", 0x7D),
        ("up", 0x7E),
        ("left", 0x7B),
        ("right", 0x7C),
        ("f1", 0x7A),
        ("f2", 0x78),
        ("f3", 0x63),
        ("f4", 0x76),
        ("f5", 0x60),
        ("f6", 0x61),
        ("f7", 0x62),
        ("f8", 0x64),
        ("f9", 0x65),
        ("f10", 0x6D),
        ("f11", 0x67),
        ("f12", 0x6F),
        ("a", 0x00),
        ("k", 0x28),
        ("y", 0x10),
        ("1", 0x12),
        ("0", 0x1D),
    ];
    for (name, code) in erwartet {
        assert_eq!(parser::code_von(name), Some(code), "der Code von {name}");
    }
}

// ---------------------------------------------------------------------------
// Der Nachschlag
// ---------------------------------------------------------------------------

#[test]
fn tastencode_99_trifft_dieselbe_funktion_mit_und_ohne_function() {
    // C3 verlangt, dass fn keine Zusatztaste ist. Der Nachschlag darf die
    // beiden Faelle nicht unterscheiden, gleich welches Ereignis ein nacktes F3
    // erzeugt.
    let belegung = Belegung::auslieferung();
    let mit_function = Tastendruck::aus_ereignis(99, None, roh::FUNKTION);
    let ohne_function = Tastendruck::aus_ereignis(99, None, 0);

    let Nachschlag::Funktion(mit) = belegung.nachschlag(mit_function) else {
        panic!("F3 mit gesetztem function trifft keine Funktion");
    };
    let Nachschlag::Funktion(ohne) = belegung.nachschlag(ohne_function) else {
        panic!("F3 ohne gesetztes function trifft keine Funktion");
    };

    assert_eq!(mit.kennung(), ohne.kennung());
    assert_eq!(mit.kennung(), "vorschau_umschalten");
}

/// Eine Funktion mit mehreren ausgelieferten Wegen ist ueber jeden davon
/// erreichbar und bleibt eine Zeile der Belegungsansicht (C3).
///
/// **Ebenfalls ohne hingeschriebene Kombination.** Die Vorgaengerin fuehrte
/// sechs Zeilen aus Funktionstaste, Cmd-Kuerzel und Kennung. Das ist dieselbe
/// Bauart, an der `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`
/// dreimal zerbrochen ist, nur eine Tabelle weiter: wer `shift+cmd+k` vom
/// Kopieren wegnimmt, macht sie rot, ohne dass die Zusage verletzt waere.
///
/// Welche Funktionen mehrere Wege tragen, sagt `resources/default-keymap.toml`;
/// die Pruefung sucht sie dort und misst an allen, die sie findet. Eine Zahl
/// steht deshalb auch hier nicht: die Datei waechst mit jeder Runde, und die
/// spaeteren Runden geben Funktionen mehrere Wege, zu denen es noch kein
/// Kommando gibt. Genau die deckt die Pruefung ab, die
/// `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` nicht sieht.
#[test]
fn beide_ausgelieferten_wege_treffen_dieselbe_funktion() {
    let belegung = Belegung::auslieferung();

    let mut geprueft = 0usize;
    for funktion in belegung.funktionen() {
        // Was das Hauptmenue zustellt, kommt im Nachschlag nicht vor; der
        // Modulkopf von `tasten::belegung` schreibt aus, warum.
        if funktion.gehalten_von().is_some() || funktion.tasten().len() < 2 {
            continue;
        }
        geprueft += 1;
        for kombination in funktion.tasten() {
            let Nachschlag::Funktion(getroffen) = belegung.nachschlag(kombination.tastendruck())
            else {
                panic!(
                    "{kombination} trifft keine Funktion, obwohl {} sie traegt",
                    funktion.kennung()
                );
            };
            assert_eq!(
                getroffen.kennung(),
                funktion.kennung(),
                "{kombination} steht bei {} und trifft eine andere Funktion",
                funktion.kennung()
            );
        }
    }

    // Ohne diese Zeile bestuende die Pruefung auch dann, wenn die
    // Auslieferungsbelegung jeder Funktion nur noch einen Weg gaebe und es
    // nichts mehr zu messen gaebe.
    assert!(
        geprueft > 0,
        "keine Funktion der Auslieferungsbelegung traegt mehr als eine Kombination"
    );
}

#[test]
fn ein_unbelegter_buchstabe_ohne_zusatztaste_faellt_auf_das_tippen() {
    // C2 der Runde 1 liess die Auswahl auf den ersten Eintrag mit diesem
    // Anfangsbuchstaben springen; seit der Runde 10 verkuerzt derselbe
    // Tastendruck die Liste. Beide Male gilt dieselbe Voraussetzung, und die
    // prueft diese Probe: kein Buchstabe der Auslieferungsbelegung ist ohne
    // Zusatztaste belegt.
    let belegung = Belegung::auslieferung();
    for buchstabe in 'a'..='z' {
        let Some(code) = parser::code_von(&buchstabe.to_string()) else {
            panic!("die Tabelle kennt {buchstabe} nicht");
        };
        let druck = Tastendruck::neu(code, ModMaske::LEER);
        assert_eq!(
            belegung.nachschlag(druck),
            Nachschlag::Tippen,
            "{buchstabe} faellt nicht auf das Tippen durch"
        );
    }
}

/// Eine unbelegte Kombination mit **Befehlstaste** endet nicht im Tippen.
///
/// **Die Zusage hat sich am 260816 verschoben, und die Probe mit ihr.** Bis
/// dahin galt sie fuer jede der fuenfzehn Masken: eine Zusatztaste, gleich
/// welche, hielt den Tastendruck vom Tippen fern. Der Nutzerentscheid vom
/// 260816-1105 teilt die vier Zusatztasten in Schreibtasten (`shift`, `opt`)
/// und Befehlstasten (`cmd`, `ctrl`); die Zusage gilt seither fuer die
/// Masken, die eine Befehlstaste halten, und nur fuer sie. Die anderen prueft
/// `eine_unbelegte_kombination_aus_schreibtasten_faellt_auf_das_tippen`
/// darunter — dieselbe Schleife, die andere Haelfte, damit keine der fuenfzehn
/// Masken ungeprueft bleibt.
///
/// **Ohne festes Beispiel, und das ist der Punkt.** Die Vorgaengerin nannte
/// `cmd+q`. Der Nachtrag des Eintrags `beenden` am 260805-0820 belegte die
/// Kombination und machte die Pruefung rot, obwohl an der Zusage nichts kaputt
/// war (`issues/260805-0820_*_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md`).
/// Ein anderes Beispiel verschoebe den Fehlschlag nur: anders als bei einem
/// Tastennamen, den die Tabelle nie aufnehmen darf, kann jede Kombination
/// eines Tages belegt werden. Die Pruefung sucht deshalb selbst, welche
/// Kombinationen die Auslieferungsbelegung frei laesst, und prueft die Zusage
/// an allen. Ein Nachtrag in `resources/default-keymap.toml` nimmt ihr damit
/// einen Fall und laesst die uebrigen stehen.
#[test]
fn keine_unbelegte_kombination_mit_befehlstaste_faellt_auf_das_tippen() {
    let belegung = Belegung::auslieferung();
    let vergeben = vergebene_tastendruecke(&belegung);

    let mut geprueft = 0usize;
    for taste in parser::TASTEN {
        for maske in masken_mit_zusatztaste() {
            if !haelt_befehlstaste(maske) {
                continue;
            }
            let kombination = Kombination::neu(taste, maske);
            if vergeben.contains(&kombination.tastendruck()) {
                continue;
            }
            assert_eq!(
                belegung.nachschlag(kombination.tastendruck()),
                Nachschlag::Unbelegt,
                "{kombination} faellt auf das Tippen durch"
            );
            geprueft += 1;
        }
    }

    // Ohne diese Zeile bestuende die Pruefung auch dann, wenn die
    // Auslieferungsbelegung eines Tages jede Kombination mit Befehlstaste
    // vergibt und es nichts mehr zu pruefen gibt.
    assert!(
        geprueft > 0,
        "die Auslieferungsbelegung laesst keine Kombination mit Befehlstaste frei"
    );
}

/// Die Gegenprobe: eine unbelegte Kombination aus **Schreibtasten** tippt.
///
/// Das ist die Zusage des Nutzerentscheids vom 260816-1105, an allen drei
/// Masken ohne Befehlstaste — `shift`, `opt` und `opt+shift` — und an jeder
/// Taste, die die Auslieferungsbelegung dort frei laesst. Der gemeldete Fall
/// ist `shift` und der Bindestrich, also `_`; die Probe nennt ihn nicht
/// eigens, weil sie den ganzen Bereich abdeckt, aus dem er stammt, und weil
/// welche Taste den Unterstrich traegt eine Frage der Tastaturbelegung des
/// Geraets ist und nicht des Kerns.
///
/// **Der Kern entscheidet ueber den Tastendruck und nicht ueber das Zeichen.**
/// Dass aus `Nachschlag::Tippen` auch wirklich ein Zeichen im Filtertext wird,
/// haengt an `krk_core::verzeichnis::filter::traegt_ein_dateiname` und an der
/// Senke in `krk-ui`; diese Probe reicht bis an die Grenze des Kerns und
/// keinen Schritt weiter.
#[test]
fn eine_unbelegte_kombination_aus_schreibtasten_faellt_auf_das_tippen() {
    let belegung = Belegung::auslieferung();
    let vergeben = vergebene_tastendruecke(&belegung);

    let mut geprueft = 0usize;
    for taste in parser::TASTEN {
        for maske in masken_mit_zusatztaste() {
            if haelt_befehlstaste(maske) {
                continue;
            }
            let kombination = Kombination::neu(taste, maske);
            if vergeben.contains(&kombination.tastendruck()) {
                continue;
            }
            assert_eq!(
                belegung.nachschlag(kombination.tastendruck()),
                Nachschlag::Tippen,
                "{kombination} faellt nicht auf das Tippen durch"
            );
            geprueft += 1;
        }
    }

    assert!(
        geprueft > 0,
        "die Auslieferungsbelegung laesst keine Kombination aus Schreibtasten frei"
    );
}

/// Die vier Zusatztasten, jede einzeln, an einer Taste, die keine Funktion
/// traegt.
///
/// Die Abnahme des Nutzerentscheids vom 260816-1105 in ihrer knappsten Form:
/// `shift` und `opt` tippen, `cmd` und `ctrl` nicht. Die beiden Schleifen
/// darueber pruefen dieselbe Regel breiter; diese Probe steht daneben, weil
/// eine Schleife ueber alle freien Kombinationen ihre Zusage nicht mehr
/// benennt, sobald sie fehlschlaegt — hier steht in der Meldung, welche der
/// vier Zusatztasten sich falsch verhaelt.
///
/// **Die Taste sucht die Probe sich selbst.** Ein fest genanntes `f5` waere
/// dieselbe Falle, in die `cmd+q` am 260805-0820 gelaufen ist.
#[test]
fn die_vier_zusatztasten_trennen_schreiben_und_befehlen() {
    let belegung = Belegung::auslieferung();
    let vergeben = vergebene_tastendruecke(&belegung);

    for (maske, name) in ModMaske::BENANNT {
        let Some(taste) = parser::TASTEN
            .into_iter()
            .find(|taste| !vergeben.contains(&Kombination::neu(*taste, maske).tastendruck()))
        else {
            panic!("die Auslieferungsbelegung laesst mit {name} keine Taste frei");
        };
        let kombination = Kombination::neu(taste, maske);
        let erwartet = if haelt_befehlstaste(maske) {
            Nachschlag::Unbelegt
        } else {
            Nachschlag::Tippen
        };
        assert_eq!(
            belegung.nachschlag(kombination.tastendruck()),
            erwartet,
            "{name} ist als Zusatztaste falsch eingeordnet ({kombination})"
        );
    }
}

/// Jede belegte Kombination wird weiterhin als Funktion gefunden.
///
/// **Die Zusage, an der die ganze Aenderung vom 260816 haengt.** Die neue
/// Unterscheidung steht hinter der Belegungssuche und kann deshalb keinem
/// Kuerzel etwas wegnehmen; das misst diese Probe an **jeder** Kombination,
/// die die Auslieferungsbelegung vergibt.
///
/// **Der Unterschied zu `beide_ausgelieferten_wege_treffen_dieselbe_funktion`
/// ist das Wort „jeder", und er ist genau der Fall, in dem ein Rueckschritt
/// sich verstecken wuerde.** Jene Probe fragt nach C3 und ueberspringt deshalb
/// jede Funktion mit nur einem Weg; die allermeisten Kombinationen der
/// Auslieferungsbelegung sind aber genau das. Ein Fehler, der eine einzelne
/// belegte Taste ins Tippen fallen liesse, bliebe dort unsichtbar.
///
/// Zugestellte Funktionen bleiben aussen vor: sie kommen im Nachschlag
/// ueberhaupt nicht vor, und warum, steht im Modulkopf von
/// `krk_core::tasten::belegung`.
#[test]
fn jede_belegte_kombination_wird_weiterhin_als_funktion_gefunden() {
    let belegung = Belegung::auslieferung();

    let mut geprueft = 0usize;
    for funktion in belegung.funktionen() {
        if funktion.gehalten_von().is_some() {
            continue;
        }
        for kombination in funktion.tasten() {
            let Nachschlag::Funktion(getroffen) = belegung.nachschlag(kombination.tastendruck())
            else {
                panic!(
                    "{kombination} von {} faellt nicht mehr auf eine Funktion",
                    funktion.kennung()
                );
            };
            assert_eq!(
                getroffen.kennung(),
                funktion.kennung(),
                "{kombination} trifft eine andere Funktion"
            );
            geprueft += 1;
        }
    }

    assert!(
        geprueft > 0,
        "die Auslieferungsbelegung vergibt keine einzige Kombination"
    );
}

/// Ob diese Maske eine der beiden Befehlstasten haelt.
///
/// Die Probenseite der Regel aus `Belegung::nachschlag`. Sie steht hier
/// bewusst **zweitgeschrieben** und ruft nicht in den Kern hinein: eine Probe,
/// die ihre Erwartung aus derselben Zeile bezieht wie der Prueffling, bestaende
/// jede Aenderung dieser Zeile.
fn haelt_befehlstaste(maske: ModMaske) -> bool {
    maske.enthaelt(ModMaske::BEFEHL) || maske.enthaelt(ModMaske::STEUERUNG)
}

/// Jeder Tastendruck, den die Auslieferungsbelegung vergibt.
fn vergebene_tastendruecke(belegung: &Belegung) -> Vec<Tastendruck> {
    belegung
        .funktionen()
        .iter()
        .flat_map(|funktion| funktion.tasten())
        .map(|kombination| kombination.tastendruck())
        .collect()
}

/// Die fuenfzehn nicht leeren Masken ueber den vier Zusatztasten.
///
/// Gerechnet aus [`ModMaske::BENANNT`] und nicht hingeschrieben: kaeme eine
/// fuenfte Zusatztaste dazu, waere eine Liste von Hand still unvollstaendig.
fn masken_mit_zusatztaste() -> Vec<ModMaske> {
    let bits: Vec<ModMaske> = ModMaske::BENANNT.iter().map(|(bit, _)| *bit).collect();
    (1..(1u32 << bits.len()))
        .map(|muster| {
            bits.iter()
                .enumerate()
                .filter(|(stelle, _)| muster & (1 << stelle) != 0)
                .fold(ModMaske::LEER, |maske, (_, bit)| maske | *bit)
        })
        .collect()
}

/// Jedes gebaute Kommando ist ueber die Taste erreichbar, die die
/// Auslieferungsbelegung ihm gibt.
///
/// **Ohne hingeschriebene Kombination, und das ist der Punkt.** Die
/// Vorgaengerin fuehrte fuenf Paare aus Kombination und Kommando, und die Zeile
/// des Oeffnens hat in zwei Tagen dreimal gewechselt: `return`, dann
/// `cmd+right`, dann `right`. Jede Umbelegung machte die Pruefung rot, ohne
/// dass an ihrer Zusage etwas kaputt war
/// (`issues/260804-1214_*_die-belegungspruefung-bindet-return-noch-an-das-oeffnen.md`,
/// `issues/260805-1356_*_die-belegungspruefung-bindet-cmd-right-noch-an-das-oeffnen.md`).
///
/// Die Zusage braucht die Kombination gar nicht zu kennen. Sie lautet: **es
/// gibt eine, und der Nachschlag darauf trifft dieses Kommando.** Welche es
/// ist, sagt `resources/default-keymap.toml`, und das ist die einzige Stelle,
/// die es sagen darf; eine Wiederholung hier waere eine zweite Wahrheit
/// darueber, welche Taste was ausloest. Die Pruefung liest ihre Paare deshalb
/// aus [`Kommando::KENNUNGEN`] und der Belegung und ueberlebt damit jede
/// Umbelegung, die die Auslieferungsbelegung schluessig laesst.
///
/// Gemessen wird an allen gebauten Kommandos und an jeder ihrer
/// Kombinationen, nicht an fuenfen: das ist mehr Zusage als vorher, nicht
/// weniger, und sie waechst mit [`Kommando`] mit.
#[test]
fn jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste() {
    let belegung = Belegung::auslieferung();

    for (kommando, kennung) in Kommando::KENNUNGEN {
        let Some(funktion) = belegung.funktion(kennung) else {
            panic!("die Auslieferungsbelegung kennt die Funktion {kennung} nicht");
        };
        // Die Ausnahme aus [`OHNE_KOMBINATION_AB_WERK`]: ein gebautes Kommando
        // ohne ausgelieferte Kombination ist ab dem 260812 moeglich, und dann
        // gibt es nichts nachzuschlagen. Die Zusage darunter bleibt fuer jede
        // Kombination, die eine solche Funktion spaeter doch traegt.
        assert!(
            !funktion.tasten().is_empty() || OHNE_KOMBINATION_AB_WERK.contains(&kennung),
            "{kommando:?} ist gebaut, und {kennung} traegt ab Werk keine Kombination"
        );
        for kombination in funktion.tasten() {
            let Nachschlag::Funktion(getroffen) = belegung.nachschlag(kombination.tastendruck())
            else {
                panic!("{kombination} trifft keine Funktion, obwohl {kennung} sie traegt");
            };
            assert_eq!(
                getroffen.kommando(),
                Some(kommando),
                "{kombination} steht bei {kennung} und fuehrt zu {}",
                getroffen.kennung()
            );
        }
    }
}

#[test]
fn eine_gehaltene_zusatztaste_nimmt_der_taste_ihr_kommando() {
    // Umschalt+Pfeil ab gehoert spaeter der Bereichsauswahl aus C2 und darf
    // nicht wie ein nacktes Pfeil ab wirken.
    let belegung = Belegung::auslieferung();
    let Some(code) = parser::code_von("down") else {
        panic!("die Tabelle kennt \"down\" nicht");
    };
    for (zusatz, name) in ModMaske::BENANNT {
        let nachschlag = belegung.nachschlag(Tastendruck::neu(code, zusatz));
        if let Nachschlag::Funktion(funktion) = nachschlag {
            assert_ne!(
                funktion.kommando(),
                Some(Kommando::AuswahlRunter),
                "{name}+down wirkt wie ein nacktes down"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Die y-Kuerzel auf einer deutschen Tastatur
// ---------------------------------------------------------------------------

/// Ein Tastendruck auf einer deutschen Tastatur, nachgestellt: die Aufschrift
/// entscheidet und nicht die Stelle.
///
/// **Die Sache, um die es geht.** Ein virtueller Tastencode benennt eine
/// **Stelle**. Die Stelle `kVK_ANSI_Y` traegt den Code 16, und auf einer
/// deutschen Tastatur steht dort ein **Z**; auf `kVK_ANSI_Z` mit dem Code 6
/// steht ein **Y**. Solange der Ereignisabgriff auch Buchstaben ueber die
/// Stelle nachschlug, lag `cmd+y` unter der Aufschrift Z, und `cmd+z` aus dem
/// Hauptmenue, das ueber das Zeichen anschlaegt, stiess mit ihm auf einer
/// Taste zusammen (`issues/
/// 260809-1642_*_auf-einer-deutschen-tastatur-schluckt-cmd-y-das-rueckgaengig-des-editors.md`).
///
/// Seit S2 gehen Buchstaben und Ziffern ueber das gemeldete Zeichen. Die Probe
/// stellt beide Tastendruecke nach, wie der Abgriff sie meldet, und misst, was
/// die Belegung antwortet:
///
/// | Der Nutzer drueckt | Code | Zeichen | Erwartet |
/// |---|---|---|---|
/// | Taste mit der Aufschrift **Y** | 6 | `y` | `vorschau_umschalten` |
/// | Taste mit der Aufschrift **Z** | 16 | `z` | nichts, das der Abgriff zustellt |
///
/// Die zweite Zeile ist die Gegenprobe zur ersten: sie zeigt, dass nicht etwa
/// beide Tasten dieselbe Funktion treffen. Was auf ihr liegt, naemlich `cmd+z`
/// als Rueckgaengig, haelt das Hauptmenue; der Nachschlag sieht eine zugestellte
/// Funktion nie, und das Ereignis laeuft an das Menue weiter.
///
/// Der Vorgaengerdefekt ist
/// `shared/issues/260807-2112_*_cmd-y-und-shift-cmd-y-loesen-nichts-aus-f3-schon.md`,
/// der Vorgaenger derselben Sache
/// `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-2317_*_cmd-y-liegt-auf-einer-deutschen-tastatur-unter-der-taste-z.md`,
/// und die Entscheidung
/// `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`.
///
/// **Die Kombination steht hier ausnahmsweise hingeschrieben.** Die uebrigen
/// Pruefungen dieser Datei suchen ihre Kombinationen in der Belegung, damit eine
/// Umbelegung sie nicht umwirft. Hier ist es umgekehrt: die Zusage handelt von
/// genau dieser Kombination und von den beiden Stellen, auf denen sie je nach
/// Tastatur liegt.
#[test]
fn auf_einer_deutschen_tastatur_findet_die_aufschrift_y_die_vorschau() {
    let belegung = Belegung::auslieferung();

    let Some(y) = parser::taste_mit_namen("y") else {
        panic!("die Tabelle kennt die Taste \"y\" nicht");
    };
    let Some(z) = parser::taste_mit_namen("z") else {
        panic!("die Tabelle kennt die Taste \"z\" nicht");
    };
    assert_eq!((y.code, y.herkunft.kvk()), (16, "kVK_ANSI_Y"));
    assert_eq!((z.code, z.herkunft.kvk()), (6, "kVK_ANSI_Z"));

    // Die Taste mit der Aufschrift Y: sie liegt auf der Stelle kVK_ANSI_Z und
    // meldet ein `y`.
    let aufschrift_y = Tastendruck::aus_ereignis(z.code, Some('y'), roh::BEFEHL);
    let Nachschlag::Funktion(getroffen) = belegung.nachschlag(aufschrift_y) else {
        panic!("die Taste mit der Aufschrift Y trifft keine Funktion");
    };
    assert_eq!(getroffen.kennung(), "vorschau_umschalten");

    // Dieselbe Taste mit Umschalt: der Fokusbefehl, der keinen zweiten Weg hat.
    let aufschrift_y_mit_umschalt =
        Tastendruck::aus_ereignis(z.code, Some('y'), roh::BEFEHL | roh::UMSCHALT);
    let Nachschlag::Funktion(getroffen) = belegung.nachschlag(aufschrift_y_mit_umschalt) else {
        panic!("die Taste mit der Aufschrift Y trifft mit Umschalt keine Funktion");
    };
    assert_eq!(getroffen.kennung(), "fokus_vorschau");

    // Die Gegenprobe: die Taste mit der Aufschrift Z liegt auf der Stelle
    // kVK_ANSI_Y und meldet ein `z`. Der Abgriff findet dort nichts, und das
    // Ereignis laeuft an das Hauptmenue, das Cmd+Z als Rueckgaengig fuehrt.
    for maske in [roh::BEFEHL, roh::BEFEHL | roh::UMSCHALT] {
        let aufschrift_z = Tastendruck::aus_ereignis(y.code, Some('z'), maske);
        assert_eq!(
            belegung.nachschlag(aufschrift_z),
            Nachschlag::Unbelegt,
            "die Taste mit der Aufschrift Z trifft eine Funktion des Abgriffs"
        );
    }
}

/// Die Funktionstasten bleiben an ihrer Stelle, gleich welches Zeichen das
/// Ereignis traegt.
///
/// Die andere Haelfte des Zuschnitts, und die, an der C3 der Runde 1 haengt:
/// F3 liefert denselben Tastencode auf jeder Tastaturbelegung und auch mit
/// gehaltener fn-Taste. Das Zeichen, das AppKit einer Funktionstaste beilegt,
/// liegt im privaten Bereich von Unicode und taugt als Kennung nicht; der
/// Nachschlag faellt deshalb auf die Stelle zurueck.
#[test]
fn eine_funktionstaste_wird_weiter_ueber_ihren_code_gefunden() {
    let belegung = Belegung::auslieferung();
    let Some(f3) = parser::taste_mit_namen("f3") else {
        panic!("die Tabelle kennt die Taste \"f3\" nicht");
    };

    // Drei Formen desselben Drucks: ohne Zeichen, mit dem Zeichen, das AppKit
    // einer F3 beilegt (`NSF3FunctionKey`), und mit gesetztem function-Bit.
    for (gemeldet, flaggen) in [
        (None, 0),
        (Some('\u{F706}'), 0),
        (Some('\u{F706}'), roh::FUNKTION),
    ] {
        let druck = Tastendruck::aus_ereignis(f3.code, gemeldet, flaggen);
        assert_eq!(
            druck.kennung(),
            krk_core::tasten::Tastenkennung::Code(f3.code),
            "F3 wird nicht mehr ueber ihre Stelle nachgeschlagen"
        );
        let Nachschlag::Funktion(getroffen) = belegung.nachschlag(druck) else {
            panic!("F3 trifft keine Funktion");
        };
        assert_eq!(getroffen.kennung(), "vorschau_umschalten");
    }
}

/// Jede Kombination der Auslieferungsbelegung wird ueber die Art
/// nachgeschlagen, die zu ihrer Taste gehoert.
///
/// Die Zusage in einem Satz: **jede Taste, der `zeichen_des_namens` ein
/// Zeichen zuordnet, ueber dieses Zeichen, alles uebrige ueber den Code**,
/// und zwar fuer jede der ausgelieferten Kombinationen und nicht fuer eine
/// Handvoll Beispiele. Welche Namen ein Zeichen tragen, sagt allein
/// `krk_core::tasten::parser::zeichen_des_namens` (Buchstaben, Ziffern,
/// `plus`, `minus`); die Probe wiederholt die Regel nicht, sondern haelt
/// die Kennung jeder Taste gegen dieselbe Funktion. Die Probe zaehlt
/// beide Sorten mit und besteht nur, wenn beide vorkommen; sonst bestuende sie
/// auch dann, wenn eine der beiden Nachschlagarten aus der
/// Auslieferungsbelegung verschwaende.
#[test]
fn jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte() {
    use krk_core::tasten::Tastenkennung;
    use krk_core::tasten::parser::zeichen_des_namens;

    let belegung = Belegung::auslieferung();
    let (mut ueber_zeichen, mut ueber_code) = (0usize, 0usize);

    for funktion in belegung.funktionen() {
        for kombination in funktion.tasten() {
            let taste = kombination.taste();
            match taste.kennung() {
                Tastenkennung::Zeichen(zeichen) => {
                    assert_eq!(
                        zeichen_des_namens(taste.name),
                        Some(zeichen),
                        "{kombination} bei {} geht ueber ein Zeichen, das nicht das \
                         Zeichen ihres Namens ist",
                        funktion.kennung()
                    );
                    ueber_zeichen += 1;
                }
                Tastenkennung::Code(code) => {
                    assert_eq!(
                        zeichen_des_namens(taste.name),
                        None,
                        "{kombination} bei {} traegt ein Zeichen und geht trotzdem ueber \
                         die Stelle",
                        funktion.kennung()
                    );
                    assert_eq!(code, taste.code);
                    ueber_code += 1;
                }
            }
        }
    }

    assert!(
        ueber_zeichen > 0 && ueber_code > 0,
        "die Auslieferungsbelegung kennt nur noch eine der beiden Nachschlagarten \
         ({ueber_zeichen} ueber das Zeichen, {ueber_code} ueber die Stelle)"
    );
}

// ---------------------------------------------------------------------------
// Zuweisen, Konflikt und Zuruecksetzen
// ---------------------------------------------------------------------------

#[test]
fn eine_bereits_vergebene_kombination_liefert_einen_konflikt_mit_dem_namen_der_anderen_funktion() {
    let mut belegung = Belegung::auslieferung();
    // Eine Kombination, die dem Kopieren gehoert; welche, sagt die
    // Auslieferungsbelegung.
    let kombination = ausgeliefert("kopieren");
    let name = belegung.funktion("kopieren").unwrap().name().to_owned();

    let ergebnis = belegung.zuweisen("verschieben", kombination);

    let Err(Zuweisungsfehler::Konflikt(konflikt)) = ergebnis else {
        panic!("die doppelte Zuweisung lieferte keinen Konflikt: {ergebnis:?}");
    };
    assert_eq!(konflikt.kombination, kombination);
    assert_eq!(konflikt.andere.kennung, "kopieren");
    assert_eq!(konflikt.andere.name, name);
    assert_eq!(konflikt.bewerber.kennung, "verschieben");
    assert!(
        konflikt.to_string().contains(&name),
        "die Meldung nennt die andere Funktion nicht: {konflikt}"
    );
    // Die Belegung bleibt unveraendert; nichts wird stillschweigend
    // ueberschrieben.
    assert_eq!(belegung, Belegung::auslieferung());
}

#[test]
fn eine_zweite_kombination_an_derselben_funktion_ist_kein_konflikt() {
    let mut belegung = Belegung::auslieferung();
    let neue = frei();

    assert_eq!(belegung.zuweisen("kopieren", neue), Ok(()));

    let Nachschlag::Funktion(funktion) = belegung.nachschlag(neue.tastendruck()) else {
        panic!("die neue Kombination trifft keine Funktion");
    };
    assert_eq!(funktion.kennung(), "kopieren");
    // Die ausgelieferten Wege bleiben daneben stehen. Wie viele es sind, sagt
    // die Auslieferungsbelegung und nicht eine Zahl an dieser Stelle.
    let ab_werk = Belegung::auslieferung();
    let Some(vorher) = ab_werk.funktion("kopieren") else {
        panic!("die Funktion kopieren fehlt in der Auslieferungsbelegung");
    };
    for weg in vorher.tasten() {
        assert!(
            funktion.tasten().contains(weg),
            "{weg} ist beim Zuweisen von {neue} verlorengegangen"
        );
    }
    assert_eq!(
        funktion.tasten().len(),
        vorher.tasten().len() + 1,
        "die neue Kombination kam nicht als einzige hinzu"
    );
    assert!(belegung.konflikte().is_empty());
}

#[test]
fn dieselbe_kombination_zweimal_an_dieselbe_funktion_aendert_nichts() {
    let mut belegung = Belegung::auslieferung();
    let schon_da = ausgeliefert("kopieren");

    assert_eq!(belegung.zuweisen("kopieren", schon_da), Ok(()));

    assert_eq!(belegung, Belegung::auslieferung());
}

#[test]
fn eine_unbekannte_funktion_laesst_sich_nicht_belegen() {
    let mut belegung = Belegung::auslieferung();

    assert_eq!(
        belegung.zuweisen("gibtsnicht", kombi("ctrl+j")),
        Err(Zuweisungsfehler::UnbekannteFunktion(
            "gibtsnicht".to_owned()
        ))
    );
}

#[test]
fn zuruecksetzen_stellt_die_eingebettete_tabelle_wieder_her() {
    let mut belegung = Belegung::auslieferung();
    let neue = frei();
    let ab_werk = ausgeliefert("kopieren");
    assert_eq!(belegung.zuweisen("kopieren", neue), Ok(()));
    assert_ne!(belegung, Belegung::auslieferung());

    belegung.zuruecksetzen();

    assert_eq!(belegung, Belegung::auslieferung());
    let Nachschlag::Funktion(funktion) = belegung.nachschlag(ab_werk.tastendruck()) else {
        panic!("{ab_werk} trifft nach dem Zuruecksetzen keine Funktion");
    };
    assert_eq!(funktion.kennung(), "kopieren");
    assert!(matches!(
        belegung.nachschlag(neue.tastendruck()),
        Nachschlag::Unbelegt
    ));
}

// ---------------------------------------------------------------------------
// Die Nutzerbelegung
// ---------------------------------------------------------------------------

#[test]
fn eine_fehlende_keymap_liefert_die_auslieferungsbelegung_ohne_meldung() {
    let ordner = Pruefordner::neu("fehlend");
    let ablage =
        Ablage::oeffnen(Ablageort::an(ordner.pfad())).expect("die Ablage laesst sich oeffnen");

    let geladen = geladene_belegung(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    assert!(
        !geladen.ist_ersetzt(),
        "der erste Start ist keine Meldung wert"
    );
}

#[test]
fn die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht() {
    let ordner = Pruefordner::neu("ersetzen");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["ctrl+c"]
"#,
    );

    let geladen = geladene_belegung(&ablage);
    assert!(!geladen.ist_ersetzt(), "die Datei ist gueltig");
    let belegung = geladen.wert;

    // Der neue Weg gilt.
    let Nachschlag::Funktion(funktion) = belegung.nachschlag(kombi("ctrl+c").tastendruck()) else {
        panic!("ctrl+c trifft keine Funktion");
    };
    assert_eq!(funktion.kennung(), "kopieren");
    // Die ausgelieferten Wege derselben Funktion sind fort: ersetzt, nicht
    // ergaenzt. Welche das sind, sagt die Auslieferungsbelegung; die Datei oben
    // gibt dem Kopieren einen Weg, den sie ab Werk nicht hat.
    for weg in Belegung::auslieferung()
        .funktion("kopieren")
        .unwrap()
        .tasten()
    {
        assert!(
            matches!(
                belegung.nachschlag(weg.tastendruck()),
                Nachschlag::Unbelegt | Nachschlag::Tippen
            ),
            "{weg} wirkt noch, obwohl die Nutzerdatei die Belegung ersetzt"
        );
    }
    // Und die Funktionen, die die Datei nicht nennt, stehen unbelegt da, damit
    // die Belegungsansicht sie weiter auffuehrt. Gemessen wird das am
    // Wortschatz der Auslieferungsbelegung und nicht an einer hier
    // aufgeschriebenen Zahl: die Belegungsdatei waechst mit jeder Runde, und
    // eine Zahl an dieser Stelle bindet die Pruefung an ihre Groesse statt an
    // ihre Zusage.
    let ab_werk = Belegung::auslieferung();
    assert_eq!(
        kennungen(&belegung),
        kennungen(&ab_werk),
        "die geladene Belegung fuehrt einen anderen Wortschatz als die Auslieferungsbelegung"
    );

    // Und das ist die Zusage des Namens: belegt ist allein, was die Nutzerdatei
    // nennt. Eine Datei mit einem Eintrag ergibt eine Funktion mit Tasten, nicht
    // diese eine und die ausgelieferten dazu.
    let belegt: Vec<&str> = belegung
        .funktionen()
        .iter()
        .filter(|funktion| !funktion.tasten().is_empty())
        .map(|funktion| funktion.kennung())
        .collect();
    assert_eq!(
        belegt,
        ["kopieren"],
        "die Nutzerdatei nennt eine Funktion, belegt sind aber mehrere"
    );
}

#[test]
fn eine_geaenderte_belegung_ueberlebt_sichern_und_laden() {
    let ordner = Pruefordner::neu("sichern");
    let ablage =
        Ablage::oeffnen(Ablageort::an(ordner.pfad())).expect("die Ablage laesst sich oeffnen");
    let mut belegung = Belegung::auslieferung();
    assert_eq!(belegung.zuweisen("kopieren", frei()), Ok(()));

    belegung_sichern(&ablage, &belegung);
    let geladen = geladene_belegung(&ablage);

    assert!(!geladen.ist_ersetzt());
    assert_eq!(geladen.wert, belegung);
}

#[test]
fn eine_unbekannte_funktion_in_der_nutzerdatei_fuehrt_zum_auslieferungszustand() {
    let ordner = Pruefordner::neu("unbekannt");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "kaffee_kochen"
name = "Kaffee kochen"
tasten = ["ctrl+c"]
"#,
    );

    let geladen = geladene_belegung(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    let Some(ersetzung) = geladen.ersetzung else {
        panic!("die unbekannte Funktion blieb unbemerkt");
    };
    assert!(
        ersetzung.grund.einzelheit().contains("kaffee_kochen"),
        "die Meldung nennt die unbekannte Funktion nicht: {ersetzung}"
    );
    // Die Datei bleibt liegen: eine von Hand geaenderte keymap.toml darf ein
    // Tippfehler nicht loeschen.
    assert!(ablage.pfad(Datei::Belegung).exists());
}

#[test]
fn eine_falsch_geschriebene_kombination_fuehrt_zum_auslieferungszustand() {
    let ordner = Pruefordner::neu("schreibweise");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["cmd+shift+k"]
"#,
    );

    let geladen = geladene_belegung(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    let Some(ersetzung) = geladen.ersetzung else {
        panic!("die verdrehte Reihenfolge blieb unbemerkt");
    };
    assert!(
        ersetzung.grund.einzelheit().contains("cmd+shift+k"),
        "die Meldung nennt die Kombination nicht: {ersetzung}"
    );
}

#[test]
fn eine_widerspruechliche_nutzerdatei_fuehrt_zum_auslieferungszustand() {
    let ordner = Pruefordner::neu("konflikt");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["ctrl+c"]

[[funktion]]
id = "verschieben"
name = "In das andere Fenster verschieben"
tasten = ["ctrl+c"]
"#,
    );

    let geladen = geladene_belegung(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    let Some(ersetzung) = geladen.ersetzung else {
        panic!("der Konflikt in der Nutzerdatei blieb unbemerkt");
    };
    assert!(
        ersetzung
            .grund
            .einzelheit()
            .contains("In das andere Fenster kopieren"),
        "die Meldung nennt die andere Funktion nicht: {ersetzung}"
    );
}

#[test]
fn eine_syntaktisch_kaputte_keymap_fuehrt_zum_auslieferungszustand() {
    let ordner = Pruefordner::neu("kaputt");
    let ablage = ablage_mit(&ordner, "[[funktion]\nid = \"kopieren\"\n");

    let geladen = geladene_belegung(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    assert!(geladen.ist_ersetzt());
}

#[test]
fn ein_unbekanntes_feld_in_der_nutzerdatei_bleibt_nicht_unbemerkt() {
    // Ein Tippfehler im Feldnamen wuerde sonst still ignoriert, und der Nutzer
    // suchte die Wirkung einer Zeile, die niemand liest.
    let ordner = Pruefordner::neu("feld");
    let ablage = ablage_mit(
        &ordner,
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
taste = ["ctrl+c"]
"#,
    );

    let geladen = geladene_belegung(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    assert!(geladen.ist_ersetzt());
}

#[test]
fn der_fehlertext_einer_unbekannten_funktion_nennt_sie() {
    let datei: krk_core::tasten::Belegungsdatei = toml::from_str(
        r#"
[[funktion]]
id = "kaffee_kochen"
name = "Kaffee kochen"
tasten = []
"#,
    )
    .expect("gueltiges TOML");

    assert_eq!(
        Belegung::vom_nutzer(&datei),
        Err(Belegungsfehler::UnbekannteFunktion(
            "kaffee_kochen".to_owned()
        ))
    );
}

/// Eine gesicherte `keymap.toml`, die die zurueckgezogene Kennung
/// `endgueltig_loeschen` noch fuehrt, wird als Ganzes verworfen.
///
/// **Die Probe misst eine Nutzerantwort, statt sie zu behaupten.** Mit dem
/// Wegfall des endgueltigen Loeschens zieht dieses Projekt zum ersten Mal eine
/// Funktionskennung zurueck. Wer seine Belegung je ueber die Belegungsansicht
/// gesichert hat, traegt sie in seiner Datei, denn gesichert wird die
/// vollstaendige Belegung und nicht nur das Geaenderte. Gefragt war, ob KRK den
/// einen Eintrag uebergeht, die Datei einmalig nachzieht oder es beim heutigen
/// Verhalten belaesst; der Nutzer hat am 260817 "bleibt wie heute" gewaehlt und
/// den Verlust der eigenen Belegung ausdruecklich in Kauf genommen
/// (`shared/decisions/260817-0536_*_was-geschieht-mit-einer-gespeicherten-keymap-die-die-entfallene-funktion-fuehrt.md`).
///
/// **Fuer diese Antwort ist nichts gebaut worden**, und genau deshalb steht die
/// Probe hier: ein Verhalten, das niemand geschrieben hat, haelt auch niemand
/// fest, und ein spaeterer Sonderweg fiele ohne sie erst dem Nutzer auf.
///
/// Zwei Haelften, weil die Antwort zwei Aussagen hat: der Wortschatz der
/// Auslieferung kennt die Kennung nicht mehr, und die Ladelogik faellt darauf
/// auf die **vollstaendige** Auslieferung zurueck und meldet die Ersetzung.
#[test]
fn eine_keymap_mit_der_zurueckgezogenen_kennung_wird_als_ganzes_verworfen() {
    let gesichert = r#"
[[funktion]]
id = "endgueltig_loeschen"
name = "Endgültig löschen"
tasten = ["f8", "opt+cmd+delete"]

[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["ctrl+c"]
"#;

    let datei: krk_core::tasten::Belegungsdatei =
        toml::from_str(gesichert).expect("gueltiges TOML");
    assert_eq!(
        Belegung::vom_nutzer(&datei),
        Err(Belegungsfehler::UnbekannteFunktion(
            "endgueltig_loeschen".to_owned()
        ))
    );

    let ordner = Pruefordner::neu("zurueckgezogene-kennung");
    let ablage = ablage_mit(&ordner, gesichert);
    let geladen = geladene_belegung(&ablage);

    assert_eq!(
        geladen.wert,
        Belegung::auslieferung(),
        "die Auslieferung greift, und zwar vollstaendig"
    );
    assert!(
        geladen.ist_ersetzt(),
        "die zur Seite gelegte Datei wird gemeldet"
    );
}

// ---------------------------------------------------------------------------
// Der Wirkungsbereich (Schritt 18, C5)
// ---------------------------------------------------------------------------

/// Jedes Kommando traegt genau einen Wirkungsbereich.
///
/// "Genau einen" hat zwei Haelften, und der Uebersetzer traegt die eine schon:
/// [`Kommando::wirkungsbereich`] ist eine vollstaendige Fallunterscheidung ohne
/// Auffangzweig, also nennt jedes Kommando seinen Bereich, und mehr als einen
/// kann keines nennen. Diese Pruefung traegt die andere Haelfte, die der
/// Uebersetzer nicht sieht: dass kein Kommando **zweimal** in
/// [`Kommando::KENNUNGEN`] steht. Stuende eines zweimal darin, gaebe es zwei
/// Wege von einer Kennung zu einem Kommando, und der zweite koennte einen
/// anderen Bereich bekommen als der erste.
///
/// **Sie haelt die Eindeutigkeit und nicht die Vollstaendigkeit.** Sie laeuft
/// ueber `KENNUNGEN`, also ueber die Liste, deren Vollstaendigkeit die Frage
/// ist; ein Kommando, das dort fehlt, kommt in dieser Schleife nicht vor. Bis
/// zum 260826 sagte dieser Doc-Kommentar "jedes Kommando genau einmal" und
/// versprach damit beide Haelften
/// (`shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`).
/// Die Vollstaendigkeit haelt seither
/// [`jede_variante_von_kommando_steht_genau_einmal_in_kennungen`], und die
/// liest die Varianten aus dem Quelltext.
#[test]
fn jedes_kommando_traegt_genau_einen_wirkungsbereich() {
    for (stelle, (kommando, kennung)) in Kommando::KENNUNGEN.into_iter().enumerate() {
        for (andere, weitere) in Kommando::KENNUNGEN.into_iter().skip(stelle + 1) {
            assert_ne!(kommando, andere, "{kennung} steht zweimal in KENNUNGEN");
            assert_ne!(kennung, weitere, "die Kennung {kennung} steht zweimal");
        }
        // Der Aufruf selbst ist die Probe: er liefert fuer jedes Kommando
        // einen der acht Werte und kann keinen zweiten liefern.
        // `Tabbereich` kam mit dem Vorschaufenster aus S19 dazu; `Dateibereiche`
        // (bis zum 260823 `Vorschau`), `Editor` und `Navigator` mit dem
        // eingebauten Editor, `Vorschau` mit den drei Zoombefehlen der Runde 20.
        let bereich = kommando.wirkungsbereich();
        assert!(
            matches!(
                bereich,
                Wirkungsbereich::Dateifenster
                    | Wirkungsbereich::Leiste
                    | Wirkungsbereich::Dateibereiche
                    | Wirkungsbereich::Editor
                    | Wirkungsbereich::Tabbereich
                    | Wirkungsbereich::Navigator
                    | Wirkungsbereich::Vorschau
                    | Wirkungsbereich::Ueberall
            ),
            "{kennung} traegt keinen der acht Bereiche"
        );
    }
}

/// Jede Variante von [`Kommando`] steht in [`Kommando::KENNUNGEN`], und jeder
/// Eintrag der Liste benennt eine Variante.
///
/// **Die Gegenprobe zu [`jedes_kommando_traegt_genau_einen_wirkungsbereich`].**
/// Jene laeuft ueber `KENNUNGEN` und haelt die Eindeutigkeit; die
/// Vollstaendigkeit kann sie nicht halten, weil sie ueber die Liste laeuft,
/// deren Vollstaendigkeit die Frage ist. Diese hier laeuft ueber die
/// **Varianten** aus dem Quelltext der Aufzaehlung und ist damit die zweite
/// Quelle, aus der die Frage entscheidbar wird.
///
/// **Was eine fehlende Zeile kostet.** `KENNUNGEN` ist die programmweite
/// Kommandoliste: `Kommando::kennung` endet ohne Eintrag auf `panic!`,
/// `tag_des_kommandos` in `krk-ui` auf `expect`, und `Kommando::aus_kennung`
/// liefert `None`, womit der Befehl in der Belegungsansicht steht und nichts
/// tut. Der Uebersetzer haelt davon nichts: die Laengenangabe
/// `[(Kommando, &'static str); 82]` zwingt zu 82 Eintraegen und sagt nicht,
/// welche 82
/// (`shared/issues/260826-1223_*_kennungen-ist-die-programmweite-kommandoliste-und-nichts-haelt-sie-vollstaendig.md`).
///
/// Geprueft werden **beide** Richtungen. Die zweite ist nicht nur Zierrat: sie
/// faellt aus, wenn eine Variante fortfaellt und ihre Zeile stehen bleibt, und
/// eine solche Zeile uebersetzt heute noch, solange der Name als Alias
/// weiterlebt.
#[test]
fn jede_variante_von_kommando_steht_genau_einmal_in_kennungen() {
    let varianten: BTreeSet<String> =
        varianten_der_aufzaehlung("krk-core/src/tasten/belegung.rs", "Kommando")
            .into_iter()
            .collect();
    let gefuehrt: BTreeSet<String> = Kommando::KENNUNGEN
        .into_iter()
        .map(|(kommando, _)| format!("{kommando:?}"))
        .collect();

    let fehlen: Vec<&str> = varianten
        .difference(&gefuehrt)
        .map(String::as_str)
        .collect();
    assert!(
        fehlen.is_empty(),
        "diese Varianten von Kommando stehen in keiner Zeile von KENNUNGEN \
         und sind damit unbelegbar: {}",
        fehlen.join(", ")
    );

    let ueberzaehlig: Vec<&str> = gefuehrt
        .difference(&varianten)
        .map(String::as_str)
        .collect();
    assert!(
        ueberzaehlig.is_empty(),
        "diese Eintraege von KENNUNGEN benennen keine Variante der Aufzaehlung: {}",
        ueberzaehlig.join(", ")
    );
}

/// Die drei Faelle, die das Abnahmekriterium von C5 namentlich nennt.
///
/// Sie stehen hier als Zusage ueber die **Befehle** und nicht ueber die
/// Oberflaeche: dass `delete` das Dateifenster braucht und
/// `lesezeichen_loeschen` die Leiste, ist ohne Fenster pruefbar, und genau
/// deshalb wohnt der Wirkungsbereich im Kern.
#[test]
fn die_drei_faelle_aus_c5_tragen_die_bereiche_die_c5_verlangt() {
    assert_eq!(
        Kommando::InPapierkorb.wirkungsbereich(),
        Wirkungsbereich::Dateifenster,
        "delete darf in der Leiste keine Datei loeschen"
    );
    assert_eq!(
        Kommando::Oeffnen.wirkungsbereich(),
        Wirkungsbereich::Dateifenster,
        "right darf in der Leiste in keinen Ordner einsteigen"
    );
    assert_eq!(
        Kommando::OrdnerAufwaerts.wirkungsbereich(),
        Wirkungsbereich::Dateifenster,
        "left ebenso: seit der Umbelegung vom 260805 ist es eine nackte Taste"
    );
    assert_eq!(
        Kommando::LesezeichenLoeschen.wirkungsbereich(),
        Wirkungsbereich::Leiste,
        "lesezeichen_loeschen darf bei Fokus im Dateifenster nicht wirken"
    );
}

/// Die drei Zoombefehle des PDF-Betrachters tragen den Wert fuer die Vorschau
/// allein, und die drei Faelle aus C5 der Runde 1 bleiben, was sie sind (C3.5
/// und C3.7 der Runde 20, Kernhaelfte).
///
/// Der Wert `Vorschau` war vom 260823 bis zur Runde 20 nicht in der
/// Aufzaehlung; diese Probe haelt fest, dass er mit den dreien zurueck ist und
/// dass keiner der drei auf einen der weiteren Werte ausweicht, der die
/// Vorschau nur **mit** anderen Bereichen fuehrt. Die zweite Haelfte, dass die
/// Rueckkehr die alten Zusagen nicht verschiebt, steht als Aufruf der Probe
/// darueber und nicht als Abschrift ihrer vier Zeilen.
#[test]
fn die_drei_zoombefehle_tragen_die_vorschau_allein() {
    for kommando in [
        Kommando::VorschauVergroessern,
        Kommando::VorschauVerkleinern,
        Kommando::VorschauAusgangsgroesse,
    ] {
        assert_eq!(
            kommando.wirkungsbereich(),
            Wirkungsbereich::Vorschau,
            "{} wirkt allein im Vorschaufenster",
            kommando.kennung()
        );
    }
    die_drei_faelle_aus_c5_tragen_die_bereiche_die_c5_verlangt();
}

/// Die Belegungsdatei ist aus jedem Fokus zu bekommen (Nutzerauftrag vom
/// 260901).
///
/// **Nicht `Wirkungsbereich::Vorschau`, obwohl sie dort erscheint.** Der Befehl
/// **holt** die Vorschau hervor und den Fokus hinein; ein Vorbehalt auf das
/// Vorschaufenster verlangte damit genau den Zustand, den er selbst herstellt,
/// und der Nutzer bekaeme seine Belegungsdatei aus dem Dateifenster heraus nie
/// zu sehen. Dieselbe Erwaegung traegt `fokus_vorschau` in der Probe darunter.
///
/// Er steht daneben ohne Kombination in der Auslieferung und ist ueber das
/// Hauptmenue erreichbar; das Menue fragt dieselbe Regel wie der
/// Ereignisabgriff, und ein enger Wirkungsbereich graute seinen Eintrag aus.
#[test]
fn die_belegungsdatei_ist_aus_jedem_fokus_zu_bekommen() {
    assert_eq!(
        Kommando::BelegungsdateiAnsehen.wirkungsbereich(),
        Wirkungsbereich::Ueberall,
        "die Belegungsdatei braucht keinen bestimmten Bereich im Fokus"
    );
    assert_ne!(
        Kommando::BelegungsdateiAnsehen.wirkungsbereich(),
        Wirkungsbereich::Vorschau
    );
}

/// Der Fokuswechsel wirkt aus jedem Bereich heraus, und das Anlegen eines
/// Lesezeichens ebenso.
///
/// Ohne diese Zusage waere die Leiste nach C5 nicht bedienbar: der Befehl
/// zurueck in das Dateifenster muesste aus der Leiste heraus wirken, in der er
/// per Voraussetzung steht. Der vierte Fokusbefehl der Editor-Runde faellt
/// unter dieselbe Zeile.
#[test]
fn der_fokuswechsel_wirkt_aus_jedem_bereich_heraus() {
    for kommando in [
        Kommando::FokusLeiste,
        Kommando::FokusDateifenster,
        Kommando::FokusVorschau,
        Kommando::FokusEditor,
        Kommando::LesezeichenAnlegen,
    ] {
        assert_eq!(
            kommando.wirkungsbereich(),
            Wirkungsbereich::Ueberall,
            "{} braucht keinen bestimmten Bereich im Fokus",
            kommando.kennung()
        );
    }
}

/// Die drei Befehle, deren Taste im Editor der Textflaeche gehoert, tragen den
/// Navigator und nicht mehr `Ueberall`.
///
/// Sie sind in der Runde 1 mit [`Wirkungsbereich::Ueberall`] entstanden, weil
/// es damals nichts gab, wovon sie auszunehmen waeren. Mit dem eingebauten
/// Editor gibt es etwas: `up` und `down` bewegen dort die Schreibmarke, `tab`
/// schreibt einen Tabulator. Ohne den Umzug bewegte ein `up` mit dem Fokus im
/// Editor die Auswahl im Dateifenster, und das erste Abnahmekriterium von C7
/// der Editor-Runde waere gebrochen.
///
/// Was daraus fuer die Fokuswerte folgt, prueft
/// `der_navigator_endet_am_editor_und_ueberall_nicht` in `krk-ui`; hier steht
/// allein die Aussage ueber die Befehle, die ohne Fenster pruefbar ist.
#[test]
fn die_drei_befehle_des_navigators_tragen_den_navigator() {
    for kommando in [
        Kommando::FensterWechseln,
        Kommando::AuswahlHoch,
        Kommando::AuswahlRunter,
    ] {
        assert_eq!(
            kommando.wirkungsbereich(),
            Wirkungsbereich::Navigator,
            "{} wirkt weiterhin ueberall und damit auch im Editor",
            kommando.kennung()
        );
    }
}

/// Die zwoelf Kommandos des Editors tragen die Bereiche, die der Plan ihnen
/// gibt, und die Aufteilung ist erschoepfend.
///
/// Drei Sorten, und die Grenze ist die Frage, was der Befehl voraussetzt.
/// `bearbeiten` setzt das Dateifenster voraus, dessen ausgewaehlten Eintrag es
/// oeffnet; der Fokusbefehl setzt nichts voraus, weil er den Fokus holt. Die
/// uebrigen acht arbeiten in der Datei, die der Editor haelt, und ohne Fokus
/// dort gibt es keine.
///
/// **Der Rundweg faellt aus dieser Dreiteilung heraus**, und seit dem
/// 260823-0942 ist das seine Aussage: er setzt keinen einzelnen Bereich voraus,
/// sondern bedeutet in dreien etwas. Bis dahin hiess er `editor_aus_vorschau`
/// und trug `Wirkungsbereich::Vorschau`.
#[test]
fn die_zwoelf_kommandos_des_editors_tragen_ihre_bereiche() {
    assert_eq!(
        Kommando::Bearbeiten.wirkungsbereich(),
        Wirkungsbereich::Dateifenster,
        "F4 oeffnet den ausgewaehlten Eintrag des Dateifensters"
    );
    assert_eq!(
        Kommando::EditorRundweg.wirkungsbereich(),
        Wirkungsbereich::Dateibereiche,
        "der Rundweg wirkt nicht in den drei Bereichen, in denen eine Datei im Spiel ist"
    );
    assert_eq!(
        Kommando::FokusEditor.wirkungsbereich(),
        Wirkungsbereich::Ueberall,
        "ein Befehl, der den Fokus holt, kann nicht voraussetzen, wo er steht"
    );
    for kommando in [
        Kommando::EditorSchliessen,
        Kommando::EditorAnsichtUmschalten,
        Kommando::EditorSichern,
        Kommando::EditorZeileSpringen,
        Kommando::EditorSuchen,
        Kommando::EditorWeitersuchen,
        Kommando::EditorRueckwaertsSuchen,
        Kommando::EditorErsetzen,
        Kommando::EditorAlleErsetzen,
    ] {
        assert_eq!(
            kommando.wirkungsbereich(),
            Wirkungsbereich::Editor,
            "{} arbeitet in der Datei des Editors und braucht dessen Fokus",
            kommando.kennung()
        );
    }
}

// ---------------------------------------------------------------------------
// Die Beschriftung der acht Wirkungsbereiche (Runde 3, S2, C3; Runde 20, C3.6)
// ---------------------------------------------------------------------------

/// Die acht Bereiche mit dem Text, den die Tastenbelegung als Markdown fuehrt.
///
/// Der Nutzer hat am 260811-0115 drei davon genannt, naemlich die drei, deren
/// Variantenname als Beschriftung unverstaendlich waere; die uebrigen tragen
/// den Namen aus dem Modulkopf von `belegung.rs`. `Vorschau` ist mit der
/// Runde 20 dazugekommen und verweist den Leser auf das Vorschaufenster
/// (C3.6).
///
/// **Das Feld ist die Quelle des erwarteten Texts und nicht die Quelle der
/// Werte.** Welche Werte es gibt, lesen die drei Proben darunter ueber
/// [`varianten_der_aufzaehlung`] aus dem Quelltext der Aufzaehlung; ein Wert
/// ohne Zeile in diesem Feld wird dort rot, statt still ungeprueft zu bleiben
/// (`shared/issues/260826-1302_*_ein-achter-wirkungsbereich-uebersetzt-ohne-eintrag-im-beschriftungsfeld-der-doc-kommentar-sagt-das-gegenteil.md`).
const ACHT_BESCHRIFTUNGEN: [(Wirkungsbereich, &str); 8] = [
    (Wirkungsbereich::Dateifenster, "Dateifenster"),
    (Wirkungsbereich::Leiste, "Lesezeichen- und Geräteleiste"),
    (
        Wirkungsbereich::Dateibereiche,
        "Dateifenster, Vorschau und Editor",
    ),
    (Wirkungsbereich::Editor, "Editor"),
    (Wirkungsbereich::Tabbereich, "Dateifenster und Vorschau"),
    (
        Wirkungsbereich::Navigator,
        "Dateifenster, Leiste, Vorschau und Git-Bereich",
    ),
    (Wirkungsbereich::Vorschau, "Vorschau"),
    (Wirkungsbereich::Ueberall, "überall"),
];

/// Die Stelle eines Bereichs in [`ACHT_BESCHRIFTUNGEN`].
///
/// **Der Grund fuer diese zweite Fallunterscheidung ist die erste.** Eine
/// Aufzaehlung in einer Probe waechst nicht von selbst mit der Aufzaehlung im
/// Kern: ein neunter Wert bekaeme in `Wirkungsbereich::beschriftung` seine
/// Zeile vom Uebersetzer abverlangt, in einem Feld darueber aber nicht. Diese
/// Funktion ist ebenfalls ohne Auffangzweig, also uebersetzt ein neunter Wert
/// erst, wenn er auch hier steht. **Im Feld steht er damit noch nicht**: der
/// Zweig darf jede Zahl liefern, und das Feld zieht niemand nach. Dass jeder
/// Wert im Feld steht, haelt [`jeder_wirkungsbereich_im_quelltext`] ueber die
/// Varianten aus dem Quelltext und nicht ueber das Feld.
fn stelle_in_den_acht(bereich: Wirkungsbereich) -> usize {
    match bereich {
        Wirkungsbereich::Dateifenster => 0,
        Wirkungsbereich::Leiste => 1,
        Wirkungsbereich::Dateibereiche => 2,
        Wirkungsbereich::Editor => 3,
        Wirkungsbereich::Tabbereich => 4,
        Wirkungsbereich::Navigator => 5,
        Wirkungsbereich::Vorschau => 6,
        Wirkungsbereich::Ueberall => 7,
    }
}

/// Jeder Wirkungsbereich, den der Quelltext der Aufzaehlung nennt, mit seiner
/// Zeile in [`ACHT_BESCHRIFTUNGEN`].
///
/// **Die Varianten kommen aus dem Quelltext und nicht aus dem Feld.** Bis zum
/// 260828 liefen die drei Beschriftungsproben ueber das Feld selbst, und ein
/// Wert ohne Feldzeile blieb von allen dreien unberuehrt; sein Doc-Kommentar
/// behauptete das Gegenteil
/// (`shared/issues/260826-1302_*_ein-achter-wirkungsbereich-uebersetzt-ohne-eintrag-im-beschriftungsfeld-der-doc-kommentar-sagt-das-gegenteil.md`).
/// Diese Funktion ist der eine Weg der drei Proben zu ihren Werten: sie
/// bricht ab, wenn eine Variante keine Zeile im Feld hat oder eine Zeile keine
/// Variante benennt, und liefert sonst jede Variante mit ihrer Zeile.
///
/// Der Vergleich laeuft ueber `{:?}`, weil [`varianten_der_aufzaehlung`] Namen
/// liefert und keine Werte; `Debug` einer Variante ohne Daten ist ihr Name.
fn jeder_wirkungsbereich_im_quelltext() -> Vec<(Wirkungsbereich, &'static str)> {
    let varianten = varianten_der_aufzaehlung("krk-core/src/tasten/belegung.rs", "Wirkungsbereich");
    let mut gefunden = Vec::with_capacity(varianten.len());
    for name in &varianten {
        let zeilen: Vec<(Wirkungsbereich, &str)> = ACHT_BESCHRIFTUNGEN
            .into_iter()
            .filter(|(bereich, _)| format!("{bereich:?}") == *name)
            .collect();
        assert_eq!(
            zeilen.len(),
            1,
            "Wirkungsbereich::{name} steht {}-mal im Beschriftungsfeld und nicht genau einmal",
            zeilen.len()
        );
        gefunden.push(zeilen[0]);
    }
    for (bereich, _) in ACHT_BESCHRIFTUNGEN {
        assert!(
            varianten.contains(&format!("{bereich:?}")),
            "{bereich:?} steht im Beschriftungsfeld, aber nicht in der Aufzaehlung"
        );
    }
    gefunden
}

/// Jeder der acht Bereiche traegt die Beschriftung, die C3 ihm gibt.
///
/// Ausgeschrieben und ohne Legende: die Datei nennt "Dateifenster, Leiste,
/// Vorschau und Git-Bereich" und nicht "Navigator". Wer einen dieser Texte
/// aendert, aendert
/// den Text, den der Nutzer in `~/Downloads/KRK-Tastenbelegung.md` liest, und
/// diese Probe ist die Stelle, an der er es merkt.
#[test]
fn jeder_wirkungsbereich_traegt_seine_beschriftung() {
    for (bereich, erwartet) in jeder_wirkungsbereich_im_quelltext() {
        assert_eq!(
            stelle_in_den_acht(bereich),
            ACHT_BESCHRIFTUNGEN
                .iter()
                .position(|(anderer, _)| *anderer == bereich)
                .expect("der Bereich steht nicht im Feld"),
            "{bereich:?} steht im Feld an einer anderen Stelle als in der Fallunterscheidung"
        );
        assert_eq!(
            bereich.beschriftung(),
            erwartet,
            "{bereich:?} traegt eine andere Beschriftung, als C3 sie nennt"
        );
    }
}

/// Keine zwei Bereiche tragen dieselbe Beschriftung.
///
/// Zwei gleiche Texte waeren eine Spalte, die zwei verschiedene Regeln gleich
/// benennt: wer "Dateifenster" liest, koennte nicht mehr sagen, ob der Befehl
/// den Fokus dort braucht oder ueberall wirkt. Der Uebersetzer sieht das nicht,
/// denn zwei Zweige duerfen dieselbe Zeichenkette liefern.
#[test]
fn keine_zwei_wirkungsbereiche_teilen_sich_eine_beschriftung() {
    let alle = jeder_wirkungsbereich_im_quelltext();
    for (stelle, (bereich, beschriftung)) in alle.iter().copied().enumerate() {
        for (anderer, weitere) in alle.iter().copied().skip(stelle + 1) {
            assert_ne!(bereich, anderer, "{bereich:?} steht zweimal im Feld");
            assert_ne!(
                beschriftung, weitere,
                "{bereich:?} und {anderer:?} tragen beide die Beschriftung {beschriftung:?}"
            );
        }
    }
}

/// Keine Beschriftung ist leer, und keine traegt einen senkrechten Strich.
///
/// Beides sind Zusagen an die Ausgabe und nicht an die Aufzaehlung: eine leere
/// dritte Zelle ist in der Datei die Auskunft "hier ist nichts entschieden",
/// und ein `|` in einer Zelle zerbraeche die Pipe-Tabelle. Ein Bereich, der
/// eines von beiden mitbraechte, machte aus einer Zusage einen Zufall.
#[test]
fn keine_beschriftung_ist_leer_oder_traegt_einen_senkrechten_strich() {
    for (bereich, beschriftung) in jeder_wirkungsbereich_im_quelltext() {
        assert!(
            !beschriftung.is_empty(),
            "{bereich:?} traegt eine leere Beschriftung"
        );
        assert!(
            !beschriftung.contains('|'),
            "{bereich:?} traegt einen senkrechten Strich und zerbraeche die Tabelle"
        );
    }
}

/// Die Kennungen, die die Editor-Runde der Belegungsdatei hinzugefuegt hat,
/// stehen darin.
///
/// **Der Name nennt keine Zahl, und die Probe zaehlt nicht.** Bis zum 260812
/// hiess sie `die_auslieferungsbelegung_fuehrt_vierundsiebzig_funktionen` und
/// verglich die Laenge der Belegung mit einem hingeschriebenen 74. Die Zusage
/// dahinter — die Kopfzeile von `resources/default-keymap.toml` stimmt mit dem
/// Inhalt der Datei ueberein — traegt
/// `die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` in
/// `crates/krk-core/src/tasten/belegung.rs` vollstaendig und ohne Literal: sie
/// liest die Kopfzeile und zaehlt selbst nach. Zwei Proben fuer eine Zusage
/// hiessen, dass jeder neue Eintrag in der Belegungsdatei eine davon umbenennt
/// (`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/
/// 260812-0533_*_drei-proben-stehen-gegen-die-neuen-belegungseintraege-….md`).
///
/// **Was hier bleibt, ist die Anwesenheit der Kennungen**, und sie steht
/// nirgends sonst in dieser Kiste: `jede_kennung_der_kommandos_steht_in_der_
/// auslieferungsbelegung` erreicht elf der dreizehn, denn `text_rueckgaengig`
/// und `text_wiederholen` tragen kein Kommando — das Menue stellt sie zu.
///
/// **Eine der dreizehn steht unter einem juengeren Namen**, und die Ueberschrift
/// darueber ist insoweit zu lesen wie geschrieben: den **Eintrag**
/// `editor_rundweg` hat die Editor-Runde angelegt, seinen Namen nicht. Sie hiess
/// bis zum 260823-0942 `editor_aus_vorschau`; umbenannt hat sie der
/// Nutzerentscheid jenes Tages, der `cmd+e` zum Rundweg gemacht hat
/// (`shared/decisions/260820-1034_*_wie-kommt-eine-taste-zum-umschalten-zwischen-editor-und-vorschau.md`).
/// Wer fragt, was die Editor-Runde geliefert hat, findet den heutigen Namen in
/// keinem ihrer Datensaetze.
#[test]
fn die_kennungen_der_editor_runde_stehen_in_der_auslieferungsbelegung() {
    let belegung = Belegung::auslieferung();
    for kennung in [
        "editor_rundweg",
        "fokus_editor",
        "editor_schliessen",
        "editor_ansicht_umschalten",
        "editor_sichern",
        "editor_zeile_springen",
        "editor_suchen",
        "editor_weitersuchen",
        "editor_rueckwaerts_suchen",
        "editor_ersetzen",
        "editor_alle_ersetzen",
        "text_rueckgaengig",
        "text_wiederholen",
    ] {
        assert!(
            belegung.funktion(kennung).is_some(),
            "die Auslieferungsbelegung kennt {kennung} nicht"
        );
    }
}

// Hier stand bis zum 260810-0822 die Probe
// `keine_neue_kombination_liegt_auf_den_beiden_wandernden_stellen`. Sie hielt
// fest, dass keine vom Ereignisabgriff zugestellte Kombination auf `kVK_ANSI_Y`
// oder `kVK_ANSI_Z` liegt. Ihr Grund ist mit S2 weggefallen: der Abgriff
// schlaegt Buchstaben und Ziffern seither ueber das gemeldete Zeichen nach,
// keine Stelle wandert mehr, und die Zusage verbot kuenftigen Runden zwei
// Buchstaben ohne Grund. Was sie an Sache trug, tragen
// `auf_einer_deutschen_tastatur_findet_die_aufschrift_y_die_vorschau` und
// `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte` oben,
// und zwar an der Nachschlagart selbst statt an einer Vorsichtsregel.
//
// Der Datensatz ist
// `issues/260809-1746_*_die-probe-auf-die-wandernden-stellen-hat-ihren-grund-verloren.md`,
// das Abnahmekriterium von S6 im Plan ist in derselben Aenderung nachgezogen.

/// Die sieben Funktionen aus C5 sind gebaut, und keine steht nur in der Datei.
#[test]
fn die_sieben_befehle_der_leiste_sind_gebaut() {
    let belegung = Belegung::auslieferung();
    for kennung in [
        "lesezeichen_anlegen",
        "lesezeichen_umbenennen",
        "lesezeichen_loeschen",
        "lesezeichen_hoch",
        "lesezeichen_runter",
        "fokus_leiste",
        "fokus_dateifenster",
    ] {
        let Some(funktion) = belegung.funktion(kennung) else {
            panic!("die Auslieferungsbelegung kennt {kennung} nicht");
        };
        assert!(
            funktion.kommando().is_some(),
            "{kennung} steht in der Belegung, hat aber kein Kommando"
        );
    }
}
