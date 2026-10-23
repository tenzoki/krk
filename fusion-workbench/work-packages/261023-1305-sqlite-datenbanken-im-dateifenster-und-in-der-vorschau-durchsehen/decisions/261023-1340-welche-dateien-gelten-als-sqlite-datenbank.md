# Welche Dateien gelten als SQLite-Datenbank?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, 261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md

---

## Question

Woran erkennt der Einstieg mit Pfeil rechts eine Datenbank?

## Options

1. Allein `.sqlite`, `.sqlite3`, `.db3`.
2. Dazu `.db` direkt; eine `.db`, die keine SQLite-Datei ist, meldet beim Betreten „keine Datenbank“.
3. `.db` erst nach Prüfung der ersten 16 Bytes auf einem Arbeitsfaden.

Folgen je Möglichkeit: Analysebericht 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, Abschnitt `## Open Questions`.

## Wortlaut des Nutzers (261023, Chat)

> F1:1, F2: alle direkt, F3:TSV, F4:sql listen, F5:zeilenzahl, F6:ich checke selbst, F7:ja

---
Answered: 261023-1340-welche-dateien-gelten-als-sqlite-datenbank.md — Möglichkeit 2: alle Endungen direkt, `.db` eingeschlossen, ohne Vorabprüfung der Bytes; ruled by user, Kai Stalmann <kai@stalmann.org>
