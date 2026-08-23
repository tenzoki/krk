# Does a `**Cross-references:**` line between records write the marker in the star form?

---
**Domain:** code
**Status:** open
**Filed by:** analyst
**Cross-references:** `shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md` (the answered question this one carves out of), `shared/issues/260817-1130_*_die-sternform-fuer-zitate-gilt-seit-dem-260815-und-drei-runden-schreiben-den-marker-aus.md` (the compliance defect, four rounds of it), `circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/issues/260818-0013_*_two-decision-records-cite-each-other-with-markers-neither-of-them-carries-any-more.md` (the two instances that raised it), `CLAUDE.md` `## Bindende Grundlage: die Entscheidungsdatensätze` (the location rule)

---

## Question

The project settled the star form on 260815-1230
(`shared/decisions/260815-1145_*_schreiben-zitate-im-code-den-marker-aus-oder-die-sternform.md`,
`_i_`). That answer names its scope in words: "Umgestellt wird, was heute gilt: `crates/`,
`xtask/`, `CLAUDE.md`, die Circle-Datensätze und die Spec- und Plandateien unter `planning/`."
It names what it leaves alone just as plainly, by pointing at the location rule in `CLAUDE.md`:
`history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` and `spikes/` keep
the marker they carried, **per file by its location, not per paragraph**.

`decisions/` is on the frozen side. So a `**Cross-references:**` line in a decision record is
exempt from the star form today — and it is the one line in such a file that is not a record of
anything. It is a pointer, written to be followed, and it decays exactly like the pointers the
star form was chosen to protect. The location rule is what makes the exemption decidable, and
the reason it was chosen over a per-paragraph rule is that a per-paragraph rule "wäre nicht
entscheidbar und lieferte bei jedem Durchgang einen anderen Bestand". Any answer here has to
buy its exception without buying that problem back.

The question is due now because the decay is measured, not predicted. Of the three targets named
in the `**Cross-references:**` line of
`shared/decisions/260802-0842_*_loeschen-papierkorb-oder-endgueltig.md`, all three had moved:
the Circle record `_t_`→`_b_`, the F-key decision `_a_`→`_i_`, the round-1 defect `_o_`→`_c_`.
Three dead pointers in one line, and the record that reported them
(`…/issues/260818-0013_*_…`) had seen one of the three.

## Options

1. **Leave the location rule as it stands.** A `**Cross-references:**` line keeps whatever marker
   it was written with, like every other line in a frozen store.
   - Pros: one rule, decidable by location alone, no exception to remember. The header of a record
     then also says which state its neighbours were in when the record was filed, which is a real
     piece of information about the moment of filing.
   - Cons: it is measurably not true that this information gets used, and it is measurably true
     that the pointers rot: three of three in the line above. A reader following a dead pointer
     does not learn "this was `_a_` in August", they learn nothing and go looking.
2. **Carve out the `**Cross-references:**` line by name** — the star form applies to it in every
   store, frozen or not; everything else in a frozen file keeps its wording.
   - Pros: the exception is still decidable by inspection, because it names one labelled line
     defined by the shipped record template rather than a class of prose. It fixes the one line in
     these files whose whole purpose is to be followed. It also matches what the header already is:
     `**Status:**`, `**Domain:**` and the resolution lines at the foot are all maintained after
     filing, so the header is not frozen in practice either.
   - Cons: a second rule beside the location rule, and the location rule was chosen precisely to
     avoid a second rule. Someone has to convert the existing lines once.
3. **Carve out every pointer in a frozen store**, star form wherever a citation is a pointer and
   written-out wherever the marker is the statement.
   - Pros: the widest fix, and it is the distinction the 260815 answer already draws inside its own
     scope ("ein Zeiger auf eine Datei verliert nichts, eine Aussage über einen Zustand verliert
     ihren Inhalt").
   - Cons: this is the per-paragraph rule under another name. `CLAUDE.md` rejected it for being
     undecidable, and rewriting the body of a record of a state is the thing the location rule
     exists to forbid.

## Constraints

- Whatever is chosen must stay decidable by looking at a file, not by judging a paragraph. That is
  the property `CLAUDE.md` bought with the location rule and it is not for sale here.
- The exception inside the 260815 answer survives untouched in every option: where the marker **is**
  the statement — a findings table with columns "zitiert" and "ist", a sentence about a state
  change — the letter stays.
- No answer holds without a check. The 260815 answer was accepted deliberately without one, and
  `shared/issues/260817-1130_*_…` is the record of what that cost: four rounds and several
  operators wrote the marker out again, and nobody noticed until a pass went looking. An answer
  here that also ships without a check should say so knowingly rather than by omission.

## Recommendation

Option 2. It is the smallest change that removes the actual failure, and it stays on the decidable
side of the line: "the `**Cross-references:**` line" is a label in a shipped template, so a pass
can find every instance with one grep and never has to read a sentence to classify it. Option 3
buys a little more and pays for it with the undecidability the project already refused once;
option 1 keeps a piece of information that nothing in four rounds has been shown to use, at a price
that is now measured at three dead pointers in a single line.

Two things belong to the answer rather than after it. **The conversion is small**: the store holds
roughly thirty decision records, one header line each. **And the check is the open half** — a
count probe or an `xtask` target that resolves every `_*_` citation against the file store would
cover this question and the 260815 one together, which is the only version of either answer that
has been shown to hold.

**Written in anticipation of option 2, and to be reverted if the user chooses otherwise:** the two
lines that raised this question were corrected to the star form on 260818-0201 rather than to the
current letter, on the ground that the current letter is known to be wrong again at the next
transition. That is a repair, not a ruling; the ruling is this record.

---
Answered:
Implemented:
Deferred:
Superseded by:

---

**Abgleich 260823-1336: sieben neue tote Zeiger an einem einzigen Tag.** Die Sitzung `260823-0442`
hat in ihren eigenen Datensätzen sieben Verweise mit ausgeschriebenem Marker hinterlassen, deren
Ziel dieselbe Sitzung anschließend umbenannt hat. Keiner davon löst heute auf:

- `shared/decisions/260820-1034_a_wie-kommt-eine-taste-zum-umschalten-*` (heute `_i_`), zitiert in
  der `Answered:`-Zeile desselben Datensatzes und im `Resolved:`-Vermerk von
  `shared/issues/260820-1034_c_cmd-e-bleibt-in-der-vorschau-wirkungslos-*`.
- `shared/decisions/260823-1137_o_holt-der-rueckweg-von-cmd-e-die-vorschau-*` (heute `_i_`).
- `shared/issues/260820-1034_o_cmd-e-bleibt-*` und `…_p_cmd-e-bleibt-*` (heute `_c_`).
- `shared/issues/260820-1034_p_f4-setzt-den-fokus-*` (heute `_c_`).
- `shared/issues/260823-0730_o_drei-prosastellen-*` und `260823-0733_o_die-probe-zur-editorfortsetzung-*`
  (beide heute `_c_`), zitiert unter anderem aus `shared/reviews/260823-1040-coderev-cmd-e-wird-der-rundweg.md`.

Die Erhebung ist mechanisch: alle Verweise der Form `<speicher>/<art>/<stempel>_<marker>_<thema>.md`
aus den Datensätzen, Durchsichten und Protokollen dieser Sitzung gegen den Dateibestand geprüft.
Fünf der sieben stehen nicht in einer `**Cross-references:**`-Zeile, sondern in Fließtext, in einem
`Resolved:`-Vermerk und in einer `Answered:`-Zeile. **Das ist Sachstand zu dieser Frage und keine
Antwort auf sie**: der Zerfall trifft breiter als die eine Kopfzeile, um die die Frage gestellt ist,
und jede Antwort, die allein `**Cross-references:**` umstellt, ließe fünf dieser sieben stehen.
