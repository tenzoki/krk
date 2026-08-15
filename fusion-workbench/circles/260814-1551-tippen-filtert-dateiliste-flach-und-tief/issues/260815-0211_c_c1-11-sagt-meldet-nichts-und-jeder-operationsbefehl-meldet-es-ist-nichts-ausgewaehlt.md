# C1.11 sagt „meldet nichts", und jeder Operationsbefehl meldet „es ist nichts ausgewählt"

---
**Domain:** code
**Status:** open
**Filed by:** coderev
**Cross-references:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.11 (zweiter Satz); `crates/krk-ui/src/appkit/anwendung.rs:4914-4925` (`auftrag_stellen`), `:4505-4509` (`endgueltig_loeschen`); `crates/krk-ui/src/kommandos/operationen.rs:168-193` (`betroffene`)

---

## Befund

C1.11 zweiter Satz: „Ist keine Zeile sichtbar, gibt es keine Auswahl, und ein Befehl, der eine bräuchte, **tut nichts und meldet nichts**." Das Kriterium trägt **(Probe)**.

Die erste Hälfte hält: `betroffene` läuft allein über die Sichtreihenfolge, und bei null sichtbaren Zeilen kommt eine leere Auswahl heraus. Die zweite Hälfte hält nicht. `Anwendungsdelegierter::auftrag_stellen` antwortet auf eine leere Auswahl mit einer Befehlsantwort:

```rust
let auswahl = quelle.betroffene_eintraege();
if auswahl.ist_leer() {
    self.antwort_zeigen(aktiv, "es ist nichts ausgewählt");
    return true;
}
```

Damit meldet jeder der vier Operationsbefehle, sobald ein Filtertext die Liste leerfegt: F5, F6, `delete` ohne stehenden Filtertext und `cmd+delete` gehen alle über `auftrag_stellen`. `endgueltig_loeschen` führt dieselbe Meldung noch einmal in einem eigenen Zweig.

**Es ist kein Zweig dieser Runde**, sondern das Verhalten seit der Runde 1; die Runde 10 macht den bisher seltenen Fall zum Regelfall, und C1.11 sagt für ihn etwas anderes zu, als der Baum tut.

## Wie es aufgelöst gehört

Zwei Richtungen, und die zweite ist wahrscheinlich die richtige:

1. **Den Baum an das Kriterium ziehen** — bei leerer Sicht schweigen. Das nähme dem Nutzer die einzige Auskunft darüber, warum sein Tastendruck nichts getan hat, und zwar ausgerechnet in der Lage, in der die Liste leer vor ihm steht.
2. **Das Kriterium an den Baum ziehen** — „tut nichts und meldet, dass nichts ausgewählt ist". Dann ist die Zusage dieselbe wie seit der Runde 1, und C1.11 sagt es nur genauer.

Für C1.11 gibt es keine Probe im Baum; welche der beiden Fassungen gilt, entscheidet, wie sie zu schreiben wäre.

---
Resolved: 260815-0246, shaper. Richtung 2 des Datensatzes gewählt: das Kriterium ist an den Baum gezogen worden. C1.11 sagt jetzt, dass ein Befehl ohne Auswahl keinen Auftrag stellt und dabei nicht schweigt, sondern seit der Runde 1 "es ist nichts ausgewählt" meldet. Begründung: die Meldung ist in der Lage, in der die Liste leer vor dem Nutzer steht, seine einzige Auskunft darüber, warum sein Tastendruck nichts getan hat; sie zu entfernen hieße, den einen Zweig in auftrag_stellen zu ändern, durch den alle vier Operationsbefehle laufen, also sichtbares Verhalten außerhalb dieses Specs. Die Kennzeichnung des Kriteriums nennt jetzt ausdrücklich, dass die Meldung keine Probe dieser Runde trägt.
