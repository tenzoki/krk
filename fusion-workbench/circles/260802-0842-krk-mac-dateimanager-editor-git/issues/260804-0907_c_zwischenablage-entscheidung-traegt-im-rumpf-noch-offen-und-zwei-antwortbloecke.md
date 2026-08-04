Die Zwischenablage-Entscheidung trägt im Rumpf noch "offen" und zwei Antwortblöcke

---

`decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md` ist im
Dateinamen beantwortet (`_a_`), sagt im Kopf aber weiterhin
`**Status:** open`. Am Fuß stehen zwei Abschlussblöcke übereinander: erst der
leere Vorlagenblock (`Answered:` / `Implemented:` / `Deferred:` /
`Superseded by:` ohne Werte), dann der ausgefüllte `Answered:` mit dem
Nutzerentscheid vom 260804.

---

Wer den Kopf liest, hält die Frage für unbeantwortet; wer den ersten
Fußblock liest, ebenfalls. Der Datensatz ist die bindende Grundlage für S13,
und S13 steht noch aus.

Behebung: `**Status:** answered` setzen und den leeren Vorlagenblock
entfernen. Die Antwort selbst ist unstrittig und bleibt, wie sie steht.

Nicht mit S9b behoben, weil der Auftrag den Eingriff auf
`resources/default-keymap.toml` begrenzt hat.

Cross-references:
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/history/260804-0907-s9b-drei-kombinationen-nachgetragen.md`

---
Resolved: Beides nachgezogen am 260804-0912 durch den orchestrator, der den Fehler selbst verursacht hatte: der leere Vorlagenblock ist entfernt, sodass nur noch eine `Answered:`-Zeile steht, und der Kopf trägt `**Status:** answered` statt `open`. Der Datensatz `decisions/260804-0830_a_was-die-zwischenablage-auswertung-liest.md` nennt damit an beiden Stellen denselben Stand wie sein Dateiname.
