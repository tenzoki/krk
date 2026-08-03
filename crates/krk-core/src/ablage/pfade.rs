//! Wo die drei Ablagedateien liegen, und wie der Ordner beim ersten Start
//! entsteht.
//!
//! Der Ort ist `~/Library/Application Support/KRK/`, so wie `### Frage 4` des
//! Plans ihn festlegt. Aufgeloest wird er ueber das Benutzerverzeichnis, das
//! [`benutzerverzeichnis`] als einzige Stelle im Kern ermittelt.
//!
//! [`Ablageort`] traegt die Wurzel und nichts weiter. Dass er sich auch auf
//! einen beliebigen Ordner setzen laesst, ist keine Testhintertuer, sondern die
//! Bedingung dafuer, dass die Ablage ueberhaupt ohne Zugriff auf das echte
//! Benutzerverzeichnis pruefbar ist.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Die drei Dateien, die KRK unter `Application Support` ablegt.
///
/// Eine Aufzaehlung statt dreier loser Namen: wer alle drei anfassen muss,
/// laeuft ueber [`Datei::ALLE`] und kann keine vergessen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Datei {
    /// `keymap.toml`: die vollstaendige Belegung des Nutzers.
    ///
    /// Den Inhalt beschreibt Schritt 11, nicht dieser Schritt. Die Ablage
    /// kennt von dieser Datei nur den Namen und den Weg dorthin.
    Belegung,
    /// `bookmarks.toml`: die Lesezeichen, siehe [`super::lesezeichen`].
    Lesezeichen,
    /// `session.toml`: der Sitzungszustand, siehe [`super::sitzung`].
    Sitzung,
}

impl Datei {
    /// Alle drei, in fester Reihenfolge.
    pub const ALLE: [Datei; 3] = [Datei::Belegung, Datei::Lesezeichen, Datei::Sitzung];

    /// Der Dateiname unterhalb des Ablageordners.
    pub const fn dateiname(self) -> &'static str {
        match self {
            Datei::Belegung => "keymap.toml",
            Datei::Lesezeichen => "bookmarks.toml",
            Datei::Sitzung => "session.toml",
        }
    }
}

/// Die drei Namensteile zwischen Benutzerverzeichnis und Ablageordner.
const UNTERPFAD: [&str; 3] = ["Library", "Application Support", "KRK"];

/// Das Benutzerverzeichnis, falls das System eines nennt.
///
/// Die eine Stelle im Kern, die danach fragt. Zwei Aufrufer haengen daran und
/// gehen mit einem fehlenden Benutzerverzeichnis verschieden um:
/// [`Ablageort::im_benutzerverzeichnis`] scheitert, weil es ohne Wurzel nichts
/// abzulegen gibt, und der Auslieferungszustand der Sitzung weicht auf `/` aus,
/// weil ein Dateifenster einen Ordner zeigen muss.
pub fn benutzerverzeichnis() -> Option<PathBuf> {
    std::env::home_dir()
}

/// Der Ordner, in dem die drei Dateien liegen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ablageort {
    wurzel: PathBuf,
}

impl Ablageort {
    /// Der Ort unter dem Benutzerverzeichnis:
    /// `~/Library/Application Support/KRK/`.
    ///
    /// Legt nichts an; das tut [`Ablageort::anlegen`].
    pub fn im_benutzerverzeichnis() -> io::Result<Self> {
        let Some(zuhause) = benutzerverzeichnis() else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "das System nennt kein Benutzerverzeichnis",
            ));
        };
        let mut wurzel = zuhause;
        for teil in UNTERPFAD {
            wurzel.push(teil);
        }
        Ok(Self { wurzel })
    }

    /// Der Ort an einer frei gewaehlten Wurzel.
    pub fn an(wurzel: impl Into<PathBuf>) -> Self {
        Self {
            wurzel: wurzel.into(),
        }
    }

    /// Der Ablageordner selbst.
    pub fn wurzel(&self) -> &Path {
        &self.wurzel
    }

    /// Der Pfad einer der drei Dateien.
    pub fn datei(&self, welche: Datei) -> PathBuf {
        self.wurzel.join(welche.dateiname())
    }

    /// Legt den Ablageordner an, falls er noch nicht steht.
    ///
    /// Der Aufruf ist wiederholbar: ein vorhandener Ordner ist kein Fehler.
    /// Das ist die Anlage beim ersten Start, und sie kostet danach einen
    /// Systemaufruf je Programmstart.
    pub fn anlegen(&self) -> io::Result<()> {
        fs::create_dir_all(&self.wurzel)
    }
}
