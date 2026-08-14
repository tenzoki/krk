Der Prüfschritt für die Sichtbarkeit steht im Ordnermodell zweimal wortgleich da

---

`Ordnermodell` entscheidet an zwei Stellen, ob ein Eintrag in die Sichtreihenfolge kommt, und beide Stellen tragen dieselbe Regel als eigene Fassung:

- `anhaengen` prüft `let sichtbar = !(self.verstecke_ausblenden && eintrag.versteckt);` (`crates/krk-core/src/verzeichnis/modell.rs:199`)
- `sicht_neu_aufbauen` prüft `.filter(|(_, eintrag)| !(ausblenden && eintrag.versteckt))` (`crates/krk-core/src/verzeichnis/modell.rs:437`)

Solange die Regel eine Zeile lang ist, fällt das nicht auf. Eine Regel mit fünf Eingaben und sechs Zweigen an zwei Stellen zu führen wäre der Zustand, den dieses Projekt sonst überall ausschließt: eine zweite Wahrheit über dieselbe Sache, die beim ersten Nachjustieren auseinanderläuft. Der Modulkopf von `modell.rs` beschreibt `sichtreihenfolge` als „die aktuelle Sortierung samt Filter" und benennt damit eine Sache, die im Code zweimal steht.

---

**Kontext.** Aufgefallen beim Erheben der Grundlage für den Plan der Runde 10 (`planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, erste Feststellung unter `## Was der Bau vorfindet`). Der Zustand ist vor dieser Runde entstanden und bestünde ohne sie fort; er ist deshalb ein eigener Datensatz und keine Zeile im Plan.

**Behoben wird er nebenbei.** Schritt A1 des Plans zieht den einen Prüfschritt `Ordnermodell::sichtbar(index)` und lässt beide heutigen Fassungen mit; C6.8 des Spec verlangt genau das („Es entstehen keine zwei Regeln für denselben Vorgang, sondern ein zweiter Prüfschritt in derselben Sicht"). Wer den Datensatz schließt, prüft, dass die Regel danach an genau einer Stelle steht und beide Aufrufer sie rufen.
