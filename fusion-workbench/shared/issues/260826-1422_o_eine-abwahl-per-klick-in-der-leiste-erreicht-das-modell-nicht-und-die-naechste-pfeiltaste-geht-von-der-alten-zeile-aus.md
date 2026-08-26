Eine Abwahl per Klick in der Leiste erreicht das Modell nicht, und die nächste Pfeiltaste geht von der alten Zeile aus

---

`crates/krk-ui/src/appkit/leiste.rs:570` setzt `setAllowsEmptySelection(true)`; ein Cmd-Klick auf die
gewählte Zeile hebt die Auswahl der `NSTableView` damit auf. `auswahl_geaendert` (`:233-244`)
liest dann `selectedRow() == -1`, `usize::try_from` scheitert, und die Funktion kehrt still zurück
(`:238-240`), ohne `modell.waehlen`. Das `Leistenmodell` hält weiter die alte Zeile. Der nächste
`AuswahlHoch`/`AuswahlRunter` (`:345-360`) bewegt `auswahl_bewegen` vom alten Stand, ruft
`auswahl_anzeigen` und `auswahl_melden` — die Auswahl springt scheinbar aus dem Nichts an die
Nachbarzeile der Zeile, die der Nutzer eben abgewählt hat, und navigiert dorthin.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/leiste.rs`

Denselben Sprung „aus dem Nichts" beschreibt `gueltigkeit_nachziehen` (`:296-301`) als Fehler, den
`auswahl_anzeigen` nach `reloadData` abwehrt; der Klickweg umgeht die Abwehr, weil die Meldung
mit `-1` verworfen statt an das Modell gegeben wird. Ob `Leistenmodell` eine leere Auswahl
überhaupt kennt, ist an dieser Datei nicht zu sehen; `auswahl_anzeigen` (`:391`) hat einen
`None`-Zweig mit `deselectAll:`, also kennt es sie. Zwei Wege: `setAllowsEmptySelection(false)`
wie in `belegungsansicht.rs:651` (dann ist der Klick nicht möglich), oder `-1` als Abwahl an das
Modell weitergeben. Nicht am Bündel gemessen; die Kette ist am Code abgelesen.
