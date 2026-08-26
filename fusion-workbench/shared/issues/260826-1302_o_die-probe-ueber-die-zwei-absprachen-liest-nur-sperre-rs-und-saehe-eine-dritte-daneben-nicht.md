Die Probe über die zwei Absprachen liest nur `sperre.rs` und sähe eine dritte daneben nicht

---

`ueber_der_ablage_stehen_genau_zwei_absprachen` (`crates/krk-core/tests/baum.rs:215-241`) sagt zu: „eine dritte Absprache bräuchte eine dritte Datei, und sie fällt hier auf". Sie fällt nur auf, wenn ihr Dateiname als `pub const` in **`krk-core/src/ablage/sperre.rs`** steht. Die Probe holt sich genau diese eine Datei aus dem Quellbaum und sucht nirgends sonst; eine dritte Sperrdatei, die in `ablage/mod.rs`, in `einstellungen.rs` oder in `leseprofile.rs` erklärt würde, ist für sie unsichtbar.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Domain:** code
**Tree state:** `4a57028`
**Affected:** `crates/krk-core/tests/baum.rs:210-241`

## Der Rumpf

```rust
let nadel = concat!(".lo", "ck\"");
let (_, sperre) = quelldateien()
    .into_iter()
    .find(|(name, _)| name == "krk-core/src/ablage/sperre.rs")
    .expect("krk-core/src/ablage/sperre.rs steht nicht mehr im Baum");
let benannt: Vec<&str> = sperre.lines()
    .filter(|zeile| zeile.trim_start().starts_with("pub const") && zeile.contains(nadel))
    .collect();
assert_eq!(benannt.len(), 2, …);
```

Zwei Verengungen übereinander, und keine steht im Doc-Kommentar:

1. **Eine Datei statt des Baums.** Alle drei Nachbarproben derselben Datei laufen über `quelldateien()` als Ganzes; diese greift eine Datei heraus und zählt darin.
2. **`pub const` am Zeilenanfang.** Ein `pub(crate) const`, ein `const` in einem `impl`-Block, eine Zeile, die `rustfmt` umbricht, oder ein Name, der nicht auf `.lock` endet, entgeht dem Filter — auch innerhalb von `sperre.rs`.

## Warum das trotzdem nur „gering" ist

Die zwei Zusicherungen darunter (`:231-240`) halten die beiden bekannten Namen gegen den echten Wert der Konstanten:

```rust
assert_eq!(krk_core::ablage::sperre::SCHREIBSPERRE, "schreiben.lock", …);
assert_eq!(krk_core::ablage::sperre::SITZUNGSRECHT, "sitzungsrecht.lock", …);
```

Damit ist die Zusage „diese zwei gibt es und sie heißen so" gedeckt. Ungedeckt ist allein die Zusage „und keine dritte" — genau die Hälfte, für die die Zählung überhaupt da ist, denn die andere braucht keinen Quelltextgriff.

Der Kopf der Datei nennt die Blindheit seiner Nadeln allgemein (`:17-29`: „Was eine Nadel nicht entscheiden kann") und schreibt sie bei den Nachbarproben je einzeln aus. Bei dieser steht sie nicht, und der Doc-Kommentar behauptet stattdessen die Deckung.

## Richtung

Die drei Nachbarproben zeigen die Form: über `quelldateien()` laufen, in **Code**-Zeilen suchen (`im_code`, `:50-54`), und die Fundstellen als Liste gegen eine ausgeschriebene Erwartung halten statt gegen eine Zahl. Eine Nadel, die den Gegenstand statt des Namens trifft, wäre hier der Aufruf, der eine Sperrdatei überhaupt anlegt — dann fällt auch eine dritte auf, die nicht auf `.lock` endet.

Solange das nicht geschieht, gehört der Satz „eine dritte Absprache bräuchte eine dritte Datei, und sie fällt hier auf" so eingeschränkt, wie er trägt: sie fällt auf, wenn sie in `sperre.rs` als `pub const` mit der Endung `.lock` steht.

Gefunden bei der Vollbaum-Durchsicht R6 der dreizehn übrigen Probendateien des Kerns, HEAD `4a57028`.
