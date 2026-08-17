A probe fixture still labels the executing button "Endgültig löschen"

---
`die_ausfuehrende_stelle_zeigt_auf_die_ausfuehrende_schaltflaeche`
(`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs:170`-`:180`) builds its buttons
with the label `"Endgültig löschen"` and asserts the same string back. No such wording
reaches the screen any more: the one caller passes `"In den Papierkorb räumen"`
(`crate::appkit::anwendung::in_den_papierkorb`), and the sister probe two blocks up already
uses that wording.

---

**Severity:** Low. The probe is correct and passes — the label is a pass-through argument,
so any string proves what the probe claims. The cost is that a reader of this file meets a
button title KRK no longer has, in the very file whose head explains why the wording of the
delete path matters.

**Found by:** coder, step 15 of `planning/260817-0856_o_plan-absicherung-jedes-loeschwegs.md`
**Affected:** `crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs:172`, `:179`
**Tree state:** working tree after step 15, on `8f556ed`
**Domain:** code

## Why step 15 did not fix it

Step 15 is scoped to prose and explicitly excludes probes. These two lines are the probe's
body, not its comment.

## Direction

Replace both occurrences with `"In den Papierkorb räumen"`, the wording the one caller
actually passes. One edit, no assertion changes meaning.
