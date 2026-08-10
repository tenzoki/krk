# Orchestrator — Nacharbeit zum Abnahmelauf, 260810-2128

**Directive:** Nach dem Abnahmelauf des Nutzers zu einem vollständigen und korrekten Stand gelangen.
**Mode:** custom
**Status:** Abgeschlossen

## Anlass

Der Nutzer hat am 260810 den Abnahmelauf der zehn Zeitzusagen aus C8 gefahren, den kein Agent
fahren kann, weil er KRK im Vordergrund verlangt. Zwei Läufe: `messungen/260810-1912-alle-zusagen.txt`
mit einer Runde und `messungen/260810-1918-alle-zusagen.txt` mit fünf. **Alle zehn Zusagen halten
in allen Runden** — der erste vollständig saubere Lauf des Projekts.

## Was der Lauf ergeben hat

**L9 hat sich in beiden Hälften des Urteils erholt.** Gegen den Lauf vom 260807-1538:

| | 260807-1538 | 260810-1918 |
|---|---|---|
| Anteil im Bild | 65,0 % | 90 / 70 / 85 / 80 / 85 % je Runde |
| höchster Einzelwert | 1,70 Bilder | 1,24 / 1,26 / 1,15 / 1,25 / 1,24 |
| Urteil | verfehlt, 1 von 5 Runden | gehalten, 5 von 5 |
| Systemlast vorher | { 1,41 1,67 1,76 } | { 2,15 2,71 5,13 } |

Die letzte Zeile trägt das Argument: gemessen wurde unter **höherer** Last. Eine ruhigere
Maschine scheidet als Erklärung aus. Die Ursache der Erholung ist so ungemessen wie die des
Einbruchs vom 260805.

**L6 lief in allen sechs Runden beider Läufe sauber durch.** Die zwei Diagnosen, die der
zurückgestellte Defekt `260806-1304` vorgesehen hatte, sind beide nicht eingetreten.

## Was getan wurde

1. **Ein Duplikat entfernt.** `messungen/260810-2113-alle-zusagen.txt` war kein dritter Lauf,
   sondern das Terminalprotokoll des 19:18-Laufs, entstanden durch eine `>`-Umleitung, die der
   Orchestrator in seiner Anleitung empfohlen hatte, ohne zu wissen, dass die Messstrecke ihren
   Bericht selbst nach `messungen/` schreibt und das am Ende auch ausgibt. Der Dateiname trug
   21:13, der Inhalt 19:18 — genau die Sorte Widerspruch, die einen späteren Leser in die Irre
   führt. Die Anleitung war an dieser Stelle falsch; der Fehler liegt beim Orchestrator.
2. **Die beiden echten Berichte abgelegt** (`c531e70`).
3. **Die Evidenz an den zurückgestellten L6-Defekt gehängt**, ohne seinen Marker anzufassen. Der
   Nachtrag sagt ausdrücklich, wie stark die Evidenz ist: bei einer Rate von einem Drittel je
   Lauf sind zwei saubere Läufe in rund 44 von 100 Fällen Zufall, sechs saubere Runden nur in
   rund 9 von 100 — und welche der beiden Zählungen gilt, ist aus der ursprünglichen
   Aufzeichnung nicht zu entscheiden. Ob der Datensatz schließt, gehört dem Nutzer.
4. **Die L9-Frage als Entscheidungsdatensatz abgelegt** statt nebenbei beantwortet
   (`shared/decisions/260810-2132_*_wird-die-zusage-l9-wieder-angehoben-…`). Vier Optionen, mit
   Empfehlung für „erst messen": die Streuung zwischen den Runden beträgt 20 Punkte, und beide
   bisherigen Senkungen sind aus je einem Lauf entstanden und mussten beide nachgezogen werden.
   Der Datensatz hält auch fest, dass Option 1 und Option 4 im Ergebnis dasselbe sind, solange
   niemand die weiteren Läufe fährt — wer nicht damit rechnet, entscheidet in Wahrheit Option 1.
5. **`CLAUDE.md` an drei Stellen nachgezogen** (`69d2156`): der Absatz über den Abnahmelauf führte
   `260807-1538` als letzten Lauf mit neun von zehn Zusagen; drei der vier Gegenstände der
   späteren Messrunde sind erledigt (L1, L4, L7), offen bleibt die Syntaxhervorhebung; und der
   Messlauf ist ausdrücklich vom Abnahmelauf der Runde 2 getrennt.

## Was ausdrücklich **nicht** erledigt ist

**Der Abnahmelauf der Runde 2 steht unverändert aus.** Seine 110 Kriterien sind Bedienprüfungen
an der Oberfläche und stehen sämtlich auf `- [ ]`. Der heutige Lauf nimmt die zehn Zusagen der
Runde 1 ab und ist eine andere Sache. Die Verwechslung ist naheliegend genug, dass `CLAUDE.md`
sie jetzt ausdrücklich ausschließt.

Die Geschwindigkeit der Syntaxhervorhebung aus C3 ist weiterhin ungemessen; sie gehört zu keiner
der zehn Zusagen.

## Prüfung des Bestands

Jeder in `CLAUDE.md` zitierte Datensatz und jeder zitierte Dateipfad ist am 260810-2135 gegen den
Baum gelesen: 13 Datensätze, 7 Pfade, alle vorhanden. Der Arbeitsbaum ist sauber.

## Offen

| Art | Zahl |
|---|---|
| Offene Defekte | 5 |
| Zurückgestellte Defekte | 1 (L6, `260806-1304`) |
| Offene Entscheidungen | 13 |

Zwei davon warten unmittelbar auf den Nutzer: ob der L6-Defekt schließt, und ob L9 wieder
angehoben wird.

## Commits

| Hash | Was |
|---|---|
| `c531e70` | Die zwei Messberichte, Duplikat entfernt |
| `69d2156` | `CLAUDE.md` nachgezogen, L9-Entscheidung, L6-Evidenz |

---

## Nachtrag 260810-2140: die beiden Entscheidungen sind gefallen

**L6 (`260806-1304`) ist geschlossen.** Der Nutzer hat entschieden, nachdem die Evidenz am
Datensatz stand. Geschlossen wird er als *nicht wieder aufgetreten*, ausdrücklich nicht als
*erklärt*: welcher der beiden Fälle der Abbruch vom 260806 war, war allein aus dem nächsten
Abbruch zu beantworten, und der ist ausgeblieben. Tritt der Aussetzer wieder auf, ist das ein
neuer Datensatz mit einer neuen Messreihe.

**L9 (`260810-2132`) ist zurückgestellt, Option 4 „erst messen".** `mindestanteil_prozent: 65`
bleibt unverändert; am Code ist nichts zu tun, und genau deshalb `_d_` und nicht `_a_` — die
Zurückstellung ist hier die Antwort, nicht ein Warten auf Umsetzung.

**Ein Nebeneffekt dieser Wahl ist bearbeitet worden, statt ihn eintreten zu lassen.** Ein
zurückgestellter Datensatz zählt als Grounding-Historie und fällt aus jeder Suche nach aktiver
Grundlage (`_o_` und `_a_`) heraus. Bei dieser Frage ist das richtig — sie bindet keine laufende
Arbeit —, öffnet aber genau die Falle, die der Datensatz selbst benennt: Option 1 und Option 4
sind im Ergebnis dasselbe, solange niemand die weiteren Läufe fährt, und wer nicht misst,
entscheidet in Wahrheit Option 1, ohne es aufgeschrieben zu haben.

Dagegen steht jetzt eine Stelle, die nicht aus dem Blick fällt: `CLAUDE.md` nennt im Absatz über
L9 den zurückgestellten Datensatz **samt seinem Auslöser** — weitere Abnahmeläufe an
verschiedenen Tagen. Die Datei wird in jeder Sitzung geladen und ist damit die einzige Fläche
dieses Projekts, an der eine zurückgestellte Frage sichtbar bleibt. Das ist kein Mechanismus,
sondern eine Gewohnheit; mehr steht hier nicht zur Verfügung, und es als Mechanismus zu
bezeichnen wäre falsch.

### Stand danach

| Art | Zahl |
|---|---|
| Offene Defekte | 5 |
| Zurückgestellte Defekte | 0 |
| Offene Entscheidungen | 12 |
| Zurückgestellte Entscheidungen | +1 (`260810-2132`) |
