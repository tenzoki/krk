# Wie weit reicht "im Stapel umbenennen" in der ersten Runde?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitt C4, `circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`

---

## Question

Die Circle-Directive nennt "im Stapel umbenennen" als Dateioperation, legt den Umfang aber nicht fest. Zwischen den beiden möglichen Lesarten liegt ein erheblicher Unterschied im Aufwand. Ein Umbenennen ohne Musterregeln fragt für jeden markierten Eintrag nacheinander den neuen Namen ab und ist wenig mehr als eine Wiederholung des Einzelfalls. Ein Umbenennen mit Musterregeln ist ein eigenes kleines Werkzeug: es braucht eine Regelsprache für Suchen und Ersetzen im Namen, für fortlaufende Nummerierung und für Groß- und Kleinschreibung, dazu eine Vorschau der Ergebnisse vor der Ausführung und einen Umgang mit Namenskonflikten, die erst durch die Regel entstehen. Die Frage gehört vor den Plan, weil die zweite Lesart die Fähigkeit C4 spürbar vergrößert und sich später nur mit Nacharbeit ergänzen lässt.

## Options

1. **Ohne Musterregeln** — KRK fragt für jeden markierten Eintrag nacheinander den neuen Namen ab, mit einem Weg zum Überspringen und zum Abbrechen.
   - Pro: klein, in einer Runde sicher fertig, keine neue Regelsprache. Entspricht der Maxime "supersimpel".
   - Contra: bei fünfzig Fotos, die eine gemeinsame Vorsilbe bekommen sollen, ist das fünfzigmal tippen. Genau dafür benutzen Norton- und ForkLift-Nutzer die Stapelumbenennung.

2. **Mit Musterregeln, in dieser Runde** — Suchen und Ersetzen im Namen, fortlaufende Nummerierung mit wählbarer Stellenzahl, Groß- und Kleinschreibung, dazu eine Vorschau der neuen Namen vor der Ausführung.
   - Pro: deckt den Anwendungsfall ab, für den die Funktion in den Vorbildern existiert.
   - Contra: eine eigene Regelsprache, eine eigene Vorschau und ein eigener Umgang mit Konflikten. Der größte Einzelposten in C4.

3. **Ohne Musterregeln in dieser Runde, mit Musterregeln als eigener Circle** — die einfache Form kommt jetzt, das Werkzeug später.
   - Pro: die Runde bleibt auf das Navigator-Gerüst konzentriert. Das Werkzeug bekommt einen eigenen Zuschnitt und eine eigene Klärung.
   - Contra: die einfache Form wird von der späteren wahrscheinlich vollständig ersetzt, die Arbeit daran ist dann verloren.

## Constraints

- Die Antwort muss auch für Ordner tragen, nicht nur für Dateien.
- Bei einem Namenskonflikt gilt die Festlegung aus C4: KRK fragt einmal nach, mit einer Option für alle weiteren Fälle.
- Vollständige Bedienbarkeit über die Tastatur gilt hier wie überall (C2).
- Suchen und Ersetzen über mehrere Dateien liegt ausdrücklich außerhalb des Circles. Ein Suchen und Ersetzen im Dateinamen ist davon nicht betroffen; die Abgrenzung meint Dateiinhalte.

## Recommendation

Möglichkeit 2, sofern der Nutzer die Stapelumbenennung im Alltag tatsächlich für Serien verwendet. Der Aufwand liegt fast vollständig in der Vorschau, und die Vorschau ist zugleich das, was die Funktion ungefährlich macht. Möglichkeit 3 kostet die einfache Form zweimal. Die Abwägung stützt sich auf den Gebrauch in den beiden genannten Vorbildern, nicht auf eine geprüfte Aussage über die Arbeitsweise des Nutzers; die Entscheidung liegt bei ihm.

## Antwort des Nutzers

Der Nutzer hat am 260802-1105 Möglichkeit 2 gewählt: Umbenennen im Stapel mit Musterregeln und Vorschau, in dieser Runde. Genannt hat er Suchen und Ersetzen im Namen, fortlaufende Nummerierung und eine Vorschau vor der Ausführung.

Die Groß- und Kleinschreibung, die Möglichkeit 2 des Datensatzes zusätzlich aufführte, hat der Nutzer nicht genannt. Der Spec führt sie deshalb nicht als Zusage und nennt sie ausdrücklich als nicht in dieser Runde enthalten. Eine Umschaltung der Schreibweise lässt sich über Suchen und Ersetzen nicht ausdrücken, wäre also eine eigene Regelart; sie ohne Auftrag mitzunehmen, wäre eine Ausweitung des Umfangs.

---
Answered: `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md`:142-145 — Möglichkeit 2 gewählt: Musterregeln für Suchen und Ersetzen sowie fortlaufende Nummerierung, mit Vorschau vor der Ausführung; Groß- und Kleinschreibung nicht enthalten.
Implemented:
Deferred:
Superseded by:
