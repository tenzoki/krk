# Sitzung: Untersuchung des Fokusdefekts beim Mausklick

**Datum:** 2026-08-19 10:43
**Agent:** analyst
**Auftrag:** Ursache des Defekts `260819-0900` finden, ohne etwas zu ändern

## Was gelaufen ist

Der Defektdatensatz vom 260819-0900 nimmt an, etwas im Baum hindere die
`NSTableView` des Dateifensters daran, beim Klick Ersthelfer zu werden, und nennt
drei Kandidaten. Alle drei sind am Baum ausgeschlossen: das einzige
`setRefusesFirstResponder(true)` steht an den Schaltern der Bereichsleiste, eine
Überschreibung von `hitTest:` gibt es nirgends, und der `NSBox` je Bereich
enthält seinen Inhalt, statt darüberzuliegen. Der lokale Ereignisabgriff hört
allein auf `KeyDown`.

Danach war die Frage offen, ob die Annahme überhaupt trägt. Der Baum gibt darauf
keine Antwort, also ist sie gemessen worden, nach dem Vorbild der Messung vom
260816 im Kopf von `DateifensterDelegierter::zellenansicht`: fünf weggeworfene
Programme in Objective-C, die den Aufbau nachbauen und Klicks über
`postEvent:atStart:` durch die echte Ereignisschlange schicken. Ergebnis: in
jedem geprüften Fall nimmt die Tabelle den Rang, auch unter der letzten Zeile.
Ebenso der Editor und die Leiste. Allein eine nackte `NSView` mit
`acceptsFirstResponder` und ohne `mouseDown:` bleibt beim Klick unbeteiligt, was
die Bauart der Vorschau erklärt und den SDK-Kopf `NSResponder.h:315` bestätigt.

Die Untersuchung endet deshalb nicht mit einer Ursache, sondern mit einer
Verengung auf drei prüfbare Zweige und zwei Handgriffen, die zwei davon in
Sekunden ausschließen. Ein Agent kann sie nicht ausführen: sie brauchen einen
Klick im laufenden Bündel und den Blick auf den Rahmen.

Ein eigener Defekt ist dabei belegt worden, ohne dass es das Bündel braucht: der
Klick unter die letzte Zeile lässt `aktiv` stehen, und die Fokusanzeige malt
daraufhin den Rahmen auf das andere Dateifenster.

## Geschriebene Datensätze

- `shared/analyses/260819-1043-klick-holt-den-fokus-nicht.md`
- `shared/issues/260819-1043_o_ein-klick-unter-die-letzte-zeile-laesst-das-aktive-dateifenster-stehen-und-malt-den-rahmen-auf-das-andere.md`
- `shared/decisions/260819-1043_o_welche-flaechen-holen-den-fokus-wenn-man-hineinklickt.md`

Am Code, an den Daten und an fremden Datensätzen ist nichts geändert worden. Der
Datensatz `260819-0900` braucht eine Fortschreibung; sie gehört dem Nutzer oder
dem Reconciler.

## Anmerkung zum Stilprofil

`fusion-rules analyst` liefert für dieses Projekt `default-voice-en.yaml`, weil
`CLAUDE.md` `**Artifact language:** en` deklariert. Der Baum ist durchgehend
deutsch, und der Nutzer hat für diese Analyse Deutsch verlangt; geschrieben ist
sie nach `default-voice-de.yaml`. Die Deklaration in `CLAUDE.md` steht quer zum
Bestand, und der offene Datensatz
`shared/issues/260817-1610_o_the-language-paragraph-in-claude-md-predates-the-artifact-language-declaration.md`
führt dieselbe Sache bereits.

**Status:** Complete
