# Die bestätigende Seite des Eingabewächters liegt fest auf der ersten Schaltfläche

**Datum:** 260817-1242
**Gefunden von:** coder, T1
**Schwere:** Niedrig
**Betrifft:** `crates/krk-ui/src/appkit/blaetter/mod.rs`, `Blatt::zeigen_mit_wahl`
**Baumstand:** der Stand nach T1

## Der Befund

T1 hat die Frage „welche Schaltfläche ist die ungefährliche" auf eine Antwort gebracht
(`blaetter::abbruchstelle`). Die Gegenfrage steht weiter fest im Code: der
`Eingabewaechter` schickt für `bestaetigt == true` unverändert `NSAlertFirstButtonReturn`,
also die **erste** Schaltfläche.

Heute ist das richtig, und der Grund ist nachgezählt: einen Wächter hält nur ein Blatt aus
`Blatt::neu`, dessen Reihenfolge die erste Schaltfläche als die bestätigende festlegt. Die
fünf Rufer von `waechter_anhaengen` und `textfeld_setzen` kommen alle von dort
(`pfadeingabe`, `namenseingabe`, `zeilennummer`, `suche`, `stapelumbenennen`).

Es ist trotzdem dieselbe Sorte Annahme, die der Befund `260817-1106` auf der abbrechenden
Seite gekostet hat: eine Stelle im Rumpf rechnet mit einer Reihenfolge, die ein einzelnes
Blatt anders legen darf. Für die Löschrückfrage wäre die erste Schaltfläche „Abbrechen" —
sie trägt heute keinen Wächter, weil sie kein Feld hat.

## Richtung

Die bestätigende Stelle ist ableitbar, und zwar ohne neue Angabe: der Wächter existiert,
weil der Feldeditor die Eingabetaste verbraucht, und die Eingabetaste gehört der
Schaltfläche mit `Taste::Eingabe`. Eine reine Funktion neben `abbruchstelle`, die die erste
Schaltfläche mit `Taste::Eingabe` liefert und ohne eine solche auf `abbruchstelle`
zurückfällt, macht aus der Annahme eine Ableitung. Zwei Stellen im Baum bekämen damit
Antworten, die zu ihrem Blatt passen statt zu einer Gewohnheit.

Angehängt daran hängt die Frage, die
`260817-1241_o_das-konfliktblatt-gibt-seinem-namensfeld-keinen-eingabewaechter.md` aufwirft:
was ein Wächter in einem Blatt mit vier Antworten bedeuten soll.

---
Abgleich 260817-1833 (reconciler, Baumstand `e313841`): **offen, unverändert.**
`Blatt::zeigen_mit_wahl` liegt an `crates/krk-ui/src/appkit/blaetter/mod.rs:667` und trägt an
`:711` weiter das feste `NSAlertFirstButtonReturn`, während die Gegenseite über
`blaetter::abbruchstelle` (`:416`) berechnet wird.
