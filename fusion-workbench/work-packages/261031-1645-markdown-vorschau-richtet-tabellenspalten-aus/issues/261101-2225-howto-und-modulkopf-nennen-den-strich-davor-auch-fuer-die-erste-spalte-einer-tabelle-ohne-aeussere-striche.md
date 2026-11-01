HowTo.md und der Modulkopf von markdown.rs sagen, ein Wort in einer Tabelle ohne äußere Striche liefere den Strich davor; in der ersten Spalte gibt es keinen
---
Seit `194c643a` steht in `HowTo.md` (Abschnitt „Editor und Vorschau“, Absatz „Eine Tabelle in einer Markdown-Datei …“): „allein in einer Tabelle ohne äußere Striche liefert ein Wort nur sich selbst, mit dem Leerraum seiner Zelle und dem Strich davor.“ Der Modulkopf von `crates/krk-ui/src/markdown.rs` (`# Tabellen`) sagt dasselbe als Regel. Für ein Wort der ersten Spalte gilt sie nicht: es liefert sich mit dem Leerraum dahinter und ohne Strich. Der Modulkopf zeigt das gleich danach am Beispiel („a“ liefert `a `) und widerspricht damit seiner eigenen Regel; `HowTo.md` hat kein Beispiel, der Leser bekommt nur die Regel.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-2225-reviewer-nachpruefung-markdown-tabellen.md; 261101-0924-der-modulkopf-sagt-in-einer-tabelle-ohne-aeussere-striche-liefert-ein-wort-das-wort.md

## Beleg

Kratzprobe über den Helfer `kopiert` an einer Kopie aus `git archive 194c643a`, nicht eingecheckt:

```
"a | b | c\n--|--|--\n1 | 2 | 3\n"
  kopiert "a" -> "a "     kopiert "b" -> "| b "     kopiert "c" -> "| c"
  kopiert "1" -> "1 "     kopiert "2" -> "| 2 "     kopiert "3" -> "| 3"
```

Die erste Spalte bringt keinen Strich mit. Die Probe `ein_wort_ohne_aeussere_striche_kopiert_sich_mit_strich_und_leerraum` hält die Werte für „a“ und „1“ fest, sie stimmen mit dem Beispiel überein und nicht mit der Regel.

## Abnahme

- `HowTo.md` und der Modulkopf sagen, dass der Strich davor nur ab der zweiten Spalte mitkommt, oder sie sagen es so, dass es für jede Spalte stimmt (etwa: „mit dem Leerraum seiner Zelle und dem Strich, der sie von der vorigen trennt“).
- Nur Prosa. `cargo test -p krk-ui --bin krk markdown::` bleibt grün.
