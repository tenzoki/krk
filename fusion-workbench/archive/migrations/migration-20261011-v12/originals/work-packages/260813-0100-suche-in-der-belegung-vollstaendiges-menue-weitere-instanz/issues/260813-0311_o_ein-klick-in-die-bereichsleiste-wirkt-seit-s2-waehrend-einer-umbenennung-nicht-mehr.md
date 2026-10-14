Ein Klick in die Bereichsleiste wirkt seit S2 während einer Umbenennung nicht mehr

---

Der Klick auf einen Schalter der Bereichsleiste geht durch
`Anwendungsdelegierter::kommando_ausfuehren`
(`crates/krk-ui/src/appkit/anwendung.rs:788-800`). Seit S2 stellt diese Stelle die
Zulässigkeitsfrage mit **drei** Bestandteilen statt zwei, und der neue mittlere fragt nach dem
Ersthelfer. Damit erbt der Mausklick eine Bedingung, die für den Tastendruck gedacht ist:
solange der Feldeditor einer Umbenennung in der Dateiliste den Ersthelferrang hält, weist
`zulaessig` jeden Schalter der Bereichsleiste ab, und der Klick tut nichts.

Vor S2 kam der Klick durch: `kommando_ausfuehren` fragte allein nach dem stehenden Blatt und
nach `fokus::wirkt`, und beide sagen in dieser Lage ja.

---

**Schwere:** mittel (ein Bedienweg, der heute wirkt, wirkt danach in einer benannten Lage
nicht mehr; kein Datenverlust, keine falsche Wirkung)
**Gefunden:** coder, beim Bauen von S2 der Runde 7 am 260813-0311
**Betroffen:** `crates/krk-ui/src/appkit/anwendung.rs:788-800`,
`crates/krk-ui/src/appkit/bereichsleiste.rs:478`,
`crates/krk-ui/src/kommandos/zulaessigkeit.rs`
**Domain:** code

## Warum der Ersthelfer beim Klick nicht wechselt

Jeder Schalter der Bereichsleiste trägt `setRefusesFirstResponder(true)`
(`crates/krk-ui/src/appkit/bereichsleiste.rs:478`, begründet im Modulkopf unter C1.4). Ein
Klick darauf nimmt dem Feldeditor seinen Rang deshalb **nicht** ab; der Ersthelfer bleibt das
Textfeld, und `zulaessig` antwortet für einen Befehl, der nicht auf der Ausnahmeliste steht,
`false`. Ohne diese Zeile wäre der Fall gar nicht erst entstanden — der Schalter würde den
Rang übernehmen und der Ersthelferbefund umschlagen.

`inference:` Am laufenden Bündel ist das nicht nachgesehen; es ist aus den beiden Zeilen im
Baum abgelesen. Der Beleg dafür wäre ein Klick auf den Vorschau-Schalter mitten in einer
Umbenennung, und der gehört zum Abnahmelauf und damit zur Nutzerarbeit.

## Warum der Plan es nicht nennt

S2 sagt „Verhalten: unverändert gegenüber heute, in allen drei Ausgängen des Nachschlags", und
für die drei Ausgänge des Nachschlags stimmt der Satz. Der Klick in die Bereichsleiste ist
kein Ausgang des Nachschlags: er ist ein zweiter, älterer Aufrufer derselben Senke, und der
Plan rechnet ihn nicht mit. Die Randbedingung „kein Verlust gegenüber heute" aus dem Spec
trifft ihn trotzdem.

## Drei Wege

1. **Nichts tun und den Preis benennen.** Eine Umbenennung in der Liste ist ein kurzer
   Zustand, und der Nutzer verlässt sie mit Eingabe oder Escape. Dieselbe Regel gilt nach
   C2.19 ohnehin für jeden Menüeintrag, also auch für die Vorschau-Umschaltung über das Menü;
   die Bereichsleiste wäre dann die dritte Fläche mit derselben Antwort statt einer Ausnahme.
2. **Den Schalter den Ersthelferrang nehmen lassen.** `setRefusesFirstResponder(true)` fällt
   weg, der Klick beendet die Umbenennung, und der Befehl kommt durch. Der Preis steht im
   Modulkopf der Bereichsleiste unter C1.4 und wäre neu zu bewerten.
3. **Den Klick an der Ersthelferfrage vorbeiführen.** Der Melder gäbe eine `Lage` mit
   `ersthelfer_gehoert_appkit: false` mit, weil ein Mausklick nicht danach fragt, wem die
   Taste gehört. Das ist der sauberste Schnitt und zugleich der teuerste: `kommando_ausfuehren`
   bekäme die `Lage` als Parameter, statt sie selbst zu erheben, und alle Aufrufer müssten sie
   liefern.

Der dritte Weg berührt den Zuschnitt von S6, wo `validateMenuItem:` als zweiter Frager
hinzukommt. Er gehört deshalb entschieden, bevor S6 gebaut wird, und nicht danach.

---
**Die Runde faehrt auf Weg 1, der Datensatz bleibt offen.** Der Orchestrator hat am 260813
entschieden, den Preis zu benennen statt ihn zu beseitigen, und zwar aus dem Grund, den Weg 1
selbst nennt: nach C2.19 gilt dieselbe Regel ohnehin fuer jeden Menueeintrag, also auch fuer die
Vorschau-Umschaltung ueber das Menue. Die Bereichsleiste ist damit die dritte Flaeche mit
derselben Antwort und keine Ausnahme. Eine Ausnahme waere zwei Antworten auf eine Frage, und
genau das beseitigt diese Runde gerade.

**Damit behaelt S6 seinen geplanten Zuschnitt.** Die Bedingung, Weg 3 vor S6 zu entscheiden, ist
erfuellt: Weg 1 verlangt an S6 keine Aenderung. Waehlt der Nutzer spaeter Weg 3, ist er nachziehbar,
kostet dann aber die Lage als Parameter an allen Aufrufern.

Der Verlust steht auf der Abnahmeliste des Laufs am Buendel: ein Klick auf den Vorschau-Schalter
mitten in einer Umbenennung. Er ist bisher abgeleitet und nicht gemessen.
