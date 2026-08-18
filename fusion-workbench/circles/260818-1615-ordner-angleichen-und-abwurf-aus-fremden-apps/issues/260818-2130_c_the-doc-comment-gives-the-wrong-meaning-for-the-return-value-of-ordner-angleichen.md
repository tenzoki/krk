The doc comment of `ordner_angleichen` gives the wrong meaning for its return value

---

`ordner_angleichen` (`crates/krk-ui/src/appkit/anwendung.rs:3312-3313`) closes with:

> Liefert immer `true`, wie [`Self::ordner_der_datei_zeigen`]: der Befehl war zustaendig, auch
> wenn er nur etwas zu melden hatte.

Jurisdiction is not what the value carries. `kommando_ausfuehren` states the contract at
`anwendung.rs:2889-2894`:

> **Was der Rumpf meldet, ist seit der Runde 7 nicht mehr der Rueckgabewert dieser Funktion.**
> Der Wert traegt genau eine Aufgabe weiter: er entscheidet ueber die beiden Nachwirkungen unten.
> Ein Befehl, der nichts getan hat, braucht weder einen Nachzug der Aufteilung noch eine
> vorgemerkte Sitzung.

The two follow-ups are `self.aufteilung_nachziehen()` and `self.sitzung_vormerken()`
(`anwendung.rs:3089-3092`). Whether the command was *zustaendig* is answered elsewhere and
`kommando_ausfuehren` returns `true` unconditionally at `:3093`.

Measured against that contract, `ordner_angleichen` returns `true` in both branches where it did
nothing — the equality branch (`:3331`) and the too-narrow branch (`:3346`) — so every no-op press
of `opt+cmd+s` re-lays out the row and schedules a session write.

**The cited precedent is accurate, and that is the awkward part.** `ordner_der_datei_zeigen`
(`anwendung.rs:3247`) returns `true` on its own no-op path for the same reason. So the new
function is consistent with the tree; what it adds is a doc comment that states a *reason* the
tree's own contract contradicts, and a third instance of the divergence.

---

**Severity:** Low. Both follow-ups are idempotent: `aufteilung_nachziehen` re-lays out a layout
that did not change, and `sitzung_vormerken` writes a session that did not change, under the
throttle the writer already carries. Nothing is corrupted and nothing is measurably slower.
**Found by:** coderev, comparing the new doc comment against the contract at the head of the
match it feeds.
**Affects:** `crates/krk-ui/src/appkit/anwendung.rs:3312-3313` (the doc comment), `:3331` and
`:3346` (the two `true`s), against `:2889-2894` (the contract)
**Related:** `crates/krk-ui/src/appkit/anwendung.rs:3247` — `ordner_der_datei_zeigen`, the same
divergence, older.
**Tree state:** `71413c3`
**Domain:** code

## What a fix would have to do

Two shapes, and the choice is not this reviewer's:

1. **Correct the sentence and leave the code.** Say that `true` is given deliberately although
   nothing changed, and why the two follow-ups are harmless here. That leaves the divergence
   standing but stops the tree carrying two readings of one value.
2. **Return `false` from the two no-op branches**, which is what the contract at `:2889-2894`
   asks for, and do the same at `ordner_der_datei_zeigen` so the two stay one rule. This is the
   larger change and touches a function outside this round.

Filing this as one record rather than two: the new sentence and the old precedent are the same
disagreement, and splitting them would produce two records that have to be resolved together.

**Filed by:** coderev

---
Resolved: Beide waren falsch, Kommentar und Code. Entscheidend war nach_dem_sichtbarkeitswechsel: es legt die Fensterzeile nicht neu aus, ein eingeblendeter Bereich bekommt seinen Auslegungsdurchgang allein ueber kommando_ausfuehren. Der Zweig eingeblendet-aber-nicht-gelesen liefert deshalb true, die zwei Zweige ohne Wirkung liefern false. Die sechs weiteren Stellen mit derselben Fossilie bleiben stehen; sie sind der groessere Umbau aus Moeglichkeit 2 des Datensatzes.
