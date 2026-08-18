Forty-three closure notes are written in a form that no `Resolved:` sweep finds

---

`rules/fusion-workbench-conventions.md` fixes the closure annotation of a defect record as a line
beginning `Resolved: <what was done>`. Of the 428 closed (`_c_`) defect records in this workbench,
43 carry their closure in a shape that a search for `^Resolved:` does not return: 19 write
`Resolved <timestamp> (<agent>): …`, with the colon after the agent instead of after the word, and
24 carry no line beginning with `Resolved` at all.

---

**Severity:** Low
**Found by:** reconciler, session-end pass 260818-0708
**Domain:** code

## Measured, at `e843d90`

Over `shared/issues` and `circles/*/issues`, first level only:

| shape | count |
|---|---|
| `^Resolved:` — the convention | 385 |
| `^Resolved <ts> (<agent>):` — the colon has moved | 19 |
| no line beginning `Resolved` in any shape | 24 |
| closed records in total | 428 |

The 19 of the second shape are all in the Circle
`260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`, where they are 19 of that
Circle's 43 closed records — nearly half of one Circle written one way and the rest of the project
the other. The 24 of the third shape are spread over six Circles and `shared/`; several use
`Closed <date> (<agent>):` or an unlabelled final paragraph instead.

## Why it matters here rather than in general

This project already knows the failure mode and has written it down: `CLAUDE.md` records that
every search pattern demanding `\.md` has a blind spot, and that five surveys of the marker finding
missed the same eight places
(`shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md`).
A closure-count survey has the same shape. A pass that counts `^Resolved:` against `_c_` filenames
reports 43 records closed without a note and goes looking for 43 unfinished closures that are all
in fact finished — or, worse, believes the count and re-opens them.

The reconciliation pass that filed this record hit exactly that: the first sweep returned 19 of
this Circle's 43 closed records as noteless, and every one of the 19 turned out to carry a full
closure note one character away from the pattern.

## What it is not

It is not a claim that any of the 43 is wrongly closed. Each was read; the notes are substantive
and cite commits. The defect is the shape of the line, not the work behind it.

One closed record in the same Circle is a separate matter and is **not** counted here:
`circles/260817-0833-…/issues/260817-1720_*_the-question-can-read-diese-25-eintraege-mit-25-eintraegen.md`
is closed with the note "als Entscheidungsfrage weitergereicht, nicht gebaut" and states in its own
words that nothing at the tree changed. Its question lives on as an open decision record
(`circles/260817-0833-…/decisions/260818-0512_*_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`),
so the item is not lost, but a pass that counts closed records as work done counts that one wrong.

## Fix

Two halves, and the second is the one that holds.

The narrow half is to bring the 43 lines to `Resolved: …`, keeping their wording. That is a
mechanical edit over 43 files in seven stores and touches no content.

The durable half is a check. Every state a filename claims has a companion line inside the file,
and nothing today compares the two. A script that asserts, for every `_c_` defect record, that the
body carries a line beginning `Resolved:` would cover this and would have caught the 43 the day
each was written. Filed as a defect rather than a decision because the convention already exists
and is simply not met; what shape the check takes is an implementation question for whoever writes
it.

---

**Nachmessung des zweiten Abgleichs derselben Sitzung, 260818-0807, an `9ac41ea`.** Die Zahl
steht unverändert bei 43; der Nenner ist um eins gewachsen.

| Form | 260818-0710 (an `e843d90`) | 260818-0807 (an `9ac41ea`) |
|---|---|---|
| `^Resolved:` — die Konvention | 385 | 386 |
| abweichend geschrieben oder ohne Zeile | 43 | 43 |
| geschlossene Datensätze insgesamt | 428 | 429 |

Der eine hinzugekommene ist
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`,
und er trägt seinen Abschlussvermerk in der Konvention. Turn 4 der Sitzung hat also einen
Datensatz geschlossen und die Quote nicht verschlechtert.

Kommando:

```sh
find fusion-workbench -name '*_c_*.md' -path '*issues*' | wc -l
find fusion-workbench -name '*_c_*.md' -path '*issues*' -exec grep -l '^Resolved:' {} \; | wc -l
```

Nicht behoben, und das ist die richtige Entscheidung für diesen Durchgang: der Nutzer hat am
Rebalance-Gate „Artefakt überarbeiten" gewählt und dabei die Zeigerreparatur benannt, nicht
diese. Der Datensatz bleibt offen.
