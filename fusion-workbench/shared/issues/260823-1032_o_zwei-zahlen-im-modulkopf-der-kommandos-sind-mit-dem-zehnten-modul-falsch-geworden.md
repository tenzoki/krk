Zwei Zahlen im Modulkopf der Kommandos sind mit dem zehnten Modul falsch geworden

---

`28cbb7b` zieht im Modulkopf von `crates/krk-ui/src/kommandos/mod.rs` die erste Zahl nach
(„Neun Module" → „Zehn Module") und lässt die zwei Zahlen stehen, die von derselben Menge
abhängen. Beide sind seitdem um eins zu klein.

---

**Am Baum nachgezählt**, gegen `a8be186` und gegen den Stand nach `28cbb7b`.

## Die zwei Stellen

`crates/krk-ui/src/kommandos/mod.rs:45`:

```
//! **`zulaessigkeit` steht vor den sieben uebrigen Tastenbefehlsmodulen, und
```

`crates/krk-ui/src/kommandos/mod.rs:54`:

```
//! **`fokus` steht danach und vor den sechs uebrigen.** Jeder Befehl laeuft durch
```

## Die Rechnung

Der Modulkopf trennt selbst: zehn Module, davon eines (`abwurfregel`) kein Tastenbefehl
(Zeile 8 und Zeile 39-40). Es bleiben neun Tastenbefehlsmodule.

| | vor `28cbb7b` | nach `28cbb7b` | im Text |
|---|---|---|---|
| Module gesamt | 9 | 10 | nachgezogen ✓ |
| davon Tastenbefehl | 8 | 9 | — |
| „vor den … übrigen" (nach `zulaessigkeit`) | 7 | **8** | steht auf sieben ✗ |
| „vor den … übrigen" (nach `fokus`) | 6 | **7** | steht auf sechs ✗ |

Nachzuzählen mit `grep -c '^pub mod' crates/krk-ui/src/kommandos/mod.rs` (liefert 10) und
`git show a8be186:crates/krk-ui/src/kommandos/mod.rs | grep -c '^pub mod'` (liefert 9).

## Warum das hier steht und nicht als Kleinigkeit durchgeht

Es ist derselbe Fehlschlag, den die vorige Durchsicht für `df8163d` schon festgehalten hat
(`shared/issues/260823-0730_o_drei-prosastellen-um-den-neuen-nachzug-sind-mit-df8163d-falsch-geworden.md`,
weiter offen): der Commit zieht die Stelle nach, an der die geänderte Sache steht, und übersieht
die Stellen, die von ihr **abhängen**. Zwei Commits hintereinander, dieselbe Gestalt.

Die Zahl selbst ist harmlos. Die Gestalt ist es nicht: dieser Baum trägt seine Zusagen in Prosa,
und `CLAUDE.md` führt eigens den Grund, warum an mehreren Stellen bewusst keine Zahl steht
(„sie wächst mit fast jeder Runde und ist in dieser Datei viermal in vier Tagen falsch
geworden").

## Empfehlung

Zwei Wörter tauschen: „sieben" → „acht", „sechs" → „sieben". Wer die Zahlen dauerhaft loswerden
will, schreibt „vor den übrigen Tastenbefehlsmodulen" und „danach und vor den übrigen" — die
Aussage ist die Reihenfolge, nicht die Menge, und ohne Zahl kann sie nicht falsch werden.

**Schwere:** Medium. Prosaschuld, aber in dem Modulkopf, der die Ordnung des Verzeichnisses
festlegt, und im selben Kommentarblock, den dieser Commit angefasst hat.

**Filed by:** coderev
