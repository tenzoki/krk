CLAUDE.md nennt für `Kommando` 68 Varianten, der Baum trägt 75

---

Der Abschnitt „Projektstand" in `CLAUDE.md` sagt: „Am 260811 nachgezählt:
`Wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) trägt sieben Werte,
`Kommando` in derselben Datei **68 Varianten**, `Bereich`
(`krk-ui/src/fenstermodell.rs`) fünf und `Fokus` (`krk-ui/src/kommandos/fokus.rs`)
fünf."

Am 260812-2253 nachgezählt trägt `Kommando` **75** Varianten. Die drei anderen
Zahlen stimmen unverändert.

---

**Gemessen** über die Varianten der Aufzählung in
`crates/krk-core/src/tasten/belegung.rs`. Die Runde 6 hat zwei hinzugefügt,
`Kommando::OrdnerDerDatei` und `Kommando::Teilen`; der Plan der Runde nennt den
Sprung als Kriterium C6.1 und beziffert ihn mit „73 auf 75". Die Differenz von
68 auf 73 stammt aus der Runde 5, deren Bereichsleiste fünf Kennungen gebracht
hat; nachgezogen worden ist die Zahl in `CLAUDE.md` dabei nicht.

**Warum die Zahl überhaupt dasteht.** Der Absatz erklärt eine Eigenschaft, die
schon Sitzungen gekostet hat: die vier Aufzählungen haben keinen Auffangzweig,
und wer eine erweitert, hält den Bau an. Der Absatz sagt das selbst und
verweist für die Stellen auf den Übersetzer statt auf eine Liste. Die Zahlen
daneben sind Beiwerk und veralten schneller als der Satz, den sie belegen — die
Datei führt in demselben Abschnitt aus, dass sie Aufstellungen aus genau diesem
Grund schon abgelegt hat.

**Es ist der zweite Zähldefekt derselben Bauart in `CLAUDE.md`**; der erste ist
`shared/issues/260812-1438_*_claude-md-nennt-31-von-33-dateien-mit-untergrenzen-abschnitt-es-sind-33-von-35.md`,
und seine eigene Gegenzahl ist inzwischen ebenfalls veraltet. Beide legen
dieselbe Wahl vor: die Zahl bei jeder Runde nachziehen, oder sie durch etwas
ersetzen, das nicht mitwächst.

**Gewicht:** niedrig. Keine Auswirkung auf den Code; eine falsche Zahl in der
Datei, die jeder Agent zuerst liest.

**Herkunft:** Abgleich der Runde 6 am 260812-2253.
