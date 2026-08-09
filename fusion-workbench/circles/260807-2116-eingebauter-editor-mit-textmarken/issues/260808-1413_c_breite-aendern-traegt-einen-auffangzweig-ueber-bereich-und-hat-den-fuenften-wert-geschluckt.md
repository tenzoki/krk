# `breite_aendern` trägt einen Auffangzweig über `Bereich` und hat den fünften Wert geschluckt

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht von Turn 1 der Editor-Runde
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs:335-357`
**Cross-references:** Plan S13 (Abnahmekriterium), Plan Befund 6, `CLAUDE.md` Abschnitt "Was man nicht sieht, wenn man es nicht weiß"

---

## Der Befund

`crates/krk-ui/src/fenstermodell.rs:336-340`:

```rust
if bereich.ist_beweglich() {
    let anderer = match bereich {
        Bereich::Links => Bereich::Rechts,
        _ => Bereich::Links,
    };
```

Der Auffangzweig ist vorbestehend; er stand vor dieser Runde schon so da
(`git show 4e86c02:crates/krk-ui/src/fenstermodell.rs:285`). Diese Runde hat
`Bereich` um einen fünften Wert erweitert, und dieser Zweig hat ihn **stumm**
aufgenommen. Der Übersetzer hat an dieser Stelle keine Einordnung verlangt.

## Warum das trotz der richtigen Antwort ein Defekt ist

Die Antwort ist heute richtig: der Aufruf steht hinter `if bereich.ist_beweglich()`,
und `Bereich::Editor` ist nicht beweglich (`fenstermodell.rs:156-161`), erreicht
den `match` also nicht. Der Defekt liegt in der Zusage, nicht im Ergebnis.

**Erstens** widerspricht die Stelle dem Absatz, den dieselbe Runde 180 Zeilen
darüber neu geschrieben hat (`fenstermodell.rs:145-151`):

> **Eine vollständige Fallunterscheidung und kein `matches!`.** Bis zur
> Editor-Runde stand hier `matches!(self, Links | Rechts)`, und ein neuer Bereich
> wäre still als unbeweglich durchgegangen — mit der richtigen Antwort, aber aus
> dem falschen Grund. Sie soll aus einer Zeile kommen, die jemand geschrieben
> hat, und nicht aus einem Rückfall. **Ein fünfter Bereich hält jetzt den Bau
> an**, wie es die drei übrigen vollständigen Fallunterscheidungen dieses
> Projekts auch tun.

Genau das Argument — richtige Antwort, falscher Grund — trifft auf
`breite_aendern` unverändert zu, und dort ist es nicht angewandt worden.

**Zweitens** trägt das Abnahmekriterium von S13 für diese Stelle nicht. Es lautet:

> `cargo build --workspace` übersetzt, was belegt, dass alle acht erschöpfenden
> Fallunterscheidungen über `Bereich` vollständig sind.

Ein grüner Bau belegt das für acht Stellen und schweigt über die neunte
(`sichtbar_im`, inzwischen im Plan nachgetragen) und über diese zehnte. Die
Zählung im Plan führt `breite_aendern` gar nicht, weil sie nur die erschöpfenden
Fallunterscheidungen zählt und diese keine ist.

**Drittens** ist die Stelle nicht harmlos, sobald sich `ist_beweglich` ändert.
Käme in einer späteren Runde ein zweiter beweglicher Bereich hinzu, gäbe
`_ => Bereich::Links` ihm das linke Dateifenster als Gegenüber, ohne dass
irgendetwas nachfragt. Der Zweig ist heute durch eine Bedingung an einer anderen
Stelle abgesichert, nicht durch sich selbst.

## Was zu tun ist

Den `match` auf alle fünf Werte ausschreiben und für die drei festen Bereiche
sagen, was gilt — etwa `unreachable!` mit dem Grund, oder eine Umformung, die den
Partner aus `Fensterseite::andere()` holt, statt ihn ein zweites Mal aus
`Bereich` abzuleiten. Der zweite Weg ist der kleinere: `Fensterseite::andere()`
gibt es bereits (`fenstermodell.rs:238`), und `Bereich::von_seite` führt zurück
(`fenstermodell.rs:93-98`); die Aufzählung "welcher Bereich ist das Gegenüber"
entfiele dann ganz, statt richtig geschrieben zu werden.

---
Resolved: Am 260809 in `crates/krk-ui/src/fenstermodell.rs` geschlossen, auf dem
zweiten der beiden vorgeschlagenen Wege — dem kleineren. Der `match` ueber
`Bereich` ist nicht richtig ausgeschrieben, sondern **entfallen**:

```rust
if let Some(seite) = bereich.seite() {
    let anderer = Bereich::von_seite(seite.andere());
```

Dazu kam `Bereich::seite() -> Option<Fensterseite>`, die Umkehrung von
`Bereich::von_seite` und eine vollstaendige Fallunterscheidung ohne
Auffangzweig. `ist_beweglich` zaehlt seitdem nicht mehr selbst auf, sondern
lautet `self.seite().is_some()`: beweglich ist ein Bereich genau dann, wenn er
ein Dateifenster ist, und das ist der Grund und nicht nur die Beobachtung. Die
Aufzaehlung "welche Bereiche sind Dateifenster" steht damit an einer Stelle
statt an dreien, und die Frage "welcher Bereich ist das Gegenueber" wird nicht
mehr gestellt.

Zwei Proben halten es fest:
`beweglich_ist_genau_ein_dateifenster_und_die_zuordnung_laeuft_in_beide_richtungen`
(`seite` und `von_seite` passen zusammen, `ist_beweglich` folgt aus `seite`) und
`ein_fester_bereich_aendert_nur_seine_eigene_breite` (`breite_aendern` auf
`Bereich::Editor` zieht kein Dateifenster mit und haelt das Mindestmass).
