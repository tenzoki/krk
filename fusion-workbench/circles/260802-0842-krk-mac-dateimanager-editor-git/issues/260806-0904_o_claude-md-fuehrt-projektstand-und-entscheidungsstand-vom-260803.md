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
