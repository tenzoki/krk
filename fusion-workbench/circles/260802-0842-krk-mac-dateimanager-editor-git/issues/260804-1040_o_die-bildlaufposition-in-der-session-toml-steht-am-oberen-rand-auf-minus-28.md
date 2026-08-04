Die Bildlaufposition in der session.toml steht am oberen Rand auf −28

---

Ein Tab, dessen Liste ganz oben steht, schreibt `bildlauf = -28.0` nach
`session.toml`. Die Zahl ist richtig gerechnet und trotzdem verwirrend: −28 ist
die Höhe der Spaltenüberschriften, und der Ursprung der Bildlaufansicht liegt um
genau diesen Betrag über dem oberen Rand der Liste. Der Nutzer soll diese Datei
lesen und von Hand ändern können; das ist der Grund, aus dem `### Frage 4` TOML
gewählt hat. Eine negative Zahl für "ganz oben" ist dort eine Stolperstelle.

---

Gemessen am 260804-1040 im signierten Bündel, in jedem Tab, dessen Liste nicht
gescrollt ist.

Die Wirkung ist heute allein kosmetisch. `Tabinhalt::aus_zustand` setzt das
Kennzeichen für die Wiederherstellung nur bei `bildlauf > 0.0`, ein Tab am oberen
Rand stellt also nichts wieder her und steht nach dem Start von selbst richtig.
Ein Nutzer, der `bildlauf = 0.0` von Hand einträgt, bekommt ebenfalls den oberen
Rand zu sehen, nur über einen anderen Weg.

Zwei Auflösungen sind denkbar:

1. **Beim Schreiben und Lesen um die Kopfhöhe verschieben**, sodass 0 wirklich
   "ganz oben" heißt. Die Kopfhöhe steht in der `NSTableView` und ist zur
   Laufzeit abfragbar.
2. **Die Zahl beim Schreiben bei 0 abschneiden.** Kleiner Eingriff, aber er
   verschiebt das Problem nur: eine gescrollte Liste trägt weiter einen um 28
   verschobenen Wert.

Cross-references:
`crates/krk-ui/src/tabs.rs` (`Tabinhalt::bildlauf`),
`crates/krk-ui/src/appkit/tabelle.rs` (`bildlauf_merken`, `bildlauf_herstellen`),
`crates/krk-core/src/ablage/sitzung.rs` (`Tab::bildlauf`)
