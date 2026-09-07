# Wie wird die Naht zwischen Umlaut und Umschrift gehalten, jetzt da sie eine Regel ist?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md` (die Regel, die gehalten werden soll), `260907-0826_*_gilt-die-umlautregel-auch-fuer-die-terminalausgabe-von-xtask-krk-bench-und-messmodus.md` (dieselbe Naht, Frage nach ihrem Verlauf), `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` (dieselbe Bauart von Frage: eine Gewohnheit prüfbar machen)

---

## Question

Die Schreibweise nutzersichtbarer Meldungen ist seit dem 260907 entschieden, und der Durchgang
danach hat KRKs Oberfläche durchgehend auf Umlaute gebracht. Gehalten wird die Regel damit von
nichts: sie war vorher eine Gewohnheit, und genau daran ist sie auseinandergelaufen. Der
nächste Autor, der eine Meldung schreibt, bekommt vom Übersetzer, von `clippy` und von jeder
Probe dieselbe Auskunft wie vorher, nämlich keine.

**Die tragende Frage ist nicht entscheidbar, und das ist der Befund.** Eine Probe hat als
Eingabe den Quelltext des Baums. Aus ihm ist zu entscheiden, ob ein Wort Umschrift trägt — das
geht über eine feste Stammliste. Nicht zu entscheiden ist daraus, ob eine Zeichenkette
**nutzersichtbar** ist: sie wird über `format!` aus Bruchstücken zusammengesetzt, wandert durch
`io::Error`, `Grund::Beschaedigt`, `Steuerung::ueberspringen` und `Ersetzung::fmt`, und ob sie
am Ende in der Statuszeile ankommt oder in einer `debug_assert!`-Meldung, steht nirgends im
Text der Zeichenkette. Erlebt am Bestand: `"kein Papierkorb eingehaengt"` in
`operation/loeschen.rs` und `"eine nicht genommene Sperre schuetzt nichts"` in
`verzeichnis/sys.rs` sind beide deutsche Prosa in einem Stringliteral, das erste geht in die
Abschlussliste eines Vorgangs, das zweite ist eine `#[must_use]`-Marke. Kein Muster über den
Quelltext trennt sie.

Wer die Sichtbarkeit trotzdem prüfen will, muss die Stellen aufzählen — und eine
Aufzählung von Stellen ist die Ausnahmeliste, die dieses Projekt anderswo abgeschafft hat.
Die Frage ist deshalb, welche **andere** Frage, aus Eingaben beantwortbar, die die Probe
wirklich hat, denselben Zweck erfüllt.

## Options

1. **Nichts halten; die Regel steht im Entscheidungsdatensatz und in `CLAUDE.md`.**
   - Pro: Kostet nichts und behauptet nichts. Die Regel ist an jeder Zeichenkette
     entscheidbar, sobald jemand sie kennt; das Problem ist die Kenntnis und nicht die
     Entscheidbarkeit.
   - Contra: Genau diese Lage hat die heutige Mischung erzeugt. Der Datensatz vom 260826
     nennt sie beim Namen: „die nächste Runde entscheidet die Frage für ihre eigenen
     Zeichenketten wieder von vorn — so ist die heutige Lage entstanden."
   - Was sie ausschließt: nichts.

2. **Eine schwächere, aber entscheidbare Probe: kein Stringliteral trägt beide
   Schreibweisen zugleich.** Ein Literal, das einen Umlaut oder ein ß **und** ein Wort mit
   `ae`/`oe`/`ue`/`ss` aus der Stammliste trägt, hält den Prüflauf an.
   - Pro: Vollständig aus dem Quelltext entscheidbar, ohne eine einzige Aufzählung von
     Stellen. Fängt genau das Bild, mit dem der Vorgängerdatensatz aufmacht — zwei
     Schreibweisen in einem Satz. Kostet eine Probe und eine Stammliste.
   - Contra: Fängt den häufigeren Fall **nicht**: `"ist beschaedigt"` steht allein in seinem
     Literal und wäre grün, während daneben `"Einträgen"` in einem anderen Literal steht. Die
     Probe verspräche eine Deckung, die sie nicht hat, und eine grüne Probe über einer
     ungedeckten Regel ist teurer als gar keine.
   - Was sie ausschließt: nichts, aber sie besetzt den Platz, an dem sonst eine wirksame
     Prüfung stünde.

3. **Die Sichtbarkeit zu einer Eigenschaft machen, die die Probe lesen kann: ein Typ
   `Nutzertext`, den jede Senke der Oberfläche verlangt** (`meldung_zeigen`,
   `befehlsantwort_zeigen`, `antwort_zeigen`, `Steuerung::ueberspringen`,
   `Blatt::erlaeuterung_setzen`, `Ersetzung::fmt`). Sein Erzeuger prüft unter
   `debug_assertions` gegen die Stammliste und hält den Prüflauf an.
   - Pro: Beantwortet die Frage, die tatsächlich zu beantworten ist, statt sie zu nähern:
     nutzersichtbar ist, was durch eine Senke geht, und das steht dann am Typ. Deckt auch
     Text, der zur Laufzeit aus `format!` entsteht — den Fall, an dem jede Prüfung über den
     Quelltext scheitert.
   - Contra: Ein Umbau quer durch beide Kisten, weit über den Umfang der Schreibweisenfrage
     hinaus. Die Prüfung fiele in die Laufzeit statt in den Bau; ein Abbruch in einem
     Meldeweg der Oberfläche ist ein schlechter Tausch, und `debug_assertions` heißt, dass
     das Auslieferungsbündel gar nicht prüft. Die Stammliste braucht er trotzdem.
   - Was sie ausschließt: nichts, aber sie bindet die Frage an einen Umbau, der eine eigene
     Runde wäre.

## Constraints

- Keine Aufzählung von Stellen, Dateien oder Ausnahmen. Eine Liste, die jemand pflegen muss,
  ist die Bauart, die dieses Projekt an mehreren Stellen abgeschafft hat.
- Eine Stammliste deutscher Wörter ist **keine** Ausnahmeliste in diesem Sinn: sie zählt keine
  Stellen des Baums auf, sondern nennt eine Tatsache über die deutsche Rechtschreibung, und
  ein Wort, das darin fehlt, macht die Prüfung schwächer und nie falsch.
- Was auch gewählt wird, darf keine grüne Probe über einer ungedeckten Regel hinterlassen.

## Recommendation

Keine. Möglichkeit 2 ist die einzige, die heute ohne Umbau zu haben ist, und sie ist zugleich
die, die eine Deckung behauptet, die sie nicht hat — der Grund, aus dem sie hier als eigene
Frage steht statt als eingebaute halbe Lösung. Möglichkeit 3 beantwortet die Frage wirklich
und ist eine eigene Runde. Möglichkeit 1 ist ehrlich und lässt die Regel unbewacht.

Die Entscheidung gehört dem Nutzer, weil sie zwischen „unbewacht und ehrlich" und „bewacht und
teuer" wählt und keine der beiden Seiten sich aus der Sache selbst ergibt.
