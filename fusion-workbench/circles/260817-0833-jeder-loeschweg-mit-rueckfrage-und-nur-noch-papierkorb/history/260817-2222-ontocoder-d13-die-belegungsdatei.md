# D13 — The shipped key map loses the permanent delete

**Status:** Complete
**Agent:** ontocoder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 13 (second step of bundle D)
**Binding:** `shared/decisions/260817-0536_a_bekommt-f8-den-papierkorb-nachdem-das-endgueltige-loeschen-weggefallen-ist.md` (option 1); `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`
**Tree state before the task:** `cdde9da` plus the uncommitted step 12, tree red with 52 failures
**Verification:** `make check` — exit 0 (all four acceptance commands; 1321 probes passed, 0 failed)

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`.
> The file itself stays German, as does every comment in it.

## What changed in `resources/default-keymap.toml`

Five edits, all in the one file the step names.

1. **The entry `endgueltig_loeschen` fell whole** — id, name and its two combinations. `grep -n
   "endgueltig_loeschen"` and `grep -n "opt+cmd+delete"` both return nothing.
2. **`f8` moved onto the trash command.** `in_papierkorb` now reads
   `tasten = ["delete", "cmd+delete", "f8"]`. It sits directly after `ordner_anlegen` (F7) in the
   Norton block, so the F3–F8 run in the file is unbroken and the menu order is unchanged. The
   block header's two-ways promise still holds for six functions: F3, F4, F5, F6, F7 and now F8.
3. **The header count line was pulled along**, from 85 functions with 90 combinations to 84 with 89.
4. **The binding record in the header was replaced.** `shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`
   is superseded by `shared/decisions/260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.
   The mention of `260802-0842_*_f-tasten-unter-macos-systembelegung.md` beside it stayed.
5. **Two comment sentences about the factory-free combinations became false and were corrected.**
   See below.

## The counts were measured, not copied

The plan expects 84 and 89. Both were counted against the file after the edit and both match:

```
grep -c '^\[\[funktion\]\]'                                    -> 84
grep '^tasten = ' | grep -o '"[^"]*"' | wc -l                  -> 89
```

Before the edit the same two counts gave 85 and 90, which is what the header claimed. No deviation
from the plan's expectation, so no adjusted number was needed.

## The two sentences that stopped being true

Neither was named in the dispatch by line, and both were inside the "check the comment about `f8`
and pull it along, but only inside this file" clause. `opt+cmd+delete` is now unassigned, which
makes it the second factory-free combination, so a sentence claiming there is exactly one is wrong:

- **Header, the free-combinations paragraph:** "Eine Kombination bleibt ab Werk ausdruecklich frei
  … Umschalt+Entf" became two combinations, with Opt+Cmd+Entf named beside it, the reason given
  (it means "delete immediately" in the Finder and KRK no longer has that meaning) and the deciding
  record cited.
- **At `mit_standardprogramm_oeffnen`, line ~705:** "Frei bleibt ab Werk damit allein
  Umschalt+Entf" carried the word *allein*, which is the same false claim from the other end. It
  now reads as a past state with the 260817 addition named and a pointer back to the header, so the
  fact stays authored in one place.

No other Norton or F8 mention in the file needed touching. The three that name "die sechs
Funktionen der Norton-Reihe" (lines ~9, ~640, ~847) still count six.

## The tree is green again

`make check` runs all four acceptance commands and exits 0. The 52 failures step 12 left behind
are gone and none arrived in their place: 1321 probes pass, 0 fail, `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` are clean. Menu, key-map view and Markdown
output follow this file and needed no edit, which is what the plan predicted.

Nothing was committed. Steps 12 and 13 are one commit and the orchestrator writes it.

## Files

- `resources/default-keymap.toml`

## Records

- `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 13 → `[DONE]` (the plan file
  itself was not edited; marker maintenance is the orchestrator's)
- `shared/decisions/260817-0536_a_bekommt-f8-den-papierkorb-nachdem-das-endgueltige-loeschen-weggefallen-ist.md`
  is realised by this step, both halves of it: `f8` on the trash command and `opt+cmd+delete` left
  unassigned. Its `_a_` → `_i_` walk waits for the commit hash and belongs to step 16, the same way
  step 12 left its own.
