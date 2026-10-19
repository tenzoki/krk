Die Vorschau entscheidet „`\r` als Satzende“ am ersten Megabyte, der Editor an der ganzen Datei
---
`trenntext::cr_als_satzende` ist die eine Regel für Vorschau und Editor: kein `\n`, aber ein `\r`. Der Editor fragt sie an der ganzen Datei (bis `EDITORGRENZE`), die Vorschau einer Datei über `TEXTGRENZE` (1 MiB) allein an deren Anfang. Eine CSV über 1 MiB mit `\r` als Satzende, die erst nach dem ersten MiB ein `\n` trägt, zeigt die Vorschau als Tabelle mit vielen Zeilen und der Editor als einen einzigen Satz, in dem jedes `\r` ein Feldzeichen ist. Damit kehrt der Befund `261019-0037-vorschau-und-editor-zerlegen-eine-csv-mit-einzelnem-cr-verschieden.md` für große Dateien zurück.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261019-0037-vorschau-und-editor-zerlegen-eine-csv-mit-einzelnem-cr-verschieden.md, 261019-1122-reviewer-csv-und-tsv-schlussdurchsicht-vor-dem-schliessen.md

**Schwere:** Low. Selten: eine CR-Datei über 1 MiB mit einem späten `\n`.

`inference:` der häufigste Weg zu einem solchen `\n` ist ein Umbruch in einer Zelle. Excel speichert einen Umbruch in einer Zelle als `\n`. Schreibt es ihn auch in „CSV (Macintosh)“ als `\n`, dann fällt eine solche Datei mit einer einzigen mehrzeiligen Zelle schon unter 1 MiB nicht unter die Regel, und Vorschau wie Editor zeigen sie gleich, aber als einen Satz. Am Format nicht geprüft; ich habe keine solche Datei von Excel gesehen.

**Beleg (Stand `6eae327a`):**

- `crates/krk-ui/src/vorschaumodell.rs`, `trenntext_aus_bytes`: über `TEXTGRENZE` wird `bytes` auf `TEXTGRENZE` gekürzt und erst danach `trenntext::lesbar(...)` gefragt.
- `crates/krk-core/src/text/datei.rs`, `trenntext_oeffnen`: `cr_als_satzende(&text)` über den ganzen gelesenen Text.
- `crates/krk-core/src/trenntext/mod.rs`, `cr_als_satzende`: `!bytes.contains(&b'\n') && bytes.contains(&b'\r')`.
- Die Probe `eine_csv_mit_cr_als_zeilenende_zeigt_die_vorschau_wie_der_editor` (`crates/krk-ui/src/vorschaumodell.rs`) nimmt eine Datei unter der Grenze.

**Richtung der Behebung:** Die Vorschau stellt die Frage an derselben Eingabe wie der Editor. Für eine Datei über `TEXTGRENZE` heißt das: vor dem Kürzen die ganze Datei auf ein `\n` prüfen (ein `memchr` über höchstens die Bytes, die der Editor ohnehin liest, auf dem Faden der Vorschau), oder die Frage auf den gelesenen Anfang beschränken und das im Modulkopf von `trenntext/mod.rs` als Grenze der Regel nennen. Ob die Regel Mac-Dateien mit einem Umbruch in einer Zelle decken soll, ist eine eigene Frage.

**Abnahme:** Eine Probe legt eine CSV über `TEXTGRENZE` an, mit `\r` als Satzende und einem `\n` hinter der Grenze, und zeigt, dass Vorschau und Editor dieselbe Zeilenzahl ergeben, oder der Modulkopf nennt die Grenze und eine Probe hält sie fest.
