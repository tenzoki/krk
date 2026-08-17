# Review: bundle C, the loud confirmation and its seven reasons

**Sender:** coderev
**Datum:** 260817-1759
**Reviewed-range:** `ee85950..792995a`
**Not-opened:** none

**The range as an enumeration of its seven commits**, oldest first, because `A..B` in git's notation
excludes `A` and that confusion has already cost this project a record
(`shared/issues/260817-1122_o_der-durchsichtsbereich-schliesst-seinen-ersten-commit-aus.md`):

1. `1a57418` docs(workbench): die Durchsicht des Buendels B und ihre sieben Datensaetze — no code
2. `17d3550` refactor(core): der neue dreiwertige Typ heisst Loeschzielbefund
3. `c260e64` feat(core): der Umfang eines Loeschziels wird gedeckelt gezaehlt
4. `5a0f041` feat(core): a delete target is checked for a git worktree, upwards and in the selection
5. `749a4f3` feat(ui): a delete target is asked whether its volume is local
6. `c1b52db` feat(ui): the table of triggers, and the volume question is named after its trigger
7. `792995a` feat(ui): the confirmation goes loud when the target or the scope is unusual

`ee85950` stands as `<from>` and is itself not part of the review: it is the last commit of the
previous review's range, so `ee85950..792995a` names exactly these seven in git's notation and the two
code ranges tile without a gap. `bin/fusion-review-coverage` reported `carried=none` before this pass
and `uncovered=7`, matching the enumeration exactly.

**Language.** `CLAUDE.md` carries `**Artifact language:** en` since 260817-1600, so this review and
all nine records filed with it are written in English. Existing German records are not translated.

**Basis:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` (Directive, C3, C4,
C5), `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md` (`## Approach`, bundle C, steps 7
to 11), `reviews/260817-1419-coderev-buendel-b-papierkorb-und-stufenregel.md`, and the six coder
history files `history/260817-1504…` through `…-1806…`.

## Summary

**All seven commitments the orchestrator took hold at the tree, and the four acceptance commands run
green.** The rename moved only the name, the capped count holds one descriptor at any depth and its
child probes really measure that, the first-hit abort is measured through the visited-level list, the
polarity is right at every use in bundle C including the one that fills `Loeschziel`, the seven
reasons match spec C3 value for value and word for word, the stage order is unchanged with only the
moment of the expensive fact moved, and `Loeschtexte` is complete without a catch-all. Nine findings
stand beside that, none of them high. The pattern the previous review named recurs and has moved one
step closer to the thing itself: the mechanism is measured, and the prose that tells the next person
how to use it has fallen behind the mechanism in four places.

## Counts

| Severity | Number |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 7 |

## The five records closed in this turn, read back one by one

The dispatch named four; the range closed **five**. A closure is a claim until somebody reads it
back. All five are read back at the tree and all five hold.

| # | Record | Closed in | Read back at the tree |
|---|---|---|---|
| 1 | `260817-1419_c_zwei-verschiedene-dreiwertige-typen-unter-verzeichnis-heissen-beide-befund` | `17d3550` | **yes, holds.** I re-ran the closure's own dedup command over every type declaration under `krk-core/src/verzeichnis` and it returns no line. Listed all 25 public types myself: exactly one `Befund` (`modell.rs:191`) and one `Loeschzielbefund` (`loeschzielbefund.rs:146`). |
| 2 | `260817-1623_c_ist-lokal-returns-the-inverse-of-the-field-it-fills` | `c1b52db` | **yes, holds.** `liegt_auf_netzlaufwerk` (`volumes.rs:259-289`) returns `Nein` on `boolValue() == true` and `Ja` on `false`; the one inversion stands in the body with the explanation beside it. Its own note that the swap is "not uncompilable, it has lost its occasion" is still true — that is finding 1. |
| 3 | `260817-1108_c_die-loeschfrage-entsteht-vor-beiden-sperren-und-im-leeren-fall-mit-null-eintraegen` | `792995a` | **yes, holds.** Both texts are built in `Self::loeschtexte`, called at `anwendung.rs:4743`, inside the `Vorstufe::Rueckfrage` arm. The empty case cannot reach it, so "Diese 0 Einträge …" is unreachable rather than fixed, which is what the note says. The selection is read once (`:4696`), not twice. |
| 4 | `260817-1419_c_der-abschluss-von-260817-1107-begruendet-zwei-ungepruefte-eigenschaften-zu-weit` | `792995a` | **yes, holds.** `nach_der_rueckfrage` (`loeschwarnung.rs:849-858`) is a four-row table without catch-all, and the three named probes exist and are green (`:1690`, `:1715`, `:1732`). The closing block writes a complete case split over `(Nachstufe, Option<…>)` including the unreachable `(Auftrag, None)` (`anwendung.rs:4769-4781`). |
| 5 | `260817-1419_c_der-papierkorbtest-laeuft-vor-den-beiden-billigen-sperren-und-bringt-zwei-dateisystemzugriffe-mit` | `792995a` | **yes, holds, and its extra claim too.** See commitment 6 below. I also checked the claim "ein Löschbefehl, den ein laufender Vorgang oder eine leere Auswahl anhält, greift damit gar nicht mehr auf das Dateisystem": the four facts gathered before the rule are all in memory, and `betroffene_eintraege` (`tabelle.rs:1101-1105`) borrows the tab model and calls `operationen::betroffene` with no filesystem touch. |

`260817-1107_c_der-rumpf-der-schutzschwelle-traegt-keine-probe` also gained twelve lines in `792995a`
without moving its marker. That is the correct handling: the note says its sentence "am Code ist dafür
nichts mehr zu tun" is not to be corrected but is superseded, and the location rule for `issues/`
keeps a record of a past state at its past wording.

## The seven commitments, each checked

### 1 — The rename moved only the name, and no second type carries it

**No second type under `krk_core::verzeichnis` shares a name.** Checked myself over all 25 public
type declarations in the module tree, not taken from the record. `Befund` occurs once
(`modell.rs:191`, round 10's filter answer), `Loeschzielbefund` once (`loeschzielbefund.rs:146`).
`Befundmeldung` and `Inhaltsbefund` are the older type's family, and `verzeichnis/mod.rs:108-121` now
names that family explicitly and says why `Loeschzielbefund` is not in it — the silence that produced
the finding.

**The rename really moved only the name.** Measured rather than read: I substituted the name back in
the new file and diffed it against the old one.

```
git show 1a57418:…/befund.rs > alt.rs
git show 17d3550:…/loeschzielbefund.rs | sed 's/Loeschzielbefund/Befund/g; s/loeschzielbefund/befund/g' > neu.rs
diff alt.rs neu.rs
```

The whole difference is the new 51-line section `# Warum der Typ nicht Befund heisst` plus the
re-indented ASCII sketch. The same test over the three consuming files: `papierkorb.rs` and
`anwendung.rs` are byte-identical modulo the name, `loeschwarnung.rs` differs in exactly one place and
it is a rustfmt reflow — the longer name pushed one match arm over the line limit. No behaviour moved.
`grep` finds no reference to `verzeichnis::befund` or a bare `verzeichnis::Befund` anywhere in
`crates/`, `resources/` or `xtask/`.

### 2 — One descriptor at any depth, and the child probes really measure it

**The build form is the one from `durchlauf.rs`.** `zaehlen` (`umfang.rs:217-301`) holds `offen:
Vec<PathBuf>`, opens one `Schwungleser` per iteration of `while let Some(pfad) = offen.pop()`, and the
reader falls at the end of the body before the next `oeffnen`. There is no recursion at all, not even
the depth-limited kind the plan foresaw, and the module header says so and says why. Every subdirectory
enters the stack as a **path** (`:272`), never as an open reader.

**The three bounds derive from the one sentence the module header names**, and I checked the
derivation: every descent costs at least one counter, because a subdirectory is counted before it is
recorded (`:268` before `:272`). Hence at most `SCHWELLE + 1` opens, at most `SCHWELLE + 1` recorded
paths, and exactly one open descriptor. `Genau` can never exceed `SCHWELLE`: the cap is checked after
increment at the top level (`:229`) and after every batch (`:289`).

**The child probes measure what they claim, and this is the part worth the most attention.** The
promise is honest for a reason that is built in rather than asserted: the child measures its own
descriptor supply and refuses to conclude anything if the supply is too large.

```rust
assert!((vorrat as u32) < DECKEL, "der Vorrat von {vorrat} Deskriptoren reicht für {DECKEL} …");
assert!(vorrat > 0, "das Kind bekommt gar keinen Deskriptor mehr; die Grenze {GRENZE} ist zu tief");
```

I ran the child directly, without the lowered limit, and it refused exactly as designed:

```
$ KRK_PROBE_UMFANG_KETTE=<dir> ./target/debug/deps/umfang-… --exact --ignored kind_zaehlt_die_tiefe_kette_mit_einem_deskriptor
panicked at crates/krk-core/tests/umfang.rs:391:
der Vorrat von 96 Deskriptoren reicht fuer 26 gleichzeitig offene Verzeichnisse;
ein Abstieg mit einem Deskriptor je Ebene liefe hier durch und die Probe messte nichts
```

Under `ulimit -n 24` it passes. That is the difference between a probe that measures the build form
and one that only looks as if it does, and it is exactly why `GRENZE` is 24 and not the 64 the
`durchlauf` probes use: the cap already limits opens to 26, so under 64 a per-level descent would run
through unnoticed. The module header of `tests/umfang.rs` derives that in full.

**One starter, not two.** `kind_mit_deskriptorgrenze` moved from `tests/verzeichnis.rs` into
`tests/gemeinsam/mod.rs` and takes the limit as an argument; `verzeichnis.rs` keeps its 64 in a named
constant `DESKRIPTORGRENZE`. A second copy beside it would have been the mistake that directory
exists to avoid, and it was not made.

**What I did not do:** I did not run the two mutations the commit message cites. The second one — the
loop holding every reader — is sound analytically (a 30-level chain needs up to 26 simultaneous
descriptors and the child has fewer than 26 by its own assertion), and the assertion that makes it
detectable is measured above. The first — dropping the `Unentschieden` arms — would make `zaehlen`
answer `Genau(1)` under `EMFILE` where the child asserts `Unentschieden`. Both arguments are
inference from the code, not a run.

### 3 — First-hit abort: the seam carries the promise, and its residual is named

**The seam answers the question the user has.** "Abort on the first hit" is a claim about the *access
pattern*, and the return value cannot express it, because `Loeschzielbefund::oder` makes `Ja`
absorbing — a walk that carries on past the first hit returns the same `Ja`. The two private loops
therefore take the check as `impl FnMut(&Path) -> Loeschzielbefund`, and the probes substitute a
recorder that collects the paths it was asked about and compare the **list**, not the answer:

```rust
assert_eq!(mitschrift.besucht(), vec!["/a/b/c/d", "/a/b/c", "/a/b"],
           "der Gang haelt beim ersten Treffer nicht an");
```

Six of the thirteen in-file probes assert an exact visit list, including the two cost claims that are
invisible in the result: the selection is not asked at all when the upward walk says `Ja`
(`arbeitsbaum.rs:581`), and a doubt in the upward walk cuts the selection off as well
(`:604`). `beruehrt_mit` passes `&mut pruefer` into `aufwaerts_mit`, so one recorder sees both
phases. This is a measurement of the access pattern, not an approximation of one.

**The residual is one line each and the module comment states it**: that the public wrappers really
substitute `traegt_arbeitsbaum`. `liegt_in_arbeitsbaum` and `beruehrt_einen_arbeitsbaum` are
one-expression functions, and the eleven probes in `tests/arbeitsbaum.rs` drive the same cases over
real directories for the *outcome*. The comment says what stays unmeasured and why, in its own words:
"Was dabei ungemessen bleibt und bleiben muss". No finding — the honesty is the right response to a
gap of this size.

**One detail deserves naming because it is not cosmetic.** `traegt_arbeitsbaum` answers `Nein` on
`io::ErrorKind::NotADirectory`, and the doc comment says why: the selection of a file pane carries
ordinary files, `lstat("datei/.git")` fails with `ENOTDIR` and not `ENOENT`, and without that arm
every selected file would make the confirmation undecidable and therefore loud. The catch-all beside
it is required and is the only one in this bundle: `io::ErrorKind` is `non_exhaustive`, so a complete
split is not available, and the catch-all goes in the cautious direction.

### 4 — The polarity, at every use in bundle C

**Every use is read and every one is right.** All of them:

```
volumes::liegt_auf_netzlaufwerk   (volumes.rs:283-288)   boolValue true → Nein, false → Ja      trigger polarity
arbeitsbaum::traegt_arbeitsbaum   (arbeitsbaum.rs:231-241) .git present → Ja, absent/ENOTDIR → Nein, else Unentschieden
papierkorb::fuehrt_einen_papierkorb (papierkorb.rs:185)  Ok → Ja (permission), Err → Nein
loeschwarnung::vor_der_rueckfrage  (:372-377)            Ja → Rueckfrage; Nein | Unentschieden → OhnePapierkorb
loeschwarnung::warngruende         (:669-681)            all three answers spelled out per field
anwendung.rs:4852, :4857, :4867-4868                     no inversion; None → Unentschieden
```

**The filling of `Loeschziel` is right, and the reasoning is written where the fill happens.**
`loeschtexte` (`anwendung.rs:4840-4874`) assigns `netzlaufwerk` from `liegt_auf_netzlaufwerk` and
`arbeitsbaum` from `beruehrt_einen_arbeitsbaum` with no inversion, which is correct now that both
functions answer their trigger. An unresolvable folder yields `Unentschieden` at both, never a silent
`Nein`. And it takes `pfade::benutzerverzeichnis()` — the `Option` form — rather than the module's own
`benutzerverzeichnis()` at `:7003`, which substitutes `/`; I checked both functions, and the doc's
reason for the choice is exact: a `/` there would turn "KRK does not know the home directory" into
"the folder lies inside it".

**What is not measured is the wiring itself, and that is finding 1.** Swapping the two field names at
`:4867-4868` compiles and leaves all 24 probes over the four checks and the table green, because none
of them sees this function — and `loeschtexte` is testable, being an associated function without
`&self` in a file that already carries two `#[cfg(test)]` modules.

**Two module headers still point the reader at the wrong question, and that is finding 2.**
`arbeitsbaum.rs:95-96` says flatly that the caller asks `ist_warnwuerdig`; the one caller must not,
and does not.

### 5 — The ranking of the seven reasons, against C3 value for value

**Identical, in order and in wording.** Compared field by field against the spec's trigger table and
its ranking sentence under `**Getroffene Festlegungen:**`:

| Rank | `Warngrund` | Spec's wording | `wortlaut()` at `:525-535` |
|---|---|---|---|
| 1 | `Unentscheidbar` | "nennt als Grund, dass das Ziel sich nicht einordnen ließ" (substance only) | "von einem Ziel unbekannter Einordnung" |
| 2 | `Netzlaufwerk` | „von einem Netzlaufwerk" | identical |
| 3 | `Cloudort` | „aus einem Cloud-Ordner" | identical |
| 4 | `AusserhalbBenutzerordner` | „außerhalb des Benutzerordners" | identical |
| 5 | `ImBenutzerordner` | „unmittelbar im Benutzerordner" | identical |
| 6 | `Arbeitsbaum` | „aus einem Git-Arbeitsbaum" | identical |
| 7 | `Umfang` | „mit 25 Einträgen" / „mit mehr als 25 Einträgen" | identical, both |

Six of seven are verbatim; the seventh is substance-only in the spec and the chosen form is justified
at `:508-519` — a phrase without a relative clause, because every wording sits at the same slot in the
question and a relative clause would need a second comma the other six do not have. The ranking stands
at exactly one place, the declaration order of the enum, and `Ord` is derived. The probe
`die_rangfolge_der_aufzaehlung_ist_die_des_specs` (`:1303`) reads it off as **strictly** ascending, and
the doc comment says why strict and not merely ascending: two reasons at one rank would make the named
reason depend on `sort_unstable`, which promises nothing about equal values.

**`Warngrund::Umfang` carrying a payload is the right call and is recorded.** The plan's execution note
for step 10 gives the reasoning: the sixth trigger has two wordings, so a value-free variant cannot
produce one; `Umfang` itself as payload would require `Ord` on `krk_core::verzeichnis::Umfang`, which is
an ordering over `Genau`, `MehrAls` and `Unentschieden` and would be a claim with no subject. The
private `Umfangsgrund` with two values is the smaller construct and keeps `Warngrund` at seven values
with derived `Ord`, as the plan wrote it.

**The `const _: () = assert!(SCHWELLE == 25, …)` holds what its own doc comment claims and less than
the plan's phrasing promises.** It halts the build when `SCHWELLE` moves, which is what
`loeschwarnung.rs:493-498` says. It does not hold the two wordings **to** `SCHWELLE`: it binds
`SCHWELLE` to a second literal `25`, so an edit that moves the constant and the assertion together
leaves the wordings and the probe that pins them at 25. The precedent its own doc cites is stronger in
exactly that respect — `editor.rs` asserts `STAPELBUDGET == EDITORGRENZE`, two symbols. That is finding
6, with the cheap two-sided form named.

**The dedup is right and measured.** Three undecided inputs produce one `Unentscheidbar`, not three
(`:1503`), and `dedup` suffices because the list is sorted first. Eleven probes cover C3's acceptance
criteria at the rule level, including `CloudStorageAlt` as a non-hit (component-wise `starts_with`,
not character-wise), 24 / 25 / 26 at the threshold, and the four-reason case where the collection order
differs from the ranking so the sort is really measured.

### 6 — The cost moved, the order did not

**The table and the body say the same thing, line for line.** `vor_der_rueckfrage` (`:359-378`) matches
its five-row table; the first two rows now read "ungefragt" instead of "gleichgültig" in the trash
column, and the body calls `papierkorb()` only in the `(false, false)` arm. The twelve-case probe
(`:924`) writes out all two × two × three combinations and is unchanged in its expectations, which is
the direct evidence that the order did not move.

**Against the plan's first flow diagram, node for node:**

```
Plan            Rule                       Place
R Filtertext    outside, rueckschritt.rs   anwendung.rs (unchanged)
V Vorgang       row 1                      Vorstufe::VorgangLaeuft
A Auswahl leer  row 2                      Vorstufe::NichtsAusgewaehlt
P Papierkorb    rows 3 to 5                the FnOnce, called in (false, false)
Z Umfang + 5    fourth branch              loeschtexte, anwendung.rs:4840-4874
W Ausloeser     fourth branch              warngruende
B1 / B2         laut = !gruende.is_empty() loeschbestaetigung::zeigen
F Cmd+Return    at the sheet               nach_der_rueckfrage, anwendung.rs:4770
O Auftrag       fifth stage                loeschauftrag_stellen
```

Every node is in the diagram's order. The resolved folder is computed inside the `P` closure and reused
by `Z`, so `Z` cannot precede `P`.

**The laziness is measured, with a counter-probe.**
`die_teure_tatsache_bleibt_in_den_ersten_zwei_stufen_ungefragt` (`:1033`) counts calls through a `Cell`
— necessary, because the input is a `FnOnce` and may consume itself — expects zero in the three
combinations where a cheap stage already stops, and **exactly one** in the fourth. Without that last
line a rule that never calls the trash test would be green and the third stage silently dead. The
counter-probe is there.

**`f8` lets none of the five facts arise.** `endgueltig_loeschen` (`:4586-4592`) passes
`Loeschtexte::EndgueltigBisBuendelD`, and that arm of `loeschtexte` (`:4870-4873`) calls
`operationen::loeschfrage` and returns `laut = true` — no home directory, no volume question, no
worktree walk, no count. One precision worth stating: the *resolved folder* is the first of the five
fields and it **is** computed for `f8`, inside the trash-test closure. That costs `f8` nothing extra,
because the trash test applies to both commands by design and was already verified in the previous
review; four of the five facts do not arise, and the fifth is paid for anyway.

### 7 — `Loeschtexte` is complete without a catch-all, and `f8` behaves unchanged

**The case split is complete and has no catch-all** (`:4851-4873`): two arms for two values, both
written out. Removing `EndgueltigBisBuendelD` in bundle D makes the arm reference an unknown variant
and the build stops at the branch that has to go. `endgueltig_loeschen` is the only producer of that
value, so it goes in the same step.

**`endgueltig_loeschen` behaves unchanged until bundle D.** Compared the old and the new path:

| what | before `792995a` | after |
|---|---|---|
| order type | `Art::EndgueltigLoeschen` | same |
| second button | "Endgültig löschen" | same |
| texts | `operationen::loeschfrage(&auswahl)` in `endgueltig_loeschen` | same call, in `loeschtexte` |
| `laut` | `true` | `true` |
| selection read | twice (once in `endgueltig_loeschen`, once in the body) | once |
| when the text is built | before the stage rule | in the fourth branch |

The two differences are invisible and both are improvements: one fewer read of the same value in the
same turn of the event loop, and no text built for an outcome that discards it. Letting the shared body
produce the trash wording for both commands would have shown `f8` a sheet saying "in den Papierkorb
räumen" over a permanent delete, which the commit message names as the reason the enumeration exists.

## What is checked and holds

**The four acceptance commands run green, run myself at 260817-1755.** `cargo build --workspace`,
`cargo test --workspace` (exit 0), `cargo fmt --all --check`, and
`cargo clippy --workspace --all-targets -- -D warnings` with exit 0. `krk-ui` 679 probes, `krk-core`
176 plus the integration targets, `tests/umfang.rs` 8 passed and 2 ignored (the two children),
`tests/arbeitsbaum.rs` 11 passed. **The flaky probe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` passed** — `tests/text.rs` reported 30 passed,
0 failed — so there was nothing to re-run.

**The macOS floors are read at the SDK, not inferred.** I re-read five citations from `volumes.rs`
line by line in
`$(xcrun --show-sdk-path)/System/Library/Frameworks/Foundation.framework/Headers/`: `NSURLVolumeIsLocalKey`
at `NSURL.h:338` with `API_AVAILABLE(macos(10.7), …)`, `resourceValuesForKeys:error:` at `NSURL.h:183`
with `macos(10.6)`, `fileURLWithPath:` (the form without further arguments) at `NSURL.h:52` with no
annotation, `NSURLResourceKey` at `NSURL.h:17` with no annotation, `boolValue` at `NSValue.h:73` with
no annotation. All five are right and none is above 15.0. The count "Sieben Berührungen sind jünger" is
right as well: five at 10.6 and two at 10.7. One line number is attributed to a pair and fits only one
of the two names — finding 5.

**The floor section stands in every `appkit/` file but the two justified exceptions.** Counted myself
over all 40 files under `appkit/` and `appkit/blaetter/`: 38 carry
`# Ab welchem macOS die angesprochenen Klassen stehen`, and the two without are `koordinaten.rs` and
`mod.rs`, both justified. `krk-core` needs none.

**`#[must_use]` stands at every new return value whose silent drop would go unnoticed**, each with a
written reason: `umfang::zaehlen`, `arbeitsbaum::{traegt_arbeitsbaum, liegt_in_arbeitsbaum,
beruehrt_einen_arbeitsbaum}`, `volumes::liegt_auf_netzlaufwerk`, `Warngrund::wortlaut`,
`loeschwarnung::{warngruende, nach_der_rueckfrage}`, and `Anwendungsdelegierter::loeschtexte`. Nine
places, none without the attribute. The two private loop helpers `aufwaerts_mit` and `beruehrt_mit`
carry none and need none: each has exactly one non-probe caller, and that caller is a one-expression
function that returns the value, so a silent drop is not constructible.

**The case splits are complete without catch-alls where a complete split exists.** `Vorstufe` (four
values), `Nachstufe` (two), `Loeschzielbefund` at every consumer, `Umfang` at `warngruende`, `Typ` in
the counting closure, `Warngrund` in `wortlaut`, and `Loeschtexte`. The one catch-all in the bundle is
at `io::ErrorKind`, which is `non_exhaustive`, and it is justified in place and goes in the cautious
direction.

**Each rule stands once and has a caller count.** Verified the two counts myself rather than trusting
them: `grep -rn "warngruende" crates/` finds one call (`anwendung.rs:4871`) beside doc comments, and
`vor_der_rueckfrage` one (`:4703`). The two counting probes `die_stufenregel_hat_genau_einen_aufrufer`
(`:901`) and `die_ausloesertafel_hat_genau_einen_aufrufer` (`:1276`) both expect 1 and are green. The
second's expectation moved from 0 to 1 in this bundle, which is the build form the module header
describes: a probe that expected "at most one" would have been green forever and measured nothing.

**Prose counts checked, not believed.** Thirteen modules under `verzeichnis/` (`mod.rs:3`, counted:
13). Five inputs, six triggers, seven reasons (`loeschwarnung.rs:136`; `Loeschziel` has five fields,
`Warngrund` seven values, and the arithmetic is explained rather than asserted). Eight rows in the
wording table because `Umfang` has two wordings. Five pieces and two counts in the module header,
counted: five public functions and two counting probes. Three functions in `arbeitsbaum`, four outcomes
in `traegt_arbeitsbaum`. Twelve combinations, four cases, nine fields of the `oder` table. All correct.
The counts that are **not** correct are all caller sentences, and they are findings 3, 4 and 7.

**`ist_warnwuerdig` still has no production caller, and that is now a design fact.** Counted with
`grep -rn "ist_warnwuerdig" crates/`: 27 hits, six of them calls, all six inside probes. The coder's
account is right about the reason — `warngruende` has to keep `Ja` and `Unentschieden` apart because
they produce different entries, which is C3's own criterion, and the probe
`ein_unentschiedener_eingang_nennt_seinen_ausloeser_nicht_mit` pins it. What the account does not cover
is that two module headers still instruct the reader to use it: finding 2.

**The loud form differs from the quiet one in exactly the three things C3 names, and no more.**
`loeschbestaetigung::zeigen` (`:120-140`) calls `als_warnung()` under `if laut` and nothing else;
buttons, their order and their keys come from `schaltflaechen(schaltflaeche)` with no reference to
`laut`, and the hint line is appended unconditionally. The probe
`ohne_grund_bleibt_die_ruhige_form_unveraendert` (`:1670`) pins the quiet texts word for word. The
*name* of the third difference is wrong in two places — finding 8.

**A new shared record was filed inside the range** and is correctly placed:
`shared/issues/260817-1610_o_the-language-paragraph-in-claude-md-predates-the-artifact-language-declaration.md`,
in `5a0f041`.

## Findings

### 1 — Medium: the one place a polarity swap still compiles carries no probe of its own

`Anwendungsdelegierter::loeschtexte` (`anwendung.rs:4840-4874`) is where the five facts reach the five
fields of `Loeschziel`. The wiring is correct. Nothing measures it, and the record closed in this very
turn says why that matters: `260817-1623` describes the swap as one that "would have compiled, passed
every probe and exchanged local for remote", and its own closure states plainly that the swap "is not
uncompilable; it has lost its occasion". Swapping the two field names at `:4867-4868` today leaves all
24 probes over the four checks and the trigger table green.

The usual objection does not apply here: `loeschtexte` is an associated function with no `&self` and no
`MainThreadMarker`, takes all four inputs as arguments, and `anwendung.rs` already carries two
`#[cfg(test)]` modules with 13 probes between them.

**Direction:** two probes in that module. One asserts that a local target under the user directory
yields a question without "von einem Netzlaufwerk" and `laut == false`; one counter-probe asserts a
target that must be loud, cheapest being the user directory itself, which triggers from paths alone. A
third for `Loeschtexte::EndgueltigBisBuendelD` goes red the day someone folds the two branches
together.

Record: `issues/260817-1759_o_the-one-place-a-polarity-swap-still-compiles-carries-no-probe.md`

### 2 — Medium: two module headers tell the caller to ask `ist_warnwuerdig`, and the one caller must not

`arbeitsbaum.rs:95-96` states as a fact about the tree: "Der Aufrufer fragt
`Loeschzielbefund::ist_warnwuerdig`, nicht auf `Ja` selbst." `volumes.rs:248-251` calls it the right
question and says it is asked "dort, wo die Rangfolge aus C3 steht". The place where the ranking stands
is `warngruende`, and it deliberately does not ask it: `ist_warnwuerdig` merges `Ja` and
`Unentschieden`, and those two produce different entries in its list. Following `arbeitsbaum.rs`
literally would make KRK name "aus einem Git-Arbeitsbaum" for a target it could not classify — a claim
with no measurement, and the thing C3's acceptance criterion forbids.

The design is right and measured; two files describe the design it replaced. `arbeitsbaum.rs` is the
worse of the two because it states it flatly and three times over in shorter form is *not* the problem
— the three shorter notes at `:220`, `:283` and `:333` state the polarity only and name no asker.

**Direction:** say what holds and why — for a first-polarity value `ist_warnwuerdig` would be sound,
and the one consumer still writes all three answers out because it has to name *which* reason. No
behaviour changes. Whether the polarity should sit on the type remains the second way of
`260817-1419`, untouched.

Record: `issues/260817-1800_o_two-module-headers-tell-the-caller-to-ask-ist-warnwuerdig-and-the-one-caller-must-not.md`

### 3 — Low: two module headers in `krk-core` name a caller that does not contain the call

`umfang.rs:138-141` and `arbeitsbaum.rs:151-154` both name
`Anwendungsdelegierter::loeschen_nach_rueckfrage` as the one caller. The calls stand in
`Anwendungsdelegierter::loeschtexte` (`:4869` and `:4857`), which that function reaches at `:4743`.
The rest of both sentences is right — one caller each, once per delete command, only after the two
cheap stages — and the last of those three is the half that makes the correction worth making, because
it is a claim about where in a body the call sits.

Record: `issues/260817-1801_o_two-module-headers-in-krk-core-name-a-caller-that-does-not-contain-the-call.md`

### 4 — Low: two more "no caller yet" statements remain, so the count of two undercounts

`792995a` states it corrected the two module headers that said "who calls it: nobody". Two more of the
same kind stand. `loeschzielbefund.rs:121-131` still says the checks "stehen zu diesem Zeitpunkt noch
nicht alle da" and that the type has no caller in this crate; all four checks exist and `arbeitsbaum.rs`
has used the type as its return type since `5a0f041`. And `loeschwarnung.rs:1252` still summarises its
probe as "Genau eine Stelle im Baum fragt die Ausloesertafel — heute keine", where the clause after the
dash contradicts the clause before it, the body five lines below and the assertion at `:1285`.

The needle finds them in one line over the thirteen files the bundle touched: three hits, two of them
the defect and one a false positive. Same lesson as `shared/issues/260815-1448_o_…`: the reach of the
search and the reach of the claim have to stand next to each other.

Record: `issues/260817-1802_o_two-more-no-caller-yet-statements-remain-so-the-count-of-two-undercounts.md`

### 5 — Low: one SDK line number is attributed to two symbols and fits only one

`volumes.rs:130-132` writes "`NSURLVolumeLocalizedNameKey` und `NSURLVolumeIsLocalKey` (`NSURL.h:338`,
…)". `NSURLVolumeIsLocalKey` is at 338, `NSURLVolumeLocalizedNameKey` at 344. The availability is right
for both. Introduced in `749a4f3`, which merged the two names into one parenthesis and gave the pair the
new key's line number; before that the older key carried none. The section exists so a number can be
re-read at the SDK, and a merged citation is the one form that survives a re-read while being wrong.

Record: `issues/260817-1803_o_one-sdk-line-number-is-attributed-to-two-symbols-and-fits-only-one.md`

### 6 — Low: the 25 lives in four places and the compile-time assertion binds only one pair

`const _: () = assert!(SCHWELLE == 25, …)` (`:499-502`) halts the build when `SCHWELLE` moves, which is
what its doc comment claims. It binds `SCHWELLE` to a second literal, not the two wordings to
`SCHWELLE`, so an edit that moves the constant and the assertion together — which is the natural
response to the compiler pointing at that line — leaves the wordings at `:533-534` and the probe's
literals at `:1338` and `:1342` saying 25. Beside it, `Umfang::MehrAls(_)` (`:695-697`) discards a
payload the wording depends on; the number is right only because `zaehlen` caps.

**Direction:** one probe binds the strings to the constant instead of to a second literal —
`wortlaut().contains(&SCHWELLE.to_string())` for both values — and it goes red on exactly that edit.
Keep the compile-time assertion as well; it points a person at the right file, which a probe cannot.

Record: `issues/260817-1804_o_the-25-lives-in-four-places-and-the-compile-time-assertion-binds-only-one-pair.md`

### 7 — Low: a third way of handling a missing home directory arrived, and the doc comment names two

`ablage/pfade.rs:189-193` enumerates the divergent handlings of a missing home directory and names two.
`792995a` added a third and the safety-relevant one: `loeschtexte` treats `None` as an open question and
lets it become `Warngrund::Unentscheidbar`. The sentence settles neither of its two readings — scoped to
the core it is accurate, read over the tree it has been false since round 3 brought
`belegungsausgabe.rs`. It is worth the edit rather than a bare count correction because the third
handling carries a reasoning the next caller needs before it picks a fallback.

Record: `issues/260817-1805_o_a-third-way-of-handling-a-missing-home-directory-arrived-and-the-doc-comment-names-two.md`

### 8 — Low: the third difference between loud and quiet is called "die Folgen" and none is added

Spec C3's summarising criterion names the third difference "den Folgen in der Erläuterung", and
`loeschbestaetigung.rs:12-17` repeats the word. What the loud explanation gains is the **remaining
reasons** as an "Außerdem: …" paragraph (`loeschwarnung.rs:776-780`), not a consequence.
`kommandos/loeschwarnung.rs:734-739` describes the same mechanism correctly and never says "Folgen", so
two files describing one mechanism use two words for it and only one matches the code. The operative
criterion two bullets above is satisfied.

Record: `issues/260817-1806_o_the-third-difference-between-loud-and-quiet-is-called-consequences-and-none-is-added.md`

### 9 — Low: two history filenames and four closure notes carry timestamps that no clock produced

`history/260817-1806-…` sits in `792995a`, author date 17:39, and was still in the future at 17:59 when
this review read the clock; four closure notes cite `260817-1806` as their resolution time.
`history/260817-1722-…` sits in `c1b52db`, author date 17:13. The first four commits of the bundle have
the expected sign, four to seven minutes before their commits; the last two are ahead of their own. The
convention is explicit and gives the reason ("LLMs have no clock — never guess"), and the concrete cost
is that a reconciliation pass ordering closures against commits gets the wrong order for three
findings.

This sits at the edge of a code reviewer's scope — the workbench is another agent's ground — and it is
filed because it is concrete, measured, and about the reliability of this session's own record.

Record: `issues/260817-1807_o_two-history-filenames-and-four-closure-notes-carry-timestamps-that-no-clock-produced.md`

## Cross-cutting

**The mechanism is measured; the prose that tells the next person how to use it is one step behind.**
This is the previous review's pattern, moved. There the finding was that a safeguard was prose where a
probe was available. Here the mechanisms have their probes — twelve cases for the stage rule, eleven for
the trigger table, six visit lists for the abort, two child processes for the descriptor promise — and
the drift has moved into the sentences that direct a reader to them. Four of the nine findings are
exactly that (2, 3, 4, 8), and each names a function or a thing that does not do what the sentence says.
The counts in this bundle are right; it is the **caller sentences** that are wrong, all four of them, and
that is a sharper diagnosis than "the prose drifts". A caller sentence is the one kind of prose that goes
stale the moment a caller is added, and the bundle added five.

**Two findings sit on the same missing measurement from two sides.** Finding 1 says the wiring in
`loeschtexte` is unmeasured; finding 2 says two headers point at the wrong question for the values that
wiring carries. Both are the polarity, and both would be closed by the same pass over that one function
and the two headers around it. That pass is cheaper now than after bundle D, which touches the same body
when `Loeschtexte` loses its second value.

**The child-process build form is the strongest thing in this bundle and deserves naming as a pattern.**
`tests/umfang.rs` does not merely lower the descriptor limit; the child *measures its own supply* and
refuses to conclude anything if the supply exceeds what the promise is about. I ran it without the
lowered limit and it refused, naming the number it found. That is the difference between a probe that
measures a build form and one that only looks as if it does, and it is a direct answer to the failure
`CLAUDE.md` records for round 10 — that `cargo test` inherits the session's raised limit and the promise
is then merely asserted. Any later promise about a bounded resource should be built this way.

**The seam-with-substituted-check is the second reusable pattern here.** `arbeitsbaum.rs` states in its
own comment why a general parameter is not gratuitous: the promise is about an access pattern, the return
value is absorbing, and no probe over real directories can see the difference. Taking the check as a
parameter turns an unmeasurable claim into a list comparison. The same shape would serve any later cost
promise that a result cannot express, and the residual it leaves — one line per wrapper — is stated
rather than hidden.

**One number in the dispatch was low and is worth correcting for the record.** Five records were closed
in this turn, not four. All five are read back above and all five hold.

## Recommended sequencing

**With bundle D, and in the same pass:** findings 1 and 2. Bundle D reworks `loeschen_nach_rueckfrage`
and `loeschtexte` when `Loeschtexte` loses its second value, so the probes of finding 1 and the two
headers of finding 2 fall in a body that is being opened anyway. Doing them later costs a second visit to
the same three files.

**With bundle D or bundle E, cheaply:** findings 3 and 4, which are four sentences in three files, and
finding 6, which is one probe beside an existing one. Finding 4 carries the needle that finds its own
class; run it before closing the bundle rather than after.

**With bundle E's pass over superseded wording (C6):** findings 5, 7 and 8. All three are prose, all
three are in files C6 touches or should touch, and finding 8 names the one line the spec would gain if
C6's pass reaches it.

**No release blocker.** None of the nine changes behaviour, and the four acceptance commands are green.
What is *not* covered by any of them is the acceptance run of C3 and C4 in the foreground, which remains
user work: whether the sheet really goes loud, whether the reason really reads as intended in the
question, and whether a Finder-mounted network volume answers `Ja`. The coder named all three as
unchecked and that account is accurate.
