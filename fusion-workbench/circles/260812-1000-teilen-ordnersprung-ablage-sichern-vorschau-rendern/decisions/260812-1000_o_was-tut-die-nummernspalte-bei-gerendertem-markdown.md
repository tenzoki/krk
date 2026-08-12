# Was zeigt die Zeilennummernspalte, wenn die Vorschau Markdown gerendert darstellt?

---
**Domain:** code
**Status:** open
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `crates/krk-ui/src/appkit/nummernspalte.rs` (eine Klasse für Editor und Vorschau); `crates/krk-ui/src/vorschaumodell.rs:451` (`zeigt_dateitext`, vollständige Fallunterscheidung); `circles/260807-2116-eingebauter-editor-mit-textmarken/_*_circle.md` (C10 der Runde 2, die Zusage einer Anzeige für beide Flächen)

---

## Question

Die Runde 2 hat der Vorschau eine Zeilennummernspalte gegeben, und zwar bewusst **dieselbe Klasse**, die der Editor benutzt: C10 sagt eine Anzeige für beide Flächen zu und nicht zwei ähnliche. Ob sie in der Vorschau steht, entscheidet allein `Vorschaumodell::zeigt_dateitext`, und zwar so: sie steht beim rohen Inhalt einer Textdatei und weder bei einem Bild noch bei Metadaten, einem Hinweis, einem leeren Tab oder Text aus der Zwischenablage. Gezählt wird über `krk_core::text::zeilen`.

Mit gerendertem Markdown stimmt die Rechnung nicht mehr. Eine Überschriftszeile bleibt zwar eine Zeile, aber ein Absatz, der im Quelltext eine lange Zeile ist, bricht in der Anzeige über mehrere Bildschirmzeilen um; und wenn die Auszeichnungszeichen verschwinden, verschiebt sich die Höhe jeder ausgezeichneten Zeile gegen die Rohansicht. Die Nummern neben dem gerenderten Text zeigen dann Dateizeilen, die nicht neben dem stehen, was sie nummerieren.

Die Frage ist zu stellen, weil sie eine abgenommene Zusage der Runde 2 berührt und weil die geteilte Klasse der Punkt ist, an dem eine unbedachte Antwort zwei Flächen auseinanderreißt.

Sie hält keinen Planschritt auf und bindet einen.

## Options

1. **Bei gerendertem Markdown keine Nummernspalte.** `zeigt_dateitext` bekommt den Markdown-Fall als weiteren Zweig, der `false` liefert.
   - Folge: die Spalte zeigt nie eine falsche Zahl. Die Änderung ist eine Zeile in einer Fallunterscheidung, die ohnehin angefasst wird, und die geteilte Klasse bleibt unberührt. Der Editor behält seine Spalte für Markdown, weil er die Zeichen stehen lässt und dort die Zahlen stimmen.
   - Preis: dieselbe Datei hat im Editor Nummern und in der Vorschau keine. Das ist ein weiterer Fall des Unterschieds, den Festlegung A ohnehin angenommen hat.

2. **Die Spalte bleibt und zählt die Dateizeilen weiter.** Die Nummern stehen neben dem gerenderten Text, so gut sie treffen.
   - Folge: keine Änderung an `zeigt_dateitext`, keine an der Klasse.
   - Preis: die Zahlen sind an jeder Stelle, an der die Anzeige höher oder niedriger ist als die Rohform, falsch. Eine Zeilennummer, die um zwei danebensteht, ist schlechter als keine: sie sieht aus wie eine Auskunft und ist keine.

3. **Die Spalte bleibt und zählt Anzeigezeilen statt Dateizeilen.** Nummeriert wird, was auf dem Schirm eine Zeile ist.
   - Folge: die Zahlen stimmen zu dem, was danebensteht.
   - Preis: sie beantworten dann eine andere Frage als im Editor, wo sie Dateizeilen zählen. Zwei Bedeutungen derselben Anzeige in einem Programm, und die Änderung träfe die geteilte Klasse, also auch den Editor. Das ist derselbe Fehler, den die Runde 4 an `let _ =` gefunden hat: dasselbe Zeichen mit zwei entgegengesetzten Bedeutungen im selben Baum.

## Constraints

- `nummernspalte.rs` ist **eine** Klasse für zwei Flächen, und C10 der Runde 2 sagt genau das zu. Eine Antwort darf keine zweite Klasse daneben anlegen.
- `zeigt_dateitext` ist eine vollständige Fallunterscheidung ohne Auffangzweig: ein sechster Inhalt hält den Bau an und erzwingt die Antwort auf die Frage, ob neben ihm Zeilennummern stehen. Wenn diese Runde `Inhalt` um eine Variante erweitert, hält der Bau hier an, und das ist gewollt.
- Die Zählung kommt aus `krk_core::text::zeilen`, also aus dem Kern und ohne AppKit. Anzeigezeilen kennt der Kern nicht und soll er nicht kennenlernen.

## Recommendation

**Wir empfehlen Möglichkeit 1.** Sie ist die einzige, die keine falsche Auskunft erzeugt und die geteilte Klasse nicht anfasst. Möglichkeit 3 sähe zwar am besten aus, kostet aber die Eindeutigkeit der Anzeige über zwei Flächen hinweg, und dieses Projekt hat für genau diesen Fehler bereits einen Defekt bezahlt.

Der Preis von Möglichkeit 1 ist gering. Wer Zeilennummern zu einer Markdown-Datei braucht, öffnet sie mit `f4` im Editor; dort stehen sie und stimmen.

---
Answered:
Implemented:
Deferred:
Superseded by:
