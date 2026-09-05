CLAUDE.md nennt zwei Stellen, an denen der Nutzer die Betriebsregel liest — mit der Anleitung im Paket sind es drei

---
`CLAUDE.md`, Abschnitt „Was man nicht sieht, wenn man es nicht weiß", erster Absatz: „Sie steht seit der Runde 15 an zwei Stellen, an denen der Nutzer sie beim Installieren liest, und beide sind für ihn und nicht für einen Agenten geschrieben: in den ersten dreißig Zeilen der `README.md` und im festen Text jeder Releaseseite".

Seit dem 260905 reist `HowTo.md` im Releasepaket mit (`xtask/src/veroeffentlichung.rs`, `paket_stellen`), und ihr Abschnitt „Vor dem Aktualisieren" sagt dieselbe Regel: die neue Fassung über die alte kopieren, die alte nicht vorher löschen. Sie erreicht den Nutzer damit genau im Augenblick des Installierens, und das ist die Eigenschaft, mit der der Satz seine zwei Stellen begründet.

**Der Satz ist damit für den Zweck falsch, für den er dasteht.** Er sagt einem Schreiber, wo er die Regel nachzieht, wenn sie sich ändert. Wer ihm folgt, lässt die dritte Stelle stehen, und sie ist die einzige der drei, die der Nutzer nach der Installation noch neben der App liegen hat.

**Abnahmetest:** Der Absatz nennt jede Stelle, an der der Nutzer die Betriebsregel beim Installieren liest, oder er nennt keine Zahl und den Weg, sie zu finden. Eine Änderung der Regel an einer Stelle ist von dort aus vollständig nachzuziehen.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
