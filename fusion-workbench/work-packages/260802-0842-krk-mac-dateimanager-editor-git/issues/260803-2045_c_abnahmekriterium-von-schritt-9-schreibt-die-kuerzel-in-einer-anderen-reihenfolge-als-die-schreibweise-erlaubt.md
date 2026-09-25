Das Abnahmekriterium von Schritt 9 schreibt vier Kürzel in einer Reihenfolge, die die Kombinationsschreibweise desselben Schrittes verbietet

---

Schritt 9 des Plans `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` legt unter `Änderungen` fest: "Die Kombinationsschreibweise ist `[ctrl+][opt+][shift+][cmd+]<taste>` in dieser festen Reihenfolge." Das Abnahmekriterium desselben Schrittes verlangt zwei Zeilen weiter, dass die sechs Zeilen der C3-Tabelle "mit genau den dort genannten Kürzeln" stehen, und zählt sie als Zeichenketten auf: `f3`+`cmd+y`, `f5`+`cmd+shift+k`, `f6`+`cmd+shift+v`, `f7`+`cmd+shift+n`, `f8`+`cmd+opt+delete`, `delete`+`cmd+delete`.

Vier dieser sechs Zeichenketten verletzen die Reihenfolge, die derselbe Schritt vorschreibt: `cmd+shift+k`, `cmd+shift+v` und `cmd+shift+n` setzen `cmd` vor `shift`, `cmd+opt+delete` setzt `cmd` vor `opt`. In der vorgeschriebenen Schreibweise heißen sie `shift+cmd+k`, `shift+cmd+v`, `shift+cmd+n` und `opt+cmd+delete`. Die beiden übrigen, `cmd+y` und `cmd+delete`, tragen nur eine Zusatztaste und sind von der Frage nicht berührt.

Beide Vorschriften zugleich sind nicht erfüllbar.

---

Herkunft: gefunden beim Schreiben von `resources/default-keymap.toml` (Plan Schritt 9).

Woher die Abweichung kommt: die Reihenfolge `ctrl, opt, shift, cmd` ist die, in der macOS die Zusatztasten schreibt (⌃⌥⇧⌘). Der Spec nennt dieselben Kürzel in C3 in der umgangssprachlichen Prosaform "Cmd+Shift+K". Das Abnahmekriterium hat diese Prosaform übernommen, ohne sie in die Schreibweise der Datei zu übersetzen.

Wie die Datei sich entschieden hat: sie folgt durchgehend der vorgeschriebenen Reihenfolge und schreibt `shift+cmd+k`, `shift+cmd+v`, `shift+cmd+n` und `opt+cmd+delete`. Der Grund ist die Begründung der Schreibweise selbst: sie ist die Form, die der Parser aus Schritt 11 liest, und eine Datei mit zwei Reihenfolgen nebeneinander zwingt den Parser zu einer Sonderregel. Die zugeordneten Funktionen sind unverändert die der C3-Tabelle; verschieden ist allein die Schreibweise.

Folge für die Prüfung: ein wörtliches `grep 'cmd+shift+k' resources/default-keymap.toml` findet nichts. Wer die sechs Zuordnungen prüfen will, greift die Funktionstasten ab: `grep -B2 -E '^tasten = \["(f3|f5|f6|f7|f8|delete)"' resources/default-keymap.toml`.

Was zu entscheiden ist: entweder das Abnahmekriterium von Schritt 9 nennt die vier Kürzel künftig in der Schreibweise der Datei, oder die Schreibweise gibt die Reihenfolge auf. Der erste Weg ändert vier Zeichenketten in der Plandatei, der zweite die Schreibweise und damit den Parser aus Schritt 11.

---
Resolved: Der erste Weg gilt. Das Abnahmekriterium von Schritt 9 nennt die vier Kürzel jetzt in der Schreibweise der Datei, also `shift+cmd+k`, `shift+cmd+v`, `shift+cmd+n` und `opt+cmd+delete`. Der Grund steht im Plan und nicht nur hier: der Parser aus Schritt 11 liest die Datei, die Reihenfolge der Zusatztasten ist sein Vertrag, und eine Kombination in anderer Reihenfolge wäre für ihn eine andere Kombination. `resources/default-keymap.toml` bleibt unverändert; die Datei stand schon richtig.

Die Durchsicht auf dieselbe Verwechslung an anderer Stelle hat zwei weitere Fundstellen ergeben und beide unangetastet gelassen. C3 des Specs nennt die Kürzel in der Mac-Prosaform "Cmd+Shift+K", was für Prosa die richtige Form ist. Das Abnahmekriterium von Schritt 7 nennt `cmd+shift+k` als Namen der Prüfung `cmd_shift_k_behaelt_beide_bits` aus `crates/krk-core/tests/tasten.rs`; auch dort gleicht kein Kriterium eine Zeichenkette gegen eine vom Parser gelesene Datei ab. Übersetzt wurde allein die Stelle, an der ein Kriterium die Zeichenkette wörtlich prüft.

Mitgenommen an derselben Zeile: die Vorschrift, `cmd+v` komme in keiner Tastenliste vor, prüft jetzt ausdrücklich den vollständigen Eintrag und nicht die Teilzeichenkette. `shift+cmd+v` ist das vom Spec vorgeschriebene Kürzel für das Verschieben und enthält `cmd+v`; ein roher Substring-Abgleich hätte am gewollten Eintrag angeschlagen.

Geändert: `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 9, Abnahmekriterium und zwei erläuternde Absätze darunter.
