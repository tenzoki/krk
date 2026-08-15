# Schreiben Zitate im Code den Marker aus oder die Sternform?

---
**Domain:** code
**Status:** open
**Filed by:** orchestrator
**Cross-references:** `shared/issues/260815-1047_c_vier-verweise-im-code-nennen-einen-marker-den-ihr-ziel-nicht-mehr-traegt-drei-davon-sind-neu.md` (der Anlass); `CLAUDE.md` (`## Bindende Grundlage: die Entscheidungsdatensatze`, die Markertabelle); `shared/issues/260810-1851_*_acht-verweise-in-spec-und-plan-der-runde-2-stehen-in-kurzform-und-entgehen-jeder-suche.md` (derselbe Gegenstand von der Suchseite her)

---

## Question

Zitiert eine Kommentarzeile im Code einen Entscheidungs- oder Defektdatensatz, schreibt sie heute dessen Marker aus: `260814-1830_a_bleibt-der-filtertext-…`. Der Marker wandert aber, und zwar planmäßig: ein Entscheid läuft `_o_` → `_a_` → `_i_`, ein Defekt `_o_` → `_p_` → `_c_`. Jedes ausgeschriebene Zitat wird damit im Lauf seines Ziels ein- bis zweimal falsch, ohne dass irgendetwas es meldet.

Am 260815 hat das in einer einzigen Sitzung dreimal zugeschlagen: der `coder` schrieb drei Zitate auf `_a_`, der Orchestrator zog den Datensatz eine halbe Stunde später auf `_i_`, und die Durchsicht fand die drei Verweise ins Leere. Beim Berichtigen kamen vier weitere aus früheren Runden dazu, insgesamt sieben von siebzehn geprüften.

Die Frage ist jetzt zu stellen, weil sieben Stellen gerade berichtigt worden sind: sie sind heute richtig und werden mit der nächsten Zustandsänderung ihrer Ziele wieder falsch.

## Options

1. **Bei der ausgeschriebenen Form bleiben und von Hand nachziehen.**
   - Pro: ein Zitat sagt beim Lesen, in welchem Zustand der Datensatz war, als der Kommentar entstand. Das ist eine echte Auskunft und geht bei jeder anderen Form verloren.
   - Kontra: die Nachführung hängt daran, dass jemand daran denkt. Gemessen: sieben von siebzehn waren falsch, und niemand hat es bemerkt, bis eine Durchsicht ausdrücklich danach suchte.
2. **Die Sternform `_*_` schreiben**, wie `fusion` sie für seine eigene `portfolio.md` vorschreibt: `260814-1830_*_bleibt-der-filtertext-….md`.
   - Pro: ein Zitat, das nicht altert. Der Leser löst den Stern gegen den Speicher auf und liest den heutigen Marker am gefundenen Dateinamen ab. Kostet ihn nichts, was er nicht ohnehin tut.
   - Kontra: der Zustand beim Entstehen des Kommentars ist nicht mehr abzulesen. `CLAUDE.md` führt die Sternform bisher nur für die Aufzeichnungen eines Standes und nicht für Code.
3. **Den Marker ganz weglassen** und nur Zeitstempel und Thema zitieren: `260814-1830 bleibt-der-filtertext-…`.
   - Pro: kürzer als beides, und ebenfalls alterungsfrei.
   - Kontra: bricht die Suchmuster des Projekts. `CLAUDE.md` hält unter `## Bindende Grundlage` ausdrücklich fest, dass Verweise in Kurzform jeder Suche entgehen, die `\.md` verlangt; dieser Fehlertyp ist unter `shared/issues/260810-1851_*_…` schon einmal aufgenommen.

## Constraints

- Die Antwort gilt für Zitate **im Code**. Für die Aufzeichnungen eines Standes gilt weiter die Ortsregel aus `CLAUDE.md`: `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/` behalten ihren damaligen Marker, und dort ändert diese Frage nichts.
- Jede Antwort außer Möglichkeit 1 verlangt einen einmaligen Umbau der heute siebzehn Fundstellen unter `crates/` und `xtask/`.
- Ohne Prüfung bleibt jede Antwort eine Verabredung. Ob eine Zählprobe oder ein `xtask`-Ziel die Zitate gegen den Dateibestand hält, gehört zur Antwort dazu, denn Möglichkeit 1 ist ohne Prüfung nachweislich nicht haltbar.

## Recommendation

Möglichkeit 2, und zwar aus dem Grund, den Möglichkeit 1 gerade selbst geliefert hat: sie ist am 260815 in einer einzigen Sitzung dreimal gebrochen worden, von zwei verschiedenen Bearbeitern, ohne dass es jemandem auffiel. Die Auskunft, die dabei verlorengeht — welchen Zustand der Datensatz beim Schreiben des Kommentars trug —, steht ohnehin genauer im Datensatz selbst, in seinen Zeilen `Answered:` und `Implemented:`.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: `shared/history/260815-0912-orchestrator-session.md` — Möglichkeit 2, Nutzerentscheid vom 260815-1230: die Sternform `_*_`, **ohne** die Prüfung im Bau. Die Empfehlung des Datensatzes ist damit angenommen, ihr Zusatz aus dem Abschnitt `## Constraints`, ohne Prüfung bleibe jede Antwort eine Verabredung, ist bewusst in Kauf genommen.

**Zwei Grenzen der Antwort, benannt statt verschwiegen.** Die Sternform hält gegen einen Markerwechsel und gegen nichts sonst: wird der Namensteil eines Datensatzes umgeschrieben, zeigt das Zitat weiter ins Leere. Genau das ist am 260815 geschehen, als der Datensatz zur Wettrennprobe zweimal umbenannt wurde. Und ohne Prüfung im Bau bemerkt weiterhin niemand ein totes Zitat, bis jemand ausdrücklich danach sucht.

**Der Geltungsbereich ist der lebende Text und nicht die Aufzeichnung eines Standes.** Nach der Ortsregel in `CLAUDE.md` (`## Bindende Grundlage`) behalten `history/`, `reviews/`, `analyses/`, `issues/`, `decisions/`, `messungen/` und `spikes/` je Datei ihren damaligen Marker; dort ändert diese Antwort nichts. Umgestellt wird, was heute gilt: `crates/`, `xtask/`, `CLAUDE.md`, die Circle-Datensätze und die Spec- und Plandateien unter `planning/`.

**Die Ausnahme innerhalb des Geltungsbereichs bleibt bestehen:** wo der Marker die Aussage selbst ist, etwa in einer Befundtabelle mit den Spalten „zitiert" und „ist" oder in einem Satz über einen Zustandswechsel, bleibt der Buchstabe stehen. Die Probe dafür ist, was der Stern kostet: ein Zeiger auf eine Datei verliert nichts, eine Aussage über einen Zustand verliert ihren Inhalt.
