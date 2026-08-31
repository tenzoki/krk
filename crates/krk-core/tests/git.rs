//! Abnahmeproben des Gitlesers (Stufe A, Runde 23).
//!
//! Jede Probe legt ihr eigenes Pruefrepository an, ueber die Fassung des
//! selbstabraeumenden Pruefordners aus `tests/gemeinsam/mod.rs`. **Eine vierte
//! Pruefordner-Fassung entsteht nicht** (C8.6, Bedingung 9 des Specs); die
//! Zaehlprobe `genau_drei_pruefordner_fassungen_stehen_im_baum` in
//! `tests/baum.rs` bleibt gruen.
//!
//! # Warum `/usr/bin/git` und nicht `gix`
//!
//! Die Repositorys werden mit dem Werkzeug des Systems angelegt und nicht mit
//! der Kiste, die hier geprueft wird. Eine Probe, die ihren Gegenstand auch als
//! Werkzeug benutzt, prueft die Uebereinstimmung der Kiste mit sich selbst;
//! gefragt ist aber, ob KRK dasselbe sieht wie `git`. Das ist derselbe Grund,
//! aus dem der Pruefordner seine Roehre ueber `/usr/bin/mkfifo` anlegt, statt
//! eine Bindung dafuer zu bauen. Und die Stufe A schreibt nicht, kann sich ein
//! Repository also gar nicht selbst bauen.
//!
//! Jeder Aufruf faehrt mit einer eigenen Identitaet und ohne die Konfiguration
//! des Benutzers: [`git`] setzt `HOME` auf den Pruefordner und `user.name`,
//! `user.email` und `init.defaultBranch` je Aufruf. Sonst entschiede die
//! `~/.gitconfig` des Geraets, welchen Branchnamen die Probe erwartet.
//!
//! # Zwei Gegenstaende in einer Datei
//!
//! Die Proben des synchronen Lesers (Schritt 3 der Runde 23) und die des
//! nebenlaeufigen Laufs (Schritt 4) stehen hier zusammen, weil beide dasselbe
//! Pruefrepository brauchen und `repository` es an genau einer Stelle anlegt.
//! Die Abschnittsueberschriften trennen sie.

mod gemeinsam;

use std::path::Path;
use std::process::Command;

use gemeinsam::{Pruefordner, aufrufstellen, kind_mit_deskriptorgrenze, kindauftrag};
use krk_core::git::lauf::{Gitfrage, Gitlauf, Gitmeldung, VERLAUFSSCHRITT};
use krk_core::git::leser::{Gitleser, Oeffnung};
use krk_core::git::texte::zusammenfassung;
use krk_core::git::{Kopf, Marke};
use krk_core::verzeichnis::{Ordnermodell, lesen};

/// Die Deskriptorgrenze der Kindproben.
///
/// 64, die Zahl, unter der ein aus dem Finder gestartetes Buendel ungefaehr
/// laeuft, und dieselbe, die die Deskriptorproben des Durchlaufs seit der
/// Runde 10 fahren.
const GRENZE_DESKRIPTOREN: usize = 64;

/// Der Branchname, auf dem jedes Pruefrepository steht.
const BRANCH: &str = "haupt";

// ---------------------------------------------------------------------------
// Pruefrepositorys anlegen
// ---------------------------------------------------------------------------

/// Ruft `git` im genannten Ordner und haelt, dass der Aufruf gelungen ist.
fn git(ordner: &Path, argumente: &[&str]) -> String {
    let ergebnis = Command::new("/usr/bin/git")
        .current_dir(ordner)
        .env("HOME", ordner)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "Probe")
        .env("GIT_AUTHOR_EMAIL", "probe@example.org")
        .env("GIT_COMMITTER_NAME", "Probe")
        .env("GIT_COMMITTER_EMAIL", "probe@example.org")
        .args(argumente)
        .output()
        .expect("git laesst sich nicht starten");
    assert!(
        ergebnis.status.success(),
        "git {argumente:?} in {} ist gescheitert\n--- stdout ---\n{}\n--- stderr ---\n{}",
        ordner.display(),
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
    String::from_utf8_lossy(&ergebnis.stdout).trim().to_owned()
}

/// Dasselbe, aber ein Fehlschlag ist erlaubt; die Ausgabe faellt weg.
///
/// Genau ein Aufruf braucht das, naemlich der `merge`, der den Konflikt
/// herstellt: er endet mit 1, und das ist sein Zweck.
fn git_darf_scheitern(ordner: &Path, argumente: &[&str]) {
    let _ = Command::new("/usr/bin/git")
        .current_dir(ordner)
        .env("HOME", ordner)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "Probe")
        .env("GIT_AUTHOR_EMAIL", "probe@example.org")
        .env("GIT_COMMITTER_NAME", "Probe")
        .env("GIT_COMMITTER_EMAIL", "probe@example.org")
        .args(argumente)
        .output()
        .expect("git laesst sich nicht starten");
}

/// Ein frisches Repository mit einem ersten Commit.
fn repository(zweck: &str) -> Pruefordner {
    let ordner = Pruefordner::neu(zweck);
    git(ordner.pfad(), &["init", "-q", "-b", BRANCH]);
    git(ordner.pfad(), &["config", "user.name", "Probe"]);
    git(
        ordner.pfad(),
        &["config", "user.email", "probe@example.org"],
    );
    ordner.datei("erste.txt", "eins\n");
    git(ordner.pfad(), &["add", "-A"]);
    git(ordner.pfad(), &["commit", "-q", "-m", "der erste Commit"]);
    ordner
}

/// Der Leser zu einem Ordner, oder ein Fehlschlag mit Meldung.
fn leser(ordner: &Path) -> Gitleser {
    match Gitleser::oeffnen(ordner) {
        Oeffnung::Offen(leser) => *leser,
        andere => panic!("{} liefert {andere:?} statt eines Lesers", ordner.display()),
    }
}

/// Die Marken eines Ordners als sortierte Paare, fuer den Vergleich.
fn marken_sortiert(leser: &Gitleser, ordner: &Path) -> Vec<(String, Marke)> {
    let mut gefunden = leser
        .marken(ordner)
        .expect("die Marken sind unentschieden geblieben");
    gefunden.sort_by(|links, rechts| links.0.cmp(&rechts.0));
    gefunden
}

// ---------------------------------------------------------------------------
// C3: was der Kopf sagt
// ---------------------------------------------------------------------------

/// C3.1: Der Kopf nennt den Branch, auf dem HEAD steht.
#[test]
fn der_kopf_nennt_den_branch() {
    let ordner = repository("branch");
    let kopf = leser(ordner.pfad()).kopf().expect("kein Kopf");
    assert_eq!(
        kopf,
        Kopf::Branch(BRANCH.to_owned()),
        "der Kopf nennt nicht den Branch, auf dem HEAD steht"
    );
}

/// C3.6, A6: Bei abgeloestem HEAD steht der Kurzhash und kein erfundener
/// Branchname.
///
/// Der Verlauf steht dabei wie sonst; das ist die zweite Haelfte von A6 und
/// wird hier mitgeprueft, weil ein abgeloester HEAD sonst als „kein Verlauf"
/// durchginge.
#[test]
fn ein_abgeloester_kopf_traegt_den_kurzhash() {
    let ordner = repository("abgeloest");
    let voll = git(ordner.pfad(), &["rev-parse", "HEAD"]);
    git(ordner.pfad(), &["checkout", "-q", "--detach", &voll]);

    let leser = leser(ordner.pfad());
    let Some(Kopf::Abgeloest(kurzhash)) = leser.kopf() else {
        panic!("ein abgeloester HEAD liefert {:?}", leser.kopf());
    };
    assert_eq!(
        kurzhash.len(),
        7,
        "der Kurzhash traegt nicht sieben Zeichen"
    );
    assert!(
        voll.starts_with(&kurzhash),
        "der Kurzhash {kurzhash} ist kein Anfang von {voll}"
    );
    assert_eq!(
        leser.verlauf(0, 50).expect("kein Verlauf").len(),
        1,
        "der Verlauf steht bei abgeloestem HEAD nicht wie sonst"
    );
}

/// C3.7, A7: Ein Repository ohne Commit nennt den Branchnamen, liefert einen
/// leeren Verlauf und keinen Fehler.
///
/// **Die Fussangel, gegen die diese Probe steht:** `head_name()` liefert bei
/// ungeborenem HEAD den Namen, `head_id()` scheitert mit `Unborn`. Wer die
/// beiden Faelle nicht trennt, bekommt einen Fehler statt einer leeren Liste.
/// Der leere Verlauf ist deshalb `Some(leer)` und nicht `None`: „es gibt keine
/// Commits" ist eine entschiedene Antwort.
#[test]
fn ein_repository_ohne_commit_nennt_den_branch_und_liefert_keinen_verlauf() {
    let ordner = Pruefordner::neu("ungeboren");
    git(ordner.pfad(), &["init", "-q", "-b", BRANCH]);

    let leser = leser(ordner.pfad());
    assert_eq!(
        leser.kopf(),
        Some(Kopf::OhneCommit(BRANCH.to_owned())),
        "ein ungeborener HEAD wird nicht vom gewoehnlichen getrennt"
    );
    assert_eq!(
        leser.verlauf(0, 50),
        Some(Vec::new()),
        "ein Repository ohne Commit liefert keinen leeren Verlauf, sondern etwas anderes"
    );
}

/// C3.10: Ein Unterordner wird als Repository behandelt, und die
/// Zusammenfassung meint den Unterordner.
///
/// Zwei Aussagen in einer Probe, und sie gehoeren zusammen: `discover` findet
/// den Baum aufwaerts, **und** der Status ist auf den angezeigten Ordner
/// beschraenkt. Ohne die zweite waere die erste eine Falle, denn ein Ordner in
/// einem Repository ist nicht dasselbe wie ein Repository.
#[test]
fn ein_unterordner_gilt_als_repository_und_seine_zusammenfassung_meint_ihn() {
    let ordner = repository("unterordner");
    let unter = ordner.ordner("unter");
    std::fs::write(unter.join("drin.txt"), "drin\n").expect("Datei");
    ordner.datei("draussen.txt", "draussen\n");

    let von_unten = leser(&unter);
    assert_eq!(
        von_unten.kopf(),
        Some(Kopf::Branch(BRANCH.to_owned())),
        "aus dem Unterordner heraus steht der Branch nicht"
    );
    assert_eq!(
        marken_sortiert(&von_unten, &unter),
        vec![("drin.txt".to_owned(), Marke::Neu)],
        "die Marken des Unterordners tragen einen Eintrag von ausserhalb"
    );
    assert_eq!(
        zusammenfassung(&marken_sortiert(&von_unten, &unter)),
        "1 neu in diesem Ordner",
        "die Zusammenfassung des Unterordners zaehlt den ganzen Baum"
    );

    // Und von der Wurzel aus gesehen faellt derselbe Befund auf den Ordner,
    // ueber den er zu erreichen ist. Ohne diese Haelfte traege ein Ordner nie
    // eine Marke, und ein unverfolgter Unterbaum bliebe in der Liste unsichtbar.
    let von_oben = leser(ordner.pfad());
    assert_eq!(
        marken_sortiert(&von_oben, ordner.pfad()),
        vec![
            ("draussen.txt".to_owned(), Marke::Neu),
            ("unter".to_owned(), Marke::Neu),
        ],
        "der Befund im Unterbaum faellt nicht auf den Ordner, der ihn traegt"
    );
}

// ---------------------------------------------------------------------------
// C4: der Verlauf
// ---------------------------------------------------------------------------

/// C4.1, E12: Der erste Aufruf liefert fuenfzig Commits, wenn so viele da sind.
///
/// Der zweite Aufruf setzt hinter dem letzten an und doppelt keine Zeile; das
/// ist die Regel, auf der das Nachladen aus E12 aufsetzt, und sie steht hier,
/// weil eine gedoppelte Zeile in der Liste sonst erst am Buendel auffiele.
#[test]
fn der_erste_aufruf_liefert_fuenfzig_commits() {
    let ordner = repository("fuenfzig");
    for nummer in 2..=60 {
        ordner.datei("erste.txt", format!("stand {nummer}\n"));
        git(ordner.pfad(), &["add", "-A"]);
        git(
            ordner.pfad(),
            &["commit", "-q", "-m", &format!("Commit {nummer}")],
        );
    }

    let leser = leser(ordner.pfad());
    let erste = leser.verlauf(0, 50).expect("kein Verlauf");
    assert_eq!(erste.len(), 50, "der erste Aufruf liefert nicht fuenfzig");

    let letzter = erste.last().expect("die Liste ist leer").id;
    let weitere = leser.verlauf(50, 50).expect("kein Nachschlag");
    assert_eq!(
        weitere.len(),
        10,
        "der Nachschlag liefert nicht die uebrigen zehn"
    );
    assert!(
        !weitere.iter().any(|commit| commit.id == letzter),
        "der Nachschlag doppelt den Commit, hinter dem er ansetzt"
    );
    assert_eq!(
        erste[0].kurzbeschreibung, "Commit 60",
        "der Verlauf beginnt nicht beim juengsten Commit"
    );
    assert_eq!(erste[0].autor, "Probe", "der Autor steht nicht am Commit");
    assert_eq!(
        erste[0].email, "probe@example.org",
        "die E-Mail steht nicht am Commit"
    );
}

/// C4.5: Ein Repository mit drei Commits liefert drei und meldet damit, dass
/// nichts mehr folgt.
///
/// **Woran der Rufer es merkt**, ist die Laenge: sie ist kleiner als die
/// gefragte Zahl. Ein eigenes Kennzeichen daneben waere eine zweite Quelle fuer
/// dieselbe Auskunft.
#[test]
fn drei_commits_liefern_drei_und_melden_das_ende() {
    let ordner = repository("dreicommits");
    for nummer in 2..=3 {
        ordner.datei("erste.txt", format!("stand {nummer}\n"));
        git(ordner.pfad(), &["add", "-A"]);
        git(
            ordner.pfad(),
            &["commit", "-q", "-m", &format!("Commit {nummer}")],
        );
    }

    let leser = leser(ordner.pfad());
    let verlauf = leser.verlauf(0, 50).expect("kein Verlauf");
    assert_eq!(verlauf.len(), 3, "drei Commits liefern nicht drei Zeilen");
    assert!(
        verlauf.len() < 50,
        "die Laenge meldet nicht, dass nichts mehr folgt"
    );

    assert_eq!(
        leser.verlauf(3, 50),
        Some(Vec::new()),
        "hinter dem aeltesten Commit kommt noch etwas"
    );
}

/// C4.2, C4.3: Die Vereinigung aller Schwuenge traegt jeden Commit genau
/// einmal, auch wo der Graph sich verzweigt.
///
/// **Eine lineare Kette kann das nicht messen.** Ein Nachschlag, der beim
/// zuletzt angezeigten Commit ansetzt, liefert allein dessen Vorfahren; wo
/// mehrere Zweige nebeneinander in der Warteschlange stehen, faellt jeder
/// Commit dauerhaft heraus, der beim Schwungende darin stand und kein Vorfahre
/// des letzten angezeigten ist. Das Pruefrepository hier traegt deshalb zwei
/// Zweige von je dreissig Commits und eine Zusammenfuehrung darueber, also mehr
/// als [`VERLAUFSSCHRITT`]: der erste Schwung endet mitten in der Verzweigung.
///
/// Der Sollstand kommt von `git rev-list HEAD` und nicht aus einer Zahl in
/// dieser Probe: gefragt ist, ob KRK dasselbe sieht wie `git`.
#[test]
fn die_vereinigung_aller_schwuenge_traegt_jeden_commit_genau_einmal() {
    let ordner = repository("zusammenfuehrung");
    let pfad = ordner.pfad().to_owned();

    git(&pfad, &["branch", "zweig"]);
    for nummer in 1..=30 {
        ordner.datei("haupt.txt", format!("stand {nummer}\n"));
        git(&pfad, &["add", "-A"]);
        git(&pfad, &["commit", "-q", "-m", &format!("Haupt {nummer}")]);
    }
    git(&pfad, &["checkout", "-q", "zweig"]);
    for nummer in 1..=30 {
        ordner.datei("zweig.txt", format!("stand {nummer}\n"));
        git(&pfad, &["add", "-A"]);
        git(&pfad, &["commit", "-q", "-m", &format!("Zweig {nummer}")]);
    }
    git(&pfad, &["checkout", "-q", BRANCH]);
    git(
        &pfad,
        &[
            "merge",
            "-q",
            "--no-ff",
            "-m",
            "die Zusammenfuehrung",
            "zweig",
        ],
    );

    let soll: Vec<String> = git(&pfad, &["rev-list", "HEAD"])
        .lines()
        .map(str::to_owned)
        .collect();
    assert!(
        soll.len() > VERLAUFSSCHRITT,
        "das Pruefrepository traegt nicht mehr Commits als ein Schwung: {}",
        soll.len()
    );

    let leser = leser(&pfad);
    let mut gesehen: Vec<String> = Vec::new();
    let mut bereits = 0;
    loop {
        let schwung = leser
            .verlauf(bereits, VERLAUFSSCHRITT)
            .expect("der Verlauf ist unentschieden geblieben");
        let laenge = schwung.len();
        gesehen.extend(schwung.into_iter().map(|commit| commit.id.to_string()));
        bereits += laenge;
        if laenge < VERLAUFSSCHRITT {
            break;
        }
    }

    let mut gesehen_sortiert = gesehen.clone();
    gesehen_sortiert.sort_unstable();
    let vorher = gesehen_sortiert.len();
    gesehen_sortiert.dedup();
    assert_eq!(
        vorher,
        gesehen_sortiert.len(),
        "ein Commit steht in zwei Schwuengen"
    );

    let mut soll_sortiert = soll.clone();
    soll_sortiert.sort_unstable();
    assert_eq!(
        gesehen_sortiert, soll_sortiert,
        "die Schwuenge tragen nicht jeden Commit des Repositorys genau einmal"
    );
}

// ---------------------------------------------------------------------------
// C5: die fuenf Markenzustaende
// ---------------------------------------------------------------------------

/// C5.3, E11: Die fuenf Zustaende an je einem Eintrag, gegen die erwartete
/// Zuordnung von Name auf Buchstabe.
///
/// Ein Repository, fuenf Eintraege, fuenf Marken. Der Konflikt entsteht ueber
/// eine Zusammenfuehrung zweier Zweige, die dieselbe Zeile verschieden
/// aendern; die Umbenennung ueber `git mv`, damit sie als solche im Index
/// steht und nicht als Paar aus Loeschung und Neuzugang.
///
/// **Der unveraenderte Eintrag traegt keine Marke** (A11): `erste.txt` steht
/// im Repository und kommt in der Liste nicht vor. Ohne diese Zusicherung
/// bestuende die Probe auch dann, wenn jeder Eintrag eine Marke bekaeme.
#[test]
fn die_fuenf_zustaende_tragen_ihre_fuenf_buchstaben() {
    let ordner = repository("fuenfzustaende");
    let pfad = ordner.pfad().to_owned();

    // Der Grundstand: vier verfolgte Dateien neben `erste.txt`.
    ordner.datei("geaendert.txt", "grund\n");
    ordner.datei("vorgemerkt.txt", "grund\n");
    ordner.datei("konflikt.txt", "grund\n");
    ordner.datei("altername.txt", "grund\n");
    git(&pfad, &["add", "-A"]);
    git(&pfad, &["commit", "-q", "-m", "der Grundstand"]);

    // Der Konflikt: zwei Zweige aendern dieselbe Zeile.
    git(&pfad, &["checkout", "-q", "-b", "zweig"]);
    ordner.datei("konflikt.txt", "aus dem Zweig\n");
    git(&pfad, &["add", "konflikt.txt"]);
    git(&pfad, &["commit", "-q", "-m", "aus dem Zweig"]);
    git(&pfad, &["checkout", "-q", BRANCH]);
    ordner.datei("konflikt.txt", "aus dem Hauptzweig\n");
    git(&pfad, &["add", "konflikt.txt"]);
    git(&pfad, &["commit", "-q", "-m", "aus dem Hauptzweig"]);
    git_darf_scheitern(&pfad, &["merge", "zweig"]);

    // Die uebrigen vier Zustaende.
    ordner.datei("geaendert.txt", "geaendert\n");
    ordner.datei("vorgemerkt.txt", "vorgemerkt\n");
    git(&pfad, &["add", "vorgemerkt.txt"]);
    git(&pfad, &["mv", "altername.txt", "neuername.txt"]);
    ordner.datei("neu.txt", "neu\n");

    let gefunden = marken_sortiert(&leser(&pfad), &pfad);
    let erwartet = vec![
        ("geaendert.txt".to_owned(), Marke::Geaendert),
        ("konflikt.txt".to_owned(), Marke::Konflikt),
        ("neu.txt".to_owned(), Marke::Neu),
        ("neuername.txt".to_owned(), Marke::Umbenannt),
        ("vorgemerkt.txt".to_owned(), Marke::Vorgemerkt),
    ];
    assert_eq!(
        gefunden, erwartet,
        "die fuenf Zustaende treffen nicht ihre fuenf Marken"
    );

    let buchstaben: String = gefunden
        .iter()
        .map(|(_, marke)| marke.buchstabe())
        .collect();
    assert_eq!(
        buchstaben, "MKNUS",
        "die Buchstaben stehen nicht wie in E11"
    );
    assert!(
        !gefunden.iter().any(|(name, _)| name == "erste.txt"),
        "ein unveraenderter Eintrag traegt eine Marke; A11 sagt: leere Zelle"
    );
}

/// C5.3 (Zuordnungshaelfte): Ein Eintrag, dessen Name auf der Platte zerlegt
/// vorliegt, bekommt seine Marke.
///
/// **Die beiden Seiten stammen aus verschiedenen Quellen**, und das ist der
/// ganze Fall: der Bestand kommt unveraendert aus `readdir`, der Befund kommt
/// aus `gix`, das `core.precomposeUnicode` anwendet und den vorkomponierten
/// Namen liefert. Eine Datei, die als `U+0055 U+0308` auf der Platte steht,
/// heisst im Befund `U+00DC` und traegt eine andere Bytefolge.
///
/// Die zwei Zusicherungen vor dem eigentlichen Vergleich halten genau diese
/// Voraussetzung fest: ohne sie liefe die Probe auch dann gruen, wenn die
/// beiden Seiten laengst dieselbe Schreibweise fuehrten, und sie sagte dann
/// nichts mehr ueber den Fall aus, fuer den sie steht.
#[test]
fn ein_zerlegt_benannter_eintrag_bekommt_seine_marke() {
    /// Der Name, wie er auf der Platte steht: `U` mit Kombinationszeichen.
    const ZERLEGT: &str = "U\u{308}bung.txt";
    /// Derselbe Name vorkomponiert, wie `gix` ihn meldet.
    const VORKOMPONIERT: &str = "\u{dc}bung.txt";

    let ordner = repository("zerlegter-name");
    let pfad = ordner.pfad().to_owned();
    ordner.datei(ZERLEGT, "grund\n");
    git(&pfad, &["add", "-A"]);
    git(
        &pfad,
        &["commit", "-q", "-m", "der zerlegt benannte Eintrag"],
    );
    ordner.datei(ZERLEGT, "geaendert\n");

    let bestand = lesen(&pfad).expect("der Ordner laesst sich nicht lesen");
    assert!(
        bestand.iter().any(|eintrag| eintrag.name == ZERLEGT),
        "die Platte traegt den Namen nicht zerlegt; die Voraussetzung der Probe steht nicht"
    );
    let marken = leser(&pfad)
        .marken(&pfad)
        .expect("die Marken sind unentschieden geblieben");
    assert!(
        marken.iter().any(|(name, _)| name == VORKOMPONIERT),
        "gix meldet den Namen nicht vorkomponiert; die Voraussetzung der Probe steht nicht: {marken:?}"
    );

    let mut modell = Ordnermodell::neu(1);
    modell.anhaengen(bestand);
    modell.abschliessen();
    assert!(
        modell.gitmarken_setzen(1, &marken),
        "kein einziger Befund ist eingetragen worden"
    );

    let index = modell
        .bestand()
        .iter()
        .position(|eintrag| eintrag.name == ZERLEGT)
        .expect("den zerlegt benannten Eintrag gibt es im Bestand nicht");
    assert_eq!(
        modell.gitmarke(index as u32),
        Some(Marke::Geaendert),
        "der zerlegt benannte Eintrag traegt keine Marke"
    );
}

/// Die Liste [`Marke::ALLE`] fuehrt jede Variante von `Marke` genau einmal.
///
/// **Der Uebersetzer haelt sie nicht**: die Feldbreite `[Marke; 5]` zwingt zu
/// fuenf Eintraegen und sagt nichts darueber, welche fuenf. Entscheidbar wird
/// die Frage aus einer zweiten Quelle, und die ist der Quelltext der
/// Aufzaehlung; dieselbe Bauform haelt `Kommando::KENNUNGEN`.
#[test]
fn jede_marke_steht_genau_einmal_in_alle() {
    let varianten = gemeinsam::varianten_der_aufzaehlung("krk-core/src/git/mod.rs", "Marke");
    let in_der_liste: Vec<String> = Marke::ALLE
        .iter()
        .map(|marke| format!("{marke:?}"))
        .collect();
    assert_eq!(
        in_der_liste, varianten,
        "Marke::ALLE fuehrt nicht genau die Varianten der Aufzaehlung, in ihrer Reihenfolge"
    );
}

// ---------------------------------------------------------------------------
// C6: der Ordner ohne Repository
// ---------------------------------------------------------------------------

/// C6.5: Ein Ordner, unter dem bis zur Wurzel kein `.git` liegt, ist eine
/// **entschiedene** Auskunft und kein Fehler.
#[test]
fn ein_ordner_ohne_repository_wird_entschieden_verneint() {
    let ordner = Pruefordner::neu("ohnerepo");
    ordner.datei("gewoehnlich.txt", "nichts mit Git\n");

    let antwort = Gitleser::oeffnen(ordner.pfad());
    assert!(
        matches!(antwort, Oeffnung::KeinRepository),
        "ein Ordner ohne Repository liefert {antwort:?} statt einer entschiedenen Verneinung"
    );
}

// ---------------------------------------------------------------------------
// C7.8 und C7.9: der Deskriptormangel
// ---------------------------------------------------------------------------

/// C7.8, C7.9: Ein Deskriptormangel laesst den Befund unentschieden und
/// entscheidet ihn nicht negativ.
///
/// **Die Probe laeuft im Kind, weil `cargo test` die angehobene
/// Deskriptorgrenze der Anmeldesitzung erbt.** Im selben Prozess gemessen
/// behauptete sie die Zusage, statt sie zu messen: bei tausend freien
/// Deskriptoren geriete `gix` nie in den Mangel. Die Form ist die der
/// Deskriptorproben aus der Runde 10 in `tests/verzeichnis.rs`, und sie ist es
/// ausdruecklich: eine zweite Bauart daneben haette dieselbe Frage zweimal
/// verschieden beantwortet.
///
/// Angelegt und abgeraeumt wird das Pruefrepository vom **Elternteil**:
/// `remove_dir_all` haelt selbst Deskriptoren und koennte unter der abgesenkten
/// Grenze nicht aufraeumen.
#[test]
fn ein_deskriptormangel_laesst_den_gitbefund_unentschieden() {
    let ordner = repository("deskriptormangel");

    let ergebnis = kind_mit_deskriptorgrenze(
        GRENZE_DESKRIPTOREN,
        "kind_liest_unter_abgesenkter_deskriptorgrenze",
        ordner.pfad(),
    );

    assert!(
        ergebnis.status.success(),
        "der Gitleser haelt unter abgesenkter Deskriptorgrenze nicht\n\
         --- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
}

/// Die Kindprobe zu C7.8 und C7.9.
///
/// Zwei Durchgaenge, und der erste ist der eigentliche Gegenstand: ohne einen
/// freien Deskriptor muss `oeffnen` **unentschieden** melden und nicht „kein
/// Repository" — der Ordner ist eines, und dass der Prozess es gerade nicht
/// feststellen kann, ist eine Aussage ueber den Prozess.
///
/// Der zweite Durchgang misst den Vorrat, den der Leser braucht: mit
/// [`FREIE_DESKRIPTOREN`] freien kommen alle vier Auskuenfte zustande. Das ist
/// zugleich die Gegenprobe zum ersten — ohne ihn saehe der erste Durchgang auch
/// dann bestanden aus, wenn `gix` an diesem Ordner grundsaetzlich scheiterte.
#[test]
#[ignore = "Kindprobe, vom Elternteil ueber KRK_KINDPROBE_AUFTRAG gestartet"]
fn kind_liest_unter_abgesenkter_deskriptorgrenze() {
    /// Wie viele Deskriptoren der zweite Durchgang frei laesst.
    ///
    /// Dreissig, also im niedrigen zweistelligen Bereich, und die Zahl ist
    /// gemessen und nicht gesetzt: der Durchgang unten scheitert, sobald der
    /// Leser mehr braucht. Die Zusage aus C7.9 ist damit an dieser Stelle
    /// belegt und nicht behauptet.
    const FREIE_DESKRIPTOREN: usize = 30;

    let Some(ordner) = kindauftrag() else {
        return;
    };

    let mut gehalten = Vec::new();
    while gehalten.len() < 4 * GRENZE_DESKRIPTOREN {
        match std::fs::File::open("/dev/null") {
            Ok(datei) => gehalten.push(datei),
            Err(_) => break,
        }
    }
    let vorrat = gehalten.len();
    assert!(
        vorrat < 4 * GRENZE_DESKRIPTOREN,
        "das Kind bekommt {vorrat} Deskriptoren; die Grenze {GRENZE_DESKRIPTOREN} hat nicht \
         gegriffen, und die Probe messte nichts"
    );
    assert!(
        vorrat > 0,
        "das Kind bekommt gar keinen Deskriptor; gemessen waere der Mangel und nicht die Bauart"
    );

    let ohne = Gitleser::oeffnen(&ordner);
    assert!(
        matches!(ohne, Oeffnung::Unentschieden),
        "ohne freien Deskriptor liefert der Leser {ohne:?}; ein Mangel des eigenen Prozesses \
         darf nicht zur Auskunft „dieser Ordner liegt in keinem Repository\" werden"
    );

    for _ in 0..FREIE_DESKRIPTOREN {
        gehalten.pop();
    }
    let leser = match Gitleser::oeffnen(&ordner) {
        Oeffnung::Offen(leser) => *leser,
        andere => panic!(
            "mit {FREIE_DESKRIPTOREN} freien Deskriptoren liefert der Leser {andere:?}; \
             sein Bedarf liegt hoeher als die Probe zusagt"
        ),
    };
    assert!(leser.kopf().is_some(), "der Kopf bleibt unentschieden");
    assert!(
        leser.verlauf(0, 50).is_some(),
        "der Verlauf bleibt unentschieden"
    );
    assert!(
        leser.marken(&ordner).is_some(),
        "die Marken bleiben unentschieden"
    );
    drop(gehalten);
}

// ---------------------------------------------------------------------------
// Schritt 4: der Gitlauf
// ---------------------------------------------------------------------------

/// Alle Meldungen eines Laufs, bis sein Kanal schliesst.
///
/// `iter()` und nicht `recv_timeout`: der Faden endet von selbst, und ein
/// Zeitmass hier waere eine zweite Zusage neben der, die geprueft wird.
fn meldungen_einsammeln(lauf: &Gitlauf) -> Vec<Gitmeldung> {
    lauf.meldungen().iter().collect()
}

/// Wie eine Meldung heisst, ohne ihre Nutzlast.
///
/// Die Reihenfolgeproben vergleichen Namen und nicht Werte: was in der
/// Verlaufsliste steht, haelt die Probe des Lesers, und hier ist gefragt,
/// **welche** Meldung wann kommt.
fn art(meldung: &Gitmeldung) -> &'static str {
    match meldung {
        Gitmeldung::Kopf(_) => "Kopf",
        Gitmeldung::Verlauf(_) => "Verlauf",
        Gitmeldung::Marken(_) => "Marken",
    }
}

/// C6.1 (Laufhaelfte), A8: Ein ganzer Lauf meldet Kopf, Verlauf und Marken —
/// genau drei Meldungen, in dieser Reihenfolge.
///
/// Die Reihenfolge ist die ihrer gemessenen Kosten und zugleich die, die A8
/// verlangt: Branch und Verlauf stehen schon, waehrend die Markenspalte noch
/// leer ist. Eine Probe, die nur die Menge der Meldungen pruefte, liesse die
/// umgekehrte Reihenfolge durch.
#[test]
fn ein_ganzer_lauf_meldet_kopf_verlauf_und_marken_in_dieser_reihenfolge() {
    let ordner = repository("lauf-ganz");
    ordner.datei("neu.txt", "neu\n");

    let lauf = Gitlauf::starten(ordner.pfad().to_path_buf(), Gitfrage::Ganz, 1);
    let gemeldet = meldungen_einsammeln(&lauf);

    let arten: Vec<&str> = gemeldet.iter().map(art).collect();
    assert_eq!(
        arten,
        vec!["Kopf", "Verlauf", "Marken"],
        "der Lauf meldet nicht genau die drei Auskuenfte in ihrer Reihenfolge"
    );
    assert_eq!(
        gemeldet[0],
        Gitmeldung::Kopf(Kopf::Branch(BRANCH.to_owned())),
        "die Kopfmeldung nennt nicht den Branch"
    );
    let Gitmeldung::Verlauf(verlauf) = &gemeldet[1] else {
        panic!("die zweite Meldung ist kein Verlauf");
    };
    assert_eq!(
        verlauf.len(),
        1,
        "der Verlauf traegt nicht den einen Commit"
    );
    assert_eq!(
        gemeldet[2],
        Gitmeldung::Marken(vec![("neu.txt".to_owned(), Marke::Neu)]),
        "die Markenmeldung traegt nicht den unverfolgten Eintrag"
    );
}

/// C4.2, C4.3: Ein Nachschlag meldet allein den Verlauf, und genau einmal.
///
/// Der Kopf steht schon, die Marken stehen schon; sie ein zweites Mal zu holen
/// hiesse, den teuersten der drei Wege ohne Anlass zu fahren. Und die
/// Nachladeregel aus C4.3 haengt an der Laenge: der Nachschlag hinter dem
/// aeltesten Commit ist leer, und daran erkennt der Rufer, dass nichts mehr
/// folgt.
#[test]
fn ein_nachschlag_meldet_allein_den_verlauf() {
    let ordner = repository("lauf-nachschlag");
    ordner.datei("neu.txt", "neu\n");
    for nummer in 2..=3 {
        ordner.datei("erste.txt", format!("stand {nummer}\n"));
        git(ordner.pfad(), &["add", "erste.txt"]);
        git(
            ordner.pfad(),
            &["commit", "-q", "-m", &format!("Commit {nummer}")],
        );
    }
    let juengster = leser(ordner.pfad())
        .verlauf(0, VERLAUFSSCHRITT)
        .expect("kein Verlauf")[0]
        .id;

    let lauf = Gitlauf::starten(
        ordner.pfad().to_path_buf(),
        Gitfrage::WeitererVerlauf { bereits: 1 },
        2,
    );
    let gemeldet = meldungen_einsammeln(&lauf);

    assert_eq!(
        gemeldet.iter().map(art).collect::<Vec<&str>>(),
        vec!["Verlauf"],
        "der Nachschlag meldet nicht genau eine Verlaufsmeldung"
    );
    let Gitmeldung::Verlauf(verlauf) = &gemeldet[0] else {
        panic!("die Meldung ist kein Verlauf");
    };
    assert_eq!(
        verlauf.len(),
        2,
        "der Nachschlag liefert nicht die zwei aelteren Commits"
    );
    assert!(
        !verlauf.iter().any(|commit| commit.id == juengster),
        "der Nachschlag doppelt den Commit, hinter dem er ansetzt"
    );

    // Und hinter dem aeltesten kommt nichts mehr: die leere Liste ist die
    // entschiedene Antwort, an der C4.3 haengt.
    let am_ende = Gitlauf::starten(
        ordner.pfad().to_path_buf(),
        Gitfrage::WeitererVerlauf { bereits: 3 },
        3,
    );
    assert_eq!(
        meldungen_einsammeln(&am_ende),
        vec![Gitmeldung::Verlauf(Vec::new())],
        "hinter dem aeltesten Commit meldet der Lauf nicht die leere Liste"
    );
}

/// Ein abgebrochener Lauf meldet nichts mehr.
///
/// Geprueft wird ueber [`Gitlauf::abbrechen`], und `Drop` ruft nichts anderes:
/// ein Lauf, dessen [`Gitlauf`] faellt, nimmt denselben Weg, nur dass sein
/// Empfaenger dann mitfaellt und keine Meldung mehr entgegennehmen koennte.
///
/// **Wovon diese Probe abhaengt, und der Satz gehoert dazu:** das
/// Abbruchkennzeichen wird auf dem Hauptfaden gesetzt, waehrend der Arbeitsfaden
/// erst noch anlaeuft. Sie misst damit, dass die Pruefung **vor** der ersten
/// Einheit steht und nicht erst danach — dieselbe Form und dieselbe
/// Voraussetzung wie `der_abbruch_greift_in_einem_ordner_ohne_unterordner` in
/// `tests/verzeichnis.rs`. Ein Faden, der zwischen `starten` und `abbrechen`
/// bereits eine Auskunft fertig haette, liesse sie rot werden; das Anlegen des
/// Fadens allein kostet mehr als der Speicherzugriff daneben.
#[test]
fn ein_abgebrochener_lauf_meldet_nichts_mehr() {
    let ordner = repository("lauf-abbruch");
    ordner.datei("neu.txt", "neu\n");

    // Kontrollauf: derselbe Ordner meldet ohne Abbruch alle drei Auskuenfte.
    let ungestoert = Gitlauf::starten(ordner.pfad().to_path_buf(), Gitfrage::Ganz, 4);
    assert_eq!(
        meldungen_einsammeln(&ungestoert).len(),
        3,
        "ohne Abbruch meldet der Lauf nicht alle drei Auskuenfte"
    );

    let lauf = Gitlauf::starten(ordner.pfad().to_path_buf(), Gitfrage::Ganz, 5);
    lauf.abbrechen();
    assert_eq!(
        meldungen_einsammeln(&lauf),
        Vec::new(),
        "ein abgebrochener Lauf meldet weiter; ein ausbleibender Befund heisst \
         unentschieden, ein gemeldeter waere eine Aussage"
    );
}

/// C6.1 (Laufhaelfte), E5: Ein Ordner ohne Repository meldet
/// [`Kopf::KeinRepository`] und danach nichts.
///
/// Die eine entschiedene Verneinung des Laufs. Sie steht **vor** Verlauf und
/// Marken und nicht an ihrer Stelle: ein Ordner ohne Repository hat keinen
/// Verlauf und keine Marken, und das ist keine Auskunft, sondern die Folge der
/// ersten.
#[test]
fn ein_ordner_ohne_repository_meldet_kein_repository_und_danach_nichts() {
    let ordner = Pruefordner::neu("lauf-ohnerepo");
    ordner.datei("gewoehnlich.txt", "nichts mit Git\n");

    let lauf = Gitlauf::starten(ordner.pfad().to_path_buf(), Gitfrage::Ganz, 6);
    assert_eq!(
        meldungen_einsammeln(&lauf),
        vec![Gitmeldung::Kopf(Kopf::KeinRepository)],
        "ein Ordner ohne Repository meldet nicht genau die eine entschiedene Verneinung"
    );
}

/// C7.1: Ausserhalb von `krk-core/src/git/` fragt kein ausgelieferter Code den
/// Statusweg des Gitlesers.
///
/// **Warum das die pruefbare Gestalt von C7.1 ist.** „Keine Statusabfrage laeuft
/// auf dem Hauptfaden" ist am Quelltext nicht unmittelbar zu entscheiden — auf
/// welchem Faden eine Zeile laeuft, sagt keine Nadel. Entscheidbar ist die
/// Frage dahinter: gibt es ueberhaupt einen zweiten Weg an den Status, neben
/// dem Kanal? [`Gitleser::marken`] ist der teure der vier Wege — 12 bis 164 ms
/// gemessen —, und wenn ihn allein `git/lauf.rs` ruft, kann ihn niemand sonst
/// auf den Hauptfaden legen.
///
/// Gezaehlt werden **Aufrufstellen** und keine Erklaerungen: die Zahl der
/// Erklaerungen ist eins, und das ist hier nicht die Frage.
/// [`gemeinsam::aufrufstellen`] zaehlt jede Empfaengerform und jeden Pfad und
/// laesst einen laengeren Bezeichner aus, der auf dieselben Zeichen endet —
/// `gueltige_marken(` in `krk-ui/src/leistenmodell.rs` ist kein Aufruf von
/// `marken`.
///
/// Gelesen wird unter `crates/*/src`, also der Code, der ausgeliefert wird.
/// Diese Datei selbst faellt damit heraus, und das ist gewollt: eine Probe, die
/// den Leser prueft, ruft ihn, und sie laeuft auf keinem Zeichendurchgang. Es
/// ist dieselbe Grenze, die der Nutzer am 260830 fuer
/// `git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen` gewaehlt
/// hat.
///
/// # Was diese Nadel nicht sieht
///
/// Ein `use … as anders;`, das den Namen wechselt, und ein
/// `#[cfg(test)]`-Modul unter `src/`, das mitgezaehlt wird, obwohl es nicht
/// ausgeliefert wird. Der Kopf von `tests/baum.rs` sagt, warum keine Suche im
/// Quelltext restlos dicht ist.
#[test]
fn keine_statusabfrage_steht_ausserhalb_des_gitmoduls() {
    let nadel = concat!("mar", "ken");
    let baum = gemeinsam::quelldateien();

    // Gegenprobe zuerst: findet die Nadel ueberhaupt etwas, wo der eine Rufer
    // steht? Ohne sie bestuende die Probe auch nach einer Umbenennung.
    let (_, lauf) = baum
        .iter()
        .find(|(name, _)| name == "krk-core/src/git/lauf.rs")
        .expect("krk-core/src/git/lauf.rs steht nicht mehr im Baum");
    assert!(
        aufrufstellen(lauf, nadel) > 0,
        "der Gitlauf ruft den Statusweg nicht mehr unter diesem Namen; die Nadel findet \
         nichts und die Probe belegt nichts"
    );

    let fremde: Vec<(String, usize)> = baum
        .iter()
        .filter(|(name, _)| name.contains("/src/") && !name.starts_with("krk-core/src/git/"))
        .map(|(name, inhalt)| (name.clone(), aufrufstellen(inhalt, nadel)))
        .filter(|(_, zahl)| *zahl > 0)
        .collect();
    assert!(
        fremde.is_empty(),
        "ausserhalb von krk-core/src/git/ fragt ausgelieferter Code den Statusweg unmittelbar: \
         {fremde:?}; der eine Weg herein ist der Kanal des Gitlaufs"
    );
}
