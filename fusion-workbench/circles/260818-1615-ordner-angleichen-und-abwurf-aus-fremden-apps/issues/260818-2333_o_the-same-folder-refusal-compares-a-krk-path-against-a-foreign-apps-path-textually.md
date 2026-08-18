# The same-folder refusal compares a KRK path against a foreign application's path textually, and two spellings of one folder defeat it

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `crates/krk-ui/src/appkit/tabelle.rs:2992-2995` (`ziel_ist_quellordner`), `crates/krk-ui/src/appkit/zwischenablage.rs:271-289` (`dateiverweise`, the foreign side of the comparison), `crates/krk-ui/src/appkit/anwendung.rs:3378-3392` (the same question asked about two KRK paths, with its reasoning), spec `shared/planning/260818-1510_*_spec-…` §C6 Lage 3 and its third acceptance criterion

---

## What is wrong

C6 Lage 3 promises that a drag whose source folder is the drop target is refused while the mouse is still down. The implementation decides it with a textual path comparison:

```rust
// tabelle.rs:2992-2995
ziel_ist_quellordner: !quellen.is_empty()
    && quellen
        .iter()
        .all(|quelle| quelle.parent() == Some(ziel.as_path())),
```

`ziel` comes from KRK's own folder model. `quellen` comes from `zwischenablage::dateiverweise`, that is from `NSURL::path` on URLs written by whatever application started the drag. **The two sides no longer come from the same source**, and nothing normalises either of them. One folder under two spellings — `/tmp` against `/private/tmp`, a bookmark that reaches a folder through a symbolic link, a difference in upper and lower case on the case-insensitive volume this machine runs — reads as two folders, the refusal does not fire, and the pointer accepts a drop that C6 says it must refuse.

## Why the reasoning that covers the other comparison does not cover this one

`ordner_angleichen` compares two paths without `canonicalize` as well, and its doc comment (`anwendung.rs:3378-3392`) argues the case explicitly and correctly: both paths come from KRK, the comparison can only err in one direction, and the consequence of erring is one redundant directory read. Neither half of that argument survives here. One side is foreign, and the consequence is not a redundant read but the loss of a refusal the spec put there deliberately. `abwurf_pruefen`'s own doc comment discusses this field only under the question "all sources or some" and says nothing about spelling.

## What the consequence actually is — measured, not assumed

Filing this at Medium rather than Critical, because the downstream outcome was measured on this machine on 260818 and it is not data loss:

- `copyfile(3)` with `COPYFILE_ALL`, source and destination naming the same file through a symlinked directory, returns 0 and leaves the file intact (10 bytes before, 10 bytes after).
- The move path goes through `rename(2)`, which is a documented no-op when both arguments name the same file.

What the user gets instead is the conflict query firing once per dragged entry for a drop that should never have been offered, and an "operation" that copies nothing. `abwurf_ausfuehren` deliberately does not re-ask any of the C6 questions (`anwendung.rs`, its doc comment), so nothing downstream closes the hole.

## Suggested direction, not a prescription

Three options, in the order I would weigh them:

1. Compare the two folders by identity rather than by spelling — `std::fs::metadata` on both and a `(st_dev, st_ino)` comparison. It answers the question that was actually asked, it costs one `stat` on the target plus one per distinct source parent, and it is the same change of mechanism `critical-stance.md` §4 asks for when a comparison cannot be made reliable in its own terms. `krk-core` already owns the syscall layer this would live near (`verzeichnis/sys.rs`).
2. `canonicalize` both sides. Cheaper to write, but it resolves symlinks, which this project has repeatedly decided not to do, and it needs an error branch on every pointer movement.
3. Accept the hole and say so in the doc comment and under C6. Honest, and it costs nothing — but then the C6 acceptance criterion should be narrowed to say "the same folder under the same spelling", because as written it will be reported as failing.

Whichever is chosen, note that option 1 also touches the per-pointer-movement cost, which is the subject of a separate record filed in the same pass.
