# Betritt der Doppelklick eine SQLite-Datenbank, und öffnet return sie weiter mit dem Standardprogramm?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, 261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md

---

## Question

Zur Bestätigung des Verhaltens aus dem Archivpaket.

## Options

1. Ja: Doppelklick betritt wie bei Archiven, `return` bleibt „Mit dem Standardprogramm öffnen“.
2. Anders (vom Nutzer zu nennen).

Folgen je Möglichkeit: Analysebericht 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, Abschnitt `## Open Questions`.

## Wortlaut des Nutzers (261023, Chat)

> F1:1, F2: alle direkt, F3:TSV, F4:sql listen, F5:zeilenzahl, F6:ich checke selbst, F7:ja

---
Answered: 261023-1340-betritt-der-doppelklick-eine-sqlite-datenbank-und-oeffnet-return-sie-weiter-extern.md — Möglichkeit 1 bestätigt; ruled by user, Kai Stalmann <kai@stalmann.org>

Implemented: 148f84dd — Archivformat::Sqlite, Betreten wie ein Archiv (Pfeil rechts, Doppelklick), return weiter extern.
