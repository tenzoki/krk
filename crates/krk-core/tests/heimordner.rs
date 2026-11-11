//! Abnahme des Heimordners `~/krkhome/`: Erkennung und Form der Eintraege
//! (Schritt 1.1), Anlegen (Schritt 1.2) und die
//! Handlungen an Aufgaben (Schritt 3.1, Faehigkeit C6) des
//! Plans `260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`,
//! Faehigkeiten C2 und C3 des Spec. Die Uebernahme der alten Zettel und die
//! Handlungen an `notes.txt` sind am 261111 gefallen, und ihre Proben mit
//! ihnen (Schritt 4 des Plans
//! `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`); was
//! von der Form aus Themenzeile und Text steht, traegt die Geheimnisse. Die Rennprobe zum exklusiven Anlegen steht
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
use krk_core::heimordner::eintraege::geheimnisse::{
    self, Feld, Geheimnisentwurf, Geheimnisse, Passwort, Suchfehler,
};
use krk_core::heimordner::eintraege::{
    Abweisung, Aufgaben, Notiz, Notizen, Richtung, aufgabe_in_grundform, aufgaben, aufgabenzeile,
    ist_themenzeile, notizen,
};
use krk_core::heimordner::ort::{
    Notizort, Ortsfehler, abweisungssatz, gehaltene_notizdatei, im_ablageordner, notizort,
    ort_lesen, ortswechsel, schon_der_ort, schreibform, schutzort, startzeile, wahlsatz,
    wechselsatz, zu_merken, zurueckgeschrieben,
};
use krk_core::heimordner::schluesselmerker::{GUELTIGKEIT, Schluesselmerker};
use krk_core::heimordner::tresor::{
    self, FORMATVERSION, KOPFLAENGE, Kopf, Kopfschaden, Oeffnungsfehler, Parameter, Pin, Pinfehler,
    SALZLAENGE, Schluessel,
};
use krk_core::heimordner::verbindung::{
    Protokoll, Verbindungsziel, Weg, Zielfehler, protokoll_pruefen, verbindungsziel,
};
use krk_core::heimordner::{
    ALTE_QUICKNOTEDATEI, ALTER_GEHEIMNISNAME, Bereitstellung, Heimordner, Hindernis, ORDNERNAME,
    PROTOKOLLORDNER, QUICKNOTEDATEI, Sonderdatei, Umbenennung, bereitstellen,
    ohne_ersetzen_umbenennen, protokollname,
};
use krk_core::verzeichnis::sys::Ortszeit;

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
        heim.sonderdatei(&ziel.join("tasks.txt")),
        Some(Sonderdatei::Aufgaben)
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

    assert_eq!(heim.sonderdatei(Path::new("/Users/probe/tasks.txt")), None);
    assert_eq!(
        heim.sonderdatei(Path::new("/Users/probe/krkhome/unter/tasks.txt")),
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
        heim.sonderdatei(Path::new("/Users/probe/krkhome/tasks.txt")),
        Some(Sonderdatei::Aufgaben)
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

/// Schritt 3 des Plans
/// `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md` (C3.1,
/// C3.2, C3.5): `appointments.md` und `notes.txt` sind keine Eintragsdateien
/// mehr. Keine Sorte traegt einen der Namen, und die Erkennung am Pfadtext
/// antwortet fuer sie im Notizordner wie anderswo mit `None`, ueber die
/// geschriebene wie ueber die aufgeloeste Form des Ordners.
#[test]
fn appointments_md_und_notes_txt_sind_keine_eintragsdateien_mehr() {
    for name in ["appointments.md", "notes.txt"] {
        assert!(
            Sonderdatei::ALLE
                .iter()
                .all(|sorte| sorte.dateiname() != name),
            "eine Sorte traegt noch den Namen {name}"
        );
    }
    assert_eq!(
        Sonderdatei::ALLE.map(Sonderdatei::dateiname),
        ["tasks.txt", "secrets.txt"],
        "die Eintragsdateien des Notizordners"
    );
    let heim = Heimordner::im_benutzerverzeichnis(Path::new("/Users/probe"));
    for pfad in [
        "/Users/probe/krkhome/appointments.md",
        "/Users/probe/appointments.md",
        "/Users/probe/krkhome/unter/appointments.md",
        "/Users/probe/krkhome-alt/appointments.md",
        "/Users/probe/krkhome/Appointments.MD",
        "/Users/probe/krkhome/appointments.txt",
        "/Users/probe/krkhome/notes.txt",
        "/Users/probe/notes.txt",
        "/Users/probe/krkhome/unter/notes.txt",
        "/Users/probe/krkhome/Notes.TXT",
    ] {
        assert_eq!(heim.sonderdatei(Path::new(pfad)), None, "{pfad}");
    }

    // Dasselbe ueber einen Verweis, also ueber beide Formen des Ordners.
    let ordner = Pruefordner::neu("heim-termine-verweis");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("ziel");
    fs::create_dir(&ziel).expect("das Ziel laesst sich nicht anlegen");
    std::os::unix::fs::symlink(&ziel, zuhause.join(ORDNERNAME))
        .expect("der Verweis laesst sich nicht anlegen");
    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
    for basis in [zuhause.join(ORDNERNAME), ziel] {
        assert_eq!(
            heim.sonderdatei(&basis.join("tasks.txt")),
            Some(Sonderdatei::Aufgaben),
            "die Probe haengt daran, dass der Ordner unter {} erkannt ist",
            basis.display()
        );
        for name in ["appointments.md", "notes.txt"] {
            assert_eq!(
                heim.sonderdatei(&basis.join(name)),
                None,
                "{name} unter {}",
                basis.display()
            );
        }
    }
}

/// Die genaue Frage antwortet fuer `secrets.txt` wie vorher und fuer
/// `appointments.md` in keiner Schreibung mit einer Sorte.
#[test]
fn die_genaue_frage_nennt_allein_secrets_txt_als_geheimnisse() {
    let ordner = Pruefordner::neu("heim-genau-termine");
    let zuhause = kanonisch(&ordner);
    let heimpfad = zuhause.join(ORDNERNAME);
    fs::create_dir(&heimpfad).expect("Heimordner laesst sich nicht anlegen");
    for name in ["secrets.txt", "appointments.md"] {
        fs::write(heimpfad.join(name), b"").expect("leere Datei");
    }
    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);

    assert_eq!(
        heim.sonderdatei_genau(&heimpfad.join("secrets.txt")),
        Some(Sonderdatei::Geheimnisse)
    );
    for name in ["appointments.md", "Appointments.MD"] {
        assert_eq!(heim.sonderdatei_genau(&heimpfad.join(name)), None, "{name}");
    }
}

/// Schritt 2 des Plans der Quicknote (E1): `quicknote.txt` liegt im Notizordner,
/// ist aber keine Eintragsdatei. Weder die Erkennung am Pfadtext noch die
/// genaue Frage antwortet fuer sie mit einer Sorte, und ihr Pfad ist der
/// geschriebene Ort samt Name. Dasselbe gilt fuer den frueheren Namen
/// `scratch.txt`, den allein die Uebernahme noch nennt (C2.6 des Spec
/// `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`).
#[test]
fn die_quicknotedatei_ist_keine_eintragsdatei() {
    assert_eq!(QUICKNOTEDATEI, "quicknote.txt");
    assert_eq!(ALTE_QUICKNOTEDATEI, "scratch.txt");
    for sorte in Sonderdatei::ALLE {
        assert_ne!(sorte.dateiname(), QUICKNOTEDATEI, "{sorte:?}");
        assert_ne!(sorte.dateiname(), ALTE_QUICKNOTEDATEI, "{sorte:?}");
    }

    let ordner = Pruefordner::neu("heim-quicknote");
    let zuhause = kanonisch(&ordner);
    let heimpfad = zuhause.join(ORDNERNAME);
    fs::create_dir(&heimpfad).expect("Heimordner laesst sich nicht anlegen");
    fs::write(heimpfad.join(QUICKNOTEDATEI), b"notiert").expect("quicknote.txt");
    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);

    let pfad = heim.quicknotedatei();
    assert_eq!(pfad, heim.geschrieben().join("quicknote.txt"));
    assert_eq!(heim.sonderdatei(&pfad), None);
    assert_eq!(heim.sonderdatei_genau(&pfad), None);

    let frueher = heim.alte_quicknotedatei();
    assert_eq!(frueher, heim.geschrieben().join("scratch.txt"));
    assert_eq!(heim.sonderdatei(&frueher), None);
    assert_eq!(heim.sonderdatei_genau(&frueher), None);
}

/// F2 legt weder `quicknote.txt` noch eine Datei unter dem frueheren Namen an
/// und fasst eine vorhandene `scratch.txt` nicht an: die Uebernahme ist Sache
/// der Sicherung der Quicknote und nicht von `bereitstellen` (C2.6).
#[test]
fn f2_legt_die_quicknotedatei_nicht_an_und_laesst_die_fruehere_liegen() {
    let lage = Lage::neu("heim-quicknote-f2", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let frueher = lage.heimpfad().join(ALTE_QUICKNOTEDATEI);
    fs::write(&frueher, b"von vorher").expect("scratch.txt");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(bereitstellung.umbenennung, None);
    assert!(!lage.heimpfad().join(QUICKNOTEDATEI).exists());
    assert_eq!(fs::read(&frueher).expect("scratch.txt"), b"von vorher");
}

/// Die Umbenennung ohne Ersetzen an zwei beliebigen Namen, so wie die
/// Sicherung der Quicknote sie ruft: jeder Ausgang, den ein Pruefordner
/// herstellen kann, und in keinem wird eine vorhandene Datei ersetzt (C2.2
/// bis C2.4).
#[test]
fn ohne_ersetzen_umbenennen_ersetzt_nie_eine_vorhandene_datei() {
    use std::os::unix::fs::MetadataExt;

    let ordner = Pruefordner::neu("heim-umbenennen");
    let wurzel = kanonisch(&ordner);
    let alt = wurzel.join(ALTE_QUICKNOTEDATEI);
    let neu = wurzel.join(QUICKNOTEDATEI);

    // Keine alte Datei: nichts geschieht, nichts entsteht.
    assert_eq!(ohne_ersetzen_umbenennen(&alt, &neu), None);
    assert!(!neu.exists());

    // Allein die alte: sie traegt danach den neuen Namen, dieselbe Inode.
    fs::write(&alt, b"alter Inhalt").expect("alt");
    let inode = fs::metadata(&alt).expect("alt").ino();
    assert_eq!(
        ohne_ersetzen_umbenennen(&alt, &neu),
        Some(Umbenennung::Umbenannt)
    );
    assert!(!alt.exists());
    assert_eq!(fs::metadata(&neu).expect("neu").ino(), inode);
    assert_eq!(fs::read(&neu).expect("neu"), b"alter Inhalt");

    // Beide stehen: beide bleiben Byte fuer Byte, und die neue behaelt ihre
    // Inode, ist also nicht ersetzt.
    fs::write(&alt, b"noch eine alte").expect("alt");
    assert_eq!(
        ohne_ersetzen_umbenennen(&alt, &neu),
        Some(Umbenennung::BeideStehen)
    );
    assert_eq!(fs::read(&alt).expect("alt"), b"noch eine alte");
    assert_eq!(fs::read(&neu).expect("neu"), b"alter Inhalt");
    assert_eq!(fs::metadata(&neu).expect("neu").ino(), inode);

    // Der Ordner fehlt: wie ohne alte Datei, und kein Ordner entsteht.
    let fort = wurzel.join("fehlt");
    assert_eq!(
        ohne_ersetzen_umbenennen(&fort.join(ALTE_QUICKNOTEDATEI), &fort.join(QUICKNOTEDATEI)),
        None
    );
    assert!(!fort.exists());
}

/// Die Meldung der Uebernahme nennt beide Dateinamen und kommt aus den
/// neutralen Saetzen der Umbenennung, je Ausgang einer; die Saetze der
/// Sicherung nennen in jeder Sprache den heutigen Namen und nie den frueheren
/// (C2.5).
#[test]
fn die_uebernahme_der_quicknote_meldet_sich_mit_beiden_namen() {
    use krk_core::heimordner::quicknote_uebernahmemeldung;
    use krk_core::sprache::{Sprache, Text};

    assert_eq!(
        quicknote_uebernahmemeldung(&Umbenennung::Umbenannt, "~/krkhome"),
        "scratch.txt heißt jetzt quicknote.txt"
    );
    let beide = quicknote_uebernahmemeldung(&Umbenennung::BeideStehen, "~/krkhome");
    assert_eq!(
        beide,
        "quicknote.txt und scratch.txt stehen beide in ~/krkhome; KRK benennt keine um, und es gilt quicknote.txt"
    );
    for ausgang in [
        Umbenennung::AlterNameBleibt("der Grund".to_owned()),
        Umbenennung::Gescheitert("der Grund".to_owned()),
    ] {
        let meldung = quicknote_uebernahmemeldung(&ausgang, "~/krkhome");
        for teil in ["scratch.txt", "quicknote.txt", "der Grund"] {
            assert!(meldung.contains(teil), "{ausgang:?}: {meldung}");
        }
    }
    for sprache in [Sprache::De, Sprache::Fr, Sprache::En] {
        for schluessel in [Text::QuicknoteFremdGeaendert, Text::QuicknoteNichtGelesen] {
            let wortlaut = sprache.text(schluessel);
            assert!(wortlaut.contains(QUICKNOTEDATEI), "{sprache:?}: {wortlaut}");
            assert!(
                !wortlaut.contains(ALTE_QUICKNOTEDATEI),
                "{sprache:?}: {wortlaut}"
            );
        }
    }
}

/// Der Rumpf von `ohne_ersetzen_umbenennen` nennt kein `rename` und stellt
/// vor dem harten Verweis keine Frage an die Platte (C2.4).
///
/// `rename(2)` ersetzte ein vorhandenes Ziel still, und eine Existenzpruefung
/// vor dem Verweis liesse das Fenster offen, in dem eine entstehende Datei
/// ersetzt wuerde; die eine Frage, ob der neue Name frei ist, beantwortet
/// `link(2)` selbst. Gelesen werden die Codezeilen des Rumpfs: `hard_link`
/// ist der erste Aufruf, der die Platte erreicht, und `rename` steht nirgends.
///
/// **Was sie nicht sieht:** eine Pruefung beim Rufer vor dem Aufruf, und einen
/// Helfer, den der Rumpf vor dem Verweis ruft und der selbst die Platte fragt;
/// heute ruft er vor dem Verweis nichts.
#[test]
fn ohne_ersetzen_umbenennen_prueft_nicht_vor_dem_verweis_und_benennt_nicht_um() {
    let inhalt = fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/heimordner/bereitstellen.rs"
    ))
    .expect("heimordner/bereitstellen.rs ist nicht lesbar");
    let kopf = concat!("pub fn ohne_ersetzen_", "umbenennen(");
    let beginn = inhalt
        .find(kopf)
        .expect("ohne_ersetzen_umbenennen steht nicht in bereitstellen.rs");
    let rest = &inhalt[beginn..];
    let ende = rest.find("\n}\n").expect("der Rumpf endet nicht");
    let rumpf: String = rest[..ende]
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        !rumpf.contains("rename"),
        "der Rumpf benennt ueber rename um und ersetzte ein vorhandenes Ziel"
    );
    let verweis = rumpf
        .find(concat!("fs::hard_", "link("))
        .expect("der Rumpf legt keinen harten Verweis an");
    let (_, rumpf_ab_klammer) = rumpf[..verweis]
        .split_once('{')
        .expect("der Rumpf oeffnet nicht");
    for nadel in [
        "fs::",
        "metadata",
        "exists(",
        "File::",
        "OpenOptions",
        "read_link",
        "canonicalize",
        "is_file(",
        "is_dir(",
    ] {
        assert!(
            !rumpf_ab_klammer.contains(nadel),
            "vor dem harten Verweis steht {nadel}: eine Pruefung dort laesst das Fenster offen"
        );
    }
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
/// `ohne_inhaltsauftrag` und `ist_protokollordner` ebenso wenig.
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
/// `ist_protokollordner` steht seit dem 261111 dabei: „Leere Datei anlegen“
/// fragt es auf dem Hauptfaden, und es ruft wie die zwei anderen allein `ist`.
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
    for name in [
        "ist",
        "sonderdatei",
        "ohne_inhaltsauftrag",
        "ist_protokollordner",
    ] {
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
// Die Form aus Themenzeile und Text, die `secrets.txt` traegt (frueher die
// Handlungen an `notes.txt`, Schritt 4.1). Hinzufuegen und Aendern einer Notiz
// sind am 261111 gefallen; Lesen, Loeschen und Verschieben rufen die
// Geheimnisse weiter.
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
// Anlegen (Schritt 1.2); die Uebernahme der alten Zettel ist am 261111
// gefallen (C3.7 des Spec
// `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`)
// ---------------------------------------------------------------------------

/// Die zwei Zettel des frueheren Notizblatts im Ablageordner. Der Kern nennt
/// die Namen nicht mehr; hier stehen sie, weil die Proben halten, dass F2 sie
/// liegen laesst.
const ZETTELNAMEN: [&str; 2] = ["note-1.txt", "note-2.txt"];

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
        for (name, inhalt) in ZETTELNAMEN.iter().zip(zettel) {
            if let Some(inhalt) = inhalt {
                fs::write(ablage.join(name), inhalt).expect("Zettel");
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

    /// Der Weg von F2 am Vorgabeort.
    fn bereitstellen(&self) -> Result<Bereitstellung, Hindernis> {
        bereitstellen(&self.heim())
    }

    fn lesen(&self, sorte: Sonderdatei) -> String {
        fs::read_to_string(self.heimpfad().join(sorte.dateiname()))
            .unwrap_or_else(|_| panic!("{} fehlt", sorte.dateiname()))
    }

    /// C3.7: die alten Zettel sind Byte fuer Byte, was sie vorher waren, ein
    /// fehlender ist nicht entstanden, und im Ablageordner steht sonst nichts,
    /// auch keine `reported.toml`: F2 beruehrt die Ablage nicht.
    fn zettel_unveraendert(&self) {
        let mut erwartet = Vec::new();
        for (name, inhalt) in ZETTELNAMEN.iter().zip(self.zettel) {
            let pfad = self.ablage.join(name);
            match inhalt {
                Some(inhalt) => {
                    assert_eq!(
                        fs::read(&pfad).expect("der Zettel ist weg"),
                        inhalt,
                        "{name} ist veraendert"
                    );
                    erwartet.push((*name).to_owned());
                }
                None => assert!(!pfad.exists(), "{name} ist entstanden"),
            }
        }
        assert_eq!(
            namen_in(&self.ablage),
            erwartet,
            "F2 hat im Ablageordner etwas angelegt"
        );
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
    let vorher = b"- [ ] Eigenes
nicht anfassen\r\n\xef\xbb\xbf";
    fs::write(lage.heimpfad().join("tasks.txt"), vorher).expect("tasks.txt");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(
        fs::read(lage.heimpfad().join("tasks.txt")).expect("tasks.txt"),
        vorher
    );
    assert_eq!(lage.lesen(Sonderdatei::Geheimnisse), "");
    assert!(!bereitstellung.ordner_angelegt);
    assert_eq!(bereitstellung.angelegt, vec![Sonderdatei::Geheimnisse]);
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    lage.zettel_unveraendert();
}

/// C3.5 des Spec `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`,
/// C2 und C7.1 des frueheren: in einem fehlenden und in einem leeren
/// Notizordner legt der Aufruf genau `tasks.txt` und `secrets.txt` an und
/// sonst keine Datei, weder `notes.txt` noch `appointments.md`; beide haben
/// null Bytes. Daneben steht seit C4.1 der Unterordner `minutes/`.
#[test]
fn der_erste_aufruf_legt_genau_tasks_txt_und_secrets_txt_an() {
    for (zweck, ordner_steht) in [("heim-erster-fehlt", false), ("heim-erster-leer", true)] {
        let lage = Lage::neu(zweck, [None, None]);
        if ordner_steht {
            fs::create_dir(lage.heimpfad()).expect("Heimordner");
        }

        let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

        assert_eq!(bereitstellung.ordner_angelegt, !ordner_steht, "{zweck}");
        assert_eq!(
            bereitstellung.angelegt,
            vec![Sonderdatei::Aufgaben, Sonderdatei::Geheimnisse],
            "{zweck}"
        );
        assert_eq!(
            namen_in(&lage.heimpfad()),
            vec!["minutes", "secrets.txt", "tasks.txt"],
            "{zweck}"
        );
        for name in Sonderdatei::ALLE.map(Sonderdatei::dateiname) {
            assert_eq!(
                fs::metadata(lage.heimpfad().join(name))
                    .unwrap_or_else(|_| panic!("{name} fehlt"))
                    .len(),
                0,
                "{name} entsteht mit null Bytes"
            );
        }
        assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
        lage.zettel_unveraendert();
    }
}

/// C3.7: ein neu angelegter Vorgabeort uebernimmt die alten Zettel `note-1.txt`
/// und `note-2.txt` aus dem Ablageordner nicht. Es entsteht keine `notes.txt`,
/// die zwei Dateien bleiben Byte fuer Byte liegen, im Ablageordner entsteht
/// nichts, und kein Satz der Statuszeile nennt sie oder eine Uebernahme. Auch
/// ein zweiter Aufruf und ein geloeschter und neu angelegter Ordner aendern
/// daran nichts.
#[test]
fn ein_neu_angelegter_vorgabeort_uebernimmt_die_alten_zettel_nicht() {
    let lage = Lage::neu(
        "heim-keine-uebernahme",
        [
            Some("erste Zeile\n# eine Raute\n".as_bytes()),
            Some("ohne Schlussumbruch".as_bytes()),
        ],
    );

    for durchgang in ["erster", "zweiter", "nach dem Loeschen"] {
        if durchgang == "nach dem Loeschen" {
            fs::remove_dir_all(lage.heimpfad()).expect("der Heimordner laesst sich nicht loeschen");
        }
        let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");
        assert_eq!(
            bereitstellung.ordner_angelegt,
            durchgang != "zweiter",
            "{durchgang} Aufruf"
        );
        assert_eq!(
            namen_in(&lage.heimpfad()),
            vec!["minutes", "secrets.txt", "tasks.txt"],
            "{durchgang} Aufruf"
        );
        assert_eq!(lage.lesen(Sonderdatei::Aufgaben), "", "{durchgang} Aufruf");
        assert!(
            bereitstellung.meldungen().is_empty(),
            "{durchgang} Aufruf meldet: {:?}",
            bereitstellung.meldungen()
        );
        lage.zettel_unveraendert();
    }
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

    assert_eq!(bereitstellung.umbenennung, Some(Umbenennung::Umbenannt));
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
        vec!["minutes", "secrets.txt", "tasks.txt"]
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

    assert_eq!(bereitstellung.umbenennung, Some(Umbenennung::BeideStehen));
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
    assert_eq!(erste.umbenennung, None);
    assert!(
        erste.angelegt.contains(&Sonderdatei::Geheimnisse),
        "{erste:?}"
    );
    assert_eq!(fs::metadata(&neu).expect("secrets.txt fehlt").len(), 0);
    assert!(erste.meldungen().is_empty(), "{erste:?}");

    let chiffrat = ein_chiffrat();
    fs::write(&neu, &chiffrat).expect("secrets.txt");
    let zweite = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(zweite.umbenennung, None);
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
    fs::write(lage.heimpfad().join(Sonderdatei::Aufgaben.dateiname()), b"").expect("Eintragsdatei");
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
            bereitstellung.umbenennung,
            Some(Umbenennung::Gescheitert(_))
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

/// C3.5 und C3.6 des Spec
/// `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`: die
/// Bereitstellung legt `notes.txt` und `appointments.md` nicht an und fasst
/// vorhandene nicht an. In einem bestehenden Ordner bleiben beide nach zwei
/// Aufrufen Byte fuer Byte, mit ihrer Groesse und ihrer Aenderungszeit, und
/// kein Satz der Statuszeile nennt sie.
#[test]
fn vorhandene_notes_txt_und_appointments_md_bleiben_byte_fuer_byte() {
    let lage = Lage::neu("heim-bestand-vorhanden", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let bestand: [(&str, &[u8]); 2] = [
        (
            "notes.txt",
            b"# Vorspann\n## Eigenes\nnicht anfassen\r\n\xef\xbb\xbf ohne Schluss",
        ),
        (
            "appointments.md",
            b"# Termine\n## 261002 09:30\nZahnarzt ohne Schluss",
        ),
    ];
    for (name, inhalt) in bestand {
        fs::write(lage.heimpfad().join(name), inhalt).expect("Bestandsdatei");
    }
    let stand = |name: &str| {
        let pfad = lage.heimpfad().join(name);
        let angaben = fs::metadata(&pfad).unwrap_or_else(|_| panic!("{name} fehlt"));
        (
            fs::read(&pfad).unwrap_or_else(|_| panic!("{name} ist nicht lesbar")),
            angaben.len(),
            angaben.modified().expect("Aenderungszeit"),
        )
    };
    let davor = bestand.map(|(name, _)| stand(name));

    for durchgang in ["erster", "zweiter"] {
        let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");
        for ((name, inhalt), davor) in bestand.iter().zip(&davor) {
            assert_eq!(&stand(name), davor, "{name}, {durchgang} Aufruf");
            assert_eq!(stand(name).0, *inhalt, "{name}, {durchgang} Aufruf");
            assert!(
                bereitstellung
                    .angelegt
                    .iter()
                    .chain(bereitstellung.nicht_angelegt.iter().map(|(sorte, _)| sorte))
                    .all(|sorte| sorte.dateiname() != *name),
                "{bereitstellung:?}"
            );
        }
        assert!(
            bereitstellung
                .meldungen()
                .iter()
                .all(|satz| !satz.contains("appointments") && !satz.contains("notes")),
            "{:?}",
            bereitstellung.meldungen()
        );
    }
    assert_eq!(
        namen_in(&lage.heimpfad()),
        vec![
            "appointments.md",
            "minutes",
            "notes.txt",
            "secrets.txt",
            "tasks.txt"
        ]
    );
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
            vec!["minutes", "secrets.txt", "tasks.txt"]
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

/// Ein Verweis auf einen Ordner: die Dateien entstehen im Ziel, und der
/// Verweis bleibt ein Verweis.
#[test]
fn ein_verweis_auf_einen_ordner_laesst_die_dateien_im_ziel_entstehen() {
    let lage = Lage::neu("heim-verweisziel", [Some("Text\n".as_bytes()), None]);
    let ziel = lage.zuhause.join("anderswo");
    fs::create_dir(&ziel).expect("Zielordner");
    std::os::unix::fs::symlink(&ziel, lage.heimpfad()).expect("Verweis");

    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

    assert_eq!(namen_in(&ziel), vec!["minutes", "secrets.txt", "tasks.txt"]);
    assert!(!bereitstellung.ordner_angelegt);
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
// `minutes/` (Schritt 5 des Plans
// `261111-0852-plan-menue-navigation-und-notizordner-mit-minutes.md`, C4 des
// Spec `261111-0809-spec-menue-navigation-und-notizordner-mit-minutes.md`)
// ---------------------------------------------------------------------------

/// C4.1: F2 legt `minutes/` an, wenn der Notizordner fehlt und wenn er schon
/// steht, auch neben vorhandenen Dateien; der Ordner entsteht leer, das Feld
/// sagt `Ok`, und kein Satz der Statuszeile nennt das Anlegen.
#[test]
fn f2_legt_minutes_an_wenn_es_fehlt() {
    for (zweck, ordner_steht) in [("minutes-heim-fehlt", false), ("minutes-heim-steht", true)] {
        let lage = Lage::neu(zweck, [None, None]);
        let aufgaben = b"- [ ] bleibt\n";
        if ordner_steht {
            fs::create_dir(lage.heimpfad()).expect("Heimordner");
            fs::write(lage.heimpfad().join("tasks.txt"), aufgaben).expect("tasks.txt");
        }

        let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

        let minutes = lage.heimpfad().join("minutes");
        assert_eq!(lage.heim().protokollordner(), minutes, "{zweck}");
        assert!(
            fs::symlink_metadata(&minutes)
                .unwrap_or_else(|_| panic!("{zweck}: minutes/ fehlt"))
                .is_dir(),
            "{zweck}: minutes ist kein Ordner"
        );
        assert!(namen_in(&minutes).is_empty(), "{zweck}");
        assert_eq!(bereitstellung.protokollordner, Ok(()), "{zweck}");
        assert_eq!(bereitstellung.ordner_angelegt, !ordner_steht, "{zweck}");
        assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
        if ordner_steht {
            assert_eq!(
                fs::read(lage.heimpfad().join("tasks.txt")).expect("tasks.txt"),
                aufgaben,
                "{zweck}"
            );
        }
    }
}

/// C4.1, zweiter Satz: ein vorhandenes `minutes/` und sein Inhalt bleiben
/// Byte fuer Byte, samt einem Unterordner darin, auch ueber zwei Aufrufe.
#[test]
fn ein_vorhandenes_minutes_und_sein_inhalt_bleiben_byte_fuer_byte() {
    let lage = Lage::neu("minutes-vorhanden", [None, None]);
    let minutes = lage.heimpfad().join("minutes");
    fs::create_dir_all(minutes.join("alt")).expect("Unterordner");
    let dateien: [(&str, &[u8]); 3] = [
        (
            "261110-0900-besprechung.md",
            b"# Besprechung\r\n\xef\xbb\xbfohne Schlussumbruch",
        ),
        ("ohne-praefix.txt", b""),
        ("alt/260101-0000-neujahr.md", b"lange her\n"),
    ];
    for (name, inhalt) in dateien {
        fs::write(minutes.join(name), inhalt).expect("Protokoll");
    }

    for durchgang in ["erster", "zweiter"] {
        let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");

        assert_eq!(bereitstellung.protokollordner, Ok(()), "{durchgang} Aufruf");
        assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
        assert_eq!(
            namen_in(&minutes),
            vec!["261110-0900-besprechung.md", "alt", "ohne-praefix.txt"],
            "{durchgang} Aufruf"
        );
        assert_eq!(
            namen_in(&minutes.join("alt")),
            vec!["260101-0000-neujahr.md"]
        );
        for (name, inhalt) in dateien {
            assert_eq!(
                fs::read(minutes.join(name)).expect("das Protokoll ist weg"),
                inhalt,
                "{durchgang} Aufruf: {name}"
            );
        }
    }
}

/// C4.1 und E8: eine gewoehnliche Datei namens `minutes` ergibt `KeinOrdner`
/// im Feld und **kein** Hindernis der Funktion. Die Datei bleibt Byte fuer
/// Byte, die Eintragsdateien entstehen wie sonst, und die Statuszeile nennt
/// den Grund mit dem Pfad des Unterordners.
#[test]
fn eine_datei_namens_minutes_steht_im_feld_und_haelt_f2_nicht_auf() {
    let lage = Lage::neu("minutes-datei", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let minutes = lage.heimpfad().join("minutes");
    fs::write(&minutes, b"ich bin eine Datei").expect("Datei");

    let bereitstellung = lage.bereitstellen().expect("F2 gelingt trotzdem");

    assert_eq!(bereitstellung.protokollordner, Err(Hindernis::KeinOrdner));
    assert_eq!(
        fs::read(&minutes).expect("die Datei ist weg"),
        b"ich bin eine Datei"
    );
    assert_eq!(
        bereitstellung.angelegt,
        vec![Sonderdatei::Aufgaben, Sonderdatei::Geheimnisse]
    );
    assert_eq!(
        namen_in(&lage.heimpfad()),
        vec!["minutes", "secrets.txt", "tasks.txt"]
    );
    assert_eq!(
        bereitstellung.meldungen(),
        vec![
            "~/krkhome/minutes ist kein Ordner; KRK legt dort nichts an und öffnet keinen Tab"
                .to_owned()
        ]
    );
}

/// Ein Verweis ins Leere an der Stelle von `minutes`: `Unerreichbar` im Feld,
/// F2 gelingt, und das fehlende Ziel entsteht nicht. Ein Verweis auf einen
/// Ordner gilt als Ordner und bleibt ein Verweis.
#[test]
fn ein_verweis_an_der_stelle_von_minutes_folgt_der_regel_des_notizordners() {
    let lage = Lage::neu("minutes-verweis", [None, None]);
    fs::create_dir(lage.heimpfad()).expect("Heimordner");
    let minutes = lage.heimpfad().join("minutes");
    let ziel = lage.zuhause.join("gibt-es-nicht");
    std::os::unix::fs::symlink(&ziel, &minutes).expect("Verweis");

    let bereitstellung = lage.bereitstellen().expect("F2 gelingt trotzdem");

    assert!(
        matches!(
            bereitstellung.protokollordner,
            Err(Hindernis::Unerreichbar(_))
        ),
        "{bereitstellung:?}"
    );
    assert!(!ziel.exists(), "das Ziel ist entstanden");
    let meldungen = bereitstellung.meldungen();
    assert_eq!(meldungen.len(), 1, "{meldungen:?}");
    assert!(
        meldungen[0].starts_with("~/krkhome/minutes ist nicht erreichbar: "),
        "{meldungen:?}"
    );

    fs::create_dir(&ziel).expect("Zielordner");
    let bereitstellung = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(bereitstellung.protokollordner, Ok(()));
    assert!(bereitstellung.meldungen().is_empty(), "{bereitstellung:?}");
    assert!(
        fs::symlink_metadata(&minutes)
            .expect("der Verweis ist weg")
            .file_type()
            .is_symlink(),
        "der Verweis ist ersetzt"
    );
}

/// Der Name des Unterordners und sein Pfad in der geschriebenen Form, auch
/// wenn der Notizordner ein Verweis ist.
#[test]
fn der_protokollordner_liegt_an_der_geschriebenen_form() {
    assert_eq!(PROTOKOLLORDNER, "minutes");
    let ordner = Pruefordner::neu("minutes-pfad");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("anderswo");
    fs::create_dir(&ziel).expect("Zielordner");
    std::os::unix::fs::symlink(&ziel, zuhause.join(ORDNERNAME)).expect("Verweis");

    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);

    assert_eq!(heim.aufgeloest(), Some(ziel.as_path()));
    assert_eq!(
        heim.protokollordner(),
        zuhause.join(ORDNERNAME).join("minutes")
    );
}

/// Eine Ortszeit der Tafel; die Sekunde zaehlt im Namen nicht.
fn zeit(jahr: i32, monat: u8, tag: u8, stunde: u8, minute: u8) -> Ortszeit {
    Ortszeit {
        jahr,
        monat,
        tag,
        stunde,
        minute,
        sekunde: 59,
    }
}

/// C4.2, C4.4 und C4.5: `JJMMTT-HHMM-` davor und `.md` dahinter, ausser der
/// Name endet ohne Ruecksicht auf Gross- und Kleinschreibung schon darauf;
/// jedes der fuenf Zeitfelder steht zweistellig.
#[test]
fn der_protokollname_traegt_datum_uhrzeit_und_md() {
    let beispiel = zeit(2026, 11, 11, 14, 32);
    let tafel = [
        // C4.2: das Beispiel der Spezifikation.
        ("besprechung", beispiel, "261111-1432-besprechung.md"),
        // C4.4: die Endung wird nicht verdoppelt, in keiner Schreibweise, und
        // die Schreibweise des Nutzers bleibt stehen.
        ("besprechung.md", beispiel, "261111-1432-besprechung.md"),
        ("Besprechung.MD", beispiel, "261111-1432-Besprechung.MD"),
        ("notiz.Md", beispiel, "261111-1432-notiz.Md"),
        // C4.5: eine andere Endung bleibt und bekommt `.md` dahinter.
        ("liste.txt", beispiel, "261111-1432-liste.txt.md"),
        // `.md` mitten im Namen oder ohne Punkt ist keine Endung.
        ("plan.md.txt", beispiel, "261111-1432-plan.md.txt.md"),
        ("md", beispiel, "261111-1432-md.md"),
        ("amd", beispiel, "261111-1432-amd.md"),
        // Ein Name mit Umlaut und Leerzeichen bleibt, wie er getippt ist.
        ("Jour fixe März", beispiel, "261111-1432-Jour fixe März.md"),
        ("ä", beispiel, "261111-1432-ä.md"),
        // Einstellige Werte fuer Monat, Tag, Stunde und Minute, und fuer das
        // Jahr im Jahrhundert.
        ("kurz", zeit(2027, 1, 2, 3, 4), "270102-0304-kurz.md"),
        ("kurz", zeit(2105, 9, 9, 0, 0), "050909-0000-kurz.md"),
        ("spaet", zeit(2026, 12, 31, 23, 59), "261231-2359-spaet.md"),
    ];
    for (name, zeit, erwartet) in tafel {
        assert_eq!(protokollname(name, zeit), erwartet, "{name} um {zeit:?}");
    }
}

/// C4.6, Kernhaelfte: `ist_protokollordner` erkennt `minutes` unmittelbar im
/// Notizordner in der geschriebenen und in der aufgeloesten Form und sonst
/// nichts.
#[test]
fn ist_protokollordner_erkennt_minutes_allein_unmittelbar_im_notizordner() {
    let ordner = Pruefordner::neu("minutes-erkennung");
    let zuhause = kanonisch(&ordner);
    let ziel = zuhause.join("anderswo");
    fs::create_dir(&ziel).expect("Zielordner");
    std::os::unix::fs::symlink(&ziel, zuhause.join(ORDNERNAME)).expect("Verweis");
    let heim = Heimordner::im_benutzerverzeichnis(&zuhause);
    let geschrieben = zuhause.join(ORDNERNAME);
    assert_eq!(heim.aufgeloest(), Some(ziel.as_path()));

    let tafel = [
        ("geschriebene Form", geschrieben.join("minutes"), true),
        ("aufgeloeste Form", ziel.join("minutes"), true),
        (
            "mit Schlussstrich",
            PathBuf::from(format!("{}/minutes/", geschrieben.display())),
            true,
        ),
        (
            "Unterordner von minutes",
            geschrieben.join("minutes/alt"),
            false,
        ),
        (
            "Unterordner in der aufgeloesten Form",
            ziel.join("minutes/alt"),
            false,
        ),
        ("minutes ausserhalb", zuhause.join("minutes"), false),
        (
            "minutes in einem anderen Ordner",
            zuhause.join("projekt/minutes"),
            false,
        ),
        (
            "minutes in einem Unterordner des Notizordners",
            geschrieben.join("alt/minutes"),
            false,
        ),
        ("der Notizordner selbst", geschrieben.clone(), false),
        ("der Notizordner aufgeloest", ziel.clone(), false),
        (
            "andere Schreibweise des Namens",
            geschrieben.join("Minutes"),
            false,
        ),
        ("ein laengerer Name", geschrieben.join("minutes-alt"), false),
        ("die Wurzel", PathBuf::from("/"), false),
        ("ein nackter Name", PathBuf::from("minutes"), false),
    ];
    for (fall, pfad, erwartet) in tafel {
        assert_eq!(
            heim.ist_protokollordner(&pfad),
            erwartet,
            "{fall}: {}",
            pfad.display()
        );
    }

    // Ohne aufgeloeste Form gilt allein die geschriebene.
    let ohne = Heimordner::ohne_aufloesung(geschrieben.clone(), Some(&zuhause));
    assert!(ohne.ist_protokollordner(&geschrieben.join("minutes")));
    assert!(!ohne.ist_protokollordner(&ziel.join("minutes")));
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
        heim.sonderdatei(&ziel.join("tasks.txt")),
        Some(Sonderdatei::Aufgaben)
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

/// H2.8 in der Fassung seit dem 261111: F2 legt an einem anderen Ort als dem
/// Vorgabeort dieselben zwei Dateien leer an wie dort, und die Zettel im
/// Ablageordner sind an beiden Orten Byte fuer Byte unveraendert.
#[test]
fn ein_anderer_ort_bekommt_dieselben_zwei_leeren_dateien_wie_der_vorgabeort() {
    let lage = Lage::neu(
        "heim-ort-zettel",
        [
            Some("erster Zettel\n".as_bytes()),
            Some("zweiter Zettel\n".as_bytes()),
        ],
    );
    let ort = lage.zuhause.join("woanders");
    let heim = Heimordner::am_ort(ort.clone(), Some(&lage.zuhause));

    let anderswo = bereitstellen(&heim).expect("kein Hindernis erwartet");

    assert!(anderswo.ordner_angelegt);
    assert_eq!(anderswo.angelegt, Sonderdatei::ALLE.to_vec());
    assert_eq!(namen_in(&ort), vec!["minutes", "secrets.txt", "tasks.txt"]);
    assert!(anderswo.meldungen().is_empty(), "{anderswo:?}");
    lage.zettel_unveraendert();

    let vorgabe = lage.bereitstellen().expect("kein Hindernis erwartet");
    assert_eq!(vorgabe.angelegt, anderswo.angelegt);
    assert_eq!(
        namen_in(&lage.heimpfad()),
        vec!["minutes", "secrets.txt", "tasks.txt"]
    );
    assert!(vorgabe.meldungen().is_empty(), "{vorgabe:?}");
    lage.zettel_unveraendert();
}

/// H2.7: fehlt der Ordner ueber dem Ort, meldet F2 das mit dem Ort und legt
/// nichts an, auch nicht die fehlende Stufe darueber.
#[test]
fn ein_fehlender_oberer_ordner_legt_nichts_an() {
    let lage = Lage::neu("heim-ort-oben", [Some("Text\n".as_bytes()), None]);
    let oben = lage.zuhause.join("nicht-eingehaengt");
    let heim = Heimordner::am_ort(oben.join("notizen"), Some(&lage.zuhause));

    let ausgang = bereitstellen(&heim);

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
    let bereitstellung = bereitstellen(&heim).expect("kein Hindernis erwartet");
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
fn gehaltene_notizdatei_erkennt_die_eintragsdateien_am_gefragten_ort() {
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
        gehaltene_notizdatei(Some(&geltend.geschrieben().join("tasks.txt")), false, None),
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

// ---------------------------------------------------------------------------
// Der gemerkte Schluessel (Schritt 1 des Plans
// `261016-0306_*_plan-secrets-mit-sechs-feldern-schluesselmerker-und-verbindungen.md`)
// ---------------------------------------------------------------------------

/// Eine Wachzeit in Sekunden; der Merker liest keine Uhr, die Proben reichen
/// sie hinein.
fn sekunden(n: u64) -> std::time::Duration {
    std::time::Duration::from_secs(n)
}

/// Die Frist endet genau bei zehn Minuten: nach 599 Sekunden gilt der
/// Schluessel, nach 600 nicht mehr, und danach ist er fort, auch fuer eine
/// Frage mit frueherer Uhr.
#[test]
fn der_gemerkte_schluessel_gilt_bis_vor_der_zehnten_minute() {
    assert_eq!(GUELTIGKEIT, sekunden(600));
    let schluessel = kleiner_schluessel("0417");
    let mut merker = Schluesselmerker::neu();
    assert_eq!(merker.gueltiger(sekunden(1000)), None);

    merker.merken(schluessel.clone(), sekunden(1000));
    assert_eq!(merker.gueltiger(sekunden(1000)), Some(&schluessel));
    assert_eq!(merker.restdauer(sekunden(1000)), Some(sekunden(600)));
    assert_eq!(merker.gueltiger(sekunden(1599)), Some(&schluessel));
    assert_eq!(merker.restdauer(sekunden(1599)), Some(sekunden(1)));
    assert_eq!(merker.restdauer(sekunden(1600)), None);
    assert_eq!(merker.gueltiger(sekunden(1600)), None);
    assert_eq!(
        merker.gueltiger(sekunden(1001)),
        None,
        "der abgelaufene Schluessel ist getilgt und kommt nicht wieder"
    );
}

/// Die Frage verlaengert die Frist nicht: wer bis Sekunde 599 immer wieder
/// fragt, hat bei Sekunde 600 trotzdem keinen Schluessel mehr.
#[test]
fn die_frage_verlaengert_die_frist_nicht() {
    let mut merker = Schluesselmerker::neu();
    merker.merken(kleiner_schluessel("0417"), sekunden(0));
    for jetzt in (0..600).step_by(50) {
        assert!(merker.gueltiger(sekunden(jetzt)).is_some(), "bei {jetzt} s");
    }
    assert!(merker.gueltiger(sekunden(599)).is_some());
    assert!(merker.gueltiger(sekunden(600)).is_none());
}

/// Ersetzen nimmt den neuen Schluessel und setzt die Frist ab dem neuen
/// Merken; Verwerfen leert den Merker.
#[test]
fn ersetzen_setzt_die_frist_neu_und_verwerfen_leert() {
    let alt = kleiner_schluessel("1111");
    let neu = kleiner_schluessel("2222");
    assert_ne!(alt, neu);
    let mut merker = Schluesselmerker::neu();
    merker.merken(alt, sekunden(0));
    merker.merken(neu.clone(), sekunden(500));
    assert_eq!(merker.gueltiger(sekunden(1099)), Some(&neu));
    assert_eq!(merker.gueltiger(sekunden(1100)), None);

    merker.merken(neu, sekunden(2000));
    merker.verwerfen();
    assert_eq!(merker.gueltiger(sekunden(2000)), None);
    assert_eq!(merker.restdauer(sekunden(2000)), None);
    merker.verwerfen();
}

/// Eine Wachzeit vor dem Merken laesst den Schluessel nicht gelten, sondern
/// fallen: die Null, mit der `wachzeit` einen Fehler beantwortet, verlaengert
/// nichts.
#[test]
fn eine_uhr_vor_dem_merken_zaehlt_als_abgelaufen() {
    let mut merker = Schluesselmerker::neu();
    merker.merken(kleiner_schluessel("0417"), sekunden(100));
    assert_eq!(merker.restdauer(sekunden(0)), None);
    assert_eq!(merker.gueltiger(sekunden(0)), None);
    assert_eq!(merker.gueltiger(sekunden(100)), None);
}

/// `Debug` des Merkers zeigt weder Schluesselbytes noch Salz.
#[test]
fn debug_des_merkers_zeigt_keinen_schluessel() {
    let mut merker = Schluesselmerker::neu();
    merker.merken(kleiner_schluessel("0417"), sekunden(7));
    let text = format!("{merker:?}");
    assert!(
        !text.contains("salz") && !text.contains("schluessel:"),
        "{text}"
    );
    assert!(text.contains("gemerkt_seit"), "{text}");
}

/// Die Wachzeit laeuft vorwaerts und ist nicht null.
#[test]
fn die_wachzeit_laeuft() {
    let erste = krk_core::verzeichnis::sys::wachzeit();
    let zweite = krk_core::verzeichnis::sys::wachzeit();
    assert!(erste > std::time::Duration::ZERO);
    assert!(zweite >= erste);
}

/// `CLOCK_MONOTONIC` wird an genau einer Stelle gelesen, in
/// `verzeichnis::sys::wachzeit`, und der Merker liest keine Uhr: weder
/// `Instant` noch `SystemTime` noch die Wachzeit.
#[test]
fn die_wachzeit_wird_an_einer_stelle_gelesen() {
    let nadel = concat!("clock_gettime", "_nsec_np(");
    let mut fundstellen = Vec::new();
    for (name, inhalt) in gemeinsam::quelldateien() {
        let treffer = inhalt
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//") && zeile.contains(nadel))
            .count();
        if treffer > 0 {
            fundstellen.push((name, treffer));
        }
    }
    assert_eq!(
        fundstellen,
        [("krk-core/src/verzeichnis/sys.rs".to_owned(), 2)],
        "Bindung und ein Aufruf, beide in sys.rs"
    );

    let (_, sys) = gemeinsam::quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/verzeichnis/sys.rs")
        .expect("sys.rs steht im Baum");
    let rumpf = sys
        .split("pub fn wachzeit() -> Duration {")
        .nth(1)
        .and_then(|rest| rest.split("\n}\n").next())
        .expect("wachzeit steht in sys.rs");
    assert!(rumpf.contains(nadel) && rumpf.contains("CLOCK_MONOTONIC"));

    let (_, merker) = gemeinsam::quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/heimordner/schluesselmerker.rs")
        .expect("schluesselmerker.rs steht im Baum");
    for uhr in ["Instant", "SystemTime", "wachzeit("] {
        assert!(
            !merker
                .lines()
                .filter(|zeile| !zeile.trim_start().starts_with("//"))
                .any(|zeile| zeile.contains(uhr)),
            "der Merker liest selbst eine Uhr: {uhr}"
        );
    }
}

// ---------------------------------------------------------------------------
// Das Format der Geheimnisse (Schritt 2 desselben Plans)
// ---------------------------------------------------------------------------

/// Ein Stand im frueheren Format: zwei Spalten, Thema und Text.
const GEHEIMNISSE_ALT: &str = "# Vorspann\n\n## Bank\nKontonummer 4711\nPIN beim Schalter\n\n## Mail\nkai@example.org\n\n## Leer\n";

/// (1) Ein Stand im frueheren Format liest sich ohne Verlust: jedes Thema ist
/// ein Name, jeder Text sind die Notizen, die Felder sind leer, und der
/// Stand schreibt sich Byte fuer Byte zurueck.
#[test]
fn das_fruehere_format_liest_sich_ohne_verlust() {
    let geheimnisse = Geheimnisse::lesen(GEHEIMNISSE_ALT);
    assert_eq!(geheimnisse.anzahl(), 3);
    let notizen = Notizen::lesen(GEHEIMNISSE_ALT);
    for stelle in 0..3 {
        let geheimnis = geheimnisse.geheimnis(stelle).expect("Eintrag");
        let notiz = notizen.notiz(stelle).expect("Notiz");
        assert_eq!(geheimnis.name, notiz.thema);
        assert_eq!(geheimnis.notizen, notiz.text);
        for feld in [Feld::Protokoll, Feld::Adresse, Feld::Konto, Feld::Passwort] {
            assert_eq!(geheimnis.wert(feld), "", "{feld:?} an Stelle {stelle}");
        }
    }
    assert_eq!(
        geheimnisse.geheimnis(0).expect("Bank").notizen,
        "Kontonummer 4711\nPIN beim Schalter"
    );
    assert_eq!(geheimnisse.schreiben(), GEHEIMNISSE_ALT);
}

/// (2) Ohne Handlung ist die Rundreise byte-treu, auch mit CRLF und ohne
/// Schlussumbruch, und die Werte einer CRLF-Feldzeile tragen kein `\r`.
#[test]
fn die_rundreise_ohne_handlung_ist_byte_treu() {
    for stand in [
        GEHEIMNISSE_ALT,
        "## NAS\r\nprotocol: smb\r\naddress: nas.local/daten\r\nNotiz\r\n",
        "## NAS\nprotocol: smb\naddress: nas.local\naccount:\npassword: x",
        "## Ohne Schluss\nText",
        "",
        "Nur Vorspann\n",
    ] {
        assert_eq!(Geheimnisse::lesen(stand).schreiben(), stand);
    }
    let crlf =
        Geheimnisse::lesen("## NAS\r\nprotocol: smb\r\naddress: nas.local/daten\r\nNotiz\r\n");
    let nas = crlf.geheimnis(0).expect("Eintrag");
    assert_eq!(nas.protokoll, "smb");
    assert_eq!(nas.adresse, "nas.local/daten");
    assert_eq!(nas.notizen, "Notiz\r");
}

/// Die Leseregel: Schluessel ohne Ruecksicht auf Gross- und Kleinschreibung,
/// nach dem Doppelpunkt nichts oder ein Leerzeichen, die erste andere Zeile
/// beendet die Folge.
#[test]
fn die_feldzeilen_folgen_der_leseregel() {
    let stand = "## A\nProtocol: SMB\nADDRESS:\nkonto: x\naccount: kai\n## B\naddress:nah\n## C\npassword: geheim\n\nprotocol: afp\n";
    let geheimnisse = Geheimnisse::lesen(stand);
    let a = geheimnisse.geheimnis(0).expect("A");
    assert_eq!((a.protokoll, a.adresse, a.konto), ("SMB", "", ""));
    assert_eq!(
        a.notizen, "konto: x\naccount: kai",
        "eine fremde Zeile beendet die Folge"
    );
    let b = geheimnisse.geheimnis(1).expect("B");
    assert_eq!(
        b.adresse, "",
        "ohne Leerzeichen nach dem Doppelpunkt kein Feld"
    );
    assert_eq!(b.notizen, "address:nah");
    let c = geheimnisse.geheimnis(2).expect("C");
    assert_eq!(c.passwort, "geheim");
    assert_eq!(c.protokoll, "", "eine Leerzeile beendet die Folge");
    assert_eq!(c.notizen, "\nprotocol: afp");
}

/// (3) `aendern` an einem alten Eintrag schreibt ihn in der Grundform mit
/// allen vier Feldzeilen, behaelt Notizen und Trenner und laesst jeden
/// anderen Block Byte fuer Byte.
#[test]
fn aendern_schreibt_einen_alten_eintrag_in_der_grundform() {
    let neu = geheimnisse::aendern(GEHEIMNISSE_ALT, 0, Feld::Konto, "kai")
        .expect("zulaessig")
        .expect("aendert");
    assert_eq!(
        neu.text,
        "# Vorspann\n\n## Bank\nprotocol:\naddress:\naccount: kai\npassword:\nKontonummer 4711\nPIN beim Schalter\n\n## Mail\nkai@example.org\n\n## Leer\n"
    );
    assert_eq!(neu.auswahl, Some(0));
    let bank = Geheimnisse::lesen(&neu.text)
        .geheimnis(0)
        .expect("Bank")
        .clone();
    assert_eq!(bank.konto, "kai");
    assert_eq!(bank.notizen, "Kontonummer 4711\nPIN beim Schalter");

    // Jede Spalte aendert genau ihr Feld.
    for (feld, wert) in [
        (Feld::Name, "Bank neu"),
        (Feld::Protokoll, "smb"),
        (Feld::Adresse, "nas.local/daten"),
        (Feld::Konto, "kai"),
        (Feld::Passwort, "Xq7"),
        (Feld::Notizen, "neu\nzweizeilig"),
    ] {
        let neu = geheimnisse::aendern(GEHEIMNISSE_ALT, 1, feld, wert)
            .expect("zulaessig")
            .expect("aendert");
        let vorher = Geheimnisse::lesen(GEHEIMNISSE_ALT);
        let nachher = Geheimnisse::lesen(&neu.text);
        for anderes in Feld::ALLE {
            let erwartet = if anderes == feld {
                wert.to_owned()
            } else {
                vorher.geheimnis(1).expect("Mail").wert(anderes).to_owned()
            };
            assert_eq!(
                nachher.geheimnis(1).expect("Mail").wert(anderes),
                erwartet,
                "{feld:?} aendert {anderes:?}"
            );
        }
        assert_eq!(nachher.geheimnis(0), vorher.geheimnis(0));
        assert_eq!(nachher.geheimnis(2), vorher.geheimnis(2));
        assert!(
            neu.text.starts_with(
                "# Vorspann\n\n## Bank\nKontonummer 4711\nPIN beim Schalter\n\n## Mail\n"
            ) || feld == Feld::Name
        );
        assert!(neu.text.ends_with("\n\n## Leer\n"));
    }

    // Ohne Aenderung keine Handlung, auch fuer ein leeres Feld eines alten
    // Eintrags; den gibt es nicht, ebenso keine.
    assert_eq!(
        geheimnisse::aendern(GEHEIMNISSE_ALT, 0, Feld::Konto, ""),
        Ok(None)
    );
    assert_eq!(
        geheimnisse::aendern(GEHEIMNISSE_ALT, 0, Feld::Name, "Bank"),
        Ok(None)
    );
    assert_eq!(
        geheimnisse::aendern(GEHEIMNISSE_ALT, 9, Feld::Konto, "x"),
        Ok(None)
    );
}

/// Das Dateiende bleibt: ein Stand ohne Schlussumbruch endet nach dem Aendern
/// ebenso ohne.
#[test]
fn aendern_wahrt_das_dateiende() {
    let neu = geheimnisse::aendern("## A\nText", 0, Feld::Protokoll, "ftp")
        .expect("zulaessig")
        .expect("aendert");
    assert_eq!(
        neu.text,
        "## A\nprotocol: ftp\naddress:\naccount:\npassword:\nText"
    );
    let neu = geheimnisse::aendern("## A", 0, Feld::Adresse, "a.b")
        .expect("zulaessig")
        .expect("aendert");
    assert_eq!(
        neu.text,
        "## A\nprotocol:\naddress: a.b\naccount:\npassword:"
    );
}

/// (4) Eine Notizzeile `password: x` nach einer Grundform bleibt Notiz, auch
/// wenn sie ueber das Aendern der Notizen hineinkommt.
#[test]
fn eine_notizzeile_wie_ein_feld_bleibt_nach_der_grundform_notiz() {
    let stand = "## A\nprotocol: smb\naddress: x\naccount:\npassword: alt\npassword: neu\n";
    let a = Geheimnisse::lesen(stand).geheimnis(0).expect("A").clone();
    assert_eq!(a.passwort, "alt");
    assert_eq!(a.notizen, "password: neu");

    let neu = geheimnisse::aendern("## B\n", 0, Feld::Notizen, "password: x")
        .expect("zulaessig")
        .expect("aendert");
    let b = Geheimnisse::lesen(&neu.text)
        .geheimnis(0)
        .expect("B")
        .clone();
    assert_eq!(b.passwort, "");
    assert_eq!(b.notizen, "password: x");
}

/// (5) Ein doppelter Schluessel beendet die Feldfolge.
#[test]
fn ein_doppelter_schluessel_beendet_die_feldfolge() {
    let stand = "## A\nprotocol: smb\nprotocol: afp\naddress: x\n";
    let a = Geheimnisse::lesen(stand).geheimnis(0).expect("A").clone();
    assert_eq!(a.protokoll, "smb");
    assert_eq!(a.adresse, "");
    assert_eq!(a.notizen, "protocol: afp\naddress: x");
}

/// Abweisungen: ein Zeilenumbruch in einem einzeiligen Feld und eine Zeile
/// `## ` in den Notizen; der Stand bleibt.
#[test]
fn aendern_und_hinzufuegen_weisen_ab() {
    for feld in [
        Feld::Name,
        Feld::Protokoll,
        Feld::Adresse,
        Feld::Konto,
        Feld::Passwort,
    ] {
        for wert in ["a\nb", "a\rb"] {
            assert_eq!(
                geheimnisse::aendern(GEHEIMNISSE_ALT, 0, feld, wert),
                Err(Abweisung::UmbruchImGeheimnisfeld),
                "{feld:?}"
            );
        }
    }
    assert_eq!(
        geheimnisse::aendern(GEHEIMNISSE_ALT, 0, Feld::Notizen, "a\n## b"),
        Err(Abweisung::KopfzeileInGeheimnisnotizen)
    );
    let mut entwurf = Geheimnisentwurf {
        name: "X".into(),
        passwort: Passwort::neu("a\nb".into()),
        ..Geheimnisentwurf::default()
    };
    assert_eq!(
        geheimnisse::hinzufuegen("", &entwurf),
        Err(Abweisung::UmbruchImGeheimnisfeld)
    );
    entwurf.passwort = Passwort::neu("ab".into());
    entwurf.notizen = "## zweiter".into();
    assert_eq!(
        geheimnisse::hinzufuegen("", &entwurf),
        Err(Abweisung::KopfzeileInGeheimnisnotizen)
    );
    assert!(!Abweisung::UmbruchImGeheimnisfeld.meldung().is_empty());
    assert!(!Abweisung::KopfzeileInGeheimnisnotizen.meldung().is_empty());
}

/// `hinzufuegen` haengt einen Eintrag in der Grundform an, laesst jeden
/// alten Byte fuer Byte und waehlt den neuen; ein leerer Entwurf ergibt einen
/// leeren Eintrag mit vier leeren Feldzeilen.
#[test]
fn hinzufuegen_haengt_die_grundform_an() {
    let entwurf = Geheimnisentwurf {
        name: "NAS Buero".into(),
        protokoll: "smb".into(),
        adresse: "nas.local/daten".into(),
        konto: "kai".into(),
        passwort: Passwort::neu("Xq7".into()),
        notizen: "Zugang nur im Bueronetz.\n".into(),
    };
    let neu = geheimnisse::hinzufuegen(GEHEIMNISSE_ALT, &entwurf).expect("zulaessig");
    assert_eq!(
        neu.text,
        format!(
            "{GEHEIMNISSE_ALT}## NAS Buero\nprotocol: smb\naddress: nas.local/daten\naccount: kai\npassword: Xq7\nZugang nur im Bueronetz.\n"
        )
    );
    assert_eq!(neu.auswahl, Some(3));

    let leer =
        geheimnisse::hinzufuegen("## A\nText", &Geheimnisentwurf::default()).expect("zulaessig");
    assert_eq!(
        leer.text,
        "## A\nText\n## \nprotocol:\naddress:\naccount:\npassword:"
    );
    assert_eq!(
        geheimnisse::hinzufuegen("", &Geheimnisentwurf::default())
            .expect("zulaessig")
            .text,
        "## \nprotocol:\naddress:\naccount:\npassword:\n"
    );
}

/// Loeschen und Verschieben sind die der Notizen.
#[test]
fn loeschen_und_verschieben_rechnen_ueber_die_notizen() {
    assert_eq!(
        geheimnisse::loeschen(GEHEIMNISSE_ALT, 1),
        notizen::loeschen(GEHEIMNISSE_ALT, 1)
    );
    assert_eq!(
        geheimnisse::verschieben(GEHEIMNISSE_ALT, 1, Richtung::Hoch),
        notizen::verschieben(GEHEIMNISSE_ALT, 1, Richtung::Hoch)
    );
}

/// (6) `finden` mit keinem, einem und zwei Treffern, getrimmt verglichen;
/// `name_frei` fragt dieselbe Regel.
#[test]
fn finden_trifft_keinen_einen_oder_mehrdeutig() {
    let stand = "## NAS \nprotocol: smb\naddress: nas.local\naccount: kai\npassword: Xq7\n## Doppelt\n## Doppelt\nprotocol: afp\n";
    assert_eq!(geheimnisse::finden(stand, "Fehlt"), Err(Suchfehler::Keiner));
    assert_eq!(
        geheimnisse::finden(stand, " Doppelt"),
        Err(Suchfehler::Mehrdeutig(2))
    );
    let nas = geheimnisse::finden(stand, "NAS").expect("ein Treffer");
    assert_eq!(nas.name, "NAS ");
    assert_eq!(nas.protokoll, "smb");
    assert_eq!(nas.adresse, "nas.local");
    assert_eq!(nas.konto, "kai");
    assert_eq!(nas.passwort, Passwort::neu("Xq7".into()));
    assert!(!geheimnisse::name_frei(stand, " NAS"));
    assert!(!geheimnisse::name_frei(stand, "Doppelt"));
    assert!(geheimnisse::name_frei(stand, "Neu"));
    assert_eq!(geheimnisse::finden("", "NAS"), Err(Suchfehler::Keiner));
}

/// `verbindungsname` liefert fuer `cmd+d` in der Tabelle von `secrets.txt`
/// allein den getrimmten Namen, und jeder Fall, in dem ein Klick nicht
/// verbaende, ist ein Hindernis: kein Eintrag, kein Name, kein oder ein
/// unbekanntes Protokoll, ein mehrfach vergebener Name (261017).
///
/// Kein Hindernis und keine Antwort traegt Adresse, Konto, Passwort oder
/// Notizen; die Probe sucht die Werte des Eintrags in jeder Ausgabe.
#[test]
fn verbindungsname_liefert_allein_den_namen_oder_ein_hindernis() {
    use geheimnisse::Markenhindernis;
    let stand = "Vorspann\n## NAS \nprotocol: smb\naddress: nas.local\naccount: kai\npassword: Xq7\nGeheime Notiz\n\
                 ##   \nprotocol: smb\n\
                 ## Ohne\nprotocol:\naddress: a.b\n\
                 ## Fremd\nprotocol: gopher\npassword: Pw2\n\
                 ## Doppelt\nprotocol: sftp\n\
                 ## Doppelt \nprotocol: afp\n\
                 ## Alt\nnur Notizen\n";
    assert_eq!(geheimnisse::verbindungsname(stand, 0), Ok("NAS".to_owned()));
    assert_eq!(
        geheimnisse::verbindungsname(stand, 1),
        Err(Markenhindernis::NameLeer)
    );
    assert_eq!(
        geheimnisse::verbindungsname(stand, 2),
        Err(Markenhindernis::OhneProtokoll {
            name: "Ohne".to_owned()
        })
    );
    assert_eq!(
        geheimnisse::verbindungsname(stand, 3),
        Err(Markenhindernis::UnbekanntesProtokoll {
            name: "Fremd".to_owned(),
            protokoll: "gopher".to_owned()
        })
    );
    for stelle in [4, 5] {
        assert_eq!(
            geheimnisse::verbindungsname(stand, stelle),
            Err(Markenhindernis::Mehrdeutig {
                name: "Doppelt".to_owned(),
                zahl: 2
            })
        );
    }
    assert_eq!(
        geheimnisse::verbindungsname(stand, 6),
        Err(Markenhindernis::OhneProtokoll {
            name: "Alt".to_owned()
        }),
        "ein Eintrag des frueheren Formats hat kein Protokoll"
    );
    assert_eq!(
        geheimnisse::verbindungsname(stand, 7),
        Err(Markenhindernis::KeinEintrag)
    );
    assert_eq!(
        geheimnisse::verbindungsname("", 0),
        Err(Markenhindernis::KeinEintrag)
    );

    let ausgaben: Vec<String> = (0..8)
        .map(|stelle| match geheimnisse::verbindungsname(stand, stelle) {
            Ok(name) => name,
            Err(hindernis) => format!("{hindernis:?} {}", hindernis.meldung()),
        })
        .collect();
    for ausgabe in &ausgaben {
        for wert in [
            "nas.local",
            "kai",
            "Xq7",
            "Geheime Notiz",
            "a.b",
            "Pw2",
            "Vorspann",
        ] {
            assert!(!ausgabe.contains(wert), "{ausgabe} traegt {wert}");
        }
    }
}

/// (7) `Debug` eines `Passwort`, eines `Verbindungsdaten`, eines
/// `Geheimnisentwurf` und eines `Geheimnis` enthaelt das Passwort nicht.
#[test]
fn debug_zeigt_kein_passwort() {
    let geheim = "Zz9-unverwechselbar";
    assert_eq!(
        format!("{:?}", Passwort::neu(geheim.into())),
        "Passwort(****)"
    );
    let stand =
        format!("## NAS\nprotocol: smb\naddress: nas.local\naccount: kai\npassword: {geheim}\n");
    let daten = geheimnisse::finden(&stand, "NAS").expect("ein Treffer");
    let text = format!("{daten:?}");
    assert!(!text.contains(geheim), "{text}");
    assert!(text.contains("nas.local"), "{text}");
    let entwurf = Geheimnisentwurf {
        passwort: Passwort::neu(geheim.into()),
        ..Geheimnisentwurf::default()
    };
    assert!(!format!("{entwurf:?}").contains(geheim));
    let geheimnisse = Geheimnisse::lesen(&stand);
    let geheimnis = geheimnisse.geheimnis(0).expect("NAS");
    assert_eq!(geheimnis.passwort, geheim, "die Tabelle zeigt den Klartext");
    assert!(!format!("{geheimnis:?}").contains(geheim));
}

/// `Feld::ALLE` nennt die Spalten in der Folge der Tabelle.
#[test]
fn die_spalten_stehen_in_der_folge_der_tabelle() {
    assert_eq!(
        Feld::ALLE,
        [
            Feld::Name,
            Feld::Protokoll,
            Feld::Adresse,
            Feld::Konto,
            Feld::Passwort,
            Feld::Notizen
        ]
    );
}

// ---------------------------------------------------------------------------
// Protokolle und Verbindungsziel (Schritt 3 desselben Plans)
// ---------------------------------------------------------------------------

/// Jedes Protokoll mit Name, Schema und Weg, und die URL, die daraus mit
/// Adresse und Konto entsteht.
#[test]
fn jedes_protokoll_hat_schema_weg_und_url() {
    let erwartet = [
        (Protokoll::Smb, "smb", "smb", Weg::Einhaengen),
        (Protokoll::Afp, "afp", "afp", Weg::Einhaengen),
        (Protokoll::Nfs, "nfs", "nfs", Weg::Einhaengen),
        (Protokoll::Webdav, "webdav", "http", Weg::Einhaengen),
        (Protokoll::Webdavs, "webdavs", "https", Weg::Einhaengen),
        (Protokoll::Ftp, "ftp", "ftp", Weg::Einhaengen),
        (Protokoll::Http, "http", "http", Weg::Uebergeben),
        (Protokoll::Https, "https", "https", Weg::Uebergeben),
        (Protokoll::Ssh, "ssh", "ssh", Weg::Uebergeben),
        (Protokoll::Sftp, "sftp", "sftp", Weg::Sftp),
        (Protokoll::Vnc, "vnc", "vnc", Weg::Uebergeben),
    ];
    assert_eq!(
        Protokoll::ALLE.to_vec(),
        erwartet.iter().map(|(p, ..)| *p).collect::<Vec<_>>()
    );
    for (protokoll, name, schema, weg) in erwartet {
        assert_eq!(protokoll.name(), name);
        assert_eq!(protokoll.url_schema(), schema);
        assert_eq!(protokoll.weg(), weg);
        assert_eq!(Protokoll::aus_text(name), Some(protokoll));
        assert_eq!(Protokoll::aus_text(&name.to_uppercase()), Some(protokoll));
        assert_eq!(Protokoll::aus_text(&format!(" {name}\t")), Some(protokoll));

        let ziel = verbindungsziel(name, " host.local/teil ", "kai").expect("ein Ziel");
        let mit_konto = matches!(protokoll, Protokoll::Ssh | Protokoll::Sftp | Protokoll::Vnc);
        let url = if mit_konto {
            format!("{schema}://kai@host.local/teil")
        } else {
            format!("{schema}://host.local/teil")
        };
        match weg {
            Weg::Einhaengen => assert_eq!(
                ziel,
                Verbindungsziel::Einhaengen {
                    url,
                    konto: Some("kai".into())
                }
            ),
            Weg::Uebergeben => assert_eq!(ziel, Verbindungsziel::Uebergeben { url }),
            // Seit dem Arbeitspaket der sftp-Ordner ist das Ziel keine URL,
            // sondern ein zerlegtes Fernziel; die Zerlegung im Einzelnen
            // haelt `tests/fern.rs`.
            Weg::Sftp => assert_eq!(
                ziel,
                Verbindungsziel::Sftp(
                    krk_core::fern::fernziel("host.local/teil", "kai").expect("ein Ziel")
                )
            ),
        }
    }
}

/// Ein Konto mit `@` und Leerzeichen wird in der URL der Uebergabe kodiert;
/// beim Einhaengen reist es roh neben der URL; ein leeres Konto faellt weg.
#[test]
fn das_konto_wird_in_der_url_kodiert() {
    assert_eq!(
        verbindungsziel("ssh", "server", "kai @büro"),
        Ok(Verbindungsziel::Uebergeben {
            url: "ssh://kai%20%40b%C3%BCro@server".into()
        })
    );
    assert_eq!(
        verbindungsziel("SMB", "nas/daten", " kai @büro "),
        Ok(Verbindungsziel::Einhaengen {
            url: "smb://nas/daten".into(),
            konto: Some("kai @büro".into())
        })
    );
    assert_eq!(
        verbindungsziel("vnc", "mac.local", "  "),
        Ok(Verbindungsziel::Uebergeben {
            url: "vnc://mac.local".into()
        })
    );
    assert_eq!(
        verbindungsziel("smb", "nas", ""),
        Ok(Verbindungsziel::Einhaengen {
            url: "smb://nas".into(),
            konto: None
        })
    );
    assert_eq!(
        verbindungsziel("https", "example.org/pfad", "kai"),
        Ok(Verbindungsziel::Uebergeben {
            url: "https://example.org/pfad".into()
        })
    );
}

/// `#` und `?` in der Adresse sind auf dem Einhaengeweg Teil des Pfades und
/// werden kodiert, damit sie ihn nicht beenden; auf dem Uebergabeweg bleiben
/// sie Anfrage und Fragment
/// (`issues/261016-0614_*_die-verbindungs-url-kuerzt-den-pfad-an-raute-und-fragezeichen-und-der-vergleich-mit-der-eingehaengten-freigabe-liest-roh-gegen-kodiert.md`).
#[test]
fn raute_und_fragezeichen_beenden_den_pfad_einer_freigabe_nicht() {
    assert_eq!(
        verbindungsziel("smb", "nas.local/Daten#1/x", ""),
        Ok(Verbindungsziel::Einhaengen {
            url: "smb://nas.local/Daten%231/x".into(),
            konto: None
        })
    );
    assert_eq!(
        verbindungsziel("webdavs", "dav.example.org/a?b", ""),
        Ok(Verbindungsziel::Einhaengen {
            url: "https://dav.example.org/a%3Fb".into(),
            konto: None
        })
    );
    assert_eq!(
        verbindungsziel("https", "example.org/suche?q=1#oben", ""),
        Ok(Verbindungsziel::Uebergeben {
            url: "https://example.org/suche?q=1#oben".into()
        })
    );
}

/// Unbekanntes Protokoll, leere Adresse und ein Schema in der Adresse sind
/// Fehler, in dieser Reihenfolge geprueft; die Meldung nennt das Protokoll.
#[test]
fn ein_ziel_ohne_bekanntes_protokoll_oder_adresse_ist_ein_fehler() {
    assert_eq!(
        verbindungsziel(" rdp ", "host", ""),
        Err(Zielfehler::UnbekanntesProtokoll("rdp".into()))
    );
    assert_eq!(
        verbindungsziel("", "", ""),
        Err(Zielfehler::UnbekanntesProtokoll(String::new()))
    );
    assert_eq!(
        verbindungsziel("smb", "  ", ""),
        Err(Zielfehler::LeereAdresse)
    );
    assert_eq!(
        verbindungsziel("smb", "smb://nas", ""),
        Err(Zielfehler::SchemaInDerAdresse)
    );
    assert_eq!(protokoll_pruefen("Webdavs"), Ok(Protokoll::Webdavs));
    assert_eq!(
        protokoll_pruefen("gopher"),
        Err(Zielfehler::UnbekanntesProtokoll("gopher".into()))
    );
    let meldung = Zielfehler::UnbekanntesProtokoll("gopher".into()).meldung();
    assert!(
        meldung.contains("gopher") && meldung.contains("webdavs"),
        "{meldung}"
    );
    assert!(!Zielfehler::LeereAdresse.meldung().is_empty());
    assert!(Zielfehler::SchemaInDerAdresse.meldung().contains("://"));
}

/// Das Modul der Verbindungsziele nennt den Typ der Zugangsdaten nicht: kein
/// Wert dort kann ein Passwort tragen. Gelesen werden die Codezeilen, nicht
/// die Kommentare.
#[test]
fn das_verbindungsziel_nennt_den_typ_der_zugangsdaten_nicht() {
    let (_, inhalt) = gemeinsam::quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/heimordner/verbindung.rs")
        .expect("verbindung.rs steht im Baum");
    let code: Vec<&str> = inhalt
        .lines()
        .filter(|zeile| !zeile.trim_start().starts_with("//"))
        .collect();
    assert!(code.len() > 50, "verbindung.rs ist nicht gelesen");
    for nadel in ["Passwort", "passwort", "geheimnisse", "password"] {
        assert!(
            !code.iter().any(|zeile| zeile.contains(nadel)),
            "verbindung.rs nennt `{nadel}` im Code"
        );
    }
}
