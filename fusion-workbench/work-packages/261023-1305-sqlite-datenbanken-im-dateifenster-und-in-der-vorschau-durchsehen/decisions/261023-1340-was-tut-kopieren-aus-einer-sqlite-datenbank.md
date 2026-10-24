# Was tut Kopieren aus einer SQLite-Datenbank?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, 261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md

---

## Question

Ein allein lesbares Archiv lässt das Kopieren heute zu. Was geschieht mit einer Tabelle als Quelle?

## Options

1. Ausgrauen; braucht eine neue Unterscheidung in `Ortsart` oder `Ortsbedarf`.
2. Export: die Tabelle wird beim Kopieren eine TSV-Datei, deren Name eine Endung bekommt, die der Eintrag nicht trägt.

Folgen je Möglichkeit: Analysebericht 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, Abschnitt `## Open Questions`.

## Wortlaut des Nutzers (261023, Chat)

> F1:1, F2: alle direkt, F3:TSV, F4:sql listen, F5:zeilenzahl, F6:ich checke selbst, F7:ja

---
Answered: 261023-1340-was-tut-kopieren-aus-einer-sqlite-datenbank.md — Möglichkeit 2: Kopieren exportiert die Tabelle als TSV-Datei; ruled by user, Kai Stalmann <kai@stalmann.org>

Implemented: e5d2c387 — Kopieren legt <name>.tsv bzw. <name>.sql an.
