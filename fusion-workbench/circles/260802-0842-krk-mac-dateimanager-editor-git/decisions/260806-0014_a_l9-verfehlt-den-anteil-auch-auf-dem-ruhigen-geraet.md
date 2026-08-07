# L9 verfehlt den 95-Prozent-Anteil auch auf dem ruhigen Gerät — Ursache beheben oder Zusage ändern?

---
**Domain:** code
**Status:** answered
**Filed by:** coder
**Cross-references:** issues/260805-2335_o_l1-und-l9-verfehlen-den-anteil-im-ersten-gesamtlauf-unter-fremdlast.md, messungen/260805-2207-MacBookPro15-1-abnahme.txt, messungen/260805-2207-MacBookPro15-1-abnahme-begleittext.md, planning/260802-1428_o_plan-navigator-geruest-runde-1.md `#### 22.`

---

## Frage

L9 sagt zu: während einer laufenden Kopie erreichen mindestens 95 Prozent der
Tastendrücke ihr nächstes Bild (Spanne bis zum Ende des Zeichendurchgangs
höchstens eine Bildlänge, 16,667 ms bei 60 Hz). Die Abnahme-Messreihe aus S22
auf dem ruhigen Referenzgerät hält den Anteil nur in einer von fünf Runden:
90, 85, 90, 100 und 85 Prozent. Fremdlast scheidet als Erklärung aus, denn
der Lauf war ruhig, und L1, dasselbe Maß ohne laufende Kopie, hält in allen
fünf Runden. C8 verlangt bei einer verfehlten Zusage diesen Datensatz statt
einer stillschweigenden Lockerung. Wie geht KRK mit der Verfehlung um?

## Befund, an dem die Optionen hängen

Die Verfehlung ist klein und scharf begrenzt. Jede verpasste Eingabe liegt
zwischen 17,2 und 23,4 ms, also im **zweiten** Bild, keine einzige darüber;
je Runde verpassen zwei bis drei von zwanzig Eingaben. Das Muster (ein
gelegentlicher Rutscher um genau ein Bild, nur während der Kopie) spricht
für eine konkurrierende Arbeit im Hauptfaden je Fortschrittsmeldung der
laufenden Kopie, nicht für eine strukturell zu langsame Eingabeverarbeitung.

## Optionen

1. **Ursache im Hauptfaden untersuchen und beheben, Zusage unverändert.**
   Verdächtig ist die Verarbeitung der Kopier-Fortschrittsmeldungen
   (Auffrischung der Vorgangsanzeige) zwischen Tastenereignis und
   Zeichendurchgang. Folgen: ein Untersuchungs-Defekt mit Nachmessung; kein
   Eingriff in Spec oder Maß; Runde 1 schließt erst nach Fix und bestandener
   Nachmessung, S23 (Auslieferungspaket) rückt entsprechend nach hinten.
   Kein Eingriff auf Verdacht: erst messen, wo die verlorene Bildlänge
   entsteht, dann ändern.
2. **Zusage ehrlich anpassen.** Die gemessene Wirklichkeit trüge die Form
   "während einer Kopie erreicht jede Eingabe spätestens das zweite Bild,
   mindestens 85 Prozent das erste" (gemessen: alle Verfehlungen unter
   zwei Bildlängen). Folgen: eine Änderung an C8, die nur du beschließen
   kannst; die Kopplung an das L1-Maß (dasselbe Maß seit 260803-1810) wird
   gelöst; die Maxime superschnell verliert im Kopierfall eine zugesicherte
   Bildlänge, dauerhaft und sichtbar im Spec. Ein bloßes Absenken auf
   90 Prozent genügte nicht, denn zwei Runden liegen bei 85 Prozent.
3. **Erst die Messvorschrift prüfen, dann entscheiden.** Der Auslösetakt
   der Messung (97 ms) und der Takt der Fortschrittsmeldungen könnten sich
   überlagern und die Trefferquote systematisch drücken. Folgen: klärt nur
   die Deutung, nicht die Sache; die gemessenen Spannen sind echte Wege vom
   Ereignis bis zum Zeichendurchgang, ein Nutzer erlebte sie genauso. Als
   alleinige Antwort stellt diese Option die Zusage nicht wieder her.

## Empfehlung

Option 1. Die Verfehlung ist ein begrenzter Ein-Bild-Rutscher mit klarer
Verdachtsstelle, kein strukturelles Zu-langsam; die Aussicht, die Zusage
ohne Lockerung zu halten, ist gut. Option 3 lohnt als erster Schritt der
Untersuchung innerhalb von Option 1, nicht als eigener Weg.

---
Answered: planning/260802-1036_*_spec-navigator-geruest.md:360 — Der Nutzer hat
am 260807 Möglichkeit 2 gewählt, gegen die Empfehlung dieses Datensatzes: L9
sagt jetzt zu, dass während einer laufenden Kopie jede Eingabe spätestens das
zweite Bild erreicht und mindestens 85 Prozent das erste. Die Zeile L9 der
Zusagentabelle steht in C8 des Specs, die Herleitung und der Preis der Wahl im
selben Abschnitt unter `Getroffene Festlegungen` (:394), die Messvorschrift im
Absatz `Die Vorschrift, prüfbar formuliert`. Im Plan
`planning/260802-1428_*_plan-navigator-geruest-runde-1.md` sind `### Frage 5`,
`### Frage 6`, S21 und S22 nachgezogen; beide Schritte bleiben abgenommen. Beide
Hälften der neuen Fassung sind an der Abnahmereihe nachgerechnet: kleinster
Rundenanteil 85 Prozent, größter Einzelwert 23,429 ms bei zwei Bildlängen von
33,333 ms. Die Auswertung in `crates/krk-bench/src/messen.rs` trägt die neue
Form noch nicht, gemeldet als
`issues/260807-0832_*_die-messstrecke-kann-die-neue-zweiteilige-fassung-von-l9-nicht-abnehmen.md`.
