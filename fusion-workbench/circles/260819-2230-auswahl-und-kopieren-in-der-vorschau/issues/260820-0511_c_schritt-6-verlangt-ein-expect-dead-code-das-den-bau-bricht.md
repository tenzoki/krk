Schritt 6 verlangt ein `expect(dead_code)`, das den Bau bricht

---

Der Plan (`planning/260819-2245_o_plan-auswahl-und-kopieren-in-der-vorschau.md`, Schritt 6) und die Dispatch-Anweisung sagen beide, `text_auf_ablage_schreiben` habe „bis Schritt 7 keinen Rufer" und solle deshalb `#[cfg_attr(not(test), expect(dead_code, …))]` tragen, nach dem Vorbild aus `crates/krk-ui/src/kommandos/rueckschritt.rs`.

Das trifft die Verdrahtung nicht, die derselbe Schritt zwei Sätze vorher verlangt: `text_schreiben` reicht `NSPasteboard::generalPasteboard()` selbst in `text_auf_ablage_schreiben` hinein. Das ist ein Ruf, und `text_schreiben` ist seinerseits live (zwei Rufer in `appkit/tabelle.rs:1553` und `:1581`). `text_auf_ablage_schreiben` ist damit ab Schritt 6 transitiv erreichbar, nicht tot.

---

Geprüft: mit der Delegation implementiert und **ohne** die `expect(dead_code)`-Zeile laufen `cargo build --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` und `cargo test --workspace` grün. Mit der Zeile hätte `-D warnings` den Bau an `unfulfilled_lint_expectations` angehalten, weil die Erwartung „tot" nie zutrifft — anders als beim Vorbild `rueckschritt.rs`, wo die neue Funktion zum fraglichen Zeitpunkt wirklich gar keinen Rufer hatte.

Die Zeile ist deshalb in dieser Implementierung weggelassen. Der Rest des Schritts (Signatur, Modulkopf, Probe) ist wie geplant umgesetzt.

---
Resolved: die `expect(dead_code)`-Zeile aus dem Plantext wurde nicht gesetzt; `text_auf_ablage_schreiben` bleibt ohne sie warnungsfrei, weil `text_schreiben` es ab Schritt 6 bereits ruft. Alle vier Prüfkommandos laufen grün.
