# Auf welchem Referenzgerät gelten die zehn Zeitzusagen aus C8?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C8, `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`

---

## Antwort: das Referenzgerät

Der Nutzer hat am 260802-1127 Möglichkeit 1 gewählt und das Gerät benannt, auf dem diese Sitzung läuft. Damit ist der Datensatz vollständig beantwortet.

| Angabe | Wert |
|---|---|
| Modell | MacBook Pro 15 Zoll, 2018 (Modellkennung `MacBookPro15,1`) |
| Prozessor | 8-Core Intel Core i9, 2,3 GHz, Hyper-Threading aktiv |
| Arbeitsspeicher | 16 GB |
| Grafik | Intel UHD Graphics 630 und Radeon Pro 560X |
| Bildschirm | 2880×1800 Retina, 60 Hz |
| Betriebssystem zum Zeitpunkt der Festlegung | macOS 15.7.7 |

Die Angaben stammen aus `system_profiler` auf ebendiesem Gerät und sind insoweit geprüft.

Die Wahl ist die strengere von zweien, und zwar bewusst. Der Nutzer gibt an, sein eigentlicher Arbeitscomputer sei ein "M2 Pro Max", also ein Apple-Silicon-Mac; die Bezeichnung ist mehrdeutig und vermutlich als M2 Max oder M2 Pro zu lesen. Diese Angabe ist eine Aussage des Nutzers und keine von uns geprüfte Tatsache. Für die Zusagen ist die Mehrdeutigkeit ohne Belang: gemessen und abgenommen wird auf dem Intel-Gerät von 2018. Was dort die zehn Zahlen hält, hält sie auf dem neueren Apple-Silicon-Mac erst recht, weil dieser in Einzelkernleistung, Speicheranbindung und Datenträgerdurchsatz durchweg darüber liegt.

Die Bildwiederholrate des benannten Geräts beträgt 60 Hz. Damit trifft die Herleitung von L1 und L9 unverändert zu: 16 ms sind auf diesem Bildschirm genau ein Einzelbild, und die in Abschnitt C8 des Specs vermerkte 120-Hz-Einschränkung greift hier nicht.

Sobald KRK auf mehr als diesem einen Gerät abgenommen wird, ist ein Mindestgerät nachzuziehen. Das ist dann ein eigener Entscheidungsdatensatz und keine stille Erweiterung dieses hier.

## Erledigter Teil: die Zahlen

Die ursprüngliche Frage lautete, welche Zahlen KRK zur Zusage macht, damit die Maxime "superschnell" prüfbar wird. Dieser Teil ist beantwortet. Der Nutzer hat am 260802-1105 Möglichkeit 1 gewählt und alle zehn Werte der Tabelle in Abschnitt C8 des Specs unverändert übernommen. Sie sind damit Abnahmekriterien der Runde 1, nachzulesen in `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C8.

Die zweite Hälfte der Frage, das Gerät, ist seit dem 260802-1127 ebenfalls beantwortet, siehe den Abschnitt darüber.

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
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C8, Unterabschnitt "Messbedingungen" — Referenzgerät ist das MacBook Pro 15 Zoll von 2018 (`MacBookPro15,1`), 8-Core Intel Core i9 mit 2,3 GHz, 16 GB Arbeitsspeicher, 2880×1800 bei 60 Hz, macOS 15.7.7. Bewusst die strengere Wahl gegenüber dem Apple-Silicon-Arbeitsrechner des Nutzers. Die zehn Zahlen selbst waren bereits am 260802-1105 bestätigt.
Implemented:
Deferred:
Superseded by:
