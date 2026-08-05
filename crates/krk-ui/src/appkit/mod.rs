#![allow(unsafe_code)]
//! Die Bruecke zu AppKit, und die einzige Stelle in `krk-ui` mit `unsafe`.
//!
//! Das Attribut oben ist die eine Ausnahme von `#![deny(unsafe_code)]` in
//! `main.rs`. Es steht hier und nirgends sonst: Lint-Regeln schlagen in die
//! eingebetteten Module durch, deshalb deckt der Kopf dieser Datei den ganzen
//! Teilbaum `src/appkit/` ab, und keine Datei darunter braucht die Ausnahme
//! ein zweites Mal.
//!
//! Sechzehn Module, entlang dessen geschnitten, was AppKit als eigenstaendige
//! Objekte fuehrt:
//!
//! ```text
//! anwendung ──> menue
//!           ──> fenster ──> aufteilung ──> tabelle ──> krk-core::verzeichnis
//!           ──> ereignisse            ──> tableiste     crate::tabs
//!           ──> bildtakt ──> crate::messmodus           crate::kommandos
//!           ──> fsevents ──> crate::auffrischung        blaetter
//!           ──> volumes  ──> crate::auffrischung        zwischenablage
//!           ──> terminal              ──> statuszeile
//!
//! papierkorb ──> krk-core::operation::Papierkorb   (Aufruf von unten nach oben)
//! ```
//!
//! [`anwendung`] haelt `NSApplication` und den Anwendungsdelegierten und ist
//! der einzige Eintrittspunkt von aussen. [`menue`] baut das Hauptmenue von
//! Hand, weil es ohne Oberflaechenbau kein Nib gibt, aus dem es kaeme.
//! [`fenster`] baut das Fenster und seinen Delegierten. [`leiste`] haelt die
//! Lesezeichen- und Geraeteleiste aus C5, den zweiten fokussierbaren Bereich.
//! [`aufteilung`] haelt
//! die `NSSplitView` mit den vier Bereichen aus C7, ihre Mindestbreiten und die
//! Markierung des aktiven Dateifensters. [`tabelle`] haelt das Dateifenster:
//! `NSTableView` in einer `NSScrollView`, Datenquelle und Delegierter, und die
//! Anbindung an das Tabmodell. [`tableiste`] ist die Leiste an seinem Kopf,
//! [`statuszeile`] die Zeile an seinem Fuss. [`ereignisse`] haelt den lokalen
//! Ereignisabgriff und ist der einzige Eintrittspunkt fuer Tastendruecke; er
//! schlaegt sie im Kern nach und reicht das Kommando an eine gewoehnliche
//! Rust-Senke weiter. [`bildtakt`] haelt den `CADisplayLink` und den Nachschlag
//! der Bildwiederholrate, die beiden Beruehrungen mit AppKit, die die
//! Fruehmessung aus Schritt 8 braucht. [`blaetter`] haelt die gemeinsame Huelle
//! fuer die Dialoge am Fenster und darin das Eingabeblatt der Pfadeingabe aus
//! C2. [`zwischenablage`] haelt die beiden Beruehrungen aus C10, das Lesen von
//! `NSPasteboard` und die Uebergabe einer Web-Adresse an den Systembrowser.
//! [`terminal`] haelt die eine aus C11: die Aufloesung der eingestellten
//! Buendelkennung und die Uebergabe des angezeigten Ordners an die so gefundene
//! Anwendung, beides ueber `NSWorkspace`.
//! [`fsevents`] haelt die Bindung an FSEvents und beobachtet die Ordner, die
//! gerade auf dem Schirm stehen; [`volumes`] haelt die `NSWorkspace`-
//! Beobachtung und meldet, wann ein Datentraeger kommt und geht (beide C9).
//! [`papierkorb`] haelt `NSFileManager.trashItemAtURL:` und ist die eine
//! Stelle, an der ein Aufruf von unten nach oben laeuft: die
//! Operationsmaschine im Kern bekommt ihn ueber eine Schnittstelle
//! hereingereicht, die AppKit nicht kennt.
//!
//! Sechs Pfeile fuehren aus diesem Verzeichnis heraus, und alle sechs tragen
//! nur gewoehnliche Rust-Werte: [`bildtakt`] gibt `crate::messmodus` die
//! Bildwiederholrate und die Zeitpunkte der Bildgrenzen, [`tabelle`] haelt das
//! Tabmodell aus `crate::tabs` und rechnet mit `crate::kommandos`,
//! [`aufteilung`] rechnet die Breiten mit `crate::fenstermodell`, und
//! [`fsevents`] wie [`volumes`] reichen Pfade an `crate::auffrischung`. Keines
//! der Ziele nennt eine `objc2`-Kiste.

mod anwendung;
mod aufteilung;
mod bildtakt;
mod blaetter;
mod ereignisse;
mod fenster;
mod fsevents;
mod leiste;
mod menue;
mod papierkorb;
mod statuszeile;
mod tabelle;
mod tableiste;
mod terminal;
mod volumes;
mod zwischenablage;

pub use anwendung::starten;
