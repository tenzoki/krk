One reflowed line in `umfang.rs` runs 113 characters in a block that wraps at 78

---

`926377f` rewrote the "Wer sie ruft" paragraph of `krk-core/src/verzeichnis/umfang.rs` and left
one line unwrapped. It is 113 characters of ordinary prose in a comment block whose other lines
end at 78.

---

**Severity:** Low, cosmetic. `cargo fmt --all --check` passes — rustfmt does not reflow doc
comments — so nothing catches it.
**Found by:** coderev, review `reviews/260818-0410-coderev-bundle-f-die-messungen-und-der-waechter.md`
**Affected:** `crates/krk-core/src/verzeichnis/umfang.rs:146`
**Tree state:** `a4d8211`
**Domain:** code

## What stands in the tree

```rust
// crates/krk-core/src/verzeichnis/umfang.rs:143-148
//! **vierten** Zweig seiner Stufenregel. Die Zaehlung faellt deshalb erst an,
//! wenn die beiden billigen Stufen jenes Rumpfes durch sind und das Blatt
//! wirklich erscheint; ein Befehl, den ein laufender Vorgang oder eine leere
//! Auswahl anhaelt, oeffnet hier kein Verzeichnis. `dead_code` traf das Modul auch vorher nicht, denn `krk-core`
//! ist eine Bibliothek und alles hier ist von ihrer Wurzel aus erreichbar; eine
//! Ausnahme nach dem Vorbild von `krk-ui/src/kommandos/rueckschritt.rs` brauchte
```

The over-long line is the seam where the new sentence was inserted before the sentence that was
already there.

**Not the same as the tree's other long lines.** Checked over every file this Circle's range
touched: the remaining lines above 100 characters are all a single unbreakable token — a record
path or a table row — where wrapping is not possible. This one is plain prose and wraps.

## Direction

Rewrap the paragraph at the file's prevailing 78 characters, exactly as `b0eee2c` did for
`resources/default-keymap.toml`.

---
Also seen: 260826-1221 by coderev — nachgemessen an HEAD 004ff72, `umfang.rs:146` steht unveraendert bei 113 Zeichen.
