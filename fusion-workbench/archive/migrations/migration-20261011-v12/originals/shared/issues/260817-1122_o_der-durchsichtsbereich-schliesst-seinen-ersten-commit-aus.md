Der angegebene Durchsichtsbereich schliesst seinen ersten Commit aus

---

Die Durchsicht `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/reviews/260817-1105-coderev-buendel-a-die-unbedingte-rueckfrage.md`
trägt `**Reviewed-range:** 664a0fd..472eb81`. In der Bereichsschreibweise von git ist der
erste Commit **ausgeschlossen**: `664a0fd..472eb81` deckt `375d07c` und `472eb81`, nicht
aber `664a0fd`. `bin/fusion-review-coverage` meldet `664a0fd` daher als ungedeckt, obwohl
die Durchsicht seine Datei `crates/krk-ui/src/kommandos/loeschwarnung.rs` ausdrücklich in
ihrem Umfang führt und gelesen hat.

Der Inhalt ist also gedeckt, die Angabe nicht. Der richtige Bereich wäre
`2793287..472eb81` gewesen, also der Turn-Anker vor dem ersten Commit des Bündels.

---

**Verursacht vom Orchestrator, nicht vom Reviewer.** Der Auftrag an `coderev` hat den
Bereich `664a0fd..472eb81` wörtlich vorgegeben und ausdrücklich verlangt, genau diesen in
die Zeile `**Reviewed-range:**` einzutragen. Der Reviewer hat getan, was dastand.

Die Folge ist nicht schwer, aber sie ist genau die Sorte Rauschen, gegen die die
Deckungsmessung gebaut ist: die nächste Sitzung liest `664a0fd uncovered` und kann aus der
Zahl allein nicht erkennen, dass der Inhalt gelesen wurde.

**Zwei Wege zur Behebung, beide klein.** Entweder die Zeile in der Durchsicht auf
`2793287..472eb81` berichtigen, mit einer Notiz warum. Oder der Orchestrator gibt künftig
den Turn-Anker als Bereichsanfang an, so wie er ihn ohnehin in `control.turn_start_head`
führt. Der zweite Weg behebt die Ursache und nicht den Fall.

---
Abgleich 260817-1129 (reconciler): **offen, unverändert.** Die Zeile
`**Reviewed-range:** 664a0fd..472eb81` steht so in der Durchsicht, und der Abschnitt
`## Review coverage` des Sitzungsprotokolls `shared/history/260816-2113-orchestrator-session.md`
führt `664a0fd` weiter unter „Not covered". Keiner der beiden genannten Wege ist gegangen.
