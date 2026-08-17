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
