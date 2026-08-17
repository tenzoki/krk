The third difference between loud and quiet is called "die Folgen" and no consequence is added

---
Spec C3's last acceptance criterion reads: "Die laute Form unterscheidet sich von der ruhigen genau
in drei Dingen: dem Warnzeichen, dem Grund in der Frage und **den Folgen in der Erläuterung**."
`crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs:12-17` repeats the same three, with the same
word. What the loud explanation actually gains is neither a consequence nor a statement about one: it
gains the **remaining warning reasons**, as a paragraph "Außerdem: …" (`loeschwarnung.rs:776-780`).

---

**Severity:** Low. Behaviour is right and matches the operative criterion two bullets above the one
quoted, "Treffen mehrere Auslöser zugleich zu, nennt die Frage einen davon, und die Erläuterung führt
die übrigen auf". The defect is that the summarising criterion and one of the two files that repeat it
name a thing the loud form does not produce, so anyone checking C3 against the tree looks for a
consequence sentence and finds none.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs:12-17`;
`shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, C3, last acceptance criterion
**Tree state:** `792995a`
**Domain:** code

## The three differences, measured

Read off `loeschbestaetigung::zeigen` (`:120-140`) and `frage_und_erlaeuterung`
(`loeschwarnung.rs:755-787`):

| the spec's word | what the tree does |
|---|---|
| das Warnzeichen | `if laut { blatt.als_warnung(); }` — `loeschbestaetigung.rs:132-134`. Exactly this. |
| der Grund in der Frage | the first reason's wording between the count and "in den Papierkorb räumen" — `loeschwarnung.rs:759-772`. Exactly this. |
| die Folgen in der Erläuterung | `"\n\nAußerdem: {die übrigen Wortlaute}."` — `loeschwarnung.rs:776-780`. **The remaining reasons, not a consequence.** |

Everything else is identical in both forms, and that half of the criterion holds: the buttons, their
order and their keys come from `schaltflaechen(schaltflaeche)` with no reference to `laut`, and the
hint line "Return und Esc brechen ab. Zum Bestätigen Cmd+Return." is appended unconditionally
(`:131-135`). The probe `ohne_grund_bleibt_die_ruhige_form_unveraendert` (`loeschwarnung.rs:1670`)
pins that the quiet texts are unchanged word for word.

## Where the word is right and where it is not

`kommandos/loeschwarnung.rs` describes the same thing correctly and never says "Folgen": "Sonst geht
der Wortlaut des **ersten** Grundes in die Frage … und die **uebrigen** stehen als eigener Absatz in
der Erlaeuterung" (`:734-739`). So the two files that describe one mechanism use two different words
for the third difference, and only one of them matches the code.

The spec's own earlier line is where the word comes from: "Die Erläuterung trägt den Pfad und die
Folgen" in `## Was der Nutzer entschieden hat`. Read there it is loose prose about the quiet
explanation, which does carry the path and the folder count; carried into C3's summary it names a
third difference that does not exist.

## Direction

Fix `loeschbestaetigung.rs:12-17` to say "die übrigen Gründe in der Erläuterung", matching
`loeschwarnung.rs` and the code. The spec is user-approved and a correction there is not this
record's to make; note instead that C3's operative criterion is the "Treffen mehrere Auslöser
zugleich zu" bullet, which the tree satisfies, and that the summary bullet's third item is the same
thing under a misleading name. If the spec is edited in bundle E's pass over superseded wording
(C6), this is one line for it.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, and holds without
re-measuring the code.** This record was filed against `792995a`, and the only commit since is
`e313841`, which touches nothing under `crates/` or `resources/` — it adds this Circle's Bundle C
review and its nine records and nothing else (`git show --stat e313841`). The cited lines are
therefore the lines the review read. `make check` at 260817-1833: exit 0, "alle vier gruen".
