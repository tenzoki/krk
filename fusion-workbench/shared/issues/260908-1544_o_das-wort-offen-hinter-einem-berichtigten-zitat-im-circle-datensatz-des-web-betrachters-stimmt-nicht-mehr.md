Das Wort „offen" hinter einem berichtigten Zitat im Circle-Datensatz des Web-Betrachters stimmt nicht mehr

---

Der Circle-Datensatz von `260804-0933-eingebauter-web-betrachter-im-vorschaufenster`, Abschnitt
`### 4. Die Messreihe hinter der dritten offenen Frage ist schlechter geworden`, schreibt
hinter das Zitat auf
`260812-2133_*_merkzeichen-einloesen-kostet-bei-tiefer-verschachtelung-das-zweieinhalbfache-und-verfehlt-l7-frueher.md`
das Wort „offen". Der zitierte Datensatz trägt heute `_c_`.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** Werkbankführung
**Baumstand:** `2f4b7b2`
**Schwere:** gering. Kein Bau, kein Verhalten. Der Zeiger löst auf; falsch ist die
Zustandsangabe daneben.

## Wie er entstanden ist

Beim Behebungslauf am 260908-1539 zu
`260818-0752_*_ein-zitat-im-circle-datensatz-des-web-betrachters-nennt-einen-namensteil-den-es-nie-gab.md`
gefunden. Jener Datensatz verlangt die Berichtigung des Namensteils und begründet, dass die
Umgebung richtig bleibe: „Der Datensatz traegt weiter `_o_`, also bleibt die Aussage der
Umgebung richtig." Diese Voraussetzung ist seither entfallen. Der Behebungslauf hat den
Namensteil gezogen und die Zustandsangabe unangetastet gelassen, weil sie eine Aussage über
einen Zustand ist und keine Adresse.

## Warum er nicht nebenbei behoben ist

Zweierlei ist zu entscheiden und nichts davon ist abzuleiten:

1. Ob das Wort auf den heutigen Zustand gezogen wird oder als Aussage über den 260812er Stand
   stehen bleibt. Der Absatz ist Teil eines `## Activation proposal` vom 260812-2307, also die
   Aufzeichnung eines Standes; ein Circle-Datensatz steht aber nicht in den sieben Speichern
   der Ortsregel aus `CLAUDE.md` und ist danach lebender Text.
2. Ob ein Circle-Datensatz eines abgesagten Circles überhaupt noch nachgeführt wird. Der Circle
   trägt seit dem 260821-2202 `_d_` und wird nie aktiviert
   (`260821-2202_*_zeigt-krk-web-inhalt-selbst-an-oder-gibt-er-ihn-an-den-systembrowser-ab.md`,
   Möglichkeit 2).

Die zweite Frage entscheidet die erste mit: wird der Datensatz nicht mehr nachgeführt, ist die
Zustandsangabe eine Aufzeichnung und bleibt.

## Abnahme

Entweder das Wort steht auf dem Zustand, den die Datei heute trägt, oder der Datensatz nennt
namentlich, dass es als Aufzeichnung stehen bleibt, mit der Regel, aus der das folgt.
