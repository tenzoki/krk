`KRK_SIGN_IDENTITY=-` signiert ad hoc, und keine Stelle sagt es
---
Die erste Suchstufe nimmt jeden nichtleeren Wert; `-` ist für `codesign --sign` die Ad-hoc-Identität. Modulkopf und Hilfetext formulieren die Regel „nicht ad hoc" absolut.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/sign.rs`, `xtask/src/main.rs`

## Befund

`aus_umgebung` (`sign.rs:254-261`) trimmt den Wert und prüft auf nichtleer; `-` kommt durch, und `signieren_mit` reicht es an `codesign --force --sign -` (`:234-238`). Das ist die Ad-hoc-Signatur, die der Modulkopf `:6-10` als das ausschließt, was das Modul nie tut, und die der Hilfetext `main.rs:40-41` mit „weicht nicht auf eine Ad-hoc-Signatur aus" beschreibt. Der Weitergabehinweis sagte danach: „signiert ist dieses Buendel mit \"-\", und dieser Name ist nicht der einer Developer-ID" (`:194-198`).

## Was nicht behauptet wird

Kein stilles Ausweichen: der Wert kommt vom Nutzer. Die Regel ist für die drei Suchstufen wahr; nur ihr Wortlaut deckt die Umgebungsvariable mit ab.

## Abhilfe

`-` in `aus_umgebung` benennend abweisen („`-` ist die Ad-hoc-Identität, und KRK wird nicht ad hoc signiert") oder den Satz in `sign.rs:6-10` und `main.rs:40-41` auf die Suche einschränken.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L2

---
Resolved: Der Strich wird benennend abgewiesen, die erste der zwei Moeglichkeiten. `aus_umgebung`
(`xtask/src/sign.rs`) liefert jetzt `Result<Option<String>, Abbruch>`; die reine Haelfte `aus_wert`
weist den Wert `-` (Konstante `ADHOC`) mit einer Anleitung ab, statt ihn als Namen durchzureichen.
Ein `None` waere die falsche Antwort gewesen: die Suche liefe weiter und naehme eine Identitaet,
die der Nutzer nicht gemeint hat. Der Satz in `sign.rs` und der Hilfetext in `main.rs` bleiben
absolut und sind damit wahr; der Modulkopf sagt die Regel fuer die ausdrueckliche Angabe eigens.
Gehalten von `der_strich_signiert_nicht_ad_hoc` und
`ein_leerer_wert_geht_weiter_und_ein_name_wird_genommen`. Die reine Haelfte ist getrennt, weil
`std::env::set_var` in dieser Ausgabe von Rust `unsafe` ist und die Kiste seit heute
`#![deny(unsafe_code)]` traegt.
