# The three remaining Turn 2 defects, and the data loss the first of them turned out to reach

**Date:** 2026-08-19
**Status:** Complete
**Agent:** coder
**Circle:** `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps`
**Issues:** `issues/260818-2333_o_the-same-folder-refusal-compares-a-krk-path-against-a-foreign-apps-path-textually.md` (medium), `issues/260818-2334_o_every-pointer-movement-decodes-the-whole-drag-pasteboard-and-nothing-names-that-cost.md` (medium), `issues/260818-2336_o_a-vanished-row-silently-redirects-the-drop-to-the-parent-folder-and-the-doc-comment-does-not-cover-it.md` (low)
**Tree state at start:** `4d27c1c`, working tree clean
**Verification:** `make check` — exit 0

## Summary of what changed, and where

| Defect | Decision | Where the change landed |
|---|---|---|
| 2333 — same-folder refusal compares textually | behaviour changed, but **not** at `abwurf_pruefen` | `krk-core/src/operation/mod.rs` |
| 2334 — per-movement pasteboard decode | measured, then cached | `krk-ui/src/appkit/tabelle.rs` |
| 2336 — vanished row redirects the drop | behaviour changed: the drop is refused | `krk-ui/src/appkit/tabelle.rs` |

## Defect 2333: the record's severity was too low, and the fix belongs elsewhere

### What the measurement in the record missed

The record filed this at Medium on the grounds that no data is lost: `copyfile(3)` with
`COPYFILE_ALL` onto the same file returns 0 and leaves the file intact, and `rename(2)` on the
same file is a documented no-op. Both statements are true about those two calls. **The tree does
not reach them with an existing destination.** `operation::ziel_klaeren` asks the conflict
question first, and `Konfliktantwort::Ueberschreiben` answers it by calling
`loeschen::baum_entfernen(ziel)` — a real `remove_file`/`remove_dir`, not the trash. Where the
destination *is* the source under a second spelling, that call deletes the user's file, and the
copy that follows then fails on a source that no longer exists. The entry appears in the
completion list as "gibt es nicht mehr", over a file that existed before the drop.

Proven, not reasoned: a test written against the unfixed tree failed with
`die Quelle ist weg: Os { code: 2, kind: NotFound }`. It is now
`ein_ziel_das_ueber_einen_verweis_die_quelle_selbst_ist_wird_uebersprungen`
(`krk-core/tests/operation.rs`).

A second, smaller hazard sits in the same three lines. The folder guard
`ziel.starts_with(quelle.pfad)` is textual as well, so a folder dropped onto a second spelling of
one of its own ancestors made the copy descend into its own tree. Also proven against the unfixed
tree: 139 entries copied before the test's assertion caught it
(`ein_ziel_das_ueber_einen_verweis_in_der_quelle_liegt_wird_uebersprungen`).

Both hazards predate the round in the code, and both became **reachable** with it: until C4 both
paths came out of KRK and carried the spelling the user had walked, while the drop takes its
source paths from whatever the sending application wrote, and applications write them resolved.

### Why the fix is not in `abwurf_pruefen`

The question `abwurf_pruefen` asks — "is the folder under the pointer the folder being dragged
from?" — cannot be *decided* there at any price. Even an exact `(st_dev, st_ino)` comparison per
pointer movement would still be a prediction, because the folder can change between the last
movement and the release. Making the prediction exact would also put a syscall into a path that
runs on every pointer movement, which is precisely what defect 2334 is about.

The question that protects the data is a different one, and it *is* decidable: "does this
destination name the same entry as this source?", asked at the moment of the access. It now lives
in `operation::zielpfad`, in two guards over `(st_dev, st_ino)`:

- `benennen_denselben_eintrag(&ziel, quelle.pfad)` replaces `ziel == quelle.pfad`. It asks
  **without** following the last path component (`lstat(2)`), because overwriting would remove the
  name and not what the name points at.
- `liegt_im_ordner(zielordner, quelle.pfad)` replaces `ziel.starts_with(quelle.pfad)`. It asks
  **with** following (`stat(2)`), because a path runs through its symlinks; an `lstat` on a
  symlinked ancestor would see the link instead of the folder it reaches. The two different
  follow behaviours are the substance of that pair and are written out at both functions.

Cost: two `lstat(2)` per entry, plus one `stat(2)` per level of the target path for a folder
source. Per entry of a running operation, not per pointer movement.

`abwurf_pruefen`'s textual comparison **stays**, and its doc comment now says plainly that it is a
prediction, what slips through it, and where the question is decided instead. What a slip costs
the user is now a pointer that accepts, followed by one "Quelle und Ziel sind derselbe Eintrag"
line per entry in the completion list — the same way C6 already answers its fourth Lage.

### Two unit tests moved out of `src/`

`operation::tests::ein_ordner_kann_nicht_in_sich_selbst_kopiert_werden` and
`eine_quelle_kann_nicht_auf_sich_selbst_kopiert_werden` handed `zielpfad` invented paths
(`/tmp/krk-ordner`) that existed on no volume. That worked while the guards compared text; a
filesystem question needs a folder that exists. `CLAUDE.md` allows exactly one self-clearing test
folder per crate, and the core's is `tests/gemeinsam/mod.rs`, reachable only from `tests/`. Both
were therefore replaced by integration tests covering the same two guards in **both** spellings:

| Guard | same spelling | second spelling |
|---|---|---|
| destination is the source | `eine_quelle_kann_nicht_auf_ihren_eigenen_ordner_kopiert_werden` (new) | `ein_ziel_das_ueber_einen_verweis_die_quelle_selbst_ist_wird_uebersprungen` (new) |
| destination lies in the source | `ein_ordner_laesst_sich_nicht_in_sich_selbst_kopieren` (existing) | `ein_ziel_das_ueber_einen_verweis_in_der_quelle_liegt_wird_uebersprungen` (new) |

A comment at the remaining `src/` test module says where they went and why.

## Defect 2334: measured first, then cached

The record marked its cost claim `speculation:` and both it and the review recommended measuring
before building anything. Measured, on the reference machine on 260819, `release` profile, per
call of `zwischenablage::dateiverweise` against an `NSPasteboard` carrying n file URLs:

| n | per call |
|---|---|
| 1 | 0.13 ms |
| 10 | 0.65 ms |
| 100 | 6.0 ms |
| 1 000 | 155 ms |
| 5 000 | 585 ms |
| 20 000 | 1.73 s |

Linear, and it runs on the main thread on **every** pointer movement. A frame at 60 Hz is 16.7 ms:
from a hundred dragged entries on, this one call eats more than a third of it; from a thousand on
the application stands still. That is the round's own stand-in for an eleventh time promise
("die Liste bleibt bildlauffähig"), so the cost does warrant the state. The measurement harness was
temporary and is not in the tree; the numbers are recorded at both doc comments that need them.

### The fifth ivar and its clearing rule

`QuelleIvars::abwurfquellen: RefCell<Option<Abwurfquellen>>` holds three things: the
`draggingSequenceNumber` of the drag session, whether the pasteboard carries any file reference,
and the one folder all dragged entries live in, if they all live in one. The target folder changes
with every row and is deliberately **not** in it.

The rule, stated as the record asked:

- **Set** only in `DateifensterQuelle::abwurfquellen`, and only when the remembered sequence number
  is not the running drag's.
- **Read** only there, and only when the numbers match.
- **Never cleared.** It is replaced, not emptied.

The three cases the task named:

- **The drag leaves the list and returns.** Same number, same pasteboard, the entry is still
  correct. Clearing here would pay the measured time again on every re-entry.
- **The drag is cancelled.** No further call arrives. The entry holds paths and a boolean, no
  handle to anything that can expire.
- **A second drag with no release in between.** It carries a different number and the first
  `validateDrop:` replaces the entry. That is what the key is for.

It is therefore the shape of `beschlossener_vorgang` and deliberately not that of
`gemeldeter_abwurfgrund`: it does not fall with the status line, because a keystroke during a
standing drag has nothing to do with that drag's pasteboard. No third rule was invented.

What it does not carry is safety against a reused sequence number; that is written out as
`speculation:` at the field, with the reason it is not worth guarding against.

`gemeinsamer_quellordner` is the derivation, a pure free function in `tabelle.rs` beside
`abwurfmeldung`, with a test covering its three `None` cases separately — empty pasteboard, entries
from two folders, and an entry with no parent at all. The last is the one an `unwrap` would have
swallowed.

## Defect 2336: a vanished row now refuses the drop

`abwurf_annehmen` collapsed two different things into one `None`: the `-1` that means "the whole
list", and a row number whose entry has disappeared. The first is a legitimate C4 target, the
second is a row that is no longer what the pointer showed. It also discarded the `Typ`, so a row
that had become a file went through as a destination folder.

The split is now `abwurfziel(benennt_eine_zeile, typ_der_zeile)`, a pure free function in
`tabelle.rs` with a written-out table over all eight combinations and no catch-all, so a fourth
`Typ` stops the build. `Abwurfziel::Keines` returns `false` to AppKit: the entries fly back,
nothing is written, and the user repeats a gesture that took a second. That is the one outcome the
old fallback could not offer, because writing into the parent folder cannot be undone by repeating
the gesture.

**Why this is not `abwurfregel::marke`**, written out at the function: the two tables differ at the
file row, and that difference is the whole point. During the drag a file row means "the list", and
the displayed folder is an announced target the user can see before releasing. At release a row
number arrives only because `abwurf_pruefen` set it, and it set one only for a folder; a file there
means the list changed under the pointer, and the displayed folder was never announced. Calling
`marke` would also have broken `die_marke_hat_genau_einen_aufrufer`, which promises that a row
number becomes a target at exactly one place **during the drag**.

## Tests, and the proof that each one fires

Five tests were added. Each was watched go red with the behaviour deliberately broken, then
restored:

| Test | broken by | went red with |
|---|---|---|
| `ein_ziel_das_ueber_einen_verweis_die_quelle_selbst_ist_wird_uebersprungen` | the unfixed tree | `die Quelle ist weg: NotFound` |
| `ein_ziel_das_ueber_einen_verweis_in_der_quelle_liegt_wird_uebersprungen` | the unfixed tree | 139 entries copied |
| `eine_quelle_kann_nicht_auf_ihren_eigenen_ordner_kopiert_werden` | guard 1 forced to `false` | 0 entries expected, 1 copied |
| `die_tafel_des_abwurfziels_geht_auf` | `(true, None)` back to `AngezeigterOrdner` | `benennt_eine_zeile=true, typ=None` |
| `der_gemeinsame_quellordner_entsteht_nur_aus_einem_ordner` | `parent()?` back to a default | `Some("/")` for the root |

**What is not tested, and what holds it instead.** The cache's clearing rule and the refused drop
both need a live `DateifensterQuelle` and an `NSDraggingInfo`, and `libtest` gives no main thread.
What holds them is construction: `abwurfquellen` is the only reader and writer of its field and
compares the key on every path through it, and `abwurf_annehmen` has no arm that reaches a target
without `abwurfziel` having named one. The pure halves of both — the table and the derivation —
are the parts a test can hold, and they are held. The visible behaviour is user acceptance work,
like the rest of C4 to C7.

## Two corrections outside this agent's files, reported and not made

1. **The spec's cost enumeration** (`shared/planning/260818-1510_*`, `## Verhältnis zu den zehn
   Zeitzusagen`) names "ein Vergleich zweier Pfade und eine Frage nach dem Schreibrecht des
   Zielordners" — two O(1) terms. The third term, the pasteboard decode, is now O(1) per pointer
   movement as well, but the sentence should say so rather than stay silent about it.
2. **C6's third acceptance criterion** promises the pointer refuses a drag out of a Finder window
   showing the drop target's folder. As written it will be reported as failing for two spellings of
   one folder. It should say "under the same spelling", with the note that a slip now ends in the
   completion list rather than in a deletion.

Neither file was edited; both are the user's to decide at a gate.

## Left alone deliberately

Three further open records of this Circle were not touched: `260818-1704` (the plan's test-count
claim), `260818-2221` (the drop passes its target as the source folder and the completion reads it
twice), `260818-2228` (step 9 calls the new caller the third).

Not committed, and none of the three issue records was closed — both are the user's.
