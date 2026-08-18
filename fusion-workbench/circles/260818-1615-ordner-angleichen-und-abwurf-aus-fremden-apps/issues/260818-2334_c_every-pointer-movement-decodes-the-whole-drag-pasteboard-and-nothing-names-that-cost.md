# Every pointer movement decodes the whole drag pasteboard, and neither spec nor plan names that cost

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `crates/krk-ui/src/appkit/tabelle.rs:2987` (the per-movement call), `:2992-2995` (the per-movement O(n) scan), `crates/krk-ui/src/appkit/zwischenablage.rs:271-289` (`dateiverweise`), spec `shared/planning/260818-1510_*_spec-…` `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1` (the enumeration of what the round newly puts on the main thread, and its second criterion)

---

## What is wrong

`abwurf_pruefen` runs on every `validateDrop:`, that is on every pointer movement over the list. Two of the six facts it gathers scale with the number of entries the user is dragging:

```rust
// tabelle.rs:2987
let quellen = super::zwischenablage::dateiverweise(&zug.draggingPasteboard());
```

`dateiverweise` calls `readObjectsForClasses:options:`, which materialises one `NSURL` per pasteboard item, and then builds one `PathBuf` per URL. For a drag of n entries that is n Objective-C objects plus n heap allocations, per pointer movement. Immediately after, the same n paths are walked again:

```rust
// tabelle.rs:2992-2995
ziel_ist_quellordner: !quellen.is_empty()
    && quellen.iter().all(|quelle| quelle.parent() == Some(ziel.as_path())),
```

The whole result is used for exactly one bit — `traegt_dateien: !quellen.is_empty()` — plus the folder comparison. Nothing is kept between movements.

## Why this is a finding rather than an observation

The spec enumerated what the round newly puts on the main thread and named two items:

> Was diese Runde daneben neu auf den Hauptfaden legt, ist die Prüfung während des Ziehens: ein Vergleich zweier Pfade und eine Frage nach dem Schreibrecht des Zielordners, je Zeigerbewegung höchstens einmal.

Both named items are O(1). The third one, the only one that scales, is not in that sentence, and the plan does not add it. Whoever reads the round's own account of its cost will not find it.

The round's stand-in for an eleventh time promise is exactly the criterion this threatens:

> Während ein Ziehvorgang über der Dateiliste steht, bleibt die Liste bildlauffähig und die Anwendung antwortet auf Tastendrücke, die nicht zum Ziehen gehören.

That criterion is user acceptance work, and whether it survives a drag of several thousand entries out of a Finder window is unmeasured. This project has an explicit stance on that shape of problem: the sort key is built once at read time and must not fall back into a pairwise comparison, because L3 and L10 depend on it (`CLAUDE.md`).

## What is not claimed

I have not measured it. `speculation:` — for a handful of entries the cost is certainly irrelevant, and the failure, if it exists, needs a large multi-selection drag. The record is filed for the missing statement as much as for the cost: the round accounted for its per-movement work and left out the one term that is not constant.

## Suggested direction, not a prescription

The pasteboard content does not change during a drag session. `NSDraggingInfo::draggingSequenceNumber` (`NSDragging.h:81`, no availability annotation, so 10.0) identifies the session, so the decoded `Vec<PathBuf>` and the derived facts can be cached in an ivar keyed on that number and rebuilt only when it changes. That is a fourth ivar with a clearing rule, so it needs the same care the two existing ones got — and the honest alternative is to measure first and add nothing if the cost does not show. Either way the spec's enumeration should be corrected to name the third term.

---
Resolved: Gemessen und dann zwischengespeichert. release-Bau, je Aufruf von dateiverweise: 1 Eintrag 0,13 ms, 100 Eintraege 6,0 ms, 1000 Eintraege 155 ms, 5000 Eintraege 585 ms; ein Bild bei 60 Hz hat 16,7 ms. Ab hundert gezogenen Eintraegen frisst dieser eine Aufruf ein Drittel davon, ab tausend steht die Anwendung. Der fuenfte Ivar haelt draggingSequenceNumber, ob die Ablage Dateien traegt, und den gemeinsamen Ordner. Er wird ersetzt und nie geraeumt, gelesen und geschrieben allein in abwurfquellen und nur bei passendem Schluessel: verlaesst das Ziehen die Liste und kehrt zurueck, ist die Nummer dieselbe und der Eintrag gueltig; ein zweites Ziehen ohne Loslassen traegt eine andere Nummer und ersetzt ihn beim ersten validateDrop:. Das ist die Bauform von beschlossener_vorgang, keine dritte Regel.
