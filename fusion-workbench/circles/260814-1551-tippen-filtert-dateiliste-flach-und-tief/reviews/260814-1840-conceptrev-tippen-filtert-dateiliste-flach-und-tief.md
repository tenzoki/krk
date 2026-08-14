# Concept Evaluation: Spec „Tippen filtert die Dateiliste, flach und als gefilterter Ordnerbaum"

**Date:** 2026-08-14 18:40
**Target:** `fusion-workbench/circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`
**Verdict:** tangled
**Diagrams evaluated:** 2  |  **Validation:** by-tool (`@mermaid-js/mermaid-cli` 11.16.0, beide Blöcke gerendert, Exit 0)

## Verdict

**Das zweite Bild kann den negativen Befund nicht erzeugen, den drei Abnahmekriterien von ihm verlangen.** Der Durchlauf im Abschnitt `## Der Durchlauf und was ihn beendet` trägt keine Kante für „keine Einträge mehr". Der Knoten `Durchlauf endet` hat genau einen Eingang, und der ist die gestrichelte Abbruchkante des Nutzers. Damit produziert der gezeichnete Mechanismus die Auskunft „unter diesem Ordner liegt kein Treffer" auf keinem Weg, während C2.6, C3.10 und der Zweig `U|nein` des ersten Bildes sie voraussetzen. Das erste Bild ist strukturell sauber und deckt sich auf jedem nachgerechneten Pfad mit C1.6, C2.5, C2.6 und C2.8; es erbt seinen einen Mangel aus dem zweiten. Die Dichte ist in beiden Bildern unauffällig, und keines trägt einen Gott-Knoten: der größte Ausgangsgrad ist 2.

## Per-diagram measurements

| # | Abschnitt | Typ | Knoten | Kanten | Ratio | Max fan-out | Max fan-in | Kreise | Waisen | Untergraph | Verdikt |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `## Wie eine Zeile entsteht` | flowchart TD | 9 | 13 | 1.44 | 2 (`V`, `F`, `N`, `T`, `D`, `U`) | 4 (`JA`) | 0 | 0 | nein | acceptable |
| 2 | `## Der Durchlauf und was ihn beendet` | flowchart TD | 11 | 13 | 1.18 | 2 (`START`, `ABST`) | 2 (`PRUEF`, `ABST`, `LISTE`) | 2 | 0 | ja (`FADEN`) | tangled |

Erreichbarkeit im zweiten Bild, gerechnet: `ENDE` ist von `SOFORT`, `LISTE`, `TREFFER`, `MELD` und `WACHS` aus nicht erreichbar. Die beiden Kreise sind `ABST → PRUEF → ART → ABST` und `ART → WEITER → PRUEF → ART`.

## Findings

### B1 (substanziell, Bild 2): Der Durchlauf hat keinen Ausgang für „kein Treffer darunter"

Der Knoten `ENDE` („Durchlauf endet") trägt genau eine eingehende Kante, die gestrichelte von `ABST`. Jeder andere Weg endet in `LISTE`, und `LISTE` ist eine Senke. Ein Ordner wird also nach dem Bild nur auf zwei Arten fertig: es findet sich ein Treffer, oder der Nutzer bricht ab. Die dritte Möglichkeit, dass der Ordner abgearbeitet ist und nichts enthielt, kommt im Graphen nicht vor. Es gibt keine Kante „keine Einträge mehr" aus `WEITER` oder `PRUEF` heraus.

Der Kreis `ART → WEITER → PRUEF → ART` verlässt sich damit auf zwei Ausgänge, die beide an eine Bedingung geknüpft sind: `PRUEF|ja` verlangt einen Namenstreffer, `ART|ja` verlangt einen Unterordner. Ein Ordner, der nur nicht passende gewöhnliche Dateien enthält, erfüllt keine von beiden. Der gezeichnete Durchlauf kommt aus diesem Kreis nicht heraus.

Drei Stellen des Specs verlangen genau den Befund, den das Bild nicht liefert:

- **C2.6:** „Ein Ordner, unter dem kein Treffer liegt und dessen Name nicht passt, ist nicht zu sehen." Diese Entscheidung braucht den negativen Ausgang.
- **C3.10:** „Ein Ordner, den der Durchlauf nicht öffnen kann, gilt als ‚kein Treffer darunter'." Das Bild hat für den Fehlschlag beim Absteigen überhaupt keinen Zweig; `ART|ja` führt unbedingt auf `ABST`.
- **Bild 1, Knoten `U`** („liegt unter ihm ein Treffer?"): Der Zweig `U|ja` wird vom zweiten Bild bedient, `U|nein` von nichts. Die beiden Bilder setzen sich in ihrem negativen Ast nicht zusammen.

Das ist derselbe Fehlertyp, den die Runde 7 dreimal gefunden hat: die Abnahmekriterien sagen mehr zu, als der gezeichnete Mechanismus einlöst.

### B2 (substanziell, Bild 2): Die Abbruchkante hängt am falschen Knoten, und C3.4 fällt damit

C3.4 sagt zu: „Der Abbruch greift innerhalb von zwei Stapeln, wie beim vorhandenen Lesevorgang." Die Abbruchkante verlässt allein `ABST`, und dieser Knoten trägt die Aufschrift „absteigen, Stapel zu 1.024 Eintraegen". Er beantwortet damit zwei verschiedene Fragen zugleich: in einen Unterordner hinabsteigen, und den nächsten Stapel des gerade gelesenen Ordners holen. Einen eigenen Knoten für die Stapelgrenze gibt es nicht.

Die Folge ist am Graphen abzulesen. Ein Ordner mit 50.000 gewöhnlichen Einträgen durchläuft `WEITER → PRUEF` fünfzigtausendmal und passiert `ABST` dabei kein einziges Mal. Der Abbruch wird auf diesem Weg nie geprüft. Das Bild hält die Zwei-Stapel-Zusage nur für Bäume, deren Ordner wenige Einträge und viele Unterordner haben, und das ist keine Eigenschaft, die ein Spec zusichern kann.

### B3 (substanziell, Bild 2 gegen C3.1): Die Aufschrift des Untergraphen entscheidet eine Frage, die der Spec offen führt

Drei Stellen des Dokuments sagen Verschiedenes über dieselbe Sache:

| Stelle | Aussage |
|---|---|
| Untergraph `FADEN` | „eigener Faden, je Ordner des angezeigten Ordners" |
| C3.1 | „Der Durchlauf läuft auf einem eigenen Faden … ein Kanal mit der Kapazität eines Stapels." |
| `## Offen für den Planner` | „Ein Faden je Tab oder ein Faden je Ordner, ein Kanal oder mehrere … entscheidet der Planner." |

Das Bild ist von den dreien das Konkreteste und wird als die Festlegung gelesen werden. Unter seiner Lesart erzeugt ein angezeigter Ordner mit 200 Unterordnern 200 Fäden, und C3.6 („Je Tab läuft nie mehr als einer") braucht dann eine Lesart, in der „einer" Durchläufe zählt und keine Fäden. C3.1 nennt daneben genau einen Kanal, was zu einem Faden je Ordner nur mit einer Zuordnung passt, die das Bild nicht zeigt.

Entweder nimmt die Aufschrift das „je Ordner" zurück, oder C3.1 und die offene Frage ziehen nach. Beides ist vertretbar; unentschieden bleiben kann es nicht, weil die drei Stellen sich heute widersprechen.

### B4 (Bild 2, Prosa gegen Graph): Die Prosa zählt einen Kreis, der Graph trägt zwei

Der Einleitungssatz des Abschnitts sagt: „Es trägt genau einen Kreis, und der ist gewollt." Gemessen sind es zwei: `ABST → PRUEF → ART → ABST`, das gemeinte Absteigen, und `ART → WEITER → PRUEF → ART`, das Weiterrücken innerhalb eines Ordners.

Die Fehlzählung ist nicht kosmetisch, denn der nicht genannte Kreis ist genau der aus B1, dem der Ausgang fehlt. Der Satz erklärt den harmlosen Kreis für gewollt und übergeht den defekten.

### B5 (substanziell, beide Bilder): Zwei Schnitte für dieselbe Frage

Bild 1 fragt in `T`: „ist es ein Ordner?". Bild 2 fragt in `ART`: „Ordner und keine Verknuepfung?". Das sind zwei verschiedene Prädikate für dieselbe Klasse von Einträgen, und der Unterschied trifft die symbolische Verknüpfung auf einen Ordner.

Bild 2 behandelt sie richtig: sie geht auf `WEITER`, es wird nicht abgestiegen, und weil `PRUEF` vor `ART` steht, erscheint sie bei passendem Namen als Treffer. Das deckt sich mit C3.9.

Bild 1 lässt sie offen. Antwortet `T` mit „ja", landet die Verknüpfung bei `U` („liegt unter ihm ein Treffer?"), und diese Frage beantwortet der Durchlauf für keine Verknüpfung, weil er in keine hinabsteigt. Der Knoten stellt dann eine Frage, die aus dem, was der Mechanismus erhebt, nicht zu entscheiden ist. Antwortet `T` mit „nein", fällt die Verknüpfung auch bei ausgeschaltetem „Deep" weg, und C1.6 („Ist ‚Deep' aus, bleibt jeder Ordner sichtbar") deckt sie dann nicht mehr.

Das sichtbare Ergebnis stimmt in beiden Lesarten mit C3.9 überein, deshalb ist das kein Widerspruch zu einem Kriterium. Es ist ein Knoten, dessen Antwort der Graph nicht liefert. Der Spec benennt das zugrunde liegende Prädikat selbst als überladen: „Der Defekt hängt an `Eintrag::ist_ordner`, das mehrere Aufrufer mit verschiedenen Fragen hat" (Abschnitt `## Nicht Gegenstand dieser Runde`).

### Was trägt, und zwar ohne Einschränkung

Die folgenden Punkte sind nachgerechnet und in Ordnung. Sie stehen hier, damit die fünf Befunde nicht als Urteil über die Bilder insgesamt gelesen werden.

- **Beide Blöcke parsen.** `mmdc` 11.16.0 rendert beide ohne Fehler.
- **Bild 1 ist ein sauberer Entscheidungsbaum.** Neun Knoten, dreizehn Kanten, kein Kreis, größter Ausgangsgrad 2, keine Waise, jeder Entscheidungsknoten binär mit `ja`/`nein` beschriftet. Jede Fallunterscheidung ist damit je Knoten überschneidungsfrei und vollständig. Der hohe Eingangsgrad 4 an `JA` ist die normale Gestalt eines Entscheidungsbaums mit zwei Senken und kein Gott-Knoten.
- **Bild 1 deckt sich mit den Kriterien, die es betreffen.** `D|nein → JA` ist genau C1.6. Das Paar `N|ja → JA` und `U|ja → JA` ist genau C2.5. `T|nein → NEIN` und `U|nein → NEIN` sind C2.6. Der Vorrang von `V` vor `F` entspricht C6.8, wonach das Ausblenden versteckter Dateien und der Filter zwei Prüfschritte in derselben Sicht sind und keine zwei Regeln.
- **Die Abbruchbeschriftung in Bild 2 ist vollständig.** „Filtertext geloescht oder geaendert" deckt C3.5 und C3.6, „Ordnerwechsel, Deep aus" deckt C3.7.
- **Der positive Ast von Bild 2 stimmt.** `START → SOFORT → LISTE` ist C3.2, `PRUEF|ja → TREFFER` ist C3.3, `WACHS` ist C4.5.
- **Die Dichte ist in beiden Bildern unauffällig.** Kantenzahl je Knoten 1,44 und 1,18, größter Ausgangsgrad 2, keine Waise, kein Knoten ohne Weg vom Einstieg. Der Typ `flowchart TD` passt zu einem Entscheidungsbaum und zu einem Durchlauf mit Wiederholung.

## What a clean redraw would require

Vier der fünf Befunde verlangen fehlenden Mechanismus und nicht ein aufgeräumteres Bild. Der Planner findet sie in dieser Reihenfolge vor:

1. **Ein zweiter Ausgang für den Durchlauf.** Der Kreis über die Einträge eines Ordners braucht eine Kante „keine Einträge mehr" auf einen Endknoten „Ordner abgearbeitet, kein Treffer". Dieser Knoten ist es, der `U|nein` in Bild 1 bedient, C2.6 entscheidet und der Statuszeile sagt, dass die Zahl steht. Ohne ihn setzen sich die beiden Bilder nicht zusammen.
2. **`ABST` in zwei Knoten teilen.** „In den Unterordner absteigen" und „nächsten Stapel holen" sind zwei Vorgänge. Die Abbruchkante gehört an den Stapelknoten, denn dort ist die Grenze, auf die sich die Zwei-Stapel-Zusage aus C3.4 bezieht.
3. **Einen Fehlschlagzweig am Absteigen.** C3.10 verlangt, dass ein nicht zu öffnender Ordner als „kein Treffer darunter" gilt, ohne Meldung und ohne den Durchlauf anzuhalten. Dieser Zweig mündet auf denselben Endknoten wie Punkt 1.
4. **Die Fadenfrage entweder entscheiden oder aus dem Bild nehmen.** Die Aufschrift des Untergraphen darf nicht festlegen, was `## Offen für den Planner` als offen führt und was C3.1 anders sagt.
5. **Ein Prädikat für „Ordner" über beide Bilder.** Entweder beide Bilder fragen dieselbe Frage, oder die beiden Fragen bekommen verschiedene Namen und der Spec sagt, welche davon `Eintrag::ist_ordner` beantwortet. Der Spec kennt die Überladung dieses Prädikats bereits.

Der Umbau kostet drei bis vier Knoten und keine neue Struktur. Die Bilder sind nicht zu dicht und nicht falsch geschichtet; ihnen fehlt der negative Ast.
