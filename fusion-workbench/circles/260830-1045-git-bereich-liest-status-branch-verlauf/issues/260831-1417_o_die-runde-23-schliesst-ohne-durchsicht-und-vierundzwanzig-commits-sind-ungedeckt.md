Die Runde 23 schließt ohne Durchsicht, und vierundzwanzig Commits sind ungedeckt

---
Der Plan der Runde 23 führt siebzehn Schritte, und keiner davon ist eine Durchsicht. Weder `coderev` noch `ontorev` ist in dieser Runde gelaufen: `circles/260830-1045-git-bereich-liest-status-branch-verlauf/reviews/` ist leer, und der jüngste Eintrag in `shared/reviews/` stammt vom 260826, also aus der Zeit vor der Runde.

Der Deckungsmesser sagt dasselbe in Zahlen. `bin/fusion-review-coverage` am Sitzungsstand:

```
anchor=workbench-root  since=d1fbaac  head=HEAD
commits=24  reviews=0  unusable=0  uncovered=24  verdict=uncovered
```

**Die Runde hält damit ihre eigenen Endbedingungen ein und trotzdem nicht die Gewohnheit des Projekts.** Der Plan verlangt unter `## Where this Circle stops` keine Durchsicht, und CLAUDE.md bindet sie allein an eine Auslieferung („Wird eine gefahren, geht ihr die Durchsicht der Runde voraus"); eine Auslieferung ist nicht gefahren, der Arbeitsbaum trägt weiter `version = "1.4.0"` und HEAD keinen Tag. Nach dem Buchstaben fehlt also nichts. Die Runde 21 hat ihre Durchsicht dagegen ohne Auslieferungsanlass gefahren (`097abc2`), und die Runde 23 ist die umfangreichste seit langem: eine fremde Kiste mit 197 Paketen auf dem Bauziel, ein sechster Bereich, ein sechster Fokuswert, eine fünfte Spalte, ein neuer Arbeitsfaden mit Kanal und ein zweiter Befundvektor im Ordnermodell.

**Was das kostet, ist nicht die Grünfärbung, sondern die zweite Lesart.** `make check` ist grün, 885 Proben in `krk-ui` und 234 im Kern laufen durch, und der Abgleich hat jede behauptete Erledigung gegen den Baum gelesen. Was keine dieser Prüfungen leistet, ist die Frage, ob die Bauform selbst richtig gewählt ist — genau die Frage, aus der die neun stillen Nachzugsstellen dieser Runde hervorgegangen sind und aus der die offene Nutzerfrage `260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md` lebt.

**Der Fall ist im Projekt schon einmal aufgeschrieben worden**, mit derselben Diagnose und einer Runde Abstand: `shared/issues/260826-2205_*_der-deckungsmesser-meldet-am-sitzungs-head-ungedeckt-…` und `shared/decisions/260815-1812_*_der-eine-codecommit-der-sitzung-260815-1328-ohne-durchsicht-ist-nicht-nur-markdown.md`, letzterer offen.

**Abnahme:** entweder ist eine Durchsicht über `d1fbaac..HEAD` gefahren und liegt unter `reviews/` dieses Circles, oder der Nutzer hat ausdrücklich entschieden, dass diese Runde ohne eine schließt, und die Entscheidung steht als Datensatz. Der zweite Weg ist die günstigere Antwort, wenn die Durchsicht der Stufe B vorbehalten bleiben soll, in der der Schreibweg entsteht.

---
**Filed by:** reconciler, Kai Stalmann <kai@qantr.com>
**Domain:** code
Gefunden beim Abgleich zum Abschluss der Runde 23, mit `bin/fusion-review-coverage` gegen den Stand vor der Runde.
Verwandt: `260819-1440_*_was-sagt-der-marker-c-an-einem-spec-gebaut-oder-abgenommen.md` — dieselbe Trennung zwischen „gebaut" und „geprüft", eine Ebene höher.
