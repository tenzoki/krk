Step 9 calls `abwurf_ausfuehren` the third caller of `auftrag_starten`, while the plan's own Current State counts three existing ones

---

The plan
(`circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_o_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`)
says both of these:

- Current State: "`auftrag_starten` (`:5368`) nimmt einen fertigen `Auftrag` und hat **heute drei
  Rufer** (`:4913`, `:5171`, `:5331`)".
- Approach point 5 and step 9: the drop enters "als **dritter Rufer von `auftrag_starten`**", and
  step 9 is headed "die Maschine bekommt ihren dritten Eingang".

Three existing plus one new is four. Counted against the tree at `07347b8`, the three that exist
are `loeschauftrag_stellen` (the confirmed delete of round 12), `stapel_beauftragen` and
`auftrag_stellen`; `abwurf_ausfuehren` is the fourth.

---

**Severity:** Low. Nothing in the code depends on the count, and the implementation is unaffected
— step 9 built the body the plan describes. What the wording costs is a doc comment written from
the plan instead of from the tree: the first draft of `abwurf_ausfuehren` said "der dritte Rufer",
and the tree says four. It was corrected before the step was reported.

**Where the count is now right:** the doc comment of `abwurf_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs`) names the fourth caller and the three that precede it,
and says outright that the plan calls it the third.

**Found by:** coder, implementing step 9 and counting `self.auftrag_starten(` against the tree.
**Affects:** the plan document only.
**Related:** `crates/krk-ui/src/appkit/anwendung.rs`, doc comments of `auftrag_starten` and
`abwurf_ausfuehren`.
**Tree state:** `07347b8` plus the working tree of step 9.
**Domain:** code

## What a fix would have to do

Correct the three places in the plan that say "dritter" for this caller, or state that the count
means entry points that build an `Auftrag` from user input and say which three it counts. The
same round already carries a defect about counts that no compiler holds
(`issues/260818-1704_o_der-plan-sagt-die-proben-blieben-nach-schritt-1-gruen-sie-fallen-zu-51.md`);
this is one more of that kind, in the plan rather than in the tree.

**Filed by:** coder
