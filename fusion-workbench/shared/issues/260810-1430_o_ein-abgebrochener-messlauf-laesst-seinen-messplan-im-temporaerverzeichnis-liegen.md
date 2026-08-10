Ein abgebrochener Messlauf laesst seinen Messplan im Temporaerverzeichnis liegen

---

`plan_schreiben` (`crates/krk-bench/src/messen.rs:1551`) legt
`krk-messplan-<pid>.toml` unter dem Temporaerverzeichnis an. Abgeraeumt wird die
Datei an genau einer Stelle, `messen.rs:1046`, und die liegt **hinter** der
Rundenschleife. Jeder `?` in der Schleife kehrt vorher zurueck, und die Datei
bleibt stehen. Auf dem Referenzgeraet liegen neun solche Dateien, vom 260805
bis zum 260807.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Abnahme des Defekts
`circles/260807-2116-eingebauter-editor-mit-textmarken/issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`
(Nachweis, dass ein voller Testlauf keinen Ordner zuruecklaesst)
**Betroffen:** `crates/krk-bench/src/messen.rs`
**Domain:** code

## Der Befund

```text
$TMPDIR/krk-messplan-16615.toml   Aug 6 12:15
$TMPDIR/krk-messplan-16677.toml   Aug 6 12:16
$TMPDIR/krk-messplan-16748.toml   Aug 6 12:16
$TMPDIR/krk-messplan-29958.toml   Aug 7 15:10
$TMPDIR/krk-messplan-55095.toml   Aug 5 23:58
$TMPDIR/krk-messplan-60917.toml   Aug 7 15:50
$TMPDIR/krk-messplan-61288.toml   Aug 7 16:02
$TMPDIR/krk-messplan-85953.toml   Aug 7 17:28
$TMPDIR/krk-messplan-91514.toml   Aug 6 13:54
```

Neun Dateien, neun Prozesskennungen. Der Testlauf legt keine davon an: alle
Zeitstempel liegen vor dem 260810, und ein voller `cargo test --workspace` am
260810-1420 hat die Menge nicht veraendert. Es sind Reste abgebrochener
Messlaeufe.

## Warum die Abraeumzeile nicht greift

```rust
let plan = plan_schreiben(self, &unterordner)?;          // 1029
for nummer in 1..=self.runden {
    let (gemeldete_rate, runde) = self.eine_gesamtrunde(&plan)?;   // 1040
    ...
}
let _ = std::fs::remove_file(&plan);                     // 1046
```

Die Abraeumzeile steht auf dem Erfolgsweg. `eine_gesamtrunde` kann fehlschlagen,
und genau der Fall ist der haeufige: der Abnahmelauf verlangt KRK im
Vordergrund, und aus dem Hintergrund gestartet meldet die Messstrecke
`NICHT_IM_VORDERGRUND` statt Zahlen.

## Fehlszenario

Kein Fehlverhalten der Messung; der Preis ist eine wachsende Zahl von Dateien
unter dem Temporaerverzeichnis, jede mit einer Prozesskennung, die es nicht mehr
gibt. Beim naechsten Abgleich der Reste sind sie Rauschen, und wer sie fuer
Spuren eines laufenden Vorgangs nimmt, sucht an der falschen Stelle. Der
Zusammenhang zum Pruefordner-Defekt ist die Bauform: dort raeumt `Drop` ab, hier
eine Zeile im Erfolgsweg.

## Vorgeschlagene Behebung

Die Bauform, die im Baum schon dreimal steht: ein Halter mit `Drop`. Der
Rueckgabewert von `plan_schreiben` wird ein Wert, der die Datei besitzt, und
sein `Drop` loescht sie. Damit fallen der Erfolgsweg und jeder Abbruchweg
zusammen, und die Zeile 1046 entfaellt.

Zwei Nachbarn im Baum tun das schon: `Wegwerfordner` in
`krk-bench/src/wegwerfordner.rs` raeumt in `Drop` auch den Steckbrief neben dem
Ordner ab, und `messen.rs:1422` loescht eine Datei in einem `Drop`. Ein zweiter
Mechanismus entsteht hier also nicht.

## Zustaendigkeit

`coder`.
