Ein Kommentar im Einzugstakt nennt zwei Kanäle; der Takt bedient drei

---
`DateifensterQuelle::einziehen` (`crates/krk-ui/src/appkit/tabelle.rs:3677-3680`, am Ende des Rumpfs) begründet die Abbruchbedingung des Zeitgebers:

> Gefragt wird `arbeitet_noch` und nicht `liest_noch`: der Takt bedient **zwei** Kanaele, und ein Durchlauf laeuft gerade dann, wenn kein Lesevorgang mehr laeuft.

Seit der Runde 23 sind es drei. `Tabliste::arbeitet_noch` (`crates/krk-ui/src/tabs.rs:968-974`) fragt Lesevorgang, Durchlauf **und** Gitlauf ab, und ihr eigener Doc-Kommentar schreibt aus, dass der dritte Kanal der längste ist. `einzug_je_tab` (`tabs.rs:1342-1348`) räumt ebenfalls drei Kanäle.

Der Kommentar liegt in `appkit/tabelle.rs`, das die Dateiliste von Schritt 12 der Runde 23 angefasst hat; die Erhebung jenes Schritts ging über Spalten und Schalter und traf ihn nicht.

**Abnahmetest:** der Satz nennt drei Kanäle oder keine Zahl.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23.
