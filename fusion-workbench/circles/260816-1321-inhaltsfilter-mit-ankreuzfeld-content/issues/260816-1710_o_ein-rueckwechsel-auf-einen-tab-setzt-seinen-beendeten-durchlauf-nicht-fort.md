Ein Rückwechsel auf einen Tab setzt seinen beim Wegwechseln beendeten Durchlauf nicht fort
---
Seit Schritt D1 beendet ein Tabwechsel den Durchlauf des verlassenen Tabs (C4.5, Nutzerentscheid
vom 260816-1410, Möglichkeit 1). Der Rückwechsel stößt ihn nicht wieder an: `Tabliste::waehlen`
(`crates/krk-ui/src/tabs.rs`) ruft `durchlauf_nachziehen_an` allein auf der **verlassenen** Stelle.
Wer auf den Tab zurückwechselt, sieht die Befunde, die bis zum Wegwechseln eingetroffen waren, und
der Lauf setzt nicht fort. Erst die nächste Änderung des Filtertexts oder eines der beiden Schalter
stößt ihn neu an. Am Schirm ist das eine Liste, die für einen vollständigen Filterstand gehalten
wird und keiner ist; nichts sagt dem Nutzer, dass sie stehengeblieben ist.
---
Gefunden bei der Umsetzung von D1 des Plans
`planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`.

Die Beschreibung der gewählten Möglichkeit im Datensatz
`decisions/260816-1359_a_beendet-ein-tabwechsel-den-durchlauf-des-verlassenen-tabs-jetzt-wo-er-dateien-liest.md`
sagt: „Der Rückwechsel stößt ihn über denselben Weg wieder an, den ein Ordnerwechsel benutzt." Der
Plan nennt für `waehlen` an derselben Stelle genau **einen** Ruf, den auf der verlassenen Stelle,
und die Datei des Schritts ist allein `tabs.rs`. Beides zusammen geht nicht auf; D1 ist dem Plan
gefolgt, und der Befund steht hier statt in einer Improvisation.

**Warum ein zweiter Ruf nicht in `tabs.rs` allein unterzubringen ist.** Ein Anstoß für die
betretene Stelle liefe über dieselbe Methode und startete den Lauf; der Einzugstakt, der die
Befunde abholt, wird aber von `crate::appkit::tabelle` angeworfen, und `tab_gewechselt` fragt dafür
heute `Tabliste::liest_noch()` und nicht `arbeitet_noch()`. Ohne die zweite Änderung liefe der
Arbeitsfaden, und kein Befund käme je an — die Zeilen erschienen nie. Der Rückgabewert von
`durchlauf_nachziehen_an` trägt `#[must_use]` genau für diese Auskunft, und `waehlen` gibt ihn
heute nicht heraus: es liefert, ob der sichtbare Tab ein anderer geworden ist.

**Drei Wege stehen offen.** Erstens: `waehlen` ruft für beide Stellen, `tab_gewechselt` fragt
`arbeitet_noch()` statt `liest_noch()`. Zweitens: `tab_gewechselt` ruft nach dem Wechsel
`Dateifenster::durchlauf_nachziehen`, das den Einzugstakt schon selbst anwirft (`tabelle.rs:2153`)
— dann bleibt `tabs.rs` unberührt und die Stelle steht dort, wo der Takt ohnehin entschieden wird.
Drittens: es bleibt, wie es ist, und C4.5 wird um die Zusage des Rückwechsels ergänzt oder der
Datensatz um den Satz gekürzt. Welcher Weg gilt, ist keine Frage dieses Schritts.
