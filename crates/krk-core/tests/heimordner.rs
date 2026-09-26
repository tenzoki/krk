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
use krk_core::ablage::Grund;
use krk_core::ablage::einstellungen::Ortswert;
use krk_core::heimordner::eintraege::{
    Abweisung, Aufgaben, Notiz, Notizen, Richtung, aufgabe_in_grundform, aufgaben, aufgabenzeile,
    ist_themenzeile, notizen,
};
use krk_core::heimordner::ort::{
    Notizort, Ortsfehler, abweisungssatz, gehaltene_notizdatei, im_ablageordner, notizort,
    ort_lesen, ortswechsel, schon_der_ort, schreibform, schutzort, startzeile, wahlsatz,
    wechselsatz, zu_merken, zurueckgeschrieben,
};
use krk_core::heimordner::tresor::{
    self, FORMATVERSION, KOPFLAENGE, Kopf, Kopfschaden, Oeffnungsfehler, Parameter, Pin, Pinfehler,
    SALZLAENGE, Schluessel,
};
use krk_core::heimordner::{
    ALTE_ZETTEL, ALTER_GEHEIMNISNAME, AlteGeheimnisse, Bereitstellung, Heimordner, Hindernis,
    ORDNERNAME, Sonderdatei, Uebernahmeausgang, Zettelbefund, bereitstellen,
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

/// `260926-1119_*` Moeglichkeit 1: unter einer dritten Schreibweise erkennt
/// die genaue Frage `secrets.txt` an Geraet und Inode, auch unter anderer
/// Gross- und Kleinschreibung des Namens; eine gleichnamige Datei anderswo
/// bleibt eine gewoehnliche, und `notes.txt` bleibt bei den zwei Formen.
#[test]
fn die_genaue_frage_erkennt_secrets_txt_unter_einer_dritten_schreibweise() {
    let ordner = Pruefordner::neu("heim-genau");
    let zuhause = kanonisch(&ordner);
    let heimpfad = zuhause.join(ORDNERNAME);
    fs::create_dir(&heimpfad).expect("Heimordner laesst sich nicht anlegen");
    fs::write(heimpfad.join("secrets.txt"), b"").expect("leere Datei");
    fs::write(heimpfad.join("notes.txt"), b"").expect("Notizen");
    // Die dritte Schreibweise: ein zweiter Verweis anderswo auf den Ordner.
    let zweiter = zuhause.join("anderswo");
    std::os::unix::fs::symlink(&heimpfad, &zweiter).expect("zweiter Verweis");
    let fremd = zuhause.join("fremd");
    fs::create_dir(&fremd).expect("fremder Ordner");
    fs::write(fremd.join("secrets.txt"), b"").expect("gleichnamige Datei");

    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
    let dritte = zweiter.join("secrets.txt");
    assert_eq!(
        heim.sonderdatei(&dritte),
        None,
        "der Text erkennt sie nicht"
    );
    assert_eq!(
        heim.sonderdatei_genau(&dritte),
        Some(Sonderdatei::Geheimnisse)
    );
    assert_eq!(
        heim.sonderdatei_genau(&heimpfad.join("secrets.txt")),
        Some(Sonderdatei::Geheimnisse)
    );
    assert_eq!(heim.sonderdatei_genau(&fremd.join("secrets.txt")), None);
    assert_eq!(heim.sonderdatei_genau(&zweiter.join("notes.txt")), None);
    assert_eq!(heim.sonderdatei_genau(&zweiter.join("fehlt.txt")), None);

    // Ein zweiter Name derselben Inode mit anderer Schreibung: wie `Secrets.txt`
    // auf einem Volume ohne Unterscheidung von Gross und Klein.
    // Ein eigener Ordner, weil `fremd/secrets.txt` auf einem solchen Volume
    // denselben Namen belegt.
    let dritter = zuhause.join("dritter");
    fs::create_dir(&dritter).expect("dritter Ordner");
    let anders = dritter.join("SECRETS.TXT");
    fs::hard_link(heimpfad.join("secrets.txt"), &anders).expect("zweiter Name");
    assert_eq!(
        heim.sonderdatei_genau(&anders),
        Some(Sonderdatei::Geheimnisse)
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

/// C7.13: `ohne_inhaltsauftrag` nennt `secrets.txt` fuer den erkannten
/// Ordner, in beiden Formen, und fuer jeden anderen Ordner nichts.
#[test]
fn ohne_inhaltsauftrag_nennt_die_geheimnisse_allein_im_erkannten_ordner() {
    let ordner = Pruefordner::neu("heim-immer");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("ablage-ziel");
    fs::create_dir(&ziel).expect("Zielordner laesst sich nicht anlegen");
    std::os::unix::fs::symlink(&ziel, zuhause.join(ORDNERNAME))
        .expect("Verweis laesst sich nicht anlegen");
    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);

    for erkannt in [zuhause.join(ORDNERNAME), ziel.clone()] {
        assert_eq!(
            heim.ohne_inhaltsauftrag(&erkannt),
            Some("secrets.txt"),
            "{}",
            erkannt.display()
        );
    }
    assert_eq!(
        Sonderdatei::Geheimnisse.dateiname(),
        "secrets.txt",
        "der Name, den die Ausnahme nennt, ist der der Sonderdatei"
    );
    for anderer in [
        zuhause.clone(),
        ziel.join("unter"),
        zuhause.join("krkhome-alt"),
    ] {
        assert_eq!(
            heim.ohne_inhaltsauftrag(&anderer),
            None,
            "{}",
            anderer.display()
        );
    }
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

/// `ist` und `sonderdatei` stellen keinen Systemaufruf, und
/// `ohne_inhaltsauftrag` ebenso wenig.
///
/// Gefragt werden sie je Lesevorgang eines Tabs auf dem Hauptfaden; ein
/// Dateisystemaufruf dort blockierte an einem haengenden Netzlaufwerk die
/// Ereignisschleife und traefe die Zeitzusagen. Das haelt kein Uebersetzer,
/// deshalb liest diese Probe die beiden Ruempfe.
///
/// **Was sie nicht sieht:** einen Systemaufruf in einer Hilfsfunktion, die
/// einer der Ruempfe ruft. Heute rufen `sonderdatei` und `ohne_inhaltsauftrag`
/// allein `ist`, und `ist` keine Funktion dieses Moduls; wer eine
/// Hilfsfunktion einzieht, nimmt ihren Namen hier auf. `ohne_inhaltsauftrag`
/// (bis zum 260926 `immer_gelistet`) steht seit Schritt 5.2 dabei, weil
/// `Tabliste::lesen_starten` es je Lesevorgang auf dem Hauptfaden fragt.
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
    for name in ["ist", "sonderdatei", "ohne_inhaltsauftrag"] {
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
    assert_eq!(
        bereitstellung.angelegt,
        vec![Sonderdatei::Aufgaben, Sonderdatei::Geheimnisse]
    );
    assert_eq!(bereitstellung.uebernahme, None);
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    lage.zettel_unveraendert();
}

/// C2, C7.1: der erste Aufruf legt Ordner, `notes.txt`, `tasks.txt` und
/// `secrets.txt` an und sonst nichts; `secrets.txt` hat null Bytes.
#[test]
fn der_erste_aufruf_legt_genau_die_drei_dateien_an() {
    let lage = Lage::neu("heim-erster", [None, None]);

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert!(bereitstellung.ordner_angelegt);
    assert_eq!(bereitstellung.angelegt, Sonderdatei::ALLE.to_vec());
    assert_eq!(
        namen_in(&lage.heimpfad()),
        vec!["notes.txt", "secrets.txt", "tasks.txt"]
    );
    assert_eq!(lage.lesen(Sonderdatei::Notizen), "");
    assert_eq!(lage.lesen(Sonderdatei::Aufgaben), "");
    let geheimnisse = lage.heimpfad().join(Sonderdatei::Geheimnisse.dateiname());
    assert_eq!(
        fs::metadata(&geheimnisse).expect("secrets.txt fehlt").len(),
        0,
        "`secrets.txt` entsteht mit null Bytes"
    );
    let uebernahme = bereitstellung.uebernahme.as_ref().expect("Uebernahme");
    assert_eq!(uebernahme.ausgang, Uebernahmeausgang::NichtsZuUebernehmen);
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    lage.zettel_unveraendert();
}

// ---------------------------------------------------------------------------
// Eine `.secrets.txt` von vorher wird zu `secrets.txt`
// (`260926-1308_*_heisst-die-geheimnisdatei-secrets-txt-ohne-punkt.md`)
// ---------------------------------------------------------------------------

/// Ein Chiffrat, wie es der Editor schreibt, mit kleinen Parametern.
fn ein_chiffrat() -> Vec<u8> {
    tresor::verschliessen(BEKANNTER_EINTRAG.as_bytes(), &kleiner_schluessel("0417"))
        .expect("verschliessen")
}

/// Steht allein `.secrets.txt`, heisst sie danach `secrets.txt`, Byte fuer Byte
/// und dieselbe Inode; der alte Name ist weg, und die PIN oeffnet sie weiter.
#[test]
fn eine_alte_secrets_txt_wird_umbenannt_und_bleibt_byte_fuer_byte() {
    use std::os::unix::fs::MetadataExt;
    let lage = Lage::neu("heim-alt-umbenannt", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let alt = lage.heimpfad().join(ALTER_GEHEIMNISNAME);
    let neu = lage.heimpfad().join(Sonderdatei::Geheimnisse.dateiname());
    let chiffrat = ein_chiffrat();
    fs::write(&alt, &chiffrat).expect(".secrets.txt");
    let inode = fs::metadata(&alt).expect(".secrets.txt").ino();

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(
        bereitstellung.alte_geheimnisse,
        Some(AlteGeheimnisse::Umbenannt)
    );
    assert!(!alt.exists(), "der alte Name steht noch");
    assert_eq!(fs::read(&neu).expect("secrets.txt fehlt"), chiffrat);
    assert_eq!(fs::metadata(&neu).expect("secrets.txt").ino(), inode);
    let geoeffnet = tresor::oeffnen(&fs::read(&neu).expect("secrets.txt"), &pin("0417"))
        .expect("die PIN oeffnet die umbenannte Datei");
    assert_eq!(geoeffnet.klartext, BEKANNTER_EINTRAG.as_bytes());
    assert!(
        !bereitstellung.angelegt.contains(&Sonderdatei::Geheimnisse),
        "{bereitstellung:?}"
    );
    assert_eq!(
        namen_in(&lage.heimpfad()),
        vec!["notes.txt", "secrets.txt", "tasks.txt"]
    );
    let meldungen = bereitstellung.meldungen().join("\n");
    assert!(
        meldungen.contains(".secrets.txt heißt jetzt secrets.txt"),
        "{meldungen}"
    );
    lage.zettel_unveraendert();
}

/// Stehen `.secrets.txt` und `secrets.txt`, bleiben beide Byte fuer Byte, und
/// die Statuszeile sagt es.
#[test]
fn stehen_beide_bleiben_beide_und_die_statuszeile_sagt_es() {
    let lage = Lage::neu("heim-alt-beide", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let alt = lage.heimpfad().join(ALTER_GEHEIMNISNAME);
    let neu = lage.heimpfad().join(Sonderdatei::Geheimnisse.dateiname());
    let altes_chiffrat = ein_chiffrat();
    let neues_chiffrat = ein_chiffrat();
    assert_ne!(
        altes_chiffrat, neues_chiffrat,
        "jedes Chiffrat traegt seine Nonce"
    );
    fs::write(&alt, &altes_chiffrat).expect(".secrets.txt");
    fs::write(&neu, &neues_chiffrat).expect("secrets.txt");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(
        bereitstellung.alte_geheimnisse,
        Some(AlteGeheimnisse::BeideStehen)
    );
    assert_eq!(fs::read(&alt).expect(".secrets.txt"), altes_chiffrat);
    assert_eq!(fs::read(&neu).expect("secrets.txt"), neues_chiffrat);
    assert!(
        !bereitstellung.angelegt.contains(&Sonderdatei::Geheimnisse),
        "{bereitstellung:?}"
    );
    let meldungen = bereitstellung.meldungen().join("\n");
    assert!(
        meldungen.contains("secrets.txt und .secrets.txt stehen beide"),
        "{meldungen}"
    );
    lage.zettel_unveraendert();
}

/// Ohne `.secrets.txt` entsteht `secrets.txt` leer wie bisher, und es gibt
/// nichts zu melden; eine vorhandene `secrets.txt` bleibt Byte fuer Byte.
#[test]
fn ohne_alte_datei_entsteht_secrets_txt_leer_oder_bleibt_wie_sie_war() {
    let lage = Lage::neu("heim-alt-keine", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let neu = lage.heimpfad().join(Sonderdatei::Geheimnisse.dateiname());

    let erste = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(erste.alte_geheimnisse, None);
    assert!(
        erste.angelegt.contains(&Sonderdatei::Geheimnisse),
        "{erste:?}"
    );
    assert_eq!(fs::metadata(&neu).expect("secrets.txt fehlt").len(), 0);
    assert!(erste.meldungen().is_empty(), "{erste:?}");

    let chiffrat = ein_chiffrat();
    fs::write(&neu, &chiffrat).expect("secrets.txt");
    let zweite = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(zweite.alte_geheimnisse, None);
    assert!(zweite.angelegt.is_empty(), "{zweite:?}");
    assert_eq!(fs::read(&neu).expect("secrets.txt"), chiffrat);
    assert!(
        !lage.heimpfad().join(ALTER_GEHEIMNISNAME).exists(),
        "der alte Name entsteht nie"
    );
    lage.zettel_unveraendert();
}

/// Laesst sich die alte Datei nicht umbenennen, bleibt sie unberuehrt, und
/// daneben entsteht keine leere `secrets.txt`. Gespielt mit einem
/// schreibgeschuetzten Heimordner: `link(2)` braucht das Schreibrecht am
/// Ordner.
#[test]
fn eine_gescheiterte_umbenennung_legt_keine_leere_datei_daneben() {
    use std::os::unix::fs::PermissionsExt;
    let lage = Lage::neu("heim-alt-gescheitert", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    for sorte in [Sonderdatei::Notizen, Sonderdatei::Aufgaben] {
        fs::write(lage.heimpfad().join(sorte.dateiname()), b"").expect("Eintragsdatei");
    }
    let alt = lage.heimpfad().join(ALTER_GEHEIMNISNAME);
    let chiffrat = ein_chiffrat();
    fs::write(&alt, &chiffrat).expect(".secrets.txt");
    fs::set_permissions(lage.heimpfad(), fs::Permissions::from_mode(0o555)).expect("schreibschutz");

    let bereitstellung = lage.bereitstellen();

    fs::set_permissions(lage.heimpfad(), fs::Permissions::from_mode(0o755))
        .expect("schreibschutz aufheben");
    let bereitstellung = bereitstellung.expect("kein Hindernis erwartet");
    assert!(
        matches!(
            bereitstellung.alte_geheimnisse,
            Some(AlteGeheimnisse::Gescheitert(_))
        ),
        "{bereitstellung:?}"
    );
    assert_eq!(fs::read(&alt).expect(".secrets.txt"), chiffrat);
    assert!(
        !lage
            .heimpfad()
            .join(Sonderdatei::Geheimnisse.dateiname())
            .exists(),
        "neben der alten Datei ist eine neue entstanden"
    );
    assert!(
        bereitstellung.nicht_angelegt.is_empty(),
        "{bereitstellung:?}"
    );
    let meldungen = bereitstellung.meldungen().join("\n");
    assert!(
        meldungen.contains("lässt sich nicht in secrets.txt umbenennen"),
        "{meldungen}"
    );
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

/// C7.1: eine vorhandene `secrets.txt` bleibt Byte fuer Byte, gleich ob sie
/// Chiffrat oder null Bytes traegt; F2 fragt dabei keine PIN und legt nichts
/// daneben an.
#[test]
fn eine_vorhandene_geheimnisdatei_bleibt_byte_fuer_byte() {
    for (zweck, vorher) in [
        (
            "heim-geheim-voll",
            b"KRKSEC\x01\x00\xff Chiffrat".as_slice(),
        ),
        ("heim-geheim-leer", b"".as_slice()),
    ] {
        let lage = Lage::neu(zweck, [None, None]);
        fs::create_dir(lage.heimpfad()).expect("Heimordner");
        let pfad = lage.heimpfad().join(Sonderdatei::Geheimnisse.dateiname());
        fs::write(&pfad, vorher).expect("secrets.txt");

        let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

        assert_eq!(fs::read(&pfad).expect("secrets.txt fehlt"), vorher);
        assert!(
            !bereitstellung.angelegt.contains(&Sonderdatei::Geheimnisse),
            "{bereitstellung:?}"
        );
        assert_eq!(
            namen_in(&lage.heimpfad()),
            vec!["notes.txt", "secrets.txt", "tasks.txt"]
        );
    }
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

    assert_eq!(
        namen_in(&ziel),
        vec!["notes.txt", "secrets.txt", "tasks.txt"]
    );
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

/// Die Saetze der Statuszeile fuer die vier Hindernisse am Vorgabeort, im
/// Wortlaut, mit Umlauten und mit dem Ordner so, wie der Nutzer ihn kennt;
/// dazu der Satz ohne Benutzerverzeichnis, der seit Schritt 2.1 des Plans
/// `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md` ein Ortsfehler ist.
#[test]
fn die_hindernisse_melden_sich_im_wortlaut() {
    let heim = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe"));
    let ort = heim.anzeigename();
    assert_eq!(ort, "~/krkhome");
    assert_eq!(
        Hindernis::KeinOrdner.meldung(ort),
        "~/krkhome ist kein Ordner; KRK legt dort nichts an und öffnet keinen Tab"
    );
    assert_eq!(
        Hindernis::Unerreichbar("Grund".to_owned()).meldung(ort),
        "~/krkhome ist nicht erreichbar: Grund"
    );
    assert_eq!(
        Hindernis::NichtAnlegbar("Grund".to_owned()).meldung(ort),
        "~/krkhome lässt sich nicht anlegen: Grund"
    );
    assert_eq!(
        Hindernis::ObererOrdnerFehlt.meldung(ort),
        "~/krkhome lässt sich nicht anlegen, weil der Ordner darüber fehlt, etwa ein nicht eingehängtes Laufwerk; KRK legt nichts an"
    );
    assert_eq!(
        Ortsfehler::KeinBenutzerverzeichnis.meldung(),
        "Das System nennt kein Benutzerverzeichnis, also gibt es keinen Notizordner"
    );
}

// ---------------------------------------------------------------------------
// Der eingestellte Ort (Schritt 2.1 des Plans
// `260926-1506_*_plan-home-menue-und-einstellbarer-ort.md`, H2 des Spec
// `260926-1451_*_spec-home-menue-und-einstellbarer-ort.md`)
// ---------------------------------------------------------------------------

/// Das Benutzerverzeichnis der Tafeln; keine Probe beruehrt es.
const ZUHAUSE: &str = "/Users/probe";

/// Der Ablageordner der Tafeln, in der Schreibweise des Systems.
const ABLAGE: &str = "/Users/probe/Library/Application Support/KRK";

/// H2.4 und H2.5: jede zulaessige Form ergibt ihren bereinigten Pfad, jede
/// abgewiesene ihren Fehler mit dem Wert. Der Ablageordner wird ohne Ruecksicht
/// auf Gross- und Kleinschreibung verglichen und Bestandteil fuer Bestandteil,
/// so dass `KRK-alt` daneben gilt.
#[test]
fn ort_lesen_nimmt_genau_die_zulaessigen_formen() {
    let zuhause = Some(Path::new(ZUHAUSE));
    let ablage = Some(Path::new(ABLAGE));
    let zulaessig: &[(&str, &str)] = &[
        ("~/krkhome", "/Users/probe/krkhome"),
        ("~/", "/Users/probe"),
        ("~/a/../b", "/Users/probe/b"),
        ("~/./a//b/", "/Users/probe/a/b"),
        ("~/Dropbox/Notizen", "/Users/probe/Dropbox/Notizen"),
        ("/Volumes/X/notizen", "/Volumes/X/notizen"),
        (
            "/Users/probe/Library/Application Support/KRK-alt",
            "/Users/probe/Library/Application Support/KRK-alt",
        ),
        (
            "~/Library/Application Support",
            "/Users/probe/Library/Application Support",
        ),
    ];
    for (text, erwartet) in zulaessig {
        assert_eq!(
            ort_lesen(text, zuhause, ablage),
            Ok(PathBuf::from(erwartet)),
            "{text}"
        );
    }

    let abgewiesen: Vec<(&str, Ortsfehler)> = vec![
        ("", Ortsfehler::Leer),
        ("  \t", Ortsfehler::Leer),
        ("notizen", Ortsfehler::NichtAbsolut("notizen".to_owned())),
        (
            "./notizen",
            Ortsfehler::NichtAbsolut("./notizen".to_owned()),
        ),
        ("~", Ortsfehler::NichtAbsolut("~".to_owned())),
        (
            "~kai/notizen",
            Ortsfehler::FremdesBenutzerverzeichnis("~kai/notizen".to_owned()),
        ),
        (
            "~kai",
            Ortsfehler::FremdesBenutzerverzeichnis("~kai".to_owned()),
        ),
        (
            "~/Library/Application Support/KRK",
            Ortsfehler::ImAblageordner("~/Library/Application Support/KRK".to_owned()),
        ),
        (
            "~/Library/Application Support/KRK/notizen",
            Ortsfehler::ImAblageordner("~/Library/Application Support/KRK/notizen".to_owned()),
        ),
        (
            "~/library/application support/krk",
            Ortsfehler::ImAblageordner("~/library/application support/krk".to_owned()),
        ),
        (
            "/Users/probe/Library/Application Support/KRK-alt/../KRK/x",
            Ortsfehler::ImAblageordner(
                "/Users/probe/Library/Application Support/KRK-alt/../KRK/x".to_owned(),
            ),
        ),
    ];
    for (text, fehler) in abgewiesen {
        assert_eq!(ort_lesen(text, zuhause, ablage), Err(fehler), "{text:?}");
    }

    // Ohne Benutzerverzeichnis ist `~/` nicht aufzuloesen, ein absoluter Ort
    // gilt weiter; ohne Ablageordner faellt allein dessen Ausschluss.
    assert_eq!(
        ort_lesen("~/krkhome", None, ablage),
        Err(Ortsfehler::KeinBenutzerverzeichnis)
    );
    assert_eq!(
        ort_lesen("/Volumes/X", None, ablage),
        Ok(PathBuf::from("/Volumes/X"))
    );
    assert_eq!(
        ort_lesen("~/Library/Application Support/KRK", zuhause, None),
        Ok(PathBuf::from(ABLAGE))
    );
}

/// H2.3: jede Meldung eines unzulaessigen Werts nennt den Wert.
#[test]
fn jede_meldung_eines_unzulaessigen_werts_nennt_ihn() {
    for fehler in [
        Ortsfehler::NichtAbsolut("notizen".to_owned()),
        Ortsfehler::FremdesBenutzerverzeichnis("notizen".to_owned()),
        Ortsfehler::KeinText("notizen".to_owned()),
        Ortsfehler::ImAblageordner("notizen".to_owned()),
    ] {
        let meldung = fehler.meldung();
        assert!(meldung.contains("notizen"), "{fehler:?}: {meldung}");
        assert!(meldung.contains("Notizordner"), "{fehler:?}: {meldung}");
    }
    assert!(Ortsfehler::Leer.meldung().contains("leer"));
    assert!(
        Ortsfehler::KeinText("5".to_owned())
            .meldung()
            .contains("kein Text, sondern 5")
    );
}

/// H3.7 im Kern: `schreibform` und `ort_lesen` ergeben fuer jeden zulaessigen
/// Ort den Ort zurueck, unter dem Benutzerverzeichnis in der Form `~/…`, das
/// Benutzerverzeichnis selbst und jeden anderen Ort absolut.
#[test]
fn schreibform_und_ort_lesen_ergeben_den_ort_zurueck() {
    use std::os::unix::ffi::OsStrExt;
    let zuhause = Path::new(ZUHAUSE);
    let faelle: &[(&str, &str)] = &[
        ("/Users/probe/krkhome", "~/krkhome"),
        ("/Users/probe/Dropbox/Notizen", "~/Dropbox/Notizen"),
        ("/Users/probe", "/Users/probe"),
        ("/Users/probe-alt/notizen", "/Users/probe-alt/notizen"),
        ("/Volumes/X/notizen", "/Volumes/X/notizen"),
        ("/", "/"),
    ];
    for (ort, form) in faelle {
        let ort = Path::new(ort);
        let text = schreibform(ort, Some(zuhause)).expect("ein UTF-8-Pfad hat eine Schreibform");
        assert_eq!(text, *form, "{}", ort.display());
        assert_eq!(
            ort_lesen(&text, Some(zuhause), Some(Path::new(ABLAGE))),
            Ok(ort.to_path_buf()),
            "{text}"
        );
    }
    assert_eq!(
        schreibform(Path::new("/Users/probe/krkhome"), None).as_deref(),
        Some("/Users/probe/krkhome")
    );
    let ohne_utf8 = Path::new(std::ffi::OsStr::from_bytes(b"/Volumes/\xff"));
    assert_eq!(schreibform(ohne_utf8, Some(zuhause)), None);
}

/// Der Vorgabeort ist genau `<benutzerverzeichnis>/krkhome`, und nur mit einem
/// bekannten Benutzerverzeichnis. Der Anzeigename kuerzt unter dem
/// Benutzerverzeichnis zu `~/…` und nennt jeden anderen Ort ausgeschrieben.
/// Keiner dieser Orte besteht, und keiner liegt unmittelbar im
/// Benutzerverzeichnis ausser den zwei Vorgabeorten, also fragt der Bau hier
/// hoechstens nach dem nicht vorhandenen `/Users/probe/krkhome`.
#[test]
fn der_vorgabeort_und_der_anzeigename() {
    let zuhause = Path::new(ZUHAUSE);
    assert!(Heimordner::im_benutzerverzeichnis(zuhause).ist_vorgabeort());
    assert!(Heimordner::am_ort(zuhause.join(ORDNERNAME), Some(zuhause)).ist_vorgabeort());
    for (ort, bekannt, anzeige) in [
        ("/Users/probe/krkhome", false, "/Users/probe/krkhome"),
        ("/Users/probe/krkhome/unter", true, "~/krkhome/unter"),
        ("/Users/probe/Dropbox/krkhome", true, "~/Dropbox/krkhome"),
        ("/Volumes/X/notizen", true, "/Volumes/X/notizen"),
    ] {
        let heim = Heimordner::am_ort(PathBuf::from(ort), bekannt.then_some(zuhause));
        assert!(!heim.ist_vorgabeort(), "{ort}");
        assert_eq!(heim.anzeigename(), anzeige, "{ort}");
    }
}

/// H2.5: ein Verweis **unmittelbar** im Benutzerverzeichnis wird beim Bau
/// leicht aufgeloest, und der Ordner ist ueber Verweis und Ziel erkannt. Zwei
/// Ebenen tiefer entsteht beim Bau keine aufgeloeste Form: erkannt wird allein
/// die geschriebene, bis F2 ueber `canonicalize` erneuert.
#[test]
fn die_leichte_form_entsteht_allein_unmittelbar_im_benutzerverzeichnis() {
    let ordner = Pruefordner::neu("heim-ort-verweis");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("ziel");
    fs::create_dir(&ziel).expect("Zielordner");

    let flach = zuhause.join("notizen");
    std::os::unix::fs::symlink(&ziel, &flach).expect("Verweis im Benutzerverzeichnis");
    let heim = Heimordner::am_ort(flach.clone(), Some(&zuhause));
    assert!(heim.ist(&flach));
    assert!(
        heim.ist(&ziel),
        "der Verweis im Benutzerverzeichnis ist nicht aufgeloest"
    );
    assert_eq!(
        heim.sonderdatei(&ziel.join("notes.txt")),
        Some(Sonderdatei::Notizen)
    );

    fs::create_dir(zuhause.join("tief")).expect("Zwischenordner");
    let tief = zuhause.join("tief").join("notizen");
    std::os::unix::fs::symlink(&ziel, &tief).expect("Verweis zwei Ebenen tiefer");
    let heim = Heimordner::am_ort(tief.clone(), Some(&zuhause));
    assert!(heim.ist(&tief), "die geschriebene Form gilt");
    assert!(
        !heim.ist(&ziel),
        "zwei Ebenen tiefer hat der Bau den Verweis gelesen"
    );
    assert_eq!(heim.sonderdatei(&ziel.join("notes.txt")), None);
    let erneuert = heim.aufgeloest_erneuern();
    assert!(
        erneuert.ist(&ziel),
        "F2 loest den Ort ueber canonicalize auf"
    );
    assert!(erneuert.ist(&tief));
    assert_eq!(erneuert.anzeigename(), "~/tief/notizen");
    assert!(!erneuert.ist_vorgabeort());
}

/// H2.6: `sonderdatei_genau` erkennt `secrets.txt` unter einer dritten
/// Schreibweise auch an einem Ort ausserhalb von `~/krkhome`, und der
/// Vorgabeort erkennt sie dort nicht.
#[test]
fn die_genaue_frage_gilt_am_eingestellten_ort() {
    let ordner = Pruefordner::neu("heim-ort-genau");
    let zuhause = kanonisch(&ordner);
    let ort = zuhause.join("anderswo").join("notizen");
    fs::create_dir_all(&ort).expect("eingestellter Ort");
    fs::write(ort.join("secrets.txt"), b"").expect("leere Geheimnisdatei");
    let dritte = zuhause.join("zweiter");
    std::os::unix::fs::symlink(&ort, &dritte).expect("zweiter Verweis");

    let heim = Heimordner::am_ort(ort.clone(), Some(&zuhause));
    let gefragt = dritte.join("secrets.txt");
    assert_eq!(
        heim.sonderdatei(&gefragt),
        None,
        "der Text erkennt sie nicht"
    );
    assert_eq!(
        heim.sonderdatei_genau(&gefragt),
        Some(Sonderdatei::Geheimnisse)
    );
    assert_eq!(
        heim.sonderdatei_genau(&ort.join("secrets.txt")),
        Some(Sonderdatei::Geheimnisse)
    );
    let vorgabe = Heimordner::im_benutzerverzeichnis(&zuhause);
    assert_eq!(vorgabe.sonderdatei_genau(&gefragt), None);
    assert_eq!(vorgabe.sonderdatei_genau(&ort.join("secrets.txt")), None);
}

/// H2.8: F2 legt an einem anderen Ort mit Text in beiden Zetteln an und findet
/// eine leere `notes.txt`; die Zettel sind Byte fuer Byte unveraendert. Am
/// Vorgabeort uebernimmt derselbe Aufruf danach wie bisher.
#[test]
fn die_alten_zettel_kommen_allein_am_vorgabeort() {
    let lage = Lage::neu(
        "heim-ort-zettel",
        [
            Some("erster Zettel\n".as_bytes()),
            Some("zweiter Zettel\n".as_bytes()),
        ],
    );
    let ort = lage.zuhause.join("woanders");
    let heim = Heimordner::am_ort(ort.clone(), Some(&lage.zuhause));

    let anderswo = bereitstellen(&heim, &lage.ablage).expect("kein Hindernis erwartet");

    assert!(anderswo.ordner_angelegt);
    assert_eq!(anderswo.uebernahme, None);
    assert_eq!(anderswo.angelegt, Sonderdatei::ALLE.to_vec());
    assert_eq!(
        fs::read(ort.join("notes.txt")).expect("notes.txt fehlt"),
        b""
    );
    assert_eq!(
        namen_in(&ort),
        vec!["notes.txt", "secrets.txt", "tasks.txt"]
    );
    assert!(anderswo.meldungen().is_empty(), "{anderswo:?}");
    lage.zettel_unveraendert();

    let vorgabe = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(
        vorgabe.uebernahme.map(|u| u.ausgang),
        Some(Uebernahmeausgang::Geschrieben)
    );
    assert_eq!(
        lage.lesen(Sonderdatei::Notizen),
        "## Zettel 1\nerster Zettel\n## Zettel 2\nzweiter Zettel\n"
    );
    lage.zettel_unveraendert();
}

/// H2.7: fehlt der Ordner ueber dem Ort, meldet F2 das mit dem Ort und legt
/// nichts an, auch nicht die fehlende Stufe darueber.
#[test]
fn ein_fehlender_oberer_ordner_legt_nichts_an() {
    let lage = Lage::neu("heim-ort-oben", [Some("Text\n".as_bytes()), None]);
    let oben = lage.zuhause.join("nicht-eingehaengt");
    let heim = Heimordner::am_ort(oben.join("notizen"), Some(&lage.zuhause));

    let ausgang = bereitstellen(&heim, &lage.ablage);

    assert_eq!(ausgang, Err(Hindernis::ObererOrdnerFehlt));
    assert!(!oben.exists(), "die Stufe darueber ist entstanden");
    assert_eq!(
        ausgang.expect_err("Hindernis").meldung(heim.anzeigename()),
        "~/nicht-eingehaengt/notizen lässt sich nicht anlegen, weil der Ordner darüber fehlt, etwa ein nicht eingehängtes Laufwerk; KRK legt nichts an"
    );
    lage.zettel_unveraendert();
}

/// H2.11: jede Meldung nennt den eingestellten Ort, unter dem
/// Benutzerverzeichnis als `~/…` und ausserhalb ausgeschrieben.
#[test]
fn die_meldungen_nennen_den_eingestellten_ort() {
    let zuhause = Path::new(ZUHAUSE);
    for (ort, anzeige) in [
        ("/Users/probe/Dropbox/Notizen", "~/Dropbox/Notizen"),
        ("/Volumes/X/notizen", "/Volumes/X/notizen"),
    ] {
        // Keiner der Orte liegt unmittelbar im Benutzerverzeichnis: der Bau
        // stellt keinen Systemaufruf.
        let heim = Heimordner::am_ort(PathBuf::from(ort), Some(zuhause));
        let name = heim.anzeigename();
        assert_eq!(name, anzeige);
        for (hindernis, satz) in [
            (
                Hindernis::KeinOrdner,
                format!("{anzeige} ist kein Ordner; KRK legt dort nichts an und öffnet keinen Tab"),
            ),
            (
                Hindernis::Unerreichbar("Grund".to_owned()),
                format!("{anzeige} ist nicht erreichbar: Grund"),
            ),
            (
                Hindernis::NichtAnlegbar("Grund".to_owned()),
                format!("{anzeige} lässt sich nicht anlegen: Grund"),
            ),
        ] {
            assert_eq!(hindernis.meldung(name), satz);
        }
    }

    // Der eine Satz aus `Bereitstellung`, der den Ordner nennt.
    let lage = Lage::neu("heim-ort-beide", [None, None]);
    let ort = lage.zuhause.join("woanders");
    fs::create_dir(&ort).expect("eingestellter Ort");
    fs::write(ort.join(ALTER_GEHEIMNISNAME), b"alt").expect(".secrets.txt");
    fs::write(ort.join("secrets.txt"), b"neu").expect("secrets.txt");
    let heim = Heimordner::am_ort(ort.clone(), Some(&lage.zuhause));
    let bereitstellung = bereitstellen(&heim, &lage.ablage).expect("kein Hindernis erwartet");
    assert_eq!(
        bereitstellung.meldungen(),
        vec![
            "secrets.txt und .secrets.txt stehen beide in ~/woanders; KRK benennt keine um, und es gilt secrets.txt"
                .to_owned()
        ]
    );
}

/// Ein Text als Ortswert.
fn text(wert: &str) -> Ortswert {
    Ortswert::Text(wert.to_owned())
}

/// H2.2, H2.3 und H2.4 am Start: `notizort` ueber Wert und Ersetzungsgrund.
///
/// Ein gueltiger Wert ergibt seinen Ort, jede Fehlerform des Werts ihren
/// Fehler; `NichtAnlegbar` mit dem Auslieferungswert ergibt den Vorgabeort;
/// `Beschaedigt` und `NichtLesbar` ergeben **keinen** Ort, gleich was der Wert
/// sagt. Der letzte Fall ist der eine Zweig, der an der offenen Nutzerfrage
/// `260926-1506_*_welcher-notizordner-gilt-wenn-settings-toml-beim-start-beschaedigt-ist.md`
/// haengt; eine andere Antwort aendert diese Zeilen.
#[test]
fn notizort_folgt_wert_und_ersetzungsgrund() {
    let zuhause = Some(Path::new(ZUHAUSE));
    let ablage = Some(Path::new(ABLAGE));
    let ort = |wert: &Ortswert, schaden: Option<&Grund>| -> Notizort {
        notizort(wert, schaden, zuhause, ablage)
    };

    let gueltig = ort(&text("/Volumes/X/notizen"), None).expect("ein gueltiger Ort");
    assert_eq!(gueltig.geschrieben(), Path::new("/Volumes/X/notizen"));
    assert!(!gueltig.ist_vorgabeort());

    for (wert, fehler) in [
        (text(""), Ortsfehler::Leer),
        (
            text("notizen"),
            Ortsfehler::NichtAbsolut("notizen".to_owned()),
        ),
        (
            text("~kai/x"),
            Ortsfehler::FremdesBenutzerverzeichnis("~kai/x".to_owned()),
        ),
        (
            text("~/Library/Application Support/KRK"),
            Ortsfehler::ImAblageordner("~/Library/Application Support/KRK".to_owned()),
        ),
        (
            Ortswert::KeinText("5".to_owned()),
            Ortsfehler::KeinText("5".to_owned()),
        ),
    ] {
        assert_eq!(ort(&wert, None), Err(fehler), "{wert:?}");
    }
    assert_eq!(
        notizort(&text("~/krkhome"), None, None, ablage),
        Err(Ortsfehler::KeinBenutzerverzeichnis)
    );

    let fehlte = Grund::NichtAnlegbar("kein Platz".to_owned());
    let vorgabe = ort(&text(&format!("~/{ORDNERNAME}")), Some(&fehlte))
        .expect("eine fehlende Datei ergibt den Vorgabeort");
    assert!(vorgabe.ist_vorgabeort());

    for (schaden, satzteil) in [
        (Grund::Beschaedigt("Zeile 3".to_owned()), "ist beschädigt"),
        (Grund::NichtLesbar("Rechte".to_owned()), "ist nicht lesbar"),
    ] {
        assert_eq!(
            ort(&text("/Volumes/X/notizen"), Some(&schaden)),
            Err(Ortsfehler::EinstellungenBeschaedigt(satzteil.to_owned())),
            "{schaden:?}"
        );
    }
}

/// H2.10: der Wechselsatz kommt allein, wenn ein gemerkter Ort da ist und der
/// geltende gueltig und ein anderer ist; `~/krkhome` und `/<zuhause>/krkhome`
/// sind derselbe.
#[test]
fn ortswechsel_meldet_allein_einen_anderen_ort() {
    let zuhause = Some(Path::new(ZUHAUSE));
    let geltend: Notizort = Ok(Heimordner::am_ort(
        PathBuf::from("/Users/probe/Dropbox/Notizen"),
        zuhause,
    ));
    let vorgabe: Notizort = Ok(Heimordner::im_benutzerverzeichnis(Path::new(ZUHAUSE)));

    assert_eq!(ortswechsel(None, &geltend, zuhause), None, "nichts gemerkt");
    assert_eq!(
        ortswechsel(
            Some(Path::new("/Users/probe/Dropbox/Notizen")),
            &geltend,
            zuhause
        ),
        None,
        "derselbe Ort"
    );
    assert_eq!(
        ortswechsel(Some(Path::new("~/krkhome")), &vorgabe, zuhause),
        None,
        "derselbe Ort in anderer Schreibweise"
    );
    assert_eq!(
        ortswechsel(Some(Path::new("/Users/probe/krkhome/")), &vorgabe, zuhause),
        None,
        "ein Schlussstrich ist dieselbe Schreibweise"
    );
    assert_eq!(
        ortswechsel(Some(Path::new("/Users/probe/krkhome")), &geltend, zuhause),
        Some(wechselsatz("~/Dropbox/Notizen", "~/krkhome"))
    );
    assert_eq!(
        ortswechsel(
            Some(Path::new("/Users/probe/krkhome")),
            &Err(Ortsfehler::Leer),
            zuhause
        ),
        None,
        "ein ungueltiger geltender Ort ist kein Wechsel"
    );
    let satz = wechselsatz("~/Dropbox/Notizen", "~/krkhome");
    for teil in [
        "~/Dropbox/Notizen",
        "~/krkhome",
        "bleibt alles liegen",
        "F2",
    ] {
        assert!(satz.contains(teil), "{satz}");
    }
}

/// H2.3 und H2.10: die Startzeile ueber jede Variante von `Ortsfehler` und
/// ueber einen gueltigen Ort.
#[test]
fn startzeile_nennt_den_fehler_des_werts_und_schweigt_zum_schaden_der_datei() {
    let zuhause = Some(Path::new(ZUHAUSE));
    let gemerkt = Some(Path::new("/Users/probe/krkhome"));
    for fehler in [
        Ortsfehler::KeinBenutzerverzeichnis,
        Ortsfehler::Leer,
        Ortsfehler::NichtAbsolut("x".to_owned()),
        Ortsfehler::FremdesBenutzerverzeichnis("~kai".to_owned()),
        Ortsfehler::KeinText("5".to_owned()),
        Ortsfehler::ImAblageordner("~/Library/Application Support/KRK".to_owned()),
    ] {
        assert_eq!(
            startzeile(&Err(fehler.clone()), gemerkt, zuhause),
            Some(fehler.meldung()),
            "{fehler:?}"
        );
    }
    for fehler in [
        Ortsfehler::EinstellungenBeschaedigt("ist beschädigt".to_owned()),
        Ortsfehler::EinstellungenUngelesen("Sperre".to_owned()),
    ] {
        assert_eq!(
            startzeile(&Err(fehler.clone()), gemerkt, zuhause),
            None,
            "{fehler:?}: den Grund nennt schon der Lader"
        );
    }
    let vorgabe: Notizort = Ok(Heimordner::im_benutzerverzeichnis(Path::new(ZUHAUSE)));
    assert_eq!(startzeile(&vorgabe, gemerkt, zuhause), None);
    assert_eq!(startzeile(&vorgabe, None, zuhause), None);
    assert!(
        startzeile(&vorgabe, Some(Path::new("/Volumes/X")), zuhause)
            .is_some_and(|zeile| zeile.contains("/Volumes/X"))
    );
}

/// Die zwei Fehler zur ungelesenen Datei nennen den Weg hinaus und dass F2
/// nichts anlegt.
#[test]
fn die_meldungen_zur_ungelesenen_datei_nennen_den_weg() {
    let beschaedigt = Ortsfehler::EinstellungenBeschaedigt("ist beschädigt".to_owned()).meldung();
    for teil in [
        "settings.toml ist beschädigt",
        "kein Notizordner",
        "F2 legt nichts an",
        "berichtigen und KRK neu starten",
        "„Home“ → „Ort wählen…“",
    ] {
        assert!(beschaedigt.contains(teil), "{beschaedigt}");
    }
    let ungelesen = Ortsfehler::EinstellungenUngelesen("Sperre belegt".to_owned()).meldung();
    for teil in ["Sperre belegt", "F2 legt nichts an", "KRK neu starten"] {
        assert!(ungelesen.contains(teil), "{ungelesen}");
    }
}

/// H3, Schritt 3.3: die Frage „haelt der Editor eine Datei dieses Orts?“,
/// als Tafel. Gefragt wird sie mit demselben Editorzustand gegen zwei Orte.
#[test]
fn gehaltene_notizdatei_erkennt_die_drei_dateien_am_gefragten_ort() {
    let geltend = Heimordner::am_ort(PathBuf::from("/Users/probe/krkhome"), None);
    let gewaehlt = Heimordner::am_ort(PathBuf::from("/Volumes/X/notizen"), None);
    for sorte in Sonderdatei::ALLE {
        let pfad = geltend.geschrieben().join(sorte.dateiname());
        assert_eq!(
            gehaltene_notizdatei(Some(&pfad), false, Some(&geltend)),
            Some(sorte.dateiname().to_owned()),
            "{} am geltenden Ort",
            sorte.dateiname()
        );
        // Dieselbe Datei gegen den gewaehlten Ort: dort liegt sie nicht.
        assert_eq!(
            gehaltene_notizdatei(Some(&pfad), false, Some(&gewaehlt)),
            None
        );
    }
    // Eine Datei des gewaehlten statt des geltenden Orts (S1 der Zweitlesung).
    let dort = gewaehlt.geschrieben().join("secrets.txt");
    assert_eq!(
        gehaltene_notizdatei(Some(&dort), false, Some(&geltend)),
        None
    );
    assert_eq!(
        gehaltene_notizdatei(Some(&dort), false, Some(&gewaehlt)),
        Some("secrets.txt".to_owned())
    );
    // `secrets.txt` unter einer dritten Schreibweise: der Pfadtext erkennt sie
    // nicht, der Schluessel im Editor schon.
    let dritte = Path::new("/private/elsewhere/secrets.txt");
    assert_eq!(
        gehaltene_notizdatei(Some(dritte), false, Some(&geltend)),
        None
    );
    assert_eq!(
        gehaltene_notizdatei(Some(dritte), true, Some(&geltend)),
        Some("secrets.txt".to_owned())
    );
    // Eine andere Datei, kein Pfad, kein Ort.
    let andere = geltend.geschrieben().join("liste.txt");
    assert_eq!(
        gehaltene_notizdatei(Some(&andere), false, Some(&geltend)),
        None
    );
    assert_eq!(gehaltene_notizdatei(None, false, Some(&geltend)), None);
    assert_eq!(
        gehaltene_notizdatei(Some(&geltend.geschrieben().join("notes.txt")), false, None),
        None
    );
    assert_eq!(
        gehaltene_notizdatei(None, true, None),
        Some("secrets.txt".to_owned())
    );
}

/// Die kanonische Form eines gewaehlten Orts wird mit derselben Regel gegen
/// den Ablageordner gehalten wie ein Text in `settings.toml`.
#[test]
fn im_ablageordner_folgt_der_regel_von_ort_lesen() {
    let ablage = Path::new("/Users/probe/Library/Application Support/KRK");
    assert!(im_ablageordner(ablage, ablage));
    assert!(im_ablageordner(&ablage.join("notizen"), ablage));
    assert!(im_ablageordner(
        Path::new("/Users/probe/library/application support/krk/x"),
        ablage
    ));
    assert!(!im_ablageordner(
        Path::new("/Users/probe/Library/Application Support/KRK-alt"),
        ablage
    ));
    assert!(!im_ablageordner(Path::new("/Volumes/X"), ablage));
}

/// Die Saetze von „Ort waehlen…“ nennen, was der Nutzer wissen muss.
#[test]
fn die_saetze_der_ortswahl_nennen_datei_und_orte() {
    let abweisung = abweisungssatz("secrets.txt");
    assert!(abweisung.starts_with("Zuerst secrets.txt im Editor schließen"));
    assert_eq!(
        wahlsatz("~/neu", Some("~/krkhome")),
        wechselsatz("~/neu", "~/krkhome")
    );
    let ohne_alten = wahlsatz("~/neu", None);
    assert!(ohne_alten.contains("~/neu") && ohne_alten.contains("F2"));
    assert!(schon_der_ort("~/neu").contains("„~/neu“ ist schon der Notizordner"));
    let zurueck = zurueckgeschrieben("~/krkhome");
    assert!(zurueck.contains("„~/krkhome“") && zurueck.contains("seit dem Start"));
}

/// Die Sitzung merkt den geltenden Ort; gilt keiner, bleibt der gemerkte.
#[test]
fn zu_merken_behaelt_den_alten_ort_wenn_keiner_gilt() {
    let gemerkt = Path::new("/Users/probe/krkhome");
    let geltend: Notizort = Ok(Heimordner::am_ort(PathBuf::from("/Volumes/X"), None));
    assert_eq!(
        zu_merken(&geltend, Some(gemerkt)),
        Some(PathBuf::from("/Volumes/X"))
    );
    assert_eq!(
        zu_merken(&Err(Ortsfehler::Leer), Some(gemerkt)),
        Some(gemerkt.to_path_buf())
    );
    assert_eq!(zu_merken(&Err(Ortsfehler::Leer), None), None);
}

/// Gilt kein Ort, gelten die Schutzregeln am zuletzt geltenden Ort, ohne
/// einen gemerkten am Vorgabeort. Eine `secrets.txt` dort bleibt die
/// Geheimnisdatei; F2 sieht diesen Ordner nicht (Probe in `heimgriff.rs`).
#[test]
fn der_schutzort_ist_der_gemerkte_sonst_der_vorgabeort() {
    let zuhause = Path::new(ZUHAUSE);
    let vorgabe = schutzort(None, Some(zuhause)).expect("mit Benutzerverzeichnis");
    assert!(vorgabe.ist_vorgabeort());
    assert_eq!(
        vorgabe.sonderdatei(&zuhause.join(ORDNERNAME).join("secrets.txt")),
        Some(Sonderdatei::Geheimnisse)
    );
    let gemerkt =
        schutzort(Some(Path::new("/Volumes/X/notizen")), Some(zuhause)).expect("ein gemerkter Ort");
    assert_eq!(gemerkt.geschrieben(), Path::new("/Volumes/X/notizen"));
    let von_hand = schutzort(Some(Path::new("~/Dropbox/Notizen")), Some(zuhause))
        .expect("ein gemerkter Ort in der Form ~/");
    assert_eq!(
        von_hand.geschrieben(),
        Path::new("/Users/probe/Dropbox/Notizen")
    );
    assert_eq!(schutzort(None, None), None);
}

// ---------------------------------------------------------------------------
// Das Dateiformat von secrets.txt (Schritt 5.1, Faehigkeit C7)
// ---------------------------------------------------------------------------

/// Kleine Parameter, damit die Proben im Profil `dev` nicht auf die halbe
/// Sekunde der echten warten. Die Parameter reisen im Kopf mit, also prueft
/// jede Probe mit ihnen denselben Weg wie mit denen des Codes.
fn klein() -> Parameter {
    Parameter::neu(64, 1, 1).expect("die kleinen Parameter sind gueltig")
}

fn pin(ziffern: &str) -> Pin {
    Pin::aus_eingabe(ziffern).expect("vier Ziffern")
}

/// Ein Schluessel mit festem Salz und kleinen Parametern.
fn kleiner_schluessel(ziffern: &str) -> Schluessel {
    tresor::schluessel_ableiten(&pin(ziffern), &[3; SALZLAENGE], klein()).expect("Ableitung")
}

/// Der bekannte Eintrag, dessen Text in keinem Chiffrat vorkommen darf.
const BEKANNTER_EINTRAG: &str = "## Bank\nKontonummer 4711-0815-GEHEIM\n";

fn enthaelt(heuhaufen: &[u8], nadel: &[u8]) -> bool {
    heuhaufen
        .windows(nadel.len())
        .any(|fenster| fenster == nadel)
}

/// C7.5: Verschliessen und Oeffnen mit derselben PIN ergibt wieder dieselben
/// Bytes, auch fuer einen leeren Inhalt.
#[test]
fn verschliessen_und_oeffnen_ergeben_denselben_klartext() {
    let schluessel = kleiner_schluessel("0417");
    for klartext in ["", BEKANNTER_EINTRAG, "Umlaute äöü\r\nohne Schluss"] {
        let datei = tresor::verschliessen(klartext.as_bytes(), &schluessel).expect("verschliessen");
        let geoeffnet = tresor::oeffnen(&datei, &pin("0417")).expect("oeffnen");
        assert_eq!(geoeffnet.klartext, klartext.as_bytes());
        assert_eq!(geoeffnet.schluessel, schluessel);
    }
}

/// C7.3 und C7.4: die Datei beginnt mit dem Kopf, und der Text des bekannten
/// Eintrags kommt in ihr nicht vor, weder ganz noch in einem Stueck.
#[test]
fn der_bekannte_eintrag_steht_nicht_im_chiffrat() {
    let datei = tresor::verschliessen(BEKANNTER_EINTRAG.as_bytes(), &kleiner_schluessel("0417"))
        .expect("verschliessen");

    assert_eq!(&datei[..6], b"KRKSEC");
    assert_eq!(datei.len(), KOPFLAENGE + BEKANNTER_EINTRAG.len() + 16);
    for stueck in ["Kontonummer", "4711-0815", "GEHEIM", "Bank"] {
        assert!(
            !enthaelt(&datei, stueck.as_bytes()),
            "`{stueck}` steht im Chiffrat"
        );
    }
}

/// C7.6: eine falsche PIN und ein veraendertes Byte geben dieselbe Abweisung,
/// keinen Text, und die Datei auf der Platte bleibt Byte fuer Byte dieselbe.
/// Das gilt fuer ein Byte im Chiffrat, im Pruefwert und in jedem Feld des
/// Kopfes, das die Pruefung gelten laesst: der ganze Kopf geht als
/// zusaetzliche authentifizierte Daten ein.
#[test]
fn falsche_pin_und_veraendertes_byte_geben_dieselbe_abweisung() {
    let ordner = Pruefordner::neu("tresor-abweisung");
    let pfad = ordner.pfad().join("secrets.txt");
    let datei = tresor::verschliessen(BEKANNTER_EINTRAG.as_bytes(), &kleiner_schluessel("0417"))
        .expect("verschliessen");
    fs::write(&pfad, &datei).expect("die Pruefdatei laesst sich nicht schreiben");

    let gelesen = fs::read(&pfad).expect("lesbar");
    let ausgang = tresor::oeffnen(&gelesen, &pin("0418"));
    assert_eq!(
        ausgang.as_ref().map(|g| g.klartext.clone()).unwrap_err(),
        &Oeffnungsfehler::PinFalschOderVeraendert
    );
    assert_eq!(
        fs::read(&pfad).expect("lesbar"),
        datei,
        "die Datei hat sich geaendert"
    );

    // Ein Byte im Chiffrat, das letzte Byte des Pruefwerts, und je ein Byte
    // in Speicher, Durchlaeufen, Salz und Nonce des Kopfes. In den zwei
    // Parameterfeldern wird das Bit fuer die Zwei gekippt (64 KiB werden 66,
    // ein Durchlauf wird drei), damit der Wert in den Grenzen bleibt und die
    // Probe die Pruefung erreicht statt den Kopfschaden.
    let stellen = [KOPFLAENGE + 3, datei.len() - 1, 8, 12, 20, 36, 59];
    for stelle in stellen {
        let mut veraendert = datei.clone();
        veraendert[stelle] ^= if stelle == 8 || stelle == 12 {
            0x02
        } else {
            0x01
        };
        let ausgang = tresor::oeffnen(&veraendert, &pin("0417"));
        assert!(
            matches!(ausgang, Err(Oeffnungsfehler::PinFalschOderVeraendert)),
            "Byte {stelle}: {ausgang:?}"
        );
    }
}

/// C7.6: jeder Schaden am Kopf wird als solcher gemeldet, bevor abgeleitet
/// wird.
#[test]
fn jeder_kopfschaden_wird_als_solcher_gemeldet() {
    let datei =
        tresor::verschliessen(b"Inhalt", &kleiner_schluessel("0417")).expect("verschliessen");
    let mit = |stelle: usize, bytes: &[u8]| {
        let mut kopie = datei.clone();
        kopie[stelle..stelle + bytes.len()].copy_from_slice(bytes);
        kopie
    };
    let faelle: Vec<(&str, Vec<u8>, Kopfschaden)> = vec![
        ("leer", Vec::new(), Kopfschaden::Abgeschnitten),
        (
            "nur die Kennung",
            b"KRKSEC".to_vec(),
            Kopfschaden::Abgeschnitten,
        ),
        (
            "mitten im Kopf",
            datei[..KOPFLAENGE - 1].to_vec(),
            Kopfschaden::Abgeschnitten,
        ),
        (
            "fremde Kennung",
            mit(0, b"KRKSEX"),
            Kopfschaden::FalscheKennung,
        ),
        (
            "Klartext",
            b"## Thema\nText\n".to_vec(),
            Kopfschaden::FalscheKennung,
        ),
        ("Version 0", mit(6, &[0]), Kopfschaden::UnbekannteVersion(0)),
        ("Version 2", mit(6, &[2]), Kopfschaden::UnbekannteVersion(2)),
        (
            "Ableitung 2",
            mit(7, &[2]),
            Kopfschaden::UnbekannteAbleitung(2),
        ),
        (
            "kein Speicher",
            mit(8, &0u32.to_le_bytes()),
            Kopfschaden::UngueltigeParameter,
        ),
        (
            "zu viel Speicher",
            mit(8, &u32::MAX.to_le_bytes()),
            Kopfschaden::UngueltigeParameter,
        ),
        (
            "kein Durchlauf",
            mit(12, &0u32.to_le_bytes()),
            Kopfschaden::UngueltigeParameter,
        ),
        (
            "zu viele Durchlaeufe",
            mit(12, &65u32.to_le_bytes()),
            Kopfschaden::UngueltigeParameter,
        ),
        (
            "keine Spur",
            mit(16, &0u32.to_le_bytes()),
            Kopfschaden::UngueltigeParameter,
        ),
        (
            "zu viele Spuren",
            mit(16, &17u32.to_le_bytes()),
            Kopfschaden::UngueltigeParameter,
        ),
    ];
    for (name, bytes, schaden) in faelle {
        let ausgang = tresor::oeffnen(&bytes, &pin("0417"));
        assert!(
            matches!(&ausgang, Err(Oeffnungsfehler::KopfBeschaedigt(s)) if *s == schaden),
            "{name}: {ausgang:?}"
        );
    }
}

/// C7.7 und die Beschreibung des Kopfes: eine Datei, allein nach der Tabelle
/// im Modulkopf von `heimordner/tresor.rs` gebaut, mit den Kisten selbst und
/// ohne einen Weg dieses Moduls, mit Formatversion 1 und Parametern, die
/// kleiner sind als die des Codes, oeffnet mit derselben PIN. Die Parameter
/// lesen sich also aus dem Kopf und nicht aus dem Code.
#[test]
fn eine_von_hand_gebaute_datei_mit_kleineren_parametern_oeffnet() {
    use argon2::{Algorithm, Argon2, Params, Version};
    use chacha20poly1305::aead::{Aead, KeyInit, Payload};
    use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};

    let (m, t, p) = (32u32, 2u32, 1u32);
    assert!(m < Parameter::DES_CODES.speicher_kib());
    let salz = [0x5a; 16];
    let nonce = [0xa5; 24];
    let mut kopf = Vec::new();
    kopf.extend_from_slice(b"KRKSEC");
    kopf.push(1);
    kopf.push(1);
    kopf.extend_from_slice(&m.to_le_bytes());
    kopf.extend_from_slice(&t.to_le_bytes());
    kopf.extend_from_slice(&p.to_le_bytes());
    kopf.extend_from_slice(&salz);
    kopf.extend_from_slice(&nonce);
    assert_eq!(kopf.len(), 60);

    let mut schluessel = [0u8; 32];
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(m, t, p, Some(32)).expect("Parameter"),
    )
    .hash_password_into(b"9031", &salz, &mut schluessel)
    .expect("Ableitung");
    let chiffrat = XChaCha20Poly1305::new(&Key::from(schluessel))
        .encrypt(
            &XNonce::from(nonce),
            Payload {
                msg: BEKANNTER_EINTRAG.as_bytes(),
                aad: &kopf,
            },
        )
        .expect("Verschluesselung");
    let mut datei = kopf.clone();
    datei.extend_from_slice(&chiffrat);

    let geoeffnet = tresor::oeffnen(&datei, &pin("9031")).expect("die Datei oeffnet nicht");
    assert_eq!(geoeffnet.klartext, BEKANNTER_EINTRAG.as_bytes());
    assert_eq!(
        geoeffnet.schluessel.parameter(),
        Parameter::neu(m, t, p).expect("gueltig")
    );
    assert_eq!(geoeffnet.schluessel.salz(), &salz);
    assert_eq!(
        Kopf::lesen(&datei).expect("lesbar").version(),
        FORMATVERSION
    );
}

/// C7.3: zwei Sicherungen desselben Klartexts mit demselben Schluessel
/// ergeben verschiedene Bytes, weil jede eine neue Nonce zieht, und tragen
/// dasselbe Salz und dieselben Parameter.
#[test]
fn zwei_sicherungen_ergeben_verschiedene_bytes_mit_demselben_salz() {
    let schluessel = kleiner_schluessel("0417");
    let erste = tresor::verschliessen(BEKANNTER_EINTRAG.as_bytes(), &schluessel).expect("erste");
    let zweite = tresor::verschliessen(BEKANNTER_EINTRAG.as_bytes(), &schluessel).expect("zweite");

    assert_ne!(erste, zweite);
    let (kopf1, kopf2) = (
        Kopf::lesen(&erste).expect("lesbar"),
        Kopf::lesen(&zweite).expect("lesbar"),
    );
    assert_eq!(kopf1.salz(), kopf2.salz());
    assert_eq!(kopf1.salz(), schluessel.salz());
    assert_eq!(kopf1.parameter(), kopf2.parameter());
    assert_ne!(kopf1.nonce(), kopf2.nonce());
}

/// C7.3: der Schluessel aus `oeffnen` verschliesst so, dass Salz und
/// Parameter im neuen Kopf denen der geoeffneten Datei gleichen. Eine
/// gewoehnliche Sicherung uebernimmt sie also und hebt die Parameter nicht auf
/// die des Codes an.
#[test]
fn der_schluessel_aus_oeffnen_behaelt_salz_und_parameter() {
    let alt = tresor::verschliessen(b"vorher", &kleiner_schluessel("2468")).expect("verschliessen");
    let geoeffnet = tresor::oeffnen(&alt, &pin("2468")).expect("oeffnen");
    let neu = tresor::verschliessen(b"nachher", &geoeffnet.schluessel).expect("verschliessen");

    let (kopf_alt, kopf_neu) = (
        Kopf::lesen(&alt).expect("lesbar"),
        Kopf::lesen(&neu).expect("lesbar"),
    );
    assert_eq!(kopf_neu.salz(), kopf_alt.salz());
    assert_eq!(kopf_neu.parameter(), kopf_alt.parameter());
    assert_ne!(kopf_neu.parameter(), Parameter::DES_CODES);
    assert_eq!(
        tresor::oeffnen(&neu, &pin("2468"))
            .expect("oeffnen")
            .klartext,
        b"nachher"
    );
}

/// C7.3: ein neuer Schluessel zieht ein frisches Salz und nimmt die Parameter
/// des Codes. Die einzige Probe, die die volle Ableitung bezahlt, und zwar
/// zweimal.
#[test]
fn ein_neuer_schluessel_zieht_frisches_salz_mit_den_parametern_des_codes() {
    let erster = tresor::neuer_schluessel(&pin("1357")).expect("erster");
    let zweiter = tresor::neuer_schluessel(&pin("1357")).expect("zweiter");

    assert_eq!(erster.parameter(), Parameter::DES_CODES);
    assert_ne!(erster.salz(), zweiter.salz());
    assert_ne!(erster, zweiter);
}

/// C7.2: die PIN besteht aus genau vier ASCII-Ziffern.
#[test]
fn die_pin_nimmt_genau_vier_ziffern_an() {
    for zahl in 0..10_000 {
        let eingabe = format!("{zahl:04}");
        assert!(Pin::aus_eingabe(&eingabe).is_ok(), "{eingabe} abgewiesen");
    }
    for eingabe in [
        "",
        "123",
        "12345",
        "abcd",
        "12a4",
        " 1234",
        "1234 ",
        "12 4",
        "1234\n",
        "\t123",
        "-123",
        "+123",
        "١٢٣٤",
        "１２３４",
    ] {
        assert_eq!(
            Pin::aus_eingabe(eingabe),
            Err(Pinfehler::KeineVierZiffern),
            "{eingabe:?} angenommen"
        );
    }
}

/// Die Saetze, die aus diesem Format an den Nutzer gehen, im Wortlaut und mit
/// Umlauten. Fuer eine falsche PIN und eine veraenderte Datei ist es ein Satz.
#[test]
fn die_meldungen_des_tresors_stehen_im_wortlaut() {
    assert_eq!(
        Oeffnungsfehler::PinFalschOderVeraendert.meldung(),
        "PIN falsch oder Datei verändert"
    );
    assert_eq!(
        Pinfehler::KeineVierZiffern.meldung(),
        "Die PIN besteht aus genau vier Ziffern."
    );
    assert_eq!(
        Oeffnungsfehler::KopfBeschaedigt(Kopfschaden::UnbekannteVersion(7)).meldung(),
        "Der Kopf der Datei ist beschädigt: unbekannte Formatversion 7"
    );
    assert_eq!(
        Oeffnungsfehler::KopfBeschaedigt(Kopfschaden::UngueltigeParameter).meldung(),
        "Der Kopf der Datei ist beschädigt: die Parameter der Ableitung sind ungültig"
    );
}

/// Der Rumpf einer freien Funktion in `heimordner/tresor.rs`, ohne
/// Kommentarzeilen; er endet an der ersten schliessenden Klammer am
/// Zeilenanfang.
fn freier_rumpf(inhalt: &str, name: &str) -> String {
    let kopf = format!("\npub fn {name}(");
    let beginn = inhalt
        .find(&kopf)
        .unwrap_or_else(|| panic!("{kopf} steht nicht in heimordner/tresor.rs"));
    let rest = &inhalt[beginn..];
    let ende = rest
        .find("\n}\n")
        .unwrap_or_else(|| panic!("der Rumpf von {name} endet nicht"));
    rest[..ende]
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// C7.3: eine gewoehnliche Sicherung leitet nicht ab. `verschliessen` nennt
/// weder die Ableitung noch Argon2, und abgeleitet wird allein in `oeffnen`
/// und `neuer_schluessel`, deren einziger Weg `schluessel_ableiten` ist.
#[test]
fn verschliessen_leitet_nicht_ab_und_abgeleitet_wird_an_zwei_stellen() {
    let inhalt = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/heimordner/tresor.rs"
    ))
    .expect("heimordner/tresor.rs ist nicht lesbar");

    let verschliessen = freier_rumpf(&inhalt, "verschliessen");
    assert!(
        verschliessen.contains("encrypt"),
        "nicht gelesen:\n{verschliessen}"
    );
    for nadel in [
        "schluessel_ableiten",
        "neuer_schluessel",
        "Argon2",
        "hash_password",
    ] {
        assert!(
            !verschliessen.contains(nadel),
            "verschliessen nennt `{nadel}`:\n{verschliessen}"
        );
    }

    // Ausserhalb des Pruefmoduls steht `hash_password_into` einmal, in
    // `schluessel_ableiten`, und `schluessel_ableiten(` wird genau zweimal
    // gerufen, in `oeffnen` und in `neuer_schluessel`.
    let code = inhalt
        .split("#[cfg(test)]")
        .next()
        .expect("Code vor den Proben");
    let code: String = code
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code.matches("hash_password_into").count(), 1);
    assert_eq!(
        code.matches("schluessel_ableiten(").count(),
        3,
        "Definition und zwei Rufer"
    );
    assert!(freier_rumpf(&inhalt, "oeffnen").contains("schluessel_ableiten("));
    assert!(freier_rumpf(&inhalt, "neuer_schluessel").contains("schluessel_ableiten("));
}

/// Die Messung, aus der [`Parameter::DES_CODES`] stammt. Laeuft nur auf
/// ausdruecklichen Aufruf und nur sinnvoll im Profil `release`:
///
/// ```sh
/// cargo test --release -p krk-core --test heimordner -- --ignored argon2 --nocapture
/// ```
///
/// Gibt je Reihe aus Speicher und Durchlaeufen den Median aus fuenf
/// Ableitungen aus. Das Ergebnis vom 260926 auf dem Referenzgeraet steht im
/// Modulkopf von `heimordner/tresor.rs`.
#[test]
#[ignore = "Messung auf dem Referenzgeraet, nur im Profil release aussagekraeftig"]
fn argon2_parameter_auf_dem_referenzgeraet_messen() {
    use std::time::Instant;

    let pin = pin("0417");
    let salz = [0x42; SALZLAENGE];
    for (speicher_mib, durchlaeufe) in [
        (64, 3),
        (128, 2),
        (128, 3),
        (128, 4),
        (128, 5),
        (128, 6),
        (128, 7),
        (192, 3),
        (256, 2),
        (256, 3),
        (256, 4),
        (384, 2),
        (512, 2),
    ] {
        let parameter = Parameter::neu(speicher_mib * 1024, durchlaeufe, 1).expect("gueltig");
        let mut zeiten: Vec<f64> = (0..5)
            .map(|_| {
                let beginn = Instant::now();
                let _ = tresor::schluessel_ableiten(&pin, &salz, parameter).expect("Ableitung");
                beginn.elapsed().as_secs_f64()
            })
            .collect();
        zeiten.sort_by(f64::total_cmp);
        println!(
            "argon2id m={speicher_mib} MiB t={durchlaeufe} p=1: Median {:.3} s (min {:.3}, max {:.3})",
            zeiten[2], zeiten[0], zeiten[4]
        );
    }
}
