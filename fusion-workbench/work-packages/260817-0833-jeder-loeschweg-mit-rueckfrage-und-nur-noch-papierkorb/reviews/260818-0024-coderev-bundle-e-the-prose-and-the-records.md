# Code review — Bundle E, the prose of the tree and the records of round 1

**Reviewed-range:** `f7a85c1..da716c1`
**Not-opened:** none
**Sender:** coderev
**Tree state at review:** `da716c1`
**Plan under review:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, steps 15 to 17

## Summary

Bundle E does what the plan asks and does it carefully: the whole reviewed range changes comments,
key-map comments and workbench records only, no executable line moves, and `make check` is green.
The load-bearing question of the bundle — does any prose in the tree still assert the pre-260817
delete behaviour — comes back with one real answer: **four doc comments in `krk-ui/src/appkit/`
still say the wrong branch of the backspace rule removes files, while the rule's own module head
was rewritten in this bundle to say the opposite.** All four escape both sweeps the executor ran
because none of them carries the word "endgueltig". Six findings in total, one Medium, five Low.

## Totals

| Severity | Count |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 1 |
| Low | 5 |

## Acceptance state

Measured at `da716c1` with `export PATH="$HOME/.cargo/bin:$PATH"`:

```
make check                                            exit 0
  cargo build --workspace                             ok
  cargo test --workspace                              ok, 0 failed
  cargo fmt --all --check                             ok
  cargo clippy --workspace --all-targets -- -D warnings  ok
```

And the property that makes the whole bundle cheap to trust:

```
$ git diff 8f556ed..da716c1 -- crates resources \
    | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' | grep -vE '^[+-]\s*(//|#|$)'
(empty)
```

Not one changed line in `crates/` or `resources/` is anything but a comment or a blank. Point 3 of
the dispatch — the backspace rule must be untouched in table and body — is answered by that one
command: `rueckschritt.rs` changes at lines 1-2, 16-24 and the doc of `Rueckschritt::InDenPapierkorb`,
and nowhere else.

## The tragende Frage, measured

The executor reports the sweep falling from 51 to 33 lines. The count is **34**, not 33:

```
$ git archive 8f556ed crates | tar -x -C "$T"; cd "$T"
$ grep -rniE "endgueltig|endgültig" --include="*.rs" crates | wc -l
      51
$ git archive 522cf51 crates | tar -x -C "$T2"; cd "$T2"
$ grep -rniE "endgueltig|endgültig" --include="*.rs" crates | wc -l
      34
```

`git diff --stat 522cf51..da716c1 -- crates` is empty, so the working tree gives 34 as well.

**All 34 were opened individually.** The six classes the executor names are the right classes and
every one of the 34 lines falls into one of them. Three qualifications:

- **One line is in the wrong class, and it is the one the previous review had routed here.**
  `loeschwarnung.rs:167` sits under "datierte Rückblicke, in den Schritten 12 bis 14 geschrieben
  und richtig". It is neither: it says `operationen::loeschfrage` "**faellt** mit diesem Loeschweg
  weg" in the future tense, and the symbol fell in `82707ef`. The bundle-D review recommended in
  as many words "**L2 goes with step 15 as it stands** — its search already reaches the line"
  (`reviews/260817-2243-coderev-bundle-d-the-removal.md:206`). The search did reach it and the
  classification sent it back. The record
  `issues/260817-2243_o_the-loeschwarnung-module-header-still-says-loeschfrage-will-fall-…` stays
  open; it is not refiled here.
- With that one line, the claim "keine der 33 trifft mehr eine falsche Aussage" is not yet true.
  Everything else in the 34 does read correctly.
- Three line citations in the session record (`loeschbestaetigung.rs:73`, `:172`, `:179`) are one
  low against the committed tree: they were taken before the same commit edited that file.

## Findings by theme

### Prose that still asserts the old state

**F1 — Four doc comments say the wrong branch of the backspace rule deletes files. Medium.**
`issues/260818-0025_o_four-doc-comments-still-say-the-wrong-branch-of-the-backspace-rule-deletes-files.md`

`rueckschritt.rs:32-34` now reads "ihr falscher Zweig raeumt nichts mehr, er fragt". Four other
places describing the same rule still read the other way:

| Place | Text |
|---|---|
| `crates/krk-ui/src/appkit/anwendung.rs:4466-4467` | "**Der eine Zweig dieser Runde, dessen falsche Haelfte Dateien wegraeumt**" |
| `crates/krk-ui/src/appkit/anwendung.rs:2891`, `:2894` | "**Der eine Zweig, dessen falsche Haelfte Dateien wegraeumt.**" … "alles andere geht unveraendert in den Papierkorb" |
| `crates/krk-ui/src/appkit/anwendung.rs:2660` | "zwei Fassungen koennten auseinanderlaufen, und dann raeumte die falsche Haelfte Dateien weg" |
| `crates/krk-ui/src/appkit/ereignisse.rs:299-300` | the same sentence, in the doc of `Anschlag::ist_nackter_rueckschritt` |

The first of these sits two lines under a paragraph the same commit **did** rewrite. The last sits
eight lines above `ereignisse.rs:307`, which the executor's record examines by name and correctly
declares already right — the rest of that same doc comment was not read.

Why both sweeps missed all four: none carries "endgueltig", and the second sweep pattern
(`ohne rueckfrage|ohne nachfrage|opt\+cmd\+delete|beide loeschbefehle`) does not match a sentence
that names the *consequence* rather than the absence of the question. One search finds all four:
`grep -rniE "raeumte? .{0,20}(Dateien )?weg|wegraeumt" --include="*.rs" crates/krk-ui/src/appkit`.

### Measurement and claim

**F2 — The sweep reports 33 remaining lines, the search returns 34. Low.**
`issues/260818-0026_o_the-sweep-of-step-15-reports-33-remaining-lines-and-the-search-returns-34.md`

**F3 — The record claims the rest of CLAUDE.md was counted against the tree. Low.**
`issues/260818-0029_o_the-record-claims-the-rest-of-claude-md-was-counted-against-the-tree.md`

The four enumerations the record names are right, all four measured here: `Wirkungsbereich` 7,
`Bereich` 5, `Fokus` 5, `Kommando` with no number and no Git variant (`grep -ci git` over the enum
gives 0; `KENNUNGEN` is `78`). The edited paragraph at `CLAUDE.md:140` is right too, traced through
`anwendung.rs:4506` → `:4536` → `:4459` → `:4621`. What is not right is the sentence around them:
"die übrige Datei" also contains "**Zehn Runden sind gefahren**" (twelve are) and "liegt als
`v0.4.1` aus" (`Cargo.toml` is `0.5.1`, four tags stand after `v0.4.1`).

Both of those are defects in CLAUDE.md rather than in this bundle, and neither arose from this
Circle's Directive, so they sit in the shared store: the round count is already open as
`shared/issues/260816-2138_o_*`, and the version line is newly filed as
`shared/issues/260818-0028_o_claude-md-says-the-bundle-ships-as-v0-4-1-and-four-tags-have-been-set-since.md`.

### The key map

**F4 — The Norton block header promises a Cmd shortcut for all six of its entries. Low.**
`issues/260818-0027_o_the-norton-block-header-promises-a-cmd-shortcut-for-every-one-of-its-six-entries.md`

Step 15 correctly changed four counts from "sechs" to "fünf" and closed
`issues/260817-2243_c_the-keymap-head-says-six-norton-functions-…`. It left `:129-131`, which heads
the block "je zwei Wege" and states "Jede dieser Funktionen ist ueber die Funktionstaste und ueber
ein Cmd-Kuerzel erreichbar" over six entries, one of which (`bearbeiten`, `["f4"]`) has none. The
round-1 spec, rewritten in step 17, phrases the same fact correctly; only the key map's own block
header still claims otherwise.

### The records of round 1

**F5 — The Directive chronicle says the current state stands at its end. Low.**
`issues/260818-0030_o_the-directive-chronicle-says-the-current-state-stands-at-its-end-and-it-no-longer-does.md`

**F6 — Six more written-out markers, a known pattern.** No new record; appended as `Also seen` to
`shared/issues/260817-1130_o_die-sternform-fuer-zitate-gilt-seit-dem-260815-…`. Step 17 added eleven
written-out markers to the two round-1 records; five of them state the marker as the fact and fall
under the express exception, six are pure pointers and do not. The commit message of `24bbccc`
argues the spelling explicitly, but weighs `_i_` against `_a_` rather than against the star form.

## What holds, checked rather than assumed

**Both "more than a rename" changes are correct against the code.**

`Blatt::als_warnung` (`blaetter/mod.rs:566-570`) now justifies itself over the loud form and
`warngruende`. The wiring supports it exactly: `als_warnung` has one caller,
`loeschbestaetigung.rs:134`, guarded by `if laut`; `laut` is the third element of
`Self::loeschtexte`, which is `!gruende.is_empty()` with `gruende = loeschwarnung::warngruende(&ziel)`
(`anwendung.rs:4805`, `:4808`, `:4679`, `:4693`). The quiet form does not reach the call.

`rueckschritt.rs:16-40` now says the confirmation is the second lock and the case split is milder
but not superfluous. `Rueckschritt::InDenPapierkorb` maps to `self.in_den_papierkorb()`
(`anwendung.rs:4536`), which is `loeschen_nach_rueckfrage(Art::InDenPapierkorb, …)` (`:4459`), whose
fourth stage shows the sheet. The claim holds.

**The five decision records of step 16 are attributable, hash for hash.** Spot-checked with
`git log -S` on the core symbol, more than the two or three the dispatch asked for:

| Cited hash | Claim | `git log -S` |
|---|---|---|
| `472eb81` | every trash operation asks once, `loeschen_nach_rueckfrage` | earliest commit touching that symbol |
| `ee85950` | trash check before the sheet | wires `fuehrt_einen_papierkorb` (introduced in `e2760cd`) — the claim says "vor dem Blatt", which is `ee85950` |
| `792995a` | the loud form | matches |
| `82707ef` | `EndgueltigLoeschen` gone, key map entry gone, new probe | matches |
| `5a0f041` | `liegt_in_arbeitsbaum` | sole commit for that symbol |
| `c1b52db` | `Warngrund::Arbeitsbaum` | sole commit for that symbol |

The cited line numbers resolve too: `arbeitsbaum.rs:288` and `:338`, `loeschwarnung.rs:411`, `:486`,
`:642`, `anwendung.rs:4621`, `tests/belegung.rs:1642` each land on the named item. `grep -rn
"EndgueltigLoeschen" --include="*.rs" crates` returns 0 lines, as the record states, and the key map
head's "84 Funktionen mit 89 Kombinationen" counts out exactly against the file.

**The recording rule was kept in step 17.** No dated record was overwritten. The four places that
carry a recorded state — `_b_circle.md:63` (Stand 260802-1735), the spec's Festlegung at `:279`,
the chronicle at `:541-557`, and the Directive's own history — all keep their wording and receive
a dated Nachtrag beside them. The Directive sentence itself was replaced, which is what step 17
instructs and what round 1 did three times before; the removed sentence stands verbatim in the
Nachtrag. The three earlier corrections the Nachtrag names (260802-1127, 260802-1423, 260802-1445)
are the three the chronicle actually records for the Directive.

**The arithmetic of the spec rewrite is right.** The Kürzel table drops to five rows; "zwanzig
Belegungen" becomes "neunzehn" (5+1+3+3+5+1+1); "sechs Kürzel geprüft" becomes five with the sixth
named. `grep -n "sechs" `over the spec leaves only sentences about other counts, and every
remaining "endgültig" is either a dated record or an explicit "bis zum 260817 lautete es".

**The new comment in `tests/belegung.rs:294-299` is accurate.** It says the key-map head now lists
two ex-works-free combinations while the probe checks one; `resources/default-keymap.toml:62-67`
lists Umschalt+Entf and Opt+Cmd+Entf, and the probe body at `:335` checks `shift+delete` alone. The
gap itself is already filed as `issues/260817-2354_o_*`.

## Cross-cutting observation

The one Medium finding and the closed key-map finding from bundle D are the same shape: **a fact
stated in N places, corrected in fewer than N.** Bundle D left four counts of the Norton row; this
bundle leaves four statements of the backspace rule's consequence. Both times the sweep was scoped
by a word ("endgueltig", "sechs") and both times the missed places state the same fact in other
words. This project already carries three records of that pattern
(`shared/issues/260812-2253_*`, `shared/issues/260812-1438_*`, `shared/issues/260810-1851_*`, the
last one about search patterns with a blind spot). It is worth noting that the pattern survived a
bundle written specifically to eliminate it.

The cheap countermeasure the tree already uses elsewhere is the caller count as a probe
(`die_regel_hat_genau_einen_aufrufer`, `die_ausloesertafel_hat_genau_einen_aufrufer`). There is no
equivalent for prose, and there cannot be a general one — but a rule whose consequence is stated in
five places could name its four echoes in the module head of the rule, the way
`appkit/ereignisse.rs` names its two askers. That is a suggestion, not a finding.

## Recommended sequencing

- **Before the round closes:** F1. It is the direct answer to the bundle's own acceptance question,
  and leaving it means the round ships with the tree asserting both readings of its
  security-relevant rule.
- **With the session reconciliation:** F2 and F3, both corrections to records written this session,
  both one edit each.
- **Cleanup, any later round:** F4, F5, and the two shared CLAUDE.md records. None blocks anything.

The 22 open findings from the reviews of bundles A to D are deliberately not restated here.
