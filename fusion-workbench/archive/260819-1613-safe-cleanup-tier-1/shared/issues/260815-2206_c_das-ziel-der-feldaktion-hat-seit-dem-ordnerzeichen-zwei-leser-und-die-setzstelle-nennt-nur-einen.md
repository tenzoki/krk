Das Ziel der Feldaktion hat seit dem Ordnerzeichen zwei Leser, und die Setzstelle nennt nur einen

---

`DateifensterDelegierter::feld` setzt Ziel und Aktion des Namensfeldes
(`tabelle.rs:2808-2811`); der SAFETY-Block darüber begründet allein die Aktion. Seit
`3b128c3` liest `Namensfeld::delegierter` (`:2947-2950`) dasselbe `target` zurück, um nach
Escape die Anzeigeform zu holen. Wer das Ziel später umhängt oder die Aktion an eine andere
Stelle zieht, bricht damit Zusage 3 des Nutzerentscheids — und zwar **still**: `delegierter`
liefert `None`, `bearbeitung_abbrechen` fällt durch sein `if let` und meldet nichts.

---

**Schwere:** niedrig. Heute hält die Kopplung: `Namensfeld` entsteht nur für die
beschreibbare Spalte, und dieselbe Bedingung setzt zwei Zeilen später das Ziel. Der Befund
betrifft die nächste Änderung an dieser Verdrahtung, nicht den heutigen Zustand.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:2795-2812` und `:2941-2950`
**Domain:** code

## Warum das in diesem Baum eine eigene Zeile verdient

Die Datei führt denselben Fall schon einmal vor, im Modulkopf: „`clickedRow` … hat seit dem
260812 **zwei Abnehmer statt einen**, den Doppelklick aus C3 der Runde 4 und die Auswahl vor
dem Rechtsklick aus C1 der Runde 6." Genau diese Notiz fehlt beim Ziel der Feldaktion.

Geprüft ist daneben, dass das Zurücklesen selbst tragfähig ist: `NSControl.target` ist im Kopf
des Systems `@property (nullable, weak) id target` (`NSControl.h:24`, ohne
`API_AVAILABLE`), also eine nullende schwache Referenz für Programme ab 10.10; ein
gestorbener Delegierter liefert `nil` und keinen Absturz. Der `downcast` ist
`isKindOfClass:`-basiert und trifft die eine Klasse, die dort steht.

## Vorschlag

Eine Zeile am SAFETY-Block von `feld`, die den zweiten Leser benennt, und eine Rückverweisung
im Doc-Kommentar von `delegierter`. Wer mehr will, prüft in `bearbeitung_abbrechen` auf
`None` und legt eine Meldung in die Statuszeile statt still weiterzugehen; das entspräche der
Projektregel, dass ein stilles Fallenlassen nicht vorkommt.

---
Resolved: Beide Notizen stehen (260816, coder), in
`crates/krk-ui/src/appkit/tabelle.rs`.

An der Setzstelle in `DateifensterDelegierter::feld` steht ein eigener Absatz vor dem
SAFETY-Block, in derselben Form wie die Notiz zu `clickedRow` im Modulkopf: das Ziel hat seit
dem Ordnerzeichen zwei Abnehmer statt einen, `Namensfeld::delegierter` liest dasselbe
`target` zurück, weil es der einzige Weg von der Zelle zu ihrem Delegierten ist, und drei
Überschreibungen gehen darüber — `becomeFirstResponder`, `textDidEndEditing:` und
`abortEditing`. Der Absatz sagt auch, was der Bruch kostet: `delegierter()` liefert dann
`None`, und alle drei Methoden fallen durch ihr `if let`, ohne dass etwas meldet. Der
SAFETY-Block selbst trägt jetzt die geprüfte Zusage des Zurücklesens mit ihrer
Kopfzeilen-Fundstelle: `NSControl.target` ist `@property (nullable, weak) id`
(`NSControl.h:24`, ohne `API_AVAILABLE`), also nullend, und ein gestorbener Delegierter
liefert `nil` statt eines Absturzes.

**Der Befund nannte zwei Leser, es sind drei** — `bearbeitung_beendet` ist seit dem
Nutzerentscheid vom 260816-0935 dazugekommen. Die Notiz nennt deshalb die Methoden und nicht
eine Zahl.

Der Doc-Kommentar von `Namensfeld::delegierter` trägt die Rückverweisung auf die Setzstelle.

**Die vorgeschlagene Meldung in der Statuszeile ist nicht gebaut, und der Grund steht am
Code:** eine Meldung erreicht die Statuszeile allein über die Quelle, an die diese Zelle nur
über genau diesen Delegierten kommt. Wer `None` melden wollte, bräuchte das, was `None`
gerade sagt, dass es fehlt. Der Fall wird deshalb von der Notiz an der Setzstelle gehalten
und nicht von einer Meldung; der Doc-Kommentar von `delegierter` schreibt das aus, damit die
Ausnahme von der Projektregel gegen stilles Fallenlassen dort begründet dasteht und nicht
stillschweigend.

Verification: `make check` — exit 0.
