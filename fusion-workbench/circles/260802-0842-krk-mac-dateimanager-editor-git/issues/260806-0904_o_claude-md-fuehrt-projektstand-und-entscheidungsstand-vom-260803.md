`CLAUDE.md` führt Projektstand und Entscheidungsstand vom 260803

---

`CLAUDE.md` trägt in `## Projektstand` und `## Bindende Grundlage` den Stand vom 260803-1321 und weicht damit an vier Stellen vom Dateibestand ab:

- "8 der 24 Schritte tragen `[DONE]`, als nächstes steht S8" — tatsächlich sind 35 von 36 Schritten abgenommen, offen allein S6b.
- "Drei Defekte sind offen ..., alle drei betreffen Schritt 7" — tatsächlich sind 17 Defekte offen, darunter 5 aus dem Coderev `reviews/260806-0834-coderev-turn-21-s19-bis-s23.md`.
- "Beantwortet oder umgesetzt sind ... sieben Fragen" — tatsächlich stehen 23 Datensätze auf `_i_` und 1 auf `_a_` (Stand 260806-0904, nach dem Abgleich).
- "**Offen** sind fünf Fragen" mit namentlicher Liste — tatsächlich sind 10 offen; die Liste fehlt u. a. `decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`, die die Rundenschließung hält.

Die Datei erklärt selbst den Dateibestand für verbindlich, aber eine Aufstellung, die um Größenordnungen daneben liegt, schickt jeden neuen Sitzungsstart in die falsche Richtung. Der Satz zur Sortierfrage ("bindet Schritt S12") ist zusätzlich überholt, S12 ist längst abgenommen.

---

## Warum es zählt

`CLAUDE.md` ist die erste Datei, die jede Sitzung liest. Ein Projektstand, der 27 abgenommene Schritte unterschlägt, kostet jeden Agenten einen eigenen Abgleich, bevor er dem Dokument wieder trauen kann.

**Zuständig:** die CLAUDE.md-Revision am Sitzungsende (`/fusion:cleanup` bzw. `/fusion:revise-claude-md`); kein Codeeingriff.

**Aufgefallen bei:** dem Reconciler-Abgleich 260806-0904 nach Turn 21.

---

**Abgleich 260806-1647 nach Turn 23 — der Defekt bleibt offen, und seine eigenen Zahlen sind inzwischen selbst überholt.** Zwei der vier gemeldeten Stellen haben sich seither bewegt, zwei nicht:

- `## Projektstand` ist am 260806-0014 nachgezogen worden (Commit `e8626b6`) und trägt jetzt den Stand jenes Zeitpunkts. Zeile 40 sagt "34 der 36 Schritte tragen dort `[DONE]`, offen sind S6b … und S23"; **beide sind seither abgenommen**, S23 am 260806-0813 (`d577295`) und S6b in diesem Turn (`194ea16`). Es sind 36 von 36.
- `## Bindende Grundlage` steht unverändert auf dem 260803-1321. Zeile 71 sagt "sieben Fragen" beantwortet oder umgesetzt — ausgezählt am Dateibestand sind es **25** (`_i_`: 23 im Circle, 2 in `shared/`), und **kein** Datensatz steht mehr auf `_a_`. Zeile 73 sagt "fünf Fragen" offen — es sind **11** (8 im Circle, 3 in `shared/`), unter ihnen die beiden neuen dieses Turns, `260806-1303` (Vordergrund für den Abnahmelauf) und die weiterhin rundenschließende `260806-0014` (L9).
- Zeile 81, "Die Sortierfrage bindet Schritt S12", ist unverändert und im Präsens falsch. Der Punkt ist als eigener Defekt weitergeführt, weil er nicht nur `CLAUDE.md` betrifft: `issues/260806-1647_*_die-sortierfrage-bindet-s12-und-steht-in-keiner-planstelle.md`.
- Die Zahl der offenen Defekte ist von 17 auf 8 gefallen.
