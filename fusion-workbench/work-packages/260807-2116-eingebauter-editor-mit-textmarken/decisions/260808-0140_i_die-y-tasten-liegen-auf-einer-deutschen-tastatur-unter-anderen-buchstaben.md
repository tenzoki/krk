# Die y-Tasten liegen auf einer deutschen Tastatur unter anderen Buchstaben: was nun?

---
**Domain:** code
**Status:** implemented
**Filed by:** planner
**Cross-references:** `shared/issues/260807-2112_*_cmd-y-und-shift-cmd-y-loesen-nichts-aus-f3-schon.md` (der Defekt, dessen beide Verdächtige hier widerlegt werden), `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-2317_*_cmd-y-liegt-auf-einer-deutschen-tastatur-unter-der-taste-z.md` (derselbe Befund, am 260804-0830 vom Nutzer geschlossen), `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md` (der Entscheid, der dadurch der Sache nach nicht eingelöst ist), `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260807-2147_*_spec-eingebauter-editor-mit-textmarken.md` (C8), `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md` (Befund 4, S1 und S2)

---

## Question

Die Fähigkeit C8 des Spec setzt voraus, dass `cmd+y` und `shift+cmd+y` wegen eines Fehlers im Programm nichts auslösen, und verlangt, dass die Behebung "die Regel und nicht die einzelne Kombination" trifft. **Diese Voraussetzung ist falsch. Beide im Defekt genannten Verdächtigen sind am Code widerlegt, und ein dritter Fehler im Programm ist nicht zu finden, weil keiner da ist.**

Der erste Verdächtige, das Hauptmenü greife `cmd+y` ab, scheidet doppelt aus. Das Hauptmenü trägt sieben Einträge, und ihre Kürzel sind `cmd+q`, `cmd+x`, `cmd+c`, `cmd+v`, `cmd+a`, `cmd+n` und `shift+cmd+w` (`crates/krk-ui/src/appkit/menue.rs:184-252`, die Kürzel kommen aus der Belegung). Keiner trägt ein `y`. Und die Reihenfolge stimmt ohnehin: der lokale Ereignisabgriff sieht den Tastendruck vor `NSApplication::sendEvent:` und damit vor jedem Menükürzel; der Modulkopf von `menue.rs:31-42` schreibt es aus.

Der zweite Verdächtige, die Normalisierung der Zusatztasten, scheidet an einem Prüfstein aus. `normalisieren` (`crates/krk-core/src/tasten/normalisierung.rs:181-196`) liest genau vier Bits und wirft Feststelltaste, Zehnerblock, Hilfe und Funktionstastenbit weg; auf der anderen Seite des Vergleichs steht dieselbe Maskenform aus `Kombination::lesen` (`crates/krk-core/src/tasten/parser.rs:369-410`). Verglichen wird `u8` gegen `u8`, und eine rohe AppKit-Maske kommt darin nicht vor. Der Prüfstein: `f3` trägt am Referenzgerät das Funktionstastenbit (`spikes/fn-tasten/messung-A.txt`, `roh=0x00800100`) und wirkt trotzdem. Ein roher Maskenvergleich ließe `f3` ebenso scheitern.

**Die Ursache liegt in der Tastaturbelegung des Geräts.** KRK belegt den virtuellen Tastencode und nicht das gemeldete Zeichen; das ist die Festlegung aus C3 der Runde 1, und für die Funktionstasten ist sie richtig, weil F3 denselben Code liefert, gleich ob der Nutzer fn hält. Ein Tastencode benennt aber eine **Stelle** auf der Tastatur (`parser.rs:105-107`). Die Stelle `kVK_ANSI_Y` trägt den Code 16 (`parser.rs:209`), und auf einer deutschen Tastatur steht dort ein **Z**. Wer die Taste mit der Aufschrift Y drückt, erzeugt Code 6, also `kVK_ANSI_Z`, und dieser Code steht in der ganzen Auslieferungsbelegung in keiner einzigen Tastenliste.

**Das ist bekannt, und der Nutzer hat es einmal entschieden.** Der Defekt `circles/260802-0842-krk-mac-dateimanager-editor-git/issues/260803-2317_*_cmd-y-liegt-auf-einer-deutschen-tastatur-unter-der-taste-z.md` beschreibt genau diesen Fall und führt dieselben drei Wege wie unten. Der Nutzer hat ihn am 260804-0830 mit Weg 1 geschlossen, und seine Begründung trug drei Gründe: `f3` sei der ausgelieferte Hauptweg zur Vorschau und von der Tastaturbauart nicht berührt, das Cmd-Kürzel sei der zweite Weg und nicht der einzige, und die Belegung sei ab Werk änderbar.

**Der zweite dieser drei Gründe trägt seit dem 260807 nicht mehr, und das ist der Anlass, die Frage neu zu stellen.** Die Funktion `fokus_vorschau` ist an jenem Tag hinzugekommen und trägt genau eine Kombination, `shift+cmd+y` (`resources/default-keymap.toml:349`). Sie hat keinen zweiten Weg. Der einzige Tastenweg in das Vorschaufenster liegt damit auf einer deutschen Tastatur nicht dort, wo er beschriftet ist, und der Entscheid `decisions/260805-2216_*_tastenweg-des-fokus-in-das-vorschaufenster.md` ist der Sache nach nicht eingelöst.

Die Frage ist jetzt zu stellen, weil der Spec zwei Abnahmekriterien darauf gebaut hat, die in ihrer heutigen Fassung nicht erfüllbar sind, und weil der Editor ein vierter Fokusbereich mit einem vierten Fokusbefehl wird.

## Options

1. **So lassen, wie am 260804 entschieden, und die beiden Abnahmekriterien von C8 umschreiben.**
   - Pro: kein Programmteil ändert sich, kein Umfang wächst. Der Nutzer kann jede Kombination ab Werk umbelegen und sie auf die Taste unter seinem Finger legen; die Belegungsansicht aus C3 kann das.
   - Contra: der einzige Tastenweg in das Vorschaufenster bleibt an einer Stelle, die nicht beschriftet ist. Der Spec verliert zwei Abnahmekriterien und bekommt zwei, die den Ist-Zustand beschreiben statt einen Anspruch. Wer KRK zum ersten Mal startet, findet `shift+cmd+y` nicht.

2. **Die beiden Kombinationen tauschen: `cmd+z` statt `cmd+y`, `shift+cmd+z` statt `shift+cmd+y`.**
   - Pro: zwei Zeilen in `resources/default-keymap.toml`, und auf einer deutschen Tastatur liegen beide danach unter der Aufschrift Y. Die Konflikterkennung meldet nichts, weil `z` in keiner anderen Tastenliste steht.
   - Contra: auf einer amerikanischen Tastatur ist es danach falsch, und zwar genau um denselben Betrag. Es ist die geräteabhängige Vorbelegung, die C3 der Runde 1 ausschließt, nur mit umgekehrtem Vorzeichen. Und `cmd+z` ist auf dem Mac das Rückgängig; der Plan sieht es in S7 als Menüeintrag vor, und der Menüeintrag schlüge über das **Zeichen** an, während `vorschau_umschalten` über den **Code** anschlüge — zwei Funktionen auf einer beschrifteten Taste bei zwei Zustellern. Die Konflikterkennung ließe das durch, weil sie genau diesen Fall als zulässig führt, und der Nutzer sähe es nicht kommen.

3. **Buchstaben und Ziffern über das gemeldete Zeichen nachschlagen, Funktionstasten weiter über den Tastencode.**
   - Pro: die sachlich vollständige Auflösung, und die einzige, die auf jeder Tastaturbelegung stimmt, auch auf einer französischen, wo weit mehr als zwei Tasten wandern. Sie löst die Frage einmal statt sie je Kombination zu verschieben.
   - Contra: eine zweite Nachschlagart neben der bestehenden, in `Belegung::nachschlag`. Der Datensatz von 260803 nennt das "genau die Sonderregel, die die Maxime supersimpel meidet" und hat sie einer späteren Runde vorbehalten.
   - **Ein Punkt, den der Datensatz von 260803 nicht hatte und der diesen Weg billiger macht, als er dort aussah:** das Hauptmenü schlägt bereits heute über das Zeichen nach. `NSMenuItem.keyEquivalent` nimmt eine Zeichenkette entgegen (`crates/krk-ui/src/appkit/menue.rs:322-342`), und genau deshalb wirken `cmd+c` und `cmd+v` auf jeder Tastaturbelegung an der beschrifteten Stelle. Eine zeichenbasierte Nachschlagart wäre also keine fremde Mechanik, sondern die, die vier Funktionen dieses Projekts schon tragen, nur an einer zweiten Stelle. Der Zuschnitt "Buchstaben über das Zeichen, Funktionstasten über den Code" beschreibt damit nicht zwei Wege, sondern beendet eine Asymmetrie, die es heute schon gibt.

## Constraints

- Die Festlegung aus C3 der Runde 1 steht: KRK erkennt die Tastaturbauart nicht und liefert keine je nach Gerät verschiedene Vorbelegung aus. Weg 2 verletzt sie in der anderen Richtung, Weg 3 hebt sie nicht auf, sondern macht sie gegenstandslos.
- Jede Kombination, die in KRK etwas auslöst, steht in der Belegung, wird von der Konflikterkennung gesehen und ist umbelegbar (C3 der Runde 1). Jede Antwort muss das halten.
- Genau eine der 58 ausgelieferten Kombinationen war am 260803 betroffen; seit dem 260807 sind es zwei, weil `fokus_vorschau` hinzugekommen ist. Alle übrigen Buchstaben der Belegung liegen auf deutscher und amerikanischer Tastatur an derselben Stelle.
- **Diese Frage hält keinen Planschritt auf.** Keine der dreizehn neuen Kombinationen der Editor-Runde liegt auf `y` oder `z`; der Plan hat sie ausdrücklich darum herum gelegt. S1 benennt die Ursache und schließt den Defekt, S2 setzt die gewählte Antwort um, und kein anderer der 42 Schritte hängt an S2.

## Recommendation

**Wir empfehlen Weg 1 für diese Runde und Weg 3 als eigenes, benanntes Vorhaben.** Die Trennung folgt daraus, dass die beiden verschiedene Fragen beantworten. Weg 1 beantwortet "was tut diese Runde", und die Antwort ist: nichts, weil der Editor davon nicht berührt ist und der Nutzer die zwei betroffenen Kombinationen mit zwei Handgriffen in der Belegungsansicht dorthin legen kann, wo er sie will. Weg 3 beantwortet "wann stimmt KRKs Belegung auf jedem Gerät", und das ist eine Fähigkeit für sich, mit einer eigenen Abnahme und einem eigenen Umfang.

**Weg 2 empfehlen wir ausdrücklich nicht**, und zwar aus einem Grund, den der Datensatz von 260803 noch nicht kennen konnte: der Plan dieser Runde legt Rückgängig auf `cmd+z` als Menüeintrag, und der Menüeintrag schlägt über das Zeichen an. Ein `vorschau_umschalten` auf dem Tastencode 6 stünde danach auf derselben beschrifteten Taste wie das Rückgängig, ohne dass die Konflikterkennung es meldet, weil sie zwei Zusteller als zulässig führt. Das ist genau die Art von Verwicklung, die man nicht bemerkt, bevor sie im Betrieb auffällt.

**Was an dieser Empfehlung eine Auslegung ist und nicht eine Feststellung:** wir wissen nicht, wie sehr den Nutzer stört, dass der einzige Tastenweg in die Vorschau nicht dort liegt, wo er beschriftet ist. Am 260804 hat er das für `cmd+y` ausdrücklich hingenommen, damals aber mit `f3` als zweitem Weg im Rücken. Für `shift+cmd+y` gibt es diesen Rücken nicht, und ob das den Unterschied macht, kann nur er sagen.

**Ein Prüflauf von einer Minute, unabhängig von der Wahl:** ⌘ und die Taste mit der Aufschrift **Z** drücken. Blendet die Vorschau ein und aus, ist die Erklärung am laufenden Bündel bestätigt. ⇧⌘ und dieselbe Taste setzen dann den Fokus in die Vorschau.

---
Answered: circles/260807-2116-eingebauter-editor-mit-textmarken/history/260807-2139-orchestrator-session.md §"12. Die y-Tasten auf der deutschen Tastatur" — Möglichkeit 3 gewählt: Buchstaben und Ziffern werden künftig über das gemeldete Zeichen nachgeschlagen, Funktionstasten weiter über den Tastencode. Damit stimmt die Belegung auf jeder Tastaturbelegung, nicht nur auf der deutschen und der amerikanischen. Der Preis einer zweiten Nachschlagart in Belegung::nachschlag ist angenommen; tragend für die Wahl war, dass das Hauptmenü über NSMenuItem.keyEquivalent (menue.rs:322-342) bereits heute über das Zeichen nachschlägt, die zeichenbasierte Art im Projekt also keine fremde Mechanik ist. Der Datensatz von 260803, der diesen Weg einer späteren Runde vorbehielt, ist damit überholt. Entschieden vom Nutzer am 260808-0155.

---
Implemented: `crates/krk-core/src/tasten/parser.rs` (`Tastenkennung`, `Taste::kennung`, `zeichen_als_kennung`, `taste_mit_zeichen`, `Kombination::aus_tastendruck`), `crates/krk-core/src/tasten/mod.rs` (`Tastendruck` trägt Stelle und Zeichen, `Tastendruck::kennung`), `crates/krk-core/src/tasten/belegung.rs` (`Belegung::nachschlag` vergleicht Maske und Kennung), `crates/krk-ui/src/appkit/ereignisse.rs` (`gemeldetes_zeichen` über `charactersByApplyingModifiers:`) — Weg 3 ist umgesetzt: Buchstaben und Ziffern werden über das gemeldete Zeichen nachgeschlagen, Funktionstasten weiter über den Tastencode. Auf einer deutschen Tastatur liegen `cmd+y` und `shift+cmd+y` seither unter der Aufschrift Y und `cmd+z` unter der Aufschrift Z; der Zusammenstoß mit dem Rückgängig des Editors ist damit weg (`issues/260809-1642_c_auf-einer-deutschen-tastatur-schluckt-cmd-y-das-rueckgaengig-des-editors.md`). Der Commit folgt durch den Orchestrator; Planschritt S2, umgesetzt am 260809-1746, Abnahme in `history/260809-1746-coder-s2-y-frage-nachschlag-ueber-das-zeichen.md`.
