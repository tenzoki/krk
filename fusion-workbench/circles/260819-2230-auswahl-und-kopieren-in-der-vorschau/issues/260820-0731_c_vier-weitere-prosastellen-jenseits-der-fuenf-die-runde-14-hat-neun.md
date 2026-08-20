Vier weitere Prosastellen jenseits der fünf: die Runde 14 hat neun

---

Der Plan (`planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Abschnitt `## Was der Übersetzer einfordert, und was er nicht einfordert`) führt unter „Nichts hält es" vier Prosastellen. Der Befund `260820-0604_c_der-modulkopf-von-textautomatik-…` hat eine fünfte nachgetragen. Beim Durchgang von Schritt 8, dem letzten Blick auf den ganzen Baum vor der Bündelabnahme, sind vier weitere aufgefallen.

Alle vier sind Aussagen über den Baum, die diese Runde falsch oder unvollständig gemacht hat. Keine hält der Übersetzer, keine hält eine Probe, und keine ist an einer der drei Dateien aufgetaucht, die der Plan für Schritt 8 nennt.

## Die vier Stellen

| Stelle | Was sie sagte | Warum das nicht mehr trägt |
|---|---|---|
| `crates/krk-ui/src/appkit/editor.rs:3107` | die Textanzeige der Vorschau lehne Bearbeitbarkeit und Auswählbarkeit ab, „damit sie den Fokus nicht als Textsystem nimmt" | dieselbe Aussage wie die fünfte Stelle, an einer zweiten Datei: die Vorschau ist auswählbar, und sie nimmt den Fokus als Textsystem |
| `crates/krk-ui/src/kommandos/zulaessigkeit.rs:136` | „Die Textfläche des eingebauten Editors ist die eine Ausnahme davon" — am Feld `Lage::ersthelfer_gehoert_appkit` | es sind seit dieser Runde zwei, und das Feld ist die Stelle, an der ein Leser die Menge nachschlägt |
| `crates/krk-ui/src/appkit/menue.rs:784` | `NSTextView` stehe zweimal im Programm, als Textfläche des Editors und als Feldeditor | seit dieser Runde dreimal; die Klassenliste der Probe bleibt trotzdem bei sechs, weil `Vorschautext` eine Unterklasse ist |
| `crates/krk-ui/src/kommandos/fokus.rs:82` | zu `Fokus::Vorschau` komme der Fokus „per Mausklick in die Inhaltsfläche der Vorschau" | seit Schritt 5 nimmt auch die Textanzeige den Rang, und `bereich_des_ersthelfers` entscheidet über den Ansichtsbaum (`anwendung.rs:5642-5654`), also liefern beide Ansichten diesen Wert |

Die vierte ist eine Unvollständigkeit und keine Falschaussage. Sie steht hier trotzdem, weil sie in dieselbe Richtung irreführt: wer sie liest und danach eine Anzeige an den Fokus hängt, prüft die falsche Ansicht ab.

## Was der Befund über den Plan sagt

**Die Zahl vier im Plan war eine Erhebung und keine Zusage**, und sie ist auf demselben Weg entstanden wie die drei Zählerwartungen aus `260820-0646_o_…`: am Baum behauptet statt am Baum gezählt. Der Unterschied ist, dass eine falsche Zählerwartung beim ersten `make check` rot wird und eine fehlende Prosastelle nicht. Neun statt vier heißt: die Erhebung hat weniger als die Hälfte gefunden.

**Das Suchmuster ist der wahrscheinliche Grund.** Alle vier gefundenen Stellen des Plans liegen in Modulköpfen oder Doc-Kommentaren von `appkit/`-Dateien, die die Runde ohnehin anfasst. Die vier hier liegen in Dateien, die der Plan gar nicht nennt, und drei davon sagen dasselbe wie eine der genannten, nur an anderer Stelle. Eine Erhebung, die von den geänderten Dateien ausgeht, findet sie nicht; eine, die von der *Aussage* ausgeht — „wer behauptet im Baum etwas über die Auswählbarkeit der Vorschau" —, findet sie.

---

Resolved: alle vier in Schritt 8 nachgezogen, jede mit einer Berichtigung, die die Runde nennt und den tragenden Teil der alten Aussage stehen lässt. `editor.rs` sagt jetzt, dass der Unterschied zwischen den beiden Flächen nur noch die Bearbeitbarkeit ist und beide den Fokus als Textsystem nehmen. `zulaessigkeit.rs` nennt beide eigenen Textflächen und hält daneben fest, dass die Fläche eines Blattes ausdrücklich nicht dazugehört. `menue.rs` zählt drei `NSTextView` und schreibt aus, warum die Klassenliste trotzdem sechs Einträge behält. `fokus.rs` sagt, dass der Bereich über den Ansichtsbaum entschieden wird und beide Ansichten der Vorschau ihn liefern. `make check` läuft danach grün, alle vier Kommandos.

Nicht behoben ist die Ursache: die Erhebung der Prosastellen bleibt eine Handarbeit ohne Muster. Ob das eine Vorkehrung bekommt, ist offen und gehört zum Rundenabschluss.
