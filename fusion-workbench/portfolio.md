# Portfolio

**Generated:** 260819-0804 (by playmaker session 260819-0804-playmaker-direct-dispatch)
**Domain bias:** code

---

**What is next.** Round 13 closed coherent on 260819, no Circle is active, and the choice of the
next round is yours between three candidates of which only one is anticipated. The anticipated one
is the built-in web viewer in the preview pane, filed on 260804 and passed over for thirteen
rounds since; it needs an investigation of the rendering mechanism and a clarification round over
three questions before it can be activated. Beside it lies the deferred round
`260816-2255-befehle-absetzen-und-makros-speichern`, with a finished spec and a finished plan,
nothing built and nothing wrong with it. Taking it up means creating a new Circle, because
deferred is a terminal state. The backlog holds one live idea, a second keyboard shortcut for the
editor, and it is the smallest of the three.

**This portfolio is written in English, and the previous one was German.** `CLAUDE.md` declares
`**Artifact language:** en` on line 4, added on 260817, and the portfolio is a persisted file for
the project's own use. See `## Warnings`, point 4, for why the switch is not silent and what in
`CLAUDE.md` still contradicts it.

**What moved since the run of 260818-1018.** Round 13 built two capabilities and found a
data-loss path older than itself. The user ran the acceptance himself, so the round closes
coherent rather than bounded, as the second of thirteen to do so. Five defect records opened and
none of the counts fell: 138 open defects now against 133, and 29 open decisions unchanged.

---

## Active (_t_)

(none)

`fusion-workbench/.active-circle` is absent and no Circle record carries the active marker
(`_t_`). Both together are the ordinary state after a closure, and no warning attaches to it.
Round 13, `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps`, closed on 260819.

## Anticipated (_a_) — ranked

**Recommended next:** `260804-0933-eingebauter-web-betrachter-im-vorschaufenster` — the only
anticipated Circle, every precondition built into the tree, one open decision record binding it;
an investigation and a clarification round stand before activation, and one line of its record
needs correcting first.

### Rank 1 — the built-in web viewer

`circles/260804-0933-eingebauter-web-betrachter-im-vorschaufenster/_*_circle.md`

**The Directive in one sentence.** KRK displays a web address in a viewer of its own, living in an
ordinary tab of the preview pane, driven from the keyboard and carrying jump marks on every
visible link; `Opt+Cmd+G` opens the address from the clipboard inside KRK rather than in the
system browser.

This Circle holds rank 1 as the only candidate, so the ordering is not an achievement of the
ranking heuristic. What the heuristic contributes is the precondition check, and it comes out
clean again. One open decision record binds it,
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1428_*_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md`,
which asks how KRK addresses from Rust an interface that exists only from macOS 26 onward; the
record's own Grounding section classes that binding as an inference rather than a finding. Its
`## Dependencies` section names exactly one Circle, `260802-0842-krk-mac-dateimanager-editor-git`
(round 1), which is terminal and built into the tree. The two parts it sits on, the clipboard
evaluation from step S13 and the preview pane from step S19, stand. For the `code` weighting the
bounded closure of that dependency would be a deduction if coherent (`_c_`) alone counted as a
fulfilled precondition. It is not applied here, and the reason
is under `## Warnings`, point 1.

**What round 13 changed for it.** Four things move, and none blocks activation. Counted against
the tree on 260819: `enum Kommando` (`crates/krk-core/src/tasten/belegung.rs`) carries 79 variants
and `resources/default-keymap.toml` 85 `[[funktion]]` blocks, each one more than the record cites,
and `opt+cmd+s` is now taken. The tree accepts drags for the first time, registered on the file
table alone (`crates/krk-ui/src/appkit/tabelle.rs:4336`), so a web view accepting its own drops
would be a second registration point and a question for the activation spec. The rule that exactly
one wrapper around `NSPasteboard` exists held under the round's pressure: the new module
`crates/krk-ui/src/appkit/abwurf.rs` names only `NSPasteboardType` constants and reads through
`appkit/zwischenablage.rs`, so the clause in this Circle's `## Dependencies` still resolves. The
status line still carries six ranks, so the viewer's messages are still the seventh, and the
enumeration has no catch-all branch.

**What stands before activation, unchanged since 260804.** First an investigation of the rendering
mechanism, because the Circle deliberately fixes neither a system interface nor a foreign crate,
and the open decision on availability checking hangs on that choice. Then the clarification round
over the three open questions in the record, of which the first decides the scope: clipboard and
the page's own links give KRK a viewer, while address entry and stored web addresses give it a
browser.

**One correction belongs before activation.** Line 438 of the record cites a filename part that
never existed, filed as
`shared/issues/260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`
and still open. It sits inside `## Grounding snapshot`, which is read as binding ground at
activation. The playmaker does not correct it: it writes only the three sections its mandate names.

No other Circle record carries anticipated (`_a_`). The second candidate for the next round is
under `## Archived` and is deferred rather than anticipated, so it is not ranked here.

## Backlog — ranked

**Recommended to shape:** `shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`
— one idea, no split needed, its own stated precondition is answered, and round 9 built the
precedent for the same case.

```
/fusion:direct shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md
```

### Rank 1 — a second shortcut for the editor entry

`shared/backlog/260813-2033_*_der-editor-einstieg-braucht-ein-erreichbares-kuerzel-neben-f4.md`

A second, better reachable key combination for `bearbeiten`, which sits on `f4` alone today.
Exactly one idea, so no split is proposed. The entry has carried recommended (`_p_`) since
260814-1513 and keeps it.

The precondition the entry brings with it is answered, and the idea survives the answer. The entry
suspects `f4` is awkward only because the system setting "Use F1, F2, etc. keys as standard
function keys" is off. Measured on 260802-1137 on the acceptance machine with that setting off,
`fn+F3`, `fn+F5` and `fn+F8` arrive as ordinary `keyDown` events, and KRK cannot tell a held `fn`
key from a bare function key at all
(`shared/decisions/260802-0842_*_f-tasten-unter-macos-systembelegung.md`, addendum 260802-1409,
evidence `spikes/fn-tasten/messung-A.txt`). The same addendum promises that every function of the
Norton row carries an additional Cmd shortcut by default, and round 9 took exactly that step:
`notizzettel` sits on `f2` and `cmd+k`.

What a clarification round would have to carry is the choice of combination, and it is tight. All
four Cmd levels of `e` are taken: `cmd+e` on `editor_aus_vorschau`, `shift+cmd+e` on
`fokus_editor`, `opt+cmd+e` on `editor_schliessen`, `ctrl+cmd+e` on `editor_ansicht_umschalten`.
Counted on 260819, the keymap carries 85 entries and `Kommando` 79 variants, each one more than at
the run of 260818-1018, and round 13 took `opt+cmd+s`. Alongside that sits the same precondition
as for the web viewer: a new combination reaches no user who has assigned a key themselves since
round 7 (`shared/issues/260814-0656_*_eine-neue-funktion-kommt-bei-jedem-nutzer-mit-eigener-keymap-unbelegt-an.md`).

The entry is not a duplicate of the open defect
`circles/260811-1304-statusleiste-mit-bereichsschaltern/issues/260812-0512_*_f4-nimmt-am-schmalen-fenster-eine-datei-in-einen-editor-an-den-niemand-sieht.md`.
That defect concerns the same `f4` with a different symptom and would survive whichever second
combination is added.

**Performed this run:**

None. This run holds no user confirmation for any of the four confirmation-gated operations, so it
performed none. None is proposed either: the one live entry carries exactly one idea, there is no
second entry to merge it with, its idea is still live, and deferring it would be a disposition
that belongs to you. The one autonomous write, the ranking rename between `_o_` and `_p_`, had
nothing to do: the entry already stands at `_p_` and stays rank 1.

Two entries stand closed and name in their body the Circle they became:
`shared/backlog/260813-0822_*_titelleiste-fuehrt-name-und-version.md` (round 8) and
`shared/backlog/260813-2033_*_ein-scratchpad-das-per-taste-mittig-erscheint-und-sich-selbst-sichert.md`
(round 9).

Half of the recommended entry is defect-shaped rather than idea-shaped. It stands under
`## Warnings`, point 10; the playmaker files no record for it.

## Recently closed (_c_ / _b_)

| Circle | Marker | Closure in one sentence |
|---|---|---|
| `260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps` | `_c_` | Coherent on 260819 after the user ran the acceptance himself against the built bundle 0.5.2: `opt+cmd+s` puts the other file pane on the active pane's folder, and a file list now accepts files and folders dropped from foreign applications, copying by default and moving with `cmd`. |
| `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb` | `_c_` | Coherent on 260818: KRK knows exactly one deletion path, every operation asks once with "Cancel" preselected, unusual targets and large volumes carry a warning sign, and permanent deletion fell out of the application, the keymap and the menu. |
| `260816-1321-inhaltsfilter-mit-ankreuzfeld-content` | `_b_` | Bounded on 260816-2030: the file filter takes content into account, switched on through the tenth checkbox "Content", reading text only and only up to 1 MB; built and re-read against the tree, not accepted against the bundle. |
| `260814-1551-tippen-filtert-dateiliste-flach-und-tief` | `_b_` | Bounded on 260815: typing filters the file list at any position in the name, the filter text belongs to the tab, and a ninth checkbox "Deep" widens it to the subtree; ten of the 77 criteria with a bundle share left unaccepted. |
| `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln` | `_b_` | Bounded on 260814-1300: notes as the tenth sheet with two notes as tabs, `f2` and `cmd+k` in, `Esc` back; 16 of the 29 criteria with a bundle share untouched by any observation. |

Older closures: `260813-0939-titelleiste-fuehrt-version-und-semantische-tags` (coherent on
260813-1415), `260813-0100-suche-in-der-belegung-vollstaendiges-menue-weitere-instanz` (bounded on
260813), `260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern` (bounded on 260812),
`260811-1304-statusleiste-mit-bereichsschaltern` (bounded on 260812-0820),
`260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen` (bounded on 260811-2210),
`260809-2040-tastenbelegung-als-markdown-in-downloads` (bounded on 260811-1415),
`260807-2116-eingebauter-editor-mit-textmarken` (bounded on 260810-1445),
`260802-0842-krk-mac-dateimanager-editor-git` (bounded on 260807-1035).

Thirteen rounds have run: ten bounded, three coherent. **The three coherent ones carry their
marker for three different reasons**, and the difference is under `## Warnings`, point 1.

### What round 13 found beyond its own Directive

The round reached a data-loss path older than itself, and it sits in the shared core rather than on
the round's own surface. `operation::ziel_klaeren` answered "overwrite" with
`loeschen::baum_entfernen`, a real `remove_file` rather than the trash. Where the target under a
second spelling was the source, that deleted the user's file, and the completion list then reported
"no entry of that name" about a file that had existed before the drop. The same textual protection
let a folder descend 139 entries into its own tree.

The guard is now an inode comparison rather than a textual one, in
`crates/krk-core/src/operation/mod.rs`: `zielpfad` asks `benennen_denselben_eintrag` over
`(st_dev, st_ino)` before it hands a target to `ziel_klaeren`, using `lstat(2)` for "would I write
over what I am reading" and `stat(2)` for "where does this path lead". Both callers of
`ziel_klaeren` sit behind it, `operation::kopieren` and `operation::verschieben`, so `f5` and `f6`
inherit the guard and not only the drop. Two consequences belong in the portfolio. Every future
round inherits a copy and move path that is safer than the one every closed round was built
against. And the pre-check during the drag, `ziel_ist_quellordner` in
`DateifensterQuelle::abwurf_pruefen`, still compares text and is therefore a prediction; the
decidable answer is given at the moment of access, which is where it can be given.

## Archived (_s_ / _d_)

| Circle | Marker | State |
|---|---|---|
| `260816-2255-befehle-absetzen-und-makros-speichern` | `_d_` | Deferred on 260817-0445 in favour of round 12. Nothing is built and the Directive is reachable; the round was simply not up. Left complete: a spec with 54 acceptance criteria (`shared/planning/260816-2240_*_spec-befehle-absetzen-und-makros-speichern.md`), a plan with 22 steps in five bundles (`circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_*_plan-befehle-absetzen-und-makros-speichern.md`), two decisions and one defect record with a measurement. |

No Circle record carries superseded (`_s_`).

**Deferred is a terminal state.** A `mv` back to anticipated is disallowed. Whoever takes this
round up creates a new Circle that cites the deferred one through `## Dependencies` and adopts its
spec and plan (`rules/circle-records.md`, `### Worked transitions`). It is the one candidate for
the next round that is not anticipated and therefore does not appear in the ranking above, named
here so the choice stays open to you rather than being pre-decided by the portfolio's silence.

## Warnings

**1. The `_c_` marker now carries three different meanings in this project, and `_b_` is not a
failure.** `CLAUDE.md` records that ten of the thirteen rounds close bounded and always for the
same reason: the acceptance run of the ten timing promises needs KRK in the foreground and is
therefore the user's own work, which no agent can perform. The marker measures the user's
availability there, not the round's maturity, and a heuristic counting coherent (`_c_`) alone as a
fulfilled precondition gives a misleading answer here. This run applied no such deduction, as every
run before it. The other direction has now split three ways. Round 8,
`260813-0939-titelleiste-fuehrt-version-und-semantische-tags`, closed coherent after an acceptance
run the user documented on disk. Round 12, `260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb`,
closed coherent at a Rebalance gate with no acceptance run at all, on the ground that its Directive
says nothing about the ten timing promises. Round 13 closed coherent after an acceptance run the
user did perform, recorded only as an assertion in the closure note. Anyone reading `_c_` as
"accepted by the user against the bundle" is right about two of the three and wrong about round 12.

**2. Round 13's acceptance run left no acceptance record on disk.** The closure note states that
the user accepted ten checks against the built bundle 0.5.2 on 260819, including every criterion
for C4 to C7 that needs a drag from a second application. Round 8 left a file for the same act,
`circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1405-abnahmeliste-e2.md`,
and named it in its closure note. Nothing under `messungen/` or in round 13's `history/` carries
the counterpart; the newest file there is the reconciliation of 260819-0102, written before the
acceptance run. The claim is the user's own and this is not a challenge to it. What is missing is
the artifact that would let a later round check which of the ten checks passed against which
bundle.

**3. Two records of round 13 are still uncommitted.** `git status` shows the record renamed from
`_t_circle.md` to `_c_circle.md` as a deletion plus an untracked file. The closure is on disk and
not in git; a `/fusion:cleanup` or a plain commit closes that.

**4. `CLAUDE.md` contradicts itself about the artifact language, and it is behind on the round
count.** Line 4 declares `**Artifact language:** en`, added on 260817, while line 176 still states
that `bin/fusion-rules` emits `default-voice-de.yaml` for long-form agents. The helper emits
`default-voice-en.yaml`, checked on this run. The tree is mid-switch and inconsistent by store:
round 13's defect records carry English titles, its session histories German ones. This portfolio
follows the declaration. Separately, the line "Zehn Runden sind gefahren" and the table under it do
not know rounds 11 to 13, and `## Projektstand` is dated 260815-0600 and names the delivery as
`v0.4.1` while `Cargo.toml` carries `0.5.2`. The file says of itself that the file inventory binds
and not the line, which keeps the error small without removing it. A run of `/fusion:curate` closes
the gap.

**5. Three acceptance runs are still outstanding, and all three are the user's work.** Round 11
left its list finished (`messungen/260816-abnahme-inhaltsfilter.md`, 28 observations at four places
with the handling and the expected result). Round 10 left ten of its 77 acceptance criteria with a
bundle share open, four of them safety-relevant
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/history/260815-0400-abnahmeliste-g2.md`).
Round 9 left 21 criteria without full evidence. No marker moves for any of them: bounded is a
terminal state. What the runs bring in is the evidence, not the letter.

**6. The acceptance run of the ten timing promises has not run since 260810-1918.** It now lies
before rounds 5 to 13. That last run was the first fully clean one, all ten promises across all
five passes. A second thing hangs on it: the deferred record
`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-nachdem-die-messung-sich-erholt-hat.md`
waits for further runs on different days and, being deferred, drops out of every search for active
ground. If it is never measured again, "stay at 65" is decided in substance without anyone having
written it down.

**7. The release gate stands open again.** Checked on 260819: `git tag --points-at HEAD` returns
nothing and 13 commits lie between `v0.5.2` and `HEAD`. `Cargo.toml` carries `0.5.2`. Station 1 of
`cargo xtask release` compares tag and version and stops the path. The state returns after every
round that adds commits and sets no tag; the tag is the user's work. `cargo xtask bundle` and
`make check` do not depend on it.

**8. 138 defect records are open**, 35 of them in the shared store, five more than at the run of
260818-1018. All five new ones come out of round 13's session: three in its own store
(`260818-1704_*_`, `260818-2221_*_`, `260818-2228_*_`) and two in the shared one
(`260818-1635_*_`, `260818-2145_*_`). Round 13's closure note places its three as holding nothing
up. The list:
`find fusion-workbench/shared/issues fusion-workbench/circles/*/issues -maxdepth 1 -name '*_o_*.md'`

**9. 29 decision records are open and twelve are answered but not implemented**, both unchanged
since the run of 260818-1018. Round 13 opened none and closed its one decision as implemented
(`circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/decisions/260818-1633_*_gilt-ein-unentscheidbares-schreibrecht-beim-abwurf-als-erlaubnis-oder-als-abweisung.md`).
No open question holds up a plan step; all of them bind future work. The list:
`find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'`

**10. The recommended backlog entry describes a defect in half of its body, and the playmaker files
none.** The user decision of 260802-1409 promises that every function of the Norton row carries an
additional Cmd shortcut by default and names "F4 Bearbeiten" among its six. The comment on
`bearbeiten` in `resources/default-keymap.toml` justifies the deviation by saying the two-way rule
applies to the first five functions, checked on 260819 and unchanged by rounds 12 and 13. The two
statements do not reconcile. Either the comment is an unevidenced reinterpretation of an
implemented user decision, which makes it a defect, or the user took `bearbeiten` out deliberately,
in which case the record for that is missing. The decision is yours; the playmaker writes to
neither the backlog nor the defect store.

**11. No dependency cycle.** The directed graph over the non-terminal Circles has one node and no
edge inside that set: the web viewer is the only non-terminal Circle, and its one dependency edge
leads to round 1, `260802-0842-krk-mac-dateimanager-editor-git`, which is terminal. Counted against
the record's `## Dependencies` section on 260819, that section names one Circle and not four; the
portfolio of 260818-1018 and the activation proposals before it speak of rounds 1, 5, 6 and 7,
which the section does not carry. The cycle verdict is unaffected either way, because every named
Circle is terminal. No `## Dependency warning` was appended to any Circle record.

**12. No parent-grounding-stale note was appended this run, and the condition was not met.** The
trigger is a child Circle reaching Bounded Closure (`_b_`); round 13 closed coherent (`_c_`). Every
bounded Circle already carries its note on the web viewer's record. What round 13 did age is
recorded in the activation proposal of 260819-0804 instead: two counts each grown by one, drag
acceptance now present in the tree, the single clipboard wrapper holding, and the status line
staying at six ranks.

**13. The web viewer's record now carries nineteen playmaker sections from eleven runs**, eleven
activation proposals and eight stale-grounding notes, in a little over 1270 lines. The length grows
with every run in which the Circle stays anticipated without being worked on. This run's one
section carries the changes on its own. Whoever wants the current state reads the last section, not
all nineteen.
