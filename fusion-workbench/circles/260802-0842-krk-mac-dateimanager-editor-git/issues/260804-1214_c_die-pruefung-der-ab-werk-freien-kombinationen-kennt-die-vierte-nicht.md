Die Prüfung der ab Werk freien Kombinationen kennt die vierte nicht

---

`crates/krk-core/tests/belegung.rs:141-155` prüft unter dem Namen `die_drei_ab_werk_freien_kombinationen_kommen_nicht_vor`, dass `shift+delete`, `cmd+c` und `cmd+v` in keiner Tastenliste stehen. Seit S11c sind es vier: die Eingabetaste ist am 260804 freigeworden, und der Kopfkommentar von `resources/default-keymap.toml` führt sie seither als vierte auf.

Die Prüfung schlägt nicht fehl, sie deckt nur eine Kombination zu wenig ab. Ihr Name und ihr Kommentar ("C3: Umschalt+Entf, Cmd+C und Cmd+V bleiben unbelegt") sagen außerdem eine Dreizahl an, die die Datendatei nicht mehr trägt.

---

## Warum es zählt

Genau diese Prüfung ist der maschinelle Halt gegen eine versehentliche Wiederbelegung. Bei `return` wäre eine Wiederbelegung besonders naheliegend, weil sie bis zum 260804 bestand und in Dateimanagern die vertraute Taste für den Einstieg ist; der Nutzer hat die Freigabe ausdrücklich gegen diesen Hinweis gewählt (`planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Schritt 11c). Die eine Kombination, die den Schutz am ehesten braucht, ist die eine, die er nicht deckt.

## Was zu tun ist

`"return"` in die Liste aufnehmen und die Prüfung samt Kommentar auf vier umbenennen, etwa `die_vier_ab_werk_freien_kombinationen_kommen_nicht_vor`. Eine Zeile in der Liste, eine im Namen, eine im Kommentar.

Die Änderung gehört dem `coder`: sie fasst `crates/` an, und der Auftrag zu S11c schließt das aus.

## Abgrenzung

`ctrl+b` und `ctrl+s` sind mit S11c ebenfalls unbelegt geworden, gehören aber **nicht** in diese Liste und stehen auch nicht im Kopfkommentar. Die Liste führt Kombinationen, die ein Leser belegt erwartete und die ausdrücklich frei bleiben. `ctrl+b` und `ctrl+s` waren eine Behelfsbelegung, deren Grund weggefallen ist; sie sind schlicht nicht mehr vergeben.

---

Herkunft: gefunden bei der Umsetzung von Schritt 11c am 260804-1214, beim Abgleich des geänderten Kopfkommentars gegen die Prüfungen.

---

Resolved: 260804 — `"return"` steht in der Liste, und die Prüfung heißt jetzt `die_ab_werk_freien_kombinationen_kommen_nicht_vor`, ohne Zahl im Namen. Die Zahl im Namen war die eigentliche Falle: sie bindet die Prüfung an die Größe der Liste statt an ihre Zusage und verlangt bei jedem Zuwachs eine Umbenennung, so wie hier. Der Kommentar nennt jetzt die Zusage ("eine Kombination, die ein Leser belegt erwartete und die ausdrücklich frei bleibt, steht in keiner Tastenliste") und dahinter die vier Kombinationen mit je ihrem Grund.

`ctrl+b` und `ctrl+s` bleiben draußen, wie die Abgrenzung des Defekts es vorsieht, und aus einem zweiten Grund: die Liste sagt zu, dass eine Kombination frei *bleibt*. Für `ctrl+s` wäre das eine Zusage gegen den Editor späterer Runden, wo es die vertraute Sicherntaste ist. Der Grund steht als Kommentar in der Prüfung.
