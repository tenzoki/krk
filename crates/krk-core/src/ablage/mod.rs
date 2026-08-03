//! Die Ablage: drei TOML-Dateien unter `~/Library/Application Support/KRK/`.
//!
//! Vier Module, in der Reihenfolge, in der ein Wert sie durchlaeuft:
//!
//! ```text
//! pfade ──> mod (Ablage: laden, sichern, melden) ──> atomar
//!                      ^                ^
//!                      │                │
//!               lesezeichen          sitzung
//! ```
//!
//! [`pfade`] loest den Ordner auf und legt ihn beim ersten Start an.
//! [`atomar`] schreibt jede Datei ueber eine Nachbardatei und `rename`.
//! [`sitzung`] und [`lesezeichen`] halten zwei der drei Inhalte; den dritten,
//! die Belegung aus `keymap.toml`, baut Schritt 11 und legt ihn ueber
//! [`Ablage::laden`] und [`Ablage::sichern`] hier ab. Die Ablage ist deshalb
//! ueber den Inhalt allgemein gehalten: sie kennt Pfad, Format und
//! Fehlerbehandlung, nicht die Felder.
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
//! # Der eine Ausgabeweg
//!
//! [`melden`] ist die einzige Stelle im Kern, die eine [`Ersetzung`] an den
//! Nutzer gibt. Heute schreibt sie auf die Standardfehlerausgabe, so wie der
//! Plan es in Schritt 10 vorschreibt. Im Buendel erreicht dieser Weg niemanden,
//! weil eine ueber `open` gestartete Anwendung keinen Standardfehler hat; die
//! Frage, welchen Weg KRK stattdessen nimmt, liegt als offene Entscheidung
//! `decisions/260803-2025_o_wie-zeigt-krk-dem-nutzer-fehler.md`. Weil die
//! Ablage die Meldung als Wert zurueckgibt und nur diese eine Funktion sie
//! ausgibt, kostet die Antwort eine Zeile und nicht fuenf.

pub mod atomar;
pub mod lesezeichen;
pub mod pfade;
pub mod sitzung;

use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub use lesezeichen::{Lesezeichen, Lesezeichenliste};
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
}

impl Grund {
    /// Der Satzteil, der den Grund benennt.
    fn beschreibung(&self) -> &'static str {
        match self {
            Grund::NichtLesbar(_) => "ist nicht lesbar",
            Grund::Beschaedigt(_) => "ist beschaedigt",
        }
    }

    /// Die Einzelheit, die das System oder der Leser gemeldet hat.
    pub fn einzelheit(&self) -> &str {
        match self {
            Grund::NichtLesbar(text) | Grund::Beschaedigt(text) => text,
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

/// Der eine Ausgabeweg, auf dem die Ablage etwas an den Nutzer meldet.
///
/// Siehe den Abschnitt "Der eine Ausgabeweg" im Modulkopf: aendert sich der
/// Weg, aendert sich diese Funktion und sonst keine Zeile.
pub fn melden(ersetzung: &Ersetzung) {
    eprintln!("krk: {ersetzung}");
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

    /// Der Wert, und eine etwaige Ersetzung ueber [`melden`] hinaus.
    pub fn gemeldet(self) -> T {
        if let Some(ersetzung) = &self.ersetzung {
            melden(ersetzung);
        }
        self.wert
    }
}

/// Der Zugang zu den drei Dateien unter `Application Support`.
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

    /// Der Pfad einer der drei Dateien.
    pub fn pfad(&self, welche: Datei) -> PathBuf {
        self.ort.datei(welche)
    }

    /// Liest eine der drei Dateien.
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

    /// Schreibt eine der drei Dateien, atomar ueber [`atomar::schreiben`].
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
