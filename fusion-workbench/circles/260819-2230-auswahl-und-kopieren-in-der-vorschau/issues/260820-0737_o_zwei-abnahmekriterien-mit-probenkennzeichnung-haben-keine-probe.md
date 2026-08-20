Zwei Abnahmekriterien mit Probenkennzeichnung haben keine Probe

---

Der Spec kennzeichnet C2.3 und C2.4 je mit **(Probe)** und nennt sogar, was gezählt werden
soll:

> C2.3 … **(Probe** über das Fehlen eines zweiten Lesevorgangs**)**
> C2.4 … **(Probe** über die Zahl der Durchgänge**)**

Der Plan führt beide unter den Kriterien, die diese Runde ohne Messstrecke prüfbar macht,
und begründet damit ausdrücklich, dass kein Abnahmelauf gegen L7 geschuldet ist
(`planning/260819-2245_o_plan-…`, Abschnitt `## Nutzerarbeit`, letzter Absatz; ebenso
`shared/decisions/260819-2216_a_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`).

Im Baum steht keine der beiden Proben. Die Tabelle „Eine Probe hält es (6 Stellen)" im Plan
führt sie auch nicht auf; gezählt werden dort die Kachelung, die Klammerregel, die
Anmeldungen, die Hüllen, die Abfangstellen und die Menübauer.

---

**Die Sache selbst stimmt, gemessen am Baum:** `Parser::new_ext(…).into_offset_iter()`
steht in `crates/krk-ui/src/markdown.rs:593` genau einmal, und ein zweiter Lesevorgang
entsteht nicht, weil `Quellbezug::quelle` aus `self.quelle.to_owned()` in
`Zerlegung::abschliessen` (`markdown.rs:1546`) kommt und damit aus der Eingabe des
Durchgangs. Der Befund richtet sich nicht gegen die Umsetzung, sondern gegen die Lücke
zwischen einer Zusage mit Probenkennzeichnung und dem, was der Baum nachmisst.

**Warum das hier zählt.** Die Zusage trägt Gewicht: sie ist der Ersatz für einen
Abnahmelauf, den der Nutzer nicht fahren muss. Eine Zusage, die kein Kommando prüft, hält
nur so lange, wie niemand einen zweiten Durchgang danebenstellt — und genau davor sollen die
Zählproben dieses Projekts schützen.

**Richtung:** zwei Zählproben über `crate::quellbaum` nach dem Vorbild der übrigen dieser
Runde, mit zusammengesetzten Nadeln — `into_offset_iter` genau einmal im Baum, und
`Quellbezug` bekommt seine Quelle aus keiner Lesefunktion des Dateisystems. Die Lage ist
vor der Erwartung am Baum zu erheben; der Befund
`260820-0646_o_der-plan-schreibt-zaehlerwartungen-ohne-sie-gegen-den-baum-zu-halten…`
in diesem Speicher sagt, warum.

**Schwere:** niedrig im heutigen Schaden, mittel in der Bedeutung, weil an diesen zwei
Kriterien der Verzicht auf einen Abnahmelauf hängt.
**Baumstand:** `b28cdd6`.

---
Abgleich 260820-0834, gegen `05cb614`: **trifft unveraendert zu, mit zwei verschobenen
Zeilennummern.** Die Wurzelbehebung hat `markdown.rs` um 238 Zeilen veraendert, und die beiden
Belege dieses Befundes sind mitgewandert: `Parser::new_ext(…).into_offset_iter()` steht jetzt in
`crates/krk-ui/src/markdown.rs:582` (im Befund: `:593`), und `quelle: self.quelle.to_owned()` in
`Zerlegung::abschliessen` jetzt in `:1594` (im Befund: `:1546`). Die Sache selbst ist unveraendert:
genau ein Durchgang, kein zweiter Lesevorgang — und weiterhin keine Probe, die es nachmisst. Der
Text oben bleibt unangetastet; er ist die Aufzeichnung des Standes `b28cdd6`.

Der Befund haelt den Datensatz
`shared/decisions/260819-2216_a_schuldet-diese-runde-einen-abnahmelauf-gegen-die-zusage-l7.md`
auf `_a_`: an C2.3 und C2.4 haengt der Verzicht auf einen Abnahmelauf, und eine der beiden
Ersatzzusagen hat keinen Pruefer.
