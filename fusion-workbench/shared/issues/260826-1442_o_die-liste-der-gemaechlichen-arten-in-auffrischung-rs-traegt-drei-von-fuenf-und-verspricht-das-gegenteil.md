Die Liste der gemächlichen Arten in `auffrischung.rs` trägt drei von fünf und verspricht das Gegenteil
---
`die_gemaechlichen()` im Prüfmodul führt Kopieren, Verschieben und Papierkorb mit dem Kommentar „so fällt beim Hinzukommen einer vierten auf, dass sie hier fehlt“. Die Runde 17 hat Zippen und Entpacken hinzugefügt, und es ist nicht aufgefallen: die Liste ist `[Art; 3]`, die Probe darüber spricht von „allen vier Operationsarten“, `Art` hat sechs. Die zwei neuen Arten sind gegen `schiebt_auffrischung_auf` und `aufgeschobene_ordner` ungeprüft.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

- `crates/krk-ui/src/auffrischung.rs:796-808`: `fn die_gemaechlichen() -> [Art; 3]`.
- `crates/krk-ui/src/auffrischung.rs:810-812`: „geht sie für alle vier Operationsarten durch“.
- `crates/krk-core/src/operation/auftrag.rs`, `pub enum Art`: sechs Werte (Kopieren, Verschieben, InDenPapierkorb, UmbenennenImStapel, Zippen, Entpacken).
- `crates/krk-ui/src/auffrischung.rs:332-341`: `schiebt_auffrischung_auf` ist vollständig, ohne Auffangzweig — der Übersetzer hält diese Stelle. Was er nicht hält, ist die Probe, die behauptet, sie hielte es.

## Vorschlag

Die Liste zu `[Art; 5]` mit `Zippen { ziel }` und `Entpacken { ziele }` erweitern und den Kommentar auf die Zahl der Aufzählung verweisen statt auf eine Zahl.
