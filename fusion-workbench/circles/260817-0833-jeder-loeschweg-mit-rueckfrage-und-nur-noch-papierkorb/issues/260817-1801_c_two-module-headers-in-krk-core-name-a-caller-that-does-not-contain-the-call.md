Two module headers in `krk-core` name a caller that does not contain the call

---
`crates/krk-core/src/verzeichnis/umfang.rs:138-141` and
`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:151-154` both open their section "Wer sie ruft"
with "Genau einer, und er steht seit dem elften Schritt derselben Runde da:
`Anwendungsdelegierter::loeschen_nach_rueckfrage` in `krk-ui`". Neither call stands in that
function. `umfang::zaehlen` is called at `appkit/anwendung.rs:4869` and
`arbeitsbaum::beruehrt_einen_arbeitsbaum` at `:4857`, both inside
`Anwendungsdelegierter::loeschtexte` (`:4840`), which `loeschen_nach_rueckfrage` (`:4679`) calls
at `:4743`.

---

**Severity:** Low. Nothing behaves wrongly. The cost is one wrong turn for anyone following the
citation: `loeschen_nach_rueckfrage` runs from `:4679` to `:4788` and contains neither name, so a
reader who opens it and searches finds nothing and has to fall back to grepping the whole file.
Both sentences were written in `792995a`, the same commit that created `loeschtexte`.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-core/src/verzeichnis/umfang.rs:138-141`,
`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:151-154`
**Tree state:** `792995a`
**Domain:** code
**Cross-references:**
`issues/260817-1802_o_two-more-no-caller-yet-statements-remain-so-the-count-of-two-undercounts.md`
(the same commit's other stale caller sentences),
`shared/issues/260815-1448_o_die-neun-berichtigten-zahlen-stehen-weiter-unverankert-und-die-benannte-ursache-traegt-keinen-datensatz.md`

## Why the rest of each sentence is right

The substance of both sentences holds and only the name is off by one level:

- **"Genau einer"** — counted with `grep -rn "umfang::zaehlen\|beruehrt_einen_arbeitsbaum" crates/`:
  one call each, plus doc comments. Correct.
- **"einmal je Loeschbefehl"** — `loeschtexte` is called once, in the fourth branch of the stage
  rule (`:4743`). Correct.
- **"erst, wenn die beiden billigen Stufen jenes Rumpfes durch sind"** — correct, and this is the
  half that makes naming the right function worth the edit: the claim is about *where in the body*
  the call sits, and it is checkable only in the function that holds it.

## Direction

Name `Anwendungsdelegierter::loeschtexte` and say that `loeschen_nach_rueckfrage` reaches it in the
fourth branch of the stage rule — which is what carries the cost claim. Two sentences, no
behaviour. Worth doing in the same pass as the record cross-referenced above, since both are
caller sentences in the same two files.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, and holds without
re-measuring the code.** This record was filed against `792995a`, and the only commit since is
`e313841`, which touches nothing under `crates/` or `resources/` — it adds this Circle's Bundle C
review and its nine records and nothing else (`git show --stat e313841`). The cited lines are
therefore the lines the review read. `make check` at 260817-1833: exit 0, "alle vier gruen".

---
Resolved 260818 (coder, tree state `ae665e5`): both sections name the function that holds the call
and keep the claim about where in the body it sits.

`crates/krk-core/src/verzeichnis/umfang.rs:136-149` and
`crates/krk-core/src/verzeichnis/arbeitsbaum.rs:163-176` now read: the one caller is
`Anwendungsdelegierter::loeschtexte`, and `Anwendungsdelegierter::loeschen_nach_rueckfrage` reaches
it in the **fourth** branch of its stage rule. The half that carries the cost claim ("erst, wenn die
beiden billigen Stufen jenes Rumpfes durch sind") stands unchanged and is now checkable in the
function that holds the call.

Two more sentences of the same class were pulled along in the same pass, both in
`crates/krk-ui/src/kommandos/loeschwarnung.rs`:

- `:208-215` said `loeschen_nach_rueckfrage` calls `warngruende` and `frage_und_erlaeuterung` in its
  fourth branch. Both calls sit in `loeschtexte`; the sentence now routes through it.
- `:1262-1266`, the body of `die_ausloesertafel_hat_genau_einen_aufrufer`, named
  `loeschen_nach_rueckfrage` as the one caller of the trigger table. Corrected to `loeschtexte`,
  with the fourth branch kept as the route.

Checked with: `grep -n "loeschtexte" crates/krk-ui/src/appkit/anwendung.rs` (called at `:4687`,
defined at `:4784`), `grep -rn "umfang::zaehlen\|beruehrt_einen_arbeitsbaum" crates/` (one call
each, both in the body of `loeschtexte` at `:4799` and `:4810`), and reading the match in
`loeschen_nach_rueckfrage` at `:4647-4692`, whose fourth arm `Vorstufe::Rueckfrage` opens at
`:4681`. `make check` exit 0.
