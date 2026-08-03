Die Entscheidung zur `unsafe`-Grenze steht auf "beantwortet", obwohl der Commit sie umgesetzt hat

---

`decisions/260803-1208_a_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md` trägt
den Marker `_a_` und die leere Zeile `Implemented:`. Der Datensatz sagt selbst: "Erst
der Commit, der ihn bringt, zieht diesen Datensatz auf `implemented`." Dieser Commit
ist da.

---

**Beleg.** `569e8e0 feat(ui): S6 Fenster, Menue und echte Dateiliste` bringt
`crates/krk-ui/src/main.rs` von `#![warn(unsafe_code)]` auf `#![deny(unsafe_code)]`
und legt `crates/krk-ui/src/appkit/mod.rs` mit `#![allow(unsafe_code)]` als einziger
Ausnahme an. Am 260803-1309 nachgeprüft:

```
$ grep -n 'unsafe_code' crates/krk-ui/src/main.rs
1:#![deny(unsafe_code)]
$ grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]' crates/krk-ui/src
crates/krk-ui/src/appkit/mod.rs
```

Die Historie von S6, `history/260803-1244-fenster-menue-und-echte-dateiliste.md`,
belegt die Wirksamkeit zusätzlich: ein probeweise eingefügter `unsafe`-Block ließ den
Bau mit `error: usage of an unsafe block` scheitern.

**Was zu tun ist.** Im Datensatz die Zeile
`Implemented: 569e8e0 — krk-ui/src/main.rs traegt deny(unsafe_code), appkit/mod.rs die
einzige Ausnahme` ergänzen und die Datei von `_a_` auf `_i_` umbenennen. Der
Kopfabschnitt `**Status:**` zieht auf `implemented` mit.

**Warum das hier steht und nicht schon erledigt ist.** Der Befund ist bei der
Umsetzung von S7 aufgefallen, gehört aber zu S6. Der Auftrag an diesen `coder`
schreibt vor, einen Befund außerhalb der eigenen Dateien zu melden statt ihn nebenbei
zu ändern.

---
Resolved: Der Marker steht jetzt auf umgesetzt: `decisions/260803-1208_i_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md` mit `Implemented: 569e8e0`. Nachgezogen vom orchestrator am 260803-1315.
