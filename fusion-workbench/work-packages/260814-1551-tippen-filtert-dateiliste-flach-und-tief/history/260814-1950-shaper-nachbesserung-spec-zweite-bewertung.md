# Shaper — zweite Nachbesserung am Spec der Filterrunde

**Date:** 2026-08-14 19:50
**Modus:** in-Circle, Nachbesserung
**Circle:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`
**Bearbeitet:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`
**Anlass:** `reviews/260814-1938-conceptrev-tippen-filtert-dateiliste-flach-und-tief.md` (Urteil `tangled`, ein substanzieller Befund, ein Befund außerhalb der Bilder, ein fehlendes Bild, drei Kleinere) und die Antwort des Nutzers vom 260814-1910 zur Tastenwiederholung

## Auftrag

Drei Befunde und drei Nachträge. Der Spec geht danach ohne weitere Diagrammprüfung an den Planner; die Maße sind deshalb selbst gerechnet und die neuen Kriterien Pfad für Pfad gegen die Bilder gelesen.

## Befund 1: wer die Sichtbarkeit ausspricht

**Gewählte Lesart: ein Ordner, dessen eigener Name den Filtertext trägt, erscheint, und was unter ihm liegt, ist für seine Sichtbarkeit gleichgültig.** Sie war nicht wirklich zu wählen — vier Stellen trugen sie bereits (Bild 1 über `N|ja → JA`, C2.5 mit seinem „oder", C2.8 mit der leeren Liste beim Einstieg, C2.13 für die symbolische Verknüpfung), und allein C3.13 samt den zwei Ausgangsknoten des zweiten Bildes sprach dagegen. Die Gegenlesart hätte drei Kriterien umgestoßen statt eines.

Die Behebung geht über den Vorschlag der Bewertung hinaus, und zwar an der Wurzel statt am Ausgang. Die Bewertung wollte zwei Knotenaufschriften und einen Halbsatz; das hätte den Widerspruch aus den Ausgängen genommen und die Eintrittskante `START -->|je Ordner des angezeigten Ordners| VERKN` stehen lassen, die den Durchlauf für jeden Ordner laufen lässt, auch für den längst entschiedenen. Jetzt trägt die Eintrittskante die Bedingung selbst: der Durchlauf läuft je Ordner, dessen Name den Filtertext **nicht** trägt. Damit ist der Widerspruch nicht behandelt, sondern unmöglich, und der Durchlauf liest nebenbei weniger.

Geändert:

- **C3.13** verliert „und der Ordner erscheint danach auf keinem Weg mehr" und sagt stattdessen, dass der Befund die Frage `liegt unter ihm ein Treffer?` mit nein beantwortet und über die Sichtbarkeit nicht entscheidet.
- **C3.14 neu:** für einen Ordner, dessen Name passt, läuft kein Durchlauf.
- **C3.2** nimmt die namentlich passenden Ordner zu den Dateien, die sofort stehen.
- **C3.3** gilt jetzt ausdrücklich für Ordner, deren Name nicht passt.
- **Beschreibung von C3** und **C2.13** mitgezogen.
- **Bild 2:** Eintrittskante bedingt, die Ausgänge `ZEIGEN`/`WEGLASSEN` sind zu `UJA`/`UNEIN` geworden und sprechen einen Befund an Bild 1 aus. Dazu die zwei kosmetischen Befunde der Bewertung: `ABST` heißt jetzt „diesen Eintrag als nächsten Ordner nehmen" statt „in ihn absteigen" (N5, der Wortlaut stand gegen C3.9), und die gemischt bedingten Kanten nach `LISTE`/`ENDE` laufen über einen Rechenknoten und einen Entscheidungsknoten `ist noch ein Ordner unentschieden?` (N6).

**Maße Bild 2 nach dem Umbau:** 19 Knoten, 27 Kanten, Ratio 1,42; eine Quelle, zwei Senken; größter Ausgangsgrad 3 an `STAPEL`, größter Eingangsgrad 3 an `NOCH` und `FERTIG`; sieben Kreise, jeder mit Ausgang; keine Waise, jeder Knoten erreicht eine Senke. Mit einem Skript über den geparsten Graphen gerechnet, nicht abgelesen.

## Befund 2: C6.10 gegen C1.18

`allein` ist gefallen. C6.10 nennt jetzt beide Eingangsgrößen — ob ein Filtertext steht und ob der Anschlag aus einer Wiederholung stammt, die bei stehendem Filtertext begann — und behält die drei ausgeschlossenen Größen.

## Befund 3: das dritte Bild

Neuer Abschnitt `## Die Rückschritt-Taste und was sie erreicht`. Zehn Knoten, zehn Kanten, eine Quelle, fünf Senken, größter Ausgangsgrad 3 an `welche Taste?`, kein Kreis, keine Waise.

**Das Zeichnen hat gefunden, was der Prüfer vorhergesagt hat, und noch etwas dazu.** Die zweite Größe darf nicht „stammt der Anschlag aus einer Wiederholung" lauten. Ohne den Zusatz „die bei stehendem Filtertext begann" hörte auch ein gehaltener Rückschritt **ohne** jeden Filtertext nach dem ersten Anschlag auf zu räumen — und das änderte das heutige Verhalten, das C1.16 ausdrücklich unangetastet lässt. Am Baum nachgesehen: nichts liest heute `isARepeat`; die einzige Fundstelle schreibt es, und zwar als `false` in den synthetischen Ereignissen des Messmodus (`crates/krk-ui/src/appkit/ereignisse.rs:471-481`). Ein gehaltener Rückschritt räumt heute also wiederholt.

Daraus: **C1.20 neu** (eine Wiederholung ohne Filtertext räumt weiter), eine **elfte Feststellung** in der Ausgangslage, ein siebter Punkt unter `## Abgeleitet und nicht gefragt` — der Nutzer hat diese Rückfrage nicht vorgelegt bekommen, sie ist erschlossen und am Spec-Tor überstimmbar —, und ein neuer Punkt unter `## Offen für den Planner`: die Frage braucht ein Bit mehr, als das Tastenereignis mitbringt, und wo es gehalten wird, entscheidet der Planner.

**Nebenbefund für die Abnahme:** der Messmodus kann den Wiederholungszweig nicht fahren, weil seine Ereignisse sich nie als Wiederholung melden. C1.18 und C1.20 bleiben in ihrem Bündelanteil Nutzerarbeit.

## Die drei Nachträge

- **Kreisherleitung berichtigt.** Sieben stimmt, drei plus zwei nicht. Es sind zwei innere Wiederholungen ohne Abstieg, vier über die Rückkehr (die sich darin unterscheiden, auf welchem Weg der Ordner fertig wurde) und einer über den Abstieg ohne die Rückkehr.
- **Der Datensatz zur Tastenwiederholung ist beantwortet.** Alle drei Stellen nachgezogen: Kopfzeile (sieben beantwortet, vier offen), C1.18 (Verweis auf den `_a_`-Datensatz statt „hängt an"), Tabelle der offenen Fragen (die Zeile ist heraus, der Absatz sagt warum).
- **Zählungen am Baum nachgeprüft.** 83 Belegungseinträge, 77 `Kommando`-Varianten, fünf Ränge, acht Ankreuzfelder, `STAPELGROESSE = 1024` — alle fünf halten. HEAD steht auf `7149869`, die drei Commits seit `43dfe90` berühren `crates/` und `resources/` nicht; steht jetzt im Prüfvorbehalt. Berichtigt: die Runde 9 ist mit 21 ihrer **29 Bündelkriterien** ohne Beleg geschlossen, nicht mit „21 Kriterien"; beide Runden sind jetzt zitiert.
- **Zwei eigene Zahlen beim Schreiben abgefangen.** Der neue Abschnitt sagte zunächst „elf Abnahmekriterien" für die Rückschritt-Regel (die Zahl stammte aus der Bewertung und zählte C2.13 und C3.13 mit, die von Verknüpfungen handeln); es sind zehn, C1.14 bis C1.20 und C6.9 bis C6.11. Und die Formulierung „die vier Kriterien" über einer Tabelle mit mehr als vier Einträgen ist ersetzt.

## Stand der Abnahmekriterien

75 Kriterien: 65 allein **(Probe)**, zwei allein **(Bündel)**, acht beides. Zehn haben einen Bündelanteil und sind Nutzerarbeit. Aufteilung: C1 zwanzig, C2 dreizehn, C3 vierzehn, C4 zehn, C5 sieben, C6 elf. Die Zahl steht jetzt im Spec, mit dem Hinweis, dass sie am Dokument nachzuzählen ist.

## Prüfung

Alle drei Mermaid-Blöcke mit `@mermaid-js/mermaid-cli` 11.16.0 gerendert, Exit 0. Knoten, Kanten, Fan-in, Fan-out, Kreise, Waisen und Erreichbarkeit einer Senke mit einem eigenen Skript über den geparsten Graphen gerechnet. Die Nummerierung aller sechs Kriterienlisten ist lückenlos geprüft, ebenso die Zahlwörter in der Prosa gegen die Struktur.

## Nebenbefund außerhalb des Auftrags

Sechs der sieben mit `_a_` markierten Entscheidungsdatensätze dieses Circles trugen im Kopf weiter `**Status:** open`. Die Felder sind gesetzt; die Ursache liegt im Ablauf, mit dem ein Datensatz beantwortet wird, und ist als Defekt abgelegt: `shared/issues/260814-1955_o_sechs-beantwortete-entscheidungsdatensaetze-tragen-im-kopf-weiter-status-open.md`. Der Datensatz zur Tastenwiederholung trägt dazu zwei `Answered:`-Blöcke, den leeren aus der Vorlage und den ausgefüllten darunter; auch das steht im Defekt.
