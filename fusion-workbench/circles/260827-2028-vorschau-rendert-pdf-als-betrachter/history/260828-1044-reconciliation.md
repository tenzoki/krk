# Abgleich — 260828-1044

**Circle:** 260827-2028-vorschau-rendert-pdf-als-betrachter (aktiv, `_t_`)
**Anker:** `2033626` → HEAD `48cd818`; Turns laut `fusion-events turns`: 1 (scope=checkout)
**Domain:** code

## Zahlen

- Pläne gelesen: 2 (Plan, Spec dieses Circles; `shared/planning` ohne offene Einträge). Aktualisiert: 2 — Plan `_p_` → `_c_` mit Reconciliation Log, Spec `_o_` → `_c_` mit Statuszeile.
- Entscheidungen gelesen: 2. Aktualisiert: 2 — `260827-2028_a_` → `_i_` (`Implemented: 2aee690, 22b8442, 5ff1ee4`); `260828-0712_o_` bleibt offen, Suchvermerk angehängt.
- Defekte gelesen: 4 (zwei im Circle, zwei geschlossene unter `shared/issues/`). Aktualisiert: 4 mit Abgleichsvermerk; kein Marker geändert.
- Reviews: keine im Circle.
- Neu gefilet: 1 (`issues/260828-1044_o_fuenf-history-dateien-…`).

## Befunde

- Alle elf `[DONE]` halten; Belegtabelle im Reconciliation Log des Plans. `make check` auf `48cd818` grün.
- Abweichung Plan ↔ Bau, ohne Handlungsbedarf: der `PDFViewDelegate` ist seit `8a8e638` die eigene Klasse `Verweisdelegierter` (`betrachter.rs:259-280`), nicht der Betrachter selbst; Grund ist der Absturzbericht unter `analyses/`.
- Circle-Datensatz: Kopffelder stimmen (`Active spec/plan` zeigt auf den Plan, Sitzungs-History richtig); `## Turn log` steht (uncommittet, 2 Zeilen), nennt „00:35–09:45", während `48cd818` um 10:40 eingetragen ist — Sache des Orchestrators.
- History-Dateien: Inhalte decken sich mit den Commits; fünf Dateinamen tragen Zeitstempel nach ihrem Commit (neuer Defekt oben).
- CLAUDE.md, nur gemeldet, nicht geändert (Kurator): Zeile 81 „`Wirkungsbereich` … trägt sieben Werte" ist seit `2aee690` falsch (acht); Zeile 137 „Es sind seit der Runde 14 zwei" eigene Textflächen — der PDF-Betrachter ist **nicht** in `ist_eigene_textflaeche` angemeldet, die Zahl hält, aber der Absatz kennt die dritte Fläche der Vorschau nicht; die Zeile zur Vorschau in der Rundentabelle fehlt für die Runde 20 (Kurator-Sache beim Abschluss).
- `shared/backlog/260828-0909_*` wird parallel vom Shaper bearbeitet; nicht angefasst, nicht bewertet.

## Nicht angefasst

Code, `Cargo.*`, `resources/`, der Circle-Datensatz (Marker `_t_` und Turn log gehören dem Orchestrator).
