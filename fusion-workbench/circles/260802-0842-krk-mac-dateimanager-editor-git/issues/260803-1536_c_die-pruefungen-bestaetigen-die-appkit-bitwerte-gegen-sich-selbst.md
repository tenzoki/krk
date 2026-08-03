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

---
Resolved: 260803-2025. Beide Gegenproben sind da.

**Die acht Bitwerte**, in `crates/krk-ui/src/appkit/ereignisse.rs`: `die_acht_rohen_bitwerte_des_kerns_stimmen_mit_appkit_ueberein` hält jeden Wert aus `krk_core::tasten::normalisierung::roh` gegen sein Gegenstück in `NSEventModifierFlags` und nennt bei einem Fehlschlag den Namen. Daneben steht `die_maske_eines_pfeils_kommt_leer_im_kern_an`: sie geht den Weg, den `behandeln` geht, also über `modifierFlags().0 as u64`, damit der Vergleich nicht bloß zwei Konstanten betrifft, die niemanden angehen.

**Nachgewiesen, dass die Prüfung greift.** `roh::BEFEHL` versuchsweise auf `1 << 21` gesetzt — genau der Fall, den dieser Datensatz beschreibt — und die Prüfung gefahren: `der Wert fuer Command weicht von NSEventModifierFlags ab, left: 2097152, right: 1048576`. Danach zurückgenommen.

**Abweichung von der vorgeschlagenen Stelle, mit Grund.** Der Datensatz sagt, die Prüfung gehöre nicht unter `src/appkit/`, weil sie keinen Objective-C-Aufruf macht. Sie liegt trotzdem dort, in `ereignisse.rs`. Der geschlossene Defekt `260803-1345_c_dateiliste-von-s8-legt-objc2-code-ausserhalb-von-appkit-ab.md` hat die Grenze anders gezogen: sie hängt an jeder Berührung mit `objc2` und nicht an der Übersetzerregel, und `260803-1530_o_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen.md` schlägt genau dafür eine Prüfvorschrift auf `use objc2` vor. Eine Datei mit `use objc2_app_kit` neben `appkit/` wäre der erste Verstoß gegen diese Vorschrift. `ereignisse.rs` ist außerdem der richtige Ort der Sache nach: es ist die Datei, deren `behandeln` die Annahme macht, dass die Bits übereinstimmen. Nachgeprüft: `grep -rEln '^[[:space:]]*use +objc2' crates/krk-ui/src` gibt weiter keine Zeile außerhalb von `appkit/` aus.

**Die fünf Tastencodes**, in `crates/krk-core/tests/tasten.rs`: `die_fuenf_verdrahteten_tastencodes_stimmen_mit_der_carbon_tabelle_ueberein` hält sie einmal als Zahl gegen ihre Konstanten. Die Quelle ist benannt und am 260803-2025 im SDK nachgesehen, nicht aus dem Gedächtnis zitiert: `Carbon.framework/Frameworks/HIToolbox.framework/Headers/Events.h`, `kVK_Return = 0x24` (Zeile 266), `kVK_PageUp = 0x74` (304), `kVK_PageDown = 0x79` (309), `kVK_DownArrow = 0x7D` (313), `kVK_UpArrow = 0x7E` (314). Alle fünf stimmen. Der Kommentar sagt dazu, dass es dafür keine Messung gibt und warum der Weg über `objc2` hier nicht offensteht.

Nebenbefund, eigener Datensatz: `issues/260803-2025_o_der-tastencode-von-pfeil-ab-steht-an-zwei-stellen.md`.
