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
