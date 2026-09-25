# Konzeptprüfung: Spec KRK Navigator-Gerüst (Runde 1)

**Datum:** 2026-08-02 11:18
**Ziel:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`
**Verdikt:** acceptable
**Geprüfte Diagramme:** 2 (zusätzlich 1 Vergleichsdiagramm aus dem Circle-Datensatz)
**Validierung:** by-tool (mermaid 11.16.0 `parse()` unter jsdom, Node v24.2.0)

## Verdikt

Beide Diagramme sind strukturell tragfähig und zeigen ein Design, das an einem Blick erfassbar ist; zum Verdikt "clean" fehlen zwei kleine Korrekturen. Beide Blöcke parsen fehlerfrei, kein Knoten ist verwaist, alle vierzehn Kanten des ersten Diagramms tragen ein Label, der Diagrammtyp passt in beiden Fällen zum Inhalt, und jedes Diagramm zeigt genau ein Anliegen. Die beiden auffälligen Kennzahlen des ersten Diagramms, ein Ausgangsgrad von 5 am Knoten `Tastenbelegung` und zwei Zyklen über `Lokales Dateisystem`, sind im Fließtext darunter begründet und halten der Prüfung stand. Abzüge entstehen an zwei Stellen. Die Selbstprüfung am Dokumentende nennt 13 Kanten, tatsächlich sind es 14. Und das erste Diagramm bildet die Ausblendbarkeit des zweiten Dateifensters nicht ab, obwohl C7 sie ausdrücklich zusagt. Keiner der beiden Punkte verdeckt das Design, beide sind in wenigen Minuten behoben.

## Messwerte je Diagramm

| # | Typ | Knoten | Kanten | Verhältnis | Max. Ausgangsgrad | Max. Eingangsgrad | Zyklen | Verwaist | Kanten ohne Label | Geschichtet | Verdikt |
|---|-----|--------|--------|-----------|-------------------|-------------------|--------|----------|-------------------|-------------|---------|
| 1 | flowchart LR | 7 | 14 | 2,00 | `K` Tastenbelegung = 5 | `P1`/`P2`/`V`/`O` = je 3 | 2, beide erklärt | 0 | 0 | teilweise (1 subgraph) | acceptable |
| 2 | stateDiagram-v2 | 2 (+Startknoten) | 4 | 2,00 | `Aktiv` = 2 | `Aktiv` = 2 | 2 Selbstübergänge, gewollt | 0 | 0 | entfällt | clean |

Zum Vergleich das Diagramm aus `_t_circle.md`, Abschnitt `## Grounding snapshot`: 7 Knoten, 8 Kanten, Verhältnis 1,14, max. Ausgangsgrad 2, keine Zyklen, keine verwaisten Knoten, alle Kanten beschriftet.

Die Zyklen des ersten Diagramms lauten vollständig: `FS -> P1 -> O -> FS` und `FS -> P2 -> O -> FS`. Der einzige Knoten ohne eingehende Kante ist `K`, der einzige ohne ausgehende ist `V`.

## Befunde

### 1. Die Selbstprüfung nennt eine falsche Kantenzahl (gering, Genauigkeit)

Zeile 295 schreibt "7 Knoten und 13 Kanten". Die Knotenzahl stimmt, die Kantenzahl nicht: das Diagramm führt in den Zeilen 33 bis 46 vierzehn Kanten. Damit liegt das Kanten-Knoten-Verhältnis bei 2,00 statt bei 1,86. Der Unterschied ist für die Bewertung folgenlos, der falsche Wert steht aber ausgerechnet in der Zeile, mit der der Spec seine eigene Dichte belegt. Ein Leser, der die Selbstprüfung als Beleg nimmt, rechnet mit einem dünneren Graphen als dem vorliegenden.

### 2. C7 sagt drei ausblendbare Bereiche zu, das Diagramm zeigt zwei (gering bis mittel, Abdeckung)

Der Knoten `K` trägt zwei Kanten mit dem Label "blendet ein und aus", auf `L` (Lesezeichen) und auf `V` (Vorschau). C7 nennt in seinem zweiten Abnahmekriterium ausdrücklich drei Bereiche: die Lesezeichenleiste, das zweite Dateifenster und die Vorschau. Die Kante `K -> P2` existiert zwar, trägt aber das Label "navigiert in". Wer das Diagramm als Auskunft darüber liest, was die Tastatur erreicht, übersieht damit, dass auch das zweite Dateifenster ausblendbar ist. Der Befund betrifft eine fehlende Kante, nicht eine fehlende Struktur, und lässt sich durch eine zweite Kante `K -->|blendet ein und aus| P2` oder durch ein zusammengesetztes Label auflösen.

### 3. Das Zustandsdiagramm kennt keinen Endzustand (gering, Vollständigkeit)

Das zweite Diagramm beginnt mit `[*] --> Aktiv`, endet aber nirgends. Nach C6 und C1 lässt sich ein Vorschau-Tab schließen, der modellierte Tab entsteht im Graphen also, verschwindet aber nie. Wir halten den Befund bewusst klein, weil die Einleitung des Diagramms es auf das Halteverhalten pro Tab begrenzt und nicht auf dessen gesamte Lebensdauer. Wer die Abgrenzung so liest, hat recht; ein `Aktiv --> [*]` und ein `Inaktiv --> [*]` würden den Graphen dennoch schließen und kosten zwei Zeilen.

### 4. Umlaute sind ohne Notwendigkeit ersetzt (kosmetisch)

Die Labels des ersten Diagramms schreiben "Geraete", "loeschen" und "Eintraege", das zweite "zurueck" und "unveraendert". Die Prosa des Dokuments und das Diagramm im Circle-Datensatz verwenden die Umlaute direkt ("Geräteordner", "öffnet"). Wir haben geprüft, ob die Transliteration technisch nötig ist: ein Testdiagramm mit `A["Lesezeichen und Geräte"] -->|löscht Einträge| B` und ein Zustandsdiagramm mit `Inaktiv --> Aktiv: Nutzer wechselt zurück, Inhalt unverändert` parsen beide fehlerfrei, sowohl in einem Flowchart-Label in Anführungszeichen als auch in einem unquotierten Übergangslabel. Die Ersetzung kauft also nichts und lässt die deutschen Labels schlechter lesen als die Prosa daneben.

### 5. Kein Befund: der Ausgangsgrad 5 am Knoten `Tastenbelegung`

`K` zeigt auf fünf der sechs übrigen Knoten. Nach der reinen Kennzahl ist das ein God-Node-Kandidat, nach der Sache ist es keiner. Die Begründung in Zeile 49 trägt: wenn die Tastatur die einzige vollständige Bedienoberfläche ist, muss jede Funktion von ihr aus erreichbar sein, und C2 macht genau das zum Abnahmekriterium. Ein Graph, der diese Zusage einhält, hat den hohen Ausgangsgrad zwangsläufig.

Eine Beobachtung für den Planner, ausdrücklich kein Mangel des Spec: `K` bündelt zwei Dinge, die technisch auseinanderfallen, nämlich die frei konfigurierbare Zuordnung aus C3 und den Weg, auf dem ein Tastenereignis zur Funktion wird. Im Plan wird daraus vermutlich eine Zuordnungstabelle plus eine Versandstelle, und der Ausgangsgrad wandert auf die Versandstelle. Auf Spec-Ebene wäre diese Trennung vorweggenommene Technikwahl, die das Dokument in `## Offen für den Planner` bewusst aufschiebt. Die jetzige Darstellung ist auf ihrer Flughöhe korrekt.

### 6. Kein Befund: die zwei Zyklen über das Dateisystem

Beide Zyklen laufen über `O` (Dateioperationen) und `FS` (Dateisystem) zurück in je ein Dateifenster. Zeile 49 benennt sie und nennt den Grund: lesen, auswählen, schreiben, erneut lesen ist die Arbeitsschleife eines Dateimanagers, und C4 macht das automatische Auffrischen nach jeder Operation zum Abnahmekriterium. Ein Datenfluss-Kreis dieser Art ist keine Abhängigkeitsumkehr zwischen Modulen. Die Anforderung aus `design-diagrams.md`, einen gewollten Zyklus in der Prosa zu begründen, ist erfüllt.

### 7. Beobachtung: die Schichtung ist nur zur Hälfte gezeichnet

Ein einziger `subgraph` fasst die vier Oberflächenbereiche zusammen, die drei übrigen Knoten stehen ungruppiert daneben. Damit sieht der Leser die Fensterzeile, aber keine Tiers. Bei 7 Knoten und 14 Kanten liest sich der Graph trotzdem ohne Mühe, und zwei weitere `subgraph`-Blöcke für Eingabe und für Ausführung wären eher Zeremonie als Gewinn. Wir führen den Punkt als Beobachtung, nicht als Mangel, und würden ihn erst dann aufgreifen, wenn der Graph in einer späteren Runde durch Editor und Git wächst.

## Abgleich mit dem Circle-Datensatz

Der Spec verfeinert die Struktur des Grounding-Diagramms konsistent. Wir haben die Kanten beider Graphen gegeneinander gestellt, und jede Beziehung des Datensatzes findet sich im Spec wieder oder ist mit Begründung entfallen:

| Kante im Circle-Datensatz | Im Spec |
|---|---|
| `L -> P1` "setzt Ordner" | unverändert übernommen |
| `L -> P2` "setzt Ordner" | unverändert übernommen |
| `P1 -> V` "Auswahl zeigt" | übernommen, Label auf "aktive Auswahl" geschärft |
| `P2 -> V` "Auswahl zeigt" | übernommen, Label auf "aktive Auswahl" geschärft |
| `P1 -> D` "Quelle" | übernommen als `P1 -> O`, Label unverändert |
| `P2 -> D` "Ziel" | übernommen als `P2 -> O`, Label unverändert |
| `V -> E` "öffnet" | entfallen, Editor ist nicht Teil der Runde 1 |
| `E -> G` "arbeitet auf Datei" | entfallen, Git ist nicht Teil der Runde 1 |

Der Spec entfernt genau die zwei Kanten und die zwei Knoten, die seine eigene Rundenabgrenzung ausschließt, und fügt zwei Knoten hinzu, die der Datensatz nicht hatte: die Tastenbelegung und das lokale Dateisystem. Beide Ergänzungen schließen echte Lücken des Vergleichsdiagramms. Im Datensatz ist `D` (Dateioperationen) eine Senke, die Dateien verändert, ohne dass ein Speicher im Graphen vorkommt; der Spec macht mit `O -> FS` und den beiden Rückkanten sichtbar, worauf die Operationen wirken. Die Bedienung über die Tastatur fehlte im Datensatz vollständig, obwohl sie in beiden Dokumenten als Maxime steht.

Eine Namensdrift bleibt. Der Datensatz schreibt "Vorschaufenster", "Lesezeichen und Geräteordner" und "Oberfläche"; der Spec schreibt "Vorschau", "Lesezeichen und Geraete" und "Fensterzeile". Der Knoten für die Dateioperationen heißt im Datensatz `D` und im Spec `O`. Wer beide Graphen nebeneinander legt, muss die Zuordnung selbst herstellen. Das kostet keinen Punkt im Verdikt, widerspricht aber der Regel "eine Benennung pro Sache".

## Prüfverfahren

Die Validierung lief mit Werkzeug, nicht per Lesen. Beide Blöcke des Spec und der Block des Circle-Datensatzes wurden über `mermaid.parse()` aus mermaid 11.16.0 unter jsdom geprüft. Ergebnis: `flowchart-v2` OK, `stateDiagram` OK, `flowchart-v2` OK. Die Kennzahlen oben stammen aus einer Auszählung der aus dem Quelltext übertragenen Kantenliste, die Zyklen aus einer vollständigen Tiefensuche über beide Flowcharts.
