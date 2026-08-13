Die Ersthelfer-Zählprobe sieht einen Doppelbau über `downcast_ref` nicht

---

`die_frage_nach_dem_ersthelfer_steht_an_genau_einer_stelle`
(`crates/krk-ui/src/appkit/ereignisse.rs:690-731`) hält nach ihrem eigenen Doc-Kommentar die
Zusage, dass niemand „anderswo im Baum eine eigene Prüfung auf `NSTextView`, `NSTextField`
und `NSText`" schreibt. Sie prüft das mit zwei Nadeln: `fn ersthelfer_gehoert_appkit` genau
einmal, und `isKindOfClass(` in genau einer Datei.

**Die zweite Nadel bindet an eine Schreibweise, und der Baum kennt schon eine zweite.**
`Anwendungsdelegierter::ersthelferbereich` fragt den Ersthelfer nach seinem Typ über
`ersthelfer.downcast_ref::<NSView>()` (`crates/krk-ui/src/appkit/anwendung.rs:4070`) und nicht
über `isKindOfClass`. Wer die verbotene zweite Fassung in derselben idiomatischen Form
schriebe —

```rust
ersthelfer.downcast_ref::<NSTextField>().is_some()
    || ersthelfer.downcast_ref::<NSTextView>().is_some()
```

— hätte genau den Doppelbau gebaut, den die Probe abwehren soll, und beide Nadeln blieben
grün. Die erste greift nicht, weil die neue Funktion einen anderen Namen trägt; die zweite
nicht, weil `isKindOfClass` darin nicht vorkommt.

---

**Schwere:** mittel. Heute gibt es keinen zweiten Bau; nachgezählt am 260813 hat
`ersthelfer_gehoert_appkit` genau eine Erklärung und eine Aufrufstelle (`anwendung.rs:2552`).
Der Befund betrifft die Wache und nicht den Zustand: die Zusage der Runde, dass diese Frage
genau einmal beantwortet wird, ist nicht so abgesichert, wie der Doc-Kommentar der Probe
behauptet.

**Gefunden:** coderev, Durchsicht von `ca66c39..40b5fb0` am 260813-0540

**Betroffen:** `crates/krk-ui/src/appkit/ereignisse.rs:690-731`,
`crates/krk-ui/src/appkit/anwendung.rs:4070`

**Domain:** code

## Vorschlag

Die zweite Nadel um die zweite Schreibweise erweitern, also neben `isKindOfClass(` auch
`downcast_ref::<NSText` erfassen, und die Erwartung weiter als Dateiliste formulieren.
`anwendung.rs` fällt dann nicht mit hinein, weil es `downcast_ref::<NSView>` prüft und nicht
eine Textklasse. Alternativ die Probe ehrlicher beschriften: sie hält, dass die **vorhandene**
Prüfung nicht wandert, und nicht, dass keine zweite entsteht.

**Verwandt:** `crates/krk-ui/src/quellbaum.rs` schreibt den Unterschied zwischen
Erklärungs- und Aufruferzählung aus und behauptet dort, eine Erklärungszählung „hält, was sie
verspricht". Dieser Fall ist das Gegenbeispiel; siehe den Datensatz von derselben Durchsicht
zu der Reichweite des Quellbaumlesers.
