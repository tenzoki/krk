`README.md` und Hilfetext beschreiben Station 2 als „keine `use objc2`-Zeile", der Code fängt seit dem 260806 auch den ausgeschriebenen Pfad
---
Die Stationstabelle und der Hilfetext nennen die Hälfte der Prüfung.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `README.md`, `xtask/src/main.rs`, `xtask/src/release.rs`

## Befund

`README.md:256`: „keine `use objc2`-Zeile außerhalb von `crates/krk-ui/src/appkit/`". `main.rs:77-78`: „prueft die AppKit-Grenze (keine `use objc2`-Zeile ausserhalb von …)". `verletzt_grenze` (`release.rs:492-498`) fragt `ist_objc2_use` **oder** `nennt_objc2_pfad`; der Modulkopf `:39-41` und die Abbruchmeldung `:386-388` sagen es richtig („weder als `use`-Zeile noch als ausgeschriebener Pfad"). Zwei Stellen sind stehengeblieben.

## Abhilfe

Beide Sätze um „noch als ausgeschriebener Pfad" ergänzen.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L5
