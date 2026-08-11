# Concept Evaluation: Plan Tastenbelegung als Markdown in Downloads

**Date:** 2026-08-11 08:53
**Target:** `fusion-workbench/circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0838_o_plan-tastenbelegung-als-markdown-in-downloads.md`
**Verdict:** acceptable
**Diagrams evaluated:** 3  |  **Validation:** by-tool (mermaid-cli 11.16.0, alle drei Blöcke nach SVG und PNG gerendert und angesehen)

## Spruch

Die drei Graphen sind gesund: kein Zyklus, kein Gott-Knoten, keine Waise, kein falscher Diagrammtyp, und die Schichtung des ersten Bildes deckt sich mit der wirklichen Kistenordnung des Projekts. Der eine Befund, der an der Frage dieses Auftrags hängt: **die Zweiteilung wird von genau einem der drei Bilder getragen, und das Aufbaubild schweigt dazu.** Das zweite Schaubild zeichnet sie sauber, mit zwei verschiedenen Mechanismen an zwei verschiedenen Knotenarten und mit den Zahlen 65 und 6 an den Zweigkanten. Das erste zeigt die dritte Spalte ausschließlich über `KMD`, führt keinen Knoten für den in S1 gemessenen Wert und nennt in der Beschriftung von `AUS` vier Stücke, unter denen ausgerechnet `wirkung` fehlt, die Funktion, die die Trennung im Programmtext ausführt.

Verwischt wird die Trennung dadurch nicht, denn kein Bild zeichnet die beiden Wege gleich. Sie ist im Aufbaubild nur unsichtbar, und ein Leser, der bei diesem Bild anfängt, nimmt aus ihm mit, die Spalte entstehe vollständig aus der Belegung. Das ist die Aussage, deren Gegenteil der Kopf des Plans ausschreibt. Der Abstand zu **clean** sind ein Knoten und ein Wort in einer Beschriftung.

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Spruch |
|---|-----|--------|--------|--------|--------------|--------------|--------|-------------|--------|--------|
| 1 | `flowchart TD` | 14 | 13 | 0,93 | 5 (`AUS`) | 2 (`GLI`) | 0 | ja, drei `subgraph` | 0 | acceptable |
| 2 | `flowchart TD` | 9 | 10 | 1,11 | 2 (`K`, `P`) | 3 (`Z`) | 0 | Entscheidungsbaum, kein `subgraph` nötig | 0 | acceptable |
| 3 | `flowchart TD` | 4 | 3 | 0,75 | 1 | 2 (`S3`) | 0 | DAG, vier Knoten | 0 | clean |

Unbeschriftete Kanten: zwei im ersten Bild (`KMD → WBB`, `ATOM → DAT`), fünf im zweiten (alle im Rumpf des Entscheidungsbaums, die vier bedeutungstragenden Zweigkanten sind beschriftet), keine im dritten. Die Zahlen in den Knotenbeschriftungen sind am Baum nachgeprüft und stimmen: `resources/default-keymap.toml` führt 71 Blöcke `[[funktion]]` und sechs Zeilen `gehalten_von = "menue"`, `Kommando::KENNUNGEN` 65 Paare, `Wirkungsbereich` sieben Werte.

## Befunde

### 1. Das Aufbaubild führt keinen Weg für die sechs gemessenen Befehle (substanziell)

Im ersten Schaubild geht die dritte Spalte über eine einzige Kette: `AUS -->|"Spalte 3, ueber das Kommando"| KMD --> WBB`. Ein zweiter Lieferant kommt nicht vor. Der Knoten `KMD` trägt die Beschriftung "65 Kommandos auf sieben Werte", nennt die Zahl also, ohne den Rest zu den 71 im Bild zu haben. Wer das Bild ohne den Kopf des Plans liest, schließt daraus auf einen Mechanismus für alle Funktionen.

Der Plan sagt an drei Stellen etwas anderes. Die `Decidability:`-Zeile nennt den Wechsel des Mechanismus für die sechs zugestellten Textbefehle, S1 misst ihn am Objective-C-Laufzeitsystem, und S3 legt das Ergebnis als gemessenen Wert im Programmtext ab. Ein gemessener Wert, der von außerhalb der Belegung in die Ausgabe kommt, ist strukturell eine zweite Quelle, und eine zweite Quelle ist genau das, was ein Aufbaubild zeigen kann.

Verschärft wird der Befund durch die Beschriftung von `AUS`. Sie lautet "belegungsausgabe: markdown, ausgeben, Ausgang, meldung". S3 b) zählt die Stücke des Moduls selbst auf, und dort stehen `markdown`, `wirkung`, `Ausgang` mit `meldung` und `ausgeben`. Das Bild ersetzt `wirkung` durch `meldung`, eine Methode an `Ausgang`, und lässt damit ausgerechnet die Funktion weg, in der die Fallunterscheidung wohnt.

### 2. Der rechte Zweig des zweiten Bildes entscheidet über die Gruppe, der Bau entscheidet je Befehl (substanziell)

Das zweite Schaubild trägt die Zweiteilung, um die es dieser Runde geht, und trägt sie gut: `K` fragt nach dem Kommando, der linke Zweig läuft über `W` und `B` durch die Belegung, der rechte über `M` in einen zweiten Entscheidungsknoten `P`, dessen Beschriftung den Mechanismuswechsel ausspricht ("S1: welche Klasse beantwortet den Selektor?"). Zwei Wege, zwei Knotenarten, beide Zweigkanten mit ihrer Zahl beschriftet. So wird aus der Behauptung im Kopf eine sichtbare Gabelung.

Die Granularität stimmt am rechten Zweig nicht. `P` sitzt über `M`, und `M` ist die Menge aller sechs zugestellten Textbefehle; seine beiden Ausgänge, "die Ableitung des Shapers haelt" und "die Ableitung bricht", gelten damit der ganzen Menge. S3 b) sieht den anderen Fall ausdrücklich vor: bricht die Ableitung für einen einzelnen der sechs, bekommt der Zweig eine `match`-Verzweigung über die Kennungen, mit leerer Zeichenkette allein für den betroffenen Befehl. Der Graph kennt das Alles-oder-nichts, der Plan kennt das Einzelne. Am Gate ist das die Stelle, an der ein Leser die Zusicherung falsch abliest, weil hier die Ehrlichkeit über den unentscheidbaren Teil verhandelt wird.

### 3. `P` ist eine Entscheidung der Bauzeit in einem sonst laufzeitlichen Baum (kosmetisch)

Alle übrigen Knoten des zweiten Bildes beschreiben, was `wirkung()` beim Erzeugen der Datei tut. `P` beschreibt, was S1 einmalig misst, bevor der Programmtext entsteht. Die Beschriftung sagt es mit dem vorangestellten "S1:", der Leser bekommt den Unterschied also mit; die Knotenform sagt es nicht. Eine zweite Form oder ein Zusatz wie "einmalig in S1 bestimmt, danach fester Wert" trennt die beiden Zeitpunkte, ohne das Bild zu verändern.

### 4. Eine Kante des ersten Bildes trägt Zugriffs- statt Flussrichtung (geringfügig)

`DEL -->|"leiht den Wert des Betriebs"| IV` zeigt, wer zugreift; die Belegung fließt in der Gegenrichtung, und die nächste Kante `IV -->|"eine Belegung, ohne Kopie"| AUS` ist wieder ein Fluss. Dieselbe Mischung hat im Diagramm des Specs den stummen Zyklus erzeugt, den unsere Bewertung vom 260811-0803 beanstandet hat. Hier entsteht kein Zyklus, weil keine Rückkante schließt, und der Befund bleibt deshalb geringfügig. Wer ihn räumen will, beschriftet die Kante als Fluss und dreht sie um.

### 5. Was gegenüber der Bewertung des Specs nachgezogen wurde (kein Befund, zur Einordnung)

Beide substanziellen Punkte der Spec-Bewertung sind erledigt. Der Zyklus `AUS → B → MOD → AUS` kommt in keinem der drei Bilder mehr vor. Die fehlende Gabelung ist gezeichnet: `GLI` hat mit `AUS` und `BM` zwei Abnehmer, die Belegungsansicht steht als `ANS` im Bild, und die Prosa unter dem Schaubild benennt beide tragenden Kanten ausdrücklich. Die Schichtung ist am gerenderten Bild geprüft und läuft durchgängig in eine Richtung, von `krk-ui/src/appkit` über die Modelle neben `appkit` in den Kern; keine Kante läuft gegen die Kistenordnung, die `CLAUDE.md` festlegt.

Das dritte Schaubild deckt sich Kante für Kante mit den Abhängigkeitszeilen der vier Schritte: S1 und S2 ohne Abhängigkeit, S3 auf beiden, S4 auf S3. Die Unabhängigkeit von S1 und S2 steht als Aussage in der Prosa darunter und ist im Graphen als fehlende Kante zwischen ihnen sichtbar.

## Was ein sauberer Nachzug verlangt

Der Spruch lautet **acceptable**, deshalb steht hier keine Umgestaltung des Entwurfs. Der Entwurf trennt die Frage richtig, und die Trennung steht im Kopf, im zweiten Schaubild und in S1 und S3. Was folgt, ist die kleinste Änderung, die das Aufbaubild mit ihr zur Deckung bringt, und sie ist Sache des Planners, nicht dieser Bewertung.

Das erste Schaubild bekommt einen Quellknoten für den gemessenen Wert, etwa `MESS["S1: gemessener Text der sechs zugestellten Befehle"]`, mit einer Kante nach `AUS`, beschriftet als "Spalte 3, ohne Kommando". Damit hat die dritte Spalte im Bild zwei Lieferanten, und die Aussage der `Decidability:`-Zeile ist im Aufbau ablesbar statt nur im Fließtext. In derselben Beschriftung ersetzt `wirkung` das `meldung`, denn `meldung` hängt an `Ausgang` und ist schon dort genannt.

Im zweiten Schaubild wandert die Entscheidung `P` von der Gruppe auf den einzelnen Befehl. Der Knoten `M` heißt dann "einer der sechs vom Menue zugestellten Textbefehle", und der Zweig "die Ableitung bricht" liest sich als "die Ableitung bricht für diesen Befehl". Der Rest des Bildes bleibt, wie er ist.
