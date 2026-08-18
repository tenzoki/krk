Step 16 killed 22 pointers in living text, and five of them are in `crates/`

---

Step 16 of this Circle's plan moved five decision records to their terminal markers in commit
`24bbccc`. Twenty-two citations elsewhere still name those records by the marker they carried
before the move. Every one of them resolves to a filename that does not exist, and five of the
twenty-two are module heads under `crates/krk-core/src/verzeichnis/`, which the star-form
decision of 260815 binds by name.

---

**Severity:** Medium
**Found by:** reconciler, session-end pass 260818-0708
**Domain:** code

## Measured, at `e843d90`

Every citation of the form `<store>/<kind>/YYMMDD-HHMM_x_<slug>` in non-exempt files was resolved
against the file store. Exempt by the location rule in `CLAUDE.md` are `history/`, `reviews/`,
`analyses/`, `issues/`, `decisions/`, `messungen/` and `spikes/`; those keep the marker they were
written with and are not counted here.

| citing file | dead pointers |
|---|---|
| `shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md` | 9 (of its 10; the tenth predates this Circle) |
| `circles/260817-0833-…/planning/260817-0856_*_plan-absicherung-jedes-loeschwegs.md` | 4 |
| `circles/260817-0833-…/_t_circle.md` | 4 |
| `crates/krk-core/src/verzeichnis/arbeitsbaum.rs` | 3 (`:32`, `:179`, `:181`) |
| `crates/krk-core/src/verzeichnis/loeschzielbefund.rs` | 1 (`:147`) |
| `crates/krk-core/src/verzeichnis/umfang.rs` | 1 (`:152`) |

By target, all five of them records this Circle moved:

| target as cited | target as it stands | citations |
|---|---|---|
| `260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-…` | `_i_` since `24bbccc` | 8 |
| `260817-0536_a_sieht-die-git-pruefung-…` | `_i_` since `24bbccc` | 5 |
| `260802-0842_i_loeschen-papierkorb-oder-endgueltig` | `_s_` since `24bbccc` | 5 |
| `260817-0536_a_bekommt-f8-den-papierkorb-…` | `_i_` since `24bbccc` | 2 |
| `260817-0536_a_was-geschieht-mit-einer-gespeicherten-keymap-…` | `_i_` since `24bbccc` | 2 |

One further dead pointer sits in the same sweep and is **not** counted above because it predates
this Circle: `shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md:218` names the
round-1 spec as `260802-1036_o_…`, and that file has carried `_c_` since round 1 closed. It is
already reported in `shared/issues/260817-1130_*_die-sternform-fuer-zitate-gilt-seit-dem-260815-und-drei-runden-schreiben-den-marker-aus.md`.

## Why the five in `crates/` are the sharp half

The other seventeen are workbench prose, and the reader who follows one lands in a store they can
list. The five in `crates/` are module heads: they are the only written record of *why* the code
reads the way it does, and the star-form answer of 260815-1230
(`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`,
`_i_`) names `crates/` first in its scope. Commit `e49412a` converted 163 citations to the star
form; the three files here were written afterwards, in `4b50cc1`, `c260e64` and `5a0f041`, with the
letter spelled out again. Step 16 then moved the targets, and the writing-out became a dead
pointer rather than merely a compliance miss.

## What this is not

It is not the class question. Whether a `**Cross-references:**` line inside a frozen store should
carry the star form is open at
`shared/decisions/260818-0201_*_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`,
and no citation counted here lives in such a line — all twenty-two are in living text, where the
260815 answer already binds without needing a new ruling.

It is also not a defect of the plan. Step 16 asks for exactly the movement it got. What no step of
the plan asks for is the sweep afterwards: nothing in the seventeen steps names the citers of a
record that moves, and the plan is itself one of the six files that now point at a name that is
gone.

## Fix

The narrow fix is one `sed` over six files, bringing all twenty-two to the star form `_*_` rather
than to today's letter, on the same ground the analyst gave at 260818-0201: today's letter is known
to be wrong again at the next transition. Three of the six files are `crates/` sources and belong
to `coder`; the plan, the spec and the Circle record are workbench prose.

The broad fix is the check that neither the 260815 answer nor the 260818-0201 record has: a probe
or an `xtask` target that resolves every workbench citation in the tree against the file store. The
sweep that produced this record is fifteen lines of shell and found twenty-three dead pointers in a
tree that four passes had already read.
