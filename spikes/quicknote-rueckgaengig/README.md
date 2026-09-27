# Prüfprogramm: Erreicht `cmd+z` in einer leeren Quicknote die Datei darunter?

**Wegwerf-Prüfcode, kein Produktcode.** Dieses Verzeichnis beantwortet eine einzelne
ungeprüfte Annahme und wird danach nicht weitergepflegt. Nichts hier gehört in KRK
übernommen. Die Frage ist beantwortet; das Verzeichnis bleibt nur als Beleg stehen.

## Die Frage

Defekt
`fusion-workbench/work-packages/260926-2246-f10-oeffnet-quicknote-mit-fluechtigem-puffer/issues/260927-0225_*_ob-cmd-z-in-einer-leeren-quicknote-die-datei-darunter-erreicht-ist-ungemessen-und-ein-spike-koennte-es-messen.md`:
Die Quicknote ist eine gewöhnliche `NSTextView`, deren Delegierter
`undoManagerForTextView:` mit einem eigenen Verwalter beantwortet; die Textfläche
des Editors darunter meldet am Verwalter des Fensters an. Welchen Verwalter nimmt
`NSWindow.undo:`, wenn der eigene Stapel der Quicknote leer ist?

## Wie das Programm es misst

Zwei deckungsgleiche Textflächen in einem Fenster, „Datei“ am Verwalter des Fensters
und „Quicknote“ mit eigenem Verwalter über den Delegierten, getauscht wie in
`flaeche_waehlen`. Am Verwalter des Fensters stehen getippter Text und eine
angemeldete Handlung. Fünf Zustände mit leerem eigenem Stapel, dazu eine Gegenprobe
mit der Datei vorn. `cmd+z` geht über `NSApp.postEvent(_:atStart:)`, `shift+cmd+z`
über den geprüften Menüeintrag; warum, steht an `shiftCmdZ` in `quicknote.swift`.
Wie der Vorgänger `spikes/zellen-rueckgaengig/` läuft das Programm ohne KRK und
ohne Nutzerarbeit: es kommt als `.regular` selbst nach vorn.

## Bauen und starten

```sh
spikes/quicknote-rueckgaengig/starten.sh    # schreibt messung.txt daneben
```

Voraussetzung sind nur die Command Line Tools.

## Das Ergebnis

Gemessen am **260927** auf **macOS 15.7.9**, Gerät **MacBookPro15,1**. Der Befund mit
Tabelle steht in `messungen/260927-0232-quicknote-rueckgaengig.txt`.

- In keinem Zustand erreicht `cmd+z` oder `shift+cmd+z` den Verwalter des Fensters.
- Bei leerem eigenem Stapel ist „Rückgängig“ grau, das Tastenäquivalent löst nichts
  aus; und an der Menüprüfung vorbei nimmt `NSWindow.undo:` den eigenen Verwalter
  der Quicknote und tut nichts.
- Anders als beim Feldeditor der Messung vom 260926 genügt für eine gewöhnliche
  Textfläche der Verwalter über den Delegierten.
