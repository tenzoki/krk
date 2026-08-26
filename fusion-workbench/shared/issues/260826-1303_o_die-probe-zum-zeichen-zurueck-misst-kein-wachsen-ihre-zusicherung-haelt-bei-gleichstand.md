# Die Probe zum Zeichen-Zurück misst kein Wachsen; ihre Zusicherung hält bei Gleichstand

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Severity:** Medium
**Affected:** `crates/krk-core/tests/verzeichnis.rs:1094-1122`, tragend `:1106-1109`
**Tree state:** `4a57028`
**Cross-references:** `crates/krk-core/tests/verzeichnis.rs:691-699` (`filterordner`), `:359-364` (`geladenes_modell`, ohne `tief_setzen`); `crates/krk-core/src/verzeichnis/modell.rs:374` (`tief: true` ab Werk)

---

## Was ist

Die Probe heißt `ein_zeichen_zurueck_laesst_die_liste_wieder_wachsen` und hält
das Wachsen so:

```rust
// tests/verzeichnis.rs:1104-1109
assert!(modell.letztes_zeichen_weg(), "es war etwas wegzunehmen");
assert_eq!(modell.filtertext(), "aa");
assert!(
    modell.zeilenzahl() >= eng,
    "die Liste waechst um die Eintraege, die wieder passen"
);
```

`>=` ist bei Gleichstand erfüllt. Die Zusicherung hielte damit auch dann, wenn
`letztes_zeichen_weg` den Filtertext zwar kürzte, die Sicht aber gar nicht neu
aufbaute — und sogar dann, wenn der Filter überhaupt nie eine Zeile wegnähme.

## Und Gleichstand ist der Fall, der eintritt

Am `filterordner` (`:691-699`) nachgerechnet. `geladenes_modell` setzt den
Schalter nicht, also fährt die Probe seit `20c9833` mit `tief == true`; ein
Ordner ohne Namenstreffer steht dann unter Vorbehalt und ist `Unentschieden`,
also unsichtbar, solange kein Durchlauf antwortet — und in dieser Probe läuft
keiner.

| Filtertext | sichtbar |
|---|---|
| `aaa` | `aaa-ordner`, `AAA-gross.txt`, `bbbaaaccc.rs` — drei |
| `aa` | dieselben drei |
| `a` | dieselben drei |
| leer | alle fünf |

`eng` ist 3, und `zeilenzahl()` ist danach zweimal wieder 3. Die Zusicherung
läuft `3 >= 3`. Kein Anschlag lässt die Liste in dieser Probe wachsen.

**Vor der Vorgabenänderung war es dieselbe Lage**, nur mit anderen Zahlen: flach
stehen bei jedem der drei Filtertexte vier Zeilen, also `4 >= 4`. Die Zusicherung
war nie eine über das Wachsen.

Was die Probe wirklich hält, steht am Ende: `assert_eq!(modell.zeilenzahl(), 5)`
bei leerem Filtertext (`:1113-1117`) und `assert!(!modell.letztes_zeichen_weg())`
(`:1118-1121`). Beides ist echt.

## Was zu tun wäre

Ein Bestand, an dem die Kürzung wirklich einen Eintrag zurückholt, und dann eine
Gleichheit statt einer Ungleichung. Am billigsten am vorhandenen Ordner: eine
sechste Datei, deren Name `aa` trägt und `aaa` nicht, etwa `baaz.txt`. Dann steht
`eng` auf 3 und die Zeilenzahl nach dem ersten Rücknehmen auf 4, und die
Zusicherung heißt `assert_eq!(modell.zeilenzahl(), 4)`.

Eine Ungleichung genügt hier nicht, gleich wie der Bestand aussieht: sie
unterscheidet „gewachsen" nicht von „unverändert", und genau das ist der
Unterschied, den der Name behauptet.

**Gefunden:** coderev, Vollbaum-Durchsicht R5 der drei größten Probendateien des
Kerns.
