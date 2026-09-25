# C8 — The working tree, upward and in the selection

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 8 (second step of bundle C)
**Tree state before the task:** `c260e64`
**Verification:** `make check` — exit 0

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`
> as of 260817. The code itself stays German, identifiers and prose alike: the tree is
> German throughout and existing artifacts are not translated.

## What the task asked for

A new module `krk-core/src/verzeichnis/arbeitsbaum.rs` answering the fifth trigger of the
loud confirmation: does this delete operation touch a Git working tree? Three functions,
all `#[must_use]` — a single-path check for an immediate `.git` entry, an upward walk from
the resolved folder to the user directory or the root, and a composite that runs the
selection loop only when the walk says `Nein`. No caller in `krk-ui`; that arrives in
step 10.

## The naming decision the plan left to the executor

**The third function is called `beruehrt_einen_arbeitsbaum`.** The plan's API table named
it `arbeitsbaum::befund`, and a function of that name returning a `Loeschzielbefund` would
have stood beside `Ordnermodell::befund` returning a `modell::Befund` — the very confusion
that commit `17d3550` resolved one level up, rebuilt one level down. The plan therefore
required a name taken from the question rather than from the return type.

"Beruehren" is the verb that carries both halves of the question: the operation **lies in**
a working tree (upward) or it **takes one along** (the selection). `liegt_in_arbeitsbaum`
alone names only the first half, and it is already the name of the upward walk.

Three alternatives were weighed and rejected; the module header records each with its
reason.

- `traegt_der_ast_einen_arbeitsbaum`, the plan's own second suggestion. "Ast" is not a word
  of this module tree, it would need its own definition here, and it names a shape in the
  tree rather than the question. The checked levels lie **above** the folder and the
  selected entries **below** it, which does not make one branch.
- `liegt_das_loeschziel_darin`. `Loeschziel` becomes a `struct` in step 9 with a field
  `arbeitsbaum` that this very function fills; the name would read as "takes a
  `&Loeschziel`" and be circular.
- `wird_beruehrt`. Reads best at the call site and stands for nothing without the module
  path. A name has to carry when imported, and `use …::arbeitsbaum::wird_beruehrt;` is
  legal.

That the name repeats its module's noun is the shape of `inhalt::traegt_der_inhalt` and
not an oversight.

## How the first-hit abort was measured rather than claimed

**The abort is invisible in the return value, and that follows from step 6.**
`Loeschzielbefund::oder` makes `Ja` absorbing — deliberately, so that the order in which a
caller collects facts does not matter. A walk that keeps going after the first hit
therefore returns the same `Ja`. No tree built from real folders distinguishes the two
cases, however it is arranged.

The consequence for this step: a `Pruefordner` probe **cannot** check the cost promise. So
the two loops are separated from the access they perform. Private `aufwaerts_mit` and
`beruehrt_mit` take the check as an `impl FnMut(&Path) -> Loeschzielbefund`; the two public
functions are one line each, substituting `traegt_arbeitsbaum`. The in-module probes pass a
`Mitschrift` that answers from a table and records who it was asked about, then compare the
**list** of visited levels. That measures the access pattern.

Three promises are measured this way, each of which would otherwise be an assertion:

1. the upward walk stops at the first hit — three levels visited, `/a` and `/` not;
2. the selection is not consulted at all when the walk says `Ja`;
3. the selection loop stops at the first hit — the third of three entries is not visited.

What stays unmeasured, and must: that the public functions really substitute
`traegt_arbeitsbaum`. That is one line each and checkable by eye, and the real-tree probes
drive the same cases through the file system anyway.

**One stop promise is observable at the return value**, and it is checked with real
folders: the boundary at the user directory. A `.git` above the boundary must not be found.
The probe carries its own negative control — the same tree without a boundary must produce
`Ja`, otherwise the probe checks nothing.

## Two design decisions the task did not settle

**The boundary at the user directory is inclusive.** The user directory itself is still
checked, and only then does the walk stop. Excluding it would leave a blind spot exactly at
the boundary, because a home directory carrying a `.git` is not an invention but the
ordinary shape of a working tree for configuration files. The price is named in the module
header: with a `.git` there, **every** deletion below the home directory is loud for that
reason. That is the user's answer carried through consistently, not an added case.

**A path that is not absolute is `Unentschieden` before anything is read.** Otherwise
`Path::new("").join(".git")` would ask about `.git` **in the process's working directory**
and return an answer about a folder nobody asked about. The single check sits in
`traegt_arbeitsbaum`, the only place in the module that touches the file system, and
therefore covers all three functions. A probe distinguishes the check from its absence:
without it the empty path gets a decided `Nein` from `crates/krk-core`.

## Two consequences named in prose rather than closed in code

**A selected symbolic link pointing at a working tree answers `Ja`**, although only the
link would be removed. Intermediate path components are followed, which is what a path
means in a file system and not a choice of this module. The error runs in the loud
direction, and closing it would cost a second `lstat(2)` per selected entry. Given the
reach the user chose — in a source tree the loud form is the normal case — that would be an
access per entry for a difference nobody in this project will see. A probe pins the
behaviour so that a silent change turns it red.

**An `Unentschieden` from the upward walk also cuts off the selection.** The plan states
"only if it says `Nein`", and the cheaper form was its choice. The confirmation is already
loud, but its reason reads "could not be classified" instead of "from a Git working tree",
even where a selected entry might have supplied the exact reason. The case requires a
folder between the target and the boundary that cannot be read, and is therefore rare. A
probe pins the choice so that it does not flip unnoticed.

## What `ENOTDIR` costs if it is not separated from `ENOENT`

`traegt_arbeitsbaum` maps both `NotFound` and `NotADirectory` to `Nein`, and the second
branch is not cosmetic. A file pane's selection carries ordinary files, and `lstat(2)` on
`datei/.git` fails with `ENOTDIR`, not `ENOENT`. Without that branch every selected file
would make the confirmation undecided and therefore loud — in a file manager, the normal
case. The catch-all arm over `io::ErrorKind` is unavoidable, because the type is
`non_exhaustive`; it runs in the cautious direction.

## What was written

- `crates/krk-core/src/verzeichnis/arbeitsbaum.rs` (new, 689 lines): the three public
  functions, the two private loops, the constant `VERWALTUNGSEINTRAG`, and 13 in-module
  probes with the `Mitschrift`.
- `crates/krk-core/tests/arbeitsbaum.rs` (new, 313 lines): 11 probes over real trees via
  `Pruefordner`, including the five cases the task named individually.
- `crates/krk-core/src/verzeichnis/mod.rs`: `pub mod arbeitsbaum;`, the module count from
  twelve to thirteen, the dependency sketch, a header paragraph, and a note on
  `aufwaerts`, which now has a second caller inside the core.

The module header carries the three points the plan step requires: no Git integration is
created — presence of the `.git` entry is checked, not its content; the boundary at the
user directory limits cost alone, because a path above it already goes loud through the
first trigger; and the consequence of the user's reversal at the spec gate, which in this
project makes almost every deletion loud. The polarity is recorded on each function: the
first one, where `Ja` is the warning ground and `Unentschieden` belongs with it.

No re-exports at the `verzeichnis` level, on the precedent of `umfang::zaehlen`: the module
name is the subject of all three questions and should stand at the call site.

## The five named cases, individually

| Case | Probe |
|---|---|
| working tree at the folder itself | `der_arbeitsbaum_am_ordner_selbst_wird_gefunden` |
| two levels above | `der_arbeitsbaum_zwei_ebenen_darueber_wird_gefunden` |
| none in the whole branch | `ohne_arbeitsbaum_im_ganzen_ast_bleibt_es_ruhig` |
| selected subfolder as root | `ein_ausgewaehlter_unterordner_als_wurzel_wird_gefunden` |
| abort at the first hit | `der_aufwaertsgang_bricht_beim_ersten_treffer_ab`, `die_schleife_ueber_die_auswahl_bricht_beim_ersten_treffer_ab`, `die_auswahl_wird_nur_bei_nein_gefragt` (in-module, with the recording check) |

The second case writes out both answers, so that the difference stands on the page: the
folder itself answers `Nein`, and the narrow form the user reversed at the spec gate would
have stayed silent here. That is the path of the 260817-0344 incident.

Beyond the five: `.git` as a **file** (Git's linked working tree) is the same hit; a
working tree **at** the user directory is still found; a working tree **above** the
boundary is not; files and vanished paths in the selection stay quiet; an over-long name
component produces `Unentschieden` on a real path and carries it through the walk. The last
one goes through `ENAMETOOLONG` and not through withdrawn permissions on purpose: a probe
using `chmod 0o000` would not hold under a run as `root`, where the permission check falls
away, and would then silently assert the opposite.

## Verification

`make check` — exit 0. All four acceptance commands green: 1,299 probes passing across the
workspace, 10 ignored, among them the 24 new probes of this step (13 in-module, 11 over
real trees). `cargo fmt --all` ran once before the closing pass; three calls in the test
module needed line breaks.

The load-dependent race probe `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`
(`tests/text.rs`) was green in this run. It is described by two open records,
`shared/issues/260816-0055_o_…` and `shared/issues/260815-1019_o_…`, and is not a finding
of this step.

## What this step did not touch

No caller in `krk-ui` — that is step 10. No `Warngrund`, no `Loeschziel`, no table of
triggers — that is step 9. `CLAUDE.md` untouched: the module is new and the count of
modules under `verzeichnis` is not named there.
