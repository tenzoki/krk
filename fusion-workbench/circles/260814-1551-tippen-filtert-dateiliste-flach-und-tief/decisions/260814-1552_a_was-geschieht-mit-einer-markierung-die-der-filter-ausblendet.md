# Was geschieht mit einer Markierung, die der Filter ausblendet?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `crates/krk-ui/src/kommandos/operationen.rs:162-192` (`betroffene`, die eine Auswahlregel); `crates/krk-core/src/verzeichnis/modell.rs:339-416` (Markierung und Markierungsstand); `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_c_spec-navigator-geruest.md` (C2, Mehrfachauswahl)

---

## Question

Der Baum hat für diesen Fall bereits eine Regel, und ein Filter macht sie vom Ausnahmefall zum Regelfall. `betroffene` läuft allein über die sichtbaren Zeilen, mit der ausgeschriebenen Begründung: „eine Markierung, die der Nutzer beim Drücken der Taste nicht vor sich hatte, gehört nicht in den Auftrag." Eine ausgeblendete Markierung besteht dabei fort, sie wirkt nur nicht. Heute betrifft das allein versteckte Dateien, die der Nutzer eigens einblendet und wieder ausblendet. Mit einem Filter, den jeder Tastendruck verändert, kann ein Nutzer dreißig Einträge markieren, drei Buchstaben tippen und dann eine Löschung auslösen, die vier Einträge trifft statt dreißig. Die bestehende Regel verhält sich dabei richtig und still. Ob still hier richtig ist, ist die Frage.

## Options

1. **Die bestehende Regel bleibt, unverändert und ohne Zusatz.** Der Filter blendet aus, die ausgeblendete Markierung besteht fort und wirkt nicht; wird der Filter geleert, wirkt sie wieder.
   - Pro: keine zweite Regel neben `betroffene`; das Verhalten ist bereits gebaut, geprüft und begründet.
   - Kontra: der Nutzer sieht nicht, dass ein Teil seiner Markierung gerade nicht wirkt. Die Zahl im Markierungsstand zählt über alle Einträge und stimmt dann nicht mit dem überein, was ein Befehl trifft.
2. **Die bestehende Regel bleibt, und die Statuszeile sagt es.** Der Markierungsstand nennt die wirksamen und die gesamten Markierungen getrennt, solange ein Filter steht.
   - Pro: das Verhalten bleibt, die stille Stelle wird sichtbar.
   - Kontra: hängt an der Frage, wo die Filterzahl in der Rangfolge der einen Statuszeile steht, die daneben als eigener Datensatz liegt. Beide Antworten müssen zusammenpassen.
3. **Der Filter hebt jede Markierung auf, sobald er sich ändert.**
   - Pro: es gibt nie eine Markierung, die der Nutzer nicht sieht; die Zahl im Markierungsstand und der Auftrag stimmen immer überein.
   - Kontra: nimmt dem Nutzer Arbeit weg, die er gerade getan hat. Wer dreißig Einträge markiert und dann tippt, um den einunddreißigsten zu finden, verliert alle dreißig. Der Baum hat diese Bauart beim Rechtsklick ausdrücklich abgelehnt, weil sie die Markierung des Nutzers wegnähme.
4. **Der Filter blendet markierte Einträge nie aus.** Ein markierter Eintrag bleibt sichtbar, auch wenn sein Name nicht passt.
   - Pro: der Auftrag und die Anzeige stimmen immer überein, ohne dass eine Markierung verlorengeht.
   - Kontra: der Filter tut dann nicht mehr, was er sagt, und die Liste zeigt Einträge, die nicht zum Filtertext passen. Bei dreißig Markierungen ist die gefilterte Liste kaum kürzer als die ungefilterte.

## Constraints

- `betroffene` bleibt die eine Auswahlregel. Eine zweite daneben ist durch die Runde 6 ausgeschlossen (`decisions/260812-1145_*_bewegt-ein-rechtsklick-in-der-dateiliste-die-auswahl.md` in jenem Circle).
- Das Stapelumbenennen holt seine Namen aus derselben Auswahl und prüft Kollisionen gegen den vollen Bestand. Es braucht keine eigene Antwort.
- `markierung_aufheben` hebt heute jede Markierung auf, auch die unsichtbare, mit der ausgeschriebenen Begründung, dass „jede Markierung aufheben" jede heißt. Möglichkeit 3 dürfte diese Aussage nicht verwässern.
- Was für den Filter gilt, gilt danach auch für das Ein- und Ausblenden versteckter Dateien. Zwei Regeln für denselben Vorgang entstehen nicht.

## Recommendation

Möglichkeit 2. Die bestehende Regel ist die richtige, und sie ist in der Runde 6 gegen einen ernstgemeinten Gegenvorschlag verteidigt worden; was ihr fehlt, ist die Auskunft. Möglichkeit 3 nimmt Arbeit weg, was der Baum beim Rechtsklick bereits abgelehnt hat. Möglichkeit 4 macht den Filter zu etwas anderem, als der Nutzer entschieden hat.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1610 — Moeglichkeit 2, wie empfohlen. Eine Operation fasst nur an, was sichtbar ist; die bestehende Regel in `krk-ui/src/kommandos/operationen.rs:162-192` bleibt unveraendert. Die Statuszeile sagt an, wie viele Markierungen der Filter gerade ausblendet, damit das Verschwinden keine Ueberraschung ist.
