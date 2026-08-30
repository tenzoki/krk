//! Die Woerter des Git-Bereichs: reine Funktionen mit Proben, und die einzige
//! Stelle, an der sie stehen.
//!
//! Vier Texte, und jeder beantwortet genau eine Frage der Anzeige: was oben
//! steht ([`kopfzeile`]), was darunter steht ([`zusammenfassung`]), was eine
//! Zeile der Verlaufsliste traegt ([`verlaufszeile`]), und was in einem Ordner
//! ohne Repository dasteht ([`KEIN_REPOSITORY`]).
//!
//! **Sie stehen im Kern und nicht in `krk-ui`, damit sie eine Probe haben.**
//! `krk-ui` hat kein Bibliotheksziel; eine Datei unter `crates/krk-ui/tests/`
//! erreicht nichts aus jener Kiste. Ein Satz, den der Nutzer liest, ohne dass
//! eine Probe seinen Wortlaut haelt, ist ein Satz, den die naechste Runde
//! unbemerkt aendert — dieselbe Herleitung, mit der die Meldungen der
//! Statuszeile hier wohnen.
//!
//! **Die Schreibweise ist die mit Umlauten**, wie der Baum sie seit dem 260826
//! fuehrt; die Frage ist als Datensatz offen
//! (`shared/decisions/260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md`),
//! und A14 der Runde 23 folgt bis dahin dem Baum.

use super::{Commit, Kopf, Marke};
use crate::leseprofil::bausteine::kalendertext;

/// Was der Git-Bereich in einem Ordner ohne Repository zeigt (A14).
///
/// Ein Satz und keine Fehlermeldung: die meisten Ordner, die KRK zeigt, liegen
/// in keinem Git-Baum, also ist das der Normalfall und nicht der Sonderfall.
pub const KEIN_REPOSITORY: &str = "Dieser Ordner liegt in keinem Git-Repository.";

/// Was an der Stelle der Zusammenfassung steht, solange kein Commit da ist
/// (A7, A14).
pub const OHNE_COMMIT: &str = "noch kein Commit";

/// Was an der Stelle der Zusammenfassung steht, wenn keine Marke uebrig bleibt
/// (A3, A14).
pub const UNVERAENDERT: &str = "unverändert";

/// Woran die Zusammenfassung sagt, dass sie den Ordner meint und nicht das
/// Repository (A3).
///
/// Der Zusatz ist nicht Zierat: der Status ist ueber die Pfadmuster auf den
/// angezeigten Ordner beschraenkt, und eine Zahl, die als Auskunft ueber das
/// ganze Repository gelesen wuerde, waere falsch. Was die Wahl kostet, gehoert
/// dazu: wer wissen will, ob das Repository als ganzes sauber ist, sieht es in
/// KRK nicht — die repositoryweite Zusammenfassung kostete in einem Baum mit
/// 100 000 Eintraegen gemessen 220 ms statt 12 ms.
const ORDNERZUSATZ: &str = " in diesem Ordner";

/// Wie eine Zeile ihre Angaben trennt.
///
/// Ein Mittelpunkt, wie ihn die Statuszeile der Operationen schon fuehrt
/// (`krk-ui/src/kommandos/operationen.rs`, `TRENNER`), und aus demselben
/// Grund: die Zeile ist einzeilig, und ein Umbruch waere dort abgeschnitten
/// statt gelesen.
const TRENNER: &str = " · ";

/// Die obere Zeile des Git-Bereichs (A6).
///
/// Vollstaendig ueber die vier Werte von [`Kopf`] und ohne Auffangzweig: ein
/// fuenfter Zustand des Kopfes soll den Bau anhalten und nicht still in einen
/// Satz fallen, der ihn nicht meint.
#[must_use = "die Zeile ist die Anzeige und keine Nebenwirkung"]
pub fn kopfzeile(kopf: &Kopf) -> String {
    match kopf {
        Kopf::Branch(name) | Kopf::OhneCommit(name) => name.clone(),
        Kopf::Abgeloest(kurzhash) => format!("{kurzhash} (abgelöst)"),
        Kopf::KeinRepository => KEIN_REPOSITORY.to_owned(),
    }
}

/// Die Zusammenfassung des Status fuer den angezeigten Ordner (A3).
///
/// Je Markenzustand die Zahl der betroffenen Eintraege, die Zustaende mit null
/// weggelassen, dazu der Zusatz, dass der Satz den Ordner meint. Bleibt keiner
/// uebrig, steht [`UNVERAENDERT`] — **ohne** den Zusatz, weil A14 den Wortlaut
/// dieses einen Satzes ausschreibt und er dort aus einem Wort besteht. Das ist
/// die einzige Stelle, an der A3 und A14 einander beruehren, und so ist sie
/// aufgeloest.
///
/// Die Reihenfolge der Zahlen ist die von [`Marke::ALLE`] und damit die der
/// Aufzaehlung; eine zweite Reihenfolge daneben waere eine zweite Liste.
#[must_use = "die Zusammenfassung ist die zweite Zeile des Git-Bereichs"]
pub fn zusammenfassung(marken: &[(String, Marke)]) -> String {
    let mut teile = Vec::new();
    for marke in Marke::ALLE {
        let zahl = marken.iter().filter(|(_, steht)| *steht == marke).count();
        if zahl > 0 {
            teile.push(format!("{zahl} {}", wort(marke)));
        }
    }
    if teile.is_empty() {
        return UNVERAENDERT.to_owned();
    }
    format!("{}{ORDNERZUSATZ}", teile.join(", "))
}

/// Eine Zeile der Verlaufsliste (A5).
///
/// Vier Angaben in einer Zeile, und die Kurzbeschreibung steht vorn: sie ist
/// die Angabe, an der der Nutzer einen Commit wiedererkennt, und sie bekommt
/// deshalb den Platz, der uebrig bleibt. Der Kurzhash steht hinten, weil er die
/// feste Breite hat und am ehesten wegfallen kann, wenn die Zeile zu schmal
/// wird.
///
/// Das Datum kommt aus [`kalendertext`] und wird hier nicht selbst geformt;
/// eine zweite Datumsform in diesem Vorhaben waere eine zweite Antwort auf
/// dieselbe Frage. Laesst der Zeitpunkt sich nicht in einen Kalendertag
/// uebersetzen, bleibt die Angabe leer statt einen Platzhalter zu erfinden.
#[must_use = "die Zeile ist die Anzeige und keine Nebenwirkung"]
pub fn verlaufszeile(commit: &Commit) -> String {
    let datum = kalendertext(commit.zeit).unwrap_or_default();
    let kurzhash: String = commit.id.to_hex_with_len(7).to_string();
    [
        commit.kurzbeschreibung.as_str(),
        commit.autor.as_str(),
        datum.as_str(),
        kurzhash.as_str(),
    ]
    .join(TRENNER)
}

/// Das Wort, mit dem die Zusammenfassung einen Markenzustand nennt.
///
/// Ausgeschriebene deutsche Woerter und nicht die Buchstaben der Spalte: die
/// Spalte ist schmal und braucht ein Zeichen, der Satz ist es nicht und wird
/// gelesen. Alle fuenf sind so gewaehlt, dass sie hinter einer Zahl stehen
/// koennen, ohne dekliniert zu werden.
fn wort(marke: Marke) -> &'static str {
    match marke {
        Marke::Geaendert => "geändert",
        Marke::Vorgemerkt => "vorgemerkt",
        Marke::Neu => "neu",
        Marke::Konflikt => "in Konflikt",
        Marke::Umbenannt => "umbenannt",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::ObjectId;
    use std::time::{Duration, SystemTime};

    /// Ein Commit mit festen Feldern; die Zeit ist ein fester Zeitpunkt.
    fn commit(kurzbeschreibung: &str, autor: &str, sekunden: u64) -> Commit {
        Commit {
            id: ObjectId::empty_blob(gix::hash::Kind::Sha1),
            kurzbeschreibung: kurzbeschreibung.to_owned(),
            nachricht: format!("{kurzbeschreibung}\n\nRumpf\n"),
            autor: autor.to_owned(),
            email: "wer@example.org".to_owned(),
            zeit: SystemTime::UNIX_EPOCH + Duration::from_secs(sekunden),
        }
    }

    fn marken(paare: &[(&str, Marke)]) -> Vec<(String, Marke)> {
        paare
            .iter()
            .map(|(name, marke)| ((*name).to_owned(), *marke))
            .collect()
    }

    /// C3.2, A3: je Zustand die Zahl, Zustaende mit null weggelassen, der
    /// Zusatz dahinter.
    #[test]
    fn die_zusammenfassung_nennt_nur_die_zustaende_mit_eintraegen() {
        let stand = marken(&[
            ("a", Marke::Geaendert),
            ("b", Marke::Geaendert),
            ("c", Marke::Neu),
        ]);
        assert_eq!(
            zusammenfassung(&stand),
            "2 geändert, 1 neu in diesem Ordner",
            "die Zusammenfassung nennt einen Zustand ohne Eintraege oder laesst den Zusatz weg"
        );
    }

    /// C3.2, A3: die Reihenfolge ist die der Aufzaehlung und nicht die des
    /// Eintreffens.
    #[test]
    fn die_zusammenfassung_folgt_der_reihenfolge_der_aufzaehlung() {
        let stand = marken(&[
            ("a", Marke::Umbenannt),
            ("b", Marke::Konflikt),
            ("c", Marke::Neu),
            ("d", Marke::Vorgemerkt),
            ("e", Marke::Geaendert),
        ]);
        assert_eq!(
            zusammenfassung(&stand),
            "1 geändert, 1 vorgemerkt, 1 neu, 1 in Konflikt, 1 umbenannt in diesem Ordner"
        );
    }

    /// C3.2, A14: bleibt keiner uebrig, steht das eine Wort da.
    #[test]
    fn ein_unveraenderter_ordner_traegt_genau_ein_wort() {
        assert_eq!(zusammenfassung(&[]), "unverändert");
        assert!(
            !zusammenfassung(&[]).contains(ORDNERZUSATZ),
            "A14 schreibt den Wortlaut dieses Satzes aus; er traegt den Zusatz nicht"
        );
    }

    /// C3.1, C3.6, C3.7, C6.1: die vier Lagen des Kopfes, jede mit ihrem Text.
    #[test]
    fn die_kopfzeile_traegt_je_lage_ihren_text() {
        assert_eq!(kopfzeile(&Kopf::Branch("main".to_owned())), "main");
        assert_eq!(
            kopfzeile(&Kopf::Abgeloest("a1b2c3d".to_owned())),
            "a1b2c3d (abgelöst)"
        );
        assert_eq!(kopfzeile(&Kopf::OhneCommit("master".to_owned())), "master");
        assert_eq!(kopfzeile(&Kopf::KeinRepository), KEIN_REPOSITORY);
    }

    /// C3.3, A5: vier Angaben in einer Zeile, die Kurzbeschreibung vorn.
    #[test]
    fn die_verlaufszeile_traegt_vier_angaben_in_dieser_reihenfolge() {
        let zeile = verlaufszeile(&commit("Der Bereich liest", "Kai Stalmann", 1_770_000_000));
        let teile: Vec<&str> = zeile.split(TRENNER).collect();
        assert_eq!(
            teile.len(),
            4,
            "die Zeile traegt nicht vier Angaben: {zeile}"
        );
        assert_eq!(teile[0], "Der Bereich liest");
        assert_eq!(teile[1], "Kai Stalmann");
        assert_eq!(
            teile[2],
            kalendertext(SystemTime::UNIX_EPOCH + Duration::from_secs(1_770_000_000))
                .expect("der Zeitpunkt liegt im Kalender"),
            "das Datum ist nicht das der einen Datumsform dieses Vorhabens"
        );
        assert_eq!(
            teile[3].len(),
            7,
            "der Kurzhash traegt nicht sieben Zeichen"
        );
    }

    /// A14: die drei Saetze stehen im Wortlaut und mit Umlauten da.
    #[test]
    fn die_drei_saetze_stehen_im_wortlaut_aus_a14() {
        assert_eq!(
            KEIN_REPOSITORY,
            "Dieser Ordner liegt in keinem Git-Repository."
        );
        assert_eq!(OHNE_COMMIT, "noch kein Commit");
        assert_eq!(UNVERAENDERT, "unverändert");
    }

    /// C5.3, E11: jeder der fuenf Zustaende traegt seinen Buchstaben, und keine
    /// zwei teilen sich einen.
    #[test]
    fn jede_marke_traegt_ihren_eigenen_buchstaben() {
        let erwartet = [
            (Marke::Geaendert, 'M'),
            (Marke::Vorgemerkt, 'S'),
            (Marke::Neu, 'N'),
            (Marke::Konflikt, 'K'),
            (Marke::Umbenannt, 'U'),
        ];
        for (marke, buchstabe) in erwartet {
            assert_eq!(
                marke.buchstabe(),
                buchstabe,
                "{marke:?} traegt den falschen Buchstaben"
            );
        }
        let mut buchstaben: Vec<char> = Marke::ALLE.iter().map(|m| m.buchstabe()).collect();
        buchstaben.sort_unstable();
        buchstaben.dedup();
        assert_eq!(
            buchstaben.len(),
            Marke::ALLE.len(),
            "zwei Marken teilen sich einen Buchstaben"
        );
    }

    /// Jede der fuenf traegt ein eigenes Wort; ohne die Zusicherung liest sich
    /// „2 neu, 1 neu" wie ein Fehler der Zaehlung.
    #[test]
    fn jede_marke_traegt_ihr_eigenes_wort() {
        let mut woerter: Vec<&str> = Marke::ALLE.iter().map(|m| wort(*m)).collect();
        woerter.sort_unstable();
        woerter.dedup();
        assert_eq!(
            woerter.len(),
            Marke::ALLE.len(),
            "zwei Marken teilen sich ein Wort"
        );
    }

    /// Die Rangfolge ist total: keine zwei Marken teilen sich einen Rang.
    ///
    /// Ohne die Zusicherung entschiede bei zwei gleichrangigen Marken die
    /// Reihenfolge des Statusstroms, und die ist nebenlaeufig erzeugt.
    #[test]
    fn die_rangfolge_der_marken_ist_total() {
        let mut raenge: Vec<u8> = Marke::ALLE.iter().map(|m| m.rang()).collect();
        raenge.sort_unstable();
        raenge.dedup();
        assert_eq!(
            raenge.len(),
            Marke::ALLE.len(),
            "zwei Marken teilen sich einen Rang"
        );
        assert!(
            Marke::Konflikt.rang() > Marke::Umbenannt.rang()
                && Marke::Umbenannt.rang() > Marke::Neu.rang(),
            "die Umbenennung faellt hinter Konflikt und vor Neu"
        );
    }
}
