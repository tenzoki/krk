The record claims the rest of CLAUDE.md was counted against the tree, and two statements in it do not hold

---
Commit `522cf51` and `history/260817-2356-coder-e15-kommentare-und-claude-md.md` both state
"Die übrige Datei ist gegen den Baum nachgezählt und stimmt", and the record names what was
counted: `Wirkungsbereich` seven, `Bereich` fünf, `Fokus` fünf, `Kommando` without a number and
without a Git variant. All four check out. The sentence around them claims more than the four,
and the wider claim is false: the paragraph at `CLAUDE.md:39` carries two statements the tree
contradicts.

---

**Severity:** Low. No code depends on it. It is filed because the claim is the kind
`rules/critical-stance.md` §3 singles out: "I checked it" stated over a surface wider than the
one that was actually read. The next pass that trusts the sentence will not re-check CLAUDE.md,
and the two stale statements sit in the section every session reads first.

**Found by:** coderev, review `reviews/260818-0024-coderev-bundle-e-the-prose-and-the-records.md`
**Affected:** commit message `522cf51`, `history/260817-2356-coder-e15-kommentare-und-claude-md.md`
**Cross-references:** `shared/issues/260816-2138_o_claude-md-nennt-zehn-gefahrene-runden-es-sind-elf.md`,
`shared/issues/260818-0028_o_claude-md-says-the-bundle-ships-as-v0-4-1-and-four-tags-have-been-set-since.md`
**Tree state:** `da716c1`
**Domain:** code

## What the four named counts are, measured

```
$ awk '/^pub enum Wirkungsbereich/,/^}/' crates/krk-core/src/tasten/belegung.rs \
    | grep -cE '^    [A-ZÄÖÜ][A-Za-z]*,'
7
$ awk '/^pub enum Bereich/,/^}/' crates/krk-ui/src/fenstermodell.rs | grep -cE '^    [A-Z][A-Za-z]*,'
5
$ awk '/^pub enum Fokus/,/^}/' crates/krk-ui/src/kommandos/fokus.rs | grep -cE '^    [A-Z][A-Za-z]*,'
5
$ awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs | grep -ciE 'git'
0
```

CLAUDE.md states seven, five, five, and no Git variant, and carries no number for `Kommando`.
Four for four. The edited paragraph at `:140` also holds: `delete` reaches
`papierkorb_oder_zeichen_zurueck` and from there `loeschen_nach_rueckfrage`
(`anwendung.rs:4506`, `:4536`, `:4459`, `:4621`), the rule has exactly one caller and the probe
`die_regel_hat_genau_einen_aufrufer` holds the count.

## The two statements outside those four

| Statement in CLAUDE.md | Measured at `da716c1` |
|---|---|
| `:24` "**Zehn Runden sind gefahren.**", with a ten-row table | Twelve. `ls fusion-workbench/circles/*/*_circle.md` gives fourteen, one `_a_` (never run) and one `_d_` (deferred). Already open as `shared/issues/260816-2138_o_*`, and stale a second time since that record was filed. |
| `:39` "liegt als `v0.4.1` aus" | `Cargo.toml:13` is `0.5.1`; four tags stand after `v0.4.1`. Filed as `shared/issues/260818-0028_o_*`. |

Both statements sit in `## Worum es geht` and `## Projektstand`, which is where "die übrige Datei"
begins.

## Direction

Nothing to change in CLAUDE.md under this record — the two defects have their own records in the
shared store, where they belong, because neither arose from this Circle's Directive. What belongs
here is the sentence in the session record. Narrow it to what was measured: the four
enumerations and the `Kommando` paragraph, named, with the commands that measured them. A claim
that names its scope survives the next reader; one that says "the rest of the file" does not.

---
Resolved: 260818-0201 by analyst — **an addendum, not a rewrite**, appended to
`history/260817-2356-coder-e15-kommentare-und-claude-md.md`. The log records a state and keeps its
wording; the addendum narrows the claim and carries the measurement.

**Re-measured independently at `ae665e5` rather than taken from this record.** The four
enumerations this record names all hold, by the four commands it quotes: `Wirkungsbereich` seven,
`Bereich` five, `Fokus` five, `Kommando` with no Git variant. So does the edited Rückschritt
paragraph. The two statements it calls false are false, both by direct measurement: fourteen Circle
records against a ten-row table, and `Cargo.toml:13` reading `0.5.1` with four tags standing after
`v0.4.1`.

**The survey went past this record, because "the rest of the file" cannot be answered by checking
the two statements someone already found.** Five further claims of `CLAUDE.md` were measured for
the addendum and all five hold — the two `#![allow(unsafe_code)]` sites, the 38-of-40 coverage of
the macOS-floor section with exactly the two named exceptions, the pinned toolchain `1.97.1`, a
`Cargo.lock` with no `cc` and no `-sys` package but `windows-sys`, and the single hull around
`NSPasteboard`. That is a fuller answer to "does the rest of the file hold" than either the record
or the sentence it corrects, and it is written into the addendum so the next pass can see where
the ground has been walked.

**Two sentences beyond the two are named in the addendum and are not separately filed**, because
they are arithmetic on the same wrong number rather than independent claims: `:39` "Was die Runden
2 bis 10 hinzugefügt haben" and `:78` "er liegt vor den Runden 5 bis 10 — keine der sechs ist gegen
die zehn Zusagen gemessen". Whoever corrects the round count corrects them in the same pass; a
separate record would be three records for one edit.

**Nothing in `CLAUDE.md` was changed.** Both defects have their own records in the shared store, as
this record's `## Direction` says they should, and the file is outside the scope of this task.
