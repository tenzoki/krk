Two more "no caller yet" statements remain, so the commit's count of two undercounts

---
`792995a` states in its message: "Two module headers in krk-core said 'who calls it: nobody', which
this task made false. Both sentences are corrected here rather than filed." Two were corrected
(`umfang.rs`, `arbeitsbaum.rs`). At least two more of the same kind stand:
`crates/krk-core/src/verzeichnis/loeschzielbefund.rs:121-131` still says the checks do not all
exist yet and that the type has no caller in this crate, and
`crates/krk-ui/src/kommandos/loeschwarnung.rs:1252` still summarises its probe as "heute keine"
while the probe's own body and assertion say one.

---

**Severity:** Low. No behaviour. Both are statements about the tree that the tree contradicts, and
one of them contradicts its own body five lines further down.
**Found by:** coderev, review `reviews/260817-1759-coderev-bundle-c-the-loud-confirmation.md`
**Affected:** `crates/krk-core/src/verzeichnis/loeschzielbefund.rs:121-131`,
`crates/krk-ui/src/kommandos/loeschwarnung.rs:1252`
**Tree state:** `792995a`
**Domain:** code
**Cross-references:**
`issues/260817-1801_o_two-module-headers-in-krk-core-name-a-caller-that-does-not-contain-the-call.md`,
`shared/issues/260815-1448_o_die-neun-berichtigten-zahlen-stehen-weiter-unverankert-und-die-benannte-ursache-traegt-keinen-datensatz.md`

## The first: `loeschzielbefund.rs`, section "Wer ihn beantwortet"

> Die Pruefungen, die ihn liefern, entstehen in derselben Runde und **stehen zu diesem Zeitpunkt
> noch nicht alle da** … **Solange keine davon dasteht, hat der Typ in dieser Kiste keinen
> Aufrufer**, und `dead_code` trifft ihn trotzdem nicht.

Both halves are false since bundle C:

| the four checks the paragraph lists | where it stands now |
|---|---|
| Papierkorb | `krk-ui/src/appkit/papierkorb.rs:185` (bundle B) |
| Netzlaufwerk | `krk-ui/src/appkit/volumes.rs:259` (`749a4f3`) |
| Git-Arbeitsbaum | `krk-core/src/verzeichnis/arbeitsbaum.rs:227` (`5a0f041`) |
| Umfang | `krk-core/src/verzeichnis/umfang.rs:217` — answers with `Umfang`, and its `Unentschieden` variant cites the type |

And the in-crate caller exists: `arbeitsbaum.rs` uses `super::Loeschzielbefund` as its return type
in all three functions (`:227`, `:288`, `:342`, imported at `:169`), which arrived with `5a0f041`. The
`dead_code` sentence beside it stays right for its own reason and is not the defect.

## The second: `loeschwarnung.rs:1252`

The probe was renamed in `792995a` from `die_ausloesertafel_hat_noch_keinen_aufrufer` to
`die_ausloesertafel_hat_genau_einen_aufrufer`, its assertion moved from `0` to `1`, and its body
rewritten to say so ("Der eine Aufrufer ist seit dem elften Schritt dieser Runde da, und die
Erwartung steht deshalb auf eins"). The one-line summary above it was carried over unchanged from
`c1b52db`:

```
/// Genau eine Stelle im Baum fragt die Ausloesertafel — heute keine.
```

The clause after the dash contradicts the clause before it, the body below it and the assertion at
`:1285`.

## Direction

Fix both sentences. For `loeschzielbefund.rs` the section can now say plainly which four checks
answer the type and where they stand, which is more useful than the "not yet" framing it replaces.
For `loeschwarnung.rs:1252` drop the trailing clause.

**And the reach question behind all four sentences.** The commit that corrected two of them found
them by knowing which files it had touched, not by a search. A search would have found all four
and cost one line. Measured over exactly the 13 files bundle C touched:

```
grep -n "keinen Aufrufer\|noch keinen\|Zum Zeitpunkt dieses Schrittes\|heute keine" <the 13 files>
```

returns three hits — `loeschzielbefund.rs:128`, `loeschwarnung.rs:1252` and one false positive
(`papierkorb.rs:42`, which speaks of an `Auftrag` and not of a caller). Two of three are the defect
in this record; the needle is short and the reading is quick. That is the same lesson as
`shared/issues/260815-1448_o_…-berichtigten-zahlen-…`: the reach of the search and the reach of the
claim have to stand next to each other.

---
Reconciliation 260817-1833 (reconciler, tree state `e313841`): **open, and holds without
re-measuring the code.** This record was filed against `792995a`, and the only commit since is
`e313841`, which touches nothing under `crates/` or `resources/` — it adds this Circle's Bundle C
review and its nine records and nothing else (`git show --stat e313841`). The cited lines are
therefore the lines the review read. `make check` at 260817-1833: exit 0, "alle vier gruen".
