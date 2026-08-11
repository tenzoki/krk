Ziffern in Dateiliste und Leiste laufen auseinander, und das Änderungsdatum trägt ein Komma

---

Zwei Befunde an derselben Fläche, vom Nutzer am 260811-1730 gemeldet:

1. **Die Zahlen stehen nicht untereinander.** In den Dateilisten, in den Lesezeichen und in
   „Geräte und Orte" laufen Größe und Änderungsdatum von Zeile zu Zeile verschieden breit, weil
   die Ziffern verschieden breit sind. Untereinander gelesen ergibt das keine Spalte.
2. **Das Änderungsdatum trägt ein Komma zwischen Datum und Zeit.** Ein Leerzeichen genügt.

---

**Schwere:** Niedrig — nichts ist falsch, es liest sich schlecht
**Gefunden:** Nutzer
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs`, `crates/krk-ui/src/appkit/leiste.rs`
**Domain:** code

## Befund 1: die Schrift, und was der Nutzer wirklich braucht

Beide Flächen nehmen die Proportionalschrift des Systems:

- `crates/krk-ui/src/appkit/tabelle.rs:1784` — `NSFont::systemFontOfSize(groesse)`
- `crates/krk-ui/src/appkit/leiste.rs:438` — `NSFont::systemFontOfSize(NSFont::smallSystemFontSize())`

In einer Proportionalschrift ist die `1` schmaler als die `8`. Zwei Zeilen mit gleich vielen
Ziffern sind deshalb verschieden breit, und rechtsbündig gesetzte Zahlen wackeln.

**Der Nutzer hat „Type mit fester Laufweite" gesagt, und das ist mehr, als der Zweck verlangt.**
Eine durchgehende Festbreitenschrift setzt auch die **Namen** fest, und Dateinamen lesen sich
darin schlechter — sie sind der Hauptinhalt der Spalte. macOS hat für genau diesen Fall eine
eigene Antwort: `NSFont::monospacedDigitSystemFontOfSize(_:weight:)` liefert die Systemschrift
mit **festbreiten Ziffern** und proportionalen Buchstaben. Namen bleiben lesbar, Zahlen stehen
untereinander.

**Zu entscheiden ist damit:** festbreite Ziffern (die kleinere, zielgenaue Änderung) oder eine
durchgehende Festbreitenschrift (was der Nutzer wörtlich sagte). Wer das anfasst, legt es fest
und schreibt den Grund dazu.

**Die Einrückung ist ein zweiter Teil derselben Sache.** Der Nutzer sagt, Datumswert und Name
müssten „gleich eingerückt" sein. Ob die Spalten heute unterschiedliche Innenabstände tragen, ist
**nicht geprüft**; die Schriftwahl allein löst das nicht, wenn die Zellen verschieden gesetzt
sind. Beim Anfassen mitmessen.

**Die Belegungsansicht ist nicht betroffen** und soll es auch nicht werden — sie hat zwei
Textspalten und keine Zahlen (`belegungsansicht.rs:358-360`).

## Befund 2: das Komma kommt nicht aus KRK

`crates/krk-ui/src/appkit/tabelle.rs:1777-1779`:

```rust
let datumsformat = NSDateFormatter::new();
datumsformat.setDateStyle(NSDateFormatterStyle::ShortStyle);
datumsformat.setTimeStyle(NSDateFormatterStyle::ShortStyle);
```

`NSDateFormatter` verbindet Datum und Zeit mit dem Trenner, den die **Sprachregion** dafür
vorsieht; im deutschen Raum ist das `, `. KRK schreibt das Komma nirgends hin.

**Das macht den Fix zu einer Abwägung und nicht zu einer Streichung:**

- **Ein eigenes Format setzen** (`setDateFormat`) entfernt das Komma sicher — und hebt zugleich
  die Anpassung an die Sprachregion des Nutzers auf. Wer sein System auf `en_US` stellt, bekäme
  dann trotzdem die deutsche Reihenfolge. Dieses Projekt hat für die Sortierordnung dieselbe
  Frage offen (`circles/260802-0842-…/decisions/260806-1730_*_welche-sprache-bestimmt-die-sortierordnung.md`),
  und eine Antwort hier bindet die dort mit.
- **Den Trenner nachträglich ersetzen** — das Komma aus der fertigen Zeichenkette entfernen —
  hält die Sprachregion, ist aber eine Regel über eine Zeichenkette, deren Aufbau KRK nicht
  kennt. Bei einer Region, die kein Komma benutzt, greift sie ins Leere; bei einer, die ein
  Komma **im Datum** führt, greift sie zu weit.
- **Zwei Formatierer nebeneinander**, einer für das Datum und einer für die Zeit, und KRK setzt
  das Leerzeichen selbst. Behält die Sprachregion für beide Teile und macht den Trenner zu KRKs
  Sache. Kostet einen zweiten `NSDateFormatter` — der Kommentar bei `tabelle.rs:1634` begründet
  gerade, warum es **einer** ist und nicht je Zelle einer; zwei feste sind damit vereinbar.

**Empfehlung: der dritte Weg**, weil er als einziger beides hält — die Sprachregion und die
Kontrolle über den Trenner.

## Warum beides zusammen in einem Datensatz steht

Es ist dieselbe Fläche, dieselbe Datei und derselbe Handgriff: wer die Schrift der Tabelle
anfasst, steht drei Zeilen vom Formatierer entfernt. Getrennt abgelegt würden sie zweimal
denselben Kontext aufbauen.
