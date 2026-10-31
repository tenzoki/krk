Das Setzen der Tabellenmerkmale baut je Zelle die Grundschrift neu und misst die Breite eines Leerzeichens neu
---
`auszeichnungen_legen` in `crates/krk-ui/src/appkit/textmerkmale.rs` ruft im Arm `Auszeichnung::Tabellenzelle` für jede Zelle `grundschrift(ansicht, art)`, und `zellenmerkmal` baut daraus mit `grundabsatz` einen Absatzstil, dessen `defaultTabInterval` `leerzeichenbreite` mit `sizeWithAttributes:` an einem `" "` misst. Schrift und Leerzeichenbreite sind für alle Zellen eines Rufs dieselben; sie hängen allein an `ansicht` und `art`. Bei der Abnahmedatei des Pakets (124 320 Zellen) kostet das im Nachbau 1,48 s auf dem Hauptfaden statt 0,42 s, also rund dreieinhalbmal so viel wie das Setzen der Blöcke selbst. Derselbe Arm von `Listenzeile` ruft `grundschrift` ebenso je Stelle; dort sind es wenige Stellen je Datei.
---
**Filed by:** analyst, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-0015-ab-welcher-groesse-zeigt-die-vorschau-eine-markdown-tabelle-nicht-mehr-als-eine-nstexttable.md (`## Befund: wo die Zeit hingeht`); 261031-1720-plan-markdown-vorschau-richtet-tabellenspalten-aus.md

## Beleg

- Nachbau in Swift unter `/private/tmp/claude-501/-Users-kai-Projects-productive-F08-KRK-krk/0d876e38-8be9-4836-8bba-c89e56b51c4e/scratchpad/tabmess/mess2.swift`, gemessen am 261101 auf einem M2 Max: 31 079 Zeilen zu 4 Spalten ohne den Zusatz je Zelle 0,424 s, mit Systemschrift und `sizeWithAttributes:` je Zelle 1,476 s; 1 000 Zeilen 0,063 s, 4 000 Zeilen 0,206 s, 8 000 Zeilen 0,390 s mit Zusatz.
- Am Bündel nicht gemessen; der Nachbau rechnet `grundmerkmale` und die Fallunterscheidung über `Ansicht` und `Darstellungsart` nicht mit.

## Abnahme

`auszeichnungen_legen` bestimmt Grundschrift und Grundabsatz höchstens einmal je Ruf und nicht je Stelle. Die Probe `zwei_zellen_einer_tabelle_teilen_ihr_nstexttable` bleibt grün, und jede Zelle trägt weiter einen eigenen Absatzstil mit ihrem eigenen Block. Ob der Absatzstil geteilt wird, entscheidet der Bau; ein `NSMutableParagraphStyle` je Zelle bleibt nötig, weil jeder seinen eigenen `NSTextTableBlock` trägt.

**Resolved:** `75850c88` — `auszeichnungen_legen` misst den Tabschritt einmal je Ruf (`tabschritt`, `absatz_mit_schritt`), Probe `auszeichnungen_legen_misst_den_tabschritt_einmal_je_ruf`.
