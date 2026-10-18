# Behält eine CSV- oder TSV-Datei beim Sichern ihre Bytefolgenmarke und ihre Zeilenenden?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 260808-0021_*_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md, 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md

---

## Question

Der Editor hält jede Datei als UTF-8 ohne Bytefolgenmarke und mit `\n` als einzigem Zeilenende, und er sichert immer so. Das hat der Nutzer am 260808-0043 gegen die Empfehlung entschieden (`260808-0021_*_was-sagt-der-editor-beim-sichern-ueber-den-unveraenderten-teil-der-datei-zu.md`, Möglichkeit 2, umgesetzt in `krk_core::text::datei::sicherungsform`). Für CSV hat diese Regel zwei Folgen, die bei einer Textdatei nicht auftreten. Excel schreibt „CSV UTF-8“ mit Bytefolgenmarke, und Excel liest eine CSV-Datei ohne Marke als Windows-1252: nach einem Sichern in KRK erscheinen dort „Ã¤“ statt „ä“. RFC 4180 nennt `\r\n` als Zeilenende, und Excel schreibt es; ein Sichern in KRK schreibt jede Zeile neu, auch die unveränderten. Der Auftrag dieses Pakets verlangt, dass unveränderte Teile der Datei ihre Form behalten. Die Frage muss vor dem Bau des Sicherns entschieden sein, weil sie über eine Nutzerentscheidung hinweggeht.

## Options

1. **Die Regel vom 260808 gilt auch für CSV und TSV.** Nichts ändert sich am Sichern.
   - Pros: eine Regel für jede Datei; kein neuer Zustand im Editormodell.
   - Cons: Umlaute in Excel kaputt nach jedem Sichern einer Datei, die Excel mit Marke geschrieben hat; jede Zeile einer CRLF-Datei ändert sich, und ein Versionsvergleich zeigt die ganze Datei als geändert.
2. **Eine Tabellendatei behält ihre Form; jede andere Datei bleibt bei der Regel vom 260808.** Beim Einlesen merkt sich der Editor für eine Datei mit der Endung `.csv`, `.tsv` oder `.tab`, ob sie mit einer Marke beginnt und ob ihr erstes Zeilenende `\r\n` ist (`Dateiform`). Der gehaltene Stand bleibt unverändert (ohne Marke, nur `\n`), sodass Zeilenindex, Suche und Textmarken weiter rechnen dürfen. `sicherungsform` stellt beim Schreiben Marke und `\r\n` wieder her.
   - Pros: eine Datei aus Excel kommt so zurück, wie sie kam, bis auf die geänderten Zellen; die Zusage des gehaltenen Standes bleibt unberührt, weil nur das Sichern eine Fallunterscheidung bekommt.
   - Cons: eine zweite Sicherungsform neben der ersten; eine Datei mit gemischten Zeilenenden kommt einheitlich zurück, in der Form ihrer ersten Zeile; ein einzelnes `\r` in einem Feld wird zu `\r\n`.
3. **Die Regel vom 260808 fällt für jede Datei.** Möglichkeit 1 des alten Datensatzes wird nachträglich gewählt.
   - Pros: eine Regel, und sie hält die Form überall.
   - Cons: ändert ein Verhalten, das der Nutzer ausdrücklich gewählt hat, für jede Datei und weit über dieses Paket hinaus.

## Constraints

- Der gehaltene Stand bleibt UTF-8 ohne Marke mit `\n` (Modulkopf von `crates/krk-core/src/text/datei.rs`); drei Stellen rechnen davon.
- Geschrieben wird weiter über `datei::sichern` und `ablage::atomar`; ein zweiter Schreibweg entsteht nicht.

## Recommendation

Möglichkeit 2. Der Unterschied zur Lage vom 260808 ist benannt: bei CSV trägt die Marke die Kodierung für Excel, und `\r\n` ist die Norm des Formats. Der Plan führt dafür einen eigenen Schritt, der bei Möglichkeit 1 entfällt; bei Möglichkeit 3 ist er ein eigenes Paket.

---
Answered: 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md — Empfehlung des Plans angenommen („ja“); ruled by user, Kai Stalmann <kai@stalmann.org>
