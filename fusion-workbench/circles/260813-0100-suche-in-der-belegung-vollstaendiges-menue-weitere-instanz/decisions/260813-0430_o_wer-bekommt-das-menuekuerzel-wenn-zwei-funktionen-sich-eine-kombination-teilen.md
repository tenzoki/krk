# Wer bekommt das Menükürzel, wenn zwei Funktionen sich eine Kombination teilen?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:** `issues/260813-0416_o_zwei-menueeintraege-mit-cmd-a-und-appkit-nimmt-dem-spaeteren-das-kuerzel.md`,
`shared/planning/260813-0053_o_spec-…` (C2.1, C2.4, C2.8, C2.18),
`decisions/260805-0713_*_ist-eine-kombination-bei-zwei-zustellern-ein-konflikt.md` (Runde 1),
`crates/krk-ui/src/menuemodell.rs` (`zugestellte_kuerzel`),
`resources/default-keymap.toml` (Kopfabschnitt zu den Zustellern, `alle_markieren`,
`text_alles_auswaehlen`)

---

## Frage

C2.1 verlangt, dass **jede** Funktion der Belegung im Hauptmenü steht, und C2.4, dass jeder
Eintrag sein Kürzel aus der Belegung nimmt. Ausgeliefert tragen zwei Funktionen dieselbe
Kombination: `alle_markieren` und `text_alles_auswaehlen`, beide `cmd+a`. Der Entscheid vom
260805 erlaubt das ausdrücklich, weil verschiedene Zusteller sie tragen und „zwei Funktionen
mit verschiedenen Zustellern einander nie begegnen".

**In der Menüleiste begegnen sie einander jetzt doch.** Eine Menüleiste verträgt dieselbe
Tastenentsprechung nicht zweimal; AppKit nimmt sie dem später stehenden Eintrag still weg
(gemessen am 260813, siehe den Defekt). Damit ist zu entscheiden, welcher der beiden Einträge
das Kürzel behält — oder ob die Doppelung überhaupt bestehen bleibt.

Die Frage ist grundsätzlich und nicht auf `cmd+a` beschränkt: sie fällt bei jeder künftigen
Doppelung wieder an, und die Belegungsdatei lässt sie weiterhin zu.

## Möglichkeiten

1. **Das Menükürzel bekommt der Zusteller; der Befehl von KRK zeigt keines.**
   `menuemodell::zugestellte_kuerzel` sammelt die Kürzel der sechs zugestellten Funktionen,
   und ein Befehlseintrag mit derselben Kombination bekommt keine.
   - Dafür: Kein Verlust an Wirkung. Ein Befehl von KRK braucht sein Menükürzel nicht — der
     Ereignisabgriff sieht jeden Tastendruck **vor** dem Menü, und im Dateifenster verbraucht
     `alle_markieren` das `cmd+a`, bevor das Menü es sieht. Eine zugestellte Funktion hat
     diesen Weg nicht: `Belegung::nachschlag` überspringt sie, und ohne Menükürzel erreicht
     `cmd+a` den Feldeditor auf keinem Weg (gemessen am 260804-1309). C2.18 bleibt erfüllt.
     Die Regel ist aus der Sache abgeleitet und nicht aus der Stellung in der Leiste, also
     unabhängig von der Reihenfolge der Belegungsdatei.
   - Dagegen: „Alle Einträge markieren" steht im Menü ohne `Cmd+A`, obwohl `Cmd+A` es auslöst.
     Der Nutzer, der das Menü als Nachschlagewerk benutzt, findet die Kombination dort nicht —
     wohl aber in der Belegungsansicht und in der Markdown-Ausgabe, die beide die Belegung
     fragen und nicht das Menümodell.
   - **Gebaut ist diese Möglichkeit**, weil die Runde autonom auf Empfehlungen fährt und ein
     Zwischenstand mit gebrochenem C2.18 der teuerste aller Ausgänge wäre.
2. **AppKit entscheiden lassen, also nichts tun.** Der früher stehende Eintrag behält das
   Kürzel.
   - Dafür: keine Zeile Code, und jeder Eintrag zeigt genau das, was in der Belegung steht.
   - Dagegen: `Cmd+A` fällt in jedem Textfeld aus, denn „Dateilisting" steht vor „Bearbeiten".
     Das bricht C2.18 und C2.8. Und wer die Reihenfolge entscheidet, ist die Stellung in der
     Belegungsdatei — eine Datei, die niemand mit dieser Wirkung im Sinn sortiert hat.
3. **Die Doppelung aus der Belegungsdatei nehmen.** `alle_markieren` bekommt eine andere
   Kombination, oder `text_alles_auswaehlen` verliert `cmd+a`.
   - Dafür: Der Fall entfällt ganz, und jeder Menüeintrag zeigt sein Kürzel.
   - Dagegen: Beides nimmt dem Nutzer etwas, das er heute hat — entweder das Markieren aller
     Einträge auf `Cmd+A` oder das Auswählen im Textfeld. Es hebt daneben den Entscheid vom
     260805 auf, ohne dass die Sache, die ihn trug, sich geändert hätte. Und es ist eine
     Änderung an `resources/default-keymap.toml`, also Arbeit des `ontocoder`.
4. **Der Zusteller bekommt das Kürzel, und der Befehl bekommt eine Zweitform.** Der Eintrag
   „Alle Einträge markieren" zeigte `Cmd+A` als Beschriftungszusatz statt als
   Tastenentsprechung.
   - Dafür: Das Menü bliebe als Nachschlagewerk vollständig.
   - Dagegen: Eine Beschriftung, die ein Kürzel nur behauptet, ist eine zweite Wahrheit neben
     der Belegung, und C2.4 sagt ausdrücklich, dass kein Kürzel als Zeichenkette im
     Programmtext steht. Der Weg baut genau das wieder auf, was diese Runde einspart.

## Randbedingungen

- Ausgeliefert ist `cmd+a` die **einzige** doppelt vergebene Kombination; am 260813 über alle
  81 Funktionen nachgezählt. Der Nutzer kann in der Belegungsansicht weitere anlegen, denn die
  Konflikterkennung lässt sie bei verschiedenen Zustellern zu.
- Die Belegungsansicht und die Markdown-Ausgabe sind von jeder dieser Möglichkeiten außer der
  dritten unberührt: sie zeigen die Kombinationen der Belegung und nicht die des Menüs.
- Ob AppKit nach einem **ausgegrauten** Eintrag mit passender Tastenentsprechung weitersucht,
  ist am Baum nicht entscheidbar. Für Möglichkeit 1 fällt die Frage nicht an — dort trägt nur
  noch ein Eintrag die Kombination. Für Möglichkeit 2 hinge das Ergebnis daran. `inference:`
- Ein Kürzel, das der Nutzer in der Belegungsansicht ändert, kann eine neue Doppelung anlegen
  oder eine bestehende auflösen. Jede Möglichkeit außer der dritten rechnet sie bei jedem
  Menüaufbau neu aus und braucht dafür nichts Gespeichertes.

## Empfehlung

Möglichkeit 1. Sie ist die einzige, die niemandem etwas nimmt, was er heute hat, und ihre
Regel steht auf einem Grund und nicht auf einer Reihenfolge: der Ereignisabgriff sieht jeden
Tastendruck vor dem Menü, ein Befehl von KRK ist also auch ohne Menükürzel erreichbar, eine
zugestellte Funktion nicht. Der Preis fällt an der Anzeige an und nicht an der Wirkung, und er
fällt an genau einem Eintrag.

Die Runde fährt bis zu einer Antwort auf Möglichkeit 1.

---
Answered:
Implemented:
Deferred:
Superseded by:
