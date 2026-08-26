Ein Fehler von `ScopeStack::apply` lässt den Zerlegerstand stehen, während ein Fehler von `parse_line` ihn fallen lässt
---
`rechnen` behandelt die zwei Fehler, die `syntect` je Zeile liefern kann, verschieden: nach `Err` aus `parse_line` fällt der Stand und es entsteht kein Haltepunkt mehr (die im Doc-Kommentar begründete Regel), nach `Err` aus `stapel.apply` bricht nur die Schleife über die Stücke ab, und der Stand mit dem halb angewandten Stapel wird als gültig wieder eingesetzt und in Haltepunkte aufgehoben. Der Modulkopf nennt nur den ersten Fall.
---
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>

## Am Baum

`crates/krk-ui/src/hervorhebung.rs:1139-1142`:

```rust
for (stueck, befehl) in ScopeRegionIterator::new(&befehle, zeile) {
    if stand.stapel.apply(befehl).is_err() {
        break;
    }
```

und `:1203` `zerleger = Some(stand);` danach ohne Unterschied. Der Gegenzweig `:1135` `Err(_) => {}` lässt `stand` fallen, und `:993-1001` begründet, warum das die Gleichheit von „von vorn“ und „fortgeschrieben“ trägt.

## Was daran hängt

Die Gleichheit der beiden Wege bleibt hier zufällig gewahrt, weil ein voller Durchgang denselben halben Stapel deterministisch erzeugt. Was nicht gewahrt bleibt: der Doc-Kommentar (`:993`, „Bricht die Kiste an einer Zeile ab, endet damit auch das Aufheben“) gilt nur für einen der zwei Abbrüche, und der Rest der Zeile nach dem `break` bleibt ohne Einfärbung, ohne dass der Stand als beschädigt gilt. Wer den `Err`-Zweig später ändert, findet die zweite Stelle nicht.

`inference:` Ob `apply` mit den eingebundenen Sprachdefinitionen je `Err` liefert, ist wie beim Abbruch von `parse_line` ungemessen; die Prüfung `das_fortschreiben_haelt_nach_einem_abbruch_der_kiste` deckt allein `parse_line`.

## Vorschlag

Beide Fehler gleich behandeln: bei `apply`-Fehler den Stand ebenfalls fallen lassen (`zerleger` bleibt `None`), oder den Doc-Kommentar auf den einen Fall einschränken und begründen, warum der zweite anders ist.
