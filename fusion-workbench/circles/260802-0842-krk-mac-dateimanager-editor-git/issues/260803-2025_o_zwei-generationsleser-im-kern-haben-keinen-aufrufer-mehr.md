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
