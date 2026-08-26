Drei Prosastellen in den Probendateien des Kerns behaupten mehr oder anderes, als daneben steht

---

Drei kleine Stellen, alle derselben Art: der Text neben einer Probe sagt etwas, das die Probe nicht sagt. Ein toter Verweis auf eine Probe, die es seit der Runde 10 nicht mehr gibt; ein Kommentar über eine Bytefolge mitten in einem Mehrbytezeichen, über einer Zusicherung, die etwas anderes zählt; und ein Modulkopf, der die großen Prüfdateien unter `/tmp` verortet, wo sie nicht liegen.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/belegung.rs:181`; `crates/krk-core/tests/text.rs:205-207`; `crates/krk-core/tests/operation.rs:10-11`

## 1. Ein Verweis auf eine Probe, die es nicht mehr gibt

```rust
// belegung.rs:180-181
/// Gesucht statt hingeschrieben, und aus demselben Grund wie in
/// [`keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke`]:
```

Diese Probe steht nirgends im Baum. Die Sprungmarke ist mit der Runde 10 gefallen; ihre Nachfolgerin heißt `keine_unbelegte_kombination_mit_befehlstaste_faellt_auf_das_tippen` (`belegung.rs:838`) und ist am 260816 noch einmal geteilt worden. Der Verweis steht in eckigen Klammern, also als Intra-Doc-Link — den prüft niemand: `cargo doc` erzeugt für ein Probenziel keine Dokumentation, und `rustdoc` sieht die Datei nie.

## 2. Ein Kommentar über etwas, das die Zusicherung nicht misst

```rust
// text.rs:205-207
// Ein Bytepaar mitten in einem Mehrbytezeichen ist kein Treffer: gesucht
// wird ueber Zeichen und nicht ueber Bytes.
assert!(suche::alle(text, "pfel 🍎 und").len() == 1);
```

Die Zusicherung prüft, dass eine gültige Teilzeichenfolge genau einmal vorkommt. Über ein Bytepaar mitten in einem Mehrbytezeichen sagt sie nichts, und sie kann es nicht: der Suchtext ist ein `&str` und trägt keine halben Zeichen. Was der Kommentar meint, hält die Schleife darüber (`:190-196`, `is_char_boundary` an beiden Enden jedes Treffers); hier steht er über der falschen Zeile.

(Daneben: `assert!(… == 1)` statt `assert_eq!`, das im Fehlerfall die Zahl nennt. Dieselbe Datei nimmt sonst durchweg `assert_eq!`.)

## 3. Die großen Prüfdateien liegen nicht unter `/tmp`

```
// operation.rs:10-11
//! Zwei der vier Abnahmepunkte brauchen eine 200-MB- und eine 500-MB-Datei.
//! Beide entstehen unter `/tmp`, auf demselben APFS-Datentraeger wie ihr Ziel,
```

Sie entstehen im `Pruefordner`, und der liegt unter `std::env::temp_dir()` (`tests/gemeinsam/mod.rs:71`) — auf macOS also unter `/var/folders/…/T` und nicht unter `/tmp`. Der Modulkopf des Prüfordners hält den Unterschied ausdrücklich fest (`:9-11`); dieser hier hat ihn nicht mitbekommen. Die Aussage daneben („auf demselben APFS-Datentraeger wie ihr Ziel") trägt weiter und ist die, auf die es der Probe ankommt.

## Richtung

Drei Einzelkorrekturen, keine davon berührt eine Zusicherung:

1. Den Verweis auf `keine_unbelegte_kombination_mit_befehlstaste_faellt_auf_das_tippen` umschreiben — oder auf den Datensatz `260805-0820_*_die-belegungspruefung-nimmt-cmd-q-als-beispiel-fuer-eine-unbelegte-kombination.md`, den derselbe Doc-Kommentar zwei Zeilen tiefer ohnehin nennt und der die Begründung wirklich trägt.
2. Den Kommentar über die Schleife stellen, wo die Zeichengrenzen geprüft werden, und die Zeile auf `assert_eq!` ziehen.
3. `/tmp` durch „unter dem Temporärverzeichnis" ersetzen, in der Sprache, die `tests/gemeinsam/mod.rs` schon führt.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
