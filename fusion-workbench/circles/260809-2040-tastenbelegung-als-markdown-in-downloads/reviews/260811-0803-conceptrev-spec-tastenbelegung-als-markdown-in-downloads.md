# Concept Evaluation: Spec Tastenbelegung als Markdown in Downloads

**Date:** 2026-08-11 08:03
**Target:** `fusion-workbench/circles/260809-2040-tastenbelegung-als-markdown-in-downloads/planning/260811-0753_o_spec-tastenbelegung-als-markdown-in-downloads.md`
**Verdict:** acceptable
**Diagrams evaluated:** 1  |  **Validation:** by-tool (mmdc 11.13.0 aus dem npx-Zwischenspeicher, Rendern nach SVG und PNG gelungen)

## Spruch

Das Diagramm ist gültiges, sauber beschriftetes Mermaid mit geringer Dichte, und es widerspricht der Directive nicht: ein zweiter Weg von der Quelle zur Darstellung ist nicht gezeichnet. Es trägt die Zusage aber auch nicht, und das ist der Befund. Die Directive sagt zu, die Ausgabe entstehe aus **derselben** Belegung wie die Belegungsansicht. Diese Aussage ist strukturell eine Gabelung: ein Knoten, zwei Abnehmer. Der Graph zeichnet nur den einen Abnehmer, die Belegungsansicht kommt als Verbraucher darin nicht vor. Wer das Bild liest, sieht keine zweite Aufbereitung, aber er sieht auch nicht, dass es dieselbe erste ist.

Dazu kommt ein stummer Zyklus über drei Knoten, und er ist der einzige Punkt, den wir vor der Abnahme geändert sehen möchten. Kein Gott-Knoten, keine Verfilzung, kein falscher Diagrammtyp: die Struktur ist im Kern gesund, und der Abstand zu **clean** sind zwei Kanten und ein Knoten.

## Messwerte

| # | Typ | Knoten | Kanten | Dichte | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Waisen | Spruch |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `flowchart TD` | 10 | 10 | 1,0 | 3 (`AUS`) | 5 (`AUS`) | 1 (stumm) | teilweise | 0 | acceptable |

Drei `subgraph`-Blöcke (`Auslösung`, `Quellen der drei Spalten`, `Ergebnis`), ein Knoten (`BL`) außerhalb aller drei. Alle zehn Kanten tragen eine Beschriftung. Der Zyklus lautet `AUS → B → MOD → AUS`.

## Befunde

### 1. Ein stummer Zyklus aus gemischter Kantenbedeutung (substanziell, aber kein Entwurfsfehler)

Neun der zehn Kanten zeigen, wohin ein Wert fließt. Die zehnte, `AUS -->|"liest, ändert nicht"| B`, zeigt in die Gegenrichtung: sie drückt eine Abhängigkeit aus, nicht einen Fluss. Genau daraus entsteht der Kreis `AUS → B → MOD → AUS`, und die Prosa unter dem Bild erklärt nur die gestrichelte Kante, nicht ihn.

Der Entwurf hat diesen Kreis nicht. Am Code nachgelesen ist der Weg linear: `Belegungsmodell::neu(belegung)` (`crates/krk-ui/src/belegungsmodell.rs:313`) baut das Modell aus der Belegung, und das Modell liefert danach die Zeilen. Die Kante `AUS → B` sagt dasselbe wie der Pfad `B → MOD → AUS`, nur rückwärts. Sie ist redundant, und ihr Preis ist ein Kreis, den ein Leser für eine wechselseitige Abhängigkeit halten kann.

Sichtbar wird der Preis auch am Bild selbst. Der Graph deklariert `TD`, aber der Layouter hält die Richtung nicht durch: er setzt `Quellen` neben `Ergebnis` statt darüber, und die Kanten `MOD → AUS` und `AUS → B` kreuzen sich. Wir haben das gerendert und angesehen, es ist keine Ableitung.

### 2. Die Belegungsansicht fehlt als Abnehmer, und mit ihr der Beleg für die Directive

Das ist der Befund, der an der besonderen Frage dieses Auftrags hängt. Die Directive verspricht dieselbe Belegung wie die Bildschirmansicht und keine zweite Aufbereitung daneben. Im Bild kommt die Belegungsansicht ausschließlich als `BL`, als ausgeschlossene Arbeitskopie, vor. Der Knoten `MOD` hat damit genau einen Abnehmer, die Ausgabefunktion.

Ein Graph, der die Zusage trüge, hätte an `MOD` zwei ausgehende Kanten: eine zur Ausgabefunktion und eine zur Belegungsansicht. Erst diese Gabelung macht aus der Behauptung eine sichtbare Struktur, und erst sie macht ihren Bruch später sichtbar, falls eine spätere Runde daneben etwas Zweites baut. Am Code ist die Gabelung vorhanden: die Ansicht holt ihre beiden Spalten aus `modell.funktionstext(stelle)` und `modell.tastentext(stelle)` (`crates/krk-ui/src/appkit/belegungsansicht.rs:347` und `:349`).

Der Graph dementiert die Directive also nicht. Zwei getrennte Pfade von der Quelle zur Darstellung stehen nicht darin, und das ist die gute Nachricht. Er belegt sie nur nicht.

### 3. `anzeige()` steht auf der falschen Ebene

Die drei Spalten sind mit drei verschiedenen Beziehungen gezeichnet. Spalte 1 kommt über eine Kette aus zwei Kanten (`B → MOD → AUS`), Spalte 2 und Spalte 3 kommen als Wurzeln ohne Herkunft unmittelbar aus `ANZ` und `WB`. Der Leser nimmt daraus mit, `ANZ` sei eine eigene Quelle neben dem Modell.

Am Code liegt es anders: `anzeige` wird nicht neben dem Modell abgerufen, sondern darin. `Belegungsmodell::tastentext` ruft es auf (`crates/krk-ui/src/belegungsmodell.rs:412`), und über diese Methode bekommt die Bildschirmansicht ihre Spalte "Belegung". Spalte 1 und Spalte 2 fallen also beide aus demselben Knoten. `inference:` Ob die neue Ausgabefunktion `anzeige` unmittelbar ruft oder über `tastentext` geht, ist eine Planerfrage und im Spec unter `## Offen für den Planner` bewusst offen; der Graph nimmt sie aber zeichnerisch vorweg, und zwar in der Form, die weiter von der bestehenden Ansicht entfernt liegt.

`WB` als Wurzel ist dagegen richtig gezeichnet. Die Beschriftungen der Wirkungsbereiche sind das eine, was diese Runde neu anlegt, und der Knoten sagt es in seiner Beschriftung auch.

### 4. `AUS` sitzt im Teilgraphen "Ergebnis", ist aber kein Ergebnis (kosmetisch)

Die Ausgabefunktion ist der verarbeitende Schritt, nicht sein Resultat. Ihre Einordnung neben die geschriebene Datei und die Statuszeile ist der zweite Grund, aus dem das Rendern die Richtung `TD` nicht halten kann: zwei Kanten verlassen `Ergebnis` wieder nach `Quellen`. Wir führen das als kosmetisch, weil es an der Aussage nichts ändert.

### 5. `BL` außerhalb aller Teilgraphen ist richtig, seine Herkunft fehlt (geringfügig)

Der Knoten steht bewusst in keiner der drei Schichten, denn er ist ausdrücklich keine Quelle. Das ist vertretbar und ausdrücklich kein Befund. Nur seine Herkunft fehlt: `BL` ist selbst ein `Belegungsmodell` über einer Kopie derselben Belegung (`crates/krk-ui/src/appkit/anwendung.rs:2159`, `Belegungsmodell::neu(self.ivars().belegung.borrow().clone())`). Als loser Kasten liest er sich wie ein fremdes Ding. Eine Kante von der Belegung zu ihm würde zeigen, dass die Abweichung aus einer Kopie entsteht und nicht aus einem zweiten Bestand, und das ist genau die Aussage, die der Abschnitt `## Die Abweichung bei offener Belegungsansicht` in Prosa trägt.

## Was ein sauberer Nachzug verlangt

Der Spruch ist **acceptable** und nicht **tangled**, deshalb steht hier keine Umgestaltung des Entwurfs. Der Entwurf stimmt. Was folgt, ist die kleinste Änderung, die das Bild mit ihm zur Deckung bringt, und sie ist Sache des Planners oder einer Nachbesserung des Specs, nicht dieser Bewertung.

Die Kante `AUS -->|"liest, ändert nicht"| B` entfällt. Sie sagt nichts, was `B → MOD → AUS` nicht schon sagt, und mit ihr verschwinden der Kreis und die gekreuzten Kanten. Wer den Lesevorgang festhalten will, dreht sie um und beschriftet sie als Fluss.

Die Belegungsansicht kommt als Knoten hinzu, mit einer Kante von `MOD` zu ihr. Das ist der eigentliche Gewinn: die Zusage der Directive wird von einer Behauptung im Fließtext zu einer Gabelung, die man sieht. `ANZ` rückt dabei hinter `MOD`, weil beide Abnehmer die Schreibweise über dasselbe Modell beziehen.

## Nebenbefund außerhalb der Diagrammbewertung

Der Spec nennt `anzeige()` unter `crates/krk-ui/src/belegungsmodell.rs:527`; die Signatur steht in Zeile 530, der Kommentarblock beginnt bei 523. Der Verweis führt an die richtige Stelle, die Zahl ist um drei daneben. `pfade.rs:71` für `benutzerverzeichnis()` stimmt genau. Beides am Code geprüft am 260811-0803.
