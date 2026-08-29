Zwei Probenköpfe in `statuszeile.rs` zählen fünf Ränge, und `Rang::ALLE` trägt sechs

---

`crates/krk-ui/src/appkit/statuszeile.rs:1508-1509`: „Der Zusatz haengt an der Seite und nicht am Rang:
er steht auf jedem der fuenf." `:1590-1591`: „sie gilt auf allen fuenf und in beide Richtungen."
Beide Proben laufen über `Rang::ALLE` (`:1512`, `:1594`), und das trägt seit der Runde 10 sechs
Werte (`:235-242`, `pub const ALLE: [Rang; 6]`); `:1059` und `:1311` halten die Sechs
ausdrücklich. Die Proben selbst sind richtig — sie iterieren —, nur ihr Kopf zählt den Stand vor
der Runde 10.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-ui/src/appkit/statuszeile.rs` (Prosa in `mod tests`)

Derselbe Befund, den R3 dieser Sitzung an drei Prosastellen in `tabelle.rs` gemeldet hat: die
Rangfolge hat sechs Ränge, die Prosa um sie herum zählt fünf. Hier liegt er an der Quelle der
Rangfolge selbst. Zum Code-Stand der offenen Entscheidung `260814-1552` (wo die Filterzahl steht):
`Rang::ALLE` führt `Filterstand` an fünfter Stelle zwischen `Tabmeldung` und `Markierungsstand`,
die Probe `der_filterstand_steht_zwischen_tabmeldung_und_markierungsstand` (`:1057-1082`) hält
genau das, und der Doc-Kommentar `:230-234` sagt, dass bei anderem Ausgang „diese Zeile wandert
und sonst nichts". Weg: „fuenf" → „sechs" oder die Zahl streichen.

---
Abgleich 260829-1252, am Baum `b9d9cbc`: **gilt weiter, und die Zahl ist ein zweites Mal gewandert.** Die Runde 20 hat einen siebten Rang gelegt (`crates/krk-ui/src/appkit/statuszeile.rs:275`, `pub const ALLE: [Rang; 7]`; Probe `:1472` „sieben Raenge seit der Runde 20"). Von den zwei zitierten Probenköpfen sagt der eine weiter „jedem der fuenf" (`:1694-1695`, `der_namenszusatz_gilt_auf_jedem_rang`); der andere ist umformuliert und zählt jetzt „sechs" (`:1598`, `:1651`) — gegen die sieben von heute ebenso falsch. Der Weg aus dem Datensatz bleibt: die Zahl streichen.
