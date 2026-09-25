The loeschwarnung module header still says `loeschfrage` "will fall" and it fell in the same commit

---
`crates/krk-ui/src/kommandos/loeschwarnung.rs:166-167` reads "`operationen::loeschfrage`, der
Wortlaut des endgueltigen Loeschens, **faellt** mit diesem Loeschweg weg." The function was
deleted by `82707ef`, the same commit that rewrote the paragraph 88 lines below it in the same
header.

---

**Severity:** Low. Prose only. It matters because the sentence names a symbol that no longer
exists and states its removal as still to come, in the module that this round made the single
home of the delete texts.
**Found by:** coderev, review `reviews/260817-2243-coderev-bundle-d-the-removal.md`
**Affected:** `crates/krk-ui/src/kommandos/loeschwarnung.rs:166-167`
**Tree state:** `f7a85c1`
**Domain:** code

## Measured

```
$ grep -rn "loeschfrage" crates/krk-ui/src/kommandos/operationen.rs
(no match)
$ sed -n '160,168p' crates/krk-ui/src/kommandos/loeschwarnung.rs
//! # Warum die Texte der Loeschfrage eigens dastehen
//!
//! Nach dieser Runde kennt KRK genau einen Loeschweg, und er fragt vorher
//! genau einmal nach. Ein Wortlaut, der an zwei Stellen entstuende, waere zwei
//! Wahrheiten ueber dieselbe Frage; deshalb steht er hier und nicht im Blatt,
//! das ihn zeigt, und nicht in [`super::operationen`], das die Texte aller
//! uebrigen Dateioperationen traegt. `operationen::loeschfrage`, der Wortlaut
//! des endgueltigen Loeschens, faellt mit diesem Loeschweg weg.
```

The same commit rewrote `:254-256` of this file from "**`f8` kommt erst mit Buendel D dazu**" to
"**`f8` ist mit Buendel D dazugekommen**", so the tense of the header was under the executor's
hand in this very edit and this one sentence was left behind.

## Relation to step 15

Step 15 of the plan sweeps the comments of the tree with
`grep -rniE "endgueltig|endgültig" --include="*.rs" crates`. The sentence contains
"endgueltigen" and that search will reach it. It is filed here rather than left to step 15
because it is not stale comment prose about the removed command in general: it is a reference to
a symbol that Bundle D itself deleted, in a file Bundle D itself edited.

## Direction

Past tense, and drop the dangling symbol reference or mark it as gone: "`operationen::loeschfrage`,
der Wortlaut des endgueltigen Loeschens, ist mit jenem Loeschweg weggefallen."

---
Resolved: `crates/krk-ui/src/kommandos/loeschwarnung.rs:167` steht jetzt im Perfekt und
nennt den Loeschweg als vergangenen: „`operationen::loeschfrage`, der Wortlaut des
endgueltigen Loeschens, ist mit jenem Loeschweg weggefallen." Wortlaut wie unter
`## Direction` vorgeschlagen.

**Derselbe Satz ist die Stelle, die
`issues/260818-0026_*_the-sweep-of-step-15-reports-33-remaining-lines-and-the-search-returns-34.md`
als falsch eingeordnet meldet** — jener Datensatz zaehlt `loeschwarnung.rs:167` unter den
datierten Rueckblicken, und mit dieser Korrektur trifft die Einordnung zu. Die beiden
Datensaetze meinen bei dieser Zeile dasselbe; ihr uebriger Gegenstand ist verschieden (dort
die Zahl 33 gegen 34 im Sitzungsbericht, hier der haengende Symbolverweis).

`make check` — exit 0.
