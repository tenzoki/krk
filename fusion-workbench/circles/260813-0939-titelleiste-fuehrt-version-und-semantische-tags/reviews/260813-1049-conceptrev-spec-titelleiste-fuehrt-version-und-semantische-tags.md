# Concept Evaluation: Spec Titelleiste führt Version und semantische Tags

**Date:** 2026-08-13 10:49
**Target:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1037_o_spec-titelleiste-fuehrt-version-und-semantische-tags.md`
**Verdict:** acceptable
**Diagrams evaluated:** 2  |  **Validation:** by-tool (`@mermaid-js/mermaid-cli` 11.16.0, beide Blöcke nach SVG gerendert)

## Verdict

**acceptable.** Beide Graphen sind strukturell sauber, und die Befunde betreffen die Beschriftung, nicht den Bau. Gemessen: kein Zyklus, kein Gott-Knoten, kein freistehender Knoten, in beiden Bildern sichtbare Schichtung über `subgraph`, in beiden eine Kantenzahl unterhalb der Knotenzahl. Das Design, das die Bilder zeigen, ist ein Baum mit einer Quelle und drei Wegen, und genau das ist die Aussage, die der Spec beansprucht. Was den Ausschlag von *clean* zu *acceptable* gibt, sind drei Beschriftungsbefunde: die Kantenverben in Bild 1 zeigen in zwei entgegengesetzte Richtungen, der Teilgraph BILLIG in Bild 2 sagt einen Abbruch zu, den nur einer seiner vier Knoten trägt, und dasselbe Bild führt neun Kästen für die sechs Stationen, die die Prosa nennt. Keiner der drei verdeckt das Design; jeder kostet den Leser einen zweiten Durchgang, und der dritte hängt an einem Abnahmekriterium.

## Per-diagram measurements

| # | Typ | Knoten | Kanten | Max. Ausgangsgrad | Max. Eingangsgrad | Zyklen | Geschichtet | Waisen | Urteil |
|---|---|---|---|---|---|---|---|---|---|
| 1 | flowchart TD | 10 | 9 | 3 (`Q`) | 3 (`PR`) | 0 | ja (4 Teilgraphen) | 0 | acceptable |
| 2 | flowchart LR | 12 | 11 | 2 (`S0`) | 1 | 0 | ja (2 Teilgraphen, je `direction TB`) | 0 | acceptable |

Dichte: 0,90 und 0,92 Kanten je Knoten. Beide Graphen liegen damit unter der Baumgrenze und tragen keine Spur eines Knäuels. Der Typ passt in beiden Fällen: ein gerichteter `flowchart` für einen Herkunftsgraphen und für eine Stationsfolge mit Abbruchzweig ist die Wahl, die die Typentabelle vorsieht.

## Findings

**B1 (mittel, Bild 1): Die Kantenverben zeigen in zwei Richtungen.** `Q -->|erbt| ENV`, `Q -->|erbt| PL` und `PL -->|liest| UE` benennen, was das *Ziel* mit der *Quelle* tut. `ENV -->|schreibt| TL` benennt das Umgekehrte. Wer den Pfeilen folgt, liest „Q erbt ENV" und „PL liest UE", und beide Lesarten stehen quer zum gemeinten Sinn. Das trifft ein Bild, dessen erklärter Zweck laut der Prosa darüber die Richtung ist: „keiner der drei Wege hat einen zweiten Ursprung". Am Design ändert der Befund nichts, an der Belastbarkeit des Belegs schon. Sauber wird es, wenn alle vier Verben dieselbe Seite meinen, etwa `Q -->|"vererbt an"| ENV` und `PL -->|"wird gelesen von"| UE`.

**B2 (mittel, Bild 2): Der Teilgraph sagt einen Abbruch zu, den nur einer seiner vier Knoten trägt.** BILLIG heißt „billig, bricht ab, bevor etwas kostet", und die einzige Kante nach `ABBRUCH` geht von `S0` aus. `S1` (AppKit-Grenze), `SID` (Identitätssuche) und `SZ` (Zielprüfung) brechen heute ebenfalls ab; für die Identitätssuche sagt `CLAUDE.md` es wörtlich, der Bündelbau „bricht ohne Bündel ab, wenn keine greift". Der Graph behauptet also weniger, als der Baum hergibt, und ein Leser schließt daraus, die neue Station sei die einzige Weigerung auf dem Weg. Zwei saubere Auflösungen: drei weitere gepunktete Kanten nach `ABBRUCH`, oder eine Teilgraph-Beschriftung, die den gezeigten Ausschnitt auf die neue Station beschränkt.

**B3 (mittel, Bild 2): Neun Kästen für sechs Stationen.** Beschriftet sind `1 · AppKit-Grenze` bis `6 · beglaubigen und anheften`. Dazwischen stehen `Identitätssuche` und `Zielprüfung` ohne Nummer und ohne optische Unterscheidung, davor die neue Station ohne Nummer. Die Prosa nennt sechs Stationen und zählt im selben Satz acht Verrichtungen auf. Das wäre eine Kleinigkeit, hinge nicht C3.9 daran: das Kriterium wird „über die Reihenfolge der Stationen" abgenommen, und wer Stationen zählen soll, findet im Bild keine Zahl, auf die er sich stützen kann. Die beiden unnummerierten Kästen als `1a` und `1b` zu führen oder sie unter Station 1 zu nesten, kostet eine Zeile und stellt die Zählbarkeit her.

**B4 (niedrig, Bild 1): Die Endknoten `AB` und `WEG` tragen die Auslieferungsfolge in ein Herkunftsbild.** Dieselbe Verzweigung steht in Bild 2 als `ABBRUCH` und `FERTIG`. Eine Entscheidung, zwei Darstellungen, ein Dokument. Die Herkunftsaussage verliert nichts, wenn beide Knoten entfallen und Bild 1 an `PR` endet. Streng nach „ein Bild, ein Anliegen" ist das der Punkt, an dem Bild 1 über sein Thema hinausgreift; da es bei zehn Knoten bleibt, verdeckt es nichts.

**B5 (niedrig, Bild 1): Zwei von neun Kanten sind unbeschriftet.** `TAG --> PR` und `BAUM --> PR`. Die Bedeutung trägt die Teilgraph-Beschriftung „was die Zahl deckt (C3)", und der Leser kommt ohne Verb aus. Kosmetisch.

**B6 (niedrig, Prosa gegen Graph): Vier Rollen in der Prosa, drei Kanten im Bild.** „Nach dieser Runde hat die Zahl aus der `Cargo.toml` drei Abnehmer statt einen, und einen Prüfer." `Q` hat den Ausgangsgrad 3, und der dritte dieser Wege *ist* der Prüfer. Die Zählung ist damit um eine Rolle zu großzügig, oder der vierte Weg fehlt im Bild. Der Folgesatz („keiner der drei Wege") legt die erste Lesart nahe; entschieden ist es nicht.

**Kein fehlendes Diagramm.** Die zwei strukturellen Behauptungen dieses Spec sind die Herkunft der Zahl und die Reihenfolge der Stationen, und für beide liegt ein Graph vor. Die Anordnung der Titelleiste aus C1 und C2 ist ein Layout und keine Struktur im Sinne der Typentabelle; sie verlangt kein drittes Bild.

## What a clean redraw would require

Nicht einschlägig. Das Urteil lautet *acceptable*, und kein Befund verlangt eine andere Struktur. B1 bis B3 sind an Ort und Stelle zu beheben, ohne einen Knoten oder eine Kante zu verschieben.
