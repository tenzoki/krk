The drop passes its target folder as the source folder, so the completion reads that one folder twice

---

`Anwendungsdelegierter::abwurf_ausfuehren` (`crates/krk-ui/src/appkit/anwendung.rs`, step 9 of
the round-13 plan) calls
`auftrag_starten(seite, auftrag, ziel, quellen.len())` — the third argument is `Vorgang.quellordner`,
documented as "Der Ordner, aus dem die Eintraege stammen" (`anwendung.rs:423`). For a drop that
folder belongs to a foreign application, so the plan fills the field with the **target** folder
instead (step 9: "`auftrag_starten(seite, auftrag, ziel, quellen.len())` ruft").

The consequence is one redundant directory read per accepted drop. `Vorgang::ordner`
(`anwendung.rs:448`) starts from `quellordner` and pushes the `ziel` carried inside
`Art::Kopieren` / `Art::Verschieben`, so for a drop it returns the same path twice, and
`vorgang_beenden` runs `auffrischung::ordner_neu_lesen` once per entry.

---

**Severity:** Low, and the two things that could have made it worse were checked and do not
happen:

- The second read reaches the same result. `DateifensterQuelle::neu_lesen`
  (`tabelle.rs:888`) re-derives the selection name from the inventory still standing at that
  moment and re-notes it as `wunschauswahl` (`Tabliste::aktiven_neu_lesen`, `tabs.rs:716`), and
  the scroll position is re-read from the view first. Nothing is lost between the two runs.
- The first run's batches cannot land in the second run's model.
  `Ordnermodell::lesevorgang_beginnen` (`krk-core/src/verzeichnis/modell.rs:392`) raises the
  generation, and stale batches are dropped against it.

What remains is a wasted walk over the target folder on every drop, plus a field whose
documented meaning does not hold for the third caller.

**Not affected:** the source folder of a move drop. If one of the two panes displays it, the
Dateisystemwache refreshes it while the operation runs — a drop is not one of the arts that defer
the refresh (`auffrischung::schiebt_auffrischung_auf` defers only `UmbenennenImStapel`).

**Found by:** coder, implementing step 9 and following `quellordner` into `Vorgang::ordner`.
**Affects:** `crates/krk-ui/src/appkit/anwendung.rs`, `abwurf_ausfuehren` and the
`Vorgang.quellordner` doc comment.
**Related:** `issues/260818-2129_c_the-redundant-read-the-path-comparison-allows-is-not-without-consequence.md`
— the same class of claim about a redundant read, checked there and found to be wrong; checked
here and found to hold.
**Tree state:** `07347b8` plus the working tree of step 9.
**Domain:** code

## What a fix would have to do

Two candidates, and neither is free:

1. **Give `Vorgang::ordner` a deduplication.** One line at the one place that answers "which
   folders does this operation rewrite". It also covers any future caller that passes the same
   folder twice, and it does not touch the meaning of the field.
2. **Pass the real source folder** — `quellen.first().and_then(Path::parent)` — and decide what
   an empty source list or a root path means. That fills the field truthfully and refreshes the
   source pane on completion instead of relying on the Dateisystemwache, at the cost of a case
   split the plan did not ask for.

Option 1 is the smaller change and the one this defect points at. Neither belongs in step 9,
which is bound to the plan's wording.

**Filed by:** coder
