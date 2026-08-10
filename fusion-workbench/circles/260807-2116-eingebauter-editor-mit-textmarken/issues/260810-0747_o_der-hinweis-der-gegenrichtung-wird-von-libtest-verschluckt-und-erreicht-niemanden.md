# Der Hinweis der Gegenrichtung wird von libtest verschluckt und erreicht niemanden

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:2810-2818` (`keine_unbekannte_einstellung_steht_an_der_textflaeche`, der `eprintln!`-Zweig), Modulkopf `:161-168`
**Cross-references:** `issues/260810-0417_c_die-laufzeitprobe-bindet-den-bau-an-die-macos-version-des-pruefenden-geraets.md`, `Makefile:40` (`make test`, von `make check` gerufen)

---

## Der Befund

Der Commit trennt die beiden Richtungen der Probe: `getragen \ eingeordnet`
hält den Bau an, `eingeordnet \ getragen` wird ein `eprintln!`. Der
`Resolved:`-Abschnitt von `260810-0417` sagt dazu zu: „ein Eintrag ohne
Entsprechung an der Klasse läuft grün durch und **schreibt den Hinweis**."

Er schreibt ihn, und niemand liest ihn. `libtest` fängt Standardausgabe und
Standardfehlerausgabe eines Tests ab und gibt sie nur aus, wenn der Test
**fehlschlägt** oder wenn `--nocapture` beziehungsweise `--show-output` gesetzt
ist. Der Zweig läuft genau dann, wenn der Test **nicht** fehlschlägt.

Gemessen, nicht der Dokumentation entnommen. Ein minimaler Test, mit derselben
Werkzeugkette übersetzt:

```rust
#[test]
fn passes_and_prints() {
    eprintln!("HINWEIS-AUF-STDERR");
    println!("HINWEIS-AUF-STDOUT");
}
```

```
$ ./capt
running 1 test
test passes_and_prints ... ok
test result: ok. 1 passed; 0 failed; ...

$ ./capt --nocapture
running 1 test
HINWEIS-AUF-STDERR
HINWEIS-AUF-STDOUT
test passes_and_prints ... ok
```

Kein Kommando des Projekts setzt das Flag: weder `cargo test --workspace` aus
`CLAUDE.md` noch `make check` noch `make test`. Der Hinweis geht auf allen
heute begangenen Wegen ins Leere.

## Warum das die Aussage des Commits berührt

Die Asymmetrie der Probe ist **richtig begründet** — ein Schalter, den es nicht
mehr gibt, ändert keine Zeichen, und eine grüne Reihe auf einem unterstützten
System rot zu färben wäre falsch. Der Befund richtet sich nicht gegen die
Entscheidung, sondern gegen die Zusage, die daneben steht: die harmlose
Richtung ist nicht „ein Hinweis", sondern heute schlicht **stumm**. Der
Modulkopf beschreibt sie als „ein Hinweis auf der Standardfehlerausgabe" und
sagt damit mehr zu, als der Bau hält.

Das ist genau die Sorte Aussage, die `260810-0417` an der Vorform bemängelt
hat: die Probe soll sagen, was sie hält. Sie sagt es an dieser Stelle wieder zu
weit.

## Was heute hält

Kein Defekt am Ausgeführten. Die gefährliche Richtung hält den Bau an und nennt
die Namen — gegengeprüft im `Resolved:` von `260810-0417`. Der Befund betrifft
allein die zweite Richtung und ihre Beschreibung.

## Vorschlag

Drei Wege, in der Reihenfolge, in der ich sie für tragfähig halte:

1. **Den Satz zurücknehmen.** Modulkopf und Testkommentar sagen, dass der
   Hinweis nur unter `cargo test -- --nocapture` sichtbar wird. Das kostet zwei
   Zeilen und ist ehrlich.
2. **Die Sichtbarkeit herstellen.** `make check` um `-- --nocapture` ergänzen —
   trifft dann aber die Ausgabe aller Proben.
3. **Die Frage verlegen.** Die Gegenrichtung ist eine Aussage über den
   Quelltext gegen die Laufzeit, keine Zusicherung; sie passt zu der
   Grenzprüfung in `xtask/src/release.rs`, die ihre Meldungen ungefiltert
   ausgibt.

Was nicht trägt, ist die heutige Kombination: ein Zweig, der läuft, schreibt und
nicht ankommt, samt einem Kopf, der ihn als Hinweis führt.
