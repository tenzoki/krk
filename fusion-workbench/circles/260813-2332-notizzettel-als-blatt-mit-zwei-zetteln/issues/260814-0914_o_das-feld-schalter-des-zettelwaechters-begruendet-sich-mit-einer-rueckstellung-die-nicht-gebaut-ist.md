Das Feld `schalter` des Zettelwächters begründet sich mit einer Rückstellung, die nicht gebaut ist

---

`ZettelwaechterIvars::schalter` (`crates/krk-ui/src/appkit/blaetter/zettel.rs:157`) trägt
diesen Doc-Kommentar:

> Der Tabschalter, damit ein abgewiesener Wechsel die Anzeige nicht stehen lässt, wo sie
> nicht hingehört.

Eine solche Rückstellung gibt es nicht. `Zettelwaechter::tab_gewechselt` (`:248`) liest den
Schalter mit `selectedSegment()` und ruft an keiner Stelle `setSelectedSegment`. In den zwei
Zweigen, in denen der Wechsel abgewiesen wird — `zettel_an_stelle` liefert `None`, oder
`ivars.tabklick` steht auf `None` —, kehrt die Methode mit `return` zurück und lässt den
Schalter dort, wo der Klick ihn hingestellt hat.

---

**Schwere:** niedrig. Beide Abweisungszweige sind im heutigen Baum unerreichbar:
`NSSegmentedControl` liefert `-1` nur, solange nichts ausgewählt ist, und ein Klick wählt
aus; `tabklick` ist zwischen `Zettelwaechter::neu` und dem Setzen des Rückrufs in `zeigen`
für die Dauer weniger Zeilen leer, in denen der Schalter noch in keinem Fenster hängt.

**Der Wert des Feldes selbst ist unbestritten.** `tab_gewechselt` braucht den Schalter, um
die angeklickte Stelle zu lesen — der Absender kommt als `Option<&NSSegmentedControl>`
herein und wird bewusst nicht benutzt. Falsch ist allein die Begründung.

**Zwei Wege heraus, und die Wahl gehört zum Bau.** Entweder der Kommentar sagt, wofür das
Feld wirklich da ist (die angeklickte Stelle lesen, ohne dem Absender zu trauen), oder die
beschriebene Rückstellung wird gebaut — ein `setSelectedSegment` auf den Stand des Modells
in beiden `return`-Zweigen. Das Zweite ist mehr Code für einen Fall, den heute nichts
erreicht; das Erste ist eine Zeile.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Ohne Rückstellung wäre ein wirklich abgewiesener Wechsel eine Auseinanderentwicklung von
  Anzeige und Modell: der Tab zeigte den einen Zettel, die Textfläche und
  `Zettelmodell::offener` den anderen, und das Getippte ginge in den falschen.
