//! Abnahme der Zusagen, die Aussagen ueber den **Quellbaum** sind.
//!
//! Vier Zusagen sagen eine Zahl von Stellen zu und keinen Rueckgabewert: genau
//! zwei Dateien mit `#![allow(unsafe_code)]` (C4.5), genau drei
//! Pruefordner-Fassungen (C4.6), genau zwei Absprachen ueber der Ablage (C3.14)
//! und kein Schreibweg an der Schreibsperre vorbei. An keinem Wert ist
//! abzulesen, dass es keine weitere gibt; geprueft wird deshalb am Baum.
//!
//! Daneben steht eine Probe anderer Art: sie sagt keine Zahl zu, sondern haelt
//! die Zahlen, die die **Prosa** des Ablagemoduls nennt, gegen die, die der
//! Baum fuehrt. Ihre Erwartung ist deshalb kein Literal, sondern
//! `Datei::ALLE`; siehe
//! `keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`.
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
use gemeinsam::{
    aufrufstellen, quelldateien, varianten_der_aufzaehlung, varianten_mit_nutzlast_der_aufzaehlung,
};

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
/// tut: sie legt an einem Wegwerfort etwas an und raeumt es in `Drop` wieder
/// ab. Die Gegenprobe sucht deshalb die drei Zeichen dieser Sache in
/// **derselben** Datei — `impl Drop`, den Ort und `remove_dir_all` — und findet
/// damit jede vierte Fassung, gleich wie sie heisst.
///
/// **Der Ort ist eine Nadel aus zweien**, und der Grund steht im Baum: neben
/// `std::env::temp_dir()` kennt dieses Projekt genau einen weiteren Wegwerfort,
/// den Messplatz unter `~/Library/Caches/krk-messplatz`. Alle drei anerkannten
/// Fassungen tragen einen Absatz, der ihn ausdruecklich ausschliesst — die
/// Frage ist hier also schon einmal gestellt worden —, und eine vierte Fassung
/// **dort** waere in jeder anderen Hinsicht dieselbe Sache und einer Nadel auf
/// `temp_dir()` allein unsichtbar.
///
/// **Was auch das nicht findet**, und der Satz gehoert dazu: eine Fassung, die
/// ueber zwei Dateien verteilt ist, eine, die ihren Ordner Eintrag fuer
/// Eintrag statt mit `remove_dir_all` abraeumt, oder eine an einem **dritten**
/// Ort, den dieses Projekt heute nicht kennt. Der Kopf dieser Datei sagt,
/// warum keine Nadel das leisten kann; der Befund dazu ist
/// `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0720_*`.
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
    let orte = [concat!("temp_", "dir()"), concat!("krk-", "messplatz")];
    let abraeumen = concat!("remove_dir", "_all");
    let weitere: Vec<String> = baum
        .iter()
        .filter(|(name, inhalt)| {
            im_code(inhalt, abraeumer)
                && orte.iter().any(|ort| im_code(inhalt, ort))
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
/// Ablageordners es brauchen, und `Ablage::pfad` liefert den Pfad jeder
/// Ablagedatei ohne Durchgang. Beides zusammen ergibt einen Schreibweg an der
/// Sperre vorbei, den kein Typ versperrt.
///
/// **Diese Zaehlung haengt weniger an einer Schreibweise als die uebrigen, und
/// das ist der Grund, aus dem sie hier steht.** Es gibt in Rust zwei Wege an
/// eine fremde Funktion: den Pfad an der Aufrufstelle oder ein `use`, das sie in
/// den Geltungsbereich holt. Beide nennen das Modul, also enthaelt jede Datei,
/// die `schreiben` unter diesem Namen erreicht, eine der vier Zeichenketten
/// `atomar::schreiben`, `atomar::{`, `atomar::*` oder `atomar as`. Die vierte
/// faengt die Einbindung unter anderem Namen
/// (`use krk_core::ablage::atomar as werkzeug;`), die sonst keine Nadel sieht.
///
/// # Der eine Weg, den keine Nadel sehen kann
///
/// **Eine Wiederausfuhr macht den Namen `atomar` entbehrlich.** Stuende in
/// einer der benannten Dateien `pub use atomar::schreiben;`, dann erreichte
/// jede Datei des Baums das atomare Schreiben ueber
/// `krk_core::ablage::schreiben`, ohne eine der vier Zeichenketten zu fuehren —
/// und diese Probe bliebe gruen. Sie ist damit **fuer den ganzen Baum blind**,
/// sobald eine einzige Zeile in einer Datei steht, die auf der Liste ohnehin
/// erlaubt ist. Gegen diesen Weg hilft keine Erweiterung der Nadelliste,
/// sondern nur, dass er hier benannt ist: wer eine Wiederausfuhr anlegt, hat
/// diesen Absatz gelesen. Der Befund dazu ist
/// `circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0715_*`.
///
/// Gesucht wird in Code-Zeilen: eine Datei, die den Namen nur bespricht — der
/// Kopf von `ablage::sperre` etwa, oder diese Zeile hier —, erreicht nichts.
/// Was daneben bleibt, ist ein Pfad, den jemand ueber zwei Zeilen umbricht;
/// `rustfmt` tut das nicht, und der Kopf dieser Datei sagt, warum keine Nadel
/// restlos dicht ist.
#[test]
fn nur_benannte_dateien_erreichen_das_atomare_schreiben() {
    let wege = [
        concat!("atomar::", "schreiben"),
        concat!("atomar::", "{"),
        concat!("atomar::", "*"),
        concat!("atomar", " as "),
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

/// C3.14: Der Leseweg der Leseprofile oeffnet keine Datei ueber ihren Pfad.
///
/// **Was das Kriterium sagt und was diese Probe misst, ist nicht dasselbe, und
/// das steht hier ausgeschrieben.** C3.14 sagt „keine neue Stelle **im Baum**";
/// gemessen wird der Leseweg der Leseprofile, also `krk-core/src/leseprofil/`
/// und `krk-core/src/ablage/leseprofile.rs`. Die weitere Zusage ueber den
/// ganzen Baum haelt diese Probe **nicht**: Kopieren, Entpacken, der
/// Verzeichnisleser und die Proben oeffnen ueber ihren Pfad, jeder aus einem
/// eigenen Grund, und eine Aufzaehlung darueber waere eine Liste ueber zwei
/// Kisten, die bei jeder Aenderung in der anderen rot wird, ohne dass an ihr
/// abzulesen waere, warum. Was C3.14 fuer die Zukunft schuetzen will, ist der
/// fuenfte Baustein, der `std::fs::read` ruft, und genau der faellt hier auf
/// (`circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten/issues/260824-1852_*_c3-14-nennt-seinen-eigenen-nachweis-und-nichts-im-baum-fuehrt-ihn.md`).
///
/// **Eine leere Menge und keine Aufzaehlung.** Der Leseweg liest ueber
/// `text::datei::anlesen` und `verzeichnis::leser::lesen_hoechstens`, und die
/// gehen beide durch `verzeichnis::sys::ohne_warten_oeffnen`, also ueber den
/// Deskriptor. Eine erlaubte Ausnahme gibt es hier nicht, deshalb steht keine
/// Liste da, die jemand stillschweigend verlaengern koennte.
///
/// Die Nadeln stehen zusammengesetzt da, aus dem Grund, den der Kopf dieser
/// Datei nennt; gesucht wird in Code-Zeilen, damit dieser Kommentar sie nennen
/// darf.
#[test]
fn der_leseweg_der_leseprofile_oeffnet_keine_datei_ueber_ihren_pfad() {
    let wege = [
        concat!("File::", "open"),
        concat!("fs::", "read("),
        concat!("read_to", "_string"),
        concat!("Open", "Options"),
    ];
    let leseweg = [
        "krk-core/src/leseprofil/",
        "krk-core/src/ablage/leseprofile.rs",
    ];

    let alle = quelldateien();
    let gelesen: Vec<&(String, String)> = alle
        .iter()
        .filter(|(name, _)| leseweg.iter().any(|teil| name.starts_with(*teil)))
        .collect();
    assert!(
        gelesen.len() > 1,
        "unter {leseweg:?} steht kein Leseweg; die Probe haette nichts zu pruefen"
    );

    let mit_pfadoeffnung: Vec<&str> = gelesen
        .iter()
        .filter(|(_, inhalt)| wege.iter().any(|weg| im_code(inhalt, weg)))
        .map(|(name, _)| name.as_str())
        .collect();
    assert!(
        mit_pfadoeffnung.is_empty(),
        "der Leseweg der Leseprofile oeffnet in {mit_pfadoeffnung:?} eine Datei ueber ihren \
         Pfad; gelesen wird ueber text::datei::anlesen und verzeichnis::leser::lesen_hoechstens, \
         und beide gehen ueber den Deskriptor"
    );
}

/// Kein Fadenstart im Baum wirft seinen Rueckgabewert weg.
///
/// **Was hier gehalten wird.** `thread::Builder` liefert seinen Fadenstart als
/// `io::Result` — anders als `thread::spawn`, das selbst in Panik geraet — und
/// es gibt ihn genau dafuer, dass der Aufrufer den Fehlschlag behandeln kann.
/// Ein `.expect(…)` oder `.unwrap()` daran nimmt dem Aufrufer die Wahl wieder
/// ab und laesst den **rufenden** Faden in Panik geraten, der in diesem Projekt
/// ueberall der Hauptfaden ist: die Sitzung endet samt allem, was
/// `krk_core::ablage` noch nicht geschrieben hat
/// (`shared/issues/260826-1221_*_zwei-fadenstarts-des-verzeichnisbaums-brechen-mit-panik-ab-waehrend-derselbe-mangel-am-deskriptor-sorgfaeltig-behandelt-ist.md`).
///
/// **Warum eine Probe und nicht der Uebersetzer.** `Result` traegt `#[must_use]`,
/// und ein `.expect(…)` verbraucht den Wert; der Bau ist damit zufrieden. Die
/// Zusage ist keine ueber den Typ, sondern eine ueber die Behandlung, und die
/// haelt hier niemand ausser dieser Stelle.
///
/// **Wo die Kette endet, und warum nicht am naechsten Strichpunkt.** Der erste
/// Entwurf dieser Probe las vom Bauer bis zum naechsten `;` und sah keine der
/// vier Stellen: der Rumpf des uebergebenen Abschlusses traegt selbst einen
/// Strichpunkt, und die Kette war zu Ende, bevor ihr letztes Glied gelesen war.
/// Eine Probe, die die falsche Stelle liest, ist schlimmer als keine, denn sie
/// bestaetigt. Gelesen wird deshalb bis zum ersten Strichpunkt **ausserhalb
/// jeder Klammer**; nachgeprueft ist es, indem `.expect(…)` versuchsweise
/// wieder eingesetzt wurde und die Probe rot wurde.
///
/// **Was die Nadel nicht sieht.** Ein Fadenstart, dessen `Result` erst mehrere
/// Anweisungen spaeter ausgepackt wird, entgeht ihr; `krk-ui` schreibt genau so
/// (`let gestartet = …; if let Err(fehler) = gestartet`), und das ist die
/// richtige Behandlung und keine Umgehung. Die Probe faengt die Form, die den
/// Defekt getragen hat, und behauptet nicht mehr.
#[test]
fn kein_fadenstart_im_baum_wirft_seinen_rueckgabewert_weg() {
    let bauer = concat!("Builder", "::new()");
    let weggeworfen = [concat!(".exp", "ect("), concat!(".unw", "rap(")];
    let mut fundstellen: Vec<String> = Vec::new();
    for (name, inhalt) in quelldateien() {
        // Kommentarzeilen fallen vorab: die Doc-Kommentare dieser Datei nennen
        // jede Nadel im Klartext, und ohne den Schnitt faende die Probe sich
        // selbst.
        let code: String = inhalt
            .lines()
            .filter(|zeile| !zeile.trim_start().starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n");
        for (anfang, _) in code.match_indices(bauer) {
            let mut tiefe = 0i32;
            let mut ende = code.len();
            for (versatz, zeichen) in code[anfang..].char_indices() {
                match zeichen {
                    '(' | '[' | '{' => tiefe += 1,
                    ')' | ']' | '}' => tiefe -= 1,
                    ';' if tiefe <= 0 => {
                        ende = anfang + versatz;
                        break;
                    }
                    _ => {}
                }
            }
            let kette = &code[anfang..ende];
            if weggeworfen.iter().any(|nadel| kette.contains(nadel)) {
                fundstellen.push(format!(
                    "{name}: {}",
                    kette.split_whitespace().collect::<Vec<_>>().join(" ")
                ));
            }
        }
    }
    assert!(
        fundstellen.is_empty(),
        "ein Fadenstart wirft seinen Rueckgabewert weg und laesst den rufenden \
         Faden in Panik geraten: {fundstellen:#?}"
    );
}

/// C3.14: Ueber der Ablage stehen genau zwei Absprachen und keine dritte.
///
/// Die Schreibsperre und das Sitzungsrecht, jede auf ihrer eigenen Datei im
/// Ablageordner.
///
/// # Gesucht wird der Gegenstand und nicht der Name
///
/// Bis zum 260905 las diese Probe **eine** Datei, `ablage/sperre.rs`, und
/// zaehlte darin die Zeilen, die mit `pub const` anfingen und auf `.lock`
/// endeten. Zwei Verengungen uebereinander, und beide standen nicht im
/// Doc-Kommentar: eine dritte Absprache, deren Name in `ablage/mod.rs` oder in
/// `einstellungen.rs` stuende, war unsichtbar, und eine, die `pub(crate) const`
/// hiesse, in einem `impl`-Block staende, von `rustfmt` umgebrochen waere oder
/// nicht auf `.lock` endete, entging dem Filter auch innerhalb von `sperre.rs`
/// (`shared/issues/260826-1302_*_die-probe-ueber-die-zwei-absprachen-liest-nur-sperre-rs-und-saehe-eine-dritte-daneben-nicht.md`).
///
/// Gesucht wird deshalb der **Gegenstand**: jede Sperrdatei entsteht ueber
/// `sperre::sperrdatei_oeffnen`, und die Probe laeuft ueber den ganzen
/// Quellbaum und haelt die Aufrufstellen als ausgeschriebene Liste. Eine dritte
/// Absprache faellt damit auf, gleich wie ihre Konstante geschrieben ist, wo sie
/// steht und worauf ihr Name endet.
///
/// **Was auch das nicht findet**, und der Satz gehoert dazu: eine Sperre, die
/// `OpenOptions` selbst aufmacht statt ueber die eine Huelle zu gehen. Der Kopf
/// dieser Datei sagt, warum keine Nadel restlos dicht ist.
#[test]
fn ueber_der_ablage_stehen_genau_zwei_absprachen() {
    let huelle = concat!("sperrdatei", "_oeffnen");
    let mut stellen: Vec<(String, usize)> = quelldateien()
        .iter()
        .map(|(name, inhalt)| (name.clone(), aufrufstellen(inhalt, huelle)))
        .filter(|(_, zahl)| *zahl > 0)
        .collect();
    stellen.sort();
    assert_eq!(
        stellen,
        vec![
            // Die Schreibsperre ueber einem vollstaendigen Durchgang.
            ("krk-core/src/ablage/mod.rs".to_owned(), 1),
            // Das Sitzungsrecht, genommen in `Schreibsperre::nehmen`.
            ("krk-core/src/ablage/sperre.rs".to_owned(), 1),
        ],
        "ueber der Ablage stehen nicht mehr genau zwei Absprachen"
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

/// Jede Prosastelle unter `ablage/`, die eine Zahl von Ablagedateien nennt,
/// nennt die Zahl, die [`Datei::ALLE`] fuehrt.
///
/// **Die Zahl selbst ist hier nicht ersetzbar, und deshalb steht sie unter
/// einer Probe.** An etlichen Stellen des Ablagemoduls traegt die Zahl die
/// Aussage — „acht Ablagedateien in zwei Formaten", „die sechs TOML-Dateien
/// gehen ueber `Zugang::laden`" —, und ein Zeiger auf `Datei::ALLE` naehme dem
/// Satz seinen Inhalt. Ohne eine Probe daneben ist eine solche Zahl die zweite
/// Fassung einer Liste, und die zweite Fassung ist die, die veraltet: fuenf
/// Erhebungen in Folge haben Stellen mit „vier Dateien" nachgezogen und dabei
/// jedes Mal welche stehen lassen
/// (`shared/issues/260826-1225_*_drei-prosastellen-der-ablage-nennen-die-zahl-der-dateien-falsch-und-jedes-bisherige-suchmuster-musste-sie-uebersehen.md`).
///
/// **Die Erwartung kommt aus dem Baum und nicht aus dieser Datei:**
/// `Datei::ALLE.len()` fuer die Ablagedateien, und die Zahl der Werte mit
/// [`Format::Toml`] fuer die TOML-Dateien. Eine achte Ablagedatei laesst die
/// Probe rot werden und nennt jede Stelle, die nachzuziehen ist.
///
/// # Was gelesen wird
///
/// Nur die Doc-Kommentare (`//!` und `///`) unter `krk-core/src/ablage/`, mit
/// abgezogenem Kommentarzeichen und zu einem Text zusammengezogen. Der
/// Zusammenzug ist der Punkt: die Stelle in `ablage/sperre.rs` stand ueber
/// einen Zeilenumbruch verteilt („dieselben vier\n Dateien") und ist deshalb
/// jeder zeilenweisen Suche entgangen, die sie haette finden sollen.
///
/// # Was nicht gezaehlt wird, und wo die Probe blind ist
///
/// - **Ein Zahlwort vor „der"** — „sieben der acht Ablagedateien" nennt eine
///   Teilmenge und keine Gesamtzahl. Es faellt heraus, weil gesucht wird, wo
///   ein Zahlwort **unmittelbar** vor einem der vier Hauptwoerter steht.
/// - **Ein Zitat einer frueheren Fassung.** Das Modul zitiert an einer Stelle
///   eine Begruendung, die bis zum 260824 dastand; sie nennt die damalige Zahl
///   und soll es weiter tun. Erkannt wird das an einem `„` in den 80 Zeichen
///   davor, ohne schliessendes `"` dazwischen. Ein laengeres Zitat faellt aus
///   diesem Fenster heraus und laesst die Probe rot werden; das ist der
///   gewollte Ausgang, denn eine unbemerkte Ausnahme waere teurer als eine
///   Probe, die einmal von Hand zu lesen ist.
/// - **Ein Zahlwort ohne Hauptwort** („Alle sieben, in fester Reihenfolge")
///   erreicht die Probe nicht. Die Zeile darunter ist dort das Feldliteral
///   `[Datei; 8]`, das der Uebersetzer haelt.
#[test]
fn keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien() {
    use krk_core::ablage::pfade::{Datei, Format};

    let alle = Datei::ALLE.len();
    let toml = Datei::ALLE
        .iter()
        .filter(|datei| datei.format() == Format::Toml)
        .count();

    let zahlwoerter = [
        ("ein", 1),
        ("eine", 1),
        ("einer", 1),
        ("zwei", 2),
        ("drei", 3),
        ("vier", 4),
        ("fuenf", 5),
        ("sechs", 6),
        ("sieben", 7),
        ("acht", 8),
        ("neun", 9),
        ("zehn", 10),
    ];
    let hauptwoerter = [
        ("Ablagedateien", alle),
        ("Nutzdateien", alle),
        ("Dateien", alle),
        ("TOML-Dateien", toml),
    ];

    let mut falsch: Vec<String> = Vec::new();
    for (name, inhalt) in quelldateien() {
        if !name.starts_with("krk-core/src/ablage/") {
            continue;
        }
        let prosa = prosa_zusammengezogen(&inhalt);
        for (stelle, wort) in wortstellen(&prosa) {
            let Some((_, gezaehlt)) = zahlwoerter
                .iter()
                .find(|(zahlwort, _)| wort.eq_ignore_ascii_case(zahlwort))
            else {
                continue;
            };
            let Some((hauptwort, erwartet)) =
                naechstes_hauptwort(&prosa, stelle, &wort).and_then(|folgt| {
                    hauptwoerter
                        .iter()
                        .find(|(hauptwort, _)| *hauptwort == folgt)
                })
            else {
                continue;
            };
            if steht_im_zitat(&prosa, stelle) {
                continue;
            }
            if gezaehlt != erwartet {
                falsch.push(format!(
                    "{name}: \"{wort} {hauptwort}\" — der Baum fuehrt {erwartet}"
                ));
            }
        }
    }

    assert!(
        falsch.is_empty(),
        "Prosastellen unter ablage/ nennen eine andere Zahl als der Baum: {falsch:#?}"
    );
}

/// Die Doc-Kommentare einer Quelldatei, ohne Kommentarzeichen und zu einem
/// Text zusammengezogen.
///
/// Der Zusammenzug ueber die Zeilengrenze hinweg ist Absicht; der Doc-Kommentar
/// der Probe darueber sagt, warum.
fn prosa_zusammengezogen(inhalt: &str) -> String {
    let mut prosa = String::new();
    for zeile in inhalt.lines() {
        let getrimmt = zeile.trim_start();
        let Some(rest) = getrimmt
            .strip_prefix("//!")
            .or_else(|| getrimmt.strip_prefix("///"))
        else {
            continue;
        };
        prosa.push(' ');
        prosa.push_str(rest.trim());
    }
    prosa
}

/// Jedes Wort des Textes mit seinem Byteversatz, von Satzzeichen befreit.
fn wortstellen(prosa: &str) -> Vec<(usize, String)> {
    let mut worte = Vec::new();
    let mut anfang = None;
    for (versatz, zeichen) in prosa.char_indices() {
        if zeichen.is_whitespace() {
            if let Some(start) = anfang.take() {
                worte.push((start, prosa[start..versatz].to_owned()));
            }
        } else if anfang.is_none() {
            anfang = Some(versatz);
        }
    }
    if let Some(start) = anfang {
        worte.push((start, prosa[start..].to_owned()));
    }
    worte
        .into_iter()
        .map(|(versatz, wort)| {
            (
                versatz,
                wort.trim_matches(|zeichen: char| {
                    !zeichen.is_alphanumeric() && zeichen != '-' && zeichen != '_'
                })
                .to_owned(),
            )
        })
        .collect()
}

/// Das Wort, das dem Zahlwort an `stelle` unmittelbar folgt, ohne Satzzeichen.
fn naechstes_hauptwort(prosa: &str, stelle: usize, zahlwort: &str) -> Option<String> {
    let rest = prosa.get(stelle..)?;
    let hinter_dem_zahlwort = rest.find(zahlwort)? + zahlwort.len();
    let folgt = rest.get(hinter_dem_zahlwort..)?.split_whitespace().next()?;
    Some(
        folgt
            .trim_matches(|zeichen: char| {
                !zeichen.is_alphanumeric() && zeichen != '-' && zeichen != '_'
            })
            .to_owned(),
    )
}

/// Ob die Stelle in einem Zitat einer frueheren Fassung steht.
///
/// Gesucht wird ein `„` in den 80 Zeichen davor, ohne schliessendes `"`
/// dazwischen. Die Schranke steht im Doc-Kommentar der rufenden Probe.
fn steht_im_zitat(prosa: &str, stelle: usize) -> bool {
    let davor: String = prosa[..stelle].chars().rev().take(80).collect();
    match (davor.find('„'), davor.find('"')) {
        (Some(_), None) => true,
        (Some(auf), Some(zu)) => auf < zu,
        _ => false,
    }
}

/// C4.8 der Runde 7: Beide Sperrgriffe der Ablage tragen `#[must_use]`, und
/// beide tragen eine Begruendung.
///
/// **Was das Kriterium zusagt.** Ein Griff, dessen Fallenlassen die Sperre
/// abgibt, gehoert in eine Bindung, die so lange lebt wie der Durchgang. Beide
/// Typen unter `krk-core/src/ablage/sperre.rs` sind von dieser Bauart:
/// `Schreibgriff` haelt die Schreibsperre ueber dem Ablageordner,
/// `Sitzungsrecht` das Recht, die Sitzung zu schreiben. Faellt einer von ihnen
/// still weg, laeuft der Durchgang ungeschuetzt weiter beziehungsweise haelt
/// eine zweite Instanz sich fuer die erste — beides ohne Meldung.
///
/// **Warum eine Probe und nicht der Bau.** `unused_must_use` ist erst unter
/// `-D warnings` ein Fehler, und er faellt nur an, wo ein Rueckgabewert
/// tatsaechlich fallengelassen wird. Verschwindet das Attribut, bleibt der Bau
/// gruen, solange kein Aufrufer es ausnutzt — und der naechste, der es tut,
/// bekommt keine Warnung mehr. C4.8 traegt die Kennzeichnung **(Probe)**, und
/// der Abgleich der Runde 7 hat festgehalten, dass es keine gab
/// (`circles/260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz/issues/260813-0647_*_neun-abnahmekriterien-versprechen-eine-probe-und-haben-keine.md`).
///
/// **Gefordert ist die Begruendung und nicht nur das Attribut.** Ein nacktes
/// `#[must_use]` meldet „unused return value"; die Begruendung sagt dem
/// naechsten Leser, was er verliert. Dieses Projekt schreibt sie an beiden
/// Stellen aus, und die Probe haelt das fest.
#[test]
fn beide_sperrgriffe_der_ablage_tragen_must_use_mit_begruendung() {
    let (_, sperre) = quelldateien()
        .into_iter()
        .find(|(name, _)| name == "krk-core/src/ablage/sperre.rs")
        .expect("krk-core/src/ablage/sperre.rs steht im Baum");

    let zeilen: Vec<&str> = sperre.lines().collect();
    for typ in ["pub struct Schreibgriff", "pub struct Sitzungsrecht"] {
        let stelle = zeilen
            .iter()
            .position(|zeile| zeile.starts_with(typ))
            .unwrap_or_else(|| panic!("{typ} steht nicht mehr in sperre.rs"));
        // Zwischen Attribut und Typ steht heute `#[derive(Debug)]`; gesucht
        // wird deshalb in den Zeilen davor bis zum Ende des Doc-Kommentars.
        let traegt_marke = zeilen[..stelle]
            .iter()
            .rev()
            .take_while(|zeile| zeile.starts_with("#[") || zeile.starts_with("///"))
            .any(|zeile| zeile.starts_with(concat!("#[must", "_use = \"")));
        assert!(
            traegt_marke,
            "{typ} traegt kein must_use mit Begruendung; ein fallengelassener Griff gibt seine Sperre still ab"
        );
    }
}

// ---------------------------------------------------------------------------
// Die ALLE-Listen neben ihren Aufzaehlungen
// ---------------------------------------------------------------------------

/// Die zwei `ALLE`-Listen, die diese Nadel nicht lesen kann, mit dem Grund.
///
/// **Eine Ausnahme steht hier und nicht als stiller Uebersprung im Code.** Der
/// Durchlauf haelt jeden Eintrag dieser Liste gegen den Baum: eine Ausnahme,
/// deren Fundstelle verschwindet oder deren Aufzaehlung umzieht, laesst die
/// Probe rot werden, statt als toter Eintrag stehen zu bleiben. Eine dritte
/// Ausnahme ist damit eine bewusste Eintragung und kein Versehen.
const UNLESBARE_ALLE_LISTEN: [(&str, &str, &str); 1] = [(
    "krk-core/src/ablage/pfade.rs",
    "Datei",
    "`Datei::Zettel(Zettel)` traegt Daten, und die Liste fuehrt eine Zeile je Zettel: \
         acht Eintraege zu sieben Varianten. Weder die Nadel ueber die Aufzaehlung noch \
         die ueber die Liste liest datentragende Varianten, und eine Gleichheit waere \
         hier ohnehin die falsche Zusage",
)];

/// Die Nadel, an der eine Liste `ALLE` erkannt wird.
///
/// **Zusammengesetzt, weil diese Datei in dem Baum liegt, den sie liest.** Als
/// ein Stueck geschrieben faende sie sich selbst dreimal — im Zerleger, im
/// Zaehlabgleich und in dessen Meldung — und der Abgleich schluege fehl. Die
/// Bauform ist die des Dateikopfes.
const ALLE_NADEL: &str = concat!("const ", "ALLE: [");

/// Jede Liste `ALLE` im Baum fuehrt genau die Varianten ihrer Aufzaehlung, in
/// deren Reihenfolge.
///
/// # Was der Uebersetzer davon haelt: nichts
///
/// `pub const ALLE: [Bereich; 6]` zwingt zu sechs Eintraegen und sagt nichts
/// darueber, **welche** sechs. Ein siebter Bereich uebersetzt anstandslos,
/// solange niemand die Liste anfasst, und schlaegt erst zur Laufzeit zu — als
/// `index out of bounds`, wo ein Feld ueber [`Bereich::index`] gegriffen wird,
/// oder als stilles Nichts, wo eine Schleife ueber `ALLE` den neuen Wert
/// einfach auslaesst. Die Runde 23 hat dafuer bezahlt: neun Stellen mussten von
/// Hand nachgezogen werden, und der Uebersetzer hat keine einzige genannt.
///
/// Entscheidbar wird die Frage aus einer zweiten Quelle, und die ist der
/// Quelltext der Aufzaehlung. So haelt die Runde 22 schon `Kommando::KENNUNGEN`
/// (`crates/krk-core/tests/belegung.rs`), so das Beschriftungsfeld von
/// `Wirkungsbereich` und so `Marke::ALLE` (`crates/krk-core/tests/git.rs`); der
/// Nutzer hat diese Bauform am 260907 fuer alle Listen gewaehlt und die fremde
/// Kiste `strum` verworfen
/// (`260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`).
///
/// # Warum ein Durchlauf und nicht dreizehn Proben
///
/// Dreizehn Proben deckten dreizehn Listen ab und liessen die vierzehnte, die
/// jemand naechste Woche schreibt, ungedeckt — genau die Luecke, gegen die
/// diese Probe gebaut ist, eine Ebene hoeher. Der Durchlauf sucht die Listen im
/// Baum, statt sie aufzuzaehlen; eine neue Liste ist damit vom Tag ihrer
/// Entstehung an gehalten, ohne dass jemand daran denkt.
///
/// **Das gilt fuer beide Kisten.** `krk-ui` hat kein Bibliotheksziel, eine
/// Datei unter `crates/krk-ui/tests/` erreicht also nichts aus `krk-ui`; sechs
/// der Listen liegen dort. Diese Probe erreicht sie trotzdem, weil sie beide
/// Seiten aus dem Quelltext liest und keine der beiden verlinkt.
///
/// # Was diese Nadel nicht sieht
///
/// - **Sie liest allein `crates/`**, weil `gemeinsam::quelldateien()` es tut.
///   Eine Liste `ALLE` in `xtask/` bekaeme sie nicht zu sehen; heute steht dort
///   keine.
/// - **Sie erkennt eine Liste an ihrem Namen `ALLE`.** Eine Liste, die
///   Vollstaendigkeit meint und anders heisst, entgeht ihr, und eine
///   Teilmenge wie `RECHTER_RAND: [Bereich; 3]` soll ihr entgehen. Der Name
///   ist die Zusage; ob eine Liste sie meint, ist an nichts sonst abzulesen.
/// - **Mehrzeilige Eintraege und Kommentare im Listenrumpf** bricht sie ab,
///   statt sie zu ueberspringen.
///
/// Leer laufen kann sie nicht: der Zaehlabgleich je Datei haelt fest, dass der
/// Zerleger jede Zeile mit `const ALLE: [` auch wirklich aufgenommen hat.
#[test]
fn jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung() {
    let mut gesehen: Vec<(String, String)> = Vec::new();

    for (datei, inhalt) in quelldateien() {
        let stellen = alle_listen(&datei, &inhalt);

        // **Der Zerleger wird gegen die rohe Zeilenzahl gehalten.** Ohne das
        // liesse eine geaenderte Schreibweise ihn still nichts mehr finden, und
        // die Probe bestaetigte einen Baum, den sie nicht gelesen hat.
        let roh = inhalt
            .lines()
            .filter(|zeile| {
                let rumpf = zeile.trim_start();
                !rumpf.starts_with("//") && rumpf.contains(ALLE_NADEL)
            })
            .count();
        assert_eq!(
            stellen.len(),
            roh,
            "in {datei} stehen {roh} Zeilen mit `{ALLE_NADEL}`, der Zerleger nimmt {} auf",
            stellen.len()
        );

        for (aufzaehlung, zeile) in stellen {
            gesehen.push((datei.clone(), aufzaehlung.clone()));
            if UNLESBARE_ALLE_LISTEN
                .iter()
                .any(|(ausnahme, name, _)| *ausnahme == datei && *name == aufzaehlung)
            {
                continue;
            }

            let gelistet = gelistete_namen(&datei, &inhalt, &aufzaehlung, zeile);
            let varianten = varianten_der_aufzaehlung(&datei, &aufzaehlung);

            // **Die fehlende Variante zuerst.** Sie ist die Auskunft, fuer die
            // diese Probe gebaut ist; eine Doppelung faellt bei gleicher
            // Feldlaenge immer zusammen mit einer Luecke an, und ihre Meldung
            // zuerst verdeckte den Namen, den der Leser sucht.
            let fehlen: Vec<&str> = varianten
                .iter()
                .filter(|name| !gelistet.contains(name))
                .map(String::as_str)
                .collect();
            assert!(
                fehlen.is_empty(),
                "diese Varianten von {aufzaehlung} stehen in keinem Eintrag von \
                 {aufzaehlung}::ALLE ({datei}): {}",
                fehlen.join(", ")
            );

            let doppelt: Vec<&str> = gelistet
                .iter()
                .enumerate()
                .filter(|(stelle, name)| gelistet[..*stelle].contains(name))
                .map(|(_, name)| name.as_str())
                .collect();
            assert!(
                doppelt.is_empty(),
                "{aufzaehlung}::ALLE in {datei} fuehrt diese Varianten mehr als einmal: {}",
                doppelt.join(", ")
            );

            let ueberzaehlig: Vec<&str> = gelistet
                .iter()
                .filter(|name| !varianten.contains(name))
                .map(String::as_str)
                .collect();
            assert!(
                ueberzaehlig.is_empty(),
                "diese Eintraege von {aufzaehlung}::ALLE ({datei}) benennen keine Variante \
                 der Aufzaehlung: {}",
                ueberzaehlig.join(", ")
            );

            // Die Reihenfolge ist bei mehreren dieser Listen tragend: `index()`
            // rechnet die Stelle in `ALLE` aus, und die Bereichsleiste reiht
            // ihre Schalter danach. Wo sie es nicht ist, kostet die Zusage
            // nichts, und eine bewusste Umstellung faellt hier auf.
            assert_eq!(
                gelistet, varianten,
                "{aufzaehlung}::ALLE in {datei} fuehrt die Varianten in einer anderen \
                 Reihenfolge als die Aufzaehlung"
            );
        }
    }

    for (datei, aufzaehlung, grund) in UNLESBARE_ALLE_LISTEN {
        assert!(
            gesehen
                .iter()
                .any(|(gefunden, name)| gefunden == datei && name == aufzaehlung),
            "die Ausnahme {aufzaehlung}::ALLE in {datei} steht nicht mehr im Baum; \
             sie ist einzutragen oder zu streichen (Grund war: {grund})"
        );
    }

    assert!(
        gesehen.len() > UNLESBARE_ALLE_LISTEN.len(),
        "unter crates/ steht keine lesbare Liste ALLE; die Nadel greift ins Leere"
    );
}

/// Die Fundstellen `const ALLE: [<Aufzaehlung>; N]` einer Datei: je Fund der
/// Name der Aufzaehlung und die Zeile, in der die Liste beginnt.
///
/// Getrennt vom Lesen des Rumpfes, weil der Rumpf einer Ausnahme aus
/// [`UNLESBARE_ALLE_LISTEN`] gar nicht erst gelesen werden soll: er ist ja
/// gerade der, an dem die Nadel abbraeche.
fn alle_listen(datei: &str, inhalt: &str) -> Vec<(String, usize)> {
    let mut gefunden = Vec::new();
    for (nummer, zeile) in inhalt.lines().enumerate() {
        let rumpf = zeile.trim();
        if rumpf.starts_with("//") {
            continue;
        }
        let ohne_sicht = rumpf
            .strip_prefix("pub ")
            .or_else(|| rumpf.strip_prefix("pub(crate) "))
            .or_else(|| rumpf.strip_prefix("pub(super) "))
            .unwrap_or(rumpf);
        let Some(rest) = ohne_sicht.strip_prefix(ALLE_NADEL) else {
            continue;
        };
        let aufzaehlung = rest
            .split(';')
            .next()
            .expect("ein split liefert immer ein erstes Stueck")
            .trim();
        assert!(
            !aufzaehlung.is_empty()
                && aufzaehlung
                    .chars()
                    .all(|zeichen| zeichen.is_ascii_alphanumeric() || zeichen == '_'),
            "in {datei} traegt die Zeile `{rumpf}` keinen einfachen Elementtyp"
        );
        gefunden.push((aufzaehlung.to_owned(), nummer));
    }
    gefunden
}

/// Die Namen, die die Liste `ALLE` ab Zeile `zeile` aufzaehlt, ohne den
/// Vorsatz `<Aufzaehlung>::`.
///
/// Ein Eintrag steht mit oder ohne diesen Vorsatz da: `Marke::ALLE` schreibt
/// ihn aus, die Probentafel in `loeschzielbefund.rs` fuehrt die drei Werte ein
/// und laesst ihn weg. Beide Schreibweisen benennen dieselbe Variante, und die
/// Nadel nimmt beide.
fn gelistete_namen(datei: &str, inhalt: &str, aufzaehlung: &str, zeile: usize) -> Vec<String> {
    let zeilen: Vec<&str> = inhalt.lines().collect();
    let kopf = zeilen[zeile].trim();
    let anfang = kopf
        .find("] = ")
        .unwrap_or_else(|| panic!("in {datei} steht `{kopf}` ohne `] = `"))
        + "] = ".len();

    let mut text = kopf[anfang..].trim().to_owned();
    let mut ende = zeile;
    while !text.ends_with("];") {
        ende += 1;
        let weiter = zeilen.get(ende).unwrap_or_else(|| {
            panic!("die Liste {aufzaehlung}::ALLE in {datei} endet an keiner Zeile auf `];`")
        });
        text.push(' ');
        text.push_str(weiter.trim());
        text = text.trim_end().to_owned();
    }

    let innen = text
        .strip_prefix('[')
        .and_then(|rumpf| rumpf.strip_suffix("];"))
        .unwrap_or_else(|| {
            panic!("die Liste {aufzaehlung}::ALLE in {datei} steht nicht als `[…];` da: `{text}`")
        });

    let vorsatz = format!("{aufzaehlung}::");
    let mut namen = Vec::new();
    for stueck in innen.split(',') {
        let eintrag = stueck.trim();
        if eintrag.is_empty() {
            continue;
        }
        let name = eintrag.strip_prefix(&vorsatz).unwrap_or(eintrag);
        let bezeichner: &str = name
            .split(|zeichen: char| !(zeichen.is_ascii_alphanumeric() || zeichen == '_'))
            .next()
            .expect("ein split liefert immer ein erstes Stueck");
        // **Ein Eintrag mit Nutzlast bleibt ganz stehen.** Der Durchlauf
        // uebergeht die eine Liste, die welche fuehrt (`Datei::ALLE`, siehe
        // `UNLESBARE_ALLE_LISTEN`); ihre eigene Probe braucht den Eintrag
        // dagegen im Wortlaut, weil ihre Zusage „einmal je Wert des Feldes"
        // lautet und nicht „genau einmal".
        assert!(
            !bezeichner.is_empty()
                && (name == bezeichner
                    || (name.starts_with(&format!("{bezeichner}(")) && name.ends_with(')'))),
            "die Liste {aufzaehlung}::ALLE in {datei} traegt den Eintrag `{eintrag}`; \
             diese Nadel liest eine Variante mit oder ohne `{vorsatz}` davor und mit \
             hoechstens einer Nutzlast in runden Klammern"
        );
        namen.push(name.to_owned());
    }
    assert!(
        !namen.is_empty(),
        "die Liste {aufzaehlung}::ALLE in {datei} liefert keinen Eintrag; \
         eine leere Liste waere eine Probe, die alles bestaetigt"
    );
    namen
}

// ───────────────────────────────────────────────────────────────────────────
// Die Untergrenzen-Angabe in den Modulkoepfen unter `krk-ui/src/appkit/`
// ───────────────────────────────────────────────────────────────────────────

/// Die Ueberschrift, unter der ein AppKit-Modul seine Untergrenzen nennt.
///
/// Sie steht zusammengesetzt da, aus dem Grund, den der Kopf dieser Datei
/// nennt: als ein Stueck faende sie sich hier selbst. Der Pfadfilter in
/// [`appkit_dateien`] schliesst diese Datei ohnehin aus, doch wer den Filter
/// spaeter weitet, soll die Nadel nicht mitzaehlen.
const UNTERGRENZEN_UEBERSCHRIFT: &str =
    concat!("# Ab welchem macOS die angesprochenen ", "Klassen stehen");

/// Die Dateien unter `krk-ui/src/appkit/`, in die Tiefe, je Name und Inhalt.
#[must_use]
fn appkit_dateien() -> Vec<(String, String)> {
    let gefunden: Vec<(String, String)> = quelldateien()
        .into_iter()
        .filter(|(name, _)| name.replace('\\', "/").starts_with("krk-ui/src/appkit/"))
        .collect();
    assert!(
        gefunden.len() > 1,
        "unter krk-ui/src/appkit/ steht keine Datei; die Probe haette nichts zu pruefen"
    );
    gefunden
}

/// Die `//!`-Zeilen am Kopf einer Datei, ohne den Vorsatz.
///
/// Vor dem Kopf duerfen Leerzeilen und Kistenattribute (`#![…]`) stehen —
/// `appkit/mod.rs` traegt `#![allow(unsafe_code)]` als erste Zeile. Die erste
/// Zeile, die weder das eine noch das andere ist, beendet den Kopf.
#[must_use]
fn modulkopf(inhalt: &str) -> Vec<&str> {
    let mut kopf = Vec::new();
    for zeile in inhalt.lines() {
        let gestutzt = zeile.trim();
        if let Some(rumpf) = gestutzt.strip_prefix("//!") {
            kopf.push(rumpf.trim());
        } else if gestutzt.is_empty() || gestutzt.starts_with("#!") {
            continue;
        } else {
            break;
        }
    }
    kopf
}

/// Der Text unter [`UNTERGRENZEN_UEBERSCHRIFT`] im Modulkopf, oder `None`,
/// wenn die Ueberschrift dort nicht steht.
///
/// Gelesen wird bis zur naechsten Ueberschrift derselben Ebene oder bis zum
/// Ende des Kopfes. Die Ueberschrift selbst gehoert nicht zum Text: sonst
/// zaehlte das Wort „Klassen“ aus ihr als Nennung.
#[must_use]
fn untergrenzen_abschnitt(inhalt: &str) -> Option<String> {
    let mut drin = false;
    let mut gesammelt: Vec<&str> = Vec::new();
    for zeile in modulkopf(inhalt) {
        if zeile.starts_with("# ") {
            if drin {
                break;
            }
            drin = zeile == UNTERGRENZEN_UEBERSCHRIFT;
            continue;
        }
        if drin {
            gesammelt.push(zeile);
        }
    }
    drin.then(|| gesammelt.join(" "))
}

/// Die Namen, die die `use`-Zeilen auf oberster Ebene aus einer
/// **Frameworkbindung** hereinholen.
///
/// Eine Frameworkbindung ist eine Kiste, deren Name auf `objc2_` beginnt —
/// `objc2_app_kit`, `objc2_foundation`, `objc2_pdf_kit`, `objc2_quartz_core`,
/// `objc2_core_foundation`. Die Kernkiste `objc2` steht nicht darunter, und
/// das ist am Kistennamen entschieden und nicht an einer Liste: sie fuehrt
/// Rust-Werkzeug (`Retained`, `ProtocolObject`, `msg_send!`), das kein
/// macOS-Alter hat.
///
/// # Warum nicht nach Klassen gefragt wird
///
/// Ob ein hereingeholter Name eine Objective-C-Klasse benennt, ist am
/// Quelltext **nicht** entscheidbar: `NSPoint` und `NSPasteboard` sehen gleich
/// aus, das eine ist eine C-Struktur und das andere eine Klasse, und nichts in
/// der `use`-Zeile trennt sie. Gefragt wird deshalb die Frage, die der Baum
/// beantworten kann — steht jeder hereingeholte Name im Abschnitt —, und ihre
/// Antwort ist eine Obermenge: jede Klasse ist ein hereingeholter Name. Der
/// Preis ist, dass der Abschnitt auch das nennen muss, was keine Klasse ist;
/// der Gewinn ist, dass keine Ausnahmeliste danebensteht, die jemand pflegen
/// muesste.
///
/// # Was diese Nadel nicht sieht
///
/// Eine `use`-Zeile **innerhalb** eines Moduls (die Pruefmodule am Dateiende
/// schreiben sie eingerueckt) und einen voll ausgeschriebenen Pfad mitten im
/// Rumpf (`objc2_foundation::NSNotification` als Argumenttyp). Beide holen
/// einen Namen herein, den diese Probe nicht einfordert.
#[must_use]
fn frameworknamen(inhalt: &str) -> std::collections::BTreeSet<String> {
    let mut namen = std::collections::BTreeSet::new();
    let zeilen: Vec<&str> = inhalt.lines().collect();
    let mut nummer = 0;
    while nummer < zeilen.len() {
        let zeile = zeilen[nummer];
        if !zeile.starts_with("use objc2") {
            nummer += 1;
            continue;
        }
        let mut anweisung = zeile.to_owned();
        while !anweisung.contains(';') {
            nummer += 1;
            let weiter = zeilen
                .get(nummer)
                .expect("eine use-Zeile endet vor dem Dateiende auf ein Semikolon");
            anweisung.push(' ');
            anweisung.push_str(weiter.trim());
        }
        nummer += 1;

        let rumpf = anweisung
            .trim_start_matches("use ")
            .split(';')
            .next()
            .expect("ein split liefert immer ein erstes Stueck");
        let (kiste, rest) = rumpf
            .split_once("::")
            .unwrap_or_else(|| panic!("die Zeile `{anweisung}` traegt keinen Pfad mit `::`"));
        if !kiste.trim().starts_with("objc2_") {
            continue;
        }
        for stueck in rest.split(|zeichen: char| !zeichen.is_ascii_alphanumeric() && zeichen != '_')
        {
            if !stueck.is_empty() {
                namen.insert(stueck.to_owned());
            }
        }
    }
    namen
}

/// Ob `name` im Text als ganzes Wort steht, und nicht bloss als Dateiname
/// einer SDK-Kopfzeile.
///
/// Ganz und nicht als Teil: sonst deckte `NSRect` die Nennung von
/// `NSRectEdge` mit ab und `NSString` die von `NSStringDrawing`.
///
/// # Warum `NSAlert.h` keine Nennung von `NSAlert` ist
///
/// Die Abschnitte belegen ihre Zahlen mit der Kopfzeile, in der sie stehen,
/// und eine solche Angabe traegt den Klassennamen mit: `NSAlert.h:22` steht
/// dort, um die Aufzaehlung `NSAlertStyle` zu belegen, und nicht, um etwas
/// ueber `NSAlert` zu sagen. Eine Wortsuche ohne diese Einschraenkung nahm die
/// Angabe als Nennung — die Gegenprobe am 260907 blieb gruen, nachdem
/// `NSAlert` aus dem Abschnitt von `appkit/hinweis.rs` entfernt war, weil
/// `NSAlert.h` danebenstand. Ein Name unmittelbar vor `.h` zaehlt deshalb
/// nicht.
#[must_use]
fn steht_als_wort(text: &str, name: &str) -> bool {
    let zeichen: Vec<char> = text.chars().collect();
    let gesucht: Vec<char> = name.chars().collect();
    let wortzeichen = |z: char| z.is_ascii_alphanumeric() || z == '_';
    zeichen
        .windows(gesucht.len())
        .enumerate()
        .any(|(i, fenster)| {
            let nach = i + gesucht.len();
            let kopfzeile = zeichen.get(nach) == Some(&'.') && zeichen.get(nach + 1) == Some(&'h');
            fenster == gesucht.as_slice()
                && !(i > 0 && wortzeichen(zeichen[i - 1]))
                && !zeichen.get(nach).copied().is_some_and(wortzeichen)
                && !kopfzeile
        })
}

/// Moeglichkeit 1 des Entscheids `260811-2050`: der Abschnitt steht da.
///
/// `objc2` fuehrt keine Verfuegbarkeitsangaben mit sich, der Uebersetzer haelt
/// die Untergrenze macOS 15 also nicht, und wer eine spaeter hinzugekommene
/// Methode anspricht, bekommt keine Warnung, sondern einen Absturz auf dem
/// Referenzgeraet. Die Gegenmassnahme ist der Abschnitt im Modulkopf, und sie
/// war eine blosse Gewohnheit: die Deckung war bis zum 260811 auf fuenf
/// Dateien abgesunken und ist von Hand wiederhergestellt worden.
///
/// # Die zwei Ausnahmen stehen nicht als Liste da
///
/// `appkit/koordinaten.rs` rechnet auf einer Zeichenkette und `appkit/mod.rs`
/// haengt Module ein; keine der beiden holt einen Namen aus einer
/// Frameworkbindung herein, und keine schuldet deshalb einen Abschnitt. Die
/// Bedingung ist die Eigenschaft und nicht der Dateiname: eine dritte Datei
/// ohne solchen Import faellt von selbst heraus, und `koordinaten.rs` schuldet
/// den Abschnitt in dem Augenblick, in dem sie den ersten hereinholt.
///
/// # Was diese Probe nicht haelt
///
/// **Die Richtigkeit der Zahl.** Sie sieht, dass die Ueberschrift dasteht,
/// nicht ob unter ihr etwas Wahres steht. Die Zahl am SDK zu pruefen waere
/// Moeglichkeit 3 des Entscheids, verlangte Xcode und waere ein halber
/// Uebersetzer; der Nutzer hat sie ausdruecklich verworfen. Die Richtigkeit
/// der Zahl bleibt eine Zusage des Menschen.
#[test]
fn jede_appkit_datei_mit_frameworkimport_traegt_den_untergrenzen_abschnitt() {
    let mut ohne: Vec<String> = Vec::new();
    for (name, inhalt) in appkit_dateien() {
        if frameworknamen(&inhalt).is_empty() {
            continue;
        }
        if untergrenzen_abschnitt(&inhalt).is_none() {
            ohne.push(name);
        }
    }
    assert!(
        ohne.is_empty(),
        "diesen Dateien fehlt im Modulkopf der Abschnitt \
         `{UNTERGRENZEN_UEBERSCHRIFT}`, obwohl sie eine Frameworkbindung ansprechen: {ohne:?}"
    );
}

/// Moeglichkeit 2 des Entscheids `260811-2050`: jeder hereingeholte Name steht
/// namentlich im Abschnitt.
///
/// Das faengt den zweithaeufigsten Fehler, die **vergessene** Klasse. Was ein
/// hereingeholter Name ist und warum nicht nach Klassen gefragt wird, steht an
/// [`frameworknamen`]; was ein Abschnitt ist, an [`untergrenzen_abschnitt`].
///
/// # Was diese Probe nicht haelt
///
/// **Die Richtigkeit der Zahl** — siehe die Probe darueber; das gilt hier
/// genauso und ist der Grund, warum der Name dieser Probe von `steht` spricht
/// und nicht von `stimmt`.
///
/// **Den Satz um den Namen herum.** Geprueft ist, dass der Name im Abschnitt
/// als ganzes Wort vorkommt. Ein Satz, der ihn nennt, um zu sagen, dass die
/// Datei ihn *nicht mehr* anspricht, deckt ihn genauso ab wie eine Angabe.
///
/// **Die Formen, die [`frameworknamen`] nicht sieht** — die eingerueckte
/// `use`-Zeile eines Pruefmoduls und den voll ausgeschriebenen Pfad im Rumpf.
/// Sie sind dort einzeln benannt.
#[test]
fn jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt() {
    let mut fehlend: Vec<String> = Vec::new();
    for (name, inhalt) in appkit_dateien() {
        let Some(abschnitt) = untergrenzen_abschnitt(&inhalt) else {
            continue;
        };
        let ungenannt: Vec<String> = frameworknamen(&inhalt)
            .into_iter()
            .filter(|gesucht| !steht_als_wort(&abschnitt, gesucht))
            .collect();
        if !ungenannt.is_empty() {
            fehlend.push(format!("{name}: {}", ungenannt.join(", ")));
        }
    }
    assert!(
        fehlend.is_empty(),
        "diese Namen kommen aus einer Frameworkbindung herein, ohne im Abschnitt \
         `{UNTERGRENZEN_UEBERSCHRIFT}` genannt zu sein:\n{}",
        fehlend.join("\n")
    );
}

/// `Datei::ALLE` fuehrt jede Variante, und die datentragende einmal je Zettel.
///
/// **Die Ausnahme des Durchlaufs bekommt hier ihre eigene Probe.**
/// `jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung` uebergeht
/// `Datei::ALLE`, weil `Datei::Zettel(Zettel)` Daten traegt und die Liste
/// deshalb mehr Eintraege fuehrt als die Aufzaehlung Varianten hat; eine
/// Gleichheit waere dort die falsche Zusage. **Die richtige lautet: jede
/// datenlose Variante genau einmal, und die datentragende einmal je Wert ihres
/// Feldes**, und sie steht hier
/// (`shared/issues/260907-0858_*_zwei-alle-listen-bleiben-vom-durchlauf-ungedeckt-*`).
///
/// Beide Seiten kommen aus dem Quelltext und keine aus der anderen: die
/// Erwartung aus den Aufzaehlungen `Datei` und `Zettel`, der Bestand aus der
/// Liste. Eine achte Ablagedatei ohne Zeile in `ALLE` laesst die Probe rot
/// werden und nennt ihren Namen — genau die Auskunft, fuer die der Durchlauf
/// gebaut ist.
#[test]
fn die_ablageliste_fuehrt_jede_datei_und_je_einen_zettel() {
    const PFADE: &str = "krk-core/src/ablage/pfade.rs";

    let quellen = quelldateien();
    let (_, inhalt) = quellen
        .iter()
        .find(|(pfad, _)| pfad == PFADE)
        .expect("unter crates/ steht keine pfade.rs");

    let zettel = varianten_der_aufzaehlung(PFADE, "Zettel");
    let erwartet: Vec<String> = varianten_mit_nutzlast_der_aufzaehlung(PFADE, "Datei")
        .into_iter()
        .flat_map(|(name, nutzlast)| match nutzlast {
            None => vec![name],
            // Der Wortlaut der Nutzlast ist zugleich der Name der Aufzaehlung,
            // aus der die Werte kommen; eine zweite Angabe daneben waere eine
            // zweite Wahrheit darueber, was in den Klammern steht.
            Some(typ) => {
                assert_eq!(
                    typ, "Zettel",
                    "Datei traegt eine Nutzlast vom Typ {typ}; diese Probe kennt allein Zettel"
                );
                zettel
                    .iter()
                    .map(|wert| format!("{name}({typ}::{wert})"))
                    .collect()
            }
        })
        .collect();

    let zeile = alle_listen(PFADE, inhalt)
        .into_iter()
        .find(|(aufzaehlung, _)| aufzaehlung == "Datei")
        .map(|(_, zeile)| zeile)
        .unwrap_or_else(|| panic!("in pfade.rs steht keine Liste `{ALLE_NADEL}Datei; N]`"));
    let gelistet = gelistete_namen(PFADE, inhalt, "Datei", zeile);

    assert_eq!(
        gelistet, erwartet,
        "Datei::ALLE fuehrt nicht jede Variante genau einmal und den Zettel je Wert, \
         oder in einer anderen Reihenfolge als die Aufzaehlung"
    );
}

/// Der Kuerzer langer Namenslisten hat genau zwei Rufer, und den Wortlaut
/// „… und N weitere" traegt er allein.
///
/// **Die Bauform stammt von `die_zeichenregel_hat_drei_rufer_und_der_vergleich_drei`**
/// (`krk-core/tests/verzeichnis.rs`), und die Frage ist dieselbe: eine Regel,
/// die an einer Stelle steht, bleibt nur dann an einer Stelle, wenn jemand
/// nachzaehlt. Der Unterschied zur Vorlage ist, dass die Heimat hier selbst
/// ein Rufer ist: `namenszeile` in `ablage/neuerungen.rs` kuerzt die Namen des
/// Blattes, `uebersprungenliste` in `kommandos/operationen.rs` die
/// Abschlussliste der uebersprungenen Eintraege. Ein dritter Rufer ist kein
/// Fehler, sondern ein Grund, diese Zeile zu aendern; eine dritte Fassung des
/// **Wortlauts** ist einer, und die faengt die zweite Behauptung.
#[test]
fn der_kuerzer_langer_namenslisten_hat_genau_zwei_rufer() {
    let kuerzer = concat!("gekue", "rzt");
    let heimat = "krk-core/src/ablage/neuerungen.rs";
    // Der Wortlaut in zwei Stuecken, damit diese Datei sich nicht selbst
    // findet; dasselbe Mittel wie bei jeder Nadel dieser Datei.
    let wortlaut = concat!("… und {} ", "weitere");

    let mut rufer = Vec::new();
    let mut wortlautstellen = Vec::new();
    let mut heimat_erklaert = false;
    for (name, inhalt) in quelldateien() {
        if name == heimat {
            heimat_erklaert = im_code(&inhalt, &format!("pub fn {kuerzer}("));
        }
        // Eine Probe ist kein Rufer im Sinne des Kriteriums und ihr Wortlaut
        // keine zweite Fassung: sie prueft beide, statt sie zu setzen.
        // Gefragt ist, wer die Kuerzung im Betrieb ausloest.
        if !name.contains("/src/") {
            continue;
        }
        let stellen = aufrufstellen(&inhalt, kuerzer);
        if stellen > 0 {
            rufer.push((name.clone(), stellen));
        }
        if im_code(&inhalt, wortlaut) {
            wortlautstellen.push(name);
        }
    }

    assert!(heimat_erklaert, "{heimat} erklaert den Kuerzer nicht mehr");

    assert_eq!(
        rufer,
        vec![
            (heimat.to_owned(), 1),
            ("krk-ui/src/kommandos/operationen.rs".to_owned(), 1),
        ],
        "der Kuerzer hat andere Rufer als die Namenszeile des Blattes und die \
         Abschlussliste der uebersprungenen Eintraege"
    );
    assert_eq!(
        wortlautstellen,
        vec![heimat.to_owned()],
        "der Wortlaut „… und N weitere\" steht nicht mehr allein in {heimat}"
    );
}
