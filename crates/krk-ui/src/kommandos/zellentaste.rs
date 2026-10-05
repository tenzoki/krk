//! Wem ein Anschlag gehoert, solange eine Zelle der Eintragstabelle bearbeitet
//! wird: dem Text der Zelle oder dem Befehl, den die Belegung dafuer nennt.
//!
//! **Keine Zeile AppKit.** Wie im ganzen Verzeichnis [`crate::kommandos`] steht
//! hier keine `use objc2`-Zeile. Die drei Eingaben liest der
//! Anwendungsdelegierte — eine am Ersthelfer, eine am Anschlag, eine am
//! nachgeschlagenen Kommando —, die Regel selbst steht hier und ist ohne
//! Fenster pruefbar.
//!
//! ```text
//!  zelle_laeuft ────────┐
//!  traegt_befehlstaste ─┼──> gehoert_der_zelle() ──> ja: die Taste geht an AppKit
//!  seite ───────────────┘                            nein: der Befehl laeuft
//! ```
//!
//! # Warum es diese Regel gibt
//!
//! Seit dem 261005 traegt `eintrag_loeschen` in
//! `resources/default-keymap.toml` neben `shift+cmd+delete` das nackte
//! `delete` (Nutzerauftrag vom 261005: die Rueckschritt-Taste loescht den
//! gewaehlten Eintrag). Die laufende Zelle der Eintragstabelle ist aber eine
//! von KRKs eigenen Textflaechen (`Anwendungsdelegierter::ist_eigene_textflaeche`):
//! der Ersthelfer gehoert dort nicht AppKit, und
//! [`zulaessigkeit::zulaessig`](super::zulaessigkeit::zulaessig) sagt zu jedem
//! Befehl der Tabelle ja, damit `cmd+return` die Zelle uebernehmen kann. Ohne
//! diese Regel loeschte der Rueckschritt, mit dem der Nutzer einen Vertipper
//! in der Zelle berichtigt, den ganzen Eintrag.
//!
//! Bis zum 261005 fiel das nicht an, und zwar ohne dass eine Regel es gehalten
//! haette: kein Befehl der Eintragstabellen trug ab Werk eine Kombination ohne
//! `cmd`. Die nackten Tasten gehoerten Befehlen des Dateifensters, und die
//! weist der Fokus im Editor ab.
//!
//! # Der Schnitt
//!
//! - **Ohne `cmd`.** Jede Taste, mit der ein Feldeditor Text schreibt oder
//!   aendert, kommt ohne die Befehlstaste aus: Zeichen, `delete`, `return`,
//!   die Pfeile, auch mit `shift`, `opt` oder `ctrl` davor. Die Kombinationen
//!   mit `cmd` sind bewusst an KRK vergeben und bleiben es: `cmd+return`
//!   uebernimmt die Zelle, `shift+cmd+delete` uebernimmt sie und loescht den
//!   Eintrag (Entscheid
//!   `260926-0112_*_was-tut-return-in-einer-notizzelle-und-welche-tasten-tragen-die-editoren.md`).
//! - **Allein Befehle der Editorseite.** [`Seite::Editor`] sind die Befehle,
//!   die an der Datei im Editor arbeiten; solange eine Zelle laeuft, ist der
//!   Text der Zelle der naehere Gegenstand. Ein Befehl mit [`Seite::Beide`]
//!   behaelt seine Taste: `esc` traegt `Wirkungsbereich::Ueberall` und muss die
//!   Zelle erreichen, um sie abzubrechen, und die Funktionstasten ebenso.
//!   [`Seite::Ausserhalb`] erreicht diese Regel der Sache nach nie, weil der
//!   Fokus im Editor solche Befehle schon abweist; die Zeile steht, damit die
//!   Fallunterscheidung vollstaendig ist.
//! - **Allein die laufende Zelle.** In der Textflaeche des Editors und in der
//!   Quicknote braucht es die Regel nicht: dort sagt schon die Form nein zu
//!   jedem Tabellenbefehl (`zulaessigkeit::form_passt`), und das nackte
//!   `delete` geht als Rueckschritt an AppKit wie vor dem 261005.
//!
//! Die Regel fragt damit nicht nach `delete`, sondern nach der Art des
//! Anschlags. Wer in der F1-Ansicht eine weitere nackte Taste an einen
//! Tabellenbefehl bindet, bekommt in der laufenden Zelle dieselbe Antwort, ohne
//! dass hier eine Zeile dazukaeme.
//!
//! # Wo die Regel steht und wen sie nicht trifft
//!
//! Sie steht **hinter** [`zulaessigkeit::zulaessig`](super::zulaessigkeit::zulaessig)
//! und nicht darin, aus dem Grund, aus dem [`super::rueckschritt`] dort nicht
//! steht: die Zulaessigkeit sieht das Kommando und nicht den Anschlag, und
//! `delete` und `shift+cmd+delete` sind zu diesem Zeitpunkt dasselbe
//! `Kommando::EintragLoeschen`. Eine Antwort dort traefe beide Wege und graute
//! den Menueeintrag „Eintrag löschen“ aus, solange eine Zelle laeuft. Der
//! Menueeintrag reicht keinen Anschlag herein und erreicht diese Regel nicht.
//!
//! **Sie steht vor der Ausfuehrung und entscheidet, ob der Tastendruck
//! geschluckt wird.** Das trennt sie von [`super::rueckschritt`], dessen drei
//! Ausgaenge alle geschluckt werden: sagt sie ja, antwortet
//! `Anwendungsdelegierter::kommando_ausfuehren_bei` mit „nicht zulaessig“, der
//! Ereignisabgriff reicht das Ereignis unveraendert weiter, und der Feldeditor
//! nimmt das Zeichen zurueck. Das Hauptmenue faengt es unterwegs nicht ab:
//! `delete` steht dort als Kuerzel von „In den Papierkorb räumen“, und der
//! Eintrag ist mit dem Fokus im Editor ausgegraut.
//!
//! # Der eine Aufrufer
//!
//! `Anwendungsdelegierter::kommando_ausfuehren_bei` (`crate::appkit::anwendung`)
//! ist der einzige, und die Probe `die_regel_hat_genau_einen_aufrufer` haelt
//! die Zahl fest.

use krk_core::tasten::Seite;

/// Ob dieser Anschlag dem Text der laufenden Zelle gehoert und der
/// nachgeschlagene Befehl deshalb unterbleibt.
///
/// Der Rumpf ist diese Tafel:
///
/// | `zelle_laeuft` | `traegt_befehlstaste` | `seite` | Antwort |
/// |---|---|---|---|
/// | nein | gleichgueltig | gleichgueltig | nein |
/// | ja | ja | gleichgueltig | nein |
/// | ja | nein | [`Seite::Editor`] | **ja** |
/// | ja | nein | [`Seite::Beide`], [`Seite::Ausserhalb`] | nein |
///
/// Die vier Zeilen decken alle zwoelf Faelle ab, und die Probe
/// `die_tafel_aus_zwoelf_faellen_geht_auf` schreibt sie aus. Ueber [`Seite`]
/// verzweigt der Rumpf vollstaendig und ohne Auffangzweig: eine weitere Seite
/// haelt den Bau hier an.
///
/// `#[must_use]`, weil ein Rufer, der die Antwort fallen liesse, den Befehl
/// ausfuehrte, den die Regel eben dem Text ueberlassen hat.
#[must_use = "fallengelassen loescht der Rueckschritt in einer Zelle den Eintrag"]
pub fn gehoert_der_zelle(zelle_laeuft: bool, traegt_befehlstaste: bool, seite: Seite) -> bool {
    if !zelle_laeuft || traegt_befehlstaste {
        return false;
    }
    match seite {
        Seite::Editor => true,
        Seite::Beide | Seite::Ausserhalb => false,
    }
}

#[cfg(test)]
mod tests {
    use krk_core::tasten::{Belegung, Kommando, ModMaske};

    use crate::quellbaum::{aufrufstellen, quelldateien};

    use super::*;

    /// Die Regel hat genau einen Aufrufer ausserhalb dieser Datei; ein zweiter
    /// waere eine zweite Stelle, die entscheidet, wem eine Taste gehoert.
    ///
    /// Die Nadel steht zusammengesetzt da, weil die Probe in dem Baum liegt,
    /// den sie liest.
    #[test]
    fn die_regel_hat_genau_einen_aufrufer() {
        let zuhause = "krk-ui/src/kommandos/zellentaste.rs";
        let name = concat!("gehoert_der", "_zelle");
        let aufrufe: usize = quelldateien()
            .iter()
            .filter(|(datei, _)| datei != zuhause)
            .map(|(_, inhalt)| aufrufstellen(inhalt, name))
            .sum();
        assert_eq!(
            aufrufe, 1,
            "die Regel der Zellentaste hat nicht genau einen Aufrufer"
        );
    }

    /// Die ganze Regel auf einen Blick: zwei mal zwei mal drei Faelle,
    /// ausgeschrieben und nicht gerechnet.
    #[test]
    fn die_tafel_aus_zwoelf_faellen_geht_auf() {
        // zelle_laeuft, traegt_befehlstaste, seite, Antwort.
        const TAFEL: [(bool, bool, Seite, bool); 12] = [
            (false, false, Seite::Editor, false),
            (false, false, Seite::Beide, false),
            (false, false, Seite::Ausserhalb, false),
            (false, true, Seite::Editor, false),
            (false, true, Seite::Beide, false),
            (false, true, Seite::Ausserhalb, false),
            (true, false, Seite::Editor, true),
            (true, false, Seite::Beide, false),
            (true, false, Seite::Ausserhalb, false),
            (true, true, Seite::Editor, false),
            (true, true, Seite::Beide, false),
            (true, true, Seite::Ausserhalb, false),
        ];
        for (zelle_laeuft, traegt_befehlstaste, seite, antwort) in TAFEL {
            assert_eq!(
                gehoert_der_zelle(zelle_laeuft, traegt_befehlstaste, seite),
                antwort,
                "zelle_laeuft={zelle_laeuft} traegt_befehlstaste={traegt_befehlstaste} \
                 seite={seite:?}"
            );
        }
    }

    /// Der Anlass der Regel am ausgelieferten Stand: das nackte `delete` in
    /// einer laufenden Zelle gehoert dem Text, in der Tabelle ohne laufende
    /// Zelle loescht es den Eintrag, und `shift+cmd+delete` loescht in beiden
    /// Lagen.
    #[test]
    fn der_nackte_rueckschritt_bleibt_in_der_laufenden_zelle_ein_rueckschritt() {
        let belegung = Belegung::auslieferung();
        let loeschen = belegung
            .funktionen()
            .iter()
            .find(|funktion| funktion.kommando() == Some(Kommando::EintragLoeschen))
            .expect("die Auslieferung fuehrt eintrag_loeschen");
        let seite = Kommando::EintragLoeschen.wirkungsbereich().seite();
        let befehlstaste = |text: &str| {
            let kombination = loeschen
                .tasten()
                .iter()
                .find(|kombination| kombination.to_string() == text)
                .unwrap_or_else(|| panic!("eintrag_loeschen traegt {text} nicht"));
            kombination.maske().enthaelt(ModMaske::BEFEHL)
        };

        assert!(gehoert_der_zelle(true, befehlstaste("delete"), seite));
        assert!(!gehoert_der_zelle(false, befehlstaste("delete"), seite));
        assert!(!gehoert_der_zelle(
            true,
            befehlstaste("shift+cmd+delete"),
            seite
        ));
        assert!(!gehoert_der_zelle(
            false,
            befehlstaste("shift+cmd+delete"),
            seite
        ));
    }

    /// Welche ausgelieferten Kombinationen die Regel ueberhaupt treffen kann:
    /// die ohne `cmd` an einem Befehl der Editorseite. Ab Werk sind es zwei,
    /// und eine weitere soll hier auffallen, damit ihr Verhalten in der
    /// laufenden Zelle bewusst mitentschieden wird.
    ///
    /// **Nur das nackte `delete` begegnet einer laufenden Zelle.**
    /// `shift+f10` kopiert die Quicknote und ist allein in ihrer Form
    /// zulaessig; solange sie offen ist, ist die Tabelle ausgeblendet, und
    /// keine Zelle laeuft. Die Regel sieht den Anschlag deshalb nie.
    #[test]
    fn ab_werk_trifft_die_regel_zwei_kombinationen_und_eine_begegnet_der_zelle() {
        let belegung = Belegung::auslieferung();
        let mut getroffen: Vec<(String, String)> = Vec::new();
        for funktion in belegung.funktionen() {
            let Some(bereich) = funktion.wirkungsbereich() else {
                continue;
            };
            for kombination in funktion.tasten() {
                let befehlstaste = kombination.maske().enthaelt(ModMaske::BEFEHL);
                if gehoert_der_zelle(true, befehlstaste, bereich.seite()) {
                    getroffen.push((funktion.kennung().to_owned(), kombination.to_string()));
                }
            }
        }
        assert_eq!(
            getroffen,
            [
                ("quicknote_kopieren".to_owned(), "shift+f10".to_owned()),
                ("eintrag_loeschen".to_owned(), "delete".to_owned())
            ]
        );
    }
}
