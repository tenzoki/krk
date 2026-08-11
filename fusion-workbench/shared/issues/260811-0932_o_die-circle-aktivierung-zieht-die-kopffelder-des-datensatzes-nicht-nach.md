Die Circle-Aktivierung zieht die Kopffelder des Datensatzes nicht nach

---

Beim Aktivieren eines Circles (`_a_` → `_t_`) benennt der Orchestrator den Datensatz um und
schreibt `.active-circle`. **Drei Kopffelder desselben Datensatzes bleiben dabei stehen, und
niemandes Prompt beauftragt sie:** `**Status:**` behält `anticipated`, `**Active spec/plan:**`
und `**Active session history:**` behalten `(none yet)`.

Gemessen am 260811 beim Aktivieren von
`circles/260809-2040-tastenbelegung-als-markdown-in-downloads`: der Datensatz trug den Marker
`_t_` und im Kopf `**Status:** anticipated`. Marker und Feld widersprachen sich, und zwei
Verweisfelder zeigten ins Leere, obwohl Spec, Plan und Sitzung längst existierten.

---

**Schwere:** Niedrig
**Gefunden:** ontocoder beim Nachziehen der zwei Verweisfelder, dann vom Orchestrator um den
Statusbefund erweitert
**Betroffen:** fusion, nicht KRK — `agents/orchestrator.md`, `rules/circle-records.md`
**Domain:** code

## Warum das niemandes Arbeit ist, und genau das ist der Defekt

Die Zuständigkeiten sind an dieser Stelle vollständig verteilt und decken die Kopffelder
trotzdem nicht ab:

- **Der Orchestrator** darf laut seinem Prompt am Circle-Datensatz genau **eine** inhaltliche
  Änderung vornehmen: die `## Closure note` bei Phase 4. Wörtlich: „the only Circle-record
  content write the orchestrator performs; full-content edits remain off-limits". Ein Kopffeld
  zu setzen fällt nicht darunter.
- **Der shaper** füllt den Grounding-Snapshot in seiner `portfolio-activation`-Betriebsart. Die
  ist aber ausdrücklich **nicht von einem Agenten aufrufbar**: „reachable only by the user
  running shaper directly with the mode contract — no skill or agent dispatches it". Ein
  Orchestrator, der einen Circle aktiviert, kann sie nicht anstoßen.
- **`/fusion:next`** führt die Aktivierungsschreibvorgänge selbst aus, ohne den shaper zu rufen —
  und schreibt dabei ebenfalls nur Marker und Zeiger.
- **Der playmaker** liest die Felder für `portfolio.md`, schreibt sie aber nicht.

Es bleibt also niemand übrig. Der Ausweg im gemessenen Fall war ein eigens beauftragter
`ontocoder`, und das ist ein Umweg und keine Zuständigkeit.

## Was es kostet

`rules/circle-records.md` begründet die Verweisfelder ausdrücklich damit, dass die Konsumenten
sich ohne sie **stillschweigend verschlechtern**: die Suche von `/fusion:circle-stash`, das
Rendern des Portfolios durch den playmaker, die Wiederaufnahme durch den Orchestrator. Ein
`(none yet)` ist dabei nicht besser als ein falscher Pfad, sondern nur leiser.

Beim Statusfeld kommt hinzu, dass es dem Marker **widerspricht**, statt bloß zu fehlen. Wer den
Datensatz liest, findet zwei Aussagen über denselben Zustand, und die Regel sagt an anderer
Stelle, dass der Marker die Wahrheit trägt.

## Denkbare Wege

1. **Der Orchestrator darf die Kopffelder setzen.** Die Ausnahmeliste in `agents/orchestrator.md`
   wird von „nur die Closure note" auf „die Closure note und die Kopffelder" erweitert. Billigste
   Änderung, und der Orchestrator ist ohnehin die Stelle, die Marker und Zeiger schreibt.
2. **Die Aktivierung ruft den shaper**, und dessen `portfolio-activation`-Betriebsart wird für
   Agenten aufrufbar. Sauberer im Sinne der Rollenteilung, aber sie hebt eine ausdrückliche
   Festlegung auf.
3. **Ein `bin/`-Helfer setzt die Felder**, so wie `bin/fusion-paths` die Pfade auflöst. Dann
   hängt es an keinem Prompt und wird nicht unter Aufgabendruck übersprungen — dieselbe
   Überlegung, die der Orchestrator-Prompt selbst für den Drift-Check anstellt.

## Was daran allgemein ist

Es ist dieselbe Form wie bei den zwei Befunden über die eigene Buchführung vom 260810
(`shared/issues/260810-1945_*_der-orchestrator-hat-in-drei-turns-keine-aufgabenereignisse-emittiert.md`):
eine Pflege, die neben der eigentlichen Handlung steht statt an ihr zu hängen, wird
übersprungen. Hier ist sie nicht einmal übersprungen worden, sondern nie jemandem zugeteilt.
