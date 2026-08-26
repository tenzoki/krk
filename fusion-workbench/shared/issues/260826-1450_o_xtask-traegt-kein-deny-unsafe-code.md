`xtask` trägt kein `#![deny(unsafe_code)]`
---
Die drei `krk-*`-Kisten erzwingen die Grenze an ihrer Wurzel; das Bauwerkzeug nicht, obwohl es heute keine `unsafe`-Stelle hat und die Zeile nichts kostet.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/main.rs`
**Verwandt:** `shared/issues/260826-1302_o_die-probenziele-des-kerns-tragen-kein-deny-unsafe-code-und-eines-fuehrt-fuenf-unsafe-stellen.md`

## Befund

`grep -rn '#!\[' xtask/src` trifft nichts; `grep -rn unsafe xtask/src` trifft allein den Doc-Kommentar `release.rs:43`. `CLAUDE.md` nennt die Grenze für `krk-core`, `krk-ui` und `krk-bench` und schweigt zu `xtask`. Das Werkzeug ruft neun fremde Programme und schreibt in den Arbeitsbaum; eine `unsafe`-Stelle, die jemand beim Nächsten einbaute, fiele hier nicht auf.

## Abhilfe

`#![deny(unsafe_code)]` an `main.rs:1`, und den `CLAUDE.md`-Satz auf „vier Kisten" oder auf die Regel stellen.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L6
