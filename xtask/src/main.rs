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
mod messen;
mod release;
mod sign;

use std::process::ExitCode;

const HILFE: &str = "\
xtask — Bauwerkzeug fuer KRK

  cargo xtask bundle
      Baut target/KRK.app: uebersetzt das Binaerziel, legt die Buendelstruktur
      an, kopiert resources/Info.plist mit eingesetzter Version, schreibt
      PkgInfo und signiert das Buendel lokal.

      Die Signaturidentitaet sucht der Bau in drei Stufen: die
      Umgebungsvariable KRK_SIGN_IDENTITY, falls sie nichtleer ist; sonst
      eine Identitaet namens \"KRK Entwicklung\" im Schluesselbund; sonst
      die einzige gueltige Identitaet des Schluesselbunds, falls es genau
      eine gibt. Findet keine Stufe eine Identitaet, bricht der Bau mit
      einer Anleitung ab und weicht nicht auf eine Ad-hoc-Signatur aus.

  cargo xtask release
      Baut das Auslieferungspaket (Schritt 23): prueft die AppKit-Grenze
      (keine `use objc2`-Zeile ausserhalb von crates/krk-ui/src/appkit/),
      uebersetzt beide Mac-Ziele, fuegt sie mit lipo zu einer universellen
      Binaerdatei zusammen, baut dasselbe Buendel wie `bundle`, signiert mit
      einer Developer-ID-Identitaet und gehaerteter Laufzeitumgebung, reicht
      ueber \"xcrun notarytool submit --wait\" zur Beglaubigung ein und heftet
      das Ergebnis mit \"xcrun stapler staple\" an.

      Die Identitaetssuche laeuft in denselben drei Stufen wie bei `bundle`,
      nur sucht die zweite nach dem Namensanfang \"Developer ID Application\".
      Die Beglaubigung braucht das vollstaendige Xcode und ein Schluesselbund-
      profil des Entwicklerkontos in KRK_NOTARY_PROFILE; fehlt eines, bricht
      allein sie ab, und das signierte Buendel bleibt liegen.

  cargo xtask messen --alle --ordner-a P --ordner-b P --ordner100k P --kopierziel P
      Der eine Einstiegspunkt fuer beide Messstrecken (Schritt 21): baut das
      Buendel und faehrt den Abnahmelauf ueber alle zehn Zusagen L1 bis L10
      in krk-bench. Weitere Marken: --runden N, --ziel PFAD.

  cargo xtask messen --kopflos --ordner P [--kalt] [--ziel P]
      Die kopflose Strecke aus Schritt 3, unveraendert durchgereicht.

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
            let gebaut = bundle::bauen()?;
            println!("Buendel: {}", gebaut.buendel.display());
            Ok(())
        }
        "release" => release::ausfuehren(&argumente[1..]),
        "messen" => messen::ausfuehren(&argumente[1..]),
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
