# Die Zahl der Entscheidungsdatensätze im Plan der Runde 18 berichtigt

**Date:** 2026-08-25
**Agent:** coder
**Task:** P-0
**Status:** Complete

## Auftrag

Der Plan `shared/planning/260825-1725_p_plan-vorschau-vertieft-und-zwei-fehler.md`
sprach an drei Stellen von „fünf Entscheidungsdatensätzen". Der Planer hat sieben
geschrieben; alle sieben stehen auf `_a_`. Eng begrenzt auf diese eine Datei, kein
Quelltext, kein Commit.

## Was geändert wurde

Selbst gezählt statt der Aufzählung im Auftrag gefolgt:
`ls -1 fusion-workbench/shared/decisions/260825-1725_a_*.md | wc -l` gibt 7.
`grep -n 'Entscheidungsdatensätz'` über den Plan findet genau drei Stellen, und alle
drei trugen die falsche Zahl — dieselben drei, die der Auftrag vermutet hat:

- Zeile 5, Kopfzeile `**Spec:**` — „in fünf Entscheidungsdatensätzen abgelegt".
- Zeile 398, Abschnitt „Kein weiterer Schritt für `analyst`, und warum" — „in die fünf
  Entscheidungsdatensätze eingegangen".
- Zeile 409, Endbedingung unter „Where this Circle stops" — „Die fünf
  Entscheidungsdatensätze dieser Runde".

Ersetzt wurde allein die Zeichenfolge `fünf Entscheidungsdatensätz` durch
`sieben Entscheidungsdatensätz`, byteweise über `perl -i -pe`. Der Vergleich gegen die
Sicherungskopie weist genau drei geänderte Zeilen aus (`5c5`, `398c398`, `409c409`).
Die Marke `[DONE]` an Schritt 1 (Zeile 179) steht unberührt.

## Was ausdrücklich nicht geändert wurde

Der Plan trägt siebzehn weitere Vorkommen von „fünf" und „fünfter", die eine andere
Sache zählen: fünf Erweiterungen der Vorschau, der fünfte Baustein, die fünf Teilbäume
der Fokusanzeige, die fünf Leseprofile von gestern. Keines davon ist angefasst.

## Abnahme

    a-Datensätze=7  falsche-Stellen=0  richtige-Stellen=3
    exit=0

Kein Quelltext betroffen, also kein Baulauf. Die Änderung steht unbeglichen im
Arbeitsbaum.
