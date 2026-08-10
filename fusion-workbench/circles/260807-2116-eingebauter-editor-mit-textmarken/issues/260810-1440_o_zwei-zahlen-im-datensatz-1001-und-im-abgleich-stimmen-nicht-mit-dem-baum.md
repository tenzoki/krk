# Zwei Zahlen im Datensatz `260810-1001` und im Abschluss-Abgleich stimmen nicht mit dem Baum

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coder, bei der Nachziehung von `CLAUDE.md` am 260810-1417
**Betroffen:** `issues/260810-1001_o_die-neuen-proben-behaupten-den-hauptfaden-den-libtest-ihnen-nicht-gibt.md`, `history/260810-1404-reconciliation.md` (Punkt 5)
**Cross-references:** `decisions/260810-1044_o_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`, `issues/260810-1404_o_vierzehn-geschlossene-datensaetze-zeigen-auf-zeilen-die-ihre-eigene-sitzung-verschoben-hat.md`

---

## Der Befund

Beide Stellen sprechen von den Proben, die über `MainThreadMarker::new_unchecked`
den Hauptfaden behaupten, und beide zählen falsch. Am Baum vom 260810-1440
nachgesehen, in `crates/krk-ui/src/appkit/editor.rs`:

| Aussage | Behauptet | Gezählt |
|---|---|---|
| Stellen mit `MainThreadMarker::new_unchecked` | eine | **zwei**: `verwalter_ohne_fenster` und `an_einer_flaeche` |
| Aufrufer von `an_einer_flaeche` | vier | **sechs** |

Beide Stellen liegen im `#[cfg(test)] mod tests`.

## Warum das zählt

Der Datensatz `260810-1001` ist die Grundlage der offenen Nutzerentscheidung
`260810-1044`, und die Entscheidung wägt den Preis eines Umbaus gegen den Umfang
des Betroffenen ab. Zwei Stellen statt einer und sechs Aufrufer statt vier
verschieben diesen Umfang. Wer die Entscheidung trifft, soll die richtige Zahl
vor sich haben.

Die Zahl ist nicht durch die Sitzung gewandert: `an_einer_flaeche` hat im Lauf
Aufrufer bekommen, die zweite `new_unchecked`-Stelle stand schon bei der Anlage
des Datensatzes da. Damit ist das hier kein Fall von
`260810-1404` (Verweise, die ein späterer Commit verschoben hat), sondern eine
Zählung, die von Anfang an nicht stimmte.

## Behebung

Die zwei Zahlen in `260810-1001` und in Punkt 5 des Abgleichsberichts durch die
gezählten ersetzen, und in beiden Fällen das Stück benennen statt der Zahl zu
vertrauen — dieselbe Form, die `260810-1404` für die abgewanderten Verweise
vorschlägt. Der Entscheidungsdatensatz `260810-1044` zitiert `260810-1001` und
zieht mit.
