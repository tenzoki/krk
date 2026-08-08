Der Circle-Datensatz trägt den Marker `_t_` und nennt sich in seinem Kopf "anticipated"

---

Die Datei `circles/260807-2116-eingebauter-editor-mit-textmarken/_t_circle.md` trägt seit
der Aktivierung am 260807-2132 den Marker `_t_` im Dateinamen. Ihre Kopfzeile 5 sagt
weiterhin `**Status:** anticipated`, und die Felder `**Active spec/plan:**` und
`**Active session history:**` stehen auf `(none yet)`, obwohl seit dem 260807-2139 eine
Sitzungshistorie unter `history/260807-2139-orchestrator-session.md` liegt.

---

## Warum das zählt

Der Datensatz ist die einzige Stelle, an der der Zustand eines Circles steht, und er sagt
ihn an zwei Orten: im Marker des Dateinamens und in der Kopfzeile. Beide Orte
widersprechen sich hier. `rules/circle-records.md` gibt für den Kopf die Wertliste
`anticipated | active | closed | bounded | superseded | deferred` vor; zum Marker `_t_`
gehört `active`.

Wer den Zustand über den Dateinamen liest, sieht einen aktiven Circle. Wer ihn über den
Kopf liest, sieht einen vorgesehenen. Der Playmaker liest beides, `portfolio.md` wird
daraus erzeugt, und `/fusion:next` würde den Circle bei der nächsten Durchsicht erneut
unter den vorgesehenen Kandidaten anbieten.

## Wie es entstanden ist

`inference:`, nicht am Ereignisprotokoll geprüft. Die Aktivierung am 260807-2132 hat den
Datensatz umbenannt und die Kopfzeilen nicht mitgezogen. Die drei betroffenen Felder sind
genau die, die sich bei einem Übergang von vorgesehen auf aktiv ändern.

## Was zu tun ist

Drei Zeilen im Kopf von `_t_circle.md` nachziehen: `**Status:**` auf `active`,
`**Active session history:**` auf `circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md`
und `**Active spec/plan:**` auf den Spec dieser Runde, sobald er abgenommen ist
(`circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`).

Der Shaper hat den Defekt beim Lesen des Datensatzes gefunden und ihn nicht selbst
behoben: die Kopfzeilen eines Circle-Datensatzes gehören dem Orchestrator, und der Shaper
schreibt außerhalb des portfolio-activation-Modus keine.

**Aufgefallen bei:** Klärung und Spec-Schreibung für diese Runde am 260807-2147.
