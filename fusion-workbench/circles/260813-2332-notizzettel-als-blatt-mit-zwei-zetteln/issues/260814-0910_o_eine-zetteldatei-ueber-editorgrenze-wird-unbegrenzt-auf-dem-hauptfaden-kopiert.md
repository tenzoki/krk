Eine Zetteldatei über EDITORGRENZE wird unbegrenzt auf dem Hauptfaden kopiert

---

Der Spec sagt im Abschnitt zum Verhältnis zu den zehn Zeitzusagen zu: „Der Zettel liest und
schreibt seine Datei auf dem Hauptfaden, und die obere Schranke dafür ist `EDITORGRENZE`
mit 16 MB. Eine Datei darüber wird nicht geladen, sondern beiseitegelegt."

Die Schranke gilt für das **Laden**. Für das Beiseitelegen gilt sie nicht:
`Zugang::text_laden` (`crates/krk-core/src/ablage/mod.rs:564`, Zweig `:595`) reicht im Zweig
`Textstand::Unlesbar` den offenen Deskriptor an `beiseite_legen` weiter, das über
`atomar::schreiben` in `io::copy(quelle, &mut datei)` mündet
(`crates/krk-core/src/ablage/atomar.rs:156`) — ohne `take`, ohne Obergrenze. Eine Datei von
40 GB unter dem Namen `note-1.txt` wird bei jedem `f2` vollständig kopiert, synchron auf
dem Hauptfaden und unter dem gehaltenen Schreibgriff.

---

**Schwere:** mittel. Die Oberfläche steht für die Dauer der Kopie, eine zweite Instanz von
KRK wartet an der Sperre, und der Ablageordner wächst um die Größe der Fremddatei.

**Erreichbar ist der Fall genau so, wie C5 ihn beschreibt.** Der Spec lädt den Nutzer
ausdrücklich ein, die Zetteldateien mit fremden Programmen zu öffnen und zu ändern; die
Grenze steht dafür da, „den Fall abzufangen, in dem eine fremde Datei unter dem Namen eines
Zettels liegt". Genau dieser Fall löst die Kopie aus.

**Der Editor tut das nicht.** `text::datei::oeffnen` lässt den Deskriptor im Fall `ZuGross`
fallen (`crates/krk-core/src/text/datei.rs:498`) und kopiert nichts. Das Verhalten ist mit
dieser Runde neu.

**Es ist zuerst eine Lücke im Spec und dann eine im Bau.** C5 verlangt „Eine Zetteldatei
über `EDITORGRENZE` wird nicht geladen und geht denselben Weg beiseite", und der Bau tut
genau das. Was weder Spec noch Plan festlegen, ist, wie groß „beiseite" werden darf. Die
Frage gehört vor eine Behebung: eine Obergrenze auch für die Kopie, ein Verzicht auf die
Kopie oberhalb einer zweiten Zahl, oder ein Umbenennen statt eines Kopierens — Letzteres
verstieße gegen die erste der drei Regeln von `beiseite_legen`.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Die Probe `eine_zu_grosse_zetteldatei_wird_nicht_geladen_und_geht_beiseite`
  (`crates/krk-core/tests/ablage.rs`) misst den Fall an `EDITORGRENZE + 1` und hält die
  Vollständigkeit der Kopie ausdrücklich fest; sie sagt über die obere Grenze nichts.
- Der Modulkopf von `ablage/mod.rs` schreibt die Eigenschaft mit aus: „Sie wird dabei aus
  ihrem offenen Deskriptor kopiert und steht zu keinem Zeitpunkt vollständig im
  Arbeitsspeicher." Das trifft zu und beantwortet die Frage nach der Zeit nicht.
