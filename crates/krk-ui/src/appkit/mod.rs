#![allow(unsafe_code)]
//! Die Bruecke zu AppKit, und die einzige Stelle in `krk-ui` mit `unsafe`.
//!
//! Das Attribut oben ist die eine Ausnahme von `#![deny(unsafe_code)]` in
//! `main.rs`. Es steht hier und nirgends sonst: Lint-Regeln schlagen in die
//! eingebetteten Module durch, deshalb deckt der Kopf dieser Datei den ganzen
//! Teilbaum `src/appkit/` ab, und keine Datei darunter braucht die Ausnahme
//! ein zweites Mal.
//!
//! Neun Module, entlang dessen geschnitten, was AppKit als eigenstaendige
//! Objekte fuehrt:
//!
//! ```text
//! anwendung ──> menue
//!           ──> fenster ──> aufteilung ──> tabelle ──> krk-core::verzeichnis
//!           ──> ereignisse            ──> tableiste     crate::tabs
//!           ──> bildtakt ──> crate::messmodus
//!                                     ──> statuszeile
//! ```
//!
//! [`anwendung`] haelt `NSApplication` und den Anwendungsdelegierten und ist
//! der einzige Eintrittspunkt von aussen. [`menue`] baut das Hauptmenue von
//! Hand, weil es ohne Oberflaechenbau kein Nib gibt, aus dem es kaeme.
//! [`fenster`] baut das Fenster und seinen Delegierten. [`aufteilung`] haelt
//! die `NSSplitView` mit den vier Bereichen aus C7, ihre Mindestbreiten und die
//! Markierung des aktiven Dateifensters. [`tabelle`] haelt das Dateifenster:
//! `NSTableView` in einer `NSScrollView`, Datenquelle und Delegierter, und die
//! Anbindung an das Tabmodell. [`tableiste`] ist die Leiste an seinem Kopf,
//! [`statuszeile`] die Zeile an seinem Fuss. [`ereignisse`] haelt den lokalen
//! Ereignisabgriff und ist der einzige Eintrittspunkt fuer Tastendruecke; er
//! schlaegt sie im Kern nach und reicht das Kommando an eine gewoehnliche
//! Rust-Senke weiter. [`bildtakt`] haelt den `CADisplayLink` und den Nachschlag
//! der Bildwiederholrate, die beiden Beruehrungen mit AppKit, die die
//! Fruehmessung aus Schritt 8 braucht.
//!
//! Drei Pfeile fuehren aus diesem Verzeichnis heraus, und alle drei tragen nur
//! gewoehnliche Rust-Werte: [`bildtakt`] gibt `crate::messmodus` die
//! Bildwiederholrate und die Zeitpunkte der Bildgrenzen, [`tabelle`] haelt das
//! Tabmodell aus `crate::tabs`, und [`aufteilung`] rechnet die Breiten mit
//! `crate::fenstermodell`. Keines der drei Ziele nennt eine `objc2`-Kiste.

mod anwendung;
mod aufteilung;
mod bildtakt;
mod ereignisse;
mod fenster;
mod menue;
mod statuszeile;
mod tabelle;
mod tableiste;

pub use anwendung::starten;
