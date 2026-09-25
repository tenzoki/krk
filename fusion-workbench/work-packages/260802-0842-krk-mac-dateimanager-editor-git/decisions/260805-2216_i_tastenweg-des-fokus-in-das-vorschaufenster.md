# Wie erreicht der Eingabefokus das Vorschaufenster über die Tastatur?

---
**Domain:** code
**Status:** implemented
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

## Antwort des Nutzers vom 260807

**Möglichkeit 1, ein eigener Fokusbefehl für die Vorschau**, der Empfehlung
folgend. Die Kennung ist `fokus_vorschau`, und **die Taste hat der Nutzer
selbst bestimmt: `shift+cmd+y`.** Der Buchstabe ist nicht frei gewählt. Die
Vorschau trägt in dieser Belegung schon das `y`, denn `cmd+y` blendet sie ein
und aus, und der Fokusbefehl erbt ihn, wie `l` und `d` es für die Leiste und
das Dateifenster tun. `shift+cmd+v` wäre der naheliegende Buchstabe gewesen
und liegt seit S9 beim Verschieben aus C4.

**Eine Folgefrage hat der Nutzer nicht gestellt bekommen, und sie ist
mitentschieden: `fokus_vorschau` blendet eine ausgeblendete Vorschau ein,
bevor er den Fokus setzt.** Der Nutzer ist am 260807 nur zur Leiste gefragt
worden, in
`decisions/260805-1730_*_holt-der-fokusbefehl-eine-ausgeblendete-leiste-hervor.md`.
Seine Antwort dort redet aber nicht von einer Taste, sondern von einem
Befehlstyp: wer den Fokus in einen Bereich verlangt, verlangt ihn zu sehen.
Für die Vorschau davon abzuweichen hieße, `shift+cmd+y` stumm abzuweisen,
während `shift+f3` aus C10 dasselbe Fenster hervorholt und `shift+cmd+l` seine
Leiste. Drei Befehle auf denselben Randbereichen mit zwei Antworten sind der
Sonderfall, den die Maxime "supersimpel" ausschließt. Für die Vorschau ist die
Asymmetrie zudem in C10 mit `shift+f3` bereits beschlossen.

**Der zweite Punkt dieses Datensatzes ist ebenfalls erledigt.** Das vierte
Abnahmekriterium von C2 nannte als Bedingung allein das Dateifenster und die
Lesezeichenleiste, und zwar mit der Formel "dann und nur dann". Die Prüfung am
260807 hat ergeben, dass der Satz nicht bloß unvollständig, sondern **falsch**
war, und zwar in beiden Richtungen: die Hälfte "nur dann" bricht seit S19,
weil die vier Tabbefehle bei Fokus in der Vorschau wirken, und die Hälfte
"dann" bricht schon seit S18, weil jeder Befehl seinen eigenen
Wirkungsbereich trägt und `lesezeichen_loeschen` im Dateifenster nichts tut.
C5 schrieb den zweiten Fall in seinem eigenen vierten Kriterium bereits aus;
der Spec widersprach sich damit selbst. Das Kriterium zählt jetzt keine
Bereiche mehr auf, sondern nennt die Regel, aus der sie folgen.

**Umgesetzt wird die Antwort in den neuen Planschritten S19b und S19c**,
`planning/260802-1428_*_plan-navigator-geruest-runde-1.md`. S19 bleibt
abgenommen und unverändert.

---
Answered: `planning/260802-1036_*_spec-navigator-geruest.md`:319 — Möglichkeit 1, ein eigener Fokusbefehl auf `shift+cmd+y`; das Abnahmekriterium steht in C6 (ebd.:310), die Belegung in C3 (ebd.:200), und das falsche vierte Kriterium von C2 ist berichtigt (ebd.:145 mit der Herleitung in :161).
Implemented: `9a47c4a` — `Kommando::FokusVorschau` unter der Kennung `fokus_vorschau` in `crates/krk-core/src/tasten/belegung.rs`, Wirkungsbereich `Ueberall` bei den beiden anderen Fokusbefehlen, in der Belegungsansicht unter `Funktionsbereich::Vorschau`; der Eintrag mit `shift+cmd+y` in `resources/default-keymap.toml` im C5-Block hinter `fokus_dateifenster`. Die Schritte S19b und S19c des Plans tragen seither `[DONE]`.
Deferred:
Superseded by:
