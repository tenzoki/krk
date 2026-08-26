Die Besitzregel des Freigabewählers gilt nur in einer der zwei Hüllen, und der Unterschied steht nirgends

---

`crates/krk-ui/src/appkit/teilen.rs` gibt eine Regel aus und hält sie an einer
von zwei Stellen. Die Regel steht am `thread_local!` (`:115-136`):

> Ein `Retained`, das am Ende von [`anbieten`] faellt, nimmt ihm seinen
> Besitzer, und was AppKit mit einem Dialog ohne Besitzer tut, ist keine
> Zusage, auf die sich bauen liesse.

`anbieten` (`:217-227`) hält den Wähler danach fest. `eintrag_anfuegen`
(`:253-262`) baut in derselben Datei einen zweiten Wähler und lässt ihn
augenblicklich fallen:

```rust
let eintrag = auswaehler_bauen(pfade).standardShareMenuItem(mtm);
```

Nach dem Semikolon lebt der `NSSharingServicePicker` nur noch, wenn der
zurückgegebene `NSMenuItem` ihn stark hält. Ob er das tut, steht weder im Kopf
des Systems noch in dieser Datei. `NSMenuItem.target` ist ausdrücklich
**schwach** (`NSMenuItem.h:93`, `@property (nullable, weak) id target;`), und
`representedObject` ist stark (`:98`) — welches von beiden `standardShareMenuItem`
benutzt, sagt Apple nicht.

`inference:` Wahrscheinlich ist es unbedenklich. Der von Apple gezeigte Gebrauch
legt den Wähler in eine lokale Bindung und lässt sie am Ende des Rumpfes
fallen; hielte der Eintrag ihn nicht, wäre das Muster von Apple selbst kaputt.
Verifiziert ist das nicht, und mehr als eine Vermutung ist es damit auch nicht.

**Genau das ist der Befund.** Nicht, dass eine der beiden Stellen falsch wäre,
sondern dass dieselbe Frage in derselben Datei zweimal entgegengesetzt
beantwortet ist und nur eine der beiden Antworten eine Begründung trägt. Wer
die Datei liest, kann nicht entscheiden, ob `eintrag_anfuegen` eine bewusste
Ausnahme oder ein Versehen ist — und beide Auflösungen sind falsch, solange
niemand die Frage beantwortet hat.

---

**Zwei Nebenbefunde an derselben Stelle**

- **Die Reihenfolge in `anbieten` (`:223-225`) kauft nicht, was ihr Kommentar
  ihr zuschreibt.** Er sagt: „Erst zeigen, dann festhalten: die Zuweisung setzt
  den vorigen Dialog ab, und der soll gehen, nachdem der neue steht." Der
  vorige Dialog verliert seinen Besitzer aber unabhängig davon, ob der neue
  schon steht; wäre die Regel am `thread_local!` wörtlich zu nehmen, nähme ein
  zweiter Aufruf einem noch offenen ersten Dialog den Besitzer, und die
  Reihenfolge änderte daran nichts. `inference:` Erreichbar ist das
  vermutlich nicht, weil `showRelativeToRect:` eine Verfolgungsschleife fährt
  und ein zweiter Tastenbefehl währenddessen nicht durchkommt. Ausgeschrieben
  ist diese Voraussetzung nirgends, und sie ist es, die den Fall trägt — nicht
  die Reihenfolge der zwei Zeilen.
- **Freigegeben wird der Wähler nie.** Er bleibt im `thread_local!` stehen,
  nachdem der Nutzer den Dialog geschlossen hat, bis zum nächsten Aufruf von
  `anbieten`; der letzte überlebt das Programm, denn `NSApplication`s
  Beendigung ruft `exit()` und führt keine Rust-`thread_local`-Destruktoren
  aus. Ein Objekt samt seiner `NSURL`-Liste, ohne Wirkung auf den Nutzer. Der
  saubere Weg wäre `NSSharingServicePickerDelegate` mit
  `sharingServicePicker:didChooseSharingService:`, das AppKit auch beim
  Abbrechen mit `nil` ruft (`NSSharingService.h:276-279`); das wäre eine
  zweite Berührung mit dem Protokoll und ist für sich genommen keinen Schritt
  wert.

**Was zu tun ist**

Die Frage einmal beantworten und die Antwort in den Modulkopf schreiben, in
einem der beiden Zuschnitte:

1. Der Menüeintrag hält den Wähler — dann sagt `eintrag_anfuegen` das in einem
   Satz, mit der Stelle, an der es nachgelesen ist, und die Regel am
   `thread_local!` bekommt ihre Grenze („gilt für den gezeigten Dialog, nicht
   für den Menüeintrag").
2. Es ist nicht belegbar — dann geht der Wähler des Menüweges denselben Weg wie
   der des Tastenweges und wird ebenfalls festgehalten, und die Regel gilt ohne
   Ausnahme.

**Kontext**

- Der Befund ist heute keine sichtbare Fehlfunktion und wäre eine stille: ein
  Teilen-Eintrag, der aufklappt und nichts tut. Am Bündel fällt so etwas nur
  auf, wenn jemand ihn wirklich anklickt.
- Beide Hüllen tragen ausdrücklich keine Probe, und das ist begründet
  (`teilen.rs`, „Diese beiden Huellen tragen keine Probe"). Der Befund ist
  deshalb nur durch Lesen oder am Bündel zu haben.
- Gefunden bei der Durchsicht von Turn 1 der Runde 6; nicht behoben.

Also seen: 260826-1338 by coderev — gilt am Stand `7ac511a` unverändert: `eintrag_anfuegen` (`crates/krk-ui/src/appkit/teilen.rs:270`) lässt den Wähler weiter nach `standardShareMenuItem` fallen, `anbieten` (`:235-237`) hält seinen im `thread_local!`. Seit der Runde 17 läuft dieser zweite Weg für jedes Kontextmenü der Dateiliste (`tabelle.rs:1236-1243`), neben Zip, Unzip und Finder; der Eintrag ist damit der häufiger benutzte der beiden, und die Frage ist weiter unbeantwortet.
