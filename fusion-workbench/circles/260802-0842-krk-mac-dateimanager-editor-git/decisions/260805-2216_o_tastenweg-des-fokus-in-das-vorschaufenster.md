# Wie erreicht der Eingabefokus das Vorschaufenster über die Tastatur?

---
**Domain:** code
**Status:** open
**Filed by:** coder (bei der Umsetzung von S19)
**Cross-references:** `planning/260802-1428_*_plan-navigator-geruest-runde-1.md` `#### 19.`, `planning/260802-1036_*_spec-navigator-geruest.md` `### C6`, `### C10`

---

## Frage

Seit S19 ist das Vorschaufenster der dritte fokussierbare Bereich: C10 setzt
voraus, dass der Fokus dort stehen kann ("Liegt er … im Vorschaufenster, lösen
sie nichts aus"), und die Auslieferungsbelegung sagt seit S9, dass die vier
Tabbefehle aus C1 "auf den Bereich wirken, der den Eingabefokus hat" — nach C6
also auch auf die Vorschau-Tabs. Gebaut ist in S19 der Mausweg: ein Klick in
die Inhaltsfläche der Vorschau macht sie zum Ersthelfer, und cmd+t, cmd+w,
ctrl+tab und ctrl+shift+tab bedienen dann die Vorschau-Tabs.

**Einen Tastenweg dorthin gibt es nicht.** Die Leiste hat seit S18 die beiden
Befehle `fokus_leiste` (shift+cmd+l) und `fokus_dateifenster` (shift+cmd+d);
für die Vorschau fehlt das Gegenstück, und `resources/default-keymap.toml`
gehört dem ontocoder. Damit sind die Vorschau-Tabs über die Tastatur nur
erreichbar, solange der Fokus schon per Maus in der Vorschau steht — eine
Spannung zu C2, das jede Funktion über mindestens einen Tastenbefehl
erreichbar verlangt.

Dazu kommt eine zweite, kleinere Spannung im Spec selbst: das vierte
Abnahmekriterium von C2 sagt "Ein Tastenbefehl wirkt dann und nur dann, wenn
der Eingabefokus in einem Dateifenster oder in der Lesezeichenleiste steht".
Der Satz stammt aus der Zeit vor dem ausgebauten C6/C10 und kennt den dritten
Bereich nicht; die vier Tabbefehle wirken seit S19 auch bei Fokus in der
Vorschau.

## Möglichkeiten

1. **Ein Fokusbefehl für die Vorschau**, etwa `fokus_vorschau`, analog zu
   `fokus_leiste`. Pro: schließt die C2-Lücke mit dem vorhandenen Mechanismus
   (`fokus_setzen` trägt den Arm schon). Contra: verbraucht eine Kombination;
   die Wahl der Taste ist Nutzersache. Umsetzung: eine Zeile in
   `resources/default-keymap.toml` (ontocoder) und ein Kommando in der
   Aufzählung (coder).
2. **Kein Tastenweg in dieser Runde.** Die Vorschau-Tabs bleiben per Maus
   fokussierbar; C2 wird für die Vorschau erst mit dem Editor-Ausbau
   eingelöst. Pro: keine neue Taste. Contra: die C2-Zusage bleibt für die
   Vorschau-Tabbefehle offen.

In beiden Fällen sollte der C2-Satz "Dateifenster oder Lesezeichenleiste" im
Spec um den dritten Bereich ergänzt werden, sonst widersprechen sich C2 und
C6/C10 weiter.

## Empfehlung

Möglichkeit 1: der Mechanismus ist vorhanden, es fehlt nur die Taste, und die
Ein-Regel-Antwort ("die Tabbefehle wirken auf den Bereich mit Tabs, der den
Fokus hat") trägt erst mit einem Tastenweg in alle drei Bereiche.
