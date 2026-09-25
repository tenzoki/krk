Das Verbot von `setEnabled` trifft jede `NSControl` und nicht nur Menüeinträge

---

`die_freigabe_eines_eintrags_wird_nirgends_gesetzt`
(`crates/krk-ui/src/appkit/menue.rs:1102-1150`) hält C2.17: die Ausgrauung eines
**Menüeintrags** wird an einer Stelle entschieden, und niemand setzt sie an einer zweiten.
Geprüft wird das mit zwei Verboten über den ganzen Quellbaum von `krk-ui`:

```rust
for nadel in [concat!("setEnabled", "("), concat!("setAutoenablesItems", "(")] { … }
```

**`setEnabled:` gehört `NSControl` und nicht `NSMenuItem` allein.** Jede Schaltfläche, jedes
Textfeld und jedes Kontrollkästchen dieses Baums trägt die Methode. Heute ruft sie niemand,
also ist die Probe grün; die nächste Schaltfläche, die während einer laufenden Operation grau
werden soll — die Belegungsansicht hat drei, die Blätter aus C4 mehrere —, macht sie rot, und
zwar aus einem Grund, der mit C2.17 nichts zu tun hat. Der billigste Weg zurück ins Grüne wäre
dann, das Verbot zu streichen, und damit fiele auch der Teil weg, der trägt.

`setAutoenablesItems(` ist dagegen richtig geschnitten: die Methode gehört `NSMenu` und
niemandem sonst.

---

**Schwere:** gering. Kein Fehlverhalten; eine Wache, deren nächster Fehlalarm die Wache selbst
kostet.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/menue.rs:1118-1135`

**Domain:** code

## Vorschlag

Das Verbot auf den Empfänger einschränken, den C2.17 meint: statt `setEnabled(` die Nadel an
einen Menüeintrag binden — `eintrag.setEnabled(`, `posten.setEnabled(` — oder die Probe auf
die Datei `menue.rs` und den `define_class!`-Block des Delegierten begrenzen. Wo das zu
brüchig ist: den Doc-Kommentar um den Satz ergänzen, dass ein Treffer außerhalb der
Menüeinträge kein Verstoß ist und die Nadel dann nachzuziehen ist.

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813. Das Verbot von `setEnabled(` gilt jetzt nur noch in Dateien, die `NSMenuItem` ueberhaupt nennen — nur dort ist ein Menueeintrag zur Hand, an dem der Aufruf etwas anrichten koennte. Eine Schaltflaeche, die waehrend einer Operation grau werden soll, macht die Wache damit nicht mehr aus einem sachfremden Grund rot. `setAutoenablesItems(` bleibt im ganzen Baum verboten, weil die Methode `NSMenu` gehoert und niemandem sonst; der Doc-Kommentar sagt jetzt, warum die zwei Verbote verschieden weit geschnitten sind. Was der Schnitt nicht faengt — eine Datei, die einen Menueeintrag entgegennimmt, ohne den Typ zu nennen —, steht ebenfalls dort.
