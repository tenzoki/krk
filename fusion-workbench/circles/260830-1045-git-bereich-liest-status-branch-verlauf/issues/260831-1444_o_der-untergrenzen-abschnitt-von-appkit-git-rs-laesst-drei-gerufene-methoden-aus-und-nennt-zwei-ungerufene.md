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

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
Gefunden in der Durchsicht der Runde 23 durch Abgleich der Methodenaufrufe der Datei gegen ihren Untergrenzen-Abschnitt. Die zwölf anderen von der Runde angefassten Dateien unter `appkit/` tragen den Abschnitt; die zwei begründeten Ausnahmen `koordinaten.rs` und `mod.rs` sind unverändert die zwei.
