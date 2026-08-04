Das AppKit-Abnahmekriterium von Schritt 15 ist wörtlich nicht erfüllbar

---

Schritt 15 verlangt: `grep -rn 'AppKit\|objc2' crates/krk-core/src` liefert keinen Treffer. Das Kommando liefert Treffer, und zwar schon vor Schritt 15. Es sucht in Prosa mit, und die Doku des Kerns erklärt an mehreren Stellen, was der Kern gerade **nicht** tut.

---

## Der Befund

Am Stand `e43316d`, also vor der Umsetzung von Schritt 15, meldete das Kommando 18 Treffer in sieben Dateien:

| Datei | Treffer |
|---|---|
| `crates/krk-core/src/lib.rs` | 1 |
| `crates/krk-core/src/tasten/mod.rs` | 3 |
| `crates/krk-core/src/tasten/normalisierung.rs` | 8 |
| `crates/krk-core/src/tasten/parser.rs` | 1 |
| `crates/krk-core/src/verzeichnis/mod.rs` | 1 |
| `crates/krk-core/src/verzeichnis/sprungmarke.rs` | 3 |
| `crates/krk-core/src/zwischenablage.rs` | 1 |

Jeder einzelne steht in einem Dokumentationskommentar, die meisten in Sätzen der Form "der Kern kennt AppKit nicht" oder "diese Datei nennt keine `objc2`-Kiste". Schritt 15 hat drei weitere hinzugefügt, alle in der Begründung der Abhängigkeitsumkehr in `operation/loeschen.rs` und `operation/mod.rs`.

## Was das Kriterium eigentlich prüfen will, hält

Im **Code** gibt es keinen Treffer. Nachgeprüft am 260804-1649:

```
grep -rn 'AppKit\|objc2' crates/krk-core/src | grep -v '://' | grep -v '///'
```

liefert nichts. `crates/krk-core/Cargo.toml` nennt als Abhängigkeiten allein `serde` und `toml`; eine `objc2`-Kiste könnte der Kern also gar nicht einbinden, ohne dass der Bau abbricht.

## Warum das nicht nebenbei behoben ist

Zwei Wege, und beide gehören dem Nutzer:

1. **Das Kriterium schärfen**, etwa auf `grep -rn '^[^/]*\(AppKit\|objc2\)' crates/krk-core/src` oder auf eine Prüfung der `Cargo.toml`. Das ändert das Abnahmekriterium eines Schrittes und, für die sieben Dateien von vorher, rückwirkend das von S2, S7, S9 und S13.
2. **Die Prosa umschreiben**, so dass keine Datei des Kerns die beiden Wörter mehr nennt. Das nähme genau die Erklärungen weg, die einem späteren Leser sagen, warum die Grenze so verläuft, und in `operation/loeschen.rs` gerade die Begründung der einen Abhängigkeitsumkehr des Entwurfs.

Der zweite Weg macht den Code schlechter, um ein Kommando zufriedenzustellen; der erste ist eine Änderung am Plan. Verwandt, aber nicht dasselbe: `issues/260803-1530_o_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen.md` handelt von `krk-ui`, dieser Defekt von `krk-core`.

**Aufgefallen bei:** der Umsetzung von Schritt 15 am 260804-1649.
