//! Abnahme der Belegungsmaschine (Schritt 11 des Plans).
//!
//! Alle Pruefungen laufen ohne Fenster und ohne AppKit. Die, die eine
//! `keymap.toml` brauchen, legen ihren eigenen Ablageordner unter dem
//! Temporaerverzeichnis an und fassen das echte Benutzerverzeichnis nicht an.
//!
//! Schritt 20 laesst die Pruefungen der Belegungsansicht in diese Datei
//! hineinwachsen; deshalb waehlt das Abnahmekommando das Testprogramm mit
//! `--test belegung` und filtert nicht ueber Pruefungsnamen.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use krk_core::ablage::{Ablage, Ablageort, Datei};
use krk_core::tasten::belegung::{self, Belegung, Belegungsfehler, Zuweisungsfehler};
use krk_core::tasten::normalisierung::roh;
use krk_core::tasten::parser::{self, Herkunft};
use krk_core::tasten::{Kombination, Kommando, ModMaske, Nachschlag, Tastendruck};

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
            "krk-belegung-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&pfad);
        fs::create_dir_all(&pfad).expect("Pruefordner laesst sich nicht anlegen");
        Self { pfad }
    }

    fn pfad(&self) -> &Path {
        &self.pfad
    }

    /// Eine Ablage in diesem Ordner, mit dem gegebenen Inhalt von
    /// `keymap.toml`.
    fn ablage_mit(&self, keymap: &str) -> Ablage {
        let ablage =
            Ablage::oeffnen(Ablageort::an(self.pfad())).expect("die Ablage laesst sich oeffnen");
        fs::write(ablage.pfad(Datei::Belegung), keymap).expect("keymap.toml laesst sich schreiben");
        ablage
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.pfad);
    }
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

/// Die Kombination zu einer Zeichenkette, oder ein Abbruch mit klarer Meldung.
fn kombi(text: &str) -> Kombination {
    match Kombination::lesen(text) {
        Ok(kombination) => kombination,
        Err(fehler) => panic!("\"{text}\" ist keine Kombination: {fehler}"),
    }
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

#[test]
fn jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste() {
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
        // C3: jede Funktion ausser der fuer den Editor reservierten traegt
        // mindestens eine Kombination.
        match funktion.reserviert_fuer() {
            Some(_) => assert!(
                funktion.tasten().is_empty(),
                "{} ist reserviert und traegt trotzdem eine Taste",
                funktion.kennung()
            ),
            None => assert!(
                !funktion.tasten().is_empty(),
                "{} traegt keine Kombination",
                funktion.kennung()
            ),
        }
    }
}

#[test]
fn die_drei_ab_werk_freien_kombinationen_kommen_nicht_vor() {
    // C3: Umschalt+Entf, Cmd+C und Cmd+V bleiben unbelegt.
    let belegung = Belegung::auslieferung();
    for text in ["shift+delete", "cmd+c", "cmd+v"] {
        let druck = kombi(text).tastendruck();
        assert!(
            matches!(
                belegung.nachschlag(druck),
                Nachschlag::Sprungmarke | Nachschlag::Unbelegt
            ),
            "{text} ist ab Werk belegt"
        );
    }
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
    let mit_function = Tastendruck::aus_ereignis(99, roh::FUNKTION);
    let ohne_function = Tastendruck::aus_ereignis(99, 0);

    let Nachschlag::Funktion(mit) = belegung.nachschlag(mit_function) else {
        panic!("F3 mit gesetztem function trifft keine Funktion");
    };
    let Nachschlag::Funktion(ohne) = belegung.nachschlag(ohne_function) else {
        panic!("F3 ohne gesetztes function trifft keine Funktion");
    };

    assert_eq!(mit.kennung(), ohne.kennung());
    assert_eq!(mit.kennung(), "vorschau_umschalten");
}

#[test]
fn beide_ausgelieferten_wege_treffen_dieselbe_funktion() {
    // Die sechs Zeilen der C3-Tabelle: Funktionstaste und Cmd-Kuerzel zeigen
    // auf eine Funktion und stehen in einer Zeile der Belegungsansicht.
    let belegung = Belegung::auslieferung();
    let paare = [
        ("f3", "cmd+y", "vorschau_umschalten"),
        ("f5", "shift+cmd+k", "kopieren"),
        ("f6", "shift+cmd+v", "verschieben"),
        ("f7", "shift+cmd+n", "ordner_anlegen"),
        ("f8", "opt+cmd+delete", "endgueltig_loeschen"),
        ("delete", "cmd+delete", "in_papierkorb"),
    ];
    for (funktionstaste, kuerzel, kennung) in paare {
        for text in [funktionstaste, kuerzel] {
            let Nachschlag::Funktion(funktion) = belegung.nachschlag(kombi(text).tastendruck())
            else {
                panic!("{text} trifft keine Funktion");
            };
            assert_eq!(
                funktion.kennung(),
                kennung,
                "{text} trifft die falsche Funktion"
            );
        }
    }
}

#[test]
fn ein_unbelegter_buchstabe_ohne_zusatztaste_faellt_auf_die_sprungmarke() {
    // C2: Tippt der Nutzer Buchstaben ohne Zusatztaste, springt die Auswahl auf
    // den ersten Eintrag, dessen Name so beginnt. Kein Buchstabe der
    // Auslieferungsbelegung ist ohne Zusatztaste belegt.
    let belegung = Belegung::auslieferung();
    for buchstabe in 'a'..='z' {
        let Some(code) = parser::code_von(&buchstabe.to_string()) else {
            panic!("die Tabelle kennt {buchstabe} nicht");
        };
        let druck = Tastendruck::neu(code, ModMaske::LEER);
        assert_eq!(
            belegung.nachschlag(druck),
            Nachschlag::Sprungmarke,
            "{buchstabe} faellt nicht auf die Sprungmarke durch"
        );
    }
}

#[test]
fn eine_unbelegte_kombination_mit_zusatztaste_faellt_nicht_auf_die_sprungmarke() {
    // Die Sprungmarke tippt Anfangsbuchstaben. Cmd+Q ist kein Anfangsbuchstabe,
    // sondern ein Kuerzel des Systems, und muss weitergehen duerfen.
    let belegung = Belegung::auslieferung();
    assert_eq!(
        belegung.nachschlag(kombi("cmd+q").tastendruck()),
        Nachschlag::Unbelegt
    );
}

#[test]
fn jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste() {
    let belegung = Belegung::auslieferung();
    let erwartet = [
        ("up", Kommando::AuswahlHoch),
        ("down", Kommando::AuswahlRunter),
        ("pageup", Kommando::SeiteHoch),
        ("pagedown", Kommando::SeiteRunter),
        ("return", Kommando::Oeffnen),
    ];
    for (text, kommando) in erwartet {
        let Nachschlag::Funktion(funktion) = belegung.nachschlag(kombi(text).tastendruck()) else {
            panic!("{text} trifft keine Funktion");
        };
        assert_eq!(funktion.kommando(), Some(kommando));
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
// Zuweisen, Konflikt und Zuruecksetzen
// ---------------------------------------------------------------------------

#[test]
fn eine_bereits_vergebene_kombination_liefert_einen_konflikt_mit_dem_namen_der_anderen_funktion() {
    let mut belegung = Belegung::auslieferung();
    let kombination = kombi("shift+cmd+k"); // gehoert dem Kopieren

    let ergebnis = belegung.zuweisen("verschieben", kombination);

    let Err(Zuweisungsfehler::Konflikt(konflikt)) = ergebnis else {
        panic!("die doppelte Zuweisung lieferte keinen Konflikt: {ergebnis:?}");
    };
    assert_eq!(konflikt.kombination, kombination);
    assert_eq!(konflikt.andere.kennung, "kopieren");
    assert_eq!(konflikt.andere.name, "In das andere Fenster kopieren");
    assert_eq!(konflikt.bewerber.kennung, "verschieben");
    assert!(
        konflikt
            .to_string()
            .contains("In das andere Fenster kopieren"),
        "die Meldung nennt die andere Funktion nicht: {konflikt}"
    );
    // Die Belegung bleibt unveraendert; nichts wird stillschweigend
    // ueberschrieben.
    assert_eq!(belegung, Belegung::auslieferung());
}

#[test]
fn eine_zweite_kombination_an_derselben_funktion_ist_kein_konflikt() {
    let mut belegung = Belegung::auslieferung();
    let neue = kombi("ctrl+k");

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
    for ausgeliefert in vorher.tasten() {
        assert!(
            funktion.tasten().contains(ausgeliefert),
            "{ausgeliefert} ist beim Zuweisen von ctrl+k verlorengegangen"
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
    let schon_da = kombi("f5");

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
    assert_eq!(belegung.zuweisen("kopieren", kombi("ctrl+k")), Ok(()));
    assert_ne!(belegung, Belegung::auslieferung());

    belegung.zuruecksetzen();

    assert_eq!(belegung, Belegung::auslieferung());
    let Nachschlag::Funktion(funktion) = belegung.nachschlag(kombi("f5").tastendruck()) else {
        panic!("F5 trifft nach dem Zuruecksetzen keine Funktion");
    };
    assert_eq!(funktion.kennung(), "kopieren");
    assert!(matches!(
        belegung.nachschlag(kombi("ctrl+k").tastendruck()),
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

    let geladen = belegung::laden(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    assert!(
        !geladen.ist_ersetzt(),
        "der erste Start ist keine Meldung wert"
    );
}

#[test]
fn die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht() {
    let ordner = Pruefordner::neu("ersetzen");
    let ablage = ordner.ablage_mit(
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["ctrl+c"]
"#,
    );

    let geladen = belegung::laden(&ablage);
    assert!(!geladen.ist_ersetzt(), "die Datei ist gueltig");
    let belegung = geladen.wert;

    // Der neue Weg gilt.
    let Nachschlag::Funktion(funktion) = belegung.nachschlag(kombi("ctrl+c").tastendruck()) else {
        panic!("ctrl+c trifft keine Funktion");
    };
    assert_eq!(funktion.kennung(), "kopieren");
    // Die ausgelieferten Wege derselben Funktion sind fort: ersetzt, nicht
    // ergaenzt.
    assert!(matches!(
        belegung.nachschlag(kombi("f5").tastendruck()),
        Nachschlag::Unbelegt | Nachschlag::Sprungmarke
    ));
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
    assert_eq!(belegung.zuweisen("kopieren", kombi("ctrl+k")), Ok(()));

    belegung
        .sichern(&ablage)
        .expect("die Belegung laesst sich sichern");
    let geladen = belegung::laden(&ablage);

    assert!(!geladen.ist_ersetzt());
    assert_eq!(geladen.wert, belegung);
}

#[test]
fn eine_unbekannte_funktion_in_der_nutzerdatei_fuehrt_zum_auslieferungszustand() {
    let ordner = Pruefordner::neu("unbekannt");
    let ablage = ordner.ablage_mit(
        r#"
[[funktion]]
id = "kaffee_kochen"
name = "Kaffee kochen"
tasten = ["ctrl+c"]
"#,
    );

    let geladen = belegung::laden(&ablage);

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
    let ablage = ordner.ablage_mit(
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
tasten = ["cmd+shift+k"]
"#,
    );

    let geladen = belegung::laden(&ablage);

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
    let ablage = ordner.ablage_mit(
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

    let geladen = belegung::laden(&ablage);

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
    let ablage = ordner.ablage_mit("[[funktion]\nid = \"kopieren\"\n");

    let geladen = belegung::laden(&ablage);

    assert_eq!(geladen.wert, Belegung::auslieferung());
    assert!(geladen.ist_ersetzt());
}

#[test]
fn ein_unbekanntes_feld_in_der_nutzerdatei_bleibt_nicht_unbemerkt() {
    // Ein Tippfehler im Feldnamen wuerde sonst still ignoriert, und der Nutzer
    // suchte die Wirkung einer Zeile, die niemand liest.
    let ordner = Pruefordner::neu("feld");
    let ablage = ordner.ablage_mit(
        r#"
[[funktion]]
id = "kopieren"
name = "In das andere Fenster kopieren"
taste = ["ctrl+c"]
"#,
    );

    let geladen = belegung::laden(&ablage);

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
