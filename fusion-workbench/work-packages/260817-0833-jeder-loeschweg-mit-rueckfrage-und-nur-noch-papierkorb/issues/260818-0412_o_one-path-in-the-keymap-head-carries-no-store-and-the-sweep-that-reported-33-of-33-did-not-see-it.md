One path in the keymap head carries no store, and the sweep that reported 33 of 33 did not see it

---

`b0eee2c` unwrapped 22 split paths in `resources/default-keymap.toml` and gave ten prefix-less
ones their Circle. Its history record reports `paths checked: 33  unresolved: 0`. Line 8 names
the spec as a bare filename with no store at all, resolves against nothing, and is not among the
33.

---

**Severity:** Low. One reference, and the file it means is findable by search. Filed because the
sweep's own closing measurement says every path in the file resolves, and one does not — the
same shape as the blind spot `260817-1419` records for the folder boundary `crates/` and
`shared/issues/260810-1851` for the `\.md` needle.
**Found by:** coderev, review `reviews/260818-0410-coderev-bundle-f-die-messungen-und-der-waechter.md`
**Affected:** `resources/default-keymap.toml:8`
**Related:** `issues/260817-2243_c_two-decision-paths-in-the-keymap-head-are-split-across-comment-lines-and-escape-every-search.md`,
`history/260818-0340-ontocoder-pfade-und-blattsperre-in-der-belegung.md`
**Tree state:** `a4d8211`
**Domain:** code

## What stands in the tree

```toml
# resources/default-keymap.toml:8
# Quelle: Spec `260802-1036_*_spec-navigator-geruest.md`, Faehigkeiten C1 bis
```

Every other backticked workbench reference in the file now carries a store: `shared/decisions/…`
on the two lines directly below it, `circles/<circle>/decisions/…` at the ten places `b0eee2c`
resolved. This one carries none, so the path rule in `CLAUDE.md` has nothing to apply — that
rule resolves `planning/…`, `decisions/…`, `analyses/…` and `issues/…` against a Circle, and a
bare filename is none of those forms.

The file it means:

```
fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md
```

**Counted myself, at `a4d8211`.** 35 backticked path-like tokens in the file, 28 unique. 20 of
them are workbench references ending in `.md`; all 20 resolve against their marker glob. The
remaining unresolved one is this line. (`settings.toml` at `:617` is not a project path but the
generic file name in prose, and is not a finding.)

**Why the sweep missed it.** The check the history record describes resolves "jeden Pfad in
Rückwärtsanführungszeichen über seinen Marker-Glob". A token with no `/` in it has no path to
glob, so it drops out of the candidate set before the resolution runs. The blind spot is the
candidate selection, not the resolution — the third variant of the pattern this project already
carries twice.

## Direction

Give the reference its store, in the form the rest of the file now uses:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_*_spec-navigator-geruest.md`.
The paragraph wraps at 78 characters and the path is one unbreakable word, so it takes its own
line, as the two references below it do.

Whoever runs the next path survey over this file selects candidates by "backticked token that
names a workbench record", not by "backticked token containing a slash".
