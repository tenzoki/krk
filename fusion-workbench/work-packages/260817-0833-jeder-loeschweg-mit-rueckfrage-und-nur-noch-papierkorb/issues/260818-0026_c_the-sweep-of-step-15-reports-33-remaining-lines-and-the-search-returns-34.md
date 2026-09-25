The sweep of step 15 reports 33 remaining lines and the search returns 34

---
Commit `522cf51` and the session record `history/260817-2356-coder-e15-kommentare-und-claude-md.md`
both state that after the pass `grep -rniE "endgueltig|endgültig" --include="*.rs" crates` returns
**33 lines**. Run at the same tree state it returns **34**. The "before" figure of 51 is correct.

---

**Severity:** Low. The classification behind the number is sound — all 34 lines fall into the six
classes the record names, and every one of them was read individually in this review. What is
wrong is only the count, and it is wrong in a durable record that a later pass will use as its
baseline: the next sweep that gets 34 will look for a line that arrived since, and there is none.

**Found by:** coderev, review `reviews/260818-0024-coderev-bundle-e-the-prose-and-the-records.md`
**Affected:** commit message `522cf51`, `history/260817-2356-coder-e15-kommentare-und-claude-md.md`
**Tree state:** `da716c1`
**Domain:** code

## Measured

```
$ git archive 8f556ed crates | tar -x -C "$T" ; cd "$T"
$ grep -rniE "endgueltig|endgültig" --include="*.rs" crates | wc -l
      51
$ git archive 522cf51 crates | tar -x -C "$T2" ; cd "$T2"
$ grep -rniE "endgueltig|endgültig" --include="*.rs" crates | wc -l
      34
```

`git diff --stat 522cf51..da716c1 -- crates` is empty, so the working tree at `da716c1` gives the
same 34.

## The 34, against the six classes the record names

Every line was opened. The classification holds without exception; the count does not.

| Class from the record | Lines |
|---|---|
| changed in `522cf51`, still carrying the word | `tests/belegung.rs:292`, `:297`; `operation/loeschen.rs:4`; `belegungsmodell.rs:905`; `loeschbestaetigung.rs:45` |
| correct since bundle D | `appkit/ereignisse.rs:307` |
| dated retrospectives | `rueckschritt.rs:88`; `belegungsmodell.rs:1183`, `:1184`; `anwendung.rs:4481`, `:4816`, `:5691`, `:6446`; `loeschwarnung.rs:167`, `:254` |
| filenames of decision records | `tests/belegung.rs:291`; `operation/loeschen.rs:6`; `loeschzielbefund.rs:134`; `arbeitsbaum.rs:162`; `umfang.rs:150`; `loeschbestaetigung.rs:74` |
| the word in its ordinary sense | `verzeichnis/modell.rs:464`; `zettelmodell.rs:454`; `anwendung.rs:844`; `hinweis.rs:31`; `papierkorb.rs:183`; `operation/loeschen.rs:57` |
| the probe that must carry the withdrawn identifier | `tests/belegung.rs:1622`, `:1625`, `:1645`, `:1646`, `:1660` |
| test fixture, filed separately as `issues/260817-2355_o_*` | `loeschbestaetigung.rs:173`, `:180` |

Three of the citations in the record are one line low against the committed tree —
`loeschbestaetigung.rs:73`, `:172`, `:179` are `:74`, `:173`, `:180` after the same commit's own
edits to that file. Same cause as the count: the numbers were taken before the file was written.

**One row of that table is the wrong class**, and it is the line the previous review had routed
into this step. `loeschwarnung.rs:167` is listed as a dated retrospective "in den Schritten 12 bis
14 geschrieben und richtig". It is neither: it says `operationen::loeschfrage` "**faellt** mit
diesem Loeschweg weg" in the future tense, and the symbol fell in `82707ef`. The bundle-D review
had written "**L2 goes with step 15 as it stands** — its search already reaches the line"
(`reviews/260817-2243-coderev-bundle-d-the-removal.md:206`). The search did reach it; the
classification sent it back untouched. The defect keeps its own open record,
`issues/260817-2243_o_the-loeschwarnung-module-header-still-says-loeschfrage-will-fall-and-it-fell-in-the-same-commit.md`,
and is not refiled here — but it means the claim "keine der 33 trifft mehr eine falsche Aussage"
does not yet hold either.

## Direction

Correct the number to 34 in the session record. The commit message is written and stays as it is;
the record is the surface a later pass reads. Correct the three line citations at the same time,
or drop the line numbers there — the file names alone carry the classification.

---
Resolved: Beide Teile des Datensatzes sind abgetragen.

**Die Zahl.** `history/260817-2356-coder-e15-kommentare-und-claude-md.md` sagt jetzt „Nach
dem Durchgang liefert dieselbe Suche 34 Zeilen" statt 33. Nachgemessen am Arbeitsbaum und
an `522cf51`: `grep -rniE "endgueltig|endgültig" --include="*.rs" crates | wc -l` liefert
beide Male 34. Die Commit-Nachricht von `522cf51` bleibt, wie sie ist. Die drei um eine
Zeile zu niedrigen Zitate im selben Bericht sind mitgezogen:
`loeschbestaetigung.rs:73` → `:74`, `:172` → `:173`, `:179` → `:180`.

**Die falsche Klasse.** `loeschwarnung.rs:167` ist nicht umgeordnet, sondern richtiggestellt
worden: die Zeile steht jetzt im Perfekt („ist mit jenem Loeschweg weggefallen"), womit die
Einordnung „datierter Rueckblick, richtig" zutrifft und keine der 34 Zeilen mehr eine
falsche Aussage traegt. Der eigene Datensatz dieser Zeile,
`issues/260817-2243_c_the-loeschwarnung-module-header-still-says-loeschfrage-will-fall-and-it-fell-in-the-same-commit.md`,
traegt die Aenderung und verweist hierher zurueck; die beiden meinen bei dieser einen Zeile
dasselbe.

Nicht nachgezogen sind die uebrigen Zeilennummern des Berichts. Er ist die Aufzeichnung
eines Standes zu `522cf51`, und die Nummern verschieben sich mit jedem spaeteren Commit,
dieser hier eingeschlossen (`anwendung.rs:4481` liegt nach der Korrektur der vier
Doc-Kommentare vier Zeilen tiefer). Nachgezogen ist allein, was schon beim Schreiben falsch
war.

`make check` — exit 0.
