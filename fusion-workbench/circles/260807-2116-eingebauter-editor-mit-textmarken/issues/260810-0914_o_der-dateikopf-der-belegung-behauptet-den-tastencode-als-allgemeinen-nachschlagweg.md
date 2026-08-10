# Der Dateikopf der Belegung behauptet den Tastencode als allgemeinen Nachschlagweg

---
**Domain:** data
**Schwere:** Low
**Gefunden von:** ontocoder, beim Beheben von `260810-0011_*_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md`
**Betroffen:** `resources/default-keymap.toml`, Zeile 42
**Cross-references:** `crates/krk-core/src/tasten/parser.rs` (Modulkopf, `Taste::kennung`), `issues/260810-0011_*_zwei-kommentarbloecke-der-belegungsdatei-behaupten-den-nachschlag-ueber-den-tastencode.md` (dieselbe weggefallene Begründung, zwei andere Blöcke derselben Datei), `decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`

---

## Der Befund

Der Kopf von `resources/default-keymap.toml` begründet in seinem Absatz über die
fn-Taste (Zeilen 41–47), warum sie in keiner Kombination vorkommt, und stützt
das auf einen Satz über den Nachschlag:

Zeile 42: "KRK belegt den Tastencode, und F3 mit gehaltener fn-Taste erzeugt
denselben Tastencode wie ein nacktes F3."

Der erste Halbsatz ist seit S2 (`00719cb`) als allgemeine Aussage falsch.
`Taste::kennung` (`crates/krk-core/src/tasten/parser.rs:192-198`) legt jeden
einbuchstabigen Namen auf `Tastenkennung::Zeichen`; über den Tastencode gehen
nur noch Funktionstasten, Pfeilblock und Steuertasten.

**Die Schlussfolgerung des Absatzes bleibt richtig.** F3 ist eine
Funktionstaste, wird weiter über den Code nachgeschlagen, und die Messung aus
`spikes/fn-tasten/messung-A.txt` trägt weiter. Falsch ist allein die Reichweite
der Prämisse.

## Warum das zählt

Der Kopf ist die Stelle, an der ein Leser der Datei die Regel sucht, bevor er
sich eine Kombination legt. Zwei Blöcke weiter unten haben genau diese Prämisse
zu einer Meidung von `y` und `z` geführt, die es nie gebraucht hätte
(`260810-0011`). Der Satz im Kopf ist derselbe Fehler an der Stelle mit der
größten Reichweite: er lädt dazu ein, den Stellen-Nachschlag als das allgemeine
Verhalten zu lesen, und die beiden nachgezogenen Blöcke widersprechen ihm nun
sichtbar.

Der Schaden ist geringer als bei `260810-0011`, weil hier keine Regel für
eigene Belegungen daran hängt. Deshalb `Low` und nicht `Medium`.

## Was zu tun ist

Eine Wortänderung, kein Umbau. Den Halbsatz auf die Tastensorte einschränken,
über die der Absatz tatsächlich spricht — etwa: "Funktionstasten schlägt KRK
über den Tastencode nach, und F3 mit gehaltener fn-Taste erzeugt denselben
Tastencode wie ein nacktes F3." Alles Weitere des Absatzes bleibt unverändert,
samt Messbeleg und der Einschränkung über die Touch Bar.

Die allgemeine Regel ist im selben Bau schon einmal ausgeschrieben, im Block zum
eingebauten Editor (Zeilen 484–499) mit Verweis auf den Modulkopf von
`parser.rs`; der Kopf braucht sie nicht zu wiederholen und soll sie nicht
umformulieren.

Ausführender ist `ontocoder`, weil `.toml` nicht dem `coder` gehört. Nicht in
`260810-0011` mit erledigt, weil dessen Schreibgrenze ausdrücklich auf die
beiden dort genannten Blöcke lautete.
