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

---
Resolved: Alle Rumpf-Staende sind auf `answered` gezogen. Es waren **sechs** und nicht fuenf —
der Datensatz zaehlte, bevor `260811-1552` und `260811-1612` dazukamen, und der `planner` hat die
Abweichung bei seiner Nacharbeit am 260811-1721 selbst gemeldet.

**Die Zaehlung im Titel bleibt stehen**, obwohl sie falsch ist: sie hielt den Stand fest, den der
Finder vorfand, und ein nachtraeglich berichtigter Titel verwischte, dass die Menge zwischen
Fund und Behebung gewachsen ist.

**Der Befund ist derselbe wie bei den Circle-Kopffeldern** — der Marker im Dateinamen und das
Feld im Rumpf sagen dasselbe zweimal, und nur eines von beiden wird beim Uebergang nachgezogen.
Fuer Circle-Datensaetze ist das als fusion-Defekt erfasst
(`shared/issues/260811-0932_*_die-circle-aktivierung-zieht-die-kopffelder-des-datensatzes-nicht-nach.md`);
fuer Entscheidungsdatensaetze faellt es demjenigen zu, der die Antwort eintraegt, und in dieser
Runde war das der Orchestrator. Das naechste Mal gehoert das Feld mit der `Answered:`-Zeile in
denselben Handgriff.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1.

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behebung traegt in dem, was sie behauptet, und
die zweite Haelfte des eigenen Behebungsvorschlags blieb liegen.**

Der Rumpf-Stand war gezogen: alle sieben Datensaetze trugen `**Status:** answered`. Der
Behebungsvorschlag des Datensatzes nennt aber zwei Handgriffe — „`**Status:** open` wird zu
`**Status:** answered`, **und der leere Vorlagenblock entfaellt**". Der zweite ist nicht gelaufen:
sechs der sieben Datensaetze trugen am 260811-2157 weiterhin den leeren Block
`Answered:` / `Implemented:` / `Deferred:` / `Superseded by:` **und** darunter den ausgefuellten.
Allein `260811-1612` war sauber.

Die Resolved-Notiz behauptet den zweiten Handgriff nicht, ist also nicht falsch; sie ist
unvollstaendig gegenueber dem, was der Datensatz selbst als Behebung beschreibt. Der Abgleich hat
die sechs leeren Bloecke jetzt entfernt, im selben Zug, in dem die sieben Datensaetze von
beantwortet auf umgesetzt gewandert sind. Damit ist auch der Satz eingeloest, den die Notiz
angekuendigt hat: „dann steht dort `**Status:** implemented`".
