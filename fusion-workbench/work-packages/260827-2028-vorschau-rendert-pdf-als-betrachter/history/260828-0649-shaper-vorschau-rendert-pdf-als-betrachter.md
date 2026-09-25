# Shaper, user-direct: Spec für die Runde 20, die Vorschau rendert PDF als Betrachter

**Datum:** 2026-08-28, Klärungsrunde 1 um 00:44, Spec um 06:49
**Modus:** user-direct, dispatched vom Orchestrator in Phase 0b (Circle `260827-2028-vorschau-rendert-pdf-als-betrachter`, aktiv)
**Status:** Complete

## Eingang

Die Directive und der Grounding snapshot aus `_t_circle.md`, dazu der offene Datensatz `decisions/260827-2028_*_welche-tasten-bekommen-zoom-und-seitensprung-des-pdf-betrachters.md`.

## Klärungsrunde 1 (260828-0044) und Antworten (wörtlich „1b, 2a, 3a")

1. Tasten für Zoom und Seitensprung: b, drei Befehle `cmd+plus`, `cmd+minus`, `cmd+0`; kein Sprungblatt, gesprungen wird durch Blättern, der Seitenzähler ist die einzige Seitenauskunft.
2. Fortlaufende Rolle oder je eine Seite: a, fortlaufend.
3. Pfeil hoch und runter im PDF: a, wirkungslos wie in der Textvorschau.

Keine zweite Runde nötig.

## Befunde am Grounding snapshot, am 260828 gegen den Baum geprüft

- `Wirkungsbereich` trägt sieben Werte und keinen für „nur Vorschau" mehr (`belegung.rs`, seit 260823).
- Der Konflikttest kennt keine Bereiche: Kombination plus Zusteller (`belegung.rs`, Modulkopf).
- `+` und `-` stehen nicht in `parser::TASTEN`; Buchstaben und Ziffern werden über das Zeichen nachgeschlagen, und dieser Weg trägt die zwei neuen Namen `plus` und `minus`.
- `opt+cmd+g` ist an `zwischenablage_springen` vergeben; mit Antwort 1b gegenstandslos. `cmd+0` ist frei.
- Bild-auf/-ab, Pos1/Ende tragen `Dateifenster` und laufen in der Vorschau an AppKit (Datensatz 260819-2216 zu den Pfeiltasten).
- `cmd+c` ist `text_kopieren` mit Zusteller Menü; in der Vorschau unzulässig, läuft an AppKit, und die Textansicht fängt es in `writeSelectionToPasteboard:types:` ab. Der Betrachter braucht dieselbe eine Abfangstelle, sonst schreibt seine Klasse selbst auf `NSPasteboard`.
- `Cargo.lock` führt kein `cc`.
- `Rang::ALLE` hat sechs Werte ohne Auffangzweig.

## Ergebnis

- Spec: `planning/260828-0649_o_spec-vorschau-rendert-pdf-als-betrachter.md`, fünf Fähigkeiten, 45 Abnahmekriterien, zehn am Tor überstimmbare Festlegungen A1 bis A10.
- Datensatz `decisions/260827-2028_*_welche-tasten-…` mit `Answered:`-Zeile, Marker `_o_` → `_a_`.
- Keine neuen Defekte, keine neuen Entscheidungsdatensätze: alle drei Fragen sind beantwortet, die offenen Fragen zu L7 und zur Schriftgröße der Vorschau stehen unter `## User Decisions Pending` des Specs.
- Nicht committet.
