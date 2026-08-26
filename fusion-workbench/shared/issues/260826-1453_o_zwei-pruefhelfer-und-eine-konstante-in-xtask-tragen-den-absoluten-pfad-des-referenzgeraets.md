Zwei Prüfhelfer und eine Konstante in `xtask` tragen den absoluten Pfad des Referenzgeräts
---
`buendel()` in zwei Prüfmodulen liefert `/Users/k1/Projects/productive/krk/target/KRK.app`, obwohl `bundle::wurzel()` daneben liegt.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/beglaubigung.rs`, `xtask/src/veroeffentlichung.rs`

## Befund

`beglaubigung.rs:493-495` und `veroeffentlichung.rs:689-691` bauen denselben Helfer `buendel()` mit dem absoluten Pfad des Referenzgeräts. Beide dienen allein dem Meldungstext, die Proben laufen deshalb überall grün. `beglaubigung.rs:429` trägt den Pfad in der aufgezeichneten `codesign`-Ausgabe `AUSGELIEFERT`; dort ist er Datum und bleibt.

`bundle::buendelpfad(&bundle::wurzel())` (`bundle.rs:290-292`) liefert denselben Wert gerätunabhängig und ist die eine Stelle, die `target/KRK.app` zusammensetzt — der Helfer ist die zweite.

## Abhilfe

Beide Helfer auf `bundle::buendelpfad(&bundle::wurzel())` stellen; einer davon genügt, wenn er `pub(crate)` in einem Prüfmodul steht.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L9
