# Two review defects: the drop's message clears both panes, and `vorgang_laeuft` gets its `#[must_use]`

**Date:** 2026-08-18
**Status:** Complete
**Agent:** coder
**Circle:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps`
**Issues:** `issues/260818-2332_o_the-drop-writes-a-rank-1-message-without-clearing-the-other-pane-and-loses-it.md` (high), `issues/260818-2335_o_vorgang-laeuft-carries-no-must-use-and-the-plan-number-that-kept-it-off-is-already-false.md` (low)
**Tree state at start:** `a7419cd`, working tree clean
**Verification:** `make check` — exit 0

## Defect 1: the drop's rank-1 message was lost over the non-active pane

Two files, `crates/krk-ui/src/appkit/anwendung.rs` and `crates/krk-ui/src/appkit/tabelle.rs`.

**The one deletion rule got a name instead of a second copy.** The both-sides loop that stood
inline in `kommando_ausfuehren` moved out into
`Anwendungsdelegierter::befehlsantwort_beidseitig_loeschen`. It is the same loop over
`Fensterseite::ALLE` calling `DateifensterQuelle::befehlsantwort_loeschen`, at the same point in
the same order; nothing about the keystroke path changed. `befehlsantwort_loeschen` therefore
still has exactly **two** call sites in the tree — the loop and `doppelklick` — where the record
predicted three. Extracting rather than adding is what keeps that number where it was.

**The drop reaches the rule through an eighth callback.** A `DateifensterQuelle` can only reach
its own side; the rank-1 field belongs to both panes together. `befehlsantwort_raeumer:
RefCell<Option<Box<dyn Fn()>>>` is the way out, wired in `oberflaeche_aufbauen` beside the two
callbacks the drop already had, weak like all seven before it. `DateifensterQuelle::
befehlsantwort_beidseitig_loeschen` is the one place that decides what a missing callback means:
nothing happens. That is deliberately not a narrower second rule — a half clearing at this side
alone would be exactly the second deletion rule the round is avoiding — and the case cannot arise,
the same argument `vorgang_laeuft_fragen` beside it already makes.

**The clearing edge is `abwurfmeldung`'s `Some`, and nothing wider.** It sits inside the `if let
Some(meldung)` arm in `abwurf_pruefen`, before `befehlsantwort_zeigen`. `validateDrop:` runs on
every pointer movement, and `abwurfmeldung` is the de-duplication that turns that stream into the
few reason changes; hanging the clearing on the same `Some` means it runs exactly as often as a
message is written and no more. Three things follow, and each was checked:

- `gemeldeter_abwurfgrund` is set **after** the clearing, because the clearing put it to `None` on
  both sides. The judgement just made belongs there, not the cleared one.
- `beschlossener_vorgang` is untouched. `befehlsantwort_loeschen` does not read or write it, so the
  opposite deletion rule that field deliberately carries stays opposite; a keystroke during a
  standing drag still cannot take the release its operation.
- The other pane's `gemeldeter_abwurfgrund` falls with its message, which is the wanted effect: the
  C7 message follows the pointer between panes instead of sticking to the side that first had it.

**No live borrow crosses the new call.** At that point in `abwurf_pruefen` everything in hand is
owned — `eintrag_in_zeile` and `angezeigter_ordner` return owned values, as the function's doc
comment requires. The callback re-enters this same source through `befehlsantwort_loeschen` →
`meldung_gewechselt` → `statuszeile_nachziehen`, which is the regime `befehlsantwort_zeigen`
already ran in one line further down.

## Defect 2: `#[must_use]` on `vorgang_laeuft`

`Anwendungsdelegierter::vorgang_laeuft` carries the attribute with its reason written out, in the
shape the round's seven others use. Nothing else changed: the build was already green without a
bare caller, so the attribute adds a guard rather than fixing a live drop.

The plan's Testing Strategy sentence ("diese Runde setzt vier neue `#[must_use]` und ein `let _ =`")
is **not** corrected here — the plan is not this agent's file, and the correction was reported to
the user instead. Counted numbers stand in the record.

## Documentation pulled back into line

Six doc comments said something the change made false or incomplete, and all six were corrected
rather than left:

1. `befehlsantwort_loeschen` — "ihre zwei Aufrufer" now names the extracted rule as the first, and
   separates two call sites from three occasions.
2. `doppelklick` — its argument for clearing only its own side cited "a third callback, which does
   not exist today". The callback now exists. The decision stands on reach, not on availability,
   and the doc now says so, so nobody later attaches it because it happens to be there.
3. `abwurfmeldung` — names the clearing that hangs on the same `Some`.
4. `gemeldeter_abwurfgrund` (ivars) — the message following the pointer between panes.
5. `abwurf_pruefen` — the order list now carries the clearing.
6. `oberflaeche_aufbauen` — "zwei Rueckrufe" for the drop became three.

## Why there is no new test

The behaviour needs a live `DateifensterQuelle` (an `NSTableView` and an `NSScrollView`) and an
`NSDraggingInfo`, and `libtest` gives no main thread; the tree's own standard is that a test may
assert the main thread only where it never reaches AppKit (`blaetter/mod.rs`,
`editor.rs::an_einer_flaeche`). Two substitutes were considered and rejected as tests that could
not fail for the right reason:

- **A caller count on `befehlsantwort_loeschen`.** `crate::quellbaum`'s module head forbids exactly
  this — a caller count belongs only where an acceptance criterion promises the number itself, and
  none does here. It is also blind in both directions.
- **A `statuszeile::zeile` test showing the non-active pane's message winning when the active pane
  is empty.** It would pass before and after the fix, which is the shape of test this session
  already found once and removed. The contest itself is already measured by
  `bei_gleichem_rang_gewinnt_die_aktive_seite` and
  `der_hoehere_rang_der_inaktiven_seite_schlaegt_den_niedrigeren_der_aktiven`.

What holds the fix instead is construction: the clearing and the write live in the one `if let`
arm, so the message cannot be written without it, and the clearing itself has one body in the tree.
The visible behaviour is user acceptance work, like the rest of C7.

## Left alone deliberately

Three findings of the same review are open and were not touched: `260818-2333` (the same-folder
refusal compares paths textually), `260818-2334` (every pointer movement decodes the whole
pasteboard), `260818-2336` (a vanished row redirects the drop to the parent folder). The last two
describe `abwurf_pruefen`, the function this change edits; neither the pasteboard decode nor the
vanished-row fallback was moved, renamed, or reasoned about here.

Not committed, and the two issue records were not closed — both are the user's.
