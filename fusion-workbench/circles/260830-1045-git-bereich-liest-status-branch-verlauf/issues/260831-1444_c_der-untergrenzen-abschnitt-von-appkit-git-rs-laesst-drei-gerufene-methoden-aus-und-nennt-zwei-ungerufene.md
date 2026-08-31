Der Untergrenzen-Abschnitt von `appkit/git.rs` lässt drei gerufene Methoden aus und nennt zwei ungerufene

---
C9.9 und Bedingung 8 der Runde 23 verlangen, dass jede angefasste Datei unter `crates/krk-ui/src/appkit/` den Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` mit **jeder** neu angesprochenen Klasse und Methode trägt. Er steht in `crates/krk-ui/src/appkit/git.rs:123-170`, und er ist weder vollständig noch genau.

Gerufen, aber nicht genannt:

- `deselectAll:` (`git.rs:760`), `NSTableView`
- `documentView` (`git.rs:814`), `NSScrollView`
- `NSTableColumn::initWithIdentifier:` (`git.rs:984`) — die Klasse steht in der Aufzählung, dieser Erzeuger nicht

Genannt, aber in dieser Datei nirgends gerufen: `window` und `makeFirstResponder:` (`git.rs:135`). Beide kommen im Rumpf nicht vor.

Keine der drei ausgelassenen Methoden liegt über macOS 15 — alle drei stehen seit 10.0 —, der Befund ist also kein Absturzrisiko. Er trifft die Vorkehrung selbst: `objc2` führt keine Verfügbarkeitsangaben mit sich, der Übersetzer hält die Untergrenze nicht, und dieser Abschnitt ist nach CLAUDE.md die einzige Gegenmaßnahme. Ein Abschnitt, der zwei nicht gerufene Namen führt und drei gerufene ausläßt, ist beim nächsten Lesen keine prüfbare Liste mehr.

**Abnahmetest:** die Menge der AppKit-Methoden, die `crates/krk-ui/src/appkit/git.rs` ruft, und die Menge der im Abschnitt genannten stimmen überein.

**Resolved:** 260831. Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` in `crates/krk-ui/src/appkit/git.rs` ist Methode für Methode gegen den Rumpf der Datei und gegen das SDK unter `xcrun --show-sdk-path` neu erhoben, nicht fortgeschrieben. `window` und `makeFirstResponder:` sind gestrichen: sie kommen im Rumpf nirgends vor. Aufgenommen sind `deselectAll:` (`NSTableView.h:338`), `documentView` (`NSScrollView.h:48`) und `initWithIdentifier:` (`NSTableColumn.h:31`) — keines trägt im Kopf des Systems ein `API_AVAILABLE`, alle drei stehen damit seit 10.0. Die Erhebung hat drei weitere Lücken gefunden, die der Datensatz nicht nennt und die mitgezogen sind: die zwei hier **gebauten** Protokollmethoden `numberOfRowsInTableView:` (`NSTableView.h:743`) und `tableViewSelectionDidChange:` (`:717`), das vierte bediente Protokoll `NSObjectProtocol` (die Datei nahm drei an und adoptiert vier), sowie `alloc` und `init`, die der Rumpf über `NSTableColumn::alloc`, `NSScrollView::alloc`, `NSView::alloc`, `NSTableView::alloc` und `msg_send![super(this), init]` ruft. Eine Zeilenangabe war falsch: `isFlipped` steht in diesem SDK an `NSView.h:141` und nicht an `:236`; jede übrige Zeilenangabe des Abschnitts ist einzeln am SDK nachgelesen und stimmt. **Keine der aufgenommenen Berührungen liegt über macOS 15**, die höchste bleibt `NSTableViewStyle` samt `setStyle:` seit 11.0. Die Zählangabe „Zehn Berührungen sind jünger als ihre Klasse" ist gefallen: die Liste darunter führt seit ihrer Niederschrift mehr als zehn, und eine Zahl neben ihrer Liste ist die Kopie, die driftet. An ihrer Stelle steht, dass die Liste die Vorkehrung und keine Zusammenfassung ist, wodurch sie am 260831 falsch war, und mit welchem `grep` die Kandidaten für die nächste Erhebung zu holen sind.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23 durch Abgleich der Methodenaufrufe der Datei gegen ihren Untergrenzen-Abschnitt. Die zwölf anderen von der Runde angefassten Dateien unter `appkit/` tragen den Abschnitt; die zwei begründeten Ausnahmen `koordinaten.rs` und `mod.rs` sind unverändert die zwei.
