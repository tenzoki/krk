`make tasten` ist der interaktive Tastenlogger und trägt keine dritte Spalte

---
Schritt 16 des Plans der Runde 23 verlangt, `make tasten` „vor der ersten Codeänderung und danach in je zwei Dateien" zu schreiben und zu vergleichen, und erwartet dabei „in der dritten Spalte der Zeilen `fenster_wechseln`, `auswahl_hoch` und `auswahl_runter` den Wechsel von „Dateifenster, Leiste und Vorschau" auf „Dateifenster, Leiste, Vorschau und Git-Bereich"". Entscheidung 2 desselben Plans sagt es noch einmal: „dieser Text steht in der dritten Spalte jeder Zeile von `make tasten` und in `docs/tastenbelegung.md`".

**Beides trifft auf diesen Baum nicht zu**, und der Schritt 16 kann den Vergleich deshalb nicht fahren.

- `make tasten` (`Makefile:89-90`) baut das Bündel und startet es mit `--tasten-protokoll`. Die Marke schaltet den Protokollmodus des **Ereignisabgriffs** ein (`crates/krk-ui/src/main.rs:103-105`, `appkit/anwendung.rs:8574-8576`). Die Ausgabe entsteht in `ereignisse::protokollieren` (`crates/krk-ui/src/appkit/ereignisse.rs:817-834`) und lautet je **empfangenem Tastendruck** eine Zeile der Form `tastencode=… zeichen=… maske=… kombination=… funktion=…`. Keine Tabelle, keine Spalten, kein `Wirkungsbereich`.
- Der Lauf öffnet ein Fenster und endet erst mit `Cmd+Q` (die Hilfezeile des Ziels sagt es: „Tastencodes protokollieren, Beenden mit Cmd+Q"). Er verlangt KRK im Vordergrund und ist damit Nutzerarbeit wie der Abnahmelauf aus Schritt 17; **kein Agent kann ihn fahren**. Schritt 16 führt ihn trotzdem unter den Abnahmekommandos ohne Fenster.
- `docs/tastenbelegung.md` gibt es in diesem Baum nicht; ein Verzeichnis `docs/` steht nicht darin.

**Die Fläche, die es wirklich gibt**, ist die Markdown-Ausgabe der Runde 3: `belegungsausgabe::markdown` (`crates/krk-ui/src/belegungsausgabe.rs`) baut die Tabelle, deren dritte Spalte `wirkung(funktion)` liest (`:263-272`, erste Begründungslage: `kommando.wirkungsbereich().beschriftung()`). Geschrieben wird sie in den Ordner „Downloads", ausgelöst vom Menüeintrag „Tastenbelegung als Markdown sichern" (Selektor `tastenbelegungSichern:`) — also aus der laufenden Anwendung heraus und ebenfalls nicht kopflos.

**Was Schritt 16 stattdessen belegt hat.** Die erwartete Änderung ist an ihren zwei Eingaben geprüft, und die beiden zusammen legen die Tabellenzeilen fest:

- `Wirkungsbereich::beschriftung` (`crates/krk-core/src/tasten/belegung.rs`) hat genau eine ihrer acht Zeilen geändert, die für `Navigator`; die übrigen sieben stehen wörtlich wie vor der Runde. `Wirkungsbereich` trägt vor und nach der Runde acht Werte.
- Genau drei Kommandos tragen `Wirkungsbereich::Navigator`: `FensterWechseln`, `AuswahlHoch` und `AuswahlRunter` (`belegung.rs`, der Zweig unter dem Kommentar „Die drei Befehle des Navigators"), mit den Kennungen `fenster_wechseln`, `auswahl_hoch` und `auswahl_runter`.

**Abnahme:** Schritt 16 des Plans nennt für die Tabelle die Fläche, die sie erzeugt, und nicht `make tasten`; Entscheidung 2 desselben Plans ebenso, und ohne `docs/tastenbelegung.md`. Ob der Vergleich der Tabelle in die Nutzerarbeit von Schritt 17 wandert oder ob die Fläche einen kopflosen Aufruf bekommt, ist eine Frage für den Nutzer und keine dieses Defekts.

---
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Domain:** workbench — der Plan der Runde 23, Schritt 16 und Entscheidung 2. Am Code ist nichts falsch; falsch ist die Beschreibung des Prüfmittels.
Gefunden in Schritt 16 selbst, beim Versuch, den Vergleich zu fahren. Die Hälfte des Schritts, die `make menue` betrifft, ist gefahren und trägt genau die erwarteten Abweichungen.
