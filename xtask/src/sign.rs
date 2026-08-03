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

/// Signiert das Buendel.
pub fn signieren(buendel: &Path, identitaet: &Identitaet) -> Result<(), Abbruch> {
    // --force, weil ein kopiertes Binaerprogramm je nach Zielplattform bereits
    // eine Signatur des Uebersetzers tragen kann; ohne die Marke bricht
    // codesign dann mit "is already signed" ab.
    let ausgabe = Command::new("/usr/bin/codesign")
        .args(["--force", "--sign", &identitaet.name])
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
fn auflisten() -> Result<String, Abbruch> {
    security_fragen(&["find-identity", "-p", "codesigning"])
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
}
