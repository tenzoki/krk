# Konzeptprüfung: Implementierungsplan KRK Navigator-Gerüst (Runde 1)

**Datum:** 2026-08-02 14:47
**Ziel:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`
**Verdikt:** acceptable
**Geprüfte Diagramme:** 4
**Validierung:** by-tool (mermaid 11.16.0 `parse()` unter jsdom, Node v24.2.0)

## Verdikt

Alle vier Graphen sind strukturell tragfähig, und der Abhängigkeitsgraph der 23 Schritte deckt sich Kante für Kante mit den Abhängigkeitszeilen der Schritte im Fließtext. Wir haben alle 29 Kanten maschinell gegen die 29 Prosa-Angaben gestellt: kein Unterschied. Kein Graph hat einen Zyklus, keinen verwaisten Knoten, keinen God-Node; die beiden großen Diagramme sind über `subgraph` mit ausdrücklicher Richtung geschichtet, und die Kanten laufen mit der Schichtung, nicht gegen sie. Zum Verdikt "clean" fehlen zwei Dinge. Der Abhängigkeitsgraph lässt zwei Kanten aus, die die Sache verlangt: S21 misst L7, die Vorschau-Zusage, ohne von S19 abzuhängen, der die Vorschau erst baut, und S23, das Auslieferungspaket, hängt von keinem der vier Schritte S17 bis S20 ab. Der Schichtungsgraph zeichnet ausschließlich die abwärts gerichteten Aufrufe und lässt damit beide Rückwege des Entwurfs unsichtbar, obwohl der Plan einen davon, den einzigen Auffrischungspfad, in Frage 3 zu seinem Kernargument macht.

Die Selbstprüfung am Dokumentende haben wir Zahl für Zahl nachgerechnet. Sie stimmt, mit einer Zähleinheit als Ausnahme und einem Satz, dessen Reichweite enger ist, als er klingt. Gegenüber dem Spec, dessen Selbstprüfung eine falsche Kantenzahl trug, ist das eine deutliche Verbesserung.

## Messwerte je Diagramm

| # | Zeilen | Typ | Knoten | Kanten | Verhältnis | Max. Ausgangsgrad | Max. Eingangsgrad | Zyklen | Verwaist | Kanten ohne Label | Geschichtet | Verdikt |
|---|--------|-----|--------|--------|-----------|-------------------|-------------------|--------|----------|-------------------|-------------|---------|
| 1 Ladepfad | 77–97 | flowchart LR | 9 | 8 | 0,89 | `APPEND` = 3 | alle = 1 | 0 | 0 | 1 (`Q -> GATE`) | nein, nicht nötig | clean |
| 2 Dateioperation | 159–172 | stateDiagram-v2 | 5 benannt (+ `[*]`) | 11 | 2,20 | `Laeuft` = 5 | `Laeuft` = 3 | 3 Schleifen, erwartbar | 0 | 0 | entfällt | clean |
| 3 Aufbau | 194–236 | flowchart TD | 13 | 15 | 1,15 | `TBL`/`EVT`/`SIDE`/`WRAP`/`SCAN` = je 2 | `WRAP` = 4 | 0 | 0 | 1 (`WRAP -> CLS`) | ja, 4 subgraphs mit `direction` | acceptable |
| 4 Schritte | 283–356 | flowchart TD | 23 | 29 | 1,26 | `S1` = 4 | `S6` = 3 | 0 | 0 | 28 (im DAG unschädlich) | ja, 6 Phasen mit `direction` | acceptable |

Quellen der Zahlen: `mermaid.parse()` für die Syntax, eine eigene Auszählung der aus dem Quelltext übertragenen Kantenlisten für die Grade, eine vollständige Tiefensuche über alle drei Flowcharts für die Zyklen.

Ergänzende Kennzahlen zum Abhängigkeitsgraphen: eine Quelle (`S1`), fünf Senken (`S17`, `S18`, `S19`, `S20`, `S23`), genau eine Kante gegen die Schrittnummerierung (`S11 -> S10`), weiteste Spanne `S5 -> S23`.

## Abgleich der Selbstprüfung mit den Diagrammen

Der Nutzer hat ausdrücklich nach diesem Punkt gefragt, weil die Selbstprüfung des Specs am 260802-1118 abwich. Wir haben jede Zahl des Absatzes in Zeile 648 einzeln nachgerechnet:

| Behauptung der Selbstprüfung | Nachgerechnet | Ergebnis |
|---|---|---|
| Schichtungsgraph: 13 Knoten, 15 Kanten, Verhältnis 1,15, ohne Zyklus | 13 / 15 / 1,15 / azyklisch | stimmt |
| Eingangsgrad 4 am Knoten `Sichere Huellen um jeden AppKit-Aufruf` | Eingangsgrad `WRAP` = 4, höchster im Graphen | stimmt |
| Ladepfad: 9 Knoten, 8 Kanten, eine Verzweigung, ohne Zyklus | 9 / 8 / Verzweigung an `GATE` / azyklisch | stimmt |
| Zustandsgraph: 6 Zustände, 11 Übergänge | 11 Übergänge; 5 benannte Zustände plus `[*]` | Übergänge stimmen, Zustandszahl nur mit Pseudo-Zustand |
| "die Schleifen an `Laeuft` und über `Uebersprungen`" | drei Schleifen: `Laeuft` auf sich selbst, über `Uebersprungen`, über `Konflikt` | eine Schleife ungenannt |
| Abhängigkeitsgraph: 23 Knoten, 29 Kanten, Verhältnis 1,26, zyklenfrei, sechs Phasen | 23 / 29 / 1,26 / azyklisch / 6 subgraphs | stimmt |
| "Rückwärtskanten hat er keine: jede Kante läuft in Phasenreihenfolge vorwärts" | keine Kante läuft gegen die Phasenreihenfolge; `S11 -> S10` läuft gegen die Schrittnummerierung | stimmt für Phasen, verdeckt die Nummerierung |
| Weiteste Spanne `S5 -> S23` | Spanne 18, größte im Graphen | stimmt |

Sechs von acht Angaben halten ohne Einschränkung. Die Korrektur, die der Planner gemeldet hat, hat gegriffen: unter der Lesart "Phasenreihenfolge", die der Satz selbst mitliefert, gibt es tatsächlich keine Rückwärtskante. Wir halten den Satz trotzdem für zu eng, und der Grund steht in Befund 4.

## Befunde

### 1. S21 misst die Vorschau-Zusage L7, ohne von S19 abzuhängen (substanziell, fehlende Kante)

Der Abhängigkeitsgraph erlaubt eine Ausführungsreihenfolge, in der die Messung einer Zusage vor der Fähigkeit läuft, die sie misst. S21 ("Messmodus in der Anwendung") deckt nach Zeile 546 die Zusagen L1, L5, L6, L7, L8 und L9 ab. L7 lautet im Spec, Zeile 249: "Vorschau einer Textdatei bis 1 MB sichtbar, sonst die Metadaten, 100 ms". Gebaut wird die Vorschau in S19. Im Graphen ist S19 kein Vorfahre von S21: die Vorfahren von S21 sind S1 bis S16, S19 gehört nicht dazu. Dieselbe Aussage steht auch in der eigenen Tabelle des Plans in Zeile 136, die L7 der Strecke "in der Anwendung, Schritt S21" zuordnet, ohne S19 zu erwähnen.

Die anderen fünf Zusagen dieser Strecke sind sauber abgedeckt. L5 (Tab- und Fensterwechsel) hängt an S12, L6 (Einstieg in einen Unterordner) an S13, L8 und L9 an S16; alle vier sind Vorfahren von S21. Die Lücke betrifft genau L7.

Der Plan braucht die Kante `S19 --> S21`. Ohne sie kann der `coder` S21 abnehmen, obwohl eine der sechs gemessenen Zusagen an einer Ansicht hängt, die noch nicht existiert.

### 2. Das Auslieferungspaket S23 hängt von keinem Schritt der Phase E ab (substanziell, fehlende Kanten)

Die Senke des Graphen beherrscht nicht die Arbeit, die vor ihr liegt. S23 ("Auslieferungspaket") hat als Vorfahren S1 bis S16 sowie S21 und S22. Nicht darunter: S17 (Stapel-Umbenennen), S18 (Lesezeichen- und Geräteleiste), S19 (Vorschaufenster), S20 (Belegungsansicht). Der Graph sagt damit, dass sich Runde 1 ausliefern lässt, während vier ihrer 23 Schritte offen sind, und drei davon setzen Fähigkeiten des Specs um: C5, C6 und die Belegungsansicht aus C3.

Wir vermuten hier keine Absicht, sondern die übliche Nebenwirkung eines DAG, der nur technische Voraussetzungen führt und nicht die Vollständigkeit der Runde. Die Lesart ist trotzdem folgenreich, weil der Plan seine Phase F ausdrücklich "Abnahme und Auslieferung" nennt. Vier Kanten von S17, S18, S19 und S20 auf S22 oder S23 würden die Aussage herstellen, die der Phasenname bereits verspricht.

### 3. Der Schichtungsgraph zeichnet nur die Aufrufe nach unten und verliert damit beide Rückwege (substanziell, fehlende Kanten)

Die gesamte Systemschicht ist im Graphen eine Senke: `AK` (AppKit) und `FS` (Dateisystem, FSEvents, NSWorkspace) haben eingehende Kanten und keine ausgehenden. Der Entwurf hat aber zwei Wege, die aus dem System zurück in die Anwendung führen, und beide sind Kernaussagen des Plans.

Der erste ist der Auffrischungspfad. Frage 3 macht ihn in Zeile 107 zum Argument: "Die Operationsmaschine ruft am Ende dieselbe Funktion `ordner_neu_lesen(pfad)` auf, die auch der FSEvents-Rückruf aufruft. Eine Funktion, zwei Auslöser." Im Graphen existiert weder `ordner_neu_lesen` noch der FSEvents-Rückruf. Sichtbar ist nur `SCAN -->|liest ueber getattrlistbulk| FS`, also die eine Richtung. Genau die Rückrichtung war im Spec-Diagramm gezeichnet und begründet: die Kanten `FS -->|liefert Einträge| P1` und `FS -->|liefert Einträge| P2` bilden dort den Zyklus, den die Prüfung vom 1118 als gewollt bestätigt hat.

Der zweite ist die Papierkorb-Schnittstelle. S15 hält in Zeile 490 fest, dass `NSFileManager.trashItemAtURL:` in `krk-ui/src/appkit/` liegt und "über eine Schnittstelle injiziert" wird, damit `krk-core` AppKit-frei bleibt. Diese Injektion ist die einzige Abhängigkeitsumkehr des ganzen Entwurfs und im Graphen die einzige Kante, die gegen die Schichtung liefe. Sie fehlt.

Der Wechsel der Kantenbedeutung vom Spec zum Plan ist für sich legitim. Der Spec zeichnet Datenfluss, der Plan zeichnet Aufrufe, und ein Aufrufgraph ist an dieser Stelle die genauere Auskunft. Der Graph ist aber nicht deshalb zyklenfrei, weil der Entwurf keine Rückwege hätte, sondern weil die gewählte Kantenbedeutung sie nicht zeigt. Ein Leser, der die Zyklenfreiheit als Aussage über die Kopplung nimmt, liest mehr, als der Graph belegt.

### 4. Die Schrittnummerierung ist keine topologische Ordnung (mittel, Genauigkeit)

`S11 --> S10` ist die einzige Kante des Graphen, die gegen die Nummerierung läuft, und sie ist echt: S10 nennt in Zeile 443 als Abhängigkeiten "S9, S11". Wer die 23 Schritte in ihrer Nummernfolge abarbeitet, trifft S10 vor dessen Voraussetzung S11.

Die Selbstprüfung nennt diesen Fall nicht, und ihre Formulierung deckt ihn genau nicht ab: "jede Kante läuft in Phasenreihenfolge vorwärts" ist wahr, weil S10 und S11 beide in Phase B liegen. Der Satz ist damit korrekt und trotzdem irreführend, weil ein Leser die Zusage "keine Rückwärtskanten" auf die Nummern bezieht, die im Diagramm stehen. Wir halten das für den einen Punkt, an dem die Selbstprüfung noch nachzuziehen ist.

Die Sache selbst ist billig zu beheben: S10 und S11 tauschen die Nummern, dann ist die Nummernfolge eine gültige Ausführungsreihenfolge. Für den Graphen ändert sich dabei nichts.

### 5. Zwei kosmetische Punkte

Zwei Kanten tragen kein Label, obwohl ihre Nachbarn eines tragen: `Q --> GATE` im Ladepfad und `WRAP --> CLS` im Schichtungsgraphen. Die zweite ist die interessantere, weil ihre Beziehung nicht selbsterklärend ist; alle übrigen vierzehn Kanten dieses Graphen benennen ihre Beziehung. Die 28 unbeschrifteten Kanten des Abhängigkeitsgraphen sind demgegenüber kein Befund: in einem Schritt-DAG bedeutet jede Kante "setzt voraus", und ein Label an jeder Kante wäre Rauschen.

Die Diagramme des Plans ersetzen die Umlaute weiterhin ("Eintraege", "Huellen", "hoechstens", "Uebersprungen"), obwohl die Prosa daneben sie schreibt. Der Spec ist an dieser Stelle inzwischen umgestellt und schreibt "Geräteordner" und "liefert Einträge" direkt im Diagramm. Die Prüfung vom 1118 hat nachgewiesen, dass die Ersetzung technisch nichts kauft. Nach der Umstellung des Specs unterscheiden sich die beiden Dokumente jetzt in der Schreibweise, was der Regel "eine Benennung pro Sache" zuwiderläuft.

### 6. Kein Befund: der Eingangsgrad 4 am Knoten `Sichere Huellen um jeden AppKit-Aufruf`

Nach der reinen Kennzahl ist `WRAP` der auffälligste Knoten des Schichtungsgraphen, nach der Sache ist er kein God-Node. Vier Oberflächenknoten zeigen auf ihn, und der Absatz in Zeile 238 trägt die Begründung: der Technologieentscheid macht jeden AppKit-Aufruf zu einem unsicheren Fremdaufruf, und der Entwurf bezahlt diese Kosten an genau einer Stelle, damit `krk-core` ohne Fenster testbar bleibt. Der Ausgangsgrad von `WRAP` ist 2. Ein Knoten, der viel empfängt und wenig weitergibt, ist eine Fassade, kein Gottobjekt.

Die Beobachtung der letzten Prüfung hat sich bestätigt. Zum Spec hatten wir notiert, dass der Knoten `Tastenbelegung` mit Ausgangsgrad 5 im Plan vermutlich in eine Zuordnungstabelle und eine Versandstelle zerfällt. Genau das ist eingetreten: aus `K` sind `EVT` (NSEvent-Abgriff, Ausgangsgrad 2), `KEYS` (Belegungstabelle, Ausgangsgrad 1) und `CFG` (Ablage in TOML) geworden. Der hohe Ausgangsgrad ist dabei verschwunden und nicht auf einen anderen Knoten gewandert.

### 7. Kein Befund: die drei Schleifen im Zustandsgraphen

Der Zustandsgraph der Dateioperation hat drei Schleifen: `Laeuft` auf sich selbst, `Laeuft -> Uebersprungen -> Laeuft` und `Laeuft -> Konflikt -> Laeuft`. In einem Lebenszyklus sind Schleifen die Normalform und nicht der Defekt, den `HYG-NO-CYCLES` meint; eine Abhängigkeitsumkehr zwischen Modulen ist etwas anderes als ein Zustand, der wiederkehrt. Alle drei bilden Festlegungen aus C4 ab, und alle Zustände erreichen einen Endzustand. Der Graph schließt damit die Lücke, die wir am Zustandsdiagramm des Specs als Befund 3 notiert hatten: dort fehlte der Endzustand, hier führen `Abgebrochen` und `Fertig` beide nach `[*]`.

Die Selbstprüfung nennt zwei der drei Schleifen. Die dritte, über `Konflikt`, ist aus dem Diagramm heraus verständlich und im Fließtext nicht eigens begründet. Wir führen das als Ungenauigkeit der Selbstprüfung, nicht als Mangel des Graphen.

## Trägt der Plan die Struktur des Specs?

Die Antwort ist ja, mit einer Ausnahme in der Darstellung und keiner im Zuschnitt. Wir haben die neun Fähigkeiten des Specs gegen die 23 Schritte gestellt: jede hat mindestens einen Schritt, und keine hat einen Schritt ohne Fähigkeit.

| Fähigkeit im Spec | Schritte im Plan |
|---|---|
| C1 Fenster, Tabs, aktives Fenster | S12 |
| C2 Tastaturnavigation | S13 |
| C3 Tastenbelegung | S7, S9, S10, S20 |
| C4 Dateioperationen | S15, S16, S17 |
| C5 Lesezeichen und Geräte | S18 |
| C6 Vorschau | S19 |
| C7 Sichtbarkeit der Bereiche | S12 |
| C8 Messbare Geschwindigkeit | S3, S8, S21, S22 |
| C9 Datenträgerwechsel | S14 |

Auch die Reihenfolge trägt: die Belegungsmaschine aus C3 steht vor der Navigation aus C2, weil der Spec die Navigationsbefehle als Kommandos hinter der Belegung definiert, und die Dateioperationen aus C4 stehen hinter beiden.

Die Ausnahme betrifft den Schichtungsgraphen. Der Spec-Graph führt `L` (Lesezeichen und Geräteordner) und `V` (Vorschaufenster) als getrennte Knoten mit gegenläufigen Beziehungen: `L` setzt den Ordner eines Dateifensters, `V` empfängt dessen aktive Auswahl. Der Plan fasst beide zum Knoten `SIDE` ("Lesezeichen, Geraete, Vorschau") zusammen, und dabei verschwinden beide Beziehungen aus dem Graphen. Sie sind im Plan nicht verlorengegangen, S18 hält in Zeile 518 fest, dass die Auswahl in der Leiste den Ordner des aktiven Dateifensters setzt. Sichtbar ist sie im Graphen nicht mehr. Zusammen mit Befund 3 ergibt das ein Muster: der Schichtungsgraph zeigt die Modulgrenzen genau und die Beziehungen zwischen den Bereichen sparsamer als der Spec.

## Was ein sauberer Nachzug verlangt

Das Verdikt ist "acceptable", ein Nachzug ist also nicht erzwungen. Wer ihn will, braucht vier Eingriffe, drei davon an einer Zeile:

1. Kante `S19 --> S21` in den Abhängigkeitsgraphen und `S19` in die Abhängigkeitszeile von S21 (Befund 1). Die Abnahme von S21 hängt daran.
2. Kanten von `S17`, `S18`, `S19` und `S20` auf `S22` oder `S23`, damit die Auslieferung die Runde beherrscht (Befund 2).
3. Zwei Kanten im Schichtungsgraphen für die Rückwege: eine aus `FS` in den Auffrischungspfad und eine von `WRAP` auf `OPS` für die injizierte Papierkorb-Schnittstelle (Befund 3). Beide laufen gegen die gezeichnete Schichtung, und genau deshalb gehören sie hinein: sie sind die einzigen Stellen, an denen die Grenze in die andere Richtung überschritten wird. Der Graph bekommt damit einen Zyklus, den der Spec bereits geführt und begründet hat.
4. S10 und S11 tauschen die Nummern, damit die Nummernfolge eine gültige Ausführungsreihenfolge ist, und den Satz der Selbstprüfung entsprechend fassen (Befund 4).

Keiner der vier Eingriffe ändert den Entwurf. Die Schichtung, der Zuschnitt der vier Kisten und die Phasenfolge halten der Prüfung stand.

## Prüfverfahren

Die Validierung lief mit Werkzeug. Alle vier Blöcke wurden über `mermaid.parse()` aus mermaid 11.16.0 unter jsdom geprüft; Ergebnis: `flowchart-v2` OK, `stateDiagram` OK, `flowchart-v2` OK, `flowchart-v2` OK. Die Kennzahlen stammen aus einer Auszählung der aus dem Quelltext übertragenen Kantenlisten, die Zyklen aus einer vollständigen Tiefensuche über die drei Flowcharts, die Vorfahrenmengen von S21, S22 und S23 aus einer Rückwärtssuche über den Schritt-DAG. Der Abgleich der 29 Diagrammkanten gegen die 29 Abhängigkeitszeilen der Schritte lief ebenfalls maschinell und über alle 23 Schritte, nicht stichprobenhaft.
