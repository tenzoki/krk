//! Der Wegwerfordner der Proben dieser Kiste: **die** eine Fassung.
//!
//! Ein Ordnername unter dem Temporaerverzeichnis, der Prozesskennung und
//! Laufnummer traegt und sich in `Drop` wieder abraeumt, samt dem Steckbrief,
//! den [`crate::fixture`] neben den Ordner legt.
//!
//! Bis zum 260810 stand diese halbe Seite zweimal im Baum, in den Probenmodulen
//! von `fixture.rs` und `messen.rs`. Beide liegen in **einer** Kiste, also
//! genuegt eine Fassung; der Defekt dazu ist
//! `issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.
//!
//! Der Ordner liegt unter `std::env::temp_dir()` und ausdruecklich **nicht**
//! unter `~/Library/Caches/krk-messplatz`: der Messplatz gehoert der
//! Messstrecke, nicht den Proben.
//!
//! # Angelegt wird hier nichts
//!
//! [`Wegwerfordner::neu`] liefert nur einen freien Namen. Die Proben dieser
//! Kiste geben ihn an den Pruefordner-Erzeuger weiter, und der will ihn selbst
//! anlegen. Das unterscheidet diese Fassung von der in `krk-ui`, wo der Ordner
//! meist schon stehen muss.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::fixture;

/// Die Laufnummer, die zwei Ordner desselben Zwecks im selben Prozess trennt.
static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein Ordnername unter dem Temporaerverzeichnis, der sich selbst abraeumt.
pub struct Wegwerfordner {
    pfad: PathBuf,
}

impl Wegwerfordner {
    /// Ein Name, unter dem noch nichts liegt. Angelegt wird der Ordner nicht.
    pub fn neu(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let pfad = std::env::temp_dir().join(format!(
            "krk-bench-probe-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&pfad);
        Self { pfad }
    }

    pub fn pfad(&self) -> &Path {
        &self.pfad
    }
}

impl Drop for Wegwerfordner {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.pfad);
        // Der Steckbrief liegt **neben** dem Ordner und faellt deshalb nicht
        // mit ihm; ohne diese Zeile blieben Dateien im Temporaerverzeichnis
        // liegen.
        if let Ok(steckbrief) = fixture::steckbriefpfad(&self.pfad) {
            let _ = fs::remove_file(steckbrief);
        }
    }
}
