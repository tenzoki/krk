`fusion-workbench/agentstate.yaml` does not parse as YAML, and its artifact-language note sits inside the work queue

---

The session-state file is not valid YAML. A mapping key, `artifact_language:`, sits at the same
two-space indent as the `- id:` items of the `work_queue:` block sequence, so the document ends
mid-collection. `ruby -ryaml -e 'YAML.load_file(…)'` fails with
`Psych::SyntaxError: did not find expected '-' indicator while parsing a block collection at line
25 column 3`. Read as written, the note is a fifteenth work-queue entry rather than session
context.

---

**Severity:** Low. Nothing behaves wrongly today, because neither consumer parses YAML: the hook
matches lines flatly and says so (`$FUSION_PLUGIN_ROOT/hooks/lib/state-file.ts:75`, "Deliberately
flat rather than a YAML parse"), and `$FUSION_PLUGIN_ROOT/bin/monitor` renders the file as
syntax-highlighted text (`bin/monitor:397-406`). The cost is that the file's declared format is
no longer the format it is in, so the next consumer that does parse it — or a human reading the
indentation to find out what belongs to what — gets a wrong answer, and the block that carries
this project's artifact-language rule is the block that reads wrong.
**Found by:** reconciler, Phase 3 of session 260817-1208
**Affected:** `fusion-workbench/agentstate.yaml`, the `artifact_language:` block under
`work_queue:`
**Tree state:** `e313841`
**Domain:** code
**Cross-references:**
`$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md` `## fusion-workbench Layout` (the
file is root-anchored and read by `bin/monitor` and `hooks/lib/state-file.ts`),
`shared/issues/260817-1610_o_the-language-paragraph-in-claude-md-predates-the-artifact-language-declaration.md`
(the same declaration, described in the wrong place in `CLAUDE.md`)

## Measured

```
$ ruby -ryaml -e 'YAML.load_file("fusion-workbench/agentstate.yaml")'
Psych::SyntaxError: did not find expected '-' indicator
  while parsing a block collection at line 25 column 3
```

Line 25 is the first `- id: "T1"` of `work_queue:`. The block that closes the collection is:

```yaml
work_queue:
  - id: "T10"
    …
  artifact_language: |
    Seit 260817-1600 trägt CLAUDE.md die Zeile **Artifact language:** en …
```

Two spaces of indent make `artifact_language` a sibling of the sequence items, which YAML does
not allow. At four spaces it would be a field of the `T10` entry, which is also wrong. It belongs
under `session:` or under `plan_context:`, at the same level as `plan_context.key_findings`, which
is the block it is written in the style of.

## Why it is filed here and not in `shared/`

The file is session state for this Circle's session, and the block was written while executing
this Circle's Directive, so the Origin Rule puts it in this Circle. It is not a defect in KRK: no
line of `crates/` is involved.

## Direction

Move the `artifact_language:` block to `session:` or to `plan_context:` and re-indent it, then
check the file with `ruby -ryaml -e 'YAML.load_file("fusion-workbench/agentstate.yaml")'`. The
file belongs to the orchestrator, so the move is the orchestrator's, not the reconciler's.

Worth considering beside the fix, and not decided here: the file is untracked
(`.gitignore:15`, and it is live state by the conventions' tracked-workbench split), so a
malformed version leaves no trace once it is overwritten. A one-line parse check after each write
would catch it at the point where the writer still knows what it meant.

---
Resolved: 260817-1848 by orchestrator — the `artifact_language:` block was moved out of the
`work_queue:` sequence into `plan_context:`, where it belongs by content: it is session context,
not a queue entry. `work_queue:` now holds nothing but its twelve `- id:` entries and
`plan_context:` holds four keys.

**The evidence is deliberately not a commit hash.** `fusion-workbench/agentstate.yaml` is not
tracked — it is live state under the split in `rules/fusion-workbench-conventions.md`
`## Which of them a tracked workbench tracks`, and a committed copy would be a statement about a
session that has ended. So this closure cites the working tree at `e313841` and nothing else, and
a reader who wants to verify it reads the file rather than a diff. That is a property of the
store, not a gap in the record.

The finding's second half stands as written and is worth keeping: no consumer breaks on the fault,
because `hooks/tracker.ts` and `bin/monitor` read the file line-wise rather than parsing it. The
file was therefore wrong for roughly three hours without anything noticing, which is the reason a
malformed line here is worth a record at all.
