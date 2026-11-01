Der Modulkopf von markdown.rs sagt, in einer Tabelle ohne äußere Striche liefere ein Wort das Wort; es liefert Strich und Leerraum mit
---
`crates/krk-ui/src/markdown.rs`, Modulkopf `# Tabellen`, letzter Satz des Absatzes „Was eine Auswahl liefert …“: „Eine Tabelle ohne aeussere Striche traegt an ihren Zeilen keine Klammer, dort liefert ein Wort das Wort.“ Gemessen liefert die Auswahl eines Wortes dort ein Bruchstück mit dem Strich der Nachbarzelle und dem Leerraum der Zelle. Keine Probe hält den Satz.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-0924-reviewer-schlusspruefung-markdown-tabellen.md; 260819-2216_*_welche-auszeichnungszeichen-fahren-an-den-raendern-der-auswahl-mit.md

## Beleg

Kopie des Baums auf `f70c6202`, nicht eingecheckte Probe über den Helfer `kopiert` im Prüfmodul:

```
Quelle "a | b\n--|--\n1 | 2\n"
kopiert(q, "b") → "| b"
kopiert(q, "1") → "1 "
kopiert(q, "a") → "a "
```

`"| b"` kommt von der Halbregel an `Quellbezug::quelltext`: der Abschnitt ohne Textzeichen für das `|` zwischen den Zellen liegt am Anfang von „b“ und fährt mit; die Zeile hat keine Klammer, also schließt die zweite Stufe nicht auf die ganze Zeile auf.

## Abnahme

Eines von beiden, und die Wahl steht im Bericht:

- Der Satz im Modulkopf beschreibt, was die Auswahl in einer Tabelle ohne äußere Striche wirklich liefert, und eine Probe neben `ein_wort_in_einer_zelle_kopiert_seine_quellzeile` hält es an `"a | b\n--|--\n1 | 2\n"` fest; oder
- das Verhalten wird so geändert, dass der Satz stimmt, mit derselben Probe. Ändert das die Klammerregel, ist es eine Frage an den Nutzer (Datensatz zur Klammerregel) und kein Bauschritt.
