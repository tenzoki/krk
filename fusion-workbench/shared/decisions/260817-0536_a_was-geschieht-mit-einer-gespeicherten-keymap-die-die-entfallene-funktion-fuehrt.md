# Was geschieht mit einer gespeicherten keymap.toml, die die entfallene Funktion `endgueltig_loeschen` noch führt?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md` (C5), `shared/decisions/260817-0536_a_wie-wird-jeder-loeschweg-abgesichert-und-faellt-das-endgueltige-loeschen-weg.md`

---

## Question

Mit dem Wegfall des endgültigen Löschens verschwindet die Funktionskennung `endgueltig_loeschen` aus der ausgelieferten Belegung. Es ist das erste Mal in diesem Projekt, dass eine Kennung zurückgezogen wird, und die bestehende Ladelogik hat diesen Fall nie gesehen.

Wer seine Tastenbelegung je über die Belegungsansicht gesichert hat, trägt die Kennung in seiner `keymap.toml`: gesichert wird die vollständige Belegung und nicht nur das Geänderte (`Belegung::sichern` schreibt `Belegungsdatei::from(self)`). Beim nächsten Start prüft `Belegung::bauen` jede Kennung der Nutzerdatei gegen den Wortschatz der Auslieferung und liefert bei der ersten unbekannten `Belegungsfehler::UnbekannteFunktion`. `belegung::laden` fällt daraufhin auf die **vollständige Auslieferungsbelegung** zurück und meldet eine Ersetzung; die Datei auf der Platte bleibt stehen, wird aber nicht mehr gelesen.

Die Folge: ein Nutzer verliert jede eigene Tastenbelegung wegen eines einzigen Eintrags, den KRK selbst zurückgezogen hat, und erfährt es aus einer Zeile der Statuszeile beim Start. Die Frage gehört vor die Umsetzung, weil sie über einen Datenverlust an Nutzereinstellungen entscheidet und weil sie sich bei jeder späteren Runde wiederholt, die eine Funktion zurückzieht.

## Options

1. **Nichts ändern; das heutige Verhalten gilt** — die ganze Nutzerbelegung wird verworfen, die Auslieferung greift, die Statuszeile nennt Datei und Grund.
   - Pro: kein Eingriff, keine neue Fallunterscheidung. Das Verhalten ist bewusst gebaut: eine widersprüchliche Datei ergibt keine halbe Belegung.
   - Contra: der Nutzer verliert alle eigenen Kombinationen für einen Eintrag, an dem er nichts falsch gemacht hat. Er sieht nur eine Zeile, und die Zeile verschwindet mit dem nächsten Befehl.

2. **Eine unbekannte Kennung wird übergangen, der Rest der Datei gilt** — KRK liest die Belegung ohne den unbekannten Eintrag und meldet, welcher Eintrag übergangen wurde.
   - Pro: der Nutzer behält seine Arbeit. Der Fall ist genau der, für den die Meldung gedacht ist.
   - Contra: `UnbekannteFunktion` fängt heute auch echte Tippfehler in einer von Hand gepflegten Datei; danach würde ein vertippter Eintrag stillschweigend wirkungslos, statt aufzufallen. Die Unterscheidung „zurückgezogen" gegen „vertippt" verlangt eine Liste der zurückgezogenen Kennungen im Code, also einen neuen Speicher, der mit jeder Runde wächst.

3. **KRK zieht die Datei einmalig nach** — beim ersten Start nach der Runde entfernt KRK den Eintrag aus der `keymap.toml` und schreibt sie zurück, mit einer Meldung.
   - Pro: der Nutzer behält seine Arbeit, und das Problem tritt genau einmal auf.
   - Contra: KRK schriebe ungefragt in eine Datei, die der Nutzer von Hand pflegen darf. Die Ladelogik trägt heute die ausdrückliche Zusage, die Datei in jedem Fall stehen zu lassen, „ein Tippfehler darin darf die Arbeit des Nutzers nicht löschen"; diese Zusage fiele.

## Constraints

- Die Belegung des Nutzers ist von Hand änderbar; das Laden scheitert nie und liefert immer eine benutzbare Belegung.
- Es gibt genau eine Quelle der ausgelieferten Belegung, `resources/default-keymap.toml`. Eine zweite Liste daneben wäre die Verdopplung, die dieses Projekt vermeidet.
- Die Meldung geht über die Statuszeile, wie jede andere Ablagemeldung. Eine zweite Ausgabestelle entsteht nicht.

## Recommendation

Keine ohne die Antwort des Nutzers auf eine Vorfrage: wie oft rechnet er damit, künftig Funktionen zurückzuziehen? Bei „einmalig" trägt Möglichkeit 1 mit einer deutlicheren Meldung; bei „das kommt wieder" ist Möglichkeit 2 samt Liste der zurückgezogenen Kennungen die Investition wert. Eine Empfehlung ohne diese Auskunft wäre geraten.

## Antwort des Nutzers

**Am 260817, bei der Abnahme des Specs: Möglichkeit 1.** Es bleibt beim heutigen Verhalten. Eine `keymap.toml`, die `endgueltig_loeschen` noch führt, wird als Ganzes verworfen, die Auslieferungsbelegung greift, und die Statuszeile nennt die zur Seite gelegte Datei und den Grund. Ein neuer Sonderweg entsteht nicht; der Vorgang steht schon und wird nicht angefasst.

Der Nutzer nimmt den Verlust seiner eigenen Belegung ausdrücklich in Kauf. Die Vorfrage der Empfehlung, wie oft künftig Funktionen zurückgezogen werden, ist damit nicht beantwortet, sondern übergangen: die Antwort trägt für diese Runde, und eine spätere Runde, die eine zweite Kennung zurückzieht, darf die Frage neu stellen.

---
Answered: `shared/planning/260817-0536_o_spec-absicherung-jedes-loeschwegs.md`, Abschnitt `## Was der Nutzer entschieden hat` — Möglichkeit 1: die Nutzerbelegung wird verworfen, die Auslieferung greift, die Statuszeile nennt die Datei; der Verlust ist benannt und angenommen.
Implemented:
Deferred:
Superseded by:
