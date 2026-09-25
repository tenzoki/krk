Two history filenames and four closure notes carry timestamps that no clock produced

---
`history/260817-1806-coder-t10-die-laute-warnform.md` is committed in `792995a`, whose author date is
17:39, and four closure notes in this Circle's issue store cite `260817-1806` as their resolution
time. At 17:59 that timestamp was still 7 minutes in the future.
`history/260817-1722-coder-t9-tafel-der-ausloeser-und-die-umbenennung.md` sits in `c1b52db`, author
date 17:13, and is 9 minutes ahead of its own commit.

---

**Severity:** Low. Nothing behaves wrongly and no record is lost. What is lost is the ordering: a
session whose artefacts carry timestamps ahead of the commits containing them cannot be laid against
the commit log, and this project has already spent two records on exactly that kind of unreliability
(`shared/issues/260811-2157_o_…-ohne-eigene-turn-grenze.md`,
`shared/issues/260817-1122_o_der-durchsichtsbereich-schliesst-seinen-ersten-commit-aus.md`).
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `history/260817-1722-coder-t9-tafel-der-ausloeser-und-die-umbenennung.md`,
`history/260817-1806-coder-t10-die-laute-warnform.md`, and the `Resolved: 260817-1806` lines in
`issues/260817-1108_c_…`, `issues/260817-1107_c_…`,
`issues/260817-1419_c_der-abschluss-von-260817-1107-…`,
`issues/260817-1419_c_der-papierkorbtest-laeuft-vor-den-beiden-billigen-sperren-…`
**Tree state:** `792995a`
**Domain:** code
**Cross-references:** `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md` `## Timestamps`

## Measured

Commit author time against the history filename each commit carries:

| commit | author time | history filename | offset |
|---|---|---|---|
| `17d3550` | 15:11 | `260817-1504-…` | −7 min |
| `c260e64` | 15:33 | `260817-1529-…` | −4 min |
| `5a0f041` | 16:08 | `260817-1602-…` | −6 min |
| `749a4f3` | 16:30 | `260817-1623-…` | −7 min |
| `c1b52db` | 17:13 | `260817-1722-…` | **+9 min** |
| `792995a` | 17:39 | `260817-1806-…` | **+27 min** |

The first four have the expected sign: the file is written, then committed. The last two are ahead of
their own commits, which no clock produces. `date` at the time of this review returned
`260817-1759`, so `260817-1806` is in the future outright.

## Why it is worth a record rather than a shrug

The convention is explicit and gives the reason: "Always obtain `YYMMDD-HHMM` from
`date +%y%m%d-%H%M`. LLMs have no clock — never guess or estimate the time." Two artefacts of this
turn are evidence that the step was skipped at least twice, and it is the kind of skip that leaves no
other trace: a filename looks exactly as correct as one that came from the clock.

The concrete cost is small and real. Four closure notes state when three findings were resolved; the
stated time is after the commit that resolved them, so a reconciliation pass that orders closures
against commits gets the wrong order for those three.

## Direction

Do not rewrite the two filenames. They are records of a state and the location rule for `history/`
keeps them as they stand, and renaming them would orphan the four `Resolved:` citations that point at
one of them. What is worth doing is one line in each of the two history files noting the commit each
belongs to (`c1b52db`, `792995a`) with its author time, so the chronology is recoverable from the
file rather than from this record. And the cheap prevention for the next turn is to read the clock in
the same command that writes the file, rather than before the work.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, and the measured scope is
larger than this record states.** The four closure notes are named correctly and completely:
three `Resolved: 260817-1806` lines plus the `Nachtrag 260817-1806` in
`issues/260817-1107_c_…`, all four inside `792995a` (author time 17:39). The filename count is
not. Every history file of this Circle measured against the author time of the commit that added
it (`git log --diff-filter=A`), four run ahead rather than two:

| history filename | commit | author time | offset |
|---|---|---|---|
| `260817-1104-coder-a2-blatt-beschriftung-und-laut.md` | `375d07c` | 10:38 | **+26 min** |
| `260817-1345-coder-b5-frage-nach-dem-papierkorb.md` | `e2760cd` | 13:41 | **+4 min** |
| `260817-1722-coder-t9-tafel-der-ausloeser-und-die-umbenennung.md` | `c1b52db` | 17:13 | **+9 min** |
| `260817-1806-coder-t10-die-laute-warnform.md` | `792995a` | 17:39 | **+27 min** |

The first of the four predates this session: `375d07c` belongs to the Bundle A turn of session
260816-2113, so the skipped clock read is not a Turn-2 habit but at least a two-session one. One
further closure note runs ahead and is not listed under **Affected**:
`Resolved: 260817-1722` in `issues/260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`,
committed in `c1b52db` at 17:13, 9 minutes behind its own stated resolution time.

**One artefact named in the dispatch cannot be measured at all.** The `# Updated:` line of
`fusion-workbench/agentstate.yaml` was reported as hand-set, but the file is untracked
(`.gitignore:15`, and it is live state by
`$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md` `## Which of them a tracked
workbench tracks`), so no past value of that line exists anywhere to compare against a commit.
Its current value, `260817-1816`, is not in the future: `date +%y%m%d-%H%M` returned
`260817-1818` two minutes later. That is the whole of what evidence can say about it.

The **Direction** of this record is unaffected: do not rename the files, add the commit and its
author time as one line inside each of the four, and read the clock in the same command that
writes the file.

---
Resolved: 260818-0201 by analyst — **nothing renamed, nine addenda written**, following this
record's own `## Direction` and the wider scope the Abgleich of 260817-1833 measured.

**Re-measured first, at `ae665e5`, rather than carried over.** Every history file of this Circle
against the author time of the commit that added it
(`git log --diff-filter=A --format='%h %ad' --date=format:'%y%m%d-%H%M' -1 -- <file>`), all
twenty-four of them:

| history filename | commit | author time | offset |
|---|---|---|---|
| `260817-1104-coder-a2-blatt-beschriftung-und-laut.md` | `375d07c` | 260817-1038 | **+26 min** |
| `260817-1345-coder-b5-frage-nach-dem-papierkorb.md` | `e2760cd` | 260817-1341 | **+4 min** |
| `260817-1722-coder-t9-tafel-der-ausloeser-und-die-umbenennung.md` | `c1b52db` | 260817-1713 | **+9 min** |
| `260817-1806-coder-t10-die-laute-warnform.md` | `792995a` | 260817-1739 | **+27 min** |

Four, exactly the four the Abgleich named — the Abgleich's correction of this record's count of
two is confirmed, and the eight files written since (Bundles D and E, and the two reconciliations)
add no fifth. The last of them, `260818-0157-…` in `ae665e5` at `260818-0159`, runs two minutes
behind its commit, which is the expected sign.

**What was written.** One addendum in each of the four history files, naming the commit that added
it, that commit's author time, and the size of the offset. One addendum in each of the five
closure notes the Abgleich enumerated — the three `Resolved: 260817-1806` lines, the
`Nachtrag 260817-1806` in `260817-1107_c_…`, and the `Resolved: 260817-1722` in
`260817-1623_c_…` — each naming the commit that carries the work the line closes. Nine files, nine
addenda, no existing wording touched.

**Why no rename, stated as a measurement rather than as a preference.** This record's `## Direction`
argued it from the location rule and from the four citations pointing at one of the two names. Both
hold, and the second is now counted: `grep -rn '260817-1806-coder-t10'` returns citations across
`issues/`, `history/` and `reviews/`, every one of them in a store the location rule freezes. A
rename would therefore have had to be chased into files that are not allowed to change — the
citation and the record it cites cannot both be corrected, and the record of a state is the one
that wins. The addendum buys back exactly what the rename would have bought, the recoverability of
the chronology from the file itself, and costs nothing.

**One artefact stays unmeasurable, as the Abgleich already recorded**: the `# Updated:` line of
`fusion-workbench/agentstate.yaml`. The file is untracked, so no past value exists to compare
against any commit. Nothing was written there.

**The prevention this record proposed is not implemented and is not a defect of this closure.**
"Read the clock in the same command that writes the file" is a working habit, and no gate in this
project enforces it — the same shape as the star-form convention, which
`shared/issues/260817-1130_*_die-sternform-fuer-zitate-…` records as having been broken across four
rounds precisely because it was accepted without a check in the build. If the habit is to hold, it
needs a check; that is a question for the user and not something a closure note can settle.
