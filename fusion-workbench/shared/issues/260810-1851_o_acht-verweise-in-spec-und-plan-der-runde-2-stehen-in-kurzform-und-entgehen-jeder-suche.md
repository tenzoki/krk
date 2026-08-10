Acht Verweise in Spec und Plan der Runde 2 stehen in Kurzform und entgehen jeder Suche

---

Spec und Plan der Runde 2 tragen acht Verweise auf Datensatzdateien, die den Dateinamen mit
Auslassungspunkten abkürzen statt ihn auszuschreiben, etwa `260808-1413_o_…` ohne `.md`. Beide
Suchmuster, mit denen dieses Projekt bisher nach überholten Zustandsmarkern gesucht hat, greifen
nicht: sie verlangen die Endung `.md`. Die Stellen sind damit für jede Erhebung unsichtbar, und
eine von ihnen ist bereits falsch.

---

**Schwere:** Niedrig
**Gefunden:** ontocoder, bei der Behebung von
`shared/issues/260810-1746_*_spec-und-plan-der-runde-2-tragen-sechs-verweise-mit-ausgeschriebenem-zustandsmarker.md`
**Domain:** data

## Die acht Stellen

In `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/`:

| Datei | Zeilen |
|---|---|
| `260807-2147_*_spec-eingebauter-editor-mit-textmarken.md` | 556 (zweimal) |
| `260808-0140_*_plan-eingebauter-editor-mit-textmarken.md` | 492, 690, 701, 716, 853, 884 |

**Bereits falsch:** `plan:716` zitiert `260808-1413_o_…`; die Datei trägt `_c_`.

## Warum das ein eigener Datensatz ist

Der Befund über die sechs ausgeschriebenen Marker ist mit derselben Sitzung behoben. Diese acht
Stellen haben eine **andere Gestalt**, und die Behebung ist keine Ersetzung, sondern ein
Ausschreiben: der volle Name muss ermittelt und eingesetzt werden. Das falsifiziert daneben zwei
Absätze im `## Reconciliation Log` des Plans, die Zählungen über die Verweise führen. An den
behobenen Befund angehängt wäre das eine zweite Aufgabe unter fremdem Titel.

Der Plan führt sechs der acht bereits selbst als offenen Befund. Eine siebte Stelle derselben
Liste ist am 260810-1404 mit vollem Namen und Sternform behoben worden; sie ist das Vorbild für
die Behebung der übrigen.

## Was daraus folgt

Das eigentliche Ergebnis ist nicht die Liste, sondern die Erkenntnis über die Erhebung: **jedes
Suchmuster dieses Projekts, das `\.md` verlangt, hat einen blinden Fleck.** Der Befund über die
veralteten Marker ist inzwischen fünfmal erhoben worden, und keine dieser fünf Erhebungen konnte
diese acht Stellen sehen. Wer den nächsten Durchgang fährt, erweitert das Muster, bevor er zählt,
sonst zählt er wieder an derselben Stelle vorbei.

Zusammenhang: `shared/issues/260810-1730_*_die-erzeugung-von-portfolio-md-schreibt-den-zustandsmarker-aus-und-macht-jede-handkorrektur-zunichte.md`
beschreibt dieselbe Ursache an einer dritten Stelle. Wer diese Datensätze anfasst, prüft zuerst,
ob eine gemeinsame Antwort trägt.
