# An welcher Stelle der Bedeutungen von `Esc` steht der Filtertext?

---
**Domain:** code
**Status:** open
**Filed by:** shaper
**Cross-references:** `crates/krk-ui/src/appkit/anwendung.rs:4318-4338` (`abbrechen`, die heutige Reihenfolge: Blatt, dann laufende Operation); `resources/default-keymap.toml:405-408` (`abbrechen`, belegt mit `esc`); `crates/krk-ui/src/kommandos/operationen.rs:267` (`waehrend_blatt_erlaubt`, die eine Zeile, die `Abbrechen` während eines Blattes durchlässt); `circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/_t_circle.md` (`## Directive`)

---

## Question

Die Directive sagt: „`Esc` nimmt zuerst den Filtertext zurück, bevor es seine übrigen Bedeutungen bekommt." Diese Vorbelegung ist entstanden, bevor die übrigen Bedeutungen im Einzelnen benannt waren. `Anwendungsdelegierter::abbrechen` trägt heute zwei, in fester Reihenfolge: ein stehendes Blatt schließen, sonst eine laufende Dateioperation abbrechen. Nach dieser Runde kommt der Filtertext dazu, und wo er einzuordnen ist, entscheidet einen Fall, der im Alltag vorkommt.

Der Fall: der Nutzer filtert, markiert die Treffer, kopiert sie mit F5, und die Operation läuft. Der Filtertext steht weiter. Er drückt `Esc`, um das Kopieren anzuhalten. Steht der Filtertext an erster Stelle, wird stattdessen die Liste wieder lang, das Kopieren läuft weiter, und erst der zweite Druck hält es an. Kein Datenverlust, aber der Tastendruck tut das Gegenteil dessen, was der Nutzer meinte.

Ein zweiter Fall entscheidet sich mit: ein laufender Durchlauf über den Unterbaum hört auf, sobald der Filtertext weg ist, denn ohne Filtertext hat er keinen Gegenstand. Das Anhalten des Durchlaufs braucht deshalb keinen eigenen Rang, gleich welche Antwort fällt.

## Options

1. **Filtertext zuerst, wie in der Directive vorbelegt.** Reihenfolge: Filtertext, Blatt, laufende Operation.
   - Pro: hält die Vorbelegung wörtlich. Der Filter ist das, was der Nutzer beim Drücken vor sich hat, und `Esc` räumt zuerst den Schirm.
   - Kontra: eine laufende Dateioperation und ein stehendes Blatt sind beide dringender als ein Filtertext, und beide sind heute über `Esc` erreichbar. Ein Blatt hinter dem Filter wäre außerdem unerreichbar, denn `waehrend_blatt_erlaubt` lässt allein `Abbrechen` durch, und dieser eine Weg ginge dann an den Filter.
2. **Filtertext zuletzt.** Reihenfolge: Blatt, laufende Operation, Filtertext.
   - Pro: keine bestehende Bedeutung von `Esc` rückt nach hinten. `Esc` heißt weiter „halte an, was läuft", und der Filter ist das, was übrigbleibt, wenn nichts läuft. Der Fall aus der Frage verhält sich, wie der Nutzer es meint.
   - Kontra: kehrt die Vorbelegung um. Wer während eines Kopiervorgangs den Filter loswerden will, muss warten oder zweimal drücken.
3. **Filtertext an zweiter Stelle.** Reihenfolge: Blatt, Filtertext, laufende Operation.
   - Pro: das Blatt bleibt unerreichbar hinter nichts, und der Filter steht vor der Operation, wie die Vorbelegung es nahelegt.
   - Kontra: trägt den Nachteil von Möglichkeit 1 für die laufende Operation unverändert und gewinnt dafür nichts, was Möglichkeit 2 nicht auch hätte.

## Constraints

- Die Reihenfolge ist eine vollständige Fallunterscheidung und bleibt eine: `Esc` tut genau eines je Druck.
- Ein stehendes Blatt steht in jeder Antwort vorn. `waehrend_blatt_erlaubt` lässt allein `Abbrechen` durch, und wer diesen Weg an den Filter gibt, macht das Blatt über die Tastatur unschließbar.
- Der laufende Durchlauf über den Unterbaum bekommt keinen eigenen Rang. Er endet mit dem Filtertext, weil er ohne ihn gegenstandslos ist.
- Steht kein Filtertext, verhält sich `Esc` in jeder Antwort wie heute.
- Die Antwort gilt auch für das Ankreuzfeld „Deep": es bleibt stehen, wenn `Esc` den Filtertext löscht. `Esc` schaltet keinen Schalter.

## Recommendation

Möglichkeit 2. `Esc` heißt in diesem Baum seit der Runde 1 „halte an, was läuft", und ein Filtertext läuft nicht. Wir haben die Vorbelegung „zuerst" so gelesen, dass sie den Filter vor die Bedeutungslosigkeit einer freien Taste setzen wollte und nicht vor das Abbrechen eines laufenden Kopiervorgangs; diese Lesart ist eine Erschließung und keine Aussage des Nutzers, und deshalb liegt die Frage hier statt im Spec. Der Spec fährt bis zu einer Antwort auf Möglichkeit 2, und C1 nennt die Stelle.

---
Answered:
Implemented:
Deferred:
Superseded by:

---

## Abgleich 260815-1216 (reconciler, Stand `9a2d0e0`)

**Diese Frage ist weder beantwortet noch gegenstandslos geworden.** Der Marker bleibt `_o_`.

**Der Baum fährt auf der Empfehlung, Möglichkeit 2, ohne Antwort des Nutzers.** `Anwendungsdelegierter::abbrechen` (`crates/krk-ui/src/appkit/anwendung.rs:4565-4588`) prüft in dieser Reihenfolge: ein stehendes Blatt schließen, eine laufende Dateioperation abbrechen, und erst dann `filter_leeren` auf dem sichtbaren Tab des aktiven Dateifensters. Der Kommentar an der dritten Stelle nennt sie ausdrücklich „den dritten Rang". C1.7 des Spec beschreibt dieselbe Reihenfolge und hält fest, dass sie sich mit einer anderen Antwort ändert.

**Die Sitzung vom 260815-0912 hat die Tragweite der Frage vergrößert, ohne sie zu berühren.** Seit `897605e` übersteht der Filtertext jeden Ordnerwechsel (`decisions/260814-1830_i_bleibt-der-filtertext-…`, Möglichkeit 2). `Esc` ist damit der einzige Griff, der einen stehenden Filtertext in einem Zug wegnimmt — die Rückschritt-Taste nimmt ihn Zeichen für Zeichen zurück, und kein Ordnerwechsel räumt ihn mehr ab. Ein Filtertext steht deshalb länger und über mehr Ordner hinweg als zur Zeit der Fragestellung, und der Fall aus dem Abschnitt `## Question`, in dem `Esc` während eines laufenden Kopiervorgangs gedrückt wird, tritt entsprechend häufiger bei stehendem Filtertext ein. Am Abwägungsstoff der drei Möglichkeiten ändert das nichts.
