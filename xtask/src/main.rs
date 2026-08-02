//! Bauwerkzeug fuer KRK: buendeln und signieren.
//!
//! ```text
//! cargo xtask bundle
//! ```
//!
//! Der Alias `xtask` steht im Abschnitt `[alias]` der `.cargo/config.toml` und
//! loest auf `run --package xtask --` auf; ein eingebautes Cargo-Kommando ist
//! das nicht.
//!
//! Warum es dieses Werkzeug ueberhaupt gibt: ein nacktes Binaerprogramm aus dem
//! Terminal erbt die Freigaben des Terminals und loest keine eigene Rueckfrage
//! des Systemmechanismus fuer Transparenz, Zustimmung und Kontrolle aus. Jede
//! Zusage zum Zugriff auf geschuetzte Ordner ist deshalb nur am signierten
//! Buendel pruefbar, und das Buendel steht daher vor dem ersten Fenster.

mod bundle;
mod sign;

use std::process::ExitCode;

const HILFE: &str = "\
xtask — Bauwerkzeug fuer KRK

  cargo xtask bundle
      Baut target/KRK.app: uebersetzt das Binaerziel, legt die Buendelstruktur
      an, kopiert resources/Info.plist mit eingesetzter Version, schreibt
      PkgInfo und signiert das Buendel lokal.

      Die Signaturidentitaet kommt aus der Umgebungsvariablen
      KRK_SIGN_IDENTITY. Fehlt sie, wird im Schluesselbund die lokale
      Identitaet \"KRK Entwicklung\" gesucht. Fehlt auch die, bricht der Bau
      mit einer Anleitung ab und weicht nicht auf eine Ad-hoc-Signatur aus.

  cargo xtask --hilfe
";

fn main() -> ExitCode {
    let argumente: Vec<String> = std::env::args().skip(1).collect();
    match ausfuehren(&argumente) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Abbruch::Aufruf(meldung)) => {
            eprintln!("xtask: {meldung}\n\n{HILFE}");
            ExitCode::from(2)
        }
        Err(Abbruch::Lauf(meldung)) => {
            eprintln!("xtask: {meldung}");
            ExitCode::FAILURE
        }
    }
}

/// Warum ein Lauf geendet hat.
///
/// Dieselbe Trennung wie in `krk-bench`: ein falscher Aufruf ist etwas anderes
/// als ein gescheiterter Bau, und wer das Werkzeug aus einem Skript heraus
/// ruft, will das am Rueckgabewert unterscheiden koennen.
#[derive(Debug)]
pub enum Abbruch {
    /// Die Befehlszeile stimmt nicht. Rueckgabewert 2.
    Aufruf(String),
    /// Der Bau selbst ist gescheitert. Rueckgabewert 1.
    Lauf(String),
}

fn ausfuehren(argumente: &[String]) -> Result<(), Abbruch> {
    let Some(befehl) = argumente.first() else {
        return Err(Abbruch::Aufruf("kein Unterbefehl genannt".to_owned()));
    };
    match befehl.as_str() {
        "bundle" => {
            if let Some(ueberzaehlig) = argumente.get(1) {
                return Err(Abbruch::Aufruf(format!(
                    "bundle kennt {ueberzaehlig:?} nicht"
                )));
            }
            let buendel = bundle::bauen()?;
            println!("Buendel: {}", buendel.display());
            Ok(())
        }
        "--hilfe" | "--help" | "-h" | "hilfe" => {
            println!("{HILFE}");
            Ok(())
        }
        anderer => Err(Abbruch::Aufruf(format!(
            "unbekannter Unterbefehl {anderer:?}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn worte(zeile: &[&str]) -> Vec<String> {
        zeile.iter().map(|wort| (*wort).to_owned()).collect()
    }

    fn ist_aufruffehler(ergebnis: Result<(), Abbruch>) -> bool {
        matches!(ergebnis, Err(Abbruch::Aufruf(_)))
    }

    #[test]
    fn ohne_unterbefehl_ist_der_aufruf_falsch() {
        assert!(ist_aufruffehler(ausfuehren(&[])));
    }

    #[test]
    fn ein_unbekannter_unterbefehl_ist_ein_aufruffehler() {
        assert!(ist_aufruffehler(ausfuehren(&worte(&["buendle"]))));
    }

    #[test]
    fn bundle_nimmt_keine_weiteren_marken() {
        assert!(ist_aufruffehler(ausfuehren(&worte(&["bundle", "--adhoc"]))));
    }

    #[test]
    fn die_hilfe_ist_kein_fehler() {
        assert!(ausfuehren(&worte(&["--hilfe"])).is_ok());
    }
}
