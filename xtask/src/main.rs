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

mod beglaubigung;
mod bundle;
mod git;
mod messen;
mod release;
mod sign;
mod veroeffentlichung;
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

      **Fuer die Weitergabe reicht dieses Buendel nicht.** Lokal signiert
      heisst: ohne gehaertete Laufzeitumgebung, ohne Beglaubigung, ohne
      angeheftetes Ticket, und uebersetzt allein fuer die Architektur der
      Baumaschine. Gatekeeper weist ein so gebautes Buendel auf einem
      zweiten Mac ab. Wer weitergeben will, nimmt ./release.sh <zahl>.
      Denselben Hinweis gibt der Lauf am Ende noch einmal aus, dann mit der
      Identitaet, die er gefunden hat.

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
      Baut das Auslieferungspaket (Schritt 23) in acht Stationen: prueft Tag
      und Arbeitsbaum, prueft die AppKit-Grenze (keine `use objc2`-Zeile
      ausserhalb von crates/krk-ui/src/appkit/), uebersetzt beide Mac-Ziele,
      fuegt sie mit lipo zu einer universellen Binaerdatei zusammen, baut
      dasselbe Buendel wie `bundle`, signiert mit einer
      Developer-ID-Identitaet und gehaerteter Laufzeitumgebung, reicht ueber
      \"xcrun notarytool submit --wait\" zur Beglaubigung ein, heftet das
      Ergebnis mit \"xcrun stapler staple\" an, packt es zu
      target/KRK-<version>.zip, **schiebt HEAD und refs/tags/v<version> zu
      origin** und haengt das Zip an eine oeffentliche GitHub-Releaseseite.
      Das Schieben ist die einzige Wirkung dieser Kette, die ueber das Geraet
      hinausgeht und sich nicht zuruecknehmen laesst. Dazwischen laufen drei
      Vorlaeufe, die einer spaeteren Station zuarbeiten: die Buendelvorlage,
      die Identitaetssuche und die Zielpruefung.

      Station 1 ist die Vorpruefung, und sie steht ganz vorn, damit ein
      Abbruch keinen Uebersetzungslauf kostet: HEAD muss einen Tag v<version>
      mit der Zahl aus [workspace.package] tragen, keine verfolgte Datei darf
      geaendert sein, und gh muss vorhanden und angemeldet sein. Unbeachtete
      Dateien zaehlen nicht mit. Sie liest allein; geschrieben hat der
      Halbschritt davor, \"cargo xtask version\". `cargo xtask bundle` fragt
      nach keinem der drei.

      Dass gh schon hier gefragt wird und nicht erst in Station 8, hat einen
      Grund: eine fehlende Voraussetzung soll auffallen, solange nichts
      geschehen ist, und am Kopf der achten Station waere die Einreichung bei
      Apple bereits gelaufen. Station 8 fragt trotzdem noch einmal, weil ihr
      zweiter Rufer keine Station vor sich hat.

      Die Identitaetssuche laeuft in denselben drei Stufen wie bei `bundle`,
      nur sucht die zweite nach dem Namensanfang \"Developer ID Application\".
      Die Beglaubigung braucht das vollstaendige Xcode und ein Schluesselbund-
      profil des Entwicklerkontos in KRK_NOTARY_PROFILE; fehlt eines, bricht
      allein sie ab, und das signierte Buendel bleibt liegen.

  ./certify-only.sh <zahl>
      Beglaubigt ein bereits gebautes target/KRK.app und tut sonst nichts.
      Der Weg fuer den Fall, dass ein Auslieferungslauf erst an der
      Beglaubigung gescheitert ist — etwa am Zeitueberlauf des Uploads zu
      Apple — und das universelle, mit Developer-ID signierte Buendel fertig
      dasteht. Das Skript reicht an \"make beglaubigen VERSION=<zahl>\" weiter.

  cargo xtask beglaubigen <zahl>
      Dasselbe ohne die zwei Huellen. Es prueft zweierlei am gebauten Buendel
      und reicht es dann ein: die Versionszahl gegen die Info.plist des
      Buendels, damit nicht ein altes target/KRK.app von vorgestern bei Apple
      landet, und den Signaturstand gegen die zwei Bedingungen der
      Beglaubigung, naemlich eine Developer-ID in der Signaturkette und die
      gehaertete Laufzeitumgebung. Danach laeuft dieselbe Station 7 wie bei
      release: \"xcrun notarytool submit --wait\" und \"xcrun stapler staple\".

      **Es baut nichts** — kein Uebersetzungslauf, kein lipo, keine Montage,
      keine Signierung; ohne Buendel bricht es ab und nennt release.

      **Und es prueft weder Tag noch Arbeitsbaum.** Genau darin liegt sein
      Zweck: Station 1 von release haelt eine Wiederholung in dieser Lage an,
      weil der Tag v<zahl> nach dem Lauf nicht mehr allein auf HEAD steht.
      Daraus folgt die Grenze: ein so beglaubigtes Buendel ist nicht durch die
      Vorpruefungen der Auslieferungskette gegangen, und es ist nicht gesagt,
      dass ein Tag den Stand benennt, aus dem es gebaut wurde.

  cargo xtask veroeffentlichen <zahl>
      Packt das beglaubigte target/KRK.app zu target/KRK-<zahl>.zip, schiebt
      HEAD und refs/tags/v<zahl> zur Gegenseite und legt eine oeffentliche
      GitHub-Releaseseite an, an der das Zip haengt und deren Text sagt, wie
      installiert wird, ohne die gemerkten Daten zu verlieren.

      **Es baut nichts und es beglaubigt nichts** — kein Uebersetzungslauf,
      kein lipo, keine Montage, keine Signierung, keine Einreichung bei
      Apple. Es fragt bloss nach, ob das Ticket schon am Buendel haengt, und
      es fragt an einer Datei und nicht bei einem Dienst; fehlt es, bricht es
      ab und nennt ./certify-only.sh <zahl>.

      Vorausgesetzt ist gh, das GitHub-Kommandozeilenwerkzeug, vorhanden und
      angemeldet. Das ist die dritte aeussere Voraussetzung der Kette, neben
      dem vollstaendigen Xcode und dem Entwicklerkonto; geprueft wird sie ganz
      zuerst, damit ein Abbruch weder ein Zip hinterlaesst noch etwas
      geschoben hat. Abhilfe: \"brew install gh\" und \"gh auth login\".

      Dieselbe Station faehrt `cargo xtask release` als achte. Der Unterschied
      zwischen beiden Wegen ist eine einzige Frage: dieser hier prueft selbst,
      ob v<zahl> auf HEAD steht, weil vor ihm keine Station stand. Den
      Arbeitsbaum prueft er nicht — das tut Station 1 von release.

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
        "beglaubigen" => beglaubigung::ausfuehren(&argumente[1..]),
        "veroeffentlichen" => veroeffentlichung::ausfuehren(&argumente[1..]),
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

    /// Der Abschnitt des Hilfetexts, der zu der Kopfzeile `kopf` gehoert.
    ///
    /// Die Hilfe ist in Abschnitte gegliedert: eine Kopfzeile mit genau zwei
    /// Leerzeichen Einzug, darunter ein Rumpf mit tieferem Einzug und
    /// Leerzeilen. Der Abschnitt endet an der naechsten Kopfzeile.
    ///
    /// **Warum die Proben den Abschnitt nehmen und nicht den ganzen Text.**
    /// Eine Wendung, die irgendwo in der Hilfe steht, sagt nichts darueber, ob
    /// sie beim richtigen Befehl steht. Seit dem 260821 sagen zwei Abschnitte
    /// "Es baut nichts", und eine Probe ueber den ganzen Text bliebe gruen,
    /// auch wenn der Satz beim falschen von beiden stuende.
    fn hilfeabschnitt(kopf: &str) -> &'static str {
        let anfang = HILFE
            .find(kopf)
            .unwrap_or_else(|| panic!("die Hilfe fuehrt keinen Abschnitt {kopf:?}"));
        let rest = &HILFE[anfang + kopf.len()..];
        let mut versatz = 0;
        for zeile in rest.split_inclusive('\n') {
            // Eine Kopfzeile ruecken genau zwei Leerzeichen ein; jede
            // Rumpfzeile rueckt tiefer ein, und eine Leerzeile rueckt gar
            // nicht ein.
            if zeile.starts_with("  ") && !zeile.starts_with("   ") {
                return &rest[..versatz];
            }
            versatz += zeile.len();
        }
        rest
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

    /// Der Nur-Beglaubigungsweg steht in der Verteilung und in der Hilfe.
    ///
    /// Dieselbe Bauart wie die Probe darueber: ohne Zahl ist es ein
    /// Aufruffehler und kein unbekannter Unterbefehl, der Befehl ist also
    /// verteilt worden.
    #[test]
    fn beglaubigen_steht_in_verteilung_und_hilfe() {
        let Err(Abbruch::Aufruf(meldung)) = ausfuehren(&worte(&["beglaubigen"])) else {
            panic!("beglaubigen ohne Zahl ist ein Aufruffehler");
        };
        assert!(meldung.contains("genau ein Argument"), "{meldung}");
        assert!(HILFE.contains("cargo xtask beglaubigen <zahl>"), "{HILFE}");
        assert!(HILFE.contains("./certify-only.sh <zahl>"), "{HILFE}");
    }

    /// Der Veroeffentlichungsweg steht in der Verteilung und in der Hilfe.
    ///
    /// Dieselbe Bauart wie die zwei Proben darueber: ohne Zahl ist es ein
    /// Aufruffehler und kein unbekannter Unterbefehl, der Befehl ist also
    /// verteilt worden.
    #[test]
    fn veroeffentlichen_steht_in_verteilung_und_hilfe() {
        let Err(Abbruch::Aufruf(meldung)) = ausfuehren(&worte(&["veroeffentlichen"])) else {
            panic!("veroeffentlichen ohne Zahl ist ein Aufruffehler");
        };
        assert!(meldung.contains("genau ein Argument"), "{meldung}");
        assert!(
            HILFE.contains("cargo xtask veroeffentlichen <zahl>"),
            "{HILFE}"
        );
    }

    /// Der Abschnitt zum neuen Weg sagt, dass er nichts baut (C6.2).
    ///
    /// Beides gehoert dazu und nicht nur das erste: wer den Befehl sucht,
    /// entscheidet zwischen ihm, `release` und `beglaubigen`, und die
    /// Entscheidung haengt daran, was er ausser dem Veroeffentlichen noch tut.
    #[test]
    fn der_abschnitt_zum_veroeffentlichen_sagt_dass_er_nichts_baut() {
        let abschnitt = hilfeabschnitt("cargo xtask veroeffentlichen <zahl>");
        assert!(abschnitt.contains("baut nichts"), "{abschnitt}");
        assert!(abschnitt.contains("beglaubigt nichts"), "{abschnitt}");
    }

    /// Der Abschnitt zu `bundle` sagt, was das Buendel fuer die Weitergabe
    /// bedeutet (C6.6).
    ///
    /// Der Abschlusshinweis des Laufs sagt es seit dem 260815, der Hilfetext
    /// schwieg dazu — und er ist die Stelle, die jemand **vor** dem Bau liest,
    /// wenn er den passenden Unterbefehl erst sucht. Defekt
    /// `shared/issues/260815-1436_*_der-hilfetext-zu-bundle-schweigt-zur-weitergabe-obwohl-die-ausgabe-des-befehls-sie-jetzt-nennt.md`.
    #[test]
    fn der_abschnitt_zu_bundle_nennt_die_weitergabe() {
        let abschnitt = hilfeabschnitt("cargo xtask bundle");
        assert!(abschnitt.contains("Weitergabe"), "{abschnitt}");
        assert!(abschnitt.contains("Gatekeeper"), "{abschnitt}");
        assert!(abschnitt.contains("./release.sh <zahl>"), "{abschnitt}");
    }

    /// Der Abschnitt zu `release` nennt das Schieben.
    ///
    /// **Warum gerade diese Wendung gehalten wird.** Das Schieben ist die
    /// einzige Wirkung des Wegs, die ueber das Geraet hinausgeht und sich nicht
    /// zuruecknehmen laesst; wer den Befehl nachschlaegt, bevor er ihn tippt,
    /// muss sie dort lesen. Bis zum 260821 sagte der Abschnitt allein
    /// „veroeffentlicht es als GitHub-Release", und das Schieben stand nur im
    /// Abschnitt zu `veroeffentlichen`, den nicht liest, wer `release` sucht
    /// (Durchsicht 260821-1346, F1).
    #[test]
    fn der_abschnitt_zu_release_nennt_das_schieben() {
        let abschnitt = hilfeabschnitt("cargo xtask release");
        assert!(abschnitt.contains("schiebt HEAD"), "{abschnitt}");
        assert!(abschnitt.contains("origin"), "{abschnitt}");
        assert!(abschnitt.contains("nicht zuruecknehmen"), "{abschnitt}");
    }

    /// Die Hilfe sagt, was der Weg nicht prueft, und was daraus folgt.
    ///
    /// Ein Weg, der Station 1 uebergeht, muss das dort sagen, wo jemand ihn
    /// nachschlaegt. Ohne die zweite Zusage liest sich die erste als
    /// Bequemlichkeit statt als Grenze.
    #[test]
    fn die_hilfe_nennt_die_grenze_des_nur_beglaubigungswegs() {
        assert!(
            HILFE.contains("weder Tag noch Arbeitsbaum"),
            "die Hilfe sagt nicht, was der Weg auslaesst"
        );
        assert!(
            HILFE.contains("Vorpruefungen der Auslieferungskette"),
            "die Hilfe nennt die Folge nicht"
        );
        let abschnitt = hilfeabschnitt("cargo xtask beglaubigen <zahl>");
        assert!(abschnitt.contains("Es baut nichts"), "{abschnitt}");
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
