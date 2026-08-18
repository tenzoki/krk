The keymap now carries two enumerations of the `opt+cmd` row, and the older one is short by two

---

`resources/default-keymap.toml` names the members of the `opt+cmd` row twice, fifteen lines
apart, and the two lists disagree.

**The older one, at `:266-272`,** was reflowed by this round to take `opt+cmd+s` in:

> Die opt+cmd-Reihe traegt in diesem Programm, was einen Ordner herstellt oder liefert:
> opt+cmd+c … opt+cmd+g … opt+cmd+s … opt+cmd+l, opt+cmd+d, opt+cmd+b, opt+cmd+left und
> opt+cmd+right schalten Bereiche ein und aus.

Eight combinations. The file binds **eleven**. Missing are `opt+cmd+e` ("Editor schließen",
`:790`) and `opt+cmd+n` ("Weitere Instanz starten", `:1012`), and neither of them "stellt einen
Ordner her oder liefert ihn" nor "schaltet einen Bereich ein und aus", so the sentence's premise
does not cover them either.

**The new one, at `:293-296`,** is complete and correct:

> Belegt sind in der opt+cmd-Reihe zehn Kombinationen, naemlich opt+cmd+b, opt+cmd+c, opt+cmd+d,
> opt+cmd+e, opt+cmd+g, opt+cmd+l, opt+cmd+left, opt+cmd+n, opt+cmd+o und opt+cmd+right.

Counted against the `tasten` lines of the file at `71413c3`, those ten plus `opt+cmd+s` are
exactly the eleven bound combinations. (`opt+cmd+f` appears at `:423` in prose only, as a free
alternative, and is correctly absent from both lists.)

So the tree now holds two hand-maintained lists of one set. Nothing holds either of them, one is
already wrong, and the next combination added to the row has to find both.

---

**Severity:** Low. Prose only; no behaviour depends on either list. The shortfall in the older
list predates this round — what this round added is the second list beside it.
**Found by:** coderev, enumerating the `opt+cmd` bindings from the `tasten` lines and comparing
both comment blocks against the result.
**Affects:** `resources/default-keymap.toml:266-272` and `:293-296`
**Related:** `git:48bb57f` "der Kopf des Norton-Blocks verspricht nur noch, was der Block haelt"
— the same shape of defect, fixed once already in this file.
**Tree state:** `71413c3`
**Domain:** data

## What a fix would have to do

Leave **one** enumeration in the file and have the other cite it. Which one survives is a
judgement about where a reader looks: the older block sits at the head of the row and reads as
the row's own documentation, the newer one sits at the entry that was just added and is complete.

The sentence "Die opt+cmd-Reihe traegt in diesem Programm, was einen Ordner herstellt oder
liefert" needs the same pass whichever list stays: with `opt+cmd+e` and `opt+cmd+n` in it, the
row has three purposes and not one, and a head that promises one is the promise `48bb57f` took
out of the Norton block.

**Filed by:** coderev
