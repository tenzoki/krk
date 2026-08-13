Der Modulkopf von `titelzusatz.rs` lässt die eine Bedingung weg, die der SDK-Kopf an `Left` knüpft

---

Der Abschnitt „`Left` und nicht `Leading`" in `crates/krk-ui/src/appkit/titelzusatz.rs:54-68` gibt den SDK-Kopf genauer wieder als der Plan — und lässt dabei die einzige Verfügbarkeitsbedingung weg, die dort an dem Wert hängt, der tatsächlich gesetzt wird. Der Kopf sagt: `NSLayoutAttributeLeft` gilt nur „for applications linked on Mac OS 10.11 or later". Der Modulkopf zählt `Left` unter den unbedingt zulässigen Werten auf und heftet eine Versionsangabe nur an `Leading`/`Trailing` (10.12) und `Top` (10.13).

---

**Schwere:** niedrig. Auf dem Bauziel folgenlos, aber es steht in genau dem Abschnitt, den `CLAUDE.md` als die Gegenmaßnahme gegen fehlende Verfügbarkeitsangaben führt.

**Was im Baum steht** (`titelzusatz.rs:58-63`):

```
//! des Systems (`NSTitlebarAccessoryViewController.h:23-30`) laesst davon
//! allein `Bottom` (die Vorgabe), `Right` und `Left` zu, dazu `Leading` und
//! `Trailing` fuer Anwendungen ab 10.12 und `Top` ab 10.13, letzteres nur
//! zusammen mit `NSWindowStyleMaskFullSizeContentView`. Woertlich: "All other
//! values are currently invalid and will assert."
```

**Was im SDK steht.** `MacOSX.sdk/…/AppKit.framework/Headers/NSTitlebarAccessoryViewController.h:23`, am 260813 nachgelesen:

> The layoutAttribute defaults to NSLayoutAttributeBottom … NSLayoutAttributeRight is also supported … **For applications linked on Mac OS 10.11 or later, NSLayoutAttributeLeft is also supported**; placing the item on the left side of the window (adjacent and to the right of the close/minimize/maximize buttons). All other values are currently invalid and will assert.

`Left` steht also nicht neben `Bottom` und `Right`, sondern in derselben Reihe wie `Leading`, `Trailing` und `Top`: ein Wert mit einer Bedingung, nur mit einer niedrigeren Zahl.

**Folgenlos, und warum.** `.cargo/config.toml` setzt `MACOSX_DEPLOYMENT_TARGET=15.0`, die Bedingung ist also erfüllt. Der Wert selbst ist richtig gewählt: `steuerung.setLayoutAttribute(NSLayoutAttribute::Left)` (`titelzusatz.rs:192`), nicht `Leading`, und die Begründung daneben (`Leading` wechselt die Seite mit der Schreibrichtung) trägt.

**Die übrigen Angaben des Abschnitts sind am SDK geprüft und stimmen alle**, jede mit ihrer Zeilennummer:

| Angabe im Modulkopf | Im SDK |
|---|---|
| `NSTitlebarAccessoryViewController` seit 10.10, `.h:20` | `API_AVAILABLE(macos(10.10))` in Zeile 19, `@interface` in Zeile 20 |
| `addTitlebarAccessoryViewController:` seit 10.10, `NSWindow.h:323` | Zeile 323, `API_AVAILABLE(macos(10.10))` |
| `NSTextField::labelWithString:` seit 10.12, `NSTextField.h:93` | Zeile 93, `API_AVAILABLE(macos(10.12))` |
| `secondaryLabelColor` seit 10.10, `NSColor.h:202` | Zeile 202, `API_AVAILABLE(macos(10.10))` |
| `NSViewController` seit 10.5, `.h:49` | `API_AVAILABLE(macos(10.5))` in Zeile 49 |
| `setView:` ohne eigene Angabe, `.h:78` | Zeile 78, keine Angabe |
| „a non-wrapping, non-editable, non-selectable text field", `NSTextField.h:87-93` | wörtlich in Zeile 88 |
| „All other values are currently invalid and will assert." | wörtlich in Zeile 23 |
| Höchste Untergrenze der Datei: 10.12 | trifft zu |

**Was zu tun ist**

Einen Halbsatz nachtragen: `Left` ist zulässig für Anwendungen, die gegen 10.11 oder später gebunden sind, und KRK bindet gegen 15.0. Damit steht im Baum kein Satz mehr, den ein Blick ins SDK widerlegt — das ist die Begründung, mit der der Ausführer den Abschnitt überhaupt gegenüber dem Planwortlaut geweitet hat (`history/260813-1310-coder-strang-b-titelleiste.md`, Abschnitt „Drei Stellen, an denen ich vom Planwortlaut abgewichen bin").

**Kontext**

- Gefunden bei der Durchsicht von Turn 1 der Runde 8, Bereich `59b0a6c..21dbc59`.
- Berührt C6.4 („jede dort genannte Zahl ist am SDK nachgelesen") und die offene Frage `shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`: eine Angabe von Hand, die an einer Bedingung vorbeiliest, ist ein Beleg für deren dritte Stufe.

---

**Abgleich 260813-1345: zu Recht offen, unverändert.** Der Abschnitt
`crates/krk-ui/src/appkit/titelzusatz.rs:54-68` zählt `Left` weiter unter den unbedingt
zulässigen Werten; die Zeichenfolge `10.11` kommt in der Datei nicht vor. Die Sache selbst ist
richtig gewählt: `setLayoutAttribute(NSLayoutAttribute::Left)` steht in `:192`, und `Leading`
kommt im ganzen Baum nicht vor.

Beim Abgleich ist ein zweiter Fall derselben Sorte in derselben Gegend aufgefallen:
`keyWindow` und `isEqual:` fehlen im Untergrenzen-Abschnitt von `anwendung.rs`, obwohl A1 und A2
sie neu in die Datei gebracht haben. Abgelegt als
`260813-1345_o_keywindow-und-isequal-stehen-nicht-im-untergrenzen-abschnitt-von-anwendung-rs.md`.
Beide zusammen sind der zweite und dritte Beleg für die dritte Stufe der offenen Frage
`shared/decisions/260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`.
