The loud question can read "Diese 25 Einträge mit 25 Einträgen in den Papierkorb räumen?"

---

Spec C3 gives the sixth trigger the wording „mit 25 Einträgen" / „mit mehr als 25 Einträgen", and
that wording goes into the question between the entry count and „in den Papierkorb räumen". When
the scope trigger is the *named* reason and the selection is flat, both numbers are the same
number and the sentence says it twice.

---

**Severity:** Low. Nothing is wrong: the sentence is grammatical, both numbers are correct, and
the user is not misled about what will be deleted. What suffers is legibility, in the one round
whose whole purpose is a confirmation the user actually reads.
**Found by:** coder, while implementing step 10 (task T9)
**Affected:** `crates/krk-ui/src/kommandos/loeschwarnung.rs` (`Warngrund::wortlaut`,
`frage_und_erlaeuterung`), spec `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`
section C3
**Tree state:** `3fcd375` plus the uncommitted steps 9 and 10
**Domain:** code

## How it arises

`Warngrund::Umfang` is rank 7, the last. It is the *named* reason only when it is the only reason
— nothing else fired and every input was answered. One such case is a flat selection of exactly
25 files in an ordinary folder under the user's home directory, on a local disk, outside any git
work tree:

```text
  Auswahl:  25 Dateien, keine Ordner       ⇒ Umfang::Genau(25)
  Frage:    "Diese 25 Einträge mit 25 Einträgen in den Papierkorb räumen?"
```

The first 25 counts the selected rows, the second counts the subtree — and for a flat selection
those are the same set. The same doubling appears in the „mehr als" form whenever the selection
itself already exceeds the threshold: "Diese 30 Einträge mit mehr als 25 Einträgen …".

The six other wordings are locative and read cleanly in the same slot ("Diese 3 Einträge von
einem Netzlaufwerk in den Papierkorb räumen?"). Only the scope wording is a quantity, and only a
quantity can collide with the count that is already in the sentence.

## Why it was not fixed while writing the code

The wording is the spec's, in the column „Wortlaut in der Frage" of its C3 table, and the spec is
accepted and binding. Its acceptance criterion is explicit about the number: "Umfasst der
Unterbaum des Vorgangs 25 Einträge, trägt die Frage die Zahl 25. Umfasst er mehr, trägt sie
'mehr als 25'." Changing the phrasing on my own authority would have hidden a question the spec
gate owns, and no reformulation is obviously right — each of the ways below trades something.

## Direction

Four ways, and the first is the cheapest.

1. **Leave it.** The sentence is correct, the case needs a flat selection at exactly the
   threshold, and the loud form is expected to be the everyday case anyway (see the spec's own
   note on the git reach). Cost: nothing. Risk: the one sentence the round exists for reads badly
   in a case that is not rare.
2. **Say what the second number counts.** „mit 25 Einträgen insgesamt" / „mit mehr als 25
   Einträgen insgesamt". One word, keeps the number and the „mehr als" the criterion demands,
   and turns the repetition into an explanation of itself: "Diese 25 Einträge mit 25 Einträgen
   insgesamt …". Cost: one string per wording plus their two probes. Deviates from the spec's
   literal column.
3. **Drop the entry count from the question when the scope is the named reason.** "Diesen Vorgang
   mit mehr als 25 Einträgen in den Papierkorb räumen?" Removes the collision at its root, but
   loses the count that C2 requires in the first line ("nennt … wie viele Einträge betroffen
   sind"), so it would need C2 to be re-read as well.
4. **Move the scope reason out of the question slot** and into the explanation only, leaving the
   question to name the next-ranked reason or stay quiet. This contradicts C3's ranking, which
   makes the scope a named reason of last resort, and would mean a selection of 4000 entries
   asks a quiet question.

Ways 2 and 3 change wording the user accepted at the spec gate; ways 1 and 4 do not need code.
Whichever is chosen, the place is `Warngrund::wortlaut` — two string literals and the
`const _: () = assert!(SCHWELLE == 25, …)` that binds them to the threshold.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, unchanged.** Both
wordings stand verbatim in `crates/krk-ui/src/kommandos/loeschwarnung.rs:533-534`
(`"mit 25 Einträgen"` and `"mit mehr als 25 Einträgen"`), and the spec wording they come from is
accepted, so nothing here is a defect to fix without a user decision. Recorded as read, not as
resolved.

---
Closed 260818 (coder, Bündel C/D-Nachzug): **als Entscheidungsfrage weitergereicht, nicht
gebaut.** Der Datensatz ist
`decisions/260818-0512_o_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`.

**Warum nicht gebaut.** Die beiden Wortlaute stehen wörtlich in der Spalte „Wortlaut in der
Frage" der C3-Tafel des Specs, und der Spec ist am Gate angenommen. Die zwei Möglichkeiten,
die die Doppelung wirklich auflösen, ändern beide diesen Text: Möglichkeit 2 weicht von der
Spalte ab, Möglichkeit 3 bricht daneben ein Kriterium von C2. Der Datensatz sagt es selbst
(„no reformulation is obviously right"), und ein Executor, der hier eigenmächtig umformuliert,
entzieht dem Gate eine Frage, die ihm gehört — der Wortlaut ist das, was der Nutzer im
Ernstfall liest.

Der Entscheidungsdatensatz trägt die vier Möglichkeiten dieses Befundes mit ihren
Folgewirkungen: welche Zeichenketten, welche Proben und welche Abnahmekriterien jede berührt,
und dass allein Möglichkeit 4 das Verhalten ändert und nicht nur den Text. Empfohlen ist
Möglichkeit 2.

Am Baum ist nichts geändert. `crates/krk-ui/src/kommandos/loeschwarnung.rs:604-605` trägt
beide Wortlaute unverändert.
