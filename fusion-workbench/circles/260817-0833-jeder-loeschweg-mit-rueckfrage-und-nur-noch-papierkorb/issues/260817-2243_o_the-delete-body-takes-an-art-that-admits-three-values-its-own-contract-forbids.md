The delete body takes an `Art` that admits three values its own contract forbids

---
`Anwendungsdelegierter::loeschen_nach_rueckfrage` keeps its parameter `art: Art` after the second
delete command fell. `Art` has four variants; exactly one of them is legal here. Passing any of
the other three would show the delete confirmation and, on confirmation, start a copy, a move or
a batch rename. Nothing in the type, in a `debug_assert!` or in a probe holds the restriction.

---

**Severity:** Low. One caller exists and it passes the right value, the function is private, and
the build is green. It is filed because the safeguard that used to cover the neighbouring
parameter was deliberate and is now gone: the enum `Loeschtexte` existed only so that the
compiler would stop the build when the second delete command fell, and it did its job and fell
with it. The remaining `art` parameter has no such holder and never had one.
**Found by:** coderev, review `reviews/260817-2243-coderev-bundle-d-the-removal.md`
**Affected:** `crates/krk-ui/src/appkit/anwendung.rs:4620`, its caller at `:4457-4459`
**Tree state:** `f7a85c1`
**Domain:** code

## Measured

Both parameters now carry one value each, from one caller:

```
$ grep -n "loeschen_nach_rueckfrage" crates/krk-ui/src/appkit/anwendung.rs
4444:    /// Der Rumpf ist [`Self::loeschen_nach_rueckfrage`]. Hier stehen allein die
4458:        self.loeschen_nach_rueckfrage(Art::InDenPapierkorb, "In den Papierkorb räumen")
4620:    fn loeschen_nach_rueckfrage(&self, art: Art, schaltflaeche: &str) -> bool {
```

**No dead distinction is left behind.** Neither parameter drives a branch. `art` travels into the
`Cell` at `:4682`, out of it in the sheet's callback, and into `loeschauftrag_stellen`, which
puts it into the `Auftrag` without reading it (`:4849-4856`). `schaltflaeche` goes straight to
`loeschbestaetigung::zeigen`. `Self::loeschtexte` lost its `textform` parameter and its `match`
in the same commit, and `cargo clippy --workspace --all-targets -- -D warnings` is clean, so no
unreachable branch survives.

What is left is only that the signature is wider than the contract. The doc comment above the
function is titled "Der eine Rumpf jedes Loeschbefehls" and its five-stage diagram describes a
delete throughout; `Art::Kopieren`, `Art::Verschieben` and `Art::UmbenennenImStapel` all
type-check there.

## Direction

Two ways, and the choice is a design call rather than a defect fix:

1. Drop both parameters and let the body name `Art::InDenPapierkorb` and the button caption
   itself. Smallest, and it removes the question. It costs the seam that a later round would use
   if a second confirmed operation ever wants this body.
2. Keep the signature and add a `debug_assert!(matches!(art, Art::InDenPapierkorb))` with the
   reason, so the restriction is stated where the compiler cannot state it.

Not a candidate: merging `loeschen_nach_rueckfrage` into `in_den_papierkorb`. The split is what
keeps the five-stage rule readable next to the two pieces the command contributes, and the
executor's note says the separation was deliberate.
