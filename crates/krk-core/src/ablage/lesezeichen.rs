//! Die Lesezeichen aus C5: frei benannte Verweise auf Ordner.
//!
//! Die Reihenfolge der Liste ist die Reihenfolge in der Leiste; ein eigenes
//! Ordnungsfeld gibt es nicht, weil zwei Ordnungen zwei Wahrheiten waeren.
//!
//! Anlegen, Umbenennen, Loeschen und Verschieben sind Befehle der Leiste und
//! gehoeren zu Schritt 18. Hier steht nur, was auf der Platte liegt.
//!
//! Die Geraete und Standardorte aus dem unteren Teil der Leiste stehen nicht
//! in dieser Datei: sie kommen vom System und werden nicht abgelegt.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Ein Lesezeichen: ein Name und der Ordner, auf den er zeigt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Lesezeichen {
    /// Der Name, den der Nutzer vergeben hat.
    pub name: String,
    /// Der Ordner, den die Auswahl im aktiven Dateifenster oeffnet.
    pub ordner: PathBuf,
}

impl Lesezeichen {
    /// Ein Lesezeichen aus Name und Ordner.
    pub fn neu(name: impl Into<String>, ordner: impl Into<PathBuf>) -> Self {
        Self {
            name: name.into(),
            ordner: ordner.into(),
        }
    }
}

/// Alle Lesezeichen in ihrer Reihenfolge, wie sie in `bookmarks.toml` stehen.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Lesezeichenliste {
    /// Die Lesezeichen von oben nach unten.
    pub eintraege: Vec<Lesezeichen>,
}

impl Lesezeichenliste {
    /// Eine Liste aus vorhandenen Lesezeichen.
    pub fn aus(eintraege: Vec<Lesezeichen>) -> Self {
        Self { eintraege }
    }
}
