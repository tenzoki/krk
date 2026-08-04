//! Ordner und leere Datei anlegen (C4).
//!
//! Zwei kurze Funktionen ohne Arbeitsfaden. Sie sind sofort fertig, es gibt
//! nichts zu melden und nichts abzubrechen; ein Auftrag an die
//! Operationsmaschine waere hier mehr Aufwand als Sache.
//!
//! Beide legen **nichts** ueber einen vorhandenen Eintrag. Ein Anlegen, das
//! eine bestehende Datei leert, waere ein Datenverlust ohne Rueckfrage.
//! Die Namenspruefung ist dieselbe wie beim Umbenennen und steht dort.

use std::fs::{self, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

use super::umbenennen::name_pruefen;

/// Legt einen Ordner im genannten Ordner an.
///
/// Liefert den Pfad des neuen Ordners; die Oberflaeche stellt die Auswahl
/// darauf (C4).
pub fn ordner_anlegen(elternordner: &Path, name: &str) -> io::Result<PathBuf> {
    name_pruefen(name)?;
    let pfad = elternordner.join(name);
    fs::create_dir(&pfad)?;
    Ok(pfad)
}

/// Legt eine leere Datei im genannten Ordner an.
pub fn datei_anlegen(elternordner: &Path, name: &str) -> io::Result<PathBuf> {
    name_pruefen(name)?;
    let pfad = elternordner.join(name);
    OpenOptions::new()
        .write(true)
        // `create_new` ist der Unterschied zwischen "leg an" und "leere".
        .create_new(true)
        .open(&pfad)?;
    Ok(pfad)
}
