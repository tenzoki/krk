Die Zusage „der Schrägstrich entsteht an genau einer Stelle" ist von keiner Zählprobe gehalten

---

Der Nutzerentscheid vom 260815-2058 sagt: „Er entsteht in `DateifensterDelegierter::beschriften`
für `Spalte::Name` und **nirgends sonst**. Sortierung, Filter, Zwischenablage, Vorschau und
jede Dateioperation lesen weiterhin `eintrag.name`." Das ist eine Aussage über den **Baum**,
und an keinem Rückgabewert abzulesen. Dieses Projekt hält solche Aussagen mit Zählproben über
`quellbaum` fest — für die zwei Filterregeln, für die Ersthelferfrage, für den einen
Menübauer. Für den Schrägstrich hält sie nichts.

---

**Schwere:** mittel. Heute stimmt die Zusage: `namensform` hat genau einen Rufer
(`tabelle.rs:2718`), `ohne_ordnerzeichen` genau einen (`:2882`), und der einzige Leser eines
Namenszellentextes ist `umbenennung_beenden`. Ein zweiter Rufer, den ein späterer Turn
anlegt, fällt aber durch jede Prüfung: der Bau bleibt grün, die beiden neuen Proben bleiben
grün, und die teuerste Zusage dieser Änderung fällt still.
**Gefunden von:** coderev, Durchsicht von `3b128c3`
**Betroffen:** `crates/krk-ui/src/appkit/tabelle.rs:325-362`, Prüfmodul ab `:3268`
**Domain:** code

## Das Werkzeug steht schon da

`crates/krk-ui/src/quellbaum.rs` liest jede `.rs`-Datei unter `crates/` (`quelldateien`,
`:95`) und zählt Aufrufstellen unter Ausschluss von Erklärung und Prosa (`aufrufstellen`,
`:133`). Die Bauanleitung für eine neue Zählprobe steht im Modulkopf derselben Datei, samt
der Pflicht, die verbleibende Blindheit im Doc-Kommentar der Probe zu benennen.

Der Modulkopf warnt dabei ausdrücklich vor der falschen Sorte: eine **Aufruferzählung** steht
nur dort, wo ein Abnahmekriterium die Zahl selbst zusagt. Hier tut es das — der Entscheid
sagt „an einer Stelle und nirgends sonst" —, und damit ist die Aufruferzählung die richtige
und nicht die verbotene Form.

## Vorschlag

Eine Probe neben den beiden neuen, die über `quellbaum::quelldateien` zählt:
`aufrufstellen(inhalt, "namensform")` ergibt über alle Kisten genau einen Treffer außerhalb
des Prüfmoduls, ebenso `ohne_ordnerzeichen`. Die Nadel ist wie üblich mit `concat!`
zusammenzusetzen, sonst zählt die Probe sich selbst mit. Ihr Doc-Kommentar benennt die
Blindheit, die bleibt: eine zweite Fassung derselben Regel unter anderem Namen sieht auch
diese Zählung nicht.
