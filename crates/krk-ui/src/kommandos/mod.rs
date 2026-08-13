//! Was die Kommandos aus C2 und C10 tun, ohne AppKit.
//!
//! **Keine Zeile AppKit.** In diesem Verzeichnis steht keine `use objc2`-Zeile,
//! und das ist nachpruefbar, nicht nur gemeint. Es haelt die Rechnung hinter
//! den Tastenbefehlen; die Ansicht dazu ist [`crate::appkit::tabelle`], die das
//! Ergebnis in eine `NSTableView` stellt und die Blaetter am Fenster zeigt.
//!
//! Sechs Module entlang dessen geschnitten, was ein Tastenbefehl bewegt:
//!
//! ```text
//! zulaessigkeit Ob ein Befehl hier gerade wirken darf: kein Blatt, der
//!              Ersthelfer gehoert nicht AppKit, und der Fokus passt (C2 der
//!              Runde 7)
//! fokus        Ob ein Befehl dort wirkt, wo der Nutzer steht (C5)
//! navigation   Auswahl bewegen: Zeile, Bildschirmseite, Anfang, Ende (C2)
//! auswahl      Mehrfachauswahl: markieren und weiterruecken (C2)
//! pfadeingabe  Einen Pfad pruefen und sagen, wohin KRK geht (C2 und C10)
//! operationen  Der Ablauf der Dateioperationen: Verzug, Buendelung, Texte (C4),
//!              die Antworten des Terminal-Befehls (C11) und die Texte der
//!              beiden Pfadkopierer und des Oeffners (C1 bis C3 der Runde 4)
//! ```
//!
//! **`zulaessigkeit` steht vor den fuenf anderen, und das ist die Reihenfolge
//! des Weges.** Sie ist seit der Runde 7 die erste Frage jedes Befehls, und
//! `fokus` ist einer ihrer drei Bestandteile geworden statt der einen Regel
//! daneben. Zwei Frager stellen sie, der Ereignisabgriff ueber
//! `Anwendungsdelegierter::kommando_ausfuehren` und die Ausgrauung des
//! Hauptmenues ueber `validateMenuItem:`; dass es eine Funktion ist und nicht
//! zwei Abfragen, ist der Grund, aus dem ihre Antworten nicht auseinanderlaufen
//! koennen.
//!
//! **`fokus` steht danach und vor den vier uebrigen.** Jeder Befehl laeuft durch
//! diese eine Regel, bevor irgendein anderes Modul ihn zu sehen bekommt. Bis
//! Schritt 18 wohnte sie in `operationen` und galt allein fuer die
//! Loeschtasten; mit der Leiste aus C5 betrifft sie jedes Kommando und gehoert
//! deshalb nicht mehr zu den Dateioperationen.
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
pub mod fokus;
pub mod navigation;
pub mod operationen;
pub mod pfadeingabe;
pub mod zulaessigkeit;
