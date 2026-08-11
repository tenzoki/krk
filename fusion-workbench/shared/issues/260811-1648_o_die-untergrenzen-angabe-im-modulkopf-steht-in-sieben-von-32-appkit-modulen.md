Die Angabe der macOS-Untergrenze im Modulkopf steht in sieben von 32 AppKit-Modulen

---

`CLAUDE.md` führt unter "Technologiewahl" als Gegenmaßnahme gegen die fehlenden Verfügbarkeitsangaben von `objc2` eine Gewohnheit: "jedes AppKit-Modul dieses Projekts nennt in seinem Modulkopf die Untergrenze jeder Klasse, die es anspricht". Gezählt am 260811 über `crates/krk-ui/src/appkit/*.rs` und `crates/krk-ui/src/appkit/blaetter/*.rs` erwähnen sieben von 32 Dateien überhaupt eine macOS-Version:

```
37 editor.rs      9 menue.rs      3 nummernspalte.rs      2 fsevents.rs
 2 anwendung.rs   1 leiste.rs     1 aufteilung.rs
```

Ohne jede Nennung sind unter anderen `tabelle.rs`, `zwischenablage.rs`, `terminal.rs`, `volumes.rs`, `vorschau.rs`, `fenster.rs`, `ereignisse.rs`, `papierkorb.rs`, `statuszeile.rs` und alle neun Module unter `blaetter/`.

---

Gefunden vom `planner` am 260811 beim Erheben der Grundlage für die Runde 4 (`circles/260811-1257-vier-tastenbefehle-pfade-kopieren-oeffnen`). Der Befund ist ein Widerspruch zwischen der Dokumentation und dem Baum und kein Fehler im Programm: die angesprochenen Klassen stehen sämtlich seit macOS 10.0 zur Verfügung, soweit nachgesehen, und ein Absturz ist von dieser Lage nicht zu erwarten.

**Er ist nebenbei gefunden und nicht durch die Directive dieser Runde verursacht**, deshalb liegt er im gemeinsamen Speicher und nicht im Circle. Die Runde 4 zieht die Angabe für die beiden Module nach, die sie ohnehin anfasst (`zwischenablage.rs` und das neue `standardprogramm.rs`), und lässt die übrigen stehen; das steht in ihrem Umsetzungsplan.

Zwei Wege stehen offen, und sie schließen einander nicht aus. Der eine trägt die Angabe in den 25 Modulen nach, die sie nicht haben — Handarbeit, einmalig, und danach stimmt der Satz in `CLAUDE.md`. Der andere schwächt den Satz in `CLAUDE.md` auf das ab, was gilt, etwa: die Angabe steht dort, wo eine Klasse oder eine Methode nach macOS 10.0 hinzugekommen ist. Welcher der beiden richtig ist, hängt daran, ob die Gewohnheit die Untergrenze **jeder** angesprochenen Klasse meint oder allein die der zweifelhaften; der Satz in `CLAUDE.md` sagt heute das erste, `menue.rs:135-146` macht das erste vor, und die Mehrheit der Module tut das zweite.
