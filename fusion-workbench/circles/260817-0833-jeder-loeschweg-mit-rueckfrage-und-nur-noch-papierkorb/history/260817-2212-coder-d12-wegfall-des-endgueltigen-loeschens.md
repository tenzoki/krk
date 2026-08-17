# D12 — The permanent delete leaves the code

**Status:** Complete
**Agent:** coder
**Circle:** 260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb
**Source record:** `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 12 (first step of bundle D)
**Binding:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C5; `shared/decisions/260817-0536_a_was-geschieht-mit-einer-gespeicherten-keymap-die-die-entfallene-funktion-fuehrt.md`
**Tree state before the task:** `cdde9da`, working tree clean
**Verification:** `cargo build --workspace` — exit 0; `cargo clippy --workspace --all-targets -- -D warnings` — exit 0; `cargo fmt --all --check` — exit 0; `cargo test --workspace --no-fail-fast` — exit 101, 52 failures, all of them the expected ones (see below)

> This log is written in English because `CLAUDE.md` declares `**Artifact language:** en`.
> The code stays German, identifiers and prose alike.

## What fell

`Kommando::EndgueltigLoeschen` and `Art::EndgueltigLoeschen` no longer exist. `grep -rn
"EndgueltigLoeschen" crates/` returns nothing. Everything the compiler demanded went with them,
and three things it did not demand went too because they were now false statements about the
tree.

### The compiler-driven removals

| Site | What went |
|---|---|
| `krk-core/src/operation/auftrag.rs` | the `Art` value, `Auftrag::endgueltig_loeschen`, the arm in `zielordner`; the probe `ein_loeschauftrag_hat_keinen_zielordner` now builds its order with `in_den_papierkorb` |
| `krk-core/src/operation/mod.rs` | the dispatch arm into `loeschen::endgueltig_loeschen` |
| `krk-core/src/operation/loeschen.rs` | `endgueltig_loeschen` and, with it, the `Typ` import |
| `krk-core/src/tasten/belegung.rs` | the `Kommando` value, its `KENNUNGEN` row, its arm in `wirkungsbereich` |
| `krk-core/tests/belegung.rs` | the assertion about its `Wirkungsbereich` |
| `krk-core/tests/operation.rs` | `endgueltiges_loeschen_raeumt_einen_ordner_mit_inhalt_ab` |
| `krk-ui/src/belegungsmodell.rs` | its arm in `bereich`, the shipped-keymap assertion at the old line 953 |
| `krk-ui/src/auffrischung.rs` | the arm in `schiebt_auffrischung_auf`, and `die_gemaechlichen` shrinks from four kinds to three |
| `krk-ui/src/kommandos/fokus.rs` | its entry in the C5 focus list |
| `krk-ui/src/kommandos/operationen.rs` | its arm in `ueberschrift`, `loeschfrage`, and the two probes over `loeschfrage` |
| `krk-ui/src/appkit/anwendung.rs` | the command dispatch arm, `Anwendungsdelegierter::endgueltig_loeschen`, two arms over the operation kind |

`Kommando::KENNUNGEN` is now `[(Kommando, &'static str); 78]`, down from 79. The type carries the
number, so the build is the check.

**`baum_entfernen` was not touched.** It keeps its two callers, `operation::mod.rs:244` (replacing
an existing target) and `operation/verschieben.rs:123` (moving across a volume boundary), plus its
own recursion. `git diff crates/krk-core/src/operation/loeschen.rs` shows no line of it.

### The judgement call: `Loeschtexte` went with the second value

Step 11 introduced a private enum `Loeschtexte` in `appkit/anwendung.rs` with exactly two values,
and its own doc comment said why: the complete match in `Anwendungsdelegierter::loeschtexte` would
stop the build at the arm that has to go once bundle D removed the second value. It did. What
remained was a one-value enum threaded as a parameter through two functions, which carries no
distinction at all, so the enum, the parameter `textform` on `loeschen_nach_rueckfrage`, the
parameter on `loeschtexte` and the match all went. The body of the surviving arm is now the body of
`loeschtexte`.

**What was deliberately *not* collapsed:** `in_den_papierkorb` still delegates to
`loeschen_nach_rueckfrage(art, schaltflaeche)`, and that body now has exactly one caller passing
exactly one value per parameter. Folding the two into one function is the honest end state, but it
is a structural change the plan does not ask for and it would rewrite roughly 200 lines of doc
comment that bundle E's step 15 is about to read. It is named here so the next reader does not
mistake it for an oversight.

### Three prose sites the compiler did not name

All three stated something about the tree that stopped being true with this step, and all three
were in the acceptance criterion's literal reach ("`EndgueltigLoeschen` does not occur under
`crates/` any more"). Corrected in prose only, no code:

- `krk-ui/src/kommandos/rueckschritt.rs`, module header: `f8` and `opt+cmd+delete` were listed as
  carrying the permanent delete. `f8` now falls out at `ist_nackter_rueckschritt` instead, which is
  a different reason for the same outcome, and the header says so.
- `krk-ui/src/appkit/ereignisse.rs`, `ist_nackter_rueckschritt`: same correction at the point where
  the distinction is made.
- `krk-ui/src/kommandos/loeschwarnung.rs`, module header: "`f8` kommt erst mit Bündel D dazu" was
  written in the future tense; bundle D is here. The count of keys running through
  `in_den_papierkorb` in the paragraph above it went from two to three for the same reason.

Step 15 owns the wider prose sweep over `endgueltig|endgültig` and will pass all three again. Four
further doc links pointing at now-removed items were repaired so `cargo doc` gains no new
unresolved-link warnings; the tree's pre-existing ones are untouched.

## The new probe: a user answer measured instead of asserted

`crates/krk-core/tests/belegung.rs::eine_keymap_mit_der_zurueckgezogenen_kennung_wird_als_ganzes_verworfen`.

This project has never withdrawn a function id before. Anyone who ever saved their key map through
the key-map view carries `endgueltig_loeschen` in their `keymap.toml`, because saving writes the
complete map and not only what changed. The user was asked what should happen and chose option 1 on
260817: nothing is built, the whole user file is discarded, the shipped map takes over, and the
status line names the file
(`shared/decisions/260817-0536_a_was-geschieht-mit-einer-gespeicherten-keymap-die-die-entfallene-funktion-fuehrt.md`).

**An answer that builds nothing leaves nothing behind that a later change would break.** The probe
is that missing thing. It has two halves, because the answer has two statements: `Belegung::vom_nutzer`
rejects the file with `Belegungsfehler::UnbekannteFunktion("endgueltig_loeschen")`, and
`belegung::laden` falls back to the **complete** shipped map and reports the replacement.

## The tree is red, and these are the failures

`cargo test --workspace --no-fail-fast` exits 101 with 52 failing probes in two binaries. Every one
of them names `endgueltig_loeschen`, and every one has the same root cause: `resources/default-keymap.toml`
still carries the entry, which step 13 removes. The two crates fail from opposite sides of the same
fact.

```
51 in krk-ui (bin krk)      the shipped map lists a function that now has no Kommando,
                            hence no Funktionsbereich
   49 × belegungsmodell.rs:831   panic inside `gliederung`, reached by every probe
                                 that builds `Belegung::auslieferung()`
    1 × belegungsmodell.rs:962   `jede_kennung_hat_einen_funktionsbereich`
    1 × belegungsausgabe.rs:842  `jede_kennung_ohne_kommando_wird_vom_menue_zugestellt`

 1 in krk-core (tests/belegung.rs)
                            the new probe: the vocabulary still knows the id,
                            so no `UnbekannteFunktion` is raised
```

Nothing else in the workspace fails. `krk-bench`, `xtask`, and the other fifteen krk-core test
binaries are green.

Two predictions in the dispatch did not hold, and neither is a defect:

- `krk-core/tests/belegung.rs::jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` passes.
  It checks that every built command has a shipped key, not that every shipped entry has a command,
  so a surplus entry in the file does not reach it.
- `belegungsausgabe.rs:758` (`mit_kommando == Kommando::KENNUNGEN.len()`) is not among the failing
  assertions. Its probe fails earlier, at line 842.

The key map was **not** touched and no probe was weakened.

## Files

- `crates/krk-core/src/operation/auftrag.rs`
- `crates/krk-core/src/operation/mod.rs`
- `crates/krk-core/src/operation/loeschen.rs`
- `crates/krk-core/src/tasten/belegung.rs`
- `crates/krk-core/tests/belegung.rs`
- `crates/krk-core/tests/operation.rs`
- `crates/krk-ui/src/belegungsmodell.rs`
- `crates/krk-ui/src/auffrischung.rs`
- `crates/krk-ui/src/kommandos/fokus.rs`
- `crates/krk-ui/src/kommandos/operationen.rs`
- `crates/krk-ui/src/kommandos/rueckschritt.rs` (module header prose only)
- `crates/krk-ui/src/kommandos/loeschwarnung.rs` (module header prose only)
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/ereignisse.rs` (doc comment prose only)

## Records

- `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`, step 12 → `[DONE]`
- No decision record moved. `shared/decisions/260817-0536_a_was-geschieht-mit-einer-gespeicherten-keymap-die-die-entfallene-funktion-fuehrt.md`
  is realised by the new probe, but its marker walk belongs to step 16 and waits for the commit.
