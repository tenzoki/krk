umbruch_setzen ruft setContainerSize:, das seit 10.11 weich abgekuendigt ist

---

`Editorbereich::umbruch_setzen` setzt die Groesse des Textbehaelters ueber `setContainerSize:`. Der Kopf
des Systems sagt dazu: "Methods names with 'containerSize' are soft deprecated starting with OS X 10.11.
It will be officially deprecated in a future release … Use -size instead." Der Untergrenzen-Abschnitt
der Datei fuehrt `NSTextContainer` als 10.0 und sagt nichts von der Abkuendigung.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

## Wo es steht

- `crates/krk-ui/src/appkit/editor.rs:2864-2868`: `behaelter.setContainerSize(…)`.
- `AppKit.framework/Headers/NSTextContainer.h:119-121` (am 260826 gelesen):
  `// Methods names with "containerSize" are soft deprecated starting with OS X 10.11.`
  `@property NSSize containerSize; // Use -size instead`.

## Warum es zaehlt

KRK wird bis macOS 26 unterstuetzt; eine amtliche Abkuendigung kostet dann einen Absturz, den
`objc2` nicht abfaengt (CLAUDE.md, Technologiewahl). `size` traegt dieselbe Semantik und keinen Preis.

## Umfang

`krk-ui`, `appkit/editor.rs`.
