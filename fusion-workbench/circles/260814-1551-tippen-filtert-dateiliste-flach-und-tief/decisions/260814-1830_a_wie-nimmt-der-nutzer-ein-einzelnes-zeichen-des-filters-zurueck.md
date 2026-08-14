# Wie nimmt der Nutzer ein einzelnes Zeichen des Filters zurück?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `resources/default-keymap.toml:156-158` (`in_papierkorb`, belegt mit `delete` und `cmd+delete`); `crates/krk-ui/src/belegungsmodell.rs:628-631` (die Tippsuche der Runde 7, die eine Rücktaste hat, samt der Begründung, warum die Dateiliste keine hat); `crates/krk-core/src/verzeichnis/sprungmarke.rs:33-35` (die Sekundenregel, die mit dem Filter wegfällt)

---

## Question

Der Filtertext steht, bis der Nutzer ihn löscht; die Sekundenregel der Sprungmarke fällt mit ihr weg. Damit entsteht eine Lücke, die es bei der Sprungmarke nicht gab: ein Vertipper im vierten Zeichen ließ sich bisher aussitzen, weil der Puffer nach einer Sekunde von selbst leer war. Ein stehender Filtertext tut das nicht.

Die Tippsuche der Belegungsansicht aus der Runde 7 hat für genau diesen Fall eine Rücktaste, und ihr Modulkopf schreibt aus, warum die Dateiliste keine hat: dort ist die Rückschritt-Taste an das Räumen in den Papierkorb vergeben (`delete` und `cmd+delete`). Ein Nutzer, der im Dateifenster ein Zeichen zurücknehmen will, hat heute keinen Weg dafür, und die einzige Rücknahme ist `Esc`, das den ganzen Text löscht.

## Options

1. **Kein Weg für ein einzelnes Zeichen. `Esc` löscht den ganzen Filtertext, und der Nutzer tippt neu.**
   - Pro: keine Belegung ändert sich, kein Befehl kommt hinzu, keine Taste bekommt eine zweite Bedeutung. Bei einem Filter von drei oder vier Zeichen kostet der Neuanfang wenige Anschläge.
   - Kontra: die Dateiliste bleibt hinter der Tippsuche der Belegungsansicht zurück, und das an einer Stelle, an der der Nutzer dieselbe Handlung erwartet. Bei einem längeren Filtertext ist der Neuanfang spürbar.
2. **Die Rückschritt-Taste nimmt ein Zeichen zurück, solange ein Filtertext steht, und räumt sonst in den Papierkorb.**
   - Pro: die vertraute Taste an der Stelle, an der der Nutzer sie sucht, ohne eine neue Kombination.
   - Kontra: eine Taste, deren Bedeutung vom Zustand abhängt, und ausgerechnet eine, deren andere Bedeutung Dateien wegräumt. Wer den stehenden Filter übersieht, drückt `delete` in der Erwartung zu löschen und ändert stattdessen die Liste; wer ihn im umgekehrten Fall übersieht, räumt eine Datei weg. Die Belegungsansicht und das Hauptmenü zeigen je Befehl eine Bedeutung, und eine zweite passt dort nicht hinein.
3. **Eine eigene, heute freie Kombination**, etwa `shift+delete`.
   - Pro: eine Bedeutung je Taste, sauber in Belegungsansicht, Hauptmenü und Markdown-Ausgabe zu führen; `shift+delete` ist am 260814 in `resources/default-keymap.toml` unbelegt.
   - Kontra: die 85. Zeile der Belegung für eine Handlung, die auf jeder anderen Tastatur der Welt auf der Rückschritt-Taste liegt. Der Nutzer muss sie lernen.

## Constraints

- `delete` und `cmd+delete` tragen `in_papierkorb`, `opt+cmd+delete` trägt `endgueltig_loeschen`, `ctrl+delete` trägt das Löschen in der Lesezeichenleiste. Am 260814 gegen `resources/default-keymap.toml` gezählt; frei ist unter den Rückschritt-Kombinationen allein `shift+delete`.
- Der Wirkungsbereich trennt die Fälle nicht: das Räumen in den Papierkorb und der Filter wirken beide im Dateifenster.
- Jede Antwort außer Möglichkeit 1 bringt eine Zeile in `Kommando::wirkungsbereich`, eine in `bereich_des_kommandos` und einen Platz im Hauptmenü mit sich.
- `Esc` löscht den ganzen Filtertext, gleich welche Antwort fällt.

## Recommendation

Möglichkeit 1 für diese Runde, und die Frage bleibt danach offen. Möglichkeit 2 hängt eine zweite Bedeutung an die destruktivste Taste des Dateifensters, und dieses Projekt hat eine zustandsabhängige Tastenbedeutung schon einmal aus diesem Grund verworfen. Möglichkeit 3 ist die saubere Lösung und lässt sich jederzeit nachtragen, ohne dass etwas Gebautes umzieht; sie jetzt zu setzen, hieße eine Kombination zu vergeben, bevor der Nutzer am laufenden Bündel gemerkt hat, ob er sie vermisst.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: Nutzer am 260814-1845 im Orchestrator-Dialog — die Ruecktaste nimmt ein Zeichen zurueck, **solange ein Filtertext steht**. Damit folgt das Dateifenster der Belegungsansicht aus der Runde 7, die `letztes_zeichen_weg` schon so bedient (`crates/krk-ui/src/belegungsmodell.rs`), und die Empfehlung des Datensatzes ("in dieser Runde gar nicht") ist verworfen.

**Die Fallunterscheidung ist sicherheitsrelevant und nicht nur eine Bequemlichkeit.** `delete` traegt im Dateifenster heute "In den Papierkorb raeumen" (`resources/default-keymap.toml:156-158`, zusammen mit `cmd+delete`). Ohne die Unterscheidung raeumte ein Nutzer, der einen Vertipper im Filter korrigieren will, Dateien weg. Die Regel lautet deshalb: steht ein Filtertext, nimmt die nackte Ruecktaste ein Zeichen davon zurueck und erreicht `in_papierkorb` nicht; steht keiner, wirkt sie wie bisher. `cmd+delete` bleibt in jeder Lage das Raeumen, damit ein Weg zum Papierkorb auch bei stehendem Filter offen ist.

Der Preis ist benannt und angenommen: eine Taste, deren Bedeutung vom Zustand abhaengt, ist in diesem Baum bisher die Ausnahme. Die Runde 7 hat mit `kommandos::zulaessigkeit` die Stelle gebaut, an der eine solche Frage einmal beantwortet und von zwei Fragern gelesen wird; der Planner entscheidet, ob die Regel dort hingehoert.
