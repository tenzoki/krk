Der Teilersatz an der Textfläche prüft allein die Länge, und der Doc-Kommentar sagt mehr
---
`Editorbereich::stand_einsetzen` ersetzt seit `ff2d7b76` bei einem Umbau nur den geänderten Bereich (`Einsatz::Bereich`, `replaceCharactersInRange:withString:`). Ob die Fläche vorher den Stand trug, gegen den der Bereich gebildet ist, prüft die Stelle nicht: sie prüft allein, dass der Bereich in die Länge der Fläche passt. Der Doc-Kommentar sagt aber, ein Bereich über den Text hinaus heiße, die Fläche trage nicht den erwarteten Stand, und dann werde der ganze Stand geschrieben. Weicht die Fläche ab und der Bereich passt trotzdem, entsteht in einem Release-Bau still ein falscher Text. Die einzige Nachprüfung ist ein `debug_assert_eq!` über die Länge.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261019-0037-die-zelluebernahme-an-16-mib-ist-ohne-setstring-gemessen-und-die-100-ms-grenze-des-plans-ist-nicht-entschieden.md, 261019-1122-reviewer-csv-und-tsv-schlussdurchsicht-vor-dem-schliessen.md

**Schwere:** Low. Heute ist kein Weg bekannt, auf dem Fläche und Stand vor einem Umbau auseinanderlaufen: `textDidChange:` schreibt synchron zurück (`text_zurueckschreiben`), und jeder Umbau ruft vorher `zelle_uebernehmen`. Der Befund betrifft die Zusage und die Absicherung, nicht einen beobachteten Fehler.

**Beleg (Stand `6eae327a`), `crates/krk-ui/src/appkit/editor.rs`:**

- `stand_einsetzen`, Doc-Kommentar: „Reicht der Bereich über den Text der Fläche hinaus, trägt sie also nicht den Stand, gegen den er gebildet ist, oder gibt sie keinen Textspeicher her, wird der ganze Stand geschrieben“. Der Schluss gilt nur in einer Richtung: ein Bereich, der passt, sagt nichts darüber, ob der Stand stimmt.
- Rumpf: der Wächter ist `if bereich.location + bereich.length <= flaeche.string().length()`.
- Danach `debug_assert_eq!(flaeche.string().length(), koordinaten::einheiten(...stand()))`: im Release-Bau aus, und auch im Debug-Bau ein Vergleich der Längen und nicht der Zeichen.
- Rufer mit `Einsatz::Bereich`: `umbau_anwenden` (Tabellenhandlungen, Zellübernahme, „Eintrag hinzufügen“ in der Rohansicht über `eintrag_im_text_hinzufuegen`) und `umkehren` (`cmd+z`, `shift+cmd+z`).

**Richtung der Behebung:** Den Doc-Kommentar auf das bringen, was der Wächter hält (er verhindert die Objective-C-Ausnahme, mehr nicht), und die Vorbedingung „die Fläche trägt den Stand des Modells“ als Invariante benennen, mit den Wegen, die sie halten. Eine Prüfung der Zeichen wäre an 16 MiB so teuer wie das `setString:`, das der Teilersatz einspart; sie gehört deshalb nicht in den Release-Bau.

**Abnahme:** Der Doc-Kommentar von `stand_einsetzen` behauptet nichts, was der Rumpf nicht prüft, und nennt die Invariante samt den Wegen, die sie halten.
