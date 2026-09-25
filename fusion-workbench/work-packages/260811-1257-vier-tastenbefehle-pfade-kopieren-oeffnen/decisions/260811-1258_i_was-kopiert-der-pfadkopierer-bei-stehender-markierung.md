# Was kopiert der Pfadkopierer bei stehender Markierung: die markierten Einträge oder den unter der Auswahl?

---
**Domain:** code
**Status:** implemented
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
Answered: **Moeglichkeit 1, `betroffene()` erben**, samt der empfohlenen Rueckmeldung in der
Statuszeile: bei einer Zeile der Pfad, bei mehreren die Zahl der kopierten Pfade. Nutzerantwort
am 260811-1505.

Damit gilt fuer den Pfadkopierer dieselbe Regel wie fuer die vier Dateioperationen — eine Regel
und keine zweite daneben.

**Ein Befund haengt unmittelbar daran und gehoert vor dem Bau geprueft.** Der Playmaker-Lauf vom
260811-1415 hat festgehalten: **die Markierung faellt heute mit jedem Lesevorgang**, weil sie eine
Menge von Eintragsindizes ist und ein Lesevorgang die Indizes neu vergibt. Der Pfadkopierer setzt
genau auf dieser Markierung auf.

Was das fuer diese Antwort heisst, ist **nicht** entschieden und gehoert in den Spec: ob es
genuegt, dass der Kopierer die Markierung nimmt, wie er sie vorfindet — dann kopiert er nach
einem Lesevorgang die Auswahl statt der vorher markierten Zeilen, und der Nutzer merkt es an der
Rueckmeldung —, oder ob die Fluechtigkeit der Markierung ein eigener Gegenstand ist. Das zweite
waere ein anderer Circle. Der Spec sagt, welcher der beiden Faelle gilt, und behauptet nichts
ueber den anderen.

---
Implemented: `d23bfdb` — `DateifensterQuelle::eintragspfad_kopieren`
(`crates/krk-ui/src/appkit/tabelle.rs:904`) ruft `betroffene_eintraege()` und damit
`operationen::betroffene`; eine zweite Regel daneben entsteht nicht. Bei leerer Menge bleibt die
Zwischenablage unberuehrt und die Statuszeile traegt `nichts_zu_kopieren()`.
Gegen den Baum gelesen im Abgleich `history/260811-2157-reconciliation.md`.
