# Schuldet das Paket zu den Markdown-Tabellen einen Abnahmelauf gegen die zehn Zusagen aus C8?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261031-1720-plan-markdown-vorschau-richtet-tabellenspalten-aus.md, 261017-0955-schuldet-das-sftp-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261018-0822-schuldet-das-csv-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md, 261023-1340-schuldet-das-sqlite-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md

---

## Question

Das Paket berührt eine gemessene Strecke, L7, die Vorschau des ausgewählten Eintrags. Für eine Markdown-Datei läuft `markdown::rendern` auf dem Faden `krk-vorschau` künftig mit `ENABLE_TABLES`. Enthält die Datei eine Tabelle, legt AppKit sie auf dem Hauptfaden als `NSTextTable` aus. Prüfordner A führt `.md`-Dateien (`crates/krk-bench/src/fixture.rs`, Liste `ENDUNGEN`). Ob deren Inhalt Tabellen bildet, ist nicht geprüft (`inference:` eher nicht, weil der Erzeuger keine Markdown-Struktur schreibt). Die übrigen neun Zusagen berühren keine geänderte Stelle.

## Options

1. **Messen.** Der Nutzer fährt den Abnahmelauf am Bündel im Vordergrund.
2. **Verzichten.** Dieses Paket fährt keinen Lauf. Die zehn Zusagen stehen seit dem 261017 ohnehin auf den Gegenständen der späteren Messrunde und bleiben dort.

## Constraints

- Kein Agent kann den Lauf fahren, er verlangt KRK im Vordergrund.
- Die Antwort hält keinen Bauschritt an, nur das Schließen des Pakets.

## Recommendation

Möglichkeit 2. Auf dem Faden kommt eine lineare Zerlegung hinzu, deren Kosten der Plan in Schritt 2 kopflos nachmisst, gegen die 19 bis 30 ms je MB aus der Wurzel-`Cargo.toml`. Die Auslegung einer großen Tabelle auf dem Hauptfaden prüft der Nutzer beim Ansehen am Bündel mit der großen Abnahmedatei aus dem Plan (`## Testing Strategy`), ohne Messstrecke. Das entspricht dem Vorgehen beim sftp-, CSV- und SQLite-Paket.

**Answered:** 261031 by user: „1“ auf die Vorlage mit A1, B1 (kein Messlauf, wie beim sftp-, CSV- und SQLite-Paket) und C1. Möglichkeit 2 dieses Datensatzes: das Paket verzichtet auf einen Abnahmelauf gegen die zehn Zusagen; die Auslegung einer großen Tabelle prüft der Nutzer am Bündel.
