Die Untergrenzen-Angabe in zettel.rs nennt NSSegmentedControl seit macOS 10.0; es ist 10.3

---

Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` in
`crates/krk-ui/src/appkit/blaetter/zettel.rs:79` führt `NSSegmentedControl` in der Liste der
Klassen, die „seit macOS 10.0 zur Verfuegung" stehen. `NSSegmentedControl` ist mit macOS
10.3 hinzugekommen; `indexOfSelectedItem` daran trägt im SDK `API_AVAILABLE(macos(10.4))`,
was eine Einführung in 10.0 ausschließt.

Die beiden Angaben mit Zahl in derselben Liste sind dagegen am SDK nachgeprüft und richtig:
`setSegmentStyle:` seit 10.5 (`NSSegmentedControl.h:91`) und
`segmentedControlWithLabels:trackingMode:target:action:` seit 10.12
(`NSSegmentedControl.h:130`).

---

**Schwere:** niedrig. Keine Wirkung auf das Bündel: 10.3 liegt so weit unter dem Zielsystem
15.0 wie 10.0, und keine Berührung in dieser Datei braucht eine Prüfung zur Laufzeit.

**Warum es trotzdem aufgeschrieben ist.** Der Abschnitt ist in diesem Projekt die einzige
Gegenmaßnahme gegen einen Absturz aus einer zu jungen Methode — `objc2` führt keine
Verfügbarkeitsangaben mit sich, und der Übersetzer hält die Untergrenze nicht. Seine
Deckung ist einmal von 33 auf 5 Dateien abgesunken und von Hand wiederhergestellt worden
(`CLAUDE.md`, Technologiewahl). Ein Abschnitt, dessen Zahlen nicht stimmen, ist als
Gegenmaßnahme nur so gut, wie der nächste Leser ihm glaubt.

**Was zu tun ist.** `NSSegmentedControl` aus der 10.0-Liste herausnehmen und mit 10.3 zu
den beiden Berührungen mit eigener Zahl stellen.

**Kontext**

- Gefunden bei der Durchsicht von Turn 1, `reviews/260814-0908-coderev-turn-1-notizzettel.md`.
- Geprüft am SDK unter
  `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk`,
  Datei `System/Library/Frameworks/AppKit.framework/Headers/NSSegmentedControl.h`. Die
  Klassendeklaration selbst trägt dort kein `API_AVAILABLE` — die Einführung in 10.3 steht
  in Apples Dokumentation, nicht im Kopf.
