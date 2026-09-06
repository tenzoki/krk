# Analyst: neunzehn offene Entscheidungsfragen zur Vorlage aufbereitet

**Datum:** 2026-09-06 21:02
**Agent:** analyst
**Art:** Document Study
**Status:** Complete
**Baumstand:** `8276170`, Zweig `main`, `## main...origin/main [voraus 17]`

## Auftrag

Neunzehn offene Entscheidungsdatensätze aus zehn Runden so verdichten, dass der Nutzer sie im
Chat nacheinander beantworten kann: Frage in einem Satz, Möglichkeiten aus dem Datensatz,
Empfehlung, Aktualitätsprüfung gegen den heutigen Baum. Nichts entscheiden, nichts umbenennen,
keine Datei ändern.

## Was gelaufen ist

Alle neunzehn Datensätze vollständig gelesen. Jede Frage einzeln gegen den Baum geprüft, über
`grep`, `awk` über die betroffenen Aufzählungen, `git log -S` für die Datierung der
entscheidenden Änderungen, und Lesen der im Datensatz genannten Quelldateien.

Keiner der neunzehn trägt eine Zeile `Answer located:`; nachgeprüft über alle
Entscheidungsspeicher.

## Ergebnis

- **Vier Fragen sind vom Baum faktisch entschieden** und brauchen nur die Zustimmung:
  `260813-0159_*_…` (Menügliederung), `260813-0430_*_…` (Menükürzel bei doppelter
  Kombination), `260814-1830_*_gilt-das-ankreuzfeld-deep-…` (je Tab). Dazu zwei, die auf der
  Empfehlung fahren und eine Bestätigung brauchen: `260814-1552_*_…` (Rang der Filterzahl) und
  `260814-1830_*_an-welcher-stelle-…` (Reihenfolge von `Esc`).
- **Zwei haben ihre Voraussetzung verloren.** `260812-2002_*_…` spricht von „der einen Lücke"
  der Deckungszusage; daneben steht ein zweiter offener Befund derselben Zusage
  (`260812-1805_*_…`, YAML-Kopfzeilen). `260811-1230_*_…` beschreibt einen Fehler, der seit der
  Fragestellung zweimal wieder eingetreten ist; `crates/krk-ui/src/tabs.rs:819` trägt ihn heute
  („Rang 5 von 6", die Aufzählung hat sieben).
- **Die übrigen dreizehn sind unverändert offen.** Drei davon tragen im Datensatz bewusst keine
  Empfehlung.

## Geschrieben

- `260906-2102-neunzehn-offene-entscheidungsfragen-zur-vorlage.md`

Keine Befunde gefiltert: die zwei gefundenen Abweichungen sind Bestandteil der Vorlage und
werden mit der jeweiligen Antwort mitentschieden.
