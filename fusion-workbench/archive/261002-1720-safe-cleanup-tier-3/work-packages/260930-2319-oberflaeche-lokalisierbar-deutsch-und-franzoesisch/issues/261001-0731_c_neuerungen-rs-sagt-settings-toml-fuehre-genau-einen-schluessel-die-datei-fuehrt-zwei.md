`neuerungen.rs` sagt, `settings.toml` führe „genau einen“ obersten Schlüssel; die Auslieferungsfassung führt zwei
---
Der Doc-Kommentar an `Vergleichsform::ObersteSchluessel` in `crates/krk-core/src/ablage/neuerungen.rs:146` lautet „Die Form von `settings.toml`, die heute genau einen fuehrt: `terminal`“. `resources/default-settings.toml` führt `terminal` (Zeile 65) und `notizordner` (Zeile 99), und `Einstellungen` in `ablage/einstellungen.rs` trägt beide Felder. Die Zahl ist mit dem Notizordner falsch geworden und wird mit dem Sprachschlüssel der Lokalisierung ein zweites Mal falsch.
---
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
Gefunden bei der Bestandserhebung für den Spec zur Lokalisierung der Oberfläche, Stand `e984b3f`. Abnahme: der Kommentar nennt keine Zahl mehr, sondern verweist auf `resources/default-settings.toml` als Quelle, oder er zählt die Schlüssel mit einem Befehl wie `grep -E '^[a-z_]+ =' resources/default-settings.toml`.
---
Resolved: Schritt 4 des Plans `261001-0850_*_plan-oberflaeche-folgt-der-systemsprache-deutsch-franzoesisch-englisch.md`. Der Doc-Kommentar an `Vergleichsform::ObersteSchluessel` in `crates/krk-core/src/ablage/neuerungen.rs` nennt keine Zahl mehr und zeigt auf `resources/default-settings.toml` als Quelle, mit dem Zaehlkommando `grep -E '^[a-z_]+ =' resources/default-settings.toml`.
