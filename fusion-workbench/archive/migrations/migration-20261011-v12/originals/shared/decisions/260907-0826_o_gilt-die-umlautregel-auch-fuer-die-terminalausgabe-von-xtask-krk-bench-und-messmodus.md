# Gilt die Umlautregel auch für die Terminalausgabe von xtask, krk-bench und --messmodus?

---
**Domain:** code
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260826-1225_*_welche-schreibweise-gilt-fuer-nutzersichtbare-deutsche-meldungen-umlaut-oder-umschrift.md` (die Antwort, die diese Frage aufwirft), `260907-0826_*_wie-wird-die-naht-zwischen-umlaut-und-umschrift-gehalten-jetzt-da-sie-eine-regel-ist.md` (dieselbe Naht, andere Frage)

---

## Question

Der Nutzer hat am 260907 entschieden: Umlaute in allem, was ein Mensch liest; die Umschrift
bleibt für Kommentare und Bezeichner. Der Durchgang, der die Antwort umgesetzt hat, hat die
Oberfläche von KRK erfasst — Statuszeile, Blätter, Menü, Spaltenüberschriften, Vorschau — und
dabei drei Flächen stehen gelassen, die keine Oberfläche sind und trotzdem ein Mensch liest:

1. **`xtask`** — die Bau- und Auslieferungskette. Ihre Meldungen gehen an das Terminal des
   Entwicklers, zwischen die Ausgabe von `cargo`, `codesign` und `xcrun`.
2. **`krk-bench`** — die kopflose Messstrecke. Ihre Berichte liest, wer den Lauf gefahren hat.
3. **`crates/krk-ui/src/messmodus.rs`** und die `eprintln!("krk: …")`-Zeilen in `krk-ui` — der
   Messmodus und die Fadenstart-Hinweise. Sie liegen **in** `krk-ui` und gehen trotzdem auf
   die Standardfehlerausgabe und nicht in die Statuszeile; ein über den Finder gestartetes
   Bündel hat gar keine.

Erhoben am Stand `0115cf5` mit einem Scanner über jedes Stringliteral der vier Kisten
außerhalb der `#[cfg(test)]`-Module und außerhalb von `tests/`, gefiltert erst auf
`ae|oe|ue|ss` im Wortinneren und dann gegen eine von Hand durchgesehene Stammliste, die
Falschtreffer wie `neue`, `Quelle`, `Fassung`, `muss` und `dass` ausschließt. Nach Abzug der
Übersetzer- und Prüfdiagnostik (`#[must_use]`, `assert`, `debug_assert`, `panic`, `expect`)
bleiben 338 Prosastellen mit Umschrift, verteilt wie folgt:

| Kiste | Stellen |
|---|---|
| `xtask` | 123 |
| `crates/krk-bench` | 107 |
| `crates/krk-ui` | 61, davon 19 in `messmodus.rs` |
| `crates/krk-core` | 47 |

Von den 108 Stellen aus `krk-core` und `krk-ui` gehen 61 durch KRKs Oberfläche; die hat der
Durchgang vom 260907 auf Umlaute gebracht. Die verbleibenden 47 sind Übersetzerdiagnostik, die
das erste Sieb nicht erwischt hat, Bezeichner in Formatplatzhaltern (`{groesse}`,
`{HOECHSTENS_EINTRAEGE}`), die maschinenlesbare Ausgabe von `--menue-protokoll` und eben die
19 aus `messmodus.rs` samt den `eprintln!("krk: …")`-Zeilen daneben. Dazu kommen die 230
Stellen aus `xtask` und `krk-bench`, die dieser Datensatz zur Entscheidung stellt.

**Der Datensatz, den diese Frage aufwirft, hat sie nicht gemessen.** Seine Constraint-Zeile
lautet „Die Antwort gilt für `krk-core` und `krk-ui` gemeinsam", und die Begründung dafür ist
die geteilte Statuszeile: „Eine Regel je Kiste wäre keine Antwort: die Meldungen beider landen
in derselben Statuszeile." Diese Begründung trägt für `xtask` und `krk-bench` nicht — sie
haben keine Statuszeile — und für den Messmodus auch nicht, denn der schreibt in ein Terminal.

Die Antwort ist damit für die Oberfläche eindeutig und für das Terminal offen. Wer die
Antwort wörtlich nimmt („alles, was ein Mensch liest"), muss die 246 Stellen nachziehen; wer
sie im gemessenen Umfang liest, lässt sie stehen. Die Frage muss entschieden werden, weil
sonst die nächste Runde die Sache für ihre eigenen Zeichenketten wieder von vorn entscheidet —
so ist die Mischung entstanden, die der Vorgängerdatensatz beschreibt.

## Options

1. **Die Regel endet an KRKs Oberfläche; das Terminal behält die Umschrift.**
   - Pro: Die Naht folgt der Begründung des Vorgängerdatensatzes und nicht seinem Wortlaut,
     und sie ist an jeder Stelle entscheidbar: geht der Satz durch ein Fenster von KRK oder
     durch ein Terminal. Die Terminalausgabe steht neben der von `cargo`, `git` und `xcrun`
     und ist ohnehin gemischt; ein Bündel aus dem Finder hat keine Standardfehlerausgabe, die
     Sätze erreichen dort niemanden. Kostet null Stellen.
   - Contra: `messmodus.rs` liegt in `krk-ui`, die Kiste, für die die Antwort gilt — die Naht
     läuft dann mitten durch eine Kiste. Und ein Mensch liest diese Sätze wirklich.
   - Was sie ausschließt: nichts. Eine spätere Ausweitung kostet denselben Durchgang.

2. **Die Regel gilt für jede deutsche Prosa, die ein Mensch zur Laufzeit liest, Terminal
   eingeschlossen.**
   - Pro: Eine Naht statt zweier, und sie ist der Wortlaut der Antwort. Kein Fall, in dem
     jemand entscheiden muss, ob ein Terminal ein Fenster ist.
   - Contra: 230 Stellen in `xtask` und `krk-bench` sind nachzuziehen, dazu die 19 aus
     `messmodus.rs`, und die Proben, die auf den Wortlaut prüfen, ziehen mit. `xtask` und
     `krk-bench` sind Werkzeug und keine Anwendung; ihre Sätze stehen zwischen englischen
     Ausgaben fremder Programme.
   - Was sie ausschließt: nichts.

3. **Die Regel gilt zusätzlich für die Diagnostik des Übersetzers und der Proben**
   (`#[must_use]`, `assert`, `panic`, `debug_assert`).
   - Pro: Die vollständigste Lesart von „was ein Mensch liest".
   - Contra: Diese Texte sind Kommentar in Attributform — sie erklären dem Entwickler, warum
     eine Zeile dasteht, und der Vorgängerdatensatz nimmt Kommentare ausdrücklich von der
     Frage aus („Doc-Kommentare und Bezeichner sind von der Frage nicht berührt"). Dieselbe
     Erhebung zählt dafür 132 Stellen allein außerhalb der Proben, und die Proben selbst
     kommen dazu.
   - Was sie ausschließt: nichts.

## Constraints

- Was immer entschieden wird, muss an jeder einzelnen Zeichenkette entscheidbar sein, ohne
  eine Liste von Ausnahmen. Das war die tragende Eigenschaft der Antwort vom 260907.
- Die Oberfläche von KRK trägt seit dem Durchgang vom 260907 durchgehend Umlaute; keine
  Möglichkeit hier nimmt daran etwas zurück.

## Recommendation

Möglichkeit 1. Die Naht „geht der Satz durch ein Fenster von KRK" ist so entscheidbar wie die
Naht „liest das ein Mensch oder der Übersetzer" und trifft zusätzlich den Grund, aus dem die
Frage überhaupt gestellt wurde: der Nutzer sah in **einer** Statuszeile zwei Schreibweisen
nebeneinander. Im Terminal steht die deutsche Prosa ohnehin neben englischer Fremdausgabe, und
dort ist die gemischte Schreibweise kein Widerspruch, den man dem Nutzer zumutet, sondern eine
Eigenschaft der Werkzeugkette.
