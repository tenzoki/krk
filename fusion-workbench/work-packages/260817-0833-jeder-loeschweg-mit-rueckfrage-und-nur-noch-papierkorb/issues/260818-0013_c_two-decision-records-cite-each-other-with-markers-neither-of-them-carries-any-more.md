Two decision records cite each other with markers neither of them carries any more
---
Step 16 of this Circle's plan moved five decision records to their terminal markers in commit `24bbccc`. Two `**Cross-references:**` lines elsewhere still name those records by the marker they carried before the move, so both citations resolve to a filename that no longer exists.

- `fusion-workbench/shared/decisions/260817-0536_i_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md` cites the superseded record as `…_i_loeschen-papierkorb-oder-endgueltig.md`. It carries `_s_` since `24bbccc`.
- `fusion-workbench/shared/decisions/260802-0842_s_loeschen-papierkorb-oder-endgueltig.md` cites `…_a_f-tasten-unter-macos-systembelegung.md`. It carries `_i_`.

Found by the analyst during step 17, outside its declared scope, and filed here rather than fixed in place.
---
**The class matters more than the two instances.** A citation that spells the marker out dies at its target's first transition, and every decision record in this project is expected to transition at least twice. The workbench's own convention for `portfolio.md` answers exactly this with the wildcard form `YYMMDD-HHMM_*_<slug>.md`, so that the reader resolves the marker against the store rather than against a frozen copy of it (`rules/circle-records.md`, `### Citation form in the portfolio`). That rule is written for the generated portfolio and does not today bind a hand-written `**Cross-references:**` line.

So there are two possible fixes and they are not the same size. The narrow one corrects these two lines. The broad one asks whether every cross-reference between records should carry `_*_` at the marker position, which would be a convention question for the project and not a defect fix. The second instance above predates this Circle entirely, which is evidence that the narrow fix will not hold: nothing stops the next transition from producing the next stale citation.

**Second finding, same shape, worth checking together:** the plan text of step 16 names the superseding record with `_a_`. The executing analyst wrote `_i_` instead, deliberately, because the target moved in the same task. The plan was left unchanged and the deviation is recorded in commit `24bbccc`. A plan that cites a marker is exposed to the same decay as a record that does.

**Domain:** code
**Filed by:** orchestrator

---
Resolved: 260818-0201 by analyst — **both lines corrected, and the class question filed where it
belongs.** This record said the narrow fix would not hold and it was right twice over: the second
line held not one dead pointer but three.

**Measured, at `ae665e5`.** Every target named in the two `**Cross-references:**` lines, resolved
against the file store with `ls`:

| citing record | target as cited | target as it stands |
|---|---|---|
| `260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-…` | `…_i_loeschen-papierkorb-oder-endgueltig.md` | `_s_` since `24bbccc` |
| `260817-0536_*_wie-wird-jeder-loeschweg-abgesichert-…` | `…_o_spec-absicherung-jedes-loeschwegs.md` | `_o_` — still correct |
| `260802-0842_*_loeschen-papierkorb-oder-endgueltig` | `…/_t_circle.md` | `_b_` |
| `260802-0842_*_loeschen-papierkorb-oder-endgueltig` | `…_a_f-tasten-unter-macos-systembelegung.md` | `_i_` |
| `260802-0842_*_loeschen-papierkorb-oder-endgueltig` | `…_o_directive-zeile-widerspricht-loeschantwort.md` | `_c_` |

Four of five dead, not two. This record named one of the three in the second line; the Circle
record and the round-1 defect were not seen. That is the class argument, restated in numbers by the
very instance filed to illustrate it.

**The narrow fix, and the form it was written in.** Both lines were brought to the star form
`_*_` rather than to today's letter. Writing the letter would have been "correct" for as long as it
took the next transition to land, which is what this record predicted; the star form resolves
against the store and does not age. That choice runs slightly ahead of the rule — by the location
rule in `CLAUDE.md`, `decisions/` keeps its marker per file, so these two lines were not obliged to
change at all — and it is therefore recorded as provisional in the decision below, to be reverted
if the user rules the other way.

**The class question is filed:**
`shared/decisions/260818-0201_*_does-a-cross-references-line-between-records-write-the-marker-in-the-star-form.md`.
It is in the shared store because it binds every record in the project and arose beside this
Circle's Directive rather than from it.

**Two existing records were checked first, and neither asks this question.**
`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`
answers the star-form question for living text and terminates at `_i_`; it explicitly routes
`decisions/` to the location rule instead, which is the gap the new record addresses.
`shared/issues/260817-1130_*_die-sternform-fuer-zitate-gilt-seit-dem-260815-und-drei-runden-schreiben-den-marker-aus.md`
is a compliance defect about living text that already carries the star form as its rule — a
different kind of statement, so no `Also seen:` line was appended to it. The new record cites both.

**The second finding of this record — that the plan text of step 16 cites a marker too — is left
where it is**, because the plan file and its marker are the reconciliation's business at session
end and outside this task. It is covered by the class question all the same: a plan file is living
text and the 260815 answer already binds it, so the plan is a compliance instance of a rule that
exists, not a case awaiting one.
