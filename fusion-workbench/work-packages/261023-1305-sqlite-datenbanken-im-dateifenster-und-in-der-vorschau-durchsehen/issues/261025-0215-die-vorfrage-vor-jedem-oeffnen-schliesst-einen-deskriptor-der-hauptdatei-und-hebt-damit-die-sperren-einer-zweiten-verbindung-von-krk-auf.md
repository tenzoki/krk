# Die Vorfrage vor jedem Öffnen schließt einen Deskriptor der Hauptdatei und hebt damit die Sperren einer zweiten Verbindung von KRK auf

---
`archiv::sqlite::oeffnen` öffnet und schließt vor **jedem** Öffnen einer Datenbank einen eigenen Deskriptor auf die Hauptdatei. Nach POSIX hebt dieses Schließen jede `fcntl`-Sperre auf, die der Prozess auf der Datei hält, auch die einer anderen, gerade lesenden SQLite-Verbindung von KRK. Der neue Doc-Kommentar an `sofort_unveraenderlich` nennt die Regel, der Modulkopf nennt die Folge nicht.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Low
**Cross-references:** 261025-0030-die-vorfrage-zum-leeren-wal-liest-neben-einem-laufenden-exklusiven-schreiber-ohne-sperre-und-niemand-sagt-es.md, 261024-2215-wie-liest-krk-eine-wal-datenbank-mit-wal-und-ohne-shm.md

## Befund

- `crates/krk-core/src/archiv/sqlite.rs:238`, `oeffnen`: als erstes steht `if sofort_unveraenderlich(datei)`. Das gilt für jede Datenbank, nicht nur für eine mit WAL-Kopf.
- `crates/krk-core/src/archiv/sqlite.rs:302`, `sofort_unveraenderlich`: `let Ok(mut offen) = ohne_warten_oeffnen(datei) else { … }`. Der Deskriptor geht am Ende der Funktion zu. Er wird geöffnet, **bevor** der Kopf gelesen ist, also auch bei einer Datenbank im Rollback-Betrieb.
- `crates/krk-core/src/archiv/sqlite.rs:316`, `wal_kopf`, und `:334`, `ruhende_wal_datenbank_ohne_wal`: in der Rückfallstufe noch einmal Öffnen und Schließen.
- Der Doc-Kommentar über `sofort_unveraenderlich` (Zeile 297 ff.) und der an `verzeichnis::sys::fremde_sperre` (`crates/krk-core/src/verzeichnis/sys.rs:1206`) nennen die Regel selbst: „POSIX hebt beim Schliessen jedes Deskriptors auf eine Datei alle `fcntl`-Sperren auf, die der Prozess auf ihr haelt, auch die einer anderen Verbindung desselben Prozesses“. Die Behebung von 0030 verhindert ein **zweites** Schließen. Das erste bleibt, und es stand schon vor `4777a1cb` da (`wal_kopf` in `oeffnen` seit `a1fec2b7`).
- Der Modulkopf `# Nur lesen` (Zeile 119 ff.) nennt den Restwettlauf mit einem fremden Schreiber und den Restfall mit der gelöschten `-shm`. Diesen Fall nennt er nicht.

Erreichbar ist das so (Inferenz aus dem Quelltext, nicht gemessen): Die Vorschau liest eine Tabelle über `Archivleser::datenbank_lesen` → `oeffnen` (`mode=ro`, Rollback-Betrieb, die Verbindung hält `SHARED`). Zugleich liest das Dateifenster nach einer FSEvents-Meldung das Inhaltsverzeichnis derselben Datei neu, über denselben Weg `oeffnen`. Dessen `sofort_unveraenderlich` schließt seinen Deskriptor und hebt das `SHARED` der Vorschau auf. Ein fremder Schreiber bekommt dann `EXCLUSIVE` und schreibt mitten in das Lesen. Ebenso umgekehrt. SQLite beschreibt genau dieses Muster unter `howtocorrupt.html`, Abschnitt 2.2.

Die Folgen sind klein. Die Datenbank bleibt heil, denn KRK schreibt nie. Das Lesen einer Tabelle sichert die Kennungsprüfung vor und nach dem Lesen ab (`VonAussenVeraendert`, seit `4777a1cb` ein vorübergehender Fehler mit erneutem Versuch). Ungesichert ist das Inhaltsverzeichnis: Es kann einmal falsche Zeilenzahlen oder `SQLITE_CORRUPT` zeigen, bis die nächste Auffrischung es neu liest.

## Erwartet

Eines von beiden:
1. Der Modulkopf von `archiv/sqlite.rs` (`# Nur lesen`) nennt den Fall als dritte Restklasse neben Restwettlauf und Restfall, mit der Folge für das Inhaltsverzeichnis.
2. Oder: Kein Deskriptor auf die Hauptdatei wird außerhalb von SQLite geschlossen, solange eine andere Verbindung von KRK auf derselben Datei offen sein kann. Das ginge zum Beispiel über einen prozessweiten Riegel je Pfad um Vorfrage und Öffnen, oder über einen Deskriptor, der so lange offen bleibt wie die Verbindung.

## Abnahme

Bei 1: Der Modulkopf trägt den Absatz, und der Doc-Kommentar an `sofort_unveraenderlich` verweist darauf. Bei 2: Eine Probe in `crates/krk-core/tests/archiv_sqlite.rs` hält eine `mode=ro`-Verbindung offen, ruft danach `oeffnen` für dieselbe Datei auf und prüft mit `fremde_sperre` aus einem Kindprozess (oder mit `/usr/bin/sqlite3 … BEGIN EXCLUSIVE`, das scheitern muss), dass die Sperre der ersten Verbindung noch steht.
