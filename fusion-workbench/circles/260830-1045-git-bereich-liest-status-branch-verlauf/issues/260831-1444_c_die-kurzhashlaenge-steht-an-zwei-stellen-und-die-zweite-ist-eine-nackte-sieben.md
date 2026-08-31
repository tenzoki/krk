Die Kurzhashlänge steht an zwei Stellen, und die zweite ist eine nackte Sieben

---
`crates/krk-core/src/git/leser.rs:91` führt `const KURZHASHLAENGE: usize = 7;` mit der ausgeschriebenen Begründung, warum die Länge fest ist und nicht aus `ObjectId::shorten()` kommt; `kurzhash` (`leser.rs:436`) liest sie.

`texte::verlaufszeile` schreibt denselben Kurzhash ein zweites Mal und nennt die Zahl nackt (`crates/krk-core/src/git/texte.rs:113`):

```rust
let kurzhash: String = commit.id.to_hex_with_len(7).to_string();
```

Die Konstante ist privat und für `texte.rs` nicht erreichbar. Wer die Länge ändert, ändert den Kurzhash der Kopfzeile bei abgelöstem HEAD und lässt den der Verlaufsliste stehen; die Probe `die_verlaufszeile_traegt_vier_angaben_in_dieser_reihenfolge` (`texte.rs:254`) prüft gegen dieselbe hingeschriebene Sieben und würde grün bleiben.

**Abnahmetest:** `grep -n 'to_hex_with_len' crates/krk-core/src/git/` nennt genau eine Zahl, und beide Schreiber lesen sie.

**Resolved:** 260831, mit der einzigen Codeänderung dieses Zuges. `KURZHASHLAENGE` steht jetzt in `crates/krk-core/src/git/mod.rs` — bei `Commit`, `Kopf`, `Marke` und der `ObjectId`-Wiederausfuhr, aus denen beide Schwestermodule ohnehin lesen — als `pub(crate) const`, und beide Schreiber lesen sie: `leser::kurzhash` über `use super::{Commit, KURZHASHLAENGE, Kopf, Marke}` und `texte::verlaufszeile` über dieselbe Zeile. Die nackte Sieben in `texte.rs:113` ist weg. Die Probe `die_verlaufszeile_traegt_vier_angaben_in_dieser_reihenfolge` prüft gegen die Konstante statt gegen ihre eigene hingeschriebene Sieben und wird damit rot, wenn die Länge sich ändert und ein Schreiber nicht mitgeht. **Der Weg ist die Verschiebung in den gemeinsamen Elter und nicht das Öffnen der Konstante in `leser`**: eine Abhängigkeit `texte` → `leser` entstünde sonst für eine Zahl, und `texte` hängt heute an keinem der beiden Schwestermodule. Die ausgeschriebene Begründung ist mitgewandert und um den Absatz erweitert, warum die Zahl nicht bei einem der beiden Schreiber wohnt. `grep -n 'to_hex_with_len' crates/krk-core/src/git/` nennt zwei Stellen und an keiner eine Zahl; beide nennen `KURZHASHLAENGE`.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23.
