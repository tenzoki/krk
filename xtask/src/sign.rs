//! Die lokale Signierung des Buendels.
//!
//! KRK wird ausserhalb der App-Sandbox ausgeliefert. Der Zugriff auf die von
//! macOS geschuetzten Ordner laeuft deshalb ueber den Systemmechanismus fuer
//! Transparenz, Zustimmung und Kontrolle, und der greift am signierten
//! Anwendungsbuendel an. Daraus folgt die Regel dieses Moduls: **es wird nicht
//! ad hoc signiert.** Eine Ad-hoc-Signatur bekommt bei jedem Bau einen anderen
//! Hash; das System haelt dann jeden Bau fuer eine andere Anwendung und fragt
//! den Nutzer jedes Mal erneut. Fehlt eine Identitaet, bricht der Bau mit einer
//! Anleitung ab, statt still auf den bequemen Weg auszuweichen.

use std::path::Path;
use std::process::Command;

use crate::Abbruch;

/// Die Umgebungsvariable, die eine Identitaet ausdruecklich setzt.
pub const UMGEBUNGSVARIABLE: &str = "KRK_SIGN_IDENTITY";

/// Der Name der lokalen Entwicklungsidentitaet, nach der sonst gesucht wird.
pub const ENTWICKLUNGSIDENTITAET: &str = "KRK Entwicklung";

/// Der Namensanfang jeder Auslieferungsidentitaet von Apple.
///
/// Apple stellt sie unter `Developer ID Application: <Name> (<Team>)` aus;
/// einen frei waehlbaren Namen wie bei der Entwicklungsidentitaet gibt es
/// nicht. Die Release-Suche prueft deshalb den Anfang statt den ganzen Namen.
pub const DEVELOPER_ID_PRAEFIX: &str = "Developer ID Application";

/// Eine Signaturidentitaet samt der Stelle, von der sie kommt.
///
/// Die Herkunft steht in der Ausgabe des Baus, damit am Protokoll ablesbar
/// bleibt, womit ein Buendel signiert wurde.
pub struct Identitaet {
    /// Was `codesign --sign` bekommt.
    pub name: String,
    /// Woher der Name stammt, fuer die Meldung an den Nutzer.
    pub herkunft: String,
}

/// Bestimmt die Identitaet, mit der signiert wird.
///
/// Drei Stufen in dieser Rangfolge: die ausdrueckliche Angabe schlaegt alles,
/// dann die projekteigene Entwicklungsidentitaet, dann die eindeutige Lage.
/// Gibt es genau eine gueltige Identitaet, ist die Wahl nicht mehrdeutig, und
/// der Bau nimmt sie und schreibt hin, welche. Bei null oder mehr als einer
/// bricht er ab, denn erst dort waere die Wahl geraten.
pub fn bestimmen() -> Result<Identitaet, Abbruch> {
    if let Some(name) = aus_umgebung() {
        return Ok(Identitaet {
            name,
            herkunft: UMGEBUNGSVARIABLE.to_owned(),
        });
    }
    if enthaelt_identitaet(&auflisten()?, ENTWICKLUNGSIDENTITAET) {
        return Ok(Identitaet {
            name: ENTWICKLUNGSIDENTITAET.to_owned(),
            herkunft: format!("den Schluesselbund unter dem Namen {ENTWICKLUNGSIDENTITAET:?}"),
        });
    }
    let gueltige = gueltige_namen(&auflisten_gueltige()?);
    if let [einzige] = gueltige.as_slice() {
        return Ok(Identitaet {
            name: einzige.clone(),
            herkunft: "den Schluesselbund als einzige gueltige Identitaet".to_owned(),
        });
    }
    Err(Abbruch::Lauf(anleitung(&gueltige)))
}

/// Bestimmt die Identitaet fuer das Auslieferungspaket (Schritt 23).
///
/// Dieselben drei Stufen wie [`bestimmen`], nur sucht die zweite nach dem
/// Praefix [`DEVELOPER_ID_PRAEFIX`] statt nach dem Namen der
/// Entwicklungsidentitaet: die Beglaubigung nimmt allein eine von Apple
/// ausgestellte Developer-ID-Signatur an. Die dritte Stufe bleibt, damit auf
/// einem Geraet ohne Entwicklerkonto Bau, `lipo` und die Signierung mit
/// gehaerteter Laufzeitumgebung durchlaufen und erst der Beglaubigungsteil
/// benennend abbricht — genau der Abnahmeweg, den der Plan fuer diesen Fall
/// vorschreibt.
pub fn bestimmen_fuer_release() -> Result<Identitaet, Abbruch> {
    if let Some(name) = aus_umgebung() {
        return Ok(Identitaet {
            name,
            herkunft: UMGEBUNGSVARIABLE.to_owned(),
        });
    }
    // Wie bei der Entwicklungsidentitaet ohne `-v`: wer eine Developer-ID
    // angelegt hat, hat sich fuer sie entschieden, und die Suche hat sie nicht
    // an der Vertrauensbewertung auszusortieren. Gezaehlt wird trotzdem ueber
    // genau einen Abschnitt der Ausgabe; warum das noetig ist, steht bei
    // [`abschnitt_der_treffer`].
    let developer_ids = developer_id_namen(&gueltige_namen(&auflisten()?));
    match developer_ids.as_slice() {
        [einzige] => {
            return Ok(Identitaet {
                name: einzige.clone(),
                herkunft: format!(
                    "den Schluesselbund unter dem Namensanfang {DEVELOPER_ID_PRAEFIX:?}"
                ),
            });
        }
        [] => {}
        mehrere => {
            let aufzaehlung: Vec<String> = mehrere
                .iter()
                .map(|name| format!("\x20      {name:?}"))
                .collect();
            return Err(Abbruch::Lauf(format!(
                "Mehrere Developer-ID-Identitaeten gefunden, die Wahl waere nicht eindeutig:\n\
                 \n\
                 {}\n\
                 \n\
                 Eine davon ausdruecklich waehlen: {UMGEBUNGSVARIABLE}=\"<Name>\" cargo xtask release",
                aufzaehlung.join("\n")
            )));
        }
    }
    let gueltige = gueltige_namen(&auflisten_gueltige()?);
    if let [einzige] = gueltige.as_slice() {
        return Ok(Identitaet {
            name: einzige.clone(),
            herkunft: "den Schluesselbund als einzige gueltige Identitaet".to_owned(),
        });
    }
    Err(Abbruch::Lauf(anleitung(&gueltige)))
}

/// Liest aus einer Namensliste die Developer-ID-Identitaeten.
fn developer_id_namen(namen: &[String]) -> Vec<String> {
    namen
        .iter()
        .filter(|name| name.starts_with(DEVELOPER_ID_PRAEFIX))
        .cloned()
        .collect()
}

/// Der Abschlusshinweis nach einem lokalen Buendelbau.
///
/// `bundle` erzeugt ein Buendel, das der Nutzer weitergeben kann, und die
/// Signaturmeldung sagt nichts darueber, was dabei geschieht. Am 260812 ist ein
/// so gebautes Buendel auf einem zweiten Mac als moegliche Schadsoftware
/// abgewiesen worden, und der Nutzer hat KRK fuer beschaedigt gehalten
/// (`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`).
///
/// **Unterschieden wird nach der Art der Identitaet und nicht nach dem
/// Unterbefehl.** Wer `bundle` ueber [`UMGEBUNGSVARIABLE`] mit einer
/// Developer-ID signiert, hat das Signaturproblem nicht, und ein pauschaler
/// Warnsatz waere dort falsch. Die Grenze ist [`DEVELOPER_ID_PRAEFIX`],
/// dieselbe wie in [`bestimmen_fuer_release`]; eine zweite Wahrheit daneben
/// entsteht nicht.
///
/// Die Architektur steht in beiden Faellen, denn sie haengt nicht an der
/// Identitaet: `bundle` uebersetzt ohne Ziel-Tripel, also fuer die Architektur
/// der Baumaschine, und ist damit nie universell. Erst `release` baut beide
/// Ziele und fuegt sie mit `lipo` zusammen. Der Aufrufer reicht sie deshalb
/// herein, statt dass hier zur Laufzeit gemessen wuerde; die Tatsache steht
/// schon beim Uebersetzen fest.
///
/// **Er reicht sie unter dem Namen herein, den `lipo` benutzt.** Der Hinweis
/// steht da, damit jemand die Weitergabefaehigkeit prueft, und das Werkzeug
/// dafuer ist `lipo`; stuende hier `aarch64`, waehrend `lipo -info` fuer
/// dasselbe Programm `arm64` schreibt, truege die Ausgabe zwei Namen fuer eine
/// Architektur. Die Umrechnung ist `release::lipo_name`, angewandt auf
/// `std::env::consts::ARCH`, und sie steht dort, weil dort die Namen stehen,
/// die `lipo` melden muss.
///
/// Der Hinweis gehoert allein an `bundle`. `release` faehrt genau den Weg, auf
/// den er zeigt, und bekommt ihn nicht.
#[must_use]
pub fn weitergabehinweis(identitaet: &str, architektur: &str) -> String {
    let lage = if identitaet.starts_with(DEVELOPER_ID_PRAEFIX) {
        format!(
            "signiert ist dieses Buendel mit der Developer-ID {identitaet:?} und damit richtig. \
             Beglaubigt ist es nicht: bundle reicht nichts bei Apple ein und heftet kein Ticket \
             an, und ohne Beglaubigung weist Gatekeeper es auf einem anderen Mac ab"
        )
    } else {
        format!(
            "dieses Buendel bleibt auf dieser Maschine. Signiert ist es mit {identitaet:?}, \
             einer Entwicklungsidentitaet, und Gatekeeper weist ein so signiertes Buendel auf \
             jedem anderen Mac als moegliche Schadsoftware ab"
        )
    };
    format!(
        "Weitergabe: {lage}. Universell ist es ausserdem nicht: gebaut wurde allein fuer \
         {architektur}.\n\
         Wer weitergeben will, nimmt \"cargo xtask release\": es baut beide Mac-Architekturen \
         und fuegt sie zusammen, signiert mit einer Developer-ID und heftet nach der \
         Beglaubigung das Ticket an."
    )
}

/// Signiert das Buendel.
pub fn signieren(buendel: &Path, identitaet: &Identitaet) -> Result<(), Abbruch> {
    signieren_mit(buendel, identitaet, &[])
}

/// Signiert das Buendel mit gehaerteter Laufzeitumgebung (Schritt 23).
///
/// `--options runtime` schaltet die gehaertete Laufzeitumgebung ein, ohne die
/// die Beglaubigung das Buendel ablehnt. `--timestamp` holt einen gesicherten
/// Zeitstempel von Apples Dienst, die zweite Bedingung der Beglaubigung; der
/// Aufruf braucht dafuer Netz.
pub fn signieren_gehaertet(buendel: &Path, identitaet: &Identitaet) -> Result<(), Abbruch> {
    signieren_mit(
        buendel,
        identitaet,
        &["--options", "runtime", "--timestamp"],
    )
}

fn signieren_mit(buendel: &Path, identitaet: &Identitaet, zusatz: &[&str]) -> Result<(), Abbruch> {
    // --force, weil ein kopiertes Binaerprogramm je nach Zielplattform bereits
    // eine Signatur des Uebersetzers tragen kann; ohne die Marke bricht
    // codesign dann mit "is already signed" ab.
    let ausgabe = Command::new("/usr/bin/codesign")
        .args(["--force", "--sign", &identitaet.name])
        .args(zusatz)
        .arg(buendel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("codesign laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "codesign ist gescheitert ({}): {}",
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    println!(
        "Signiert mit {:?}, gefunden ueber {}.",
        identitaet.name, identitaet.herkunft
    );
    Ok(())
}

fn aus_umgebung() -> Option<String> {
    let gesetzt = std::env::var(UMGEBUNGSVARIABLE).ok()?;
    let getrimmt = gesetzt.trim();
    if getrimmt.is_empty() {
        return None;
    }
    Some(getrimmt.to_owned())
}

/// Fragt den Schluesselbund nach allen Identitaeten fuer die Codesignatur.
///
/// Ohne `-v`, und das ist kein Versehen. `-v` zeigt nur die als gueltig
/// bewerteten Identitaeten, und eine selbstsignierte Identitaet gilt ohne
/// gesetzte Vertrauenseinstellung als nicht vertrauenswuerdig
/// (`CSSMERR_TP_NOT_TRUSTED`). `codesign` signiert mit ihr trotzdem, und
/// `codesign --verify --strict` nimmt das Ergebnis an; am 260802-1927 auf dem
/// Referenzgeraet gemessen, und die Filterwirkung von `-v` am 260802-2253 gegen
/// einen eigens angelegten Schluesselbund mit zwei selbstsignierten
/// Identitaeten nachgeprueft: `-p codesigning` fand beide, `-v -p codesigning`
/// keine. Mit `-v` wuerde diese Stufe eine Identitaet ablehnen, die
/// nachweislich traegt, und den Nutzer durch einen Vertrauensdialog schicken,
/// der fuer die Entwicklung nichts aendert.
///
/// Ausgewertet wird davon allein der Abschnitt der Treffer: ohne `-v` gibt
/// `find-identity` die gueltigen Identitaeten ein zweites Mal aus, und wer
/// ueber die ganze Ausgabe zaehlt, zaehlt sie doppelt. Die Beschraenkung steht
/// hier und nicht bei den Aufrufern, damit keiner von ihnen sie vergessen
/// kann; die Begruendung steht bei [`abschnitt_der_treffer`].
fn auflisten() -> Result<String, Abbruch> {
    let ausgabe = security_fragen(&["find-identity", "-p", "codesigning"])?;
    Ok(abschnitt_der_treffer(&ausgabe).to_owned())
}

/// Die Ueberschrift des Abschnitts, den [`auflisten`] auswertet.
const ABSCHNITT_TREFFER: &str = "Matching identities";

/// Die Ueberschrift des Abschnitts darunter, der Eintraege wiederholt.
const ABSCHNITT_GUELTIGE: &str = "Valid identities only";

/// Beschraenkt die Ausgabe von `security find-identity` auf einen Abschnitt.
///
/// Ohne `-v` gibt `find-identity` **zwei** Abschnitte aus:
/// `Matching identities` mit allen Identitaeten und darunter
/// `Valid identities only` mit denen, die die Vertrauensbewertung bestehen.
/// Eine gueltige Identitaet steht damit in beiden. Wer ueber die ganze Ausgabe
/// zaehlt, zaehlt sie zweimal, und genau daran brach die Release-Suche bei
/// einer einzigen Developer-ID immer ab: sie traf den Zweig `mehrere`, und der
/// Zweig `[einzige]` war nur erreichbar, wenn die Identitaet **nicht** gueltig
/// war, also gerade nicht signieren konnte
/// (`shared/issues/260812-2357_*_die-identitaetssuche-zaehlt-jede-identitaet-doppelt…`).
///
/// **Gelesen wird der erste Abschnitt und nicht der zweite**, denn er fuehrt
/// auch die ungueltigen. Das ist dieselbe Wahl, aus der [`auflisten`] `-v`
/// weglaesst: wer eine Identitaet angelegt hat, hat sich fuer sie entschieden,
/// und die Suche hat sie nicht an der Vertrauensbewertung auszusortieren. Zu
/// `-v` zu greifen haette den Defekt ebenfalls behoben und dabei die Absicht
/// geaendert.
///
/// Fehlt eine der beiden Ueberschriften, bleibt der jeweilige Schnitt aus.
/// Sollte Apple die Form eines Tages aendern, ist zu viel zu lesen das kleinere
/// Uebel: es fuehrt auf denselben benennenden Abbruch wie bisher, waehrend eine
/// leer gelesene Liste die Suche in die falsche Stufe schickte.
fn abschnitt_der_treffer(liste: &str) -> &str {
    let ab_treffern = match liste.find(ABSCHNITT_TREFFER) {
        Some(stelle) => &liste[stelle + ABSCHNITT_TREFFER.len()..],
        None => liste,
    };
    match ab_treffern.find(ABSCHNITT_GUELTIGE) {
        Some(stelle) => &ab_treffern[..stelle],
        None => ab_treffern,
    }
}

/// Fragt den Schluesselbund nach den als gueltig bewerteten Identitaeten.
///
/// Hier ist `-v` richtig, und zwar aus demselben Grund, aus dem es bei
/// [`auflisten`] falsch waere. Dort wird ein *genannter* Name geprueft: wer die
/// Entwicklungsidentitaet angelegt hat, hat sich fuer sie entschieden, und der
/// Bau hat sie nicht auszusortieren. Hier dagegen *waehlt* der Bau aus einer
/// Menge aus, ohne dass jemand die Wahl getroffen haette, und automatisch
/// gewaehlt werden darf nur, was auch signieren kann. Ohne `-v` griffe die
/// Stufe sonst nach einem abgelaufenen Zertifikat oder einem, dessen Kette sich
/// nicht aufbaut, und der Bau scheiterte danach an einer Meldung von
/// `codesign`, die den Grund nicht nennt (`errSecInternalComponent`).
fn auflisten_gueltige() -> Result<String, Abbruch> {
    security_fragen(&["find-identity", "-v", "-p", "codesigning"])
}

fn security_fragen(argumente: &[&str]) -> Result<String, Abbruch> {
    let ausgabe = Command::new("/usr/bin/security")
        .args(argumente)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("security laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "security {} ist gescheitert ({}): {}",
            argumente.join(" "),
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&ausgabe.stdout).into_owned())
}

/// Prueft, ob die Liste eine Identitaet mit genau diesem Namen fuehrt.
///
/// Geprueft wird auf den Namen samt seiner Anfuehrungszeichen, wie
/// `find-identity` ihn ausgibt. Ohne sie wuerde "KRK Entwicklung" auch auf
/// "KRK Entwicklung Alt" passen, und signiert wuerde mit der falschen.
fn enthaelt_identitaet(liste: &str, name: &str) -> bool {
    liste.contains(&format!("\"{name}\""))
}

/// Liest die Namen aus einer Liste von `security find-identity`.
///
/// Gezaehlt werden Eintraege und nicht verschiedene Namen. Zwei Zertifikate
/// koennen denselben Namen tragen, und dann ist die Wahl gerade nicht
/// eindeutig: `codesign --sign` lehnt einen mehrdeutigen Namen ab. Ein
/// Zusammenfassen nach Namen wuerde die Mehrdeutigkeit verstecken, statt sie zu
/// melden.
fn gueltige_namen(liste: &str) -> Vec<String> {
    liste.lines().filter_map(eintragsname).collect()
}

/// Liest den Namen aus einer Eintragszeile.
///
/// Eine solche Zeile sieht so aus, hier aus der Ausgabe vom 260802-2253 auf dem
/// Referenzgeraet:
///
/// ```text
///   1) 4B30A8F73354FC4A6B200FCB2F2F5C6F22586D0D "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
/// ```
///
/// Verlangt wird die laufende Nummer vor der Klammer, damit weder die
/// Ueberschriften noch die Zaehlzeile `1 valid identities found` als Eintrag
/// durchgehen. Der Name steht zwischen dem ersten und dem letzten
/// Anfuehrungszeichen nach der Klammer; er darf selbst Klammern enthalten, wie
/// die Team-Kennung im Beispiel zeigt.
fn eintragsname(zeile: &str) -> Option<String> {
    let getrimmt = zeile.trim_start();
    let klammer = getrimmt.find(')')?;
    if klammer == 0
        || !getrimmt[..klammer]
            .bytes()
            .all(|zeichen| zeichen.is_ascii_digit())
    {
        return None;
    }
    let rest = &getrimmt[klammer + 1..];
    let anfang = rest.find('"')?;
    let ende = rest.rfind('"')?;
    (ende > anfang).then(|| rest[anfang + 1..ende].to_owned())
}

/// Die Abbruchmeldung, wenn keine Stufe eine Identitaet bestimmt hat.
///
/// Zwei Faelle mit verschiedenem Kopf. "Keine gefunden" waere bei mehreren
/// gueltigen Identitaeten falsch, und eine falsche Meldung ist genau der
/// Defekt, den die dritte Stufe behebt.
fn anleitung(gueltige: &[String]) -> String {
    let (kopf, nachtrag) = if gueltige.is_empty() {
        (
            "Keine gueltige Signaturidentitaet gefunden, es entsteht kein Buendel.".to_owned(),
            "\n\nZeigt \"security find-identity -p codesigning\" eine Identitaet und \
             \"security find-identity -v -p codesigning\" keine, ist vermutlich das \
             Apple-Zwischenzertifikat im Schluesselbund veraltet. README.md nennt im Abschnitt \
             \"Abgelaufene Zertifikatskette\" die zwei Kommandos dagegen."
                .to_owned(),
        )
    } else {
        let aufzaehlung: Vec<String> = gueltige
            .iter()
            .map(|name| format!("\x20      {name:?}"))
            .collect();
        (
            format!(
                "Mehrere gueltige Signaturidentitaeten gefunden, es entsteht kein Buendel: \
                 die Wahl waere nicht eindeutig. Gueltig sind:\n\
                 \n\
                 {}",
                aufzaehlung.join("\n")
            ),
            String::new(),
        )
    };
    format!(
        "{kopf}\n\
         \n\
         KRK wird nicht ad hoc signiert: eine Ad-hoc-Signatur bekommt bei jedem Bau einen \
         anderen Hash, und der Systemmechanismus fuer Transparenz, Zustimmung und Kontrolle \
         haelt dann jeden Bau fuer eine andere Anwendung und fragt bei jedem Start erneut nach \
         dem Zugriff auf Schreibtisch, Dokumente und Downloads.\n\
         \n\
         Zwei Wege:\n\
         \n\
         1. Eine vorhandene Identitaet ausdruecklich waehlen:\n\
         \x20      {UMGEBUNGSVARIABLE}=\"<Name der Identitaet>\" cargo xtask bundle\n\
         \x20      Welche es gibt, zeigt: security find-identity -p codesigning\n\
         \n\
         2. Einmalig eine lokale Entwicklungsidentitaet mit dem Namen {ENTWICKLUNGSIDENTITAET:?} \
         anlegen. Die Schritte stehen in README.md im Abschnitt \
         \"Entwicklungsidentitaet anlegen\"; sie brauchen kein Xcode.{nachtrag}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// So sieht die Ausgabe von `security find-identity -p codesigning` aus.
    const AUSGABE: &str = r#"
Policy: Code Signing
  Matching identities
  1) F5F3F985C14F947E88D3BF9DB713C738D88A5728 "KRK Entwicklung" (CSSMERR_TP_NOT_TRUSTED)
     1 identities found
"#;

    /// Die Ausgabe von `security find-identity -v -p codesigning` mit genau
    /// einer gueltigen Identitaet, am 260802-2253 auf dem Referenzgeraet abgenommen.
    /// Anders als ohne `-v` gibt es hier weder eine Zeile `Policy:` noch zwei
    /// Abschnitte, sondern allein die Liste der gueltigen samt Zaehlzeile.
    const GUELTIGE_EINE: &str = r#"  1) 4B30A8F73354FC4A6B200FCB2F2F5C6F22586D0D "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
     1 valid identities found
"#;

    /// Dieselbe Abfrage ohne gueltige Identitaet, am 260802-2253 gegen einen eigens
    /// angelegten Schluesselbund mit zwei selbstsignierten Identitaeten
    /// abgenommen. `-v` meldet sie beide nicht.
    const GUELTIGE_KEINE: &str = "     0 valid identities found\n";

    /// Zwei gueltige Identitaeten. Auf dem Referenzgeraet gibt es nur eine, und
    /// eine zweite gueltige liesse sich ohne Eingriff in den Schluesselbund des
    /// Nutzers nicht herstellen. Die Zeilenform stammt deshalb aus
    /// `GUELTIGE_EINE`, nur die Zahlen sind fortgezaehlt.
    const GUELTIGE_ZWEI: &str = r#"  1) 4B30A8F73354FC4A6B200FCB2F2F5C6F22586D0D "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
  2) 8C11B4E60A2D77F5913EE0A4C2B8D6F30947AA12 "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     2 valid identities found
"#;

    /// Die volle Ausgabe von `security find-identity -p codesigning`, am
    /// 260813 auf dem Referenzgeraet abgenommen. **Beide Abschnitte gehoeren
    /// dazu:** ohne den zweiten misst keine Probe den Defekt 260812-2357.
    const BEIDE_ABSCHNITTE: &str = r#"Policy: Code Signing
  Matching identities
  1) 4B30A8F73354FC4A6B200FCB2F2F5C6F22586D0D "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
  2) B2CA1443DCFE16C610D45DA616D744D762270145 "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     2 identities found

  Valid identities only
  1) 4B30A8F73354FC4A6B200FCB2F2F5C6F22586D0D "Apple Development: Kai Stalmann (FJ8U4B3QAC)"
  2) B2CA1443DCFE16C610D45DA616D744D762270145 "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     2 valid identities found
"#;

    /// Dieselbe Form mit einer selbstsignierten Identitaet, die die
    /// Vertrauensbewertung nicht besteht: sie steht im ersten Abschnitt und
    /// fehlt im zweiten. Die Zeilenform stammt aus `BEIDE_ABSCHNITTE`, die
    /// Zeile der Entwicklungsidentitaet aus `AUSGABE`.
    const BEIDE_ABSCHNITTE_MIT_UNGUELTIGER: &str = r#"Policy: Code Signing
  Matching identities
  1) F5F3F985C14F947E88D3BF9DB713C738D88A5728 "KRK Entwicklung" (CSSMERR_TP_NOT_TRUSTED)
  2) B2CA1443DCFE16C610D45DA616D744D762270145 "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     2 identities found

  Valid identities only
  1) B2CA1443DCFE16C610D45DA616D744D762270145 "Developer ID Application: Kai Stalmann (QYMPYB7MWM)"
     1 valid identities found
"#;

    #[test]
    fn die_entwicklungsidentitaet_wird_in_der_liste_gefunden() {
        assert!(enthaelt_identitaet(AUSGABE, ENTWICKLUNGSIDENTITAET));
    }

    #[test]
    fn eine_leere_liste_traegt_keine_identitaet() {
        let leer = "\n  0 identities found\n";
        assert!(!enthaelt_identitaet(leer, ENTWICKLUNGSIDENTITAET));
    }

    #[test]
    fn ein_laengerer_name_gilt_nicht_als_treffer() {
        let andere = "  1) ABC \"KRK Entwicklung Alt\"\n";
        assert!(!enthaelt_identitaet(andere, ENTWICKLUNGSIDENTITAET));
    }

    #[test]
    fn aus_der_liste_der_gueltigen_wird_genau_ein_name_gelesen() {
        assert_eq!(
            gueltige_namen(GUELTIGE_EINE),
            vec!["Apple Development: Kai Stalmann (FJ8U4B3QAC)".to_owned()]
        );
    }

    #[test]
    fn ohne_gueltige_identitaet_bleibt_die_liste_leer() {
        assert!(gueltige_namen(GUELTIGE_KEINE).is_empty());
    }

    #[test]
    fn zwei_gueltige_identitaeten_werden_beide_gelesen() {
        assert_eq!(gueltige_namen(GUELTIGE_ZWEI).len(), 2);
    }

    #[test]
    fn die_zaehlzeile_und_die_ueberschriften_sind_keine_eintraege() {
        // Ohne diese Abgrenzung zaehlte "1 valid identities found" mit, und aus
        // null gueltigen Identitaeten wuerde eine.
        assert!(eintragsname("     1 valid identities found").is_none());
        assert!(eintragsname("Policy: Code Signing").is_none());
        assert!(eintragsname("  Matching identities").is_none());
    }

    #[test]
    fn die_vertrauensbewertung_gehoert_nicht_zum_namen() {
        let zeile = "  1) 753C3DBA523A88D7CC8737769B457BC5FFF757DB \"KRK Probe Eins\" (CSSMERR_TP_NOT_TRUSTED)";
        assert_eq!(eintragsname(zeile).unwrap(), "KRK Probe Eins");
    }

    #[test]
    fn die_developer_id_wird_am_namensanfang_erkannt() {
        let namen = gueltige_namen(GUELTIGE_ZWEI);
        assert_eq!(
            developer_id_namen(&namen),
            vec!["Developer ID Application: Kai Stalmann (QYMPYB7MWM)".to_owned()]
        );
    }

    #[test]
    fn eine_entwicklungsidentitaet_ist_keine_developer_id() {
        // "Apple Development: …" beginnt nicht mit dem Praefix; die zweite
        // Stufe der Release-Suche darf sie nicht greifen.
        let namen = gueltige_namen(GUELTIGE_EINE);
        assert!(developer_id_namen(&namen).is_empty());
    }

    /// Der Fall vom 260812, an dem der Hinweis haengt.
    ///
    /// "Apple Development: …" beginnt nicht mit [`DEVELOPER_ID_PRAEFIX`], ist
    /// also eine Entwicklungsidentitaet. Genau damit signiert war das Buendel,
    /// das der zweite Mac als moegliche Schadsoftware abgewiesen hat.
    #[test]
    fn eine_apple_development_identitaet_bekommt_die_maschinengrenze_genannt() {
        let namen = gueltige_namen(GUELTIGE_EINE);
        let text = weitergabehinweis(&namen[0], "x86_64");
        assert!(text.contains("bleibt auf dieser Maschine"), "{text}");
        assert!(text.contains("Entwicklungsidentitaet"), "{text}");
        assert!(text.contains("moegliche Schadsoftware"), "{text}");
        assert!(text.contains(&namen[0]), "{text}");
    }

    /// Bei einer Developer-ID faellt der Warnsatz zur Signatur weg.
    ///
    /// Wer `bundle` ueber die Umgebungsvariable mit einer Developer-ID
    /// signiert, hat richtig signiert; ein pauschaler Warnsatz waere dort
    /// falsch. Offen bleibt allein die Beglaubigung.
    #[test]
    fn eine_developer_id_wird_nicht_fuer_falsch_signiert_erklaert() {
        let namen = developer_id_namen(&gueltige_namen(GUELTIGE_ZWEI));
        let text = weitergabehinweis(&namen[0], "x86_64");
        assert!(text.contains("damit richtig"), "{text}");
        assert!(text.contains("Beglaubigt ist es nicht"), "{text}");
        assert!(!text.contains("Entwicklungsidentitaet"), "{text}");
        assert!(!text.contains("moegliche Schadsoftware"), "{text}");
    }

    /// Die zweite Luecke haengt an keiner Identitaet.
    ///
    /// `bundle` uebersetzt fuer die Architektur der Baumaschine und ist nie
    /// universell; das gilt in beiden Faellen, und in beiden fuehrt der Weg
    /// zur Weitergabe ueber `cargo xtask release`.
    ///
    /// Die Architektur geht durch dieselbe Umrechnung wie beim Aufrufer, damit
    /// die Probe den Namen misst, den der Nutzer liest: `arm64` und nicht
    /// `aarch64`.
    #[test]
    fn beide_faelle_nennen_die_architektur_und_den_weg_zur_weitergabe() {
        let entwicklung = gueltige_namen(GUELTIGE_EINE);
        let developer_id = developer_id_namen(&gueltige_namen(GUELTIGE_ZWEI));
        for name in [&entwicklung[0], &developer_id[0]] {
            let text = weitergabehinweis(name, crate::release::lipo_name("aarch64"));
            assert!(text.contains("Universell ist es ausserdem nicht"), "{text}");
            assert!(text.contains("arm64"), "{text}");
            assert!(!text.contains("aarch64"), "{text}");
            assert!(text.contains("cargo xtask release"), "{text}");
        }
    }

    /// Den Hinweis gibt allein der Unterbefehl `bundle` aus.
    ///
    /// In `release` waere er falsch: der Unterbefehl faehrt genau den Weg, auf
    /// den der Hinweis zeigt. `messen --alle` baut dasselbe Buendel fuer eine
    /// Messung und gibt es nicht weiter. Die Probe schreibt den Ausgabeort
    /// nicht fest, sondern haelt fest, dass es genau einen gibt und wo er
    /// nicht liegt.
    #[test]
    fn allein_der_unterbefehl_bundle_gibt_den_hinweis_aus() {
        let nadel = concat!("weitergabe", "hinweis(");
        for (name, quelle) in [
            ("release.rs", include_str!("release.rs")),
            ("messen.rs", include_str!("messen.rs")),
            ("bundle.rs", include_str!("bundle.rs")),
        ] {
            assert!(!quelle.contains(nadel), "{name} gibt den Hinweis aus");
        }
        assert_eq!(include_str!("main.rs").matches(nadel).count(), 1);
    }

    #[test]
    fn die_anleitung_nennt_beide_wege() {
        let text = anleitung(&[]);
        assert!(text.contains(UMGEBUNGSVARIABLE));
        assert!(text.contains(ENTWICKLUNGSIDENTITAET));
        assert!(text.contains("README.md"));
    }

    #[test]
    fn ohne_identitaet_meldet_die_anleitung_keine_gefundene() {
        let text = anleitung(&[]);
        assert!(text.starts_with("Keine gueltige Signaturidentitaet gefunden"));
        assert!(text.contains("Abgelaufene Zertifikatskette"));
    }

    #[test]
    fn bei_mehreren_meldet_die_anleitung_die_mehrdeutigkeit_und_nennt_sie() {
        // "Keine gefunden" waere hier falsch, und die falsche Meldung ist der
        // Defekt, den die dritte Stufe behebt.
        let namen = [
            "Erste Identitaet".to_owned(),
            "Zweite Identitaet".to_owned(),
        ];
        let text = anleitung(&namen);
        assert!(text.starts_with("Mehrere gueltige Signaturidentitaeten gefunden"));
        assert!(text.contains("Erste Identitaet"));
        assert!(text.contains("Zweite Identitaet"));
        assert!(!text.contains("Keine gueltige Signaturidentitaet"));
        // Der Hinweis auf die Zertifikatskette gehoert nur in den Fall ohne
        // gueltige Identitaet; hier gibt es welche.
        assert!(!text.contains("Abgelaufene Zertifikatskette"));
    }

    #[test]
    fn ueber_die_ganze_ausgabe_zaehlt_jede_identitaet_doppelt() {
        // Der Defekt selbst, festgehalten: zwei Identitaeten im
        // Schluesselbund, vier Eintraege in der Ausgabe. Schlaegt diese Probe
        // eines Tages fehl, weil `security` nur noch einen Abschnitt ausgibt,
        // ist die Beschraenkung entbehrlich geworden und nicht etwa kaputt.
        assert_eq!(gueltige_namen(BEIDE_ABSCHNITTE).len(), 4);
    }

    #[test]
    fn im_abschnitt_der_treffer_steht_jede_identitaet_genau_einmal() {
        assert_eq!(
            gueltige_namen(abschnitt_der_treffer(BEIDE_ABSCHNITTE)),
            vec![
                "Apple Development: Kai Stalmann (FJ8U4B3QAC)".to_owned(),
                "Developer ID Application: Kai Stalmann (QYMPYB7MWM)".to_owned(),
            ]
        );
    }

    #[test]
    fn die_release_suche_findet_die_einzige_developer_id_eindeutig() {
        // Die Probe zum Defekt 260812-2357: ueber die ganze Ausgabe stuende
        // hier zweimal derselbe Name, und `bestimmen_fuer_release` braeche mit
        // "Mehrere Developer-ID-Identitaeten gefunden" ab.
        let namen = gueltige_namen(abschnitt_der_treffer(BEIDE_ABSCHNITTE));
        assert_eq!(
            developer_id_namen(&namen),
            vec!["Developer ID Application: Kai Stalmann (QYMPYB7MWM)".to_owned()]
        );
    }

    #[test]
    fn der_abschnitt_der_treffer_behaelt_die_ungueltige_identitaet() {
        // Der Grund, den ersten Abschnitt zu lesen und nicht den zweiten: die
        // selbstsignierte Entwicklungsidentitaet steht nur dort. Mit `-v` oder
        // mit dem Abschnitt der Gueltigen faende `bestimmen` sie nicht mehr.
        let treffer = abschnitt_der_treffer(BEIDE_ABSCHNITTE_MIT_UNGUELTIGER);
        assert!(enthaelt_identitaet(treffer, ENTWICKLUNGSIDENTITAET));
        assert_eq!(gueltige_namen(treffer).len(), 2);
    }

    #[test]
    fn eine_ausgabe_ohne_zweiten_abschnitt_bleibt_unveraendert() {
        // Mit `-v` gibt `find-identity` weder eine Zeile `Policy:` noch zwei
        // Abschnitte aus; die Beschraenkung darf dort nichts wegnehmen.
        assert_eq!(abschnitt_der_treffer(GUELTIGE_EINE), GUELTIGE_EINE);
        assert_eq!(abschnitt_der_treffer(GUELTIGE_KEINE), GUELTIGE_KEINE);
    }
}
