# Stirbt die Prozessgruppe auch am normalen Ende des Laufs, oder nur beim Abbruch?

---
**Domain:** code
**Status:** open
**Filed by:** planner
**Cross-references:** `shared/planning/260816-2240_o_spec-befehle-absetzen-und-makros-speichern.md` (C1.9, C1.10, C1.15 und `## Was der Befehlslauf nicht kann`); `circles/260816-2255-befehle-absetzen-und-makros-speichern/planning/260816-2307_o_plan-befehle-absetzen-und-makros-speichern.md` Schritt A3 und die Zeile `**Decidability:**` im Plankopf

---

## Question

Der Spec sagt die Prozessgruppe für den **Abbruch** zu: `Esc` schickt das Signal an die Gruppe, und nach dem Abbruch lebt kein Kindprozess des Laufs mehr (C1.10). Er sagt nichts über das gewöhnliche Ende.

Beim Bauen fällt auf, dass dieselbe Frage dort ebenfalls fällig ist, und zwar aus einem Grund, der nicht auf der Hand liegt. KRK erkennt das Ende der Ausgabe am Dateiende der Röhre, und das Dateiende kommt erst, wenn **jedes** Schreibende geschlossen ist. Ein abgehängter Enkelprozess hält sein Schreibende offen, auch wenn die Shell längst geendet hat. Bei `sleep 300 & echo fertig` endet die Shell nach Sekundenbruchteilen, die Röhre bleibt fünf Minuten offen, und für KRK läuft der Vorgang weiter: die Vorgangsanzeige steht, und C1.15 weist jeden weiteren Befehl und jede Dateioperation ab.

Zwei Fragen liegen hier dicht beieinander. „Kommt noch Ausgabe?" ist aus dem, was KRK hat, nicht entscheidbar — ein offenes Schreibende sieht genauso aus wie ein langsamer Befehl. „Hat die Shell geendet?" ist entscheidbar, `waitpid` beantwortet sie.

## Options

1. **Der Lauf endet mit der Shell, und die Prozessgruppe stirbt mit ihm.** Sobald `wait` den Rückgabewert der Shell liefert, geht dasselbe Signal an die Gruppe, das auch der Abbruch schickt; damit fallen alle übrigen Schreibenden, die Röhre meldet Dateiende, und der Vorgang schließt.
   - Pro: Ein Mechanismus mit zwei Auslösern statt zweier Mechanismen. Der Vorgang endet, wenn der Nutzer es erwartet, nämlich mit dem Befehl, den er abgesetzt hat. C1.15 bleibt bedienbar.
   - Contra: Ein bewusst in den Hintergrund gestellter Prozess wird beendet. Wer `irgendwas &` schreibt, bekommt keinen laufenden Hintergrundprozess.
2. **Der Lauf endet mit dem Dateiende der Röhre, die Gruppe stirbt allein beim Abbruch.**
   - Pro: Ein abgehängter Prozess überlebt.
   - Contra: Der Vorgang gilt als laufend, solange irgendein Nachkomme lebt. Der Nutzer sieht eine Vorgangsanzeige zu einem Befehl, der fertig ist, und kann keinen zweiten absetzen. Der einzige Ausweg wäre `Esc`, also derselbe Griff, der in Möglichkeit 1 von selbst geschieht.
3. **Der Lauf endet mit der Shell, die Gruppe bleibt am Leben, die Röhre wird geschlossen.** Der Vorgang schließt, der abgehängte Prozess läuft weiter, seine Ausgabe geht ins Leere.
   - Pro: Beides zugleich, dem Anschein nach.
   - Contra: KRK ließe Prozesse zurück, von denen es dem Nutzer nichts sagt und die er in KRK nicht wiederfindet. Der nächste Lauf legte weitere daneben. Ein Dateiverwalter, der unsichtbare Prozesse hinterlässt, ist eine Zusage, die niemand bestellt hat.

## Constraints

- C1.15 verlangt, dass genau ein Vorgang läuft und ein zweiter abgewiesen wird. Ein Vorgang, der ohne Zutun des Nutzers nicht endet, macht diese Zusage zur Sperre.
- C1.10 verlangt für den Abbruch, dass kein Kindprozess überlebt. Der Mechanismus dafür steht ohnehin.
- Der Spec schließt ein eingebautes Terminal aus. Ohne Terminal gibt es keine Auftragsverwaltung, in der ein abgehängter Prozess sichtbar wäre.

## Recommendation

**Möglichkeit 1**, und die Grenze gehört ausgeschrieben zu den drei, die der Spec unter `## Was der Befehlslauf nicht kann` schon führt: **ein mit `&` abgehängter Prozess überlebt den Lauf nicht.** Wer einen Prozess laufen lassen will, der KRK überdauert, nimmt `Ctrl+O` und sein Terminal.

Der Gewinn ist mehr als die Vermeidung einer Sperre. Möglichkeit 1 ersetzt eine unentscheidbare Frage durch eine entscheidbare, und sie tut es ohne einen zweiten Mechanismus: das Signal an die Gruppe steht für den Abbruch ohnehin da und bekommt einen zweiten Auslöser.

---
Answered:
Implemented:
Deferred:
Superseded by:
Retired:
