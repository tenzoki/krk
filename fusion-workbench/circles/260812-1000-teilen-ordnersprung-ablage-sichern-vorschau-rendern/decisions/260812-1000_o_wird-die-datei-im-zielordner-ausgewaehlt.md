# Steht die Auswahl nach dem Ordnersprung auf der Datei, oder am Anfang des Ordners?

---
**Domain:** code
**Status:** open
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/tabs.rs:508` (`ordner_setzen`, zweiter Parameter `auswahl`); `crates/krk-ui/src/appkit/tabelle.rs:628` (`ordner_lesen`); C2 der Runde 1 (Aufstieg) und C10 der Runde 1 (Sprung aus der Zwischenablage), die beide denselben Parameter benutzen

---

## Question

Nachdem das aktive Dateifenster den Ordner der angezeigten Datei zeigt, steht die Auswahl irgendwo. Auf der Datei, aus der der Sprung kam, oder auf dem ersten Eintrag des Ordners.

Der Mechanismus für die erste Möglichkeit ist gebaut und wird von zwei Befehlen bereits benutzt. `Tabliste::ordner_setzen` nimmt neben dem Ordner den Namen des Eintrags entgegen, auf den die Auswahl springen soll, sobald gelesen ist; der Aufstieg aus C2 nennt dort den verlassenen Ordner, der Sprung aus C10 die in der Zwischenablage genannte Datei. Getragen wird das von `wunschauswahl`, die einen noch laufenden Lesevorgang überlebt. Eine Zeilennummer täte das nicht, ein Name tut es.

Die Frage ist trotzdem zu stellen, weil sie den Zweck des Befehls mitbestimmt. Wer zum Ordner springt, will vielleicht die Datei sehen, aus der er kam, oder vielleicht gerade ihre Nachbarn.

Sie hält keinen Planschritt auf und bindet einen.

## Options

1. **Die Auswahl steht auf der Datei, aus der der Sprung kam.** Der Name wird als `auswahl` durchgereicht.
   - Folge: der Nutzer sieht sofort, wo die Datei liegt, was die Frage ist, die der Befehl beantwortet. Sofort anschließbar sind die Befehle, die auf dem betroffenen Eintrag wirken: `shift+cmd+c` kopiert ihren Pfad, `f5` kopiert sie, das neue Teilen erfasst sie. Kein neuer Mechanismus; der dritte Aufrufer eines Weges, den zwei Befehle schon gehen.
   - Preis: keiner, der am Bestand erkennbar wäre.

2. **Die Auswahl steht am Anfang des Ordners.** `auswahl` bleibt `None`.
   - Folge: der Nutzer sieht den Ordner als Ganzes und beginnt oben. Der Aufruf ist um ein Argument kürzer.
   - Preis: der Nutzer muss die Datei im Ordner selbst suchen, und in einem Ordner mit tausend Einträgen ist das die eigentliche Arbeit. Der Befehl beantwortet die Frage „wo liegt sie" dann nur halb.

3. **Die Auswahl steht auf der Datei, und die Zeile wird zusätzlich in die Mitte des Bildes gescrollt.**
   - Folge: die Datei ist auch in einem langen Ordner sofort sichtbar, ohne dass der Blick sie am Rand sucht.
   - Preis: eine zweite Regel neben der Auswahl, und sie berührt die Bildlaufposition, die heute an mehreren Stellen zusammenwirkt (`bildlauf_merken`, `bildlauf_ausstehend`, die Sitzungswiederherstellung). Ob der Aufwand einen sichtbaren Unterschied macht, ist ungemessen: welche Bildlaufposition eine `NSTableView` nach `scrollRowToVisible:` einnimmt, weiß dieses Projekt heute nicht.

## Constraints

- Der Name und nicht die Zeilennummer ist der einzige Weg, eine Auswahl über einen laufenden Lesevorgang zu tragen. Die Begründung steht im Kopf von `Tabliste::ordner_setzen` und in `CLAUDE.md` unter „Ein Lesevorgang leert sein Ordnermodell nicht vorab".
- Zeigt der sichtbare Tab bereits den Zielordner, geht der Weg nicht über `ordner_setzen`, sondern über `Tabliste::auswahl_auf_namen` (`:585`). Auch das ist gebaut und fragt `liest()` selbst ab.
- Ist die Datei zwischen Anzeige und Sprung verschwunden, liefert `auswahl_auf_namen` den Wert `Unbekannt`. Was dann geschieht, entscheidet der Datensatz `260812-1000_*_was-tut-der-ordnersprung-wenn-es-keinen-zielordner-gibt.md`.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie kostet ein Argument, das ohnehin in der Signatur steht, und sie folgt der Vorlage zweier Befehle, die diese Frage bereits gleich beantwortet haben. Möglichkeit 3 empfehlen wir nicht in dieser Runde: sie berührt die Bildlaufposition, und der Nutzen ist ungemessen. Wenn sich der Bedarf am laufenden Bündel zeigt, ist sie ein eigener kleiner Nachtrag.

---
Answered:
Implemented:
Deferred:
Superseded by:
