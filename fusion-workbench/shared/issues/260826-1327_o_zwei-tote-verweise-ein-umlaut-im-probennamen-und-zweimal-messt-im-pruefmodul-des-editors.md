Zwei tote Verweise, ein Umlaut im Probennamen und zweimal "messt" im Pruefmodul des Editors

---

Zwei Doc-Kommentare verweisen auf eine Probe
`die_sieben_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`, die es nicht gibt — sie heisst
`die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`. Eine Probe traegt als einzige im Baum einen
Umlaut im Bezeichner, und zwei Ueberschriften schreiben "messt".

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/editor.rs:4255` und `:4586`: Verweis
  `[`die_sieben_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus`]`; die Probe heisst seit dem
  Umbau auf neun `die_abgeschalteten_stehen_an_der_gebauten_flaeche_auf_aus` (`:4896`). `:5235` verweist
  richtig. `cargo doc` meldet den toten Verweis; `make check` faehrt `cargo doc` nicht.
- `:3425`: `fn die_drei_verfehlten_zeilensprünge_tragen_drei_verschiedene_saetze()` — der einzige
  Bezeichner mit Umlaut in beiden Dateien (`grep -nP 'fn [a-z_]*[^\x00-\x7F]'`), neben `saetze` im
  selben Namen in Umschrift.
- `:3689` "Was sie messt und was sie nicht schaetzt", `:3788` "Sie messt eine Abwesenheit".

## Umfang

`krk-ui`, `appkit/editor.rs`, Pruefmodul.
