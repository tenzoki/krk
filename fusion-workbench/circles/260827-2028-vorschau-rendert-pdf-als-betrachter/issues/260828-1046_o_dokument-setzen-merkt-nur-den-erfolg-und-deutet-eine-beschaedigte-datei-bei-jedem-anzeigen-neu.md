`dokument_setzen` merkt nur den Erfolg und deutet eine beschädigte Datei bei jedem `anzeigen` neu

---

`crates/krk-ui/src/appkit/betrachter.rs:499-537`, `Pdfbetrachter::dokument_setzen`: der Merkposten `bytes` wird allein am Ende des Erfolgszweigs geschrieben (`:535`). Bei `Deutung::Beschaedigt` (`:514-518`) und `Deutung::Gesperrt` (`:519-521`) kehrt die Funktion vorher zurück. Der `Arc::ptr_eq`-Vergleich am Anfang (`:500-508`) trifft deshalb für dieselben Bytes nie, und jeder weitere Durchlauf von `Vorschaufenster::anzeigen` (`vorschau.rs`, gerufen bei jedem Tabwechsel über `tab_waehlen` und `kommando_ausfuehren`, bei jedem `einziehen` eines laufenden Ladens und bei jedem `datei_anzeigen`) reicht dieselben Bytes erneut an `PDFDocument::initWithData:` auf dem Hauptfaden.

Bei einer abgeschnittenen Datei knapp unter `BILDGRENZE` (64 MB) ist das ein wiederholter Leselauf von PDFKit über die Bytes, jedes Mal ohne Ergebnis; bei einer kennwortgeschützten Datei ein wiederholtes Deuten der Querverweistabelle. Der Doc-Kommentar (`:489-492`) beschreibt das Verhalten („in beiden Faellen bleibt das vorige Dokument samt Merkposten stehen") und hält es für gewollt, nennt aber den Preis nicht.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Betroffen:** `crates/krk-ui/src/appkit/betrachter.rs` (`PdfbetrachterIvars::bytes`, `dokument_setzen`); `crates/krk-ui/src/appkit/vorschau.rs` (`pdf_zeigen`, unverändert im Verhalten)
**Schwere:** Low (kein Kriterium des Specs verletzt; Kosten nur bei beschädigten oder gesperrten Dateien und nur auf dem Hauptfaden, dessen Deuten ohnehin ungemessen ist)

Fix: den Merkposten als `RefCell<Option<(Arc<Vec<u8>>, Deutung)>>` führen; bei `ptr_eq` die gemerkte `Deutung` liefern, in allen drei Zweigen schreiben. `pdf_zeigen` verzweigt dann wie heute, und ein Tabwechsel auf eine beschädigte Datei kostet kein zweites Deuten. Das vorige Dokument bleibt dabei in der Ansicht stehen, wie jetzt; der Betrachter ist in diesen Fällen verborgen. Der Doc-Kommentar nennt danach beide Merkposten.

Abnahme: eine Probe ohne Fenster ist nicht möglich, weil `PDFDocument` AppKit ist; die Quellbaumprobe `zoom_und_deutung_tragen_je_genau_drei_werte` bleibt grün, und der Nachweis ist die Lesung des Rumpfes. Wer den Preis messen will, wählt eine abgeschnittene 60-MB-Datei und wechselt den Tab zehnmal.
