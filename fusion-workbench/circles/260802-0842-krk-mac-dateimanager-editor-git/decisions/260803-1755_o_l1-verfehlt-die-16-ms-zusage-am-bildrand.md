# L1 verfehlt die 16-ms-Zusage am Bildrand: Zusage anpassen, Messvorschrift anpassen oder Technologie wechseln?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C8 (Zeile L1 und der Absatz zur Bildwiederholrate), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` Schritt 8 und `### Frage 5` (Absatz "Zur Ehrlichkeit der L1-Messung"), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_leistungszusagen-navigator.md`, Messbericht `messungen/260803-1554-durchstich.txt`

---

## Question

Die Frühmessung aus Schritt 8 ist gefahren. Vier der fünf abgenommenen Zusagen halten ihre Zahl mit großem Abstand. **L1 hält sie nicht**, und der Grund liegt nicht bei KRK. Bevor die restlichen fünfzehn Schritte des Plans darauf aufbauen, muss entschieden sein, was mit L1 geschieht.

**Die Zahlen.** Gemessen am 260803 auf dem Referenzgerät `MacBookPro15,1` (macOS 15.7.7, 60 Hz aus `NSScreen.maximumFramesPerSecond`), warm, zwanzig Wiederholungen je Runde:

| Zusage | 95. Perzentil, bestes bis schlechtestes | Zusage laut C8 | Urteil |
|---|---|---|---|
| L1 Tastendruck bis Ende des Zeichendurchgangs | 13,678 bis 16,225 ms | 16 ms | **verfehlt in 1 von 5 Runden** |
| L2 erste Bildschirmseite auf Prüfordner A | 43,851 bis 45,071 ms | 100 ms | gehalten |
| L3 vollständiges Lesen auf A, warm | 143,600 bis 160,411 ms | 400 ms | gehalten |
| L4 Prozessstart bis bedienbares Fenster | 294,555 bis 303,540 ms | 1000 ms | gehalten |
| L10 erste Bildschirmseite bei 100.000 Einträgen | 51,445 bis 53,052 ms | 100 ms | gehalten |

**Wie weit verfehlt.** Um 0,225 ms, also um 1,4 Prozent. Über achtzehn Runden, die am 260803 insgesamt gefahren wurden, lag das 95. Perzentil von L1 zwischen 13,678 und 16,617 ms; **acht dieser achtzehn Runden verfehlten die 16 ms**. Das Urteil wechselt also von Runde zu Runde, und ein Urteil, das wechselt, ist keines. Die vier übrigen Zusagen wechseln nicht: sie halten in jeder Runde mit dem Faktor zwei bis drei Abstand.

## Woran es liegt

**Nicht an der Geschwindigkeit von KRK, und auch nicht an Rust oder `objc2`.** Drei Befunde stützen das, alle aus derselben Messung.

**Erstens: kein einziger Tastendruck hat sein Bild verpasst.** In den beiden vollständigen Gate-Läufen zu je hundert Tastendrücken lag **kein** Einzelwert über 16,667 ms, also über einem Bild bei 60 Hz. Über alle 320 protokollierten Einzelwerte des Tages waren es zwei (0,6 Prozent). Die Auswahl springt praktisch immer im nächsten Bild um — genau das, was L1 der Sache nach zusagt.

**Zweitens: der Anteil, den KRK selbst beisteuert, liegt bei 3 bis 8 ms.** Der kleinste gemessene Einzelwert eines Laufs ist der, dessen Bildgrenze am dichtesten hinter der fertigen Arbeit lag; er kommt der reinen Verarbeitungszeit am nächsten. Er liegt bei 3,035 ms, der Median über alle Einzelwerte bei 8,007 ms. Der Rest der gemessenen Spanne ist Warten auf die nächste Bildgrenze.

**Drittens: die Messvorschrift kann die Zusage rechnerisch kaum halten.** L1 endet laut Plan am Ende des Zeichendurchgangs, festgestellt über einen `CADisplayLink`. Der taktet einmal je Bildwiederholung, bei 60 Hz also alle 16,667 ms. Trifft ein Tastendruck das Bild an einer zufälligen Stelle, ist die Wartezeit bis zur nächsten Bildgrenze über [0; 16,667] verteilt, und das 95. Perzentil einer solchen Verteilung liegt bei 15,83 ms — **für eine Anwendung, die überhaupt keine Zeit verbraucht.** Bei zwanzig Stichproben ist das 95. Perzentil der neunzehnte Wert der sortierten Reihe, dessen Erwartungswert bei 19/21 × 16,667 = 15,08 ms liegt und dessen Streuung mehrere Millisekunden beträgt. Die Zusage von 16 ms liegt damit innerhalb der Streuung des Messverfahrens.

**Woher die 16 kommen.** C8 leitet L1 her als "ein Bild bei 60 Hz Bildwiederholrate". Ein Bild bei 60 Hz sind 16,667 ms; 16 ist die gerundete Zahl. Der Unterschied von 0,667 ms war beim Aufstellen der Zusage folgenlos und ist es jetzt nicht mehr, weil er größer ist als der gemessene Verfehlungsbetrag von 0,225 ms. **Hielte die Zusage bei einem Bild statt bei 16 ms, hätte L1 in allen achtzehn Runden gehalten** — der schlechteste gemessene Wert war 16,617 ms.

`inference:` Dieselbe Rechnung trifft L9 ("keine Eingabe wartet länger als 16 ms während einer Stapeloperation"), das dieselbe Herleitung trägt und in Schritt 21 gemessen wird. Gemessen ist L9 noch nicht.

## Options

1. **Die Zusage auf ein Bild des Bildschirms stellen, statt auf gerundete 16 ms.** L1 und L9 lauten dann: "innerhalb eines Bildes der Bildwiederholrate des Bildschirms, auf dem das Fenster steht", am Referenzgerät also 16,667 ms.
   - Pro: Die Zusage sagt danach genau das, was ihre Herleitung schon sagt, und was die Messung tatsächlich prüfen kann. Kein Codeaufwand. Die Rate steht seit dieser Messung ohnehin im Bedingungskopf jedes Berichts. Alle achtzehn gefahrenen Runden hielten diese Zahl.
   - Contra: Die Zusage hängt danach am Gerät statt an einer festen Zahl; auf einem 120-Hz-Bildschirm wäre sie mit 8,3 ms doppelt so streng. C8 hält heute ausdrücklich fest, dass L1 auf jedem Mac bei 16 ms bleiben soll, gerade damit die Zahl nicht mit dem Bildschirm wandert. Diese Festlegung wäre zurückzunehmen oder auf "höchstens ein Bild, mindestens aber 16,7 ms" zu erweitern.
   - Kosten: eine Änderung an C8 (Zeile L1, Zeile L9, der Absatz über die 120-Hz-Geräte), eine an `### Frage 5` des Plans, eine Zeile in der Schwellentabelle von `crates/krk-bench/src/messen.rs`. Kein Eingriff in KRK selbst.

2. **Das Abnahmemaß für L1 und L9 ändern: nicht das 95. Perzentil der Spanne, sondern der Anteil der Tastendrücke, die ihr nächstes Bild erreichen.** Zusage etwa: mindestens 95 Prozent der Tastendrücke erscheinen im nächsten Bild.
   - Pro: Das ist die Aussage, die der Nutzer merkt, und sie ist unabhängig von der Bildwiederholrate und von der Phase, in der ein Druck das Bild trifft. Gemessen wurden 318 von 320 Tastendrücken, also 99,4 Prozent.
   - Contra: C8 nimmt alle zehn Zusagen einheitlich über das 95. Perzentil einer Zeitspanne ab. Für L1 und L9 käme ein zweites Abnahmemaß daneben, mit eigener Regel und eigener Auswertung. Die Maxime "supersimpel" wirkt hier als Ausschlussgrund gegen eine zweite Form.
   - Kosten: Änderung an C8 für zwei Zeilen, Änderung an der Auswertung in `crates/krk-bench/src/messen.rs`, die für diese beiden Zusagen dann etwas anderes rechnet als für die übrigen acht.

3. **Alles lassen und KRK schneller machen.** Die Zusage bleibt bei 16 ms, der `coder` senkt die Verarbeitungszeit.
   - Pro: keine Änderung am Spec.
   - Contra: Wirkungslos. Die gemessene Spanne wird vom Warten auf die Bildgrenze bestimmt und nicht von der Arbeit. Selbst eine Anwendung ohne jede Verarbeitungszeit erreichte ein 95. Perzentil von rund 15,8 ms, mit einer Streuung, die die 16 ms in etwa jeder zweiten Runde reißt. Die drei Millisekunden, die KRK heute braucht, wegzuoptimieren ändert am Urteil nichts.
   - Kosten: unbegrenzt, ohne Aussicht auf ein stabiles Bestehen.

4. **Den Technologieentscheid aufmachen: Rust mit `objc2` gegen Swift mit AppKit oder SwiftUI.** Der Plan sieht diesen Weg für eine verfehlte Zusage ausdrücklich vor.
   - Pro: Falls der Verdacht bestünde, dass die Bindung selbst zu teuer ist, wäre der Vergleich die einzige Antwort.
   - Contra: Der Verdacht wird von dieser Messung nicht getragen. Die Bildgrenze liegt unter jeder Technologie gleich; kein Werkzeugkasten zeichnet vor dem nächsten Bild. Die vier übrigen Zusagen hält KRK mit dem Faktor zwei bis drei Abstand, das vollständige Lesen von 100.000 Einträgen in 0,98 s gegen zugesagte 4 s. Ein Wechsel adressierte nicht die Ursache.
   - Kosten: Neuaufsetzen der bisher acht umgesetzten Schritte, ohne dass sich die gemessene Zahl ändern würde.

## Constraints

- Die zehn Zahlen aus C8 sind vom Nutzer am 260802-1105 bestätigt. Eine davon zu ändern ist seine Entscheidung, nicht die des `coder`; C8 verlangt dafür ausdrücklich einen neuen Entscheidungsdatensatz und keine stillschweigende Lockerung.
- Die Antwort muss L1 und L9 zusammen behandeln. Beide tragen dieselbe Herleitung ("ein Bild bei 60 Hz"), und zwei getrennte Antworten könnten sich widersprechen. L9 ist noch nicht gemessen; es wird in Schritt 21 auf dieselbe Weise gemessen und träfe auf dieselbe Grenze.
- Die Messung selbst bleibt, wie sie ist. Sie ist reproduzierbar (`cargo run -p krk-bench -- durchstich …`), ihr Bedingungskopf ist vollständig, und ihre Bildwiederholrate stammt aus `NSScreen` und nicht aus einer Annahme.
- Der Plan hält Schritt 8 offen, bis diese Frage beantwortet ist. Die Schritte 9 bis 23 hängen daran nur mittelbar; der `ontocoder`-Schritt 9 (Auslieferungsbelegung) berührt C8 nicht.

## Recommendation

**Möglichkeit 1, mit einer Ergänzung aus Möglichkeit 2.**

Möglichkeit 1 löst den gemessenen Widerspruch an genau der Stelle, an der er entstanden ist: die Zahl 16 ist eine gerundete Bildlänge, und die Rundung geht in die falsche Richtung. Sie zurückzunehmen ändert keine Absicht des Specs, sondern schreibt die vorhandene Herleitung genauer. Kein anderer Punkt von C8 ist berührt.

Die Ergänzung: der Messbericht sollte für L1 und L9 zusätzlich den Anteil der Tastendrücke ausweisen, die ihr nächstes Bild erreichen. Das ist die Zahl, die die Sache trägt, und sie kostet in der Auswertung zwei Zeilen. Als **zweites Abnahmekriterium** neben das 95. Perzentil zu stellen wäre die Sonderregel, die die Maxime ausschließt; als **ausgewiesene Kennzahl** ohne eigenes Urteil ist sie kein zweites Maß, sondern die Erklärung des ersten.

Gegen Möglichkeit 4 spricht diese Messung deutlich. Der Technologieentscheid Rust mit `objc2` ist von den Zahlen nicht in Frage gestellt: KRK liest 100.000 Einträge in 0,98 s statt zugesagter 4 s, zeigt die erste Bildschirmseite eines solchen Ordners nach 53 ms statt zugesagter 100 ms und steht 0,3 s nach dem Prozessstart bedienbar da statt nach zugesagter 1 s. Diese Einschätzung stützt sich auf die Messung dieses Tages und auf die Rechnung oben; sie ist keine Aussage über einen Vergleich mit Swift, der nicht angestellt wurde.

---
Answered:
Implemented:
Deferred:
Superseded by:
