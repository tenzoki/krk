# `umlaufen` behauptet, die eine Stelle des Umlaufs zu sein; `voriger` läuft daneben um

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht von Turn 1 der Editor-Runde
**Betroffen:** `crates/krk-core/src/text/suche.rs:119-138`
**Cross-references:** Plan S8

---

## Der Befund

`crates/krk-core/src/text/suche.rs:128-138`:

```rust
/// Die drei Auswahlfunktionen laufen um, und das ist die eine Stelle, an der
/// sie es tun.
fn umlaufen(treffer: &[Treffer], stelle: usize) -> Option<usize> {
    if treffer.is_empty() {
        return None;
    }
    Some(if stelle < treffer.len() { stelle } else { 0 })
}
```

Zwei der drei rufen sie: `erster_ab` (`:101-104`) und `naechster` (`:111-114`).
Die dritte nicht (`:119-126`):

```rust
pub fn voriger(treffer: &[Treffer], versatz: usize) -> Option<usize> {
    let davor = treffer.partition_point(|kandidat| kandidat.anfang < versatz);
    // Der Umlauf nach hinten fuehrt auf den letzten; die leere Liste hat
    // keinen, und `checked_sub` beantwortet beides in einem.
    davor
        .checked_sub(1)
        .or_else(|| treffer.len().checked_sub(1))
}
```

`voriger` trägt seinen Umlauf und seine Leerlistenbehandlung selbst, in einer
zweiten Formulierung. Das Verhalten ist richtig — die Probe
`die_auswahl_laeuft_in_beide_richtungen_um` (`:224-229`) deckt beide Richtungen
ab —, aber der Satz an `umlaufen` stimmt nicht: es sind zwei Stellen, nicht eine.

## Warum das zählt

Nicht wegen der drei Zeilen. Sondern weil dieses Modul die
Einzelstellen-Zusage als Bauart führt und sie an drei weiteren Stellen des
Moduls ausdrücklich ausschreibt: "wäre die zweite Wahrheit darüber, was ein
Treffer ist" (`:37`), "Er ist kein Sonderfall des Ersetzens, sondern einer der
Suche, und wird deshalb genau einmal behandelt" (`:29-31`), "**Die Regel für eine
zu große Nummer steht hier und nur hier**" (`zeilen.rs:30`). Ein Satz, der diese
Zusage behauptet, wo sie nicht gilt, macht die übrigen drei weniger verlässlich.

Konkret: wer die Umlaufregel ändern will — etwa weil C5 die Zählung "der
wievielte gerade angesteuert ist" anders schneidet —, ändert `umlaufen` und
lässt `voriger` stehen, weil der Kommentar ihm sagt, es gebe nur die eine
Stelle.

## Was zu tun ist

Eines von beiden:

1. `voriger` über `umlaufen` führen. Das geht: der gesuchte Index ist
   `davor.checked_sub(1)`, und der Rückfall auf den letzten ist
   `umlaufen(treffer, treffer.len().saturating_sub(1))`. Dann stimmt der Satz.
2. Den Satz an `umlaufen` auf das ziehen, was gilt: sie ist die Stelle für den
   Umlauf **nach vorn**, und `voriger` trägt den nach hinten mit `checked_sub`,
   weil er dort mit derselben Rechnung die leere Liste erledigt.

Der erste Weg ist der, den das Modul sonst wählt.

---

Resolved: Am 260810-0919 auf dem ersten der beiden Wege geschlossen — `voriger`
läuft jetzt über `umlaufen` —, aber mit einer anderen Rechnung als der im Befund
vorgeschlagenen.

**Warum nicht die vorgeschlagene Zeile.** Der Befund schlug vor

```rust
davor.checked_sub(1).or_else(|| umlaufen(treffer, treffer.len().saturating_sub(1)))
```

Damit stünde der Satz an `umlaufen` weiterhin nur halb: der Umlauf nach hinten
zielt in dieser Fassung auf `treffer.len() - 1`, und diese Zahl ist die
Umlaufregel selbst. Sie stünde also wieder in `voriger`, und `umlaufen` bliebe
darin auf die Leerlistenprüfung beschränkt. Der Satz behauptet mehr als das.

**Was stattdessen steht.** `umlaufen` rechnet die Stelle im Ring:

```rust
fn umlaufen(treffer: &[Treffer], stelle: usize) -> Option<usize> {
    if treffer.is_empty() {
        return None;
    }
    Some(stelle % treffer.len())
}
```

und `voriger` ist der Schritt zurück, als Schritt um `len - 1` nach vorn:

```rust
umlaufen(treffer, davor + treffer.len().saturating_sub(1))
```

Für `erster_ab` und `naechster` ist die Restrechnung dasselbe wie vorher, weil
ihre Stelle höchstens `len` ist und `len % len` gleich `0`. Alle drei
Richtungen und die leere Liste kommen jetzt aus derselben Funktion, und der Satz
an `umlaufen` trägt: er nennt den Ring ausdrücklich und warnt davor, ihn auf
`if stelle < len` zurückzukürzen, weil das `voriger` seinen Umlauf nähme.

Kein Verhalten geändert. Die bestehende Probe
`die_auswahl_laeuft_in_beide_richtungen_um` läuft unverändert grün; dazu ist
`ein_einziger_treffer_wird_aus_jeder_richtung_wieder_erreicht` neu, der scharfe
Fall der Ringrechnung mit dem Summanden 0.

Geändert: ausschließlich `crates/krk-core/src/text/suche.rs`. Abgenommen mit
`cargo build/test/clippy/fmt --workspace`, alle vier auf 0.
