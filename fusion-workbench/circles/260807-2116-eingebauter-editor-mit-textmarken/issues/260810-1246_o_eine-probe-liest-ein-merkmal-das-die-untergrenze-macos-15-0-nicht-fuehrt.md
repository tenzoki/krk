Eine Probe liest ein Merkmal, das die Untergrenze macOS 15.0 nicht fuehrt
---
`der_vorgabewert_der_schreibwerkzeuge_ueberlaesst_dem_system_die_wahl` liest `allowsWritingToolsAffordance` an einer `NSTextView`. Das SDK fuehrt die Eigenschaft nur an `NSTextField` und erst ab macOS 15.4; an `NSTextView` ist sie undokumentiert. `merkmal` bricht mit einer Panik ab, wenn der Name fehlt. Damit bindet die Probe den Bau weiter an die macOS-Fassung des pruefenden Geraets — genau das, was `260810-0417` beseitigen sollte.
---
**Schwere:** Mittel
**Gefunden:** Durchsicht des Diffs `38a02b2..HEAD`, Turn 3
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (nur Pruefcode)
**Zusammenhang:** `issues/260810-0417_c_die-laufzeitprobe-bindet-den-bau-an-die-macos-version-des-pruefenden-geraets.md`, `decisions/260810-0959_*`

## Belegstellen

`crates/krk-ui/src/appkit/editor.rs:4278-4284`:

```rust
assert_ne!(
    merkmal(&unsere, "allowsWritingToolsAffordance"),
    0,
    "die Angebotsflaeche der Schreibwerkzeuge steht aus — dann ist der Grund, \
     aus dem der Datensatz sie fuehrt, ein anderer geworden"
);
```

`crates/krk-ui/src/appkit/editor.rs:4000-4006`:

```rust
fn merkmal(flaeche: &NSTextView, merkmal: &str) -> isize {
    let schluessel = NSString::from_str(merkmal);
    let wert: Option<Retained<NSNumber>> =
        unsafe { msg_send![flaeche, valueForKey: &*schluessel] };
    wert.unwrap_or_else(|| panic!("die Flaeche fuehrt kein Merkmal {merkmal}"))
        .integerValue()
}
```

Und das SDK, gelesen und nicht erinnert:

```sh
$ grep -rn "allowsWritingToolsAffordance" "$(xcrun --show-sdk-path)/System/Library/Frameworks/AppKit.framework/Headers/"
NSTextField.h:61:@property BOOL allowsWritingToolsAffordance API_AVAILABLE(macos(15.4)); // Default is NO.
```

Eine Fundstelle, und sie steht an `NSTextField`, nicht an `NSTextView`. `NSTextView.h` fuehrt sie nicht. Auf 15.7.7 antwortet die Laufzeit trotzdem, also ist es ein undokumentierter Zugang.

Zum Vergleich die beiden, die in Ordnung sind: `setWritingToolsBehavior:` und `setAllowedWritingToolsResultOptions:` stehen in `NSTextView.h:434-435` mit `API_AVAILABLE(macos(15.0))`, also auf der Untergrenze.

## Fehlszenario

Zwei, und beide faerben `cargo test` rot, ohne dass am ausgelieferten Code etwas falsch waere:

1. **macOS 15.0 bis 15.3.** Das Projekt sagt macOS 15 als Untergrenze zu (`CLAUDE.md`, Technologiewahl). Auf einer solchen Fassung ist die Eigenschaft nach dem SDK nicht da; fehlt der Zugang an `NSTextView` mit ihr, laeuft `merkmal` in die Panik: „die Flaeche fuehrt kein Merkmal allowsWritingToolsAffordance".
2. **Apple nimmt den undokumentierten Zugang fort.** Dieselbe Panik, auf einem System, das KRK unterstuetzen soll.

Dazu kommt eine Ungereimtheit in der Aussage selbst: `assert_ne!(…, 0)` verlangt „die Angebotsflaeche steht **an**", und der Header sagt fuer `NSTextField` „Default is NO". Was der Werkswert an einer `NSTextView` bedeutet, ist damit nicht belegt, und die Probe haelt eine Zahl fest, deren Herkunft undokumentiert ist.

## Was der Datensatz `260810-0959` wissen sollte

Die vier Einstellungen, die `EINSTELLUNGEN` als `NochOffen` fuehrt (`editor.rs:3692-3699`), sind nicht vier von einer Art. Gegen die Kopfdateien:

| Setzer | im SDK | Untergrenze |
|---|---|---|
| `setWritingToolsBehavior:` | `NSTextView.h:434` | macos(15.0) |
| `setAllowedWritingToolsResultOptions:` | `NSTextView.h:435` | macos(15.0) |
| `setWritingToolsAllowedInputOptions:` | in keiner Kopfdatei | undokumentiert |
| `setAllowsWritingToolsAffordance:` | nur an `NSTextField` | macos(15.4) |

Zwei oeffentliche und zwei undokumentierte. Der ausgelieferte Code setzt keinen davon; die Entscheidung, was aus den vier wird, betrifft aber zwei verschiedene Arten von Gegenstand, und der Datensatz behandelt sie als eine.

## Vorschlag

Die Probe auf den Zuschnitt bringen, den `260810-0417` fuer ihre Nachbarin gewaehlt hat: **Hinweis statt Fehlschlag**, wenn die Laufzeit den Namen nicht fuehrt.

Konkret: `merkmal` um eine Schwester `merkmal_falls_vorhanden(flaeche, name) -> Option<isize>` ergaenzen (dasselbe `msg_send!`, ohne die Panik), die Probe auf `writingToolsBehavior` als Zusicherung stellen — das ist die oeffentliche, auf der Untergrenze — und `allowsWritingToolsAffordance` nur noch dann pruefen, wenn die Laufzeit es fuehrt. Fehlt es, ein `eprintln!` mit demselben Satz, den die Nachbarin fuer ihre Gegenrichtung benutzt.

Die Panik in `merkmal` selbst bleibt richtig, wo der Name aus `EINSTELLUNGEN` kommt: dort ist ein fehlender Name die Meldung, dass die Aufzaehlung nachzuziehen ist. Der Unterschied ist, ob KRK den Namen fuehrt oder Apple.
