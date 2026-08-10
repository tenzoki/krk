# Der Plan zitiert einen Defekt mit einem Zeitstempel, den sechs Datensätze tragen

---
**Domain:** knowledge
**Schwere:** Low
**Gefunden von:** coder, bei der Behebung von `issues/260808-1413_o_vier-platzhalter-...`
**Betroffen:** `planning/260808-0140_c_plan-eingebauter-editor-mit-textmarken.md`, Abschnitt `### Wie diese sechs Schritte geschnitten sind`
**Cross-references:** `issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`, `issues/260808-1413_c_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md`

---

## Der Befund

Der Abschnitt begründet den Zuschnitt der sechs Schritte des Nachtrags mit einer
Lehre aus drei Defekten:

> fünf Schritte mussten Stellen außerhalb ihres Umfangs mitziehen, weil der Plan
> nach Sachthema schnitt statt nach Übersetzbarkeit (`issues/260808-0931_c_...`,
> `issues/260809-1640_c_...`, `issues/260808-1413_o_...`)

Die ersten beiden Kürzungen sind eindeutig: zu `260808-0931` und zu `260809-1640`
gibt es je einen Datensatz. Die dritte ist es nicht. **Den Zeitstempel
`260808-1413` tragen sechs Datensätze** (gezählt am 260810-0918 mit
`ls 260808-1413*` im Issue-Speicher dieses Circles), und drei von ihnen standen
zum Zeitpunkt des Zitats auf offen:

- `260808-1413_c_breite-aendern-traegt-einen-auffangzweig-ueber-bereich-und-hat-den-fuenften-wert-geschluckt.md`
- `260808-1413_c_der-wert-navigator-ist-dokumentiert-als-truegen-ihn-schon-drei-befehle.md`
- `260808-1413_c_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md`
- `260808-1413_o_die-begruendung-zu-syntect-nennt-den-transitiven-fussabdruck-nicht.md`
- `260808-1413_o_umlaufen-behauptet-die-eine-stelle-des-umlaufs-zu-sein-voriger-laeuft-daneben-um.md`
- `260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`

Das Zitat identifiziert keinen von ihnen. Zwei der drei offenen sind seit dem
260810-0918 behoben und werden auf geschlossen wandern; an der Sache ändert das
nichts, weil dann noch vier Kandidaten bleiben.

## Warum das zählt

Der Satz trägt eine Begründung und keine Beobachtung: er sagt, **warum** die
sechs Schritte anders geschnitten sind als die 42 davor. Wer die Begründung
nachprüfen will, muss den Datensatz lesen können, auf den sie sich stützt. Mit
sechs Kandidaten liest er sechs oder keinen.

## Warum der Behebende ihn nicht selbst geschlossen hat

Zwei der sechs passen inhaltlich, und die Wahl ist aus dem Satz allein nicht
entscheidbar:

- Der zweite Halbsatz des Absatzes lautet "und keiner hinterlässt eine Zeile,
  die auf ihren Ablöser wartet". Das ist wörtlich der Gegenstand von
  `..._o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht...`, und dessen
  Marker stimmt mit dem zitierten `_o_` überein.
- Der erste Halbsatz lautet "fünf Schritte mussten Stellen außerhalb ihres
  Umfangs mitziehen". Das ist der Gegenstand von
  `..._c_ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md`,
  wo S19 vorgezogen werden musste; sein Marker stimmt dann aber nicht.

Ein Raten zwischen beiden wäre eine Behauptung über die Absicht des Planners.
Der `coder` hat den Satz deshalb unangetastet gelassen und diesen Datensatz
angelegt.

## Was zu tun ist

Die Kürzung durch den vollen Dateinamen ersetzen, oder beide Datensätze nennen,
falls beide gemeint waren. **Zuständig:** wer den Absatz geschrieben hat, oder
der nächste Abgleich. Kein Schritt und kein Bau ist betroffen.

## Nebenbefund zur Zitierform

Der Plan legt unter `## Wie dieser Plan auf Datensätze verweist` fest, dass ein
Verweis den Zustandsmarker nicht trägt, sondern eine Sternstelle. Die drei
Kürzungen in diesem Satz tragen ihn (`_c_`, `_c_`, `_o_`) und brechen damit die
eigene Regel des Plans. Das ist derselbe Satz und beim Berichtigen mit zu
erledigen.
