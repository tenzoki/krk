# Das Verweisziel wird am Pfad bestimmt und nicht mehr am Deskriptor

**Status:** Complete
**Agent:** coder
**Datum:** 260815-1758
**Quelldatensatz:** `shared/issues/260815-1713_c_verweisziel-beantwortet-die-ordnerfrage-mit-open-und-nicht-mit-stat.md`

## Auftrag

`verweisziel::bestimmen` beantwortete „ist das Ziel ein Verzeichnis?" mit `open(2)`. Der
Aufruf kann diese Frage nicht entscheiden. Der Auftrag war der Wechsel auf
`std::fs::metadata` samt Nachziehen des Modulkopfs, der Wertbeschreibungen, der
Zusicherung zur Ueberschneidungsfreiheit und der Roehrenprobe, dazu zwei neue Proben und
ein Datensatz zu einer beruehrten Nutzerfrage. Ausdruecklich **nicht** im Auftrag: eine
Leserechtspruefung in `Verweisziel::Ordner`, jede Aenderung an `sys::ohne_warten_oeffnen`
und an der Verzweigung in `tabelle.rs`, und ein Commit.

## Was geaendert ist

### `crates/krk-core/src/verzeichnis/verweisziel.rs`

- `bestimmen` fragt `std::fs::metadata(pfad)`. Ein Systemaufruf statt fuenf (`open`,
  `fcntl`×2, `fstat`, `close`), und geoeffnet wird nichts.
- Die Hilfsfunktion `unerreichbar` ist entfallen. Ihre Begruendung war, dass **zwei**
  Fehlschlaege denselben Wert bauen; nach dem Wechsel gibt es nur noch einen, und eine
  Funktion mit einem Rufer, deren angegebener Zweck weggefallen ist, ist keine
  Vereinfachung mehr. `use std::io` und `use super::sys` sind damit ebenfalls weg.
- `#[must_use]` bleibt, mit neuer Begruendung: der Aufruf hat gar keine Wirkung mehr, also
  ist ein fallengelassener Wert der ganze Aufruf umsonst.
- Der Modulkopf traegt die Unterscheidung, an der der Fehlschluss haengt (siehe unten).
- `Verweisziel::Unerreichbar` heisst jetzt „der Name loest sich nicht auf", mit dem
  ausdruecklichen Zusatz, dass „ohne Recht" das Recht **am Pfad** meint und nicht am Ziel.
  `Ordner` und `KeinOrdner` sagen beide, dass sie ueber das Leserecht des Ziels nichts
  aussagen.
- Die Zusicherung „drei Werte, ueberschneidungsfrei und vollstaendig" gilt jetzt fuer die
  Ausgaenge des Verfahrens **und** fuer die Zustaende, die die Werte benennen. Geprueft:
  `stat(2)` loest den Namen auf oder nicht; loest es ihn auf, ist das Ding ein Verzeichnis
  oder keines. Kein Zustand faellt in zwei Werte, keiner faellt durch. Der Doc-Kommentar
  schreibt daneben auf, dass der Satz vorher nur die halbe Geltung hatte.

### `crates/krk-core/tests/verzeichnis.rs`

- Zwei neue Proben: `eine_verknuepfung_auf_eine_datei_ohne_leserecht_ist_kein_ordner` und
  `eine_verknuepfung_auf_ein_verzeichnis_ohne_leserecht_ist_ein_ordner` (Modus `0111`).
- `eine_roehre_haelt_die_frage_nach_dem_verweisziel_nicht_an`: nur der Doc-Kommentar. Er
  stellte `File::open` als Gegenstueck gegenueber; das war nie die Alternative. Jetzt steht
  dort das blockierende `open(2)`, und dass `stat(2)` die Roehre gar nicht anfasst. Die
  Probe selbst misst eine echte Zusage und bleibt unveraendert.
- Die Abschnittsueberschrift nennt neben dem Defekt `260814-1612` auch den Befund
  `260815-1713`.

## Der Kern der Sache, wie er jetzt im Modulkopf steht

Die Fehlentscheidung kam aus einer Verallgemeinerung: `sys::ohne_warten_oeffnen` ist die im
Baum eingefuehrte Form, also schien sie hier richtig. Die Regel, die stattdessen gilt, steht
jetzt als ein Satz im Modulkopf:

> **Wer den Deskriptor danach benutzt, oeffnet. Wer nur fragt, was hinter dem Namen steht,
> fragt am Namen.**

Ausgeschrieben: der Editor und der Leseweg der Vorschau lesen aus genau dem Deskriptor, den
sie geprueft haben. Nur deshalb kaufen sie mit ihm zwei Dinge, naemlich kein Fenster
zwischen Pruefung und Benutzung (weil beides dasselbe **Objekt** trifft und nicht denselben
**Namen**) und kein Haengen an einer benannten Roehre. `bestimmen` benutzt seinen Deskriptor
nicht, gibt ihn sofort ab, und der Aufrufer oeffnet den Pfad ein zweites Mal. Also besteht
das Fenster fort, und `stat(2)` wartet an einer Roehre ohnehin nie. Beide Gewinne sind hier
keine; der Preis waren drei Fehlfaelle und eine Geraetewirkung beim blossen Fragen.

Der Modulkopf sagt ausdruecklich, dass der Wechsel **kein Rueckfall hinter die Bauform des
Editors** ist, sondern dieselbe Regel richtig herum angewandt.

## Am Referenzgeraet gemessen, vorher und nachher

Nachgefahren am 260815, uid 502, ueber `bestimmen` selbst in einer Wegwerfprobe unter
`crates/krk-core/tests/` (nach der Messung geloescht) — nicht aus dem Datensatz uebernommen:

| Ziel der Verknuepfung | vorher | nachher |
|---|---|---|
| Unix-Socket | `Unerreichbar {"Operation not supported on socket (os error 102)"}` | `KeinOrdner` |
| gewoehnliche Datei, Modus `000` | `Unerreichbar {"Permission denied (os error 13)"}` | `KeinOrdner` |
| Verzeichnis, Modus `0111` | `Unerreichbar {"Permission denied (os error 13)"}` | `Ordner` |
| benannte Roehre ohne Schreiber | `KeinOrdner` | `KeinOrdner` |

Die Messtabelle des Datensatzes ist damit in jeder Zeile bestaetigt, `EOPNOTSUPP` (102)
eingeschlossen.

## Datensaetze

- **Angelegt:**
  `shared/issues/260815-1749_o_der-pfadsprung-meldet-den-ordner-ohne-leserecht-und-der-doppelklick-schweigt.md`
  — die Nutzerfrage zur Leserechts-Ungleichheit, mit beiden Fundstellen und ohne Empfehlung.
- **Angelegt:**
  `shared/issues/260815-1752_o_zwei-modulkoepfe-nennen-das-verweisziel-am-deskriptor-obwohl-es-am-pfad-fragt.md`
  — vier Beschreibungsstellen in `verzeichnis/mod.rs` und `appkit/tabelle.rs`, die der
  Wechsel falsch gemacht hat und die ausserhalb der Auftragsgrenzen liegen.
- **Geschlossen:** `260815-1713` (der Quelldatensatz), mit Messtabelle in der Abschlussnotiz.
- **Geschlossen:** `260815-1714` (`sys.rs` und `CLAUDE.md` nennen zwei Aufrufer der Huelle,
  es sind drei). Der Datensatz hatte den Weg selbst vorgezeichnet: mit dem Wechsel faellt der
  dritte Rufer weg und alle sechs Stellen stimmen von selbst. Nachgezaehlt:
  `grep -rn 'ohne_warten_oeffnen' crates/` findet zwei Aufrufstellen.

## Pruefung

```
cargo fmt --all --check && cargo clippy --workspace --all-targets && cargo test --workspace
```

Exit 0. Alle Proben gruen, keine neue Warnung. Die beiden neuen Proben laufen mit.

## Offen

- Die vier Beschreibungsstellen aus `260815-1752`. Sie sind der einzige Punkt, an dem der
  Baum nach dieser Aenderung etwas Falsches ueber sich selbst sagt.
- Kein Commit, wie beauftragt.
