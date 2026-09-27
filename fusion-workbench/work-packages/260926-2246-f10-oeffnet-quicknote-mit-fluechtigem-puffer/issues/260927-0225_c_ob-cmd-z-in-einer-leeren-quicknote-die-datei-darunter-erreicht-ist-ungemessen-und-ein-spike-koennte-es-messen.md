Ob cmd+z in einer leeren Quicknote die Datei darunter erreicht, ist ungemessen, und ein Spike könnte es ohne den Nutzer messen
---
Die Zusage A2 des Spec („eine offene Datei kommt unverändert zurück, und `cmd+z` nimmt dort die letzte Dateiänderung zurück“) hängt an einer AppKit-Eigenschaft, die für diesen Fall niemand gemessen hat. Die Textfläche der Quicknote bekommt ihren eigenen `NSUndoManager` über `undoManagerForTextView:` (`crates/krk-ui/src/appkit/quicknote.rs`, `verwalter_fuer`). Die Textfläche des Editors darunter meldet ihre Handlungen am Verwalter des **Fensters** an (Doc-Kommentar von `rueckgaengigstapel_leeren`, `crates/krk-ui/src/appkit/editor.rs`: „die Textflaeche des Editors die einzige in KRK, die diesen Verwalter benutzt“).

Offen ist, welchen Verwalter `NSWindow.undo:` nimmt, wenn der eigene Stapel der Quicknote leer ist: direkt nach F10, direkt nach „Kopieren“ (`nach_kopie_leeren` räumt ihn mit `removeAllActions`) und nach dem letzten `cmd+z` einer Tippfolge. Nimmt AppKit dann den Verwalter des Fensters, wird eine Änderung in der unsichtbaren Datei zurückgenommen, und ihr Stand weicht still ab.

Belege, die in verschiedene Richtungen zeigen:
- Doc-Kommentar von `rueckgaengigstapel_leeren` (Messung vom 260810): der Weg über `undoManagerForTextView:` bleibe „vom Menü aus erreichbar“. Der Fall des leeren eigenen Stapels ist dort nicht genannt.
- `messungen/260926-0828-zellen-rueckgaengig.txt`, Durchgang `eigener`: ein Feldeditor mit eigenem Verwalter über ein überschriebenes `undoManager`; `cmd+z` nahm trotzdem die Handlung am Verwalter des Fensters zurück, bei leerem wie bei gefülltem eigenem Stapel. Das ist ein Feldeditor und ein anderer Anmeldeweg; übertragbar ist es nicht belegt, ausgeschlossen auch nicht.
- Der Plan `260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md` führt genau dieses Risiko unter „Risks & Mitigations“ und schiebt es in den Abnahmelauf am Bündel. Der Abnahmelauf ist Nutzerarbeit, und die Messung vom 260926 zeigt, dass diese Frage keinen Vordergrund-Lauf von KRK braucht: das Programm unter `spikes/zellen-rueckgaengig/` lief „ohne Nutzerarbeit“.

Gefunden durch Lesen des Codes und der Messberichte, nicht am laufenden Bündel nachgestellt.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260927-0225-reviewer-quicknote-und-termine-nachzug.md`; `260926-2300_*_spec-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md` (A2, A13); `260927-0110_*_plan-f10-oeffnet-quicknote-mit-fluechtigem-puffer.md` (Entscheidung 10, Risks & Mitigations)

Abnahme:
- Ein Messbericht unter `messungen/` hält für eine gewöhnliche `NSTextView` mit Delegiertem, der `undoManagerForTextView:` mit einem eigenen Verwalter beantwortet, neben einer zweiten Textfläche am Verwalter des Fensters mit einer angemeldeten Handlung fest: was `cmd+z` über das Hauptmenü bei leerem eigenem Stapel tut, was es nach `setString:` plus `removeAllActions` tut, und ob „Rückgängig“ im Menü dann grau ist. Gleiches für `shift+cmd+z`.
- Zeigt die Messung, dass der Verwalter des Fensters erreicht wird, beantwortet die Fläche der Quicknote `undo:` und `redo:` selbst an ihrem Verwalter (das Muster des `Zelleneditor`), und die Nadel von `jede_bearbeitbare_textflaeche_schaltet_die_automatiken_ab` erfasst die Unterklasse.
- Zeigt sie es nicht, nennt der Modulkopf von `appkit/quicknote.rs` unter „Warum der Verwalter ueber den Delegierten kommt“ den Messbericht als Beleg für den leeren Stapel.
---
Resolved: Gemessen am 260927 mit dem neuen Prüfprogramm `spikes/quicknote-rueckgaengig/` (ohne KRK, ohne Nutzerarbeit), Bericht `messungen/260927-0232-quicknote-rueckgaengig.txt`. In keinem der drei Zustände (direkt nach dem Öffnen, nach `setString:` plus `removeAllActions`, nach dem Zurücknehmen der letzten Tipp-Handlung) und auch nicht nach einem zurückgenommenen Leeren erreicht `cmd+z` oder `shift+cmd+z` den Verwalter des Fensters: „Rückgängig“ bzw. „Wiederholen“ ist bei leerem eigenem Stapel grau, und an der Menüprüfung vorbei nimmt `NSWindow.undo:`/`redo:` den eigenen Verwalter der Quicknote und tut nichts. A2 hält; kein Code geändert. Der Modulkopf von `crates/krk-ui/src/appkit/quicknote.rs` („Warum der Verwalter ueber den Delegierten kommt“) und `CLAUDE.md` nennen den Bericht als Beleg. Commit `fix(quicknote): cmd+z bei leerem Stapel gemessen, Kopf von rueckgaengigstapel_leeren nachgezogen`.
