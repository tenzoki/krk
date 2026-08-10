Derselbe selbstabraeumende Pruefordner steht zwoelfmal im Baum, unter zwei Namen

---

Ein Ordner unter dem Temporaerverzeichnis mit Prozesskennung und Laufnummer im
Namen, der sich in `Drop` abraeumt, ist zwoelfmal getrennt geschrieben: zehnmal
als `Pruefordner`, zweimal als `Wegwerfordner`. Vier der zwoelf liegen in einer
und derselben Kiste, wo eine Fassung genuegte.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung des Defekts `260810-1256`
**Betroffen:** `crates/krk-ui/src/{vorschaumodell,editormodell,leistenmodell}.rs`,
`crates/krk-ui/src/kommandos/pfadeingabe.rs`, `crates/krk-core/tests/*.rs`,
`crates/krk-bench/src/{fixture,messen}.rs`
**Domain:** code
**Zusammenhang:** `issues/260810-1256_*_die-proben-des-vorschaumodells-legen-ihre-ordner-unter-festen-namen-an.md`

## Die zwoelf Stellen

```text
crates/krk-core/tests/operation.rs:52      struct Pruefordner
crates/krk-core/tests/text.rs:248          struct Pruefordner
crates/krk-core/tests/verzeichnis.rs:26    struct Pruefordner
crates/krk-core/tests/belegung.rs:28       struct Pruefordner
crates/krk-core/tests/ablage.rs:49         struct Pruefordner
crates/krk-core/tests/navigation.rs:29     struct Pruefordner
crates/krk-ui/src/vorschaumodell.rs:913    struct Pruefordner
crates/krk-ui/src/editormodell.rs:1246     struct Pruefordner
crates/krk-ui/src/kommandos/pfadeingabe.rs:122  struct Pruefordner
crates/krk-ui/src/leistenmodell.rs:608     struct Pruefordner
crates/krk-bench/src/fixture.rs:592        struct Wegwerfordner
crates/krk-bench/src/messen.rs:2016        struct Wegwerfordner
```

Der Kern jeder Fassung ist dieselbe halbe Seite: `neu(zweck)` haengt
`std::process::id()` und einen `AtomicU64`-Zaehler an den Namen, `create_dir_all`,
`Drop` ruft `remove_dir_all`. Sie unterscheiden sich in den Helfern, die je Datei
dazugehoeren (`datei`, `ordner`, `verknuepfung`, `verstecken`, `roehre`), und im
Namenspraefix (`krk-test-`, `krk-vorschau-probe-`, und andere).

## Die zwei Faelle sind verschieden schwer zu heilen

**Die sechs in `krk-core/tests/`** sind sechs eigene Testziele, also sechs eigene
Kisten. Sie koennen einander nichts einbinden. Der uebliche Weg dafuer ist ein
`tests/gemeinsam/mod.rs`, das jedes Ziel per `mod gemeinsam;` einzieht; das ist
kein grosser Umbau, beruehrt aber sechs Dateien. Ein Modulkopf in
`tests/verzeichnis.rs:3-5` sagt heute ausdruecklich, ein Erzeuger sei "bewusst
noch nicht" da und komme mit Schritt 3 — dieser Satz ist mit dem Abschluss der
Runde 2 ueberholt und gehoert mit derselben Aenderung nachgezogen.

**Die vier in `krk-ui/src/`** liegen in **einer** Kiste, im selben Binaerziel, und
sind der eigentliche Befund: hier genuegt ein `#[cfg(test)] mod pruefordner;`
neben den Modellen, und die vier Fassungen fallen auf eine zusammen. Die Helfer,
die nur eine Probe braucht (`roehre` in `vorschaumodell.rs`), koennen dort bleiben
oder mitwandern.

## Fehlszenario

Kein Fehlverhalten zur Laufzeit; das ist der Grund fuer die niedrige Schwere. Was
es kostet, ist die naechste Aenderung an der Bauform: der Defekt `260810-1256`
hat gezeigt, dass eine Probe ohne Waechter jahrelang neben zehn Fassungen mit
Waechter stehen kann, ohne dass es auffaellt. Wer morgen eine dreizehnte Probe
schreibt, findet zwoelf Vorbilder und keine Quelle, und wer eine Eigenschaft
aendert (etwa: den Ordner unter `~/Library/Caches` statt unter `$TMPDIR` anlegen,
wie CLAUDE.md es fuer den Messplatz verlangt), aendert sie zwoelfmal oder
uebersieht eine.

## Vorgeschlagene Behebung

Zwei getrennte Schritte, in dieser Reihenfolge, weil der erste klein ist und der
zweite mehr Dateien beruehrt:

1. Die vier Fassungen in `krk-ui/src/` auf ein `#[cfg(test)]`-Modul der Kiste
   zusammenlegen.
2. Die sechs in `krk-core/tests/` auf ein `tests/gemeinsam/mod.rs` zusammenlegen
   und den ueberholten Satz in `tests/verzeichnis.rs:3-5` nachziehen.

Die zwei `Wegwerfordner` in `krk-bench` sind derselbe Fall wie Punkt 1 und
koennen mit ihm gehen; sie liegen ebenfalls in einer Kiste.

**Nicht mit `260810-1256` behoben**, weil dessen Aufgabe ausdruecklich verlangte,
die vorhandene Form zu nutzen und keinen zweiten Mechanismus zu bauen. Eine
Zusammenlegung ist die Gegenrichtung und braucht ihre eigene Dateigrenze.

## Zustaendigkeit

`coder`.
