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
pub fn bestimmen() -> Result<Identitaet, Abbruch> {
    if let Some(name) = aus_umgebung() {
        return Ok(Identitaet {
            name,
            herkunft: UMGEBUNGSVARIABLE.to_owned(),
        });
    }
    let liste = auflisten()?;
    if enthaelt_identitaet(&liste, ENTWICKLUNGSIDENTITAET) {
        return Ok(Identitaet {
            name: ENTWICKLUNGSIDENTITAET.to_owned(),
            herkunft: "Schluesselbund".to_owned(),
        });
    }
    Err(Abbruch::Lauf(anleitung()))
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

/// Fragt den Schluesselbund nach den Identitaeten fuer die Codesignatur.
///
/// Ohne `-v`, und das ist kein Versehen. `-v` zeigt nur die als gueltig
/// bewerteten Identitaeten, und eine selbstsignierte Identitaet gilt ohne
/// gesetzte Vertrauenseinstellung als nicht vertrauenswuerdig
/// (`CSSMERR_TP_NOT_TRUSTED`). `codesign` signiert mit ihr trotzdem, und
/// `codesign --verify --strict` nimmt das Ergebnis an; am 260802 auf dem
/// Referenzgeraet gemessen. Mit `-v` wuerde der Bau eine Identitaet ablehnen,
/// die nachweislich traegt, und den Nutzer durch einen Vertrauensdialog
/// schicken, der fuer die Entwicklung nichts aendert.
fn auflisten() -> Result<String, Abbruch> {
    let ausgabe = Command::new("/usr/bin/security")
        .args(["find-identity", "-p", "codesigning"])
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("security laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "security find-identity ist gescheitert ({}): {}",
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

fn anleitung() -> String {
    format!(
        "Keine Signaturidentitaet gefunden, es entsteht kein Buendel.\n\
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
         \"Entwicklungsidentitaet anlegen\"; sie brauchen kein Xcode."
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
    fn die_anleitung_nennt_beide_wege() {
        let text = anleitung();
        assert!(text.contains(UMGEBUNGSVARIABLE));
        assert!(text.contains(ENTWICKLUNGSIDENTITAET));
        assert!(text.contains("README.md"));
    }
}
