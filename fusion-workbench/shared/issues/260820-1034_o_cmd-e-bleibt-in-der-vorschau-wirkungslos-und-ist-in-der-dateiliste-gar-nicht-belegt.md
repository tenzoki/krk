`cmd+e` bleibt in der Vorschau wirkungslos und ist in der Dateiliste gar nicht belegt

---

Der Nutzer meldet am 260820-1030 aus dem Abnahmelauf der Runde 14: `cmd+e` habe „keine Funktion,
weder in Dateiliste noch in Vorschau". Der Befund zerfällt in zwei Hälften mit verschiedenem
Status.

---

**Gefilt von:** orchestrator, Sitzung `260819-2026`, aus dem Bündeldurchgang des Nutzers
**Baumstand:** `dad0a36`, Bündel 0.5.4 aus `05cb614`
**Schwere:** mittel. Ein Befehl steht mit Namen und Kombination im Hauptmenü und tut nichts —
genau die Gestalt, vor der `CLAUDE.md` unter „Was man nicht sieht" warnt.

## Die beiden Hälften

**In der Dateiliste ist das kein Defekt, sondern die Belegung.** `cmd+e` liegt auf
`editor_aus_vorschau` (`resources/default-keymap.toml:800-802`), und
`Kommando::EditorAusVorschau` trägt `Wirkungsbereich::Vorschau`
(`crates/krk-core/src/tasten/belegung.rs:923`). Mit dem Fokus in der Dateiliste ist der Befehl
unzulässig und wird richtigerweise abgewiesen. Wer aus der Dateiliste heraus eine Datei im Editor
öffnen will, drückt `f4`; das ist ein anderer Befehl (`bearbeiten`, `:161-163`).

**In der Vorschau ist es ein Defekt.** Dort ist der Befehl zulässig, und er soll die angezeigte
Datei im Editor öffnen. Er tut es nach dem Bericht des Nutzers nicht.

## Was geprüft ist und was nicht

**Geprüft:** Die Runde 14 hat den Befehl nicht verursacht.
`Anwendungsdelegierter::editor_aus_vorschau` (`anwendung.rs:6243`) ist im Bereich
`fce0b6f..dad0a36` unverändert; die Runde hat in dieser Datei allein `fokusansicht` (Zweig
`Fokus::Vorschau`), `ist_eigene_textflaeche` und zwei Prosastellen angefasst. Der Zweig
`Fokus::Editor` von `fokusansicht` ist unangetastet.

**Nicht geprüft:** warum der Befehl in der Vorschau wirkungslos bleibt. Die Ursache ist nicht
erhoben, und dieser Datensatz behauptet keine. Zwei Richtungen bieten sich an und sind beide
ungemessen: der Befehl erreicht seinen Ausführungszweig nicht (Zulässigkeit, Fokuswert), oder er
erreicht ihn und `editor_oeffnen_lassen` scheitert still. Der Zusammenhang mit
`260820-1034_o_f4-setzt-den-fokus-nur-dann-in-den-editor-wenn-er-schon-eine-datei-zeigt.md` ist zu
prüfen: beide enden in `editor_oeffnen_lassen`.

## Reproduktion

Eine Datei in der Vorschau anzeigen, den Fokus in die Vorschau setzen, `cmd+e` drücken.
Erwartet: die Datei öffnet sich im Editor. Beobachtet: nichts.
