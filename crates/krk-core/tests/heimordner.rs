//! Abnahme des Heimordners `~/krkhome/`: Erkennung und Form der Eintraege
//! (Schritt 1.1), Anlegen und Uebernahme der alten Zettel (Schritt 1.2) und die
//! Handlungen an Aufgaben (Schritt 3.1, Faehigkeit C6) und an Notizen
//! (Schritt 4.1, Faehigkeit C5) des
//! Plans `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`,
//! Faehigkeiten C2 und C3 des Spec. Die Rennprobe zum exklusiven Anlegen steht
//! im Pruefmodul von `heimordner/bereitstellen.rs`, weil sie einen privaten
//! Einhaengepunkt braucht.
//!
//! Ohne Fenster. Die Erkennung wird an einem Pruefordner geprueft, der die
//! Rolle des Benutzerverzeichnisses spielt; das echte Benutzerverzeichnis
//! beruehrt keine Probe.
//!
//! **Der Pruefordner wird vor dem Gebrauch kanonisch gemacht.** Er liegt unter
//! `/var/folders/…`, und `/var` ist auf macOS ein Verweis auf `/private/var`.
//! Ohne diesen Schritt gaebe `canonicalize` eine Schreibweise zurueck, unter
//! der die Probe den Zielordner gar nicht anspricht, und sie paesse eine
//! Eigenschaft des Temporaerverzeichnisses statt der Erkennung. Das echte
//! Benutzerverzeichnis unter `/Users` fuehrt keinen solchen Verweis. Dass eine
//! dritte Schreibweise nicht erkannt wird, ist entschieden und steht im
//! Modulkopf von `heimordner/mod.rs`.

mod gemeinsam;

use std::fs;
use std::path::{Path, PathBuf};

use gemeinsam::Pruefordner;
use krk_core::heimordner::eintraege::{
    Abweisung, Aufgaben, Notiz, Notizen, Richtung, aufgabe_in_grundform, aufgaben, aufgabenzeile,
    ist_themenzeile, notizen,
};
use krk_core::heimordner::{
    ALTE_ZETTEL, Bereitstellung, Heimordner, Hindernis, ORDNERNAME, Sonderdatei, Uebernahmeausgang,
    Zettelbefund, bereitstellen,
};

/// Der Pruefordner in der Schreibweise, die `canonicalize` fuer ihn liefert.
fn kanonisch(ordner: &Pruefordner) -> PathBuf {
    fs::canonicalize(ordner.pfad()).expect("der Pruefordner laesst sich nicht aufloesen")
}

// ---------------------------------------------------------------------------
// Die Erkennung
// ---------------------------------------------------------------------------

/// C2.6: ueber den Verweis und ueber sein Ziel dieselbe Antwort, vor und nach
/// dem Erneuern der aufgeloesten Form.
#[test]
fn verweis_und_ziel_geben_dieselbe_antwort_vor_und_nach_dem_erneuern() {
    let ordner = Pruefordner::neu("heim-verweis");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("ablage-ziel");
    fs::create_dir(&ziel).expect("Zielordner laesst sich nicht anlegen");
    std::os::unix::fs::symlink(&ziel, zuhause.join(ORDNERNAME))
        .expect("Verweis laesst sich nicht anlegen");

    let leicht = Heimordner::im_benutzerverzeichnis(&zuhause);
    let erneuert = leicht.aufgeloest_erneuern();

    for (stufe, heim) in [("leicht", &leicht), ("erneuert", &erneuert)] {
        for sorte in Sonderdatei::ALLE {
            let ueber_verweis = zuhause.join(ORDNERNAME).join(sorte.dateiname());
            let ueber_ziel = ziel.join(sorte.dateiname());
            assert_eq!(
                heim.sonderdatei(&ueber_verweis),
                Some(sorte),
                "{stufe}: {} ueber den Verweis nicht erkannt",
                sorte.dateiname()
            );
            assert_eq!(
                heim.sonderdatei(&ueber_ziel),
                Some(sorte),
                "{stufe}: {} ueber das Ziel nicht erkannt",
                sorte.dateiname()
            );
        }
        assert!(heim.ist(&zuhause.join(ORDNERNAME)), "{stufe}: Verweis");
        assert!(heim.ist(&ziel), "{stufe}: Ziel");
    }
}

/// Ein relatives Verweisziel wird gegen das Benutzerverzeichnis gesetzt und
/// lexikalisch bereinigt, ohne das Ziel zu beruehren.
#[test]
fn ein_relatives_verweisziel_wird_gegen_das_benutzerverzeichnis_gesetzt() {
    let ordner = Pruefordner::neu("heim-relativ");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("ablage-ziel");
    fs::create_dir(&ziel).expect("Zielordner laesst sich nicht anlegen");
    std::os::unix::fs::symlink("./umweg/../ablage-ziel", zuhause.join(ORDNERNAME))
        .expect("Verweis laesst sich nicht anlegen");

    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
    assert_eq!(
        heim.sonderdatei(&ziel.join("notes.txt")),
        Some(Sonderdatei::Notizen)
    );
}

/// Die leichte Form entsteht, auch wenn das Ziel des Verweises fehlt: sie
/// fragt allein den Verweis und nie sein Ziel.
#[test]
fn ein_verweis_ins_leere_wird_trotzdem_gelesen() {
    let ordner = Pruefordner::neu("heim-leer");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("gibt-es-nicht");
    std::os::unix::fs::symlink(&ziel, zuhause.join(ORDNERNAME))
        .expect("Verweis laesst sich nicht anlegen");

    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
    assert!(heim.ist(&ziel));
    // `canonicalize` scheitert am fehlenden Ziel, und die leichte Form bleibt.
    assert_eq!(heim.aufgeloest_erneuern(), heim);
}

/// Ein gewoehnlicher Ordner, und einer, den es noch nicht gibt: erkannt wird
/// die geschriebene Form, und das Erneuern aendert daran nichts.
#[test]
fn ohne_verweis_gilt_die_geschriebene_form() {
    let ordner = Pruefordner::neu("heim-ordner");
    let zuhause = kanonisch(&ordner);

    let fehlend = Heimordner::im_benutzerverzeichnis(&zuhause);
    assert!(fehlend.ist(&zuhause.join(ORDNERNAME)));
    assert_eq!(fehlend.aufgeloest_erneuern(), fehlend);

    fs::create_dir(zuhause.join(ORDNERNAME)).expect("Ordner laesst sich nicht anlegen");
    let vorhanden = Heimordner::im_benutzerverzeichnis(&zuhause).aufgeloest_erneuern();
    assert_eq!(
        vorhanden.sonderdatei(&zuhause.join(ORDNERNAME).join("tasks.txt")),
        Some(Sonderdatei::Aufgaben)
    );
}

/// Eine gleichnamige Datei in einem anderen Ordner ist keine Eintragsdatei,
/// und ein anderer Name im Heimordner auch nicht.
#[test]
fn gleichnamige_dateien_anderswo_und_andere_namen_sind_keine_eintragsdateien() {
    let zuhause = Path::new("/Users/probe");
    let heim = Heimordner::im_benutzerverzeichnis(zuhause);

    assert_eq!(heim.sonderdatei(Path::new("/Users/probe/notes.txt")), None);
    assert_eq!(
        heim.sonderdatei(Path::new("/Users/probe/krkhome/unter/notes.txt")),
        None
    );
    assert_eq!(
        heim.sonderdatei(Path::new("/Users/probe/krkhome-alt/tasks.txt")),
        None
    );
    assert_eq!(
        heim.sonderdatei(Path::new("/Users/probe/krkhome/notizen.txt")),
        None
    );
    assert_eq!(heim.sonderdatei(Path::new("/Users/probe/krkhome")), None);
    assert_eq!(
        heim.sonderdatei(Path::new("/Users/probe/krkhome/notes.txt")),
        Some(Sonderdatei::Notizen)
    );
}

/// Ein Schlussstrich und doppelte Trennstriche aendern die Antwort nicht.
#[test]
fn ein_schlussstrich_aendert_die_antwort_nicht() {
    let heim = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe"));

    assert!(heim.ist(Path::new("/Users/probe/krkhome")));
    assert!(heim.ist(Path::new("/Users/probe/krkhome/")));
    assert!(heim.ist(Path::new("/Users/probe//krkhome/")));
    assert!(!heim.ist(Path::new("/Users/probe")));
    assert_eq!(
        heim.sonderdatei(Path::new("/Users/probe/krkhome//tasks.txt")),
        Some(Sonderdatei::Aufgaben)
    );
}

/// Der Rumpf einer Methode aus `heimordner/mod.rs`, ohne Kommentarzeilen.
///
/// Der Rumpf endet an der ersten schliessenden Klammer auf der Einrueckung
/// einer Methode; die Doc-Kommentare stehen vor dem `fn` und kommen gar nicht
/// herein. Dieselbe Regel wie `rumpf` im Pruefmodul von
/// `krk-ui/src/appkit/anwendung.rs`, das ein Testziel nicht erreicht.
fn rumpf(inhalt: &str, name: &str) -> String {
    let kopf = format!("    pub fn {name}(");
    let beginn = inhalt
        .find(&kopf)
        .unwrap_or_else(|| panic!("{kopf} steht nicht in heimordner/mod.rs"));
    let rest = &inhalt[beginn..];
    let ende = rest
        .find("\n    }\n")
        .unwrap_or_else(|| panic!("der Rumpf von {name} endet nicht"));
    rest[..ende]
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// `ist` und `sonderdatei` stellen keinen Systemaufruf.
///
/// Gefragt werden sie je Lesevorgang eines Tabs auf dem Hauptfaden; ein
/// Dateisystemaufruf dort blockierte an einem haengenden Netzlaufwerk die
/// Ereignisschleife und traefe die Zeitzusagen. Das haelt kein Uebersetzer,
/// deshalb liest diese Probe die beiden Ruempfe.
///
/// **Was sie nicht sieht:** einen Systemaufruf in einer Hilfsfunktion, die
/// einer der Ruempfe ruft. Heute ruft `sonderdatei` allein `ist`, und `ist`
/// keine Funktion dieses Moduls; wer eine Hilfsfunktion einzieht, nimmt ihren
/// Namen hier auf.
#[test]
fn ist_und_sonderdatei_stellen_keinen_systemaufruf() {
    let inhalt = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/heimordner/mod.rs"
    ))
    .expect("heimordner/mod.rs ist nicht lesbar");
    let nadeln = [
        "fs::",
        "canonicalize",
        "metadata",
        "read_link",
        "read_dir",
        "exists(",
        "is_dir(",
        "is_file(",
        "is_symlink(",
        "File::",
    ];
    for name in ["ist", "sonderdatei"] {
        let rumpf = rumpf(&inhalt, name);
        // Ein leer geschnittener Rumpf bestuende jede Nadel; er muss mindestens
        // den Wert befragen, den er pruefen soll.
        assert!(
            rumpf.contains("self."),
            "der Rumpf von Heimordner::{name} ist nicht gelesen:\n{rumpf}"
        );
        for nadel in nadeln {
            assert!(
                !rumpf.contains(nadel),
                "der Rumpf von Heimordner::{name} nennt `{nadel}`:\n{rumpf}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Die Form der Eintraege
// ---------------------------------------------------------------------------

/// Staende in jeder grosszuegigen Schreibweise, mit Vorspann, fremden Zeilen,
/// ohne Schlussumbruch und leer.
const STAENDE: &[&str] = &[
    "",
    "\n",
    "- [ ] eins",
    "- [ ] eins\n- [x] zwei\n",
    "# Aufgaben\n\nVorspann\n- [ ] eins\n  fremd darunter\n* [X] zwei\n\t- [x] eingerueckt\n",
    "- [ ] ohne Schluss\nfremd ohne Schluss",
    "   * [ ] drei Leerzeichen\n-\t[ ] Tabulator\n- [ ]\n- [ ]x keine\n",
    "## Thema\nText\n### tiefer\n# flacher\n\n## Zweites\n",
    "Vorspann\n\n## Thema\nzwei\nZeilen",
    "ohne jeden Eintrag\nnur Text\n",
    "- [ ] Umlaute äöü und ein \r mitten\r\n",
];

/// C2.4: Lesen und Schreiben ergeben dieselben Bytes, als Aufgaben wie als
/// Notizen gelesen.
#[test]
fn lesen_und_schreiben_ergeben_dieselben_bytes() {
    for stand in STAENDE {
        assert_eq!(
            Aufgaben::lesen(stand).schreiben(),
            *stand,
            "als Aufgaben: {stand:?}"
        );
        assert_eq!(
            Notizen::lesen(stand).schreiben(),
            *stand,
            "als Notizen: {stand:?}"
        );
    }
}

/// Die grosszuegige Lesart der Aufgabenzeile, Fall fuer Fall.
#[test]
fn die_aufgabenzeile_wird_grosszuegig_gelesen() {
    let gelesen = |zeile| aufgabenzeile(zeile).map(|a| (a.erledigt, a.text));

    assert_eq!(gelesen("- [ ] a"), Some((false, "a")));
    assert_eq!(gelesen("- [x] b\n"), Some((true, "b")));
    assert_eq!(gelesen("- [X] c"), Some((true, "c")));
    assert_eq!(gelesen("* [ ] d"), Some((false, "d")));
    assert_eq!(gelesen("* [x] e"), Some((true, "e")));
    assert_eq!(gelesen("    - [ ] f"), Some((false, "f")));
    assert_eq!(gelesen("\t* [X] g"), Some((true, "g")));
    assert_eq!(gelesen("-  [ ]  h"), Some((false, " h")));
    assert_eq!(gelesen("- [ ]"), Some((false, "")));

    assert_eq!(gelesen("-[ ] a"), None);
    assert_eq!(gelesen("- [ ]a"), None);
    assert_eq!(gelesen("- [y] a"), None);
    assert_eq!(gelesen("+ [ ] a"), None);
    assert_eq!(gelesen("- a"), None);
    assert_eq!(gelesen("## - [ ] a"), None);
}

/// Die Themenzeile beginnt mit `## `, und nur die.
#[test]
fn die_themenzeile_beginnt_mit_zwei_rauten_und_leerzeichen() {
    assert!(ist_themenzeile("## Thema"));
    assert!(ist_themenzeile("## Thema\n"));
    assert!(ist_themenzeile("## "));
    assert!(!ist_themenzeile("# Titel"));
    assert!(!ist_themenzeile("### tiefer"));
    assert!(!ist_themenzeile("##Thema"));
    assert!(!ist_themenzeile(" ## eingerueckt"));
}

/// Die Grundform, in der eine beruehrte Zeile geschrieben wird.
#[test]
fn die_grundform_ist_bindestrich_und_kleines_x() {
    assert_eq!(aufgabe_in_grundform(false, "a"), "- [ ] a");
    assert_eq!(aufgabe_in_grundform(true, "b"), "- [x] b");
    let offen = aufgabe_in_grundform(false, "rund");
    let gelesen = aufgabenzeile(&offen).expect("die Grundform ist eine Aufgabe");
    assert_eq!((gelesen.erledigt, gelesen.text), (false, "rund"));
}

/// Die Zerlegung: fremde Zeilen haengen an der Aufgabe darueber, der Vorspann
/// steht fuer sich; in `notes.txt` ist alles nach der Themenzeile Text.
#[test]
fn fremde_zeilen_haengen_am_eintrag_darueber() {
    let aufgaben = Aufgaben::lesen("Vorspann\n- [ ] a\nfremd\n## auch fremd\n* [x] b\n");
    assert_eq!(aufgaben.vorspann(), ["Vorspann\n"]);
    assert_eq!(aufgaben.bloecke().len(), 2);
    assert_eq!(aufgaben.bloecke()[0].kopf(), "- [ ] a\n");
    assert_eq!(
        aufgaben.bloecke()[0].anhang(),
        ["fremd\n", "## auch fremd\n"]
    );
    assert_eq!(aufgaben.bloecke()[1].zeilen(), ["* [x] b\n"]);

    let notizen = Notizen::lesen("# Titel\n## Eins\n- [ ] kein Eintrag\n### tiefer\n## Zwei\n");
    assert_eq!(notizen.vorspann(), ["# Titel\n"]);
    assert_eq!(notizen.bloecke().len(), 2);
    assert_eq!(
        notizen.bloecke()[0].anhang(),
        ["- [ ] kein Eintrag\n", "### tiefer\n"]
    );
    assert_eq!(notizen.bloecke()[1].kopf(), "## Zwei\n");
}

/// C2.5: eine Aufgabe, ueber eine fremde Zeile hinweg verschoben, nimmt ihre
/// fremde Zeile mit, und der Vorspann bleibt oben.
#[test]
fn eine_verschobene_aufgabe_nimmt_ihre_fremde_zeile_mit() {
    let stand = "# Aufgaben\n- [ ] eins\n  gehoert zu eins\n* [X] zwei\n  gehoert zu zwei\n";

    let hoch = aufgaben::verschieben(stand, 1, Richtung::Hoch).expect("zwei steht nicht oben");
    assert_eq!(
        hoch.text,
        "# Aufgaben\n* [X] zwei\n  gehoert zu zwei\n- [ ] eins\n  gehoert zu eins\n"
    );
    assert_eq!(hoch.auswahl, Some(0));

    let runter = aufgaben::verschieben(stand, 0, Richtung::Runter).expect("eins steht nicht unten");
    assert_eq!(runter.text, hoch.text);
    assert_eq!(runter.auswahl, Some(1));

    // Zurueck an den alten Platz ergibt den alten Stand Byte fuer Byte.
    let zurueck = aufgaben::verschieben(&hoch.text, 0, Richtung::Runter).expect("zurueck");
    assert_eq!(zurueck.text, stand);
}

/// Am Rand und ausserhalb der Liste gibt es nichts zu verschieben.
#[test]
fn am_rand_gibt_es_nichts_zu_verschieben() {
    let stand = "- [ ] eins\n- [ ] zwei\n";
    assert_eq!(aufgaben::verschieben(stand, 0, Richtung::Hoch), None);
    assert_eq!(aufgaben::verschieben(stand, 1, Richtung::Runter), None);
    assert_eq!(aufgaben::verschieben(stand, 2, Richtung::Hoch), None);
    assert_eq!(aufgaben::verschieben("", 0, Richtung::Runter), None);
}

/// Der fehlende Schlussumbruch gehoert dem Dateiende: der nach oben gewanderte
/// Block bekommt einen, der neue letzte verliert seinen, und hin und zurueck
/// ergibt den alten Stand.
#[test]
fn der_fehlende_schlussumbruch_bleibt_am_dateiende() {
    let stand = "- [ ] eins\n- [x] zwei\nfremd ohne Schluss";

    let hoch = aufgaben::verschieben(stand, 1, Richtung::Hoch).expect("verschiebbar");
    assert_eq!(hoch.text, "- [x] zwei\nfremd ohne Schluss\n- [ ] eins");

    let zurueck = aufgaben::verschieben(&hoch.text, 0, Richtung::Runter).expect("zurueck");
    assert_eq!(zurueck.text, stand);
}

// ---------------------------------------------------------------------------
// Die Handlungen an Aufgaben (Schritt 3.1)
// ---------------------------------------------------------------------------

/// Ein Stand mit Vorspann, einer grosszuegig geschriebenen Aufgabe, fremden
/// Zeilen an zwei Aufgaben und einer Grundform-Aufgabe dazwischen.
const HANDLUNGSSTAND: &str = "\
# Aufgaben
- [ ] eins
  gehoert zu eins
\t* [X] zwei
- [ ] drei
  gehoert zu drei
";

/// Die Zeilen, in denen zwei Staende gleicher Zeilenzahl sich unterscheiden.
fn abweichende_zeilen(alt: &str, neu: &str) -> Vec<usize> {
    let alt: Vec<&str> = alt.split_inclusive('\n').collect();
    let neu: Vec<&str> = neu.split_inclusive('\n').collect();
    assert_eq!(alt.len(), neu.len(), "die Zeilenzahl hat sich geaendert");
    alt.iter()
        .zip(&neu)
        .enumerate()
        .filter(|(_, (a, n))| a != n)
        .map(|(nummer, _)| nummer)
        .collect()
}

/// C6.1: Hinzufuegen haengt eine offene Aufgabe in Grundform ans Ende, hinter
/// die fremden Zeilen der letzten, und waehlt sie.
#[test]
fn hinzufuegen_haengt_eine_offene_aufgabe_ans_ende() {
    let neu = aufgaben::hinzufuegen(HANDLUNGSSTAND, "vier").expect("einzeilig");
    assert_eq!(neu.text, format!("{HANDLUNGSSTAND}- [ ] vier\n"));
    assert_eq!(neu.auswahl, Some(3));
    assert_eq!(Aufgaben::lesen(&neu.text).bloecke().len(), 4);

    // Eine leere Datei bekommt die Zeile samt Umbruch, ein Vorspann ohne
    // Aufgabe bleibt oben, und ein leerer Text ist eine Aufgabe.
    let leer = aufgaben::hinzufuegen("", "erste").expect("einzeilig");
    assert_eq!(leer.text, "- [ ] erste\n");
    assert_eq!(leer.auswahl, Some(0));
    let nur_vorspann = aufgaben::hinzufuegen("# Titel\n", "").expect("leer ist zulaessig");
    assert_eq!(nur_vorspann.text, "# Titel\n- [ ] \n");
    assert_eq!(nur_vorspann.auswahl, Some(0));
    let gelesen = Aufgaben::lesen(&nur_vorspann.text);
    assert_eq!(gelesen.vorspann(), ["# Titel\n"]);
    assert_eq!(
        aufgabenzeile(gelesen.bloecke()[0].kopf()).map(|a| a.text),
        Some("")
    );
}

/// Endet die Datei ohne Umbruch, bekommt die alte letzte Zeile einen, und die
/// neue Aufgabe endet ohne.
#[test]
fn hinzufuegen_wahrt_das_dateiende() {
    let neu = aufgaben::hinzufuegen("- [x] eins\nfremd ohne Schluss", "zwei").expect("einzeilig");
    assert_eq!(neu.text, "- [x] eins\nfremd ohne Schluss\n- [ ] zwei");
    assert_eq!(neu.auswahl, Some(1));
}

/// Ein Aufgabentext mit Umbruch wird abgewiesen, beim Hinzufuegen wie beim
/// Aendern, und kein Neustand entsteht.
#[test]
fn ein_umbruch_im_aufgabentext_wird_abgewiesen() {
    assert_eq!(
        aufgaben::hinzufuegen(HANDLUNGSSTAND, "zwei\nZeilen"),
        Err(Abweisung::UmbruchImAufgabentext)
    );
    assert_eq!(
        aufgaben::text_aendern(HANDLUNGSSTAND, 0, "zwei\nZeilen"),
        Err(Abweisung::UmbruchImAufgabentext)
    );
    assert_eq!(
        aufgaben::text_aendern(HANDLUNGSSTAND, 0, "Schluss\n"),
        Err(Abweisung::UmbruchImAufgabentext)
    );
    // Die Abweisung geht vor jeder anderen Antwort, auch ohne Aufgabe.
    assert_eq!(
        aufgaben::text_aendern("", 5, "a\nb"),
        Err(Abweisung::UmbruchImAufgabentext)
    );
    assert!(
        Abweisung::UmbruchImAufgabentext
            .meldung()
            .contains("Zeilenumbruch")
    );
}

/// C6.1: Textaendern schreibt genau die Aufgabenzeile in Grundform neu, laesst
/// den Erledigt-Zustand und jede andere Zeile.
#[test]
fn text_aendern_schreibt_allein_die_aufgabenzeile_neu() {
    let neu = aufgaben::text_aendern(HANDLUNGSSTAND, 1, "zwei neu")
        .expect("einzeilig")
        .expect("die Aufgabe gibt es");
    assert_eq!(
        neu.text,
        "# Aufgaben\n- [ ] eins\n  gehoert zu eins\n- [x] zwei neu\n- [ ] drei\n  gehoert zu drei\n"
    );
    assert_eq!(neu.auswahl, Some(1));
    assert_eq!(abweichende_zeilen(HANDLUNGSSTAND, &neu.text), [3]);
}

/// Ein unveraenderter Text und eine fehlende Aufgabe ergeben keinen Neustand;
/// die grosszuegig geschriebene Zeile bleibt damit roh.
#[test]
fn text_aendern_ohne_aenderung_ergibt_nichts() {
    assert_eq!(aufgaben::text_aendern(HANDLUNGSSTAND, 1, "zwei"), Ok(None));
    assert_eq!(aufgaben::text_aendern(HANDLUNGSSTAND, 3, "vier"), Ok(None));
    assert_eq!(aufgaben::text_aendern("", 0, "a"), Ok(None));
}

/// C6.3: Abhaken aendert genau eine Zeile und nicht die Stelle der Aufgabe;
/// eine eingerueckte `* [X]`-Aufgabe kommt geoeffnet als `- [ ] ` in Grundform
/// zurueck, ihre Nachbarn unveraendert.
#[test]
fn abhaken_schreibt_genau_eine_zeile_in_grundform() {
    let geoeffnet = aufgaben::abhaken(HANDLUNGSSTAND, 1).expect("die Aufgabe gibt es");
    assert_eq!(abweichende_zeilen(HANDLUNGSSTAND, &geoeffnet.text), [3]);
    assert_eq!(
        geoeffnet.text.split_inclusive('\n').nth(3),
        Some("- [ ] zwei\n")
    );
    assert_eq!(geoeffnet.auswahl, Some(1));

    // Die Stelle bleibt: dieselben Aufgaben in derselben Reihenfolge.
    let texte = |stand: &str| -> Vec<String> {
        Aufgaben::lesen(stand)
            .bloecke()
            .iter()
            .map(|block| {
                aufgabenzeile(block.kopf())
                    .expect("Aufgabe")
                    .text
                    .to_owned()
            })
            .collect()
    };
    assert_eq!(texte(&geoeffnet.text), texte(HANDLUNGSSTAND));

    let erledigt = aufgaben::abhaken(HANDLUNGSSTAND, 0).expect("die Aufgabe gibt es");
    assert_eq!(abweichende_zeilen(HANDLUNGSSTAND, &erledigt.text), [1]);
    assert_eq!(
        erledigt.text.split_inclusive('\n').nth(1),
        Some("- [x] eins\n")
    );

    // Zweimal abgehakt steht die Grundform-Aufgabe wieder Byte fuer Byte da.
    let zurueck = aufgaben::abhaken(&erledigt.text, 0).expect("die Aufgabe gibt es");
    assert_eq!(zurueck.text, HANDLUNGSSTAND);

    assert_eq!(aufgaben::abhaken(HANDLUNGSSTAND, 3), None);
}

/// Die letzte Zeile ohne Umbruch bleibt beim Abhaken ohne.
#[test]
fn abhaken_wahrt_das_dateiende() {
    let neu = aufgaben::abhaken("- [ ] eins\n  * [ ] zwei", 1).expect("die Aufgabe gibt es");
    assert_eq!(neu.text, "- [ ] eins\n- [x] zwei");
}

/// C6.1: Loeschen entfernt allein die Aufgabenzeile; ihre fremden Zeilen
/// haengen danach an der Aufgabe darueber, und die nachrueckende ist gewaehlt.
#[test]
fn loeschen_entfernt_allein_die_aufgabenzeile() {
    let neu = aufgaben::loeschen(HANDLUNGSSTAND, 2).expect("die Aufgabe gibt es");
    assert_eq!(
        neu.text,
        "# Aufgaben\n- [ ] eins\n  gehoert zu eins\n\t* [X] zwei\n  gehoert zu drei\n"
    );
    assert_eq!(neu.auswahl, Some(1));
    let gelesen = Aufgaben::lesen(&neu.text);
    assert_eq!(gelesen.bloecke()[1].anhang(), ["  gehoert zu drei\n"]);

    // Die erste geloescht: ihre fremde Zeile faellt in den Vorspann, die
    // zweite rueckt an die Stelle und ist gewaehlt.
    let erste = aufgaben::loeschen(HANDLUNGSSTAND, 0).expect("die Aufgabe gibt es");
    assert_eq!(
        erste.text,
        "# Aufgaben\n  gehoert zu eins\n\t* [X] zwei\n- [ ] drei\n  gehoert zu drei\n"
    );
    assert_eq!(erste.auswahl, Some(0));
    assert_eq!(
        Aufgaben::lesen(&erste.text).vorspann(),
        ["# Aufgaben\n", "  gehoert zu eins\n"]
    );

    // Die letzte und einzige: keine Auswahl bleibt.
    let einzige = aufgaben::loeschen("- [ ] allein\n", 0).expect("die Aufgabe gibt es");
    assert_eq!(einzige.text, "");
    assert_eq!(einzige.auswahl, None);

    assert_eq!(aufgaben::loeschen(HANDLUNGSSTAND, 3), None);
    assert_eq!(aufgaben::loeschen("", 0), None);
}

/// Die letzte Zeile ohne Umbruch geloescht: die neue letzte verliert ihren.
#[test]
fn loeschen_wahrt_das_dateiende() {
    let neu = aufgaben::loeschen("- [ ] eins\n- [ ] zwei", 1).expect("die Aufgabe gibt es");
    assert_eq!(neu.text, "- [ ] eins");
    assert_eq!(neu.auswahl, Some(0));
}

// ---------------------------------------------------------------------------
// Die Handlungen an Notizen (Schritt 4.1)
// ---------------------------------------------------------------------------

/// Ein Stand mit Vorspann, einer Notiz mit zwei Absaetzen und Trenner, einer
/// ohne Text und einer letzten mit `#`- und `###`-Zeilen im Text.
const NOTIZSTAND: &str = "\
# Notizen

## Eins
erste Zeile

zweiter Absatz

## Zwei
## Drei
# flach
### tief
";

/// Eine Notiz als Vergleichswert.
fn notiz<'a>(thema: &'a str, text: &str) -> Notiz<'a> {
    Notiz {
        thema,
        text: text.to_owned(),
    }
}

/// Die Notiz liest Thema und Text; die Leerzeilen am Ende sind Trenner und
/// nicht Text, die Leerzeile zwischen zwei Absaetzen ist Text.
#[test]
fn die_notiz_liest_thema_und_text_ohne_trenner() {
    let gelesen = Notizen::lesen(NOTIZSTAND);
    assert_eq!(gelesen.vorspann(), ["# Notizen\n", "\n"]);
    assert_eq!(
        gelesen.notiz(0),
        Some(notiz("Eins", "erste Zeile\n\nzweiter Absatz"))
    );
    assert_eq!(gelesen.notiz(1), Some(notiz("Zwei", "")));
    assert_eq!(gelesen.notiz(2), Some(notiz("Drei", "# flach\n### tief")));
    assert_eq!(gelesen.notiz(3), None);

    // Am Dateiende ohne Umbruch und mit leerem Thema.
    let ohne_schluss = Notizen::lesen("## \nText ohne Schluss");
    assert_eq!(ohne_schluss.notiz(0), Some(notiz("", "Text ohne Schluss")));
    assert_eq!(Notizen::lesen("## Kopf").notiz(0), Some(notiz("Kopf", "")));
}

/// C5.1: Hinzufuegen haengt Themenzeile und Text ans Ende, hinter den Text der
/// letzten Notiz, und waehlt die neue; der Vorspann bleibt oben.
#[test]
fn notiz_hinzufuegen_haengt_thema_und_text_ans_ende() {
    let neu = notizen::hinzufuegen(NOTIZSTAND, "Vier", "a\n\nb").expect("zulaessig");
    assert_eq!(neu.text, format!("{NOTIZSTAND}## Vier\na\n\nb\n"));
    assert_eq!(neu.auswahl, Some(3));
    let gelesen = Notizen::lesen(&neu.text);
    assert_eq!(gelesen.vorspann(), ["# Notizen\n", "\n"]);
    assert_eq!(gelesen.notiz(3), Some(notiz("Vier", "a\n\nb")));

    // Leere Datei, nur Vorspann, leerer Text, leeres Thema.
    let leer = notizen::hinzufuegen("", "Erste", "").expect("zulaessig");
    assert_eq!(leer.text, "## Erste\n");
    assert_eq!(leer.auswahl, Some(0));
    let nur_vorspann = notizen::hinzufuegen("# Titel\n", "", "").expect("zulaessig");
    assert_eq!(nur_vorspann.text, "# Titel\n## \n");
    assert_eq!(nur_vorspann.auswahl, Some(0));
    assert_eq!(
        Notizen::lesen(&nur_vorspann.text).notiz(0),
        Some(notiz("", ""))
    );

    // Schlussumbrueche des Textes sind Trenner und werden nicht geschrieben.
    let mit_schluss = notizen::hinzufuegen("", "T", "text\n\n").expect("zulaessig");
    assert_eq!(mit_schluss.text, "## T\ntext\n");
}

/// Endet die Datei ohne Umbruch, bekommt die alte letzte Zeile einen, und die
/// neue Notiz endet ohne.
#[test]
fn notiz_hinzufuegen_wahrt_das_dateiende() {
    let neu = notizen::hinzufuegen("## Eins\nText", "Zwei", "x\ny").expect("zulaessig");
    assert_eq!(neu.text, "## Eins\nText\n## Zwei\nx\ny");
    assert_eq!(neu.auswahl, Some(1));
    let ohne_text = notizen::hinzufuegen("Vorspann", "Zwei", "").expect("zulaessig");
    assert_eq!(ohne_text.text, "Vorspann\n## Zwei");
}

/// C5.2: eine Zeile mit `## ` im Notiztext und ein Umbruch im Thema werden
/// abgewiesen, beim Hinzufuegen wie beim Aendern, und die Meldung nennt den
/// Grund. Eine Abweisung ist kein Neustand, der Stand bleibt also, wie er war.
#[test]
fn eine_themenzeile_im_text_und_ein_umbruch_im_thema_werden_abgewiesen() {
    for text in ["## x", "davor\n## x", "a\n\n## \nb", "a\n## x\n"] {
        assert_eq!(
            notizen::hinzufuegen(NOTIZSTAND, "T", text),
            Err(Abweisung::ThemenzeileImNotiztext),
            "{text:?}"
        );
        assert_eq!(
            notizen::aendern(NOTIZSTAND, 0, "Eins", text),
            Err(Abweisung::ThemenzeileImNotiztext),
            "{text:?}"
        );
    }
    for thema in ["zwei\nZeilen", "Schluss\n"] {
        assert_eq!(
            notizen::hinzufuegen(NOTIZSTAND, thema, "t"),
            Err(Abweisung::UmbruchImThema)
        );
        assert_eq!(
            notizen::aendern(NOTIZSTAND, 1, thema, ""),
            Err(Abweisung::UmbruchImThema)
        );
    }
    // Die Abweisung geht vor jeder anderen Antwort, auch ohne Notiz.
    assert_eq!(
        notizen::aendern("", 7, "T", "## x"),
        Err(Abweisung::ThemenzeileImNotiztext)
    );

    // Zulaessig bleibt, was keine Themenzeile ist.
    for text in ["# flach", "### tief", "##x", " ## eingerueckt", "a ## b"] {
        assert!(
            notizen::hinzufuegen(NOTIZSTAND, "T", text).is_ok(),
            "{text:?}"
        );
    }

    assert!(
        Abweisung::ThemenzeileImNotiztext
            .meldung()
            .contains("„## “")
    );
    assert!(
        Abweisung::UmbruchImThema
            .meldung()
            .contains("Zeilenumbruch")
    );
    assert_ne!(
        Abweisung::UmbruchImThema.meldung(),
        Abweisung::UmbruchImAufgabentext.meldung()
    );
}

/// C5.1: ein geaendertes Thema schreibt allein die Themenzeile neu; Text und
/// Trenner bleiben Byte fuer Byte.
#[test]
fn notiz_aendern_schreibt_allein_die_themenzeile_wenn_nur_das_thema_neu_ist() {
    let neu = notizen::aendern(NOTIZSTAND, 0, "Eins neu", "erste Zeile\n\nzweiter Absatz")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(abweichende_zeilen(NOTIZSTAND, &neu.text), [2]);
    assert_eq!(neu.text.split_inclusive('\n').nth(2), Some("## Eins neu\n"));
    assert_eq!(neu.auswahl, Some(0));
}

/// C5.1: ein geaenderter Text ersetzt die Textzeilen, laesst Themenzeile und
/// Trenner, und jede andere Notiz bleibt.
#[test]
fn notiz_aendern_ersetzt_den_text_und_laesst_den_trenner() {
    let neu = notizen::aendern(NOTIZSTAND, 0, "Eins", "nur noch\neins")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(
        neu.text,
        "# Notizen\n\n## Eins\nnur noch\neins\n\n## Zwei\n## Drei\n# flach\n### tief\n"
    );
    assert_eq!(
        Notizen::lesen(&neu.text).notiz(0),
        Some(notiz("Eins", "nur noch\neins"))
    );

    // Eine Notiz ohne Text bekommt einen, beide zugleich geaendert.
    let beides = notizen::aendern(NOTIZSTAND, 1, "Zwo", "neu")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(
        beides.text,
        "# Notizen\n\n## Eins\nerste Zeile\n\nzweiter Absatz\n\n## Zwo\nneu\n## Drei\n# flach\n### tief\n"
    );
    assert_eq!(beides.auswahl, Some(1));

    // Der Text geleert: die Textzeilen fallen, der Trenner bleibt.
    let geleert = notizen::aendern(NOTIZSTAND, 0, "Eins", "")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(
        geleert.text,
        "# Notizen\n\n## Eins\n\n## Zwei\n## Drei\n# flach\n### tief\n"
    );
}

/// Die letzte Notiz ohne Schlussumbruch geaendert: das Dateiende bleibt.
#[test]
fn notiz_aendern_wahrt_das_dateiende() {
    let text = notizen::aendern("## A\nalt", 0, "A", "neu\nzwei")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(text.text, "## A\nneu\nzwei");
    let kopf = notizen::aendern("## A", 0, "A", "neu")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(kopf.text, "## A\nneu");
    let thema = notizen::aendern("## A\nText", 0, "B", "Text")
        .expect("zulaessig")
        .expect("die Notiz gibt es");
    assert_eq!(thema.text, "## B\nText");
}

/// Unveraendertes Thema und unveraenderter Text, eine fehlende Notiz und ein
/// Text, der sich allein in Schlussumbruechen unterscheidet, ergeben keinen
/// Neustand.
#[test]
fn notiz_aendern_ohne_aenderung_ergibt_nichts() {
    assert_eq!(
        notizen::aendern(NOTIZSTAND, 0, "Eins", "erste Zeile\n\nzweiter Absatz"),
        Ok(None)
    );
    assert_eq!(
        notizen::aendern(NOTIZSTAND, 0, "Eins", "erste Zeile\n\nzweiter Absatz\n\n"),
        Ok(None)
    );
    assert_eq!(notizen::aendern(NOTIZSTAND, 3, "Vier", ""), Ok(None));
    assert_eq!(notizen::aendern("", 0, "A", ""), Ok(None));
}

/// Rundlauf: jede Notiz jedes Standes mit ihren eigenen Werten geaendert ergibt
/// nichts, und was hinzugefuegt wurde, liest sich als dieselbe Notiz zurueck.
#[test]
fn notizen_laufen_rund() {
    for stand in STAENDE.iter().copied().chain([NOTIZSTAND]) {
        let gelesen = Notizen::lesen(stand);
        for index in 0..gelesen.bloecke().len() {
            let alt = gelesen.notiz(index).expect("jeder Block ist eine Notiz");
            assert_eq!(
                notizen::aendern(stand, index, alt.thema, &alt.text),
                Ok(None),
                "{stand:?}, Notiz {index}"
            );
        }
        for (thema, text) in [("T", ""), ("", "x"), ("mit äöü", "a\n\n\tb\n# c")] {
            let neu = notizen::hinzufuegen(stand, thema, text).expect("zulaessig");
            let auswahl = neu.auswahl.expect("die neue ist gewaehlt");
            assert_eq!(
                Notizen::lesen(&neu.text).notiz(auswahl),
                Some(notiz(thema, text)),
                "{stand:?}"
            );
            // Der alte Stand steht unveraendert davor, bis auf einen
            // ergaenzten Umbruch am alten Ende.
            assert!(neu.text.starts_with(stand), "{stand:?}");
        }
    }
}

/// C5.1: Loeschen nimmt die ganze Notiz, Themenzeile, Text und Trenner; der
/// Vorspann bleibt, und die nachrueckende ist gewaehlt.
#[test]
fn notiz_loeschen_nimmt_thema_text_und_trenner() {
    let erste = notizen::loeschen(NOTIZSTAND, 0).expect("die Notiz gibt es");
    assert_eq!(
        erste.text,
        "# Notizen\n\n## Zwei\n## Drei\n# flach\n### tief\n"
    );
    assert_eq!(erste.auswahl, Some(0));

    let letzte = notizen::loeschen(NOTIZSTAND, 2).expect("die Notiz gibt es");
    assert_eq!(
        letzte.text,
        "# Notizen\n\n## Eins\nerste Zeile\n\nzweiter Absatz\n\n## Zwei\n"
    );
    assert_eq!(letzte.auswahl, Some(1));

    let einzige = notizen::loeschen("# Titel\n## Allein\nText\n", 0).expect("die Notiz gibt es");
    assert_eq!(einzige.text, "# Titel\n");
    assert_eq!(einzige.auswahl, None);

    // Die letzte ohne Schlussumbruch: die neue letzte verliert ihren.
    let ohne_schluss = notizen::loeschen("## A\na\n## B\nb", 1).expect("die Notiz gibt es");
    assert_eq!(ohne_schluss.text, "## A\na");

    assert_eq!(notizen::loeschen(NOTIZSTAND, 3), None);
    assert_eq!(notizen::loeschen("nur Vorspann\n", 0), None);
}

/// C5.1: Verschieben nimmt Text und Trenner mit, der Vorspann bleibt oben, am
/// Rand gibt es nichts zu tun, und hin und zurueck ergibt den alten Stand.
#[test]
fn notiz_verschieben_nimmt_den_text_mit() {
    let runter = notizen::verschieben(NOTIZSTAND, 0, Richtung::Runter).expect("nicht unten");
    assert_eq!(
        runter.text,
        "# Notizen\n\n## Zwei\n## Eins\nerste Zeile\n\nzweiter Absatz\n\n## Drei\n# flach\n### tief\n"
    );
    assert_eq!(runter.auswahl, Some(1));
    let zurueck = notizen::verschieben(&runter.text, 1, Richtung::Hoch).expect("nicht oben");
    assert_eq!(zurueck.text, NOTIZSTAND);
    assert_eq!(zurueck.auswahl, Some(0));

    let ohne_schluss = "## A\na\n## B\nb";
    let hoch = notizen::verschieben(ohne_schluss, 1, Richtung::Hoch).expect("nicht oben");
    assert_eq!(hoch.text, "## B\nb\n## A\na");

    assert_eq!(notizen::verschieben(NOTIZSTAND, 0, Richtung::Hoch), None);
    assert_eq!(notizen::verschieben(NOTIZSTAND, 2, Richtung::Runter), None);
    assert_eq!(notizen::verschieben(NOTIZSTAND, 3, Richtung::Hoch), None);
    assert_eq!(notizen::verschieben("", 0, Richtung::Runter), None);
}

// ---------------------------------------------------------------------------
// Anlegen und Uebernahme (Schritt 1.2)
// ---------------------------------------------------------------------------

/// Ein Benutzerverzeichnis mit Ablageordner und zwei alten Zetteln, deren
/// Inhalt die Probe waehlt; `None` heisst: der Zettel fehlt.
///
/// Haelt den Pruefordner, damit `Drop` ihn erst am Ende der Probe abraeumt,
/// und merkt sich die Zettel, damit jede Probe sie am Ende unveraendert findet.
struct Lage {
    _ordner: Pruefordner,
    zuhause: PathBuf,
    ablage: PathBuf,
    zettel: [Option<&'static [u8]>; 2],
}

impl Lage {
    fn neu(zweck: &str, zettel: [Option<&'static [u8]>; 2]) -> Self {
        let ordner = Pruefordner::neu(zweck);
        let zuhause = kanonisch(&ordner);
        let ablage = zuhause.join("ablage");
        fs::create_dir(&ablage).expect("Ablageordner laesst sich nicht anlegen");
        for (alter, inhalt) in ALTE_ZETTEL.iter().zip(zettel) {
            if let Some(inhalt) = inhalt {
                fs::write(ablage.join(alter.datei), inhalt).expect("Zettel");
            }
        }
        Self {
            _ordner: ordner,
            zuhause,
            ablage,
            zettel,
        }
    }

    fn heim(&self) -> Heimordner {
        Heimordner::im_benutzerverzeichnis(&self.zuhause)
    }

    fn heimpfad(&self) -> PathBuf {
        self.zuhause.join(ORDNERNAME)
    }

    fn bereitstellen(&self) -> Result<Bereitstellung, Hindernis> {
        bereitstellen(&self.heim(), &self.ablage)
    }

    fn lesen(&self, sorte: Sonderdatei) -> String {
        fs::read_to_string(self.heimpfad().join(sorte.dateiname()))
            .unwrap_or_else(|_| panic!("{} fehlt", sorte.dateiname()))
    }

    /// C3.5: die alten Zettel sind Byte fuer Byte, was sie vorher waren, und ein
    /// fehlender ist nicht entstanden.
    fn zettel_unveraendert(&self) {
        for (alter, inhalt) in ALTE_ZETTEL.iter().zip(self.zettel) {
            let pfad = self.ablage.join(alter.datei);
            match inhalt {
                Some(inhalt) => assert_eq!(
                    fs::read(&pfad).expect("der Zettel ist weg"),
                    inhalt,
                    "{} ist veraendert",
                    alter.datei
                ),
                None => assert!(!pfad.exists(), "{} ist entstanden", alter.datei),
            }
        }
    }
}

/// Die Namen im Ordner, sortiert.
fn namen_in(ordner: &Path) -> Vec<String> {
    let mut namen: Vec<String> = fs::read_dir(ordner)
        .expect("der Ordner ist nicht lesbar")
        .map(|eintrag| {
            eintrag
                .expect("Eintrag")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    namen.sort();
    namen
}

/// C2.2, C2.1 fuer den gewoehnlichen Weg: eine vorhandene Datei bleibt Byte fuer
/// Byte, eine fehlende entsteht leer.
#[test]
fn eine_vorhandene_datei_bleibt_und_eine_fehlende_entsteht_leer() {
    let lage = Lage::neu("heim-vorhanden", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let vorher = b"## Eigenes\nnicht anfassen\r\n\xef\xbb\xbf";
    fs::write(lage.heimpfad().join("notes.txt"), vorher).expect("notes.txt");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(
        fs::read(lage.heimpfad().join("notes.txt")).expect("notes.txt"),
        vorher
    );
    assert_eq!(lage.lesen(Sonderdatei::Aufgaben), "");
    assert!(!bereitstellung.ordner_angelegt);
    assert_eq!(bereitstellung.angelegt, vec![Sonderdatei::Aufgaben]);
    assert_eq!(bereitstellung.uebernahme, None);
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    lage.zettel_unveraendert();
}

/// C2: der erste Aufruf legt Ordner, `notes.txt` und `tasks.txt` an und sonst
/// nichts, insbesondere keine `.secrets.txt` vor der Stufe 5.
#[test]
fn der_erste_aufruf_legt_genau_die_zwei_dateien_an() {
    let lage = Lage::neu("heim-erster", [None, None]);

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert!(bereitstellung.ordner_angelegt);
    assert_eq!(bereitstellung.angelegt, Sonderdatei::ALLE.to_vec());
    assert_eq!(namen_in(&lage.heimpfad()), vec!["notes.txt", "tasks.txt"]);
    assert_eq!(lage.lesen(Sonderdatei::Notizen), "");
    assert_eq!(lage.lesen(Sonderdatei::Aufgaben), "");
    let uebernahme = bereitstellung.uebernahme.as_ref().expect("Uebernahme");
    assert_eq!(uebernahme.ausgang, Uebernahmeausgang::NichtsZuUebernehmen);
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    lage.zettel_unveraendert();
}

/// C3.2: Hat derselbe Aufruf den Ordner angelegt, werden beide Zettel zu
/// Notizen, Text unveraendert und ein fehlender Schlussumbruch ergaenzt.
#[test]
fn beide_zettel_werden_zu_notizen_wenn_der_ordner_neu_ist() {
    let lage = Lage::neu(
        "heim-uebernahme",
        [
            Some("erste Zeile\n# eine Raute\n### drei Rauten\n".as_bytes()),
            Some("ohne Schlussumbruch".as_bytes()),
        ],
    );

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(
        lage.lesen(Sonderdatei::Notizen),
        "## Zettel 1\nerste Zeile\n# eine Raute\n### drei Rauten\n## Zettel 2\nohne Schlussumbruch\n"
    );
    assert_eq!(lage.lesen(Sonderdatei::Aufgaben), "");
    let uebernahme = bereitstellung.uebernahme.as_ref().expect("Uebernahme");
    assert_eq!(uebernahme.ausgang, Uebernahmeausgang::Geschrieben);
    assert_eq!(
        bereitstellung.meldungen(),
        vec!["Zettel 1 und Zettel 2 als Notizen in notes.txt übernommen".to_owned()]
    );
    // Die Notizen liest die eine Form wieder als zwei Bloecke.
    let stand = lage.lesen(Sonderdatei::Notizen);
    assert_eq!(Notizen::lesen(&stand).bloecke().len(), 2);
    lage.zettel_unveraendert();
}

/// C3.3: Ein Zettel mit einer Themenzeile wird nicht uebernommen, der andere
/// schon, und die Meldung nennt den abgewiesenen samt seiner Datei.
#[test]
fn ein_zettel_mit_themenzeile_bleibt_draussen_der_andere_kommt() {
    let lage = Lage::neu(
        "heim-themenzeile",
        [
            Some("oben\n## mitten drin\nunten\n".as_bytes()),
            Some("harmlos\n".as_bytes()),
        ],
    );

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(lage.lesen(Sonderdatei::Notizen), "## Zettel 2\nharmlos\n");
    let uebernahme = bereitstellung.uebernahme.as_ref().expect("Uebernahme");
    assert_eq!(uebernahme.zettel[0].1, Zettelbefund::Themenzeile);
    assert_eq!(uebernahme.zettel[1].1, Zettelbefund::Notiz);
    assert_eq!(
        bereitstellung.meldungen(),
        vec![
            "Zettel 2 als Notiz in notes.txt übernommen".to_owned(),
            "Zettel 1 ist nicht übernommen, weil er eine Zeile mit „## “ trägt; note-1.txt liegt unverändert im Ablageordner".to_owned(),
        ]
    );
    lage.zettel_unveraendert();
}

/// C3.2: Ein Zettel aus nichts als Leerraum und ein fehlender ergeben keine
/// Notiz und keine Meldung.
#[test]
fn leerraum_und_ein_fehlender_zettel_ergeben_keine_notiz() {
    let lage = Lage::neu("heim-leerraum", [Some(" \n\t\n".as_bytes()), None]);

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(lage.lesen(Sonderdatei::Notizen), "");
    let uebernahme = bereitstellung.uebernahme.as_ref().expect("Uebernahme");
    assert_eq!(uebernahme.zettel[0].1, Zettelbefund::Leer);
    assert_eq!(uebernahme.zettel[1].1, Zettelbefund::Fehlt);
    assert_eq!(uebernahme.ausgang, Uebernahmeausgang::NichtsZuUebernehmen);
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    lage.zettel_unveraendert();
}

/// Ein Zettel, der sich nicht als Text lesen laesst, wird nicht uebernommen und
/// gemeldet; der andere kommt trotzdem.
#[test]
fn ein_unlesbarer_zettel_wird_gemeldet_und_nicht_uebernommen() {
    let lage = Lage::neu(
        "heim-unlesbar",
        [
            Some(b"gut\n".as_slice()),
            Some(b"\xff\xfe kein UTF-8".as_slice()),
        ],
    );

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(lage.lesen(Sonderdatei::Notizen), "## Zettel 1\ngut\n");
    let meldungen = bereitstellung.meldungen();
    assert_eq!(meldungen.len(), 2, "{meldungen:?}");
    assert!(
        meldungen[1].starts_with("Zettel 2 ist nicht übernommen")
            && meldungen[1].ends_with("note-2.txt liegt unverändert im Ablageordner"),
        "{meldungen:?}"
    );
    lage.zettel_unveraendert();
}

/// C3.4: Bestand der Ordner schon, uebernimmt KRK nichts, auch wenn
/// `notes.txt` fehlt und neu entsteht.
#[test]
fn ein_bestehender_ordner_bekommt_nichts_uebernommen() {
    let lage = Lage::neu("heim-bestehend", [Some("Text\n".as_bytes()), None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(lage.lesen(Sonderdatei::Notizen), "");
    assert!(!bereitstellung.ordner_angelegt);
    assert_eq!(bereitstellung.uebernahme, None);
    lage.zettel_unveraendert();
}

/// C3.4, der Weg des Nutzers: `notes.txt` nach der Uebernahme loeschen, noch
/// einmal F2: sie entsteht leer, und die Zettel kommen kein zweites Mal.
#[test]
fn nach_geloeschter_notizdatei_kommt_keine_zweite_uebernahme() {
    let lage = Lage::neu("heim-zweiter", [Some("Text\n".as_bytes()), None]);
    let erste = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(
        erste.uebernahme.map(|u| u.ausgang),
        Some(Uebernahmeausgang::Geschrieben)
    );
    fs::remove_file(lage.heimpfad().join("notes.txt")).expect("notes.txt loeschen");

    let zweite = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(lage.lesen(Sonderdatei::Notizen), "");
    assert_eq!(zweite.angelegt, vec![Sonderdatei::Notizen]);
    assert_eq!(zweite.uebernahme, None);
    lage.zettel_unveraendert();
}

/// Eine gewoehnliche Datei an der Stelle von `krkhome`: `KeinOrdner`, und die
/// Datei bleibt, wie sie war.
#[test]
fn eine_datei_an_der_stelle_des_ordners_ist_kein_ordner() {
    let lage = Lage::neu("heim-datei", [Some("Text\n".as_bytes()), None]);
    fs::write(lage.heimpfad(), b"ich bin eine Datei").expect("Datei");

    assert_eq!(lage.bereitstellen(), Err(Hindernis::KeinOrdner));

    assert_eq!(
        fs::read(lage.heimpfad()).expect("die Datei ist weg"),
        b"ich bin eine Datei"
    );
    lage.zettel_unveraendert();
}

/// Ein Verweis auf einen Ordner: die Dateien entstehen im Ziel, und weil der
/// Verweis schon stand, wird nichts uebernommen.
#[test]
fn ein_verweis_auf_einen_ordner_laesst_die_dateien_im_ziel_entstehen() {
    let lage = Lage::neu("heim-verweisziel", [Some("Text\n".as_bytes()), None]);
    let ziel = lage.zuhause.join("anderswo");
    fs::create_dir(&ziel).expect("Zielordner");
    std::os::unix::fs::symlink(&ziel, lage.heimpfad()).expect("Verweis");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(namen_in(&ziel), vec!["notes.txt", "tasks.txt"]);
    assert!(!bereitstellung.ordner_angelegt);
    assert_eq!(bereitstellung.uebernahme, None);
    assert!(
        fs::symlink_metadata(lage.heimpfad())
            .expect("der Verweis ist weg")
            .file_type()
            .is_symlink(),
        "der Verweis ist ersetzt"
    );
    lage.zettel_unveraendert();
}

/// Ein Verweis ins Leere: `Unerreichbar`, und das fehlende Ziel entsteht nicht.
#[test]
fn ein_verweis_ins_leere_ist_unerreichbar() {
    let lage = Lage::neu("heim-leere", [None, None]);
    let ziel = lage.zuhause.join("gibt-es-nicht");
    std::os::unix::fs::symlink(&ziel, lage.heimpfad()).expect("Verweis");

    let ausgang = lage.bereitstellen();

    assert!(
        matches!(ausgang, Err(Hindernis::Unerreichbar(_))),
        "{ausgang:?}"
    );
    assert!(!ziel.exists(), "das Ziel ist entstanden");
}

/// Die Saetze der Statuszeile fuer die vier Hindernisse, im Wortlaut, mit
/// Umlauten und mit dem Ordner so, wie der Nutzer ihn kennt.
#[test]
fn die_hindernisse_melden_sich_im_wortlaut() {
    assert_eq!(
        Hindernis::KeinOrdner.meldung(),
        "~/krkhome ist kein Ordner; KRK legt dort nichts an und öffnet keinen Tab"
    );
    assert_eq!(
        Hindernis::Unerreichbar("Grund".to_owned()).meldung(),
        "~/krkhome ist nicht erreichbar: Grund"
    );
    assert_eq!(
        Hindernis::NichtAnlegbar("Grund".to_owned()).meldung(),
        "~/krkhome lässt sich nicht anlegen: Grund"
    );
    assert_eq!(
        Hindernis::KeinBenutzerverzeichnis.meldung(),
        "Das System nennt kein Benutzerverzeichnis, also gibt es kein ~/krkhome"
    );
}
