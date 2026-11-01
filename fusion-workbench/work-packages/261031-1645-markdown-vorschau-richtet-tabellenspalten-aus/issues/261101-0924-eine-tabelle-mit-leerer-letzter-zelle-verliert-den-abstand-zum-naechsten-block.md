Eine Tabelle, deren letzte Zelle leer ist, verliert den Abstand zum nächsten Block
---
`Zerlegung::absetzen` (`crates/krk-ui/src/markdown.rs`) füllt den Abstandswunsch auf, indem es die `\n` am Textende zählt. Das `ZELLENENDE` einer leeren letzten Zelle ist ein solches `\n` und zählt als Leerzeile mit. Der Absatz oder die Tabelle danach steht dann ohne Leerzeile unmittelbar unter der Tabelle; zwei Tabellen hintereinander stehen ohne Abstand aufeinander.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-0924-reviewer-schlusspruefung-markdown-tabellen.md; 261031-1720-plan-markdown-vorschau-richtet-tabellenspalten-aus.md (Risikotabelle, Zeile „Der Wunschmechanismus der Abstände verschluckt ein leeres Zellende“)

## Beleg

Gefahren an einer Kopie des Baums auf `f70c6202` mit einer nicht eingecheckten Probe über `kachelung_pruefen`:

```
"| a | b |\n|---|---|\n| 1 |  |\n\nDanach.\n"  → "a\nb\n1\n\nDanach."   (Zelle (1,1) = Text 6..7, "Danach." ab 7)
"| a | b |\n|---|---|\n| 1 | 2 |\n\nDanach.\n"  → "a\nb\n1\n2\n\nDanach." (eine Leerzeile vor "Danach.")
"| a |\n|---|\n|  |\n\n| b |\n|---|\n| 2 |\n"   → "a\n\nb\n2\n"          (Tabelle 1 beginnt bei 3, direkt hinter der leeren Zelle bei 2)
```

Im ersten und dritten Fall ist das letzte `\n` der Absatz der leeren Zelle und keine Leerzeile. `absetzen` zählt `self.text.chars().rev().take_while(|z| *z == '\n')` und findet die zwei, die `ABSATZABSTAND` verlangt.

Die Probe `ein_absatz_nach_einer_tabelle_steht_hinter_einer_leerzeile` nimmt eine gefüllte letzte Zelle; `PLANTABELLE` endet mit einer leeren, aber ohne Block dahinter. Darum ist es keiner Probe aufgefallen.

## Abnahme

- Hinter einer Tabelle, deren letzte Zelle oder letzte Zellen leer sind, steht vor dem nächsten Block dieselbe Leerzeile wie hinter einer gefüllten: der erste Fall oben ergibt `"a\nb\n1\n\n\nDanach."`, der dritte `"a\n\n\nb\n2\n"` (oder die gleichwertige Form, die das Zellende vom Abstand trennt).
- Eine Probe im Prüfmodul von `markdown.rs` hält beide Fälle; `die_kachelung_deckt_quelle_und_text_lueckenlos` bleibt grün, und `KACHELBEISPIELE` bekommt den ersten Fall dazu.
- Die Lösung zählt Zellenden nicht als Abstand; sie macht keinen Sonderfall für „leere Zelle“ in `absetzen` an anderer Stelle als dem Tabellenende.
