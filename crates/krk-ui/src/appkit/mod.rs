#![allow(unsafe_code)]
//! Die Bruecke zu AppKit, und die einzige Stelle in `krk-ui` mit `unsafe`.
//!
//! Das Attribut oben ist die eine Ausnahme von `#![deny(unsafe_code)]` in
//! `main.rs`. Es steht hier und nirgends sonst: Lint-Regeln schlagen in die
//! eingebetteten Module durch, deshalb deckt der Kopf dieser Datei den ganzen
//! Teilbaum `src/appkit/` ab, und keine Datei darunter braucht die Ausnahme
//! ein zweites Mal.
//!
//! Sechs Module, entlang dessen geschnitten, was AppKit als eigenstaendige
//! Objekte fuehrt:
//!
//! ```text
//! anwendung ──> menue
//!           ──> fenster ──> tabelle ──> krk-core::verzeichnis
//!           ──> ereignisse ──┘      ──> krk-core::tasten
//!           ──> bildtakt ──> crate::messmodus
//! ```
//!
//! [`anwendung`] haelt `NSApplication` und den Anwendungsdelegierten und ist
//! der einzige Eintrittspunkt von aussen. [`menue`] baut das Hauptmenue von
//! Hand, weil es ohne Oberflaechenbau kein Nib gibt, aus dem es kaeme.
//! [`fenster`] baut das Fenster und seinen Delegierten. [`tabelle`] haelt das
//! Dateifenster: `NSTableView` in einer `NSScrollView`, Datenquelle und
//! Delegierter, und die Anbindung an das Ordnermodell des Kerns.
//! [`ereignisse`] haelt den lokalen Ereignisabgriff und ist der einzige
//! Eintrittspunkt fuer Tastendruecke; er schlaegt sie im Kern nach und reicht
//! das Kommando an die Datenquelle des Dateifensters weiter. [`bildtakt`] haelt
//! den `CADisplayLink` und den Nachschlag der Bildwiederholrate, die beiden
//! Beruehrungen mit AppKit, die die Fruehmessung aus Schritt 8 braucht.
//!
//! Der Pfeil von [`bildtakt`] nach `crate::messmodus` ist der einzige, der aus
//! diesem Verzeichnis herausfuehrt, und er traegt nur gewoehnliche Rust-Werte:
//! die Bildwiederholrate als Zahl und die Zeitpunkte der Bildgrenzen.

mod anwendung;
mod bildtakt;
mod ereignisse;
mod fenster;
mod menue;
mod tabelle;

pub use anwendung::starten;
