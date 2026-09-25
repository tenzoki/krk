# Konzeptprüfung: Spec "Der eingebaute Editor mit Roh- und Formatansicht und Textmarken"

**Datum:** 2026-08-07 22:02
**Ziel:** `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_o_spec-eingebauter-editor-mit-textmarken.md`
**Urteil:** acceptable
**Geprüfte Diagramme:** 3  |  **Prüfung:** by-tool (`@mermaid-js/mermaid-cli` 11.16.0 über `npx`)

## Urteil

Die drei Diagramme tragen das Design, das sie zeigen sollen, und ein einziger Befund hält sie vom Urteil "clean" ab. Alle drei parsen ohne Beanstandung, keiner trägt einen Zyklus außerhalb des Zustandsautomaten, keiner trägt einen Knoten mit auffälliger Ausgangsverzweigung: der höchste Wert im ganzen Dokument liegt bei zwei. Das ist für einen Spec dieser Länge ein ungewöhnlich flaches Bild, und es spricht für den Zuschnitt der acht Fähigkeiten.

Der Befund liegt beim zweiten Diagramm, dem Lesezeichen-Graphen. Er zeigt drei Gegenstände in einem Bild: die Ablageform in `bookmarks.toml`, die Wirkung einer Auswahl in der Leiste und das Verfahren, mit dem der Sprung eine verschobene Textstelle wiederfindet. Der Pfeil bedeutet darin an drei Stellen Verschiedenes, nämlich "enthält", "löst aus" und "prüft danach". Der obere Teil ist ein Datenmodell, für das die Typtabelle in `rules/design-diagrams.md` ein `erDiagram` vorsieht und keinen Flowchart. Verdeckt wird durch die Mischung nichts: der Graph teilt sich bei `O` und `T` und läuft danach kreuzungsfrei auseinander. Deshalb erkennen wir auf "acceptable" und nicht auf "tangled". Die Dichte ist an keiner Stelle das Problem, und wir halten den Graphen ausdrücklich nicht für einen Hairball.

Zwei Sachen fehlen. C4 beschreibt den einzigen Bereich in KRK, der einen verlierbaren Zustand hält, nennt fünf Anlässe für die Nachfrage und drei Antworten darauf, und trägt kein Diagramm. Das ist der stärkste Kandidat für ein viertes Bild. In C1 sind die beiden Übergänge aus dem Zustand `Editor` heraus unbedingt gezeichnet, obwohl C4 genau an diese beiden Kanten eine Bedingung hängt.

## Messungen je Diagramm

| # | Zeile | Typ | Knoten | Kanten | Max. Ausgang | Max. Eingang | Zyklen | Geschichtet | Urteil |
|---|-------|-----|--------|--------|--------------|--------------|--------|-------------|--------|
| 1 | 26 | `stateDiagram-v2` | 4 (3 Zustände + Start) | 7 | 2 | 2 | 3 wechselseitige Paare, in einem Zustandsautomaten erwartbar | `direction LR`, keine Subgraphen nötig | clean |
| 2 | 44 | `flowchart TD` | 11 | 12 | 2 (`LZ`, `O`, `T`, `P`, `S`) | 2 (`ZS`, `G`) | 0 | `TD`, keine Subgraphen | acceptable |
| 3 | 67 | `flowchart TD` | 9 | 10 | 2 (`F`, `T`, `N`, `V`) | 3 (`W`, Senke) | 0 | `TD`, keine Subgraphen | clean |

Kanten-Knoten-Verhältnis: 1,09 im zweiten und 1,11 im dritten Diagramm. Unbeschriftete Kanten: drei im zweiten (`B → LZ`, `LZ → O`, `LZ → T`), zwei im dritten (`E → F`, `M → N`). Waisen oder unerreichbare Knoten: keine.

## Befunde

**1. Diagramm 2 mischt drei Gegenstände, substanziell und begrenzt.** Die Knoten `B["bookmarks.toml unter Application Support"]` und `LZ["Eine Liste, eine Ordnung, zwei Sorten"]` beschreiben eine Ablage, `O` und `T` beschreiben zwei Satzarten mit ihren Feldern, `ED`, `P`, `S`, `ZS` und `ME` beschreiben einen Ablauf zur Laufzeit. Am deutlichsten wird die Mischung am Knoten `ED["Editor"]`: er ist Ziel der Kante `T -->|"Auswahl öffnet die Datei im Editor"| ED` und zugleich Quelle der Kante `ED -->|"springt zur gemerkten Zeile"| P`. Im ersten Fall ist er ein Bauteil, im zweiten ein Schritt. Dieselbe Doppelung trägt `O`, das einmal auf `DF` zeigt (eine Auswahl wirkt) und einmal auf `G` (eine Prüfung greift). Regel 5 der Autorenregeln verlangt hier zwei Bilder: ein `erDiagram` für die eine Liste mit zwei Sorten und einen `flowchart` für den Sprung mitsamt der Suche in der Nähe. Der Preis der jetzigen Form ist gering, weil die Teilbäume kurz sind und sich nach der Verzweigung nicht wieder verflechten. Wir tragen den Befund vor, ohne daraus eine Neuplanung abzuleiten.

**2. C4 trägt keinen Zustandsautomaten, obwohl es der zustandsreichste Teil des Specs ist.** Fünf Anlässe lösen die Nachfrage aus: Editor schließen, Anwendung beenden, andere Datei aufnehmen, Vorschau einblenden, Sitzungssicherung als offene Frage. Drei Antworten stehen zur Wahl, und "abbrechen" führt in den Ausgangszustand zurück. Dazu kommt die von außen geänderte Datei als eigener Fall. Acht Abnahmekriterien tragen diesen Zusammenhang heute in Prosa. Ein `stateDiagram-v2` über die Zustände "unverändert", "geändert", "Nachfrage offen" und "gesichert" zeigte in einem Blick, was die acht Zeilen einzeln aufzählen, und machte prüfbar, ob die Fallunterscheidung vollständig ist. Mittelschwerer Befund nach dem Abschnitt "When to include a diagram" der Regel.

**3. Die beiden Kanten aus `Editor` heraus sind unbedingt gezeichnet, obwohl C4 eine Bedingung an sie hängt.** Im ersten Diagramm stehen `Editor --> Vorschau: Editor schließen, oder Vorschau anzeigen` und `Editor --> Nichts: Editor schließen bei ausgeblendeter Vorschau` ohne Vorbehalt. C4 sagt für beide zu, dass bei ungespeicherten Änderungen zuerst die Nachfrage erscheint und "abbrechen" den Übergang verhindert. Der Spec sagt selbst, dieses Diagramm trage die Abnahmekriterien von C1, und genau an dieser Stelle trägt es sie unvollständig. Ein Wächter am Kantenlabel, etwa "Editor schließen, wenn nichts offen ist", genügte; ein eigener Zustand für die Nachfrage gehörte in das Bild zu Befund 2 und nicht hierher.

**4. Dieselbe Auslösung ist im ersten Diagramm zweimal verschieden beschriftet.** Die Kante `Vorschau --> Editor` trägt "F4, oder Übergang aus der Vorschau", die Kante `Nichts --> Editor` trägt "F4, aus dem Dateifenster". Gemeint ist beide Male dasselbe F4 aus dem Dateifenster. Kosmetisch, kein Designbefund.

**Was der Prüfung standhält, und zwar bemerkenswert.** Diagramm 3 bildet nach eigener Aussage den gebauten Weg ab und nicht einen gewünschten, und es hält diese Zusage: die Kette von `E` über `F`, `T`, `M`, `N` und `V` ist linear, jede Verzweigung ist zweiwertig und beschriftet, und die drei Wege nach `W` sind kein God-Node, sondern eine gemeinsame Senke mit der Bedeutung "unverändert an AppKit weiter". Der Eingangsgrad 3 auf `W` ist damit sachlich richtig und kein Strukturfehler. Der Spec benennt außerdem, dass der erste Verdächtige des Defekts aus C8 vor dem Graphen liegt, beim Abgriff des Menüs. Ein Diagramm, das seine eigene Grenze nennt, ist mehr wert als eines, das sie überzeichnet. Ebenso trägt Diagramm 2 die offene Frage an einem benannten Knoten: `S["Suche in der Nähe"]` und seine beiden Ausgänge hängen ausdrücklich am Datensatz `260807-2147_o_wie-weit-reicht-die-suche-in-der-naehe-einer-textmarke.md`. Der Graph zeigt damit, wo die Antwort ankommt, statt die Lücke zu glätten.

**Kein Befund ist die fehlende Reihenfolge der Fähigkeiten.** C1 bis C8 ließen sich als Abhängigkeits-DAG zeichnen, und der Spec tut es nicht. Das ist Absicht: der Abschnitt "Offen für den Planner" überlässt die Reihenfolge dem Plan und nennt allein den sachlichen Grund, C8 vorzuziehen. Ein Diagramm an dieser Stelle behauptete eine Festlegung, die der Spec bewusst nicht trifft.

---

## Abgleichvermerk 260810-0805

Der Spec, den dieser Bericht geprüft hat, ist seither zweimal gewachsen: am 260809-2043 um die drei Anzeigefähigkeiten C9, C10 und C11, und am 260810-0714 um die Nachträge aus S42. **Die Diagramme dieses Berichts sind damit nicht mehr der volle Diagrammbestand des Specs**, und der Bericht ist als Momentaufnahme vom 260807-2202 zu lesen, nicht als Aussage über den heutigen Stand.

Der Anlass, aus dem der Bericht entstand, hat nebenbei einen Defekt hervorgebracht, der inzwischen geschlossen ist: `shared/issues/260808-0017_c_fusion-rules-gibt-conceptrev-die-stilprofile-nicht-aus.md`. Der `conceptrev` bekommt beide Stilprofile heute über `fusion-rules`, nachgemessen am 260810-0805 für alle sechzehn Agentennamen.

Am Bericht selbst ist nichts geändert.
