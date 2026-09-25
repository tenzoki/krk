# Wie heißt die Ausgabedatei, und was geschieht, wenn sie schon da ist?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `circles/260809-2040-tastenbelegung-als-markdown-in-downloads/_a_circle.md` (Directive und Grounding, Abschnitt `### Befund zum Downloads-Ordner`), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260803-2025_i_wie-zeigt-krk-dem-nutzer-fehler.md` (die Statuszeile als Meldeweg)

---

## Question

Die Ausgabe landet in einem Ordner, der dem Nutzer gehört und in dem er selbst Dateien ablegt. Zwei Fragen hängen zusammen und werden deshalb in einem Datensatz gestellt: unter welchem Namen die Datei entsteht, und was ein zweiter Aufruf mit der Datei des ersten macht. Wer den Namen fest wählt, muss das Überschreiben beantworten; wer das Überschreiben vermeiden will, braucht einen veränderlichen Namen. Die Frage gehört vor den ersten Planschritt, weil das Abnahmekriterium der Fähigkeit an ihr hängt.

Der Downloads-Ordner unterscheidet sich dabei von der Ablage unter `~/Library/Application Support/KRK/`: dort schreibt KRK vier Dateien, deren Namen es selbst vergeben hat und deren einziger Schreiber es ist. Im Downloads-Ordner kann eine Datei desselben Namens von irgendwoher stammen.

## Options

1. **Fester Name, vorhandene Datei wird überschrieben.** Etwa `KRK-Tastenbelegung.md`.
   - Pro: der Nutzer weiß immer, wo die aktuelle Fassung liegt, und der Ordner füllt sich nicht. Wer die Datei versioniert, hat einen stabilen Pfad, und genau das war einer der vier Gründe des Nutzers für Markdown.
   - Contra: KRK überschreibt ohne Rückfrage eine Datei, die es nicht angelegt haben muss. Der Name ist unwahrscheinlich, aber nicht unmöglich.
2. **Fester Name mit Zeitstempel**, etwa `KRK-Tastenbelegung-260809-2040.md`.
   - Pro: keine vorhandene Datei wird je berührt, und die Ausgaben lassen sich der Reihe nach vergleichen. Das Muster ist im Projekt vertraut, die Messberichte unter `messungen/` tragen es.
   - Contra: der Ordner sammelt Dateien, die niemand aufräumt, und der Pfad ist nicht stabil. Wer die Ausgabe versioniert, muss bei jedem Lauf umbenennen.
3. **Fester Name, bei Kollision ein Zähler**, etwa `KRK-Tastenbelegung 2.md`.
   - Pro: nichts wird überschrieben, und im gewöhnlichen Fall entsteht der einfache Name. Es ist das Verhalten, das macOS beim Laden zeigt, also für den Downloads-Ordner das erwartete.
   - Contra: nach dem dritten Lauf liegen drei Dateien da, und welche die neueste ist, sagt der Name nicht. Der Zähler braucht eine Schleife und eine Grenze, also die meiste Mechanik der drei.
4. **Fester Name, vorhandene Datei bleibt stehen, KRK meldet es und schreibt nicht.**
   - Pro: die vorsichtigste Antwort, und sie kostet am wenigsten Code.
   - Contra: der zweite Aufruf tut nichts Sichtbares außer einer Meldung, und der Nutzer muss von Hand aufräumen, bevor er eine frische Ausgabe bekommt. Für eine Funktion, deren Zweck die aktuelle Fassung ist, ist das die falsche Vorgabe.

## Constraints

- Der Zielordner ist der Downloads-Ordner des Nutzers, festgelegt am 260809-2035. Er wird über `pfade::benutzerverzeichnis()` aufgelöst, die eine Stelle im Kern, die nach dem Benutzerverzeichnis fragt.
- Gelingen und Scheitern melden sich über die Statuszeile mit ihren fünf Rängen. Ein eigenes Blatt oder eine eigene Meldeform entsteht nicht.
- Der Ordner kann fehlen, und der Zugriff kann vom System abgelehnt werden. Jede Antwort braucht ein Verhalten für beide Fälle; welche Meldung dabei erscheint, folgt aus dem Punkt darüber.

## Recommendation

**Wir empfehlen Möglichkeit 1**, den festen Namen mit Überschreiben. Die Ausgabe zeigt einen Stand, der sich mit jeder Umbelegung ändert; eine alte Fassung daneben ist in aller Regel keine Aufbewahrung, sondern Abfall. Der stabile Pfad bedient außerdem genau den Grund, aus dem der Nutzer Markdown gewählt hat, nämlich die Versionierbarkeit: ein Git-Repository will denselben Dateinamen wiedersehen.

Der Einwand gegen Möglichkeit 1, KRK könne eine fremde Datei überschreiben, lässt sich billig entschärfen, ohne zu Möglichkeit 3 zu wechseln: ein Name, der KRK nennt, kollidiert praktisch nicht, und die Meldung in der Statuszeile nennt den geschriebenen Pfad, sodass der Nutzer sofort sieht, was entstanden ist.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: **Moeglichkeit 1, fester Name und eine vorhandene Datei wird ueberschrieben.**
Nutzerantwort am 260811-0110, festgehalten in `history/260811-0107-orchestrator-session.md`.

Der Name ist damit stabil und verlinkbar, und es gibt zu jedem Zeitpunkt genau eine Datei. Der
konkrete Name ist in dieser Antwort nicht festgelegt; der Datensatz nennt `KRK-Tastenbelegung.md`
als Beispiel, und der Spec legt ihn fest.

**Der Preis steht ausdruecklich dabei:** ein zweiter Aufruf zerstoert kommentarlos, was vorher
unter diesem Namen lag — auch dann, wenn es nicht von KRK stammte. Der Downloads-Ordner gehoert
dem Nutzer, und KRK ist dort nicht der einzige Schreiber; das unterscheidet ihn von der Ablage
unter `~/Library/Application Support/KRK/`, wo KRK seine vier Dateien selbst benannt hat und
allein beschreibt. Der Nutzer hat das gewaehlt, nachdem der Preis benannt war.

Ob die Ueberschreibung gemeldet wird, ist damit nicht entschieden und gehoert in den Spec.

---
Implemented: `fd863e3` — `belegungsausgabe::ausgeben` (`crates/krk-ui/src/belegungsausgabe.rs`) haengt
`Downloads/KRK-Tastenbelegung.md` an `krk_core::ablage::pfade::benutzerverzeichnis()` und
schreibt ueber `krk_core::ablage::atomar::schreiben`, also erst in eine Nachbardatei und dann
mit `rename`. Eine vorhandene Datei wird ohne Rueckfrage ersetzt; eine Probe des Moduls misst
das in einem Pruefordner. Am Baum geprueft am 260811-1403.
