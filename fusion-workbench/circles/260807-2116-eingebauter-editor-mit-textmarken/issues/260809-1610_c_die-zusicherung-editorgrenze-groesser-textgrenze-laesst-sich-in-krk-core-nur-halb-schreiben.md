Die Zusicherung `EDITORGRENZE > TEXTGRENZE` lässt sich in `krk-core` nur halb schreiben

---

S10 verlangt eine Zusicherung zur Übersetzungszeit in der Form
`const _: () = assert!(EDITORGRENZE > vorschau-TEXTGRENZE)`, „in derselben Form wie
`vorschaumodell.rs:97-100`". Sie hält fest, dass der Editor mehr annimmt als die
Vorschau, und genau das war der Grund für die zweite Zahl.

Die beiden Zahlen liegen in zwei Kisten. `EDITORGRENZE` steht seit S10 in
`crates/krk-core/src/text/datei.rs`, `TEXTGRENZE` steht in
`crates/krk-ui/src/vorschaumodell.rs:83`. `krk-ui` hängt von `krk-core` ab, nicht
umgekehrt, also kann `krk-core` die Zahl der Vorschau nicht benennen.

**Was S10 deshalb gebaut hat.** In `datei.rs` steht
`const _: () = assert!(EDITORGRENZE > 1024 * 1024);` mit der 1 MB als Zahl statt als
Bezug. Diese Hälfte fängt ein **Absenken** von `EDITORGRENZE` unter die
Vorschaugrenze. Sie fängt **nicht**, dass jemand `TEXTGRENZE` in `krk-ui` über
16 MB anhebt: dann stünde die Aussage „der Editor nimmt mehr an als die Vorschau"
nicht mehr, und kein Bau hielte an. Der Kommentar an der Zusicherung sagt das
ausdrücklich, damit niemand die halbe für die ganze hält.

**Vorschlag.** Die vollständige Zusicherung gehört dorthin, wo beide Zahlen sichtbar
sind, also nach `krk-ui`. S23 baut den Übergang aus der Vorschau in den Editor und ist
die Stelle, an der beide Grenzen ohnehin nebeneinander zu lesen sind; eine Zeile

```rust
const _: () = assert!(krk_core::text::datei::EDITORGRENZE > TEXTGRENZE);
```

dort ersetzt die halbe in `krk-core` nicht, sondern ergänzt sie um die fehlende
Richtung. Die halbe kann stehen bleiben: sie schützt `krk-core` für sich genommen und
kostet nichts.

Die Alternative wäre, `TEXTGRENZE` nach `krk-core` zu ziehen, damit beide Zahlen an
einem Ort stehen. Das ist die sauberere Form, ändert aber `vorschaumodell.rs`, und der
Nutzer hat die Vorschau für S10 ausdrücklich unangetastet gelassen. Die Entscheidung
gehört deshalb nicht in S10.

Gemeldet von: `coder`, bei der Umsetzung von S10.

---

Resolved: Am 260810-0919 auf dem Weg des Vorschlags geschlossen, aber an einer
anderen Stelle als der dort genannten. Die fehlende Richtung steht jetzt als

```rust
const _: () = assert!(krk_core::text::datei::EDITORGRENZE > TEXTGRENZE);
```

in `crates/krk-ui/src/vorschaumodell.rs`, unmittelbar unter der bestehenden
`BILDGRENZE > TEXTGRENZE`, mit einer Begründung im Ton der Nachbarin. Ein
Anheben von `TEXTGRENZE` über 16 MB hält damit den Bau an.

**Nicht S23, sondern die Konstante selbst.** Der Vorschlag legte die vollständige
Zusicherung in den Übergang aus der Vorschau in den Editor (Schritt 23), weil
dort beide Grenzen ohnehin nebeneinander zu lesen sind. Sie liegt jetzt neben
`TEXTGRENZE`, und zwar aus zwei Gründen: dort steht schon eine Zusicherung
derselben Form über dieselbe Zahl, und wer `TEXTGRENZE` anhebt, liest die Zeile
darunter, während er den Übergangscode aus S23 nicht aufsucht. Der zweite Grund
war der ausschlaggebende — eine Zusicherung wirkt zwar überall in der Kiste
gleich, aber gelesen wird sie nur da, wo jemand hinsieht.

Die halbe Zusicherung in `krk-core/src/text/datei.rs` steht wie vorgeschlagen
weiter, ihr Kommentar nennt jetzt aber die Gegenrichtung als gebaut statt als
anstehend. `TEXTGRENZE` ist nicht nach `krk-core` gezogen worden; die Vorschau
behält ihre Zahl, und die im Befund genannte Alternative bleibt damit ungenutzt
und unnötig.

Geändert: `crates/krk-ui/src/vorschaumodell.rs` (nur die neue Zusicherung samt
Kommentar), `crates/krk-core/src/text/datei.rs` (nur der Kommentar an der halben
Zusicherung). Abgenommen mit `cargo build/test/clippy/fmt --workspace`, alle
vier auf 0.
