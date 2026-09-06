Der Doc-Kommentar an Grund::einzelheit nennt „die vier übrigen Gründe"; es sind drei

---

`Grund` (`crates/krk-core/src/ablage/mod.rs:163`) trägt seit dieser Runde vier Varianten:
`NichtLesbar`, `Beschaedigt`, `NichtAnlegbar` und neu `ZuGross`. Der Doc-Kommentar an
`Grund::einzelheit` (`:215`) schreibt:

> **`Cow` und nicht `&str`, seit `Grund::ZuGross` dazugekommen ist.** … Die vier uebrigen
> Gruende reichen ihren Text weiter und kosten weiterhin keine Kopie.

Übrig sind drei. Die Zahl stammt aus der Zeit vor der neuen Variante, in der `einzelheit`
über drei Werte lief; mit `ZuGross` sind es vier insgesamt und drei ohne Kopie.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten.

**Warum es trotzdem aufgeschrieben ist.** In diesem Projekt sind die Zahlen in Kommentaren
die Form, in der eine Vollständigkeit nachgelesen wird — `beschreibung()` zwei Zeilen
darüber sagt richtig „ein **fünfter** Grund haelt den Bau an", und beide Sätze stehen im
selben `impl`. Eine falsche und eine richtige Zählung nebeneinander lassen den Leser
raten, welche gilt. Der Spec dieser Runde hat aus demselben Grund eine Fehlzählung des
Shapers eigens berichtigt („sechster Aufrufer" statt „zweiter").

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Die Abweichung selbst — `einzelheit` gibt `Cow<'_, str>` statt `&str` zurück — ist geprüft
  und trägt: `ZuGross` hält eine Zahl, und der Satz entsteht beim Lesen, damit
  `EDITORGRENZE` nicht ein zweites Mal im Baum steht.

---
Resolved: Die Zahl ist nicht berichtigt, sondern ersetzt. Der Doc-Kommentar an `Grund::einzelheit` (`crates/krk-core/src/ablage/mod.rs`) sagt jetzt "Jeder andere Grund traegt seinen Text schon und reicht ihn weiter; die Kopie kostet allein `Grund::ZuGross`" und schreibt dazu, dass das eine Regel und keine Zaehlung ist. Damit bleibt er bei einem fuenften Grund richtig. Aus demselben Grund ist der Nachbarsatz an `Grund::beschreibung` von "ein fuenfter Grund haelt den Bau an" auf "ein weiterer Grund" gezogen; er war heute richtig und waere es mit der naechsten Variante nicht mehr gewesen.
