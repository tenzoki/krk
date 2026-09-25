Der Tastencode von Pfeil ab steht an zwei Stellen

---

`crates/krk-ui/src/appkit/ereignisse.rs` führt `const CODE_PFEIL_AB: u16 = 125;` neben `krk_core::tasten::code::PFEIL_AB`, das denselben Wert trägt. Zwei Wahrheiten für eine Zahl, und die Datei kennt beide: sie schreibt bereits `use krk_core::tasten::{self, Tastendruck}`.

---

## Wo es steht

- `crates/krk-core/src/tasten/mod.rs`, Modul `code`: `pub const PFEIL_AB: u16 = 125;`
- `crates/krk-ui/src/appkit/ereignisse.rs`: `const CODE_PFEIL_AB: u16 = 125;`

Die zweite Fassung dient `pfeil_ab_senden`, dem synthetischen Tastendruck der L1-Messung. Sie ist über `tasten::code::PFEIL_AB` erreichbar und braucht keine eigene Zahl.

## Was heute davor steht

Die Prüfung `die_maske_eines_pfeils_kommt_leer_im_kern_an` (`ereignisse.rs`, angelegt am 260803-2025) schlägt fehl, sobald die beiden Zahlen auseinanderlaufen: sie speist `CODE_PFEIL_AB` in den Nachschlag des Kerns und erwartet `Kommando::AuswahlRunter`. Die Divergenz fällt also auf. Ein Schaden ist heute nicht möglich; der Datensatz hält die Doppelung fest, damit sie beim nächsten Anfassen der Datei verschwindet.

Der zweite Wert daneben, `ZEICHEN_PFEIL_AB` (`'\u{F701}'`, `NSDownArrowFunctionKey`), ist **keine** Doppelung: der Kern kennt keine Zeichen, nur Tastencodes.

## Was zu tun ist

`CODE_PFEIL_AB` entfernen und an seiner Stelle `tasten::code::PFEIL_AB` schreiben.

**Aufgefallen bei:** der Behebung von `issues/260803-1536_c_die-pruefungen-bestaetigen-die-appkit-bitwerte-gegen-sich-selbst.md`.

---
Resolved: Mit der Umsetzung von Schritt 11 am 260803-2317 aufgelöst. `crates/krk-ui/src/appkit/ereignisse.rs` schreibt jetzt `const CODE_PFEIL_AB: u16 = code_von_pflicht("down");`. Die Zahl steht nicht mehr in der Datei, sondern kommt zur Übersetzungszeit aus `krk_core::tasten::parser::TASTEN`, der einen Tastentabelle des Programms. Ein Tippfehler im Namen bricht den Bau ab, statt eine zweite Wahrheit anzulegen. `krk_core::tasten::code`, das Modul mit den fünf abgeschriebenen Konstanten aus Schritt 7, ist zusammen mit der verdrahteten Tabelle entfallen; auch die Prüfdatei `crates/krk-core/tests/tasten.rs` holt ihre Tastencodes seither über `code_von_pflicht`. Belegt durch `grep -rn ": u16 = " crates/`: außerhalb von `parser.rs` steht keine Konstante mehr, die einen Tastencode als Zahl trägt.
