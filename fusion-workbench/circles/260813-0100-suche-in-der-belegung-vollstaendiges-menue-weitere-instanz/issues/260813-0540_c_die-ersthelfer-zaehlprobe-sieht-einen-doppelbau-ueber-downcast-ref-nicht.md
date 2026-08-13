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

---

Resolved: Behoben in Turn 2 der siebten Runde am 260813, zusammen mit den zwei verwandten Befunden derselben Durchsicht (`die-zaehlproben-in-krk-ui-sagen-im-baum-und-lesen-nur-eine-kiste`, `zwei-aufruferzaehlungen-haengen-an-der-schreibweise-des-aufrufs`).

**Die zweite Nadel erfasst jetzt beide Schreibweisen, die dieser Baum kennt:** `isKindOfClass(` und `downcast_ref::<NSText`. Das eine Wort deckt alle drei Textklassen ab, weil `NSTextView` und `NSTextField` damit beginnen, und laesst die Frage nach `NSView` in `appkit/anwendung.rs` heraus, die keine Textklasse nennt. Der im Datensatz gezeigte Doppelbau macht die Probe damit rot.

**Die Probe ist zugleich ehrlicher beschriftet, wie der Datensatz es als Alternative nennt** — und beides zusammen, nicht statt einander. Ihr Doc-Kommentar sagt jetzt, was sie nicht faengt: eine dritte Schreibweise derselben Frage, etwa ueber `class()` und einen Vergleich. Der Kopf von `crates/krk-ui/src/quellbaum.rs` traegt die Begruendung im Langen.

**Der verwandte Punkt am Kopf von `quellbaum.rs` ist mitbehoben.** Dort stand, eine Erklaerungszaehlung „haelt, was sie verspricht"; das war zu weit gegriffen, und die Runde hat den Gegenbeweis selbst geliefert. Der Abschnitt sagt jetzt, dass sie gegen eine Kopie unter **demselben Namen** haelt, nennt die vierte Pruefordner-Fassung als Gegenbeispiel und zieht drei Folgerungen als Bauanleitung fuer jede neue Zaehlprobe: nach dem Gegenstand suchen statt nach dem Namen, jede schon vorhandene Schreibweise erfassen, und die verbleibende Blindheit am Doc-Kommentar benennen.
