# T9 — The trigger table, and the rename that made it namable

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 10 (fourth step of bundle C), plus a user decision of 260817-1640 that lands on step 9
**Binding:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C3 — the ranking of the triggers and their wordings
**Tree state before the task:** `3fcd375` plus the uncommitted steps 9 and 10 of earlier tasks
**Verification:** `make check` — exit 0

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`.
> The code stays German, identifiers and prose alike; existing artifacts are not translated,
> so the execution annotations added to the German plan are German.

## Four operations, one subject

The task carried four items, and they are one thing: the table of triggers, and the names the
table is fed with.

1. The rename `ist_lokal` → `liegt_auf_netzlaufwerk` (user decision, 260817-1640).
2. Step 10 — `Warngrund`, `Loeschziel`, `warngruende`, and the reasons entering the two texts.
3. The open polarity record `260817-1419_o_…`, which this task was supposed to satisfy and
   instead partly falsified.
4. The module list in `kommandos/mod.rs`, still describing `loeschwarnung` as texts only.

## 1 — The rename

`volumes::ist_lokal` returned `Ja` for "local", i.e. for harmless; the field consuming it,
`Loeschziel.netzlaufwerk`, carries `Ja` for "is a network volume", i.e. for warn-worthy. Same
type on both sides, no compiler check, and `Unentschieden` is a fixed point of the inversion — so
a swap would have kept "undecided counts as loud" visibly true while the *stated reason* was
wrong in both decided cases. The user chose way 1 of
`260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md`: the function is named
after the trigger and returns the trigger's answer, and the inversion happens once, in the body,
beside the module header that explains it.

What moved: the name, one `if`/`else` swap, the polarity section of the module header, four
probes. What did not: the resource value is still `NSURLVolumeIsLocalKey`, so the availability
section is untouched, and nothing outside `volumes.rs` referenced the old name — `grep -rn
ist_lokal crates/` had exactly one file before the change.

Two probes flipped their expectation, and both are real measurements rather than restatements:
the user's home directory answers `Nein`, and the `autofs` mount under
`/System/Volumes/Data/home` answers `Ja`. The second is the only negative-of-the-old-sense
evidence in the file and the reason a hardcoded answer cannot pass; its doc comment now also says
why an `autofs` automount is the right thing to call a "Netzlaufwerk" here — the trigger hangs on
the missing `local` flag, not on a protocol.

**The counting probe kept its name and changed its subject.** After the rename `ist_warnwuerdig`
is the *correct* question for this value, so `hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt`
lost the hazard it was written against. It stays, because a second and unchanged promise is left:
this module *answers* the trigger and does not *judge* it. Whether the confirmation goes loud, and
which of the six reasons it names, is decided once, in `warngruende`. A file that asks its own
answer for warn-worthiness lays down the C3 ranking a second time, and the two would drift with
nothing to stop them.

Inverting the probe was considered and rejected: an assertion that this file *does* ask would turn
a module boundary into an obligation to cross it. Dropping it was rejected because the boundary is
real and nothing else holds it. The reasoning sits at the probe's doc comment, per the task.

## 2 — Step 10, the trigger table

In `crates/krk-ui/src/kommandos/loeschwarnung.rs`:

- `pub enum Warngrund`, seven values in the spec's C3 ranking, `Ord` derived.
- `pub enum Umfangsgrund { GenauDieSchwelle, MehrAlsDieSchwelle }` beside it.
- `pub struct Loeschziel` with the five fields the plan names.
- `#[must_use] pub fn warngruende(ziel: &Loeschziel) -> Vec<Warngrund>`, sorted and deduplicated.
- `frage_und_erlaeuterung` takes the reasons; the first goes into the question, the rest into the
  explanation.
- The table written out in the module header, cases written out one by one in the probes, and a
  caller count over `crate::quellbaum`.

### Why `Warngrund::Umfang` carries a value

The sixth trigger has two wordings, „mit 25 Einträgen" and „mit mehr als 25 Einträgen", so
`wortlaut()` cannot form it from a payload-free variant. Three ways were weighed:

| way | why not |
|---|---|
| `Warngrund::Umfang(Umfang)` | needs `Ord` derived on `krk_core::verzeichnis::Umfang` — a fourth file, outside this task's bounds, and an ordering over `Genau`/`MehrAls`/`Unentschieden` would be a claim with no subject |
| a hand-written `Ord` on `Warngrund` | the plan asks for a derived one, and a manual impl is a second place the ranking could drift from the declaration order |
| eight variants, splitting the scope trigger | contradicts the plan's "seven values" and puts two ranks where the spec has one |

What stands instead is a two-value enum next to `Warngrund`, carrying **no number**: the number is
`SCHWELLE` in every case that can reach this trigger, because `umfang::zaehlen` caps at
`SCHWELLE + 1`, so `Genau` above the threshold cannot occur. A `const _: () = assert!(SCHWELLE ==
25, …)` binds the two spelled-out wordings to the constant at compile time — the idiom
`appkit/editor.rs` uses to bind the undo budget to the editor limit. `Warngrund` keeps seven
variants with derived `Ord`; the probe table has eight rows because the scope trigger has two
wordings.

The type's doc comment states outright why `Ord` is derived here and deliberately not on
`Loeschzielbefund`, so the pair does not read as an inconsistency: there an ordering would be a
claim with no subject, here it *is* the content. A derived `Ord` in this project therefore means
"the order carries a promise", not "these values are comparable".

### The one substantive design decision: an undecided input does not name its own trigger

`netzlaufwerk == Unentschieden` yields `Unentscheidbar` and **not** additionally
`Warngrund::Netzlaufwerk`. KRK does not know whether the volume is one; naming it in the
explanation would be a claim with no measurement. That is exactly C3's acceptance criterion
("nennt als Grund, dass das Ziel sich nicht einordnen ließ"), and "Unentschieden gilt als laut"
stays fully kept because `Unentscheidbar` sits at rank 1 and makes the sheet loud.

The consequence runs against what the task expected: `Loeschzielbefund::ist_warnwuerdig` gets no
caller here either. It merges `Ja` and `Unentschieden`, and those two are precisely what has to be
kept apart, because they lead to *different* reasons. Every check therefore writes all three
answers out, exhaustively and without a catch-all. See item 3.

### The rest of the mechanics

- Triggers 1, 2 and 4 are computed in the function from the two resolved paths. They hang on the
  pair: if either path is `None`, none of the three is answerable and one single
  `Unentscheidbar` results.
- The cloud places are `~/Library/CloudStorage` and `~/Library/Mobile Documents`, compared with
  `Path::starts_with`, i.e. component by component — `CloudStorageAlt` is not one, and everything
  below a cloud place is.
- Duplicates: sort by the derived `Ord`, then `dedup`. Three undecided inputs give one reason, not
  three. A probe holds that.
- Four is the largest set of reasons reachable with every input answered: a cloud place lies below
  the home directory, so it excludes both home-directory triggers, and those exclude each other.
  Two probes cover the four-reason case and the same target with the volume undecided, which puts
  `Unentscheidbar` in front and removes `Netzlaufwerk` from the list.
- The quiet form is byte-for-byte what it was: with an empty list the reason slot expands to
  nothing, and `ohne_grund_bleibt_die_ruhige_form_unveraendert` holds both strings whole.

### Two things the plan does not discuss

**The caller count expects zero, and the probe's name says so.** `warngruende`'s one caller
arrives with step 11. A probe expecting one today would be red; one expecting "at most one" would
be green forever and measure nothing. So it is
`die_ausloesertafel_hat_noch_keinen_aufrufer`, and its doc comment says step 11 sets expectation
and name to one, at the same time as the `expect(dead_code)` on `warngruende` becomes unfulfilled.
Step 1 carried the same shape for `frage_und_erlaeuterung`.

**One line in `appkit/anwendung.rs` moved.** `frage_und_erlaeuterung` gained its third argument,
so `in_den_papierkorb` passes `&[]` and stays in the quiet form; the comment there names step 11
as the place that puts the reasons in. Nothing else in that file was touched — gathering the facts
and the loud form are step 11.

## 3 — The open polarity record `260817-1419_o_…`

**One of its claims no longer holds, and the progress note says so.** The record predicted that
bundle C would bring the first call site of `ist_warnwuerdig`. Step 10 wrote the one place that
would have made that call and does not make it, for the reason above: the function merges exactly
the two answers `warngruende` must keep apart.

What the record asked for, and where it stands: the count in `volumes.rs` stands but with a
changed subject (see item 1); the count in `appkit/papierkorb.rs` is not done and that file is
outside this task's bounds; the count in `kommandos/loeschwarnung.rs` is **not** done and would
now over-promise, because after step 10 that single file carries both polarities — the trash
answer (`Ja` is permission, the merged question forbidden) and the trigger answers (`Ja` warns,
where the merged question would be allowed and merely useless). A file-level zero would state a
ban that applies to one function and not the other, and the next person adding a polarity-1
consumer would have to break a green probe to write correct code. What holds instead is written
out at `warngruende` and in the module header.

The record stays open. Its substance — the polarity belongs to the question, not to the value, and
one three-valued type cannot make the swap uncompilable — is the second way and is untouched. Its
own cost argument has expired, though, and the note records that too: "Bündel C berührt beide
Dateien ohnehin" was true when it was written and is no longer, because bundle C is now past both.

## 4 — The module list in `kommandos/mod.rs`

The entry described `loeschwarnung` as "die Texte der einen Rückfrage … und wie ein Ziel
eingeordnet wird". Two rounds of growth later the module carries the stage sequence (T5), the
trigger table with its ranking (this step) and the texts that follow from both. The entry now says
that, and names C4 alongside C2 and C3, since `ohne_papierkorb` lives there too.

## Files changed

- `crates/krk-ui/src/appkit/volumes.rs` — rename, the one inversion, the polarity section of the
  module header, four probes
- `crates/krk-ui/src/kommandos/loeschwarnung.rs` — `Warngrund`, `Umfangsgrund`, `Loeschziel`,
  `warngruende`, `CLOUDORTE`, `liegt_an_einem_cloudort`, the third argument to
  `frage_und_erlaeuterung`, the module header sections, and fifteen new probes (12 → 27 in the
  file, counted with `cargo test -p krk-ui loeschwarnung`)
- `crates/krk-ui/src/kommandos/mod.rs` — the module list entry
- `crates/krk-ui/src/appkit/anwendung.rs` — one call site, `&[]` passed through
- `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md` — execution annotations at steps 9
  and 10, the data structures, the API table, two diagram labels
- `issues/260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills.md` — `Resolved:`,
  marker `_o_` → `_c_`
- `issues/260817-1419_o_die-einzige-sicherung-gegen-den-polaritaetsfehler-…` — progress note, stays
  open
- `issues/260817-1720_o_the-question-can-read-diese-25-eintraege-mit-25-eintraegen.md` — new

## What still stands on the `expect(dead_code)` side

`volumes::liegt_auf_netzlaufwerk` keeps its
`#[cfg_attr(not(test), expect(dead_code, reason = …))]`, because this step sets **no** caller for
it — `Loeschziel.netzlaufwerk` is filled in step 11, in `appkit/anwendung.rs`. The same is true of
`krk_core::verzeichnis::umfang::zaehlen` and `arbeitsbaum::beruehrt_einen_arbeitsbaum`, which carry
no such attribute at all and never needed one: `krk-core` is a library and its items are reachable
from the crate root, so `dead_code` does not touch them. The new `warngruende` carries one for the
same reason `frage_und_erlaeuterung` did in step 1, and step 11 must remove it — with the caller in
place the expectation becomes unfulfilled and `-D warnings` stops the build.

Measured, not assumed. Both attributes were removed one at a time and
`cargo clippy --workspace --all-targets -- -D warnings` run on each:

```text
without the one on liegt_auf_netzlaufwerk:
  error: function `liegt_auf_netzlaufwerk` is never used

without the one on warngruende:
  error: variants `GenauDieSchwelle` and `MehrAlsDieSchwelle` are never constructed
  error: multiple variants are never constructed
  error: struct `Loeschziel` is never constructed
  error: constant `CLOUDORTE` is never used
  error: function `liegt_an_einem_cloudort` is never used
  error: function `warngruende` is never used
```

The second list is why there is **one** attribute and not six: `warngruende` is the root of the
reachability chain, and covering it covers the enum variants, the struct, the constant and the
helper. Both were restored and `make check` re-run to the exit code below.

## Verification

`make check` — exit 0. Build, 675 probes in `krk-ui` and 98 in `krk-core` green over the whole
workspace, `cargo fmt --all --check` clean, `cargo clippy --workspace --all-targets -- -D warnings`
clean. `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`, the probe that occasionally fails
under load, was green on the run that produced this exit code.

The plan step is **not** marked `[DONE]` and nothing is committed — both belong to the
orchestrator. One caveat worth naming: closing `260817-1623` used `git mv`, which stages the
rename, so that one path already sits in the index. Its content change is unstaged like everything
else.
