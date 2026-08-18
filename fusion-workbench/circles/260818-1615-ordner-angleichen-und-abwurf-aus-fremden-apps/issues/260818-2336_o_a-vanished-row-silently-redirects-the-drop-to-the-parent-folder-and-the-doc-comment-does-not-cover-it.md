# A vanished row silently redirects the drop to the parent folder, and the doc comment's case split does not cover it

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `crates/krk-ui/src/appkit/tabelle.rs:3057-3065` (`abwurf_annehmen`, the target), `:3037-3050` (the doc comment that reasons about the race), `:2965-3026` (`abwurf_pruefen`, which only ever marks a folder row)

---

## What is wrong

`abwurf_annehmen` reads the target back from the row AppKit hands it:

```rust
// tabelle.rs:3059-3065
let ziel = match usize::try_from(zeile)
    .ok()
    .and_then(|zeile| self.eintrag_in_zeile(zeile))
{
    Some((pfad, _)) => pfad,
    None => self.angezeigter_ordner(),
};
```

The doc comment above it reasons about the race between the last pointer position and the release, and names one outcome:

> Frischt die Liste in dieser Spanne auf und steht an der Zeile ein anderer Eintrag, geht dessen Pfad als Ziel mit; die Operationsmaschine hängt jeden Namen daran an und meldet den gescheiterten Eintrag mit seinem Grund in der Abschlussliste.

That covers `Some((pfad, _))` with a different entry. It does not cover `None` with a non-negative row, which is what a refresh that **shortens** the list produces. In that branch the drop silently retargets from the marked sub-folder to the displayed folder — its parent, in the common case. That is not a reported skip, it is a successful operation into a folder the user did not point at, and nothing appears in the completion list because nothing failed.

The case split is therefore neither complete nor uniform in outcome class: two of its branches end in "reported skip" and one ends in "silently did something else". `critical-stance.md` §4 asks a case split to be disjoint and complete; this one has a third branch whose existence the prose does not acknowledge.

## Second, smaller point in the same three lines

The `Some((pfad, _))` arm discards the `Typ`. `abwurf_pruefen` sets a row as the drop target only for `Some(Typ::Ordner)` (`abwurfregel::marke`), so any row reaching `abwurf_annehmen` was a folder when it was judged. After a refresh it may be a file, and the drop then treats a regular file as a destination folder. That one **is** covered by the doc comment's reported-skip reasoning, and it is cheap to make impossible rather than merely reported.

## Suggested direction, not a prescription

Both points close with the same two-line change: match on `Some((pfad, Typ::Ordner))` and let every other shape — a different type, a vanished row — fall to `return false`. Returning `false` is the honest answer for a drop whose target no longer exists: AppKit flies the entries back, nothing is written anywhere, and the user repeats a gesture that took a second. Falling back to the parent folder is the one outcome that cannot be undone by repeating the gesture.

The doc comment then needs its third branch named, so that the next reader does not restore the fallback as a convenience.
