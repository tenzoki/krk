# Concept Evaluation: Plan Titelleiste führt Version und semantische Tags

**Date:** 2026-08-13 11:24
**Target:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`
**Verdict:** acceptable
**Diagrams evaluated:** 3  |  **Validation:** by-tool (`@mermaid-js/mermaid-cli` 11.16.0, alle drei Blöcke nach SVG gerendert)

## Verdict

**acceptable, und kein Befund benennt einen Fehler am Entwurf.** Alle drei Graphen sind azyklisch, keiner trägt einen freistehenden Knoten, und der höchste Ausgangsgrad im ganzen Dokument beträgt zwei. Ein Gott-Knoten kann bei diesem Wert nicht vorliegen. Die Dichte liegt bei 1,06, 1,09 und 0,92 Kanten je Knoten, also im Bereich eines Baumes mit wenigen Zusammenführungen. Der Entwurf, den die Bilder zeigen, ist drei unabhängige Arbeitsstränge mit genau einer benannten Naht, eine reine Funktion mit vier Eingaben und einem ausdrücklich gezeichneten Nebenweg, und eine lineare Kette mit einer billigen Sperre ganz vorn. Was das Urteil von *clean* auf *acceptable* zieht, ist ein einziger Befund, und er ist geerbt: Bild 3 zeichnet im billigen Vorlauf eine Weigerung, wo der Baum an fünf Knoten weigern kann. Der Diagrammprüfer des Spec hatte denselben Punkt als B2 benannt, der Plan erledigt seinen Nachbarbefund B3 vollständig und geht auf B2 nicht ein.

## Per-diagram measurements

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgangsgrad | Max. Eingangsgrad | Zyklen | Geschichtet | Waisen | Urteil |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | flowchart TD | 16 | 17 | 1,06 | 2 (`A2`, `D2`) | 5 (`E1`) | 0 | ja (4 Teilgraphen, je `direction TB`) | 0 | acceptable |
| 2 | flowchart TD | 11 | 12 | 1,09 | 2 (`SF`, `LAGE`, `KO`, `REGEL`) | 4 (`LAGE`) | 0 | teilweise (1 Teilgraph) | 0 | acceptable |
| 3 | flowchart LR | 13 | 12 | 0,92 | 2 (`S1`) | 1 | 0 | ja (2 Teilgraphen, je `direction TB`) | 0 | acceptable |

Bild 1 hat drei Quellen (`A1`, `B1`, `D1`) und eine Senke (`E2`), Bild 2 vier Quellen und drei Senken, Bild 3 eine Quelle und zwei Senken. Der Typ passt in allen drei Fällen: ein gerichteter `flowchart` deckt nach der Typentabelle sowohl die Abhängigkeitsordnung von Arbeitsschritten als auch einen Datenfluss und eine Stationsfolge ab. Für Bild 3 wäre ein `sequenceDiagram` die falsche Wahl, weil keine Beteiligten einander rufen, sondern ein Vorgang seine eigenen Stufen durchläuft.

## Findings

**F1 (mittel, Bild 3): Der billige Vorlauf zeigt eine Weigerung, wo fünf möglich sind.** Der Teilgraph `BILLIG` heißt „billig · bricht ab, bevor eine Uebersetzung laeuft", und die einzige Kante nach `AB` geht von `S1` aus, der neuen Tag-Prüfung. Am Baum nachgesehen, geben alle fünf Knoten dieser Phase einen `Result`-Typ zurück und brechen ab: `bundle::vorbereiten` (`xtask/src/bundle.rs:170`), `appkit_grenze_pruefen` (`xtask/src/release.rs:130`), `sign::bestimmen_fuer_release` (`xtask/src/sign.rs:81`) und `ziele_pruefen` (`xtask/src/release.rs:361`). Für die Identitätssuche sagt `CLAUDE.md` es wörtlich: der Bündelbau „bricht ohne Bündel ab, wenn keine greift". Die Beschriftung des Teilgraphen ist als Aussage über die Phase richtig, jeder Abbruch dieser Phase liegt vor dem ersten Übersetzungslauf. Der Leser, der Weigerungen zählt, entnimmt dem Bild dennoch, die neue Station sei die einzige. Am Entwurf ändert der Befund nichts, an der Belastbarkeit des Bildes als Beleg für C3.9 schon. Vier weitere gepunktete Kanten nach `AB` kosten vier Zeilen; eine Beschriftung, die den gezeigten Ausschnitt ausdrücklich auf die neue Station beschränkt, kostet eine.

**F2 (niedrig, Bild 1): Vier Teilgraphen für fünf benannte Stränge.** Die Überschrift kündigt „vier Stränge, eine Naht, eine Vorbedingung" an, und `SA` bis `SD` bilden diese vier. `E1` und `E2` stehen ohne Behälter daneben, während die Implementation Steps sie unter „### Strang E — Abnahme" führen und die Decidability-Zeile im Kopf ebenfalls von „Strang E" spricht. Die Asymmetrie ist sachlich begründbar, denn E ist der Zusammenlauf und kein paralleler Arbeitsstrang. Wer aber von der Schrittliste ins Bild schaut, sucht einen fünften Rahmen. Ein Teilgraph `SE` um die beiden Knoten stellt die Deckung her, ohne eine Kante zu verschieben.

**F3 (niedrig, Bild 2): Die Schichtung ist zur Hälfte gezeichnet.** `ERHEBUNG` fasst die vier Eingaben zusammen, die Abnehmerseite bleibt lose: `ABGRIFF`, `MENUE` und `ZEICHEN` liegen ohne Rahmen im Bild. Bei elf Knoten und einer Kantenzahl knapp über der Knotenzahl liest sich der Fluss trotzdem in einem Durchgang, von den Quellen über `LAGE` und `REGEL` zu den drei Senken. Ein zweiter Teilgraph um die beiden Frager würde die im Baum vorhandene Trennung sichtbar machen: `kommando_ausfuehren` und `validateMenuItem:` rufen die Regel, der Zeichenzweig ruft sie gerade nicht.

**Was Bild 2 richtig macht und hier genannt gehört.** Die Kante `LAGE -->|"drei Werte einzeln, unveraendert"| ZEICHEN` zeichnet den Nebenweg, den `CLAUDE.md` als stehende Falle dieses Baumes führt: zwei Stellen mit zwei verschiedenen Fragen, an denen schon einmal ein Fehlbefund entstanden ist. Ein Bild, das allein die Regel und ihre zwei Frager zeigte, wäre gefälliger und stellte genau die Lesart her, aus der der Fehlbefund `260810-1102` hervorging. Der Graph nimmt den Nebenweg auf und beschriftet ihn.

**Kein fehlendes Diagramm.** Die drei strukturellen Behauptungen dieses Plans sind die Abhängigkeitsordnung der Schritte, der Aufbau der Zulässigkeitsregel nach dem Umbau und die Reihenfolge der Auslieferungsstationen. Für jede liegt ein Graph vor. Die Anordnung des Titelleisten-Zusatzes aus C1 und C2 ist ein Layout und verlangt nach der Typentabelle kein Bild; die drei neuen Typen im Abschnitt „Data Structures" tragen keine Beziehung untereinander und wären als `classDiagram` drei unverbundene Kästen.

## Nachprüfung der Befunde am Spec

**B3 ist erledigt, und zwar vollständig.** Der Spec-Prüfer hatte beanstandet, dass neun Kästen sechs Stationen tragen und C3.9 „über die Reihenfolge der Stationen" abgenommen wird, ohne dass das Bild eine Zahl anbietet, auf die sich das Zählen stützen kann. Bild 3 des Plans führt die Zahlen 1 bis 7 lückenlos, und die drei Vorläufe tragen `Vorlauf a` bis `Vorlauf c` samt der Station, der sie zuarbeiten. Die gezeichnete Folge stimmt mit dem Quelltext überein: `ausfuehren` ruft heute `bundle::vorbereiten` (`release.rs:91`), `appkit_grenze_pruefen` (`:92`), `sign::bestimmen_fuer_release` (`:93`) und `ziele_pruefen` (`:101`), bevor die Übersetzungsschleife in Zeile 104 beginnt, und genau diese Verschränkung zeigt das Bild. Der Plan trägt die Zählung außerdem in einen eigenen Prosaabschnitt und in Schritt D3, der alle drei Stellen im Baum in einem Zug anfasst: den Modulkopf von `release.rs`, der heute „Der Weg in sechs Stationen" sagt, den Hilfetext in `main.rs` und `README.md:216-246`. Am Baum geprüft, nicht aus der Prosa übernommen.

**B1 kehrt nicht wieder.** Die Kantenverben des Plans zeigen alle in dieselbe Richtung: `KO -->|"traegt den Wirkungsbereich"| KO2` liest sich entlang des Pfeils als „Kommando trägt den Wirkungsbereich", und dasselbe gilt für die übrigen beschrifteten Kanten. Das Bild, an dem B1 hing, ist nicht in den Plan übernommen worden.

**B2 besteht fort.** Der Befund F1 oben ist derselbe Punkt an einem neu gezeichneten Bild. Der Plan nennt B3 zweimal ausdrücklich als erledigt, zu B2 sagt er nichts.

**B4 bis B6 sind gegenstandslos.** Alle drei betrafen das Herkunftsbild des Spec, das der Plan nicht fortschreibt.

## What a clean redraw would require

Nicht einschlägig. Das Urteil lautet *acceptable*, kein Befund verlangt eine andere Struktur, und keiner benennt einen Zyklus, einen Gott-Knoten oder eine fehlende Schicht. F1 bis F3 sind an Ort und Stelle zu beheben, ohne einen Knoten zu verschieben oder eine Abhängigkeit umzuhängen. Für das Nutzer-Tor heißt das: die Bilder tragen den Plan, und wer sie prüft, prüft die Beschriftung und nicht den Bau.
