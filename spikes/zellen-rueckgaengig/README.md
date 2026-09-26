# Prüfprogramm: Was erreicht `cmd+z` in einer offenen Tabellenzelle?

**Wegwerf-Prüfcode, kein Produktcode.** Dieses Verzeichnis beantwortet eine einzelne
ungeprüfte Annahme und wird danach nicht weitergepflegt. Nichts hier gehört in KRK
übernommen. Die Frage ist beantwortet; das Verzeichnis bleibt nur als Beleg stehen.

## Die Frage

Defekt
`fusion-workbench/work-packages/260925-2356-f2-oeffnet-krkhome-statt-notizfenster/issues/260926-0813_*_cmd-z-bei-offener-zelle-nimmt-einen-tabellenumbau-zurueck-und-verliert-oder-verschiebt-den-getippten-text.md`
und die Zweitlesung `fusion-workbench/shared/consultations/260926-0811-zweitlesung-stufe-3-aufgabeneditor.md`,
Frage 1 und die zwei ersten offenen Fragen: Erreicht `cmd+z` in einer offenen Zelle einer
view-basierten `NSTableView` den Verwalter des Fensters, in dem ein Tabellenumbau steht?
Was tut `reloadData` mit einer offenen Zelle? Und nimmt AppKit einen eigenen Feldeditor
aus `windowWillReturnFieldEditor:toObject:` für diese Zellen an, samt eigenem Verwalter?

## Wie das Programm es misst

Eine Tabelle mit zwei Zeilen A und B, je Zeile ein bearbeitbares `NSTextField` in einer
`NSTableCellView` wie in `crates/krk-ui/src/appkit/eintragsansicht.rs`. Ein Hauptmenü
mit "Rückgängig" (`undo:`, `cmd+z`) und "Wiederholen". Ein Umbau vertauscht die Zeilen
und meldet seine Umkehrung am Verwalter des Fensters an, wie `umkehrung_anmelden`. Die
Tasten gehen über `NSApp.postEvent(_:atStart:)` in die eigene Ereignisschlange; eine
Unterklasse von `NSWindow` zeichnet auf, wann `undo:` dort ankommt.

Vier Durchgänge (`vorgabe`, `eigener`, `eigenerundo`, `eigenerfaengt`, dieser ein
zweites Mal ohne eigene Menüprüfung) mit je sieben Fällen; die Fälle 1 und 2 sind die Nutzerprüfungen 1 und 2 der Zweitlesung. Danach
`mehrzeilig.swift`, eine Kopie von `spikes/mehrzeilige-zelle/zelle.swift` mit dem
eigenen Feldeditor.

## Bauen und starten

```sh
cd spikes/zellen-rueckgaengig
./starten.sh alle      # je Durchgang ein messung-<durchgang>.txt
```

Voraussetzung sind nur die Command Line Tools.

## Das Ergebnis

Gemessen am **260926** auf **macOS 15.7.9**, Gerät **MacBookPro15,1**. Der Befund mit
Tabelle steht in `messungen/260926-0828-zellen-rueckgaengig.txt`.

- Mit dem Feldeditor von AppKit nimmt `cmd+z` in einer offenen Zelle ohne Getipptes den
  Umbau zurück und schließt die Zelle; nach Getipptem das zweite `cmd+z`.
- Ein eigener Feldeditor wird angenommen, aber ein überschriebenes `undoManager` genügt
  nicht: `NSWindow.undo:` nimmt trotzdem den Umbau zurück. Erst wenn der Feldeditor
  `undo:` und `redo:` selbst beantwortet, endet `cmd+z` am Anfang der Zelle. Grau wird
  der Menüeintrag dort nur mit einer eigenen Antwort auf `validateMenuItem:`.
- `reloadData` unter einer offenen Zelle ruft beide Delegiertenwege mit Zeile -1: der
  getippte Text fällt still.
- Die mehrzeilige Form aus 4.2 wächst mit dem eigenen Feldeditor genauso.
