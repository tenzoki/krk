# Lesefehler in einer SQLite-Datenbank sprechen weiter von einem Archiv

---
Der geschlossene Defekt `261023-2035-in-einer-sqlite-datenbank-sagt-die-statuszeile-weiter-archiv.md` hat die Sätze der Statuszeile und des Tabs dem Format folgen lassen. Die Gründe aus `Lesefehler::grund` tun es nicht. Kopiert der Nutzer eine Tabelle, während ein Schreiber übergibt, steht in der Abschlussliste „das Archiv hat sich seit dem Lesen seines Inhaltsverzeichnisses geändert“.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Low
**Cross-references:** 261023-2035-in-einer-sqlite-datenbank-sagt-die-statuszeile-weiter-archiv.md

## Befund

- `crates/krk-core/src/archiv/lesen.rs`, `Lesefehler::grund`: `Lesefehler::VonAussenVeraendert => text(Text::ArchivBeimLesenVeraendert)`, `Lesefehler::Fehlt => text(Text::ArchivEintragFehlt)` („der Eintrag steht nicht mehr an seiner Stelle im Archiv“).
- Beide entstehen im Datenbankweg: `Archivleser::datenbank_lesen` (Kennung vorher und nachher) und `archiv::sqlite::sicht_lesen` (`Lesefehler::Fehlt`).
- Weitergereicht über `Zulieferung::aus_archiv` (`operation/archiv.rs`, `andere => io::Error::other(andere.grund())`) und den Kopierweg in `operation/fern.rs`.

## Erwartet

Diese Gründe folgen dem Format des Lesers, wie `Archivformat::nur_lesbar_text` und die übrigen drei Fragen am Format.

## Abnahme

Eine Probe kopiert eine Tabelle aus einer Datenbank, deren Kennung zwischen Verzeichnis und Lesen wechselt, und findet in der Abschlussliste keinen Satz mit „Archiv“.
