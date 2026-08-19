Ein Klick in das Dateifenster holt den Fokus nicht, der Rahmen bleibt stehen
---
Wer bei Fokus im Editor, in der Vorschau oder in der Leiste in ein Dateifenster klickt,
bekommt den Fokus nicht dorthin. Der Fokusrahmen bleibt um den alten Bereich stehen. Vom
Nutzer am 260819 beobachtet und auf Nachfrage als „Rahmen bleibt am alten Bereich"
eingegrenzt, also nicht als Anzeigefehler.
---
**Die Zusage, gegen die das steht.** Ein Klick auf einen der bedienbaren Bereiche —
Dateifenster, Vorschau, Editor, Lesezeichen- und Geräteleiste — soll den Fokus dorthin
legen. Der Nutzer hat das am 260819 als Erwartung formuliert; eine Festlegung dazu steht
bisher nirgends im Baum.

**Was der Baum heute tut.** KRK führt zwei verschiedene Größen, und der Klick bedient nur
eine:

- `aktiv` sagt, welches der beiden Dateifenster gemeint ist. Gesetzt wird es über
  `DateifensterQuelle::angefasst()` (`crates/krk-ui/src/appkit/tabelle.rs:3146`), das genau
  zwei Rufer hat: `tableView:shouldSelectRow:` (`:3564`) und die Tableiste (`:4406`). Der
  Rückruf landet auf `Anwendungsdelegierter::aktives_setzen` (`anwendung.rs:1159`).
- `Fokus` sagt, wohin die Tasten gehen. Gesetzt wird er allein über die Überschreibung von
  `makeFirstResponder:` in `appkit/fenster.rs:226`, deren Modulkopf ausdrücklich festhält:
  „Es gibt keine zweite Tür."

Keiner der beiden Rufer von `angefasst()` rührt den Ersthelfer an. Dass der Rahmen stehen
bleibt, heißt: `makeFirstResponder:` findet beim Klick nicht statt.

**Warum das eine offene Frage und keine fertige Diagnose ist.** Eine `NSTableView` wird beim
Klick von sich aus Ersthelfer; das ist ihr Standardverhalten. Etwas hindert sie in diesem
Baum daran, und was, ist nicht ermittelt. Die naheliegenden Kandidaten sind nicht geprüft:
ein gesetztes `refusesFirstResponder`, ein Container, der den Rang abfängt, oder eine
Ansicht, die die Zeile trägt und selbst nicht annimmt.

**Zwei Belege dafür, dass es ein Muster ist und kein Einzelfall.**

1. **Die Vorschau hat die Fähigkeit von Hand bekommen.** `crates/krk-ui/src/appkit/vorschau.rs:243`
   trägt `acceptsFirstResponder`, und `:250` ein eigenes `mouseDown:`, das
   `makeFirstResponder` ruft. Wäre das Standardverhalten überall wirksam, hätte niemand das
   schreiben müssen.
2. **Die Lesezeichen- und Geräteleiste hat davon nichts.** In `crates/krk-ui/src/appkit/leiste.rs`
   findet sich weder `acceptsFirstResponder` noch `mouseDown:` noch ein Ruf auf
   `makeFirstResponder`. Sie ist damit derselbe Fall wie das Dateifenster; der Nutzer hat sie
   in seiner Aufzählung mit genannt.

Der Editor ist vermutlich unauffällig, weil eine `NSTextView` den Rang von sich aus nimmt.
Geprüft ist auch das nicht.

**Was zu klären ist, bevor etwas gebaut wird.**

- Warum die `NSTableView` des Dateifensters den Rang beim Klick nicht nimmt.
- Ob die Leiste denselben Grund hat oder einen eigenen.
- Ob die Lösung an jede Ansicht einzeln gehört, wie es die Vorschau vormacht, oder ob es
  eine gemeinsame Stelle gibt. Vier Bereiche mit vier eigenen `mouseDown:`-Überschreibungen
  wären vier Wahrheiten über dieselbe Regel.
- Was mit einem Klick auf die freie Fläche unter der letzten Zeile geschieht, wo
  `shouldSelectRow:` nicht feuert.

**Verweise:**
- `crates/krk-ui/src/appkit/fenster.rs` Modulkopf — der eine Weg für jeden Fokuswechsel
- `crates/krk-ui/src/appkit/vorschau.rs:243`, `:250` — der Bereich, der es hat
- `crates/krk-ui/src/appkit/leiste.rs` — der Bereich, der es nicht hat
- `crates/krk-ui/src/appkit/tabelle.rs:3146`, `:3564`, `:4406` — `angefasst()` und seine zwei Rufer
- `crates/krk-ui/src/kommandos/fokus.rs:334` — `wirkt()`, das den Fokus gegen den Wirkungsbereich liest

---
Resolved: Die Praemisse dieses Datensatzes ist widerlegt und der Datensatz damit gegenstandslos. Der Nutzer hat am 260819 nachgemessen: ein Klick auf eine **Zeile** des Dateifensters holt den Fokus sehr wohl, und die Fokusbefehle der Tastatur wandern korrekt. Betroffen ist allein der Klick auf die **freie Flaeche** unter der letzten Zeile, und allein im Dateifenster; Vorschau, Editor und Leiste sind unauffaellig. Der Analyst hatte diese Lage bereits als eigenen Defekt gefilt, bevor die Messung vorlag: `shared/issues/260819-1043_*_ein-klick-unter-die-letzte-zeile-laesst-das-aktive-dateifenster-stehen-und-malt-den-rahmen-auf-das-andere.md`. Dort wird sie behoben. Aufgeschrieben hatte ich die Beobachtung des Nutzers breiter, als sie war; die drei in diesem Datensatz genannten Ursachenkandidaten sind am Baum ausgeschlossen und die Analyse `shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md` haelt fest, warum ein Nachbau den Fehler nicht reproduzieren konnte.
