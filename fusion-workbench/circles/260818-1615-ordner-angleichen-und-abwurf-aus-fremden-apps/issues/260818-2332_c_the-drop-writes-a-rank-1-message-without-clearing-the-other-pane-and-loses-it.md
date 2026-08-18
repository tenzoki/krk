# The drop writes a rank-1 status message without clearing the other pane, and the message is then never seen

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `crates/krk-ui/src/appkit/tabelle.rs:3016-3019` (`abwurf_pruefen`, the write), `:2734-2737` (`befehlsantwort_zeigen`), `:2754-2766` (`befehlsantwort_loeschen`), `crates/krk-ui/src/appkit/anwendung.rs:2905-2907` (the both-sides clearing in `kommando_ausfuehren`), `crates/krk-ui/src/appkit/statuszeile.rs:597-615` (`zeile`, active side first within a rank), `:235-242` (`Rang::ALLE`, `Befehlsantwort` is rank 1); spec `shared/planning/260818-1510_*_spec-…` §C7, second acceptance criterion

---

## What is wrong

`DateifensterQuelle::abwurf_pruefen` writes the C7 message through `befehlsantwort_zeigen`, which fills the **rank-1** slot (`Rang::Befehlsantwort`) of the pane the pointer is over. Every other writer of that slot arrives through `Anwendungsdelegierter::kommando_ausfuehren`, and that path first clears the slot **on both panes**:

```rust
// anwendung.rs:2905-2907
for seite in Fensterseite::ALLE {
    self.dateifenster(seite).quelle().befehlsantwort_loeschen();
}
```

The drop is the third writer of that field and the only one that does not. `statuszeile::zeile` scans rank first and, within a rank, the **active** side before the other one:

```rust
// statuszeile.rs:597-605
for rang in Rang::ALLE {
    for seite in [aktiv, aktiv.andere()] {
        ...
        if let Some(text) = quellen(seite).text(rang) { return Some(Meldung { … }); }
```

So whenever the active pane still holds a command answer from the last keystroke, and the user drags a promise-file item over the **other** pane, the drop's message loses the rank-1 contest and the status line keeps showing the older answer. The pointer shows the system's refusal symbol and nothing says why — which is exactly the state C7 was written against ("der Unterschied zwischen ‚KRK kann das nicht' und ‚KRK sagt, dass es das nicht kann'").

## How to reproduce (user work, this is a drag)

1. Focus the left file pane and press a key that leaves an answer in the status line (any refused command does).
2. Drag a Mail attachment over the **right** file pane without pressing a key in between.
3. The status line keeps the left pane's answer. The C7 sentence "die Quelle liefert keine Datei auf dem Datenträger" is written into the right pane's field and never rendered.

A drop over the **active** pane is unaffected: it overwrites that pane's own slot.

## Second-order effect

`abwurfmeldung`'s de-duplication then works against a message that was never displayed. `gemeldeter_abwurfgrund` is set to `Some(KeineDatei)` on the first pointer move, so a second pass over the same list in the same period writes nothing either. The state clears itself at the next keystroke, so it is not permanent — but for the whole span between two keystrokes the user gets silence.

## Why the affected acceptance criterion cannot catch it early

C7's criterion is user acceptance work ("Die Statuszeile nennt dabei den Grund, und die Meldung geht in das Dateifenster, über dem der Zeiger stand"), and the natural way to test it is to drag over the pane one has just been working in — the one case that works.

## Suggested direction, not a prescription

The cheapest fix that matches the existing rule is to give the drop the same both-sides clearing the keystroke path has: clear `befehlsantwort` on both panes before writing, in `abwurf_pruefen`, at the one point where a message is actually written (i.e. inside the `if let Some(meldung)` arm, not on every pointer move). That keeps one deletion rule rather than adding a second, and it keeps `befehlsantwort_loeschen`'s doc comment true by raising its caller count from two to three with a named third reason.

The alternative — giving the drop its own rank — would add a seventh rank for one message and is worse.

Whichever is chosen, `befehlsantwort_zeigen`'s doc comment at `tabelle.rs:2739-2753` must be corrected: it names two callers of the deletion rule and the drop has already made the writer set three.

---
Resolved: Kein zweiter Raeumweg, sondern einer mit zwei Zugaengen: die Schleife aus kommando_ausfuehren ist als befehlsantwort_beidseitig_loeschen herausgezogen, und der Abwurf bekommt einen achten, schwach gehaltenen Rueckruf. Die Raeumkante ist das Some von abwurfmeldung, also genau so oft wie eine Meldung geschrieben wird. befehlsantwort_loeschen behaelt zwei Aufrufstellen statt der im Datensatz vorhergesagten drei.
