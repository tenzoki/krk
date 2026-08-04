//! Umbenennen im Stapel: das Regelmodell, die Vorschau und die
//! Kollisionspruefung (C4).
//!
//! ```text
//!  markierte Namen ──┐
//!                    ├──> vorschau(regel, markierte, bestand) ──> Vorschau
//!  Regel ────────────┘                                              │
//!  Bestand des Ordners ────────────────────> kollision::pruefen ────┘
//!
//!  Ausfuehrung: je Zeile ohne Kollision genau ein
//!               operation::umbenennen (S15)
//! ```
//!
//! # Was hier steht und was nicht
//!
//! Hier steht, **welchen neuen Namen** ein Eintrag bekaeme und **was daran
//! nicht geht**. Das Umbenennen selbst steht in
//! [`crate::operation::umbenennen`] aus S15 und wird je Eintrag von dort
//! gerufen; ein zweiter Umbenennungsweg entsteht nicht. Auch die Namenspruefung
//! ist dieselbe: [`crate::operation::name_pruefen`] entscheidet hier wie dort,
//! was kein Name ist.
//!
//! Die Oberflaeche dazu ist `krk-ui`'s `appkit/blaetter/stapelumbenennen.rs`.
//! Sie zeigt, was hier ausgerechnet wurde, und rechnet selbst nichts.
//!
//! # Groß- und Kleinschreibung ist nicht enthalten
//!
//! Der Nutzer hat sie am 260802-1105 nicht genannt, und der Spec fuehrt sie
//! ausdruecklich als nicht zugesagt
//! (`decisions/260802-1036_a_umbenennen-im-stapel-umfang.md`). Eine Umschaltung
//! der Schreibweise laesst sich ueber Suchen und Ersetzen nicht ausdruecken,
//! waere also eine eigene Regelart und damit eine Ausweitung des Umfangs.
//!
//! # Warum das Modul neben `operation::umbenennen` steht und nicht darin
//!
//! `operation::umbenennen` fasst das Dateisystem an. Dieses Modul nicht: es
//! rechnet auf Zeichenketten und ist deshalb ohne Pruefordner pruefbar. Die
//! fuenf Faelle aus dem Abnahmekriterium von S17 laufen in
//! `tests/umbenennen.rs` ohne eine einzige angelegte Datei.

pub mod kollision;
pub mod regel;
pub mod vorschau;

pub use kollision::Kollision;
pub use regel::{HOECHSTE_STELLENZAHL, Nummerierung, Regel, Regelfehler};
pub use vorschau::{Vorschau, Vorschauzeile, vorschau};
