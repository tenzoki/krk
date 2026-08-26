`iconutil` wird über den Suchpfad gerufen, während Kommentar und Meldung `/usr/bin/iconutil` sagen, und `messen.rs` liest `CARGO` ein zweites Mal
---
Zwei Prosastellen behaupten einen festen Pfad, den der Aufruf nicht nimmt; und `bundle::cargo()` hat einen dritten Leser, der es nicht ruft.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/bundle.rs`, `xtask/src/messen.rs`
**Verwandt:** `shared/decisions/260821-1221_o_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md` — offen, hier nicht entschieden.

## Befund

1. `bundle.rs:427` ruft `Command::new("iconutil")`. Der Doc-Kommentar `:85-86` sagt „liegt unter `/usr/bin/iconutil`, wie `codesign`", die Abbruchmeldung `:434-435` wiederholt es. `codesign` wird mit vollem Pfad gerufen (`sign.rs:234`), `iconutil` nicht; die zwei Sätze beschreiben eine Gewohnheit, die der Aufruf nicht teilt.
2. `bundle::cargo()` (`bundle.rs:278-280`) begründet sich mit „Beide inneren Aufrufe lesen ihn hier" (`:275-277`). `messen.rs:70` baut denselben Ausdruck `std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())` ein drittes Mal nach, statt die Funktion zu rufen.
3. Bestand der Suchpfad-Aufrufe am `c13bf1c`: `gh` (`veroeffentlichung.rs:59`, begründet), `iconutil` (`bundle.rs:427`), `rustup` (`release.rs:604`, mit begründetem Ausfallzweig), `cargo` über `CARGO` (`bundle.rs:279`, `messen.rs:70`).

## Abhilfe

Für 1 entweder den Aufruf auf `/usr/bin/iconutil` stellen oder die zwei Sätze streichen — welches, hängt an `260821-1221`. Für 2 `bundle::cargo()` rufen und den Kommentar auf „drei" oder auf die Regel stellen.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L4
