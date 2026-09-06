The paragraph that dropped the `Art` parameter still counts two `Angaben` in its next sentence

---

`285b58f` removed `art: Art` from `loeschen_nach_rueckfrage`, leaving the command exactly one
piece to bring along. Its own doc comment says so in one sentence and then, two lines later,
speaks of "die zwei Angaben des Befehls". The two sentences stand in the same paragraph and
contradict each other.

---

**Severity:** Low. Prose only, no behaviour. Filed because the sentence is the justification for
keeping the cut between `in_den_papierkorb` and `loeschen_nach_rueckfrage`, and it is now
arguing from a count the same commit made false.
**Found by:** coderev, review `reviews/260818-0410-coderev-bundle-f-die-messungen-und-der-waechter.md`
**Affected:** `crates/krk-ui/src/appkit/anwendung.rs:4665-4668`
**Tree state:** `a4d8211`
**Domain:** code

## What stands in the tree

```rust
// crates/krk-ui/src/appkit/anwendung.rs:4665-4668
/// **Der Schnitt zu [`Self::in_den_papierkorb`] bleibt trotzdem**, und der
/// Befehl bringt weiterhin ein Stueck mit: die Beschriftung der zweiten
/// Schaltflaeche. Zusammengelegt truege eine Funktion die Stufenregel und
/// die zwei Angaben des Befehls in einem Rumpf.
```

"ein Stueck" in the first sentence, "die zwei Angaben" in the second. Read against the code, the
first is right: `in_den_papierkorb` (`:4476-4478`) passes exactly one argument, the button
label, and the signature is `fn loeschen_nach_rueckfrage(&self, schaltflaeche: &str) -> bool`
(`:4673`). The second sentence is the pre-`285b58f` count, when `art: Art` stood beside the
label.

The same doc comment three paragraphs up already carries the corrected count for the sibling
function: `in_den_papierkorb`'s doc (`:4453-4455`) says "Hier steht allein das eine Stueck".

## Direction

Replace "die zwei Angaben des Befehls" with "die eine Angabe des Befehls". Note that the
argument the sentence makes gets weaker with the correction, not stronger — one label is a
thinner reason for a separate function than two pieces were. Whether the cut still earns its
keep is a separate question and not this record's; `schaltflaeche` has one caller and one value,
which is the same shape the commit used to justify dropping `art`.

---
Resolved: Behoben, wie unter Direction verlangt. Der Doc-Kommentar von `loeschen_nach_rueckfrage` (`crates/krk-ui/src/appkit/anwendung.rs`) sagt jetzt "die eine Angabe des Befehls" statt "die zwei Angaben des Befehls"; damit stimmt er mit seinem eigenen ersten Satz ("bringt weiterhin ein Stueck mit") und mit der Signatur `fn loeschen_nach_rueckfrage(&self, schaltflaeche: &str) -> bool` ueberein. Dass das Argument fuer den Schnitt mit der Berichtigung schwaecher wird, ist im Datensatz benannt und bleibt eine eigene Frage; sie ist hier nicht beantwortet. Geprueft: cargo test/clippy/fmt/doc -p krk-ui je Exit 0.
