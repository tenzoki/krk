# Code review — Bundle D, the removal of the final delete

**Reviewed-range:** `cdde9da..f7a85c1`
**Not-opened:** none
**Sender:** coderev
**Tree state at review:** `f7a85c1`
**Plan under review:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, steps 12 to 14

## Summary

Bundle D removes `Kommando::EndgueltigLoeschen` and `Art::EndgueltigLoeschen` from the program,
the entry `endgueltig_loeschen` from the shipped key map, and pulls seven prose numbers about the
size of the key map back to the tree. All three steps do what the plan asks, the four acceptance
commands are green, and the security-relevant backspace rule is untouched in its logic. Four
defects were found, one of them Medium: the key map head states the size of the Norton row four
times and all four are now off by one, and no remaining step of the plan reaches the file.

## Totals

| Severity | Count |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 3 |

## Acceptance state

Measured at `f7a85c1` with `export PATH="$HOME/.cargo/bin:$PATH"`:

```
cargo build --workspace                              ok
cargo test --workspace                               ok, 1321 passed, 0 failed, 10 ignored
cargo clippy --workspace --all-targets               ok
cargo clippy --workspace --all-targets -- -D warnings ok
cargo fmt --all --check                              ok
```

The `-D warnings` run matters here and is not part of `make check`: it is the run under which
`unused_must_use` and `dead_code` are errors, and a removal of this size is exactly the change
that leaves an unreachable private function behind. Nothing was left behind.

## The four counts the plan hinges on

Each measured against the tree, not against the plan's arithmetic:

| Claim | Measured | Command |
|---|---|---|
| `Kommando` carries 78 variants | 78 | `awk '/^pub enum Kommando/,/^}/' … \| grep -cE '^    [A-ZÄÖÜ][A-Za-z]*,$'` |
| `Kommando::KENNUNGEN` is `[…; 78]` | 78, and the length stands in the type | `belegung.rs:647` |
| 84 functions ship | 84 entries, 84 ids, no duplicate | `grep -c '^\[\[funktion\]\]'` |
| 89 combinations ship | 89, over five empty lists | counted over every `tasten = ` line |

`84 = 78 + 6` closes against the six menu-delivered text commands, which is the identity the
whole prose of `belegungsausgabe.rs` and `menue.rs` is built on, and every one of those
statements now reads 84 and 78.

## Findings by theme

### Counts that no step holds

**M1 — the key map head says "six" Norton functions in four places and there are five.**
`resources/default-keymap.toml:9`, `:170`, `:640`, `:849`. The Norton block from `:129` to `:161`
holds `vorschau_umschalten`, `kopieren`, `verschieben`, `ordner_anlegen` and `in_papierkorb` —
five functions, five Cmd shortcuts — and the same file excludes `bearbeiten` from the two-ways
rule in its own comment at `:170`. Before `82707ef` it was six and six.

Nothing downstream corrects it. Step 13 named exactly two head changes and both were made. Step
15's scope is `grep -rniE "endgueltig|endgültig" --include="*.rs" crates`, which reaches neither
`resources/` nor a `.toml`, and none of the four lines carries the word. Step 17 cuts the
`Endgültig löschen` row out of the round-1 Cmd-shortcut table, which leaves line 9 pointing at a
five-row table while saying six, and does not touch this file.

Record: `issues/260817-2243_o_the-keymap-head-says-six-norton-functions-in-four-places-and-there-are-five.md`

### Citations that cannot be found

**L1 — two decision paths in the key map head are split across comment lines.**
`resources/default-keymap.toml:12-13` and `:66-67` break each path after `shared/decisions/`, so
`grep -rn "shared/decisions/260817-0536" resources/` finds nothing. The line directly above at
`:11` carries its whole path unbroken, and so do all seven citations of the same records
elsewhere in the tree. It is not a width rule: the file already holds 21 lines over 80
characters, the longest at 199, and the broken lines are 97 and 96 after the break.

Record: `issues/260817-2243_o_two-decision-paths-in-the-keymap-head-are-split-across-comment-lines-and-escape-every-search.md`

### Prose about symbols this commit deleted

**L2 — `loeschwarnung.rs:166-167` still says `operationen::loeschfrage` "faellt … weg".** It fell
in `82707ef`. The same commit rewrote `:254-256` of the same header from "`f8` kommt erst mit
Buendel D dazu" to "`f8` ist mit Buendel D dazugekommen", so the tense of this header was under
the executor's hand and this one sentence was left behind. Step 15's search does reach it; it is
filed here because the sentence names a symbol Bundle D deleted, in a file Bundle D edited.

Record: `issues/260817-2243_o_the-loeschwarnung-module-header-still-says-loeschfrage-will-fall-and-it-fell-in-the-same-commit.md`

### A signature wider than its contract

**L3 — `loeschen_nach_rueckfrage` takes an `Art` that admits three values its contract forbids.**
`anwendung.rs:4620`. One caller, one legal variant, no guard. No dead branch survives — both
parameters are pure pass-through and `-D warnings` is clean — but the safeguard that covered the
neighbouring parameter was deliberate and fell with the thing it guarded.

Record: `issues/260817-2243_o_the-delete-body-takes-an-art-that-admits-three-values-its-own-contract-forbids.md`

## The four points the dispatch asked about

**1. The enum `Loeschtexte` fell with its second value. Are the consequences carried?** Yes,
completely. `grep -rn "Loeschtexte" crates/` returns nothing, `Self::loeschtexte` lost its
`textform` parameter and its `match` in the same edit, its `#[must_use]` survived, and clippy
under `-D warnings` finds no unreachable code. The enum was built so that the removal of its
second value would stop the build at the branch that had to go; it did that and then fell itself,
which is what its own doc comment said would happen.

**2. Does `loeschen_nach_rueckfrage` carry dead distinctions?** No. Neither `art` nor
`schaltflaeche` drives a branch. `art` travels into the `Cell` at `:4682`, out again in the
sheet's callback, and into `loeschauftrag_stellen`, which puts it into the `Auftrag` unread.
`schaltflaeche` goes straight to `loeschbestaetigung::zeigen`. What is left is only that the
signature admits three values the body's own five-stage description forbids, which is L3 above
and a design call, not a dead branch.

**3. Do the five new prose statements about `f8` and `opt+cmd+delete` hold?** All five, and three
of them were re-derived from the code rather than read:

- `ereignisse.rs:306` — "`f8` faellt an der zweiten Haelfte heraus, denn es ist keine
  Rueckschritt-Taste". `ist_nackter_rueckschritt` is
  `self.druck.maske.ist_leer() && self.druck.code == code_von_pflicht("delete")`. The function and
  numeric-pad flags are stripped by the normalisation before the lookup, so `f8` arrives with an
  empty mask and fails at the code comparison — the second half, as stated, not the first.
- `anwendung.rs:4479-4481` — "`f8` faellt an derselben Frage heraus". Same predicate, and
  `papierkorb_oder_zeichen_zurueck` returns `self.in_den_papierkorb()` at `:4510` before the rule
  is consulted.
- `rueckschritt.rs:76-81` — "Die uebrigen Loeschwege erreichen die Regel nie". Verified at the
  early return above: `rueckschritt(…)` at `:4515` is unreachable for `f8` and for `cmd+delete`.
- `loeschwarnung.rs:228-229` — "die drei Tasten `delete`, `cmd+delete` und `f8` und der
  Menueeintrag … laufen durch `in_den_papierkorb` hindurch". `in_den_papierkorb` has exactly three
  call sites, all three inside `papierkorb_oder_zeichen_zurueck`, and `loeschen_nach_rueckfrage`
  has exactly one caller.
- The two `ontocoder` sentences at `resources/default-keymap.toml:62-67` and `:703-707` match
  `shared/decisions/260817-0536_a_bekommt-f8-den-papierkorb-…` word for word on both halves: `f8`
  takes the trash with three combinations, `opt+cmd+delete` stays unassigned, and the Finder
  meaning "sofort löschen" is the record's own reason.

One remark that did not reach the bar for a record. The record closes with "Die Kombination steht
damit einer späteren Runde zur Verfügung"; the key map head says "Sie wird nicht neu vergeben"
and does not carry that half. In the context of a file that describes the shipped state the
sentence reads as "the delivery does not reassign it", so it is not wrong — but the key map head
is the only place a later round would look, and it now sounds more permanent than its source.

**4. The three numbers step 14 deliberately did not change.** All three are right today, and the
reason given — that the removal made them right by itself — checks out against the history:

| Place | Says | Is | Was it right before? |
|---|---|---|---|
| `menue.rs:128` | "die einzigen der 84 Funktionen ohne Kommando" | 84 = 78 + 6 | no; it said 84 while 85 shipped |
| `menue.rs:799` | "als einzige der 84 Funktionen kein Kommando" | 84 | no, same |
| `belegung.rs:407` | "die Schreibweise der 77 vorhandenen" | 78 − 1 = 77 | no; it said 77 while 79 `Kommando` existed |

All three were written at `d73be91`, "die Belegung ihren 84. Eintrag", when the counts were 84
and 78. `37ca972` then added the 85th entry and the 79th command without pulling them, and
`82707ef` took the same one back out. `git show cdde9da:crates/krk-ui/src/appkit/menue.rs` shows
`:128` already reading 84 before this range. So they were stale for one round and are right again
— which is the correct disposition to leave them at, but it is a coincidence and not a mechanism.

## The standing questions of this project

**The one-line rule of the key map holds.** 84 `[[funktion]]` blocks, 84 `id =` lines, no
duplicate id.

**The block order is unchanged.** `diff` of the id lists before and after the range shows exactly
one deletion and no move (`5d4 < id = "endgueltig_loeschen"`). Since the order steers the menu
order and no probe holds it, this was checked as a list and not by reading.

**Every touched file under `crates/krk-ui/src/appkit/` still carries its macOS-floor section.**
`anwendung.rs:168`, `ereignisse.rs:234`, `menue.rs:179`. No file was added or removed under that
directory, so the tree's coverage is what it was.

**The backspace rule is untouched in its logic.** `git diff` on
`crates/krk-ui/src/kommandos/rueckschritt.rs` over the whole range is 11 lines and all of them
are `//!` header prose. The four-row table, the three truth values, the enum `Rueckschritt` and
the function body are byte-identical. The one caller is unchanged in its structure: the naked
backspace still reaches the rule and everything else still returns early.

## Cross-cutting observation

Three of the four findings are the same shape: a statement about a count or a symbol that was
true when it was written, is false now, and sits in a place no automatic check and no planned
step reaches. The project already knows this shape — `shared/issues/260812-2253_*`,
`shared/issues/260812-1438_*` and `shared/issues/260810-1851_*` are three earlier instances — and
Bundle D adds a fourth surface to it, `resources/`, which step 15's sweep excludes by
construction because its search is scoped to `--include="*.rs" crates`.

The pattern is not that the executors were careless. Step 14 exists precisely because prose
numbers drift, and it pulled seven of them correctly. What it did not have was a scope that
covers the data file, and step 15 does not have one either.

## Recommended sequencing

Nothing here blocks the bundle. All four are prose or signature shape; none changes behaviour and
none touches the security-relevant branch.

1. **Fold M1 and L1 into step 15.** Both live in `resources/default-keymap.toml`, both are one
   edit, and step 15 is the comment sweep. Its scope needs one addition:
   `resources/default-keymap.toml` alongside the `.rs` search, so the data file stops being
   outside every sweep.
2. **L2 goes with step 15 as it stands** — its search already reaches the line.
3. **L3 is a design call for a later step or a later round**, not a fix for this bundle. Two ways
   are named in the record.
