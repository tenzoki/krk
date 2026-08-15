`wird_ersthelfer` nimmt das Ordnerzeichen weg, bevor die Oberklasse antwortet, und behandelt ihr Nein nicht

---

`Namensfeld::wird_ersthelfer` (`tabelle.rs:2879-2889`) setzt `stringValue` auf den Namen ohne
Zeichen und ruft **danach** die Fassung der Oberklasse. Die Reihenfolge ist richtig begründet
und gemessen. Unbehandelt bleibt der zweite Ausgang: liefert
`[super becomeFirstResponder]` `false`, beginnt keine Bearbeitung, das Zeichen ist trotzdem
weg, und nichts holt es zurück — die Zeile steht bis zum nächsten Zeichendurchgang ohne ihr
Kennzeichen da.

---

**Schwere:** niedrig, und der Befund ist `inference:` und nicht gemessen. Für ein
bearbeitbares Feld in einem Schlüsselfenster liefert `becomeFirstResponder` „ja"; ein Weg zum
Nein ist in diesem Baum nicht gezeigt. Die Fallunterscheidung ist trotzdem unvollständig, und
die Projektregel dazu ist ausdrücklich: eine Fallunterscheidung ist disjunkt und vollständig,
sonst ist die Frage falsch geschnitten.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:2879-2889`
**Domain:** code

## Warum der offensichtlichere Haken hier nicht hilft

`-[NSTextField textShouldBeginEditing:]` (`NSTextField.h:34`, seit 10.0) sieht wie die
genauere Tür aus, weil sie erst fällt, wenn wirklich bearbeitet wird. Sie hilft nicht: `NSText`
stellt diese Frage erst beim ersten **Ändern** des Textes, nicht beim Erscheinen des
Feldeditors — dieselbe Messung, an der die Delegiertenfassung
`control:textShouldBeginEditing:` gescheitert ist. Der Schrägstrich stünde dann bis zum ersten
Tastendruck im Editor. `becomeFirstResponder` bleibt die richtige Stelle.

## Vorschlag

Den Rückgabewert der Oberklasse auswerten und bei `false` den vorherigen Text
wiederherstellen — die Zeichenkette liegt in `anzeige` bereits vor, es kostet einen
`else`-Zweig und keine neue Regel:

```
let ergebnis: bool = unsafe { msg_send![super(self), becomeFirstResponder] };
if !ergebnis { self.setStringValue(&NSString::from_str(&anzeige)); }
ergebnis
```
