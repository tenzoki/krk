CLAUDE.md sagt, die Umlautprobe lese die Terminalausgaben mit; sie liest `xtask` und `krk-bench` nicht
---
`CLAUDE.md:245` (Abschnitt `## Sprache`) schließt, nach dem Verweis auf die offene Frage `260907-0826_*_gilt-die-umlautregel-auch-fuer-die-terminalausgabe-von-xtask-krk-bench-und-messmodus.md`, mit:

```
die Terminalausgaben tragen die Umschrift, und die Umlautprobe liest sie mit, ohne dort etwas zu finden.
```

Die offene Frage nennt drei Terminalausgaben: `xtask`, `krk-bench` und den Messmodus. Die Umlautprobe `kein_stringliteral_des_betriebscodes_traegt_einen_umlaut_ausser_in_der_sprachtabelle` (`crates/krk-core/tests/baum.rs:1599`) liest über `betriebscode_der_oberflaeche` (`:1522-1531`) allein Dateien, deren Name mit `krk-core/src/` oder `krk-ui/src/` beginnt. Den Messmodus (`crates/krk-ui/src/messmodus.rs`) liest sie also mit; `xtask/src/` und `crates/krk-bench/src/` liest sie nicht. „liest sie mit“ stimmt damit für eine der drei.

Abnahme: der Satz in `CLAUDE.md` nennt, welche Terminalausgaben die Probe liest (den Messmodus in `krk-ui`) und welche nicht (`xtask`, `krk-bench`), oder er fällt; die Probe bleibt, wie sie ist, denn ihr Schnitt folgt dem Spec (`## Out of Scope`).
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
Gefunden bei der Abschlussdurchsicht `261001-1929-reviewer-lokalisierung-abschluss.md` über `b81a284..e46a678`; Arbeitspaket `260930-2319-oberflaeche-lokalisierbar-deutsch-und-franzoesisch`. Schwere: Low.
---
Resolved: 261001 — Der Satz in `CLAUDE.md` (Abschnitt `## Sprache`) nennt jetzt, dass die Umlautprobe von den drei Terminalausgaben der offenen Frage allein den Messmodus mitliest, weil er in `krk-ui/src/` steht, und `xtask/` und `crates/krk-bench/` nicht. Die Probe ist unverändert. Nicht committet.
