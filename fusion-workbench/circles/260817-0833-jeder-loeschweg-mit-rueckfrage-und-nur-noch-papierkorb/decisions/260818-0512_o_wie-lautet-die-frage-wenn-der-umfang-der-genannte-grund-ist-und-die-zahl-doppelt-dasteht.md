# Wie lautet die Frage, wenn der Umfang der genannte Grund ist und die Zahl doppelt dasteht?

---
**Domain:** code
**Status:** open
**Filed by:** coder
**Cross-references:**
`issues/260817-1720_c_the-question-can-read-diese-25-eintraege-mit-25-eintraegen.md`
(der Befund, aus dem diese Frage hervorgeht),
`shared/planning/260817-0536_*_spec-absicherung-jedes-loeschwegs.md`, C3
(die Tafel, deren Spalte „Wortlaut in der Frage" der Nutzer am Spec-Gate angenommen hat)

---

## Frage

Der sechste Auslöser der lauten Rückfrage ist der einzige, dessen Wortlaut eine
**Zahl** ist, und er steht in der Frage genau neben einer anderen Zahl. Ist der
Umfang der genannte Grund und ist die Auswahl flach, sind beide Zahlen dieselbe
Zahl, und der Satz sagt sie zweimal:

```text
  Auswahl:  25 Dateien, keine Ordner        ⇒ Umfang::Genau(25)
  Frage:    "Diese 25 Einträge mit 25 Einträgen in den Papierkorb räumen?"
```

Dasselbe in der „mehr als"-Form, sobald die Auswahl selbst über der Schwelle
liegt: „Diese 30 Einträge mit mehr als 25 Einträgen in den Papierkorb räumen?"

Die erste Zahl zählt die ausgewählten Zeilen, die zweite den Unterbaum. Bei
einer flachen Auswahl sind das dieselben Einträge. Die sechs übrigen Wortlaute
sind ortsbezogen und lesen sich an derselben Stelle sauber („Diese 3 Einträge
von einem Netzlaufwerk in den Papierkorb räumen?").

**Warum das über Lesbarkeit hinausgeht.** Beide Zahlen sind richtig, und der
Nutzer wird über nichts getäuscht — der Befund steht deshalb auf Niedrig. Der
Satz lässt sich aber falsch lesen: „25 Einträge mit 25 Einträgen" legt nahe, es
seien zwei verschiedene Mengen, also fünfzig. Die Rückfrage ist der eine Schutz,
den diese Runde gegen einen zweiten Schadensfall baut, und ihr Wortlaut ist das,
was der Nutzer im Ernstfall liest.

## Warum sie nicht nebenbei entschieden ist

Die beiden Wortlaute stehen wörtlich in der Spalte „Wortlaut in der Frage" der
C3-Tafel des Specs, und der Spec ist angenommen und bindend. Sein
Abnahmekriterium ist über die Zahl ausdrücklich: „Umfasst der Unterbaum des
Vorgangs 25 Einträge, trägt die Frage die Zahl 25. Umfasst er mehr, trägt sie
'mehr als 25'." Jede Umformulierung ändert Text, den der Nutzer am Gate
abgenommen hat.

## Möglichkeiten

Vier Wege, und keiner ist offensichtlich richtig; deshalb steht die Frage hier
statt einer Änderung am Baum.

1. **Es bleibt, wie es steht.** Kosten: keine. Preis: der eine Satz, um den
   diese Runde läuft, liest sich in einem Fall schlecht, der nicht selten ist —
   eine flache Auswahl an der Schwelle. **Was das nach unten festlegt:** nichts;
   die Frage lässt sich jederzeit wieder aufmachen, weil kein Text und keine
   Probe wandert.

2. **Die zweite Zahl sagt, was sie zählt:** „mit 25 Einträgen insgesamt" und
   „mit mehr als 25 Einträgen insgesamt". Ein Wort, die Zahl und die Form „mehr
   als" bleiben, und aus der Doppelung wird ihre eigene Erklärung.
   **Downstream:** zwei Zeichenketten in `Warngrund::wortlaut`
   (`crates/krk-ui/src/kommandos/loeschwarnung.rs:604-605`), zwei Zeilen der
   ausgeschriebenen Wortlauttafel in `die_tafel_der_sieben_wortlaute` (`:1463`
   und `:1467`) und eine Zeile der Erläuterungsprobe (`:1770`). Die
   Übersetzungszeit-Zusicherung `nennt_die_zahl` hält weiter, weil die Zahl im
   Wortlaut stehen bleibt. Die Erläuterung liest sich mit dem Zusatz ebenfalls
   noch („Außerdem: mit mehr als 25 Einträgen insgesamt."). **Preis:** die
   Spalte des Specs stimmt nicht mehr wörtlich, das Abnahmekriterium schon.

3. **Die Zahl der Einträge fällt aus der Frage, wenn der Umfang der genannte
   Grund ist:** „Diesen Vorgang mit mehr als 25 Einträgen in den Papierkorb
   räumen?" Die Kollision ist an der Wurzel weg. **Downstream:** C2 verlangt
   für die erste Zeile, dass sie „nennt, wie viele Einträge betroffen sind";
   dieser Weg bricht das für einen von sieben Fällen und braucht deshalb eine
   Wiederlesung von C2 mit. `frage_und_erlaeuterung` bekäme einen Zweig nach dem
   genannten Grund, also eine Fallunterscheidung, die die Funktion heute nicht
   hat, und die drei Fragenproben (`:1273`, `:1313`, `:1765`) bekämen eine
   vierte daneben. **Preis:** die einzige Zahl in der ersten Zeile ist dann eine
   über den Unterbaum, und der Nutzer erfährt nicht mehr, wie viele Zeilen er
   ausgewählt hat.

4. **Der Umfang wird kein genannter Grund mehr**, sondern steht nur noch in der
   Erläuterung; die Frage nennt den nächstrangigen Grund oder bleibt stumm.
   **Downstream:** das widerspricht der Rangfolge aus C3, die den Umfang
   ausdrücklich als Grund letzter Instanz führt, und hätte zur Folge, dass eine
   Auswahl von 4000 Einträgen in einem gewöhnlichen Ordner eine **ruhige**
   Rückfrage stellt. Das ist die einzige der vier Möglichkeiten, die das
   Verhalten und nicht nur den Wortlaut ändert. **Preis:** die Schwelle verliert
   ihre Wirkung als Warngrund; sie wäre nur noch eine Zeile in der Erläuterung.

## Randbedingungen

- Das Abnahmekriterium „trägt die Zahl 25 beziehungsweise 'mehr als 25'" bindet
  die Möglichkeiten 1, 2 und 3; Möglichkeit 4 lässt es leerlaufen.
- Die Zusicherung `const _: () = assert!(nennt_die_zahl(…))`
  (`loeschwarnung.rs`) bindet beide Wortlaute an `SCHWELLE`. Jede Neufassung
  muss die Dezimalschreibung der Schwelle im Wortlaut behalten, sonst hält der
  Bau an. Die Möglichkeiten 1 bis 3 tun das, 4 nimmt den Wortlaut nicht weg und
  ist ebenfalls verträglich.
- Die Wortlaute sind Fügungen und keine Sätze, weil sie in beide Texte passen
  müssen: in die Frage als Einschub und in die Erläuterung als Glied einer
  Aufzählung. Eine Neufassung, die nur in der Frage trägt, ist keine.

## Empfehlung

Möglichkeit 2. Sie kostet ein Wort und fünf Zeilen am Baum, hält beide
Abnahmekriterien und die Übersetzungszeit-Zusicherung, und sie ändert kein
Verhalten — nur den Text, an dem der Befund hängt. Möglichkeit 3 löst die
Doppelung sauberer, verlangt aber eine Wiederlesung von C2 und nimmt dem Nutzer
die Zahl, die ihm sagt, wie viel er markiert hat. Möglichkeit 4 ist als einzige
eine Verhaltensänderung und sollte nicht aus einem Lesbarkeitsbefund folgen.
