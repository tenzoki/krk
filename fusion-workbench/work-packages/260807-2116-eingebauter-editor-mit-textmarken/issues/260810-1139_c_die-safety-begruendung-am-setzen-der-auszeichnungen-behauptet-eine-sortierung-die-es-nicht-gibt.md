# Die SAFETY-Begründung am Setzen der Auszeichnungen behauptet eine Sortierung, die es nicht gibt

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, beim Bau des Fortschreibens der Einfärbung
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs` (`Editorbereich::formatierung_anwenden`), `crates/krk-ui/src/hervorhebung.rs` (`Formatierung::auszeichnungen`)
**Cross-references:** `issues/260810-0054_*_die-einfaerbung-laeuft-mit-0-3-mb-s-und-haengt-beim-tippen-in-grossen-dateien-hinterher.md`

---

## Der Befund

Die SAFETY-Begründung vor `addAttributes:range:` lautete bis zum 260810-1139:

> Der Bereich liegt im Text; die Laenge ist oben geprueft, und die Stellen der
> Formatierung sind aufsteigend und ueberschneidungsfrei.

**Der zweite Halbsatz ist falsch.** Für `Formatierung::einfaerbungen` gilt er, weil
`anfuegen` sie in Textreihenfolge anhängt; für `Formatierung::auszeichnungen` gilt
er nicht. Eine `Auszeichnung::Listenzeile` wird **nach** den Stücken ihrer Zeile
angehängt und beginnt **vor** ihnen. Gemessen an
`- Punkt mit ` + "`Code`" + `` in Markdown:

```
  [ { anfang: 12, laenge:  6, art: FesteSchrift },
    { anfang:  0, laenge: 25, art: Listenzeile  } ]
```

Zwei Aussagen der Begründung sind damit unhaltbar: die Reihenfolge ist nicht
aufsteigend, und die beiden Stellen überschneiden sich.

## Was daran hält und warum

**Der Aufruf ist trotzdem zulässig, und zwar aus einem Grund und nicht aus
Glück.** Die einzige Überschneidung ist die zwischen `Listenzeile` und den
übrigen Auszeichnungen, und die beiden setzen **verschiedene Merkmalsnamen** —
`NSParagraphStyleAttributeName` gegen `NSFontAttributeName`.
`addAttributes:range:` legt zusammen, statt zu ersetzen, also kommt keine
Reihenfolge zum Tragen. `Ueberschrift` und `FesteSchrift` setzen beide die
Schrift und überschneiden einander nie: die Fallunterscheidung in
`hervorhebung::rechnen` fragt die Überschriftsstufe zuerst und die feste Schrift
nur sonst.

Für die Bedingung, die `addAttributes:range:` wirklich stellt — jeder Bereich
liegt innerhalb der Textlänge —, genügt die Längenprüfung, die eine Zeile darüber
steht.

---
Resolved: Die SAFETY-Begründung nennt jetzt die Bedingung, die wirklich gilt, und
sagt ausdrücklich, dass die Auszeichnungen weder aufsteigend noch
überschneidungsfrei sind — mit dem gemessenen Gegenbeispiel und dem Grund, aus dem
das ohne Belang ist. Der Doc-Kommentar von `Formatierung::auszeichnungen` in
`hervorhebung.rs` sagt „in Textreihenfolge" und ist damit ebenfalls ungenau; er
bleibt stehen, weil die Reihenfolge zeilenweise stimmt und die Ausnahme jetzt an
der Stelle steht, an der sie zählt. Das Fortschreiben der Einfärbung verlässt sich
nicht auf die Sortierung: `hervorhebung::teilen` teilt die beiden Listen linear und
nicht über eine Teilungssuche, und der Grund steht dort.
