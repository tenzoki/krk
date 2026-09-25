Der Turn-Log des aktiven Circles sagt „noch kein Turn gefahren", und Turn 1 ist gefahren

---

Der Abschnitt `## Turn log` von
`circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/_t_circle.md:58-60`
trägt am 260817-1129 unverändert den Platzhalter `(noch kein Turn gefahren)`. Turn 1 ist
gefahren: drei Schritte, drei Commits (`664a0fd`, `375d07c`, `472eb81`), eine Durchsicht mit
sieben Befunden (`a8b4bf8`). Dem Datensatz fehlt daneben der Eintrag über seine eigene
Aktivierung.

---

**Warum es zählt.** Der Circle-Datensatz ist die Stelle, an der eine spätere Sitzung den
Stand einer Runde abliest, ohne das Sitzungsprotokoll zu öffnen. Steht dort „noch kein Turn
gefahren", liest sie eine Runde, die aufgesetzt und nicht begonnen ist, und plant von vorn.
Der Widerspruch ist nicht abstrakt: derselbe Datensatz nennt im Kopf
`**Active session history:** shared/history/260816-2113-orchestrator-session.md`, und dieses
Protokoll führt unter `## Per-Turn-Log` den gefahrenen Turn 1 aus.

**Der Vergleich mit den gefahrenen Runden zeigt die Form, die fehlt.** Die elfte Runde
(`circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/_b_circle.md:76-79`) trägt an
derselben Stelle die Aktivierung mit Datum, Sitzung und Anker und darunter ihre Turns. Beide
Einträge fehlen hier.

**Gehört dem Orchestrator.** Der Turn-Log des Circle-Datensatzes wird beim Turn-Ende
geschrieben, und Circle-Datensätze sind für den Abgleich nicht schreibbar. Der Befund ist
deshalb abgelegt statt behoben.

**Gefunden von:** reconciler, Abgleich 260817-1129
**Betrifft:** `fusion-workbench/circles/260817-0833-jeder-loeschweg-mit-rueckfrage-und-nur-noch-papierkorb/_t_circle.md`
**Domain:** code

---
Resolved: Der Orchestrator hat den Eintrag zu Turn 1 am 260817-1135 in den Abschnitt
`## Turn log` des Datensatzes geschrieben, in derselben Sitzung, in der der Befund entstand.
Der Eintrag nennt die Commits `664a0fd`..`472eb81` und `a8b4bf8`, das vollständige Bündel A,
die erreichte Schutzschwelle, den Kohärenz-Befund `ok`, die sieben Befunde der Durchsicht
und den mitbehobenen Defekt.

Der Befund ist richtig gestellt: der Abgleich darf Circle-Datensätze nicht schreiben, also
war das Nachziehen Sache des Orchestrators, und dieser Datensatz ist der Weg, auf dem es
ihn erreicht hat.
