//! Der eine Zugang zu `git`.
//!
//! **Hier steht der einzige Prozessaufruf von `git` im ganzen Baum.** Die Probe
//! `xtask_ruft_git_an_genau_einer_stelle` in `release` haelt die Zahl auf eins.
//! Bis zum 260813 stand der Aufruf in `release`, weil `release` der einzige
//! Abnehmer war; seit `version` einen Stand eintraegt und taggt, sind es zwei,
//! und der Aufruf ist an die Stelle gewandert, die beide gemeinsam haben. Eine
//! zweite waere die zweite Wahrheit darueber, wie tief das Bauwerkzeug in den
//! Zustand des Arbeitsbaums schaut — und seit das Werkzeug auch schreibt, die
//! zweite Wahrheit darueber, was es schreiben darf.
//!
//! **Lesen und Schreiben stehen hier verschieden da, und das ist Absicht.** Die
//! drei Fragen [`VERZEICHNIS`], [`TAGS_AUF_HEAD`] und [`STAND`] sind
//! Konstanten: sie tragen keinen Wert aus der Befehlszeile, also laesst sich an
//! ihnen einmal nachsehen, dass keine von ihnen schreibt, und die Probe
//! `keine_der_drei_fragen_schreibt` tut das Wort fuer Wort. Die schreibenden
//! Kommandos koennen keine Konstanten sein, weil jedes einen Wert aus der
//! Befehlszeile traegt; sie entstehen als reine Funktionen und werden ebenso
//! Wort fuer Wort nachgesehen.
//!
//! **Es sind seit dem 260821 drei, und sie entstehen an zwei Orten.** `version`
//! baut das Setzen des Tags und den Eintrag, `veroeffentlichung` das Schieben.
//! Nachgesehen werden alle drei an einer einzigen Stelle,
//! `version::tests::die_schreibenden_kommandos_tragen_keine_gewalt`: sie liest
//! die Listen, die hier ankommen, und keine anderen. Genau daran haengt ihre
//! Aussagekraft, und darum ist sie nicht mit dem dritten Bauer mitgewandert.
//!
//! Die zwei kleinen Lesehilfen [`geaenderte_dateien`] und [`tag_steht`] stehen
//! ebenfalls hier und nicht bei ihren Abnehmern: `release` und `version`
//! stellen dieselben zwei Fragen, und zwei Auslegungen derselben Ausgabe waeren
//! zwei Antworten darauf, was ein sauberer Arbeitsbaum ist.

use std::path::Path;
use std::process::Command;

use crate::Abbruch;

/// Die erste der drei Fragen: liegt ueberhaupt ein Git-Verzeichnis vor?
///
/// Sie steht getrennt, damit die Antwort darauf nicht am Wortlaut einer
/// Fehlermeldung der beiden anderen haengt.
pub(crate) const VERZEICHNIS: &[&str] = &["rev-parse", "--git-dir"];

/// Die zweite Frage: welche Tags stehen auf HEAD?
///
/// `--points-at HEAD` fragt allein; `git tag` legt erst dann einen Tag an,
/// wenn ein Name dabeisteht. Annotierte und leichte Tags stehen in dieser
/// Ausgabe gleich, und das ist die Zusage aus C3.3: gefragt ist, welcher Name
/// auf HEAD steht, und nicht, wie er entstanden ist.
pub(crate) const TAGS_AUF_HEAD: &[&str] = &["tag", "--points-at", "HEAD"];

/// Die dritte Frage: welche verfolgten Dateien sind geaendert?
///
/// `--porcelain` meldet vorgemerkte und nicht vorgemerkte Aenderungen in
/// derselben Form und fuehrt geloeschte verfolgte Dateien mit;
/// `--untracked-files=no` haelt unbeachtete Dateien draussen, wie es der
/// Entscheid vom 260813-1010 verlangt.
///
/// **Ohne Pfadfilter, und das ist eine Festlegung.** Gezaehlt wird das ganze
/// Verzeichnis. Eine Liste der bauwirksamen Ordner muesste jemand pflegen, und
/// sie zu ergaenzen zu vergessen ist die zweite Art, eine Pruefung im
/// Vorbeigehen zu verlieren — dieselbe Erwaegung, die schon bei
/// `release::GRENZWURZEL` steht. Was der Verzicht kostet, steht in
/// `shared/issues/260813-1515_*_die-auslieferungspruefung-schlaegt-nach-jeder-agentensitzung-an-weil-vier-werkbankdateien-verfolgt-sind.md`;
/// beide Abnehmer zaehlen die betroffenen Dateien deshalb nicht nur, sie nennen
/// sie beim Namen.
pub(crate) const STAND: &[&str] = &["status", "--porcelain", "--untracked-files=no"];

/// Ruft `git` im Projektverzeichnis und liefert seine Standardausgabe.
///
/// Nach dem Muster von `security_fragen` in `sign`: absoluter Pfad, weil der
/// Baum jedes Systemwerkzeug so ruft, `.current_dir` auf die Projektwurzel,
/// weil die Antwort sonst am Arbeitsverzeichnis des Aufrufers haengt.
/// Startfehler und ein Rueckgabewert ungleich null werden beide zum
/// Laufabbruch.
///
/// Ob der Aufruf liest oder schreibt, entscheidet allein die Argumentliste;
/// diese Funktion sieht sie nicht an. Wer sie ansieht, steht im Modulkopf.
pub(crate) fn rufen(wurzel: &Path, argumente: &[&str]) -> Result<String, Abbruch> {
    let ausgabe = Command::new("/usr/bin/git")
        .args(argumente)
        .current_dir(wurzel)
        .output()
        .map_err(|fehler| Abbruch::Lauf(format!("git laesst sich nicht starten: {fehler}")))?;
    if !ausgabe.status.success() {
        return Err(Abbruch::Lauf(format!(
            "git {} ist gescheitert ({}): {}",
            argumente.join(" "),
            ausgabe.status,
            String::from_utf8_lossy(&ausgabe.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&ausgabe.stdout).into_owned())
}

/// Die geaenderten verfolgten Dateien aus der Ausgabe von [`STAND`].
///
/// Eine Datei je Zeile, samt der zweistelligen Zustandsspalte, die
/// `--porcelain` voranstellt. Leerzeilen fallen weg; der Rest bleibt so
/// stehen, wie `git` ihn schreibt, damit die Meldung die Datei so nennt, wie
/// der Nutzer sie im naechsten `git status` wiederfindet.
pub(crate) fn geaenderte_dateien(ausgabe: &str) -> Vec<&str> {
    ausgabe
        .lines()
        .map(str::trim_end)
        .filter(|zeile| !zeile.trim().is_empty())
        .collect()
}

/// Steht `erwartet` in der Ausgabe von [`TAGS_AUF_HEAD`]?
///
/// Verglichen wird die ganze Zeile und nicht ihr Anfang: sonst deckte
/// `v0.1.0-rc1` die Auslieferung von `0.1.0`.
pub(crate) fn tag_steht(tags_auf_head: &str, erwartet: &str) -> bool {
    tags_auf_head.lines().any(|zeile| zeile.trim() == erwartet)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die drei Fragen lesen, jede einzeln nachgesehen.
    ///
    /// `tag` steht in der Liste der schreibenden Unterbefehle nicht, weil es
    /// beides kann: mit `--points-at` fragt es, mit einem Namen legt es an.
    /// Genau diese Unterscheidung prueft der zweite Teil.
    #[test]
    fn keine_der_drei_fragen_schreibt() {
        const SCHREIBENDE: [&str; 14] = [
            "add",
            "commit",
            "checkout",
            "reset",
            "restore",
            "clean",
            "push",
            "stash",
            "merge",
            "rebase",
            "init",
            "update-ref",
            "branch",
            "notes",
        ];
        for frage in [VERZEICHNIS, TAGS_AUF_HEAD, STAND] {
            let unterbefehl = frage[0];
            assert!(
                !SCHREIBENDE.contains(&unterbefehl),
                "git {unterbefehl} schreibt"
            );
            if unterbefehl == "tag" {
                assert!(
                    frage.contains(&"--points-at"),
                    "git tag ohne --points-at legt einen Tag an: {frage:?}"
                );
                assert!(
                    frage
                        .iter()
                        .all(|wort| wort.starts_with("--") || *wort == "tag" || *wort == "HEAD"),
                    "ein Name hinter git tag legt ihn an: {frage:?}"
                );
            }
        }
    }

    /// Unbeachtete Dateien bleiben aussen vor, und das haengt an der Marke.
    #[test]
    fn die_standabfrage_laesst_unbeachtete_dateien_aussen_vor() {
        assert!(STAND.contains(&"--untracked-files=no"), "{STAND:?}");
        // Kein Pfadfilter: die Abfrage traegt kein `--` und keinen Pfad.
        assert!(
            !STAND.contains(&"--"),
            "ein Pfadfilter waere eine Liste, die jemand pflegen muss: {STAND:?}"
        );
        assert!(
            STAND
                .iter()
                .all(|wort| wort.starts_with("--") || *wort == "status"),
            "{STAND:?}"
        );
    }

    #[test]
    fn leerzeilen_zaehlen_nicht_als_geaenderte_datei() {
        assert!(geaenderte_dateien("").is_empty());
        assert!(geaenderte_dateien("\n\n").is_empty());
        assert_eq!(
            geaenderte_dateien("M  Cargo.toml\n M README.md\n"),
            vec!["M  Cargo.toml", " M README.md"]
        );
    }

    #[test]
    fn ein_tag_gilt_nur_bei_ganzer_uebereinstimmung() {
        assert!(tag_steht("v0.1.0\n", "v0.1.0"));
        assert!(tag_steht("anderer\nv0.1.0\nnoch-einer\n", "v0.1.0"));
        assert!(!tag_steht("v0.1.0-rc1\nv0.1.10\n", "v0.1.0"));
        assert!(!tag_steht("", "v0.1.0"));
    }
}
