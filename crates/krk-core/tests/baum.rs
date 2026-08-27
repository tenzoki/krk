//! Abnahme der Zusagen, die Aussagen ueber den **Quellbaum** sind.
//!
//! Vier Zusagen sagen eine Zahl von Stellen zu und keinen Rueckgabewert: genau
//! zwei Dateien mit `#![allow(unsafe_code)]` (C4.5), genau drei
//! Pruefordner-Fassungen (C4.6), genau zwei Absprachen ueber der Ablage (C3.14)
//! und kein Schreibweg an der Schreibsperre vorbei. An keinem Wert ist
//! abzulesen, dass es keine weitere gibt; geprueft wird deshalb am Baum.
//!
//! # Gezaehlt werden Erklaerungen und keine Aufrufer
//!
//! Die Unterscheidung ist nicht kosmetisch, und `krk_ui::quellbaum` schreibt sie
//! aus. Eine Erklaerungszaehlung haelt gegen eine zweite Fassung **desselben
//! Namens** und laesst sie rot werden. Eine Aufruferzaehlung ist in beide
//! Richtungen blind und steht nur dort, wo ein Kriterium die Zahl selbst
//! zusagt.
//!
//! # Was eine Nadel nicht entscheiden kann
//!
//! **Keine Suche im Quelltext entscheidet, ob irgendwo eine zweite Fassung
//! derselben Sache steht.** Eine Fassung unter anderem Namen, in anderer
//! Schreibweise oder ueber zwei Dateien verteilt entgeht jeder Nadel; die Runde
//! 7 hat das an der eigenen C4.6-Probe vorgefuehrt, die eine vierte
//! Pruefordner-Fassung namens `Ordner` nicht sah. Die Gegenmassnahme ist nicht
//! eine schaerfere Nadel, sondern eine andere Frage: gesucht wird, wo es geht,
//! nach dem **Gegenstand** statt nach seinem Namen — nach dem `impl Drop` neben
//! einem Temporaerordner statt nach `struct Pruefordner`, nach jedem Weg an eine
//! Funktion statt nach einer Schreibweise ihres Aufrufs. Was danach an
//! Blindheit bleibt, steht am jeweiligen Doc-Kommentar und wird nicht
//! verschwiegen.
//!
//! # Die Nadel steht zusammengesetzt da
//!
//! Diese Datei liegt in dem Baum, den sie liest. Eine Nadel, die als ein Stueck
//! im Quelltext staende, faende sich selbst und zaehlte eine Fundstelle zu viel;
//! sie wird deshalb mit `concat!` aus zwei Teilen gebaut. Die Bauform stammt von
//! `es_gibt_genau_einen_menuebauer` in `krk_ui::appkit::teilen`, der aeltesten
//! Probe dieser Art.

mod gemeinsam;
use gemeinsam::quelldateien;

/// Ob eine Nadel in einer **Code**-Zeile der Datei steht und nicht in einem
/// Kommentar.
///
/// Der Unterschied ist in dieser Datei tragend. Die Doc-Kommentare hier nennen
/// jede Nadel im Klartext, damit ein Leser weiss, wonach gesucht wird; ein
/// `contains` ueber den ganzen Text fand deshalb diese Datei selbst und jede
/// andere, die den Namen nur bespricht. Gefragt ist aber, wer eine Sache
/// **tut**, und das steht nie hinter `//`.
fn im_code(inhalt: &str, nadel: &str) -> bool {
    inhalt
        .lines()
        .any(|zeile| !zeile.trim_start().starts_with("//") && zeile.contains(nadel))
}

/// C4.5: Die Ausnahme von `deny(unsafe_code)` steht an genau zwei Stellen.
///
/// **Die Runde 7 bringt einen fuenften Fremdaufruf, `flock(2)`, und trotzdem
/// keine dritte Ausnahme.** Er ist in `verzeichnis/sys.rs` gelandet, der einen
/// Datei des Kerns mit dieser Ausnahme; eine eigene Datei fuer die beiden
/// Sperren der Ablage waere die dritte gewesen. Die Probe nennt die beiden
/// Dateien beim Namen, damit ein Umzug hier auffaellt und nicht nur eine Zahl
/// gleich bleibt.
#[test]
fn genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code() {
    let nadel = concat!("#![allow(unsafe", "_code)]");
    // **Verglichen wird die ganze Zeile und nicht ihr Vorkommen im Text.** Die
    // Ausnahme wird an mehreren Stellen des Baums besprochen, unter anderem im
    // Kopf dieser Datei; ein `contains` zaehlte jede Erwaehnung mit und machte
    // aus einer Zusage ueber den Bau eine ueber die Prosa.
    let dateien: Vec<String> = quelldateien()
        .into_iter()
        .filter(|(_, inhalt)| inhalt.lines().any(|zeile| zeile.trim() == nadel))
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        dateien,
        vec![
            "krk-core/src/verzeichnis/sys.rs".to_owned(),
            "krk-ui/src/appkit/mod.rs".to_owned(),
        ],
        "die Liste der Ausnahmen von deny(unsafe_code) hat sich geaendert"
    );
}

/// C4.6: Es gibt genau drei Pruefordner-Fassungen, eine je Kiste.
///
/// Dass es drei sind und nicht eine, liegt an den Kistengrenzen und nicht an
/// Nachlaessigkeit: `krk-ui` und `krk-bench` haben nur ein Binaerziel, und ein
/// Testziel erreicht den Code eines Binaerziels nicht. Der Modulkopf von
/// `tests/gemeinsam/mod.rs` schreibt es aus. Eine vierte waere ein Doppelbau.
///
/// # Gesucht wird der Gegenstand und nicht sein Name
///
/// Bis zur Runde 7 suchte die Gegenprobe die Nadel `impl Drop for Pruefordner`.
/// Sie band damit an den **Namen**, und eine vierte Fassung namens `Ordner`
/// stand seit S13 in `krk-core/src/ablage/sperre.rs`, ohne dass die Probe
/// etwas gemeldet haette; denselben blinden Fleck hatte sie fuer den
/// anerkannten `Wegwerfordner`
/// (`issues/260813-0540_*_eine-vierte-pruefordner-fassung-steht-im-baum-und-die-probe-sieht-sie-nicht.md`).
///
/// Was eine Pruefordner-Fassung ausmacht, ist nicht ihr Name, sondern was sie
/// tut: sie legt unter dem Temporaerverzeichnis etwas an und raeumt es in
/// `Drop` wieder ab. Die Gegenprobe sucht deshalb die drei Zeichen dieser
/// Sache in **derselben** Datei — `impl Drop`, `temp_dir()` und
/// `remove_dir_all` — und findet damit jede vierte Fassung, gleich wie sie
/// heisst.
///
/// **Was auch das nicht findet**, und der Satz gehoert dazu: eine Fassung, die
/// ueber zwei Dateien verteilt ist, oder eine, die ihren Ordner Eintrag fuer
/// Eintrag statt mit `remove_dir_all` abraeumt. Der Kopf dieser Datei sagt,
/// warum keine Nadel das leisten kann.
#[test]
fn genau_drei_pruefordner_fassungen_stehen_im_baum() {
    let fassungen = [
        ("krk-core/tests/gemeinsam/mod.rs", "struct Pruefordner"),
        ("krk-ui/src/pruefordner.rs", "struct Pruefordner"),
        ("krk-bench/src/wegwerfordner.rs", "struct Wegwerfordner"),
    ];
    let baum = quelldateien();
    for (datei, nadel) in fassungen {
        let (_, inhalt) = baum
            .iter()
            .find(|(name, _)| name == datei)
            .unwrap_or_else(|| panic!("{datei} steht nicht mehr im Baum"));
        assert!(
            inhalt.contains(nadel),
            "{datei} erklaert seinen Pruefordner nicht mehr"
        );
    }

    // Die drei Zeichen der Sache. Zwei Vorkehrungen gegen den Selbstfund, und
    // beide sind noetig: die Nadeln stehen zusammengesetzt da, weil diese Datei
    // in dem Baum liegt, den sie liest, und gesucht wird nur in Code-Zeilen,
    // weil die Doc-Kommentare darueber alle drei im Klartext nennen.
    let abraeumer = concat!("impl Drop", " for ");
    let ort = concat!("temp_", "dir()");
    let abraeumen = concat!("remove_dir", "_all");
    let weitere: Vec<String> = baum
        .iter()
        .filter(|(name, inhalt)| {
            im_code(inhalt, abraeumer)
                && im_code(inhalt, ort)
                && im_code(inhalt, abraeumen)
                && !fassungen.iter().any(|(fassung, _)| fassung == name)
        })
        .map(|(name, _)| name.clone())
        .collect();
    assert!(
        weitere.is_empty(),
        "eine vierte Pruefordner-Fassung steht im Baum: {weitere:?}"
    );
}

/// C3.7 der Runde 19: Genau eine Stelle im Baum zaehlt einen Ordnerbestand
/// nach Typ und nach versteckt, und sie steht in `leseprofil/bausteine.rs`.
///
/// Gesucht wird nach dem Gegenstand und nicht nach dem Namen einer Funktion:
/// eine Datei, die in ihren Code-Zeilen das Kennzeichen eines Eintrags liest
/// (`.versteckt`) **und** nach seinem Typ fragt (`.typ ==`, `== Typ::`,
/// `.ist_ordner()`, `.ist_verknuepfung()`). Drei Dateien tun beides, und die
/// Probe nennt sie beim Namen wie
/// [`genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code`]: in
/// `verzeichnis/eintrag.rs` entsteht das Kennzeichen, in
/// `verzeichnis/modell.rs` liest es der Ausblendeschalter, und in
/// `leseprofil/bausteine.rs` wird gezaehlt. Die zwei ersten gruppieren nichts,
/// und die Probe kann das nicht sehen; was sie sieht, ist eine **vierte**
/// Datei, die beide Fragen stellt, und die ist dann zu lesen.
///
/// `leseprofil/datei.rs` traegt `zaehlung.versteckt` und faellt bewusst
/// heraus: es liest den Schluessel aus der Profildatei und keinen Eintrag, und
/// nach einem Typ fragt es nicht, sondern ordnet ihn zu. Gezaehlt wird unter
/// `crates/*/src`, wie C3.7 es sagt: eine Abnahmeprobe unter `tests/` stellt
/// beide Fragen an einen Eintrag, den sie gelesen hat, und gruppiert nichts.
///
/// # Was diese Nadel nicht sieht
///
/// Eine Datei, die beide Felder ueber ein Muster bindet
/// (`Eintrag { typ, versteckt, .. }`) oder den Typ mit `matches!` fragt,
/// entgeht ihr; der Kopf dieser Datei sagt, warum keine Nadel das leisten
/// kann.
#[test]
fn genau_drei_dateien_lesen_das_kennzeichen_versteckt_und_fragen_nach_dem_typ() {
    let kennzeichen = concat!(".", "versteckt");
    let typfragen = [
        concat!(".typ", " =="),
        concat!("== ", "Typ::"),
        concat!(".ist_", "ordner()"),
        concat!(".ist_", "verknuepfung()"),
    ];
    let dateien: Vec<String> = quelldateien()
        .into_iter()
        .filter(|(name, inhalt)| {
            name.contains("/src/")
                && im_code(inhalt, kennzeichen)
                && typfragen.iter().any(|frage| im_code(inhalt, frage))
        })
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        dateien,
        vec![
            "krk-core/src/leseprofil/bausteine.rs".to_owned(),
            "krk-core/src/verzeichnis/eintrag.rs".to_owned(),
            "krk-core/src/verzeichnis/modell.rs".to_owned(),
        ],
        "eine andere Datei als die drei benannten liest das Kennzeichen versteckt und fragt \
         nach dem Typ; steht dort ein zweiter Zaehlweg?"
    );
}

/// C2.7 der Runde 19, die strukturelle Haelfte: unter `leseprofil/` erreicht
/// keine Code-Zeile den Ausblendeschalter des Ordnermodells.
///
/// Die Zahlen der drei Zaehlzeilen folgen `shift+cmd+h` nicht, und das ist an
/// keinem Rueckgabewert abzulesen: eine Zaehlung, die den Schalter fragte,
/// lieferte bei ausgeblendeten Verstecken dieselben Werte wie eine, die es
/// nicht tut, solange die Probe den Schalter nicht umlegt. Gehalten wird
/// deshalb am Baum: das Modul `leseprofil` nennt weder das Ordnermodell noch
/// eines seiner drei `verstecke_*`-Glieder. Die Gegenprobe daneben haelt fest,
/// dass die Nadel etwas findet, wo der Schalter wohnt; ohne sie bestaende die
/// Probe auch nach einer Umbenennung des Schalters.
///
/// # Was diese Nadel nicht sieht
///
/// Einen Weg ueber einen Zwischentraeger, etwa einen Wahrheitswert, den ein
/// Rufer aus `krk-ui` dem Kern hereinreicht. Der Kopf dieser Datei sagt, warum
/// keine Nadel restlos dicht ist.
#[test]
fn keine_code_zeile_unter_leseprofil_erreicht_den_ausblendeschalter() {
    let nadeln = [concat!("verstecke", "_"), concat!("Ordner", "modell")];
    let baum = quelldateien();

    let (_, modell) = baum
        .iter()
        .find(|(name, _)| name == "krk-core/src/verzeichnis/modell.rs")
        .expect("krk-core/src/verzeichnis/modell.rs steht nicht mehr im Baum");
    assert!(
        im_code(modell, nadeln[0]),
        "der Ausblendeschalter des Ordnermodells heisst nicht mehr verstecke_*; die Nadel \
         findet nichts mehr und die Probe belegt nichts"
    );

    let erreicher: Vec<String> = baum
        .iter()
        .filter(|(name, inhalt)| {
            name.starts_with("krk-core/src/leseprofil/")
                && nadeln.iter().any(|nadel| im_code(inhalt, nadel))
        })
        .map(|(name, _)| name.clone())
        .collect();
    assert!(
        erreicher.is_empty(),
        "unter leseprofil/ erreicht eine Code-Zeile den Ausblendeschalter: {erreicher:?}"
    );
}

/// Nur benannte Dateien erreichen [`atomar::schreiben`].
///
/// **Die eine Luecke im Satz „kein Schreibweg an der Sperre vorbei".** Der
/// Modulkopf von `krk_core::ablage` schreibt aus, was die Typen halten und was
/// nicht: `atomar::schreiben` ist `pub`, weil zwei Schreiber ausserhalb des
/// Ablageordners es brauchen, und `Ablage::pfad` liefert den Pfad einer der vier
/// Dateien ohne Durchgang. Beides zusammen ergibt einen Schreibweg an der Sperre
/// vorbei, den kein Typ versperrt.
///
/// **Diese Zaehlung haengt ausnahmsweise nicht an einer Schreibweise, und das
/// ist der Grund, aus dem sie hier steht.** Es gibt in Rust genau zwei Wege an
/// eine fremde Funktion: den Pfad an der Aufrufstelle oder ein `use`, das sie in
/// den Geltungsbereich holt. Beide nennen das Modul, also enthaelt jede Datei,
/// die `schreiben` ueberhaupt erreichen kann, eine der drei Zeichenketten
/// `atomar::schreiben`, `atomar::{` oder `atomar::*`. Ein anderer Weg besteht
/// nicht; wer die Liste erweitert, tut es sichtbar.
///
/// Gesucht wird in Code-Zeilen: eine Datei, die den Namen nur bespricht — der
/// Kopf von `ablage::sperre` etwa, oder diese Zeile hier —, erreicht nichts.
/// Was bleibt, ist ein Pfad, den jemand ueber zwei Zeilen umbricht; `rustfmt`
/// tut das nicht, und der Kopf dieser Datei sagt, warum keine Nadel restlos
/// dicht ist.
#[test]
fn nur_benannte_dateien_erreichen_das_atomare_schreiben() {
    let wege = [
        concat!("atomar::", "schreiben"),
        concat!("atomar::", "{"),
        concat!("atomar::", "*"),
    ];
    let erreichbar: Vec<String> = quelldateien()
        .into_iter()
        .filter(|(_, inhalt)| wege.iter().any(|weg| im_code(inhalt, weg)))
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        erreichbar,
        vec![
            // Vier Schreiber hinter einem `Zugang`: `Zugang::sichern`,
            // `Zugang::text_sichern`, `Zugang::beiseite_legen` und die Anlage
            // von `settings.toml`.
            "krk-core/src/ablage/einstellungen.rs".to_owned(),
            // Die Anlage von `readers.toml`, unter einem Durchgang.
            "krk-core/src/ablage/leseprofile.rs".to_owned(),
            "krk-core/src/ablage/mod.rs".to_owned(),
            // Der Editor sichert seine Datei, ausserhalb des Ablageordners.
            "krk-core/src/text/datei.rs".to_owned(),
            // Der Rundlauf schreibt `settings.toml`, unter einem Durchgang.
            "krk-core/tests/ablage.rs".to_owned(),
            // Die Markdown-Ausgabe nach ~/Downloads, ausserhalb des Ordners.
            "krk-ui/src/belegungsausgabe.rs".to_owned(),
        ],
        "eine andere Datei als die benannten kann das atomare Schreiben erreichen"
    );
}

/// C3.14: Ueber der Ablage stehen genau zwei Absprachen und keine dritte.
///
/// Die Schreibsperre und das Sitzungsrecht, jede auf ihrer eigenen Datei im
/// Ablageordner. Gezaehlt werden die **erklaerten** Sperrdateinamen: eine
/// dritte Absprache braeuchte eine dritte Datei, und sie faellt hier auf.
#[test]
fn ueber_der_ablage_stehen_genau_zwei_absprachen() {
    let nadel = concat!(".lo", "ck\"");
    let (_, sperre) = quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/ablage/sperre.rs")
        .expect("krk-core/src/ablage/sperre.rs steht nicht mehr im Baum");
    let benannt: Vec<&str> = sperre
        .lines()
        .filter(|zeile| zeile.trim_start().starts_with("pub const") && zeile.contains(nadel))
        .collect();
    assert_eq!(
        benannt.len(),
        2,
        "ueber der Ablage stehen nicht mehr genau zwei Absprachen: {benannt:?}"
    );
    assert_eq!(
        krk_core::ablage::sperre::SCHREIBSPERRE,
        "schreiben.lock",
        "die Schreibsperre hat ihren Dateinamen gewechselt"
    );
    assert_eq!(
        krk_core::ablage::sperre::SITZUNGSRECHT,
        "sitzungsrecht.lock",
        "das Sitzungsrecht hat seinen Dateinamen gewechselt"
    );
}
