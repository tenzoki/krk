# Was kopiert der Pfadkopierer bei stehender Markierung: die markierten Einträge oder den unter der Auswahl?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** crates/krk-ui/src/kommandos/operationen.rs:157

---

## Question

Der Entwurf nennt den zweiten Befehl "den Pfad des markierten Eintrags kopieren, gleich ob Datei oder Ordner". KRK unterscheidet aber zwei Dinge, die im Alltag beide "markiert" heißen: die Auswahl, also die eine Zeile unter dem Cursor, und die Markierung, also die mit Leertaste gesetzten Einträge, von denen mehrere zugleich stehen können.

Für die vier Dateioperationen ist die Frage längst beantwortet. `betroffene()` (`crates/krk-ui/src/kommandos/operationen.rs:157`) trägt die Regel jedes Zweifensterverwalters an einer Stelle: die Markierung hat den Vorrang, sonst gilt der Eintrag unter der Auswahl, und gezählt werden allein die sichtbaren Einträge in Sichtreihenfolge. Ob der Pfadkopierer diese Regel erbt, ist trotzdem eine echte Wahl, weil Kopieren und Löschen sich in einem Punkt unterscheiden: eine Zwischenablage mit dreißig Zeilen darin sieht man erst beim Einfügen, während eine Löschrückfrage die Zahl vorher nennt.

## Options

1. **`betroffene()` erben.** Stehen Markierungen, kommen deren Pfade in die Zwischenablage, einer je Zeile, in Sichtreihenfolge. Steht keine, kommt der Pfad unter der Auswahl.
   - Pro: eine Regel für alle Befehle, die auf Einträge wirken; kein zweiter Mechanismus daneben; erlaubt das Kopieren einer Dateiliste in einem Griff.
   - Contra: der Nutzer sieht der Zwischenablage nicht an, wie viele Zeilen er gerade erzeugt hat, solange keine Rückmeldung dazukommt.
2. **Immer genau die Zeile unter der Auswahl, Markierungen bleiben unbeachtet.**
   - Pro: das Ergebnis ist immer eine Zeile und immer vorhersagbar.
   - Contra: bricht die Regel "Markierung vor Auswahl", die in KRK sonst überall gilt, und der Nutzer muss sich merken, wo sie gilt und wo nicht.
3. **Wie 1, aber bei stehender Markierung erst eine Rückfrage**, die die Zahl der Pfade nennt.
   - Pro: keine unbemerkte Zwischenablage mit dreißig Zeilen.
   - Contra: eine Rückfrage für einen Befehl, der nichts zerstört. C4 hebt Rückfragen für die verlustbehafteten Operationen auf; eine hier entwertet sie dort.

## Constraints

- Ein zweiter Weg neben `betroffene()` wäre der doppelte Mechanismus, den `critical-stance.md` §2 ausschließt.
- Die Zwischenablage ist heute reine Quelle (`crates/krk-ui/src/appkit/zwischenablage.rs`, Modulkopf). Wie geschrieben wird, entscheidet die Planung; **was** hineinkommt, entscheidet diese Frage.

## Recommendation

Option 1, zusammen mit einer kurzen Rückmeldung in der Statuszeile, die bei einer Zeile den Pfad nennt und bei mehreren die Zahl der kopierten Pfade. Das hält die eine Regel und beantwortet zugleich den Einwand gegen sie, ohne eine Rückfrage einzuführen.

---
Answered:
Implemented:
Deferred:
Superseded by:
