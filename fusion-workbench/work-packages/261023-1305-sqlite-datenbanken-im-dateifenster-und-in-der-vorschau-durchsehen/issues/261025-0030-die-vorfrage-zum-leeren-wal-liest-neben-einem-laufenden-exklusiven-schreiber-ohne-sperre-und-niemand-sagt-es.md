# Die Vorfrage zum leeren `-wal` liest neben einem laufenden exklusiven Schreiber ohne Sperre, und Modulkopf, `HowTo.md` und Entscheid sagen es nicht

---
Ein Programm im Sperrmodus `exclusive` hält während des Laufs ein leeres `-wal` und keine `-shm`. Seit `a1fec2b7` öffnet KRK genau diese Lage sofort mit `immutable=1`, also an der Sperre des laufenden Programms vorbei. Vorher kam „die Datenbank wird gerade geschrieben“. Der Modulkopf sagt, `immutable=1` gelte nur dort, wo „es nichts Neueres zu lesen gibt“, und `HowTo.md` sagt weiter, eine länger gesperrte Datenbank melde die Statuszeile.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer (nach Nutzerentscheid zum Wortlaut oder zur Vorfrage)
**Severity:** Medium
**Cross-references:** 261024-2215-wie-liest-krk-eine-wal-datenbank-mit-wal-und-ohne-shm.md, 261024-2133-shm-neben-wal-ohne-shm-zusage-korrigieren-oder-anlegen-verhindern.md, 261024-2112-mode-ro-legt-neben-einer-wal-datenbank-mit-wal-und-ohne-shm-eine-neue-shm-an.md

## Befund

- `crates/krk-core/src/archiv/sqlite.rs`, `oeffnen`: `if wal_kopf(datei) && leeres_wal_ohne_shm(datei) { return nur_lesend_oeffnen(datei, true, abbruch); }`. Eine Frage, ob ein anderes Programm die Datei sperrt, steht davor nicht.
- Nachgemessen am 261025 mit `/usr/bin/sqlite3` in einem Wegwerfordner. Ein Schreiber mit `journal_mode=wal`, `locking_mode=exclusive` und `wal_checkpoint(TRUNCATE)` bleibt offen. Der Ordner zeigt dann `x.db` und `x.db-wal` mit 0 Bytes, keine `-shm`, und der Kopf trägt in den Bytes 18 und 19 je `2`. Dann `file://…?mode=ro` → `database is locked (5)`; `file://…?mode=ro&immutable=1` → liest `3`. Nach einem weiteren `INSERT` des Schreibers hat `-wal` 4152 Bytes. Nach dessen Ende steht wieder ein leeres `-wal`. KRK kann die Lage „läuft“ von „geschlossen“ an Größe und Begleitdateien also nicht unterscheiden.
- Die Analyse `261024-2133-…`, Abschnitt 2 (Zeile `X`), hat den laufenden exklusiven Schreiber nur mit **nicht leerem** `-wal` gefahren. Für Weg B (`immutable`) steht dort „nicht gemessen (die heutige Regel wählt B hier nie, weil `-wal` steht)“. Das gilt seit Möglichkeit 4 nicht mehr.
- Der Entscheid `261024-2215-…` verwirft Möglichkeit 2 unter `## Abwägung` gerade wegen dieses Bildes („Firefox startet, während KRK liest … die Vorfrage sieht ein später startendes Programm nicht“). Möglichkeit 4 trägt dasselbe Risiko und hat dazu keine Vorfrage für ein schon laufendes Programm. Das wägt der Entscheid nicht.
- Modulkopf `# Nur lesen`: „`immutable=1` bleibt den zwei Faellen der Oeffnungsregel vorbehalten, in denen es nichts Neueres zu lesen gibt“. Neben einem laufenden Schreiber stimmt das nur im Augenblick der Vorfrage.
- `HowTo.md` (Abschnitt „Was beim Lesen an der Datei geschieht“): „hält das andere Programm die Datenbank länger gesperrt, sagt die Statuszeile „die Datenbank wird gerade geschrieben““. Für diesen Fall stimmt das nicht mehr.

Abgemildert, nicht geschlossen: `Archivleser::datenbank_lesen` (`archiv/lesen.rs`) fragt die Kennung aus Hauptdatei und `-wal` vor und nach dem Lesen einer Tabelle. Ein Checkpoint während des Lesens endet deshalb vermutlich als `VonAussenVeraendert` und nicht als falsche TSV (inference). Das Inhaltsverzeichnis (`verzeichnis_lesen`, Zeilenzahlen) wird nicht nachgeprüft. Ein Checkpoint mitten darin kann laut `uri.html` zu `immutable` falsche Zahlen oder `SQLITE_CORRUPT` geben (inference, nicht gemessen). Die Datenbank selbst bleibt heil.

## Erwartet

Eine Nutzerentscheidung zwischen zwei Folgen:
1. Der Wortlaut wird ehrlich. Modulkopf (`# Die Oeffnungsregel`, `# Nur lesen`), `HowTo.md` und ein Nachtrag im Entscheid 2215 sagen: Neben einem laufenden Programm im Modus `exclusive` liest KRK ohne Sperre den Stand der Hauptdatei. Was der Schreiber danach übergibt, erscheint erst nach der nächsten Auffrischung, die dann „wird gerade geschrieben“ meldet. Ein Checkpoint während des Lesens kann die Anzeige einmal stören.
2. Vor der Vorfrage steht ein `F_GETLK` auf den Sperrbereich von SQLite (Analyse 2133, Abschnitt 2, gemessen). Bei Sperre geht es zu `mode=ro` und damit zu „wird gerade geschrieben“, wie vor `a1fec2b7`. Das braucht eine Bindung in `verzeichnis/sys.rs`.

## Abnahme

Eine Probe in `crates/krk-core/tests/archiv_sqlite.rs` hält einen Schreiber im Modus `exclusive` mit leerem `-wal` offen und liest über `Archivzugang::verzeichnis`. Bei Folge 1 prüft sie, dass gelesen wird und keine Datei entsteht, und der Wortlaut nennt den Fall. Bei Folge 2 prüft sie `Verzeichnisfehler::Belegt` und dass keine Datei entsteht.
