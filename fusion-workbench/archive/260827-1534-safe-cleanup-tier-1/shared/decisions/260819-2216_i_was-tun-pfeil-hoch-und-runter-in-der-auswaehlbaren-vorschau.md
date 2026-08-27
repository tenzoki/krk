# Was tun Pfeil hoch und Pfeil runter in der auswählbaren Vorschau?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper
**Cross-references:** `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md` (C1.10, C1.11); `crates/krk-core/src/tasten/belegung.rs` (`fn wirkungsbereich`, `Wirkungsbereich::Navigator`); `crates/krk-ui/src/appkit/anwendung.rs:3168-3205` (`bereichskommando`)

---

## Question

Mit dem Fokus in der Vorschau sind Pfeil hoch und Pfeil runter heute wirkungslos. Beide tragen `Wirkungsbereich::Navigator`, der die Vorschau einschließt; sie sind damit zulässig, `bereichskommando` reicht sie an die Vorschau, und die führt allein die vier Tabbefehle aus. Geschluckt wird seit der Runde 7, was zulässig war, und nicht, was gewirkt hat. Der Tastendruck ist verbraucht, in der Vorschau geschieht nichts, und die Auswahl im Dateifenster bewegt sich nicht.

Mit einer auswählbaren Textfläche stellt sich die Frage neu, weil AppKit die beiden Tasten von nun an beantworten könnte: sie bewegten die Schreibmarke und blätterten den Text.

**Beim Ausarbeiten des Specs ist eine Ungleichheit dazugekommen, die in der ersten Klärungsrunde nicht vorlag.** Bild-auf, Bild-ab, Pos1 und Ende tragen `Wirkungsbereich::Dateifenster`, sind mit dem Fokus in der Vorschau **unzulässig** und laufen deshalb unverändert an AppKit weiter. Sobald dort eine auswählbare Fläche steht, blättern sie. Die Pfeiltasten bleiben wirkungslos, die Blättertasten blättern, und der Unterschied ist am Bündel nicht zu erklären.

## Options

1. **Alles bleibt, wie es ist.** Pfeil hoch und Pfeil runter bleiben zulässig, werden verbraucht und wirken in der Vorschau nicht.
   - Folge: keine Zeile Code. Die Wirkungsbereiche bleiben unangetastet.
   - Preis: die Ungleichheit zu den vier Blättertasten steht am Bündel und ist nicht erklärbar. Wer in der Vorschau liest, blättert mit Bild-ab und nicht mit dem Pfeil.

2. **Die Vorschau führt die beiden Befehle aus und blättert um eine Zeile.** `Vorschaufenster::kommando_ausfuehren` bekommt zwei Zweige neben den vier Tabbefehlen.
   - Folge: die Pfeiltasten tun in der Vorschau, was sie im Dateifenster tun, nämlich eine Zeile weiterrücken. Kein Wirkungsbereich ändert sich, keine Aufzählung wächst.
   - Preis: die Vorschau bekommt zwei ausgeführte Befehle mehr, und „die Auswahl einen Eintrag nach unten" heißt dort etwas anderes als in der Liste. Der Name des Befehls passt nicht mehr zu seiner Wirkung.

3. **Die beiden Befehle werden mit dem Fokus in der Vorschau unzulässig** und laufen wie die vier Blättertasten an AppKit weiter.
   - Folge: die Vorschau verhält sich wie ein gewöhnlicher Textbetrachter, mit allen sechs Tasten.
   - Preis: `Wirkungsbereich::Navigator` ist eigens positiv gefasst und schließt die Vorschau ein; die Vorschau herauszunehmen hieße, entweder einen achten Wert anzulegen oder die Bedeutung des vorhandenen zu ändern. Beides berührt eine der vier Aufzählungen, die dieses Projekt schmal hält.

## Constraints

- Keine der vier gewachsenen Aufzählungen soll in dieser Runde wachsen.
- Das Verhalten der Pfeiltasten im Dateifenster, in der Leiste und im Editor bleibt unberührt.

## Recommendation

**Wir empfehlen Möglichkeit 1**, weil der Nutzer sie bereits gewählt hat, und legen die Ungleichheit zu den vier Blättertasten allein deshalb vor, weil sie in der Optionsbeschreibung der ersten Runde nicht stand. Wer sie auflösen will, ohne eine Aufzählung anzufassen, nimmt Möglichkeit 2.

## Antwort 260819-2210

**Möglichkeit 1.** Wörtlich: „Bleibt so."

Die Antwort ist auf die Frage nach den Pfeiltasten gegeben worden und nicht auf die Ungleichheit zu den Blättertasten; jene ist erst beim Ausarbeiten entstanden und liegt als offene Frage dieses Datensatzes weiter beim Nutzer.

---
Answered: dieser Datensatz, Abschnitt `## Antwort` — Klärungsrunden des Orchestrators mit dem Nutzer am 260819; Sitzungsprotokoll `shared/history/260819-2026-orchestrator-session.md`. Ausformuliert im Spec `shared/planning/260819-2216_*_spec-auswahl-und-kopieren-in-der-vorschau.md`.
Implemented: `6531f38` — "Bleibt so" haengt daran, dass die Vorschauflaeche im Ereignisabgriff angemeldet ist; `Anwendungsdelegierter::ist_eigene_textflaeche` (`crates/krk-ui/src/appkit/anwendung.rs:2402`) vergleicht seither zwei Flaechen statt einer. Damit bleiben `AuswahlHoch` und `AuswahlRunter` mit `Fokus::Vorschau` zulaessig, werden entgegengenommen und erreichen AppKit nicht. Probe: `crates/krk-ui/src/kommandos/zulaessigkeit.rs:846`. Abgeglichen am 260820-0834.
Deferred:
Superseded by:
