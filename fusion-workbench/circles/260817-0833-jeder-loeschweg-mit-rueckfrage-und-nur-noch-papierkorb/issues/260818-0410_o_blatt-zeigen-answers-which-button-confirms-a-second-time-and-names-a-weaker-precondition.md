`Blatt::zeigen` answers "which button confirms" a second time, and the precondition it names is weaker than the real one

---

`285b58f` made "which button carries Return" a rule (`bestaetigungsstelle`) instead of an
assumption. `Blatt::zeigen` still carries an assumption for the neighbouring question "did the
user confirm": it hardcodes `stelle == 0`. Its doc names the precondition as "more than two
buttons → use `zeigen_mit_wahl`", but the real precondition is "the confirming button is first",
and the tree already contains a two-button sheet where it is not.

---

**Severity:** Medium. Latent — all five callers today come from `Blatt::neu`, where the two
coincide. It is filed because the divergence is silent, it decides confirmed-vs-cancelled on a
sheet, and the same class of assumption on the neighbouring question cost `260817-1106` and
`260817-1242` in this very Circle.
**Found by:** coderev, review `reviews/260818-0410-coderev-bundle-f-die-messungen-und-der-waechter.md`
**Affected:** `crates/krk-ui/src/appkit/blaetter/mod.rs:762-766`
**Tree state:** `a4d8211`
**Domain:** code

## What stands in the tree

```rust
// crates/krk-ui/src/appkit/blaetter/mod.rs:762-766
/// "Bestaetigt" heisst: die **erste** Schaltflaeche. Fuer ein Blatt mit
/// mehr als zweien ist [`Blatt::zeigen_mit_wahl`] der richtige Weg.
pub fn zeigen(self, fenster: &NSWindow, fertig: impl Fn(bool) + 'static) {
    let _griff = self.zeigen_mit_wahl(fenster, move |stelle, _fuer_alle| fertig(stelle == 0));
}
```

The stated condition ("more than two") and the load-bearing one ("the confirming button sits at
position 0") are not the same set. The counter-example is in the tree and is the sheet this
Circle is about:

```rust
// crates/krk-ui/src/appkit/blaetter/loeschbestaetigung.rs:109-114
fn schaltflaechen(vorgang: &str) -> [Schaltflaeche<'_>; 2] {
    [
        Schaltflaeche::neu("Abbrechen", Taste::Eingabe, Wirkung::Liegenlassen),
        Schaltflaeche::neu(vorgang, Taste::EingabeMitBefehl, Wirkung::Ausfuehren),
    ]
}
```

Two buttons, so the stated condition permits `zeigen`; the executing button is at position 1, so
`fertig(stelle == 0)` would report "confirmed" for **Abbrechen**. That file avoids the trap by
going through `zeigen_mit_wahl` with its own `AUSFUEHRENDE_STELLE = 1`, and nothing in the code
or in a probe says why it must.

**Checked, and today it holds:** all five callers of `Blatt::zeigen` — `pfadeingabe:73`,
`zeilennummer:72`, `namenseingabe:118`, `suche:145`, `stapelumbenennen:436` — build their sheet
with `Blatt::neu`, whose `standardschaltflaechen` puts the confirming button first. No caller is
wrong today.

**Why it is nevertheless the same class as `260817-1242`.** That record's finding was: the
guard sent `NSAlertFirstButtonReturn` because the only sheets holding a guard came from
`Blatt::neu`, and the assumption stopped being true the moment one did not. `Blatt::zeigen`
carries the identical assumption for the identical reason, and the module header of the same
file now says that questions of this kind are answered once ("Es sind zwei Fragen dieser Art,
und seit dem 260818 ist jede einmal beantwortet"). There is a third.

## Direction

The third question is **"which button executes what the sheet asks about"**, and it is not the
same as either of the two now answered once. It is not `bestaetigungsstelle`: on the delete
confirmation Return sits on "Abbrechen" deliberately, so "carries Return" and "is the affirmative
answer" are opposite there. It is not `abbruchstelle` either.

Two ways, and the first is the smaller one:

1. **Name the real precondition and hold it with a probe.** `zeigen`'s doc says "the confirming
   button must be the first" instead of "at most two buttons", and a count in the style of
   `die_dateien_mit_blaettern_nennen_ihre_liegenlassende_schaltflaeche` asserts that every caller
   of `Blatt::zeigen` builds its sheet with `Blatt::neu`. Cheap; leaves the assumption in place
   but makes it visible and breakable.
2. **Derive it, like the other two.** A pure function `ausfuehrende_stelle` over
   `Wirkung::Ausfuehren` — the first executing button — gives the right answer for every sheet in
   the tree today: position 0 for the five `Blatt::neu` sheets and for `ungesichert` (`Sichern`),
   position 1 for `loeschbestaetigung`. `zeigen` would then read the sheet instead of assuming
   its shape, `loeschbestaetigung::AUSFUEHRENDE_STELLE` would become that function's result
   rather than a hand-written `1`, and the file would answer three questions once each instead of
   two once and one twice.

Both are user choices about how far the derivation goes; neither is urgent, because no caller is
wrong at `a4d8211`.
