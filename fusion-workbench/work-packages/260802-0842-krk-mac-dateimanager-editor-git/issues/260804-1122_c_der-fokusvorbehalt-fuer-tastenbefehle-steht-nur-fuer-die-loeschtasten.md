Der Fokusvorbehalt für Tastenbefehle steht nur für die Löschtasten, gebraucht wird er für jede Belegung

---

C4 des Specs `planning/260802-1036_o_spec-navigator-geruest.md` sagt zu: "Die Löschtasten lösen nur dann eine Löschung aus, wenn der Eingabefokus in einem Dateifenster steht. In der Pfadeingabe, im Umbenennen-Feld und in jedem anderen Textfeld bleiben Delete und Cmd+Delete die gewohnten Textbefehle."

Derselbe Vorbehalt fehlt für jede andere Belegung, obwohl der Ereignisabgriff aus Schritt 7 jeden Tastendruck der Anwendung sieht, gleich wo der Fokus steht. `behandeln` in `crates/krk-ui/src/appkit/ereignisse.rs:180-206` fragt die Belegung und schluckt das Ereignis, sobald ein gebautes Kommando dahintersteht; eine Abfrage nach dem Ersthelfer des Fensters gibt es dort nicht.

---

Herkunft: gefunden beim Nachziehen von Spec und Plan auf den Nutzerentscheid vom 260804-1122 zur Ordnernavigation.

Warum die Lücke jetzt akut wird: mit der neuen Belegung trifft sie drei alltägliche Tasten. `cmd+left` und `cmd+right` sind auf dem Mac in jedem Textfeld der Sprung an Zeilenanfang und Zeilenende, ab jetzt aber auch der Auf- und Abstieg im Verzeichnisbaum. `return` verliert seine Belegung und bestätigt in einem Blatt die Eingabe. Die Pfadeingabe aus C2 ist das erste Blatt, das der Plan baut (Schritt 13), und sie wäre ohne den Vorbehalt nicht bedienbar: wer darin einen Pfad tippt und `cmd+left` drückt, um an den Anfang zu springen, wechselt stattdessen den Ordner.

Für Schritt 13 ist die Antwort im Plan ausgeschrieben: der Abgriff fragt vor dem Nachschlag, ob der Ersthelfer des Fensters ein Textfeld ist, und reicht den Tastendruck in diesem Fall unverändert weiter. Ein Vorbehalt im Abgriff statt einer Regel je Blatt, damit die fünf Blätter aus den Schritten 16 und 17 ihn erben, ohne ihn zu wiederholen.

Offen bleibt zweierlei. Erstens die Formulierung im Spec: C4 nennt weiterhin nur die Löschtasten, während C2 seit dem 260804-1122 ein allgemeines Abnahmekriterium dafür trägt. Ob C4 auf das allgemeine Kriterium verweist oder seinen eigenen Satz behält, ist beim nächsten Durchgang durch C4 zu klären. Zweitens die Durchsicht der Schritte 16 und 17 samt Umbenennen-Feld darauf, ob der Vorbehalt im Abgriff dort wirklich reicht oder ob ein Blatt einen Tastenbefehl braucht, den der Abgriff ihm dann wegnähme.

---
Resolved: Der Fokusvorbehalt steht seit dem 260805 als eigenes Abnahmekriterium in **C2** und gilt damit für jede Belegung statt nur für die Löschtasten: "Ein Tastenbefehl wirkt dann und nur dann, wenn der Eingabefokus in einem Dateifenster oder in der Lesezeichenleiste steht. In der Pfadeingabe, im Umbenennen-Feld und in jedem anderen Textfeld behalten alle Tasten ihre gewohnte Mac-Bedeutung." C2 ist der richtige Ort, weil der Vorbehalt seit S13c mehr trägt als eine Randbedingung von C4: die Zustellerregel der Konflikterkennung beruft sich ausdrücklich auf ihn, und ohne ihn wäre `cmd+a` bei zwei Funktionen ein Konflikt. C3 verweist an der Stelle darauf, an der es ihn braucht. Die Arbeit stammt vom `planner`, der vor seinem Bericht abgebrochen wurde; der orchestrator hat sie nachgeprüft und geschlossen.
