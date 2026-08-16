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

---
Resolved: Die Fallunterscheidung ist vollständig (260816, coder),
`crates/krk-ui/src/appkit/tabelle.rs`, `Namensfeld::wird_ersthelfer`.

Gebaut ist nicht der vorgeschlagene `else`-Zweig, sondern ein `match` über beide Größen, weil
es hier zwei gibt und nicht eine: ob die Oberklasse angenommen hat, und ob **diese Methode**
überhaupt etwas weggenommen hatte. Eine Datei trug nie ein Zeichen, und für sie gäbe es
nichts zurückzuholen; ein `else`-Zweig allein am Rückgabewert schriebe ihr die unveränderte
Zeichenkette noch einmal an die Zelle. Der Merker `abgelegt` trägt die zweite Größe, und die
drei Zweige lauten:

- `(true, _)` — der Rang ist angenommen, der Feldeditor steht, und da es ein `Namensfeld` nur
  in der beschreibbaren Spalte gibt, ist die Annahme zugleich der Beginn einer Umbenennung.
  Das ist der bisherige Zweig, unverändert im Verhalten.
- `(false, true)` — abgelehnt, und die Methode hatte das Zeichen weggenommen: sie setzt die
  gemerkte Anzeigeform zurück.
- `(false, false)` — abgelehnt, nichts weggenommen, nichts zu tun.

Kein Auffangzweig, wie es die Projektregel verlangt. Kosten: der Merker, drei Zweige und
keine neue Regel. `setStringValue:` und `becomeFirstResponder` standen beide schon in der
Methode, es kommt **keine** AppKit-Methode dazu, und der Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen` im Modulkopf braucht deshalb keine
Zeile.

**Der `inference:`-Charakter des Befunds ist am Code festgehalten und nicht überschrieben.**
Der Kommentar am `match` sagt ausdrücklich, dass kein Weg zum Nein gemessen ist und der Zweig
nicht als Schutz vor einem bekannten Fall dasteht, sondern weil eine Methode aufräumt, was
sie selbst umgestellt hat, sobald der Schritt scheitert, für den sie es umgestellt hat. Er
nennt auch, warum `textShouldBeginEditing:` nicht hilft, damit der geprüfte und verworfene
Kandidat nicht ein zweites Mal geprüft wird.

**Zur Abwägung, die der Auftrag verlangte:** teurer als der Zustand ist die Behebung nicht.
Sie ist kein toter Code im Sinne des Vorbilds aus `260815-2203` — dort war der Fall
**gemessen** unmöglich (AppKit reicht dem Delegierten nie eine Zelle mit offenem Feldeditor),
und ein Schutz hätte eine widerlegte Behauptung über AppKit getragen. Hier ist der Ausgang
nicht als unmöglich gemessen, sondern nur als unwahrscheinlich eingeschätzt, und der
Rückgabewert der Oberklasse steht ohnehin schon in einer Variablen. Der Unterschied ist
"widerlegt" gegen "ungemessen", und er entscheidet verschieden.

**Kein Prüfziel dazu**, aus dem Grund, den der Kopf von `Namensfeld` ausschreibt: der Zweig
braucht ein Fenster und einen Feldeditor, `NSWindow` wirft außerhalb des Hauptfadens, und
`libtest` gibt ihn nicht her
(`shared/issues/260810-1001_*_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`).
Eine reine Regel ist nicht entstanden.

Verification: `make check` — exit 0.
