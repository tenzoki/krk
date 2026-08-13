Die Probe zur Versionszahl sagt „keine `.rs`-Datei des Baums" und liest nur `crates/`

---

`die_versionszahl_steht_in_keiner_quelldatei` (`crates/krk-ui/src/appkit/titelzusatz.rs:298-311`) nimmt C1.2 ab. Ihr Doc-Kommentar sagt: „Die Versionszahl steht in keiner `.rs`-Datei des Baums als Zeichenkette." Gelesen wird über `quellbaum::quelldateien()`, und das liest `crates/` und sonst nichts. `xtask/src/release.rs` führt die Zahl viermal als Zeichenkette und liegt außerhalb.

---

**Schwere:** niedrig. Kein falsches Verhalten, eine zu weit gefasste Zusage an einer Probe.

**Beleg**

`crates/krk-ui/src/quellbaum.rs:96-100` — die Wurzel ist `crates/`:

```rust
let wurzel = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("crates/krk-ui liegt eine Ebene unter crates/")
    .to_path_buf();
```

`xtask/src/release.rs:947-970` — vier Konstanten mit der Zahl:

```rust
const TAG_PASST: &str = "v0.1.0\n";
const TAG_UNTER_MEHREREN: &str = "release-2026-08\nv0.1.0\nvorletzter-stand\n";
const TAG_AEHNLICH: &str = "v0.1.0-rc1\nv0.1.10\n";
```

dazu die Aufrufe `stand_pruefen("0.1.0", …)` in den sieben Vergleichsproben.

**Warum das kein Fehler an `release.rs` ist.** Die Konstanten sind wörtliche Git-Ausgaben als Prüfstoff, und die Funktion bekommt die Sollversion daneben als Argument (`"0.1.0"`), nicht aus `bundle::VERSION`. Sie veralten also nicht, wenn die Zahl in der `Cargo.toml` steigt — der grüne Fall bleibt grün. Genau diese Trennung war der Zweck der reinen Funktion aus C3.14.

**Was falsch ist, ist der Satz an der Probe.** Er sagt „des Baums" und meint `crates/`. Der Modulkopf von `quellbaum.rs` verlangt in seiner dritten Folgerung ausdrücklich, „die verbleibende Blindheit am Doc-Kommentar der Probe zu benennen"; die Probe benennt eine andere Blindheit (eine gleiche Ziffernfolge in einer Messtafel) und nicht diese. Dieselbe Verwechslung von „im Baum" mit „in dieser einen Kiste" ist am 260813-0540 schon einmal als Defekt aufgetreten und hat `quelldateien` von `krk-ui/src` auf `crates/` gezogen.

**Was zu tun ist**

Den Doc-Kommentar der Probe auf ihre wirkliche Reichweite bringen: „in keiner `.rs`-Datei unter `crates/`", dazu ein Satz, dass `xtask/` außerhalb liegt und dort wörtliche Git-Ausgaben mit der Zahl als Prüfstoff stehen, die nicht angezeigt werden. Die Probe selbst zu erweitern ist der falsche Zug: sie würde an den Prüfstoff-Konstanten rot, ohne dass an C1.2 etwas faul wäre.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1 der Runde 8, Bereich `59b0a6c..21dbc59`.
- C1.2 selbst ist erfüllt: `titelzusatz::beschriftung` setzt die Zahl über `concat!("KRK ", env!("CARGO_PKG_VERSION"))` zusammen, und in `crates/` steht sie nirgends als Zeichenkette.

---

**Abgleich 260813-1345: zu Recht offen, unverändert.** Der Doc-Kommentar bei
`crates/krk-ui/src/appkit/titelzusatz.rs:298-311` sagt weiter „des Baums"; `quellbaum.rs:120-124`
liest weiter `crates/`. C1.2 selbst hält: unter `crates/` steht die Zahl nirgends wörtlich.

**Der Befund reicht weiter, als dieser Datensatz ihn fasst.** Zwei weitere Stellen in derselben
Datei tragen dieselbe Formulierung, und eine davon ist nicht bloß zu weit gefasst, sondern
falsch: `titelzusatz.rs:130` sagt „Die einzige Stelle im Baum, die Name und Version
zusammensetzt", und vier Stellen in `crates/krk-bench/` setzen einen Namen mit derselben Version
zusammen. Dazu kommt eine fünfte wörtliche Fundstelle der Zahl ausserhalb `crates/`, die dieser
Datensatz nicht aufzählt: `xtask/src/bundle.rs:587`. Abgelegt als
`260813-1345_o_zwei-weitere-stellen-sagen-im-baum-und-meinen-crates-eine-davon-ist-widerlegt.md`;
beide gehören in einen Zug behoben, weil es dieselbe Datei und dieselbe Ursache ist.
