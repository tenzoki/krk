Der Modulkopf von `menue.rs` beschreibt den Fokusvorbehalt noch als frühen Ausstieg des Abgriffs

---

`crates/krk-ui/src/appkit/menue.rs:105-109`: „Der Ereignisabgriff aus `super::ereignisse` sieht ihn
vor der Menuebehandlung von `NSApplication`. Steht die Schreibmarke in einem Textfeld, kehrt er
sofort zurueck und reicht weiter". Das ist die Bauart bis zur Runde 7. `ereignisse.rs:112-124`
sagt seither das Gegenteil: „Seit der Runde 7 ist der Vorbehalt keine Station dieses Abgriffs
mehr … Der Abgriff fragt danach ueberhaupt nicht mehr nach dem Ersthelfer", und `behandeln`
(`:602-669`) hat keinen solchen Ausstieg. Ebenso `:173-176` („weist der stumme Fokusvorbehalt
`alle_markieren` ab, sodass der Tastendruck unveraendert an AppKit geht") — heute entscheidet
`kommandos::zulaessigkeit::zulaessig` an der Senke, und der Abgriff schluckt nach Zulässigkeit
(`ereignisse.rs:193-202`).

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/menue.rs` (Prosa; der Code der Datei ist unberührt)

`CLAUDE.md` nennt genau diese Lesart als Quelle eines Fehlbefunds („Wer nur `ereignisse.rs`
liest, hält den Vorbehalt für die einzige Sperre … Genau so entstand `260810-1102`"), und
`ereignisse.rs` trägt seit `8807844` die Warnung. `menue.rs` trägt an zwei Stellen die alte
Beschreibung und führt den nächsten Leser auf denselben Weg. Der Ausgang der Sätze (die sechs
Textbefehle erreichen das Menü, weil `Belegung::nachschlag` sie überspringt) bleibt richtig; nur
der Mechanismus davor ist falsch beschrieben. Weg: die zwei Absätze auf die Zulässigkeitsregel
umschreiben und auf `ereignisse.rs` „Der Fokusvorbehalt" verweisen.


---
Resolved: Beide Absaetze sind auf die Zulaessigkeitsregel umgeschrieben (`crates/krk-ui/src/appkit/menue.rs`). Der erste sagt jetzt, dass seit der Runde 7 an der Senke entschieden wird, dass der Abgriff nach dem Ersthelfer gar nicht mehr fragt, und dass ein Befehl mit der Schreibmarke im Textfeld unzulaessig ausfaellt statt geschluckt zu werden; er verweist auf den Abschnitt „Der Fokusvorbehalt" in `ereignisse.rs`. **Der zweite war doppelt falsch und ist entsprechend berichtigt:** mit dem Fokus in der Leiste weist `alle_markieren` nicht der Fokusvorbehalt ab, sondern der Bestandteil (3) von `zulaessigkeit::zulaessig` — `Kommando::AlleMarkieren` traegt `Wirkungsbereich::Dateifenster` (nachgesehen in `crates/krk-core/src/tasten/belegung.rs`), und eine `NSTableView` stellt dem Vorbehalt gar keinen Ersthelfer, an dem er greifen koennte. Der Ausgang der Saetze — der Eintrag ist in der Leiste bedienbar — bleibt unveraendert richtig.
