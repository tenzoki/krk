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
mod git;
mod messen;
mod release;
mod sign;
mod version;

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

  ./release.sh <zahl>
      Der ganze Auslieferungsweg in einem Kommando mit einem Argument, der
      Versionszahl. Das Skript ist kein drittes Bauwerkzeug: es reicht an
      \"make ausliefern VERSION=<zahl>\" weiter, und das faehrt die beiden
      Kommandos darunter nacheinander.

  cargo xtask version <zahl>
      Setzt die Versionszahl im Feld version unter [workspace.package] der
      Wurzel-Cargo.toml, traegt Cargo.toml und Cargo.lock als eine Aenderung
      ein und setzt den Tag v<zahl> auf HEAD. Genau drei Zahlenteile, ohne
      fuehrendes \"v\": das traegt allein der Tag.

      Geprueft wird vor dem ersten Schreiben: die Zahl, das Git-Verzeichnis,
      der unveraenderte Arbeitsbaum und der freie Tagname. Ist der
      Arbeitsbaum geaendert, nennt der Abbruch jede betroffene Datei. Steht
      die Zahl schon und fehlt nur der Tag, wird nur getaggt; steht beides,
      ist nichts zu tun. Derselbe Aufruf ein zweites Mal traegt also nichts
      doppelt ein.

      Warum es zwei Kommandos sind und nicht eines: xtask liest die Zahl beim
      Uebersetzen ueber env!(\"CARGO_PKG_VERSION\"). Der Prozess muss enden,
      damit cargo das Werkzeug mit der neuen Zahl neu uebersetzt, und Station
      1 von release vergleicht danach die neu eingebackene Zahl mit dem Tag.

  cargo xtask release
      Baut das Auslieferungspaket (Schritt 23) in sieben Stationen: prueft Tag
      und Arbeitsbaum, prueft die AppKit-Grenze (keine `use objc2`-Zeile
      ausserhalb von crates/krk-ui/src/appkit/), uebersetzt beide Mac-Ziele,
      fuegt sie mit lipo zu einer universellen Binaerdatei zusammen, baut
      dasselbe Buendel wie `bundle`, signiert mit einer
      Developer-ID-Identitaet und gehaerteter Laufzeitumgebung, reicht ueber
      \"xcrun notarytool submit --wait\" zur Beglaubigung ein und heftet das
      Ergebnis mit \"xcrun stapler staple\" an. Dazwischen laufen drei
      Vorlaeufe, die einer spaeteren Station zuarbeiten: die Buendelvorlage,
      die Identitaetssuche und die Zielpruefung.

      Station 1 ist die Vorpruefung, und sie steht ganz vorn, damit ein
      Abbruch keinen Uebersetzungslauf kostet: HEAD muss einen Tag v<version>
      mit der Zahl aus [workspace.package] tragen, und keine verfolgte Datei
      darf geaendert sein. Unbeachtete Dateien zaehlen nicht mit. Sie liest
      allein; geschrieben hat der Halbschritt davor, \"cargo xtask version\".
      `cargo xtask bundle` fragt weder nach dem einen noch nach dem anderen.

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
            // Der Abschlusshinweis haengt an diesem Unterbefehl und nicht an
            // `bundle::bauen`: `messen --alle` baut dasselbe Buendel fuer eine
            // Messung und gibt es nicht weiter, und `release` faehrt genau den
            // Weg, auf den der Hinweis zeigt. Was er sagt, entscheidet die Art
            // der Identitaet; siehe [`sign::weitergabehinweis`].
            //
            // Die Architektur der Baumaschine steht schon beim Uebersetzen
            // fest, wird aber unter dem Namen gemeldet, den `lipo` benutzt:
            // wer den Hinweis nachprueft, tut es mit `lipo`, und das schreibt
            // `arm64`, wo Rust `aarch64` sagt. Die Umrechnung liest die Namen
            // aus `release` und legt keine zweite Liste an.
            println!(
                "{}",
                sign::weitergabehinweis(
                    &gebaut.identitaet.name,
                    release::lipo_name(std::env::consts::ARCH)
                )
            );
            Ok(())
        }
        "version" => version::ausfuehren(&argumente[1..]),
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

    /// Der Unterbefehl steht in der Verteilung und in der Hilfe.
    ///
    /// Ein Befehl, den die Hilfe nicht nennt, findet niemand; die Probe haelt
    /// beide Stellen aneinander, ohne den Wortlaut festzuschreiben.
    #[test]
    fn version_steht_in_verteilung_und_hilfe() {
        // Ohne Zahl ist es ein Aufruffehler und kein unbekannter Unterbefehl:
        // der Befehl ist also verteilt worden.
        let Err(Abbruch::Aufruf(meldung)) = ausfuehren(&worte(&["version"])) else {
            panic!("version ohne Zahl ist ein Aufruffehler");
        };
        assert!(meldung.contains("genau ein Argument"), "{meldung}");
        assert!(HILFE.contains("cargo xtask version <zahl>"), "{HILFE}");
        assert!(HILFE.contains("./release.sh <zahl>"), "{HILFE}");
    }

    /// Der ueberholte Satz steht nirgends mehr in der Hilfe.
    ///
    /// Bis zum 260813-1534 sagte sie, der Nutzer setze den Tag und das
    /// Werkzeug erzeuge nie einen. Der Entscheid
    /// `shared/decisions/260813-1534_*_darf-das-bauwerkzeug-den-tag-setzen-und-die-auslieferung-in-einem-kommando-fahren.md`
    /// hat das zurueckgenommen.
    #[test]
    fn die_hilfe_traegt_den_ueberholten_satz_nicht_mehr() {
        for satz in ["erzeugt nie einen", "setzt der Nutzer von Hand"] {
            assert!(!HILFE.contains(satz), "die Hilfe sagt noch {satz:?}");
        }
    }

    #[test]
    fn die_hilfe_ist_kein_fehler() {
        assert!(ausfuehren(&worte(&["--hilfe"])).is_ok());
    }
}
