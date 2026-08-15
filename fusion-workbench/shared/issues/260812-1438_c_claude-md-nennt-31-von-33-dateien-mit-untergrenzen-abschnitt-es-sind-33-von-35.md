CLAUDE.md nennt 31 von 33 Dateien mit Untergrenzen-Abschnitt, es sind 33 von 35

---

Der Abschnitt „Technologiewahl" in `CLAUDE.md` sagt, der Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen` stehe „am 260811 in **31 von 33**
Dateien unter `crates/krk-ui/src/appkit/`". Am 260812 nachgezählt sind es **33 von 35**.
Ohne den Abschnitt sind weiterhin nur `koordinaten.rs` und `mod.rs`, beide begründet; die
Deckung ist also nicht gesunken, sondern die Kiste ist um zwei Dateien gewachsen.

---

Gefunden vom `coder` beim Bau von `appkit/teilen.rs` (Planschritt 5 der Runde 6), der die
Zahl beim Anlegen der neuen Datei gegengelesen hat. Die beiden neuen Dateien sind
`appkit/standardprogramm.rs` aus der Runde 4 und `appkit/teilen.rs` aus dieser Runde.

Die Zahl veraltet mit jeder Datei, die unter `appkit/` dazukommt, und sie ist schon einmal
aus demselben Grund nachgezogen worden. Zwei Wege stehen offen: die Zahl bei jeder neuen
Datei von Hand nachziehen, oder sie durch die Nennung der zwei Ausnahmen ersetzen, die sich
selten ändern. Der zweite Weg wäre eine Änderung an `CLAUDE.md` und keine Zählpflege.

Herkunft: gemeinsamer Speicher, weil die Aussage das ganze Projekt betrifft und nicht die
Directive dieser Runde. Gefunden wurde sie beim Ausführen der Runde 6, verursacht hat sie
diese Runde nicht.

---
Abgleich 260812-2253 (reconciler): Der Defekt besteht, und die im Datensatz genannte Gegenzahl ist inzwischen selbst veraltet. Am 260812-2253 nachgezählt über `find crates/krk-ui/src/appkit -name '*.rs'`: **34 von 36**. Ohne den Abschnitt sind weiterhin allein `koordinaten.rs` und `mod.rs`. Die Kiste ist seit dem Ablegen dieses Datensatzes um eine weitere Datei gewachsen (`appkit/textmerkmale.rs`, Schritt 7 der Runde 6, Commit `9e089c0`), und die Zählung über das Wurzelverzeichnis allein liefert eine dritte Zahl, weil `appkit/blaetter/` ein Unterverzeichnis ist. Wer die Zahl in `CLAUDE.md` nach diesem Datensatz nachzieht, schreibt „33 von 35" und ist wieder falsch. Das ist das Argument für die zweite der beiden hier genannten Möglichkeiten: die zwei Ausnahmen nennen statt der Quote.

---
Abgleich 260813 (reconciler, Runde 7): **Der Defekt besteht, und die Zahl ist zum dritten Mal
gewachsen.** Am 260813 nachgezaehlt ueber `crates/krk-ui/src/appkit/*.rs` und
`crates/krk-ui/src/appkit/blaetter/*.rs`: **35 von 37**. Ohne den Abschnitt sind weiterhin
allein `koordinaten.rs` und `mod.rs`, beide begruendet; die Deckung ist also erneut nicht
gesunken. Neu ist `appkit/weitereinstanz.rs` aus Schritt S14 der Runde 7 (Commit `3caa2b7`),
und sie traegt den Abschnitt.

Die Folge der Zahlen lautet damit: `CLAUDE.md` zitiert 31 von 33, der Datensatz nannte 33 von
35, der Abgleich der Runde 6 nannte 34 von 36, heute sind es 35 von 37. Vier Staende in vier
Tagen. Der Datensatz bleibt offen; seine eigene Empfehlung, die zwei Ausnahmen statt der Quote
zu nennen, ist damit zum dritten Mal belegt.

---
Resolved: Über den zweiten der beiden Wege, die dieser Datensatz vorschlägt: die Zahl ist
durch die Nennung der zwei Ausnahmen ersetzt. `CLAUDE.md` (`## Technologiewahl`) sagt heute,
der Abschnitt stehe in **jeder** Datei unter `crates/krk-ui/src/appkit/` außer
`koordinaten.rs` und `mod.rs`, und hält ausdrücklich fest: „Eine Quote steht hier nicht — sie
ist zwischen dem 260811 und dem 260814 viermal falsch geworden, während die zwei Ausnahmen
dieselben blieben."

**Der Beleg, dass der Weg trägt**, am 260815-1405 nachgezählt: die Kiste führt heute 40
Dateien unter `appkit/` einschließlich `blaetter/`, gegenüber 35 am Tag dieses Datensatzes.
Gewachsen ist sie also um fünf, und die Aussage in `CLAUDE.md` ist trotzdem nicht veraltet:
ohne den Abschnitt sind weiterhin genau `koordinaten.rs` und `mod.rs`. Eine Quote hätte in
derselben Zeit ein sechstes Mal nachgezogen werden müssen.

Nicht behoben ist damit die Gewohnheit selbst: der Abschnitt hält sich weiter ohne Werkzeug,
und ob die Angabe prüfbar gemacht wird, ist offen
(`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`).
