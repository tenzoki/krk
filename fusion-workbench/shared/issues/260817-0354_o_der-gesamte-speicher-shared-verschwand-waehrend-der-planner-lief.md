Der gesamte Speicher shared/ verschwand aus dem Arbeitsbaum, während der Planner lief

---

Zwischen dem Commit `5a52f16` (260816-2256) und dem Ende des Planner-Laufs (260817-0344)
ist das Verzeichnis `fusion-workbench/shared/` vollständig aus dem Arbeitsbaum
verschwunden. `git status` meldete 183 gelöschte Dateien, `ls` meldete
"No such file or directory". Die Ursache ist unbekannt.

Wiederhergestellt mit `git checkout HEAD -- fusion-workbench/shared`. Alle 183
verfolgten Dateien sind zurück: 4 Pläne, 77 Defekte, 24 Entscheidungen, 74
Sitzungsprotokolle, 6 Durchsichten, 1 Beratung, 3 Backlog-Einträge. Die drei leeren
Speicher `analyses/`, `memos/` und `investigations/` sind von Hand neu angelegt worden,
weil git kein leeres Verzeichnis führt.

**Eine Datei ist verloren** und nicht wiederherstellbar: der Befund, den der Planner
während seines Laufs nach `shared/issues/` gelegt hat, zum Doc-Kommentar von
`ablage::pfad`, der vier Dateien nennt. Er war beim Verschwinden noch nicht committet.
Sein Inhalt steht dem Sinn nach im Sitzungsprotokoll des Planners
(`circles/260816-2255-befehle-absetzen-und-makros-speichern/history/260816-2307-plan-der-zwoelften-runde.md`)
und ist von dort neu zu erheben.

---

Was gesichert ist:

- Der Circle der zwölften Runde und alles darin haben überlebt. Betroffen war allein
  `shared/`.
- Das Ereignisprotokoll des Wächters (`.guard-state/events.jsonl`) endet um 20:56 UTC
  mit dem Schreiben der Commit-Nachricht. Nach diesem Zeitpunkt steht dort nichts, also
  ist das Verschwinden nicht über ein bewachtes `Write` oder `Edit` gelaufen.
- Der Orchestrator hat in diesem Zeitraum kein Kommando abgesetzt, das unter `shared/`
  schreibt oder löscht. Sein letzter Eingriff vor dem Befund war der Commit `5a52f16`.

Was nicht gesichert ist: wer oder was gelöscht hat. Der Planner lief in diesem Fenster
und ist damit zeitlich der nächste Verdacht, aber sein Auftrag verlangte allein Lesen
unter `shared/` und Schreiben in den Circle, und ein Beleg für seine Urheberschaft liegt
nicht vor. Ebenso möglich ist ein Vorgang außerhalb der Sitzung.

**Der eigentliche Befund ist nicht die Löschung, sondern dass sie unbemerkt blieb.**
Sie wäre erst beim nächsten `git status` aufgefallen, und das war Zufall: der
Orchestrator hat ihn vor einem Commit gefahren. Wäre in diesem Zustand ein
`git add`-Aufruf mit Verzeichnisargument gelaufen, hätte er 183 Löschungen in einen
Commit gestellt.
