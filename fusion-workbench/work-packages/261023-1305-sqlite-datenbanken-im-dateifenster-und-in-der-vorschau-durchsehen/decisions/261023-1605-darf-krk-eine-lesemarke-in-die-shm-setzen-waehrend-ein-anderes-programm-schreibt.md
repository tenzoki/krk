# Darf KRK eine Lesemarke in die `-shm` setzen, während ein anderes Programm in die Datenbank schreibt?

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md (`## Open Questions`), messungen/261023-1556-sqlite-nur-lesend-shm.txt

---

## Question

Schritt 1 des Plans hat gemessen: eine nur lesende Verbindung (`mode=ro`) lässt Hauptdatei und `-wal` byte-gleich und fährt beim Schließen keinen Checkpoint, setzt aber neben einem laufenden WAL-Schreiber beim ersten Lesen eine Lesemarke in die gemeinsame `-shm`; nach einem abgebrochenen Schreiber baut sie deren Index neu. Eine neue Datei legt sie nie an.

## Options

1. **Ja.** Die Zusage lautet: KRK schreibt weder die Datenbank noch ihr `-wal`; liest es, während ein anderes Programm schreibt, setzt SQLite eine Lesemarke in die gemeinsame `-shm`. Folge: die Anzeige zeigt den aktuellen Stand.
2. **Nein, immer `immutable=1`.** Folge: bei laufendem Schreiber veraltete Daten (gemessen: drei statt vier Zeilen).

## Recommendation

Möglichkeit 1 (implementation-planner).

## Antwort des Nutzers (261023)

> 1

---
Answered: Möglichkeit 1 — KRK darf die Lesemarke in die `-shm` setzen; geöffnet wird `mode=ro`, `immutable=1` allein ohne `-wal` neben einer WAL-Datenbank; ruled by user, Kai Stalmann <kai@stalmann.org>

Nachtrag 261024: Die Prämisse „Eine neue Datei legt sie nie an“ gilt nicht für eine WAL-Datenbank mit `-wal` und ohne `-shm` (Issue `261024-2112-mode-ro-legt-neben-einer-wal-datenbank-mit-wal-und-ohne-shm-eine-neue-shm-an.md`). Nach dem Entscheid `261024-2215-wie-liest-krk-eine-wal-datenbank-mit-wal-und-ohne-shm.md` (Möglichkeit 4, Rest nach Möglichkeit 1) gilt seitdem: ein leeres `-wal` ohne `-shm` liest KRK unveränderlich und legt nichts an; ein nicht leeres `-wal` ohne `-shm` (Absturz, Kopie) lässt SQLite eine `-shm` anlegen. `immutable=1` gilt damit in zwei Fällen, ohne `-wal` und bei leerem `-wal` ohne `-shm`; die Antwort oben bleibt im Übrigen stehen. Umgesetzt in `crates/krk-core/src/archiv/sqlite.rs` (`oeffnen`, Modulkopf `# Die Oeffnungsregel` und `# Nur lesen`), code-implementer.
