Zwei fremde Werkzeuge werden seit langem über den Suchpfad gerufen, und drei Stellen nennen `gh` als die erste Ausnahme

---

Der Entscheidungsdatensatz zum Suchpfad, die Risikotabelle des Plans „Artefakt und Release" und
die Frage selbst gehen davon aus, dass `xtask` bis zum 260821 **jedes** fremde Werkzeug mit
vollem Pfad gerufen hat und `gh` die erste Ausnahme ist. Der Baum trägt zwei ältere Ausnahmen,
`iconutil` und `rustup`, und keine von beiden ist an ihrem Aufrufort begründet.

---

**Gemessen am Baumstand `4e810f9`**, über `grep -rn 'Command::new' xtask/src`. Die vollständige
Aufstellung der zwanzig Prozessaufrufe zerfällt in drei Gruppen:

| Gruppe | Stellen |
|---|---|
| Voller Pfad | `/usr/bin/codesign` (2), `/usr/bin/ditto` (2), `/usr/bin/xcrun` (3), `/usr/bin/security`, `/usr/bin/git`, `/usr/bin/lipo` (2) |
| Über eine aufgelöste Variable | `Command::new(&cargo)` in `version.rs:302`, `bundle.rs:499`, `messen.rs:71` |
| **Über den Suchpfad** | `bundle.rs:427` `iconutil`, `release.rs:604` `rustup`, `veroeffentlichung.rs:177`/`:180`/`:612`/`:655` `gh` |

**Die zwei älteren, mit ihrem Alter:**

- `xtask/src/bundle.rs:427` — `Command::new("iconutil")`, seit `8695b77` vom 260811. Baut die
  `.icns` aus den sieben PNGs unter `iconset/`. Kein Kommentar an der Stelle begründet den
  fehlenden Pfad; `iconutil` liegt auf diesem Gerät unter `/usr/bin/iconutil` und wäre mit
  vollem Pfad zu rufen wie `/usr/bin/ditto` daneben.
- `xtask/src/release.rs:604` — `Command::new("rustup")`, seit `d577295` vom 260806. Prüft, dass
  beide Ziel-Tripel installiert sind. Hier ist der Suchpfad sachlich richtig — `rustup` gehört
  nicht zu macOS und liegt unter `$HOME/.cargo/bin`, genau der Ordner, den `CLAUDE.md` als nicht
  auf dem Standard-`PATH` liegend führt —, aber die Stelle sagt es nicht.

## Die drei Stellen, die etwas anderes behaupten

1. `shared/decisions/260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`,
   Abschnitt „Frage": „Das Bauwerkzeug ruft heute jedes fremde Werkzeug mit vollem Pfad" — und
   zählt fünf auf, ohne `iconutil` und `rustup`.
2. `shared/planning/260821-1221_*_plan-artefakt-und-release.md`, Risikotabelle: „`gh` wird über
   den Suchpfad gerufen und nicht mit vollem Pfad, **anders als jedes andere fremde Werkzeug
   dieses Baums**."
3. `xtask/src/veroeffentlichung.rs:37-45`, Modulkopf. **Diese Stelle ist als einzige nicht
   falsch:** sie sagt, `gh` weiche „von der Gewohnheit dieses Baums ab, der `/usr/bin/git`,
   `/usr/bin/codesign`, `/usr/bin/ditto` und `/usr/bin/xcrun` mit vollem Pfad ruft" — eine
   Aussage über vier benannte Werkzeuge und keine über alle. Sie ist trotzdem hier aufgeführt,
   weil ein Leser die Aufzählung als vollständig nimmt.

## Warum es der Rede wert ist

**Es verschiebt die offene Entscheidung.** Das Contra von Option 1 in jenem Datensatz lautet:
„Die Gewohnheit des Baums bekommt eine Ausnahme, und eine Ausnahme, die nur an einer Stelle
begründet steht, wird beim zweiten Mal übersehen." Genau das ist am Baum bereits eingetreten,
zweimal und vor der Frage. Wer die Frage beantwortet, beantwortet sie damit für drei Werkzeuge
und nicht für eins, und Option 2 (Stufensuche) müsste sich an `rustup` messen lassen, wo sie
sicher falsch wäre.

**Und es ist dieselbe Fehlerart, die dieses Projekt schon dreimal an Aufzählungen gehabt hat.**
Eine Prosastelle behauptet, eine Aufzählung sei vollständig; sie war es bei ihrer Abfassung
nicht. Vergleiche `shared/issues/260812-2253_*` (Zahl der `Kommando`-Varianten) und
`shared/issues/260812-1438_*` (Quote der Untergrenzen-Abschnitte).

## Abhilfe

Zwei Handgriffe, unabhängig voneinander:

1. **Die zwei Prosastellen berichtigen.** Der Entscheidungsdatensatz hat den Nachtrag im Abgleich
   vom 260821-1532 bekommen; die Risikotabelle des Plans steht noch. Kein Marker ist dabei zu
   bewegen — der Plan ist geschlossen, seine Risikotabelle ist eine Aufzeichnung eines Standes.
   Der Satz „anders als jedes andere fremde Werkzeug dieses Baums" ist trotzdem zu berichtigen,
   weil er eine Zusage über den ganzen Baum macht und nicht über den Stand jenes Tages.
2. **Die zwei Aufrufstellen begründen.** Je ein Satz an `bundle.rs:427` und `release.rs:604`,
   der sagt, warum dort kein voller Pfad steht. Für `rustup` ist die Begründung dieselbe wie für
   `gh`; für `iconutil` gibt es sie womöglich nicht, und dann gehört dort `/usr/bin/iconutil`
   hin. Welcher der beiden Wege für `iconutil` richtig ist, entscheidet die offene Frage.

**Schwere:** niedrig für das Verhalten — kein Lauf bricht daran ab. Mittel für die Grundlage:
eine offene Entscheidung steht auf einer Voraussetzung, die nicht zutrifft.

**Gefunden:** reconciler, Abgleich zum Sitzungsabschluss 260821-1532, Bereich `01d2365..4e810f9`

**Betroffen:** `xtask/src/bundle.rs:427`, `xtask/src/release.rs:604`,
`shared/decisions/260821-1221_*_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`,
`shared/planning/260821-1221_*_plan-artefakt-und-release.md` (Risikotabelle)

**Domain:** code

**Herkunft:** gemeinsamer Speicher. Kein Circle war in dieser Sitzung aktiv, und der Befund
betrifft den Bauweg des ganzen Projekts.
