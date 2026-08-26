`beglaubigen` prüft die Universalität nicht, obwohl die Signaturanzeige sie mitliefert
---
Der Nur-Beglaubigungsweg spricht vom „universellen" Bündel, prüft aber nur Developer-ID und `runtime`. Die Zeile `Format=… Mach-O universal (x86_64 arm64)` steht in derselben Ausgabe.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/beglaubigung.rs`

## Befund

`signaturstand_pruefen` (`beglaubigung.rs:231-268`) stellt zwei Fragen an die Ausgabe von `codesign --display --verbose=2`. Dieselbe Ausgabe trägt die Zeile `Format=app bundle with Mach-O universal (x86_64 arm64)` (`:431`, `:456`, `:480` in den aufgezeichneten Proben). Die Universalität wird nicht gefragt. `README.md:302-303` und der Modulkopf `:12-14` beschreiben das Bündel als universell.

Ein Bündel aus `cargo xtask bundle` mit `KRK_SIGN_IDENTITY=<Developer-ID>` ist nicht universell (`sign.rs:175-180`); mit `codesign --options runtime` von Hand nachsigniert bestünde es beide Fragen und ginge bei Apple ein. Konstruiert, aber billig.

## Abhilfe

Eine dritte Frage `traegt_beide_architekturen` nach dem Muster von `traegt_gehaertete_laufzeitumgebung` (`:299-306`), gegen `release::ARCHITEKTUREN`.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L3
