# Was zeigt die Spalte „Größe“ einer Tabelle im Dateifenster?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, 261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md

---

## Question

Eine Tabelle hat keine Dateigröße. Was steht in der Spalte?

## Options

1. Leer; keine Kosten.
2. Bytes über `dbstat`; beim Betreten wird die ganze Datei einmal gelesen.
3. Zeilenzahl; eine Abtastung je Tabelle und eine Zahl in einer Spalte, die sonst Bytes zeigt.

Folgen je Möglichkeit: Analysebericht 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, Abschnitt `## Open Questions`.

## Wortlaut des Nutzers (261023, Chat)

> F1:1, F2: alle direkt, F3:TSV, F4:sql listen, F5:zeilenzahl, F6:ich checke selbst, F7:ja

---
Answered: 261023-1340-was-zeigt-die-spalte-groesse-einer-tabelle.md — Möglichkeit 3: die Zeilenzahl der Tabelle; ruled by user, Kai Stalmann <kai@stalmann.org>
