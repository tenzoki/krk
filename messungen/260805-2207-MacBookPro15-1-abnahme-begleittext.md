# Begleittext zur Abnahme-Messreihe vom 260805-2207 (S22)

Zwei Berichte gehören zusammen, beide vom `MacBookPro15,1` bei 60 Hz, je fünf
Runden mit zwanzig Wiederholungen je Zusage:

- `260805-2207-MacBookPro15-1-abnahme.txt` — das ruhige Gerät. Systemlast
  vor dem Lauf { 1.90 2.16 2.30 }, nach dem Lauf { 2.50 2.84 2.62 }; keine
  konkurrierende Arbeit des Nutzers, die Grundlast von rund 2 stammt von den
  Hintergrunddiensten des Systems.
- `260805-2212-MacBookPro15-1-abnahme-unter-last.txt` — dieselbe Reihe unter
  bekannter, selbst erzeugter Last: sechs Endlosschleifen (`yes > /dev/null`)
  auf dem Sechskern-Gerät, gestartet 15 Sekunden vor dem Lauf und danach
  beendet. Systemlast vor dem Lauf { 3.12 2.87 2.64 }, nach dem Lauf
  { 9.32 6.61 4.42 }.

Der ruhige Bericht ist der Abnahmelauf; der Lastbericht ist Diagnose, kein
Abnahmemaß.

## Urteil je Zusage (ruhiger Lauf)

Neun der zehn Zusagen halten in jeder der fünf Runden, eine ist verfehlt:

| Zusage | Maß | ruhig | Urteil |
|---|---|---|---|
| L1 Tastendruck bis Zeichendurchgang | ≥ 95 % im Bild | 95/100/100/100/100 % | gehalten |
| L2 erste Bildschirmseite (kopflos) | p95 ≤ 100 ms | 5,4–5,8 ms | gehalten |
| L3 vollständig gelesen (kopflos, warm) | p95 ≤ 400 ms | 21,4–22,0 ms | gehalten |
| L4 Prozessstart bis bedienbare Prüfsitzung (warm) | p95 ≤ 1000 ms | 378,9–398,1 ms | gehalten |
| L5 Wechsel auf verdeckten Tab | p95 ≤ 50 ms | 35,2–36,5 ms | gehalten |
| L5 Wechsel des Dateifensters | p95 ≤ 50 ms | 14,1–16,6 ms | gehalten |
| L6 Einstieg in 1.000er-Unterordner | p95 ≤ 100 ms | 46,1–47,4 ms | gehalten |
| L7 Vorschau sichtbar | p95 ≤ 100 ms | 33,5–35,0 ms | gehalten |
| L8 Kopie bis Fortschrittsanzeige | p95 ≤ 200 ms | 168,4–169,4 ms | gehalten |
| L9 Tastendruck während laufender Kopie | ≥ 95 % im Bild | 90/85/90/100/85 % | **VERFEHLT** |
| L10 erste Bildschirmseite, 100.000 (kopflos) | p95 ≤ 100 ms | 5,0–5,7 ms | gehalten |
| L10 vollständig gelesen, 100.000 (kopflos, warm) | p95 ≤ 4000 ms | 216,9–224,2 ms | gehalten |

Die Spannen je Zeile sind die Runden-Perzentile vom besten bis zum
schlechtesten der fünf Runden. L4 ist warm gemessen, weil `purge` Rechte
braucht, die der Lauf nicht hat; die Zahl ist eine Untergrenze der
Kaltstart-Zusage, der Berichtskopf weist das aus. L8 ist auf dem Klonweg
innerhalb eines APFS-Datenträgers gemessen, keine Durchsatzangabe.

## Der L4-Vergleich: die Streuung kommt von außen

Frage aus dem Defekt 260803-1845: streut L4 zwischen den Runden, weil der
Startpfad von KRK streut, oder weil Fremdlast auf dem Gerät lag?

| | ruhig | unter Last |
|---|---|---|
| Runden-p95, bester bis schlechtester | 378,857–398,094 ms | 538,857–597,796 ms |
| Spannweite der fünf Runden-Perzentile | 19,2 ms | 58,9 ms |
| Median aller 100 Werte | 369,6 ms | 557,3 ms |
| kleinster / größter Einzelwert | 344,461 / 460,764 ms | 521,405 / 761,646 ms |

Auf dem ruhigen Gerät ist L4 eng: fünf Runden-Perzentile innerhalb von
19 ms, kein Einzelwert über 461 ms. Unter der benannten Last hebt sich das
Niveau um rund 50 Prozent, die Spannweite der Runden-Perzentile
verdreifacht sich, und Runde 1 zeigt mit 761,646 ms genau die Form der
Auffälligkeit vom 260803-1641: eine ganze Runde liegt hoch, mit einem
langen Ausläufer. Die Streuung reagiert also auf die Lastbedingung, nicht
auf den Startpfad. Befund zum Defekt 260803-1845: die Streuung kommt von
außen; ein eigener Startpfad-Defekt ist nicht angezeigt. Die Bedingung,
die der Plan seit dem 260804-2318 vorschreibt (ruhiges Gerät, Lastkennzahl
als neunte Kopfangabe), reicht als Absicherung.

## L1 und L9: der Befund zum Defekt 260805-2335

Der erste Gesamtlauf (260805-2134, eine Runde, Fremdlast um loadavg 3)
hatte L1 mit 75 Prozent und L9 mit 90 Prozent im Bild ausgewiesen. Die
beiden Verfehlungen trennen sich jetzt:

- **L1 war Fremdlast.** Ruhig hält L1 in allen fünf Runden (19/20, dann
  viermal 20/20); unter der reinen Rechenlast sogar fünfmal 20/20. Die
  Ausreißer von 24 bis 130 ms aus dem ersten Gesamtlauf sind in beiden
  Reihen nicht wieder aufgetreten; der größte ruhige L1-Wert ist 22,195 ms.
- **L9 ist es nicht.** Auf dem ruhigen Gerät hält L9 nur in einer von fünf
  Runden (100 Prozent in Runde 4; sonst 85 bis 90 Prozent, also zwei bis
  drei von zwanzig Eingaben über der Bildlänge). Unter Last hält es in drei
  von fünf Runden. Die Verfehlung ist klein und begrenzt: jeder verpasste
  Wert liegt zwischen 17,2 und 23,4 ms, also im zweiten Bild, nie darüber.
  Das Muster spricht gegen Fremdlast und für eine Ursache im Programm,
  naheliegend die Arbeit der laufenden Kopie beziehungsweise ihrer
  Fortschrittsanzeige im Hauptfaden.

Wie C8 es verlangt, führt die verfehlte Zusage zu einem
Entscheidungsdatensatz statt zu einer stillschweigenden Lockerung:
`decisions/260806-0014_o_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`
im aktiven Circle.

## Messplatz

Die Prüfordner liegen seit diesem Lauf unter
`~/Library/Caches/krk-messplatz/` und nicht mehr unter `/tmp`: eine
Systembereinigung hat dort in der Nacht zum 260806 sämtliche leeren,
zeitgestempelt zurückdatierten Unterordner aller vier Prüfordner gelöscht
und damit erst die Bestände verfälscht und dann einen laufenden Messlauf
abbrechen lassen (die Strecke hat die uneinheitliche Reihe korrekt
verworfen). Festgehalten als Defekt
`issues/260806-0014_o_pruefordner-unter-tmp-verlieren-leere-unterordner-an-die-systembereinigung.md`.
Alle drei Ordner wurden am neuen Ort aus ihren Startwerten neu erzeugt und
vor dem Lauf auf 10.000, 10.000 und 100.000 Einträge geprüft; das
Kopierziel liegt daneben auf demselben APFS-Datenträger.
