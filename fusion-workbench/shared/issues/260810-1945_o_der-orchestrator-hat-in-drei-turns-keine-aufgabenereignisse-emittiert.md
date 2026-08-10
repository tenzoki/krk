Der Orchestrator hat in drei Turns keine Aufgabenereignisse emittiert

---

Das Ereignisprotokoll `fusion-workbench/orchestrator-events.jsonl` trägt für die Sitzung
260810-1647 die Grenzereignisse (`session_start`, `scope_resolved`, `queue_built`, `turn_start`,
`turn_end`, `review_start`, `review_done`, `coherence_review`, `state_drift`), aber **kein
einziges** `task_start`, `task_done` oder `commit` — obwohl zwölf Aufgaben gelaufen und
siebzehn Commits entstanden sind. Der Orchestrator-Prompt verlangt sie an drei Stellen von
Phase 2, Schritt 3a und 3b.

---

**Schwere:** Niedrig
**Gefunden:** orchestrator, beim eigenen Abschlussbericht der Sitzung 260810-1647
**Betroffen:** das Verfahren, nicht der Code dieses Projekts
**Domain:** code

## Was dadurch fehlt

Der Ablaufplan, den Phase 4 aus dem Ereignisprotokoll erzeugt, ist die Stelle, an der sich
später ablesen lässt, welcher Agent wann welche Aufgabe bekam und mit welchem Commit sie
endete. Ohne die Aufgabenereignisse bleibt er auf der Ebene der Turn-Grenzen und muss aus den
Commit-Nachrichten rekonstruiert werden. Das geht hier, weil jede Commit-Nachricht `Task:` und
`Source:` führt — es ist also kein Verlust an Nachvollziehbarkeit, sondern einer an
Maschinenlesbarkeit.

**Nicht nachträglich erzeugt.** Ereignisse mit dem Zeitstempel des Abschlussberichts zu
schreiben, hieße einen Verlauf zu behaupten, den niemand gemessen hat. Der Verlauf steht in den
siebzehn Commits, und die tragen ihre echten Zeiten.

## Warum es passiert ist

Die Grenzereignisse hängen an Schritten, an denen die Sitzung ohnehin anhält und etwas anderes
tut (Zustand schreiben, Dashboard nachziehen, den Nutzer fragen). Die Aufgabenereignisse hängen
an nichts dergleichen: sie sind eine eigene Verpflichtung mitten im Arbeitsfluss, und genau
solche werden unter Aufgabendruck übersprungen. Es ist dieselbe Form wie beim eingefrorenen
Sitzungszustand, den der Orchestrator-Prompt unter „Drift check" beschreibt und dessen Antwort
dort lautet: die Prüfung an ein Ereignis hängen, das ohnehin emittiert wird.

## Denkbarer Weg

Nicht mehr Vorschrift, sondern weniger Gelegenheit zum Auslassen: die Aufgabenereignisse an
denselben Befehl hängen wie den Commit, der die Aufgabe abschließt. Ein Commit findet statt oder
findet sichtbar nicht statt; ein Ereignis daneben nicht.

Zwei Schwesterbefunde derselben Sitzung und derselben Form:
`shared/issues/260810-1907_*_die-durchsicht-von-turn-2-hat-kein-durchsichtsdokument-hinterlassen.md`.

## Dringlichkeit

Gering. Nichts am Projekt ist falsch, und die Nachvollziehbarkeit steht über die Commits.

---

## Nachtrag 260810-1948: derselbe Befund an einer zweiten Flaeche

Der Abgleich vor `session_end` hat gemeldet, dass `agentstate.yaml` bei Turn 1 eingefroren ist:
die Datei fuehrte `commits: 8` und `turn: 1`, waehrend git 17 Commits und das Ereignisprotokoll
drei `turn_start` zaehlten. Geschrieben worden ist sie zuletzt am Ende von Turn 1; Turn 2 und
Turn 3 haben sie nicht angefasst.

**Das ist derselbe Defekt wie oben, an einer anderen Flaeche**, und beide zusammen sind der
Beleg fuer die Diagnose, die oben nur behauptet war. Was in dieser Sitzung nicht eingefroren
ist: das Ereignisprotokoll und git. Beide sind Aufrufe, die entweder stattfinden oder sichtbar
nicht stattfinden. Was eingefroren ist: die beiden Flaechen, deren Pflege eine eigene
Verpflichtung neben der Arbeit ist.

Der Befund ist als `state_drift` im Ereignisprotokoll festgehalten, bevor `agentstate.yaml`
beim regulaeren Abschluss geloescht wurde — das Protokoll ueberlebt die Datei, und ein
Befund, der mit seinem Beleg verschwindet, ist keiner.

Erkannt hat ihn der Abgleich an der letzten Stelle, an der die Zahlen noch vergleichbar waren.
Dass er ueberhaupt lief, liegt daran, dass er im Orchestrator-Prompt an die Emission von
`session_end` gehaengt ist statt danebenzustehen. Das ist genau der Weg, den der Abschnitt
"Denkbarer Weg" oben fuer die Aufgabenereignisse vorschlaegt, und er hat hier funktioniert.
