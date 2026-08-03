Schritt 8 legt Perzentil und Bericht in eine Datei, die nur die eine Hälfte der Messung kennt

---

Der Absatz zur Grenze in Schritt 8 sagt: "`crates/krk-ui/src/messmodus.rs` behält, was kein AppKit berührt: den Ablauf der Messung, die zwanzig Wiederholungen, das 95. Perzentil und den Bericht." Der Bericht kann dort nicht entstehen, und das folgt aus dem Schritt selbst.

---

Schritt 8 misst L4 laut eigener Vorschrift "von außen gestartet und über einen Zeitstempel der Anwendung abgeschlossen". Die Spanne beginnt damit in einem anderen Prozess als dem, der sie beendet, und zwanzig Wiederholungen von L4 sind zwanzig Prozessstarts. L1, L2, L3 und L10 misst dagegen die laufende Anwendung in einem einzigen Prozess. **Ein Bericht über alle fünf Zusagen kann nur dort entstehen, wo beide Hälften zusammenkommen, und das ist der äußere Aufrufer.**

Dazu kommt der Bedingungskopf. Er trägt acht Angaben, darunter `sysctl -n hw.model`, `sw_vers` sowie Pfad und Startwert jedes Prüfordners aus dessen Steckbrief. `crates/krk-bench/src/bericht.rs` erhebt all das seit Schritt 3. Dieselbe Erhebung in `krk-ui` ein zweites Mal aufzubauen wäre eine zweite Wahrheit über das Berichtsformat, und die beiden liefen beim ersten Nachzug auseinander.

Umgesetzt ist deshalb: `messmodus.rs` hält den Ablauf, die zwanzig Wiederholungen und die Ausgabe der Einzelwerte; das 95. Perzentil und der Bericht liegen in `crates/krk-bench/src/messen.rs`, das die Perzentilfunktion seit Schritt 3 führt. Der Satz im Plan ist entsprechend nachzuziehen. Dieselbe Formulierung steht sinngemäß in Schritt 21 und trifft dort denselben Punkt: auch dort misst der äußere Aufrufer L4.

Was der Absatz richtig sagt und was unberührt bleibt: in `messmodus.rs` steht keine `use objc2`-Zeile, und über die Grenze gehen nur gewöhnliche Rust-Werte.
