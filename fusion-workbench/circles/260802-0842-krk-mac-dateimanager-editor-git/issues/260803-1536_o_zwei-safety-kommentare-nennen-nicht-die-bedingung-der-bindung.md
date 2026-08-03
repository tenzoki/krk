Zwei SAFETY-Kommentare nennen nicht die Bedingung, die die Bindung tatsächlich verlangt

---

Die `unsafe`-Blöcke unter `crates/krk-ui/src/appkit/` sind durchgehend
kommentiert, und die meisten Kommentare treffen genau die Bedingung, die
`objc2` dokumentiert. Zwei tun das nicht: sie begründen etwas anderes als das,
was die Bindung verlangt. Beide Bedingungen sind erfüllt, keiner der beiden
Blöcke ist unsicher. Der Defekt ist der Beleg, nicht das Verhalten.

---

## Erstens: der Ereignisabgriff

`crates/krk-ui/src/appkit/ereignisse.rs:76-81`:

```rust
// SAFETY: Der Block hat die Signatur, die der Abgriff verlangt, und
// AppKit kopiert ihn beim Einrichten auf den Haldenspeicher. Er haelt
// `ziel` fest und ueberlebt damit den Aufruf.
let merkzeichen = unsafe {
    NSEvent::addLocalMonitorForEventsMatchingMask_handler(NSEventMask::KeyDown, &block)
}?;
```

`objc2-app-kit-0.3.2/src/generated/NSEvent.rs:1172-1181` dokumentiert genau eine
Bedingung:

```
/// # Safety
///
/// `block` block's return must be a valid pointer or null.
```

Die Signatur des Blocks prüft der Übersetzer ohnehin, und die Lebensdauer regelt
`RcBlock`. Die eine Bedingung, die der Aufrufer zu tragen hat, ist der
Rückgabewert, und sie steht nicht im Kommentar. Sie ist erfüllt: der Block liefert
`std::ptr::null_mut()` oder den Zeiger, den AppKit hereingegeben hat
(`ereignisse.rs:66-73`). Genau dieser Satz gehört in den Kommentar.

## Zweitens: Datenquelle und Delegierter der Tabelle

`crates/krk-ui/src/appkit/tabelle.rs:605-611`:

```rust
// SAFETY: Beide Objekte beantworten die Protokolle, die sie oben
// implementieren, und leben laenger als die Tabelle: `Dateifenster`
// haelt sie fest.
unsafe {
    tabelle.setDataSource(Some(ProtocolObject::from_ref(delegierter.quelle())));
    tabelle.setDelegate(Some(ProtocolObject::from_ref(&*delegierter)));
}
```

Der zweite Halbsatz stimmt beim Abbau nicht. `Dateifenster` hält `sicht` und
`delegierter` (`tabelle.rs:574-577`), die Quelle nur mittelbar über den
Delegierten. Die Tabelle selbst hält niemand außer der Quelle und der
Bildlaufansicht. Fällt `Dateifenster`, ist die Reihenfolge:

```
Dateifenster fällt
  └─ sicht fällt        → NSScrollView dealloc → gibt documentView (Tabelle) frei
  └─ delegierter fällt  → dealloc → gibt quelle frei
                                      └─ quelle dealloc → gibt Tabelle frei
                                                            └─ Tabelle dealloc
```

Die Tabelle wird also **zuletzt** freigegeben, mitten im Abbau von Quelle und
Delegiertem, nicht vor ihnen. "Sie leben länger als die Tabelle" ist als
Begründung damit falsch herum.

Getragen wird die Sicherheit von etwas anderem, und `objc2` schreibt es an
dieselbe Stelle (`objc2-app-kit-0.3.2/src/generated/NSTableView.rs:402-421`):

```
/// Setter for [`dataSource`][Self::dataSource].
///
/// This is a [weak property][objc2::topics::weak_property].
```

Weil beide Eigenschaften nullende schwache Verweise sind, steht dort `nil`,
sobald Quelle oder Delegierter in ihren `dealloc` gehen. Die Tabelle sendet
danach an niemanden mehr. Das ist der richtige Satz, und er ist kürzer als der
falsche.

## Warum das jetzt zählt und nicht später

Diese sechs Dateien sind die Vorlage für jeden weiteren AppKit-Aufruf des
Projekts. S8 bringt `CADisplayLink` und `NSScreen`, S12 den `NSSplitViewDelegate`,
S15 den Papierkorb, S16 und S17 die Vorschau. Wer dort einen SAFETY-Kommentar
schreibt, schaut hierher. Ein Kommentar, der die dokumentierte Bedingung nicht
nennt, macht aus der Prüfung "ist die Bedingung erfüllt?" die Prüfung "klingt die
Begründung plausibel?", und die zweite besteht auch ein falscher Aufruf.

## Was zu tun ist

- `ereignisse.rs:76-78`: die dokumentierte Bedingung ("der Rückgabewert des
  Blocks ist ein gültiger Zeiger oder null") aufnehmen und benennen, wo sie
  erfüllt wird.
- `tabelle.rs:606-607`: den zweiten Halbsatz durch die nullende schwache
  Eigenschaft ersetzen.
- Als Regel für die folgenden Schritte: der SAFETY-Kommentar nennt zuerst die
  Bedingung, die der Bindungspunkt in seinem `# Safety`-Abschnitt fordert, und
  danach, wodurch sie hier erfüllt ist. Der Platz dafür ist der Plan, Abschnitt
  `## Aufbau`, wo die `unsafe`-Grenze schon steht.

## Was in Ordnung ist

Alle übrigen `unsafe`-Stellen tragen die Bedingung, die die Bindung nennt:
die vier `msg_send![super(this), init]` (`anwendung.rs:73`, `fenster.rs:66`,
`tabelle.rs:208`, `tabelle.rs:497`), die vier `define_class!`-Köpfe, die wörtlich
der Vorlage aus `objc2-0.6.4/src/macros/define_class.rs:293-295` folgen,
`NSWindow::initWithContentRect_…` samt `setReleasedWhenClosed(false)`
(`fenster.rs:77-92`), `NSMenuItem::initWithTitle_action_keyEquivalent`
(`menue.rs:77-87`), `makeViewWithIdentifier_owner` (`tabelle.rs:553-555`),
`NSEvent::removeMonitor` (`ereignisse.rs:88-91`) und `ereignis.as_ref()`
(`ereignisse.rs:63-65`). Beim Zeitgeber (`tabelle.rs:379-394`) fehlt allein der
Hinweis, dass `addTimer_forMode` auf dem Faden der genannten Laufschleife stehen
muss; dass es der Hauptfaden ist, folgt aus `MainThreadOnly` an der Klasse und
ist damit maschinell gesichert.

**Aufgefallen bei:** der Prüfung von Schritt 6 und 7,
`circles/260802-0842-krk-mac-dateimanager-editor-git/reviews/260803-1536-coderev-appkit-durchstich-schritt-6-und-7.md`.
