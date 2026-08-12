# Was geschieht, wenn der Nutzer das Fenster unter die Summe der Mindestbreiten zieht?

---
**Domain:** code
**Status:** answered
**Filed by:** planner
**Cross-references:** `circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_a_was-geschieht-wenn-die-mindestbreiten-nicht-mehr-hineinpassen.md`, `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_o_bereichsleiste-und-proportionale-breitenregel.md` (C4.4, Schritt 1 und 2), `crates/krk-ui/src/appkit/fenster.rs:116` (`MINDESTGROESSE`), `crates/krk-ui/src/fenstermodell.rs:169` (`Bereich::mindestbreite`)

---

## Question

Die sechste Frage dieses Circles ist am 260812-0306 beantwortet: ein **Schalter**, nach dem die Mindestbreiten nicht mehr hineinpassen, wird ohne Meldung verworfen. Der Fall hat aber einen zweiten Eingang, und die Antwort deckt ihn nicht ab.

Der Nutzer kann das Fenster schmaler ziehen. `MINDESTGROESSE` steht auf 780 Punkten Breite, der größte zugleich mögliche Satz an Mindestbreiten auf 920. Wer bei 1280 Punkten den Editor einschaltet und das Fenster danach auf 780 zieht, steht in genau dem Zustand, den die Abweisung verhindern sollte. Eine Abweisung gibt es dort nicht: das Fenster lässt sich ziehen, und die Breitenregel muss eine Antwort liefern.

Der beantwortete Datensatz kennt drei Möglichkeiten, und keine davon greift hier. Möglichkeit 1 (den Schalter verwerfen) hat keinen Schalter. Möglichkeit 2 (alle schrumpfen anteilig unter ihr Mindestmaß) ist als Antwort auf die Schalterfrage abgelehnt worden. Möglichkeit 3 (die Bereiche weichen der Reihe nach, der letzte bekommt null) ebenso.

## Options

1. **Alle sichtbaren Bereiche schrumpfen mit demselben Faktor unter ihr Mindestmaß.** Die Summe bleibt genau die verfügbare Breite, kein Bereich verschwindet.
   - Pro: Es ist dieselbe Regel wie darüber, nur mit den Mindestbreiten als Verhältnis. Der Nutzer sieht jeden eingeschalteten Bereich, wenn auch zu schmal, und beim Aufziehen des Fensters kommt jeder von selbst zurück.
   - Contra: Die Mindestbreite gilt in diesem Zweig nicht. Das ist der Einwand, aus dem der Nutzer sie in der Schalterfrage abgelehnt hat, dort allerdings gegen einen Klick und nicht gegen eine Ziehbewegung am Fensterrand.
2. **Die Mindestbreiten gewinnen, die Zeile wird breiter als das Fenster.** Der rechte Rand wird abgeschnitten.
   - Pro: Die Mindestbreite behält in jedem Fall ihre Bedeutung.
   - Contra: Ein Bereich verschwindet ganz oder halb aus dem Bild, ohne dass ein Schalter das sagt. Die `NSSplitView` legt ihre Unteransichten dann über ihren eigenen Rahmen hinaus, und die Trennlinien lassen sich nicht mehr sinnvoll ziehen.
3. **`MINDESTGROESSE` in der Breite von 780 auf 940 heben, damit der Fall nicht eintreten kann.** 920 Punkte Mindestbreiten plus 20 Punkte für die Trennlinien.
   - Pro: Die Frage entfällt statt beantwortet zu werden. Der Kommentar an der Konstante hält diese Zahl selbst als Möglichkeit fest und nennt sie eine Frage an den Nutzer.
   - Contra: **Sie macht die eben beantwortete sechste Frage gegenstandslos.** Passt jeder Satz an Mindestbreiten immer, wird die Abweisung am Schalter zu einer Vorsichtsmaßnahme, obwohl die Klärungsrunde sie ausdrücklich als Fähigkeit eingestuft hat, weil sie gemessen eintritt. Daneben lässt sich KRK dann nicht mehr schmal neben ein anderes Fenster stellen.

## Constraints

- Die Breitenregel steht einmal und muss für jede Eingabe eine Antwort liefern; ein undefinierter Fall ist keine Option.
- Die Summe der ausgegebenen Breiten soll die verfügbare Breite sein; darauf stützt sich das Auslegen der Fensterzeile.
- Was am Fensterrand gezogen wird, kann nicht abgewiesen werden: `setContentMinSize` ist die einzige Grenze, und sie ist eine Zahl und keine Regel über Bereiche.

## Recommendation

**Möglichkeit 1**, und Möglichkeit 3 ausdrücklich nicht, solange die Antwort auf die sechste Frage steht. Die beiden Eingänge in denselben Zustand bekommen damit zwei verschiedene, aber lückenlos getrennte Antworten: **wer einschaltet, wird abgewiesen; wer das Fenster zusammenzieht, bekommt alle Bereiche verkleinert.** Das ist keine Überschneidung, denn die Eingänge sind verschieden, und es ist keine Lücke, denn beide sind beantwortet.

Der Plan setzt Möglichkeit 1 um. Möglichkeit 3 wäre eine Zeile in `fenster.rs` und macht den zweiten Zweig der Breitenregel unerreichbar, ohne ihn überflüssig zu machen: eine reine Funktion braucht auch für den unerreichbaren Fall eine Antwort.


## Antwort 260812-0430

**Die Empfehlung des Plans wird uebernommen: alle sichtbaren Bereiche schrumpfen mit demselben
Faktor.** `MINDESTGROESSE` bleibt in der Breite bei 780 Punkten.

**Warum nicht die Breite auf 940 heben.** Der Datensatz fuehrt diese Moeglichkeit, und sie
kostet mehr, als sie einbringt:

1. Sie verbietet dem Nutzer Fensterbreiten, die er heute hat. Wer KRK neben einem anderen Fenster
   auf einem 13-Zoll-Schirm fuehrt, verliert die schmalen Breiten zwischen 780 und 940, und zwar
   auch dann, wenn er den Editor gar nicht offen hat — die Mindestgroesse gilt immer, der Satz von
   920 Punkten nur bei vier gleichzeitig sichtbaren Bereichen.
2. Sie machte die eben beantwortete sechste Frage des Circles gegenstandslos: es gaebe keinen
   Fall mehr, in dem ein Schalter abzuweisen waere, und die Abweisung waere wieder eine
   Vorsichtsmassnahme statt einer Faehigkeit.

**Die Ziehbewegung am Fensterrand ist nicht abweisbar**, und darin unterscheidet sie sich vom
Schalter. Ein Schalter fragt um Erlaubnis, eine Ziehbewegung teilt mit. Die Regel muss deshalb
fuer diesen Fall eine Antwort haben, und die einzige, die keine zweite Bedeutung in die
Mindestbreite traegt, ist das gleichmaessige Schrumpfen: unterhalb der Mindestsumme behaelt jeder
Bereich seinen Anteil an ihr.

**Der Preis ist benannt:** unterhalb von rund 920 Punkten mit offenem Editor faellt der Editor
unter die Breite, bei der eine Zeile Text lesbar ist (viertes Abnahmekriterium von C1 der
Editor-Runde). Das ist heute schon so — die heutige Deckelung laesst einen festen Bereich sogar
auf 0 fallen — und wird durch die Anteilsregel besser statt schlechter: alle geben gleichmaessig
nach, statt dass einer alles verliert.

Umsetzung: Schritt 1 des Plans `circles/260811-1304-statusleiste-mit-bereichsschaltern/planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, zweiter Zweig der Rechenvorschrift, Kriterium C4.4.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0430` — beantwortet vom Orchestrator, autonom auf Weisung des Nutzers; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented:
Deferred:
Superseded by:
