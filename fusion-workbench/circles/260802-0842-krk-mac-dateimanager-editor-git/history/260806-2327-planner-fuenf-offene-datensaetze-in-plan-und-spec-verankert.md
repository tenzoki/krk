# Planner 260806-2327: fünf offene Datensätze in Plan und Spec verankert

**Status:** Complete
**Agent:** planner
**Circle:** `circles/260802-0842-krk-mac-dateimanager-editor-git`
**Auftrag:** Aufgabe D2 aus Turn 25. Nachzug an Plan und Spec, damit fünf offene Entscheidungsdatensätze an dem Schritt und der Fähigkeit stehen, auf die sie zeigen. Dazu zwei Verweisberichtigungen. Kein Codeeingriff, keine neue Planungsdatei, kein Commit.

## Anlass

`issues/260806-1735_*_fuenf-offene-entscheidungen-zeigen-auf-einen-planschritt-ohne-dort-genannt-zu-sein.md`. Die Durchsicht, die den Sortierfall nachzog, fand fünf weitere Datensätze derselben Form: sie nennen einen Planschritt oder eine Fähigkeit und wurden dort nicht genannt. Die Asymmetrie ist eine Stufe schwächer als beim Sortierfall, weil alle fünf nach der Abnahme ihres Schrittes entstanden sind und keiner eine Bindung behauptet. Der Wert liegt in der Auffindbarkeit.

## Geändert

Sieben Dateien. Plan und Spec behalten `_o_`, die fünf Datensätze behalten `_o_`, kein Schritt verliert sein `[DONE]`, keine der zehn Zahlen aus C8 ist berührt, und der Umfang der Runde wächst nicht.

`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`:

- Datumszeile um 23:27 ergänzt, neuer Absatz **Nachzug 260806-2327** im Kopf.
- Je eine Notiz an S18, S18c, S19, S20, S21 und S22. Jede nennt die Frage in der Sache, die Möglichkeiten des Datensatzes, seine Empfehlung und den Satz, dass der Schritt abgenommen bleibt.
- `## Angelegte Defekte und Entscheidungen`: fünf neue Einträge am Ende der Liste der Umsetzungsmeldungen, dazu das Erhebungsdatum im Kopfabsatz.

`planning/260802-1036_o_spec-navigator-geruest.md`:

- Datumszeile und Statuszeile nachgezogen, neuer Blockzitat-Absatz **Stand 260806-2327** im Kopf.
- C3 um zwei Festlegungen, C5 um einen erstmals angelegten Abschnitt `Getroffene Festlegungen` mit einer Festlegung, C6 um eine, C7 um eine, C8 um eine Messbedingung, C10 um eine Festlegung, C11 um eine.
- `## Offene Nutzerentscheidungen`: neuer Kopfabsatz, der die fünf Fragen mit ihrer Einarbeitungsstelle aufführt.

## Berichtigt

- `decisions/260802-1810_i_sortierung-ohne-sprachsensitive-kollation.md`: Cross-references nannten den Spec-Abschnitt C1; die Sortierung gehört zu C2 und stand nie in C1.
- Vier der fünf Datensätze schrieben in ihren Cross-references den Zustandsmarker aus statt der Sternform, nicht zwei, wie die Meldung annahm. Alle vier sind auf `_*_` gezogen: `260805-1845`, `260805-2216`, `260805-2252`, `260806-1303`. Zwei davon zeigten auf einen Stand, den ihr Ziel längst verlassen hatte: `260805-1623_a_` steht heute auf `_i_`, `260806-1235_o_` auf `_c_`. Die übrigen zwei nannten den zutreffenden Marker und verstießen allein gegen die Regel.
- Zwei Verweise sind bei der Gelegenheit auflösbar gemacht worden: `260806-1303` nannte den Spec als "Spec C8" ohne Pfad, `260805-2252` nannte den Spec gar nicht, obwohl der Befund C3 als seine Fähigkeit führt.

## Nicht geändert

- Die Marker der fünf Datensätze bleiben `_o_`. Die Fragen sind verankert, nicht beantwortet.
- Das vierte Abnahmekriterium von C2, das der Datensatz `260805-2216` als überholt bezeichnet, bleibt stehen. Es zu ziehen hieße, die Frage zu beantworten.
- Die Sternform in den Cross-references des Sortierdatensatzes bleibt ausgeschrieben. Der Auftrag begrenzte die Änderung an dieser Datei auf die Berichtigung C1 nach C2; die Stelle ist als Befund gemeldet.
