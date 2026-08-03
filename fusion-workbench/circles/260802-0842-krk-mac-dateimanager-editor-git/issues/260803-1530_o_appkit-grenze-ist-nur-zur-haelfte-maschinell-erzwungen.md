Die Grenze zum Modul `appkit` ist nur zur Hälfte maschinell erzwungen

---

Der Plan sagt in `## Aufbau` zu, jeder AppKit-Aufruf liege hinter einer sicheren Hülle unter `crates/krk-ui/src/appkit/`, und die Risikotabelle nennt diese Zusage "durchgesetzt über zwei Übersetzerregeln". Für den unsicheren Teil stimmt das. Für den übrigen nicht: ein großer Teil der `objc2`-Bindungen ist als sicher deklariert, und `#![deny(unsafe_code)]` sieht sie nicht.

---

**Nachgeprüft am 260803-1530** an den Quellen von `objc2-app-kit` 0.3.2 und `objc2-foundation` 0.3.2 unter `~/.cargo/registry/src/`.

Die Regel greift, wo `objc2` selbst `unsafe` verlangt:

| Bindung | Fundstelle | Form |
|---|---|---|
| `NSView.displayLinkWithTarget:selector:` | `objc2-app-kit-0.3.2/src/generated/NSView.rs:1714` | `pub unsafe fn` |
| `NSControl.setAction:` | `objc2-app-kit-0.3.2/src/generated/NSControl.rs:113` | `pub unsafe fn` |
| `NSControl.setTarget:` | `objc2-app-kit-0.3.2/src/generated/NSControl.rs:100` | `pub unsafe fn` |

Dazu jedes `define_class!`, jedes `unsafe impl` und jedes `msg_send!`.

Sie greift nicht bei diesen, und die Liste ist nicht erschöpfend:

| Bindung | Fundstelle | Form |
|---|---|---|
| `NSScreen.maximumFramesPerSecond` | `objc2-app-kit-0.3.2/src/generated/NSScreen.rs:170` | `pub fn` |
| `NSWindow.screen` | `objc2-app-kit-0.3.2/src/generated/NSWindow.rs:1352` | `pub fn` |
| `NSWindow.beginSheet:completionHandler:` | `objc2-app-kit-0.3.2/src/generated/NSWindow.rs:1587` | `pub fn` |
| `NSEvent.keyEventWithType:…` | `objc2-app-kit-0.3.2/src/generated/NSEvent.rs:1094` | `pub fn` |
| `NSApplication.postEvent:atStart:` | `objc2-app-kit-0.3.2/src/generated/NSApplication.rs:898` | `pub fn` |
| `NSFileManager.trashItemAtURL:…` | `objc2-foundation-0.3.2/src/generated/NSFileManager.rs:552` | `pub fn` |
| `NSFileManager.mountedVolumeURLs…` | `objc2-foundation-0.3.2/src/generated/NSFileManager.rs:282` | `pub fn` |
| `NSImage.initWithContentsOfURL:` | `objc2-app-kit-0.3.2/src/generated/NSImage.rs:205` | `pub fn` |

Jeder dieser Aufrufe übersetzt heute in `crates/krk-ui/src/messmodus.rs` oder in jeder anderen Datei außerhalb von `src/appkit/` anstandslos. Die Grenze wäre dort überschritten, ohne dass irgendetwas es meldet.

---

**Warum das jetzt auffällt.** Bei der Behebung von `260803-1345_c_dateiliste-von-s8-legt-objc2-code-ausserhalb-von-appkit-ab.md` waren sechs Schritte nachzuziehen. Drei davon (S8, S16, S17) hätten den Bau abgebrochen, drei (S13, S18, S21) nicht: sie hätten die Grenze still überschritten. Der Defekt zu S8 war also nur deshalb sichtbar, weil `CADisplayLink` zufällig auf der unsicheren Seite liegt. Die Zusage aus `## Aufbau` hängt damit an der Sorgfalt beim Schreiben der Dateilisten, und genau diese Sorgfalt hat in sechs Schritten nicht gereicht.

**Vorschlag.** Eine Prüfvorschrift in derselben Form wie die drei vorhandenen Attributprüfungen aus S2, S6 und S15:

```
grep -rEln '^[[:space:]]*use +objc2' crates/krk-ui/src --include='*.rs' \
  | grep -v '^crates/krk-ui/src/appkit/'
```

Das Kommando darf keine Zeile ausgeben. Es fängt beide Hälften, weil ein `objc2`-Aufruf ohne `use`-Zeile aus einer der `objc2`-Kisten nicht zustande kommt.

**Warum der `planner` das nicht selbst eingetragen hat.** Die Vorschrift gehört an den Schritt, der die Grenze anlegt, und das ist S6. S6 ist abgenommen und mit `[DONE]` vermerkt. Ein Abnahmekriterium eines abgenommenen Schrittes nachträglich zu verschärfen ist eine Nutzerentscheidung, keine Planungsentscheidung: entweder S6 wird gegen das neue Kriterium nachgeprüft, oder die Prüfung kommt als eigener Schritt, oder sie wandert in eine Bauprüfung außerhalb der Schrittabnahme.

**Dringlichkeit.** Kein Bau ist heute kaputt, und die Dateilisten sind seit dem Nachzug vom 260803-1530 richtig. Die Frage ist, ob die Grenze auch dann hält, wenn die nächste Dateiliste unaufmerksam geschrieben wird. Sie bindet den nächsten Schritt nicht.
