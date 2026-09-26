HowTo.md nennt den Handgriff „beiseitelegen und neu starten“ auch für keymap.toml, die KRK nie anlegt
---
`HowTo.md`, Abschnitt über die Neuerungen an den eigenen Dateien, sagt nach der Tabelle mit `readers.toml`, `settings.toml` und `keymap.toml`: „Der Handgriff ist bei allen dreien derselbe: … KRK starten. Sie entsteht neu aus der Auslieferungsfassung“. Für `keymap.toml` stimmt das nicht: KRK legt sie nie an (`anlegen_falls_fehlt` steht allein in `ablage/einstellungen.rs` und `ablage/leseprofile.rs`), ihr Weg auf den Auslieferungsstand führt über `cmd+r` in der F1-Ansicht, und eine einzelne neue Funktion wird über „Zuweisen“ belegt. `README.md` unter `## Neuerungen an den eigenen Dateien übernehmen` und `CLAUDE.md` sagen es richtig.
---
**Filed by:** orchestrator, Kai Stalmann <kai@stalmann.org>

Gefunden beim Nachziehen der Texte für `appointments.md` (Arbeitspaket `260926-2240-termine-als-weitere-datei-im-heimordner`, Schritt 10), aber älter als diese Arbeit und nicht von ihr verursacht.

Abnahme: Der Absatz in `HowTo.md` beschreibt den Handgriff für `readers.toml` und `settings.toml` und für `keymap.toml` getrennt, und der Teil zu `keymap.toml` stimmt mit `README.md` überein; die Wendung „die alte … löschen“ bleibt an ihren Stellen unberührt.

---
Resolved: `HowTo.md` beschreibt den Handgriff nach der Tabelle der drei Dateien jetzt getrennt: „beiseitelegen und neu starten“ für `readers.toml` und `settings.toml`, für `keymap.toml` wie in `README.md` unter „Neuerungen an den eigenen Dateien übernehmen“ der Weg über **Auslieferungszustand** (`cmd+r`) in der F1-Ansicht, der Hinweis, dass ohne Datei nichts zu tun ist, und das Belegen einer einzelnen neuen Funktion über **Zuweisen** (`cmd+t`). Die Erhebung `grep -rnE --exclude-dir=fusion-workbench --exclude-dir=target '[Dd]ie alte.{0,24}löschen' .` liefert dieselben vier Zeilen wie vorher. Commit folgt mit diesem Datensatz (docs(howto)).
