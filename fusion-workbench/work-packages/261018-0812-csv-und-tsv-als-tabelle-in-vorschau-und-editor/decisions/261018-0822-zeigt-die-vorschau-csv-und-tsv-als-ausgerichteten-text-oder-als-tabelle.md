# Zeigt die Vorschau CSV und TSV als ausgerichteten Text oder als Tabelle?

---
**Domain:** code
**Filed by:** implementation-planner, Kai Stalmann <kai@stalmann.org>
**Cross-references:** 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-plan-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md, 261018-0822-wird-eine-csv-datei-eine-weitere-form-der-eintragsansicht-und-rechnet-jede-tabellenhandlung-auf-dem-text.md

---

## Question

Der Nutzer will, dass die Vorschau „einfache tabellen handhaben“ kann, waagerecht und senkrecht scrollbar. Heute zeigt sie `.csv` und `.tsv` als eingefärbten Rohtext mit Umbruch, und über 1 MB nur die Angaben. Die Vorschau hat drei Flächen (Text, Bild, PDF-Betrachter); alles, was kein Bild und kein PDF ist, läuft über die Textfläche, auch gerendertes Markdown und die aufbereiteten JSON-Zeilen. Die Wahl bindet, wie jede spätere Tabellenart in der Vorschau erscheint.

## Options

1. **Ausgerichteter Text in der vorhandenen Textfläche.** Auf dem Faden `krk-vorschau` zerlegt der Kernleser die Datei und baut Zeilen mit festen Spaltenbreiten in der Festbreitenschrift: links die Zeilennummer, oben die Spaltenbuchstaben, zwischen den Spalten ein senkrechter Strich, eine Zelle über 40 Zeichen gekürzt mit „…“. Für diesen Inhalt bricht die Textfläche nicht um und trägt einen waagerechten Rollbalken. Über 1 MB zeigt sie die vollständigen Sätze des ersten MB mit Hinweis, wie bei `.jsonl`.
   - Pros: kein vierter Wert von `Flaeche`, kein neues Ziel für den Fokus und keine neue Kopierstelle; Auswahl und Kopieren der Runde 14 wirken unverändert; der Weg über Archive (`archiveintrag_laden`) bekommt die Tabelle ohne eigene Zeile, weil beide Leser über `Leseart::von` gehen; die Arbeit liegt auf dem Faden und endet, bevor `laedt_noch` falsch wird.
   - Cons: keine ziehbaren Spaltenbreiten und keine Zellauswahl; kopiert wird der ausgerichtete Text samt Strichen, nicht TSV; ostasiatische Breitzeichen richten sich nur über eine kleine feste Breitentafel aus.
2. **Eine lesende `NSTableView` als vierte Fläche der Vorschau.**
   - Pros: echte Spalten, ziehbare Breiten, Zeilenauswahl, Kopieren als TSV.
   - Cons: eine zweite Tabelle neben der des Editors, mit eigener Antwort auf `copy:` (die Zählprobe in `appkit/betrachter.rs` ändert sich), eigenem Eintrag in `fokusansicht` und in `flaeche_zeigen`, eigenem Zellenbau für große Dateien; das Zerlegen bliebe auf dem Faden, das Bauen der Spalten auf dem Hauptfaden.
3. **Wie heute, eingefärbter Rohtext, nur ohne Umbruch.**
   - Pros: kleinste Änderung.
   - Cons: keine Spalten; erfüllt den Auftrag nicht.

## Constraints

- Lesen und Aufbereiten nur auf dem Faden `krk-vorschau` (Modulkopf von `vorschaumodell.rs`).
- Die Bearbeitung gehört dem Editor; die Vorschau zeigt.

## Recommendation

Möglichkeit 1. Sie gibt die Tabelle mit Spalten und beiden Rollrichtungen auf dem Weg, den Markdown und JSON-Zeilen schon gehen, und ein Druck auf `cmd+e` holt dieselbe Datei als echte Tabelle in den Editor. Möglichkeit 2 bleibt offen für ein späteres Paket, falls der Nutzertest die Zellauswahl in der Vorschau vermisst.

---
Answered: 261018-0812-csv-und-tsv-als-tabelle-in-vorschau-und-editor.md — Empfehlung des Plans angenommen („ja“); ruled by user, Kai Stalmann <kai@stalmann.org>
