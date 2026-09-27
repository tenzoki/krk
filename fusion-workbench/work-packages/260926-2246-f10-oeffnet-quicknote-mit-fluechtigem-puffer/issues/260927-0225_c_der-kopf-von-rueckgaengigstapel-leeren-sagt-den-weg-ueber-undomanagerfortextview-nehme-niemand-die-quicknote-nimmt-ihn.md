Der Kopf von rueckgaengigstapel_leeren sagt, den Weg über undoManagerForTextView: nehme niemand, und die Quicknote nimmt ihn
---
Der Doc-Kommentar von `rueckgaengigstapel_leeren` (`crates/krk-ui/src/appkit/editor.rs`) sagt über `undoManagerForTextView:`: „Der Weg steht damit offen, wird aber nicht genommen: es gibt keinen zweiten Anmelder, und ein Verwalter mehr waere ein Mechanismus ohne Fall.“ Seit `d3824d9` nimmt die Quicknote genau diesen Weg (`crates/krk-ui/src/appkit/quicknote.rs`, `verwalter_fuer`), und ihr Modulkopf verweist auf eben diesen Kommentar als Beleg. Der Satz ist damit falsch geworden; `CLAUDE.md` ist im selben Zug nachgezogen worden, der Kommentar nicht.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260927-0225-reviewer-quicknote-und-termine-nachzug.md`; `260927-0225_*_ob-cmd-z-in-einer-leeren-quicknote-die-datei-darunter-erreicht-ist-ungemessen-und-ein-spike-koennte-es-messen.md`

Abnahme: `grep -n 'Mechanismus ohne Fall' crates/krk-ui/src/appkit/editor.rs` findet nichts mehr, und der Absatz nennt die Quicknote als Fläche mit eigenem Verwalter über den Delegierten, mit Verweis auf `appkit/quicknote.rs`.
---
Resolved: Der Absatz über `undoManagerForTextView:` im Doc-Kommentar von `rueckgaengigstapel_leeren` (`crates/krk-ui/src/appkit/editor.rs`) nennt jetzt die Quicknote als die Fläche, die den Weg nimmt, mit Verweis auf `super::quicknote` (`verwalter_fuer`) und auf die Messung `messungen/260927-0232-quicknote-rueckgaengig.txt`; „Mechanismus ohne Fall“ steht nicht mehr darin. Der Folgeabsatz verweist nicht mehr auf „den letzten Satz“, sondern auf die Regel vom Verwalter des Ersthelfers. Commit `fix(quicknote): cmd+z bei leerem Stapel gemessen, Kopf von rueckgaengigstapel_leeren nachgezogen`.
