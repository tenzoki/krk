Die Prüfungen der Tastennormalisierung bestätigen die AppKit-Bitwerte gegen sich selbst

---

Die ganze C3-Abnahme hängt an acht von Hand abgeschriebenen Zahlen in
`crates/krk-core/src/tasten/normalisierung.rs:34-51`. Keine Prüfung vergleicht sie
mit dem, was AppKit wirklich führt. Die Prüfungen speisen dieselben Konstanten ein,
die die Umsetzung liest, und können deshalb nur bestätigen, dass die Umsetzung mit
sich selbst übereinstimmt.

---

## Der Kreis

`crates/krk-core/tests/tasten.rs:47-58`:

```rust
let erwartet = [
    (roh::BEFEHL, ModMaske::BEFEHL),
    (roh::STEUERUNG, ModMaske::STEUERUNG),
    …
];
for (rohes_bit, maske) in erwartet {
    assert_eq!(normalisieren(rohes_bit), maske);
}
```

`normalisieren` (`normalisierung.rs:148-163`) prüft `rohe_flaggen & roh::BEFEHL`.
Die Prüfung reicht `roh::BEFEHL` hinein. Stünde `BEFEHL` auf `1 << 21` statt auf
`1 << 20`, bliebe die Prüfung grün, und KRK hielte den Zehnerblock für die
Befehlstaste. Dasselbe gilt für
`function_feststelltaste_zehnerblock_und_hilfe_fallen_weg`
(`tests/tasten.rs:60-75`) und für
`f3_mit_und_ohne_function_ergibt_dieselbe_nachschlagemaske`
(`tests/tasten.rs:22-33`).

Der Dateikopf der Prüfungen nennt den Grund und ist darin richtig:

> Die rohen Bitwerte kommen aus `krk_core::tasten::normalisierung::roh` und
> stehen hier nicht ein zweites Mal. […] sie hier als Zahlen zu wiederholen hiesse,
> zwei Wahrheiten zu führen.

(`tests/tasten.rs:8-11`) Die Schlussfolgerung stimmt für eine Wiederholung im
Prüfungstext. Sie lässt aber offen, dass es eine **dritte** Wahrheit gibt, die
weder abgeschrieben noch geraten ist.

## Es gibt eine kostenlose Gegenprobe

`objc2-app-kit` führt die Werte selbst, in
`objc2-app-kit-0.3.2/src/generated/NSEvent.rs:387-406`:

```rust
impl NSEventModifierFlags: NSUInteger {
    const CapsLock = 1<<16;
    const Shift = 1<<17;
    const Control = 1<<18;
    const Option = 1<<19;
    const Command = 1<<20;
    const NumericPad = 1<<21;
    const Help = 1<<22;
    const Function = 1<<23;
}
```

`krk-core` darf diese Kiste nicht kennen, das ist die Architekturgrenze und
bleibt richtig. `krk-ui` kennt beide: es führt `objc2-app-kit` und `krk-core` als
Abhängigkeiten (`crates/krk-ui/Cargo.toml:17-22`). Eine Prüfung in `krk-ui`, die
für alle acht Bits `roh::X == NSEventModifierFlags::Y.0 as u64` behauptet,
schließt die Lücke, ohne die Grenze anzufassen und ohne eine zweite Wahrheit
anzulegen: sie vergleicht die vorhandene Kopie mit der Quelle, aus der sie stammt.

Nachgeprüft am 260803-1536: alle acht Werte in `normalisierung.rs:36-50` stimmen
heute mit `objc2-app-kit-0.3.2` überein. Der Befund ist die fehlende
Gegenprobe, nicht ein falscher Wert.

## Dieselbe Lücke bei den fünf Tastencodes, mit einem anderen Ausweg

`crates/krk-core/tests/tasten.rs:88-101`:

```rust
let erwartet = [
    (code::PFEIL_AUF, Kommando::AuswahlHoch),
    (code::PFEIL_AB, Kommando::AuswahlRunter),
    …
];
```

Diese Liste ist Zeichen für Zeichen die Tabelle `VERDRAHTET` aus
`crates/krk-core/src/tasten/mod.rs:90-96`, in derselben Reihenfolge, über
dieselben Konstanten. Sie kann nur scheitern, wenn jemand eine der beiden Listen
ändert und die andere nicht. Ob `PFEIL_AB` wirklich 125 ist, prüft nichts.

Der Ausweg ist hier ein anderer, weil `objc2` keine Tastencodes führt: das
Muster steht schon in derselben Datei. `tests/tasten.rs:16-17` schreibt

```rust
/// Der gemessene Tastencode von F3, aus `spikes/fn-tasten/messung-A.txt`.
const F3: u16 = 99;
```

Genau so gehören die fünf verdrahteten Codes einmal als Zahl in die Prüfung,
gegen `crates/krk-core/src/tasten/mod.rs:36-44` gehalten. Für die Pfeil- und
Bildtasten gibt es allerdings **keine** Messung im Projekt: `messung-A.txt`
enthält kein einziges Pfeiltasten-Ereignis. Der Beleg wäre also die
Carbon-`HIToolbox`-Tabelle, so benannt, oder eine eigene kurze Messung. Der
Verweis auf eine Messdatei ist an dieser Stelle nicht zu haben; siehe
`issues/260803-1536_o_normalisierung-belegt-drei-aussagen-mit-einer-messung-die-sie-nicht-traegt.md`.

## Was zu tun ist

1. Eine Prüfung in `krk-ui`, die alle acht Werte aus
   `krk_core::tasten::normalisierung::roh` gegen `NSEventModifierFlags`
   vergleicht. Sie gehört nicht unter `src/appkit/`, weil sie keinen
   Objective-C-Aufruf macht, sondern nur zwei Konstanten liest.
2. Die fünf Tastencodes in `tests/tasten.rs` einmal als Zahl gegen ihre
   Konstanten halten, mit einer benannten Quelle im Kommentar.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.
