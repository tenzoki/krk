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
