//! Was die Kommandos aus C2 und C10 tun, ohne AppKit.
//!
//! **Keine Zeile AppKit.** In diesem Verzeichnis steht keine `use objc2`-Zeile,
//! und das ist nachpruefbar, nicht nur gemeint. Es haelt die Rechnung hinter
//! den Tastenbefehlen; die Ansicht dazu ist [`crate::appkit::tabelle`], die das
//! Ergebnis in eine `NSTableView` stellt und die Blaetter am Fenster zeigt.
//!
//! Vier Module entlang dessen geschnitten, was ein Tastenbefehl bewegt:
//!
//! ```text
//! navigation   Auswahl bewegen: Zeile, Bildschirmseite, Anfang, Ende (C2)
//! auswahl      Mehrfachauswahl: markieren und weiterruecken (C2)
//! pfadeingabe  Einen Pfad pruefen und sagen, wohin KRK geht (C2 und C10)
//! operationen  Der Ablauf der Dateioperationen: Verzug, Buendelung, Texte (C4)
//! ```
//!
//! **`pfadeingabe` ist die eine Stelle, die einen Pfad prueft.** Zwei Ausloeser
//! benutzen sie, die Pfadeingabe von Hand auf Shift+Cmd+G und der Sprung zum
//! Inhalt der Zwischenablage auf Opt+Cmd+G. Der Unterschied ist allein, woher
//! der Wert kommt. Ein zweiter Navigationsweg daneben entstuende sonst, und die
//! erste Abweichung zwischen beiden waere ein Fehler ohne Pruefung.
//!
//! Was **nicht** hier steht: die Markierung selbst und der Aufstieg in den
//! uebergeordneten Ordner. Beide sind Zustand beziehungsweise Rechnung des
//! Kerns und stehen in `krk_core::verzeichnis`, wo `cargo test -p krk-core` sie
//! erreicht.

pub mod auswahl;
pub mod navigation;
pub mod operationen;
pub mod pfadeingabe;
