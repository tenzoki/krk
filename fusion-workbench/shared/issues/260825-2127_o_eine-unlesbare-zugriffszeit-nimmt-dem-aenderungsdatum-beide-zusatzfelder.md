# Eine unlesbare Zugriffszeit nimmt dem Änderungsdatum beide Zusatzfelder, und zwar stumm

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `crates/krk-core/src/operation/zippen.rs:663-704` (`zeit_uebernehmen`), `:730-734` (`epochensekunden`); `crates/krk-core/src/operation/entpacken.rs:361-374` (`eintragszeit`); `Cargo.toml:176-201` (die Messtabelle zu den zwei Zeitfeldern)

---

## Was ist

`zeit_uebernehmen` schreibt die zwei Zusatzfelder `0x5455` und `0x5855` nur dann, wenn
**beide** Zeitpunkte in vier Byte passen:

```rust
if let (Some(geaendert), Some(gelesen)) = (epochensekunden(geaendert), epochensekunden(gelesen))
```

`gelesen` ist die Zugriffszeit. Der Kommentar zwei Zeilen darüber sagt: „Die Zugriffszeit ist
die Zugabe und nicht der Gegenstand" — und die Bedingung lässt die Zugabe den Gegenstand
verhindern. Eine Datei, deren Zugriffszeit vor 1970 oder nach 2106 liegt, während ihr
Änderungsdatum tadellos ist, bekommt **kein** Zusatzfeld.

Die Folge ist genau der Zustand, den die Messtabelle in der Wurzel-`Cargo.toml` als
unzureichend ausweist: allein das MS-DOS-Feld. `unzip` liefert dann die Sekunde im
Zweisekundenraster, `ditto(1)` legt eine Winterdatei im Sommer eine Stunde daneben ab. Und es
geschieht **ohne eine Zeile in der Abschlussliste**: die zwei Meldungen darüber hängen am
Änderungsdatum, nicht an diesem Zweig.

Dieselbe Verkopplung trifft den Fall „Änderungsdatum nach 2106": dort steht immerhin schon
eine Zeile wegen des MS-DOS-Feldes, also fällt er nicht still aus.

## Warum das zählt

Der Fall ist selten und nicht ausgedacht: eine Zugriffszeit von 0 oder ein Wert jenseits von
2106 entsteht durch fremde Werkzeuge, durch Netzdateisysteme und durch von Hand gesetzte
Zeiten. Der Schaden ist klein, aber er ist unsichtbar, und die ganze Arbeit von Schritt 3
besteht darin, dass er nicht mehr unsichtbar ist.

## Was zu tun wäre

Die Zeile hält sich schon selbst die Antwort hin. Die Zugabe fällt auf den Gegenstand zurück,
so wie zwei Zeilen darüber schon für eine **fehlende** Zugriffszeit:

```rust
let gelesen = epochensekunden(gelesen).or(epochensekunden(geaendert));
```

und die Bedingung hängt allein an `epochensekunden(geaendert)`. Damit gilt der Satz aus dem
Kommentar in beiden Lagen, in denen die Zugriffszeit nichts hergibt, statt nur in einer.
Fällt auch das Änderungsdatum aus dem Bereich, bleibt es bei der heutigen Auskunft.

Eine Probe dazu ist billig: eine Quelle, deren Zugriffszeit auf 1969 gesetzt ist, muss ihre
zwei Zusatzfelder trotzdem tragen.

**Schwere:** gering. Ein seltener Randfall, aber ein stummer.

**Gefunden:** coderev, bei der Durchsicht der Runde 18 gegen `20eccd4..8478753`.
