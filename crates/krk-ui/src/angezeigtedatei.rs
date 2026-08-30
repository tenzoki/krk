//! Welche Datei "die angezeigte" ist: die der Vorschau, sonst die des
//! Editors, sonst keine.
//!
//! **Keine Zeile AppKit.** Wie [`crate::fenstertitel`] daneben rechnet dieses
//! Modul und zeichnet nicht. Wer die vier Eingaben zusammentraegt, ist
//! `Anwendungsdelegierter::ordner_der_datei_zeigen`; welche Datei daraus
//! folgt, entscheidet [`welche`], und das ist ohne Fenster pruefbar.
//!
//! ```text
//!  Vorschau sichtbar und haelt eine Datei ──> ihre Datei
//!  sonst Editor sichtbar und haelt eine   ──> seine Datei
//!  sonst                                  ──> None
//! ```
//!
//! # Ein Begriff, zwei Befehle
//!
//! Der Ordnersprung aus C2 fragt hier, und das Teilen aus C1 fragt dieselbe
//! Stelle, sobald der Fokus in der Vorschau oder im Editor steht. Zwei
//! Rechnungen nebeneinander waeren zwei Antworten auf eine Frage, und die
//! zweite fiele erst am Buendel auf.

use std::path::PathBuf;

/// Der Pfad der angezeigten Datei, oder `None`, wenn keine angezeigt wird.
///
/// # Warum die Sichtbarkeit entscheidet und nicht das Halten
///
/// **Ein verdraengter Editor behaelt seinen Stand.** Wer die Vorschau
/// einblendet, nimmt dem Editor die Flaeche, nicht seine Datei; beide koennen
/// danach einen Pfad halten, und "wer haelt eine Datei?" hat dann zwei
/// Antworten. Sichtbar ist dagegen nach
/// [`Bereich::flaeche`](crate::fenstermodell::Bereich::flaeche)
/// hoechstens einer von beiden, und damit hat die Frage genau eine Antwort.
/// Wer diese Abfrage spaeter auf `haelt_datei` umbaut, holt sich die zwei
/// Antworten zurueck.
///
/// Die Fallunterscheidung ueber die vier Eingaben ist damit vollstaendig und
/// ueberschneidungsfrei: die beiden Zweige koennen nicht zugleich greifen,
/// weil ihre erste Bedingung sich ausschliesst, und was durch beide faellt,
/// ist `None`.
///
/// **Ein sichtbarer Bereich ohne Datei gewinnt nicht.** Eine sichtbare
/// Vorschau, die nichts haelt, laesst den Editor an die Reihe — der ist dann
/// unsichtbar, haelt aber vielleicht etwas, und dieser Fall faellt trotzdem
/// auf `None`, weil die zweite Bedingung ebenfalls die Sichtbarkeit fragt.
pub fn welche(
    vorschau_sichtbar: bool,
    vorschau_pfad: Option<PathBuf>,
    editor_sichtbar: bool,
    editor_pfad: Option<PathBuf>,
) -> Option<PathBuf> {
    if vorschau_sichtbar && let Some(pfad) = vorschau_pfad {
        return Some(pfad);
    }
    if editor_sichtbar && let Some(pfad) = editor_pfad {
        return Some(pfad);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vorschaudatei() -> PathBuf {
        PathBuf::from("/Users/k1/Bilder/schirm.png")
    }

    fn editordatei() -> PathBuf {
        PathBuf::from("/Users/k1/Projekte/krk/README.md")
    }

    /// Alle acht Kombinationen der vier Eingaben, an einem Stueck.
    ///
    /// Die Tafel steht zusammen da, damit ein fehlender Fall auffaellt: zwei
    /// Wahrheitswerte und zwei Pfade, die es gibt oder nicht, ergeben acht
    /// Lagen, und jede traegt hier ihre Antwort. Die Lage "beide sichtbar"
    /// kommt nicht vor, weil `Bereich::flaeche` sie ausschliesst: beide
    /// tragen `Flaeche::RechterRand`. Die Tafel fragt deshalb je Bereich
    /// getrennt.
    #[test]
    fn alle_acht_kombinationen_tragen_ihre_antwort() {
        let v = vorschaudatei();
        let e = editordatei();

        // Die Vorschau ist sichtbar.
        assert_eq!(
            welche(true, Some(v.clone()), false, Some(e.clone())),
            Some(v.clone()),
            "die sichtbare Vorschau mit Datei gewinnt"
        );
        assert_eq!(
            welche(true, Some(v.clone()), false, None),
            Some(v.clone()),
            "die sichtbare Vorschau mit Datei gewinnt auch ohne Editordatei"
        );
        assert_eq!(
            welche(true, None, false, Some(e.clone())),
            None,
            "die sichtbare Vorschau ohne Datei laesst den unsichtbaren Editor nicht gewinnen"
        );
        assert_eq!(welche(true, None, false, None), None);

        // Der Editor ist sichtbar.
        assert_eq!(
            welche(false, Some(v.clone()), true, Some(e.clone())),
            Some(e.clone()),
            "der sichtbare Editor gewinnt gegen die unsichtbare Vorschau"
        );
        assert_eq!(
            welche(false, None, true, Some(e.clone())),
            Some(e.clone()),
            "der sichtbare Editor mit Datei gewinnt"
        );
        assert_eq!(
            welche(false, Some(v.clone()), true, None),
            None,
            "der sichtbare Editor ohne Datei laesst die unsichtbare Vorschau nicht gewinnen"
        );
        assert_eq!(welche(false, None, true, None), None);
    }

    /// Ein unsichtbarer Editor mit gehaltener Datei gewinnt nicht.
    ///
    /// Der Fall, um dessentwillen die Funktion nach der Sichtbarkeit fragt und
    /// nicht nach dem Halten: die Vorschau steht, zeigt aber nichts, und
    /// dahinter haelt ein verdraengter Editor weiter seine Datei. Ein Sprung
    /// in deren Ordner waere eine Antwort auf etwas, das der Nutzer nicht
    /// sieht.
    #[test]
    fn ein_verdraengter_editor_mit_datei_gewinnt_nicht() {
        assert_eq!(
            welche(true, None, false, Some(editordatei())),
            None,
            "die Datei des ausgeblendeten Editors ist nicht die angezeigte"
        );
    }

    /// Steht keiner der beiden Bereiche, gibt es keine angezeigte Datei.
    ///
    /// Die Lage des Nutzers, der den Editor abgeschaltet und die Vorschau
    /// ausgeblendet hat. Der Satz der Statuszeile spricht deshalb vom
    /// Ergebnis und nicht von einer Ursache (C2, fuenftes Kriterium).
    #[test]
    fn ohne_sichtbaren_bereich_gibt_es_keine_angezeigte_datei() {
        assert_eq!(
            welche(false, Some(vorschaudatei()), false, Some(editordatei())),
            None
        );
    }

    /// Die Funktion prueft nicht nach.
    ///
    /// Wie [`crate::fenstertitel::titel`] fragt sie kein Dateisystem: sie gibt
    /// wieder, was auf dem Schirm steht. Ob die Datei im Zielordner noch
    /// steht, beantwortet erst der Lesevorgang (C2, sechstes Kriterium).
    #[test]
    fn eine_verschwundene_datei_bleibt_die_angezeigte() {
        let fort = PathBuf::from("/Volumes/abgezogen/notiz.txt");
        assert!(
            !fort.exists(),
            "die Probe braucht einen Pfad, den es nicht gibt"
        );
        assert_eq!(welche(true, Some(fort.clone()), false, None), Some(fort));
    }
}
