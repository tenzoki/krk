# Shaper, anticipated-circle: die Tastenbelegung als Markdown-Datei im Downloads-Ordner

**Datum:** 2026-08-09
**Status:** Complete
**Modus:** anticipated-circle (über `/fusion:direct`)
**Angelegter Circle:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/`

Diese Sitzung hat keinen Bezug zur Directive des aktiven Circles. Sie liegt hier, weil `bin/fusion-paths shaper` `OUT_HISTORY` bei aktivem Circle in dessen `history/` auflöst; der Gegenstand gehört dem neu angelegten Circle.

## Der Entwurf

Wörtlich vom Nutzer: KRK gibt die jeweils aktuelle Tastaturbelegung in formatierter Form aus, als Markdown-Datei im Downloads-Ordner des Nutzers, und zwar so, wie sie zum Zeitpunkt des Aufrufs gilt, einschließlich der Änderungen an der Auslieferungsbelegung.

Die Form war dabei schon entschieden. Am 260809-2035 hat der Nutzer Markdown im Downloads-Ordner gewählt und PDF über den Druckdialog verworfen, mit vier angenommenen Gründen (billig zu bauen, von Hand lesbar, versionierbar, PDF selbst herstellbar) und einem angenommenen Preis (kein fertiges Druckbild). Ein Druckbild bleibt als spätere Erweiterung ausdrücklich offen und steht deshalb im Grounding, nicht in der Directive.

Der Lauf war ohne Rückfragewerkzeug beauftragt. Was sich aus dem Dateibestand nicht entscheiden ließ, liegt als Entscheidungsdatensatz im neuen Circle, statt geraten worden zu sein.

## Was am Code geprüft wurde

Sechs Befunde tragen den Grounding-Abschnitt des neuen Datensatzes.

**Der Kern des Vorhabens ist eine zweite Ausgabeform, keine zweite Aufbereitung.** `Belegung::funktionen()` führt jede Funktion genau einmal mit allen ihren Kombinationen; `Belegungsmodell` in `crates/krk-ui/src/belegungsmodell.rs` gliedert sie nach neun Funktionsbereichen und liefert die Anzeigetexte. Das Modul benutzt keine AppKit-Schnittstelle, eine Ausgabefunktion kann also daneben stehen und ohne Fenster geprüft werden.

**Die geltende Belegung ist zur Laufzeit ein einziger Wert.** `keymap.toml` hält die vollständige Belegung des Nutzers und nicht seine Abweichungen. Die Formulierung des Entwurfs, die Ausgabe zeige die Änderungen des Nutzers mit, verlangt deshalb keinen Vergleich zweier Stände.

**Die Auslieferung zählt 71 Funktionen mit 79 Kombinationen**, davon sechs vom Hauptmenü zugestellt und 65 mit einem Kommando. Nachgezählt, nicht aus dem Kopfkommentar übernommen; beide Zahlen stimmen mit ihm überein. Ab Werk hat keine Funktion eine leere Tastenliste, unbelegte Funktionen entstehen erst durch den Nutzer.

**Der Downloads-Ordner ist keine neue Art von Zugriff.** KRK schreibt heute schon außerhalb seiner Ablage: die Dateioperationen aus C4 und das Sichern des Editors schreiben in jeden Ordner, den der Nutzer anzeigt. `resources/Info.plist` trägt bereits `NSDownloadsFolderUsageDescription` samt vier weiteren Texten für den Systemmechanismus für Transparenz, Zustimmung und Kontrolle. Neu ist allein, dass diese Ausgabe ihren Zielordner selbst wählt; `pfade::benutzerverzeichnis()` ist die eine Stelle im Kern, an die ein dritter Aufrufer gehört.

**Nicht gemessen** ist, ob die Rückfrage des Systems bei einem von KRK selbst angestoßenen Schreibvorgang erscheint und wie ein abgelehnter Zugriff aussieht. Der Befund steht als `speculation:` im Grounding, mit dem Vermerk, dass der Aktivierungs-Spec einen Prüflauf am gebauten Bündel vorsehen sollte.

**Die laufende Editor-Runde bewegt die Grundlage.** Der aktive Circle hat die Belegung um dreizehn Funktionen erweitert, den neunten Funktionsbereich `Editor` eingeführt und den Nachschlag für Buchstaben und Ziffern von Tastencode auf das gemeldete Zeichen umgestellt. Er ist keine Abhängigkeit, steht aber im Grounding, weil die Ausgabe beide Nachschlagarten zeigen muss und keine der drei bewegten Zahlen fest verdrahten darf.

## Was als Entscheidungsdatensatz abgelegt ist

Fünf Datensätze, alle `_o_`, alle in `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/`:

| Datensatz | Frage | Empfehlung des Shapers |
|---|---|---|
| `260809-2040_o_wie-wird-die-ausgabe-der-belegung-ausgeloest.md` | Kommando, Menüeintrag oder beides | Kommando in der Belegung |
| `260809-2040_o_wie-heisst-die-ausgabedatei-und-was-geschieht-bei-einer-vorhandenen.md` | Name und Verhalten bei Kollision | fester Name, Überschreiben |
| `260809-2040_o_was-steht-in-der-ausgabe-und-wonach-ist-sie-gegliedert.md` | Umfang und Ordnung | alle Funktionen, nach Funktionsbereich |
| `260809-2040_o_gehoert-der-wirkungsbereich-in-die-ausgabe.md` | dritte Spalte oder nicht | nicht in diesem Circle |
| `260809-2040_o_welche-belegung-schreibt-die-ausgabe-bei-offener-belegungsansicht.md` | gesicherter Stand oder Arbeitskopie | erst nach der ersten Frage entscheiden |

Die fünfte Frage hängt an der ersten und wird gegenstandslos, wenn die Ausgabe als gewöhnliche Funktion mit Kommando entsteht: bei offenem Blatt der Belegungsansicht führt der Ereignisabgriff nichts aus.

## Was nicht geschehen ist

Kein Spec. Ein Circle im Zustand *anticipated* trägt seine Directive im eigenen Datensatz; ein Spec entsteht bei der Aktivierung. Kein Planner-Aufruf, keine Turn-Schleife. Kein Circle außer dem neuen wurde angefasst.

## Nächster Schritt

Die Aktivierung ist Sache des Nutzers, über `/fusion:next`. Der aktive Circle `260807-2116-eingebauter-editor-mit-textmarken` läuft weiter; der neue Circle wartet.
