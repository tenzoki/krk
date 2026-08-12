Sechs Sitzungsprotokolle der Runde 6 tragen einen Zeitstempel, den die Uhr nicht hergab

---

Sechs der dreizehn Protokolle unter `history/` dieses Circles sind auf eine
Uhrzeit benannt, die zum Zeitpunkt ihres Schreibens noch nicht erreicht war.
Die Abweichung wächst über die Runde von 24 Minuten auf 2 Stunden 12 Minuten.
Die Konvention verlangt, den Zeitstempel aus `date +%y%m%d-%H%M` zu holen;
`rules/fusion-workbench-conventions.md` `## Timestamps` sagt dazu: „LLMs have
no clock — never guess or estimate the time."

---

**Nachgemessen am 260812-1805**, Dateiname gegen Änderungszeit im Dateisystem:

| Dateiname | geschrieben | Abweichung |
|---|---|---|
| `260812-1432-coder-ordnersprung-…` | 14:08 | +24 min |
| `260812-1600-coder-rechtsklick-…` | 15:15 | +45 min |
| `260812-1710-coder-umsetzung-einer-formatierung-…` | 15:56 | +74 min |
| `260812-1815-coder-markdown-zerlegen-…` | 16:29 | +106 min |
| `260812-1900-coder-eine-statuszeile-…` | 17:29 | +91 min |
| `260812-1955-coder-die-zeile-laesst-sich-…` | 17:43 | +132 min |

Die übrigen sieben stimmen auf wenige Minuten. Drei der sechs — 1815, 1900 und
1955 — liegen in der Zukunft: sie sind später datiert als der Augenblick, in
dem diese Durchsicht geschrieben wird.

**Was daran schadet.** Der Speicher `history/` ist chronologisch sortiert und
wird von jedem Agenten am Sitzungsanfang überflogen; die Reihenfolge ist die
einzige Auskunft darüber, was worauf folgte. Sie ist hier erfunden. Und
Artefakte, die heute mit der richtigen Uhrzeit entstehen — diese Durchsicht,
ihre Defektdatensätze —, sortieren **vor** Arbeit, die vor ihnen stattgefunden
hat. Wer später die Reihenfolge liest, liest sie falsch herum.

**Kein Einzelfall und keine Schätzung.** Die Abweichung wächst monoton über
sechs Dateien hinweg. Das ist das Muster einer fortgeschriebenen Annahme („der
nächste Schritt dauert etwa eine Stunde") und nicht das eines Vertippers.

**Was zu tun ist.** Die sechs Dateien auf ihre Änderungszeit umzubenennen ist
der eine Handgriff; die Verweise darauf sind zu prüfen (`grep -rn` auf den
alten Namen, und dabei das Muster ohne `.md` mitnehmen — `CLAUDE.md` führt den
blinden Fleck jeder Suche, die `\.md` verlangt). Der zweite Handgriff ist die
Ursache: der schreibende Agent hat die Uhr nicht gefragt.

**Gewicht:** niedrig für den Code, mittel für den Speicher — eine
Sitzungshistorie, deren Reihenfolge nicht stimmt, beantwortet die eine Frage
nicht mehr, für die sie da ist.

**Herkunft:** Circle der Runde 6, beim Durchsehen von Turn 2 gefunden; vier der
sechs Dateien stammen aus Turn 2, zwei aus Turn 1.
