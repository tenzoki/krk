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

---
## Abgleichsvermerk 260811-2157 (`reconciler`): der Befund steht zu Recht offen, und er hat sich in der naechsten Sitzung wiederholt

**Nachgezaehlt in `fusion-workbench/orchestrator-events.jsonl`, ab dem letzten `session_start`
(2026-08-11T12:55:10, Circle `260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`):** neun
Ereignisse insgesamt, davon **kein einziges** `task_start`, `task_done` oder `commit`. Git zaehlt
fuer dieselbe Spanne (`55a4afa..HEAD`) **16 Commits**. Der Befund gilt damit unveraendert; er ist
kein Einzelfall der Sitzung 260810-1647.

Die neun: `session_start`, `gate_response`, `shaper_done`, `conceptrev_done` (zweimal),
`planner_done`, `turn_start`, `review_done`, `turn_end`.

**Zwei Grenzereignisse fehlen daneben**, und das widerspricht der Diagnose dieses Datensatzes:
`scope_resolved` und `queue_built` sind nicht emittiert, obwohl das Sitzungsprotokoll beides
ausschreibt (Abschnitte `## Momentaufnahme` und `## Aufgeloeste Pfade`). Der Satz oben, die
Grenzereignisse haengen an Schritten, an denen die Sitzung ohnehin anhaelt, traegt also nur fuer
einen Teil von ihnen.

**Der schwerere Teil ist als eigener Datensatz abgelegt:** fuenf der 16 Commits sind **nach** dem
`turn_end` von Turn 1 entstanden, ohne dass ein zweiter `turn_start` emittiert worden waere
(`shared/issues/260811-2157_o_fuenf-commits-stehen-hinter-dem-letzten-turn-ende-ohne-eigene-turn-grenze.md`).

---
Abgleich 260813 (reconciler, Runde 7): **Der Defekt tritt in der Sitzung 260813-0040 zur
Haelfte wieder auf, und die Haelfte ist neu.** Turn 1 ist vollstaendig protokolliert: vier
`task_start`, vier `task_done`, drei `commit`, zwei `review_start`, zwei `review_done`, ein
`turn_end`. Turn 2 traegt allein sein `turn_start` (`2026-08-13T03:51:01`) und danach nichts
mehr — kein `task_start`, kein `task_done`, kein `commit`, kein `turn_end`, obwohl der Turn
zwei Commits hervorgebracht hat (`dff167a` mit achtzehn behobenen Befunden und `1cd7788`).

**Der Befund ist damit nicht mehr „alle Turns einer Sitzung" sondern „der letzte Turn".**
Das deckt sich mit `shared/issues/260811-2157_*_fuenf-commits-stehen-hinter-dem-letzten-turn-ende-…`:
beide Male reisst das Protokoll ab, sobald die geplante Arbeit steht und der Nachlauf beginnt.
Belegt am Ereignisprotokoll `fusion-workbench/orchestrator-events.jsonl`, letzte Zeile.

Also seen: 260818-0710 by reconciler — dieselbe Lücke, milder: in der Sitzung 260817-2131 tragen 13
der 16 Commits aus `cdde9da..HEAD` ein `commit`-Ereignis, drei nicht (`8f556ed`, `f79f964`,
`b0eee2c`). Zwei davon legen eine Durchsicht ab, der dritte ist ein Datencommit. `task_start` und
`task_done` stehen diesmal vollständig (je 13). Der Befund ist damit nicht mehr „gar keine
Aufgabenereignisse", sondern „das Ablegen einer Durchsicht emittiert keins", und das ist eine
engere und leichter zu schließende Fassung derselben Sache.

Also seen: 260818-0807 by reconciler — die engere Fassung des vorigen Eintrags hält nicht. In
`cdde9da..HEAD` derselben Sitzung stehen jetzt 20 Commits und 16 `commit`-Ereignisse; ohne
Ereignis sind vier: `8f556ed` und `f79f964` (Ablage einer Durchsicht), `b0eee2c` (Datencommit,
das zugehörige `task_done` steht) und `1cef661` (der erste Abgleich; ein `reconciliation`-Ereignis
steht auf derselben Sekunde, ein `commit`-Ereignis nicht). **Das Ablegen einer Durchsicht ist
damit nicht der Grund:** `e843d90` und `9ac41ea` legen ebenfalls eine Durchsicht ab und tragen
beide ihr `commit`-Ereignis. Die Lücke sitzt in den Turns 1 und 2 und im Nachlauf, nicht an einer
Commit-Art. `task_start`/`task_done` stehen bei 15/16, und die Paarung geht nicht auf: `F-7` und `R-3`
tragen ein `task_done` ohne `task_start`, `F-5` ein `task_error` statt eines `task_done`.
