# Shaper: C4 des Spec der neunten Runde ist nachgezogen

**Datum:** 2026-08-14 09:22 bis 09:30
**Status:** Complete
**Modus:** user-direct, am laufenden Circle `260813-2332-notizzettel-als-blatt-mit-zwei-zetteln`
**Auftrag:** C4 im Spec `planning/260813-2348_o_spec-notizzettel-als-blatt-mit-zwei-zetteln.md` nach dem hohen Befund der Durchsicht von Turn 1 und der Nutzerantwort vom 260814-0925 nachziehen. Keine Klärungsfragen, kein Bau, kein Eingriff in den Plan.

---

## Was hereinkam

Die Durchsicht `reviews/260814-0908-coderev-turn-1-notizzettel.md` mit einem hohen Befund: C4 sagte zwei Dinge zu, die nicht zugleich hielten. „Eine gescheiterte Sicherung wirft den Stand nicht weg" und „Der Zettel liest seine Datei bei jedem Öffnen neu" gelten nur gemeinsam, wenn das Neulesen einen abweichenden Stand nicht antastet. `Zettelmodell::oeffnen` setzte den gehaltenen Text auf das Gelesene; damit war der getippte Text fort, ohne Meldung. Der Prüfer hat die Ursache im Spec verortet und die Entscheidung vor die Behebung gestellt.

Dazu die Nutzerantwort vom 260814-0925: **der getippte Stand gewinnt.** Weicht der gehaltene Text von der Datei ab, bleibt er stehen; neu gelesen wird nur, wo nichts abweicht. „Die Datei gewinnt immer" ist verworfen, „beim Öffnen nachfragen" ist in einem Blatt nicht baubar (ein Blatt über einem Blatt geht in AppKit nicht).

Und der Auftrag, zwei weitere Befunde gegen C4 zu prüfen: `issues/260814-0908_o_ein-neuoeffnen-nach-gescheiterter-sicherung-wirft-den-ungesicherten-zettelstand-weg.md` und `issues/260814-0909_o_je-sicherungsmoment-wird-hoechstens-ein-zettel-geschrieben-und-beim-beenden-gibt-es-kein-naechstes-mal.md`.

## Der Befund zur zweiten Frage

**C4 deckte den zweiten Datensatz nicht, und die neue Zusage macht die Lücke schlimmer.** Nirgends stand, wie viele Zettel ein Sicherungsmoment schreibt; `zu_sichern` liefert den ersten abweichenden, und der Bau hat damit keine Zusage gebrochen. Solange ein abweichender Stand beim Öffnen verschwand, konnten zwei Zettel kaum je zugleich abweichen. Nach der Nutzerantwort überdauert der abweichende Stand das Schließen des Blattes, und zwei abweichende Zettel sind der gewöhnliche Folgezustand einer gescheiterten Sicherung. Der vierte Moment ist der letzte, der läuft.

Die Zusage ist deshalb ergänzt: **jeder Sicherungsmoment schreibt jeden abweichenden Zettel.**

## Was geändert wurde

Ein Dokument, der Spec. Der Abschnitt „Was der Nachtrag vom 260814-0925 an C4 geändert hat" am Ende zählt die Stellen auf. Der Kern:

- Überschrift und Beschreibung von C4 tragen die Zusage, dass nichts Getipptes still verloren geht.
- Fünf neue Kriterien, zwei umformulierte: der Vorrang des gehaltenen Standes beim Öffnen und beim Tabwechsel, jeder abweichende Zettel je Moment, die Meldung gebunden an die drei Momente mit Statuszeile.
- Sechs Festlegungen anstelle der einen zum Neulesen, darunter die zwei verworfenen Möglichkeiten mit ihren Gründen und die drei benannten Preise.
- Das Bild der Sicherungsmomente: sechs neu beschriftete Kanten, zwei Absätze zur Begründung. Am 260814-0927 mit `@mermaid-js/mermaid-cli` 11.16.0 gerendert, die Beschriftungen im SVG nachgezählt.
- Eine Zeile mehr unter „Ausdrücklich außerhalb dieser Runde": die Meldung beim Beenden und ein Beenden, das sich verweigert.
- Eine Stelle außerhalb von C4: das C5-Kriterium „Eine von außen geänderte Zetteldatei zeigt sich beim nächsten Öffnen mit ihrem neuen Inhalt" trägt jetzt dieselbe Einschränkung. Ohne sie stünde der Widerspruch nach dem Nachtrag in C5 statt in C4.

## Was nicht angefasst wurde

Die neun übrigen beantworteten Fragen der Runde, die Zulässigkeitsregel der achten Runde, der Plan, die Directive im Circle-Datensatz, die zwei Defektdatensätze. Kein Bau, kein `make check`.

**Kein neuer Entscheidungsdatensatz.** Die Antwort ist gefallen und nicht zurückgestellt, und sie bindet dieselbe Stelle, an der sie jetzt steht. Das folgt der Handhabung der drei Antworten vom 260814-0005: der Datensatz gehörte dort der Frage, die der Shaper vorher offen gefiltert hatte, die zwei anderen stehen als Festlegung im Spec.

## Was offen bleibt

- **Beim Beenden erfährt der Nutzer von einer gescheiterten Sicherung nichts.** Der Preis ist im Spec benannt, die Auflösung steht außerhalb der Runde. Die Vorlage gäbe es: `beenden_erlauben` hält KRK für den ungesicherten Editor an.
- Die zwei Defektdatensätze bleiben `_o_`. Sie werden in Turn 2 gemeinsam behoben; der Spec deckt jetzt beide.
