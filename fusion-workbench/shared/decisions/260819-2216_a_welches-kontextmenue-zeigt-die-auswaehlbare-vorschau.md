# Welches Kontextmenü zeigt die auswählbare Vorschau?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper
**Cross-references:** `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/decisions/260812-1000_*_an-welchen-drei-flaechen-haengt-das-neue-kontextmenue.md`; `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C3); `crates/krk-ui/src/appkit/vorschau.rs:393-415` (`textView:menu:forEvent:atIndex:`), `:538-567` (die zweite Anschlussart für Bild und Inhaltsfläche)

---

## Question

Seit C1 der Runde 6 hängt an allen drei Ansichten der Vorschau ein Kontextmenü mit dem Teilen-Eintrag. Die Textanzeige geht dabei den Weg über `textView:menu:forEvent:atIndex:`: AppKit baut sein Menü, KRK hängt den Eintrag an. Solange die Fläche nicht auswählbar ist, hat AppKit dort wenig anzubieten. Mit einer auswählbaren Fläche bringt AppKit seine eigenen Einträge mit, Kopieren, Nachschlagen, Suchen und Sprachausgabe. Zu entscheiden ist, was der Nutzer im Menü sieht.

## Options

1. **AppKits Menü unverändert übernehmen**, mit dem Teilen-Eintrag daneben.
   - Folge: die Vorschau verhält sich wie der Editor, an dem der Teilen-Eintrag seit der Runde 6 ebenfalls neben AppKits Einträge tritt. Kostet keine Zeile: die Methode steht schon und gibt das Menü zurück, das sie bekommen hat.
   - Preis: was darin steht, entscheidet macOS und nicht KRK. Eine spätere Systemfassung kann Einträge hinzufügen, und KRK erfährt davon nichts.

2. **Das Menü auf wenige Einträge beschneiden**, etwa Kopieren, Alles auswählen und Teilen.
   - Folge: KRK bestimmt, was dort steht.
   - Preis: eine Liste erlaubter Einträge, die mit jeder macOS-Fassung nachzuziehen ist, und ein Verhalten, das Nutzer von einer Textansicht nicht erwarten. Der Baum hat eine solche Liste nirgends.

3. **Das Kontextmenü der Textanzeige ganz abschalten** und nur den Teilen-Eintrag zeigen, wie an Bild und Inhaltsfläche.
   - Folge: eine Anschlussart weniger.
   - Preis: das Kopieren wäre nur noch über die Tastatur erreichbar. Wer mit der Maus markiert, greift danach zur Maus und nicht zur Tastatur.

## Constraints

- Der Teilen-Eintrag der Runde 6 bleibt an allen drei Ansichten, und das Menü wird weiterhin an genau einer Stelle gebaut.
- Kopiert der Nutzer über das Kontextmenü, gilt dieselbe Zusage wie für `cmd+c`: bei gerendertem Markdown der Quelltext.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie ist die einzige, die keine Zeile kostet, und sie hält die Gleichheit mit dem Editor.

## Antwort 260819-2210

**Möglichkeit 1.** Wörtlich: „AppKits Menü unverändert übernehmen."

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented:
Deferred:
Superseded by:
