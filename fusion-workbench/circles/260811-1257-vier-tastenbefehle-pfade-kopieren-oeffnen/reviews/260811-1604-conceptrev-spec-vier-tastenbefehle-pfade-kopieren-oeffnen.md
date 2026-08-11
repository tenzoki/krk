# Concept Evaluation: Spec Vier Tastenbefehle für Pfade, das Öffnen und Cmd+W

**Date:** 2026-08-11 16:04
**Target:** `fusion-workbench/circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen/planning/260811-1552_o_spec-vier-tastenbefehle-pfade-kopieren-oeffnen.md`
**Verdict:** acceptable
**Diagrams evaluated:** 1  |  **Validation:** by-tool (mmdc 11.16.0 aus dem npx-Zwischenspeicher, Rendern nach SVG und PNG gelungen, Bild angesehen)

## Spruch

Das Diagramm trägt die Zusage der Directive, und der Beleg dafür ist ein einzelner Knoten. Alle vier Befehle laufen über `W` ("Wirkungsbereich des Kommandos gegen den Fokus"), einen Knoten mit Eingangsgrad 4 und Ausgangsgrad 4, und kein Pfad vom Tastendruck zu einem der vier umgeht ihn. Eine zweite Maschinerie steht nicht daneben, weder als Knoten noch als Kante. Der Graph ist ein Baum ohne Kreise, mit 16 Knoten, 21 Kanten und einer Dichte von 1,31; die Richtung `TD` hält im Rendern über alle fünf Schichten durch, ohne eine einzige rückwärts laufende Kante. Kein Gott-Knoten, keine Verfilzung, keine Waisen, der Diagrammtyp passt.

Der Abstand zu **clean** liegt in drei fehlenden Kanten und einer fehlenden Beschriftung, und zwei davon sitzen genau auf den beiden Besonderheiten, um die es in diesem Auftrag geht. Der Knoten `K3` ("Mit dem Standardprogramm öffnen") hat nur eine ausgehende Kante, obwohl C3 ihm zwei weitere zuschreibt: er wirkt auf `betroffene()` und er meldet in die Statuszeile. Und `K4` ("Tab schließen") ist im Bild von den drei neuen Befehlen durch nichts zu unterscheiden, obwohl C5 ausdrücklich drei neue Kombinationen zusagt und nicht vier. Beides ist ein Zeichenfehler und kein Entwurfsfehler: die Prosa des Specs sagt in beiden Fällen das Richtige, das Bild sagt weniger als sie. Am Abnahmetor wird aber zuerst das Bild gelesen.

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Spruch |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `flowchart TD` | 16 | 21 | 1,31 | 4 (`W`, vier verschiedene Ziele) | 4 (`W`) | 0 | ja, fünf `subgraph`-Blöcke | 0 | acceptable |

Der Knoten `T` ("Tastendruck über den Ereignisabgriff") trägt ebenfalls vier ausgehende Kanten, alle vier auf dasselbe Ziel `W`. Beschriftet sind 6 der 21 Kanten: die vier Kombinationen an `T` und die zwei Zweige des Doppelklicks an `D`. Alle 16 Knoten haben mindestens eine Kante. Die Schichtenfolge im gerenderten Bild lautet Auslösung, Fokusvorbehalt, Befehle, Geerbtes, Ergebnis, in dieser Reihenfolge von oben nach unten.

## Befunde

### 1. `K3` fehlen zwei Kanten, und an ihnen liegt der zweite Unterschied zwischen Taste und Doppelklick (substanziell)

Der Auftrag fragt, ob das Bild den Doppelklick von der Taste trennt. Es trennt ihn zur Hälfte, und die fehlende Hälfte ist die interessantere.

Getrennt ist der Weg: `D` läuft nicht über `W`, und im gerenderten Bild sieht man die beiden langen Kanten rechts an der Vorbehaltsschicht vorbeilaufen. Getrennt ist auch die Verzweigung: `D -->|"die Zeile ist ein Ordner"| EIN` und `D -->|"die Zeile ist keiner"| K3` stehen als zwei beschriftete Kanten da, während die Taste unverzweigt in `K3` mündet. Das ist die Aussage, die der Auftrag verlangt, und sie steht im Bild.

Nicht getrennt ist die Wirkungsmenge, und sie ist der zweite Unterschied. C3 sagt in seinem ersten Abnahmekriterium zu: "`return` gibt die betroffenen Einträge an das Standardprogramm des Systems. Betroffen heißt dasselbe wie in C2." Vier Kriterien weiter steht die Gegenaussage für die Maus: "Der Doppelklick wirkt auf die **angeklickte** Zeile und nicht auf die Markierung." Im Graphen führt keine Kante von `K3` zu `BE` ("betroffene(): Markierung vor Auswahl"). `BE` hat genau einen Eingang, den von `K2`. Wer den Graphen liest, entnimmt ihm, dass allein der Pfadkopierer die Markierungsregel erbt und der Öffner nicht. Das ist das Gegenteil dessen, was C3 zusagt.

Der Grund für die Lücke ist strukturell und nicht zeichnerisch nachlässig: beide Eingänge münden in **einen** Knoten, und ein Knoten kann nicht zwei verschiedene Wirkungsmengen haben. Genau das war die Sorge des Auftrags. Die Zusammenführung ist an sich richtig und trägt das Abnahmekriterium "Der Doppelklick öffnet dieselbe Umsetzung des Öffnens wie die Taste"; sie kostet aber die Aussage über das Worauf. Beides zugleich bekommt man mit einer Zwischenstufe: `W --> K3` läuft über `BE`, die Kante `D --> K3` nicht.

Dieselbe Lücke besteht zur Statuszeile. C3 verlangt an drei Stellen eine Meldung: die Zahl bei mehreren Einträgen, den Namen bei einem einzigen, den Grund bei einer Abweisung durch das System. `ST` ("Statuszeile: der Pfad oder die Zahl") hat trotzdem nur zwei Eingänge, von `K1` und `K2`. Die Beschriftung des Knotens nennt konsequenterweise auch nur, was die beiden Kopierer melden.

Ein Nebenbefund fällt dabei ab, und er gehört dem Spec und nicht dem Bild. C2 nennt den Pfadkopierer "den fünften Abnehmer" von `betroffene()`. Am Code nachgesehen stimmt der Ausgangsstand: der Kommentar über `betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:150-157`) spricht von vier Befehlen. Diese Runde legt aber zwei Abnehmer dazu, den Kopierer und den Öffner, und damit ist der Öffner der sechste. Eine Kante `K3 --> BE` hätte die Zählung von selbst aufgeworfen.

### 2. `K4` ist im Bild eine vierte neue Belegung, und es ist keine (substanziell)

Die Kante `T -->|"cmd+w"| W` hat dieselbe Form wie die drei darüber, und der Knoten `K4` sitzt in derselben Schicht wie `K1` bis `K3`, ohne Zusatz in seiner Beschriftung. Ein Leser, der das Bild vor der Tabelle in C5 sieht, nimmt vier neue Kombinationen mit. Der Spec sagt an drei Stellen das Gegenteil: die Vorbemerkung über dem Bild ("Vier Befehle, drei neue Kombinationen"), die Festlegung in C5 ("Drei neue Kombinationen und nicht vier"), und der Abgleich mit der Directive, der diesen Punkt als die eine Schärfung gegenüber der Directive führt.

Bemerkenswert ist, dass der Graph das Vokabular für den Unterschied bereits besitzt und es nur an einer von zwei Stellen einsetzt. `EIN` trägt es in seiner Beschriftung: "In den Ordner einsteigen, der bestehende Befehl auf dem Rechts-Pfeil". Dasselbe Mittel an `K4` würde genügen. Zwei Formulierungen, die tragen, wären "Tab schließen, bestehend: nur der Wirkungsbereich wächst" am Knoten oder `"cmd+w, unverändert"` an der Kante.

Der Entwurf ist an dieser Stelle in Ordnung, und das ist die Einordnung, die wir festhalten möchten: C4 und C5 beschreiben eine erweiterte Fallunterscheidung an einer bestehenden Zeile, keinen neuen Eintrag in `resources/default-keymap.toml`. Der Befund betrifft allein die Darstellung, aber er betrifft sie an der Stelle, an der die Runde ihren Zuschnitt beziffert.

### 3. Der Teilgraph "Die vier Befehle dieser Runde" hält fünf Knoten (geringfügig, verstärkt Befund 2)

Im gerenderten Bild liegt `EIN` innerhalb desselben Kastens wie `K1` bis `K4`. Die Aufschrift des Kastens nennt vier, sein Inhalt sind fünf, und die fünf zerfallen in drei Sorten: drei neue Befehle, ein bestehender Befehl mit erweitertem Wirkungsbereich, ein Befehl, den diese Runde nicht anfasst. Eine Aufschrift für drei Sorten wird der stärksten von ihnen nicht gerecht. `EIN` gehörte entweder in die Schicht "Was die Runde erbt und nicht neu baut", wo es inhaltlich hingehört, oder der Kasten heißt anders.

### 4. Die Zuordnung von Kombination zu Befehl geht am Knoten `W` verloren (geringfügig)

Fünfzehn der 21 Kanten sind unbeschriftet, darunter alle vier Kanten von `W` zu den Befehlen. Die vier Kombinationen stehen auf der ersten Hälfte des Weges, die vier Ziele auf der zweiten, und der Knoten dazwischen trennt sie. Aus dem Graphen allein ist nicht abzulesen, dass `return` bei `K3` ankommt und `opt+cmd+c` bei `K1`. Diese Zuordnung trägt allein die Tabelle in C5.

Die Zusammenführung an `W` ist der Gewinn des Bildes und soll bleiben: sie ist der sichtbare Beleg dafür, dass es einen Vorbehalt gibt und nicht vier. Die Beschriftungen liegen nur auf der falschen Hälfte. Wandern sie an die Kanten `W --> K1` bis `W --> K4`, sagt das Bild beides: dass alle vier durch dasselbe Nadelöhr gehen, und welche Taste danach wo landet.

### 5. `EIN` endet ohne Ergebnis (geringfügig)

Jeder andere Befehlsknoten erreicht die Schicht "Was der Nutzer sieht": `K1` und `K2` über die Statuszeile und die Zwischenablage, `K3` über `NSWorkspace` zum Standardprogramm, `K4` zum geschlossenen Tab. `EIN` ist ein Blatt. Der Ordner-Zweig des Doppelklicks liest sich dadurch weniger wirklich als sein Datei-Zweig, obwohl der Nutzer ihn öfter auslösen wird. Ein Knoten "das Dateifenster zeigt den Ordner" in der Ergebnisschicht schließt die Asymmetrie.

### 6. Der Belegungsnachschlag fehlt, und C5 wohnt dort (geringfügig)

Der Weg von der Taste zum Kommando ist im Bild zu `T --> W` verkürzt. Der Schritt dazwischen, der Nachschlag in der Belegung (`Belegung::nachschlag`, `crates/krk-core/src/tasten/belegung.rs:866`), steckt in der Beschriftung von `T` und hat keinen eigenen Knoten. Für die Zusage "über keine zweite Maschinerie daneben" ist genau dieser Schritt die tragende Stelle: eine Zeile in `resources/default-keymap.toml` statt einer fest verdrahteten Tastenabfrage ist der Unterschied, um den es geht, und die ganze Fähigkeit C5 handelt davon. Das Bild belegt die Zusage heute am Fokusvorbehalt, also eine Station zu spät. `inference:` Wir halten den Befund für geringfügig, weil `W` die Zusage bereits mitträgt und ein weiterer Knoten in der Auslösungsschicht die Schichtenfolge verlängert; ein Planner-Diagramm sollte ihn dagegen führen.

## Nicht zu beanstanden

**Die Schichtung hält im Rendern.** Wir haben das Bild erzeugt und angesehen, es ist keine Ableitung aus dem Quelltext. Die fünf Kästen stehen sauber untereinander, keine Kante läuft nach oben, und die einzigen Kanten, die eine Schicht überspringen, sind die drei zur Ergebnisschicht (`K1 --> ST`, `K2 --> ST`, `K4 --> TAB`). Das Überspringen ist folgerichtig: die Schicht "Geerbtes" ist keine Verarbeitungsstufe, sondern eine Herkunftsangabe, und die Prosa über dem Bild sagt das auch so ("was die Runde baut und was sie erbt"). Diese eine Schicht ist von anderer Art als die vier übrigen, was formal eine Mischung zweier Gliederungsprinzipien ist. Sie ist angekündigt, sie ist im Bild ablesbar, und sie ist der Grund, aus dem die vier Kästen unter "Was die Runde erbt" ihren Zweck erfüllen: sie zeigen auf einen Blick, warum die Runde klein ist. Wir führen das nicht als Befund.

**Die Dichte ist niedrig und die Fan-out-Werte sind unauffällig.** Der höchste Ausgangsgrad auf verschiedene Ziele ist 4 bei 16 Knoten. Ein Gott-Knoten sieht anders aus.

**Der Diagrammtyp passt.** Ein gerichteter `flowchart` mit `subgraph`-Schichten ist die richtige Wahl für Architektur und Fluss. Eine Sequenz ist hier nicht zu zeigen; die vier Befehle sind unabhängig voneinander, und eine Reihenfolge zwischen ihnen gibt es nicht.

**Der Spec enthält nur dieses eine Diagramm, und das genügt.** Die übrigen Abschnitte tragen Aufzählungen, Zahlen und Begründungen, keine Struktur, die ein zweiter Graph klarer machte. Die Tabelle in C5 ist die richtige Form für die Zuordnung von Kennung, Kombination und Wirkungsbereich; ein Diagramm daneben wäre Rauschen.

## Was ein sauberer Nachzug verlangt

Der Spruch ist **acceptable** und nicht **tangled**, deshalb steht hier keine Umgestaltung des Entwurfs. Der Entwurf stimmt, und der Graph widerspricht ihm nicht, er sagt an drei Stellen weniger als er. Was folgt, ist die kleinste Änderung, die beide zur Deckung bringt. Sie ist Sache einer Nachbesserung des Specs oder des Planners, nicht dieser Bewertung, und sie hält die Abnahme nicht auf.

Zwei Kanten kommen an `K3` hinzu: eine zu `BE` und eine zu `ST`. Läuft die Kante zu `BE` allein vom Vorbehalt her, also `W --> BE --> K3` bei unverändertem `D --> K3`, dann zeigt das Bild den zweiten Unterschied zwischen Taste und Maus, den es heute verschweigt: dieselbe Handlung, andere Wirkungsmenge.

`K4` bekommt in seiner Beschriftung das Wort, das `EIN` schon trägt. Damit steht im Bild, was C5 beziffert, und die drei neuen Kombinationen bleiben drei. Wenn `EIN` dabei in die Erbschicht wandert, stimmt auch die Aufschrift des Kastens wieder mit seinem Inhalt überein.

Die vier Kombinationen wandern von den Kanten `T --> W` auf die Kanten `W --> K1` bis `W --> K4`. Der eine Vorbehalt bleibt sichtbar, die Zuordnung kommt zurück.

---

Abgleichsvermerk 260811-2157 (`reconciler`): die Auflage dieser Durchsicht ist eingeloest. Der Kopf
des betroffenen Dokuments nennt die Nachbesserung mit Datum, und die Diagramme im Baum tragen sie.
Der Befund des Plan-Durchgangs, die fehlende Kante `S2 → S3` sei ein echter und kein zeichnerischer
Mangel, ist am Code bestaetigt: `mit_standardprogramm_oeffnen` (`crates/krk-ui/src/appkit/tabelle.rs:940`)
ruft `operationen::nichts_zu_oeffnen`, das im selben Zug wie `nichts_zu_kopieren` aus S2 entstanden
ist. Eine Reihenfolge S3 vor S2 haette den Baum rot stehen lassen.
