# Beendet ein Tabwechsel den Durchlauf des verlassenen Tabs, jetzt wo er Dateien liest?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `shared/planning/260816-1310_o_spec-inhaltsfilter-der-dateiliste.md` (C4.5); `crates/krk-ui/src/tabs.rs:770-773` (die Begründung der Runde 10, warum ein Tabwechsel gerade **nicht** abbricht) und `:790-822` (`durchlauf_nachziehen_an`); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-2102_c_plan-tippen-filtert-dateiliste-flach-und-tief.md` (Strang F)

---

## Question

Der Spec dieser Runde verlangt in C4.5, dass ein Ordnerwechsel **und ein Tabwechsel** den Durchlauf der verlassenen Ansicht beenden. Die Runde 10 hat das Gegenteil gebaut und ausdrücklich begründet: `Tabliste::durchlauf_nachziehen` wird beim Tabwechsel nicht gerufen, weil ein verdeckter Tab sich still weiterfüllen soll und ein Abbruch dem Nutzer gerade die Arbeit wegnähme, die er beim Zurückwechseln bräuchte (`tabs.rs:770-773`).

Beide Aussagen können nicht zugleich gelten, und der Widerspruch ist keine Formulierungsfrage. Für den Namensdurchlauf über Verzeichnismetadaten wiegt die Begründung der Runde 10 leicht: er ist in aller Regel in Millisekunden durch, und ihn abzubrechen kostet beim Zurückwechseln kaum etwas. Der Inhaltsdurchlauf ändert das Gewicht. Er öffnet und liest Dateien bis 1 MB, über einen Unterbaum minutenlang, für einen Tab, den niemand ansieht, und er teilt sich die Deskriptortabelle mit dem Editor, der Vorschau, den Kopiervorgängen und dem Lesevorgang des zweiten Dateifensters.

## Options

1. **Ein Tabwechsel beendet den Durchlauf des verlassenen Tabs, gleich welcher Art.** Eine Regel, kein Zweig nach der Art des Laufs. Der Rückwechsel stößt ihn über denselben Weg wieder an, den ein Ordnerwechsel benutzt.
   - Pro: C4.5 gilt wörtlich. Eine Regel statt zweier, und sie ist an jeder Stelle entscheidbar. Kein verdeckter Tab liest Dateien.
   - Kontra: die Zusage der Runde 10, dass ein verdeckter Tab sich weiterfüllt, fällt auch für den reinen Namensdurchlauf. Wer zwischen zwei Tabs mit stehendem Filter hin- und herwechselt, lässt den Unterbaum jedes Mal von vorn abschreiten.
2. **Ein Tabwechsel beendet nur einen Inhaltsdurchlauf.** Ein reiner Namensdurchlauf läuft weiter wie heute.
   - Pro: die Runde 10 bleibt in ihrem Verhalten unberührt, und die teure Arbeit hört auf.
   - Kontra: zwei Regeln für einen Vorgang, unterschieden nach einer Eigenschaft des Laufs. Der Nutzer sähe zwei verschiedene Verhaltensweisen desselben Schalters, je nachdem, ob „Content" steht.
3. **Kein Abbruch beim Tabwechsel; C4.5 wird um den Tabwechsel gekürzt.**
   - Pro: nichts zu bauen, die Runde 10 bleibt unberührt.
   - Kontra: ein verdeckter Tab liest dann Dateien über einen Unterbaum, ohne dass etwas davon auf einem Schirm steht, und er nimmt dabei Deskriptoren aus einem Vorrat, den fünf andere Teile von KRK teilen. Der Nutzer hat keinen Weg, das zu bemerken oder zu beenden, außer den Filtertext des verdeckten Tabs zu löschen.

## Constraints

- Je Tab läuft nie mehr als ein Durchlauf (C3.8). Keine Möglichkeit hier rührt daran.
- Ein Ordnerwechsel beendet den Durchlauf in jedem Fall; das ist gebaut und nicht Gegenstand dieser Frage.
- Der Abbruch wartet nicht auf den Arbeitsfaden (C4.6). Auch das ist gebaut: `Drop` setzt das Abbruchkennzeichen und kehrt zurück.
- Der Einzugstakt läuft heute über **alle** Tabs und nicht nur über den sichtbaren. Eine Antwort, die verdeckte Läufe zulässt, lässt diesen Zuschnitt unangetastet; Möglichkeit 1 macht ihn gegenstandslos, ohne ihn zu ändern.

## Recommendation

Möglichkeit 1. Der Plan fährt darauf, und was sich bei einer anderen Antwort ändert, steht dort an Schritt D1.

Die Begründung ist die eine Regel. Möglichkeit 2 kauft ein kleines Stück Bequemlichkeit mit einer dauerhaften Fallunterscheidung, die der Nutzer als zwei Verhaltensweisen desselben Schalters erlebt; das ist genau die Form, die dieses Projekt an anderen Stellen vermeidet. Was Möglichkeit 1 kostet, ist benannt und klein: ein wiederholter Namensdurchlauf über Verzeichnismetadaten, also die Arbeit, die die Runde 10 selbst in Millisekunden veranschlagt.

## Nutzerentscheid vom 260816-1410: Möglichkeit 1

**Ein Tabwechsel beendet den Durchlauf des verlassenen Tabs, gleich welcher Art.**
Eine Regel, kein Zweig nach der Art des Laufs, an jeder Stelle entscheidbar.
C4.5 gilt wörtlich. Der Rückwechsel stößt den Lauf über denselben Weg wieder an,
den ein Ordnerwechsel benutzt.

**Der Preis ist benannt und angenommen:** die Zusage der Runde 10, dass ein
verdeckter Tab sich weiterfüllt, fällt auch für den reinen Namensdurchlauf. Wer
mit stehendem Filtertext zwischen zwei Tabs hin- und herwechselt, lässt den
Unterbaum jedes Mal von vorn abschreiten. Der Nutzer hat das gegen die Aussicht
abgewogen, dass ein unsichtbarer Tab minutenlang Dateien liest und dabei
Deskriptoren aus einem Vorrat nimmt, den fünf andere Teile von KRK teilen.

**Kein Datensatz der Runde 10 ist zu überholen.** Nachgesehen am 260816 in
`circles/260814-1551-.../decisions/` und im Spec jener Runde: die Zusage über den
verdeckten Tab steht dort in keinem Abnahmekriterium und in keinem Entscheid,
sondern allein als Begründung im Doc-Kommentar von `Tabliste` (`tabs.rs:770-773`).
Sie wird deshalb nicht überholt, sondern nachgezogen — Schritt D1 des Plans.

---
Answered: circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/decisions/260816-1359_a_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md §Nutzerentscheid — Möglichkeit 1, eine Regel für jeden Durchlauf; der Doc-Kommentar der Runde 10 wird in Schritt D1 nachgezogen.
Implemented: 09baffd — Möglichkeit 1 gebaut, und sie brauchte eine vierte Bedingung: der bloße Ruf von `durchlauf_nachziehen_an` auf der verlassenen Stelle hätte den Lauf abgebrochen und im selben Zug neu gestartet, weil Filtertext und Schalter dort stehen bleiben. Die Regel liegt deshalb im Rumpf jener Methode und lautet, dass ein verdeckter Tab keinen Durchlauf bekommt. Der Kommentar der Runde 10 ist ersetzt und nennt Datensatz, Abwägung und Preis. Ein Folgedefekt ist dabei gefunden und abgelegt: `issues/260816-1710_o_ein-rueckwechsel-auf-einen-tab-setzt-seinen-beendeten-durchlauf-nicht-fort.md`.
Deferred:
Superseded by:
