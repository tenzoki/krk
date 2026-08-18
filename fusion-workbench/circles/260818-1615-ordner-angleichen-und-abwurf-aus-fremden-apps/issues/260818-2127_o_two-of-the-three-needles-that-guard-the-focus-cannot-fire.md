Two of the three needles in `das_angleichen_ruehrt_weder_fokus_noch_sichtbarkeit_an` cannot fire

---

The test at `crates/krk-ui/src/appkit/anwendung.rs:7568-7588` asserts that the body of
`ordner_angleichen` contains none of three strings, and its name and doc comment claim it
guards two C1/C2 promises: that the same file pane is active afterwards, and that no area is
hidden. The hiding half is guarded. **The focus half is not.**

- Needle `concat!("aktiv_", "setzen(")` yields `"aktiv_setzen("`. The delegate's own setter is
  `aktives_setzen` (`anwendung.rs:4115`), and `"aktiv_setzen("` is **not** a substring of
  `"aktives_setzen("`. A regression written the way this file writes it everywhere else —
  `self.aktives_setzen(ziel)`, the form at `anwendung.rs:1160` — passes the test. Only a direct
  `self.ivars().modell.borrow_mut().aktiv_setzen(…)` is caught, and that is the form nobody
  would write here, because `aktives_setzen` exists precisely to carry the follow-up work.
- Neither `fokus_setzen(` (`anwendung.rs:2157`) nor `fokus_holen(` (`anwendung.rs:2007`) is a
  needle at all. Those two are how the focus changes in this file; the test that says
  "ruehrt weder Fokus noch Sichtbarkeit an" does not look at either.

The two needles that do work are `concat!("bereich_um", "schalten(")`, which matches
`bereich_umschalten` (`:3863`), and `concat!("aus", "blenden(")`, which matches
`editor_ausblenden` (`:6445`) and any `.ausblenden(`. Those cover C2's "blendet in keiner Lage
einen Bereich aus".

---

**Severity:** Medium. Nothing is broken today — the body is correct, and `make check` is green
for the right reason. The cost is that the guard against the C1 focus promise reads as present
and is absent, which is the failure mode this tree files defects about: a green test standing
in for a measurement nobody made.
**Found by:** coderev, reading the needles against the function names in the same file. The
substring relation was checked directly, not inferred.
**Affects:** `crates/krk-ui/src/appkit/anwendung.rs:7568-7588`
(`angleichproben::das_angleichen_ruehrt_weder_fokus_noch_sichtbarkeit_an`)
**Related:** the coder's own session record repeats the three needles verbatim
(`history/260818-2103-coder-der-befehl-wirkt.md`, section "Die drei Proben, und was sie nicht
sehen"), so the record carries the same gap.
**Tree state:** `71413c3`
**Domain:** code

## What a fix would have to do

Replace `aktiv_setzen(` with `aktives_setzen(` and add `fokus_setzen(` and `fokus_holen(`, or
drop the focus claim from the test's name and doc comment. Both are defensible; what is not
defensible is a name that promises more than the needle set measures.

A stronger form is available and worth weighing: `zettelproben` already counts call sites over
the whole source tree (`aufrufstellen`, `quelldateien`). A needle set inside one function body
is the weakest of the three shapes this file uses, because the needle set *is* the whole test.

**Filed by:** coderev
