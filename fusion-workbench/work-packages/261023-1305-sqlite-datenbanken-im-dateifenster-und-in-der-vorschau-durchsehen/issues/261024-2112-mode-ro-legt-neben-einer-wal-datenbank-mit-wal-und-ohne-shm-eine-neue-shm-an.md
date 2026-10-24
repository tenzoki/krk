# `mode=ro` legt neben einer WAL-Datenbank mit `-wal` und ohne `-shm` eine neue `-shm` an

---
`archiv/sqlite.rs` sagt zu, KRK lege neben einer Datenbank keine Datei an, und der Entscheid `261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md` ruht auf der Prämisse „Eine neue Datei legt sie nie an“. Für eine WAL-Datenbank, neben der `-wal` steht und `-shm` fehlt, stimmt das nicht: die nur lesende Verbindung legt `-shm` an.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer (nach Nutzerentscheid)
**Severity:** High
**Cross-references:** 261023-1605-darf-krk-eine-lesemarke-in-die-shm-setzen-waehrend-ein-anderes-programm-schreibt.md, 261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md, messungen/261023-1556-sqlite-nur-lesend-shm.txt

## Befund

- `crates/krk-core/src/archiv/sqlite.rs`, Modulkopf `# Nur lesen`: „es legt neben der Datenbank keine Datei an, weder `-wal` noch `-shm` noch `-journal`“.
- `oeffnen` (dieselbe Datei) wählt `immutable=1` allein bei WAL-Kopf **und fehlendem** `-wal`. Steht `-wal` und fehlt `-shm`, öffnet es `mode=ro`.
- Die Messung `messungen/261023-1556-sqlite-nur-lesend-shm.txt` fährt die Fälle b, c (beide Begleitdateien stehen) und d (beide fehlen). Der Fall „`-wal` steht, `-shm` fehlt“ ist nicht gemessen.

Nachgemessen am 261024 mit `/usr/bin/sqlite3 -readonly 'file://<pfad>?mode=ro'` (dieselben Kennzeichen `SQLITE_OPEN_READONLY | SQLITE_OPEN_URI`, dieselbe Systembibliothek) in einem Wegwerfordner: WAL-Datenbank, Schreiber mit `kill -9` beendet, `-shm` entfernt, dann gelesen. Vorher `x.db x.db-wal`, nachher `x.db x.db-shm x.db-wal`; Hauptdatei und `-wal` byte-gleich (`shasum`). Ursache nach dem Quelltext von SQLite (inference): `unixOpenSharedMemory` öffnet `-shm` mit `O_RDWR|O_CREAT`, solange `readonly_shm` nicht gesetzt ist.

Die Lage entsteht etwa bei einer Kopie von Datenbank und `-wal` ohne `-shm` oder bei einem Programm mit beständigem `-wal`.

## Erwartet

Eine Nutzerentscheidung, dann eine der zwei Folgen:
1. Die Zusage wird ehrlich: Modulkopf, `HowTo.md` und der Entscheid nennen das Anlegen von `-shm` in diesem Fall, und eine Probe hält es fest.
2. KRK legt auch hier nichts an (etwa abweisen mit Meldung, oder `readonly_shm=1` in der URI prüfen und messen, was dann geschieht), und eine Probe hält, dass nach dem Lesen keine neue Datei neben der Datenbank steht.

## Abnahme

Eine Probe in `crates/krk-core/tests/archiv_sqlite.rs` legt WAL-Datenbank plus `-wal` ohne `-shm` an, liest über `Archivzugang::verzeichnis` und eine Tabelle, und prüft den Ordnerstand nach der gewählten Folge.
