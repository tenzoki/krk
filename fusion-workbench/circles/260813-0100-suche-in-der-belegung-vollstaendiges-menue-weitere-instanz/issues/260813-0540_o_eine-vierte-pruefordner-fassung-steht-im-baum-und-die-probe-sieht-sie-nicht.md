Eine vierte Prüfordner-Fassung steht im Baum, und die C4.6-Probe sieht sie nicht

---

C4.6 sagt zu: es gibt genau drei Prüfordner-Fassungen, eine je Kiste. `CLAUDE.md` führt sie
namentlich, und die Abnahme von S12 verlangt „eine vierte Prüfordner-Fassung entsteht nicht".

**Die Runde hat eine vierte gebaut.** `crates/krk-core/src/ablage/sperre.rs:209-229` erklärt
`struct Ordner` mit `neu()`, das unter `std::env::temp_dir()` einen Ordner anlegt, und
`impl Drop for Ordner`, das ihn abräumt. Das ist der Gegenstand, den C4.6 zählt.

**Die Probe dazu findet sie nicht.** `genau_drei_pruefordner_fassungen_stehen_im_baum`
(`crates/krk-core/tests/baum.rs:67-100`) sucht für die Gegenprobe die Nadel
`impl Drop for Pruefordner`. Sie bindet damit an den **Namen** und nicht an die Sache: eine
vierte Fassung namens `Ordner` entgeht ihr, und dieselbe Nadel fände auch den anerkannten
`Wegwerfordner` in `crates/krk-bench/src/wegwerfordner.rs:54` nicht, wenn er neu hinzukäme.

**Die Begründung im Doc-Kommentar trägt die Hälfte.** `sperre.rs:202-208` schreibt richtig,
dass die Proben dieses Moduls neben dem Code stehen müssen, weil sie das kistenintern sichtbare
`Schreibgriff::nehmen` brauchen, und dass `tests/gemeinsam/` von dort nicht erreichbar ist. Das
begründet, warum es die vierte Fassung **gibt**; es macht aus ihr keine Nicht-Fassung. Der Satz
„das sind zwei Sichtbarkeiten und keine zweite Fassung derselben Sache" ist die Stelle, an der
die Zählung ausgehebelt wird, ohne dass jemand die Zusage geändert hätte.

**Dazu der Ort.** `Ordner::neu` und die Probe in `crates/krk-core/src/verzeichnis/sys.rs:950`
legen ihre Ordner und Dateien im echten `std::env::temp_dir()` an. `CLAUDE.md` warnt an dieser
Stelle bereits (`shared/issues/260810-1925_*`): `Messplanwaechter::neu` räumt dort fremde
Messpläne ab, und `cargo test` greift damit in dasselbe Verzeichnis wie ein laufender Messlauf.
Zwei weitere Greifer sind dazugekommen.

---

**Schwere:** mittel. Kein Fehlverhalten am Programm; eine Zusage, die gebrochen ist, und eine
Probe, die es nicht meldet — genau die Lage, gegen die die Probe geschrieben wurde.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-core/src/ablage/sperre.rs:202-229`,
`crates/krk-core/tests/baum.rs:67-100`,
`crates/krk-core/src/verzeichnis/sys.rs:946-995`

**Domain:** code

## Vorschlag

Zwei Fragen, und die erste gehört dem Nutzer.

1. **Ist die vierte Fassung erlaubt?** Wenn ja, gehört sie in `CLAUDE.md` und in die
   Aufzählung der Probe, mit ihrer Begründung; C4.6 heißt dann „vier, eine je Sichtbarkeit".
   Wenn nein, brauchen die Proben von `sperre.rs` einen anderen Weg an
   `Schreibgriff::nehmen` — etwa eine kisteninterne Hülle, die `tests/gemeinsam/` mitbenutzen
   kann.
2. **Die Gegenprobe unabhängig vom Namen machen.** Statt `impl Drop for Pruefordner` die
   Sache suchen, die den Gegenstand ausmacht: ein `impl Drop` in derselben Datei wie ein
   `create_dir_all` und ein `remove_dir_all`. Das findet jede vierte Fassung, gleich wie sie
   heißt.
