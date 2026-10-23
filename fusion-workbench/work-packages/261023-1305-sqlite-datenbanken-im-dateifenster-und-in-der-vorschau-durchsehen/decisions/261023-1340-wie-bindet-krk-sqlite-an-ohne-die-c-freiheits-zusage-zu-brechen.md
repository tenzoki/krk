# Wie bindet KRK SQLite an, ohne die C-Freiheits-Zusage zu brechen?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, 261023-1305-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau-durchsehen.md

---

## Question

`rusqlite` bringt `libsqlite3-sys` und bricht die Zusage, dass auf den Mac-Zielen weder `cc` noch ein Paket mit einem Namen auf `-sys` in den Baum kommt. Welcher Weg wird gebaut?

## Options

1. (a) System-`libsqlite3` von Hand gebunden, Bindungen in `krk-core/src/verzeichnis/sys.rs`; keine neue Ausnahme von `deny(unsafe_code)`, fremder C-Code im Prozess.
2. (a′) dieselbe Bindung in einer dritten Ausnahmedatei; die Probe `genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code` wird geändert.
3. (c) Prozess `/usr/bin/sqlite3`; kein `unsafe`, aber Ausgabe ohne Zusage zu deuten.
4. (b2) eigener Leser des Dateiformats; keine Sichten, keine Sperrabsprache, Aufwand large.

Folgen je Möglichkeit: Analysebericht 261023-1324-sqlite-datenbanken-durchsehen-machbarkeit-und-zuschnitt.md, Abschnitt `## Open Questions`.

## Wortlaut des Nutzers (261023, Chat)

> F1:1, F2: alle direkt, F3:TSV, F4:sql listen, F5:zeilenzahl, F6:ich checke selbst, F7:ja

---
Answered: 261023-1340-wie-bindet-krk-sqlite-an-ohne-die-c-freiheits-zusage-zu-brechen.md — Möglichkeit 1 (a): System-libsqlite3 von Hand in verzeichnis/sys.rs; ruled by user, Kai Stalmann <kai@stalmann.org>
