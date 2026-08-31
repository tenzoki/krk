`krk-core/Cargo.toml` nennt `git::leser` als die einzige Stelle, die `gix` nennt

---
Der Kommentar am neuen Eintrag (`crates/krk-core/Cargo.toml:38-45`) sagt:

> `git::leser` haelt das `Repository` und ist die einzige Stelle, die die Kiste nennt.

Drei Dateien nennen sie im Code, nicht eine:

- `crates/krk-core/src/git/leser.rs` — 37 Vorkommen von `gix::`
- `crates/krk-core/src/git/mod.rs:64` — `pub use gix::ObjectId;`, und das ist keine Nebensache: es ist die Wiederausfuhr, über die `krk-ui` den Objektnamen hält, ohne `gix` selbst zu führen
- `crates/krk-core/src/git/texte.rs:181` — `gix::hash::Kind::Sha1` im Prüfmodul

Gemessen mit `grep -rn 'gix::' crates/krk-core/src`.

Der Satz trägt eine Begründung, die auch mit dem richtigen Bestand steht — der Leser hält das `Repository`, und deshalb wohnt die Abhängigkeit im Kern —, aber die Aussage über die eine Stelle ist unrichtig, und der nächste Leser, der `gix` aus dem Kern lösen will, sucht an der falschen Zahl von Stellen.

**Abnahmetest:** der Kommentar nennt die Stellen oder die Erhebung und keine Zahl, die eine Wiederausfuhr übergeht.

**Resolved:** 260831. Der Kommentar am `gix`-Eintrag in `crates/krk-core/Cargo.toml` nennt die Stellen und die Erhebung statt einer Zahl. `git::leser` trägt „fast jede Nennung"; ausdrücklich daneben stehen die Wiederausfuhr `pub use gix::ObjectId` in `git::mod` — mit der Auskunft, dass `krk-ui` über sie den Objektnamen jedes Commits hält, ohne `gix` selbst zu führen — und `gix::hash::Kind::Sha1` im Prüfmodul von `git::texte`. Wer die Kiste aus dem Kern lösen will, wird auf `grep -rn 'gix::' crates/krk-core/src` verwiesen. **Keine Zahl steht im Kommentar**, und das ist nicht Vorsicht, sondern gemessen: der Datensatz nennt 37 Vorkommen in `leser.rs`, dieselbe Erhebung liefert am 260831 achtundvierzig, und die Zahl der Dateien ist seit der Aufnahme des Befunds von drei auf vier gestiegen, weil `git::lauf` die Kiste inzwischen in einem Kommentar nennt (im Code nennt sie sie nicht). Eine dazugehörige Stelle ist mitgezogen: der Doc-Kommentar der Wiederausfuhr in `git/mod.rs` begründete sie damit, dass die Oberfläche sich den letzten angezeigten Commit merke, um beim Nachladen dort weiterzumachen — das gilt seit der Umstellung von `WeitererVerlauf { ab }` auf `{ bereits }` nicht mehr, und der Absatz nennt jetzt den wirklichen Grund (`Commit` trägt den Namen als Feld) samt dem Datum, an dem der alte wegfiel.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23.
