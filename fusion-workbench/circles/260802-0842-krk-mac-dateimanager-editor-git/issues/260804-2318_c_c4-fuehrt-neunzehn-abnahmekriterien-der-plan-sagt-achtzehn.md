C4 führt neunzehn Abnahmekriterien, der Plan sagt achtzehn

---

Das Abnahmekriterium von S16 lautet: "die **achtzehn** Abnahmekriterien aus C4, soweit sie an der Oberfläche hängen, sind im laufenden Bündel einzeln nachweisbar." Gezählt am 260804-2318 führt C4 des Specs **neunzehn** Zeilen der Form `- [ ]`. Dieselbe Zahl steht ein zweites Mal im Nachzugsvermerk vom 260804-1832 im Kopf des Plans ("zwei seiner achtzehn Kriterien").

---

## Der Nachweis

```
awk '/^### C4:/,/^### C5:/' planning/260802-1036_o_spec-navigator-geruest.md | grep -c '^- \[ \]'
19
```

Dieselbe Zählung gegen den Stand `e43316d`, also vor dem Aufräumdurchgang vom 260804-2318, liefert ebenfalls **19**. Der Durchgang hat ein Kriterium umformuliert, aber keines hinzugefügt oder entfernt; die Abweichung ist älter als er.

## Warum das mehr ist als eine schiefe Zahl

Der Defekt `issues/260804-1832_c_die-zahl-der-c4-abnahmekriterien-steht-im-plan-auf-sechzehn-und-im-spec-auf-achtzehn.md` ist genau dafür angelegt und am 260804-1832 als geschlossen vermerkt worden. Er hat die Zahl von sechzehn auf achtzehn gezogen und dabei um eins verfehlt. Eine Abnahmezahl, die zweimal nachgezogen wurde und beim zweiten Mal wieder danebenliegt, taugt als Prüfgröße nicht: wer S16 abnimmt, hakt achtzehn Punkte ab und hält den neunzehnten für nicht verlangt.

Die naheliegende Auflösung ist, die Zahl aus dem Kriterium zu nehmen. Sie zählt etwas, das an einer anderen Stelle steht und sich dort ändert, und sie ist damit dieselbe Sorte Prüfung wie die drei fest verdrahteten Zahlen, die mit S9b umgefallen sind (`issues/260804-0907_c_drei-fest-verdrahtete-zahlen-im-code-brechen-mit-den-neuen-eintraegen-aus-s9b.md`). "Die Abnahmekriterien aus C4, soweit sie an der Oberfläche hängen" sagt dasselbe ohne Literal.

## Was zu tun ist

Zwei Stellen im Plan, beide `planner`: das Abnahmekriterium von S16 und der Nachzugsvermerk vom 260804-1832 im Kopf. Kein Eingriff am Spec, kein Eingriff am Code.

## Dringlichkeit

Bindet keinen Schritt. S16 trägt `[DONE]`; die Zahl fällt beim nächsten Durchgang durch C4 oder bei der Abnahme der Runde auf.

---

**Aufgefallen bei:** dem Aufräumdurchgang vom 260804-2318, beim Nachzählen der C4-Kriterien nach der Umformulierung der Fortschrittsschwelle. Außerhalb des damaligen Auftrags und deshalb gemeldet statt behoben.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C4),
`circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (Kopf, Nachzug 260804-1832; S16, Abnahmekriterium),
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-1832_c_die-zahl-der-c4-abnahmekriterien-steht-im-plan-auf-sechzehn-und-im-spec-auf-achtzehn.md`

---
Resolved: die Zahl ist aus dem Abnahmekriterium von S16 und aus dem Nachzugsvermerk vom 260804-1832 im Plankopf entfernt statt ein drittes Mal nachgezogen, wie dieser Datensatz es vorgeschlagen hat. "Die Abnahmekriterien aus C4, soweit sie an der Oberfläche hängen" sagt dasselbe ohne Literal. Beim Nachziehen fiel derselbe Fehler ein zweites Mal auf: das Abnahmekriterium von S12 sprach von "den acht Abnahmekriterien aus C1 und den sieben aus C7", während C1 schon vorher neun Zeilen der Form `- [ ]` führte und seit dem 260805-0000 zehn führt. Auch dort ist die Zahl entfallen. Kein Eingriff am Spec, kein Eingriff am Code. Sitzungsbericht `history/260805-0000-sieben-nutzerantworten-eingearbeitet.md`.
