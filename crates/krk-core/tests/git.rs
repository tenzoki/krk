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

mod gemeinsam;

use std::path::Path;
use std::process::Command;

use gemeinsam::{Pruefordner, kind_mit_deskriptorgrenze, kindauftrag};
use krk_core::git::leser::{Gitleser, Oeffnung};
use krk_core::git::texte::zusammenfassung;
use krk_core::git::{Kopf, Marke};

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
        leser.verlauf(None, 50).expect("kein Verlauf").len(),
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
        leser.verlauf(None, 50),
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
    let erste = leser.verlauf(None, 50).expect("kein Verlauf");
    assert_eq!(erste.len(), 50, "der erste Aufruf liefert nicht fuenfzig");

    let letzter = erste.last().expect("die Liste ist leer").id;
    let weitere = leser.verlauf(Some(letzter), 50).expect("kein Nachschlag");
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
    let verlauf = leser.verlauf(None, 50).expect("kein Verlauf");
    assert_eq!(verlauf.len(), 3, "drei Commits liefern nicht drei Zeilen");
    assert!(
        verlauf.len() < 50,
        "die Laenge meldet nicht, dass nichts mehr folgt"
    );

    let letzter = verlauf.last().expect("die Liste ist leer").id;
    assert_eq!(
        leser.verlauf(Some(letzter), 50),
        Some(Vec::new()),
        "hinter dem aeltesten Commit kommt noch etwas"
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
        leser.verlauf(None, 50).is_some(),
        "der Verlauf bleibt unentschieden"
    );
    assert!(
        leser.marken(&ordner).is_some(),
        "die Marken bleiben unentschieden"
    );
    drop(gehalten);
}
