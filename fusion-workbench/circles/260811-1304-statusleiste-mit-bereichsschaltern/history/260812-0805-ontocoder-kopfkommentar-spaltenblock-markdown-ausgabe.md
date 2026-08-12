# Ontocoder: der Kopfkommentar des Spaltenblocks sagt jetzt, was in der Markdown-Ausgabe steht

**Datum:** 260812-0805
**Agent:** ontocoder
**Status:** Complete
**Maßstab:** `issues/260812-0727_o_die-drei-spaltenbefehle-stehen-nicht-in-der-markdown-ausgabe-obwohl-drei-stellen-es-zusagen.md`,
dritte der drei Stellen (Weg 3 des Datensatzes)
**Abnahme:** `make check` — **Exit 0**
**Ausgangsstand:** HEAD `6b6ea3c`, die beiden anderen Stellen vom Nutzer bereits berichtigt

## Auftrag

Genau eine Datei, `resources/default-keymap.toml`, keine Zeile Code. Der Kopfkommentar des
Spaltenblocks sagte zu, die drei Spaltenschalter stünden „wie jede andere in der
Belegungsansicht und in der Markdown-Ausgabe". Der zweite Teil stimmt nicht:
`belegungsausgabe::markdown` nimmt eine Funktion nur auf, wenn sie mindestens eine Kombination
trägt, und die drei tragen ab Werk keine.

## Was geändert ist

Eine Aussage, vier Zeilen davor, neun danach — `resources/default-keymap.toml`, Zeilen 306 bis
318 (vorher 306 bis 310):

- Der Halbsatz über die Belegungsansicht bleibt und heißt jetzt „in der Belegungsansicht am
  Schirm", damit der Gegensatz zur ausgegebenen Datei im Satzbau steht.
- Ein neuer Absatz sagt, dass die drei in der Markdown-Ausgabe der Runde 3 **nicht** stehen, und
  nennt den Grund und seine Quelle in der Machart der Datei: Umfang Möglichkeit 1, Nutzerantwort
  vom 260811-0110,
  `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/decisions/260809-2040_*_was-steht-in-der-ausgabe-und-wonach-ist-sie-gegliedert.md`.
  Der Verweis trägt die Endung `.md` und die Sternform des Markers, wie CLAUDE.md es für jeden
  Verweis verlangt, der von einer Suche gefunden werden soll.
- Der letzte Satz nennt dem Leser den Weg: wer einer der drei eine Kombination zuweist, findet
  sie danach auch in der ausgegebenen Datei.

Der Absatz darunter, der die leere Tastenliste mit der Knappheit begründet, ist unberührt.

## Die Suche nach weiteren Stellen

Der Auftrag verlangte eine Prüfung, ob weitere Stellen der Datei dasselbe Falsche sagen.
Gefunden ist keine:

- `grep -n -i 'markdown\|Ausgabe\|Downloads\|unbelegt\|belegte'` liefert drei Treffer. Zeile 18
  beschreibt das Feld `reserviert_fuer` („benannt und ab Werk unbelegt") und sagt nichts über die
  Ausgabe; Zeile 529 meint die Terminal-Ausgabe. Der einzige Treffer zur Markdown-Ausgabe war der
  berichtigte.
- Die Zählzeile des Dateikopfs (Zeile 34, „79 Funktionen mit zusammen 85 Kombinationen") ist
  nachgezählt und **stimmt**: 79 `[[funktion]]`-Blöcke, 85 Einträge in den Tastenlisten, davon
  drei leere Listen. Eine einzige Doppelung, `cmd+a`, und die ist im Dateikopf begründet.

**Nicht geprüft, weil außerhalb des Befunds:** die Zahl 39 im Absatz unter dem geänderten
(„die Auslieferung führt 39 frei gewählte Kombinationen"). Sie zitiert einen Datensatz der
Runde 1 und steht im Präsens; ob sie den heutigen Bestand trifft, hängt daran, was „frei
gewählt" gegen die Kürzel aus der Spec abgrenzt, und das entscheidet die Datei nicht.

## Abnahme

`make check` — Exit 0. Die vier Kommandos laufen grün, `cargo test --workspace` eingeschlossen;
die Belegungsdatei geht über `include_str!` in die Proben ein, ein Syntaxfehler hielte sie an.
Unabhängig davon sind die Einträge nachgezählt (79/85, unverändert gegenüber dem Stand vor der
Änderung).

## Was offen bleibt

Der Datensatz `260812-0727_o_…` bleibt **offen**, wie beauftragt. Er führt drei Stellen; diese
ist die dritte, und der Nutzer schließt ihn, wenn er alle drei gelesen hat. Nicht committet.
