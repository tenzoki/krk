# Prüfprogramm: Wächst eine mehrzeilige Tabellenzelle beim Tippen mit?

**Wegwerf-Prüfcode, kein Produktcode.** Dieses Verzeichnis beantwortet eine einzelne
ungeprüfte Annahme und wird danach nicht weitergepflegt. Nichts hier gehört in KRK
übernommen. Die Frage ist beantwortet; das Verzeichnis bleibt nur als Beleg stehen.

## Die Frage

Schritt 4.2 des Plans
`fusion-workbench/work-packages/260925-2356-f2-oeffnet-krkhome-statt-notizfenster/plans/260926-0050_*_plan-f2-oeffnet-krkhome-mit-notizen-aufgaben-geheimnissen.md`:
die Notiztabelle aus 4.3 braucht eine Zeile, die während des Tippens im Feldeditor
mitwächst, und im Baum hat das kein Vorbild. Gefragt ist je getippter Zeile nach der
Zeilenhöhe, ob sie ohne Zutun wächst, ob sie nach `noteHeightOfRowsWithIndexesChanged:`
wächst, und welcher Ersthelfer danach steht.

## Wie das Programm es misst

Eine `NSTableView` mit zwei Spalten und `usesAutomaticRowHeights`, je Zelle ein
mehrzeiliges `NSTextField` mit `preferredMaxLayoutWidth`, der Delegierte bildet
`insertNewline:` auf `insertNewlineIgnoringFieldEditor:` ab. Das Programm beginnt die
Bearbeitung über `editColumn:row:withEvent:select:`, tippt über
`NSApp.postEvent(_:atStart:)` drei Zeilen in die eigene Ereignisschlange und misst nach
jeder. Es kommt selbst in den Vordergrund und beendet sich nach etwa drei Sekunden.

## Bauen und starten

```sh
cd spikes/mehrzeilige-zelle
./starten.sh alle      # sieben Durchgänge, je ein messung-<durchgang>.txt
```

Voraussetzung sind nur die Command Line Tools.

## Das Ergebnis

Gemessen am **260926** auf **macOS 15.7.9**, Gerät **MacBookPro15,1**. Der Befund mit
Tabelle steht in `messungen/260926-0818-mehrzeilige-zelle.txt`.

- Ohne Zutun wächst die Zeile nicht, nach `noteHeightOfRowsWithIndexesChanged:` auch
  nicht, selbst mit `invalidateIntrinsicContentSize` davor: das Feld meldet während der
  Bearbeitung die Höhe seines alten Werts.
- Sie wächst, wenn das Feld vertikal den Stauchwiderstand `.required` trägt und während
  der Bearbeitung die belegte Höhe des Feldeditortexts als Eigenhöhe meldet, je Anschlag
  aus `controlTextDidChange:` ungültig gemacht. `noteHeight…` braucht es dann nicht.
- Der Feldeditor bleibt in jedem Durchgang Ersthelfer.
