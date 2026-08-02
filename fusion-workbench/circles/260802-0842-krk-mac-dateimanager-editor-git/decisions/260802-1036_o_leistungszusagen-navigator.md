# Auf welchem Referenzgerät gelten die zehn Zeitzusagen aus C8?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C8, `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`

---

## Erledigter Teil: die Zahlen

Die ursprüngliche Frage lautete, welche Zahlen KRK zur Zusage macht, damit die Maxime "superschnell" prüfbar wird. Dieser Teil ist beantwortet. Der Nutzer hat am 260802-1105 Möglichkeit 1 gewählt und alle zehn Werte der Tabelle in Abschnitt C8 des Specs unverändert übernommen. Sie sind damit Abnahmekriterien der Runde 1, nachzulesen in `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C8.

Der Datensatz bleibt trotzdem offen, weil die zweite Hälfte der Frage unbeantwortet ist.

## Question

Die zehn Zusagen nennen Zeiten, aber kein Gerät. Dieselbe Zahl bedeutet auf zwei verschiedenen Macs zwei verschiedene Anforderungen: 400 ms für das vollständige Lesen eines Ordners mit 10.000 Einträgen ist auf einem aktuellen Apple-Silicon-Mac mit schneller interner SSD eine andere Aussage als auf einem älteren Intel-Mac. Ohne benanntes Gerät ist keine der zehn Zusagen nachprüfbar, und eine Messung, die niemand wiederholen kann, ist keine Abnahme.

Der Spec nennt bislang nur "der Entwicklungs-Mac des Nutzers mit interner SSD". Zu benennen sind:

- Modell und Baujahr, etwa "MacBook Pro 14 Zoll, 2023"
- Prozessor, etwa "Apple M2 Pro"
- Bildwiederholrate des eingebauten Bildschirms, 60 Hz oder 120 Hz
- Größe des Arbeitsspeichers

Die Bildwiederholrate ist gesondert aufgeführt, weil sie zwei Zusagen unmittelbar betrifft. L1 und L9 nennen 16 ms, den konservativen Wert eines Einzelbildes bei 60 Hz. Auf einem Gerät mit 120 Hz halbiert sich das Einzelbildbudget auf 8 ms. Die Zusage bleibt bei 16 ms, aber die Aussage "eine Reaktion je Bild" trifft dann nicht mehr zu; der Nutzer soll wissen, worauf er sich festgelegt hat.

Die Angabe muss vor der ersten Messung vorliegen. Weder der Technologievergleich durch den analyst noch der Plan sind davon blockiert, weil beide gegen die Zahlen arbeiten und nicht gegen das Gerät.

## Options

1. **Der Entwicklungs-Mac des Nutzers, mit vollständiger Angabe** — der Nutzer nennt Modell, Baujahr, Prozessor, Bildwiederholrate und Arbeitsspeicher seines Geräts.
   - Pro: das Gerät steht zur Verfügung, jede Messung ist sofort wiederholbar. Die Zusagen gelten dort, wo der Nutzer KRK benutzt.
   - Contra: eine Zusage, die an ein einzelnes Gerät gebunden ist, sagt nichts über andere Macs. Wechselt der Nutzer das Gerät, ist die Grundlage neu zu setzen.

2. **Ein benanntes Mindestgerät, unabhängig vom Entwicklungs-Mac** — die Zusagen gelten auf einem festgelegten schwächsten unterstützten Mac, etwa dem ältesten Modell, das das Zielbetriebssystem noch trägt.
   - Pro: die Zusage gilt für jeden Nutzer, nicht nur für einen. Das ist die Form, in der eine Anwendung Leistungsversprechen üblicherweise trägt.
   - Contra: das Gerät muss zum Messen verfügbar sein. Ist es das nicht, bleibt die Zusage ungeprüft und damit wertlos.

3. **Zwei Geräte: der Entwicklungs-Mac als Messgerät, ein Mindestgerät als Zusage** — gemessen wird laufend auf dem Entwicklungs-Mac, abgenommen wird zusätzlich einmalig auf dem Mindestgerät.
   - Pro: die laufende Messung bleibt billig, die Zusage bleibt allgemein.
   - Contra: zwei Messreihen mit zwei Zahlenreihen. Der Aufwand verdoppelt sich, und es braucht eine Regel, welche der beiden im Konfliktfall gilt.

## Constraints

- Die zehn Zahlen selbst stehen fest und sind nicht Gegenstand dieser Frage.
- Die Messbedingungen aus C8 gelten unverändert: definierter Prüfordner mit 10.000 Einträgen, zwanzig Wiederholungen, 95. Perzentil statt Mittelwert, getrennte Werte für kalten und warmen Dateisystem-Cache.
- Jede Zusage muss von jemandem prüfbar sein, der den Quellcode nicht kennt. Ein Gerät, auf das nur der Nutzer Zugriff hat, erfüllt das nur für ihn.
- Die Angabe muss vor der ersten Messung vorliegen, nicht vor dem Plan.

## Recommendation

Möglichkeit 1, mit einer Ergänzung. Der Entwicklungs-Mac ist verfügbar, und eine Zusage, die niemand misst, hilft nicht. Der Spec sollte die vollständige Gerätebeschreibung tragen, damit später nachvollziehbar bleibt, worauf sich die Zahlen bezogen. Sobald KRK auf mehr als einem Gerät läuft, ist ein Mindestgerät nachzuziehen; das ist dann ein eigener Entscheidungsdatensatz und keine stille Erweiterung dieses hier. Die Abwägung ist eine Empfehlung, keine geprüfte Aussage.

---
Answered:
Implemented:
Deferred:
Superseded by:
