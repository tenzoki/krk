# Ein BLOB wird ganz kopiert, um seine Länge zu nennen, und eine Zeile ist unbegrenzt

---
Für den Platzhalter `[BLOB, n Bytes]` braucht die TSV allein die Länge. `Abfrage::wert` kopiert trotzdem jeden BLOB ganz in einen `Vec<u8>`. Und weil die Grenze der Vorschau erst beim Reichen eines Blocks greift, wird eine Zeile mit einer großen Zelle vollständig in den Speicher geholt, bevor die Vorschau abbrechen kann.

---
**Domain:** code
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Assigned to:** code-implementer
**Severity:** Medium
**Cross-references:** 261023-1507-wie-stehen-null-und-blob-in-der-tsv-einer-sqlite-tabelle.md, 261023-1507-plan-sqlite-datenbanken-im-dateifenster-und-in-der-vorschau.md (P7)

## Befund

- `crates/krk-core/src/verzeichnis/sys.rs`, `Abfrage::wert` und `blobbytes`: `SQLITE_BLOB => Wert::Blob(self.blobbytes(spalte)?)`, und `blobbytes` macht `to_vec()` über die ganze Länge.
- `crates/krk-core/src/archiv/sqlite.rs`, `feldtext`: `Wert::Blob(bytes) => anzahl(Zahlwort::SqliteBlob, bytes.len() as u64, &[])`. Die Bytes werden nur gezählt.
- `tabelle_lesen`: eine Zeile entsteht ganz (`naechste_zeile`), dann `satz_anhaengen`, und erst ab `BLOCK` Bytes geht ein Block an den Empfang. Die Lesegrenze der Vorschau (`archiveintrag_laden`, `art.grenze()`) greift deshalb frühestens nach der ersten vollständigen Zeile. Ein Textfeld wird dabei dreimal gehalten (`String`, `feld_kodieren`, Block).
- Weder `sqlite3_limit(SQLITE_LIMIT_LENGTH, …)` noch eine Grenze je Zelle steht in der Bindung. Eine Zelle darf in SQLite bis zu 1 GB groß sein (Vorgabe von `SQLITE_MAX_LENGTH`), und eine erzeugte Spalte (`GENERATED ALWAYS AS … VIRTUAL`) stellt sie beim Lesen erst her (`zeroblob`, `randomblob`; inference aus der Dokumentation von SQLite, nicht nachgemessen). Eine fremde `.db`, die der Nutzer nur auswählt, reicht dafür.

## Erwartet

- Für einen BLOB allein `sqlite3_column_bytes` fragen. `Wert::Blob` trägt dann die Länge statt der Bytes, oder ein eigener Wert tut es.
- Eine Grenze je Zelle oder Zeile, die Vorschau und Ausfuhr gleich behandelt, etwa über `sqlite3_limit` an der Verbindung. Ob die Ausfuhr übergroße Texte abschneiden darf, ist eine Nutzerfrage.

## Abnahme

Eine Probe liest eine Tabelle mit einem BLOB von einigen hundert MB über `tabelle_lesen` in der Vorschau-Grenze und hält den Speicherhöchststand im Kind fest (`/usr/bin/time -l`), wie `ein_langer_name_ueber_der_grenze_macht_das_tar_ungueltig_ohne_ihn_zu_lesen` in `crates/krk-core/tests/archiv.rs`.
