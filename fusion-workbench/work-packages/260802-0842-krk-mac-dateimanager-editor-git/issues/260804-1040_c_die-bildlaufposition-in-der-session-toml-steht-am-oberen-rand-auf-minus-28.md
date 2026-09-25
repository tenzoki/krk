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

---
Resolved: Auflösung 1, verschieben beim Schreiben und beim Lesen. `crates/krk-ui/src/appkit/tabelle.rs` trägt dafür `bildlauf_ursprung()`; `bildlauf_merken` zieht den Ursprung vom rohen Wert ab, `bildlauf_herstellen` rechnet ihn wieder hinzu. In `session.toml` heißt 0 damit "ganz oben".

**Auflösung 2 (bei 0 abschneiden) wäre nur die halbe Wahrheit gewesen**, wie der Datensatz selbst schreibt: eine gescrollte Liste trüge weiter einen um 28 verschobenen Wert, und der Nutzer, der die Datei von Hand ändert, hätte eine Skala mit einem Knick bei null.

**Der Ursprung kommt aus der Kopfansicht der Tabelle, nicht aus dem Inhaltsrand der Bildlaufansicht.** Beides war denkbar; gemessen ist eines. Eine Sonde in `bildlauf_merken` lieferte am 260805 im laufenden Bündel:

```
SONDE roh=-28 insets.top=0 kopf=Some(28.0)
```

`NSScrollView::contentInsets` steht auf null, AppKit hält den Spaltenkopf also in der eigenen Kopfansicht der `NSTableView`. Ein `contentInsets` an dieser Stelle hätte dauerhaft mit null gerechnet und nichts verschoben; die erste Fassung tat genau das und ist an dieser Messung gescheitert. Die Sonde ist vollständig zurückgenommen, `grep -rn SONDE crates/` findet nichts mehr.

Die Höhe wird abgefragt und nicht hingeschrieben: sie hängt an der Systemschriftgröße. Ohne Kopfansicht ist der Ursprung null und die Umrechnung fällt von selbst weg.

**Nachgemessen am laufenden, signierten Bündel am 260805-0856.** Vor der Änderung trugen die beiden sichtbaren Tabs `bildlauf = -28.0`, nach dem Start und Beenden mit unverändertem Bildlauf tragen sie `bildlauf = 0.0`. Die drei verdeckten Tabs stehen weiter auf ihrem alten Wert, weil `bildlauf_merken` nur den sichtbaren Tab liest; sie ziehen nach, sobald der Nutzer sie einmal ansieht und wieder verlässt. Das ist eine Altlast der vorhandenen Datei und keine der Rechnung.

Die Kommentare an `Tabinhalt::bildlauf` (`crates/krk-ui/src/tabs.rs`) und an `Tab::bildlauf` (`crates/krk-core/src/ablage/sitzung.rs`) sagen jetzt beide ausdrücklich, dass 0 "ganz oben" heißt, und nennen diesen Datensatz. Der Kommentar im Kern behauptete das schon vorher ("vom oberen Rand der Liste aus") und war damit die dritte Stelle, an der die Datei etwas anderes sagte als der Code.

Geprüft am 260805-0856: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
