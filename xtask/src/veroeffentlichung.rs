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
//! daneben die Teilstuecke, die ein Rufer einzeln nehmen kann.
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

    // Die aeussere Voraussetzung steht ganz vorn, und die Reihenfolge traegt
    // eine Zusage: was zuerst geprueft wird, hinterlaesst bei einem Abbruch
    // nichts. Fehlt `gh`, liegt danach kein Zip da und nichts ist geschoben.
    gh_pruefen()?;

    let buendel = bundle::buendelpfad(&bundle::wurzel());
    if !buendel.exists() {
        return Err(Abbruch::Lauf(format!(
            "Unter {} liegt kein Buendel. Dieser Weg veroeffentlicht ein bereits gebautes und \
             beglaubigtes und baut selbst nichts: kein Uebersetzungslauf, kein lipo, keine \
             Montage.\n\
             \n\
             Abhilfe ist der ganze Weg:\n\
             \x20      ./release.sh {zahl}\n\
             \x20      cargo xtask release   (ohne den Halbschritt davor)",
            buendel.display()
        )));
    }

    ticket_pruefen(&buendel, zahl)?;
    println!(
        "Ticket geprueft: das Buendel unter {} traegt die Beglaubigung angeheftet.",
        buendel.display()
    );

    let zip = zip_packen(&buendel, zahl)?;
    println!("Gepackt: {}", zip.display());
    Ok(())
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
/// DER-Struktur. Kein Aufruf unter `xtask/` schreibt sie, und die Zeichenfolge
/// `CodeResources` steht weder im Bauwerkzeug noch im `Makefile` noch in der
/// `README.md`; geschrieben hat sie `xcrun stapler`, das die Beglaubigung
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

    /// Die Voraussetzungspruefung steht vor dem ersten Wirken.
    ///
    /// **Was diese Probe nicht sieht:** sie liest die Reihenfolge des Textes im
    /// Rumpf von [`ausfuehren`] und nicht den Ablauf — dieselbe Grenze wie bei
    /// `release::tests::die_standpruefung_steht_vor_der_ersten_uebersetzung`.
    /// Was sie haelt, ist die eine Zusage, dass ein Abbruch an der aeusseren
    /// Voraussetzung nichts hinterlaesst.
    ///
    /// Die Nadeln stehen als `concat!`, weil die Probe in der Datei liegt, die
    /// sie liest.
    #[test]
    fn die_voraussetzungspruefung_steht_vor_dem_packen() {
        let quelle = include_str!("veroeffentlichung.rs");
        let anfang = quelle
            .find(concat!("pub(crate) fn ", "ausfuehren("))
            .expect("veroeffentlichung.rs fuehrt ausfuehren");
        let rumpf = &quelle[anfang..];
        let ende = rumpf.find("\n}\n").expect("ausfuehren hat ein Ende");
        let rumpf = &rumpf[..ende];

        let voraussetzung = rumpf
            .find(concat!("gh_", "pruefen()"))
            .expect("ausfuehren prueft die aeussere Voraussetzung");
        let packen = rumpf
            .find(concat!("zip_", "packen(&"))
            .expect("ausfuehren packt");
        assert!(
            voraussetzung < packen,
            "die Pruefung auf gh steht hinter dem Packen"
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
