Das Sitzungsprotokoll der Runde 14 trägt weder Directive noch Turn-Log

---

`shared/history/260819-2026-orchestrator-session.md` ist die Sitzungsdatei, unter der die
gesamte Runde 14 gelaufen ist. Nach fünfzehn Commits und drei Turns steht in ihrem Kopf
weiterhin:

```
**Directive:** (not yet stated — Setup ran ahead of the user's request)
**Mode:** (not yet resolved)
**Status:** In progress
```

und ihr Abschnitt `## Per-Turn Log` besteht aus der einen Zeile `(no Turn has started)`.
Dasselbe gilt für den Circle-Datensatz: `circles/260819-2230-auswahl-und-kopieren-in-der-vorschau/_t_circle.md`
trägt die Überschrift `## Turn log` und darunter nichts.

Die Directive existiert und ist an anderer Stelle festgehalten — `fusion-workbench/agentstate.yaml`
führt sie im Feld `session.directive` („Aus der Vorschau lässt sich nichts kopieren: die Fläche
ist nicht auswählbar. Das soll gelöst werden."), und `agentstate.yaml` ist Sitzungszustand und
wird beim sauberen Ende gelöscht. Nach diesem Löschen sagt kein Datensatz mehr, unter welcher
Directive diese fünfzehn Commits entstanden sind.

---

**Gefilt von:** reconciler, Abgleich 260820-0834
**Gefunden:** beim Lesen des Sitzungsankers für den Kohärenz-Abgleich der Runde 14.
**Schwere:** niedrig im heutigen Schaden, mittel im Rückblick. Nichts ist kaputt, solange
`agentstate.yaml` steht. Der Verlust tritt beim Sitzungsende ein, und dann fällt er niemandem
auf, weil die Datei einfach fehlt.
**Baumstand:** `05cb614`.

## Warum das im gemeinsamen Speicher liegt

Der Defekt ist neben der Runde 14 aufgefallen und nicht von ihrer Directive verursacht: er
gehört zur Buchführung des Orchestrators über eine Sitzung und träte in jeder Sitzung dieses
Projekts genauso auf. Nach der Herkunftsregel gehört er deshalb hierher und nicht in den
Speicher des Circles.

## Ein zweiter, kleinerer Befund am selben Ort

Die Setup-Aufnahme derselben Datei sagt über die Vorgängerdatei
`shared/history/260819-2007-orchestrator-session.md`: „die Datei ist in dieser Sitzung erste
Staging-Liste, damit sie nicht länger außerhalb jedes Commits sitzt". Am 260820-0834 ist sie
weiterhin ungetrackt (`git status --short` führt sie als `??`), und die Sitzungsdatei der
laufenden Sitzung steht daneben, ebenfalls ungetrackt. Die Zusage aus der eigenen Aufnahme ist
nicht eingelöst.

## Mögliche Richtungen

Nicht entschieden, hier nur festgehalten:

- Der Orchestrator schreibt die Directive in den Kopf der Sitzungsdatei, sobald sie feststeht,
  im selben Vorgang, in dem er sie nach `agentstate.yaml` schreibt. Zwei Schreibvorgänge
  nebeneinander sind die Gestalt, die dieses Projekt schon mehrfach als übersprungen gemessen
  hat — siehe `shared/decisions/260814-1955_o_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`,
  denselben Riss an anderer Stelle.
- Der Turn-Eintrag entsteht beim `turn_start`-Ereignis und nicht am Sitzungsende. Die Ereignisse
  liegen ohnehin in `orchestrator-events.jsonl`; die Sitzungsdatei ist heute die einzige Stelle,
  an der sie in Prosa auftauchen sollen und nicht auftauchen.

---
Resolved: Nachgetragen am 260820-0840 vom Orchestrator, unmittelbar nach dem Abgleich, der den
Befund erhoben hat. Das Protokoll `shared/history/260819-2026-orchestrator-session.md` trägt jetzt
die Directive im Wortlaut des Nutzers mit Verweis auf den Spec, den aufgelösten Modus, eine
Budgettabelle, deren vier Datensatzzahlen am Dateibestand erhoben und nicht mitgezählt sind, einen
Turn-Log über alle drei Turns und einen Abschnitt „Was aussteht".

**Der Befund war berechtigt und trifft den Orchestrator selbst.** Das Protokoll ist bei Setup
angelegt worden, bevor der Nutzer sein Anliegen genannt hatte, und ist danach vierzehn Commits
lang nicht angefasst worden. Die Directive stand in dieser Zeit allein in `agentstate.yaml`, und
die wird beim Sitzungsende gelöscht — ein Abbruch hätte den Anlass der Runde nur noch aus den
Commit-Messages rekonstruierbar hinterlassen.

**Was daran nicht behoben ist:** die Ursache. Das Protokoll wird an keinem Turn-Ende gemessen, und
kein Mechanismus hätte den Rückstand gemeldet; gefunden hat ihn der Abgleich am Ende. Ob fusion
das prüfen sollte, gehört nicht in diesen Datensatz und ist hier nur benannt.
