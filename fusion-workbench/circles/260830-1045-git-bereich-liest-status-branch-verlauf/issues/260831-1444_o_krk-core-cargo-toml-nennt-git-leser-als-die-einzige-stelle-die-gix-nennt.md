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

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23.
