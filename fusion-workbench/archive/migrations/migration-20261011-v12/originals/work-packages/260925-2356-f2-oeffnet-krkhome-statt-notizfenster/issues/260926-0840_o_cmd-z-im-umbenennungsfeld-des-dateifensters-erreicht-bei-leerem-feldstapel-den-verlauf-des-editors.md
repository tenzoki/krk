`cmd+z` im Umbenennungsfeld des Dateifensters erreicht bei leerem Feldstapel den Verlauf des Editors

---

Das Umbenennen „direkt in der Liste“ (`crates/krk-ui/src/appkit/tabelle.rs`, beschreibbare Namenszelle) bearbeitet mit dem Feldeditor von AppKit. Gemessen ist am 260926 an einer Zelle einer view-basierten Tabelle: ist der Stapel des Feldeditors leer — nichts getippt, oder das Getippte schon zurückgenommen —, dann nimmt `NSWindow.undo:` die nächste Handlung aus dem Verwalter des **Fensters** zurück (`messungen/260926-0828-zellen-rueckgaengig.txt`, Durchgang `vorgabe`, Fall 1 und 2). In KRK führt diesen Verwalter die Textfläche des Editors (Kopf von `rueckgaengigstapel_leeren` in `crates/krk-ui/src/appkit/editor.rs`). Ein `cmd+z` im Umbenennungsfeld nähme dann das letzte Tippen oder den letzten Tabellenumbau im Editor zurück, der daneben offen steht, und schlösse die Umbenennung.

---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

**Inference, nicht am Bündel gemessen.** Die Messung stand an einer Tabelle ohne Dateifenster; dass die Namenszelle in `tabelle.rs` denselben Weg nimmt, folgt aus der Bauart (derselbe Feldeditor von AppKit, derselbe Verwalter des Fensters) und ist nicht am laufenden Bündel geprüft. Nutzerprüfung: eine Datei im Editor öffnen und etwas tippen, dann im Dateifenster eine Umbenennung beginnen und ohne zu tippen `cmd+z` drücken; zu sehen ist, ob der Editor sein Getipptes verliert.

**Kein Datenverlust, aber ein Standwechsel ohne Anlass**: `shift+cmd+z` stellt das Getippte wieder her, und der Kopf des Editors zeigt die Abweichung. Der Nutzer sieht die Wirkung womöglich nicht, weil der Editor ausgeblendet sein kann.

**Warum nicht mitbehoben.** Der Auftrag war der Defekt `260926-0813_*_cmd-z-bei-offener-zelle-nimmt-einen-tabellenumbau-zurueck-und-verliert-oder-verschiebt-den-getippten-text.md` an der Eintragstabelle. Die Behebung dort (`Zelleneditor` in `crates/krk-ui/src/appkit/eintragsansicht.rs`, geliefert über `windowWillReturnFieldEditor:toObject:` in `crates/krk-ui/src/appkit/fenster.rs`) ließe sich auf die Namenszelle ausdehnen: der Fensterdelegierte fragt heute allein die Eintragsansicht. Ob die Umbenennung denselben Feldeditor bekommt oder einen eigenen gleicher Bauart, gehört zu dieser Ausdehnung und ist nicht entschieden.
