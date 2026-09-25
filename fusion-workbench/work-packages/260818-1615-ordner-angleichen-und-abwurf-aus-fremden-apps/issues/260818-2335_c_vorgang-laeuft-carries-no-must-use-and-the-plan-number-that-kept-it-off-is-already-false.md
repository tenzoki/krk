# `vorgang_laeuft` carries no `#[must_use]`, and the plan number that kept it off is already false

---
**Domain:** code
**Status:** closed
**Filed by:** coderev
**Cross-references:** `crates/krk-ui/src/appkit/anwendung.rs`, `fn vorgang_laeuft`; plan `circles/260818-1615-…/planning/260818-1633_*_plan-…` `## Testing Strategy` ("diese Runde setzt vier neue `#[must_use]` und ein `let _ =`"); `CLAUDE.md`, "Was man nicht sieht" (the project rule); session history `history/260818-2230-schritt-9-vorgang-laeuft-und-abwurf-ausfuehren.md`

---

## The question this record settles

The implementing agent of step 9 left `#[must_use]` off `vorgang_laeuft` deliberately and referred the judgement to the review. The stated reason was that the plan's Testing Strategy says the round sets four new `#[must_use]`, and a fifth would make that sentence false.

## Verdict: it should carry one, and the plan sentence should be corrected

Two parts.

**The rule applies.** `CLAUDE.md` states it without an exception: "Ein Rückgabewert, dessen stilles Fallenlassen unbemerkt bliebe, bekommt in diesem Projekt `#[must_use]`." `vorgang_laeuft` is a pure query with no side effect at all — since step 9 removed the side effect that used to be there, that is its entire point. A bare `self.vorgang_laeuft();` does nothing whatever and compiles green: `unused_results` is allow-by-default, so only `#[must_use]` makes such a line an error under `-D warnings`. And the thing that would be silently dropped is the answer to C6 Lage 1, the one question the drop path is forbidden to ask twice. This is the same argument the round already accepted for `bereich_einblenden` in step 3, whose message reads "eine Abweisung bleibt stumm".

**The number that argued against it was already wrong before this question arose.** Counted against the diff `71413c3..a7419cd`, this Turn added **seven** `#[must_use]` attributes:

| Where | |
|---|---|
| `appkit/abwurf.rs` | `sorten`, `beschreibbarkeit`, `angebot`, `zeiger` |
| `kommandos/abwurfregel.rs` | `marke`, `urteil` |
| `appkit/tabelle.rs` | `abwurfmeldung` |

Turn 1 added an eighth on `bereich_einblenden` (`b47355e..71413c3`). The round also added **two** `let _ =`, not one: `anwendung.rs` in `abwurf_ausfuehren` and `zwischenablage.rs` in the test helper `probenablage`. So the plan sentence is off by four on one count and by one on the other, entirely independently of `vorgang_laeuft`. It is a stale documentation number, not a constraint the code owes anything to — precisely the kind of prose count `CLAUDE.md` has repeatedly had to stop keeping.

## What to do

1. Put `#[must_use]` on `vorgang_laeuft`, with a written-out reason in the project's style: dropping the answer means starting a second operation while one is running, and nothing turns red.
2. Correct the plan's Testing Strategy sentence to the counted numbers, or replace the enumeration with the command that counts them, the way `resources/default-keymap.toml` was just changed to do for the `opt+cmd` row (`a7419cd`).

---
Resolved: Gesetzt. Die Zahl aus der Pruefstrategie des Plans, die es verhindert hatte, war schon vor diesem Datensatz falsch: die Runde setzt acht must_use und nicht vier, und zwei let _ = statt einem. Die Stelle im Plan bleibt unberichtigt und ist als eigener Punkt vermerkt.
