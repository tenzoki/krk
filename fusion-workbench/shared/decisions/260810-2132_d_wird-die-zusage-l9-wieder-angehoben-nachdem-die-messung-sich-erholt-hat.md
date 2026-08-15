# Wird die Zusage L9 wieder angehoben, nachdem die Messung sich erholt hat?

---
**Domain:** code
**Status:** deferred
**Filed by:** orchestrator (nach dem Abnahmelauf des Nutzers vom 260810)
**Cross-references:** `shared/issues/260807-1748_*_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`,
`messungen/260810-1918-alle-zusagen.txt`, `messungen/260807-1538-alle-zusagen.txt`,
`circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_*_leistungszusagen-navigator.md`,
`crates/krk-bench/src/messen.rs:1147` (`mindestanteil_prozent: 65`)

---

## Frage

L9 misst, welcher Anteil der Tastendrücke während einer laufenden Kopie im nächsten Bild
ankommt. Die Zusage stand ursprünglich bei 95 Prozent, ist am 260807 zweimal an einem Tag
gesenkt worden — auf 85, dann auf 65 Prozent bei höchstens zwei Bildlängen je Einzelwert — und
die zweite Senkung geschah gegen die Empfehlung des zugehörigen Datensatzes, weil der erste
vollständige Lauf danach den Boden von 85 nicht hielt.

**Der Abnahmelauf vom 260810 misst deutlich besser.** Beide Hälften des Urteils haben sich
erholt:

| | 260807-1538 | 260810-1918 |
|---|---|---|
| Anteil im Bild | 65,0 % | 90 / 70 / 85 / 80 / 85 % je Runde |
| höchster Einzelwert | 1,70 Bilder | 1,24 / 1,26 / 1,15 / 1,25 / 1,24 |
| Urteil | verfehlt, gehalten in 1 von 5 Runden | gehalten in 5 von 5 |
| Systemlast vor dem Lauf | { 1,41 1,67 1,76 } | { 2,15 2,71 5,13 } |

Die letzte Zeile ist die wichtigste: **gemessen wurde unter höherer Last, nicht unter
niedrigerer.** Eine ruhigere Maschine scheidet als Erklärung aus.

Damit steht der Fall, den der geschlossene Defekt `260807-1748` vorgesehen hatte, halb ein. Sein
Schlusssatz lautet: „Wer die Ursache findet, hebt die Zahl wieder an, statt sie ein drittes Mal
nachzuziehen." Die Zahl ist zurück — **die Ursache ist es nicht.** Niemand weiß, warum L9 am
260805 einbrach und warum es sich jetzt erholt hat. Genau deshalb ist das eine Frage und keine
Erledigung.

## Optionen

1. **Bei 65 Prozent bleiben.** Die Zusage ist eine Untergrenze, kein Ziel; sie wird gehalten, und
   eine gehaltene Zusage anzufassen schafft Arbeit ohne Anlass.
   - Pro: kein Risiko, keine Änderung, keine dritte Bewegung derselben Zahl an derselben Zusage.
   - Contra: Die Zusage misst dann dauerhaft etwas anderes als das, was die Anwendung leistet.
     Eine Untergrenze, die 20 Punkte unter der Wirklichkeit liegt, fängt keine Verschlechterung
     mehr auf — genau die Verschlechterung, deren Ursache bis heute unbekannt ist. Sie wäre
     stumm geworden.

2. **Auf 85 Prozent zurück**, also auf den Stand vor der zweiten Senkung vom 260807-1900.
   - Pro: Stellt die Zusage her, die vor dem unerklärten Einbruch galt, und macht die Zusage
     wieder zu einem Wächter. Der niedrigste gemessene Rundenwert liegt bei 70 Prozent, der
     Mittelwert bei 82.
   - Contra: **Zwei von fünf Runden lägen darunter** (70 und 80 Prozent). Die Zusage wäre damit
     ab morgen verfehlt, obwohl sich nichts verschlechtert hat. Das ist keine Verschärfung,
     sondern ein garantierter Fehlschlag.

3. **Auf einen Wert zwischen 65 und 85 anheben**, etwa 70 Prozent — den niedrigsten gemessenen
   Rundenwert.
   - Pro: Die Zusage würde vom heutigen Stand knapp gehalten und finge eine Verschlechterung
     wieder auf, statt sie durchzulassen. Sie bliebe eine Aussage über die Anwendung.
   - Contra: Ein Boden, der genau auf dem schlechtesten gemessenen Wert liegt, verfehlt beim
     nächsten Lauf mit hoher Wahrscheinlichkeit — die Streuung zwischen den Runden beträgt hier
     20 Punkte. Wer diesen Weg geht, braucht Abstand nach unten, und wie viel, sagt eine
     Messreihe von fünf Runden nicht.

4. **Erst messen, dann entscheiden.** Die Zusage bleibt bei 65 Prozent, und die Frage wird
   vertagt, bis mehr Läufe vorliegen — etwa fünf weitere Abnahmeläufe an verschiedenen Tagen.
   - Pro: Die Streuung von 20 Punkten zwischen den Runden ist zu groß, um aus **einem** Lauf eine
     Grenze abzuleiten. Beide bisherigen Senkungen sind aus je einem Lauf entstanden, und beide
     mussten binnen Stunden nachgezogen werden. Ein drittes Mal auf einer Stichprobe zu
     entscheiden, wiederholte den Fehler in die andere Richtung.
   - Contra: Der Abnahmelauf ist Nutzerarbeit und braucht KRK im Vordergrund; fünf weitere Läufe
     sind kein Nebenbei. Bis dahin bleibt die Zusage stumm.

## Randbedingungen

- Der Spec der Runde 1 verlangt, dass eine Zusage **über einen Entscheidungsdatensatz** abgelöst
  und nicht stillschweigend gelockert wird. Das gilt für eine Anhebung genauso.
- Der Spec der Runde 2 führt als eigenes Abnahmekriterium: „Keine der zehn Zahlen aus C8 der
  Runde 1 wird durch diese Runde geändert, gelockert oder umgedeutet." Eine Änderung an L9 gehört
  deshalb ausdrücklich **nicht** zur Runde 2, sondern hierher.
- `mindestanteil_prozent: 65` steht an einer Stelle (`crates/krk-bench/src/messen.rs:1147`), mit
  einer langen Begründung im Kommentar darüber. Wer die Zahl ändert, zieht diese Begründung mit;
  sie erzählt heute die Geschichte der zwei Senkungen.

## Empfehlung

**Option 4.** Der Grund ist nicht Vorsicht, sondern die Datenlage: die Streuung zwischen den
Runden dieses einen Laufs beträgt 20 Punkte (70 bis 90), und die Grenze, um die es geht, liegt
mit 65 nur fünf Punkte unter dem schlechtesten davon. Aus einer Stichprobe mit dieser Streuung
lässt sich keine Grenze ableiten, die morgen noch gilt — beide bisherigen Senkungen sind auf
genau diesem Weg entstanden und mussten beide nachgezogen werden.

Was gegen die Empfehlung spricht und mitgehört gehört: Option 1 und Option 4 sind im Ergebnis
dasselbe, solange niemand die weiteren Läufe fährt. Wer nicht damit rechnet, sie zu fahren,
entscheidet in Wahrheit Option 1 und sollte das so aufschreiben.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Deferred: **Option 4, „erst messen"** — vom Nutzer am 260810-2140 gewaehlt, der Empfehlung dieses
Datensatzes folgend. `mindestanteil_prozent: 65` (`crates/krk-bench/src/messen.rs:1147`) bleibt
unveraendert; am Code ist nichts zu tun, und genau deshalb ist dieser Datensatz zurueckgestellt
und nicht beantwortet.

**Der Ausloeser, der ihn wieder aufmacht, ist Teil der Antwort und steht hier, damit er nicht
vergessen wird: weitere Abnahmelaeufe an verschiedenen Tagen.** Fuenf sind im Optionstext
genannt; die Zahl ist ein Vorschlag und keine Zusage. Entscheidbar wird die Frage, sobald sich
aus mehreren Laeufen ablesen laesst, wo der Boden von L9 wirklich liegt — heute streuen die
Runden eines einzigen Laufs um 20 Punkte, von 70 bis 90.

**Die Falle, die dieser Datensatz oben selbst benennt, ist mit dieser Antwort scharf.** Option 1
(„bei 65 bleiben") und Option 4 sind im Ergebnis dasselbe, solange niemand die weiteren Laeufe
faehrt. Wird nicht gemessen, ist in der Sache Option 1 entschieden — nur ohne dass es jemand
aufgeschrieben haette. Damit das auffaellt statt zu verschwinden, nennt `CLAUDE.md` diesen
Datensatz samt seinem Ausloeser im Absatz ueber L9; die Datei wird in jeder Sitzung geladen und
ist die einzige Flaeche hier, an der ein zurueckgestellter Datensatz nicht aus dem Blick faellt.

Ein zurueckgestellter Datensatz zaehlt als Grounding-Historie und taucht in einer Suche nach
aktiver Grundlage (`_o_` und `_a_`) nicht mehr auf. Das ist bei dieser Frage richtig — sie bindet
keine laufende Arbeit — und zugleich der Grund fuer den Verweis aus `CLAUDE.md`.
