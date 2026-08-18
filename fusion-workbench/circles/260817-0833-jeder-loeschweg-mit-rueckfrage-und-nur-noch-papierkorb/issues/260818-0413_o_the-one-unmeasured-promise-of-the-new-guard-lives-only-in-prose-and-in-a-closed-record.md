The one unmeasured promise of the new guard lives only in prose and in a closed record

---

`285b58f` gave the conflict sheet's name field a guard and named, honestly, what it did not
measure: whether the field editor lets `Cmd+Return` and `Opt+Return` through. That statement
stands in a module header and in the closure note of a `_c_` record. No open record carries it,
so it is on no acceptance list and falls out of every search for open work.

---

**Severity:** Medium as a record gap, Low as behaviour. Nothing destructive: both buttons stay
reachable with the mouse or by leaving the field, and Return in the field falls on the
non-destructive "Überspringen". Filed because this Circle's own lesson is that a safeguard which
is only prose is not a safeguard, and because the untested case sits on the sheet's primary
purpose.
**Found by:** coderev, review `reviews/260818-0410-coderev-bundle-f-die-messungen-und-der-waechter.md`
**Affected:** `crates/krk-ui/src/appkit/blaetter/konflikt.rs:49-53`
**Related:** `issues/260817-1241_c_das-konfliktblatt-gibt-seinem-namensfeld-keinen-eingabewaechter.md` (closure note, last but one paragraph)
**Tree state:** `a4d8211`
**Domain:** code

## What stands in the tree

```rust
// crates/krk-ui/src/appkit/blaetter/konflikt.rs:49-53
//! Zwei der vier Antworten bleiben im Feld ohne Taste: "Überschreiben" und
//! "Umbenennen" liegen auf Cmd+Return und Opt+Return, und ob der Feldeditor die
//! beiden durchlaesst, ist am laufenden Buendel zu messen und nicht hier zu
//! behaupten. Erreichbar sind sie in jedem Fall, indem der Nutzer das Feld
//! wieder verlaesst oder die Maus nimmt.
```

The wording is right and the restraint is right — it does not claim what it has not measured.
The gap is where the statement lives. `260817-1241` is `_c_`, and `CLAUDE.md` says of exactly
this shape that a record outside the open stores drops out of the search for active grounding.

**Checked:** no `_o_` record in this Circle's `issues/` or `decisions/`, and none in `shared/`,
mentions the field editor or these two combinations. Searched
`grep -rl 'Feldeditor\|EingabeMitBefehl\|Cmd+Return'` over the workbench: only histories, the
plan, `_c_` issues and reviews.

## Why the case is worth measuring

The one reason a user tabs into that field is to rename. In the field:

| key | today | measured |
|---|---|---|
| `Return` | "Überspringen" via `bestaetigungsstelle` | yes, `die_eingabetaste_im_feld_gehoert_ihrer_eigenen_schaltflaeche` |
| `Esc` | "Abbrechen" via `abbruchstelle` | yes, `die_tafel_der_liegenlassenden_stelle` |
| `Cmd+Return` | "Überschreiben" — **destructive** | no |
| `Opt+Return` | "Umbenennen" — the reason to be in the field | no |

If the field editor swallows the last two, the user who typed a name reaches "Umbenennen" only
by leaving the field or reaching for the mouse, and Return discards what they typed by skipping
the entry. That is not a defect the code can decide; it is a measurement, and it needs KRK in the
foreground, which `CLAUDE.md` names as user work.

## Direction

An entry on the acceptance list of this Circle, in the same form as the other bundle criteria:
in the conflict sheet, tab into the name field, type a name, and press each of `Return`, `Esc`,
`Cmd+Return` and `Opt+Return`; record for each which of the four answers the sheet gives. Four
key presses at the running bundle.

If either combination does not arrive, the answer is a design question and not a patch —
whether `Opt+Return` should be reachable from inside the field at all, or whether the guard
should learn a third answer instead of two. That would be a decision record; this one only asks
that the measurement be on a list somebody reads.
