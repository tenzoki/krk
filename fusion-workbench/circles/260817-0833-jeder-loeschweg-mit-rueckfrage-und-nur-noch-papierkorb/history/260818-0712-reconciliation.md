# Reconciliation — session 260817-2131, Circle "Jeder Löschweg fragt nach, und es gibt nur noch den Papierkorb"

**Date:** 260818-0712
**Status:** Complete
**Domain:** code
**Range:** `cdde9da..e843d90`, 16 commits, three Turns
**Tree state:** `e843d90`, working tree carrying two uncommitted workbench files (`_t_circle.md`, `orchestrator-events.jsonl`)
**Verification:** `make check` — exit 0 (build, test, fmt, clippy)

## What was reviewed

| Store | Read | Changed |
|---|---|---|
| Plans (Circle + shared) | 5 | 1 (status, reconciliation log, marker `_o_`→`_c_`) |
| Defect records (Circle + shared) | 49 in the Circle, 30 open in shared | 2 appended (`Also seen:`), 2 new records filed |
| Decision records (Circle + shared) | 3 in the Circle, 29 in shared | 0 |
| Reviews (Circle) | 6, of which 3 in this session's range | 0 |
| History (Circle + shared) | 30 in the Circle, skimmed | 1 written (this file) |
| Circle record | 1 | 1 pointer line pulled |

## The five claims, each measured

**1. All seventeen plan steps stand at the tree.** Holds. Steps 1 to 11 were read against the tree
in the two earlier reconciliations (260817-1129, 260817-1833); steps 12 to 17 were read here, each
against the tree and not against the session log or the reviews. The evidence table is in the plan
file under `## Reconciliation Log`, entry `260818-0708`. The filename marker moved `_o_`→`_c_` and
the header now says so.

**2. KRK knows exactly one delete path and it leads to the trash.** Holds.
`grep -rn 'EndgueltigLoeschen' crates/ xtask/ resources/` returns nothing.
`Kommando::KENNUNGEN` is declared with 78 entries at
`crates/krk-core/src/tasten/belegung.rs:647`, and the `Kommando` enum carries 78 variants counted
independently. `Art` (`crates/krk-core/src/operation/auftrag.rs`) carries `InDenPapierkorb` and no
second delete value. `resources/default-keymap.toml:158` gives `in_papierkorb` the keys
`["delete", "cmd+delete", "f8"]`; `opt+cmd+delete` appears in no key list. One body,
`Anwendungsdelegierter::loeschen_nach_rueckfrage` (`crates/krk-ui/src/appkit/anwendung.rs:4673`),
with one caller, `in_den_papierkorb` (`:4476`).

**3. Five decision records stand at their terminal state.** Holds. The five filenames carry
`_s_` once and `_i_` four times, each `**Status:**` line agrees with its marker, and each file
carries its `Superseded by:` or `Implemented:` line naming commits. The cited commits were spot-read
at the tree and hold.

**4. No Circle finding was open before the last review; six arrived after it.** Holds, measured
against git rather than the file listing: `git ls-tree -r a4d8211` over the Circle's `issues/`
returns zero `_o_` names, and `HEAD` returns exactly the six filed by the bundle-F review
(`260818-0410` through `260818-0415`).

One qualification that does not break the claim but changes what the closure count means.
`260817-1720_c_the-question-can-read-diese-25-eintraege-mit-25-eintraegen.md` is closed with the
note "als Entscheidungsfrage weitergereicht, nicht gebaut" and states in its own words that nothing
at the tree changed. Its question lives on as
`decisions/260818-0512_*_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`,
which is open. The item is not lost and the closure is not wrong — the record was relocated to the
store whose vocabulary can express its state, which is the disposition the reconciler protocol asks
for. But a later pass that reads 43 closed records as 43 pieces of work done counts that one wrong.
Left as it stands; noted here and in the new shared record below.

**5. The reviews tile the session range without gaps.** Holds for every commit that touches code or
data. The three ranges are `cdde9da..f7a85c1` (bundle D), `f7a85c1..da716c1` (bundle E) and
`f79f964..a4d8211` (bundle F). Two commits of the sixteen fall outside all three: `f79f964` and
`e843d90`. Both were checked with `git show --stat` and touch nothing but
`fusion-workbench/`; each is the filing commit of the review that follows it, which no review can
cover.

## Divergences found and repaired

1. **The plan's status line still said a reconciliation over bundles D and E was outstanding, and
   its marker still said `_o_`.** Both were true when written and are not now. Status line rewritten
   to name all three reconciliation dates; filename moved to `_c_`.
2. **One pointer in living text named the plan by its old marker.** The Circle record's
   `**Active spec/plan:**` line at `_t_circle.md:7`. Pulled. The other 35 citers of the plan filename
   all sit in `history/`, `reviews/`, `issues/` and `decisions/` and keep their marker under the
   location rule in `CLAUDE.md`.

## Divergences found and not repaired

1. **Twenty-two dead pointers in living text, created by the Circle's own step 16.** Filed as
   `issues/260818-0710_*_step-16-killed-22-pointers-in-living-text-and-five-of-them-are-in-crates.md`.
   Not repaired here: three of the six affected files are `crates/` sources and belong to `coder`,
   and the other three are plan, spec and Circle-record prose, which the reconciler annotates but
   does not rewrite. An `Also seen:` line was added to
   `shared/issues/260817-1130_*_die-sternform-fuer-zitate-gilt-seit-dem-260815-…`, whose prediction
   this measures.
2. **Forty-three closure notes escape a `^Resolved:` search.** Filed as
   `shared/issues/260818-0710_*_forty-three-closure-notes-are-written-in-a-form-no-resolved-sweep-finds.md`.
   Not repaired here: 43 files across seven stores is a mechanical edit, but it is content in
   records the reconciler preserves rather than rewrites, and the durable half of the fix is a check
   that nobody has written.
3. **Three of the sixteen commits carry no `commit` event in `orchestrator-events.jsonl`**
   (`8f556ed`, `f79f964`, `b0eee2c`). Appended as an `Also seen:` line to the existing
   `shared/issues/260810-1945_*_der-orchestrator-hat-in-drei-turns-keine-aufgabenereignisse-emittiert.md`
   rather than filed separately, per the duplicate rule. `task_start` and `task_done` are complete
   this time (13 each), so the finding is now the narrower "filing a review emits no commit event".
4. **The spec keeps `_o_`.** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` is
   satisfied by the completed plan, but its marker belongs to the closure of the Circle and
   therefore to the Rebalance gate, not to this pass.
5. **The orchestrator's session file still says the Directive is unstated and the status is In
   Progress.** That is Phase 4 work and the reconciler runs at Phase 3; recorded, not a defect.

## Misfiled — should be a decision

None found beyond the one already relocated: `260817-1720_c` was closed by handing its question to
`decisions/260818-0512_o_…`, which is the correct store for it. No open defect record in either
store was found to be a decision wearing a defect's marker.

## Numbers that did not survive their own check

The dispatch named 19 commits in `cdde9da..HEAD`. `git rev-list --count cdde9da..HEAD` returns 16,
and `git log --oneline` lists 16. Reported rather than filed: it is a number in a prompt, not in a
record.
