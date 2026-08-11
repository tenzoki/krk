Die Breite des Vorschaufensters fällt beim Navigieren in der Dateiliste zurück

---

Wer das Vorschaufenster am rechten Rand breiter zieht und danach in der Dateiliste navigiert,
findet es wieder auf seiner alten Breite. Die eingestellte Breite hält nicht.

Vom Nutzer gemeldet am 260811-1240.

---

**Schwere:** Mittel — es ist eine Zusage, die nicht hält, und sie trifft bei jeder Bewegung in
der Liste
**Gefunden:** Nutzer
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs`, `crates/krk-ui/src/appkit/aufteilung.rs`
**Domain:** code

## Warum das ein Defekt ist und keine fehlende Funktion

**Die Maschinerie dafür steht bereits.** Am Code geprüft am 260811-1245:

- `Breiten` (`crates/krk-core/src/ablage/sitzung.rs:181`) führt ein Feld je Bereich, darunter
  `vorschau: Option<f64>`.
- Der Modulkopf derselben Datei hält fest: „**C7 verlangt, dass Tabs, Ordner, Auswahl, Breiten,
  Sichtbarkeit und Sortierung einen Neustart überstehen.**"
- Der Kommentar an `Breiten` geht weiter: „Eine gespeicherte Zahl gilt auch für einen
  ausgeblendeten Bereich, weil C7 verlangt, dass das Wiedereinblenden die vorherige Breite
  wiederherstellt."

Eine Breite, die einen **Neustart** überstehen soll, muss eine Bewegung in der Dateiliste erst
recht überstehen. Der Befund widerspricht damit einer abgenommenen Zusage der Runde 1.

## Wo die Ursache zu suchen ist

`fenstermodell::bereichsbreiten(verfuegbar, breiten, sichtbar)`
(`crates/krk-ui/src/fenstermodell.rs:609`) ist laut Modulkopf von `aufteilung.rs` die **eine**
Stelle, an der die Breiten entstehen. `aufteilung.rs` sagt dazu: die Aufteilung wird neu
gerechnet, „wenn das Fenstermodell eine Breite oder eine Sichtbarkeit geändert hat, und wenn der
Nutzer das Fenster größer zieht. Im zweiten Fall speist es die Breiten ein, die …".

**Die Frage ist, ob ein Lesevorgang in der Dateiliste einen dieser beiden Anlässe auslöst,
obwohl er weder eine Breite noch eine Sichtbarkeit ändert.** Trifft das zu, rechnet die
Aufteilung mit einem Stand, den der Nutzer inzwischen überschrieben hat, und setzt seine
Ziehbewegung zurück.

`inference:`, nicht gemessen: der Nutzer zieht die Trennlinie in AppKit, und ob diese Bewegung im
Fenstermodell ankommt, ist die zweite mögliche Bruchstelle. Kommt sie dort nie an, hält das
Modell weiter den alten Wert, und jede Neurechnung stellt ihn her — dann wäre auch der Neustart
betroffen und C7 an dieser Stelle nie erfüllt gewesen.

**Wer den Defekt anfasst, misst zuerst, welche der beiden Bruchstellen es ist**, statt eine zu
vermuten: hält `Breiten::vorschau` nach dem Ziehen den neuen Wert, oder den alten?

## Zusammenhang

Dieser Defekt liegt in derselben Maschinerie wie die vom Nutzer am 260811 vorgeschlagene
Statusleiste mit Schaltern für die fünf Bereiche, deren jede Änderung eine Neuaufteilung
**proportional zur letzten** auslösen soll. Wer jene Runde plant, klärt diesen Defekt vorher oder
mit ihr: eine proportionale Neuaufteilung auf einer Grundlage, die die Ziehbewegung des Nutzers
nicht hält, verteilt die falschen Anteile.
