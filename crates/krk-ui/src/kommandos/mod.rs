//! Was die Kommandos aus C2 und C10 tun, ohne AppKit.
//!
//! **Keine Zeile AppKit.** In diesem Verzeichnis steht keine `use objc2`-Zeile,
//! und das ist nachpruefbar, nicht nur gemeint. Es haelt die Rechnung hinter
//! den Tastenbefehlen; die Ansicht dazu ist [`crate::appkit::tabelle`], die das
//! Ergebnis in eine `NSTableView` stellt und die Blaetter am Fenster zeigt.
//!
//! Die Module sind entlang dessen geschnitten, was ein Tastenbefehl bewegt —
//! und seit der Runde 13 stehen zwei daneben, die kein Tastenbefehl sind.
//! **Wie viele es sind, sagt `grep -c '^pub mod'` auf diese Datei und keine
//! Zahl an dieser Stelle**: die Zahl hier ist zwischen dem 260823 und dem
//! 260825 zweimal falsch geworden, und die zwei Zahlen, die einmal danebenstanden,
//! sind aus demselben Grund schon gefallen
//! (`shared/issues/260823-1032_*_zwei-zahlen-im-modulkopf-der-kommandos-*`).
//!
//! ```text
//! zulaessigkeit Ob ein Befehl hier gerade wirken darf: kein Blatt, der
//!              Ersthelfer gehoert nicht AppKit, und der Fokus passt (C2 der
//!              Runde 7)
//! fokus        Ob ein Befehl dort wirkt, wo der Nutzer steht (C5)
//! rueckschritt Was die nackte Rueckschritt-Taste bedeutet: ein Zeichen des
//!              Filtertextes zurueck, nichts, oder in den Papierkorb (C1 und C6
//!              der Runde 10)
//! rundweg      Was cmd+e bedeutet: aus der Dateiliste oder aus der Vorschau in
//!              den Editor, aus dem Editor zurueck in die Dateiliste
//!              (Nutzerentscheid vom 260823-0942)
//! navigation   Auswahl bewegen: Zeile, Bildschirmseite, Anfang, Ende (C2)
//! auswahl      Mehrfachauswahl: markieren und weiterruecken (C2)
//! pfadeingabe  Einen Pfad pruefen und sagen, wohin KRK geht (C2 und C10)
//! operationen  Der Ablauf der Dateioperationen: Verzug, Buendelung, Texte (C4),
//!              die Antworten des Terminal-Befehls (C11) und die Texte der
//!              beiden Pfadkopierer und des Oeffners (C1 bis C3 der Runde 4)
//! loeschwarnung Der eine Loeschweg vor dem Auftrag: die Stufenfolge bis zur
//!              Rueckfrage, die Tafel der sechs Ausloeser mit ihrer Rangfolge,
//!              und die Texte, die daraus entstehen (C2, C3 und C4 der Runde 12)
//! abwurfregel  Was ein Abwurf aus einer fremden Anwendung trifft und ob er
//!              ausgefuehrt wird: die Marke und ihr Ziel, das Urteil und sein
//!              Grund (C4, C5 und C6 der Runde 13)
//! kontextmenue Was das Kontextmenue der Dateiliste traegt und worauf jeder
//!              Eintrag wirkt: der Archivname, der Ordnername zurueck und die
//!              Archive, die Unzip meint (Runde 17)
//! ```
//!
//! **`abwurfregel` und `kontextmenue` stehen am Schluss, und sie sind die zwei
//! Module hier, die kein Tastenbefehl sind.** Ihre Ausloeser sind Mausgesten —
//! ein Abwurf aus einer fremden Anwendung, ein Rechtsklick in der Dateiliste —,
//! und trotzdem wohnen sie hier: was das Verzeichnis zusammenhaelt, ist nicht
//! die Tastatur, sondern die Zusage seines zweiten Absatzes — die Rechnung
//! steht ohne Fenster da und ist ohne Fenster pruefbar. Fuer beide traegt diese
//! Zusage mehr als fuer jeden Tastenbefehl: ein `NSDraggingInfo` laesst sich
//! ohne Ziehsitzung nicht bauen und ein `NSMenu` nicht ohne den Hauptfaden, den
//! `libtest` nicht hergibt; jede Zeile, die im Annahmezweig oder im Menuebau
//! entschiede, waere allein von Hand nachpruefbar.
//!
//! **Die drei Menueeintraege bekommen aus demselben Grund keine
//! `Kommando`-Variante**: sie tragen weder eine Tastenkombination noch einen
//! Hauptmenueeintrag, haengen also weder an `Kommando::wirkungsbereich` noch an
//! `crate::belegungsmodell::bereich_des_kommandos`. Was sie stattdessen vor dem
//! wirkungslosen Menueeintrag schuetzt, steht bei
//! [`kontextmenue::Kontextbefehl`].
//!
//! **`zulaessigkeit` steht vor den uebrigen Tastenbefehlsmodulen, und das ist
//! die Reihenfolge des Weges.** Die Aussage ist die Reihenfolge und nicht die
//! Menge; eine Zahl an dieser Stelle waere mit dem naechsten Modul falsch, wie
//! sie es mit `rundweg` geworden ist
//! (`shared/issues/260823-1032_*_zwei-zahlen-im-modulkopf-der-kommandos-*`).
//! Sie ist seit der Runde 7 die erste Frage jedes Befehls, und `fokus` ist
//! einer ihrer drei Bestandteile geworden statt der einen Regel daneben. Zwei
//! Frager stellen sie, der Ereignisabgriff ueber
//! `Anwendungsdelegierter::kommando_ausfuehren` und die Ausgrauung des
//! Hauptmenues ueber `validateMenuItem:`; dass es eine Funktion ist und nicht
//! zwei Abfragen, ist der Grund, aus dem ihre Antworten nicht auseinanderlaufen
//! koennen. Seit der Runde 22 hat die Regel einen Rumpf und zwei Eingaenge,
//! den zweiten fuer die Dateiablage (`copy:` und `cut:` in der Dateiliste),
//! die kein Kommando ist; dieser Eingang hat seine zwei eigenen Frager an
//! denselben zwei Stellen.
//!
//! **`fokus` steht danach und vor den uebrigen.** Jeder Befehl laeuft durch
//! diese eine Regel, bevor irgendein anderes Modul ihn zu sehen bekommt. Bis
//! Schritt 18 wohnte sie in `operationen` und galt allein fuer die
//! Loeschtasten; mit der Leiste aus C5 betrifft sie jedes Kommando und gehoert
//! deshalb nicht mehr zu den Dateioperationen.
//!
//! **`rundweg` steht neben `rueckschritt` und aus demselben Grund.** Es sind
//! die zwei Regeln dieses Baums, die ein Befehl **nach seinem Fokus** trifft,
//! und beide stehen hinter `zulaessigkeit` statt darin: die Zulaessigkeit
//! entscheidet, **ob** eine Taste durchkommt, diese beiden, **was** sie dann
//! tut. Der Unterschied zwischen ihnen ist der Preis. Der falsche Zweig von
//! `rueckschritt` stellt einen Loeschbefehl, der von `rundweg` oeffnet oder
//! schliesst einen Editor; die Form ist trotzdem dieselbe, weil eine
//! Fallunterscheidung im Ausfuehrungszweig an keiner Probe zu fassen waere.
//!
//! **`rueckschritt` steht als drittes, und zwar hinter `zulaessigkeit` und
//! nicht darin.** Es beantwortet nicht, **ob** ein Befehl wirken darf, sondern
//! **welchen** von drei Ausgaengen ein Tastendruck nimmt, der die
//! Zulaessigkeitsfrage schon bestanden hat. Beides in eine Regel zu ziehen
//! ginge nicht: `zulaessig` sieht das nachgeschlagene Kommando und nicht den
//! Tastendruck, und `delete` und `cmd+delete` sind zu diesem Zeitpunkt
//! dasselbe `Kommando::InPapierkorb`. Eine Antwort dort traefe beide Wege
//! zugleich und graute den Menueeintrag aus, was C1.19 und C6.11 ausschliessen.
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

pub mod abwurfregel;
pub mod auswahl;
pub mod fokus;
pub mod kontextmenue;
pub mod loeschwarnung;
pub mod navigation;
pub mod operationen;
pub mod pfadeingabe;
pub mod rueckschritt;
pub mod rundweg;
pub mod zulaessigkeit;
