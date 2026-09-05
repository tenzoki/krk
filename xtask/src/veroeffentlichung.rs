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
//! **Neben dem Buendel reist die Anleitung `HowTo.md` mit.** Sie liegt im Zip
//! nicht lose neben der App, sondern mit ihr in einem Ordner, der so heisst wie
//! das Zip ohne seine Endung; wie sie dorthin kommt und warum, steht bei
//! [`paket_stellen`].
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
/// 4. `target/KRK-<zahl>.zip` entsteht, aus dem Buendel und der Anleitung.
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
/// **Sie hat seit dem 260821 zwei Rufer.** Der zweite ist Station 1 von
/// [`crate::release`]: auf dem langen Weg stand diese Frage bis dahin erst am
/// Kopf der achten Station, also hinter einer abgeschlossenen Einreichung bei
/// Apple, und die Begruendung des Specs — eine fehlende Voraussetzung soll
/// auffallen, solange noch nichts geschehen ist — trug dort nicht mehr
/// (Durchsicht 260821-1346, B4). Sie steht deshalb dort zusaetzlich und hier
/// unveraendert: der eigenstaendige Weg hat keine Station vor sich, und eine
/// Pruefung, die er nicht selbst faehrt, faehrt fuer ihn niemand. Zweimal
/// fragen kostet nichts, denn die Frage laesst den Baum, wie er ist.
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
/// Beide Meldungen entstehen als reine Funktionen — [`vorab_ohne_gh_meldung`]
/// und [`nicht_angemeldet_meldung`] —, damit ihr Wortlaut ohne `gh` abnehmbar
/// ist.
pub(crate) fn gh_pruefen() -> Result<(), Abbruch> {
    if let Err(fehler) = Command::new(GH).arg("--version").output() {
        return Err(Abbruch::Lauf(vorab_ohne_gh_meldung(&fehler.to_string())));
    }
    let angemeldet = Command::new(GH)
        .args(["auth", "status"])
        .output()
        .map_err(|fehler| Abbruch::Lauf(vorab_ohne_gh_meldung(&fehler.to_string())))?;
    if !angemeldet.status.success() {
        return Err(Abbruch::Lauf(nicht_angemeldet_meldung()));
    }
    Ok(())
}

/// Die Meldung, wenn `gh` nicht zu starten ist.
///
/// Sie nennt das Werkzeug beim vollen Namen, weil `gh` allein nicht sagt,
/// wonach zu suchen ist, und sie nennt die Abhilfe.
///
/// **Was sie nicht mehr nennt, ist der Stand des Laufs**, und das ist eine
/// Berichtigung vom 260821. Sie hat drei Verwendungsstellen, und der Stand ist
/// an ihnen verschieden: an der Vorpruefung ist nichts geschehen, an den zwei
/// spaeten Stellen liegt das Zip und ist geschoben. Der Satz „Es ist nichts
/// gepackt und nichts veroeffentlicht" stand bis dahin in dieser gemeinsamen
/// Haelfte und war an zwei von drei Stellen das Gegenteil dessen, was der
/// Nutzer aufraeumen muss. Er steht jetzt in [`vorab_ohne_gh_meldung`] und die
/// wahre Auskunft der spaeten Stellen in [`spaet_ohne_gh_meldung`]; geteilt
/// wird allein, was ueberall stimmt.
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
         \x20      gh auth login"
    )
}

/// Fehlt `gh` schon der Vorpruefung, ist noch nichts geschehen.
///
/// Die eine Stelle, an der der alte Schlusssatz gilt: [`gh_pruefen`] steht vor
/// dem ersten Wirken.
#[must_use]
fn vorab_ohne_gh_meldung(grund: &str) -> String {
    format!(
        "{}\n\
         \n\
         Es ist nichts gepackt und nichts veroeffentlicht.",
        gh_fehlt_meldung(grund)
    )
}

/// Verschwindet `gh` zwischen der Vorpruefung und der Releaseseite, ist bereits
/// gepackt und geschoben.
///
/// Der seltene Fall — `gh` antwortet der Vorpruefung und ist beim Anlegen weg —,
/// und der einzige, an dem der Nutzer etwas vorfindet. Die Meldung sagt ihm
/// deshalb, was steht und dass derselbe Aufruf die Seite nachholt; dasselbe,
/// was [`release_steht_meldung`] und der gescheiterte Anlegeversuch sagen.
#[must_use]
fn spaet_ohne_gh_meldung(grund: &str) -> String {
    format!(
        "{}\n\
         \n\
         Gepackt ist bereits, und geschoben ist ebenfalls schon: HEAD und der Tag stehen auf der \
         Gegenseite. Was fehlt, ist allein die Releaseseite. Derselbe Aufruf noch einmal holt sie \
         nach und schiebt dabei nichts zweites.",
        gh_fehlt_meldung(grund)
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

/// Der Name der Anleitung, im Paket wie im Quellbaum.
///
/// Sie behaelt ihren Namen: wer sie im Paket sucht, sucht die Datei, von der
/// die `README.md` und die Releaseseite sprechen.
const ANLEITUNG: &str = "HowTo.md";

/// Der Inhalt der Anleitung, beim Uebersetzen eingebacken.
///
/// **Warum eingebacken und nicht zur Laufzeit gelesen.** Zur Laufzeit waere die
/// Datei ein Fund oder ein Fehlschlag, und der Fehlschlag traefe den Lauf an
/// der Stelle, an der das Buendel gebaut, signiert und beglaubigt ist — also
/// spaet und teuer. Eingebacken gibt es den Fall nicht: fehlt die Datei,
/// uebersetzt `xtask` nicht, und das faellt vor jedem Lauf auf. Dasselbe
/// Verfahren wie bei `resources/default-keymap.toml`, das
/// `krk_core::tasten::belegung` einbackt, und bei `resources/Info.plist`, das
/// die Probe in [`crate::bundle`] liest.
///
/// **Was ausgeliefert wird, ist damit der eingecheckte Stand** und nicht der
/// Arbeitsstand: `cargo` uebersetzt `xtask` neu, sobald sich die Datei aendert,
/// und Station 1 von [`crate::release`] laesst ohnehin keinen Lauf mit einer
/// geaenderten verfolgten Datei durch.
const ANLEITUNGSTEXT: &str = include_str!("../../HowTo.md");

/// Stellt das Paket zusammen: das Buendel und die Anleitung in einem Ordner.
///
/// **Warum ueberhaupt ein Ordner.** `ditto -c -k --keepParent` packt eine
/// Quelle und nicht zwei, und ein zweites Werkzeug, das die Anleitung
/// nachtraeglich in das fertige Zip legt, waere ein zweiter Archivierer an
/// derselben Datei. Gepackt wird deshalb weiter mit demselben einen Aufruf und
/// denselben drei Schaltern; was sich aendert, ist allein seine Quelle. Der
/// Ordnername kommt aus [`paketname`], also traegt das Zip in seiner Wurzel
/// genau einen Eintrag, und der heisst wie das Zip ohne seine Endung.
///
/// **Was der Nutzer sieht.** Nach dem Doppelklick auf `KRK-<zahl>.zip` liegt
/// ein Ordner `KRK-<zahl>` da, darin `KRK.app` und daneben `HowTo.md`. Bis zu
/// dieser Aenderung lag die App selbst da. Der Ordner ist die Absicht und nicht
/// die Nebenwirkung: eine Datei namens `HowTo.md` lose im Ladeordner sagt
/// nicht, zu welchem Programm sie gehoert, und ein Archiv mit mehreren
/// Eintraegen in der Wurzel ueberliesse die Gruppierung dem Entpacker.
///
/// **Kopiert wird mit `ditto` und nicht mit `cp`.** Am Buendel haengen
/// symbolische Verweise und erweiterte Attribute, und an ihnen haengt, ob die
/// Signatur nach dem Entpacken noch traegt; `ditto` ist dasselbe Werkzeug, dem
/// das Packen selbst anvertraut ist. Das angeheftete Ticket ist eine
/// gewoehnliche Datei im Buendel und reist als solche mit.
///
/// **Ein Rest aus einem frueheren Lauf wird abgeraeumt und nicht ueberbaut.**
/// Sonst truege das Paket ein Buendel von vorgestern neben dem von heute.
fn paket_stellen(buendel: &Path, paket: &Path) -> Result<(), Abbruch> {
    let name = buendel
        .file_name()
        .ok_or_else(|| Abbruch::Lauf(format!("{} nennt kein Buendel", buendel.display())))?;
    if paket.exists() {
        fs::remove_dir_all(paket).map_err(|fehler| {
            Abbruch::Lauf(format!(
                "{} liegt aus einem frueheren Lauf und laesst sich nicht abraeumen: {fehler}",
                paket.display()
            ))
        })?;
    }
    fs::create_dir_all(paket).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} laesst sich nicht anlegen: {fehler}",
            paket.display()
        ))
    })?;

    let kopiert = Command::new("/usr/bin/ditto")
        .arg(buendel)
        .arg(paket.join(name))
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("ditto laesst sich nicht starten: {fehler}")))?;
    if !kopiert.status.success() {
        return Err(Abbruch::Lauf(format!(
            "ditto ist beim Kopieren des Buendels gescheitert ({}): {}\n\
             \n\
             Es ist nichts veroeffentlicht.",
            kopiert.status,
            String::from_utf8_lossy(&kopiert.stderr).trim()
        )));
    }

    fs::write(paket.join(ANLEITUNG), ANLEITUNGSTEXT).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "Die Anleitung laesst sich nicht nach {} schreiben: {fehler}\n\
             \n\
             Es ist nichts veroeffentlicht.",
            paket.join(ANLEITUNG).display()
        ))
    })
}

/// Packt Buendel und Anleitung zu `target/KRK-<zahl>.zip`.
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
///
/// **Die Quelle ist seit dem 260905 das Paket und nicht mehr das Buendel**, und
/// der Aufruf ist derselbe geblieben: dieselben drei Schalter, dasselbe
/// Werkzeug, ein Verzeichnis als Quelle wie zuvor. Was [`paket_stellen`] stellt,
/// raeumt dieser Weg gleich nach dem Packen wieder ab; im Zip steht es, unter
/// `target/` braucht es niemand.
fn zip_packen(buendel: &Path, zahl: &str) -> Result<PathBuf, Abbruch> {
    let ziel = buendel.with_file_name(zipname(zahl));
    let paket = buendel.with_file_name(paketname(zahl));
    paket_stellen(buendel, &paket)?;

    let gepackt = Command::new("/usr/bin/ditto")
        .arg("-c")
        .arg("-k")
        .arg("--keepParent")
        .arg(&paket)
        .arg(&ziel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("ditto laesst sich nicht starten: {fehler}")))?;
    // Derselbe Handgriff wie bei der Einreichung, die ihr Zip nach dem Absenden
    // abraeumt: was gepackt ist, wird hier nicht mehr gebraucht.
    let _ = fs::remove_dir_all(&paket);
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

/// Der Name des Ordners, den das Zip in seiner Wurzel traegt.
///
/// Die Zahl steht im Namen, damit ein entpacktes Paket auch ausserhalb des Zips
/// noch sagt, welchen Stand es traegt.
#[must_use]
fn paketname(zahl: &str) -> String {
    format!("KRK-{zahl}")
}

/// Der Name des weitergebbaren Zips zu einer Versionszahl.
///
/// Er ist der Paketname und die Endung, und die Fuegung steht hier statt einer
/// zweiten Einsetzung der Zahl: wer `KRK-<zahl>.zip` entpackt, bekommt
/// `KRK-<zahl>/`, und die zwei Namen koennen nicht auseinanderlaufen.
#[must_use]
fn zipname(zahl: &str) -> String {
    format!("{}.zip", paketname(zahl))
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
fn tagverweis(tag: &str) -> String {
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
/// Station selbst gebaut ist: [`git::Auftrag::TagsAufHead`] und
/// [`git::tag_steht`].
/// Eine vierte Frage nach `git` entsteht dabei nicht.
fn tagstand_fragen(wurzel: &Path, tag: &str) -> Result<(), Abbruch> {
    let tags = git::rufen(wurzel, &git::Auftrag::TagsAufHead)?;
    tagstand_pruefen(&tags, tag).map_err(Abbruch::Lauf)?;
    println!("Tag geprueft: {tag} steht auf HEAD.");
    Ok(())
}

/// Die reine Haelfte der Tagfrage: eine Ausgabe hinein, eine Meldung heraus.
///
/// `tags_auf_head` ist die Ausgabe von [`git::Auftrag::TagsAufHead`], ein Name je
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
/// Referenzen traegt, ist an einer Stelle nachzusehen statt an zweien. Die
/// Liste selbst steht als [`git::Auftrag::Schub`] und nicht hier — dort steht
/// die Aufsicht, die sie liest. Was hier steht, ist die Entscheidung zu
/// schieben, und die gehoert hierher.
///
/// **Der Aufruf geht durch [`git::rufen`]** und legt keine zweite
/// Git-Aufrufstelle an; die Probe
/// `git_wird_ausserhalb_der_probenordner_an_genau_einer_stelle_gerufen` in
/// `release` haelt ihre Zahl auf eins.
///
/// **Was schiefgehen kann, geht laut schief.** Ein losgeloester HEAD, ein
/// zurueckgefallener Zweig, ein auf der Gegenseite anders stehender Tag: jeder
/// dieser Faelle laesst `git` mit einem Rueckgabewert ungleich null enden, und
/// der Lauf bricht mit dessen Meldung ab. Erzwungen wird nichts, und die
/// Aufsicht in `git` liesse es auch nicht zu.
fn schieben(wurzel: &Path, tag: &str) -> Result<(), Abbruch> {
    let verweis = tagverweis(tag);
    git::rufen(wurzel, &git::Auftrag::Schub { verweis: &verweis })?;
    println!("Geschoben: HEAD und {verweis} stehen auf origin.");
    Ok(())
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

1. `KRK-{zahl}.zip` herunterladen und entpacken. Der Ordner `KRK-{zahl}` trägt danach
   `KRK.app` und daneben die Anleitung `HowTo.md`.
2. KRK beenden, falls es läuft.
3. Die neue Fassung über die alte in `/Applications` kopieren und das Ersetzen bestätigen.

## Die Anleitung liegt im Paket

`HowTo.md` beschreibt die Bedienung: die Bereiche des Fensters, die Griffe, die man sich
schlecht merkt, und die Stellen, an denen die naheliegende Annahme falsch ist. Die
vollständige Tastenbelegung gibt die laufende Anwendung selbst aus, mit **F1**.

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
        .map_err(|fehler| Abbruch::Lauf(spaet_ohne_gh_meldung(&fehler.to_string())))?;
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
        .map_err(|fehler| Abbruch::Lauf(spaet_ohne_gh_meldung(&fehler.to_string())))?;
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

    /// Fehlt `gh`, nennt die Meldung das Werkzeug und die Abhilfe.
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
    }

    /// Die geteilte Haelfte sagt ueber den Stand des Laufs nichts, und jede
    /// der zwei Stellen sagt ihn selbst.
    ///
    /// **Das ist die Berichtigung vom 260821.** Der Satz „Es ist nichts
    /// gepackt und nichts veroeffentlicht" stand in der gemeinsamen Haelfte
    /// und galt an zwei von drei Verwendungsstellen nicht: hinter dem Packen
    /// und hinter dem Schieben sagte er dem Nutzer das Gegenteil dessen, was
    /// auf der Platte und auf der Gegenseite steht. Diese Probe haelt beide
    /// Richtungen — dass der Satz aus der geteilten Haelfte heraus ist, und
    /// dass jede der zwei Stellen ihren eigenen Stand nennt.
    #[test]
    fn jede_gh_meldung_nennt_den_stand_der_an_ihrer_stelle_gilt() {
        let grund = "No such file or directory (os error 2)";
        let geteilt = gh_fehlt_meldung(grund);
        assert!(!geteilt.contains("nichts gepackt"), "{geteilt}");
        assert!(!geteilt.contains("Geschoben"), "{geteilt}");
        assert!(!geteilt.contains("geschoben"), "{geteilt}");

        let vorab = vorab_ohne_gh_meldung(grund);
        assert!(
            vorab.contains("Es ist nichts gepackt und nichts veroeffentlicht."),
            "{vorab}"
        );

        let spaet = spaet_ohne_gh_meldung(grund);
        assert!(!spaet.contains("nichts gepackt"), "{spaet}");
        assert!(spaet.contains("Gepackt ist bereits"), "{spaet}");
        assert!(spaet.contains("geschoben ist ebenfalls schon"), "{spaet}");
        assert!(spaet.contains("allein die Releaseseite"), "{spaet}");

        // Beide tragen weiter, was ueberall stimmt: Werkzeug, Grund, Abhilfe.
        for meldung in [&vorab, &spaet] {
            assert!(meldung.contains("brew install gh"), "{meldung}");
            assert!(meldung.contains(grund), "{meldung}");
        }
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
    /// Sechs Stellen, fuenf Vergleiche, eine Kette: `gh` vor der Tagfrage, die
    /// Tagfrage vor der Ticketpruefung, die Ticketpruefung vor dem Packen, das
    /// Packen vor dem Schieben, das Schieben vor dem Anlegen. Daran haengt die
    /// Zusage, die der Doc-Kommentar von [`veroeffentlichen`] ausschreibt: die
    /// drei Pruefungen stehen vorn, weil ein Abbruch an ihnen nichts
    /// hinterlaesst — weder ein Zip noch eine geschobene Referenz.
    ///
    /// **Bis zum 260821 waren es vier Stellen und drei Vergleiche**, und die
    /// zwei, die fehlten, lagen beide in der pruefenden Haelfte: wer
    /// `ticket_pruefen` hinter `zip_packen` zoege, liesse jede Probe gruen und
    /// braeche genau jene Zusage (Durchsicht 260821-1346, B1). Die Kette ist
    /// deshalb so lang wie der Rumpf.
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

        let kette = [
            (
                "die Pruefung der aeusseren Voraussetzung",
                concat!("gh_", "pruefen()"),
            ),
            ("die Tagfrage", concat!("tagstand_", "fragen(&wurzel")),
            ("die Ticketpruefung", concat!("ticket_", "pruefen(&buendel")),
            ("das Packen", concat!("zip_", "packen(&")),
            ("das Schieben", concat!("schieben(&", "wurzel")),
            ("das Anlegen", concat!("releaseseite_", "anlegen(&")),
        ];

        let mut voriger: Option<(&str, usize)> = None;
        for (name, nadel) in kette {
            let stelle = rumpf
                .find(nadel)
                .unwrap_or_else(|| panic!("die Station faehrt {name} nicht"));
            if let Some((vorname, vorstelle)) = voriger {
                assert!(vorstelle < stelle, "{vorname} steht hinter {name}");
            }
            voriger = Some((name, stelle));
        }
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
    /// Die Aufsicht ueber jeden Auftrag steht seit dem 260821 in
    /// `git::aufsichtsbefund`, auf dem Weg zum Prozessaufruf; diese Probe hier
    /// sieht die Fuegung nach, die dort nicht entsteht: dass der Verweis
    /// `refs/tags/<name>` heisst und der Tagname sein `v` traegt.
    #[test]
    fn das_schieben_traegt_genau_vier_woerter() {
        assert_eq!(tagname("0.5.6"), "v0.5.6");
        assert_eq!(tagverweis("v0.5.6"), "refs/tags/v0.5.6");
        let verweis = tagverweis(&tagname("0.5.6"));
        assert_eq!(
            git::Auftrag::Schub { verweis: &verweis }.worte(),
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

    /// Das Paket heisst wie das Zip ohne seine Endung.
    ///
    /// Daran haengt die Zusage der Releaseseite: wer `KRK-<zahl>.zip` entpackt,
    /// bekommt einen Ordner `KRK-<zahl>`. Geprueft ist die Fuegung und nicht
    /// bloss der Wortlaut beider Namen — deshalb die dritte Behauptung.
    #[test]
    fn das_paket_heisst_wie_das_zip_ohne_endung() {
        assert_eq!(paketname("0.5.6"), "KRK-0.5.6");
        assert_eq!(paketname("1.0.0"), "KRK-1.0.0");
        for zahl in ["0.5.6", "1.0.0", "12.3.45"] {
            assert_eq!(zipname(zahl), format!("{}.zip", paketname(zahl)));
        }
    }

    /// Das Zip traegt in seiner Wurzel den Paketordner, und darin liegt die
    /// Anleitung neben dem Buendel.
    ///
    /// **Sie packt wirklich**, mit demselben `ditto`, das der Lauf faehrt, und
    /// entpackt das Ergebnis wieder: eine Probe ueber den Quelltext saehe die
    /// Wirkung von `--keepParent` nicht. Das Buendel ist dabei ein
    /// nachgemachtes aus zwei Dateien, denn gefragt ist die Auslegung des
    /// Archivs und nicht die Signatur; ob die ein echtes Buendel uebersteht,
    /// entscheidet `codesign --verify` an einem gebauten und beglaubigten
    /// Buendel und keine Probe unter `make check`.
    ///
    /// Geprueft ist daneben, dass der gestellte Ordner nach dem Packen wieder
    /// weg ist: er ist Mittel und nicht Ergebnis.
    #[test]
    fn das_zip_traegt_die_anleitung_neben_dem_buendel() {
        let wurzel = crate::release::tests::wegwerfwurzel("paket");
        let buendel = wurzel.pfad().join("KRK.app/Contents");
        schreiben(&buendel.join("MacOS/krk"), "kein echtes Binaerprogramm");
        schreiben(&buendel.join("Info.plist"), "<?xml version=\"1.0\"?>");
        let buendel = wurzel.pfad().join("KRK.app");

        let zip = zip_packen(&buendel, "0.5.6").expect("das Packen gelingt");
        assert_eq!(zip, wurzel.pfad().join("KRK-0.5.6.zip"));
        assert!(
            !wurzel.pfad().join("KRK-0.5.6").exists(),
            "der gestellte Ordner steht nach dem Packen noch da"
        );

        let aus = wurzel.pfad().join("aus");
        let entpackt = Command::new("/usr/bin/ditto")
            .arg("-x")
            .arg("-k")
            .arg(&zip)
            .arg(&aus)
            .output()
            .expect("ditto laesst sich starten");
        assert!(entpackt.status.success(), "{entpackt:?}");

        let paket = aus.join("KRK-0.5.6");
        assert!(
            paket.join("KRK.app/Contents/MacOS/krk").is_file(),
            "das Buendel fehlt im Paket"
        );
        assert_eq!(
            fs::read_to_string(paket.join("HowTo.md")).expect("die Anleitung liegt im Paket"),
            ANLEITUNGSTEXT
        );
    }

    /// Legt eine Datei samt ihrer Ordner an.
    ///
    /// Dieselbe Handreichung, die `release::tests` fuehrt; sie steht hier ein
    /// zweites Mal, weil ein `use` ueber die Modulgrenze fuer drei Zeilen mehr
    /// kostet als er spart. Der Wegwerfordner dagegen wird geliehen und nicht
    /// nachgebaut: eine weitere Fassung davon waere die fuenfte im Baum.
    fn schreiben(pfad: &Path, inhalt: &str) {
        fs::create_dir_all(pfad.parent().expect("die Datei hat einen Ordner"))
            .expect("der Ordner laesst sich nicht anlegen");
        fs::write(pfad, inhalt).expect("die Datei laesst sich nicht schreiben");
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
            ("den Ordner, der dabei entsteht", "Ordner `KRK-0.5.6`"),
            ("die Anleitung neben der App", "`HowTo.md`"),
            ("wo die volle Belegung steht", "mit **F1**"),
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
