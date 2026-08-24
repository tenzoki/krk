Ein Doc-Verweis in `tests/ablage.rs` nennt einen Probennamen, den die Umbenennung weggenommen hat

---

`crates/krk-core/tests/ablage.rs:2973` verweist auf

```
/// [`jede_toml_datei_mit_ladeweg_wird_bei_beschaedigung_zur_seite_gelegt`].
```

Die Probe heißt `jede_toml_datei_wird_bei_beschaedigung_zur_seite_gelegt`
(`crates/krk-core/tests/ablage.rs:1145`), ohne `_mit_ladeweg_`. Der Verweis zeigt damit auf
nichts. Der zweite Verweis auf dieselbe Probe (`crates/krk-core/tests/ablage.rs:2052`) trägt
den richtigen Namen.

Entstanden mit `4516f4e`, das die Probe von `jede_der_vier_dateien_wird_bei_beschaedigung_…`
umbenannt hat: die eine Fundstelle ist offenbar auf einen Zwischennamen gezogen worden, den
die endgültige Umbenennung nicht mehr trägt.

Nachgezählt am 260824-1650 über alle Doc-Verweise der in diesem Bereich geänderten Dateien; es
ist der einzige, der im Baum nirgends auflöst.

---

**Warum es nicht von selbst auffällt.** Ein Testziel wird von `cargo doc` nicht dokumentiert,
also meldet nichts im Bauablauf einen kaputten `intra_doc_link`. `cargo build`, `cargo test`,
`cargo clippy --all-targets` und `cargo fmt --check` laufen am 260824-1640 sämtlich grün.

**Was zu tun ist.** `_mit_ladeweg_` streichen.

**Schwere:** niedrig.

**Gefunden:** coderev, bei der Durchsicht der Bündel C, D und E am 260824-1650.

**Betroffen:** `crates/krk-core/tests/ablage.rs` (Zeile 2973)

**Domain:** code

---
Resolved:
