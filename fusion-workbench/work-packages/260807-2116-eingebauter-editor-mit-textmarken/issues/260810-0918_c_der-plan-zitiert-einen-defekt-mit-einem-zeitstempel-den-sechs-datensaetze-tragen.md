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

---
Resolved: Am 260810-0933 im Plan `planning/260808-0140_c_plan-eingebauter-editor-mit-textmarken.md`, Zeile 1353, geschlossen. Alle drei Kürzungen tragen jetzt den vollen Dateinamen und die Sternstelle statt des Zustandsmarkers, wie `## Wie dieser Plan auf Datensätze verweist` es verlangt:

- `issues/260808-0931_*_s13-laesst-sich-nicht-allein-uebersetzen-die-speicherstelle-des-editors-kommt-erst-in-s14.md`
- `issues/260809-1640_*_der-fokus-kennt-den-editor-nicht-obwohl-der-abgriff-ihn-seit-s4-durchlaesst.md`
- `issues/260808-1413_*_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht-obwohl-der-plan-ihn-fuehrt.md`

**Die Mehrdeutigkeit ist aus dem Bestand entschieden und nicht geraten.** Von den sechs Datensätzen mit dem Zeitstempel `260808-1413` trägt genau einer den zitierten Satz, und er sagt es selbst: `..._vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht...` schreibt in seiner eigenen Abschlussnotiz, `### Wie diese sechs Schritte geschnitten sind` zitiere ihn „als einen von drei Anlässen dafür, die sechs Schritte des Nachtrags nach Übersetzbarkeit zu schneiden", und führt dazu wörtlich die Klausel „keiner hinterlässt eine Zeile, die auf ihren Ablöser wartet" an. Ein `grep` über alle sechs Datensätze nach „sechs Schritte", „Übersetzbarkeit" und „Uebersetzbarkeit" findet Treffer nur in diesem einen. Sein Marker `_o_` zum Zeitpunkt des Zitats passt außerdem zu der Form, die im Plan stand.

**Der zweite Kandidat ist damit ausgeschieden, und zwar aus zwei Gründen.** `..._ein-sichtbarer-bereich-editor-ohne-unteransicht-verliert-seine-breite-im-fenster.md` sagt nirgends, dass ein Schritt Stellen außerhalb seines Umfangs mitziehen musste; sein Gegenstand sind zwei Antworten auf die Frage, ob der Editor im Fenster steht, und aufgelöst ist er über den gemeinsamen Ausdruck `steht_im`. Das Vorziehen von S19, das dieser Datensatz hier als Grund für ihn nennt, steht nicht in ihm, sondern in `issues/260808-0931_*_...` — und der ist in derselben Klammer schon eigens zitiert. Ein zweiter Verweis auf denselben Sachverhalt hätte die drei Anlässe auf zwei verkürzt.

**Ein Nachbleibendes, außerhalb dieses Befunds:** Zeile 716 des Plans (Umsetzungsvermerk zu S15) zitiert `issues/260808-1413_o_vier-platzhalter-nennen-ihren-abloesenden-schritt-nicht...` in derselben Form, also gekürzt und mit Zustandsmarker. Diese Stelle liegt außerhalb des Abschnitts, den dieser Datensatz unter `**Betroffen:**` nennt, und ist deshalb unangetastet geblieben. Sie ist eindeutig auflösbar und wäre beim nächsten Abgleich mit zu ziehen.

Nachgewiesen mit `ls` je Verweis: alle drei Datensätze liegen im Issue-Speicher dieses Circles, Ausgang 0. Kein Schritt, keine `[DONE]`-Marke, keine Kopfzeile und kein Code ist angefasst.
