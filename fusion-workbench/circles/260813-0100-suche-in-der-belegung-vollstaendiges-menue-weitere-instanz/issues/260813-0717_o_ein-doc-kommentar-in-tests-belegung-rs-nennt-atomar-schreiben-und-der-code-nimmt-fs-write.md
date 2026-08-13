Ein Doc-Kommentar in `tests/belegung.rs` nennt `atomar::schreiben`, und der Code nimmt `fs::write`

---

Der in Turn 2 angebaute Absatz an `ablage_mit` (`crates/krk-core/tests/belegung.rs:34-41`)
endet mit:

> der Pfad kommt deshalb aus dem [`Zugang`] und der Vorgang aus `atomar::schreiben`, wie bei
> `settings.toml`.

Der Rumpf drei Zeilen weiter nimmt `fs::write` (`:45-48`):

```rust
ablage
    .durchgang(|zugang| fs::write(zugang.pfad(Datei::Belegung), keymap))
```

Die Hälfte über den Pfad stimmt, die über den Vorgang nicht. Der Vergleich mit `settings.toml`
stimmt ebenfalls nicht: jene Stelle (`crates/krk-core/tests/ablage.rs:372-380`) ruft wirklich
`atomar::schreiben`.

**Der Unterschied ist nicht folgenlos.** `atomar::schreiben` ersetzt die Zieldatei über ein
`rename`, `fs::write` schreibt sie in place. Für eine Probe, die eine `keymap.toml` einmal
anlegt, ist beides gleichwertig; für den Leser, der den Absatz als Muster nimmt, ist es das
nicht. Dazu kommt eine stillere Folge: hätte `ablage_mit` wirklich `atomar::schreiben`
gerufen, wäre `crates/krk-core/tests/belegung.rs` die sechste Datei geworden, die
`nur_benannte_dateien_erreichen_das_atomare_schreiben` zählt, und jene Probe rot. Der
Kommentar beschreibt also einen Weg, den der Baum in dieser Fassung nicht zulässt, ohne die
Liste zu erweitern.

---

**Schwere:** gering. Ein Satz, kein Verhalten. Er gehört zur Klasse, die dieselbe Durchsicht
in Turn 1 unter „Prosa, die den Baum nicht mehr trifft" viermal abgelegt hat, und er ist im
Reparatur-Turn neu entstanden.

**Gefunden:** coderev, Durchsicht von `a34bf17..dff167a` am 260813-0717

**Betroffen:** `crates/krk-core/tests/belegung.rs:41`

**Domain:** code

## Vorschlag

Den Halbsatz auf den Baum bringen: „der Pfad kommt deshalb aus dem `Zugang`, und geschrieben
wird unter der Schreibsperre". Der Vergleich mit `settings.toml` trägt dann für das, wofür er
gemeint war — beide Male ein roher Text unter einem Durchgang — und behauptet nicht mehr
dieselbe Schreibfunktion.

Wer stattdessen wirklich `atomar::schreiben` nehmen will, trägt
`krk-core/tests/belegung.rs` in die Liste von
`nur_benannte_dateien_erreichen_das_atomare_schreiben` ein. Das ist die teurere Antwort und
kauft für eine Probe, die eine Datei anlegt, nichts.
