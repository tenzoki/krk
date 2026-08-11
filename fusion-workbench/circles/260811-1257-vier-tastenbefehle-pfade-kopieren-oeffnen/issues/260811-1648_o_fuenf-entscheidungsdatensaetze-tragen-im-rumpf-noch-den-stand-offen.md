Fünf Entscheidungsdatensätze dieses Circles tragen im Rumpf noch `**Status:** open`

---

Fünf der sechs Datensätze unter `circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/decisions/` tragen im Dateinamen den Marker `_a_` und am Ende eine ausgeschriebene `Answered:`-Zeile, im Kopf aber weiterhin `**Status:** open`. Der sechste (`260811-1612_a_…`) trägt `**Status:** answered` und zeigt damit, wie es gemeint ist. Wer den Stand am Kopf abliest statt am Dateinamen, liest bei fünf von sechs das Gegenteil.

Betroffen sind:

- `260811-1257_a_wie-weit-soll-cmd-w-reichen.md:5`
- `260811-1258_a_was-kopiert-der-pfadkopierer-bei-stehender-markierung.md:5`
- `260811-1259_a_was-tut-ein-doppelklick-auf-einen-ordner.md:5`
- `260811-1300_a_welche-vier-kombinationen-gelten-ab-werk.md:5`
- `260811-1552_a_welche-sorten-legt-der-pfadkopierer-in-die-zwischenablage.md:5`

---

Gefunden vom `planner` beim Lesen der Grundlage für den Umsetzungsplan vom 260811-1648. Kein Datensatz ist inhaltlich falsch, und keiner hält den Plan auf; die Antworten stehen vollständig und sind im Spec eingearbeitet.

Dieselben fünf Dateien tragen daneben zweimal den Block `Answered:` / `Implemented:` / `Deferred:` / `Superseded by:` — einmal leer aus der Vorlage und einmal darunter ausgefüllt. Auch das ist Lesbarkeit und keine falsche Aussage; wer den Stand aufräumt, räumt beides in einem Zug auf.

Die Behebung ist eine Zeile je Datei: `**Status:** open` wird zu `**Status:** answered`, und der leere Vorlagenblock entfällt. Sie gehört in denselben Zug, in dem die fünf Datensätze nach der Umsetzung auf `_i_` wandern, dann steht dort `**Status:** implemented`.
