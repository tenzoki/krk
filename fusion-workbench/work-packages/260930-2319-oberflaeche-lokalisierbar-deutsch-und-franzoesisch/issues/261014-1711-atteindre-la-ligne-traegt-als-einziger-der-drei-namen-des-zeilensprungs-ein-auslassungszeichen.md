„Atteindre la ligne…“ trägt als einziger der drei Namen des Zeilensprungs ein Auslassungszeichen
---
Der französische Name des Befehls „Zu Zeile springen“ endet seit `8fa6d6d` auf „…“, der deutsche und der englische nicht; die drei Tabellen setzen das Zeichen damit für denselben Befehl verschieden, und die deutsche Quelle trägt es nur an zwei Befehlen.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261014-1711-reviewer-abschlussdurchsicht-lokalisierung-luecke-der-zwei-durchsichten.md, 261014-1640-drei-sprach-und-typografiebefunde-in-fr-rs-u00a0-vor-semikolon-branchee-und-aller-a-la-ligne.md

Gelesen am Stand `8fa6d6d`. Schwere: Low. Der Sinn des Eintrags stimmt („Atteindre“ war die Berichtigung von „Aller à la ligne“, dem Zeilenumbruch); verschieden ist allein das Zeichen am Ende, und der Befund vom 261014 hatte es in seiner Zeile „soll“ mitgegeben, ohne die Abweichung von der Quelle zu nennen.

| Datei:Zeile | ist | soll | Grund |
|---|---|---|---|
| `crates/krk-core/src/sprache/tabelle/fr.rs:1175` (`Kommando::EditorZeileSpringen`) | `Atteindre la ligne…` | `Atteindre la ligne`, **oder** `de.rs:1109` `Zu Zeile springen…` und `en.rs:1055` `Go to Line…` dazu | Die drei Tabellen setzen das Zeichen am selben Befehl verschieden. Zählung über `awk '/fn kommandoname/,/^}/' <tabelle> \| grep -c '…'`: `de.rs` 2, `fr.rs` 3, `en.rs` 2; die zwei, die alle drei tragen, sind `OrtWaehlen` und `Werkseinstellungen`. Die deutsche Quelle setzt das Zeichen also nicht an jedem Befehl, der ein Blatt öffnet (`EditorSuchen`, `EditorErsetzen`, `PfadEingeben`, `PinAendern`, `Umbenennen`, `LesezeichenAnlegen` stehen ohne), und `fr.rs` folgt ihr an allen bis auf diesem einen. Die erste Form folgt der Quelle und ändert einen Eintrag; die zweite folgt der macOS-Konvention (ein Befehl, der weitere Eingabe braucht, endet auf „…“) und zieht dann der Sache nach auch die übrigen Blattöffner in allen drei Tabellen nach, was den deutschen Wortlaut und damit Wortlautproben ändert (`grep -rn 'Zu Zeile springen' crates` gibt heute außerhalb der Tabelle nichts, bei den anderen Befehlen ist das je zu prüfen). |

**Abnahme:** die drei Einträge zu `Kommando::EditorZeileSpringen` enden entweder alle drei auf „…“ oder keiner; `cargo test -p krk-core` ist grün, insbesondere `keine_zwei_funktionen_tragen_in_einer_sprache_denselben_namen` (`crates/krk-core/tests/belegung.rs`), weil ein Befehlsname geändert wird. Wird die zweite Form gewählt, gilt dieselbe Abnahme für jeden weiteren Befehl, der das Zeichen bekommt, und die Zählung mit `awk` gibt in allen drei Tabellen dieselbe Zahl.
