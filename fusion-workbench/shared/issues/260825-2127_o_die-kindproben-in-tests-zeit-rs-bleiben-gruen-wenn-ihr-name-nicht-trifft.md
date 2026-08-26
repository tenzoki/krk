# Die Kindproben in `tests/zeit.rs` bleiben grün, wenn ihr Name nicht trifft

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/tests/zeit.rs:66-110` (`kindprobe_in_zone`, `kind_ist_durchgelaufen`, die zwei Elternproben), `:10-19` (der Absatz „Warum Kindprozesse"); `crates/krk-core/tests/ablage.rs:2415-2430`, `:2560-2599`, `:2796-2828` (die Form, die diese Datei abschreibt)

---

## Was ist

Der Elternteil startet die Kindprobe und prüft **allein den Rückgabewert**
(`zeit.rs:81-89`):

```rust
assert!(
    ergebnis.status.success(),
    "die Kindprobe unter TZ={zone} ist gescheitert\n ...
```

`libtest` beendet sich mit 0, wenn ein Filter **kein** Verfahren trifft. Am 260825-2127 am
gebauten Prüfziel nachgemessen:

```
$ ./target/debug/deps/zeit-… --exact --ignored --nocapture --test-threads 1 kind_rechnet_in_utc_TIPPFEHLER
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out
EXIT=0
```

Wird `kind_rechnet_in_utc` oder `kind_rechnet_in_berlin` umbenannt, verschrieben oder
gestrichen, bleiben `unter_tz_utc_stehen_feste_zeitpunkte_auf_festen_kalenderwerten` und
`unter_europe_berlin_haengt_der_versatz_am_zeitpunkt_und_nicht_am_lauf` **grün** und messen
nichts. Genau diese zwei Proben sind laut Modulkopf „die einzige Probe, die den
Sommerzeitfall überhaupt prüfen kann: ohne sie wäre die Zusage behauptet".

## Warum es die Vorlage nicht trifft

Der Modulkopf sagt, diese Datei schreibe die Form von `tests/ablage.rs` ab und mache keine
neue auf (`zeit.rs:16-19`). Sie schreibt aber nur die eine Hälfte ab. Jeder Elternteil in
`ablage.rs` prüft **zusätzlich** eine Spur, die das Kind hinterlassen hat, und die fehlt,
wenn kein Kind gelaufen ist:

```rust
assert_eq!(
    fs::read_to_string(ordner.pfad().join("recht.txt")).expect("das Kind hat nichts gemeldet"),
    "ohne", ...
```

Damit ist `ablage.rs` gegen einen verfehlten Namen dicht und `zeit.rs` nicht. Es ist keine
Schwäche der gemeinsamen Form, sondern eine weggelassene Hälfte.

## Was zu tun wäre

Der billigste Weg ist derselbe wie in der Vorlage: den Rückgabewert nicht allein stehen
lassen. Zwei Möglichkeiten, beide klein:

1. **Die Ausgabe prüfen.** Das Kind läuft schon mit `--nocapture`; `kind_ist_durchgelaufen`
   hält zusätzlich, dass `stdout` `"1 passed"` trägt. Eine Zeile, keine neue Bauform.
2. **Eine Spur schreiben**, wie `ablage.rs` es tut. Genauer, aber teurer: das Kind bräuchte
   einen Ordner, den es heute nicht bekommt.

**Schwere:** gering. Der Baum ist heute grün und die Namen stimmen; die Probe sagt aber
Deckung zu, die sie an dieser einen Stelle nicht hat, und eine Zählprobe, die nichts fängt,
ist schlechter als keine.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`. Gemessen am
gebauten Prüfziel, nicht aus dem Quelltext geschlossen.

Also seen: 260826-1302 by coderev — dieselbe Klasse am gemeinsamen Kindstarter `tests/gemeinsam/mod.rs::kind_mit_deskriptorgrenze` und seinen sechs Rufern; eigener Datensatz `260826-1302_*_sechs-elternproben-am-gemeinsamen-kindstarter-bleiben-gruen-wenn-der-kindname-nicht-trifft.md`.
