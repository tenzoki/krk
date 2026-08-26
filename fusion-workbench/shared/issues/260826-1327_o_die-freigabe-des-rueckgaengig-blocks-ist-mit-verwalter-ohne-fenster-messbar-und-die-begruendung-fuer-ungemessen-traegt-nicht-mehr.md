Die Freigabe des Rueckgaengig-Blocks ist mit verwalter_ohne_fenster messbar, und die Begruendung fuer "ungemessen" traegt nicht mehr

---

`Stapellast` traegt seine Bytes in `Drop` ab, und ob `NSUndoManager` den Block je freigibt, ist laut
Doc-Kommentar ungemessen, weil eine Messung "einen `NSUndoManager`, also einen `MainThreadMarker`"
braeuchte. Dasselbe Pruefmodul baut seit dem 260810 sechs Proben auf genau so einem Verwalter
(`verwalter_ohne_fenster`). Die Messung kostet eine siebte Probe und kein neues Pruefziel.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

`crates/krk-ui/src/appkit/editor.rs:906-915`, am Kopf von `Stapellast`:

> Dass der Verwalter den Block festhaelt und ihn mit der Handlung wieder freigibt, ist die Regel von
> Objective-C … nachgemessen ist sie hier **nicht**. Eine Messung braeuchte einen `NSUndoManager`,
> also einen `MainThreadMarker`, und darueber steht eine offene Nutzerentscheidung an den vier
> Proben, die ihn heute behaupten.

Und im selben Pruefmodul: `verwalter_ohne_fenster()` (:3874-3876) liefert genau diesen Verwalter, und
sechs Proben nehmen ihn (:3923, :3953, :3997, :4073, :4163 sowie `wert_anmelden` :4131). Die Probe
`eine_anmeldung_nach_dem_leeren_steht_im_stapel` (:4073-4123) faehrt bereits `removeAllActions`,
`undo` und einen Umlauf der Laufschleife auf diesem Verwalter.

Der geschlossene Datensatz
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1341_c_die-freigabe-des-angemeldeten-rueckgaengig-blocks-ist-geschlossen-und-nicht-gemessen.md`
ist "als Lage angenommen" geschlossen; die Lage hat sich mit `verwalter_ohne_fenster` geaendert.

## Was zu messen ist

Ein `Rc<Cell<usize>>`, eine `Stapellast::angemeldet(punkt, &zaehler)` in einem `RcBlock`, der Block ueber
`registerUndoWithTarget_handler` angemeldet, dann je einmal:

1. `removeAllActions` — der Zaehler faellt auf 0 (Weg 3 aus dem Kopf :892-895);
2. `undo` — der Zaehler faellt (Weg 1);
3. eine zweite Anmeldung nach einem `undo` raeumt den Wiederherstellungsstapel — der Zaehler faellt
   (Weg 2);
4. `drop(verwalter)` — der Zaehler faellt (Weg 4).

Jeder Weg ggf. nach einem `runMode_beforeDate`, wie `:4103` es schon tut, weil der Kopf (:924-927) einen
Freigabeverbund fuer moeglich haelt. Faellt der Zaehler nicht, greift das Budget bei jedem Umbau
(:917-922) — das ist der Preis, den der Kopf heute nur als Annahme benennt.

## Umfang

`krk-ui`, `appkit/editor.rs`, Pruefmodul.
