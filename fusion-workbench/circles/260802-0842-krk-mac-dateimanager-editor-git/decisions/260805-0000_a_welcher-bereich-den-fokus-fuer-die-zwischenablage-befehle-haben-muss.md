# Welcher Bereich muss den Fokus haben, damit die beiden Befehle aus C10 greifen?

---
**Domain:** code
**Status:** answered
**Filed by:** ontocoder (als Defekt), umgetragen vom planner am 260805-0000
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260804-0907_c_c10-sagt-nicht-welcher-bereich-den-fokus-haben-muss.md`, `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` (C4, C5, C10), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` (S13, S18, S19)

---

## Question

Die beiden Funktionen aus C10 liegen seit S9b als gewöhnliche Belegungen in `resources/default-keymap.toml`: `zwischenablage_ansehen` auf `shift+f3` und `zwischenablage_springen` auf `opt+cmd+g`. Der Tastenabgriff aus S7 reicht jeden Tastendruck weiter, gleich welcher Bereich den Eingabefokus hat, sofern er nicht in einem Textfeld steht. C10 sagt, "das aktive Dateifenster" wechsle in den Ordner, sagt aber nicht, was gilt, wenn der Fokus in der Lesezeichen- und Geräteleiste oder im Vorschaufenster liegt. C5 hat dieselbe Frage für seine eigenen Funktionen beantwortet, C10 nicht. Gebraucht wird die Antwort in S19, das die Vorschau der Zwischenablage baut, und sie wird mit S18 überhaupt erst greifbar, weil dort der zweite fokussierbare Bereich entsteht.

## Options

1. **Beide Befehle wirken nur bei Fokus im Dateifenster.** Derselbe Vorbehalt, den C4 für die Löschtasten stellt.
   - Pros: eine Regel für beide; der Sprung ändert das Dateifenster und braucht dessen Fokus; keine Sonderregel für C10.
   - Cons: das Ansehen wäre für sich genommen auch ohne diesen Vorbehalt eindeutig, weil es den aktiven Vorschau-Tab füllt.
2. **Beide Befehle wirken überall außerhalb eines Textfeldes.** Der Sprung trifft dann das zuletzt aktive Dateifenster.
   - Pros: von jedem Bereich aus erreichbar.
   - Cons: "das aktive Dateifenster" ist bei Fokus in der Leiste eine Auslegungsfrage; es entstünde ein Begriff "zuletzt aktiv", den kein anderer Befehl braucht.
3. **Getrennte Antworten je Befehl.** Das Ansehen wirkt überall, der Sprung nur im Dateifenster.
   - Pros: jeder Befehl bekommt die Bedingung, die er wirklich braucht.
   - Cons: zwei Regeln für zwei Befehle derselben Fähigkeit, also genau die Sonderregel, die die Maxime "supersimpel" ausschließt.

## Constraints

- C2 verlangt, dass ein Tastenbefehl dann und nur dann wirkt, wenn der Eingabefokus in einem Dateifenster oder in der Lesezeichenleiste steht.
- C4 stellt denselben Vorbehalt namentlich für die Löschtasten.
- Der Fokusvorbehalt aus S13 unterscheidet heute allein Textfeld von Nicht-Textfeld; feiner unterscheidet nur die Abfrage, die S16 für die Löschtasten geschrieben hat.

## Recommendation

Möglichkeit 1.

---
Answered: Nutzer am 260805-0000 — Möglichkeit 1. Begründung des Nutzers: der Sprung ändert das Dateifenster, also braucht er dessen Fokus; keine Sonderregel für C10.

**Was daraus folgt, und warum es keinen vierten Sonderfall gibt.** Bis heute gibt es genau einen fokussierbaren Bereich, das Dateifenster, und genau eine Funktion, die danach fragt: S16 lässt die Löschtasten nur wirken, wenn der Eingabefokus in einem Dateifenster steht, und fragt das an ihrer Aufrufstelle ab. Mit der Lesezeichenleiste aus S18 wird die Frage für jedes Kommando fällig, und zwei weitere handgeschriebene Abfragen wären der Anfang eines Dickichts. `Kommando` bekommt deshalb in S18 eine Eigenschaft `Wirkungsbereich` mit den Werten `Dateifenster`, `Leiste` und `Ueberall`, die Zuleitung fragt sie **einmal** vor dem Ausführen, und die Abfrage aus S16 geht darin auf. S19 trägt dann für die beiden C10-Befehle nur noch den Wert `Dateifenster` ein und schreibt keine Abfrage.

Die Eigenschaft gehört in den Kern, neben `Kommando`: dass ein Befehl das Dateifenster braucht, ist eine Aussage über den Befehl und ohne Fenster prüfbar. Welcher Bereich den Fokus gerade hat, weiß allein `krk-ui`. Die Aufrufrichtung bleibt damit von oben nach unten, und eine zweite Abhängigkeitsumkehr neben der Papierkorb-Schnittstelle entsteht nicht.

Eingearbeitet: `planning/260802-1036_o_spec-navigator-geruest.md` C10 (neues Abnahmekriterium, eine Festlegung); `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` bei S18 (die Eigenschaft) und S19 (die zwei Werte, ein Abnahmekriterium, die neue Abhängigkeit S18 → S19).
Implemented: <offen — S18 und S19>
