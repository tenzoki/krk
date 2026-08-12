# Welchen Anteil bekommt ein Bereich, der noch nie sichtbar war?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-core/src/ablage/sitzung.rs:181` (`Breiten`), `crates/krk-ui/src/fenstermodell.rs:191` (`Bereich::anfangsbreite`), `.../decisions/260811-1305_o_was-heisst-proportional-zur-letzten-aufteilung.md`

---

## Question

Eine proportionale Neuaufteilung braucht für jeden Bereich, der aufgeht, einen Anteil. Ein Bereich, der noch nie sichtbar war, hat keinen: `Breiten` führt fünf `Option<f64>`, und ein Feld ohne Wert bedeutet genau das. Der Fall ist nicht theoretisch. Der Editor steht beim allerersten Start auf `false`, weil er keine Datei hält, und seine Breite ist bis zum ersten Öffnen unbelegt.

Die heutige Regel beantwortet den Fall mit `unwrap_or_else(|| bereich.anfangsbreite())`, also mit einer Punktzahl. Unter einer Anteilsregel ist zu sagen, woraus der erste Anteil entsteht. Die Frage hängt an der Antwort auf `260811-1305_o_was-heisst-proportional-zur-letzten-aufteilung.md`: bleibt es bei der heutigen Regel, entfällt sie.

## Options

1. **Die Anfangsbreite in Punkten, beim ersten Aufgehen in einen Anteil umgerechnet.** Danach nimmt der Bereich proportional teil wie jeder andere.
   - Pros: Der Rückfall existiert bereits und ist begründet: die Anfangsbreiten sind gesetzte Zahlen, und die 460 des Editors folgen aus C1 der Editor-Runde, "rund ein Drittel der Fensterbreite". Es entsteht keine zweite Regel.
   - Cons: Der Anteil hängt an der Fensterbreite im Moment des ersten Aufgehens. Ein schmales Fenster gibt dem Editor beim ersten Mal einen größeren Anteil als ein breites.
   - **Folgen weiter unten:** Der Aktivierungs-Spec braucht ein Abnahmekriterium für den allerersten Start mit dem Editor, weil dieser Fall genau einmal je Installation eintritt und danach nicht wiederholbar ist.

2. **Die Mindestbreite.** Der Bereich geht so schmal wie möglich auf, und der Nutzer zieht ihn breiter.
   - Pros: Die übrigen Bereiche geben so wenig ab wie möglich.
   - Cons: Der Editor ginge mit 320 Punkten auf statt mit 460 und verfehlte die Zusage aus C1 der Editor-Runde, rund ein Drittel der Fensterbreite. `Bereich::anfangsbreite` würde bedeutungslos.
   - **Folgen weiter unten:** Ein abgenommenes Kriterium der Editor-Runde fällt, und `anfangsbreite` verliert seinen einzigen Aufrufer.

3. **Ein gleicher Anteil mit den übrigen sichtbaren Bereichen.** Bei drei sichtbaren Bereichen bekäme der vierte ein Viertel.
   - Pros: Eine Regel ohne Zahlentabelle.
   - Cons: Sie gibt der Lesezeichenleiste beim ersten Aufgehen denselben Platz wie einem Dateifenster. Die Anfangsbreiten sind bewusst ungleich, weil die Bereiche ungleich viel zu zeigen haben.
   - **Folgen weiter unten:** `Bereich::anfangsbreite` entfällt, und die Zusagen aus C1 beider Runden über die Anfangsgröße fallen mit.

## Constraints

- C1 der Editor-Runde sagt dem Editor "rund ein Drittel der Fensterbreite" beim ersten Aufgehen zu.
- Die Mindestbreiten gewinnen gegen jeden Anteil.
- `session.toml` ist nach C7 zum Lesen und Ändern von Hand gedacht; ein von Hand entferntes Feld führt in denselben Fall.

## Recommendation

**Möglichkeit 1.** Der Rückfall auf die Anfangsbreite steht bereits im Code und trägt die Begründung, aus der die Zahlen entstanden sind. Eine Umrechnung beim ersten Aufgehen ändert daran nichts, sondern übersetzt eine vorhandene Antwort in die neue Währung.


## Antwort 260812-0306

**Moeglichkeit 1: die Anfangsbreite in Punkten, beim ersten Aufgehen in einen Anteil
umgerechnet.**

Der Rueckfall steht bereits im Code (`unwrap_or_else(|| bereich.anfangsbreite())`) und traegt
die Begruendung, aus der die Zahlen entstanden sind: die 460 des Editors folgen aus C1 der
Editor-Runde, "rund ein Drittel der Fensterbreite". Da die Antwort auf die erste Frage die
Anteile ohnehin beim Lesen aus den gespeicherten Punktzahlen entstehen laesst, ist hier gar
nichts Eigenes zu bauen: ein unbelegtes Feld faellt auf `anfangsbreite()` zurueck und geht mit
dieser Zahl in dieselbe Verhaeltnisrechnung wie jeder andere Bereich.

Moeglichkeit 2 (Mindestbreite) liesse den Editor mit 320 statt 460 Punkten aufgehen und
verfehlte die Zusage aus C1 der Editor-Runde. Moeglichkeit 3 (gleicher Anteil) gaebe der
Lesezeichenleiste denselben Platz wie einem Dateifenster und liesse `anfangsbreite` bedeutungslos
werden.

**Die im Datensatz genannte Schwaeche bleibt und ist klein:** der erste Anteil haengt an der
Fensterbreite im Moment des ersten Aufgehens. Das ist bereits heute so und aendert sich durch
die Anteilsregel nicht.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-0306` — beantwortet vom Orchestrator in der Klaerungsrunde bei der Aktivierung des Circles; Sitzungsprotokoll `circles/260811-1304-statusleiste-mit-bereichsschaltern/history/260812-0306-klaerungsrunde.md`.
Implemented: 5e17c9e — ein unbelegtes Feld faellt auf `anfangsbreite()` zurueck und geht mit dieser Zahl in dieselbe Verhaeltnisrechnung wie jeder andere Bereich.
Deferred:
Superseded by:
