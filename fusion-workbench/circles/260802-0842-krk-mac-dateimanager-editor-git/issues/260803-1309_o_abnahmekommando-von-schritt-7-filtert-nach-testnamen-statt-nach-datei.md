Das Abnahmekommando `cargo test -p krk-core tasten` lässt sieben von acht Prüfungen aus

---

Das Abnahmekriterium von S7 verlangt: "`cargo test -p krk-core tasten` prüft die
Normalisierung". Das Wort hinter dem Paketnamen ist für `cargo test` kein Dateiname,
sondern ein Filter über die **Testnamen**. Von den acht Prüfungen in
`crates/krk-core/tests/tasten.rs` trägt genau eine das Wort `tasten` im Namen; die
beiden Prüfungen, die das Kriterium ausdrücklich verlangt, laufen unter diesem
Kommando nicht.

---

**Gemessen am 260803-1309:**

```
$ cargo test -p krk-core tasten
running 5 tests    (Einheitenprüfungen unter src/tasten/, Pfad enthält "tasten")
running 1 test     (tests/tasten.rs, nur die_fuenf_verdrahteten_tasten_liefern_ihr_kommando)
                   7 filtered out
$ cargo test -p krk-core --test tasten
running 8 tests    test result: ok. 8 passed
```

Die fünf Einheitenprüfungen kommen mit, weil ihr voller Name mit `tasten::` beginnt.
Die Integrationsprüfungen tragen diesen Präfix nicht: eine Integrationsprüfung liegt
in einem eigenen Testprogramm, dessen Name nicht Teil des Prüfungsnamens ist.

Betroffen sind unter anderem die beiden Prüfungen, die das Kriterium wörtlich nennt:
`f3_mit_und_ohne_function_ergibt_dieselbe_nachschlagemaske` und
`cmd_shift_k_behaelt_beide_bits`.

**Zwei mögliche Auflösungen, und warum die erste vorzuziehen ist.**

1. **Das Kriterium auf `cargo test -p krk-core --test tasten` ziehen.** `--test`
   wählt das Testprogramm und nicht den Namen; damit läuft die Datei vollständig.
   Wer zusätzlich die Einheitenprüfungen des Moduls sehen will, ruft
   `cargo test -p krk-core tasten` daneben auf. Der Code bleibt unberührt.
2. Jeder Prüfung in der Datei das Wort `tasten` in den Namen schreiben. Damit ginge
   das Kommando des Plans buchstäblich auf, aber die Namen trügen ein Wort, das die
   Datei ohnehin schon sagt, und der Filter bliebe eine Falle für die nächste
   hinzukommende Prüfung.

**Warum es auffällt und nicht nur formal ist.** Ein Kriterium, das mit Rückgabewert 0
endet, ohne die Prüfungen ausgeführt zu haben, die es benennt, gibt eine Zusage ohne
Deckung. Dasselbe Muster steckt in S2 und S15, die `cargo test -p krk-core` ohne
Filter verlangen und deshalb nicht betroffen sind.
