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

---
Resolved: Zwoelf Fassungen sind zu **drei** geworden, eine je Kiste, und drei
bleiben es aus Gruenden der Kistenstruktur: die zwoelf Stellen liegen in drei
Kisten, die einander nichts einbinden koennen. Neu sind
`crates/krk-core/tests/gemeinsam/mod.rs` (traegt `Pruefordner` fuer die sechs
Testziele des Kerns, jedes zieht ihn per `mod gemeinsam;` ein),
`crates/krk-ui/src/pruefordner.rs` (`#[cfg(test)] mod pruefordner;` in `main.rs`,
fuer `vorschaumodell`, `editormodell`, `leistenmodell` und
`kommandos::pfadeingabe`) und `crates/krk-bench/src/wegwerfordner.rs`
(`#[cfg(test)] mod wegwerfordner;` in `main.rs`, fuer `fixture` und `messen`).

**Warum drei und nicht eine.** Ein Testziel ist in Rust eine eigene Kiste, und
`krk-ui` wie `krk-bench` haben je nur ein Binaerziel. Die sechs Fassungen des
Kerns stehen in Testzielen, die vier von `krk-ui` und die zwei von `krk-bench` in
Binaerzielen; keines dieser drei Ziele erreicht den Code eines anderen. Eine
einzige Fassung verlangte einen Ort, den alle drei einbinden koennen, also ein
Bibliotheksziel fuer `krk-ui` oder eine eigene Kiste im Workspace. Das erste ist
Gegenstand der offenen Nutzerentscheidung
`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`
und ausdruecklich kein Nebenzug; das zweite lag ausserhalb der Dateigrenze
dieser Aufgabe. Die drei Modulkoepfe sagen das jeweils selbst, damit die naechste
Lesung nicht wieder mit "eine genuegte" anfaengt.

**Was die Zusammenlegung nebenbei geheilt hat.** Die vier Fassungen in `krk-ui`
fuehrten vier eigene Zaehler, obwohl alle Einheitsproben der Kiste in **ein**
Probenprogramm uebersetzen: zwei Proben mit demselben Zweck in verschiedenen
Modulen konnten denselben Ordnernamen bekommen. Jetzt zaehlt ein Zaehler fuer die
Kiste. Und das Abraeumen des Kerns raeumt seither in allen sechs Zielen gegen
entzogene Rechte auf, nicht nur in `operation.rs`; der schnelle
`remove_dir_all` bleibt der erste Weg, das Zurueckdrehen der Rechte kommt nur,
wenn er scheitert.

**Vereinheitlichte Unterschiede.** `datei` nimmt jetzt `impl AsRef<[u8]>` statt
einmal `&str` und einmal `&[u8]`; die Fassung mit einer Byte-Anzahl heisst
`fuelldatei`, `unterordner` heisst `ordner`, `verknuepfung` nimmt
`impl AsRef<Path>`. `ablage_mit` ist eine freie Funktion in `tests/belegung.rs`
geblieben: der gemeinsame Pruefordner haelt Ordner und Dateien, und eine
`Ablage` ist ein Gegenstand des Kerns. In `krk-ui` legt `neu` den Ordner an und
`nur_name` nur den Namen, weil die Gueltigkeitsproben des `leistenmodell`
denselben Pfad einmal vorhanden und einmal fehlend brauchen.

**Der ueberholte Satz ist nachgezogen.** Der Modulkopf von
`tests/verzeichnis.rs` sagte, ein Pruefordner-Erzeuger sei "bewusst noch nicht"
da und komme mit Schritt 3 der Runde 1; er sagt jetzt, woher der Pruefordner
kommt. Dieselbe Berichtigung in `tests/text.rs`, dessen Kopf auf die Fassung in
`verzeichnis.rs` verwies.

**Abnahme:** `cargo build --workspace` exit 0, `cargo test --workspace` exit 0
(alle 15 Testprogramme gruen, 0 Fehlschlaege), `cargo clippy --workspace
--all-targets` exit 0 ohne eine Warnung, `cargo fmt --all --check` exit 0. Nach
dem vollen Testlauf liegt kein Pruefordner mehr unter `$TMPDIR` oder `/tmp`; die
neun `krk-messplan-*.toml` dort sind aelter als dieser Lauf und haben ihre eigene
Ursache, siehe unten.

**Zwei Defekte daneben gefunden**, beide neu abgelegt:
`issues/260810-1430_*_planordner-in-messmodus-ist-die-dreizehnte-fassung-und-kann-jetzt-auf-die-gemeinsame-aufsetzen.md`
(eine dreizehnte Fassung, die dieser Datensatz nicht gezaehlt hat, und
`messmodus.rs` lag ausserhalb der Dateigrenze) und
`shared/issues/260810-1430_*_ein-abgebrochener-messlauf-laesst-seinen-messplan-im-temporaerverzeichnis-liegen.md`
(die Abraeumzeile des Messplans steht auf dem Erfolgsweg statt in einem `Drop`).
