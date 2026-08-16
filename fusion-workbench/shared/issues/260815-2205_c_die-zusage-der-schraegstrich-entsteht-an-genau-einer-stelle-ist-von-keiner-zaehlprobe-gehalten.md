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

---
Resolved: Zwei Zählproben stehen im Prüfmodul von `crates/krk-ui/src/appkit/tabelle.rs`
(260816, coder), beide über `crate::quellbaum` und in der Form der Filterprobe
`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`
(`krk-core/tests/verzeichnis.rs`).

`das_ordnerzeichen_entsteht_an_genau_einer_stelle` zählt dreimal. Erstens führt keine andere
Datei des Baums die drei Namen `namensform`, `ohne_ordnerzeichen` und `ORDNERZEICHEN` in
einer Code-Zeile — sie sind privat, und ein gleichnamiger Doppelbau anderswo wird rot.
Zweitens hat jede der zwei Regeln über `aufrufstellen` genau einen Rufer im Code dieser
Datei vor ihrem Prüfmodul: `DateifensterDelegierter::beschriften` beziehungsweise
`Namensfeld::wird_ersthelfer`. Drittens steht das Zeichen selbst in genau drei Code-Zeilen,
seiner Erklärung und den zwei Regeln, die es tragen.

`die_anzeigeform_hat_genau_zwei_leser` hält die andere Hälfte: ein Namenszellentext wird über
`stringValue` an genau zwei Stellen gelesen, `umbenennung_beenden` nach Return und
`wird_ersthelfer` vor dem Ablegen des Zeichens. Ein dritter Leser wäre eine Stelle, an der
der Schrägstrich als Name durchginge.

**Die Probe hat beim ersten Lauf sofort einen Treffer geholt, und behoben ist die Wurzel und
nicht die Zahl:** `krk-bench/src/bericht.rs` nennt eine Probe
`der_kurzstempel_passt_zur_namensform_des_projekts`. Das ist kein zweiter Bau der
Anzeigeform, sondern dasselbe deutsche Wort mitten in einem längeren Bezeichner — die Nadel
war zu grob. Sie zieht jetzt über `fuehrt_den_namen` dieselbe Bezeichnergrenze, die
`quellbaum::aufrufstellen` für ihre Seite schon zieht: gehört eines der beiden Nachbarzeichen
zu einem Bezeichner, ist es keine Fundstelle.

Die Blindheit steht wie verlangt an den Doc-Kommentaren: eine zweite Fassung derselben Regel
unter anderem Namen sieht auch diese Zählung nicht, ein `'/'` als Zeichenkonstante neben
`ORDNERZEICHEN` ginge ihr durch, und ein Leser, der den Text über `currentEditor` statt über
`stringValue` holt, fällt der zweiten Nadel nicht auf.

Verification: `make check` — exit 0.
