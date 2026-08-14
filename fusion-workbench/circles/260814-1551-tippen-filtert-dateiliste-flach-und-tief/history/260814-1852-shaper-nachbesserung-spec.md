# Shaper — Nachbesserung am Spec der Filterrunde

**Date:** 2026-08-14 18:52
**Modus:** in-Circle, Nachbesserung
**Circle:** `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/`
**Bearbeitet:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`

## Auftrag

Zwei Nachbesserungen am Spec, der am Freigabe-Gate steht und nicht abgelehnt ist. Erstens die Befunde der Bewertung `reviews/260814-1840-conceptrev-tippen-filtert-dateiliste-flach-und-tief.md` (Urteil `tangled`, fünf Befunde am zweiten Bild). Zweitens die Antwort des Nutzers vom 260814-1845 auf `decisions/260814-1830_a_wie-nimmt-der-nutzer-ein-einzelnes-zeichen-des-filters-zurueck.md`, die die Empfehlung des Datensatzes verwirft.

## Nachbesserung 1: das zweite Bild

Neu gezeichnet. Siebzehn Knoten, sechsundzwanzig Kanten, größter Ausgangsgrad 3, keine Waise, jeder Knoten erreicht einen Endpunkt; nachgerechnet, und beide Blöcke rendern mit `mmdc` fehlerfrei.

- **B1 (der fehlende negative Befund).** Der Durchlauf trägt jetzt den Endknoten `kein Treffer darunter`, gespeist aus drei Quellen: der Ordner ist abgearbeitet, er ließ sich nicht öffnen, er ist eine symbolische Verknüpfung. Der Kreis über die Einträge eines Ordners hat damit einen unbedingten Ausgang.
- **B2 (die Abbruchkante).** `ABST` ist in `in ihn absteigen` und `naechsten Stapel holen` geteilt; die Abbruchkante hängt am Stapelknoten. Ein Ordner mit fünfzigtausend gewöhnlichen Einträgen und ohne Unterordner passiert ihn neunundvierzigmal.
- **B3 (die Fadenfrage).** Entschieden zugunsten der offenen Frage: das Bild sagt nichts mehr über die Zahl der Fäden, C3.1 ist entsprechend umformuliert, und `## Offen für den Planner` führt die Frage weiter und sagt jetzt ausdrücklich, dass weder Bild noch Kriterium sie vorwegnehmen.
- **B5 (zwei Schnitte für dieselbe Frage).** Ein Schnitt: beide Bilder fragen `ist es ein Ordner?`. Die Verknüpfungsregel wohnt allein im Durchlauf, der für eine Verknüpfung `kein Treffer darunter` meldet. Damit beantwortet der Mechanismus den Knoten `U` des ersten Bildes auch für sie, `C1.6` deckt sie bei ausgeschaltetem „Deep" weiter, und das erste Bild bleibt unverändert.
- **B4 (die Kreiszählung).** Die Prosa nennt drei Wiederholungen als Mechanismen und sagt, dass ein Zähler einfacher Kreise sieben findet, weil die Rückkehr aus dem Abstieg sich mit beiden inneren Wiederholungen verbindet. Gemessen: sieben.
- **C3.10 (Fehlschlag beim Öffnen)** hat jetzt einen eigenen Zweig und mündet auf denselben Endknoten.

Neue Kriterien daraus: C2.13 (Verknüpfung auf einen Ordner), C3.13 (der Durchlauf entscheidet jeden Ordner mit einem von zwei Befunden). Umformuliert: C3.1, C3.4, C3.9.

## Nachbesserung 2: die Rückschritt-Taste

Die Regel steht jetzt im Spec, und zwar mit der Begründung des Nutzers: sicherheitsrelevant, nicht bequem. Berührt sind vier Stellen.

- Die dritte Verlustzeile in `## Was diese Runde am Spec der Runde 1 ändert` sagte, einen Weg zurück gebe es nicht. Sie sagt jetzt, die Nachsicht gegenüber einem Vertipper wechsele den Weg, und der Absatz darunter trägt die Regel samt Begründung.
- Eine zehnte Feststellung in der Ausgangslage: das Räumen läuft ohne Rückfrage (`anwendung.rs:4274-4276`), allein das endgültige Löschen zeigt eine. Nebenbefund: der Abschnitt kündigte acht Feststellungen an und trug schon vorher neun.
- C1 bekommt sechs Kriterien (C1.14 bis C1.19), darunter C1.15 als eigenes Kriterium für die gefährliche Richtung: bei stehendem Filtertext erreicht die nackte Rückschritt-Taste `in_papierkorb` nicht.
- C6 bekommt drei Kriterien (C6.9 bis C6.11): kein Auftrag `InDenPapierkorb`, die Fallunterscheidung hängt allein am stehenden Filtertext, die übrigen Löschwege bleiben unberührt.

## Ein neuer offener Datensatz

`decisions/260814-1852_o_raeumt-ein-gehaltener-rueckschritt-weiter-wenn-der-filtertext-leer-wird.md`. Die Antwort des Nutzers regelt den Fall nicht, in dem der Nutzer die Taste hält: nach dem letzten Zeichen trifft die Wiederholung auf die alte Bedeutung und räumt ohne Rückfrage. Drei Möglichkeiten, Empfehlung ist die zweite (die Wiederholung endet an der Grenze, ein neuer Druck räumt). Der Spec fährt auf der Empfehlung, Kriterium C1.18.

Der Status des beantworteten Datensatzes ist von `open` auf `answered` nachgezogen; der Dateiname trug den Marker `_a_` schon.

## Stand danach

- Abnahmekriterien: 73 (C1 19, C2 13, C3 13, C4 10, C5 7, C6 11); vorher 62.
- Beantwortete Fragen: sechs. Offene: fünf, davon eine neu.
- Beide Mermaid-Blöcke rendern; das erste Bild ist unverändert.
- Nicht angefasst: die drei übrigen offenen Fragen, die Filterzahl in der Statuszeile, C4, C5 bis auf die Umformulierung von C5.1.
