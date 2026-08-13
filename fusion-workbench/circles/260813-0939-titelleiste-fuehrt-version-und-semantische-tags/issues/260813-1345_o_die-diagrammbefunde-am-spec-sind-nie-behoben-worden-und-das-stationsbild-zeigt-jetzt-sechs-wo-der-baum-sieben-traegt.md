Die Diagrammbefunde am Spec sind nie behoben worden, und sein Stationsbild zeigt jetzt sechs, wo der Baum sieben trägt

---

Die Diagrammprüfung des Spec
(`reviews/260813-1049-conceptrev-spec-titelleiste-fuehrt-version-und-semantische-tags.md`,
Spruch `acceptable`) schliesst mit: „B1 bis B3 sind an Ort und Stelle zu beheben, ohne einen
Knoten oder eine Kante zu verschieben." Das Sitzungsprotokoll des Orchestrators übernimmt das
als Erledigungszusage: „die drei mittleren Befunde betreffen Beschriftungen und sind an Ort und
Stelle zu beheben."

**Keiner der Befunde ist behoben worden.** Der Spec ist seit `59b0a6c` nicht mehr angefasst
worden (`git log -- planning/260813-1037_o_spec-…`), und alle drei Stellen stehen unverändert.

---

**Schwere:** niedrig. Kein Bau, kein Verhalten. Der Spec ist das Dokument, gegen das die
Abnahme dieser Runde läuft, und drei seiner Aussagen führen den nächsten Leser in die Irre.

**Was unverändert dasteht**

| Befund | Stelle | Was dort steht |
|---|---|---|
| B1 (mittel) | Spec, Bild 1 | `Q -->\|erbt\| ENV` und `PL -->\|liest\| UE` lesen sich entlang des Pfeils rückwärts, `ENV -->\|schreibt\| TL` vorwärts. Vier Verben, zwei Richtungen |
| B2 (mittel) | Spec, Bild 2 | Der Teilgraph `BILLIG` sagt „bricht ab, bevor etwas kostet", und nur `S0` trägt eine Kante nach `ABBRUCH`. Alle vier Knoten der Phase brechen ab |
| B6 (niedrig) | Spec, Prosa über Bild 1 | „drei Abnehmer statt einen, und einen Prüfer" gegen einen Ausgangsgrad von 3 an `Q` |

B3 ist durch den Plan beantwortet worden (die Diagrammprüfung des Plans bestätigt es
ausdrücklich), B4 und B5 sind gegenstandslos. Am Spec selbst steht keiner der drei berichtigt.

**Ein vierter Punkt ist erst durch den Bau entstanden und wiegt schwerer als B1 bis B6.**
Das Stationsbild des Spec führt `S0` als neue Station ohne Zahl, dann `1 · AppKit-Grenze` bis
`6 · beglaubigen und anheften` (`planning/260813-1037_o_spec-…:94-105`), und die Prosa sagt
zweimal „sechs Stationen" (`:44`, `:82`). Der Plan hat in D3 auf **sieben durchgehend
numerierte Stationen und drei benannte Vorläufe** festgelegt, und der Baum trägt das seit
`f9e5137` an allen drei Stellen: `xtask/src/release.rs:3` („Der Weg in sieben Stationen"),
`xtask/src/main.rs:40`, `README.md:217`. Der Spec ist damit das einzige lebende Dokument der
Runde, das noch sechs zählt.

**Warum das nicht unter den schon erfassten Querschnitt fällt.** Die sechs Prosastellen der
Datensätze `260813-1258` und `260813-1420` liegen im Quellbaum und entstehen an Schrittgrenzen.
Diese hier liegt in der Workbench, und ihre Ursache ist eine andere: D3 zählt seine drei Stellen
abschliessend auf und meint damit den Baum. Dass der Spec eine vierte ist, hat niemand geprüft,
weil der Spec dem Plan vorausgeht und im Normalfall nicht nachgezogen wird.

**Was zu tun ist**

Zwei Dinge, die nicht aneinander hängen.

1. **Das Stationsbild und die zwei Prosastellen des Spec nachziehen**, oder — die kleinere
   Aussage — an der Stelle vermerken, dass der Plan die Zählung auf sieben festgelegt hat und
   der Spec den Stand vor dem Plan zeigt. Das zweite ist ehrlicher: ein Spec beschreibt, was
   die Runde vorfand, und der Plan entscheidet die Bauart.
2. **B1, B2 und B6 beheben oder ausdrücklich verwerfen.** Eine Erledigungszusage, die
   niemandem gehört, ist der Zustand, den dieser Datensatz festhält.

**Kontext**

- Gefunden beim Abgleich der Runde 8 gegen den Baum, 260813-1345.
- Die Diagrammprüfung des Plans (`reviews/260813-1124-conceptrev-plan-…`, Spruch `acceptable`)
  trägt drei eigene Befunde F1 bis F3, die ebenfalls „an Ort und Stelle zu beheben" sind und
  ebenfalls nicht behoben wurden. F1 ist derselbe Punkt wie B2 an einem neu gezeichneten Bild;
  die Prüfung sagt es selbst: „B2 besteht fort."
