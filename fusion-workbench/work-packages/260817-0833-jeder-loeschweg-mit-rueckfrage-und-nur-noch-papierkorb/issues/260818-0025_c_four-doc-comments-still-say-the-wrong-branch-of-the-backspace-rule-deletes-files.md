Four doc comments still say the wrong branch of the backspace rule deletes files, and the rule's own module head now says the opposite

---
Step 15 rewrote the module head of `crates/krk-ui/src/kommandos/rueckschritt.rs` so that it states
the new truth: the wrong branch of the backspace case split no longer deletes anything, it asks.
The same sentence stands unchanged in four other places, all of them describing the same rule, and
all four still assert the pre-260817 behaviour. The tree therefore carries both readings of one
rule at the same time.

---

**Severity:** Medium. Nothing executes differently — the whole reviewed range changes comments
only. But the load-bearing question of bundle E was "does any prose in the tree still assert the
old state", and the answer is yes, in the one rule the plan singles out as security-relevant. A
later reader who opens `anwendung.rs` first learns that the wrong half of this branch removes
files, and a reader who opens `rueckschritt.rs` first learns that it cannot. One of the two will
reason from the wrong premise, and this project has filed that exact failure before
(`issues/260810-1102_*`, where reading only `ereignisse.rs` produced a defect report about a defect
that did not exist).

**Found by:** coderev, review `reviews/260818-0024-coderev-bundle-e-the-prose-and-the-records.md`
**Tree state:** `da716c1`
**Domain:** code

## The five places, measured at `da716c1`

The one that was updated:

```
crates/krk-ui/src/kommandos/rueckschritt.rs:32-34
//! **Seit dem 260817 ist die Rueckfrage die zweite Sperre, und die
//! Unterscheidung hier ist dadurch milder geworden**: ihr falscher Zweig
//! raeumt nichts mehr, er fragt.
```

The four that were not:

| Place | Text | Why it is wrong now |
|---|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs:4466-4467` | "**Der eine Zweig dieser Runde, dessen falsche Haelfte Dateien wegraeumt**" | The doc comment of `papierkorb_oder_zeichen_zurueck`, the single caller of the rule. Its first paragraph two lines above **was** rewritten in `522cf51`; this sentence directly under it was left. |
| `crates/krk-ui/src/appkit/anwendung.rs:2891` | "**Der eine Zweig, dessen falsche Haelfte Dateien wegraeumt.**" | The comment on the `Kommando::InPapierkorb` arm of `kommando_ausfuehren`. The next line, `:2894`, adds "alles andere geht unveraendert in den Papierkorb" — since `472eb81` everything else goes into the confirmation, not into the trash. |
| `crates/krk-ui/src/appkit/anwendung.rs:2660` | "zwei Fassungen koennten auseinanderlaufen, und dann raeumte die falsche Haelfte Dateien weg" | Justifies why `eingabe_ausfuehren` calls `ist_nackter_rueckschritt` instead of restating the question. The stated consequence of a divergence is no longer a deletion. |
| `crates/krk-ui/src/appkit/ereignisse.rs:299-300` | "Zwei Fassungen derselben Frage koennten auseinanderlaufen, und dann raeumte die falsche Haelfte Dateien weg" | The doc comment of `Anschlag::ist_nackter_rueckschritt`. |

A fifth candidate, weaker and named here so the next pass does not have to rediscover it:
`anwendung.rs:4535` carries "// Wie vor dieser Runde (C1.16, C1.20)." on the
`Rueckschritt::InDenPapierkorb` arm. "Diese Runde" there means round 10, and at the level of
dispatch the sentence is still true — `delete` still reaches `in_den_papierkorb`. Read at
`da716c1` it nevertheless invites the reading "still deletes as it did before", which is what the
four rows above assert outright.

## Why the sweep of step 15 did not reach them

The step's scope is `grep -rniE "endgueltig|endgültig" --include="*.rs" crates`. None of the four
lines carries the word. The executor ran a second sweep,
`grep -rniE "ohne rueckfrage|ohne nachfrage|opt\+cmd\+delete|beide loeschbefehle"`, which found
three other places — and none of these four either, because they name the consequence
("raeumt Dateien weg") rather than the absence of the question.

The `ereignisse.rs` case is the sharpest: the executor's own record
(`history/260817-2356-coder-e15-kommentare-und-claude-md.md`, section "Nicht geändert, mit Grund")
names `ereignisse.rs:307` and declares it correct, which it is. Lines `:299-300` sit eight lines
higher in the **same** doc comment and were not read.

## Direction

One search finds all four and nothing else:

```sh
grep -rniE "raeumte? .{0,20}(Dateien )?weg|wegraeumt" --include="*.rs" crates/krk-ui/src/appkit
```

The correction is the same in each: the wrong half now opens the confirmation instead of removing
files, and that is why the case split is milder but not superfluous — the wording
`rueckschritt.rs:32-40` already carries. `anwendung.rs:2894` needs the second half too
("geht unveraendert in den Papierkorb" → into the confirmation before it).

Do not weaken the four sentences into "nothing can go wrong here". The rule is still
security-relevant for the reason `rueckschritt.rs` now states: a confirmation that opens on every
corrected typo gets clicked away instead of read.

---
Resolved: Alle vier Stellen auf den Stand von `rueckschritt.rs:26-33` nachgezogen — der
falsche Zweig raeumt nichts mehr, er laesst die Loeschrueckfrage aufgehen, und genau darum
ist die Fallunterscheidung milder und nicht ueberfluessig. Geaendert:

- `crates/krk-ui/src/appkit/anwendung.rs:4468-4474` — „dessen falsche Haelfte die
  Loeschrueckfrage aufgehen laesst", dazu der Grund in zwei Saetzen: seit dem 260817 fragt
  diese Haelfte, und eine Rueckfrage, die auf jeden berichtigten Vertipper aufgeht, wird
  weggeklickt statt gelesen.
- `crates/krk-ui/src/appkit/anwendung.rs:2892-2897` — dieselbe Ueberschrift auf dem
  `Kommando::InPapierkorb`-Zweig, und die zweite Haelfte mit: „alles andere geht
  unveraendert in die Rueckfrage vor dem Papierkorb".
- `crates/krk-ui/src/appkit/anwendung.rs:2659-2661` — die Folge einer Divergenz ist nicht
  mehr ein Loeschen, sondern eine Rueckfrage auf einen berichtigten Vertipper.
- `crates/krk-ui/src/appkit/ereignisse.rs:298-301` — dieselbe Korrektur am Doc-Kommentar
  von `Anschlag::ist_nackter_rueckschritt`.

Der fuenfte, schwaechere Kandidat ist mitgezogen: `anwendung.rs:4541` sagt jetzt „Wie vor
der Runde 10 (C1.16, C1.20): der Weg in den Papierkorb, seit dem 260817 mit seiner
Rueckfrage davor" statt „Wie vor dieser Runde".

Die Suche des Datensatzes,
`grep -rniE "raeumte? .{0,20}(Dateien )?weg|wegraeumt" --include="*.rs" crates/krk-ui/src/appkit`,
liefert von diesen fuenf keine Zeile mehr; die verbleibenden Treffer betreffen die
Statuszeile, die Tabelle und `ereignisse.rs:307`, das seit Buendel D richtig steht.
`make check` — exit 0.
