Die `include_str!`-Bindung koppelt eine Wendung, der Kommentar daneben verspricht eine Beschreibung

---

`xtask/src/sign.rs:660-665`, in
`beide_faelle_nennen_die_fehlende_gehaertete_laufzeitumgebung`:

```rust
// Eine Beschreibung von `release`, nicht zwei: der Hilfetext traegt
// dieselbe Wendung. Faellt sie dort, faellt diese Probe.
assert!(
    include_str!("main.rs").contains("und gehaerteter Laufzeitumgebung"),
    "der Hilfetext in main.rs nennt die gehaertete Laufzeitumgebung nicht mehr"
);
```

Es gibt weiter zwei Beschreibungen von `cargo xtask release`, und gekoppelt ist eine Wendung
von 31 Zeichen.

---

**Schwere:** niedrig. Die Probe misst richtig, was ihr Name zusagt; zu weit greift der
Kommentar daneben und der Text der Zusicherung.
**Gefunden von:** coderev, Durchsicht des Bereichs `a2670db..8c06747`
**Betroffen:** `xtask/src/sign.rs:660-665`
**Domain:** code

## Was die Zusicherung trägt und was nicht

Sie trägt: fällt die Wendung „und gehaerteter Laufzeitumgebung" aus `main.rs`, schlägt die
Probe an. Das ist ein echter Wächter und mehr, als vorher dastand.

Sie trägt nicht:

- **Die Stellung.** Geprüft wird die ganze Datei, nicht die Konstante `HILFE`. Wer die
  Wendung aus dem Hilfetext nähme und in einen Kommentar schriebe — etwa in den Aufruf-
  kommentar drei Zeilen über der Ausgabe des Hinweises —, ließe die Probe grün. Es ist
  derselbe Befund, den die vorige Durchsicht schon zweimal abgelegt hat:
  `shared/issues/260815-1446_o_…` („zählt in `main.rs` nur Vorkommen, nicht Stellung") und
  `shared/issues/260815-1447_o_…` („prüft Mitgliedschaft statt Paarung").
- **Die übrige Beschreibung.** Der Hilfetext (`main.rs:66-76`) und der Schlusssatz des
  Hinweises (`sign.rs:205-208`) beschreiben denselben Befehl mit verschiedenen Worten:
  „uebersetzt beide Mac-Ziele, fuegt sie mit lipo zu einer universellen Binaerdatei
  zusammen" gegen „baut beide Mac-Architekturen und fuegt sie zusammen", „reicht ueber
  `xcrun notarytool submit --wait` zur Beglaubigung ein und heftet das Ergebnis mit
  `xcrun stapler staple` an" gegen „heftet nach der Beglaubigung das Ticket an". Das ist in
  Ordnung — der Hilfetext ist ausführlicher als eine Statuszeile —, aber „Eine Beschreibung
  von `release`, nicht zwei" beschreibt es nicht.

## Vorschlag

Zwei Zeilen, kein Umbau: die Zusicherung gegen `crate::HILFE` statt gegen
`include_str!("main.rs")` führen, sofern die Konstante dafür sichtbar ist, und den Kommentar
auf das zurücknehmen, was er hält — die eine Wendung steht in beiden Beschreibungen und
fällt nur gemeinsam.

Der Datensatz steht zusammen mit `260815-1446` und `260815-1447`; alle drei sind derselbe
Befund an drei Stellen und gehören in einen Durchgang.

## Ablage

Gemeinsamer Speicher. Betrifft den Bauweg des ganzen Projekts und nicht die Directive einer
Runde.
