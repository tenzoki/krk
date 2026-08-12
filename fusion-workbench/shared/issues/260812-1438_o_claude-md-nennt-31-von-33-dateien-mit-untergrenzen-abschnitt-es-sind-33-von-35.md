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
