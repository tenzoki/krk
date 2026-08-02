Zwei Abnahmekriterien prüfen `unsafe` mit einem grep, das nie eine Datei nennen kann

---

Das Abnahmekriterium von Schritt 2 im Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` verlangt wörtlich:

> `grep -rln 'unsafe' crates/krk-core/src` nennt genau eine Datei, `verzeichnis/sys.rs`, und diese trägt `#[allow(unsafe_code)]`.

Das kann nicht aufgehen. Schritt 1 verlangt `#![deny(unsafe_code)]` als erste Zeile von `crates/krk-core/src/lib.rs`, und diese Zeile enthält die Zeichenkette `unsafe`. Der `grep` nennt deshalb zwangsläufig zwei Dateien, gleich wie sauber der Code ist.

Nachgeprüft am 260802-1810:

```
$ grep -rln 'unsafe' crates/krk-core/src
crates/krk-core/src/lib.rs
crates/krk-core/src/verzeichnis/sys.rs

$ grep -rn 'unsafe {' crates/krk-core/src
crates/krk-core/src/verzeichnis/sys.rs:180:        let geliefert = unsafe {
```

**Der gemeinte Sachverhalt hält.** Echter `unsafe`-Code steht ausschließlich in `verzeichnis/sys.rs`. Nur die Prüfvorschrift trifft ihn nicht.

**Derselbe Wortlaut steht auch im Abnahmekriterium von Schritt 15.** Dort tritt der Fehler erneut auf, sobald der Schritt abgenommen wird.

---

**Was zu tun ist.** Der `planner` ersetzt die Prüfvorschrift in beiden Schritten durch eine, die den gemeinten Sachverhalt trifft. Zwei Wege bieten sich an, die Wahl liegt beim `planner`:

- Auf den `unsafe`-Block statt auf das Wort prüfen, etwa `grep -rn 'unsafe {'`. Trifft den Code, nicht das Attribut. Ist gegenüber `unsafe fn` und `unsafe impl` unvollständig, die in diesem Modul aber nicht vorkommen.
- Auf das Attribut prüfen, etwa dass `#[allow(unsafe_code)]` in genau einer Datei steht. Trifft die eigentliche Zusage: es gibt genau eine Stelle, an der die Sperre geöffnet ist.

Der zweite Weg ist der genauere, weil `deny(unsafe_code)` die Zusage ohnehin maschinell erzwingt: der Bau bricht, sobald `unsafe` außerhalb einer Datei mit `#[allow]` auftaucht. Der `coder` hat das am 260802-1803 mit einem probeweise eingesetzten Block belegt und die Probe wieder entfernt.

**Aufgefallen bei:** der Umsetzung von Schritt 2, Protokoll `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1803-verzeichnisleser-und-ordnermodell.md`. Der `coder` hat den Widerspruch gemeldet statt ihn eigenmächtig aufzulösen, wie beauftragt.
