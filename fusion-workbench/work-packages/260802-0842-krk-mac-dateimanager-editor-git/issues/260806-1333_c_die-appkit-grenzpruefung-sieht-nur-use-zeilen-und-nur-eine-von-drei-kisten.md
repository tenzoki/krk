Die AppKit-Grenzprüfung sieht nur use-Zeilen und nur eine von drei Kisten

---

`appkit_grenze_pruefen` (`xtask/src/release.rs:102-106`) begeht allein
`crates/krk-ui/src` und fragt je Zeile `ist_objc2_use`. Zwei Lücken bleiben
nach der Verbesserung aus `4195aa3` offen: ein voll ausgeschriebener
`objc2::`-Pfad ohne `use`-Zeile fällt durch, und `krk-core` sowie `krk-bench`
werden gar nicht begangen.

---

**Die erste Lücke.** `ist_objc2_use` liest ausschließlich `use`-Zeilen.
Gültiges Rust braucht die aber nicht: `objc2::rc::Weak::from_retained(&x)`
steht so heute mehrfach in `appkit/anwendung.rs` und käme außerhalb von
`appkit/` unbemerkt durch. Die Prüfung würde eine grüne Zeile ausgeben, obwohl
die Grenze verletzt ist.

**Die zweite Lücke.** CLAUDE.md und der Modulkopf von `krk-core/src/lib.rs`
sagen zu, dass `krk-core` keine `objc2`-Kiste nennt. Das Abnahmekriterium von
S23 verweist dafür auf S15, also auf die Abhängigkeiten der Kiste — das trägt
für `krk-core`, weil eine Kiste ohne die Abhängigkeit den Namen nicht
übersetzen kann. Für `krk-bench` steht keine entsprechende Zusage. Der Ordner
wird von keiner der beiden Prüfungen begangen.

**Geprüft, dass der heutige Baum sauber ist.**
`grep -rn "objc2" crates/krk-ui/src --include="*.rs" | grep -v "krk-ui/src/appkit/"`
liefert zehn Treffer, alle in Modulkommentaren der Form "In dieser Datei steht
keine `use objc2`-Zeile". `grep -rn "objc2" crates/krk-core/src --include="*.rs"`
liefert zwei, ebenfalls Kommentare. Es gibt heute keinen Verstoß; die Lücke ist
eine im Gate, nicht im Code.

**Der Fix.** `dateien_pruefen` zusätzlich auf einen ausgeschriebenen Pfad prüfen
(eine Zeile, die außerhalb eines Kommentars `objc2` gefolgt von `::` enthält)
und die Wurzelliste um `crates/krk-bench/src` erweitern. Das Abnahmekriterium
von S23 im Plandokument nennt heute nur die `use`-Form und zieht mit.

`speculation:` Eine Prüfung auf `objc2::` außerhalb von Kommentaren braucht
einen einfachen Zustandsautomaten für `//` und `/* */`, sonst schlägt sie auf
denselben zehn Kommentarzeilen an, die die Verankerung am Zeilenanfang gerade
ausgeschlossen hat. Ob das den Aufwand wert ist, entscheidet der Coder.

**Betrifft:** `xtask` (`release.rs`) und das Abnahmekriterium von S23 im
Plandokument. Kein laufender Code, kein Nutzerverhalten, keine Zeitzusage aus
C8.

---

Resolved: `xtask/src/release.rs` prüft jetzt beide Formen — die `use`-Zeile über
`ist_objc2_use` und den ausgeschriebenen Pfad über `nennt_objc2_pfad` — und
begeht dabei alle drei Quellwurzeln (`crates/krk-ui/src` ohne `appkit/`,
`crates/krk-core/src`, `crates/krk-bench/src`, Liste `GRENZWURZELN`). Die
Kommentarbehandlung ist kein Zustandsautomat, sondern die Regel "erstes
nicht-leeres Zeichen ein `/`, dann wird die Zeile nicht gelesen"; die zwölf
Kommentarzeilen des Baums treffen sie sämtlich, und in `crates/` steht kein
Blockkommentar. Die Begründung steht am Programmtext. Vier neue Tests: der
ausgeschriebene Pfad schlägt an, die zwölf Kommentarzeilen wörtlich nicht, und
`die_grenzpruefung_laeuft_am_baum_gruen` lässt die Prüfung bei jedem
`cargo test` am echten Baum laufen. Das Abnahmekriterium von S23 im Plan nennt
beide Suchen und die drei Wurzeln. `make check` grün.
