# E1 und E2 teilen eine Zusicherung, die eine Probe hält, und lassen den Baum dazwischen rot

**Status:** Open
**Domain:** Plan der Filter-Runde, Strang E
**Filed by:** coder, beim Umsetzen von E1
**Related:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Schritte E1 und E2

## Befund

`tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`
(`crates/krk-core/src/tasten/belegung.rs:1560`) prüft, dass jede Kennung aus
`Kommando::KENNUNGEN` in `resources/default-keymap.toml` einen Eintrag hat. E1 fügt die
Kennung hinzu, E2 den Eintrag. Zwischen beiden Schritten ist die Zusicherung gebrochen,
und die Probe schlägt fehl:

```
TiefeSucheUmschalten nennt die Kennung tiefe_suche_umschalten,
die Auslieferungsbelegung kennt sie nicht
```

Nach E1 allein steht `make check` auf `Error 2` (`cargo test -p krk-core --lib` auf 101),
mit genau diesem einen Fehlschlag; Bau, `cargo fmt --all --check` und
`cargo clippy --workspace --all-targets -- -D warnings` laufen sauber durch.

## Warum das nicht am Umsetzer liegt

Die beiden Schritte gehören zwei Ausführenden: E1 dem `coder`, E2 dem `ontocoder`, weil
`resources/default-keymap.toml` Belegungsdaten trägt und nicht Code. Kein Ausführender
kann die Zusicherung allein halten. Der Plan ordnet E2 hinter E1 ein und macht damit einen
roten Zwischenstand unvermeidlich.

## Warum das eine Meldung wert ist

Der Orchestrator schreibt nach einem Bericht ohne `exit 0` nichts fest. Ein Plan, dessen
Schrittfolge einen solchen Bericht erzwingt, hält seine eigene Übergabe an. Nach E2 ist
der Baum wieder grün; die Meldung sagt, dass der Zwischenstand vorhergesehen und nicht
gefunden ist.

## Vorschlag

E1 und E2 als ein Paar behandeln und erst nach E2 abnehmen, oder die Reihenfolge umkehren:
`resources/default-keymap.toml` verträgt einen Eintrag, zu dem es noch kein `Kommando`
gibt — die Probe prüft diese Richtung nicht, und `Kommando::aus_kennung` antwortet dafür
ausdrücklich `None` mit der Bedeutung "noch nicht gebaut"
(`crates/krk-core/src/tasten/belegung.rs:706`). Umgekehrt geht es nicht.
