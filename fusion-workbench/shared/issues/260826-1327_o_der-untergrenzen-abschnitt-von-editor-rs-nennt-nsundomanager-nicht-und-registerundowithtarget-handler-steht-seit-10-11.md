Der Untergrenzen-Abschnitt von editor.rs nennt NSUndoManager nicht, und registerUndoWithTarget:handler: steht seit 10.11

---

Der Abschnitt "Ab welchem macOS die angesprochenen Klassen stehen" zaehlt sieben Klassen plus `NSMenu`
und `NSEvent` auf und sagt "Fuenf Methoden sind juenger als ihre Klasse". `NSUndoManager` fehlt in der
Klassenliste, und seine Methode `registerUndoWithTarget:handler:` traegt im SDK
`API_AVAILABLE(macos(10.11))` — sie ist die sechste. Unter 15.0, also folgenlos; aber der Abschnitt ist
die einzige Gegenmassnahme gegen den Absturz, den `objc2` nicht abfaengt, und behauptet
Vollstaendigkeit.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/editor.rs:375-386`: die Klassenliste; `NSUndoManager` steht nicht darin,
  obwohl `:456` es importiert und `:2101-2115`, `:2056`, `:2067`, `:3244-3248` es ansprechen.
- `:414-420`: "Fuenf **Methoden** sind juenger als ihre Klasse" — genannt sind
  `setInlinePredictionType:`, `setMathExpressionCompletionType:`, `setWritingToolsBehavior:`,
  `NSTextView.textLayoutManager` und `setAllowsWritingToolsAffordance:`.
- SDK: `Foundation.framework/Headers/NSUndoManager.h:161`
  `- (void)registerUndoWithTarget:(id)target handler:(…)undoHandler API_AVAILABLE(macos(10.11), …)`,
  gerufen in `editor.rs:2115`.
- `:431-432`: "Die Proben unter `mod tests` sprechen daneben nichts an, was eine Verfuegbarkeitsfrage
  stellt." `enabledTextCheckingTypes` (`:5193`, `:5194`, `:5211`) traegt
  `API_AVAILABLE(macos(10.6))` (`AppKit.framework/Headers/NSTextView.h:466`).

Am 260826 in `$(xcrun --show-sdk-path)/System/Library/Frameworks/…/Headers/` nachgelesen.

## Warum es zaehlt

CLAUDE.md, Abschnitt Technologiewahl: die Angabe "ist die einzige Gegenmassnahme dieses Projekts gegen
den Absturz, den `objc2` nicht abfaengt, und eine falsche wird geglaubt" — so schon der eigene Kopf
`editor.rs:392-394` zur Berichtigung von `NSLayoutManager`. Dieselbe Sorte Befund an `anwendung.rs`:
`260813-1345_*_keywindow-und-isequal-stehen-nicht-im-untergrenzen-abschnitt-von-anwendung-rs.md`.

## Was zu tun waere

`NSUndoManager` in die Klassenliste, `registerUndoWithTarget:handler:` (10.11) als sechste Methode,
`enabledTextCheckingTypes` (10.6) beim Satz ueber die Proben.

## Umfang

`krk-ui`, `appkit/editor.rs`, Modulkopf.
