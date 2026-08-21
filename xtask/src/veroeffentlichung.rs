//! Die Veroeffentlichung: der Weg vom beglaubigten Buendel zur oeffentlichen
//! Releaseseite.
//!
//! ```text
//! cargo xtask veroeffentlichen 0.5.6
//! ```
//!
//! **Wozu dieser Weg da ist.** Ein beglaubigtes `target/KRK.app` ist bis hier
//! nur lokal zu haben. Dieser Weg packt es zu einem weitergebbaren Zip, schiebt
//! den Stand samt Tag auf die Gegenseite und haengt das Zip an eine
//! oeffentliche Releaseseite, deren Text dem Nutzer sagt, wie er installiert,
//! ohne seine Daten zu verlieren. Er setzt hinter der Beglaubigung an, so wie
//! [`crate::beglaubigung`] hinter der Signierung ansetzt, und ist aus derselben
//! Gestalt gebaut: ein [`ausfuehren`] fuer den eigenstaendigen Aufruf, und
//! daneben ein [`veroeffentlichen`], das [`crate::release`] als achte Station
//! nimmt. Beide Rufer teilen einen Rumpf, und ihr einziger Unterschied steht
//! als [`Tagfrage`] da: der eigenstaendige Weg fragt selbst, ob `v<zahl>` auf
//! HEAD steht, weil vor ihm keine Station stand.
//!
//! **Er baut nichts.** Kein Uebersetzungslauf, kein `lipo`, keine Montage,
//! keine Signierung. Findet er kein Buendel, bricht er ab und nennt den ganzen
//! Weg.
//!
//! **Er reicht nichts ein.** Die Einreichung bei Apple und das Anheften des
//! Tickets stehen in [`crate::beglaubigung`] und dort allein. Dieser Weg fragt
//! bloss nach, ob das Ticket schon am Buendel haengt, und er fragt es an einer
//! Datei und nicht bei einem Dienst.
//!
//! **Und er prueft den Arbeitsbaum nicht.** Ob eine verfolgte Datei geaendert
//! ist, entscheidet Station 1 von [`crate::release`]; sie steht am Anfang der
//! Auslieferungskette und nicht hier. Was dieser Weg veroeffentlicht, ist das
//! Buendel, das unter `target/` liegt, und der Stand, auf dem HEAD steht. Wer
//! ihn eigenstaendig ruft, uebernimmt damit dieselbe Verantwortung wie beim
//! Nur-Beglaubigungsweg: es ist nicht gesagt, dass das Buendel aus dem Stand
//! gebaut wurde, den er gleich schiebt.
//!
//! **`gh` wird ueber den Suchpfad gerufen und nicht mit vollem Pfad.** Das
//! weicht von der Gewohnheit dieses Baums ab, der `/usr/bin/git`,
//! `/usr/bin/codesign`, `/usr/bin/ditto` und `/usr/bin/xcrun` mit vollem Pfad
//! ruft, und die Abweichung hat einen Grund: jene vier liefert das System, `gh`
//! wird nachinstalliert. Es liegt je nach Mac-Architektur unter
//! `/opt/homebrew/bin` oder unter `/usr/local/bin`, ein fester Pfad waere also
//! auf einem der beiden Geraete falsch. Die Frage, ob das die Regel fuer jedes
//! kuenftige fremde Werkzeug wird, liegt dem Nutzer vor:
//! `shared/decisions/260821-1221_o_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::Abbruch;
use crate::bundle;
use crate::git;
use crate::version;

/// Das GitHub-Kommandozeilenwerkzeug, gerufen ueber den Suchpfad.
///
/// Warum ohne vollen Pfad, steht im Modulkopf.
const GH: &str = "gh";

/// Die Datei, unter der das angeheftete Beglaubigungsticket im Buendel liegt.
///
/// Nicht zu verwechseln mit `Contents/_CodeSignature/CodeResources`, das die
/// Signatur schreibt; siehe [`traegt_angeheftetes_ticket`].
const TICKETDATEI: &str = "Contents/CodeResources";

/// Die vier Bytes, mit denen ein angeheftetes Ticket beginnt.
const TICKETKENNUNG: &[u8] = b"s8ch";

/// Ob dieser Weg selbst nach dem Tag auf HEAD fragt.
///
/// Die Frage ist in beiden Faellen dieselbe, der Frager nicht: der
/// eigenstaendige Weg hat vor sich keine Station, die sie schon gestellt haette,
/// die achte Station von [`crate::release`] hat eine. Eine vollstaendige
/// Fallunterscheidung statt eines Wahrheitswertes, weil dann an der
/// Aufrufstelle der Grund steht und nicht ein nacktes `false` — und weil ein
/// dritter Rufer den Bau anhielte, statt sich stillschweigend fuer eine der
/// beiden Seiten zu entscheiden.
pub(crate) enum Tagfrage {
    /// Sie ist zu stellen: kein Rufer hat sie beantwortet.
    Stellen,
    /// Sie ist beantwortet: Station 1 von [`crate::release`] hat dieselbe
    /// Wahrheit schon gegen die eingebackene Zahl geprueft.
    Erledigt,
}

/// Veroeffentlicht ein bereits beglaubigtes Buendel:
/// `cargo xtask veroeffentlichen <zahl>`.
pub(crate) fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    let [zahl] = argumente else {
        return Err(Abbruch::Aufruf(format!(
            "veroeffentlichen nimmt genau ein Argument, die Versionszahl des beglaubigten \
             Buendels, und hat {} bekommen",
            argumente.len()
        )));
    };
    version::versionszahl_pruefen(zahl).map_err(Abbruch::Aufruf)?;
    veroeffentlichen(zahl, Tagfrage::Stellen)
}

/// Die achte Station: aus dem beglaubigten Buendel wird ein weitergebbares.
///
/// Sechs Schritte in einer festgelegten Reihenfolge, und die Reihenfolge traegt
/// eine Zusage: die drei Pruefungen stehen vorn, weil ein Abbruch an ihnen
/// nichts hinterlaesst. Erst danach wirkt der Weg.
///
/// 1. `gh` ist vorhanden und angemeldet.
/// 2. Der Tag `v<zahl>` steht auf HEAD — allein bei [`Tagfrage::Stellen`].
/// 3. Das Buendel liegt da und traegt das angeheftete Ticket.
/// 4. `target/KRK-<zahl>.zip` entsteht.
/// 5. HEAD und `refs/tags/v<zahl>` gehen in einem Aufruf zur Gegenseite.
/// 6. Die Releaseseite entsteht, nachdem die Existenzfrage verneint ist.
///
/// Der Rumpf ist beiden Rufern gemeinsam, und das ist der Grund fuer
/// [`Tagfrage`]: zwei Rumpfe nebeneinander waeren zwei Antworten darauf, was
/// Veroeffentlichen heisst. Die Probe
/// `die_voraussetzungspruefung_steht_vor_dem_ersten_wirken` liest die
/// Reihenfolge hier nach.
pub(crate) fn veroeffentlichen(zahl: &str, tagfrage: Tagfrage) -> Result<(), Abbruch> {
    // Die aeussere Voraussetzung steht ganz vorn: fehlt `gh`, liegt danach kein
    // Zip da und nichts ist geschoben.
    gh_pruefen()?;

    let wurzel = bundle::wurzel();
    let tag = tagname(zahl);
    match tagfrage {
        Tagfrage::Stellen => tagstand_fragen(&wurzel, &tag)?,
        Tagfrage::Erledigt => {}
    }

    let buendel = bundle::buendelpfad(&wurzel);
    if !buendel.exists() {
        return Err(Abbruch::Lauf(ohne_buendel_meldung(&buendel, zahl)));
    }

    ticket_pruefen(&buendel, zahl)?;
    println!(
        "Ticket geprueft: das Buendel unter {} traegt die Beglaubigung angeheftet.",
        buendel.display()
    );

    let zip = zip_packen(&buendel, zahl)?;
    println!("Gepackt: {}", zip.display());

    schieben(&wurzel, &tag)?;
    releaseseite_anlegen(&wurzel, zahl, &tag, &zip)
}

/// Prueft die aeussere Voraussetzung `gh`: vorhanden und angemeldet.
///
/// Zwei Fragen, und beide fragen nur das Werkzeug selbst, nicht das Netz.
///
/// **Die erste ist ein Startversuch.** Scheitert schon das Starten von
/// `gh --version`, ist das Werkzeug nicht da; sein Rueckgabewert wird bewusst
/// nicht befragt, denn er beantwortet eine andere Frage als die nach der
/// Anwesenheit.
///
/// **Die zweite fragt allein den Rueckgabewert von `gh auth status`**, nicht
/// seinen Wortlaut. Das ist dieselbe Regel, aus der `git` seine erste Frage
/// getrennt fuehrt: eine Antwort, die am Text einer Fehlermeldung haengt,
/// aendert sich mit der naechsten Fassung des fremden Werkzeugs.
///
/// Beide Meldungen entstehen als reine Funktionen — [`gh_fehlt_meldung`] und
/// [`nicht_angemeldet_meldung`] —, damit ihr Wortlaut ohne `gh` abnehmbar ist.
fn gh_pruefen() -> Result<(), Abbruch> {
    if let Err(fehler) = Command::new(GH).arg("--version").output() {
        return Err(Abbruch::Lauf(gh_fehlt_meldung(&fehler.to_string())));
    }
    let angemeldet = Command::new(GH)
        .args(["auth", "status"])
        .output()
        .map_err(|fehler| Abbruch::Lauf(gh_fehlt_meldung(&fehler.to_string())))?;
    if !angemeldet.status.success() {
        return Err(Abbruch::Lauf(nicht_angemeldet_meldung()));
    }
    Ok(())
}

/// Die Meldung, wenn `gh` nicht zu starten ist.
///
/// Sie nennt das Werkzeug beim vollen Namen, weil `gh` allein nicht sagt,
/// wonach zu suchen ist, und sie nennt die Folge: es ist nichts gepackt und
/// nichts veroeffentlicht.
#[must_use]
fn gh_fehlt_meldung(grund: &str) -> String {
    format!(
        "Die Veroeffentlichung braucht das GitHub-Kommandozeilenwerkzeug gh, und es laesst sich \
         nicht starten: {grund}\n\
         \n\
         Anders als git, codesign, ditto und xcrun liefert das System es nicht mit; es wird \
         nachinstalliert und ueber den Suchpfad gefunden.\n\
         \n\
         Abhilfe:\n\
         \x20      brew install gh\n\
         \x20      gh auth login\n\
         \n\
         Es ist nichts gepackt und nichts veroeffentlicht."
    )
}

/// Die Meldung, wenn `gh` da ist, aber keine Anmeldung traegt.
///
/// Sie nennt den einen Handgriff, der fehlt.
#[must_use]
fn nicht_angemeldet_meldung() -> String {
    "gh ist vorhanden, aber nicht angemeldet: `gh auth status` meldet einen Rueckgabewert \
     ungleich null. Ohne Anmeldung laesst sich keine Releaseseite anlegen.\n\
     \n\
     Abhilfe:\n\
     \x20      gh auth login\n\
     \n\
     Es ist nichts gepackt und nichts veroeffentlicht."
        .to_owned()
}

/// Die Meldung, wenn unter `target/` gar kein Buendel liegt.
///
/// Die reine Haelfte der Buendelfrage: der Dateizugriff bleibt beim Rufer, der
/// Wortlaut steht hier und ist damit ohne gebautes Buendel abnehmbar. Dasselbe
/// Muster wie bei [`gh_fehlt_meldung`], [`ohne_tag_meldung`] und
/// [`ohne_ticket_meldung`].
///
/// **Sie nennt den ganzen Weg und nicht bloss die Beglaubigung.** Wer hier
/// landet, hat nichts gebaut; mit `./certify-only.sh` waere ihm nicht geholfen,
/// denn auch jener Weg setzt ein fertiges Buendel voraus. Genannt sind deshalb
/// beide Aufrufformen des ganzen Wegs, mit und ohne den Halbschritt davor.
#[must_use]
fn ohne_buendel_meldung(buendel: &Path, zahl: &str) -> String {
    format!(
        "Unter {} liegt kein Buendel. Dieser Weg veroeffentlicht ein bereits gebautes und \
         beglaubigtes und baut selbst nichts: kein Uebersetzungslauf, kein lipo, keine \
         Montage.\n\
         \n\
         Abhilfe ist der ganze Weg:\n\
         \x20      ./release.sh {zahl}\n\
         \x20      cargo xtask release   (ohne den Halbschritt davor)",
        buendel.display()
    )
}

/// Prueft, dass das Buendel das Beglaubigungsticket angeheftet traegt.
///
/// Der Prozessaufruf dieser Pruefung ist ein Dateizugriff und sonst nichts;
/// die Entscheidung selbst faellt in [`traegt_angeheftetes_ticket`], und die
/// Meldung baut [`ohne_ticket_meldung`]. Ein fehlgeschlagenes Lesen und ein
/// Inhalt ohne Kennung fuehren zum selben Abbruch: beide heissen, dass hier
/// kein Ticket haengt.
fn ticket_pruefen(buendel: &Path, zahl: &str) -> Result<(), Abbruch> {
    let pfad = buendel.join(TICKETDATEI);
    let befund = match fs::read(&pfad) {
        Ok(inhalt) if traegt_angeheftetes_ticket(&inhalt) => return Ok(()),
        Ok(_) => format!(
            "{} beginnt nicht mit der Kennung {:?}",
            pfad.display(),
            String::from_utf8_lossy(TICKETKENNUNG)
        ),
        Err(fehler) => format!("{} ist nicht zu lesen: {fehler}", pfad.display()),
    };
    Err(Abbruch::Lauf(ohne_ticket_meldung(buendel, zahl, &befund)))
}

/// Ob der gereichte Dateiinhalt ein angeheftetes Beglaubigungsticket ist.
///
/// Die reine Haelfte der Ticketpruefung: Bytes hinein, ein `bool` heraus, kein
/// Netz, kein fremdes Werkzeug, kein Buendel. Gefragt ist allein der Anfang.
///
/// **Worauf die Konstanten stehen.** Am 260821 am ausgelieferten
/// `target/KRK.app` gemessen: `Contents/CodeResources` traegt die Aenderungszeit
/// des Heftungslaufs vom 260820 um 19:44, waehrend `Info.plist`, `PkgInfo`,
/// `MacOS/` und `Contents/_CodeSignature/CodeResources` die Bauzeit 11:35
/// tragen. Die Datei beginnt mit den vier Bytes `s8ch`, gefolgt von einer
/// DER-Struktur. **Kein Aufruf unter `xtask/` schreibt sie**: genannt wird sie
/// dort allein von [`TICKETDATEI`] und den Stellen, die diese Pruefung
/// beschreiben, und keiner der Aufrufe von `codesign`, `ditto` oder `xcrun`
/// legt sie an. Geschrieben hat sie `xcrun stapler`, das die Beglaubigung
/// anheftet.
///
/// **Die gleichnamige Datei unter `_CodeSignature/` ist eine andere.** Sie ist
/// eine XML-Eigenschaftsliste und beginnt mit `<?xml`; sie stammt von
/// `codesign` und sagt ueber die Beglaubigung nichts. Deshalb steht in
/// [`TICKETDATEI`] der Pfad und nicht der blosse Dateiname.
///
/// **Warum nicht `xcrun stapler validate`.** Es beantwortet eine andere Frage
/// als die, die hier gestellt ist: es fragt Apple, ob dieser Stand beglaubigt
/// ist, und braucht dafuer eine Netzverbindung. Im Versuch hat es bei Apple
/// nachgeladen, statt die angeheftete Fassung zu lesen. Gefragt ist hier aber,
/// ob das Buendel den Nachweis *mitbringt* — denn genau darauf kommt es auf
/// dem zweiten Mac an, der ohne Netz startet.
///
/// **Die Kennung ist von Apple nicht zugesagt.** Aendert sie sich, haelt diese
/// Funktion ein beglaubigtes Buendel faelschlich fuer ungeheftet. Das ist die
/// sichere Fehlrichtung: der Lauf bricht ab, statt ein ungeheftetes Buendel zu
/// veroeffentlichen.
#[must_use]
fn traegt_angeheftetes_ticket(inhalt: &[u8]) -> bool {
    inhalt.starts_with(TICKETKENNUNG)
}

/// Die Meldung, wenn am Buendel kein Ticket haengt.
///
/// Sie nennt die Bedingung, den Pfad und den Handgriff, der sie herstellt.
#[must_use]
fn ohne_ticket_meldung(buendel: &Path, zahl: &str, befund: &str) -> String {
    format!(
        "Das Buendel unter {} traegt kein angeheftetes Beglaubigungsticket: {befund}.\n\
         \n\
         Weitergegeben wird allein ein beglaubigtes Buendel mit angeheftetem Nachweis. Ohne ihn \
         muesste der zweite Mac bei Apple nachfragen, und ohne Netzverbindung weist er die \
         Anwendung ab.\n\
         \n\
         Abhilfe ist die Beglaubigung des gebauten Buendels:\n\
         \x20      ./certify-only.sh {zahl}\n\
         \n\
         Es ist nichts gepackt und nichts veroeffentlicht.",
        buendel.display()
    )
}

/// Packt das Buendel zu `target/KRK-<zahl>.zip`.
///
/// **Warum ein zweites Mal gepackt wird.** Die Beglaubigung packt zwar auch,
/// aber das Ergebnis ist zu diesem Zeitpunkt nicht mehr da und waere auch das
/// falsche: jenes Zip entsteht in `beglaubigung.rs:344` fuer die Einreichung
/// bei Apple, wird `:369` geloescht, und erst `:379` heftet das Ticket an das
/// Buendel. Ein wiederverwendetes Zip truege den Nachweis also gerade nicht.
/// Gepackt wird deshalb hier und jetzt, nach dem Heften.
///
/// **Die zwei Namen kommen sich nicht ins Gehege.** Die Einreichung packt
/// `target/KRK.zip`, dieser Weg `target/KRK-<zahl>.zip`; siehe [`zipname`].
///
/// Gepackt wird mit demselben `ditto -c -k --keepParent`, das die Einreichung
/// fuehrt: es haelt die Buendelstruktur samt Verweisen und
/// erweiterten Attributen, und ein gewoehnliches `zip` tut das nicht. Die Datei
/// wird bei jedem Lauf neu geschrieben.
fn zip_packen(buendel: &Path, zahl: &str) -> Result<PathBuf, Abbruch> {
    let ziel = buendel.with_file_name(zipname(zahl));
    let gepackt = Command::new("/usr/bin/ditto")
        .arg("-c")
        .arg("-k")
        .arg("--keepParent")
        .arg(buendel)
        .arg(&ziel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("ditto laesst sich nicht starten: {fehler}")))?;
    if !gepackt.status.success() {
        return Err(Abbruch::Lauf(format!(
            "ditto ist gescheitert ({}): {}\n\
             \n\
             Es ist nichts veroeffentlicht.",
            gepackt.status,
            String::from_utf8_lossy(&gepackt.stderr).trim()
        )));
    }
    Ok(ziel)
}

/// Der Name des weitergebbaren Zips zu einer Versionszahl.
///
/// Die Zahl steht im Namen, damit ein geladenes Zip auch ausserhalb der
/// Releaseseite noch sagt, welchen Stand es traegt.
#[must_use]
fn zipname(zahl: &str) -> String {
    format!("KRK-{zahl}.zip")
}

/// Der Tagname zu einer Versionszahl.
///
/// Das `v` traegt allein der Tag; die Zahl selbst steht ohne es, und
/// `version::versionszahl_pruefen` weist eine Zahl mit `v` als Aufruffehler ab.
/// Dieses Modul nennt den Tag an drei Stellen — die Tagfrage, das Schieben und
/// die Releaseseite —, deshalb steht die Fuegung einmal hier.
#[must_use]
fn tagname(zahl: &str) -> String {
    format!("v{zahl}")
}

/// Der vollstaendige Verweis, unter dem `git push` den Tag findet.
///
/// `refs/tags/<name>` und nicht der blosse Name: der blosse Name waere
/// mehrdeutig, sobald ein Zweig genauso hiesse, und `git` entschiede die
/// Mehrdeutigkeit selbst. Hier soll sie gar nicht erst entstehen.
#[must_use]
pub(crate) fn tagverweis(tag: &str) -> String {
    format!("refs/tags/{tag}")
}

/// Fragt `git`, ob der erwartete Tag auf HEAD steht.
///
/// **Station 1 von [`crate::release`] wird dafuer nicht gerufen**, obwohl sie
/// dieselbe Frage mitstellt: sie prueft zusaetzlich den Arbeitsbaum und
/// vergleicht gegen die eingebackene Zahl aus `env!("CARGO_PKG_VERSION")`, und
/// beides gehoert hier nicht hin. Ihr Name steht in diesem Modul absichtlich
/// nirgends ausgeschrieben — eine Probe in `release` haelt ihn an genau einer
/// Datei fest, und ein Verweis von hier truege sie an eine zweite.
///
/// Gefragt wird stattdessen mit denselben zwei Bausteinen, aus denen jene
/// Station selbst gebaut ist: [`git::TAGS_AUF_HEAD`] und [`git::tag_steht`].
/// Eine vierte Frage nach `git` entsteht dabei nicht.
fn tagstand_fragen(wurzel: &Path, tag: &str) -> Result<(), Abbruch> {
    let tags = git::rufen(wurzel, git::TAGS_AUF_HEAD)?;
    tagstand_pruefen(&tags, tag).map_err(Abbruch::Lauf)?;
    println!("Tag geprueft: {tag} steht auf HEAD.");
    Ok(())
}

/// Die reine Haelfte der Tagfrage: eine Ausgabe hinein, eine Meldung heraus.
///
/// `tags_auf_head` ist die Ausgabe von [`git::TAGS_AUF_HEAD`], ein Name je
/// Zeile. Kein Prozessaufruf, kein Git-Verzeichnis: der gruene Fall ist hier
/// abzunehmen und nicht an einem Lauf, der einen gesetzten Tag voraussetzt.
fn tagstand_pruefen(tags_auf_head: &str, tag: &str) -> Result<(), String> {
    if git::tag_steht(tags_auf_head, tag) {
        return Ok(());
    }
    Err(ohne_tag_meldung(tags_auf_head, tag))
}

/// Die Meldung, wenn der erwartete Tag nicht auf HEAD steht.
///
/// Sie nennt den erwarteten Namen und daneben die, die stattdessen dort stehen:
/// wer sich in der Zahl vertan hat, sieht die richtige in derselben Zeile.
#[must_use]
fn ohne_tag_meldung(tags_auf_head: &str, tag: &str) -> String {
    let stehende: Vec<&str> = tags_auf_head
        .lines()
        .map(str::trim)
        .filter(|zeile| !zeile.is_empty())
        .collect();
    let daneben = if stehende.is_empty() {
        "Auf HEAD steht ueberhaupt kein Tag.".to_owned()
    } else {
        format!("Auf HEAD stehen: {}.", stehende.join(", "))
    };
    format!(
        "Auf HEAD steht kein Tag {tag}. {daneben}\n\
         \n\
         Veroeffentlicht wird ein Stand, den ein Tag benennt: die Releaseseite haengt an genau \
         diesem Namen, und das Schieben traegt ihn mit.\n\
         \n\
         Abhilfe ist der Halbschritt, der die Zahl setzt und den Tag legt:\n\
         \x20      cargo xtask version <zahl>\n\
         \n\
         Es ist nichts gepackt und nichts veroeffentlicht."
    )
}

/// Schiebt HEAD und den Tag zur Gegenseite, in einem Aufruf.
///
/// **Es ist ein Aufruf und nicht zwei.** Zwei haetten einen Zwischenzustand, in
/// dem der Zweig oben steht und der Tag nicht; und eine Liste, die beide
/// Referenzen traegt, ist an einer Stelle nachzusehen statt an zweien.
///
/// **Der Aufruf geht durch [`git::rufen`]** und legt keine zweite
/// Git-Aufrufstelle an; die Probe `xtask_ruft_git_an_genau_einer_stelle` in
/// `release` haelt ihre Zahl auf eins.
///
/// **Was schiefgehen kann, geht laut schief.** Ein losgeloester HEAD, ein
/// zurueckgefallener Zweig, ein auf der Gegenseite anders stehender Tag: jeder
/// dieser Faelle laesst `git` mit einem Rueckgabewert ungleich null enden, und
/// der Lauf bricht mit dessen Meldung ab. Erzwungen wird nichts.
fn schieben(wurzel: &Path, tag: &str) -> Result<(), Abbruch> {
    let verweis = tagverweis(tag);
    git::rufen(wurzel, &schiebe_argumente(&verweis))?;
    println!("Geschoben: HEAD und {verweis} stehen auf origin.");
    Ok(())
}

/// `git push origin HEAD refs/tags/<name>`: das dritte schreibende Kommando.
///
/// Genau vier Woerter. **Geschoben wird `HEAD` und nicht der Zweigname**, damit
/// keine vierte lesende Frage nach `git` noetig wird — die drei Konstanten dort
/// bleiben, wie sie sind, und die Probe `keine_der_drei_fragen_schreibt` laeuft
/// unveraendert durch. `HEAD` als Quellreferenz schreibt auf der Gegenseite in
/// den Zweig gleichen Namens.
///
/// **Nachgesehen wird es Wort fuer Wort, und zwar dort, wo auch die zwei
/// aelteren schreibenden Kommandos nachgesehen werden:**
/// `version::tests::die_schreibenden_kommandos_tragen_keine_gewalt`. Die
/// Aufsicht bleibt an jener einen Stelle, weil es eine ist; der Bauer steht
/// hier, weil das Schieben hierher gehoert.
///
/// **Der Verweis kommt fertig herein und wird nicht hier gefuegt.** Der
/// Rueckgabetyp ist `Vec<&str>` wie bei den zwei aelteren Bauern, damit die eine
/// Aufsicht alle drei gleich liest; ein hier zusammengesetztes Wort koennte
/// dieser Vektor nicht besitzen. Gefuegt wird in [`tagverweis`], gleich nebenan.
#[must_use]
pub(crate) fn schiebe_argumente(tagverweis: &str) -> Vec<&str> {
    vec!["push", "origin", "HEAD", tagverweis]
}

/// Der Platzhalter, an dem [`RELEASETEXT`] die Versionszahl aufnimmt.
///
/// Die eine Fuegestelle des Texts. Sie darf mehrfach vorkommen — `str::replace`
/// setzt jedes Vorkommen —, aber es ist die einzige Art von Fuegestelle, und
/// deshalb ist der Text als Ganzes lesbar und nicht als Formatzeichenkette.
const ZAHLPLATZHALTER: &str = "{zahl}";

/// Der feste Text der Releaseseite.
///
/// **Er kommt aus dem Werkzeug und nicht aus der Versionsgeschichte.** Kein
/// `git log`, keine `RELEASE_NOTES.md`: was hier steht, ist keine Aufzaehlung
/// von Aenderungen, sondern die Betriebsregel fuer den Austausch der App. Sie
/// stammt aus der Untersuchung
/// `shared/analyses/260820-2242-lesezeichenverlust-nach-installation.md`,
/// Abschnitt „Betriebsregel fuer den Austausch der App", und sie steht hier,
/// weil dies die einzige Stelle ist, die der Nutzer im Augenblick des
/// Installierens vor Augen hat.
///
/// **Dieser Text traegt Umlaute, und der Rest dieses Moduls nicht.** Das ist
/// kein Versehen und keine zweite Schreibweise: die Abbruchmeldungen dieses
/// Baums sind Terminaltexte und stehen in Umschrift, dieser hier ist
/// veroeffentlichte Prosa auf einer Webseite und folgt darin der `README.md`,
/// dem einzigen anderen Text dieses Projekts, den Fremde zu lesen bekommen.
///
/// Jede seiner Aussagen haengt einzeln an einer Behauptung der Probe
/// `der_releasetext_traegt_jede_seiner_aussagen`; faellt eine aus dem Text,
/// benennt der Ausfall, welche.
const RELEASETEXT: &str = "\
KRK {zahl} — ein Dateimanager mit Editor für macOS.

**Voraussetzung:** macOS 15 oder neuer.

**Das Bündel ist beglaubigt** und trägt den Nachweis von Apple angeheftet. Es startet
deshalb ohne Rückfrage, auch auf einem Mac ohne Netzverbindung.

## Installieren

1. `KRK-{zahl}.zip` herunterladen und entpacken.
2. KRK beenden, falls es läuft.
3. Die neue Fassung über die alte in `/Applications` kopieren und das Ersetzen bestätigen.

## Die alte Fassung vorher nicht löschen

Ein Überkopieren ist gefahrlos, ein Löschen ist es nicht. Werkzeuge, die eine App samt
ihrer Stützdateien entfernen — der App Deleter von ForkLift ist eines —, nehmen dabei den
Ordner `~/Library/Application Support/KRK/` mit. Dort hält KRK alles, was es sich merkt:
die Lesezeichen, die gesicherte Sitzung, die abweichende Tastenbelegung und die zwei
Notizzettel. Nach so einem Löschen sind sie fort.

Wer doch löschen will, kopiert vorher den Ordner `~/Library/Application Support/KRK/` an
eine andere Stelle und schreibt die Kopie nach der Installation zurück.
";

/// Der Text der Releaseseite zu einer Versionszahl.
#[must_use]
fn releasetext(zahl: &str) -> String {
    RELEASETEXT.replace(ZAHLPLATZHALTER, zahl)
}

/// Der Titel der Releaseseite zu einer Versionszahl.
///
/// Der Name der Anwendung und die Zahl, sonst nichts. Der Tag traegt sein `v`
/// selbst, der Titel braucht keins.
#[must_use]
fn releasetitel(zahl: &str) -> String {
    format!("KRK {zahl}")
}

/// Legt die oeffentliche Releaseseite an und haengt das Zip daran.
///
/// **Die Existenzfrage steht getrennt und vorn.** Erst wird gefragt, ob das
/// Release schon steht, dann wird angelegt; die Antwort haengt damit nicht am
/// Wortlaut einer Fehlermeldung des Anlegens. Das ist dieselbe Regel, aus der
/// `git` seine erste Frage getrennt fuehrt.
///
/// **Angelegt wird gleich oeffentlich**, ohne die Marke fuer einen Entwurf und
/// ohne die fuer eine Vorabfassung; wer ausliefert, liefert aus. Die Probe
/// `dieser_weg_legt_kein_release_zurueck` sieht nach, dass keine der beiden
/// Marken in diesem Modul steht.
///
/// `gh` bestimmt das Vorhaben aus der Gegenstelle des Verzeichnisses, deshalb
/// steht `.current_dir` auf der Projektwurzel — dieselbe Erwaegung wie bei
/// [`git::rufen`].
fn releaseseite_anlegen(wurzel: &Path, zahl: &str, tag: &str, zip: &Path) -> Result<(), Abbruch> {
    if release_steht(wurzel, tag)? {
        return Err(Abbruch::Lauf(release_steht_meldung(tag, zip)));
    }

    let angelegt = Command::new(GH)
        .arg("release")
        .arg("create")
        .arg(tag)
        .arg("--title")
        .arg(releasetitel(zahl))
        .arg("--notes")
        .arg(releasetext(zahl))
        .arg(zip)
        .current_dir(wurzel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(gh_fehlt_meldung(&fehler.to_string())))?;
    if !angelegt.status.success() {
        return Err(Abbruch::Lauf(format!(
            "Das Release {tag} liess sich nicht anlegen ({}): {}\n\
             \n\
             Geschoben ist bereits: HEAD und der Tag stehen auf der Gegenseite, und das Zip \
             liegt unter {}. Was fehlt, ist allein die Releaseseite. Derselbe Aufruf noch \
             einmal holt sie nach und schiebt dabei nichts zweites.",
            angelegt.status,
            String::from_utf8_lossy(&angelegt.stderr).trim(),
            zip.display()
        )));
    }
    println!(
        "Veroeffentlicht: das Release {tag} traegt {}.",
        zip.display()
    );
    Ok(())
}

/// Steht das Release zu diesem Tag auf der Gegenseite schon?
///
/// Gefragt ist allein der Rueckgabewert von `gh release view`, nicht sein
/// Wortlaut — dieselbe Regel, aus der `gh auth status` nur an seinem
/// Rueckgabewert gemessen wird.
///
/// **Null heisst: es steht.** Alles andere heisst: es steht nicht, oder es ist
/// von hier aus gerade nicht zu erfragen. Dass die zweite Haelfte dieser
/// Auskunft unscharf ist, ist keine Luecke, sondern die sichere Richtung: das
/// Anlegen gleich danach entscheidet sie, und es ueberschreibt nichts — ein
/// bestehendes Release weist `gh` beim Anlegen ab.
fn release_steht(wurzel: &Path, tag: &str) -> Result<bool, Abbruch> {
    let gefragt = Command::new(GH)
        .arg("release")
        .arg("view")
        .arg(tag)
        .current_dir(wurzel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(gh_fehlt_meldung(&fehler.to_string())))?;
    Ok(gefragt.status.success())
}

/// Die Meldung, wenn das Release schon steht.
///
/// Sie nennt die Lage und sagt, was der Lauf bis hierher getan hat: das Zip
/// liegt, geschoben ist geschoben. Ueberschrieben wird nichts, weder die Seite
/// noch die angehaengte Datei.
#[must_use]
fn release_steht_meldung(tag: &str, zip: &Path) -> String {
    format!(
        "Auf der Gegenseite steht bereits ein Release {tag}. Es wird nicht ueberschrieben, \
         weder die Seite noch die daran haengende Datei.\n\
         \n\
         Gepackt ist {}, und geschoben ist ebenfalls schon: HEAD und der Tag stehen oben. Was \
         nicht geschieht, ist das Anlegen.\n\
         \n\
         Abhilfe ist eine neue Versionszahl:\n\
         \x20      ./release.sh <zahl>",
        zip.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buendel() -> &'static Path {
        Path::new("/Users/k1/Projects/productive/krk/target/KRK.app")
    }

    /// Das Argument ist genau eines, und es ist eine Versionszahl.
    ///
    /// Dieselbe Bauart wie `beglaubigen_nimmt_genau_ein_argument`: kein
    /// Argument, zwei Argumente, und eine Zahl mit dem `v`, das allein der Tag
    /// traegt.
    #[test]
    fn veroeffentlichen_nimmt_genau_ein_argument() {
        assert!(matches!(ausfuehren(&[]), Err(Abbruch::Aufruf(_))));
        assert!(matches!(
            ausfuehren(&["0.5.6".to_owned(), "0.5.7".to_owned()]),
            Err(Abbruch::Aufruf(_))
        ));
        assert!(matches!(
            ausfuehren(&["v0.5.6".to_owned()]),
            Err(Abbruch::Aufruf(_))
        ));
    }

    /// Ohne Buendel bricht der Weg ab und nennt den ganzen Weg (C1.6).
    ///
    /// Die Meldung ist eine reine Funktion, also ist ihr Wortlaut hier
    /// abnehmbar und nicht erst an einem Lauf, der ein fehlendes Buendel
    /// voraussetzte. Geprueft ist beides, was das Kriterium verlangt: dass der
    /// Befund benannt wird, und dass die Abhilfe der **ganze** Weg ist und
    /// nicht die Beglaubigung, die selbst schon ein Buendel braucht.
    #[test]
    fn ohne_buendel_nennt_die_meldung_den_ganzen_weg() {
        let meldung = ohne_buendel_meldung(buendel(), "0.5.6");
        assert!(meldung.contains("liegt kein Buendel"), "{meldung}");
        assert!(meldung.contains("baut selbst nichts"), "{meldung}");
        assert!(meldung.contains("./release.sh 0.5.6"), "{meldung}");
        assert!(meldung.contains("cargo xtask release"), "{meldung}");
        assert!(!meldung.contains("certify-only"), "{meldung}");
    }

    /// Fehlt `gh`, nennt die Meldung das Werkzeug und die Folge.
    ///
    /// Der Wortlaut ist ohne `gh` abnehmbar, weil die Meldung eine reine
    /// Funktion ist; auf diesem Geraet ist `gh` am 260821 nicht installiert.
    #[test]
    fn ohne_gh_nennt_die_meldung_das_werkzeug_und_die_abhilfe() {
        let meldung = gh_fehlt_meldung("No such file or directory (os error 2)");
        assert!(
            meldung.contains("GitHub-Kommandozeilenwerkzeug"),
            "{meldung}"
        );
        assert!(meldung.contains("brew install gh"), "{meldung}");
        assert!(meldung.contains("No such file or directory"), "{meldung}");
        assert!(
            meldung.contains("nichts gepackt und nichts veroeffentlicht"),
            "{meldung}"
        );
    }

    /// Ist `gh` da und nicht angemeldet, nennt die Meldung den Handgriff.
    #[test]
    fn ohne_anmeldung_nennt_die_meldung_gh_auth_login() {
        let meldung = nicht_angemeldet_meldung();
        assert!(meldung.contains("gh auth login"), "{meldung}");
        assert!(meldung.contains("nicht angemeldet"), "{meldung}");
        assert!(
            meldung.contains("nichts gepackt und nichts veroeffentlicht"),
            "{meldung}"
        );
    }

    /// Der Rumpf der achten Station, in seiner festgelegten Reihenfolge.
    ///
    /// Vier Stellen, drei Vergleiche: die Pruefung auf `gh` steht vor dem
    /// Packen, das Packen vor dem Schieben, das Schieben vor dem Anlegen. Daran
    /// haengt die Zusage, dass ein Abbruch an der aeusseren Voraussetzung nichts
    /// hinterlaesst — weder ein Zip noch eine geschobene Referenz.
    ///
    /// **Was diese Probe nicht sieht:** sie liest die Reihenfolge des Textes im
    /// Rumpf von [`veroeffentlichen`] und nicht den Ablauf — dieselbe Grenze wie
    /// bei `release::tests::die_standpruefung_steht_vor_der_ersten_uebersetzung`.
    ///
    /// Die Nadeln stehen als `concat!`, weil die Probe in der Datei liegt, die
    /// sie liest.
    #[test]
    fn die_voraussetzungspruefung_steht_vor_dem_ersten_wirken() {
        let rumpf = rumpf_von(concat!("pub(crate) fn ", "veroeffentlichen("));

        let voraussetzung = rumpf
            .find(concat!("gh_", "pruefen()"))
            .expect("die Station prueft die aeussere Voraussetzung");
        let packen = rumpf
            .find(concat!("zip_", "packen(&"))
            .expect("die Station packt");
        let schieben = rumpf
            .find(concat!("schieben(&", "wurzel"))
            .expect("die Station schiebt");
        let anlegen = rumpf
            .find(concat!("releaseseite_", "anlegen(&"))
            .expect("die Station legt die Releaseseite an");

        assert!(
            voraussetzung < packen,
            "die Pruefung auf gh steht hinter dem Packen"
        );
        assert!(packen < schieben, "das Schieben steht vor dem Packen");
        assert!(
            schieben < anlegen,
            "die Releaseseite entsteht vor dem Schieben"
        );
    }

    /// Der Rumpf einer Funktion dieses Moduls, von ihrer Kopfzeile bis zur
    /// schliessenden Klammer am Zeilenanfang.
    ///
    /// Dieselbe grobe Zerlegung, die `release::tests` fuehrt: sie taugt, weil
    /// dieses Modul keine verschachtelte Funktion mit einer Klammer in Spalte
    /// eins traegt.
    fn rumpf_von(kopf: &str) -> &'static str {
        let quelle = include_str!("veroeffentlichung.rs");
        let anfang = quelle
            .find(kopf)
            .expect("veroeffentlichung.rs fuehrt diese Funktion");
        let rumpf = &quelle[anfang..];
        let ende = rumpf.find("\n}\n").expect("die Funktion hat ein Ende");
        &rumpf[..ende]
    }

    /// Der Tag wird gegen die ganze Zeile geprueft, und die Meldung nennt ihn.
    ///
    /// Die reine Haelfte der Tagfrage; sie braucht weder Git-Verzeichnis noch
    /// gesetzten Tag.
    #[test]
    fn ohne_tag_auf_head_nennt_die_meldung_den_erwarteten_namen() {
        assert!(tagstand_pruefen("v0.5.6\n", "v0.5.6").is_ok());
        assert!(tagstand_pruefen("anderer\nv0.5.6\n", "v0.5.6").is_ok());

        let meldung = tagstand_pruefen("v0.5.5\n", "v0.5.6").expect_err("der Tag steht nicht");
        assert!(meldung.contains("kein Tag v0.5.6"), "{meldung}");
        assert!(meldung.contains("v0.5.5"), "{meldung}");
        assert!(meldung.contains("cargo xtask version"), "{meldung}");
        assert!(
            meldung.contains("nichts gepackt und nichts veroeffentlicht"),
            "{meldung}"
        );

        let leer = tagstand_pruefen("", "v0.5.6").expect_err("auf HEAD steht kein Tag");
        assert!(leer.contains("ueberhaupt kein Tag"), "{leer}");
    }

    /// Das Schieben traegt genau vier Woerter, und keines erweitert es.
    ///
    /// Die Aufsicht ueber alle drei schreibenden Kommandos steht in
    /// `version::tests::die_schreibenden_kommandos_tragen_keine_gewalt`; diese
    /// Probe hier sieht die Fuegung nach, die dort nicht entsteht: dass der
    /// Verweis `refs/tags/<name>` heisst und der Tagname sein `v` traegt.
    #[test]
    fn das_schieben_traegt_genau_vier_woerter() {
        assert_eq!(tagname("0.5.6"), "v0.5.6");
        assert_eq!(tagverweis("v0.5.6"), "refs/tags/v0.5.6");
        assert_eq!(
            schiebe_argumente(&tagverweis(&tagname("0.5.6"))),
            vec!["push", "origin", "HEAD", "refs/tags/v0.5.6"]
        );
    }

    /// Das angeheftete Ticket wird an seinen ersten vier Bytes erkannt.
    ///
    /// Vier Faelle: die Kennung selbst, ein leerer Puffer, die
    /// XML-Eigenschaftsliste der gleichnamigen Datei unter `_CodeSignature/`,
    /// und ein Puffer, der die Kennung erst spaeter traegt — gefragt ist der
    /// Anfang und nicht das Vorkommen.
    #[test]
    fn das_ticket_wird_an_der_kennung_am_anfang_erkannt() {
        assert!(traegt_angeheftetes_ticket(b"s8ch\x01\x00\x00\x00"));
        assert!(!traegt_angeheftetes_ticket(b""));
        assert!(!traegt_angeheftetes_ticket(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist"
        ));
        assert!(!traegt_angeheftetes_ticket(b"\x00\x00\x00\x00s8ch"));
    }

    /// Die Kennung wird ganz verglichen und nicht nur ihr Anfang.
    ///
    /// Ein Puffer, der kuerzer ist als die Kennung, traegt sie nicht: nichts
    /// darf zur bequemen Seite geraten werden.
    #[test]
    fn ein_zu_kurzer_puffer_traegt_die_kennung_nicht() {
        assert!(!traegt_angeheftetes_ticket(b"s8c"));
        assert!(!traegt_angeheftetes_ticket(b"s8cH"));
    }

    /// Die Meldung nennt Bedingung, Pfad und Handgriff.
    #[test]
    fn ohne_ticket_nennt_die_meldung_den_handgriff() {
        let meldung = ohne_ticket_meldung(
            buendel(),
            "0.5.6",
            "/Users/k1/Projects/productive/krk/target/KRK.app/Contents/CodeResources ist nicht \
             zu lesen: No such file or directory (os error 2)",
        );
        assert!(meldung.contains("Contents/CodeResources"), "{meldung}");
        assert!(meldung.contains("./certify-only.sh 0.5.6"), "{meldung}");
        assert!(
            meldung.contains("nichts gepackt und nichts veroeffentlicht"),
            "{meldung}"
        );
    }

    /// Der Zipname traegt die Zahl.
    #[test]
    fn der_zipname_traegt_die_zahl() {
        assert_eq!(zipname("0.5.6"), "KRK-0.5.6.zip");
        assert_eq!(zipname("1.0.0"), "KRK-1.0.0.zip");
    }

    /// Der Releasetext traegt jede seiner Aussagen, jede einzeln behauptet.
    ///
    /// **Je Aussage eine eigene Behauptung, und jede mit ihrem Namen.** Faellt
    /// eine aus dem Text, sagt der Ausfall, welche fehlt; eine einzige
    /// Behauptung ueber den ganzen Text sagte nur, dass etwas fehlt. Der Text
    /// ist die einzige Stelle, die der Nutzer im Augenblick des Installierens
    /// vor Augen hat, und er laeuft mit der Zeit von der Betriebsregel weg, aus
    /// der er stammt — dagegen steht diese Liste.
    #[test]
    fn der_releasetext_traegt_jede_seiner_aussagen() {
        let text = releasetext("0.5.6");
        for (aussage, nadel) in [
            ("die Versionszahl", "KRK 0.5.6"),
            ("die Untergrenze macOS 15", "macOS 15"),
            ("dass das Buendel beglaubigt ist", "Bündel ist beglaubigt"),
            ("dass es ohne Rueckfrage startet", "ohne Rückfrage"),
            ("die Datei, die zu laden ist", "`KRK-0.5.6.zip`"),
            ("das Entpacken", "entpacken"),
            ("das Ueberkopieren", "über die alte"),
            (
                "dass die alte nicht vorher zu loeschen ist",
                "vorher nicht löschen",
            ),
            (
                "den Ordner, den ein Loeschen mitnimmt",
                "~/Library/Application Support/KRK/",
            ),
            (
                "was in ihm liegt",
                "die Lesezeichen, die gesicherte Sitzung",
            ),
            ("die Tastenbelegung", "Tastenbelegung"),
            ("die zwei Notizzettel", "Notizzettel"),
            ("die Absicherung", "kopiert vorher den Ordner"),
        ] {
            assert!(
                text.contains(nadel),
                "der Releasetext nennt nicht {aussage}"
            );
        }
        assert!(
            !text.contains(ZAHLPLATZHALTER),
            "eine Fuegestelle ist ungefuellt geblieben: {text}"
        );
    }

    /// Der Titel traegt den Namen und die Zahl.
    #[test]
    fn der_releasetitel_traegt_die_zahl() {
        assert_eq!(releasetitel("0.5.6"), "KRK 0.5.6");
        assert_eq!(releasetitel("1.0.0"), "KRK 1.0.0");
    }

    /// Das Release entsteht gleich oeffentlich.
    ///
    /// Weder die Marke fuer einen Entwurf noch die fuer eine Vorabfassung steht
    /// in diesem Modul. Die Nadeln stehen als `concat!`, weil die Probe in der
    /// Datei liegt, die sie liest — und weil ausgeschrieben schon dieser
    /// Pruefkommentar sie truege, der sie doch nur benennen will.
    #[test]
    fn dieser_weg_legt_kein_release_zurueck() {
        let quelle = include_str!("veroeffentlichung.rs");
        for nadel in [concat!("--", "draft"), concat!("--", "prerelease")] {
            assert!(
                !quelle.contains(nadel),
                "veroeffentlichung.rs legt das Release ueber {nadel} zurueck"
            );
        }
    }

    /// Die Existenzfrage steht vor dem Anlegen.
    ///
    /// Daran haengt, dass die Antwort auf „steht das Release schon?" nicht am
    /// Wortlaut einer Fehlermeldung des Anlegens haengt, sondern an einer eigens
    /// gestellten Frage.
    #[test]
    fn die_existenzfrage_steht_vor_dem_anlegen() {
        let rumpf = rumpf_von(concat!("fn releaseseite_", "anlegen("));

        let frage = rumpf
            .find(concat!("release_", "steht(wurzel"))
            .expect("die Station fragt, ob das Release steht");
        let anlegen = rumpf
            .find(concat!(".arg(\"", "create\")"))
            .expect("die Station legt an");
        assert!(
            frage < anlegen,
            "die Existenzfrage steht hinter dem Anlegen"
        );
    }

    /// Es baut nichts.
    ///
    /// Kein Uebersetzungslauf, kein `lipo`, keine Montage, keine Signierung: die
    /// Aufrufe, mit denen `release` sein Buendel herstellt, stehen hier nicht.
    ///
    /// **Die letzten zwei Nadeln sind Aufrufe und nicht die blossen Woerter.**
    /// Der Modulkopf nennt `/usr/bin/codesign` unter den vier Werkzeugen, die
    /// dieser Baum mit vollem Pfad ruft, und die Meldung zum fehlenden `gh`
    /// nennt es noch einmal; das blosse Wort truege die Zusage also nicht,
    /// sondern verboete die Begruendung. Dieselbe Erwaegung wie bei der dritten
    /// Nadel von [`dieser_weg_reicht_nichts_ein`].
    #[test]
    fn dieser_weg_baut_nichts() {
        let quelle = include_str!("veroeffentlichung.rs");
        for nadel in [
            concat!("bundle", "::uebersetzen"),
            concat!("bundle", "::vorbereiten"),
            concat!("/usr/bin/", "lipo"),
            concat!("sign", "::"),
            concat!("Command", "::new(\"/usr/bin/codesign\")"),
        ] {
            assert!(
                !quelle.contains(nadel),
                "veroeffentlichung.rs baut ueber {nadel}"
            );
        }
    }

    /// Dieser Weg reicht nichts bei Apple ein.
    ///
    /// Die Einreichung und das Anheften stehen in `beglaubigung.rs` und dort
    /// allein; hier steht keins von beidem. Die Nadeln stehen als `concat!`,
    /// weil die Probe in der Datei liegt, die sie liest.
    ///
    /// **Die dritte Nadel ist der Anheftungsaufruf und nicht das blosse Wort
    /// `stapler`.** Der Doc-Kommentar von [`traegt_angeheftetes_ticket`] nennt
    /// das Werkzeug ausdruecklich, weil er sagt, warum `xcrun stapler validate`
    /// hier nicht genommen ist; das blosse Wort truege also die Zusage nicht,
    /// sondern verboete die Begruendung. Gefragt ist der Aufruf.
    #[test]
    fn dieser_weg_reicht_nichts_ein() {
        let quelle = include_str!("veroeffentlichung.rs");
        for nadel in [
            concat!("notary", "tool"),
            concat!("NOTAR_PROFIL", "_VARIABLE"),
            concat!("stapler", "\", \"staple\""),
        ] {
            assert!(
                !quelle.contains(nadel),
                "veroeffentlichung.rs reicht ueber {nadel} ein"
            );
        }
    }
}
