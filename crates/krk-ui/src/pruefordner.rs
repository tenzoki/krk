//! Der Pruefordner der Proben dieser Kiste: **die** eine Fassung.
//!
//! Ein Ordner unter dem Temporaerverzeichnis, der Zweck, Prozesskennung und
//! Laufnummer im Namen traegt und sich in `Drop` wieder abraeumt. Prozesskennung
//! und Laufnummer sind kein Zierrat: ohne sie treffen sich zwei gleichzeitige
//! Testlaeufe in demselben Ordner, und ein Fehlschlag daraus benennt nichts, was
//! am Code falsch waere. Zwei Defekte dazu stehen im Baum
//! (`260807-0800` fuer das Leistenmodell, `260810-1256` fuer die Vorschau).
//!
//! Bis zum 260810 stand diese halbe Seite viermal in dieser einen Kiste, in den
//! Probenmodulen von `vorschaumodell`, `editormodell`, `leistenmodell` und
//! `kommandos::pfadeingabe`. Der Defekt dazu ist
//! `issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`.
//!
//! Der Ordner liegt unter `std::env::temp_dir()` und ausdruecklich **nicht**
//! unter `~/Library/Caches/krk-messplatz`: der Messplatz gehoert der
//! Messstrecke, nicht den Proben.
//!
//! # Zwei Erzeuger, weil zwei Proben den Ordner fehlend brauchen
//!
//! [`Pruefordner::neu`] legt den Ordner an, [`Pruefordner::nur_name`] liefert
//! nur einen freien Namen. Die Proben der Gueltigkeitspruefung im
//! `leistenmodell` brauchen denselben Pfad einmal vorhanden und einmal fehlend
//! und schalten mit [`Pruefordner::anlegen`] und [`Pruefordner::loeschen`]
//! zwischen beiden Lagen um.
//!
//! # Der Zaehler steht hier und nicht je Probenmodul
//!
//! Alle Einheitsproben von `krk-ui` uebersetzen in **ein** Probenprogramm, also
//! in einen Prozess. Vier Zaehler in vier Modulen konnten deshalb zweimal
//! dieselbe Laufnummer liefern, und zwei Proben mit demselben Zweck in
//! verschiedenen Modulen haetten denselben Ordner getroffen. Ein Zaehler fuer
//! die Kiste schliesst das aus.
//!
//! Diese Fassung ist nicht mit der in `krk-core/tests/gemeinsam/mod.rs` oder der
//! in `krk-bench/src/wegwerfordner.rs` zusammenlegbar: ein Testziel und ein
//! Binaerziel sind je eine eigene Kiste, und `krk-ui` hat kein
//! Bibliotheksziel, das eine gemeinsame Fassung tragen koennte.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Die Laufnummer, die zwei Ordner desselben Zwecks im selben Prozess trennt.
static ZAEHLER: AtomicU64 = AtomicU64::new(0);

/// Ein Ordner unter dem Temporaerverzeichnis, der sich selbst abraeumt.
pub struct Pruefordner {
    pfad: PathBuf,
}

impl Pruefordner {
    /// Ein frisch angelegter, leerer Ordner.
    pub fn neu(zweck: &str) -> Self {
        let ordner = Self::nur_name(zweck);
        ordner.anlegen();
        ordner
    }

    /// Ein Name, unter dem noch nichts liegt. Angelegt wird der Ordner nicht.
    pub fn nur_name(zweck: &str) -> Self {
        let laufnummer = ZAEHLER.fetch_add(1, Ordering::Relaxed);
        let pfad = std::env::temp_dir().join(format!(
            "krk-ui-probe-{zweck}-{}-{laufnummer}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&pfad);
        Self { pfad }
    }

    /// Der Ordner selbst.
    pub fn pfad(&self) -> &Path {
        &self.pfad
    }

    /// Ein Pfad im Ordner, ohne dass dort etwas angelegt wird.
    pub fn unter(&self, name: &str) -> PathBuf {
        self.pfad.join(name)
    }

    pub fn anlegen(&self) {
        std::fs::create_dir_all(&self.pfad).expect("der Pruefordner laesst sich nicht anlegen");
    }

    pub fn loeschen(&self) {
        std::fs::remove_dir_all(&self.pfad).expect("der Pruefordner laesst sich nicht loeschen");
    }

    /// Legt einen Unterordner an und liefert seinen Pfad.
    ///
    /// **Die Schwesterfassung in `krk-core/tests/gemeinsam/mod.rs` fuehrt das
    /// schon**; hier fehlte es bis zum 260810, weil vor dem `Planordner` der
    /// Messstrecke keine Probe dieser Kiste einen Unterordner ueber den
    /// Pruefordner angelegt hat
    /// (`issues/260810-1430_*_planordner-in-messmodus-ist-die-dreizehnte-fassung-und-kann-jetzt-auf-die-gemeinsame-aufsetzen.md`).
    pub fn ordner(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        std::fs::create_dir_all(&pfad).expect("der Unterordner laesst sich nicht anlegen");
        pfad
    }

    /// Legt eine Datei an und liefert ihren Pfad.
    ///
    /// Der Inhalt ist `impl AsRef<[u8]>` und nicht `&str`, weil die Proben der
    /// Vorschau Bytefolgen schreiben, die in Rust-Quelltext als Zeichenkette
    /// nicht mehr das waeren, was auf der Platte stehen soll.
    pub fn datei(&self, name: &str, inhalt: impl AsRef<[u8]>) -> PathBuf {
        let pfad = self.unter(name);
        std::fs::write(&pfad, inhalt).expect("die Pruefdatei laesst sich nicht schreiben");
        pfad
    }

    /// Legt eine benannte Roehre an und liefert ihren Pfad.
    ///
    /// Angelegt wird sie ueber `mkfifo(1)` und nicht ueber einen Fremdaufruf:
    /// `krk-ui` traegt `#![deny(unsafe_code)]`, KRK legt keine Roehren an, und
    /// ein Werkzeug des Systems zu rufen ist der kleinere Eingriff. Dieselbe
    /// Begruendung wie in `krk-core/tests/gemeinsam/mod.rs`.
    pub fn roehre(&self, name: &str) -> PathBuf {
        let pfad = self.unter(name);
        let stand = std::process::Command::new("/usr/bin/mkfifo")
            .arg(&pfad)
            .status()
            .expect("mkfifo laesst sich nicht starten");
        assert!(stand.success(), "mkfifo ist gescheitert: {stand:?}");
        pfad
    }
}

impl Drop for Pruefordner {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.pfad);
    }
}
