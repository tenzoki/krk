# Was tut `esc` in einer geänderten Zelle der Eintragstabellen?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md, 260926-0107-zweitlesung-plan-f2-krkhome.md, 260926-0112_*_was-tut-return-in-einer-notizzelle-und-welche-tasten-tragen-die-editoren.md

---

## Question

In einer Zelle der Aufgaben- oder der Notiztabelle steht der getippte Text allein im Feldeditor, bis die Zelle übernommen wird. `esc` beendet die Bearbeitung heute nach der Mac-Konvention mit Verwerfen (`abortEditing`), und der Feldeditor nimmt seinen eigenen Rückgängig-Verlauf dabei mit. Seit `return` in der Notizzelle einen Zeilenumbruch schreibt (`260926-0112_*_was-tut-return-in-einer-notizzelle-…`), kann eine Notizzelle mehrere Absätze tragen, und ein `esc` verlöre sie ohne Meldung. Das verletzt den Constraint des Spec „Nichts, was der Nutzer geschrieben hat, verschwindet ohne Meldung“. Die Zweitlesung des Plans verlangt deshalb eine Regel und überlässt die Wahl dem Nutzer.

Die Regel gilt für beide Tabellen und in Stufe 5 auch für die Tabelle von `.secrets.txt`; sie legt fest, wie KRK eine Zellenbearbeitung abbrechen lässt, und bindet damit jede spätere Tabelle mit bearbeitbaren Zellen.

## Options

1. **`esc` übernimmt eine geänderte Zelle, in beiden Tabellen, und verlässt eine unveränderte.** Die Übernahme ist ein Umbau im Verwalter des Fensters, also nimmt `cmd+z` sie zurück.
   - Pros: eine Regel für jede Zelle, nichts geht verloren, der Rückweg ist der gewohnte.
   - Cons: `esc` heißt in keiner Zelle mehr „verwerfen“; wer sich in einer Aufgabe vertippt und `esc` drückt, bekommt die vertippte Fassung und muss `cmd+z` nachschieben.
2. **`esc` verwirft in der Aufgabenzelle und übernimmt in einer geänderten Notizzelle.** Die Trennlinie ist eine Eigenschaft der Zelle: wo sie Zeilenumbrüche trägt, ist der verlorene Text womöglich lang, und dort übernimmt `esc`; wo sie einzeilig ist, sieht der Nutzer nach dem Verwerfen den alten Text wieder an derselben Stelle.
   - Pros: die Aufgabenzelle folgt der Mac-Konvention, die Notizzelle verliert nichts; die Statuszeile sagt nach der Übernahme „übernommen, `cmd+z` nimmt es zurück“.
   - Cons: dieselbe Taste tut in zwei Tabellen desselben Editors Verschiedenes.
3. **`esc` verwirft in der Aufgabenzelle; in einer geänderten Notizzelle tut es nichts und sagt in der Statuszeile, dass `cmd+return` übernimmt und ein Klick daneben ebenfalls.**
   - Pros: `esc` übernimmt nie etwas, das der Nutzer nicht bestätigt hat.
   - Cons: eine Taste ohne Wirkung; wer wirklich verwerfen will, hat in der Notizzelle keinen Weg außer Übernehmen und `cmd+z`.

## Constraints

- Nichts, was der Nutzer geschrieben hat, verschwindet ohne Meldung (Spec, Constraints).
- Eine unveränderte Zelle verlässt `esc` in jeder Möglichkeit ohne Umbau und ohne Abweichungsmarke.
- `esc` wird in einer Zelle erkannt, bevor der dritte Rang von `abbrechen` den Filtertext leert (Plan, Schritt 3.2b).

## Recommendation

Wir empfehlen Möglichkeit 2. Sie hält den Constraint dort, wo er bedroht ist, und lässt die Konvention dort stehen, wo Verwerfen nichts kostet, was der Nutzer nicht vor Augen hat. Die Trennlinie hängt an der Eigenschaft „die Zelle trägt Umbrüche“ und nicht an einem Dateinamen, und sie steht an genau einer Stelle, dem Rang für die Zelle in `abbrechen`. Möglichkeit 1 ist die einfachere Regel und kostet in der Aufgabenzelle einen Tastendruck mehr; wählt der Nutzer sie, entfällt im Plan ein Zweig. Der Plan baut Schritt 3.2b auf Möglichkeit 2 und nennt dort, was die anderen ändern.

---
Answered: dieser Datensatz `## Recommendation` — Möglichkeit 2, esc verwirft in der Aufgabenzelle und übernimmt eine geänderte Notizzelle; ruled by user, Kai Stalmann <kai@stalmann.org>

---
Implemented: e6f5b2e — esc verwirft in der Aufgabenzelle; die Notizzelle übernimmt in d356ca6
