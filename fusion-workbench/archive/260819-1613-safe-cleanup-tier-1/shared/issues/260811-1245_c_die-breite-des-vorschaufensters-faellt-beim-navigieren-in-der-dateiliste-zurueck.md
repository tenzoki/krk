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

---

## Behebung 260811-2130 (Commit `1ea5a3d`)

**Es war Bruchstelle 1. Bruchstelle 2 trifft nicht zu.** `Breiten::vorschau` hält nach dem
Ziehen den **neuen** Wert — aber erst, wenn jemand nachmisst. Die Ziehbewegung steht allein in
den Rahmen der Ansichten, und der Delegierte der Aufteilung meldet sie bewusst nicht zurück.

**Der Defekt saß in der Reihenfolge.** `kommando_ausfuehren` rief nach jedem ausgeführten Befehl
`aufteilung_nachziehen()`, und das schreibt die Modellbreiten auf den Schirm — **bevor**
`sitzung_vormerken()` überhaupt nachmisst. Ein Ab-Pfeil in der Dateiliste ist ein solcher Befehl,
ändert weder Breite noch Sichtbarkeit, und stellte die Ziehbewegung trotzdem zurück.

**C7 war an dieser Stelle erfüllt, aber nur in einem Fall, den kaum jemand trifft:** ziehen und
sofort beenden, ohne einen Tastenbefehl dazwischen. `sitzung_bauen` misst vor dem Schreiben nach,
also überlebte die Breite den Neustart. Jeder Befehl dazwischen löschte sie vorher.

**Die Behebung** ist eine Zeile plus die Zusammenlegung der zwei bestehenden Messstellen
(`breite_aendern`, `sitzung_bauen`) zu einer Funktion `bildschirmbreiten_uebernehmen`. Sie läuft
jetzt am Kopf von `kommando_ausfuehren`, bevor ein Befehl das Modell anfassen kann.

**Der Zeitpunkt ist tragend und nicht beliebig.** `breiten_uebernehmen` entscheidet an der
Sichtbarkeit des **Modells**, welche gemessene Zahl es annimmt. Eine Messung *nach* einem
Umschaltbefehl brächte den am 260804 im laufenden Bündel gemessenen Fehler zurück, bei dem das
zweite Dateifenster auf 269 statt 406 Punkten wiederkam. Der Kommentar an
`bildschirmbreiten_uebernehmen` schreibt das aus.

**Am laufenden Bündel gemessen ist nichts davon**; die Diagnose steht am Programmtext. Eine
Vorhersage zum Nachprüfen: mit der **Maus** ausgewählt überlebte die gezogene Breite auch vorher
schon, weil ein Klick in die Liste nicht durch `kommando_ausfuehren` geht.

**Ein Befund nebenbei, nicht angefasst.** `MINDESTGROESSE` (`fenster.rs`) steht auf 780 Punkten
und deckt die vier Mindestbreiten der Runde 1 (120 + 240 + 240 + 160 = 760 plus 20 Luft). Der
Editor tritt mit 320 statt 160 an die Stelle der Vorschau, sein Vierersatz summiert sich auf 920.
**Zwischen 780 und 920 Punkten Fensterbreite wird der Editor unter sein Mindestmaß gedrückt.** Die
Zahl blieb stehen, weil 940 den nutzbaren Bereich für alle verkleinerte, die den Editor nicht
benutzen; der Befund steht jetzt in der Doku der Konstanten.

`Verification: make check — exit 0`

---
Abgleichsvermerk 260811-2157 (`reconciler`): **die Behauptung traegt am Baum.**
`bildschirmbreiten_uebernehmen()` steht in `crates/krk-ui/src/appkit/anwendung.rs:2577` und wird an
zwei Stellen gerufen: am Kopf von `kommando_ausfuehren` bei `:2048`, vor dem `match`, und in
`sitzung_bauen` bei `:4287`. Der Kommentar darueber (`:2040-2047`) nennt diesen Datensatz und den
Grund. `breite_aendern` (`:2474`) verweist auf dieselbe Funktion statt eine zweite Messstelle zu
halten.

**Was die Notiz selbst schon sagt und was der Abgleich bestaetigt:** am laufenden Buendel ist
nichts davon gemessen. Die Diagnose steht am Programmtext, und die Vorhersage zur Maus ist eine
Vorhersage. Das gehoert in die Abnahme der naechsten Runde, die die Aufteilung anfasst.

**Folge fuer einen anderen Datensatz:** die Frage
`circles/260811-1304-statusleiste-mit-bereichsschaltern/decisions/260811-1305_*_wird-der-vorschaubreiten-defekt-in-dieser-runde-behoben.md`
ist damit gegenstandslos geworden und im selben Abgleich auf beantwortet gezogen.
