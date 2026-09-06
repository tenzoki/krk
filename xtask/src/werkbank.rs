//! Formprüfung der Werkbank-Datensätze.
//!
//! Das Modul trägt keinen Code für den Auslieferungsweg, sondern zwei Proben
//! über `fusion-workbench/`. Es steht in `xtask`, weil das die Kiste ist, die
//! den Arbeitsbaum als Gegenstand hat, und weil eine Probe hier mit
//! `cargo test --workspace` mitläuft, ohne dass jemand ein eigenes Ziel rufen
//! muss.
//!
//! # Warum es die zwei Proben gibt
//!
//! Der Marker im Dateinamen und der Vermerk im Rumpf sagen dasselbe, und bis
//! zum 260906 hat sie nichts gegeneinander gehalten. Beide Richtungen des
//! Auseinanderlaufens sind belegt und einzeln abgelegt:
//!
//! - Ein geschlossener Datensatz **ohne** die Zeile `Resolved:` beantwortet
//!   jede Suche als unerledigt. 59 solche Fälle standen im Bestand, verteilt
//!   über drei Schreibweisen, und fünf Erhebungen haben die größte davon nicht
//!   gesehen (`260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md`).
//! - Ein offener Datensatz **mit** einer leeren Zeile `Resolved:` beantwortet
//!   jede Suche als erledigt. Das ist die teurere Richtung: ein übersehener
//!   Abschluss kostet eine Nachprüfung, ein vorgetäuschter kostet sie nicht,
//!   weil niemand sie ansetzt
//!   (`260826-1024_*_acht-offene-defektdatensaetze-tragen-eine-leere-resolved-zeile-und-antworten-jeder-suche-als-geschlossen.md`).
//!
//! # Was die Proben nicht tun
//!
//! Sie lesen den Vermerk nicht und beurteilen ihn nicht. Ob der Abschluss
//! inhaltlich trägt, entscheidet ein Abgleich und keine Zeichenkette.
//!
//! Und sie fallen nicht aus, wenn keine Werkbank dasteht: ein Auszug des
//! Baums ohne `fusion-workbench/` ist ein zulässiger Zustand — die
//! Messläufe dieses Projekts fahren in Wegwerfordnern —, und eine Probe, die
//! dort rot würde, prüfte die Umgebung statt des Bestands.

use std::fs;
use std::path::{Path, PathBuf};

/// Die Beschriftung, die `rules/fusion-workbench-conventions.md` unter
/// `## Inline State Tracking` für den Abschlussvermerk eines Defektdatensatzes
/// festlegt. Sie steht am Zeilenanfang; jede andere Gestalt — der Doppelpunkt
/// hinter dem Agenten, eine fette Beschriftung, eine Überschrift davor —
/// entgeht der Suche, für die sie da ist.
#[cfg(test)]
const VERMERK: &str = "Resolved:";

/// Sammelt jede Datei unter `wurzel`, deren Name `muster` enthält und die
/// unmittelbar in einem Ordner namens `issues` liegt.
///
/// Der Durchlauf hält genau einen Verzeichnisleser offen und merkt sich die
/// Unterordner als Pfad vor, wie `krk_core::verzeichnis::durchlauf` es für die
/// Anwendung tut. Der Grund ist derselbe: ein Abstieg, der den Leser der
/// übergeordneten Ebene offen hält, verbraucht bei tiefen Bäumen Deskriptoren
/// nach der Tiefe statt nach einer festen Zahl.
#[cfg(test)]
fn defektdatensaetze(wurzel: &Path, muster: &str) -> Vec<PathBuf> {
    let mut treffer = Vec::new();
    let mut offen = vec![wurzel.to_path_buf()];
    while let Some(ordner) = offen.pop() {
        let Ok(eintraege) = fs::read_dir(&ordner) else {
            continue;
        };
        let im_defektspeicher = ordner.file_name().is_some_and(|name| name == "issues");
        for eintrag in eintraege.flatten() {
            let pfad = eintrag.path();
            if pfad.is_dir() {
                offen.push(pfad);
                continue;
            }
            if !im_defektspeicher {
                continue;
            }
            let name = pfad.file_name().unwrap_or_default().to_string_lossy();
            if name.ends_with(".md") && name.contains(muster) {
                treffer.push(pfad.clone());
            }
        }
    }
    treffer.sort();
    treffer
}

/// Der Werkbankordner, oder `None`, wenn dieser Auszug des Baums keinen trägt.
#[cfg(test)]
fn werkbank() -> Option<PathBuf> {
    let pfad = crate::bundle::wurzel().join("fusion-workbench");
    pfad.is_dir().then_some(pfad)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Jeder `_c_`-Defektdatensatz trägt eine Zeile, die mit `Resolved:`
    /// beginnt.
    ///
    /// Die Probe zählt keine Quote, sondern nennt die Datensätze, an denen die
    /// Zeile fehlt: eine Zahl im Fehlertext wäre der zweite Ort, an dem
    /// dieselbe Auskunft steht, und der zweite ist der, der veraltet.
    #[test]
    fn jeder_geschlossene_defektdatensatz_traegt_einen_abschlussvermerk() {
        let Some(werkbank) = werkbank() else {
            return;
        };
        let ohne: Vec<_> = defektdatensaetze(&werkbank, "_c_")
            .into_iter()
            .filter(|pfad| {
                let Ok(inhalt) = fs::read_to_string(pfad) else {
                    return false;
                };
                !inhalt.lines().any(|zeile| zeile.starts_with(VERMERK))
            })
            .collect();
        assert!(
            ohne.is_empty(),
            "geschlossen ohne eine Zeile `{VERMERK}` am Zeilenanfang: {ohne:#?}"
        );
    }

    /// Kein `_o_`-Defektdatensatz trägt eine leere Zeile `Resolved:`.
    ///
    /// Die leere Zeile stammt aus der Vorlage. Sie trägt keine Aussage und
    /// beantwortet trotzdem jede Suche nach dem Abschlussvermerk mit ja.
    #[test]
    fn kein_offener_defektdatensatz_traegt_eine_leere_abschlusszeile() {
        let Some(werkbank) = werkbank() else {
            return;
        };
        let leer: Vec<_> = defektdatensaetze(&werkbank, "_o_")
            .into_iter()
            .filter(|pfad| {
                let Ok(inhalt) = fs::read_to_string(pfad) else {
                    return false;
                };
                inhalt
                    .lines()
                    .any(|zeile| zeile.trim_end() == VERMERK || zeile.trim_end() == "**Resolved:**")
            })
            .collect();
        assert!(
            leer.is_empty(),
            "offen mit einer leeren Zeile `{VERMERK}`: {leer:#?}"
        );
    }

    /// Der Durchlauf sieht allein die Datensätze und nicht die Nachbarn.
    ///
    /// Ohne diese Probe wäre eine leere Trefferliste nicht von einem
    /// Suchmuster zu unterscheiden, das nichts findet — und genau so sähe die
    /// Zusage der beiden Proben darüber aus, wenn `issues` sich einmal anders
    /// schriebe.
    #[test]
    fn der_durchlauf_findet_die_defektdatensaetze_und_sonst_nichts() {
        let Some(werkbank) = werkbank() else {
            return;
        };
        let treffer = defektdatensaetze(&werkbank, "_c_");
        assert!(
            !treffer.is_empty(),
            "kein geschlossener Defektdatensatz gefunden — das Suchmuster greift nicht"
        );
        for pfad in &treffer {
            assert!(
                pfad.parent()
                    .and_then(Path::file_name)
                    .is_some_and(|name| name == "issues"),
                "{pfad:?} liegt nicht in einem Defektspeicher"
            );
        }
    }
}
