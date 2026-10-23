# In einer SQLite-Datenbank sagt die Statuszeile weiter „Archiv“

---
**Domain:** code
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Cross-references:** 261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md (Schritt 3)

---

## Befund

Schritt 3 hat die Texte „allein lesbar“ und „hier nicht verfügbar“ je Archivformat gemacht (`Archivformat::nur_lesbar_text`, `nicht_verfuegbar_text`), aber nur in den Dateien seiner Liste. Zwei Stellen außerhalb davon sprechen in einer Datenbank weiter von einem Archiv:

1. `Text::ArchivAlleinLesbar` („Dieses Archiv ist allein lesbar.“) wird in `crates/krk-ui/src/appkit/anwendung.rs` an drei Stellen gerufen (um die Zeilen 7473, 7797, 9074, Stand nach Schritt 3) und fragt das Format nicht.
2. Eine `.db`, die keine Datenbank ist, meldet sich über den vorhandenen Satz `ArchivUngueltig`: „… ist kein lesbares Archiv im Format sqlite: keine SQLite-Datenbank“.

## Erwartet

In einer SQLite-Datenbank nennen beide Meldungen die Datenbank, wie es die Melder im Kern seit Schritt 3 tun.

## Herkunft

Bericht des Code-Implementers zu Schritt 3, Abweichung 6.
