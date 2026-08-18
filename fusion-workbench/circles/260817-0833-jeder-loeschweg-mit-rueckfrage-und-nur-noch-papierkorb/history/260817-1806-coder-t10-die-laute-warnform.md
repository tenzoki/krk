# T10 — The loud form, and the three records that closed with it

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 11 (last step of bundle C)
**Binding:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C3 and C4
**Tree state before the task:** `3fcd375` plus the uncommitted steps 4 to 10 of earlier tasks
**Verification:** `make check` — exit 0

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`.
> The code stays German, identifiers and prose alike. The `Resolved:` notes appended to the
> three German defect records are German, per the language rule of the dispatch.

## Three operations, one chain

The task closed the chain the five bundles build: the body of the one delete path now gathers the
five facts about the target, asks the trigger table, and hands the sheet a loudness flag and two
texts. Two open defect records sat on the same body and were carried by the same change.

## 1 — Step 11: the facts, and the loud sheet

`Anwendungsdelegierter::loeschtexte` (new, `appkit/anwendung.rs`) is the one place that gathers
the five facts. Each comes from exactly one source, and the crate boundary decides where the
source lives:

```
ordner              the displayed folder, already resolved by the body
benutzerverzeichnis krk_core::ablage::pfade::benutzerverzeichnis
netzlaufwerk        appkit::volumes::liegt_auf_netzlaufwerk
arbeitsbaum         krk_core::verzeichnis::arbeitsbaum::beruehrt_einen_arbeitsbaum
umfang              krk_core::verzeichnis::umfang::zaehlen
```

`warngruende` turns them into the ranked list, its first entry goes into the question and the
rest into the explanation, and `laut` is "the list is not empty". All three go unchanged to
`loeschbestaetigung::zeigen`.

**The home directory still has exactly one asker.** `pfade::benutzerverzeichnis` is called once
per delete command, resolved once, and the same value reaches both places that need it: the field
from which `warngruende` computes triggers 1, 2 and 4, and the boundary of the git walk upwards.
The free `benutzerverzeichnis()` of this module was deliberately **not** used: it falls back to
`/` when the system names none, and a `/` here would turn "KRK does not know the home directory"
into "the folder lies inside it". `None` means the question is open, and the table makes
`Warngrund::Unentscheidbar` of it.

**The polarity was read, not inferred.** `liegt_auf_netzlaufwerk` and
`beruehrt_einen_arbeitsbaum` sit on the polarity where `Ja` warns and `Unentschieden` belongs to
it, which is exactly how the two fields of `Loeschziel` take them; the trash question in the body
above sits on the other one, where `Ja` is the permission. The module header of
`krk-core/src/verzeichnis/loeschzielbefund.rs` holds the two apart, and the doc comment of
`loeschtexte` now writes the distinction out at the point where the struct is filled.

### The deviation: one enum instead of three parameters

The plan says the body builds the texts. It does not say what happens to `endgueltig_loeschen`,
which until bundle D shares this body and carries *different* texts and an always-loud sheet.
Three routes were open, and two are wrong:

- Let the body build the trash wording for both. Rejected outright: `f8` would show a sheet
  saying "in den Papierkorb räumen" while deleting permanently. That is the class of error this
  round exists to remove.
- Keep `frage`, `erlaeuterung` and `laut` as parameters for the permanent delete and let the
  trash path build its own. Impossible: the caller runs before the body, and the texts need the
  warning reasons the body gathers.

Taken instead: the three parameters `frage: &str`, `erlaeuterung: &str`, `laut: bool` collapse
into **one**, a private module-level enum `Loeschtexte` with the values `AusDenWarngruenden` and
`EndgueltigBisBuendelD`. `in_den_papierkorb` therefore still hands over exactly the three pieces
in which the two commands differ — the order kind, the second button's label, and now the origin
of the texts instead of the texts themselves — so its existing doc comment stays true. The match
in `loeschtexte` is complete and has no catch-all, so bundle D's removal of the second value
stops the build at the arm that has to go, and the name of that value says when it dies.

A side effect worth naming: for `f8` the five facts are **not** gathered at all, because its
texts carry no warning reason. The most expensive of them, `umfang::zaehlen`, opens up to 26
directories.

### The two `expect(dead_code)` lines, measured

Both are gone, and both were `expect` rather than `allow` precisely so the build would stop once
the caller existed: `warngruende` in `kommandos/loeschwarnung.rs` and `liegt_auf_netzlaufwerk` in
`appkit/volumes.rs`. The counting probe `die_ausloesertafel_hat_noch_keinen_aufrufer` is renamed
to `die_ausloesertafel_hat_genau_einen_aufrufer` and expects 1. `liegt_auf_netzlaufwerk` has no
counting probe of its own; the one in `volumes.rs`,
`hier_wird_nicht_nach_der_warnwuerdigkeit_gefragt`, counts `ist_warnwuerdig` and is unaffected —
it still expects zero and still passes.

`Loeschzielbefund::ist_warnwuerdig` therefore still has no caller anywhere, exactly as the
execution annotation of step 10 predicted. That is not an oversight of this task: `warngruende`
must keep `Ja` and `Unentschieden` apart because they lead to different reasons.

## 2 — Record `260817-1108`: the question built before both locks

Resolved by the same change and nothing else. Building the texts moved into the body's **fourth**
branch, i.e. behind all three locks. Consequences: no text is built in any of the three exits
that stop before the sheet; the sentence "Diese 0 Einträge in den Papierkorb räumen?" can no
longer arise, because the empty case never reaches the text site; and the selection is read once
per keystroke instead of twice. `frage_und_erlaeuterung` keeps its plural branch for `0`
unchanged — it is now unreachable, not wrong.

## 3 — Record `260817-1419`, the cost record

The first of the two routes its **Richtung** offered: `vor_der_rueckfrage` takes
`impl FnOnce() -> Loeschzielbefund` instead of a `Loeschzielbefund`, and calls it only in the
`(false, false)` cell of its table. The caller count stays pinned at one, and the rule stays
testable without a window.

**The order of the stages did not change**, which was the condition. It still lives in the table
and nowhere else; what moved is *when* the expensive fact is obtained, and the rule now decides
that instead of its caller. The first two rows of the table read "ungefragt" in the trash column
where they read "gleichgültig" before — same outcome, different price.

Measured, not asserted: `die_teure_tatsache_bleibt_in_den_ersten_zwei_stufen_ungefragt` counts
the closure's calls through a `Cell` and expects zero in the three combinations where a cheap
stage already stops, and exactly one in the fourth cell. The last assertion is the counter-probe:
without it, a rule that never calls the trash test would be green and the third stage silently
disabled.

## 4 — Record `260817-1419`, the two unverified properties

Judged cheap and done. `kommandos::loeschwarnung::nach_der_rueckfrage(bestaetigt,
traegt_auswahl) -> Nachstufe` is the mirror cut to `vor_der_rueckfrage`: two values
(`KeinAuftrag`, `Auftrag`), a table over four cases written out, `#[must_use]`, no catch-all. The
sheet's completion block matches over `(Nachstufe, Option<(Art, Auswahl, PathBuf)>)`; the
`(Auftrag, None)` arm is written out although the table cannot produce it, so the case
distinction is complete without a wildcard.

Three probes: `die_tafel_der_fuenften_stufe_geht_auf` (all four cases),
`ein_abbruch_stellt_keinen_auftrag` (both values of `traegt_auswahl` against `bestaetigt ==
false`, so the precedence and not just the outcome, in the shape used for the pre-stages), and
`genau_ein_fall_stellt_einen_auftrag` (the count).

What remains for the acceptance run is what the record itself named as the real layer: that
AppKit delivers, on a click, on `Return` and on `Esc`, the return value KRK expects.

## Boundary crossed, and why

Two module headers in `krk-core` said "Wer sie ruft: zum Zeitpunkt dieses Schrittes niemand" —
`verzeichnis/umfang.rs` and `verzeichnis/arbeitsbaum.rs`. This task created their caller, so both
sentences became false at the moment of the change. The dispatch names three files as the limit
and neither of these is among them; filing a defect about a false statement one has just written
oneself is worse than the two-line prose fix, so both were corrected in prose only. No code, no
signature, no probe in either file was touched.

## What was not touched

Nothing in `blaetter/`, nothing in the key map, nothing about permanent deletion. `Kommando`,
`Art`, `Vorstufe` and `Warngrund` are unchanged in their sets of values. `operationen.rs` is
unchanged; `operationen::loeschfrage` stays where it is and falls with bundle D.

## Files

- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/kommandos/loeschwarnung.rs`
- `crates/krk-ui/src/appkit/volumes.rs`
- `crates/krk-core/src/verzeichnis/umfang.rs` (module header prose only)
- `crates/krk-core/src/verzeichnis/arbeitsbaum.rs` (module header prose only)

## Records

- `issues/260817-1108_o_…` → `_c_`, `Resolved:` note appended
- `issues/260817-1419_o_der-papierkorbtest-laeuft-vor-den-beiden-billigen-sperren-…` → `_c_`, `Resolved:` note appended
- `issues/260817-1419_o_der-abschluss-von-260817-1107-…` → `_c_`, `Resolved:` note appended
- `issues/260817-1107_c_…` — dated `Nachtrag` appended, marker unchanged

---
**Addendum 260818-0201 (analyst).** This log was added by commit `792995a`, author time
`260817-1739`. Its filename timestamp runs **27 minutes ahead** of that commit, which no clock produces: the
file cannot have been named after the moment it was committed. For placing this session against the
commit log, the author time in this line is what binds, not the filename.

The filename itself stays as it is. It is a pointer, and other records cite it; renaming it would
buy a correct timestamp at the price of dead citations. The finding is
`issues/260817-1807_*_two-history-filenames-and-four-closure-notes-carry-timestamps-that-no-clock-produced.md`,
the rule `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md` `## Timestamps`.
