# Code review — round 13, turn 1: `opt+cmd+s` puts the other file pane on this folder

**Date:** 260818-2133
**Sender:** coderev
**Reviewed-range:** `8d5baf6..71413c3`
**Not-opened:** `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/_t_circle.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/decisions/260818-1633_a_gilt-ein-unentscheidbares-schreibrecht-beim-abwurf-als-erlaubnis-oder-als-abweisung.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-1615-shaper-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-1633-planner-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-1740-coder-das-kommando-in-die-vier-pflichtstellen.md`, `fusion-workbench/circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/history/260818-2112-coder-die-acht-prosazahlen-nachziehen.md`, `fusion-workbench/orchestrator-events.jsonl`, `fusion-workbench/shared/decisions/260818-1453_a_welche-zusatztaste-macht-aus-einem-abwurf-ein-verschieben.md`, `fusion-workbench/shared/history/260818-1117-orchestrator-session.md`, `fusion-workbench/shared/history/260818-1453-shaper-runde-13-gleiches-verzeichnis-und-abwurf.md`, `fusion-workbench/shared/history/260818-1510-shaper-verzeichnis-angleichen-und-abwurf.md`
**Scope note:** all eight files carrying code or data in the range were opened in full. The spec
and the plan were read in the parts that bind this turn — C1 to C3 of the spec, steps 1 to 5 of
the plan, and the sections on what the compiler holds. The eleven files above are workbench prose
outside the dispatched scope.
**Verification run:** `cargo build --workspace`, `cargo test --workspace`, `cargo clippy
--workspace --all-targets`, `cargo fmt --all --check` — all green at `71413c3`. The three new
`angleichproben` and `das_ordnerangleichen_steht_unter_dateilisting` run and pass. The built
binary's `--menue-protokoll` was read to check where the menu entry lands.

## Summary

The turn builds C1 to C3 correctly. The `RefCell` fix is real, the three meanings of `false` are
separated the way C2 needs them, the nine prose counts all check out against the tree, and
`opt+cmd+s` was genuinely free. Six findings, none critical: one is a spec contradiction that the
built behaviour inherits, one is a test guard weaker than its name, and four are prose and record
defects.

## Totals

| Severity | Count |
|---|---|
| Critical | 0 |
| High | 0 |
| Medium | 2 |
| Low | 4 |

All six are filed as separate records under this Circle's `issues/`.

## Findings by theme

### Guards that do not measure what they are named for

**M1 — two of the three needles that guard the focus cannot fire.**
`260818-2127_o_two-of-the-three-needles-that-guard-the-focus-cannot-fire.md`

`das_angleichen_ruehrt_weder_fokus_noch_sichtbarkeit_an` (`anwendung.rs:7568-7588`) asserts the
absence of three strings in the body of `ordner_angleichen`. The needle
`concat!("aktiv_", "setzen(")` is `"aktiv_setzen("`, and the delegate's own setter is
`aktives_setzen` (`anwendung.rs:4115`) — the two strings are not in a substring relation, so the
form a regression would actually take, `self.aktives_setzen(ziel)` as at `:1160`, passes. Neither
`fokus_setzen(` (`:2157`) nor `fokus_holen(` (`:2007`) is a needle at all. The hiding half of the
claim is guarded (`bereich_umschalten(` at `:3863`, `ausblenden(` reaching `editor_ausblenden` at
`:6445`); the focus half is not. Scope: `krk-ui` only.

**On the pattern itself, since it was asked.** Reading the source text is sound *here* and should
stay, with one of the three reconsidered:

- `der_befehl_steht_vor_dem_auffangzweig` is the strongest of the three and earns its place. It
  measures the one mandatory site per command that neither compiler nor any other probe holds —
  the site `shared/issues/260818-1635_o_…` was filed about — and the coder counter-checked it by
  deleting the branch and watching it go red. It rots loudly: both `find`s carry `.expect`.
- `die_sichtbarkeit_wird_vor_dem_einblenden_gefragt` measures textual order, not evaluation
  order. In `if !sichtbar && !self.bereich_einblenden(bereich)` the two coincide, and no plausible
  rewrite makes it green while the order is wrong. Acceptable.
- The third is where the shape breaks down, and M1 is the symptom rather than the disease. A
  negative assertion over a body has no failure mode other than an incomplete needle set, and the
  needle set is the whole test. `zettelproben` already carries the stronger shape next door —
  `aufrufstellen` and `quelldateien` count call sites over the whole tree — and that shape is
  available for the focus claim.

Both helper widenings to `pub(super)` are the right call over a second copy of `diese_datei` and
`rumpf`, and both carry the reason at their doc comment. `rumpf` was checked against the new
bodies: the `"\n    }\n"` terminator cannot trip on the nested `if` blocks (those close at eight
spaces), and neither `"fn ordner_angleichen("` nor `"fn kommando_ausfuehren("` occurs twice in the
file, so neither `find` can latch onto the wrong body.

### The binding record contradicts itself

**M2 — C1 and C2 contradict for a hidden target pane already on the folder.**
`260818-2128_o_c1-and-c2-contradict-for-a-hidden-pane-already-on-the-folder.md`

C1's fifth criterion ("geschieht nichts, und die Statuszeile sagt es") and C2's first ("blendet
der Befehl es ein und stellt es auf den Ordner") both cover the case where the target pane is
hidden *and* already holds the folder, and they demand different outcomes. The spec's flowchart
resolves it silently by ordering the equality question first; `ordner_angleichen`
(`anwendung.rs:3329-3332`) follows the flowchart and its doc comment names the consequence as
deliberate. Built behaviour: the pane stays hidden and the user is told it "zeigt diesen Ordner
bereits" — about a pane he cannot see. C2's first criterion is unmet in that lage, and the
acceptance run is user work, so it will surface there unless settled first.

### Doc comments that state a reason the tree contradicts

**L1 — the redundant read the path comparison allows is not "folgenlos".**
`260818-2129_o_the-redundant-read-the-path-comparison-allows-is-not-without-consequence.md`

The `canonicalize` reasoning at `anwendung.rs:3305-3310` is right in its load-bearing direction:
two different folders can never share one `PathBuf`, so no false "already there" is reachable and
the comparison can only err towards reading. The closing claim that the read is without
consequence does not hold. `ordner_lesen` goes through `Tabliste::ordner_setzen`, which replaces
the tab (`self.tabs[stelle] = Tabinhalt::aus_zustand(&zustand)`) and carries sortierung,
`verstecke_ausgeblendet`, `tief`, `inhalt` and the filter text across by hand but not the
selection or the scroll position. The tree knows this — it is the stated reason
`Tabliste::aktiven_neu_lesen` exists. So in the slipped-through case (`/tmp` against
`/private/tmp`, a symlinked bookmark, a case difference on the case-insensitive default volume)
the command does what C1's fifth criterion promises it will not.

**L2 — the doc comment gives the wrong meaning for the return value.**
`260818-2130_o_the-doc-comment-gives-the-wrong-meaning-for-the-return-value-of-ordner-angleichen.md`

`ordner_angleichen` closes with "der Befehl war zustaendig, auch wenn er nur etwas zu melden
hatte". Jurisdiction is not what the value carries: `kommando_ausfuehren:2889-2894` defines it as
"did this command do anything", and it drives `aufteilung_nachziehen()` and `sitzung_vormerken()`
at `:3089-3092`, while `kommando_ausfuehren` itself returns `true` unconditionally at `:3093`. The
two no-op branches return `true`, so every no-op press re-lays out and schedules a session write.
Both follow-ups are idempotent, so the cost is negligible — but the cited precedent
`ordner_der_datei_zeigen` (`:3247`) does exactly the same, which makes this a third instance of a
pre-existing divergence rather than a fresh one, and the new sentence gives it a reason the
contract two hundred lines above contradicts.

### Enumerations and citations in the shipped data file

**L3 — the keymap now carries two enumerations of the `opt+cmd` row, one short by two.**
`260818-2131_o_the-keymap-now-carries-two-enumerations-of-the-opt-cmd-row-and-the-older-one-is-short-by-two.md`

`resources/default-keymap.toml:266-272` names eight members of the row; the file binds eleven.
Missing are `opt+cmd+e` ("Editor schließen", `:790`) and `opt+cmd+n` ("Weitere Instanz starten",
`:1012`), neither of which fits the sentence's premise either. The new block at `:293-296` names
all ten pre-existing combinations and is correct. The shortfall predates this round; what this
round added is the second list fifteen lines below the first. `git:48bb57f` took exactly this
shape of defect out of the Norton block of the same file.

**L4 — the letter choice cites the third rule without recording that the first two were checked.**
`260818-2132_o_the-letter-choice-cites-the-third-rule-without-recording-that-the-first-two-were-checked.md`

The citation itself resolves and is accurate — the three rules are in
`circles/260802-0842-…/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`
and rule 3 is "der Anfangsbuchstabe des deutschen Verbs". But the rules are ordered ("Sonst…",
"Sonst…"), so rule 3 applies only once rules 1 and 2 fail, and putting the other panel on the
current folder is the Norton-lineage operation where rule 2 is most likely to bite. The same
block records the freeness check for the combination and no check for the rule that chose it.
Stated with its uncertainty: this reviewer did not verify Total Commander's binding and is
reporting an unrecorded precondition, not a wrong outcome. The spec justifies the same letter
differently ("s liest sich als „selber Ordner""), so the tree now holds two readings of one
choice.

## What was checked and found right

Recorded because a clean finding is worth as much as a defect, and three of these were the
leads that prompted the review.

**The `RefCell` fix is real, and no other borrow in the new body outlives an Objective-C call.**
`let sichtbar = self.ivars().modell.borrow().sichtbar(bereich);` drops its `Ref` at the end of the
`let` statement — `sichtbar` is a `bool`, so no temporary-lifetime extension applies — and the
borrow is therefore gone before `bereich_einblenden` reaches `borrow_mut()` through
`sichtbarkeit_aendern` (`:4001-4004`). The same holds for `let aktiv = …borrow().aktiv();` one
line up. Nothing else in the body borrows `modell`: `dateifenster()` reads a `OnceCell`
(`:2615`), and `antwort_zeigen` and `ordner_lesen` take their own borrows internally and release
them before their AppKit calls, per the rule in the head of `tabelle.rs:68-72`.

**The three meanings of `false` are separated correctly, and only the refusal speaks.**
`Fenstermodell::einblenden` (`fenstermodell.rs:733-739`) returns `false` for "already visible" and
for the minimum-width refusal it inherits from `umschalten`; `bereich_einblenden` adds a third for
`zeilenmass() == None`. Meaning 1 is excluded by the `!sichtbar` guard, meaning 3 cannot arise for
a keypress because the layout stands from `oberflaeche_aufbauen`, so only meaning 2 reaches the
message. The message goes to `aktiv`, the triggering pane, as C2 requires. The `#[must_use]` added
in step 3 is enforced under `-D warnings`, and the one caller that dropped the value bare
(`zwischenablage_ansehen:1516`) now carries `let _ =`.

**All nine prose counts hold against the tree.** Counted at `71413c3`: 85 `[[funktion]]` blocks,
90 combinations across the `tasten` lines, 79 `Kommando` variants and `KENNUNGEN` declared at 79,
6 entries with `gehalten_von = "menue"` — and 79 + 6 = 85. The 140 → 280 correction at
`menue.rs:1129` was not on the plan's list of eight and is right: the table is
7 × 5 × 2 × 2 × 2 = 280, and `die_tafel_aus_zweihundertachtzig_faellen_geht_auf`
(`kommandos/zulaessigkeit.rs:436`) asserts the count itself. No stale `84`, `78` or `89` is left
anywhere under `crates/` or `resources/`; the remaining hits are point measurements in
`belegungsansicht.rs` and SDK header line numbers.

**`opt+cmd+s` was free.** The `tasten` lines bind exactly eleven `opt+cmd` combinations at
`71413c3`: the ten the new comment names, plus the new one. `opt+cmd+f` appears only at `:423` in
prose, as a free alternative.

**The menu entry lands where the plan predicted.** Read off the built binary:
`menue="Dateilisting" eintrag="Anderes Dateifenster auf diesen Ordner stellen"
kombination=opt+cmd+s`, immediately after "Ordner der angezeigten Datei zeigen" and before "Pfad
eingeben und dorthin springen".

**C2's "blendet in keiner Lage einen Bereich aus" holds structurally, not by care.**
`Bereich::teilt_flaeche_mit()` is `None` for `Links` and `Rechts`, so revealing a file pane cannot
displace the Editor/Vorschau pair the way `einblenden(Editor)` does.

**C3 holds structurally too, by reusing `ordner_lesen`.** `Tabliste::ordner_setzen` carries
sortierung, `verstecke_ausgeblendet`, `tief`, `inhalt` and the filter text into the replacement
tab, so the target keeps its own view without this command holding a second rule about it.

**The two placements the compiler could not judge are both right.**
`Wirkungsbereich::Dateifenster` (`belegung.rs:1023`) matches C1's last criterion, and the comment
correctly explains why this command sits on the other side of the line from `ordner_der_datei`:
that one's source does not hang on the focus, this one's *is* the displayed folder of a file
pane. `Funktionsbereich::Dateilisting` (`belegungsmodell.rs:246`) puts it with the ascent and the
clipboard jump, which is the group whose members set the folder a file list shows. Neither match
has a catch-all, so the compiler forced the additions; it did not choose where they went, and
both choices hold.

**The follow-up chain from the target pane is sound.** `ordner_lesen` on the non-active pane
reaches `ordnerwechsel_melden`, which re-arms the file-system watch, rewrites the window title
from the unchanged focus, and pulls the area bar — so the watch follows the target's new folder
without this command knowing about watches.

## Cross-cutting observations

**Three of the six findings are one shape: a sentence that is true of an older tree.** L1, L2 and
L4 each state a reason that was correct when written or correct in another file, and each is now
contradicted by something two hundred lines away or in a decision record. This tree's counter to
that is measurement, and it works — the nine numbers this turn corrected were all held by nothing
and all found by hand, exactly as the plan predicted. The sentences that carry a *reason* rather
than a *number* have no such sweep, and all three of these got past a careful implementation and a
plan that named the doc comment's content clause by clause.

**Two of the six are enumerations kept by hand (L3, and the needle set behind M1).** Both were
correct at the moment they were written and both are already or nearly stale. This is the same
failure `CLAUDE.md` records for its own counts and answers by not carrying them.

**The one guard that matters most is the one that works.** `der_befehl_steht_vor_dem_auffangzweig`
holds the site `shared/issues/260818-1635_o_…` was filed about, and it is the only mechanism in
the tree that does. Turn 2 adds the drop, which brings no new `Kommando` — so that guard covers
this round's whole exposure on that site, and nothing further is needed for it.

## Recommended sequencing

**Before the acceptance run:** M2. It decides which of two acceptance criteria the user is going
to tick, and settling it afterwards means running the affected part twice.

**Before turn 2 lands, or with it:** M1. The drop work will touch `anwendung.rs` heavily, and a
focus guard that reads as present and is absent is worth least exactly when the file is moving.

**Cleanup, any time before the round closes:** L1, L2, L3, L4. None blocks a release; L3 and L4
are both single-file prose edits in `resources/default-keymap.toml`, so they travel together.

**Nothing here blocks a release.** The build, the tests, the formatting and clippy under
`-D warnings` are green, and the built command does what C1 and C3 promise in every lage except
the one M2 names.
