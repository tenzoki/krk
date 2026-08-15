Die Probe über die Paarung von ZIELE und ARCHITEKTUREN prüft Mitgliedschaft statt Paarung

---

`jedes_ziel_tripel_bekommt_einen_namen_aus_den_architekturen`
(`xtask/src/release.rs:765-773`) sagt in ihrem Doc-Kommentar zu, ein drittes Ziel liefe
„von selbst mit und muesste einen Namen aus [`ARCHITEKTUREN`] bekommen, statt still
durchgereicht zu werden" (`release.rs:761-763`). Ihr Rumpf prüft etwas Schwächeres:

```rust
for ziel in ZIELE {
    let rust_name = ziel.split('-').next().unwrap();
    assert!(ARCHITEKTUREN.contains(&lipo_name(rust_name)), "{ziel} wird nicht uebersetzt");
}
```

`lipo_name` (`release.rs:111-121`) reicht einen unbekannten Namen durch. Fällt ein Ziel aus
der Umrechnung heraus, kommt sein eigener Rust-Name zurück — und wenn der zufällig in
`ARCHITEKTUREN` steht, geht die Zusicherung durch. Für `x86_64-apple-darwin` ist genau das
schon heute der Fall: Rust und `lipo` schreiben die Architektur gleich. Die Probe kann
diesen Ausfall an diesem Ziel also nicht sehen.

---

**Schwere:** niedrig. Kein Verhalten, kein Bau, kein heutiger Fehlbefund — die Paarung
stimmt am Baum. Der Befund ist eine Zusicherung, die weniger misst als ihr Kommentar sagt,
und zwar an der Stelle, die die Runde neu tragend gemacht hat.
**Gefunden von:** coderev, Durchsicht des Bereichs `cd0b5b7..093a6f4`
**Betroffen:** `xtask/src/release.rs:759-773`
**Domain:** code

## Was heute trägt und was nicht

Die Reihenfolge der beiden Aufzählungen (`release.rs:75` und `release.rs:83`) ist seit
`093a6f4` tragend, weil `lipo_name` sie über `zip` paarweise liest. Gehalten wird sie von
dreierlei, und die drei decken verschieden weit:

| Wächter | Was er fängt |
|---|---|
| `const _: () = assert!(ZIELE.len() == ARCHITEKTUREN.len())` (`release.rs:86`) | ein Ziel ohne Namen und einen Namen ohne Ziel — beim Übersetzen, und das trägt vollständig |
| `die_beiden_ziele_tragen_die_namen_die_lipo_dafuer_meldet` (`release.rs:766-769` des Prüfmoduls) | jedes Vertauschen, das eines der **zwei heutigen** Paare trifft |
| `jedes_ziel_tripel_bekommt_einen_namen_aus_den_architekturen` | nichts, was die erste Probe nicht schon fängt |

Für die zwei Ziele von heute reicht das. Die dritte Zeile ist die Lücke, und sie öffnet sich
genau in dem Fall, für den die Probe geschrieben ist: bei einem dritten Ziel. Stünden
`ZIELE` und `ARCHITEKTUREN` dann an den Stellen 3 und 4 gegeneinander vertauscht, ließen
beide Proben es durch — die erste prüft nur die zwei fest hingeschriebenen Paare, die zweite
nur Mitgliedschaft.

## Die Randfälle von `lipo_name` sind geprüft

Am Rumpf nachgelesen, und die vierte Probe (`release.rs:775-787` des Prüfmoduls) deckt sie
schon ab:

- **leere Zeichenkette:** `strip_prefix("")` gibt `Some("x86_64-apple-darwin")` zurück,
  `starts_with('-')` ist falsch, kein Treffer; `""` kommt durch. Richtig.
- **ganzes Ziel-Tripel:** `strip_prefix("aarch64-apple-darwin")` gibt `Some("")`,
  `starts_with('-')` ist falsch; das Tripel kommt durch. Richtig, und es ist die
  Bedingung `rest.starts_with('-')`, die das leistet — ohne sie träfe auch `aarch` zu.
- **Präfix auf zwei Ziele:** kann heute nicht eintreten, beide Tripel haben verschiedene
  Architekturteile. Träte es ein, gewönne das erste Paar. Das ist ein sinnvolles Verhalten
  und kein Zufall, aber es steht nirgends geschrieben.

## Was zu tun wäre

Die dritte Probe stellungsbezogen machen, dann trägt sie, was ihr Kommentar sagt:

```rust
for (ziel, erwartet) in ZIELE.into_iter().zip(ARCHITEKTUREN) {
    assert_eq!(lipo_name(ziel.split('-').next().unwrap()), erwartet, "{ziel}");
}
```

Das fängt jedes Vertauschen an jeder Stelle, auch bei zehn Zielen, und macht die zweite
Probe nicht überflüssig — die hält die zwei Namen als Tatsache über `lipo` fest, diese hier
die Paarung.

## Herkunft

Gemeinsamer Speicher. Betrifft `xtask` und den Auslieferungsweg des ganzen Projekts.
