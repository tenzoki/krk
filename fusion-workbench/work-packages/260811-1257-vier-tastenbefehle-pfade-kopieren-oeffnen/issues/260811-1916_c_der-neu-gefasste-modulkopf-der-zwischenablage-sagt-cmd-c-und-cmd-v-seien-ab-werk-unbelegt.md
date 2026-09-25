Der neu gefasste Modulkopf der Zwischenablage sagt zu, Cmd+C und Cmd+V seien ab Werk unbelegt

---

`crates/krk-ui/src/appkit/zwischenablage.rs:53-54` sagt seit `d23bfdb`: "Cmd+C und Cmd+V bleiben ab Werk unbelegt, wie es C3 der Runde 1 zugesagt hat; die beiden Kopierer liegen daneben."

Der Baum sagt das Gegenteil. `resources/default-keymap.toml:712-722` führt `text_kopieren` auf `cmd+c` und `text_einfuegen` auf `cmd+v`, beide mit `gehalten_von = "menue"`. Der Kopf derselben Datei schreibt den Wechsel ausdrücklich aus (`resources/default-keymap.toml:64-68`): "Cmd+C und Cmd+V standen bis zum 260805 in dieser Aufzählung. Sie tragen seither die Textbefehle des Menues 'Bearbeiten' und sonst nichts."

---

**Der Satz ist der Sorte, die diese Runde beseitigen wollte.** Der Spec führt unter `## Zwei schriftliche Zusicherungen, die diese Runde bricht` genau diese Datei, der Plan erweitert die Liste in Befund 4 auf fünf Stellen. Der Schritt S2 hat den Absatz um die gebrochene Zusage ("KRK schreibt die Zwischenablage in keinem Fall") neu gefasst und dabei den benachbarten Halbsatz über Cmd+C und Cmd+V mitgenommen, statt ihn zu prüfen. Er war schon vor dieser Runde falsch, seit dem 260805; die alte Fassung lautete knapp "Cmd+C und Cmd+V bleiben ab Werk unbelegt (C3)". Die neue Fassung bekräftigt ihn zusätzlich mit "wie es C3 der Runde 1 zugesagt hat" und trägt ihn damit als Aussage dieses Commits.

Am Verhalten ändert sich nichts: es ist ein Kommentar. Der Preis ist derselbe wie bei den fünf Stellen, die der Plan geführt hat — wer die Datei liest, um zu erfahren, was KRK mit der Zwischenablage tut, liest hier die Lage von vor dem 260805.

**Vorschlag für die Behebung.** Den Halbsatz durch die Lage ersetzen, die `resources/default-keymap.toml:64-68` schon ausschreibt: Cmd+C und Cmd+V tragen seit dem 260805 die Textbefehle des Menues "Bearbeiten" und sonst nichts, und genau das hält sie für eine Dateizwischenablage einer späteren Runde frei. Die beiden Pfadkopierer liegen auf `opt+cmd+c` und `shift+cmd+c` daneben.

Gefunden vom `coderev` am 260811 bei der Durchsicht des Turns 1 dieses Circles.

---
Resolved: Der Satz sagt jetzt, dass Cmd+C und Cmd+V seit dem 260805 die Textbefehle des
Menues Bearbeiten tragen und die Reservierung aus C3 der Runde 1 damit **eingeloest** und nicht
gebrochen ist. Am Bestand nachgesehen: `default-keymap.toml:713-722` traegt `text_kopieren` auf
`cmd+c` und `text_einfuegen` auf `cmd+v`, beide mit `gehalten_von = "menue"`; die Pfadkopierer
stehen auf `opt+cmd+c` und `shift+cmd+c`.

**Der Befund war neu durch S2 entstanden** und damit die einzige falsche Zusicherung, die diese
Runde selbst hinzugefuegt hat. Genau dafuer ist die Durchsicht da.

Geschlossen in der Sitzung `history/260811-1454-orchestrator-session.md`, Turn 1. Abgenommen mit `make check`, exit 0.

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behauptung traegt.**
`crates/krk-ui/src/appkit/zwischenablage.rs:53-58` sagt jetzt, Cmd+C und Cmd+V trugen seit dem
260805 die Textbefehle des Menues „Bearbeiten" und die Reservierung aus C3 der Runde 1 sei damit
eingeloest. Am Bestand nachgesehen: `resources/default-keymap.toml:712-722` fuehrt `text_kopieren`
auf `cmd+c` und `text_einfuegen` auf `cmd+v`, beide mit `gehalten_von = "menue"`.
