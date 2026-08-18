# Code review: round 13, Turn 2 — the drop from foreign applications, and the corrections to Turn 1's command

**Reviewed-range:** `71413c3..a7419cd`
**Not-opened:** `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-1615-shaper-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-1633-planner-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-1740-coder-das-kommando-in-die-vier-pflichtstellen.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-2112-coder-die-acht-prosazahlen-nachziehen.md`, `fusion-workbench/shared/history/260818-1117-orchestrator-session.md`, `fusion-workbench/shared/history/260818-1453-shaper-runde-13-gleiches-verzeichnis-und-abwurf.md`, `fusion-workbench/shared/history/260818-1510-shaper-verzeichnis-angleichen-und-abwurf.md`, `fusion-workbench/orchestrator-events.jsonl`
**Sender:** coderev
**Date:** 260818-2340
**Build state at review time:** `cargo test --workspace`, `cargo clippy --workspace --all-targets` and `cargo fmt --all --check` all green, run at `a7419cd`.

## The carried files, judged

The Turn 1 review declared eleven unopened files. Three of them were worth opening for this pass and were opened:

- `circles/260818-1615-…/decisions/260818-1633_i_gilt-ein-unentscheidbares-schreibrecht-…` — binds C6 Lage 2 and moved `_a_`→`_i_` inside this range. Opened; its `Implemented:` citation is accurate (see below).
- `shared/decisions/260818-1453_i_welche-zusatztaste-macht-aus-einem-abwurf-ein-verschieben.md` — binds C5 and likewise moved to `_i_` in this range. Opened; accurate.
- `circles/260818-1615-…/_t_circle.md` — opened, for the Grounding snapshot.

The remaining eight are the six session histories and `orchestrator-events.jsonl`. They are per-agent narrative and an event stream; neither carries a statement the code owes anything to, and both decision records they would point at have now been read directly. They stay unopened and are declared again above rather than dropped.

## Summary

Turn 2 is the strongest work in this Circle so far. Every availability number in the three module heads was re-read against the SDK during this review and all of them are correct, including the one corrected this round; the `unsafe` boundary is drawn exactly where `objc2` draws it; the two caller-count promises hold against the built tree and the `krk-bench` name collision is correctly excluded; the borrow rule of `tabelle.rs` is respected in both new bodies; and the four combinations of reveal and read in `ordner_angleichen` are disjoint and complete after `a6b3818`.

Five defects are filed. None of them is a release blocker in the sense of data loss — the one candidate for that was measured and is not one — but the first is a straightforward functional failure of a C7 acceptance criterion, and it will be found by the user's acceptance run rather than by anything a machine can check.

## Totals

| Severity | Count |
|---|---|
| Critical | 0 |
| High | 1 |
| Medium | 2 |
| Low | 2 |

## What is right, stated as plainly as what is wrong

Since C4 to C7 are entirely user acceptance work, this review is the only machine-side check the drop gets. What follows was verified, not assumed.

**The macOS floor.** Every symbol in the three new or extended module heads was looked up in `$(xcrun --show-sdk-path)` during this review, not taken from the commits. All correct, with the cited line numbers matching:

- `appkit/abwurf.rs`: `NSURLResourceKey` (`NSURL.h:17`), `fileURLWithPath:` (`:52`), the caching sentence (`:181`), `resourceValuesForKeys:error:` 10.6 (`:183`), `NSURLIsWritableKey` 10.7 (`:247`), `objectForKey:` (`NSDictionary.h:17`), `boolValue` (`NSValue.h:73`), `NSFilePromiseReceiver` 10.12 (`NSFilePromiseReceiver.h:19`), `readableDraggedTypes` (`:23`), `NSPasteboardType` (`NSPasteboard.h:23`), `NSPasteboardTypeFileURL` 10.13 (`:39`), `NSDragOperation`/`None`/`Copy`/`Move` (`NSDragging.h:25`/`:26`/`:27`/`:31`), `NSDraggingInfo` (`:69`), `draggingSourceOperationMask` (`:72`).
- `appkit/tabelle.rs`: `registerForDraggedTypes:` (`NSView.h:488`), `draggingPasteboard` (`NSDragging.h:79`), `NSTableViewDropOperation` (`NSTableView.h:25`), `setDropRow:dropOperation:` (`:319`), `validateDrop:` (`:783`), `acceptDrop:` (`:787`). The in-body citation of `NSTableView.h:317` for the meaning of row `-1` is also exact.
- `appkit/zwischenablage.rs`: `pasteboardWithName:` (`NSPasteboard.h:160`, no annotation), `writeObjects:` 10.6 (`:183`), `readObjectsForClasses:options:` 10.6 (`:190`), `NSPasteboardURLReadingFileURLsOnlyKey` 10.6 (`:146`), and the three type constants at `:24`, `:26`, `:27`. The correction of `NSPasteboardTypeFileURL` from 10.6 to 10.13 is right, and the head now attaches its line number to the name it belongs to rather than to a pair.

Zero findings here. Given that this session had already caught two collapsed pairs and one number three releases too early, that is a real result and not a formality.

**The `unsafe` boundary.** Checked against the vendored crates, not against the comment:

- `registerForDraggedTypes` is bound safely (`objc2-app-kit-0.3.2/src/generated/NSView.rs:1412-1414`, `pub fn` without `unsafe`), and the call correctly carries no block. The in-body comment's citation is exact.
- `readObjectsForClasses_options` **is** `unsafe` (`NSPasteboard.rs:373`) with two documented requirements, and the SAFETY sentence addresses exactly those two — the class array holds only `NSURL`, which implements `NSPasteboardReading`, and the options dictionary holds only the one key whose value type the header names.
- `resourceValuesForKeys_error` is bound safely (`objc2-foundation-0.3.2/src/generated/NSURL.rs:1396`) and is correctly called without a block.
- The three foreign symbol reads (`NSPasteboardTypeFileURL`, `NSURLIsWritableKey`, `NSPasteboardURLReadingFileURLsOnlyKey`) each need their block and each carry a sentence. No new `#![allow(unsafe_code)]` appeared; `appkit/mod.rs` remains the one exception in this crate.

**The one-caller promises hold in the built tree.** Verified by hand rather than by trusting the green test:

```
krk-ui/src/appkit/tabelle.rs:2974:        let marke = abwurfregel::marke(
krk-ui/src/appkit/tabelle.rs:2988:        let gefaellt = abwurfregel::urteil(&Abwurflage {
```

and nothing else in `krk-ui` outside `abwurfregel.rs` itself. The `KISTE` prefix does what its doc comment says it does: `krk-bench` carries its own `urteil` with five call sites (`bericht.rs:360`, `messen.rs:1932`, `:2504`, `:2515`, `:2576`), and all five are correctly outside the count. The registration count is likewise one, and `leiste.rs` carries no `registerForDraggedTypes` — the bookmark and device bar takes no drop by omission, exactly as C4's last criterion needs.

`abwurf.rs`'s claim to hold the only translation of `NSDragOperation` in both directions also holds: `tabelle.rs` names the type four times and every one of them is a signature or a doc comment. No second place reads or assembles a mask.

**The undecidable write right.** All three undecidable paths in `beschreibbarkeit` return `Unbekannt` and never `Nein` — a path that is not valid UTF-8 (first line, before any Objective-C), an error from `resourceValuesForKeys:error:`, and a missing or wrongly typed value, all through one `let … else`. `urteil` writes `Ja` and `Unbekannt` out side by side rather than folding them into a `_`, so a fourth `Schreibrecht` value stops the build. Three tests cover the three shapes, including the non-UTF-8 one. The decision record `260818-1633_i_…` cites `d6343e0` and the citation is accurate.

**The two ivars and their opposite clearing rules.** Both halves verified.

- `gemeldeter_abwurfgrund` falls in `befehlsantwort_loeschen` unconditionally, ahead of the `if` that guards the message itself — correct, and the reason given (a memory outliving its subject silences a repeat gesture) is the right one.
- `beschlossener_vorgang` is written only in `abwurf_pruefen` and read only in `abwurf_annehmen`. A drag that ends outside the list does leave it stale, and that is safe: AppKit calls `acceptDrop:` only after a `validateDrop:` that returned something other than `None`, so the field is always rewritten before it is read. The safety rests entirely on that ordering contract, which the code states in the right place. No `draggingExited:` reset is needed and adding one would be a second clearing rule for no gain.

**The borrow rule.** Both new bodies respect it. `abwurf_pruefen` touches the tab model only through `eintrag_in_zeile` and `angezeigter_ordner`, and both end their borrow by returning owned values before the caller reaches `setDropRow:` or the message; `abwurf_annehmen` likewise. `vorgang_laeuft_fragen` and `abwurf_annehmen` do hold a `RefCell` borrow of their own callback field across the callback invocation, but that is the file's established idiom (`vorgang_beenden` does the same), the borrowed cell is not the tab model, and its only `borrow_mut` is in a setter that runs at construction. Not a finding.

**The tables.** `marke` covers all eight combinations with no catch-all; `urteil`'s first table covers all 24 of the four leading facts and its second all four of the offer, and the two are proved independent by a third test rather than multiplied into 96 rows. `abwurfmeldung`'s 6×6 table is written out with its diagonal — the de-duplication C7 asks for — as the assertion.

**The reveal/read split (`a6b3818`).** The four combinations are disjoint and complete, and the too-narrow refusal still stops the read for the reason C2 gives:

```
                 ordner == dort            ordner != dort
sichtbar         nothing, message, false    read, true
!sichtbar        reveal, message, true      reveal + read, true
reveal refused   message, false, no read    message, false, no read
```

The `return !sichtbar` in the equal-folder arm is right and its comment explains why: the refusal branch above has already consumed the only other meaning `!sichtbar` could carry at that point.

**The keymap change (`a7419cd`).** The replacement of the enumeration by a command is correct as written. Run against the tree it yields eleven combinations, and the one further `opt+cmd+f` in the file is a prose mention of a free alternative at line 452, not an assignment.

## Findings by theme

### Status line

**1 · High — the drop writes a rank-1 message without clearing the other pane, and the message is then never seen.**
`circles/260818-1615-…/issues/260818-2332_o_…`

`abwurf_pruefen` (`tabelle.rs:3016-3019`) writes the C7 sentence into `Rang::Befehlsantwort`, the top rank. Every other writer of that slot arrives through `kommando_ausfuehren`, which first clears it on **both** panes (`anwendung.rs:2905-2907`); the drop is the third writer and the only one that does not. `statuszeile::zeile` prefers the active side within a rank (`statuszeile.rs:599`), so a drop over the **inactive** pane, while the active pane still holds a command answer, produces a message that is written and never rendered. The user sees the system's refusal symbol and no sentence — the exact state C7 exists to prevent. `abwurfmeldung`'s de-duplication then suppresses the repeat as well, until the next keystroke clears both panes.

Scope: `krk-ui`, the drop path only. The fix that keeps one deletion rule is to give the drop the same both-sides clearing, inside the arm that actually writes.

### Correctness of the early refusals

**2 · Medium — the same-folder refusal compares a KRK path against a foreign application's path textually.**
`circles/260818-1615-…/issues/260818-2333_o_…`

`ziel_ist_quellordner` (`tabelle.rs:2992-2995`) compares `PathBuf`s where one side comes from KRK's folder model and the other from `NSURL::path` on URLs written by the sending application. Two spellings of one folder — `/tmp` against `/private/tmp`, a bookmark through a symlink, a case difference on this volume — read as two folders and C6 Lage 3 does not fire.

The reasoning that legitimately covers the *other* textual comparison in this round (`ordner_angleichen`, `anwendung.rs:3378-3392`) does not transfer: there both sides are KRK's, the error is one-directional, and the consequence is one redundant read. Here one side is foreign and the consequence is a lost refusal that `abwurf_ausfuehren` deliberately does not re-ask.

Measured on this machine on 260818, so that the severity is honest rather than alarming: `copyfile(3)` with `COPYFILE_ALL` onto the same file through a symlinked directory returns 0 and leaves the file intact, and the move path's `rename(2)` is a documented no-op in that case. **This is not data loss.** What the user gets is the conflict query once per dragged entry for a drop that should never have been offered.

**3 · Medium — every pointer movement decodes the whole drag pasteboard, and neither spec nor plan names that cost.**
`circles/260818-1615-…/issues/260818-2334_o_…`

`abwurf_pruefen` calls `dateiverweise` on every `validateDrop:` (`tabelle.rs:2987`), which materialises one `NSURL` and one `PathBuf` per dragged entry, and then walks the same n paths again in the folder comparison. The whole result feeds one bit plus that comparison, and nothing is kept between movements.

The spec enumerated what the round newly puts on the main thread as "ein Vergleich zweier Pfade und eine Frage nach dem Schreibrecht des Zielordners" — two O(1) terms. The one term that scales with n is in neither spec nor plan. It is also the term that threatens the round's own stand-in for an eleventh time promise, "die Liste bleibt bildlauffähig". `speculation:` — I have not measured it, and for small drags it is certainly irrelevant. The record is filed for the missing statement as much as for the cost.

**5 · Low — a vanished row silently redirects the drop to the parent folder.**
`circles/260818-1615-…/issues/260818-2336_o_…`

`abwurf_annehmen`'s fallback `None => self.angezeigter_ordner()` (`tabelle.rs:3064`) is reached not only for the `-1` that means "the displayed folder" but also for a non-negative row whose entry has disappeared in a refresh between the last pointer position and the release. In that branch the drop retargets from the marked sub-folder to its parent, successfully and silently. The doc comment above reasons carefully about the *other* branch of that race and names its outcome as a reported skip; this third branch is not in the split, and its outcome class is different. The same three lines also drop the `Typ`, so a row that has become a file is accepted as a destination folder.

### Project conventions

**4 · Low — `vorgang_laeuft` carries no `#[must_use]`, and the plan number that kept it off is already false.**
`circles/260818-1615-…/issues/260818-2335_o_…`

The judgement was explicitly referred to this review, so here it is: **it should carry one.** It is a pure query since step 9 removed its side effect, a bare call compiles green because `unused_results` is allow-by-default, and what would be silently dropped is the answer to C6 Lage 1 — the one question the drop path is forbidden to ask twice. That is the same argument step 3 already accepted for `bereich_einblenden`.

The number that argued against it does not bind, because it was already wrong. Counted against this range, the Turn added **seven** `#[must_use]` (four in `abwurf.rs`, two in `abwurfregel.rs`, one in `tabelle.rs`), Turn 1 an eighth, and the round added **two** `let _ =`, not one. The plan's sentence is off by four and by one independently of this question.

## Cross-cutting observations

**One pattern connects findings 1 and 4.** Both are cases where the drop joins a set that had a rule, and the rule's own bookkeeping was not updated: the rank-1 writer set grew from two to three without the clearing rule growing with it, and the `#[must_use]` count grew from four to eight while the sentence stating it did not. The round handled the *same* shape correctly in three other places — the keymap comment was replaced by a counting command in `a7419cd`, `abwurf_ausfuehren` counted its own position as the fourth caller against the tree rather than trusting the plan's "third", and the availability heads give every symbol its own line number. The discipline is present; it lapsed on two sets nobody re-counted.

**Findings 2 and 5 are both incomplete case splits with a silent branch.** In each, the doc comment reasons about the branches it knows and the missing branch is the one whose outcome is "quietly did something else" rather than "reported a skip". They are worth fixing together, because both live in the three lines that turn a row into a target.

**The three availability heads and the two decision records show what this round did unusually well.** Every number checked, every claim about the tree counted against the tree, and the two records' `Implemented:` citations naming a commit and the exact functions. Nothing in that layer needed correcting.

## Recommended sequencing

**Before the user's acceptance run:** finding 1. It makes a C7 criterion fail in a configuration the user is likely to hit, and it is a small change at one point.

**Before or with finding 1, since both touch `abwurf_pruefen`:** finding 2, if option 1 of that record (compare by device and inode) is chosen — it changes the same expression that finding 3 would cache.

**Cleanup, no ordering constraint:** findings 4 and 5. Finding 4 is two lines of code and one sentence of plan text; finding 5 is a match arm and a paragraph.

**Measure first, then decide:** finding 3. If the user's acceptance run includes a large multi-selection drag and the list stays responsive, the record can close with that measurement and only the spec's enumeration needs correcting.

**Nothing here blocks `cargo xtask release`.** The build is green on all four acceptance commands, no defect risks data, and C4 to C7 are unaccepted in any case until the user runs them in the foreground.
