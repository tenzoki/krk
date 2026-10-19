Vorschau und Editor zerlegen eine CSV mit einzelnem `\r` verschieden
---
Der Leser behandelt ein einzelnes `\r` als Zeichen des Feldes (E3). Die Vorschau zerlegt die Bytes der Datei und hält sich daran. Der Editor zerlegt seinen gehaltenen Stand, in dem `einlesen` jedes einzelne `\r` schon zu `\n` gemacht hat. Eine Datei mit `\r` als einzigem Zeilenende (Excel „CSV (Macintosh)“, ältere Mac-Programme) erscheint in der Vorschau als eine einzige Zeile voller `⏎` und im Editor als richtige Tabelle. Ein einzelnes `\r` in einem ungequoteten Feld teilt im Editor den Satz in zwei, und das nächste Sichern schreibt diese Teilung auf die Platte.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-behaelt-eine-csv-oder-tsv-datei-beim-sichern-ihre-bytefolgenmarke-und-ihre-zeilenenden.md

**Schwere:** Low. Selten. Der Entscheid zur Sicherungsform nennt „ein einzelnes `\r` in einem Feld wird zu `\r\n`“ als Preis, aber nicht die Teilung eines Satzes und nicht die abweichende Vorschau.

**Beleg (Stand `c97e264e`):**

- `crates/krk-core/src/trenntext/index.rs`, `satz_abtasten`: ein Satz endet nur an `\n` oder `\r\n`; ein `\r` davor ist ein Feldzeichen.
- `crates/krk-core/src/text/datei.rs`, `gehaltene_form`: jedes `\r` wird `\n`.
- `crates/krk-ui/src/vorschaumodell.rs`, `trenntext_aus_bytes`: zerlegt die Bytes der Datei unverändert.
- Der Modulkopf von `trenntext/index.rs` sagt „es ist derselbe Leser“. Das stimmt für den Leser, nicht für die Eingabe.
- `crates/krk-core/src/text/datei.rs`, `Dateiform::der_bytes`: eine Datei ohne `\n` bekommt `crlf: false` und wird mit `\n` gesichert.

**Richtung der Behebung:** Der Vorschau dieselbe Eingabe geben wie dem Editor. Bei einem Trenntext ohne `\n` mit `\r` die Bytes vor dem Zerlegen über `datei::gehaltene_form` führen, oder `\r` allein als Satzende lesen, wenn der Text kein `\n` trägt. Entscheiden lässt sich das am Text, ohne Raten.

**Abnahme:** Eine Probe zeigt eine CSV `a;b\r1;2\r` in der Vorschau mit zwei Zeilen, wie der Editor sie zeigt.

---
Resolved: Eine Regel für beide: `crates/krk-core/src/trenntext/mod.rs`, `cr_als_satzende` (kein `\n`, aber `\r`) und `lesbar`. Die Vorschau (`crates/krk-ui/src/vorschaumodell.rs`, `trenntext_aus_bytes`) zerlegt über `lesbar`; der Editor (`datei::trenntext_oeffnen`) hält einen solchen Text mit `\n` und schreibt über `Zeilenenden::NurCr` jedes `\n` wieder als `\r` (Byte für Byte). Ein einzelnes `\r` in einer Datei mit `\n` bleibt in beiden Feldzeichen, weil der Editor den CSV-Stand nicht mehr wandelt. Proben `eine_csv_mit_cr_als_zeilenende_zeigt_die_vorschau_wie_der_editor` (`vorschaumodell.rs`) und `eine_csv_mit_cr_als_zeilenende_kommt_byte_fuer_byte_zurueck` (`editormodell.rs`). Modulkopf von `trenntext/index.rs`, Abschnitt „Der Vorspann“, berichtigt.
