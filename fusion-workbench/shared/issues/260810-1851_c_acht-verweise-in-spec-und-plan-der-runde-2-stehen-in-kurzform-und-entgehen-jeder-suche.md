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

---
Resolved: **Neun Stellen ausgeschrieben, nicht acht.** Das Suchmuster dieses Datensatzes fand
sie nicht: die Stellen tragen **drei ASCII-Punkte**, nicht das Zeichen `…`. Mit
`…-]*(\.\.\.|…)` erhoben ergaben sich neun Treffer. Die acht gemeldeten stimmen in Datei und
Zeile; der neunte ist `plan:1587`, der die Sternstelle schon trug und allein im Namen gekuerzt
war — deshalb konnte ihn auch keine Erhebung nach *festen* Markern finden.

Jeder volle Name ist gegen den Dateibestand aufgeloest, keiner geraten. Vorbild war `c0b96a6`:
Verzeichnisanteil bleibt, Marker wird Sternstelle, Name wird vollstaendig, Endung kommt dazu.

**Eine Abweichung vom reinen Ausschreiben, und sie war noetig:** bei `spec:556` ist der
Circle-Pfad ergaenzt. Die zwei Kurzformen standen ohne Verzeichnisanteil, und ein blosser
Dateiname wird relativ zum eigenen Circle gelesen — die zwei Datensaetze liegen im Circle der
Runde 1, der ausgeschriebene Name ohne Pfad liefe ins Leere.

**Zwei Absaetze im Reconciliation Log des Plans sind als Nachtrag berichtigt**, in der Handhabung
jener Datei. Der Eintrag vom 260810-0805 zaehlte "sieben Verweise", zaehlte aber Zeilen — es
waren neun Verweise —, und behauptete "der Spec traegt null", waehrend er zwei trug. Der Eintrag
vom 260810-1404 sagte "alle Verweise dieses Plans loesen auf"; die 40 geprueften waren nicht
alle, sieben standen in der Kurzform und fielen aus dem Muster, darunter ein toter Verweis.

**Die Lehre dieses Datensatzes bleibt ausserhalb dieses einen Ordners unumgesetzt:** jedes
Suchmuster des Projekts, das `\.md` verlangt, hat denselben blinden Fleck. Sie ist in den zweiten
Nachtrag aufgenommen. Der genannte Zusammenhang mit
`260810-1730_*_die-erzeugung-von-portfolio-md-…` bleibt offen.

Geschlossen in der Sitzung `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/history/260811-0107-orchestrator-session.md`.
