//! `settings.toml`: die eine Ablagedatei, die der Nutzer von Hand pflegt (C11).
//!
//! ```text
//! resources/default-settings.toml ──include_str!──> AUSLIEFERUNGSTEXT
//!                                                     │        │
//!                            erster Start ──atomar────┘        │
//!                                                              v
//!      ~/Library/.../KRK/settings.toml ──Ablage──> Einstellungen
//! ```
//!
//! # Warum eine vierte Datei
//!
//! Die drei vorhandenen scheiden aus, jede aus einem eigenen Grund, und die
//! Herleitung steht in `### Frage 4` des Plans. Der kurze Stand: `keymap.toml`
//! setzt der Befehl aus C3 vollstaendig zurueck und naehme die Terminal-Wahl
//! mit; `session.toml` ueberschreibt KRK alle zwei Sekunden und loeschte dabei
//! jeden Kommentar; `bookmarks.toml` haelt Ordnerverweise und wird bei jeder
//! Aenderung geschrieben. Aufgenommen wird hier ein Wert, der keine
//! Tastenbelegung ist, den KRK im Betrieb nicht selbst schreibt und der in
//! dieser Runde keine Oberflaeche hat.
//!
//! # Die Datei entsteht einmal und wird danach nicht mehr geschrieben
//!
//! [`laden`] legt sie beim ersten Start an, und zwar **woertlich aus
//! [`AUSLIEFERUNGSTEXT`]** und nicht ueber [`Ablage::sichern`]. Der Unterschied
//! ist der ganze Zweck der Datei: `serde` kennt keine Kommentare, und eine
//! Serialisierung von [`Einstellungen`] hinterliesse eine Datei mit einer
//! einzigen Zeile. Die fuenfzig Kommentarzeilen der Auslieferungsfassung sind
//! die Antwort auf den Einwand gegen die Buendelkennung — sie nennen das
//! `mdls`-Kommando, mit dem der Nutzer die Kennung seiner eigenen Anwendung
//! findet. Ohne sie stuende dort ein Wert, den niemand aendern kann.
//!
//! Der Schreibweg selbst ist der aus Schritt 10 und kein zweiter:
//! [`atomar::schreiben`], derselbe Ablageort, dieselbe Behandlung einer
//! beschaedigten Datei. Allein die Nutzlast ist eine andere.
//!
//! # Ein fehlendes Feld kommt aus der Auslieferungsfassung
//!
//! Das ist die eine Abweichung von `keymap.toml`, wo die Nutzerdatei die
//! Auslieferungsbelegung **ersetzt**. Dort braucht es das, weil der Nutzer eine
//! Belegung sonst nicht loswerden koennte; eine Terminal-Anwendung laesst sich
//! nicht abwaehlen, ohne die Funktion abzuschalten. Die Abweichung kostet keine
//! Verzweigung: [`Einstellungsdatei`] haelt jedes Feld als `Option`, und
//! [`Einstellungen::aus_datei`] fuellt das leere aus der Auslieferungsfassung.

use std::io;
use std::sync::LazyLock;

use serde::Deserialize;

use super::{Beiseite, Datei, Ersetzung, Geladen, Grund, Zugang, atomar};

/// Die Auslieferungsfassung der Einstellungen, in das Programm einkompiliert.
///
/// Damit gibt es keinen Start ohne Einstellungen, und die Anlage beim ersten
/// Start braucht keinen Zugriff auf das Buendel.
pub const AUSLIEFERUNGSTEXT: &str = include_str!("../../../../resources/default-settings.toml");

/// Die gelesene Auslieferungsfassung.
///
/// Sie fuellt jedes Feld, das die Nutzerdatei nicht nennt. Gebaut wird sie
/// allein aus dem eingebetteten Text und nie aus sich selbst; eine
/// Ruecksprungmarke auf diesen Wert waehrend seiner eigenen Entstehung gibt es
/// deshalb nicht.
static AUSLIEFERUNG: LazyLock<Einstellungen> = LazyLock::new(|| {
    let datei: Einstellungsdatei = toml::from_str(AUSLIEFERUNGSTEXT)
        .expect("die eingebettete Auslieferungsfassung ist kein gueltiges TOML");
    Einstellungen {
        terminal: datei
            .terminal
            .expect("die eingebettete Auslieferungsfassung nennt keinen Eintrag terminal"),
    }
});

/// Die von Hand gepflegten Einstellungen, wie KRK sie im Betrieb liest.
///
/// **Bewusst ohne `Serialize`.** Ein Serialisierungsweg waere der zweite Weg zu
/// dieser Datei, und er schriebe sie ohne ihre Kommentare; siehe den Modulkopf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Einstellungen {
    /// Die Buendelkennung der Anwendung, die "Ordner im Terminal oeffnen"
    /// ruft (C11), etwa `com.apple.Terminal`.
    ///
    /// Eine Kennung und kein Pfad: `NSWorkspace` kennt genau einen nicht
    /// abgekuendigten Weg von einem Namen zu einer installierten Anwendung, und
    /// der geht ueber die Kennung. Sie ueberlebt zudem das Verschieben und
    /// Umbenennen der Anwendung. Die Herleitung steht in `### Frage 4` des
    /// Plans, die Erklaerung fuer den Nutzer in `resources/default-settings.toml`.
    pub terminal: String,
}

impl Einstellungen {
    /// Die eingebettete Auslieferungsfassung.
    pub fn auslieferung() -> Self {
        AUSLIEFERUNG.clone()
    }

    /// Die gelesene Datei, ergaenzt um jedes Feld, das sie nicht nennt.
    fn aus_datei(datei: &Einstellungsdatei) -> Self {
        Self {
            terminal: datei
                .terminal
                .clone()
                .unwrap_or_else(|| AUSLIEFERUNG.terminal.clone()),
        }
    }
}

impl Default for Einstellungen {
    fn default() -> Self {
        Self::auslieferung()
    }
}

/// Die Gestalt von `default-settings.toml` und `settings.toml`, unveraendert.
///
/// Der Zwischenschritt zwischen TOML und [`Einstellungen`]: hier fehlt ein
/// Feld noch, statt schon aus der Auslieferungsfassung zu kommen. Denselben
/// Zuschnitt zieht `Belegungsdatei` neben `Belegung`.
///
/// `deny_unknown_fields` wie dort: ein Feld, das KRK nicht kennt, ist in einer
/// von Hand gepflegten Datei fast immer ein Tippfehler, und der Nutzer soll ihn
/// als Meldung sehen statt seine Einstellung stillschweigend zu verlieren.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Einstellungsdatei {
    #[serde(default)]
    terminal: Option<String>,
}

/// Laedt `settings.toml` und legt sie beim ersten Start an.
///
/// Scheitert nie. Die vier Faelle:
///
/// | Auf der Platte | Ergebnis |
/// |---|---|
/// | keine Datei | Vorbelegung, **keine** Meldung, die Datei entsteht |
/// | gueltige Datei | ihr Wert, keine Meldung, nichts wird geschrieben |
/// | Datei ohne `terminal` | Vorbelegung, keine Meldung, nichts wird geschrieben |
/// | kaputte Datei | Vorbelegung, Meldung, die Datei bleibt unveraendert liegen |
///
/// Die kaputte Datei bleibt aus demselben Grund liegen wie eine kaputte
/// `keymap.toml`: sie ist von Hand geschrieben, und ein Tippfehler darf die
/// Arbeit des Nutzers nicht loeschen. Ueberschrieben wird sie nie, denn in
/// dieser Runde schreibt keine Ansicht diese Datei.
///
/// Hoechstens eine Meldung kann anfallen: angelegt wird nur, was fehlt, und
/// eine fehlende Datei traegt keine Ersetzung.
pub fn laden(zugang: &Zugang<'_>) -> Geladen<Einstellungen> {
    let roh: Geladen<Einstellungsdatei> = zugang.laden(Datei::Einstellungen);
    let wert = Einstellungen::aus_datei(&roh.wert);
    if roh.ersetzung.is_some() {
        return Geladen {
            wert,
            ersetzung: roh.ersetzung,
        };
    }
    match anlegen_falls_fehlt(zugang) {
        Ok(()) => Geladen {
            wert,
            ersetzung: None,
        },
        Err(fehler) => Geladen {
            wert,
            ersetzung: Some(Ersetzung {
                datei: zugang.pfad(Datei::Einstellungen),
                welche: Datei::Einstellungen,
                grund: Grund::NichtAnlegbar(fehler.to_string()),
                // Eine Datei, die es nicht gibt, hat keinen Inhalt zu sichern.
                beiseite: Beiseite::Nicht,
            }),
        },
    }
}

/// Schreibt die Auslieferungsfassung woertlich, falls die Datei fehlt.
///
/// Wiederholbar wie [`super::Ablageort::anlegen`] eine Ebene hoeher: eine
/// vorhandene Datei ist kein Fehler und wird nicht angefasst.
fn anlegen_falls_fehlt(zugang: &Zugang<'_>) -> io::Result<()> {
    let pfad = zugang.pfad(Datei::Einstellungen);
    if pfad.try_exists()? {
        return Ok(());
    }
    atomar::schreiben(&pfad, &mut AUSLIEFERUNGSTEXT.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Die eingebettete Fassung traegt den Wert, den C11 ab Werk zusagt.
    #[test]
    fn die_auslieferungsfassung_nennt_terminal_app() {
        assert_eq!(Einstellungen::auslieferung().terminal, "com.apple.Terminal");
    }

    /// Sie traegt ihre Kommentare, und die sind der Zweck der Datei.
    #[test]
    fn die_auslieferungsfassung_traegt_ihre_kommentare() {
        assert!(
            AUSLIEFERUNGSTEXT.contains("mdls -name kMDItemCFBundleIdentifier"),
            "ohne das Kommando findet der Nutzer die Kennung seiner Anwendung nicht"
        );
        let kommentarzeilen = AUSLIEFERUNGSTEXT
            .lines()
            .filter(|zeile| zeile.starts_with('#'))
            .count();
        assert!(
            kommentarzeilen > 20,
            "die Auslieferungsfassung traegt nur {kommentarzeilen} Kommentarzeilen"
        );
    }
}
