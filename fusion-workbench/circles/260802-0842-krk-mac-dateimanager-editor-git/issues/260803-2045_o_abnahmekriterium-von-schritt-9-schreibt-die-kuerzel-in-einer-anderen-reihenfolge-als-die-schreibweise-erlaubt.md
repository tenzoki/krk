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
