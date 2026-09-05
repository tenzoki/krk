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

---
Abgleich 260905-2254 (coder, Baumstand `8779a25`): **zur Hälfte erledigt, bleibt offen.** Die Stationstabelle in `README.md` nennt jetzt beide Hälften der Prüfung: „`objc2` steht außerhalb von `crates/krk-ui/src/appkit/` nirgends, weder als `use`-Zeile noch als ausgeschriebener Pfad". Der Hilfetext steht weiter auf der halben Aussage — `xtask/src/main.rs:85`, „prueft die AppKit-Grenze (keine `use objc2`-Zeile ausserhalb von …)" —, und `xtask/` lag außerhalb der Grenze dieses Durchgangs.

---
Resolved: Der Hilfetext nennt jetzt beide Hälften der Prüfung. `xtask/src/main.rs`, Abschnitt
`cargo xtask release`: „prueft die AppKit-Grenze (objc2 steht ausserhalb von
crates/krk-ui/src/appkit/ nirgends, weder als use-Zeile noch als ausgeschriebener Pfad)".

Damit sagen alle vier Stellen dasselbe: der Modulkopf von `xtask/src/release.rs` (Station 2), die
Abbruchmeldung in `verletzt_grenze`, die Stationstabelle der `README.md` (Durchgang 260905) und
der Hilfetext. Der Wortlaut ist der der Stationstabelle, damit sie nicht auseinanderlaufen.

Geprüft: `cargo test -p xtask -p krk-bench` (Exit 0), `cargo clippy -p xtask -p krk-bench
--all-targets -- -D warnings` (Exit 0). Baumstand `ba0c6bd`.
