The redundant read that the path comparison allows is not "folgenlos" — it drops the target tab's selection and scroll position

---

The doc comment of `ordner_angleichen` (`crates/krk-ui/src/appkit/anwendung.rs:3305-3310`)
justifies comparing the two folders without `canonicalize`:

> was ohne sie durchrutscht, ist derselbe Ordner unter zwei Pfaden, und sein Ausgang ist ein
> Lesevorgang, der denselben Inhalt noch einmal liest. Das ist folgenlos, und deshalb faellt der
> Rest der Regel auf die harmlose Seite.

The direction of the argument is right and the safe half of it holds: two **different** folders
can never share one `PathBuf`, so no false "already there" is possible, and the comparison can
only err towards reading. **The claim that the read is without consequence is the part that does
not hold.**

`ordner_lesen` goes through `Tabliste::ordner_setzen` (`crates/krk-ui/src/tabs.rs:…`), which
does not re-read the standing tab — it replaces it:
`self.tabs[stelle] = Tabinhalt::aus_zustand(&zustand)` with `zustand.auswahl = auswahl`, and
`ordner_angleichen` passes `None`. Sortierung, `verstecke_ausgeblendet`, `tief`, `inhalt` and the
filter text are carried over by hand; **the selection and the scroll position are not.** The tree
knows this and says so at `Tabliste::aktiven_neu_lesen`, whose whole reason for existing is that
`ordner_setzen` with the same folder loses both ("Gegenueber `Tabliste::ordner_setzen` mit
demselben Ordner sind es zwei Unterschiede, und beide sind der Grund, aus dem diese Methode
besteht").

So in the slipped-through case the command does exactly what C1's fifth criterion promises it
will not do: "Auswahl und Bildlaufposition dort bleiben stehen."

Ways the two spellings can differ while naming one folder, all reachable in this program:
`/tmp` against `/private/tmp`, a bookmark that stores a symlinked path against the same folder
reached by walking down to it, and a case difference on the case-insensitive volume that is this
project's default target.

---

**Severity:** Low. The outcome is a lost selection in the pane the user was not looking at, in a
case that needs two spellings of one folder to arise. Nothing is lost from disk and no state goes
inconsistent.
**Found by:** coderev, following `ordner_lesen` into `ordner_setzen` to check the claim.
**Affects:** `crates/krk-ui/src/appkit/anwendung.rs:3305-3310` (the doc comment's third paragraph)
**Related:** `crates/krk-ui/src/appkit/tabelle.rs`, `DateifensterQuelle::neu_lesen` and
`Tabliste::aktiven_neu_lesen` — the pair that exists because this difference is real.
**Tree state:** `71413c3`
**Domain:** code

## What a fix would have to do

The cheap and honest fix is to the sentence, not the code: say that the slipped-through case
costs the target tab its selection and scroll position, and that this is accepted against two
`canonicalize` calls per keypress and an error exit. The decision to skip `canonicalize` stands
either way — what does not stand is a doc comment that reasons the cost down to zero.

If the cost is judged too high instead, the equality branch is not the place to fix it: the read
would have to go through a path that preserves the selection, and that path
(`aktiven_neu_lesen`) answers a different question.

**Filed by:** coderev

---
Resolved: Der Doc-Kommentar nennt den Preis jetzt beim Namen: ordner_lesen geht durch Tabliste::ordner_setzen, das den stehenden Tab ersetzt statt ihn neu zu lesen, und Auswahl und Bildlaufposition sind die zwei Groessen, die dabei nicht mitgehen. Der Verzicht auf canonicalize bleibt, wird aber nicht mehr als folgenlos begruendet; die drei Wege, auf denen zwei Schreibweisen desselben Ordners entstehen, stehen daneben.
