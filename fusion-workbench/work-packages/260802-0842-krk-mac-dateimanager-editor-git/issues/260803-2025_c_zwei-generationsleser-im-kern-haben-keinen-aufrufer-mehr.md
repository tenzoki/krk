`Meldung::generation` und `Lesevorgang::generation` haben keinen Aufrufer mehr

---

Mit dem Entfernen der Generationsprüfung aus `crates/krk-ui/src/appkit/tabelle.rs` am 260803-2025 sind zwei öffentliche Leser in `crates/krk-core/src/verzeichnis/leser.rs` ohne Aufrufer geblieben. Weil beide `pub` in einer Bibliothekskiste sind, meldet der Übersetzer nichts.

---

## Die beiden Stellen

- `Meldung::generation()` (`leser.rs:85-88`). Der einzige Aufrufer war die entfernte Bedingung `modell.gehoert_dazu(meldung.generation())`. `krk-bench` liest das Feld über ein Muster (`messen.rs:225`) und nicht über diesen Leser.
- `Lesevorgang::generation()` (`leser.rs:120-122`). Nachgeprüft am 260803-2025 mit `grep -rn '\.generation()' crates/ xtask --include='*.rs'`: keine Fundstelle außer den Definitionen selbst.

Noch benutzt und deshalb **nicht** betroffen: `Ordnermodell::generation()` und `Ordnermodell::gehoert_dazu()`. Beide stehen in `crates/krk-core/tests/verzeichnis.rs` und in `crates/krk-bench/src/messen.rs:228`.

## Was zu entscheiden ist

Ob die Generationsnummer nach dem Wegfall der Prüfung überhaupt noch die Reichweite braucht, die sie heute hat. Sie trägt weiter zwei echte Aufgaben: sie benennt den Lesefaden (`krk-verzeichnisleser-<n>`) und sagt dem Modell beim Leeren, zu welchem Lauf sein Inhalt gehört. Ob dafür beide Leser nötig sind und ob jede `Meldung` die Nummer mitführen muss, ist offen.

Zwei Wege, nicht beide:

1. Die beiden Leser entfernen und die Nummer auf das reduzieren, was sie noch trägt.
2. Sie stehen lassen, weil S12 mit zwei Dateifenstern und mehreren Tabs sie wieder braucht, und das im Kopf von `leser.rs` als Vorsorge kennzeichnen, statt es offen zu lassen.

## Warum das nicht gleich mitbehoben ist

`crates/krk-core/src/verzeichnis/leser.rs` lag außerhalb des Auftragsumfangs des `coder` vom 260803-2025; der reichte über `appkit/`, `tasten/`, `modell.rs` und `tests/tasten.rs`. Die Frage betrifft außerdem den Zuschnitt der Nummer und nicht nur zwei Zeilen.

**Aufgefallen bei:** der Behebung von `issues/260803-1536_c_die-generationspruefung-kann-nicht-greifen-und-verdeckt-den-wirksamen-mechanismus.md`.

---
Resolved: Weg 1. `Meldung::generation()` und `Lesevorgang::generation()` sind aus `crates/krk-core/src/verzeichnis/leser.rs` entfernt, dazu das damit unbenutzte Feld `Lesevorgang::generation`.

**Warum nicht Weg 2 (stehen lassen als Vorsorge für S12).** Die Annahme, die Weg 2 trägt, ist inzwischen überholt: S12 ist gebaut, und die Prüfung, ob er die Leser wieder braucht, lässt sich am Code ablesen statt vorhersagen. `crates/krk-ui/src/tabs.rs` gibt jedem Tab einen eigenen `Lesevorgang`, `einzug_je_tab` liest allein aus dessen Kanal, und ein Ordnerwechsel wirft den alten Vorgang samt Kanal weg. Ein fremder Stapel kann deshalb gar nicht ankommen; das steht im Kommentar an `einzug_je_tab` und ist der Grund, aus dem die Generationsprüfung am 260803-2025 gefallen ist. Zwei öffentliche Leser ohne Aufrufer stehen zu lassen, weil ein bereits gebauter Schritt sie vielleicht doch wieder braucht, wäre eine Vorsorge gegen etwas, das schon eingetreten ist.

**Ein dritter Leser war ebenfalls tot** und ist im selben Zug entfallen: `Lesevorgang::ist_abgebrochen()`. Nachgeprüft mit `grep -rn '\.ist_abgebrochen()' crates/ --include='*.rs'`: die drei Fundstellen außerhalb der Definition gehören `Abschluss::ist_abgebrochen` (`tests/verzeichnis.rs:196`) und `Lauf::ist_abgebrochen` (`operation/fortschritt.rs`, `appkit/anwendung.rs:1573`), keine dem `Lesevorgang`. Der Defekt nennt ihn nicht, weil er nicht am selben Tag verwaist ist; er hat aber dieselbe Ursache, dass ein `pub` in einer Bibliothekskiste keinen Übersetzerhinweis bekommt.

**Der Zuschnitt der Nummer, den der Defekt offen lässt.** Sie bleibt, wo sie ist, und trägt zwei Aufgaben: sie benennt den Lesefaden (`krk-verzeichnisleser-<n>`) und sagt `Ordnermodell::leeren`, zu welchem Lauf der Inhalt gehört. Beides sind Aufgaben des Aufrufers, der die Nummer ohnehin hält und sie deshalb nie zurückbekommen muss. In der `Meldung` bleibt sie liegen, weil `krk-bench` sie dort über ein Muster liest (`messen.rs:225`) und gegen `Ordnermodell::gehoert_dazu` hält. Der Kopfkommentar von `leser.rs` schreibt diesen Zuschnitt jetzt aus, statt weiter den entfernten Filter zu beschreiben; er war dort seit dem 260803-2025 stehen geblieben.

Geprüft am 260805-0841: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
