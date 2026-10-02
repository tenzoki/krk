# Die Untergrenzen-Abschnitte nannten 196 hereingeholte Namen nicht, in 27 von 42 AppKit-Dateien

---
Der Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` stand am 260907 in
jeder Datei unter `crates/krk-ui/src/appkit/`, die eine Frameworkbindung anspricht — die
Deckung war also vollständig. **Vollständig war er nicht.** Die Erhebung für den Prüflauf
K16 hat 196 Namen gefunden, die eine `use`-Zeile auf oberster Ebene aus einer
`objc2_`-Kiste hereinholt, ohne dass der Abschnitt der Datei sie nennt; 66 verschiedene
Namen, verteilt über 27 der 42 Dateien.

---

**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md`,
`260811-1648_*_die-untergrenzen-angabe-im-modulkopf-steht-in-sieben-von-32-appkit-modulen.md`

## Was gemessen wurde

Erhoben mit dem Vergleich, den die Probe
`jeder_frameworkimport_steht_namentlich_im_untergrenzen_abschnitt`
(`crates/krk-core/tests/baum.rs`) seither fährt: jeder Name, den eine `use`-Zeile auf
oberster Ebene aus einer Kiste mit dem Namensanfang `objc2_` hereinholt, gegen den Text
unter der Überschrift im Modulkopf.

Die sechs häufigsten machten 100 der 196 Fundstellen aus und sind harmlos —
`MainThreadMarker` (24), `NSObjectProtocol` (17), `NSRect` (17), `NSPoint` (16), `NSSize`
(14), `ns_string` (12): Rust-Werkzeug der Kiste und C-Strukturen ohne eigene
Verfügbarkeitsangabe.

**Die übrigen waren es nicht.** `nummernspalte.rs` nannte fünf Klassen und ließ neun
weitere aus, darunter `NSColor`, `NSFont`, `NSScrollView`, `NSTextView` und
`NSNotificationCenter`. `editor.rs` ließ `NSDate`, `NSRunLoop`, `NSString` und
`NSNotification` aus, `anwendung.rs` `NSMenuItem` und `NSTextView`. Dazu kamen
Aufzählungen mit echter eigener Angabe (`NSSegmentDistribution` seit 10.13,
`NSSegmentStyle` und `NSImageScaling` seit 10.5) und Konstanten (`NSRunLoopCommonModes`
seit 10.5).

Kein ausgelassener Name liegt über dem Zielsystem macOS 15; die Lücke war eine des
Nachweises und nicht ein Absturz, der schon eingebaut wäre.

## Abnahmetest

`cargo test -p krk-core --test baum` läuft grün, und die genannte Probe wird rot, sobald
eine `use`-Zeile einen Namen hereinholt, den der Abschnitt der Datei nicht nennt.

---
Resolved: Alle 196 Stellen sind in den 27 Modulköpfen nachgetragen, jede Angabe am
Xcode-SDK gelesen und nicht aus einer Liste übernommen; die zwei Proben in
`crates/krk-core/tests/baum.rs` halten den Bestand seither. Die Aufzählungen sind an ihrer
schließenden Klammer gelesen und nicht an der nächsten, so wie
`260811-2050_*_wird-die-untergrenzen-angabe-pruefbar-gemacht.md` es unter `## Constraints`
verlangt; `NSWindowStyleMask` und `NSBackingStoreType` in `fenster.rs` sind der Fall, den
jener Abschnitt beim Namen nennt.
