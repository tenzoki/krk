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

---
Resolved: `die_gemaechlichen()` in `crates/krk-ui/src/auffrischung.rs` gibt jetzt
`Vec<Art>` mit fünf Werten zurück (`Kopieren`, `Verschieben`, `InDenPapierkorb`,
`Zippen`, `Entpacken`). Die Zahl im Doc-Kommentar ist ersatzlos gefallen: an ihre
Stelle tritt die neue Zählprobe
`die_liste_der_gemaechlichen_deckt_jede_art_ausser_dem_stapel_umbenennen` in
demselben Prüfmodul. Sie liest die Varianten von `pub enum Art` aus
`crates/krk-core/src/operation/auftrag.rs` über `crate::quellbaum::quelldateien()`
und hält sie gegen die Liste plus `ein_umbenennen()`; datentragende Varianten zählt
sie mit, ihre Felder nicht. Der Satz "geht sie für alle vier Operationsarten durch"
über `allein_das_stapel_umbenennen_schiebt_die_auffrischung_auf` sagt jetzt "für jede
Operationsart" und verweist auf die Zählprobe.

Die Probe ist gegen ihren eigenen Zweck geprüft und nicht bloß grün gesehen: mit
entferntem `Art::Entpacken` in der Liste bricht sie mit
`diese Varianten von Art fasst keine Probe dieses Moduls an: ["Entpacken"]` ab.

Belege: `crates/krk-ui/src/auffrischung.rs`, `die_gemaechlichen`,
`varianten_von_art` und
`die_liste_der_gemaechlichen_deckt_jede_art_ausser_dem_stapel_umbenennen` im
`#[cfg(test)]`-Modul; `cargo test -p krk-ui auffrischung` → 29 bestanden.
