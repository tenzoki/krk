# Concept Evaluation: Spec „Tippen filtert die Dateiliste, flach und als gefilterter Ordnerbaum" (zweite Prüfung)

**Date:** 2026-08-14 19:38
**Target:** `fusion-workbench/circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`
**Vorbewertung:** `reviews/260814-1840-conceptrev-tippen-filtert-dateiliste-flach-und-tief.md` (Urteil `tangled`, fünf Befunde)
**Verdict:** tangled
**Diagrams evaluated:** 2  |  **Validation:** by-tool (`@mermaid-js/mermaid-cli` 11, beide Blöcke gerendert, Exit 0)

## Verdict

**Alle fünf Befunde der ersten Prüfung sind geschlossen, und der Umbau hat einen neuen Fehler derselben Familie erzeugt: das zweite Bild entscheidet jetzt die Sichtbarkeit, die dem ersten gehört, und liefert für drei Eintragsklassen die entgegengesetzte Antwort.** Betroffen ist jeder Ordner, dessen eigener Name den Filtertext trägt und unter dem kein Treffer liegt. Bild 1 zeigt ihn (`N|ja → JA`), Bild 2 endet für ihn auf `der Ordner erscheint nicht`, und C3.13 schreibt diese Lesart als Kriterium fest: „der Ordner erscheint danach auf keinem Weg mehr". Damit widersprechen sich C3.13 einerseits und C2.5, C2.8 sowie C2.13 andererseits. Der Fall der symbolischen Verknüpfung, nach dem eigens gefragt war, ist einer der drei.

Die Messwerte des Shapers stimmen sämtlich. Siebzehn Knoten, sechsundzwanzig Kanten, größter Ausgangsgrad 3 am Stapelknoten, sieben einfache Kreise, jeder Knoten erreicht einen Endpunkt: nachgerechnet und bestätigt. **Kein Kreis ist ohne Ausgang**; der schwächste trägt drei Ausgangskanten. Der Fehler der ersten Runde ist damit nicht wiederholt.

Das Urteil `tangled` hängt an einem einzigen Befund und an einer Ursache, deren Behebung zwei Knotenaufschriften und einen Satz kostet. Es steht nicht für einen zweiten Fehlschlag, sondern dafür, dass eine Frage unentschieden geblieben ist: welches der beiden Bilder die Sichtbarkeit eines Ordners ausspricht.

Daneben steht ein Befund, der die Bilder nicht betrifft und den die Prüfung wegen seiner Sicherheitsrelevanz mitnimmt. **Die elf neuen Kriterien zur Rückschritt-Taste haben kein Bild, und zwei von ihnen widersprechen einander.** C6.10 legt fest, die Fallunterscheidung hänge allein daran, ob ein Filtertext steht; C1.18 verlangt, dass eine Tastenwiederholung die Grenze nicht überträgt, und das ist aus dieser einen Größe nicht zu entscheiden.

## Per-diagram measurements

| # | Abschnitt | Typ | Knoten | Kanten | Ratio | Max fan-out | Max fan-in | Kreise | Kreise ohne Ausgang | Waisen | Untergraph | Verdikt |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `## Wie eine Zeile entsteht` | flowchart TD | 9 | 13 | 1,44 | 2 (`V`,`F`,`N`,`T`,`D`,`U`) | 4 (`JA`) | 0 | 0 | 0 | nein | clean |
| 2 | `## Der Durchlauf und was ihn beendet` | flowchart TD | 17 | 26 | 1,53 | 3 (`STAPEL`) | 3 (`ENDE`,`FERTIG`,`LISTE`,`NOCH`) | 7 | 0 | 0 | ja (`DURCHLAUF`, `direction TB`) | tangled |

Bild 1 ist unverändert. Beide Bilder haben genau eine Quelle und keinen unerreichbaren Knoten; in beiden erreicht jeder Knoten eine Senke.

Die sieben Kreise des zweiten Bildes, mit der Zahl ihrer Ausgangskanten:

| # | Kreis | Ausgänge |
|---|---|---|
| 1 | `NOCH → STAPEL → NOCH` | 3 |
| 2 | `IST → NOCH → PRUEF → IST` | 3 |
| 3 | `FERTIG → ZURUECK → NOCH → STAPEL → FERTIG` | 3 |
| 4 | `ABST → VERKN → FERTIG → ZURUECK → NOCH → PRUEF → IST → ABST` | 4 |
| 5 | `ABST → VERKN → OEFFNEN → STAPEL → NOCH → PRUEF → IST → ABST` | 5 |
| 6 | `ABST → VERKN → OEFFNEN → FERTIG → ZURUECK → NOCH → PRUEF → IST → ABST` | 4 |
| 7 | `ABST → VERKN → OEFFNEN → STAPEL → FERTIG → ZURUECK → NOCH → PRUEF → IST → ABST` | 3 |

## Die fünf Befunde der ersten Prüfung, einzeln nachgeprüft

| Befund | Stand | Beleg am Graphen |
|---|---|---|
| B1 — kein Ausgang für „kein Treffer darunter" | geschlossen | `KEIN` hat drei Zubringer über `FERTIG`: `VERKN|ja` (Verknüpfung), `OEFFNEN|nein` (nicht zu öffnen), `STAPEL|leer` (abgearbeitet). Auf jedem negativen Pfad erreichbar. |
| B2 — Abbruchkante am falschen Knoten | geschlossen | `STAPEL` ist ein eigener Knoten und trägt die gestrichelte Kante nach `ENDE`. Zwischen zwei Besuchen von `STAPEL` liegen höchstens die 1.024 Einträge eines Stapels; der Prüfordner ohne Unterordner aus C3.4 passiert `STAPEL` alle 1.024 Einträge. |
| B3 — Untergraph entscheidet die Fadenfrage | geschlossen | Die Aufschrift lautet „der Durchlauf, je Ordner des angezeigten Ordners" und nennt keine Fadenzahl. C3.1 ist umformuliert, `## Offen für den Planner` führt die Frage weiter. |
| B4 — Prosa zählt einen Kreis, der Graph trägt zwei | geschlossen, Zahl richtig, Herleitung falsch | Sieben Kreise gemessen, sieben genannt. Siehe Befund N4 zur Herleitung. |
| B5 — zwei Schnitte für „ist es ein Ordner?" | geschlossen | Beide Bilder fragen `ist es ein Ordner?`. Die Verknüpfungsregel steht allein in Bild 2, am Knoten `VERKN`. |

## Findings

### N1 (substanziell, Bild 2 gegen C2.5, C2.8 und C2.13): Der Durchlauf spricht ein Urteil aus, das ihm nicht zusteht

Die beiden Ausgangsknoten des zweiten Bildes tragen die Aufschriften `der Ordner erscheint, die Statuszeile zaehlt mit` und `der Ordner erscheint nicht`. Beide sprechen über die Sichtbarkeit, und die Sichtbarkeit entscheidet Bild 1. Was der Durchlauf beisteuert, ist die Antwort auf einen einzigen Knoten des ersten Bildes, `U` („liegt unter ihm ein Treffer?"), und dieser Knoten wird nur erreicht, wenn `N` zuvor mit nein geantwortet hat.

Die Eintrittskante des zweiten Bildes trägt aber keine solche Bedingung. Sie lautet `START -->|je Ordner des angezeigten Ordners| VERKN` und läuft für jeden Ordner, auch für den, den Bild 1 über seinen Namen bereits angenommen hat. Wir haben beide Bilder auf zehn Eintragsklassen gerechnet; drei decken sich nicht:

| Eintragsklasse | Name passt | Treffer darunter | Bild 1 | Bild 2 | |
|---|---|---|---|---|---|
| Ordner | ja | ja | steht in der Liste | erscheint | deckt sich |
| **Ordner** | **ja** | **nein** | **steht in der Liste** | **erscheint nicht** | **Widerspruch** |
| Ordner | nein | ja | steht in der Liste | erscheint | deckt sich |
| Ordner | nein | nein | fällt weg | erscheint nicht | deckt sich |
| **Ordner, nicht zu öffnen** | **ja** | **nein** | **steht in der Liste** | **erscheint nicht** | **Widerspruch** |
| Ordner, nicht zu öffnen | nein | nein | fällt weg | erscheint nicht | deckt sich |
| **Symlink auf Ordner** | **ja** | **nein** | **steht in der Liste** | **erscheint nicht** | **Widerspruch** |
| Symlink auf Ordner | nein | nein | fällt weg | erscheint nicht | deckt sich |

Drei Klassen, eine Ursache: der Ordner trägt den Filtertext im eigenen Namen, und unter ihm liegt nichts.

Der Widerspruch bleibt nicht im Bild. C3.13 schreibt ihn als Kriterium fest: „der Befund beendet den Durchlauf für diesen Ordner, und der Ordner erscheint danach auf keinem Weg mehr." Dagegen stehen drei Kriterien:

- **C2.5** verlangt „jeden Ordner, dessen Name ihn trägt **oder** unter dem irgendwo ein Treffer liegt".
- **C2.8** verlangt ausdrücklich, dass ein Ordner, der allein über seinen Namen passt und unter dem kein Treffer liegt, sichtbar ist und beim Einstieg auf eine leere Liste führt.
- **C2.13** verlangt, dass eine symbolische Verknüpfung auf einen Ordner bei eingeschaltetem „Deep" sichtbar ist, wenn ihr eigener Name den Filtertext trägt.

C3.9 steht auf der anderen Seite und ist in sich stimmig: „Eine Verknüpfung erscheint als Treffer, wenn ihr Name passt", und zugleich lautet ihr Befund „kein Treffer darunter". C3.9 liest den Befund also als Antwort auf `U` und nicht als Urteil über die Sichtbarkeit. C3.13 liest ihn umgekehrt. Zwei Kriterien desselben Abschnitts widersprechen sich.

Die Prosa des Specs hat es richtig: „Dieser Befund bedient den Zweig `liegt unter ihm ein Treffer? — nein` des ersten Bildes" (Abschnitt `## Der Durchlauf und was ihn beendet`). Der Graph und C3.13 gehen darüber hinaus.

### N2 (substanziell, außerhalb der Bilder): Die Fallunterscheidung der Rückschritt-Taste ist auf der genannten Größe nicht entscheidbar

C6.10 legt fest: „Die Fallunterscheidung hängt **allein** daran, ob ein Filtertext steht." C1.18 verlangt: „Ein gehaltener Rückschritt, der den Filtertext leert, räumt nicht weiter."

Beides zusammen geht nicht. Wer drei Zeichen getippt hat und die Taste hält, hat nach dem dritten Anschlag keinen Filtertext mehr. Der vierte Anschlag trifft auf den Zustand „kein Filtertext steht", und C1.16 sagt für diesen Zustand: die Taste räumt in den Papierkorb. C1.18 sagt für denselben Anschlag: sie räumt nicht. Die Regel braucht eine zweite Größe, nämlich ob das Ereignis aus einer Wiederholung stammt, die bei stehendem Filtertext begann.

Der Entscheidungsdatensatz nennt diese Größe und hält sie für verfügbar: „AppKit meldet an jedem Tastenereignis, ob es aus einer Wiederholung stammt" (`decisions/260814-1852_a_raeumt-ein-gehaltener-rueckschritt-weiter-wenn-der-filtertext-leer-wird.md`). Die Frage ist also entscheidbar, und der Mechanismus muss nicht wechseln. Falsch ist allein das Wort „allein" in C6.10. Der Fallunterscheidung fehlt eine Eingangsgröße, die sie im Text ausschließt.

Der Nutzer hat die Frage am 260814-1910 mit Möglichkeit 2 beantwortet, und der Datensatz trägt den Marker `_a_`. Der Spec führt sie an drei Stellen weiter als offen: in der Kopfzeile („Fünf sind offen"), in C1.18 und in der Tabelle `## Offene Nutzerentscheidungen`. Offen sind vier.

### N3 (Medium, fehlendes Bild): Die gefährlichste Regel der Runde hat keine formale Darstellung

Der Spec trägt zwei Mermaid-Blöcke, und beide zeigen, welche Zeile auf dem Schirm steht. Die Fallunterscheidung der Rückschritt-Taste hat keinen. Sie trägt elf Kriterien, sie hängt vom Zustand ab, sie ist in keiner Übersicht der Anwendung zu sehen (C1.19), und ihr falscher Zweig räumt Dateien ohne Rückfrage weg.

`rules/design-diagrams.md` nennt genau diese Gestalt als Anlass für ein Bild: eine Fallunterscheidung, deren Zweige verschiedene Wirkungen haben, und ein Zustand, aus dem heraus entschieden wird. Ein drittes Bild mit den Eingangsgrößen „steht ein Filtertext" und „stammt der Anschlag aus einer Wiederholung", den drei Ausgängen „Zeichen zurücknehmen", „nichts tun" und „in den Papierkorb", und den Kanten für `cmd+delete`, `opt+cmd+delete` und `f8` hätte den Widerspruch aus N2 beim Zeichnen sichtbar gemacht. Er ist es nicht, weil es das Bild nicht gibt.

Die Kriterien selbst sind bis auf N2 in Ordnung und einzeln nachvollzogen: C1.15 und C6.9 decken die gefährliche Richtung mit einer Probe und einer Bündelbeobachtung ab, C1.17 hält den Weg zum Papierkorb über `cmd+delete` offen, C6.11 nimmt `f8`, `opt+cmd+delete` und `ctrl+delete` ausdrücklich aus. Kein Bild widerspricht ihnen, weil keines sie berührt.

### N4 (gering, Prosa gegen Graph): Die Zahl sieben stimmt, ihre Herleitung nicht

Die Prosa erklärt die sieben Kreise so: „die Rückkehr … verbindet sich mit jeder der beiden inneren Wiederholungen zu je einem weiteren Kreis. Die drei sind die Mechanismen, die sieben sind ihre Verbindungen." Drei plus zwei ergibt fünf.

Gemessen sind sieben, und die Rückkehr über `FERTIG → ZURUECK → NOCH` bildet vier davon, nicht zwei. Sie unterscheiden sich darin, auf welchem Weg `FERTIG` erreicht wird: über einen leeren Stapel (Kreis 3), über eine Verknüpfung (Kreis 4), über einen nicht zu öffnenden Ordner (Kreis 6) und über einen abgearbeiteten Abstieg (Kreis 7). Die Zahl ist richtig, der Satz daneben zählt anders. Er ist der einzige Satz des Abschnitts, der sich am Graphen nicht bestätigt.

### N5 (kosmetisch, Bild 2 gegen den Wortlaut von C3.9): Der Knoten `in ihn absteigen` steht vor dem Wächter

Der Pfad für eine symbolische Verknüpfung, die tiefer im Baum gefunden wird, läuft `IST|ja → ABST → VERKN|ja → FERTIG`. Der Knoten `ABST` trägt die Aufschrift „in ihn absteigen", und C3.9 sagt: „Der Durchlauf steigt nicht in symbolische Verknüpfungen hinab."

Am Mechanismus ändert das nichts: geöffnet wird erst am Knoten `OEFFNEN`, und den erreicht eine Verknüpfung nie. Der Wortlaut ist die Falle. Wer das Bild als Ablauf liest, schreibt eine Funktion, die den Eintrag zum aktuellen Ordner macht und ihn dabei öffnet, und folgt der Verknüpfung. Eine Aufschrift wie „diesen Eintrag als nächsten Ordner nehmen" träfe, was der Knoten tut, und stünde nicht gegen C3.9.

### N6 (kosmetisch, Bild 2): Zwei Knoten brechen die `ja`/`nein`-Konvention

`ZEIGEN` und `WEGLASSEN` haben je zwei ausgehende Kanten, eine unbeschriftete nach `LISTE` und eine mit „kein Ordner mehr offen" nach `ENDE`. An jedem anderen Verzweigungsknoten beider Bilder sind beide Kanten beschriftet und schließen einander aus. Hier ist die eine Kante unbedingt und die andere bedingt. Wer die Konvention des Bildes anlegt, liest eine Fallunterscheidung, wo eine Folge gemeint ist.

### Was trägt, und zwar ohne Einschränkung

Nachgerechnet und in Ordnung. Der Abschnitt steht hier, damit `tangled` nicht als Urteil über den Umbau insgesamt gelesen wird.

- **Beide Blöcke parsen.** `@mermaid-js/mermaid-cli` 11 rendert beide, Exit 0.
- **Kein Kreis ohne Ausgang.** Sieben Kreise, der schwächste mit drei Ausgangskanten. Der tragende Fehler der ersten Prüfung ist behoben und nicht durch einen gleichartigen ersetzt.
- **Kein Gott-Knoten.** Größter Ausgangsgrad 3 an `STAPEL`, und die drei Kanten sind sachlich verschieden: Stapel trägt Einträge, Stapel ist leer, Abbruch. Größter Eingangsgrad 3 an vier Knoten, alle mit dem Charakter einer Sammelstelle.
- **Die Dichte ist unauffällig.** 1,53 Kanten je Knoten bei siebzehn Knoten. Für einen Durchlauf mit drei Wiederholungsmechanismen ist das wenig, nicht viel.
- **Die Schichtung ist sichtbar und trägt eine Aussage.** Der Untergraph `DURCHLAUF` mit `direction TB` trennt den Arbeitsfaden vom Hauptfaden, und die beiden Kanten, die ihn nach außen verlassen, sind mit „Befund an den Hauptfaden" beschriftet. Die Fadengrenze ist damit gezeichnet, ohne dass die Fadenzahl behauptet würde.
- **Die Abbruchzusage aus C3.4 trägt jetzt für jede Baumgestalt.** Der Prüfordner ohne Unterordner aus dem Kriterium läuft `STAPEL → NOCH → PRUEF → IST|nein → NOCH` und erreicht `STAPEL` nach spätestens 1.024 Einträgen. Auch die drei Kreise, die `STAPEL` umgehen, sind durch die Stapelgröße beschränkt.
- **Der negative Befund erreicht `U|nein` auf jedem Pfad, für den er gedacht ist.** In allen drei Klassen, in denen der Name nicht passt und nichts darunter liegt, endet der Durchlauf auf `kein Treffer darunter`, die symbolische Verknüpfung eingeschlossen. Die Frage der Vorbewertung ist damit beantwortet; was übrig bleibt, ist die entgegengesetzte Richtung aus N1.
- **Bild 1 ist ein sauberer Entscheidungsbaum.** Neun Knoten, dreizehn Kanten, kein Kreis, jeder Entscheidungsknoten binär und mit `ja`/`nein` beschriftet, damit je Knoten überschneidungsfrei und vollständig. Der Eingangsgrad 4 an `JA` ist die normale Gestalt eines Baums mit zwei Senken.
- **Der Typ passt in beiden Fällen.** `flowchart TD` für einen Entscheidungsbaum und für einen Durchlauf mit Wiederholung, wie es die Typtabelle in `rules/design-diagrams.md` vorsieht.

## What a clean redraw would require

Der Umbau kostet zwei Knotenaufschriften, einen Satz und ein drittes Bild. Neue Struktur entsteht nicht.

1. **Die beiden Ausgangsknoten des zweiten Bildes sprechen den Befund aus, nicht die Sichtbarkeit.** Aus `der Ordner erscheint` wird der Befund, der `U` mit ja beantwortet; aus `der Ordner erscheint nicht` der Befund, der `U` mit nein beantwortet. Damit hört das zweite Bild dort auf, wo seine Zuständigkeit endet, und die Sichtbarkeit bleibt eine Rechnung an einer Stelle.
2. **C3.13 verliert seinen letzten Halbsatz.** „Der Ordner erscheint danach auf keinem Weg mehr" ist für einen Ordner falsch, dessen eigener Name passt. Richtig ist: der Befund beendet den Durchlauf für diesen Ordner und beantwortet `U`; über die Sichtbarkeit entscheidet Bild 1 aus Name und Befund zusammen. C2.8 und C2.13 stehen dann unangetastet.
3. **Ein drittes Bild für die Rückschritt-Taste.** Zwei Eingangsgrößen, drei Ausgänge, dazu die Kanten für `cmd+delete`, `opt+cmd+delete`, `f8` und `ctrl+delete`. Es ist das Bild, das C6.10 gegen C1.18 hält, und es ist bei elf Kriterien und einer Wirkung ohne Rückfrage die billigste Prüfung, die zu haben ist.
4. **C6.10 nennt beide Eingangsgrößen.** Das Wort „allein" fällt, und an seine Stelle tritt die zweite Größe: ob der Anschlag aus einer Wiederholung stammt, die bei stehendem Filtertext begann. Die drei im Kriterium ausgeschlossenen Größen bleiben ausgeschlossen.
5. **Zwei Aufschriften.** `ABST` benennt, was der Knoten tut, ohne das Wort „absteigen", das C3.9 verbietet. Die unbeschrifteten Kanten von `ZEIGEN` und `WEGLASSEN` nach `LISTE` bekommen ihre Aufschrift oder die bedingten Kanten nach `ENDE` gehen an einen eigenen Knoten.
6. **Der Satz über die sieben Kreise nennt vier Rückkehrwege statt zwei.** Die Zahl stimmt; die Rechnung dahinter ist zu berichtigen, sonst steht wieder eine Prosaaussage neben dem Graphen, die er nicht stützt.

Der Nebenbefund aus N2 ist beim Nachziehen mitzunehmen: der Datensatz zur Tastenwiederholung ist seit 260814-1910 beantwortet, und der Spec führt ihn an drei Stellen als offen. Offen sind vier Fragen, nicht fünf.
