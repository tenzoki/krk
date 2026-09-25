# 260818-0410 — coderev: bundle F, the measurements and the guard

**Reviewed-range:** `f79f964..a4d8211`
**Not-opened:** none

**Sender:** coderev
**Tree state at review:** `a4d8211`
**Scope as dispatched:** eight commits, 18 files under `crates/`, `resources/` and `CLAUDE.md`;
the workbench records of the same range read for context, not reviewed as artifacts.

## Summary

The two commits that touch executable code hold. Six of the new probes were verified by
mutation — the fault built in, the probe seen red, the fault taken back — and every one of them
failed in the way its doc comment predicts, in one case naming the exact damage ("Überschreiben")
rather than a number. The three surveys the session ran were re-run independently and all three
reproduce; the 167-line reflow of the keymap moved no block and reattached no comment. Six
findings, none of them a release blocker and none touching behaviour: one latent duplication in
the sheet hull, one self-contradicting paragraph, one path the sweep's candidate selection could
not see, one unmeasured promise that lives outside the record stores, one production change under
a `test` commit type, one over-long line.

## Totals

| Severity | Count |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 4 |

## What was measured, and how

The dispatch asked for at least three probes verified by hand. Eight mutations were run across
six probe families. Each was applied to a working copy, the affected probe run, and the file
restored from a backup; `cargo test --workspace` is green at `a4d8211` before and after, and
`git status` shows no source file changed by this review.

| # | Mutation | Probe | Result |
|---|---|---|---|
| 1 | `bestaetigungsstelle` → always position 0 | `die_tafel_der_bestaetigenden_stelle` | red — "die Schaltflaeche mit der Eingabetaste steht in der Mitte und wird nicht gefunden", left 0 right 1 |
| 2 | same | `die_eingabetaste_im_feld_gehoert_ihrer_eigenen_schaltflaeche` | red — `die Eingabetaste faellt auf "Überschreiben", und die traegt sie nicht`; the doc comment's claim that the failure names the damage is exact |
| 3 | fallback `abbruchstelle(…)` → `0` | `die_tafel_der_bestaetigenden_stelle` | red — "ohne Schaltflaeche auf der Eingabetaste faellt die Antwort auf die liegenlassende"; the safety half of the table is held too |
| 4 | `Kommando::Notizzettel` added to `immer_erreichbar` | `waehrend_eines_blattes_kommen_genau_diese_vier_durch` | red, naming all five commands; two neighbouring probes red as well, and `waehrend_eines_blattes_kommt_allein_der_abbruch_und_die_ausnahmeliste_durch` stayed green exactly as its doc comment predicts |
| 5a | `SCHWELLE` 25 → 30 | compile-time assertion in `loeschwarnung.rs:513` | build aborts, `error[E0080]` |
| 5b | `MehrAlsDieSchwelle` wording no longer names 25 | same | build aborts, same error — both directions hold |
| 6 | `ist_warnwuerdig()` call inserted into `fuehrt_einen_papierkorb` and into `warngruende` | the two new counts | both red; `volumes::hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt` stayed green, as it should |
| 7 | `assert!` in `mit_schaltflaechen` deleted | `ein_blatt_ohne_ungefaehrlichen_ausgang_fliegt_auf` | aborts with `fatal runtime error: Rust cannot catch foreign exceptions`, SIGABRT — precisely the failure mode the probe's doc comment describes and warns about |
| 8 | keymap: `opt+cmd+delete` assigned to `in_papierkorb` | `die_ab_werk_freien_kombinationen_kommen_nicht_vor` | red — "opt+cmd+delete ist ab Werk belegt" |
| 9 | `MehrAls` guard weakened to `>= 1` | `der_umfang_loest_ab_der_schwelle_aus` | red — `MehrAls(10) left: [Umfang(MehrAlsDieSchwelle)] right: [Unentscheidbar]` |

Every report in this range that says "der Fehler eingebaut und die Probe rot gesehen" is
therefore confirmed at the probe level, not taken on trust.

Also checked, and holding: `cargo clippy --workspace --all-targets` clean, `cargo fmt --all
--check` clean, `cargo test --workspace` green (1 331 probes across the workspace).

## Findings by theme

### Duplicated answers to one question

**F-1 (Medium) — `Blatt::zeigen` answers "which button confirms" a second time.**
`crates/krk-ui/src/appkit/blaetter/mod.rs:762-766` hardcodes `fertig(stelle == 0)`, and its doc
names the precondition as "more than two buttons → use `zeigen_mit_wahl`". The load-bearing
precondition is a different one: the confirming button must sit at position 0.
`loeschbestaetigung::schaltflaechen` (`:109-114`) is a two-button sheet whose executing button
sits at position 1 — it satisfies the stated condition and would be answered wrongly. All five
callers today come from `Blatt::neu` and are correct; the assumption is the same one that cost
`260817-1242` in this Circle, on the neighbouring question. Filed as
`issues/260818-0410_o_blatt-zeigen-answers-which-button-confirms-…`.

**What holds around it, checked at every sheet in the tree.** `bestaetigungsstelle` and
`abbruchstelle` were evaluated by hand against all eleven sheets — `pfadeingabe`, `namenseingabe`,
`zeilennummer`, `suche`, `stapelumbenennen` (all from `Blatt::neu`), `konflikt`, `ungesichert`,
`uebersprungen`, `loeschbestaetigung`, `zettel`, `belegungsansicht`. Every pair is sensible; no
sheet carries two buttons on `Taste::Eingabe` or two on `Taste::Escape`; every sheet carries at
least one `Wirkung::Liegenlassen`, so the new `assert!` cannot fire at `a4d8211`.

**The half fix the dispatch asked about.** Verified against the code, not the report: with the
guard attached and `bestaetigungsstelle` still fixed at 0, Return in the conflict sheet's name
field would send `NSAlertFirstButtonReturn`, and `konflikt.rs:145-150` maps position 0 to
`Konfliktantwort::Ueberschreiben`. The claim that the half fix would have been a new defect on
the destructive exit is correct. In the shipped form the field's Return sends the code for
position 1, `Ueberspringen`, and Escape sends position 3, `Abbrechen` — both matching the sheet's
own explanatory text.

### Prose that no longer matches the code beside it

**F-2 (Low) — the `Art` paragraph counts two `Angaben` after the commit left one.**
`anwendung.rs:4665-4668` says "der Befehl bringt weiterhin ein Stueck mit" and then "die zwei
Angaben des Befehls" in the same paragraph. Filed as `issues/260818-0411_o_…`.

**F-6 (Low) — one 113-character prose line in `umfang.rs:146`**, the seam of the `926377f`
rewrite, in a block that wraps at 78. Every other line above 100 characters in the touched files
is a single unbreakable token. Filed as `issues/260818-0415_o_…`.

**The `Art` drop itself is complete.** `loeschen_nach_rueckfrage` has one caller
(`anwendung.rs:4477`), the value is written at the one construction site (`:4737`,
`Art::InDenPapierkorb`), and no other call site anywhere in `crates/` or `xtask/`. The removed
degree of freedom is used nowhere. The argument that killed it — one legal value, one caller —
applies verbatim to the surviving `schaltflaeche: &str`, which is a design question rather than a
defect and is noted inside F-2 rather than filed separately.

### Surveys re-run rather than taken over

All three counts the dispatch flagged as having grown were re-run independently.

**Six carriers of the shortened sheet lock.** Re-run with both needles over `crates/ xtask/
resources/ CLAUDE.md README.md Makefile idea.txt Cargo.toml`: thirteen hits, and none is a
carrier. `CLAUDE.md:124` now names all four commands and cites the probe; `default-keymap.toml`
does the same. Two hits state a single command's fate and derive it from `waehrend_blatt_erlaubt`
alone (`belegung.rs:638` for `Notizzettel`, `:952` for `TabSchliessen`); their premise names one
of the two gates rather than both, and their conclusion is nevertheless right, because neither
command sits on `immer_erreichbar` — held by `der_notizzettel_kommt_bei_stehendem_blatt_nicht_
durch`. Read and judged not a finding, as the `260817-1419` closure note already judged them.

**22 wrapped and ten unresolvable paths in the keymap.** Re-run: 35 backticked path-like tokens,
28 unique, 20 workbench references ending in `.md`, all 20 resolving against their marker glob.
One reference resolves against nothing and was not in the sweep's candidate set —
**F-3 (Low)**, `resources/default-keymap.toml:8`, the spec named as a bare filename with no
store. Filed as `issues/260818-0412_o_…`. The blind spot is the candidate selection ("token
containing a slash"), not the resolution; that is the third variant of a pattern this project
already records twice.

**The keymap reflow moved nothing.** Both revisions of the file were normalised to a stream of
"comment block" and "entry" tokens and compared: 402 blocks before, 402 after, identical
sequence, and the non-comment lines are byte-identical in content and order. Word-level diffing
of the 18 changed blocks shows only the intended edits — path unwrapping, store prefixes, and the
sheet-lock correction in the `mit_standardprogramm_oeffnen` block. No comment changed which entry
it precedes, so the block order that drives the menu order is untouched. Counted from the file:
84 functions, 89 combinations, matching the head; six Norton entries, five with a Cmd shortcut,
matching the header `48bb57f` rewrote.

### Records and traceability

**F-4 (Medium as a record gap) — the guard's one unmeasured promise lives only in prose.**
`konflikt.rs:49-53` and the closure note of the `_c_` record `260817-1241` both state that
whether the field editor passes `Cmd+Return` and `Opt+Return` is unmeasured. No `_o_` record in
this Circle or in `shared/` carries it, so it is on no acceptance list. The restraint in the
wording is right — the commit does not claim what it did not measure. What is missing is a place
for the measurement to be picked up. It matters because the one reason to enter that field is to
rename, and `Opt+Return` is "Umbenennen". Filed as `issues/260818-0413_o_…`.

**F-5 (Low) — `441da86` is typed `test(ui)` and splits a production branch without saying so.**
`loeschwarnung.rs:766-780`: `Umfang::MehrAls(_)` became a guarded pair whose second arm produces
`Warngrund::Unentscheidbar`. Reachable behaviour is unchanged (`zaehlen` caps at `SCHWELLE + 1`),
the branch is right and defensible because `Umfang` is publicly constructible, and it is measured
(mutation 9). The commit message mentions none of it. Filed as `issues/260818-0414_o_…`.

### Verified and clean

- **SDK line numbers.** All four claims of `926377f` read against the local SDK's `NSURL.h`:
  `:17` `NSURLResourceKey`, `:183` `resourceValuesForKeys:error:`, `:338`
  `NSURLVolumeIsLocalKey`, `:344` `NSURLVolumeLocalizedNameKey`. All four correct, including the
  `API_AVAILABLE(macos(10.7))` on both volume keys.
- **The hand-rolled `const fn nennt_die_zahl`.** Read line by line: buffer of ten bytes for a
  `u32`, digits written least-significant-first and compared reversed, `zahl == 0` produces one
  digit, the `heu.len() < laenge` early return is redundant with the loop bound but harmless.
  Correct, and its doc comment names its own limit (a substring match, not a word match)
  accurately.
- **`ist_warnwuerdig` still has no production caller** — nine mentions in `crates/`, all in probe
  code or in the type's own file. The three counting probes now stand in all three files the
  original record named.
- **`CLAUDE.md`.** The backspace paragraph, the sheet-lock paragraph and the `Kommando` counting
  instruction all read against the tree at `a4d8211` and all hold. The two `CLAUDE.md` statements
  the dispatch excluded were not re-examined.
- **The Circle's record state.** No `_o_` file in the Circle's `issues/` before this review; the
  three `_o_` decision records and the one in `shared/` are the four the dispatch names as
  intentional.

## Cross-cutting observations

**The session's own method is now stronger than its coverage.** Every survey in this range that
searched *for the statement* rather than *for the wording* found more than the one that searched
for wording, and both surveys that reported completeness had one residual each — the sheet-lock
sweep two carriers it first missed, the path sweep one reference outside its candidate set. The
lesson each of them wrote down is the right one; what neither did is state the *selection rule*
its check used, which is the step at which both failed. A survey report that names its candidate
rule is falsifiable by the next reader; one that names only its result is not.

**The `assert!` upgrade has a cost worth knowing.** Mutation 7 confirms that when the assertion
does fire in the probe build, it takes the whole `krk-ui` test binary down with SIGABRT rather
than reporting one red line — because the panic then escapes through `NSAlert::new`. The probe's
doc comment says so plainly, which is the right handling; it is recorded here so that a future
reader who meets that abort recognises it. Not a finding.

**Three questions, two answered once.** `abbruchstelle` and `bestaetigungsstelle` are now derived
rules with their own tables and their own probes. "Which button executes what the sheet asks
about" is the third, and it is still answered twice: as `stelle == 0` in `Blatt::zeigen` and as a
hand-written `AUSFUEHRENDE_STELLE = 1` in `loeschbestaetigung`. That is F-1, and it is the only
finding in this range that could ever produce a wrong answer at a sheet.

## Recommended sequencing

Nothing here blocks a release, and nothing found in this range changes behaviour.

1. **F-2, F-3, F-6** — three text edits, each in one place, each verifiable by reading the diff.
2. **F-4** — put the four key presses on this Circle's acceptance list. It is the only finding
   whose resolution needs the user at the running bundle.
3. **F-1** — a user choice between naming the precondition with a probe and deriving the third
   rule. Not urgent; no caller is wrong today.
4. **F-5** — nothing to change in the code; carry the branch split into the Circle's closure note
   so it is findable outside a `test`-typed diff.
