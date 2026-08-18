# Step 9: the running-operation question loses its side effect, and the operation machine gains an entry

**Date:** 2026-08-18
**Status:** Complete
**Agent:** coder
**Circle:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps`
**Plan:** `planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, step 9
**Tree state at start:** `07347b8`, working tree clean for `anwendung.rs`

## What was built

One file, `crates/krk-ui/src/appkit/anwendung.rs`.

1. **`vorgang_laeuft(&self) -> Option<Art>`** reads `ivars().vorgang` and reports nothing. The
   doc comment that used to justify a single answering place moved here and now names the ways
   in, including the drop, which takes the question without the message because `validateDrop:`
   runs on every pointer movement.
2. **`vorgang_laeuft_schon(seite) -> bool`** became its caller. Behaviour and message are
   unchanged; its doc comment kept the history of the message (the 260804-1915 defect) and points
   at the new function for the question itself.
3. **`abwurf_ausfuehren(&self, seite, ziel, quellen, art)`** builds through `Auftrag::kopieren` /
   `Auftrag::verschieben` and calls `auftrag_starten(seite, auftrag, ziel, quellen.len())`. It
   carries `#[expect(dead_code)]` until step 10 supplies the caller, in the shape
   `kommandos/abwurfregel.rs` and `kommandos/rueckschritt.rs` use.
4. `use crate::kommandos::abwurfregel::Abwurfvorgang;` added.

## Three things the tree said and the plan did not

Each was checked against the tree rather than taken from the plan.

- **`abwurf_ausfuehren` is the fourth caller of `auftrag_starten`, not the third.** The plan calls
  it the third in three places while its own Current State counts three existing callers. Counted
  at `07347b8`: `loeschauftrag_stellen`, `stapel_beauftragen`, `auftrag_stellen`. The doc comment
  says four and says the plan says three. Filed as
  `issues/260818-2228_o_step-9-of-the-plan-calls-the-new-caller-the-third-…`.
- **`ivars().vorgang` had a second direct reader**, `loeschen_nach_rueckfrage`, which took
  `.borrow().is_some()` for its own bool. Step 9 requires writing "there is still one answering
  place" into a doc comment; that line was routed through `vorgang_laeuft()` so the sentence is
  true rather than asserted. No behaviour change; the delete path pays one `Art` clone per
  keypress, on a path that already resolves a path and raises a sheet. This is the one step
  beyond the literal wording of the plan.
- **The target folder goes in as `quellordner`**, as step 9 prescribes, so `Vorgang::ordner`
  names it twice and the completion reads it twice. Checked what that costs: the second
  `neu_lesen` re-derives the selection name from the inventory still standing and the generation
  counter drops the first run's batches, so the only cost is one extra directory walk per drop.
  Written into the doc comment and filed as
  `issues/260818-2221_o_the-drop-passes-its-target-as-the-source-folder-…`.

Two stale counts in the same file were corrected while passing: `auftrag_starten`'s "der beiden
Wege" (four now) and the number of ways that ask about a running operation (four, three of them
through the reporting wrapper).

## Judgement left open

`vorgang_laeuft` did **not** get `#[must_use]`. The plan's Testing Strategy states that this
round sets four of them (`bereich_einblenden`, `marke`, `urteil`, `beschreibbarkeit`), and a
fifth would contradict that sentence. The project rule would arguably ask for one, since a bare
call does nothing at all. Left to the review.

## Verification

`make check` — exit 0 (build, test, `cargo fmt --all --check`, `cargo clippy --workspace
--all-targets -- -D warnings`).

An earlier run failed at `fmt-check` with exit 2 on `crates/krk-ui/src/appkit/abwurf.rs`, a file
another agent was writing at the same moment in the same tree; a later run with that file
finished is the green one. `anwendung.rs` was formatted alone, with `rustfmt --edition 2024` on
the single file, so no other agent's in-flight file was reformatted.

Neither `/tmp` nor `$TMPDIR` held a `krk-messplan-*.toml` before the run.

## Not done here

No commit. Step 10 removes the `expect(dead_code)` when
`DateifensterQuelle::abwurf_annehmen` calls the new body.
