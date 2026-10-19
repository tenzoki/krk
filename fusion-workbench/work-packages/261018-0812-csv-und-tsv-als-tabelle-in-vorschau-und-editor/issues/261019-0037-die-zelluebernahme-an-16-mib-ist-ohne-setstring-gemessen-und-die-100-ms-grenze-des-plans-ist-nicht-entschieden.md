Die Zellübernahme an 16 MiB ist ohne `setString:` gemessen, und die 100-ms-Grenze des Plans ist nicht entschieden
---
Der Plan verlangt, dass die Zellübernahme an einer 16-MiB-Datei samt `umbau_anwenden` und `stand_einsetzen` gemessen wird und dass über 100 ms ein Defekt im Paket steht, bevor es schließt. Die Messprobe lässt den teuersten Schritt weg, das Schreiben des ganzen Standes in die Textfläche, und auch einen Teil der Kopien in `umbau_anwenden`. Schon der gemessene Teil liegt bei knapp 60 ms. Ob die Grenze hält, ist damit nicht entschieden.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-schuldet-das-csv-paket-einen-abnahmelauf-gegen-die-zehn-zusagen.md

**Schwere:** Medium. Kein Auslieferungshalt, aber eine Schließbedingung des Pakets („Where this work stops“, fünfter Punkt) ist ohne Beleg.

**Beleg:**

- `crates/krk-ui/src/appkit/editor.rs`, Probe `messung_der_csv_tabelle_an_16_mib`, Doc-Kommentar: „`setString:` selbst misst sie nicht“.
- Gefahren am 261019 mit `cargo test --release -p krk-ui -- --ignored --nocapture messung_der_csv_tabelle_an_16_mib` (16 776 274 Bytes, 147 790 Zeilen, 12 Spalten): Index 18,41 ms; Zelle setzen 9,28 ms; Umkehrpunkt 12,12 ms; neuer Index und Sicht 26,98 ms; `NSString` 10,47 ms; **„Zellübernahme ohne setString: 58,85 ms“**; Spalte einfügen allein im Kern 90,03 ms.
- Nicht gemessen sind: `setString:` der verborgenen `NSTextView` mit 16 MiB, die Abschrift `modell.stand().to_owned()` in `Editorbereich::umbau_anwenden`, der Durchlauf in `Editormodell::bearbeiten` (`ist_in_gehaltener_form`, `in_gehaltene_form`), der Textvergleich in `trenntextsicht_fuer`, `trenntextspalten_angleichen` und `reloadData`.
- `inference:` mit diesen Schritten liegt die Zellübernahme über 100 ms und die Spalteneinfügung deutlich darüber; gemessen ist das nicht.

**Richtung der Behebung:** Den vollen Weg an einer Fläche messen (`an_einer_flaeche` baut eine `NSTextView` im Prüfmodul), oder die Zeit am Bündel mit einem Zeitstempel um `tabellenhandlung_ausfuehren` und `zelle_rechnen` nehmen. Liegt sie über 100 ms, ist das der Defekt an `stand_einsetzen`, den der Entscheid zur Architektur benannt hat: nur den Bereich des `Umkehrpunkts` ersetzen statt des ganzen Standes. Das betrifft jede Form.

**Abnahme:** Eine Zahl für den vollen Weg der Zellübernahme und der Spalteneinfügung an 16 MiB steht im Paket. Liegt sie über 100 ms, steht ein eigener Defekt mit dem Befund daneben.

---
Messung 261019 nach Verbilligung (offen, kein Resolved): Umgebaut ohne Architekturwechsel in `crates/krk-ui/src/appkit/editor.rs`: `umbau_anwenden` und `umkehren` reichen über `Einsatz::aus_umkehrpunkt` allein den geänderten Bereich an `stand_einsetzen`, das ihn per `replaceCharactersInRange:withString:` am Textspeicher ersetzt statt `setString:` (Rückfall auf den ganzen Stand, wenn der Bereich nicht passt); `stand_erneuern` zieht die Darstellung hinter einer Tabelle nicht mehr nach; `umbau_anwenden` schreibt den alten Stand nur noch ab, wenn die Wandlung zugreift; `gemeinsamer_anfang`/`gemeinsamer_schwanz` vergleichen blockweise; `koordinaten::einheiten` zählt UTF-16 über die Bytes. `cargo test --release -p krk-ui -- --ignored --nocapture messung_der_csv_tabelle_an_16_mib`, 16 776 274 Bytes, 147 790 Zeilen, 12 Spalten, zwei Läufe: Zelle setzen (Kern) 8,7–8,8 ms; Umkehrpunkt 2,0 ms (vorher 15 ms); neuer Index und Sicht 24,8–26,0 ms; Bereich samt NSString 0,5 ms; Bereich im Textspeicher (ohne Fläche) 0,03 ms gegen 15,6–16,2 ms für den ganzen Text; **Zellübernahme, Rechenweg 36,1–37,3 ms**; **Spalte einfügen, Rechenweg 127,4–130,4 ms** (davon Kern `spalte_einfuegen` 83,9–86,6 ms; der Bereich ist die ganze Datei). Nicht gemessen bleiben Layout der verborgenen `NSTextView` nach der Ersetzung, `reloadData` und der Kopf: eine Fläche lässt sich unter `libtest` nicht beschreiben. Die Spalteneinfügung liegt über 100 ms; die Zellübernahme hält die Grenze im Rechenweg, am Bündel ist sie nicht belegt.
