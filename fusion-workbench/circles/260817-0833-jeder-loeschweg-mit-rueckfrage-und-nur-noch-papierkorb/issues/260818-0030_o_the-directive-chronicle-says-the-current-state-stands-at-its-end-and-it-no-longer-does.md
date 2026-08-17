The Directive chronicle says the current state stands at its end, and it no longer does

---
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md:543`
opens the section `## Abgleich mit der Circle-Directive` with "Der aktuelle Stand steht am Ende."
Step 17 corrected the Directive of that Circle a fourth time, on 260818-0006, and deliberately
left the chronicle verbatim. Its last entry is dated 260802-1735 and ends "Zwischen diesem Spec
und dem Circle-Datensatz ist derzeit keine Abweichung bekannt."

---

**Severity:** Low. The correction itself is recorded in three other places, so nothing is lost —
only the one sentence that tells a reader where to look is now wrong.

**Found by:** coderev, review `reviews/260818-0024-coderev-bundle-e-the-prose-and-the-records.md`
**Affected:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md:543`
**Tree state:** `da716c1`
**Domain:** code

## Why this is not simply the recording rule at work

The plan is explicit — step 17: "Die Chronik am Dokumentende bleibt im Wortlaut stehen" — and the
recording rule in `CLAUDE.md` backs it: a recorded state keeps its wording, and the exemption is
decided per file by its location. Every dated entry of the chronicle is exactly such a record and
must stay.

Line 543 is not one of them. It carries no date, describes the section rather than a state, and
makes a promise about where the reader finds the newest entry. The fourth correction is recorded
at three places and none of them is that end:

- `circles/260802-0842-krk-mac-dateimanager-editor-git/_b_circle.md:18`, the Nachtrag under the
  Directive, which itself says "Es ist die vierte Korrektur dieses Abschnitts; die drei vorigen
  vom 260802-1127, 260802-1423 und 260802-1445 führt der Abschnitt
  `## Abgleich mit der Circle-Directive` des Specs" — pointing the reader at a chronicle that
  stops after the third,
- the same spec at `:12`, in the gate note at the head,
- `shared/decisions/260817-0536_i_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`.

The three dates the Nachtrag names are correct; the chronicle records corrections at 260802-1127
(`:545`), 260802-1423 (`:551-553`) and 260802-1445 (`:555`), with a fourth entry at 260802-1735
(`:557`) that concerns the Grounding snapshot and not the Directive.

## Direction

Two ways, both cheap, and the choice is not obvious enough to make here:

1. Replace `:543` with a sentence that says where the current state actually is — the Nachtrag in
   the Circle record and the gate note at `:12` — and leave the chronicle closed at 260802-1735.
   Keeps the plan's instruction intact.
2. Append a dated fifth entry to the chronicle, in the form the four existing ones use, and leave
   `:543` alone. This adds to a record rather than changing one, which is the pattern the rest of
   step 17 followed everywhere else, but it goes past what the plan said.

Option 2 is the one the section's own form suggests; option 1 is the one that stays inside the
plan. A user decision, not a reviewer's.
