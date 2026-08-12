Eine Probe heißt noch nach der abgeschafften Zweiteilung in feste und bewegliche Bereiche

---

Schritt 1 hat die Zweiteilung in "feste" und "bewegliche" Bereiche beseitigt und `ist_beweglich`
entfernt; der Modulkopf sagt ausdrücklich "Die Zweiteilung ist weg". Eine Probe trägt den Begriff
weiter im Namen und in ihrem Dokumentationskommentar:
`ein_fester_bereich_aendert_nur_seine_eigene_breite` (`fenstermodell.rs:1591`), mit der Zeile "Ein
fester Bereich wächst unmittelbar und zieht kein Dateifenster mit."

---

**Schwere:** niedrig (die Zusicherung der Probe gilt weiter; allein ihr Name benennt einen Begriff,
den das Programm nicht mehr kennt)
**Gefunden:** coderev, Durchsicht der Commits `5e17c9e`, `a2ea876`, `8ffaac2`
**Betroffen:** `crates/krk-ui/src/fenstermodell.rs:1587` bis `:1615`
**Domain:** code

## Zusammenhang

Das Protokoll zu Schritt 1
(`history/260812-0439-coder-schritt-1-proportionale-breitenregel.md`) zieht fünf Probennamen mit
und begründet es so: "ein Probenname, der das Gegenteil seiner Zusicherung sagt, hält dieselbe Falle
bereit wie ein Kommentar". Diese sechste ist dabei übersehen worden. Gemessen wird weiterhin das
Richtige — dass ein Bereich ohne Fensterseite nur seine eigene gespeicherte Breite ändert, während
ein Dateifenster die Trennlinie verschiebt —, und die Unterscheidung dafür ist heute
`Bereich::seite` und nicht mehr `ist_beweglich`.

Ein Name in dieser Richtung wäre `ein_bereich_ohne_fensterseite_aendert_nur_seine_eigene_breite`.
Der Kommentar darunter nennt dann `Bereich::seite` als die Unterscheidung, die es noch gibt.
