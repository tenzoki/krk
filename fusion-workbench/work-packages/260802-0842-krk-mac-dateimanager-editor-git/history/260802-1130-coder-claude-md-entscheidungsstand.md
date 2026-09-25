# CLAUDE.md: Entscheidungsstand und Pfadangaben berichtigt

**Datum:** 260802-1130
**Agent:** coder
**Status:** Complete
**Auslöser:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1105_o_beantwortete-entscheidungen-noch-als-offen-gefuehrt.md`, Teil 1 (CLAUDE.md). Teil 2, der Circle-Datensatz `_t_circle.md`, gehört einem anderen Agenten; der Defekt bleibt offen.
**Geänderte Dateien:** `CLAUDE.md` (nur diese)

## Was geprüft wurde

Beide Entscheidungsspeicher wurden am Dateisystem gelistet, nicht aus einem Dokument übernommen:

- `fusion-workbench/shared/decisions/` — 2 beantwortet, 3 offen
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/` — 2 beantwortet, 0 offen

Der Circle-Speicher war in `CLAUDE.md` bis dahin überhaupt nicht erwähnt; die beiden dort liegenden Fragen fehlten in der Aufstellung vollständig.

Während der Bearbeitung wechselte ein Marker: `260802-1036_o_leistungszusagen-navigator.md` wurde zu `_a_leistungszusagen-navigator.md`, nachdem das Referenzgerät benannt war. Die erste Pfadprüfung schlug genau darauf an, die Tabelle wurde nachgezogen und der Satz im Abschnitt `## Maximen` mit korrigiert.

## Was geändert wurde

**Abschnitt `## Bindende Grundlage`.** Die Überschrift lautete "fünf offene Entscheidungen" und veraltete bei jeder Beantwortung. Sie heißt jetzt `## Bindende Grundlage: die Entscheidungsdatensätze`, ohne Zahl. Der Abschnitt erklärt zuerst, dass der Dateibestand verbindlich ist und die Tabelle nur eine datierte Momentaufnahme, nennt die Markerbedeutungen und beide Speicher, und trennt dann beantwortete von offenen Fragen in zwei Tabellen. Jede Zeile trägt den vollständigen Pfad statt eines Dateinamens ohne Speicherangabe. Die Antworten selbst stehen weiterhin nur im jeweiligen Datensatz und im Spec.

Angehängt ist der Hinweis, dass "beantwortet" nicht "erledigt" heißt: `_a_` wird erst mit der Umsetzung zu `_i_`.

**Abschnitt `## Projektstand`.** Die Aussage "kein Quellcode, keine Architektur" wurde am Wurzelverzeichnis nachgeprüft und stimmt: dort liegen nur `CLAUDE.md`, `idea.txt`, `.gitignore`, `.claude/` und `fusion-workbench/`. Ergänzt sind der Spec und die Konzeptprüfung, die es inzwischen gibt. Beide Analysenverzeichnisse sind leer, also steht das so da — der Technologievergleich hat nicht stattgefunden. Build- und Testkommandos bleiben ausdrücklich unbenannt.

**Abschnitt `## Maximen`.** Der Satz "Der Aktivierungs-Spec muss sie in messbare Zusagen übersetzen" war überholt: C8 des Specs enthält die Zeitzusagen, und das Referenzgerät ist seit 260802-1130 entschieden. Beide Stellen sind jetzt verlinkt.

**Abschnitt `## Sprache`.** Zwei Pfade zeigten ins Leere, weil sie plugin-relativ geschrieben waren: `rules/fusion-workbench-conventions.md` und `bin/fusion-rules` gibt es unterhalb des Projekts nicht. Sie sind auf `$FUSION_PLUGIN_ROOT/...` gezogen, die Stilprofile auf ihren vollen Workbench-Pfad. Die Zeile `**Language:** de` steht unverändert in Zeile 3.

## Abnahme

Jede in Backticks gesetzte Pfadangabe der Datei wurde aufgelöst und gegen das Dateisystem geprüft, `$FUSION_PLUGIN_ROOT` expandiert. 20 Pfade, 20 vorhanden, kein Fehltreffer.

## Nicht gemacht

Keine Technologiefestlegung, kein Build- oder Testkommando, keine Effort-Schätzung, kein Commit. Der Defekt bleibt offen.
