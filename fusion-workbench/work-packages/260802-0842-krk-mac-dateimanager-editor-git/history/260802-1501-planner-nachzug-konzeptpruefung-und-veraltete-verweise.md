# Planner: Nachzug des Plans nach der Konzeptprüfung, dazu drei überholte Verweise

**Datum:** 2026-08-02, 15:01
**Agent:** planner
**Bearbeitet:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260802-1445_c_plan-nennt-die-c8-luecke-und-zwei-defekte-noch-als-offen.md`
**Gelesen, nicht geändert:** Spec, Circle-Datensatz, `spikes/`, die beiden offenen Entscheidungsdatensätze

## Auftrag

Den eigenen Plan auf den Stand ziehen: die vier Befunde der Konzeptprüfung `reviews/260802-1447-conceptrev-plan-navigator-geruest-runde-1.md` (Verdikt "acceptable") umsetzen, die drei überholten Verweise aus dem Defekt vom 260802-1445 nachziehen und den Defekt schließen. Ausdrückliche Vorgabe der Runde: beheben statt melden, solange die Sache im Plan liegt.

## Was geändert wurde

**Vier Kanten im Abhängigkeitsgraphen dazu, jetzt 33 statt 29.**

`S19 --> S21` schließt die Lücke, die Befund 1 nennt: S21 misst L7, die Vorschau-Zusage, und die Vorschau baut S19. Wir haben die übrigen fünf Zusagen derselben Messstrecke einzeln nachgeprüft, statt dem Befund zu glauben. L1 hängt an S6 und S7, L5 an S12, L6 an S13, L8 und L9 an S16; alle vier Schritte sind Vorfahren von S21. Es war wirklich nur diese eine Kante.

`S17 --> S23`, `S18 --> S23` und `S20 --> S23` beantworten Befund 2. Die Entscheidung, welche Abhängigkeiten S23 braucht, trennt zwei verschiedene Dinge, die der Befund zusammenfasst. S19 gehört nicht dazu: über S21 und S22 liegt es ohnehin vor S23, eine eigene Kante stünde doppelt im Graphen. S17, S18 und S20 gehören dazu, aber nicht als technische Voraussetzung des Bauvorgangs, sondern als Vollständigkeit der Runde: ein Auslieferungspaket entsteht erst, wenn das Stapel-Umbenennen aus C4, die Lesezeichen- und Geräteleiste aus C5 und die Belegungsansicht aus C3 stehen. Die Kanten gehen auf S23 und nicht auf S22, weil S22 die Messreihe ist und keine der drei Fähigkeiten gemessen wird. Der Unterschied steht als Absatz unter dem Diagramm.

**Zwei Rückwege im Schichtungsgraphen, jetzt 14 Knoten und 19 Kanten.**

Befund 3 trifft zu: der Graph zeichnete nur die Aufrufe nach unten. Der Auffrischungspfad ist als eigener Knoten `ordner_neu_lesen` mit seinen zwei Auslösern eingezogen, dem FSEvents-Rückruf und dem gemeldeten Abschluss einer Dateioperation. Damit trägt der Graph genau einen Zyklus, `Verzeichnisleser → Dateisystem → ordner_neu_lesen → Verzeichnisleser`, und der ist im Fließtext begründet: ein Dateimanager, der fremde Änderungen anzeigt, hat notwendig eine Rückrichtung aus dem Dateisystem, und sie läuft über das Betriebssystem, nicht über eine gegenseitige Kistenabhängigkeit.

Beim Zeichnen der Kante ist eine Ungenauigkeit der Planprosa aufgefallen und behoben. Frage 3 schrieb, die Operationsmaschine rufe `ordner_neu_lesen` auf. Die Operationsmaschine liegt in `krk-core`, die Funktion in `krk-ui`; ein Aufruf wäre eine zweite Abhängigkeitsumkehr, die der Plan nirgends begründet. Richtig ist: die Operationsmaschine meldet ihren Abschluss über denselben Fortschrittskanal, über den sie auch den Fortschritt meldet, und der Auslöser liegt wie der FSEvents-Rückruf in `krk-ui`. "Eine Funktion, zwei Auslöser" bleibt damit richtig und wird sogar sauberer. Frage 3 und die Änderungszeile von S14 sind entsprechend gefasst.

Die Papierkorb-Schnittstelle steht als `Operationsmaschine --> Sichere Hüllen um jeden AppKit-Aufruf`. Die Prüfung schlägt in ihrer Nachzugsliste die Gegenrichtung vor, begründet in Befund 3 aber die Richtung, die jetzt im Graphen steht: der Graph zeichnet Aufrufe, und der Aufruf läuft vom Kern nach oben in die Hülle, während die Übersetzungsabhängigkeit weiter von oben nach unten läuft. Genau das ist die Abhängigkeitsumkehr, die sichtbar werden sollte. Ein Zyklus entsteht dadurch nicht.

**S10 und S11 getauscht.**

Befund 4 stimmt: `S11 --> S10` war die einzige Kante gegen die Nummernfolge, also war die Nummernfolge keine gültige Ausführungsreihenfolge. S10 ist jetzt die Ablage unter Application Support, S11 die Belegungsmaschine. An der Abhängigkeit ändert das nichts, die Belegungsmaschine liest die Nutzerbelegung weiterhin über die Ablage. Die Nummern kommen an acht Stellen im Dokument vor, nicht nur in der Schrittüberschrift: in der Tabelle der Randbedingungen zu C3, in zwei Diagrammknoten und sechs Diagrammkanten, in den Abhängigkeitszeilen von S11, S12, S18 und S20, im Fließtext von S12 zur Sitzungswiederherstellung und in der Risikozeile zu den ungemessenen Tastencodes F6 und F7. Die beiden Schrittblöcke sind zusätzlich vertauscht worden, damit das Dokument weiter aufsteigend nummeriert liest, und die Phasenüberschrift heißt jetzt "Ablage und Belegung" statt "Belegung und Ablage".

**Selbstprüfung neu gerechnet.** Sie nennt jetzt 5 benannte Zustände plus den Pseudo-Zustand `[*]` statt "6 Zustände", alle drei Schleifen des Zustandsgraphen statt zweien, und sagt zur Nummernfolge, was gilt: jede der 33 Kanten läuft von der kleineren zur größeren Schrittnummer. Der frühere Satz "Rückwärtskanten hat er keine" war für die Phasen wahr und verdeckte die Nummerierung.

**Zwei kosmetische Punkte der Prüfung gleich mit.** Die beiden unbeschrifteten Kanten `Q --> GATE` und `WRAP --> CLS` tragen jetzt ein Label; alle Kanten der drei Flowcharts außer denen des Schritt-DAG sind damit beschriftet, und im Schritt-DAG bedeutet jede Kante "setzt voraus". Die Diagramme schreiben jetzt Umlaute wie die Prosa daneben und wie der inzwischen umgestellte Spec. Die beiden Zustände mit Umlaut sind über `state "Läuft" as Laeuft` deklariert, damit die Bezeichner ASCII bleiben.

**Drei überholte Verweise nachgezogen**, jeder vorher am Dateibestand geprüft statt dem Defekt geglaubt. Alle drei stimmten. Einzelheiten stehen im `Resolved:`-Abschnitt des Defekts.

**Die offene Entscheidung zu L4 ist an drei Stellen vermerkt, ohne dass etwas geändert wurde, das von der Antwort abhängt.** S8 misst bis zur Antwort die mildere Lesart und schreibt sie im Bericht aus; am Durchstich gibt es ohnehin keine wiederhergestellte Sitzung, die Lesarten fallen dort zusammen. S21 trägt den Hinweis, dass der Nachtrag des Shapers die Frage auf L5, den Tabwechsel, ausgeweitet hat. S22 wartet die Antwort ab, weil ihm ohne die Sitzungslage aus dem Nachtrag die Messvorschrift für L4 fehlt und die Messreihe damit nicht wiederholbar wäre.

## Prüfung

Alle vier Mermaid-Blöcke sind mit `mermaid.parse()` aus mermaid 11 unter jsdom, Node v24.2.0, geprüft: `flowchart-v2`, `stateDiagram`, `flowchart-v2`, `flowchart-v2`, alle vier OK. Dieselbe Auswertung hat die Kennzahlen der Selbstprüfung gerechnet: Ladepfad 9 Knoten und 8 Kanten ohne Zyklus; Schichtungsgraph 14 Knoten, 19 Kanten, höchster Eingangsgrad 5 an den sicheren Hüllen, genau ein Zyklus; Schritt-DAG 23 Knoten, 33 Kanten, zyklenfrei, höchster Ausgangsgrad 4 an S1, höchster Eingangsgrad 5 an S23, keine Kante gegen die Nummerierung. Die 33 Diagrammkanten sind Kante für Kante gegen die 33 Abhängigkeitsangaben der Schritte gestellt worden; sie decken sich.

## Was außerhalb der Zuständigkeit liegt

Nichts Neues gemeldet. Zwei Punkte zur Kenntnis:

- `issues/260802-1445_o_grounding-snapshot-traegt-den-loeschstand-an-zwei-stellen-ueberholt.md` ist offen und betrifft den Circle-Datensatz, nicht den Plan. Er war nicht Teil des Auftrags und ist nicht angefasst.
- Die Prüfung notiert in ihrem Abschnitt "Trägt der Plan die Struktur des Specs?", dass der Schichtungsgraph des Plans zwei Beziehungen des Spec-Graphen nicht mehr zeigt, weil er Lesezeichen, Geräte und Vorschau zu einem Knoten zusammenfasst. Das ist kein Befund und keine Nachzugsforderung; die Beziehung steht im Fließtext von S18. Wir haben sie nicht in den Graphen gezogen, weil der Knoten sonst wieder aufgeteilt werden müsste und der Graph damit die Modulgrenzen unschärfer zeichnete, die er gerade genau zeichnet.

## Status

Der Plan bleibt auf dem Marker für offen und steht weiter zur Abnahme. Nicht committet, wie beauftragt.
