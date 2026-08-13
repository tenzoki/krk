Ein Kommentar in blaetter/mod.rs nennt eine Taste-Variante, die es nicht gibt

---

`crates/krk-ui/src/appkit/blaetter/mod.rs:401` beginnt mit „Auch `Taste::Keine` wird gesetzt
und nicht ausgelassen". Die Aufzählung `Taste` (`blaetter/mod.rs:276-285`) führt vier Werte,
und `Keine` ist keiner davon: `Eingabe`, `EingabeMitBefehl`, `EingabeMitWahl`, `Escape`. Jeder
dieser vier trägt ein Zeichen, es gibt also keinen Wert, für den „auslassen" überhaupt eine
Möglichkeit wäre.

---

**Schwere:** niedrig (kein falsches Verhalten; ein Kommentar zeigt auf eine Variante, die die
Aufzählung nicht führt)
**Gefunden:** planner, beim Zuschnitt der Runde 7 am 260813-0159
**Betroffen:** `crates/krk-ui/src/appkit/blaetter/mod.rs:401-404`
**Domain:** code

## Warum der Satz trotzdem etwas Richtiges meint

Die Begründung darunter stimmt und ist wertvoll: `NSAlert` gibt der ersten Schaltfläche von
sich aus die Eingabetaste, und `setKeyEquivalent` muss deshalb an **jeder** Schaltfläche
laufen, auch dort, wo der Aufrufer keine Taste bestellt hat. Verloren ist allein der
Bezugspunkt. Vermutlich trug die Aufzählung einmal einen fünften Wert `Keine` mit leerem
Zeichen; heute übernimmt `Taste::Eingabe` diese Rolle stillschweigend.

## Der Bezug zur Runde 7

Die Runde 7 fasst diese Stelle an: die Schaltfläche „Fertig" der Belegungsansicht zieht nach
der Empfehlung des Datensatzes
`shared/decisions/260813-0053_o_welche-tasten-behalten-die-schaltflaechen-der-belegungsansicht-wenn-jedes-zeichen-sucht.md`
von `Taste::Eingabe` auf `Taste::EingabeMitBefehl` um, und `Taste::EingabeMitBefehl` hat bis
dahin keinen Benutzer. Wer die Zeile ändert, liest den Kommentar darüber.

## Ein Weg

Den Satz auf einen Wert umstellen, den die Aufzählung führt, etwa: „Auch die erste
Schaltfläche bekommt ihre Taste ausdrücklich gesetzt und nicht ausgelassen." Damit trägt der
Absatz seine Begründung weiter und zeigt auf nichts Verschwundenes mehr.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. Der Satz in `crates/krk-ui/src/appkit/blaetter/mod.rs` heisst jetzt „Jede Schaltflaeche bekommt ihre Taste ausdruecklich gesetzt und keine ausgelassen" und zeigt damit auf nichts Verschwundenes mehr. Die Begruendung darunter — `NSAlert` gibt der ersten Schaltflaeche von sich aus die Eingabetaste, `setKeyEquivalent` laeuft deshalb an jeder — steht unveraendert; verloren war allein der Bezugspunkt. Die Aufzaehlung `Taste` ist nicht angefasst.

---
Abgleich 260813-0644 (reconciler): **Die Behebung haelt.** Der Satz an
`crates/krk-ui/src/appkit/blaetter/mod.rs:401-404` nennt keine `Taste::Keine` mehr, und die
Aufzaehlung `Taste` ist nicht angefasst.

Eine Nebenbehauptung stimmt woertlich nicht: die Begruendung darunter steht nicht
„unveraendert", ein Wort ist mitgewandert („ohne das Loeschen" heisst jetzt „ohne das
Ueberschreiben", Commit `dff167a`). Die Aussage ist dieselbe geblieben.
