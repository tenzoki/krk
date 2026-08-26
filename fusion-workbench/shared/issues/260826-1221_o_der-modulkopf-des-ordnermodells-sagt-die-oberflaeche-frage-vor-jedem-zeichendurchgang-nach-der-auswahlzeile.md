Der Modulkopf des Ordnermodells sagt, die Oberflaeche frage vor jedem Zeichendurchgang nach der Auswahlzeile

---

`crates/krk-core/src/verzeichnis/modell.rs:14-16` behauptet ein Aufrufmuster, das der Baum
nicht traegt: „die Oberflaeche fragt vor jedem Zeichendurchgang mit
[`Ordnermodell::auswahl_zeile`] nach der Zeile, in der der ausgewaehlte Eintrag gerade steht."
Die vier Ruferstellen sind alle ereignisgetrieben; keine liegt in einem Zeichendurchgang.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `crates/krk-core/src/verzeichnis/modell.rs:13-18`
**Tree state:** `004ff72`
**Domain:** code

## Die vier Rufer, nachgezaehlt

| Stelle | Anlass |
|---|---|
| `krk-ui/src/appkit/tabelle.rs:2112` | `nach_filteraenderung`, also je Filteraenderung |
| `krk-ui/src/appkit/tabelle.rs:2211` | nach dem Setzen einer Zeile |
| `krk-ui/src/appkit/tabelle.rs:2223` | `auswahl_setzen`, also je Auswahlwechsel |
| `krk-ui/src/kommandos/operationen.rs:188` | beim Absetzen eines Operationsbefehls |

Der `#[cfg(test)]`-Block von `tabelle.rs` beginnt erst bei Zeile 4956; alle vier sind
Produktivcode. Keine von ihnen steht in `tableView:objectValueForTableColumn:row:` oder einem
anderen Zeichenweg.

## Warum die falsche Aussage teuer ist

`auswahl_zeile` (`modell.rs:607-609`) geht ueber `zeile_von` (`:577-581`), und das ist eine
lineare Suche ueber `sichtreihenfolge`:

```rust
pub fn zeile_von(&self, eintragsindex: u32) -> Option<usize> {
    self.sichtreihenfolge
        .iter()
        .position(|index| *index == eintragsindex)
}
```

Bei 100.000 Eintraegen ist das ein Gang ueber 100.000 `u32`. **Je Ereignis** ist das nichts;
**je Zeichendurchgang** waere es der teuerste Posten des Zeichnens und stuende gegen L3 und
L10. Der Satz laedt damit zu genau einer von zwei falschen Handlungen ein: entweder rechnet
jemand die Kosten in eine Messrunde ein, die es nicht gibt, oder er baut einen Zwischenspeicher
gegen eine Last, die nirgends anfaellt — und dieser Modulbaum haelt einen zweiten Wahrheitsort
ueber die Auswahl ausdruecklich fuer den Fehler, den er vermeidet (`modell.rs:13-18` selbst).

## Richtung

Der tragende Teil des Absatzes stimmt und bleibt: die Auswahl haengt am Eintragsindex und
nicht an der Zeilennummer, und die Oberflaeche rechnet sie bei Bedarf um. Zu berichtigen ist
allein „vor jedem Zeichendurchgang". Eine Formulierung, die nicht mit dem naechsten Rufer
wieder falsch wird, nennt den Anlass statt der Haeufigkeit, etwa „die Oberflaeche rechnet sie
um, wenn sie die Auswahl setzt oder die Sicht sich geaendert hat" — dieselbe Bauform, in der
`filter.rs` und `sys.rs` ihre Ruferzahlen inzwischen durch das Zaehlkommando ersetzt haben.
