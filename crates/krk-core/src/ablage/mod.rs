//! Die Ablage: vier TOML-Dateien unter `~/Library/Application Support/KRK/`.
//!
//! Fuenf Module, in der Reihenfolge, in der ein Wert sie durchlaeuft:
//!
//! ```text
//! pfade ──> mod (Ablage: laden, sichern, melden) ──> atomar
//!                   ^        ^         ^
//!                   │        │         │
//!            lesezeichen  sitzung  einstellungen
//! ```
//!
//! [`pfade`] loest den Ordner auf und legt ihn beim ersten Start an.
//! [`atomar`] schreibt jede Datei ueber eine Nachbardatei und `rename`.
//! [`sitzung`], [`lesezeichen`] und [`einstellungen`] halten drei der vier
//! Inhalte; den vierten, die Belegung aus `keymap.toml`, baut Schritt 11 und
//! legt ihn ueber [`Ablage::laden`] und [`Ablage::sichern`] hier ab. Die Ablage
//! ist deshalb ueber den Inhalt allgemein gehalten: sie kennt Pfad, Format und
//! Fehlerbehandlung, nicht die Felder.
//!
//! # Eine der vier Dateien entsteht einmal und wird nie wieder geschrieben
//!
//! `settings.toml` aus Schritt 18c ist die eine von Hand gepflegte Datei, und
//! sie geht als einzige **nicht** ueber [`Ablage::sichern`]: die Anlage beim
//! ersten Start schreibt die eingebettete Auslieferungsfassung woertlich, samt
//! deren Kommentaren, die `serde` nicht kennt. Der Weg dorthin ist derselbe
//! [`atomar::schreiben`], allein die Nutzlast ist eine andere; siehe den Kopf
//! von [`einstellungen`].
//!
//! # Ein beschaedigter Bestand laesst KRK starten
//!
//! [`Ablage::laden`] liefert keinen Fehler, sondern immer einen Wert. Eine
//! fehlende Datei ist der erste Start und keine Meldung wert. Eine nicht
//! lesbare oder syntaktisch kaputte Datei fuehrt zum Auslieferungszustand und
//! zu einer [`Ersetzung`], die die Datei benennt. Die Datei auf der Platte
//! bleibt dabei stehen: `keymap.toml` ist laut `### Frage 4` von Hand
//! aenderbar, und ein Tippfehler darin darf die Arbeit des Nutzers nicht
//! loeschen. Ueberschrieben wird sie erst beim naechsten gewoehnlichen
//! Schreibvorgang.
//!
//! # Der Kern gibt nichts aus
//!
//! [`melden`] ist die einzige Stelle im Kern, die aus einer [`Ersetzung`] einen
//! Satz fuer den Nutzer macht, und sie **schreibt ihn nicht, sondern gibt ihn
//! zurueck**. Bis Schritt 11 schrieb sie auf die Standardfehlerausgabe; im
//! Buendel erreichte das niemanden, weil eine ueber den Finder gestartete
//! Anwendung keine hat. Der Nutzer hat am 260804-0830 Moeglichkeit 1 aus
//! `decisions/260803-2025_*_wie-zeigt-krk-dem-nutzer-fehler.md` gewaehlt: die
//! Meldung gehoert in die Statuszeile am Fuss des Dateifensters, und die baut
//! `krk-ui` in Schritt 12. Der Kern schreibt seither auf keinen Kanal mehr, und
//! das Abnahmekriterium von Schritt 12 prueft es mit einem `grep` ueber
//! `crates/krk-core/src` nach dem Namen des Ausgabemakros. Deshalb steht der
//! Name hier nirgends ausgeschrieben: er wuerde die eigene Pruefung brechen.
//!
//! **Die Aufrufrichtung bleibt dabei von oben nach unten.** Der Kern ruft die
//! Oberflaeche nicht an; er liefert einen Wert, und wer ihn geladen hat,
//! entscheidet, wo er ihn hinstellt. Eine zweite Abhaengigkeitsumkehr neben der
//! Papierkorb-Schnittstelle entsteht nicht.

pub mod atomar;
pub mod einstellungen;
pub mod lesezeichen;
pub mod pfade;
pub mod sitzung;

use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub use einstellungen::Einstellungen;
pub use lesezeichen::{Lesezeichen, Lesezeichenliste, Namenshinweis, Verschiebung, Ziel};
pub use pfade::{Ablageort, Datei};
pub use sitzung::{
    Breiten, Dateifenster, Fensterseite, Sichtbarkeit, Sitzung, Sitzungsschreiber, Tab,
};

/// Warum eine Datei durch den Auslieferungszustand ersetzt wurde.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grund {
    /// Die Datei liegt da, liess sich aber nicht lesen. Traegt die Meldung des
    /// Dateisystems.
    NichtLesbar(String),
    /// Die Datei liess sich lesen, ist aber kein gueltiges TOML oder passt
    /// nicht auf die erwartete Gestalt. Traegt die Meldung des Lesers.
    Beschaedigt(String),
    /// Die Datei fehlte und liess sich nicht anlegen. Traegt die Meldung des
    /// Dateisystems.
    ///
    /// Nur `settings.toml` kann ihn tragen. Sie ist die eine Datei, die KRK
    /// beim ersten Start von sich aus anlegt, weil in dieser Runde keine
    /// Ansicht sie schreibt und der Nutzer sonst nichts zu pflegen haette. Bei
    /// den drei uebrigen ist eine fehlende Datei der erste Start und keine
    /// Meldung wert.
    NichtAnlegbar(String),
}

impl Grund {
    /// Der Satzteil, der den Grund benennt.
    fn beschreibung(&self) -> &'static str {
        match self {
            Grund::NichtLesbar(_) => "ist nicht lesbar",
            Grund::Beschaedigt(_) => "ist beschaedigt",
            Grund::NichtAnlegbar(_) => "liess sich nicht anlegen",
        }
    }

    /// Die Einzelheit, die das System oder der Leser gemeldet hat.
    pub fn einzelheit(&self) -> &str {
        match self {
            Grund::NichtLesbar(text) | Grund::Beschaedigt(text) | Grund::NichtAnlegbar(text) => {
                text
            }
        }
    }
}

/// Eine Datei wurde durch den Auslieferungszustand ersetzt.
///
/// Ein Wert und keine Ausgabe: wer laedt, entscheidet, ob und wie er ihn
/// meldet. Der Weg dorthin ist [`melden`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ersetzung {
    /// Die Datei, um die es geht. Sie steht in jeder Meldung.
    pub datei: PathBuf,
    /// Warum sie ersetzt wurde.
    pub grund: Grund,
}

impl fmt::Display for Ersetzung {
    fn fmt(&self, ausgabe: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            ausgabe,
            "{} {} und wird durch den Auslieferungszustand ersetzt: {}",
            self.datei.display(),
            self.grund.beschreibung(),
            self.grund.einzelheit()
        )
    }
}

/// Der Satz, den der Nutzer zu einer [`Ersetzung`] lesen soll.
///
/// Die eine Stelle im Kern, die eine Ersetzung in Worte fasst. Sie gibt den
/// Text zurueck und schreibt ihn nirgendwohin; siehe den Abschnitt "Der Kern
/// gibt nichts aus" im Modulkopf.
///
/// Der frueher vorangestellte Programmname `krk: ` ist mit Schritt 12
/// entfallen. Er war die Anrede eines Terminals; in einer Statuszeile, die
/// ausschliesslich KRK gehoert, benennt er nichts, und der Satz nennt die
/// betroffene Datei ohnehin selbst.
#[must_use]
pub fn melden(ersetzung: &Ersetzung) -> String {
    ersetzung.to_string()
}

/// Das Ergebnis eines Ladevorgangs: immer ein Wert, dazu die Meldung, falls
/// eine noetig war.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Geladen<T> {
    /// Der gelesene Wert, oder der Auslieferungszustand.
    pub wert: T,
    /// Gesetzt, wenn der Auslieferungszustand eingesprungen ist.
    pub ersetzung: Option<Ersetzung>,
}

impl<T> Geladen<T> {
    /// Ob der Auslieferungszustand eingesprungen ist.
    pub fn ist_ersetzt(&self) -> bool {
        self.ersetzung.is_some()
    }

    /// Der Wert und der Satz, den der Aufrufer dem Nutzer zeigen muss.
    ///
    /// Die Vorgaengerin `gemeldet` hat den Satz selbst geschrieben. Sie ist mit
    /// Schritt 12 entfallen, weil der Kern keinen Ausgabekanal mehr hat: wer
    /// laedt, bekommt den Text und stellt ihn in seine Statuszeile.
    pub fn mit_meldung(self) -> (T, Option<String>) {
        let meldung = self.ersetzung.as_ref().map(melden);
        (self.wert, meldung)
    }
}

/// Der Zugang zu den vier Dateien unter `Application Support`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ablage {
    ort: Ablageort,
}

impl Ablage {
    /// Oeffnet die Ablage an einem Ort und legt den Ordner an, falls er fehlt.
    pub fn oeffnen(ort: Ablageort) -> io::Result<Self> {
        ort.anlegen()?;
        Ok(Self { ort })
    }

    /// Oeffnet die Ablage unter `~/Library/Application Support/KRK/`.
    ///
    /// Der Weg des laufenden Programms. Der erste Start legt den Ordner an.
    pub fn im_benutzerverzeichnis() -> io::Result<Self> {
        Self::oeffnen(Ablageort::im_benutzerverzeichnis()?)
    }

    /// Der Ort, an dem diese Ablage liegt.
    pub fn ort(&self) -> &Ablageort {
        &self.ort
    }

    /// Der Pfad einer der vier Dateien.
    pub fn pfad(&self, welche: Datei) -> PathBuf {
        self.ort.datei(welche)
    }

    /// Liest eine der vier Dateien.
    ///
    /// Scheitert nie: eine fehlende, nicht lesbare oder beschaedigte Datei
    /// fuehrt zum Auslieferungszustand. Nur die letzten beiden Faelle tragen
    /// eine [`Ersetzung`]; eine fehlende Datei ist der erste Start.
    pub fn laden<T>(&self, welche: Datei) -> Geladen<T>
    where
        T: DeserializeOwned + Default,
    {
        let pfad = self.pfad(welche);
        let text = match fs::read_to_string(&pfad) {
            Ok(text) => text,
            Err(fehler) if fehler.kind() == io::ErrorKind::NotFound => {
                return Geladen {
                    wert: T::default(),
                    ersetzung: None,
                };
            }
            Err(fehler) => {
                return Geladen {
                    wert: T::default(),
                    ersetzung: Some(Ersetzung {
                        datei: pfad,
                        grund: Grund::NichtLesbar(fehler.to_string()),
                    }),
                };
            }
        };
        match toml::from_str(&text) {
            Ok(wert) => Geladen {
                wert,
                ersetzung: None,
            },
            Err(fehler) => Geladen {
                wert: T::default(),
                ersetzung: Some(Ersetzung {
                    datei: pfad,
                    grund: Grund::Beschaedigt(einzeilig(&fehler.to_string())),
                }),
            },
        }
    }

    /// Schreibt eine der vier Dateien, atomar ueber [`atomar::schreiben`].
    ///
    /// **Nicht der Weg zu `settings.toml`.** Die Serialisierung kennt keine
    /// Kommentare, und die von Hand gepflegte Datei besteht zur Haelfte aus
    /// ihnen; ihre Anlage schreibt deshalb die eingebettete
    /// Auslieferungsfassung woertlich. Siehe den Kopf von [`einstellungen`].
    pub fn sichern<T>(&self, welche: Datei, wert: &T) -> io::Result<()>
    where
        T: Serialize,
    {
        let text = toml::to_string(wert).map_err(io::Error::other)?;
        atomar::schreiben(&self.pfad(welche), &text)
    }

    /// Der Schreiber fuer den gebuendelten Sitzungszustand.
    pub fn sitzungsschreiber(&self) -> Sitzungsschreiber {
        Sitzungsschreiber::neu(self.pfad(Datei::Sitzung))
    }
}

/// Presst eine mehrzeilige Fehlermeldung in eine Zeile.
///
/// Der TOML-Leser zeichnet die Fundstelle ueber mehrere Zeilen aus. In einer
/// Meldung, die spaeter in eine Statuszeile passen muss, ist das eine Zeile zu
/// viel; die Angaben zu Zeile und Spalte bleiben erhalten.
fn einzeilig(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}
