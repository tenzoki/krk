# Die Aufzählung sieht nur die Klasse selbst und nicht ihre Oberklassen

---
**Domain:** code
**Schwere:** Low
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** `crates/krk-ui/src/appkit/editor.rs:2784-2792` (`keine_unbekannte_einstellung_steht_an_der_textflaeche`, der `instance_methods()`-Aufruf), Modulkopf `:147-168`
**Cross-references:** `issues/260810-0417_c_die-laufzeitprobe-bindet-den-bau-an-die-macos-version-des-pruefenden-geraets.md`, `issues/260810-0746_o_es-gibt-eine-dritte-tuer-und-sie-liegt-ausserhalb-aller-drei-namensformen.md`

---

## Der Befund

Die Probe zählt über `AnyClass::get(c"NSTextView")` und `instance_methods()`
auf. `instance_methods()` ist in `objc2` 0.6.4 ein direkter Aufruf von
`class_copyMethodList` (`src/runtime/mod.rs:889`), und `class_copyMethodList`
liefert **nur die Methoden der Klasse selbst**, nicht die ererbten.

Der Modulkopf führt drei Grenzen der Probe auf — prüfendes Gerät, Namensform,
Richtung. Diese vierte steht nicht dabei. Eine Einstellung, die Apple statt an
`NSTextView` an `NSText`, `NSView` oder `NSResponder` legt, fällt aus der
Aufzählung heraus, ohne dass eine Zusicherung anspricht.

Dass die Vererbungsstufen bereits Selektoren derselben Namensformen tragen,
ist gemessen (macOS 15.7.7, Build 24G720, `class_copyMethodList` je Oberklasse):

```
NSView:      setFocusRingType:  setGesturesEnabled:
NSResponder: setAccessibilityContainerType:  setAccessibilityEnabled:
             setAccessibilityRulerMarkerType:
```

Fünf Treffer der drei Formen, keiner davon in `EINSTELLUNGEN`, und die Probe
läuft grün. Keiner der fünf fasst den Textspeicher an — die Lücke ist heute
folgenlos. Sie zeigt aber, dass die Aufzählung an dieser Stelle stumm
danebengreift, statt anzuhalten.

Der Fall ist zugleich der Beleg dafür, dass die andere Richtung
(`eingeordnet \ getragen`) ein Hinweis bleiben muss: verschöbe Apple eine
heute geführte Einstellung nur eine Ebene nach oben, stünde sie in beiden
Differenzmengen falsch.

## Was heute hält

Nichts ist gebrochen. Alle sechsundzwanzig geführten Einstellungen liegen heute
an `NSTextView` selbst — nachgezählt, `class_copyMethodList(NSTextView)` liefert
in den drei Formen genau sechsundzwanzig, und sie decken sich Zeile für Zeile
mit `EINSTELLUNGEN`.

## Vorschlag

Die Aufzählung über die Vererbungskette bis `NSResponder` (oder `NSObject`)
laufen lassen und die dabei auftauchenden Selektoren mit einordnen — das sind
heute fünf zusätzliche Zeilen `Geduldet`. Wer den Aufwand nicht will, nimmt die
Grenze als vierte in die Liste im Modulkopf auf; sie ist dort die einzige, die
sich ohne Weiteres schließen ließe.
