//! Was im Fenstertitel steht: der absolute Pfad dessen, woran der Nutzer
//! gerade arbeitet (C11).
//!
//! **Keine Zeile AppKit.** Wie [`crate::fenstermodell`] und
//! [`crate::kommandos`] rechnet dieses Modul und zeichnet nicht. Wer den Titel
//! schreibt, ist `Anwendungsdelegierter::titel_nachziehen`; was darin steht,
//! entscheidet [`titel`], und das ist ohne Fenster pruefbar.
//!
//! **C11 ist seit der Titelleisten-Runde fortgeschrieben und nicht ergaenzt.**
//! Wer wissen will, was der Titel zusagt, liest die elf Abnahmekriterien im
//! Spec jener Runde
//! (`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1037_*_spec-titelleiste-fuehrt-version-und-semantische-tags.md`,
//! Abschnitt `### C2`) und nicht mehr die der Runde 2; zwei der elf sind
//! geaendert, neun stehen woertlich wie dort. Zwei Zusagen ueber dieselbe
//! Titelleiste waeren zwei Wahrheiten. Der Anlass: Name und Version stehen
//! seither in einem eigenen Bereich links in der Leiste
//! ([`crate::appkit::titelzusatz`]), und was `setTitle:` bekommt, ist
//! unveraendert allein das Ergebnis von [`titel`] — diese Datei aendert sich
//! dafuer in keiner Zeile ausser dieser.
//!
//! ```text
//!  Fokus ──┬─ Dateifenster ──> der angezeigte Ordner
//!          ├─ Leiste ────────> der Ordner des aktiven Dateifensters
//!          ├─ Editor ────────> seine Datei, sonst der Ordner
//!          ├─ Vorschau ──────> ihre Datei, sonst der Ordner
//!          └─ Anderswo ──────> None, also: den Titel stehen lassen
//! ```
//!
//! # Warum der Fokus entscheidet und nicht das aktive Dateifenster
//!
//! KRK fuehrt genau einen Fokus, und C9 macht ihn im selben Zug sichtbar. Der
//! Rahmen sagt damit, **wo** der Nutzer arbeitet, und der Titel, **woran**;
//! beide lesen dieselbe Angabe, und ein zweiter Begriff daneben entsteht
//! nicht. Die Gegenmoeglichkeit, den Titel an das aktive Dateifenster zu
//! binden, braeuchte zwei Begriffe und zeigte den Ordner des Dateifensters,
//! waehrend der Nutzer im Editor tippt.
//!
//! # Zwei Antworten, die keine Auffangzweige sind
//!
//! Die Leiste und ein Bereich ohne Pfad fallen auf den Ordner des aktiven
//! Dateifensters, und beide Male aus einem Grund und nicht aus Verlegenheit.
//! Die Auswahl in der Leiste **setzt** den Ordner des aktiven Dateifensters;
//! ihr Zusammenhang ist also genau jenes Fenster. Und wer in einem Bereich
//! steht, der nichts haelt, entscheidet seine naechste Handlung an ebendiesem
//! Ordner.
//!
//! # Was dieses Modul ausdruecklich nicht tut
//!
//! **Es kuerzt nicht.** Kein Ersetzen des Benutzerordners durch eine Tilde,
//! kein Auslassen von Zwischenordnern: der Zweck des Titels ist das Lesen und
//! Weiterreichen des Pfades, und der Nutzer hat am 260809 den absoluten Pfad
//! verlangt. Was der Titelbalken nicht fasst, kuerzt macOS selbst.
//!
//! **Es prueft nicht nach.** Die Funktion bekommt Pfade und fragt kein
//! Dateisystem. Zeigt ein Dateifenster einen Ordner, den es nicht mehr gibt,
//! steht dieser Pfad weiter im Titel; der Titel gibt wieder, was auf dem Schirm
//! steht. Das elfte Abnahmekriterium von C11 verlangt es, und es faellt daraus
//! an, dass diese Funktion rein ist.

use std::path::Path;

use crate::kommandos::fokus::Fokus;

/// Der Fenstertitel zu diesem Fokus, oder `None` fuer "stehen lassen".
///
/// **Eine erschoepfende Fallunterscheidung ueber die fuenf Fokuswerte, ohne
/// Auffangzweig.** Ein sechster Wert haelt den Bau an und erzwingt die Antwort
/// darauf, was der Titel dann zeigt.
///
/// `None` heisst nicht "leerer Titel", sondern "nicht anfassen". Es ist die
/// Antwort fuer [`Fokus::Anderswo`], und ein stehendes Blatt ergibt genau
/// diesen Wert: das achte Abnahmekriterium von C11 verlangt, dass der Titel
/// dann stehen bleibt, wie er davor stand, und es faellt hier ohne eigenen Bau
/// an.
///
/// `aktiver_ordner` ist der Ordner, den das **aktive** Dateifenster zeigt. Er
/// wird in vier der fuenf Faelle gebraucht und deshalb nicht als `Option`
/// gefuehrt: ein Dateifenster zeigt immer einen Ordner.
pub fn titel(
    fokus: Fokus,
    aktiver_ordner: &Path,
    editordatei: Option<&Path>,
    vorschaudatei: Option<&Path>,
) -> Option<String> {
    let pfad = match fokus {
        Fokus::Dateifenster => aktiver_ordner,
        // Die Auswahl in der Leiste setzt den Ordner des aktiven
        // Dateifensters; sie hat keinen eigenen Pfad, sondern genau diesen.
        Fokus::Leiste => aktiver_ordner,
        Fokus::Editor => editordatei.unwrap_or(aktiver_ordner),
        Fokus::Vorschau => vorschaudatei.unwrap_or(aktiver_ordner),
        Fokus::Anderswo => return None,
    };
    Some(pfad.display().to_string())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn ordner() -> PathBuf {
        PathBuf::from("/Users/k1/Projekte")
    }

    /// Jeder der fuenf Fokuswerte bekommt seine Antwort, und `Anderswo` keine.
    ///
    /// Die Tafel steht an einem Stueck da, damit ein fehlender Wert auffaellt:
    /// die Feldbreite `[Fokus; 5]` in `Fokus::ALLE` haelt den Bau an, und
    /// diese Probe deckt die andere Haelfte ab, naemlich dass jeder Wert eine
    /// Antwort traegt, die zu ihm gehoert.
    #[test]
    fn jeder_fokuswert_bekommt_seinen_pfad() {
        let ordner = ordner();
        let editor = PathBuf::from("/Users/k1/Projekte/krk/README.md");
        let vorschau = PathBuf::from("/Users/k1/Bilder/schirm.png");

        let fuer = |fokus| titel(fokus, &ordner, Some(&editor), Some(&vorschau));

        assert_eq!(
            fuer(Fokus::Dateifenster).as_deref(),
            Some("/Users/k1/Projekte")
        );
        assert_eq!(fuer(Fokus::Leiste).as_deref(), Some("/Users/k1/Projekte"));
        assert_eq!(
            fuer(Fokus::Editor).as_deref(),
            Some("/Users/k1/Projekte/krk/README.md")
        );
        assert_eq!(
            fuer(Fokus::Vorschau).as_deref(),
            Some("/Users/k1/Bilder/schirm.png")
        );
        assert_eq!(fuer(Fokus::Anderswo), None);
    }

    /// Der Editor gewinnt gegen das aktive Dateifenster (C11, drittes
    /// Kriterium).
    ///
    /// Die Frage, die der Nutzer am 260809 ausdruecklich gestellt hat: was im
    /// Titel steht, wenn der Editor eine andere Datei haelt als das aktive
    /// Dateifenster anzeigt. Steht der Fokus im Editor, ist es dessen Datei.
    #[test]
    fn der_editor_gewinnt_gegen_den_ordner_des_aktiven_dateifensters() {
        let editor = PathBuf::from("/ganz/woanders/notiz.txt");
        assert_eq!(
            titel(Fokus::Editor, &ordner(), Some(&editor), None).as_deref(),
            Some("/ganz/woanders/notiz.txt")
        );
    }

    /// Ein Bereich ohne Pfad faellt auf den Ordner des aktiven Dateifensters.
    ///
    /// Die beiden Faelle, die das sechste Abnahmekriterium von C11 nennt: ein
    /// Editor ohne Datei, und eine Vorschau, die den Inhalt der Zwischenablage
    /// oder nichts zeigt. Damit kommt zugleich die zuletzt gehaltene Datei
    /// eines geschlossenen Editors nicht in den Titel, wie es das siebte
    /// Kriterium verlangt: sie wird gar nicht erst gereicht.
    #[test]
    fn ein_bereich_ohne_pfad_faellt_auf_den_ordner() {
        assert_eq!(
            titel(Fokus::Editor, &ordner(), None, None).as_deref(),
            Some("/Users/k1/Projekte")
        );
        assert_eq!(
            titel(Fokus::Vorschau, &ordner(), None, None).as_deref(),
            Some("/Users/k1/Projekte")
        );
    }

    /// Der Pfad steht ungekuerzt, auch unter dem Benutzerordner (C11, neuntes
    /// Kriterium).
    ///
    /// Keine Tilde, keine ausgelassenen Zwischenordner. Die Probe nennt einen
    /// tiefen Pfad, damit ein spaeteres Kuerzen der Mitte hier auffiele.
    #[test]
    fn der_pfad_steht_ungekuerzt() {
        let tief = PathBuf::from("/Users/k1/Library/Caches/krk-messplatz/lauf/ordner");
        let titel = titel(Fokus::Dateifenster, &tief, None, None)
            .expect("das Dateifenster liefert immer einen Titel");
        assert_eq!(titel, "/Users/k1/Library/Caches/krk-messplatz/lauf/ordner");
        assert!(!titel.contains('~'), "der Benutzerordner ist gekuerzt");
    }

    /// Der Titel prueft nicht nach (C11, elftes Kriterium).
    ///
    /// Ein Ordner, den es nicht gibt, steht trotzdem darin. Die Zusage faellt
    /// daraus an, dass die Funktion rein ist; die Probe haelt es fest, damit
    /// eine spaetere Pruefung auf Vorhandensein hier auffaellt.
    #[test]
    fn ein_verschwundener_ordner_steht_weiter_im_titel() {
        let fort = PathBuf::from("/Volumes/abgezogen/ordner");
        assert!(
            !fort.exists(),
            "die Probe braucht einen Pfad, den es nicht gibt"
        );
        assert_eq!(
            titel(Fokus::Dateifenster, &fort, None, None).as_deref(),
            Some("/Volumes/abgezogen/ordner")
        );
    }
}
