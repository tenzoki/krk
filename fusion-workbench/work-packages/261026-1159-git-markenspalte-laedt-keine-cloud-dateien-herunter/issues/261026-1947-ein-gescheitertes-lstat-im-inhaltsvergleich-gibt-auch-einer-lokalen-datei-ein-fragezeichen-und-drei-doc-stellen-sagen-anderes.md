# Ein gescheitertes `lstat` im Inhaltsvergleich gibt auch einer lokalen Datei ein `?`, und drei Doc-Stellen sagen anderes

---
`Inhaltsvergleich::compare_blobs` gibt `Befund::Ungelesen` zurück, wenn sein eigenes `lstat` scheitert, bevor feststeht, ob die Datei ausgelagert ist. `ausgelagert_deuten` tut dasselbe, wenn `Metadata::from_path_no_follow` oder `Stat::from_fs` scheitern. Der Lauf liefert dann `?` für diese eine Datei und nicht `None` für alle. Der Plan hat das so gewollt („Ein Fehlschlag des eigenen `lstat` ergibt `Ungelesen`“), aber drei Doc-Stellen beschreiben es anders.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `261026-1208-plan-git-markenspalte-laedt-keine-cloud-dateien-herunter.md`, `261026-1947-reviewer-abschlussdurchsicht-git-markenspalte-cloud-dateien.md`

## Defekt

Die Stellen im Code, `crates/krk-core/src/git/leser.rs`:

```rust
let Ok(angaben) = std::fs::symlink_metadata(&pfad) else {
    return Ok(Some(Befund::Ungelesen));
};
```

(in `impl CompareBlobs for Inhaltsvergleich`, vor der Platzhalterfrage) und die zwei `else { return Some(Befund::Ungelesen) }` in `ausgelagert_deuten`.

Was die Doku dagegen sagt:

1. Doc-Kommentar von `Gitleser::marken`, Absatz „`None` heisst unentschieden. Jeder Fehlschlag fuehrt dorthin …“: ein Fehlschlag des zweiten `lstat` führt nicht zu `None`.
2. Doc-Kommentar von `Marke::Unbekannt` (`crates/krk-core/src/git/mod.rs`): „Der Inhalt der Datei liegt nur in der Cloud und ist nicht gelesen“. Die Marke entsteht auch für eine lokale Datei, deren zweites `lstat` scheitert, etwa weil sie zwischen dem `lstat` von `gix` und dem des Vergleichs gelöscht wurde. `gix` hätte sie als `Change::Removed` gemeldet, also „M“.
3. Doc-Kommentar von `Befund::Ungelesen`: „Die Datei liegt nur in der Cloud“. Gleicher Fall.

`HowTo.md` beschreibt `?` allein für Cloud-Dateien. Für den Nutzer reicht das. Die Codedoku ist der Vertrag, den der nächste Umbau liest, und sie muss stimmen.

## Richtung

Entweder die drei Doc-Stellen nennen den zweiten Ursprung von `?` (die Angaben ließen sich nicht erneut lesen), oder der Fehlschlag vor der Platzhalterfrage führt wie jeder andere Fehlschlag zu `None`. Die erste Richtung ändert kein Verhalten.

## Abnahme

- Die drei genannten Doc-Kommentare beschreiben den Fall „`lstat` des Vergleichs scheitert“ so, wie der Code ihn behandelt.
- `RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps` grün.
