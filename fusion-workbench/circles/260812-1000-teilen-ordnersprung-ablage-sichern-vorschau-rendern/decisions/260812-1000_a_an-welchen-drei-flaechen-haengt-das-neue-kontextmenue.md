# An welchen Flächen hängt das neue Kontextmenü, und was geschieht mit dem Kontextmenü, das AppKit dem Editor schon gibt?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/appkit/tabelle.rs` (die `NSTableView` des Dateifensters); `crates/krk-ui/src/appkit/vorschau.rs:574-575` (die Inhaltsfläche, weder bearbeitbar noch auswählbar); `crates/krk-ui/src/appkit/editor.rs` (die Textfläche); `crates/krk-ui/src/appkit/ereignisse.rs` (`ersthelfer_gehoert_appkit`)

---

## Question

KRK hat heute kein eigenes Kontextmenü. Unter `crates/krk-ui/src/appkit/` steht an keiner Stelle ein `menuForEvent:`, und die einzige Menüsorte des Programms ist das Hauptmenü aus `appkit/menue.rs`. Das Menü, das im Editor auf die rechte Maustaste erscheint, gehört AppKit und nicht KRK: es kommt von der `NSTextView` und trägt deren Einträge, Ausschneiden, Kopieren, Einsetzen und die Schreibwerkzeuge.

Der Nutzer hat festgelegt, dass Teilen in allen drei Bereichen wirkt, in der Dateiliste, im Editor und in der Vorschau, und dass das neue Kontextmenü zunächst genau einen Eintrag trägt. Offen ist, an welchen Flächen es hängt und wie es sich zu dem verhält, was AppKit im Editor schon zeigt.

Die drei Flächen sind ungleich: die `NSTableView` des Dateifensters hat kein eigenes Kontextmenü, die Vorschaufläche ist eine `NSTextView`, die weder bearbeitbar noch auswählbar ist und deshalb ein weitgehend leeres AppKit-Menü zeigt, und die Textfläche des Editors ist eine bearbeitbare `NSTextView` mit vollem AppKit-Menü.

Die Frage hält keinen Planschritt auf und bindet einen.

## Options

1. **Ein eigenes Kontextmenü an allen drei Flächen, im Editor an das vorhandene angehängt.** Dateiliste und Vorschau bekommen ein `menuForEvent:`, das ein Menü mit dem einen Eintrag liefert; im Editor wird derselbe Eintrag an das Menü angehängt, das AppKit ohnehin baut.
   - Folge: der Nutzer findet den Eintrag überall an derselben Stelle. Im Editor steht er unter den AppKit-Einträgen, was der Ort ist, an dem macOS ihn in anderen Programmen auch zeigt.
   - Preis: drei Anhängepunkte statt einem, und der dritte ist ein anderer Vorgang als die ersten beiden. Ein Menü, das AppKit baut und KRK erweitert, ist eine zweite Bauart neben dem eigenen Menü.

2. **Ein eigenes Kontextmenü an Dateiliste und Vorschau, der Editor bleibt bei seinem AppKit-Menü.** Im Editor erreicht der Nutzer das Teilen allein über die Tastenkombination.
   - Folge: zwei gleichartige Anhängepunkte, eine Bauart. Das AppKit-Menü des Editors bleibt unangetastet, und mit ihm die Entscheidung der Runde 2 über die Schreibwerkzeuge (`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260810-0959_*_schliesst-c4-die-schreibwerkzeuge-aus.md`).
   - Preis: der Nutzer hat gesagt „per tastenkombi und per rechte maustaste". In einem der drei Bereiche gilt dann nur die Hälfte davon, und das ist an der Oberfläche nicht erkennbar.

3. **Ein eigenes Kontextmenü allein an der Dateiliste.** Editor und Vorschau erreichen das Teilen über die Tastenkombination.
   - Folge: ein Anhängepunkt, der kleinste mögliche Eingriff. Das ist auch der Ort, an dem ein Dateiverwalter ein Kontextmenü zuerst erwarten lässt.
   - Preis: zwei der drei Bereiche haben keinen Mausweg. Die Vorschaufläche ist der Ort, an dem der Nutzer eine Datei ansieht und dann entscheidet, sie zu verschicken; gerade dort fehlt er dann.

## Constraints

- Das Menü trägt in dieser Runde genau einen Eintrag. Was sonst hineingehört, ist einer späteren Runde vorbehalten; die Festlegung des Nutzers ist ausdrücklich.
- Der Ereignisabgriff `ersthelfer_gehoert_appkit` (`crates/krk-ui/src/appkit/ereignisse.rs`) fragt nach der Nämlichkeit des Ersthelfers und nicht nach seiner Klasse. Er entscheidet über Tastendrücke, nicht über die rechte Maustaste, ist von dieser Frage also nicht berührt. Wer eine **zweite bedienbare Textfläche** anlegte, müsste sie dort anmelden; diese Runde legt keine an.
- Die Vorschaufläche ist absichtlich nicht auswählbar: eine auswählbare nähme den Fokus als Textsystem, und der Ereignisabgriff reichte jede Taste an AppKit weiter, statt die Tabbefehle auszuführen (`crates/krk-ui/src/appkit/vorschau.rs`, Modulkopf). Eine Antwort darf das nicht ändern. Ob es aus anderem Grund geändert wird, entscheidet der Datensatz `260812-1000_*_was-tut-ein-link-im-gerenderten-markdown-und-bleibt-die-vorschau-unauswaehlbar.md`.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Der Nutzer hat den Mausweg für alle drei Bereiche verlangt, und die zusätzliche Bauart im Editor ist der kleinere Preis gegenüber einem Bereich, in dem die rechte Maustaste stillschweigend nichts tut.

Der Zuschnitt sollte dabei dem folgen, was `appkit/standardprogramm.rs` vormacht: eine Stelle, die das Menü baut, für alle drei Flächen dieselbe, und die drei Flächen fragen nur, welche Einträge betroffen sind. Drei Menübauer nebeneinander wären die Wiederholung, die dieses Projekt an `appkit/nummernspalte.rs` und `appkit/tableiste.rs` bereits zweimal vermieden hat.


## Antwort 260812-1105

**Moeglichkeit 1.**

Das Kontextmenue haengt an allen drei Flaechen: Dateiliste, Editor und Vorschau. Im Editor tritt
es neben das, was AppKit der `NSTextView` von sich aus gibt.

Der Nutzer hat den Mausweg fuer alle drei Bereiche verlangt, und ein Bereich, in dem die rechte
Maustaste stillschweigend nichts tut, waere die schlechtere Auskunft als eine zusaetzliche Bauart
im Editor.

**Der Zuschnitt folgt `appkit/standardprogramm.rs`:** eine Stelle baut das Menue, fuer alle drei
Flaechen dieselbe, und die Flaechen fragen nur, welche Eintraege betroffen sind. Drei Menuebauer
nebeneinander waeren die Wiederholung, die dieses Projekt an `nummernspalte.rs` und `tableiste.rs`
bereits zweimal vermieden hat.

---
Answered: dieser Datensatz, Abschnitt `## Antwort 260812-1105` — Klaerungsrunde des Orchestrators; Sitzungsprotokoll `circles/260812-1000-teilen-ordnersprung-ablage-sichern-vorschau-rendern/history/260812-1055-orchestrator-session.md`.
Implemented:
Deferred:
Superseded by:
