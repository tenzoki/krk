`CLAUDE.md` nennt drei Kisten mit `#![deny(unsafe_code)]`, seit heute tragen es vier
---
`xtask` hat die Zeile bekommen; der Satz in `CLAUDE.md` zaehlt weiter `krk-core`, `krk-ui` und `krk-bench` auf und uebergeht sie.
---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `CLAUDE.md`, Abschnitt „Projektstand", letzter Absatz vor „Bauen und pruefen"
**Cross-references:** `260826-1450_*_xtask-traegt-kein-deny-unsafe-code.md` (geschlossen; die zweite Haelfte seiner Abhilfe steht hier)

## Befund

`CLAUDE.md` sagt: „`krk-core`, `krk-ui` und `krk-bench` tragen `#![deny(unsafe_code)]` an ihrer
Kistenwurzel; die Ausnahme `#![allow(unsafe_code)]` steht nur in `krk-core/src/verzeichnis/sys.rs`
und `krk-ui/src/appkit/mod.rs`. Der Bau erzwingt diese Grenze."

`xtask/src/main.rs` traegt die Zeile seit dem 260905. Die Aufzaehlung ist damit unvollstaendig,
und der Satz daneben liest sich als Grenze, die fuer das Bauwerkzeug nicht gilt — gerade fuer die
Kiste, die neun fremde Programme ruft und in den Arbeitsbaum schreibt.

Der geschlossene Befund `260826-1450` nennt beide Haelften der Abhilfe: die Zeile in `main.rs` und
„den `CLAUDE.md`-Satz auf ‚vier Kisten' oder auf die Regel stellen". Die erste ist getan, die
zweite lag ausserhalb der Grenze jenes Durchgangs.

## Abhilfe

Den Satz auf die Regel stellen statt auf eine Aufzaehlung, die mit der naechsten Kiste wieder
falsch wird: jede Kiste des Workspace traegt `#![deny(unsafe_code)]` an ihrer Wurzel, und die
Ausnahmen stehen namentlich. Gezaehlt wird mit
`grep -rn '#!\[deny(unsafe_code)\]' crates/*/src/*.rs xtask/src/main.rs`.

## Abnahme

Der Satz in `CLAUDE.md` nennt keine Kistenliste mehr, oder er nennt `xtask` mit.

---
Resolved: `CLAUDE.md`, Abschnitt „Projektstand", letzter Absatz vor „Bauen und prüfen": die Kistenliste ist gefallen und durch die Regel samt Zählkommando ersetzt („Jedes Mitglied des Workspace trägt `#![deny(unsafe_code)]` an seiner Kistenwurzel, `xtask` seit dem 260905 als letztes … `grep -rn '^#!\[deny(unsafe_code)\]' crates/*/src/*.rs xtask/src/*.rs`"). Am Baum `8779a25` liefert das Kommando vier Zeilen, eine je Eintrag unter `members` der Wurzel-`Cargo.toml`. Der Satz zu den zwei `#![allow(unsafe_code)]`-Ausnahmen nennt jetzt die Probe, die sie namentlich hält (`genau_zwei_dateien_oeffnen_die_regel_deny_unsafe_code`, `crates/krk-core/tests/baum.rs`), und dazu ihre Reichweite: sie liest allein `crates/` und sähe eine Ausnahme in `xtask` nicht.
