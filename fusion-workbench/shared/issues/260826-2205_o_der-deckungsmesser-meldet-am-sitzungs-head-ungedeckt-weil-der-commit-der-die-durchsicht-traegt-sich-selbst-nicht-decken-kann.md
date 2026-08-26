Der Deckungsmesser meldet am Sitzungs-HEAD `uncovered`, weil der Commit, der die Durchsicht trägt, sich selbst nicht decken kann

---

`bin/fusion-review-coverage` zählt jeden Commit des Sitzungsbereichs, auch die, die allein
`fusion-workbench/` anfassen. Der letzte Commit einer Sitzung ist regelmäßig genau so einer:
er legt die Durchsichtsdatei und die von ihr gefilterten Datensätze ab. Diese Durchsicht kann
den Commit, der sie trägt, nicht decken — ihr `**Reviewed-range:**` endet vor ihm. Das
Werkzeug meldet am Sitzungs-HEAD deshalb `verdict=uncovered`, obwohl jede Codeänderung der
Sitzung gedeckt ist.

---

**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Affected:** `bin/fusion-review-coverage` (fusion-Werkzeug, nicht KRK-Code); jede Sitzung, die ihre Durchsicht als eigenen Commit ablegt
**Tree state:** `bc5991d`
**Domain:** code

## Am 260826-2205 gemessen

Dreimal gefahren, aus dem Projektwurzelverzeichnis:

```
fusion-review-coverage --since 26e8039 --head fc829c8
  commits=6  reviews=2  unusable=0  uncovered=0  verdict=covered

fusion-review-coverage --since 26e8039 --head HEAD          # HEAD = bc5991d
  commits=7  reviews=2  unusable=0  uncovered=1  verdict=uncovered
  uncovered bc5991d docs(workbench): die Durchsicht der zweiten Haelfte von Runde 1, sechs Datensaetze

fusion-review-coverage --since fc829c8 --head HEAD
  commits=1  reviews=1  unusable=0  uncovered=1  verdict=uncovered
```

`bc5991d` ändert acht Dateien, alle unter `fusion-workbench/shared/`: die Durchsichtsdatei
`260826-2158-coderev-behebungssitzung-runde-1-kindstarter-kennungen-pruefordner.md`, sechs von
ihr gefilterte Defektdatensätze und eine `Also seen:`-Zeile an einem siebten. Keine Zeile Code.

## Warum das mehr ist als ein Schönheitsfehler

Der Orchestrator hat sein Ergebnis am Stand `fc829c8` gemessen und `verdict=covered,
uncovered=0` weitergereicht; das war richtig. Wer dieselbe Frage später am Sitzungs-HEAD
stellt — der Abgleich, der Curator, der nächste Orchestrator beim Aufsetzen —, bekommt
`uncovered` und keinen Hinweis, dass der eine ungedeckte Commit gar kein Code ist. Die zwei
Antworten widersprechen sich, und die spätere ist die, die stehen bleibt.

Die Lage wiederholt sich in **jeder** Sitzung, die ihre Durchsicht als eigenen Commit ablegt,
und das ist die Hausform dieses Projekts.

## Nachbarschaft

Nicht dasselbe wie `shared/issues/260817-1122_*_der-durchsichtsbereich-schliesst-seinen-ersten-commit-aus.md`:
dort ist die **Bereichsangabe** der Durchsicht um einen Commit zu kurz, hier ist der Bereich
richtig und der ungedeckte Commit entsteht erst danach. Auch nicht dasselbe wie
`circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260812-1816_*_die-durchsicht-von-turn-2-liest-einen-reinen-grundlagen-commit-als-codeaenderung.md`:
dort behauptet ein Durchsichtstext etwas Falsches über einen Werkbank-Commit, hier zählt das
Werkzeug ihn mit.

## Was zu tun wäre

Drei Wege, keiner davon hier entschieden:

1. Der Messer lässt Commits aus, die ausschließlich `fusion-workbench/` ändern — er hat den
   Anker `anchor=workbench-root` ohnehin schon.
2. Die Ausgabe trennt „ungedeckte Codecommits" von „ungedeckten Werkbank-Commits" und stellt
   das Urteil allein über die ersten.
3. Es bleibt, wie es ist, und der Sitzungsbericht hält den Stand fest, an dem gemessen wurde.
   Dann gehört der Stand in jede Deckungsaussage, die eine Sitzung weiterreicht.

Der erste und der zweite Weg ändern ein fusion-Werkzeug und nicht KRK; der dritte ist eine
Regel für den Orchestrator. Der Datensatz liegt hier, weil KRK die Lage trägt.

## Was geprüft ist

Die drei Läufe oben selbst gefahren am 260826-2205; `git show --stat bc5991d` gelesen; die
zwei Durchsichtsdateien unter `shared/reviews/` auf ihre `**Reviewed-range:**`-Zeilen gelesen
(`26e8039..9c02863` und `9c02863..fc829c8`, zusammen alle sechs Commits vor `bc5991d`).
