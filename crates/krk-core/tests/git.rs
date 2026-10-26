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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

/// Das Abbruchkennzeichen eines Laufs, der **nicht** abgebrochen ist.
///
/// Es steht einmal da und nicht je Probe: `Gitleser::marken` nimmt seit dem
/// 260831 eines entgegen, und die Proben, die den Abbruch nicht messen,
/// reichen dasselbe stehende `false` herein. Wer den Abbruch misst, legt sich
/// sein eigenes an; die Probe
/// `ein_gesetztes_abbruchkennzeichen_bricht_den_markenlauf_ab` tut das.
static LAEUFT: AtomicBool = AtomicBool::new(false);

// ---------------------------------------------------------------------------
// Pruefrepositorys anlegen
// ---------------------------------------------------------------------------

/// Der eine `git`-Aufruf dieser Datei; die drei Huellen darunter unterscheiden
/// sich allein darin, was sie mit dem Ergebnis anfangen und ob sie eine Zeit
/// vorgeben.
///
/// `zeit` ist der Zeitstempel in Sekunden seit 1970, den Autor und Committer
/// gemeinsam bekommen. `None` heisst „die Uhr des Geraets"; das ist der
/// Normalfall, und genau eine Probe braucht die Vorgabe, naemlich die ueber die
/// Reihenfolge des Verlaufs.
fn git_roh(ordner: &Path, zeit: Option<i64>, argumente: &[&str]) -> std::process::Output {
    let mut befehl = Command::new("/usr/bin/git");
    befehl
        .current_dir(ordner)
        .env("HOME", ordner)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_AUTHOR_NAME", "Probe")
        .env("GIT_AUTHOR_EMAIL", "probe@example.org")
        .env("GIT_COMMITTER_NAME", "Probe")
        .env("GIT_COMMITTER_EMAIL", "probe@example.org");
    if let Some(sekunden) = zeit {
        let stempel = format!("@{sekunden} +0000");
        befehl
            .env("GIT_AUTHOR_DATE", &stempel)
            .env("GIT_COMMITTER_DATE", &stempel);
    }
    befehl
        .args(argumente)
        .output()
        .expect("git laesst sich nicht starten")
}

/// Haelt, dass der Aufruf gelungen ist, und gibt seine Ausgabe zurueck.
fn gelungen(ordner: &Path, argumente: &[&str], ergebnis: &std::process::Output) -> String {
    assert!(
        ergebnis.status.success(),
        "git {argumente:?} in {} ist gescheitert\n--- stdout ---\n{}\n--- stderr ---\n{}",
        ordner.display(),
        String::from_utf8_lossy(&ergebnis.stdout),
        String::from_utf8_lossy(&ergebnis.stderr)
    );
    String::from_utf8_lossy(&ergebnis.stdout).trim().to_owned()
}

/// Ruft `git` im genannten Ordner und haelt, dass der Aufruf gelungen ist.
fn git(ordner: &Path, argumente: &[&str]) -> String {
    let ergebnis = git_roh(ordner, None, argumente);
    gelungen(ordner, argumente, &ergebnis)
}

/// Dasselbe mit vorgegebener Zeit fuer Autor und Committer.
fn git_zur_zeit(ordner: &Path, sekunden: i64, argumente: &[&str]) -> String {
    let ergebnis = git_roh(ordner, Some(sekunden), argumente);
    gelungen(ordner, argumente, &ergebnis)
}

/// Dasselbe, aber ein Fehlschlag ist erlaubt; die Ausgabe faellt weg.
///
/// Genau ein Aufruf braucht das, naemlich der `merge`, der den Konflikt
/// herstellt: er endet mit 1, und das ist sein Zweck.
fn git_darf_scheitern(ordner: &Path, argumente: &[&str]) {
    let _ = git_roh(ordner, None, argumente);
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
        .marken(ordner, &LAEUFT)
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

/// C4, M1 der Durchsicht: die Verlaufsliste steht nach der Commit-Zeit und
/// nicht nach der Reihenfolge des Graphen.
///
/// Das Pruefrepository ist eigens so gebaut, dass die beiden Reihenfolgen
/// auseinandergehen — in einer linearen Kette fallen sie zusammen, und eine
/// Probe daran belegte nichts. Zwei Zweige mit **verschraenkten** Zeiten und
/// verschieden vielen Commits, eine Zusammenfuehrung darueber:
///
/// ```text
/// c0 ── h1 ── h2 ── h3 ─────────── h4 ── m
///   \                                  /
///    └── z1 ── z2 ── z3 ──────────────┘
///
/// Zeit:  c0=+0  h1=+10 h2=+20 h3=+30
///        z1=+40 z2=+50 z3=+60 h4=+70 m=+80
/// ```
///
/// Breitenzuerst greift abwechselnd in beide Zweige und liefert
/// `m, h4, z3, h3, z2, h2, z1, h1, c0`; nach der Zeit heisst es
/// `m, h4, z3, z2, z1, h3, h2, h1, c0`. Die erste Abweichung steht an der
/// vierten Stelle.
///
/// **Zwei Zusicherungen, und die zweite ist der Massstab von aussen:** die
/// Kurzbeschreibungen stehen in der erwarteten Reihenfolge, und die Objektnamen
/// stehen in derselben Reihenfolge wie die von `git rev-list HEAD`, das seine
/// Liste ab Werk nach der Commit-Zeit ordnet. Jeder Commit traegt dafuer eine
/// eigene Sekunde; bei gleichen Zeiten entschiede die Reihenfolge der
/// Warteschlange, und die ist zwischen `git` und `gix` nichts Zugesagtes.
#[test]
fn der_verlauf_steht_nach_der_zeit_und_nicht_nach_dem_graphen() {
    let ordner = repository("zeitordnung");
    let pfad = ordner.pfad().to_owned();

    // Die Zeit des ersten Commits ist der Nullpunkt; alles Weitere liegt
    // dahinter, damit die Kette nicht rueckwaerts laeuft.
    let basis: i64 = git(&pfad, &["log", "-1", "--format=%ct"])
        .parse()
        .expect("die Zeit des ersten Commits ist keine Zahl");

    let commit = |zweig_datei: &str, nummer: u32, versatz: i64, nachricht: &str| {
        ordner.datei(zweig_datei, format!("stand {nummer}\n"));
        git(&pfad, &["add", "-A"]);
        git_zur_zeit(&pfad, basis + versatz, &["commit", "-q", "-m", nachricht]);
    };

    git(&pfad, &["branch", "zweig"]);
    commit("haupt.txt", 1, 10, "haupt 1");
    commit("haupt.txt", 2, 20, "haupt 2");
    commit("haupt.txt", 3, 30, "haupt 3");
    git(&pfad, &["checkout", "-q", "zweig"]);
    commit("zweig.txt", 1, 40, "zweig 1");
    commit("zweig.txt", 2, 50, "zweig 2");
    commit("zweig.txt", 3, 60, "zweig 3");
    git(&pfad, &["checkout", "-q", BRANCH]);
    commit("haupt.txt", 4, 70, "haupt 4");
    git_zur_zeit(
        &pfad,
        basis + 80,
        &[
            "merge",
            "-q",
            "--no-ff",
            "-m",
            "die Zusammenfuehrung",
            "zweig",
        ],
    );

    let leser = leser(&pfad);
    let verlauf = leser
        .verlauf(0, VERLAUFSSCHRITT)
        .expect("der Verlauf ist unentschieden geblieben");

    let gesehen: Vec<&str> = verlauf
        .iter()
        .map(|commit| commit.kurzbeschreibung.as_str())
        .collect();
    assert_eq!(
        gesehen,
        vec![
            "die Zusammenfuehrung",
            "haupt 4",
            "zweig 3",
            "zweig 2",
            "zweig 1",
            "haupt 3",
            "haupt 2",
            "haupt 1",
            "der erste Commit",
        ],
        "die Liste steht nicht nach der Commit-Zeit"
    );

    let soll: Vec<String> = git(&pfad, &["rev-list", "HEAD"])
        .lines()
        .map(str::to_owned)
        .collect();
    let ist: Vec<String> = verlauf.iter().map(|commit| commit.id.to_string()).collect();
    assert_eq!(
        ist, soll,
        "die Reihenfolge weicht von der von `git rev-list HEAD` ab"
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
        .marken(&pfad, &LAEUFT)
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
/// **Der Uebersetzer haelt sie nicht**: die Feldbreite `[Marke; 6]` zwingt zu
/// sechs Eintraegen und sagt nichts darueber, welche sechs. Entscheidbar wird
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

    kind_mit_deskriptorgrenze(
        "der Gitleser haelt unter abgesenkter Deskriptorgrenze nicht",
        GRENZE_DESKRIPTOREN,
        "kind_liest_unter_abgesenkter_deskriptorgrenze",
        ordner.pfad(),
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
        leser.marken(&ordner, &LAEUFT).is_some(),
        "die Marken bleiben unentschieden"
    );
    drop(gehalten);
}

/// M2 der Durchsicht: ein gesetztes Abbruchkennzeichen bricht den Markenlauf
/// ab, statt ihn zu Ende laufen zu lassen.
///
/// **Die Gegenprobe steht voran und traegt die Probe:** derselbe Leser, derselbe
/// Ordner, ein Kennzeichen auf `false` — dabei kommt eine nicht leere Liste
/// heraus. Ohne sie bestuende die Probe auch dann, wenn `marken` an diesem
/// Ordner grundsaetzlich `None` lieferte.
///
/// `None` und keine halbe Liste: eine halbe waere von „diese Eintraege sind
/// unveraendert" nicht zu unterscheiden.
#[test]
fn ein_gesetztes_abbruchkennzeichen_bricht_den_markenlauf_ab() {
    let ordner = repository("markenabbruch");
    let pfad = ordner.pfad().to_owned();
    ordner.datei("unverfolgt.txt", "neu\n");

    let leser = leser(&pfad);

    let laeuft = AtomicBool::new(false);
    let voll = leser
        .marken(&pfad, &laeuft)
        .expect("die Marken sind ohne Abbruch unentschieden geblieben");
    assert!(
        !voll.is_empty(),
        "Gegenprobe: ohne Abbruch traegt der Ordner mindestens eine Marke"
    );

    let aufgegeben = AtomicBool::new(true);
    assert_eq!(
        leser.marken(&pfad, &aufgegeben),
        None,
        "ein aufgegebener Lauf liest den Statusstrom zu Ende, statt abzubrechen"
    );
}

/// M3 der Durchsicht: ein Repository, das gefunden wird und sich nicht oeffnen
/// laesst, bleibt **unentschieden** und wird nicht verneint.
///
/// Die Konfiguration wird unlesbar **im Sinne des Zerlegers** gemacht und nicht
/// im Sinne der Rechte: `gix::discover` liefert an einer `.git/config` mit
/// `chmod 000` gemessen `Ok`, uebergeht die Datei also, waehrend eine Datei, die
/// kein INI ist, `discover::Error::Open(Config(…))` liefert — genau die
/// Variante, an der die Fallunterscheidung haengt.
///
/// **Die Gegenprobe steht voran:** derselbe Ordner liefert vor dem Zerschlagen
/// einen Leser. Ohne sie bestuende die Probe auch dann, wenn der Ordner nie
/// eines gewesen waere.
#[test]
fn ein_nicht_zu_oeffnendes_repository_bleibt_unentschieden() {
    let ordner = repository("kaputtekonfiguration");
    let pfad = ordner.pfad().to_owned();

    let vorher = Gitleser::oeffnen(&pfad);
    assert!(
        matches!(vorher, Oeffnung::Offen(_)),
        "Gegenprobe: der Ordner liefert vor dem Zerschlagen {vorher:?} statt eines Lesers"
    );

    std::fs::write(pfad.join(".git/config"), "das ist kein ini\n[[[\n")
        .expect("die Konfiguration laesst sich nicht ueberschreiben");

    let antwort = Gitleser::oeffnen(&pfad);
    assert!(
        matches!(antwort, Oeffnung::Unentschieden),
        "ein gefundenes, nicht zu oeffnendes Repository liefert {antwort:?}; \
         „kein Repository\" waere eine entschiedene Verneinung ueber einen Ordner, \
         der eines traegt"
    );
}

// ---------------------------------------------------------------------------
// Eine Datei, die nur in der Cloud liegt
// ---------------------------------------------------------------------------
//
// Ein echter Platzhalter (`SF_DATALESS`) laesst sich ohne File-Provider-Domaene
// in keiner Probe herstellen. Die Proben halten deshalb den Weg vom Kennzeichen
// zur Marke mit eingespritzter Frage: [`wolke_ist_platzhalter`] sagt ja fuer
// jeden Dateinamen, der `wolke` enthaelt. Die Deutung des Kennzeichens selbst
// haelt die Probe an `verzeichnis::ist_platzhalter`.
//
// **`chmod 000` macht jedes Lesen sichtbar**: es wuerde scheitern, und ein
// Fehlschlag fuehrt den Markenlauf zu `None`. Ein `Some(…)` belegt damit, dass
// nichts gelesen wurde. Jede Probe haelt die Sperre ueber
// `rechtesperre_haelt_oder_abbruch`, damit ein Lauf als root nicht gruen
// misst, was er nicht messen kann.

/// Die Platzhalterfrage der Proben: ja fuer jeden Dateinamen mit `wolke`.
fn wolke_ist_platzhalter(pfad: &Path, _angaben: &std::fs::Metadata) -> bool {
    pfad.file_name()
        .is_some_and(|name| name.to_string_lossy().contains("wolke"))
}

/// Die Marken eines Ordners unter der Platzhalterfrage der Proben, sortiert.
fn wolkenmarken(leser: &Gitleser, ordner: &Path) -> Option<Vec<(String, Marke)>> {
    let mut gefunden = leser.marken_mit(ordner, &LAEUFT, &wolke_ist_platzhalter)?;
    gefunden.sort_by(|links, rechts| links.0.cmp(&rechts.0));
    Some(gefunden)
}

/// Setzt die mtime von `pfad` zehn Sekunden zurueck.
///
/// Vor dem `git add` gerufen, haelt das den Eintrag aus der Racy-Pruefung: der
/// Zeitstempel des Index liegt dann sicher nach der mtime der Datei, und `gix`
/// fragt den Vergleich allein wegen der Aenderung, die die Probe herstellt.
fn mtime_zuruecksetzen(pfad: &Path) {
    let frueher = SystemTime::now() - Duration::from_secs(10);
    std::fs::File::options()
        .write(true)
        .open(pfad)
        .expect("die Datei laesst sich zum Setzen der Zeit nicht oeffnen")
        .set_modified(frueher)
        .expect("die mtime laesst sich nicht setzen");
}

/// Setzt die mtime von `pfad` auf jetzt, ohne den Inhalt anzufassen.
fn mtime_auffrischen(pfad: &Path) {
    std::fs::File::options()
        .write(true)
        .open(pfad)
        .expect("die Datei laesst sich zum Setzen der Zeit nicht oeffnen")
        .set_modified(SystemTime::now())
        .expect("die mtime laesst sich nicht setzen");
}

/// Wartet, bis die Uhr eine Sekunde ueber die ctime von `pfad` hinaus steht.
///
/// `gix` vergleicht Zeiten ab Werk ohne Nanosekunden (`gitoxide.core.useNsec`
/// steht auf `false`). Ein `chmod` in derselben Sekunde wie das `git add`
/// hinterliesse eine ctime, die zum Index passt; `gix` fragte den Vergleich
/// dann gar nicht, und die Gegenprobe saehe nichts.
fn sekunde_nach_ctime_abwarten(pfad: &Path) {
    use std::os::unix::fs::MetadataExt as _;
    let ctime = std::fs::symlink_metadata(pfad)
        .expect("die Datei laesst sich nicht befragen")
        .ctime();
    while SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("die Uhr steht vor 1970")
        .as_secs()
        <= u64::try_from(ctime).expect("eine ctime vor 1970")
    {
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Entzieht jedes Recht an `pfad` und haelt, dass Lesen danach scheitert.
fn lesen_sperren(pfad: &Path) {
    use std::os::unix::fs::PermissionsExt as _;
    std::fs::set_permissions(pfad, std::fs::Permissions::from_mode(0o000))
        .expect("die Rechte lassen sich nicht entziehen");
    gemeinsam::rechtesperre_haelt_oder_abbruch(
        "der Markenlauf liest keine ausgelagerte Datei",
        std::fs::File::open(pfad).is_err(),
    );
}

/// Ein Repository, das die genannten Dateien verfolgt, jede mit einem Eintrag,
/// der nicht racy ist.
///
/// Ein Name mit `/` legt den Ordner davor an. Jede Datei traegt `grund\n`,
/// sieben Bytes, damit die Proben gleich lang oder anders lang ersetzen
/// koennen.
fn verfolgt(zweck: &str, namen: &[&str]) -> Pruefordner {
    let ordner = repository(zweck);
    for name in namen {
        if let Some((vorne, _)) = name.rsplit_once('/') {
            let _ = ordner.ordner(vorne);
        }
        let pfad = ordner.datei(name, "grund!\n");
        mtime_zuruecksetzen(&pfad);
    }
    git(ordner.pfad(), &["add", "-A"]);
    git(
        ordner.pfad(),
        &["commit", "-q", "-m", "der verfolgte Stand"],
    );
    ordner
}

/// (1) Eine ausgelagerte Datei, an der allein ctime gestiegen ist, traegt
/// keine Marke und wird nicht gelesen (E-Stat, Moeglichkeit 2).
///
/// `chmod 000` hebt ctime und laesst mtime und Groesse stehen, genau wie das
/// Auslagern bei der Messung vom 261026-1208.
///
/// **Die Gegenprobe steht voran:** `marken` mit der Frage des Betriebs haelt
/// die Datei fuer lokal, `gix` will sie lesen und scheitert an der Sperre, und
/// der Lauf liefert `None`. Ohne sie bestuende die Probe auch dann, wenn `gix`
/// den Vergleich an dieser Datei gar nicht fragte.
#[test]
fn eine_ausgelagerte_datei_mit_allein_neuer_ctime_bleibt_ungelesen_und_ohne_marke() {
    let ordner = verfolgt("wolke-ctime", &["wolke.txt"]);
    let pfad = ordner.pfad().to_owned();
    let datei = ordner.unter("wolke.txt");
    sekunde_nach_ctime_abwarten(&datei);
    lesen_sperren(&datei);

    let leser = leser(&pfad);
    assert_eq!(
        leser.marken(&pfad, &LAEUFT),
        None,
        "Gegenprobe: ohne Platzhalterfrage liest gix die gesperrte Datei nicht; \
         die Probe belegt nicht, dass der Vergleich ueberhaupt gefragt wird"
    );

    let gefunden = wolkenmarken(&leser, &pfad)
        .expect("der Markenlauf hat die ausgelagerte Datei gelesen oder ist gescheitert");
    assert!(
        !gefunden.iter().any(|(name, _)| name == "wolke.txt"),
        "eine ausgelagerte Datei, an der allein ctime gestiegen ist, traegt eine Marke: \
         {gefunden:?}"
    );
}

/// (2) Eine ausgelagerte Datei mit neuem Inhalt gleicher Laenge und neuer
/// mtime traegt `?` und wird nicht gelesen.
#[test]
fn eine_ausgelagerte_datei_gleicher_groesse_mit_neuer_mtime_ist_unbekannt() {
    let ordner = verfolgt("wolke-gleich", &["wolke.txt"]);
    let pfad = ordner.pfad().to_owned();
    let datei = ordner.datei("wolke.txt", "nebel!\n");
    lesen_sperren(&datei);

    let leser = leser(&pfad);
    assert_eq!(
        leser.marken(&pfad, &LAEUFT),
        None,
        "Gegenprobe: ohne Platzhalterfrage liest gix die gesperrte Datei nicht"
    );

    assert_eq!(
        wolkenmarken(&leser, &pfad),
        Some(vec![("wolke.txt".to_owned(), Marke::Unbekannt)]),
        "eine ausgelagerte Datei gleicher Groesse mit neuer mtime traegt nicht „unbekannt\""
    );
}

/// (3) Eine ausgelagerte Datei anderer Laenge traegt `M`, ohne gelesen zu
/// werden.
///
/// **Die Gegenprobe steht voran:** `FastEq` urteilt ueber dieselbe gesperrte
/// Datei ebenso „geaendert", ohne zu lesen. Die Groessenregel des
/// Vergleichs ist also die von `gix` und keine neue Annahme.
#[test]
fn eine_ausgelagerte_datei_anderer_groesse_ist_geaendert() {
    let ordner = verfolgt("wolke-anders", &["wolke.txt"]);
    let pfad = ordner.pfad().to_owned();
    let datei = ordner.datei("wolke.txt", "eine laengere Wolke\n");
    lesen_sperren(&datei);

    let leser = leser(&pfad);
    assert_eq!(
        marken_sortiert(&leser, &pfad),
        vec![("wolke.txt".to_owned(), Marke::Geaendert)],
        "Gegenprobe: FastEq urteilt an einer Datei anderer Groesse nicht ohne Lesen"
    );

    assert_eq!(
        wolkenmarken(&leser, &pfad),
        Some(vec![("wolke.txt".to_owned(), Marke::Geaendert)]),
        "eine ausgelagerte Datei anderer Groesse traegt nicht „geaendert\""
    );
}

/// (4) Ein Ordner mit einer unbekannten Datei traegt `?`, solange darunter
/// nichts Gewichtigeres als „vorgemerkt" bekannt ist, und `M`, sobald eine
/// geaenderte Datei daneben liegt.
///
/// **Die Gegenprobe steht jeweils voran:** im Ordner selbst stehen beide
/// Dateien mit ihrer eigenen Marke. Ohne sie bestuende die Probe auch dann,
/// wenn eine der beiden gar keine bekaeme.
#[test]
fn ein_ordner_mit_einer_unbekannten_datei_traegt_den_hoeheren_rang() {
    // Neben einer vorgemerkten Datei.
    let ordner = verfolgt(
        "wolke-ordner-vorgemerkt",
        &["unter/wolke.txt", "unter/vorgemerkt.txt"],
    );
    let pfad = ordner.pfad().to_owned();
    let unter = ordner.unter("unter");
    ordner.datei("unter/vorgemerkt.txt", "vorgemerkt\n");
    git(&pfad, &["add", "unter/vorgemerkt.txt"]);
    let datei = ordner.datei("unter/wolke.txt", "nebel!\n");
    lesen_sperren(&datei);

    let erster = leser(&pfad);
    assert_eq!(
        wolkenmarken(&erster, &unter),
        Some(vec![
            ("vorgemerkt.txt".to_owned(), Marke::Vorgemerkt),
            ("wolke.txt".to_owned(), Marke::Unbekannt),
        ]),
        "Gegenprobe: im Ordner tragen die zwei Dateien nicht ihre eigenen Marken"
    );
    let oben = wolkenmarken(&erster, &pfad).expect("der Markenlauf ist gescheitert");
    assert!(
        oben.contains(&("unter".to_owned(), Marke::Unbekannt)),
        "ein Ordner mit einer unbekannten und einer vorgemerkten Datei traegt nicht „?\": \
         {oben:?}"
    );

    // Neben einer geaenderten Datei.
    let ordner = verfolgt(
        "wolke-ordner-geaendert",
        &["unter/wolke.txt", "unter/geaendert.txt"],
    );
    let pfad = ordner.pfad().to_owned();
    let unter = ordner.unter("unter");
    ordner.datei("unter/geaendert.txt", "geaendert\n");
    let datei = ordner.datei("unter/wolke.txt", "nebel!\n");
    lesen_sperren(&datei);

    let zweiter = leser(&pfad);
    assert_eq!(
        wolkenmarken(&zweiter, &unter),
        Some(vec![
            ("geaendert.txt".to_owned(), Marke::Geaendert),
            ("wolke.txt".to_owned(), Marke::Unbekannt),
        ]),
        "Gegenprobe: im Ordner tragen die zwei Dateien nicht ihre eigenen Marken"
    );
    let oben = wolkenmarken(&zweiter, &pfad).expect("der Markenlauf ist gescheitert");
    assert!(
        oben.contains(&("unter".to_owned(), Marke::Geaendert)),
        "ein Ordner mit einer unbekannten und einer geaenderten Datei traegt nicht „M\": \
         {oben:?}"
    );
}

/// (5) Ein Abbruch mitten im Lauf erreicht `gix` selbst: der Lauf liefert
/// `None`, und der Vergleich wird danach fuer keine weitere Datei gefragt.
///
/// Gezaehlt werden Rufe der Platzhalterfrage und nicht Zeit, weil Zeitproben
/// auf diesem Geraet unzuverlaessig sind. Jede der dreihundert Dateien bekommt
/// nach dem Commit eine neue mtime, also fragt `gix` den Vergleich fuer jede.
///
/// **Die Gegenprobe steht voran:** dieselbe Frage, ohne das Kennzeichen zu
/// setzen, wird genau dreihundertmal gerufen, und der Lauf liefert `Some`.
#[test]
fn ein_abbruch_mitten_im_markenlauf_erreicht_gix() {
    const ZAHL: usize = 300;

    let namen: Vec<String> = (0..ZAHL)
        .map(|nummer| format!("datei-{nummer:03}.txt"))
        .collect();
    let namen_als_text: Vec<&str> = namen.iter().map(String::as_str).collect();
    let ordner = verfolgt("markenabbruch-mitten", &namen_als_text);
    let pfad = ordner.pfad().to_owned();
    for name in &namen {
        mtime_auffrischen(&ordner.unter(name));
    }
    let leser = leser(&pfad);

    let gezaehlt = |rufe: &AtomicUsize, pfad: &Path| {
        if pfad
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("datei-"))
        {
            rufe.fetch_add(1, Ordering::SeqCst);
        }
    };

    let rufe = AtomicUsize::new(0);
    let laeuft = AtomicBool::new(false);
    let ohne_abbruch = leser.marken_mit(&pfad, &laeuft, &|pfad, _| {
        gezaehlt(&rufe, pfad);
        false
    });
    assert!(
        ohne_abbruch.is_some(),
        "Gegenprobe: ohne Abbruch bleibt der Markenlauf unentschieden"
    );
    assert_eq!(
        rufe.load(Ordering::SeqCst),
        ZAHL,
        "Gegenprobe: gix fragt den Vergleich nicht fuer jede Datei mit neuer mtime"
    );

    let rufe = AtomicUsize::new(0);
    let abbruch = AtomicBool::new(false);
    let mit_abbruch = leser.marken_mit(&pfad, &abbruch, &|pfad, _| {
        gezaehlt(&rufe, pfad);
        abbruch.store(true, Ordering::SeqCst);
        false
    });
    assert_eq!(
        mit_abbruch, None,
        "ein mitten im Lauf gesetztes Abbruchkennzeichen liefert eine Liste statt `None`"
    );
    let gerufen = rufe.load(Ordering::SeqCst);
    assert!(
        gerufen < ZAHL,
        "nach dem Abbruch hat gix den Vergleich fuer alle {gerufen} Dateien gefragt; \
         das Kennzeichen erreicht gix nicht"
    );
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
