Kommentare der Statuszeile zählen die Ränge noch ohne das Warten auf die Platte
---
Schwere: Niedrig. Nur Kommentare und Abschnittsüberschriften in Proben; am Verhalten ändert das nichts.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `0e9347c0` setzt `Rang::Plattenwarte` an die dritte Stelle (`crates/krk-ui/src/appkit/statuszeile.rs:325` bis `:328`). Damit ist der Filterstand der sechste Rang, und über ihm stehen fünf. Stehen geblieben sind:

- `statuszeile.rs:432`: „Was der fuenfte Rang aus dem sichtbaren Tab braucht.“
- `statuszeile.rs:486`: „Der fuenfte Rang der Statuszeile“.
- `statuszeile.rs:498`: „Er kann von den vier Raengen darueber verdraengt werden“.
- `statuszeile.rs:1436`: „Der fuenfte Rang: der Filterstand“.
- `crates/krk-ui/src/appkit/anwendung.rs:8372`: `statuszeile_nachziehen` „holt die fuenf Eingaben“; `statuszeile::zeile` nimmt seit demselben Commit sechs.
- `anwendung.rs:8356`: „Der eine Schreiber, mit zwei Anlaessen“; der Melder des Wartens kommt dazu (`:2343`). Schon der Seitenmelder (`:2335`) war ein weiterer, die Stelle war also vorher ungenau.

`statuszeile.rs:2124` nennt den Seitenzähler den siebten Rang. Das stimmt erst seit diesem Commit; bis dahin war er der sechste.

**Richtung.** Die Ordnungszahlen und Zählungen durch den Namen des Rangs ersetzen, wie es der Projektbrauch bei Zahlen vorschreibt, die mit der nächsten Änderung falsch werden. „Von den Rängen darüber“ reicht an `:498`.

**Abnahme.** `grep -n -i 'fuenfte\|vier raengen\|fuenf Eingaben\|zwei Anlaessen' crates/krk-ui/src/appkit/statuszeile.rs crates/krk-ui/src/appkit/anwendung.rs` findet keine dieser Stellen mehr.
