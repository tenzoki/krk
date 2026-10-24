# `sqlite_uri` schreibt ein Nullbyte als `%00`, und SQLite kürzt dort den Pfad, statt abzuweisen

---
Der Kommentar in `sqlite_uri` sagt, die Bibliothek weise `%00` beim Öffnen ab. Sie tut es nicht: sie liest den Pfad bis vor `%00` und öffnet diese Datei. Der Nachbar `c_pfad` in derselben Datei weist einen Pfad mit Nullbyte mit `Text::LesenPfadMitNullbyte` ab.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Low
**Cross-references:** 261023-1340-wie-bindet-krk-sqlite-an-ohne-die-c-freiheits-zusage-zu-brechen.md

## Befund

- `crates/krk-core/src/verzeichnis/sys.rs`, `sqlite_uri`, am Ende: „ein Nullbyte im Pfad steht als `%00` da, das die Bibliothek beim Oeffnen abweist.“
- Nachgemessen am 261024: `/usr/bin/sqlite3 -readonly 'file://<ordner>/x.db%00garbage?mode=ro' 'SELECT count(*) FROM t;'` liefert die Zeilen von `x.db`, Rückgabe 0.
- Aus dem Dateisystem kommt kein solcher Pfad. `Path` in Rust kann ein Nullbyte trotzdem tragen, und die Funktion ist `pub`.

## Erwartet

`sqlite_uri` (oder `Datenbank::oeffnen` über einen eigenen Weg) weist einen Pfad mit Nullbyte ab wie `c_pfad`, statt ihn zu kodieren. Der Kommentar stimmt danach.

## Abnahme

Eine Probe in `crates/krk-core/tests/sqlite_bindung.rs` reicht einen Pfad `…/x.db\0rest` herein und bekommt einen Fehler statt der Zeilen von `x.db`.

**Resolved:** 261024, code-implementer. `sqlite_uri` gibt `io::Result<CString>` zurück und weist einen Pfad mit Nullbyte mit `Text::LesenPfadMitNullbyte` ab wie `als_c_pfad` (gemeinsamer `nullbyte_fehler`); `archiv::sqlite::nur_lesend_oeffnen` reicht die Abweisung als `SQLITE_MISUSE` weiter, das keine zweite Öffnung mit `immutable=1` auslöst. Kommentar korrigiert. Probe `ein_pfad_mit_nullbyte_wird_abgewiesen_statt_gekuerzt_geoeffnet` (`tests/sqlite_bindung.rs`).
