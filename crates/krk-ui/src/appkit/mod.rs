#![allow(unsafe_code)]
//! Die Bruecke zu AppKit, und die einzige Stelle in `krk-ui` mit `unsafe`.
//!
//! Das Attribut oben ist die eine Ausnahme von `#![deny(unsafe_code)]` in
//! `main.rs`. Es steht hier und nirgends sonst: Lint-Regeln schlagen in die
//! eingebetteten Module durch, deshalb deckt der Kopf dieser Datei den ganzen
//! Teilbaum `src/appkit/` ab, und keine Datei darunter braucht die Ausnahme
//! ein zweites Mal.
//!
//! Vierundzwanzig Module, entlang dessen geschnitten, was AppKit als
//! eigenstaendige Objekte fuehrt — bis auf [`koordinaten`], das keines fuehrt
//! und trotzdem hier liegt, weil die Koordinate, in die es rechnet, AppKits ist:
//!
//! ```text
//! anwendung ──> menue
//!           ──> fenster ──> aufteilung ──> tabelle ──> krk-core::verzeichnis
//!           │            ──> bereichsleiste            crate::fenstermodell
//!           ──> ereignisse            ──> tableiste     crate::tabs
//!           ──> bildtakt ──> crate::messmodus           crate::kommandos
//!           ──> fsevents ──> crate::auffrischung        blaetter
//!           ──> volumes  ──> crate::auffrischung        zwischenablage
//!           ──> terminal              ──> statuszeile
//!           ──> hinweis                                 standardprogramm
//!           ──> vorschau ──> crate::vorschaumodell  ──> tableiste
//!           │             ──> nummernspalte
//!           ──> editor   ──> crate::editormodell
//!           │             ──> nummernspalte ──> krk-core::text::zeilen
//!           ──> belegungsansicht ──> crate::belegungsmodell
//!
//! papierkorb ──> krk-core::operation::Papierkorb   (Aufruf von unten nach oben)
//! ```
//!
//! [`anwendung`] haelt `NSApplication` und den Anwendungsdelegierten und ist
//! der einzige Eintrittspunkt von aussen. [`menue`] baut das Hauptmenue von
//! Hand, weil es ohne Oberflaechenbau kein Nib gibt, aus dem es kaeme.
//! [`fenster`] baut das Fenster und seinen Delegierten. [`leiste`] haelt die
//! Lesezeichen- und Geraeteleiste aus C5, den zweiten fokussierbaren Bereich.
//! [`vorschau`] haelt das Vorschaufenster aus C6, den dritten: Text- und
//! Bildanzeige samt der zweiten Tableiste, waehrend Tabs und Halteverhalten in
//! `crate::vorschaumodell` wohnen.
//! [`editor`] haelt die Textflaeche des eingebauten Editors:
//! eine editierbare `NSTextView` in einer `NSScrollView`, waehrend gehaltene
//! Datei, Stand, Ansichtswahl und Suchlauf in `crate::editormodell` wohnen. Er
//! und die Vorschau teilen sich denselben Platz in der Fensterzeile.
//! [`nummernspalte`] haelt die Zeilennummern aus C10, und zwar als **eine**
//! Klasse fuer beide Textflaechen: Editor und Vorschau haengen dieselbe
//! `NSRulerView`-Unterklasse in die senkrechte Linealstelle ihrer
//! Bildlaufansicht. Gezaehlt wird dabei nicht hier, sondern in
//! `krk_core::text::zeilen`.
//! [`koordinaten`] haelt den einen Wechsel zwischen den Byteversaetzen von
//! `krk_core::text` und den UTF-16-Einheiten, in denen AppKits Textsystem
//! zaehlt. Nummernspalte, Zeilensprung und Suche gehen alle drei durch ihn;
//! eine zweite Umrechnung daneben muesste die Zusage ueber die Zeichengrenzen
//! ein zweites Mal tragen.
//! [`aufteilung`] haelt
//! die `NSSplitView` mit den fuenf Bereichen aus C7, ihre Mindestbreiten und die
//! Markierung des aktiven Dateifensters.
//! [`bereichsleiste`] haelt die Leiste am Fensterfuss aus C1 bis C3 der
//! Bereichsleisten-Runde: acht Ankreuzfelder, fuenf fuer die Bereiche und drei
//! fuer die schaltbaren Spalten. Sie liegt **neben** der Aufteilung und nicht
//! darin — beide sind Unteransichten der Traegerflaeche, die
//! `fenster::fensterinhalt` baut —, und keiner ihrer Schalter nimmt den
//! Ersthelferrang an; die Begruendung steht in ihrem Modulkopf.
//! [`tabelle`] haelt das Dateifenster:
//! `NSTableView` in einer `NSScrollView`, Datenquelle und Delegierter, und die
//! Anbindung an das Tabmodell. [`tableiste`] ist die Leiste an seinem Kopf,
//! [`statuszeile`] die Zeile an seinem Fuss. [`ereignisse`] haelt den lokalen
//! Ereignisabgriff und ist der einzige Eintrittspunkt fuer Tastendruecke; er
//! schlaegt sie im Kern nach und reicht das Kommando an eine gewoehnliche
//! Rust-Senke weiter. [`bildtakt`] haelt den `CADisplayLink` und den Nachschlag
//! der Bildwiederholrate, die beiden Beruehrungen mit AppKit, die die
//! Fruehmessung aus Schritt 8 braucht. [`blaetter`] haelt die gemeinsame Huelle
//! fuer die Dialoge am Fenster und darin das Eingabeblatt der Pfadeingabe aus
//! C2. [`hinweis`] haelt daneben das eine **anwendungsmodale** Hinweisfenster:
//! kein Blatt, keine Antwort, sondern die letzte Ausgabe vor dem Beenden, wenn
//! sich der Tastenabgriff nicht einrichten laesst. Die Abgrenzung zu den
//! Blaettern steht in seinem Modulkopf.
//! [`zwischenablage`] haelt die Beruehrungen, die seine eine Frage braucht:
//! das Lesen von `NSPasteboard` und die Uebergabe einer Web-Adresse an den
//! Systembrowser aus C10, das Lesen des Inhalts fuer die Vorschau und seit der
//! Runde 4 die Gegenrichtung, mit der die beiden Pfadkopierer aus C1 und C2
//! ihren Text ablegen.
//! [`terminal`] haelt die eine aus C11: die Aufloesung der eingestellten
//! Buendelkennung und die Uebergabe des angezeigten Ordners an die so gefundene
//! Anwendung, beides ueber `NSWorkspace`.
//! [`standardprogramm`] haelt die eine aus C3 der Runde 4: die Uebergabe eines
//! Eintrags an das Programm, das das System fuer ihn fuehrt, ueber
//! `NSWorkspace::openURL:`. Ein eigenes Modul neben den beiden davor, weil
//! keines von ihnen diese Frage stellt; die Abgrenzung steht in seinem
//! Modulkopf.
//! [`belegungsansicht`] haelt die Belegungsansicht aus C3 als Blatt am
//! Fenster: die Tabelle der Funktionen, die Schaltflaechen und die
//! Meldungszeile, waehrend die Arbeitskopie der Belegung in
//! `crate::belegungsmodell` wohnt.
//! [`fsevents`] haelt die Bindung an FSEvents und beobachtet die Ordner, die
//! gerade auf dem Schirm stehen; [`volumes`] haelt die `NSWorkspace`-
//! Beobachtung und meldet, wann ein Datentraeger kommt und geht (beide C9).
//! [`papierkorb`] haelt `NSFileManager.trashItemAtURL:` und ist die eine
//! Stelle, an der ein Aufruf von unten nach oben laeuft: die
//! Operationsmaschine im Kern bekommt ihn ueber eine Schnittstelle
//! hereingereicht, die AppKit nicht kennt.
//!
//! **Jeder Weg aus diesem Verzeichnis heraus traegt nur gewoehnliche
//! Rust-Werte; keines der Ziele nennt eine `objc2`-Kiste.** Das ist die
//! Architekturgrenze, an der dieses Projekt haengt, und sie steht hier als
//! Regel und **nicht als Zaehlung**. Bis zum 260810 stand hier eine Zahl
//! ("acht Pfeile ... und alle acht"); sie war schon vor dem Editor falsch, hat
//! `anwendung` mit seinen neun Zielen gar nicht gefuehrt und `fsevents` wie
//! `volumes` Ziele zugeschrieben, die sie nicht nennen. Kein Bau und keine
//! Probe faengt eine Zahl in Prosa, und wer die Grenze nachpruefen will, haelt
//! ein Verzeichnis nach acht geprueften Stellen fuer durchgesehen. Der Befund
//! ist `issues/260809-1655_*_acht-pfeile-aus-appkit-heraus-sind-es-nicht-....md`.
//! Wer eine Zahl braucht, erhebt sie: `grep -rn 'use crate::'
//! crates/krk-ui/src/appkit/` nennt die Zeilen, dann ist sie belegt und nicht
//! behauptet.
//!
//! **Zwei Lesarten, die nicht zusammenfallen.** Der Ueberblick oben zeichnet,
//! wohin Werte fliessen; ein `use crate::` dagegen sagt, welches Modul einen
//! Nachbarn ausserhalb dieses Verzeichnisses **nennt**. Wer nur meldet, nennt
//! niemanden: [`bildtakt`] gibt Rate und Zeitpunkte an eine gewoehnliche
//! Rust-Senke, die es beim Einrichten entgegennimmt, und traegt keine
//! `use crate::`-Zeile; ebenso [`fsevents`], dessen Pfade erst beim Aufrufer
//! bei `crate::auffrischung` landen. Beide stehen im Ueberblick mit einem Pfeil
//! und in der Aufstellung unten nicht.
//!
//! Die genannten Ziele, ohne den Anspruch, alle zu sein: [`anwendung`] nennt
//! die meisten, naemlich `crate::auffrischung`, `crate::belegungsmodell`,
//! `crate::editormodell`, `crate::fenstermodell`, `crate::fenstertitel`,
//! `crate::kommandos` (`fokus` und `operationen` als zwei getrennte Wege),
//! `crate::leistenmodell`, `crate::messmodus` und `crate::tabs`. Daneben:
//! [`tabelle`] haelt das Tabmodell aus `crate::tabs` und rechnet mit
//! `crate::kommandos`, [`leiste`] haelt die Zeilen aus `crate::leistenmodell`,
//! [`vorschau`] und [`zwischenablage`] den Inhalt aus `crate::vorschaumodell`,
//! [`editor`] den Stand aus `crate::editormodell` und die Einfaerbung aus
//! `crate::hervorhebung`, [`aufteilung`] rechnet die Breiten mit
//! `crate::fenstermodell` und die Rahmenrolle mit `crate::kommandos::fokus`,
//! [`belegungsansicht`] haelt die Arbeitskopie der Belegung aus
//! `crate::belegungsmodell`, und [`volumes`] zieht `crate::leistenmodell::Ort`.

mod anwendung;
mod aufteilung;
mod belegungsansicht;
mod bereichsleiste;
mod bildtakt;
mod blaetter;
mod editor;
mod ereignisse;
mod fenster;
mod fsevents;
mod hinweis;
mod koordinaten;
mod leiste;
mod menue;
mod nummernspalte;
mod papierkorb;
mod standardprogramm;
mod statuszeile;
mod tabelle;
mod tableiste;
mod terminal;
mod volumes;
mod vorschau;
mod zwischenablage;

pub use anwendung::starten;
