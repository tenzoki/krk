//! Die Beglaubigung: Station 7 von `release`, und der Weg, der allein sie
//! faehrt.
//!
//! ```text
//! ./certify-only.sh 0.5.5
//!   └─ make beglaubigen VERSION=0.5.5     Pfad zu cargo, Notarprofil
//!        └─ cargo xtask beglaubigen 0.5.5 ← dieses Modul
//! ```
//!
//! **Warum es diesen Weg gibt.** Am 260820 ist die Einreichung eines fertigen
//! Auslieferungslaufs am Zeitueberlauf des Uploads zu Apple gescheitert
//! (`HTTPClientError.deadlineExceeded`). Das universelle, mit Developer-ID und
//! gehaerteter Laufzeitumgebung signierte Buendel lag fertig unter
//! `target/KRK.app`, und es fehlte allein das Ticket. Ein zweites
//! `./release.sh <zahl>` haette dort nicht wieder angesetzt, sondern an
//! Station 1 abgebrochen: den Tag `v<zahl>` traegt HEAD nach dem Lauf nicht
//! mehr allein, und der Arbeitsbaum ist inzwischen ein anderer. Der ganze Weg
//! noch einmal haette zudem beide Ziele neu uebersetzt, um dasselbe Buendel
//! ein zweites Mal herzustellen.
//!
//! **Was dieser Weg ausdruecklich nicht prueft: Tag und Arbeitsbaum.** Genau
//! das ist sein Zweck. Station 1 von `release` fragt beides, und sie ist es,
//! die eine Wiederholung in dieser Lage anhaelt; hier wird sie uebergangen.
//! Das ist keine Nachlaessigkeit, sondern die Daseinsberechtigung des Wegs —
//! und zugleich seine Grenze: **ein so beglaubigtes Buendel ist nicht durch
//! die Vorpruefungen der Auslieferungskette gegangen.** Weder ist gesagt, dass
//! sein Stand eingetragen ist, noch dass ein Tag ihn benennt. Wer ein Buendel
//! von Grund auf ausliefert, nimmt `./release.sh <zahl>`; dieser Weg ist die
//! Wiederaufnahme eines Laufs, der bis zur Beglaubigung gekommen ist.
//!
//! **Er baut nichts.** Kein Uebersetzungslauf, kein `lipo`, keine Montage,
//! keine Signierung. Findet er kein Buendel, bricht er ab und nennt den ganzen
//! Weg. Was von `release` bleibt, sind die zwei Pruefungen, die das gebaute
//! Buendel gegen die Erwartung des Nutzers halten:
//!
//! 1. **Die Versionszahl gegen die `Info.plist` des Buendels.** Das ist die
//!    eine Sache, die das Argument rechtfertigt. `target/KRK.app` bleibt nach
//!    jedem Lauf liegen, auch nach einem von vorgestern; ohne diese Frage
//!    reichte der Weg still ein altes Buendel bei Apple ein, und das Ticket
//!    haenge danach am falschen Stand.
//! 2. **Der Signaturstand gegen das, was Apple annimmt.** Ein mit
//!    `cargo xtask bundle` gebautes Buendel traegt eine Entwicklungsidentitaet
//!    und keine gehaertete Laufzeitumgebung; Apple weist es ab. Der Abbruch
//!    hier spart die Einreichung.
//!
//! Die Beglaubigung selbst steht in diesem Modul und wird von beiden Wegen
//! gerufen: von [`crate::release::ausfuehren`] als Station 7 und von
//! [`ausfuehren`] als der ganze Lauf. Sie stand bis zum 260820 in `release.rs`
//! und ist hierher gewandert, statt ein zweites Mal geschrieben zu werden.

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::Abbruch;
use crate::bundle;
use crate::sign;
use crate::version;

/// Die Umgebungsvariable mit dem Namen des notarytool-Schluesselbundprofils.
pub const NOTAR_PROFIL_VARIABLE: &str = "KRK_NOTARY_PROFILE";

/// Der Schluessel, unter dem die `Info.plist` die Versionszahl fuehrt.
///
/// Dieselbe Stelle, an der `resources/Info.plist` den Platzhalter
/// [`bundle::PLATZHALTER`] traegt und `bundle::version_einsetzen` die Zahl
/// einsetzt. Gelesen wird die Kopie im gebauten Buendel, nicht die Vorlage im
/// Baum: gefragt ist, was im Buendel steht, und nicht, was ein neuer Bau
/// hineinschriebe.
const VERSIONSSCHLUESSEL: &str = "CFBundleShortVersionString";

/// Der Zeilenanfang, unter dem `codesign --display` jede Zertifizierungsstelle
/// der Signaturkette nennt.
const AUTORITAET: &str = "Authority=";

/// Das Merkmal, unter dem `codesign --display` die gehaertete
/// Laufzeitumgebung meldet.
///
/// Es steht in der Merkmalsliste der Zeile `CodeDirectory`, in Klammern hinter
/// der Zahl: `flags=0x10000(runtime)`. Gesetzt hat es
/// [`sign::signieren_gehaertet`] ueber `--options runtime`.
const GEHAERTET: &str = "runtime";

/// Beglaubigt ein bereits gebautes Buendel: `cargo xtask beglaubigen <zahl>`.
pub(crate) fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    let [zahl] = argumente else {
        return Err(Abbruch::Aufruf(format!(
            "beglaubigen nimmt genau ein Argument, die Versionszahl des gebauten Buendels, und \
             hat {} bekommen",
            argumente.len()
        )));
    };
    version::versionszahl_pruefen(zahl).map_err(Abbruch::Aufruf)?;

    let buendel = bundle::buendelpfad(&bundle::wurzel());
    if !buendel.exists() {
        return Err(Abbruch::Lauf(format!(
            "Unter {} liegt kein Buendel. Dieser Weg beglaubigt ein bereits gebautes und baut \
             selbst nichts: kein Uebersetzungslauf, kein lipo, keine Montage.\n\
             \n\
             Abhilfe ist der ganze Weg:\n\
             \x20      ./release.sh {zahl}\n\
             \x20      cargo xtask release   (ohne den Halbschritt davor)",
            buendel.display()
        )));
    }

    let plist_pfad = buendel.join("Contents").join("Info.plist");
    let plist = fs::read_to_string(&plist_pfad).map_err(|fehler| {
        Abbruch::Lauf(format!(
            "{} ist nicht lesbar: {fehler}. Ohne die Buendelbeschreibung steht nicht fest, \
             welche Version dort liegt; es wird nichts eingereicht.",
            plist_pfad.display()
        ))
    })?;
    let Some(im_buendel) = bundle::plist_zeichenkette(&plist, VERSIONSSCHLUESSEL) else {
        return Err(Abbruch::Lauf(format!(
            "{} nennt keinen Schluessel {VERSIONSSCHLUESSEL} mit einer Zeichenkette. Das \
             Buendel ist nicht von `cargo xtask release` gebaut, oder seine Beschreibung ist \
             beschaedigt; es wird nichts eingereicht.",
            plist_pfad.display()
        )));
    };
    version_pruefen(zahl, &im_buendel, &buendel).map_err(Abbruch::Lauf)?;
    println!(
        "Versionszahl geprueft: das Buendel unter {} traegt {zahl}.",
        buendel.display()
    );

    let anzeige = signaturanzeige(&buendel)?;
    signaturstand_pruefen(&anzeige, &buendel).map_err(Abbruch::Lauf)?;
    println!(
        "Signaturstand geprueft: das Buendel traegt eine Developer-ID und die gehaertete \
         Laufzeitumgebung."
    );
    println!(
        "Weder Tag noch Arbeitsbaum sind geprueft: dieser Weg setzt hinter der Signierung an. \
         Das Buendel ist damit nicht durch die Vorpruefungen der Auslieferungskette gegangen."
    );

    beglaubigen(&buendel)?;
    println!("Beglaubigt und angeheftet: {}", buendel.display());
    Ok(())
}

/// Vergleicht die gereichte Zahl mit der aus der `Info.plist` des Buendels.
///
/// Zwei Zeichenketten hinein, `Ok(())` im gruenen Fall, sonst die fertige
/// Abbruchmeldung — dieselbe Bauart wie `release::stand_pruefen`, und aus
/// demselben Grund: kein Prozessaufruf, kein Dateizugriff, kein Buendel, also
/// an dieser Funktion abnehmbar.
///
/// **Der teuerste Fehler dieses Weges ist ein altes Buendel.** `target/KRK.app`
/// ueberlebt jeden Lauf und jede Sitzung; wer nach zwei Tagen `beglaubigen`
/// ruft, meint das Buendel von heute und trifft womoeglich das von vorgestern.
/// Die Zahl im Argument ist die Behauptung des Nutzers darueber, welches
/// Buendel dort liegt, und diese Funktion haelt sie gegen den Befund.
///
/// **Gegen die `Cargo.toml` wird ausdruecklich nicht geprueft.** Sie sagt, was
/// ein *neuer* Bau traege, und der findet hier nicht statt; nach einer
/// Erhoehung der Zahl waere ein noch nicht ausgeliefertes Buendel sonst nicht
/// mehr zu beglaubigen. Verglichen wird, was eingereicht wird.
fn version_pruefen(gereicht: &str, im_buendel: &str, buendel: &Path) -> Result<(), String> {
    if gereicht == im_buendel {
        return Ok(());
    }
    Err(format!(
        "Die Versionszahl deckt das gebaute Buendel nicht: gereicht ist {gereicht}, und {} \
         traegt {im_buendel}.\n\
         \n\
         Eingereicht wird, was dort liegt, und das ist ein anderer Stand als der genannte. \
         Zwei Handgriffe, je nachdem, welche Zahl gemeint war:\n\
         \x20      cargo xtask beglaubigen {im_buendel} (das Buendel, das dort liegt)\n\
         \x20      ./release.sh {gereicht} (die genannte Zahl neu bauen und ausliefern)\n\
         \n\
         Es wird nichts eingereicht.",
        buendel.display()
    ))
}

/// Liest die Signaturanzeige des Buendels mit `codesign --display`.
///
/// **Die Anzeige steht auf der Standardfehlerausgabe**, auch im gruenen Fall;
/// die Standardausgabe bleibt leer. Nachgemessen am 260820 am gebauten
/// Buendel, und aus demselben Grund schreibt das `Makefile` im Ziel `signatur`
/// ein `2>&1` hinter denselben Aufruf.
///
/// `--verbose=2` ist die Ausfuehrlichkeit, die die Zeile `CodeDirectory` mit
/// ihrer Merkmalsliste und jede `Authority=`-Zeile bringt; es ist dieselbe
/// Stufe wie das `-dvv` des `Makefile`.
fn signaturanzeige(buendel: &Path) -> Result<String, Abbruch> {
    let ausgabe = Command::new("/usr/bin/codesign")
        .args(["--display", "--verbose=2"])
        .arg(buendel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("codesign laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "codesign liest die Signatur von {} nicht ({}): {}\n\
             \n\
             Ein unsigniertes Buendel nimmt die Beglaubigung nicht an. Gebaut und signiert wird \
             es von `cargo xtask release`; es wird nichts eingereicht.",
            buendel.display(),
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&ausgabe.stderr).into_owned())
}

/// Prueft die Signaturanzeige gegen die zwei Bedingungen der Beglaubigung.
///
/// Die reine Haelfte der zweiten Pruefung: eine Zeichenkette hinein, `Ok(())`
/// im gruenen Fall, sonst die fertige Abbruchmeldung. Beide Befunde stehen in
/// einer Meldung, wenn beide zutreffen — dasselbe Muster wie bei
/// `release::stand_pruefen`.
///
/// Gefragt sind die zwei Bedingungen, an denen ein Buendel aus
/// `cargo xtask bundle` scheitert: die Signaturkette beginnt mit einer
/// Developer-ID, und die gehaertete Laufzeitumgebung steht. Erkannt wird die
/// erste am Namensanfang [`sign::DEVELOPER_ID_PRAEFIX`], also mit derselben
/// Regel, mit der `sign::bestimmen_fuer_release` die Identitaet im
/// Schluesselbund auswaehlt; eine zweite Regel daneben waere die zweite
/// Wahrheit darueber, was eine Developer-ID ist.
///
/// **Der gesicherte Zeitstempel wird nicht eigens gefragt**, obwohl Apple ihn
/// verlangt. [`sign::signieren_gehaertet`] setzt `--options runtime` und
/// `--timestamp` in einem Aufruf; die zwei sind nicht einzeln zu haben, und
/// die Merkmalsliste beantwortet damit beide Fragen. Eine dritte Pruefung
/// truege nichts bei, was die zweite nicht schon traegt.
fn signaturstand_pruefen(anzeige: &str, buendel: &Path) -> Result<(), String> {
    let developer_id = traegt_developer_id(anzeige);
    let gehaertet = traegt_gehaertete_laufzeitumgebung(anzeige);
    if developer_id && gehaertet {
        return Ok(());
    }

    let mut befunde = Vec::new();
    if !developer_id {
        befunde.push(format!(
            "Keine Zeile {AUTORITAET}… der Signaturkette beginnt mit {:?}. Apple beglaubigt \
             allein, was mit einer Auslieferungsidentitaet signiert ist; eine \
             Entwicklungsidentitaet weist die Einreichung ab.",
            sign::DEVELOPER_ID_PRAEFIX
        ));
    }
    if !gehaertet {
        befunde.push(format!(
            "Die Merkmalsliste der Zeile CodeDirectory nennt {GEHAERTET:?} nicht, das Buendel \
             ist also ohne gehaertete Laufzeitumgebung signiert. Ohne sie nimmt Apple keine \
             Beglaubigung an."
        ));
    }

    Err(format!(
        "Das Buendel unter {} ist nicht beglaubigungsfaehig signiert:\n\
         \n\
         {}\n\
         \n\
         So signiert `cargo xtask bundle`: lokal und ohne gehaertete Laufzeitumgebung. \
         Beglaubigungsfaehig baut allein `cargo xtask release`, und den ganzen Weg faehrt\n\
         \x20      ./release.sh <zahl>\n\
         \n\
         Es wird nichts eingereicht.",
        buendel.display(),
        befunde.join("\n\n")
    ))
}

/// Ob die Signaturkette mit einer Developer-ID beginnt.
///
/// Gelesen wird der Wert jeder `Authority=`-Zeile, und einer davon muss mit
/// [`sign::DEVELOPER_ID_PRAEFIX`] beginnen. Es sind mehrere: `codesign` nennt
/// die ganze Kette bis zur Wurzel, und `Developer ID Certification Authority`
/// steht auch unter einer Entwicklungsidentitaet nicht — dort fuehrt die Kette
/// ueber `Apple Worldwide Developer Relations Certification Authority`.
/// Gemessen am 260820 an drei signierten Buendeln.
#[must_use]
fn traegt_developer_id(anzeige: &str) -> bool {
    anzeige
        .lines()
        .filter_map(|zeile| zeile.trim().strip_prefix(AUTORITAET))
        .any(|name| name.starts_with(sign::DEVELOPER_ID_PRAEFIX))
}

/// Ob die Merkmalsliste der Signatur die gehaertete Laufzeitumgebung nennt.
///
/// Gesucht ist das Wort in den Klammern hinter `flags=`:
/// `flags=0x10000(runtime)` traegt sie, `flags=0x0(none)` nicht. Mehrere
/// Merkmale trennt `codesign` mit Kommas, deshalb wird die Liste zerlegt und
/// nicht als Ganzes verglichen: ein `flags=0x10000(runtime,hard)` soll
/// dasselbe sagen wie ein `flags=0x10000(runtime)`, und ein Wort, das
/// [`GEHAERTET`] bloss enthaelt, soll nicht als es gelten.
///
/// Steht hinter `flags=` keine Klammer, gilt die Laufzeitumgebung als nicht
/// gesetzt: dann sagt die Anzeige ueber die Merkmale nichts, und ein Raten zur
/// bequemen Seite hiesse, eine Einreichung auf gut Glueck loszuschicken.
#[must_use]
fn traegt_gehaertete_laufzeitumgebung(anzeige: &str) -> bool {
    anzeige
        .split_whitespace()
        .filter_map(|wort| wort.strip_prefix("flags="))
        .filter_map(|wert| wert.split_once('('))
        .filter_map(|(_, liste)| liste.strip_suffix(')'))
        .any(|liste| liste.split(',').any(|merkmal| merkmal.trim() == GEHAERTET))
}

/// Reicht das Buendel zur Beglaubigung ein und heftet das Ergebnis an.
///
/// Station 7 von `release` und der ganze Lauf von [`ausfuehren`]. Die
/// Voraussetzungen — vollstaendiges Xcode fuer `notarytool` und `stapler`, ein
/// hinterlegtes Zugangsprofil des Apple-Entwicklerkontos — werden in `release`
/// bewusst erst hier geprueft: fehlt eine, bleibt das gebaute, signierte
/// Buendel liegen, und die Meldung benennt, was fehlt. Siehe den Modulkopf von
/// `release`.
pub(crate) fn beglaubigen(buendel: &Path) -> Result<(), Abbruch> {
    werkzeug_pruefen("notarytool", buendel)?;
    werkzeug_pruefen("stapler", buendel)?;

    let profil = match std::env::var(NOTAR_PROFIL_VARIABLE) {
        Ok(wert) if !wert.trim().is_empty() => wert.trim().to_owned(),
        _ => {
            return Err(Abbruch::Lauf(format!(
                "Die Beglaubigung braucht die Zugangsdaten eines Apple-Entwicklerkontos, und \
                 die Umgebungsvariable {NOTAR_PROFIL_VARIABLE} nennt kein \
                 Schluesselbundprofil. Das gebaute und signierte Buendel liegt unter {}.\n\
                 \n\
                 Einmalig hinterlegen:\n\
                 \x20      xcrun notarytool store-credentials <Profilname> \
                 --apple-id <Apple-ID> --team-id <Team-Kennung> \
                 --password <app-spezifisches Passwort>\n\
                 \n\
                 Danach denselben Aufruf noch einmal. Die beiden Huellen setzen die Variable \
                 selbst:\n\
                 \x20      ./release.sh <zahl>       der ganze Weg\n\
                 \x20      ./certify-only.sh <zahl>  allein die Beglaubigung",
                buendel.display()
            )));
        }
    };

    // notarytool nimmt kein nacktes Buendel an; ditto packt es so, wie Apple
    // es fuer die Einreichung vorschreibt.
    let zip = buendel.with_extension("zip");
    let gepackt = Command::new("/usr/bin/ditto")
        .arg("-c")
        .arg("-k")
        .arg("--keepParent")
        .arg(buendel)
        .arg(&zip)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("ditto laesst sich nicht starten: {fehler}")))?;
    if !gepackt.status.success() {
        return Err(Abbruch::Lauf(format!(
            "ditto ist gescheitert ({}): {}",
            gepackt.status,
            String::from_utf8_lossy(&gepackt.stderr).trim()
        )));
    }

    // --wait blockiert bis zum Urteil von Apple; der Fortschritt laeuft
    // durchgereicht ins Terminal, deshalb status() statt output().
    let eingereicht = Command::new("/usr/bin/xcrun")
        .args(["notarytool", "submit"])
        .arg(&zip)
        .args(["--keychain-profile", &profil, "--wait"])
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("xcrun laesst sich nicht starten: {fehler}")))?;
    let _ = fs::remove_file(&zip);
    if !eingereicht.success() {
        return Err(Abbruch::Lauf(format!(
            "xcrun notarytool submit --wait ist gescheitert ({eingereicht}). Das gebaute und \
             signierte Buendel liegt unter {}; das Protokoll der Einreichung nennt \
             \"xcrun notarytool log\" mit der oben gemeldeten Einreichungskennung.",
            buendel.display()
        )));
    }

    let angeheftet = Command::new("/usr/bin/xcrun")
        .args(["stapler", "staple"])
        .arg(buendel)
        .status()
        .map_err(|fehler| Abbruch::Lauf(format!("xcrun laesst sich nicht starten: {fehler}")))?;
    if !angeheftet.success() {
        return Err(Abbruch::Lauf(format!(
            "xcrun stapler staple ist gescheitert ({angeheftet}), das Buendel unter {} ist \
             beglaubigt, traegt die Beglaubigung aber nicht angeheftet.",
            buendel.display()
        )));
    }
    Ok(())
}

/// Prueft, dass `xcrun` das genannte Werkzeug findet.
///
/// `notarytool` und `stapler` liegen im vollstaendigen Xcode, nicht in den
/// Command Line Tools; die Meldung benennt genau das.
fn werkzeug_pruefen(name: &str, buendel: &Path) -> Result<(), Abbruch> {
    let gefunden = Command::new("/usr/bin/xcrun")
        .args(["--find", name])
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("xcrun laesst sich nicht starten: {fehler}")))?;
    if !gefunden.status.success() {
        return Err(Abbruch::Lauf(format!(
            "xcrun findet {name} nicht: die Beglaubigung braucht das vollstaendige Xcode, die \
             Command Line Tools genuegen nicht. Das gebaute und signierte Buendel liegt unter \
             {}.\n\
             \n\
             Abhilfe: Xcode installieren und die Werkzeugkette umstellen mit\n\
             \x20      sudo xcode-select -s /Applications/Xcode.app/Contents/Developer",
            buendel.display()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die Anzeige des am 260820 ausgelieferten Buendels: universell, mit
    /// Developer-ID und gehaerteter Laufzeitumgebung signiert.
    ///
    /// Woertlich die Ausgabe von `codesign --display --verbose=2` an
    /// `target/KRK.app`, aufgenommen an dem Buendel, dessen Einreichung am
    /// Zeitueberlauf gescheitert ist — der Lage also, fuer die es diesen Weg
    /// gibt.
    const AUSGELIEFERT: &str = "\
Executable=/Users/k1/Projects/productive/krk/target/KRK.app/Contents/MacOS/krk
Identifier=org.stalmann.krk
Format=app bundle with Mach-O universal (x86_64 arm64)
CodeDirectory v=20500 size=61372 flags=0x10000(runtime) hashes=1911+3 location=embedded
Signature size=9043
Authority=Developer ID Application: Kai Stalmann (QYMPYB7MWM)
Authority=Developer ID Certification Authority
Authority=Apple Root CA
Timestamp=20. Aug 2026 at 11:35:28
Info.plist entries=18
TeamIdentifier=QYMPYB7MWM
Runtime Version=26.2.0
Sealed Resources version=2 rules=13 files=1
Internal requirements count=1 size=176
";

    /// Die Anzeige eines Buendels, wie `cargo xtask bundle` es hinterlaesst:
    /// Entwicklungsidentitaet, keine gehaertete Laufzeitumgebung.
    ///
    /// Am 260820 an einer Kopie des ausgelieferten Buendels gemessen, neu
    /// signiert mit der Entwicklungsidentitaet des Geraets. **Beide Bedingungen
    /// sind verletzt**, und die Kette fuehrt hier ueber `Apple Worldwide
    /// Developer Relations`, nicht ueber `Developer ID Certification
    /// Authority`.
    const AUS_DEM_ENTWICKLUNGSBAU: &str = "\
Executable=/tmp/KRK.app/Contents/MacOS/krk
Identifier=org.stalmann.krk
Format=app bundle with Mach-O universal (x86_64 arm64)
CodeDirectory v=20400 size=61364 flags=0x0(none) hashes=1911+3 location=embedded
Signature size=4781
Authority=Apple Development: Kai Stalmann (FJ8U4B3QAC)
Authority=Apple Worldwide Developer Relations Certification Authority
Authority=Apple Root CA
Signed Time=20. Aug 2026 at 16:05:18
Info.plist entries=18
TeamIdentifier=QYMPYB7MWM
Sealed Resources version=2 rules=13 files=1
Internal requirements count=1 size=176
";

    /// Die Anzeige eines mit Developer-ID, aber ohne gehaertete
    /// Laufzeitumgebung signierten Buendels.
    ///
    /// Der gemischte Fall, den die dritte Stufe der Identitaetssuche und
    /// [`sign::UMGEBUNGSVARIABLE`] moeglich machen: die Identitaet stimmt, das
    /// Merkmal fehlt. Am 260820 gemessen, ebenfalls an einer neu signierten
    /// Kopie. Er ist der Grund, warum die Pruefung zwei Fragen stellt und
    /// nicht eine.
    const OHNE_HAERTUNG: &str = "\
Executable=/tmp/B.app/Contents/MacOS/krk
Identifier=org.stalmann.krk
Format=app bundle with Mach-O universal (x86_64 arm64)
CodeDirectory v=20400 size=61364 flags=0x0(none) hashes=1911+3 location=embedded
Signature size=8993
Authority=Developer ID Application: Kai Stalmann (QYMPYB7MWM)
Authority=Developer ID Certification Authority
Authority=Apple Root CA
Timestamp=20. Aug 2026 at 16:05:49
Info.plist entries=18
TeamIdentifier=QYMPYB7MWM
Sealed Resources version=2 rules=13 files=1
Internal requirements count=1 size=176
";

    fn buendel() -> &'static Path {
        Path::new("/Users/k1/Projects/productive/krk/target/KRK.app")
    }

    #[test]
    fn dieselbe_zahl_laesst_die_beglaubigung_durch() {
        assert_eq!(version_pruefen("0.5.5", "0.5.5", buendel()), Ok(()));
    }

    /// Der teuerste Fehler dieses Weges, und die Meldung dazu.
    ///
    /// Beide Zahlen und beide Handgriffe stehen darin; ohne die Zahl aus dem
    /// Buendel wuesste der Nutzer nicht, welchen Stand er vor sich hat.
    #[test]
    fn eine_abweichende_zahl_haelt_die_beglaubigung_an() {
        let meldung = version_pruefen("0.5.6", "0.5.5", buendel())
            .expect_err("das Buendel traegt eine andere Zahl");
        assert!(meldung.contains("gereicht ist 0.5.6"), "{meldung}");
        assert!(meldung.contains("traegt 0.5.5"), "{meldung}");
        assert!(
            meldung.contains("cargo xtask beglaubigen 0.5.5"),
            "{meldung}"
        );
        assert!(meldung.contains("./release.sh 0.5.6"), "{meldung}");
        assert!(meldung.contains("Es wird nichts eingereicht."), "{meldung}");
        // Kein Weg vorbei: weder Gewalt noch eine Marke zum Ueberspringen.
        assert!(!meldung.contains("--force"), "{meldung}");
        assert!(!meldung.contains("--no-verify"), "{meldung}");
    }

    #[test]
    fn das_ausgelieferte_buendel_ist_beglaubigungsfaehig() {
        assert_eq!(signaturstand_pruefen(AUSGELIEFERT, buendel()), Ok(()));
    }

    /// Ein Buendel aus `cargo xtask bundle` kommt hier nicht durch, und die
    /// Meldung nennt beide Gruende.
    #[test]
    fn ein_entwicklungsbau_haelt_die_beglaubigung_an() {
        let meldung = signaturstand_pruefen(AUS_DEM_ENTWICKLUNGSBAU, buendel())
            .expect_err("Entwicklungsidentitaet ohne Haertung");
        assert!(
            meldung.contains(sign::DEVELOPER_ID_PRAEFIX),
            "der Befund nennt die fehlende Auslieferungsidentitaet nicht: {meldung}"
        );
        assert!(
            meldung.contains("gehaertete Laufzeitumgebung"),
            "der Befund nennt die fehlende Haertung nicht: {meldung}"
        );
        assert!(meldung.contains("cargo xtask release"), "{meldung}");
        assert!(meldung.contains("Es wird nichts eingereicht."), "{meldung}");
    }

    /// Der gemischte Fall: die Identitaet stimmt, das Merkmal fehlt.
    ///
    /// Er ist die Rechtfertigung der zweiten Frage. Die Meldung nennt allein
    /// den zutreffenden Befund und behauptet nicht auch noch eine falsche
    /// Identitaet.
    #[test]
    fn eine_developer_id_ohne_haertung_haelt_die_beglaubigung_an() {
        let meldung = signaturstand_pruefen(OHNE_HAERTUNG, buendel())
            .expect_err("ohne gehaertete Laufzeitumgebung nimmt Apple nichts an");
        assert!(meldung.contains("gehaertete Laufzeitumgebung"), "{meldung}");
        assert!(
            !meldung.contains("Apple beglaubigt"),
            "der Befund zur Identitaet steht zu Unrecht da: {meldung}"
        );
    }

    #[test]
    fn die_developer_id_wird_in_der_kette_gefunden() {
        assert!(traegt_developer_id(AUSGELIEFERT));
        assert!(traegt_developer_id(OHNE_HAERTUNG));
        assert!(!traegt_developer_id(AUS_DEM_ENTWICKLUNGSBAU));
    }

    /// Eine leere oder unvollstaendige Anzeige traegt nichts.
    ///
    /// Der Fall, in dem `codesign` nichts zu sagen hat: nichts darf zur
    /// bequemen Seite geraten werden.
    #[test]
    fn eine_leere_anzeige_traegt_weder_das_eine_noch_das_andere() {
        assert!(!traegt_developer_id(""));
        assert!(!traegt_gehaertete_laufzeitumgebung(""));
        assert!(!traegt_gehaertete_laufzeitumgebung(
            "CodeDirectory v=20500 size=61372 hashes=1911+3 location=embedded"
        ));
    }

    /// Die Zertifizierungsstelle der Kette ist nicht die Identitaet.
    ///
    /// `Developer ID Certification Authority` steht in jeder ausgelieferten
    /// Kette und beginnt trotzdem nicht mit dem Praefix; wer den Vergleich auf
    /// „enthaelt" lockert, nimmt sie faelschlich als Treffer.
    #[test]
    fn die_zertifizierungsstelle_gilt_nicht_als_identitaet() {
        assert!(!traegt_developer_id(
            "Authority=Developer ID Certification Authority\nAuthority=Apple Root CA\n"
        ));
    }

    /// Die Merkmalsliste wird zerlegt und nicht als Ganzes verglichen.
    ///
    /// Mehrere Merkmale trennt `codesign` mit Kommas, und ein Wort, das
    /// `runtime` bloss enthaelt, ist nicht `runtime`.
    #[test]
    fn die_merkmalsliste_wird_zerlegt() {
        assert!(traegt_gehaertete_laufzeitumgebung(
            "CodeDirectory v=20500 flags=0x10000(runtime) hashes=1911+3"
        ));
        assert!(traegt_gehaertete_laufzeitumgebung(
            "CodeDirectory v=20500 flags=0x10002(adhoc,runtime) hashes=1911+3"
        ));
        assert!(!traegt_gehaertete_laufzeitumgebung(
            "CodeDirectory v=20400 flags=0x0(none) hashes=1911+3"
        ));
        assert!(!traegt_gehaertete_laufzeitumgebung(
            "CodeDirectory v=20400 flags=0x0(runtimeless) hashes=1911+3"
        ));
    }

    /// Das Argument ist genau eines, und es ist eine Versionszahl.
    #[test]
    fn beglaubigen_nimmt_genau_ein_argument() {
        assert!(matches!(ausfuehren(&[]), Err(Abbruch::Aufruf(_))));
        assert!(matches!(
            ausfuehren(&["0.5.5".to_owned(), "0.5.6".to_owned()]),
            Err(Abbruch::Aufruf(_))
        ));
        assert!(matches!(
            ausfuehren(&["v0.5.5".to_owned()]),
            Err(Abbruch::Aufruf(_))
        ));
    }

    /// Weder Tag noch Arbeitsbaum werden gefragt, und das ist der Zweck.
    ///
    /// Sie liest den Quelltext dieses Moduls: die Vorpruefung von Station 1
    /// und die Fragen aus `git` haben hier nichts zu suchen, sonst braeche der
    /// Weg an genau der Station ab, die er umgehen soll. Die Nadeln stehen als
    /// `concat!`, weil die Probe in der Datei liegt, die sie liest — und die
    /// erste zaehlt zugleich in `release::tests`, wo eine Probe haelt, dass
    /// jene Station an genau einer Stelle des Baums steht.
    #[test]
    fn dieser_weg_fragt_weder_nach_tag_noch_nach_arbeitsbaum() {
        let quelle = include_str!("beglaubigung.rs");
        for nadel in [
            concat!("auslieferungsstand_", "pruefen"),
            concat!("git", "::rufen"),
        ] {
            assert!(!quelle.contains(nadel), "beglaubigung.rs ruft {nadel}");
        }
    }

    /// Es baut nichts.
    ///
    /// Kein Uebersetzungslauf, kein `lipo`, keine Montage: die drei Aufrufe,
    /// mit denen `release` sein Buendel herstellt, stehen hier nicht.
    #[test]
    fn dieser_weg_baut_nichts() {
        let quelle = include_str!("beglaubigung.rs");
        for nadel in [
            concat!("bundle", "::uebersetzen"),
            concat!("bundle", "::vorbereiten"),
            concat!("/usr/bin/", "lipo"),
        ] {
            assert!(
                !quelle.contains(nadel),
                "beglaubigung.rs baut ueber {nadel}"
            );
        }
    }
}
