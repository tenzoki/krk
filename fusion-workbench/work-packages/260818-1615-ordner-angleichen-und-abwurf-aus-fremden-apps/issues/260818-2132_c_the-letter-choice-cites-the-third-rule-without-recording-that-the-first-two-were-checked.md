The letter choice for `opt+cmd+s` cites the third rule without recording that the first two were checked

---

The new block in `resources/default-keymap.toml:288-291` justifies the letter:

> Der Buchstabe folgt der dritten Wahlregel der Auslieferungsbelegung, dem Anfangsbuchstaben des
> deutschen Verbs: s wie "stellen".

The citation is accurate. The three rules are in
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`,
they bind every later addition to the file by their own words, and the third is indeed "der
Anfangsbuchstabe des deutschen Verbs".

**The three rules are ordered, and the order is the load-bearing part.** Rule 2 opens with
"Sonst, wo Norton Commander oder Total Commander eine Form haben, die auf dem Mac frei ist", and
rule 3 with "Sonst". Rule 3 therefore applies only once rules 1 and 2 have been checked and come
up empty. Nothing in the file records that check for this command, and putting the other panel
on the current folder is a Norton-lineage operation with a Norton-lineage precedent — this is the
one place where rule 2 is most likely to bite, not least likely.

The same block records exactly this kind of check for the other half of the choice — "opt+cmd+s
ist ab Werk frei; am 260818 gegen alle tasten-Zeilen dieser Datei nachgezaehlt". The freeness of
the combination is documented as measured; the applicability of the rule that chose it is not.

**Uncertainty, stated rather than papered over:** this reviewer does not know Total Commander's
binding for this operation from a source it can cite, and has not verified whether rule 2 would
in fact have produced a different combination. The finding is that the precondition was not
recorded, not that the outcome is wrong.

---

**Severity:** Low. The combination works, is free, and is defensible. What is missing is one
sentence of record in a file whose whole habit is to carry that sentence.
**Found by:** coderev, resolving the citation to the decision record and reading the three rules
in their stated order.
**Affects:** `resources/default-keymap.toml:288-291`
**Related:** `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2300_i_auslieferungsbelegung-der-39-frei-gewaehlten-kombinationen.md`
(the three rules, "gelten mit angenommen und binden künftige Ergänzungen der Datei"); the spec of
this round justifies the same letter differently — "`s` liest sich als „selber Ordner"" — which
is a third reading of one choice.
**Tree state:** `71413c3`
**Domain:** data

## What a fix would have to do

Add the two lines the rule order asks for: that the Mac knows no shortcut for this operation
(rule 1), and what the Norton and Total Commander form is and why it was not taken (rule 2) — or
that there is none. If the Norton form turns out to be free on the Mac, rule 2 outranks rule 3
and the combination itself is back open; the shaper's own list of free alternatives already names
`ctrl+cmd+left` and `ctrl+cmd+right`.

While there, settle which reading of the letter the tree keeps. "s wie stellen" (rule 3, the
file) and "s liest sich als selber Ordner" (the spec) cannot both be the reason.

**Filed by:** coderev

---
Resolved: Regel 1 und Regel 2 sind gepruefte Leerstellen und stehen jetzt als solche im Kommentar. Regel 1: der Finder hat kein zweites Dateifenster, also keinen solchen Befehl. Regel 2: der Total Commander erreicht das ueber ctrl+left und ctrl+right, und das Paar ist auf dem Mac doppelt vergeben (Schreibtischwechsel des Systems, und in dieser Datei die Bereichsbreite bei :604 und :609); Regel 2 verlangt die Form selbst und nicht eine Mac-Anpassung. Regel 3 entscheidet, opt+cmd+s bleibt. Die Lesart "s wie selber Ordner" steht jetzt ausdruecklich als Merkhilfe und nicht als Begruendung.
