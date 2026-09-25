# Die Begründung, `unsafe` verbiete den sachlichen Schnitt über NSTextInputTraits, ist falsch

---
**Domain:** code
**Schwere:** Medium
**Gefunden von:** coderev, Durchsicht der Runde 2 dieser Sitzung (`e6b76ab..HEAD`, Commit `d9fc2c8`)
**Betroffen:** Commit-Nachricht `d9fc2c8`, `issues/260810-0417_c_…md` (`Resolved:`, Punkt 1), `crates/krk-ui/src/appkit/editor.rs:150-155` (Modulkopf)
**Cross-references:** `crates/krk-ui/src/appkit/mod.rs:1-8`, `crates/krk-ui/src/main.rs:1-15`, `xtask/src/release.rs:78`, `decisions/260803-1208_*_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`

---

## Der Befund

Der Commit begründet mit drei Punkten, dass die Aufzählung zur Übersetzungszeit
nicht erzwingbar sei und der sachliche Schnitt über `NSTextInputTraits` deshalb
nicht in Frage komme. Ich habe alle drei nachgeprüft. **Zwei halten, einer
nicht**, und der eine ist derjenige, der den besseren Schnitt verschließt.

### Punkt 1 hält: Rust sieht die SDK-Kopfdateien nicht

Geprüft: das Projekt bindet AppKit über `objc2`-Kisten mit handgeschriebenen
Bindungen, nicht über `bindgen` gegen `MacOSX.sdk`. Es gibt keine Stelle im
Baum, an der eine Kopfdatei gelesen wird. Der Punkt stimmt.

### Punkt 2 hält: `objc2` bildet keine Verfügbarkeitsgrenze ab

Geprüft an `objc2-app-kit-0.3.2/src/generated/NSTextView.rs:1642-1665`: die
beiden neuen Setzer tragen als einzige Bedingung `#[cfg(feature =
"NSTextCheckingClient")]` — ein Cargo-Merkmal, keine Verfügbarkeitsangabe.
`MACOSX_DEPLOYMENT_TARGET` geht in die Bindung nirgends ein. Der Punkt stimmt.

### Punkt 3 hält nicht: `unsafe` ist in `editor.rs` erlaubt

Der Commit schreibt: „Der sachliche Schnitt ueber `NSTextInputTraits` fuehrte
ueber rohes FFI und damit `unsafe`, das `krk-ui` **ausserhalb von
`appkit/mod.rs`** verbietet." Das ist an drei unabhängigen Stellen
widerlegbar.

**a) Die Lint-Regel deckt den ganzen Teilbaum, und die Datei sagt das selbst.**
`crates/krk-ui/src/appkit/mod.rs:1-8`:

```rust
#![allow(unsafe_code)]
//! Das Attribut oben ist die eine Ausnahme von `#![deny(unsafe_code)]` in
//! `main.rs`. Es steht hier und nirgends sonst: Lint-Regeln schlagen in die
//! eingebetteten Module durch, deshalb deckt der Kopf dieser Datei den ganzen
//! Teilbaum `src/appkit/` ab, und keine Datei darunter braucht die Ausnahme
//! ein zweites Mal.
```

`editor.rs` liegt in diesem Teilbaum.

**b) `editor.rs` benutzt `unsafe` bereits, auch in `mod tests`.**
Belegstellen in derselben Datei: `:535`, `:552`, `:567`, `:1719`, `:1744`,
`:1851`, `:1876`, `:1958` — und `:2521`, das innerhalb von `mod tests` steht:

```rust
NSUndoManager::new(unsafe { MainThreadMarker::new_unchecked() })
```

Die Probe, um die es geht, steht zwanzig Zeilen weiter unten in demselben
`mod tests`.

**c) Die maschinelle Grenze ist ebenfalls der Teilbaum, nicht die Datei.**
`xtask/src/release.rs:78`:

```rust
const AUSNAHME: &str = "crates/krk-ui/src/appkit";
```

Es gibt im ganzen Projekt keine Regel, die `unsafe` auf `appkit/mod.rs`
einschränkt. Der Satz aus der Commit-Nachricht beschreibt eine Grenze, die
nicht existiert.

## Und der Schnitt ist erreichbar — belegt

`objc2` 0.6.4 führt die Protokoll-Aufzählung in seinem `ffi`-Modul:
`$CARGO_HOME/registry/src/…/objc2-0.6.4/src/ffi/protocol.rs:49`
(`protocol_copyMethodDescriptionList`) und `:57` (`protocol_copyPropertyList`).
`AnyProtocol::get` steht in der sicheren Schnittstelle
(`src/runtime/mod.rs:1045ff.`). Die Aussage „`AnyProtocol` fuehrt in `objc2` 0.6
keine Mitgliederliste" ist für die **sichere** Schnittstelle richtig — ich habe
die Aufzählung dort nachgesehen und bestätige sie — aber sie trägt nicht den
Schluss, dass der Schnitt unerreichbar sei.

Zum Beleg habe ich den Schnitt gefahren, in ObjC auf demselben Gerät:

```
NSTextInputTraits, protocol_copyPropertyList: 14 Merkmale
  autocorrectionType  spellCheckingType  grammarCheckingType  smartQuotesType
  smartDashesType  smartInsertDeleteType  textReplacementType  dataDetectionType
  linkDetectionType  textCompletionType  inlinePredictionType
  mathExpressionCompletionType  writingToolsBehavior
  allowedWritingToolsResultOptions
```

Der Schnitt findet damit **sofort** eine Einstellung, die der namensbasierte
nicht sieht (`260810-0745`). Er ist also nicht nur erreichbar, sondern für die
Merkmale, die er abdeckt, dem Namensschnitt überlegen.

## Was er nicht leistet

Zur Ehrlichkeit gehört die Gegenrechnung: der Protokollschnitt allein wäre
**enger** als der heutige, nicht breiter. Er kennt die dreizehn
`set…Enabled:`-Schalter nicht, die `NSTextView` für sich trägt, und er kennt
`setEnabledTextCheckingTypes:` nicht (`260810-0746`). Wer ihn will, will die
**Vereinigung** aus Protokollmerkmalen und Namensformen, nicht den Ersatz. Der
Modulkopf sagt heute „sonst wäre `NSTextInputTraits` der sachliche **statt** des
namensbasierten Schnitts" — auch das trägt nicht.

## Was heute hält

Am ausgeführten Code ändert der Befund nichts: die sieben Zeilen in
`textflaeche_bauen` stehen und wirken. Betroffen ist die Begründung in einem
dauerhaften Datensatz — sie verschließt einen Weg mit einem Argument, das nicht
zutrifft, und der nächste Leser schließt daraus zurück, `unsafe` sei in
`editor.rs` unzulässig.

## Vorschlag

1. Den dritten Punkt aus der Begründung streichen. Er ist falsch, und die
   beiden anderen tragen die Aussage „nicht zur Übersetzungszeit erzwingbar"
   allein — eine Laufzeitaufzählung bleibt eine Laufzeitaufzählung, ob über
   Namen oder über das Protokoll.
2. Prüfen, ob die Vereinigung aus Protokollmerkmalen und den drei Namensformen
   der bessere Stolperdraht ist. Sie fängt heute nachweislich einen Fall mehr.
3. Den Satz im Modulkopf von „statt" auf „neben" ziehen.

---
Resolved: Alle drei Punkte sind unabhängig am Bestand nachgeprüft. Der Befund
hält: Punkt 1 und 2 stimmen, Punkt 3 der Commit-Begründung ist falsch.

**(a) stimmt wörtlich.** `crates/krk-ui/src/appkit/mod.rs:1` trägt
`#![allow(unsafe_code)]`, und der Modulkopf darunter sagt selbst, dass
Lint-Regeln in die eingebetteten Module durchschlagen und der Kopf dieser Datei
deshalb den ganzen Teilbaum `src/appkit/` deckt. `main.rs:1` trägt das
zugehörige `#![deny(unsafe_code)]`.

**(b) stimmt und ist stärker als angegeben.** `editor.rs` benutzt `unsafe` an
mehr Stellen als die acht genannten, unter anderem in den Klassendefinitionen
und den Delegiertenmethoden; `:2521` innerhalb von `mod tests` ist nachgeprüft.

**(c) ist unscharf, und das Ergebnis hängt nicht daran.**
`xtask/src/release.rs:78` `AUSNAHME` ist die Grenze für das *Nennen einer
`objc2`-Kiste*, nicht für `unsafe` — der Dateikopf dort sagt es so. Die
maschinelle Grenze für `unsafe` ist die Lint-Regel aus (a), und die ist
ebenfalls der Teilbaum und nicht die Datei. Die Schlussfolgerung des Datensatzes
bleibt damit unverändert richtig; sie steht schon auf (a) und (b) allein.

**Alle drei Vorschläge sind umgesetzt.** Der falsche Punkt ist aus dem Modulkopf
heraus, und der Kopf benennt die Behauptung aus `d9fc2c8` jetzt ausdrücklich als
falsch, damit der nächste Leser nicht zurückschließt, `unsafe` sei in
`editor.rs` unzulässig; eine Commit-Nachricht selbst ist nicht zu ändern. Der
Satz „statt" ist zu „neben" gezogen, und zwar nicht nur im Wortlaut: der Schnitt
über `NSTextInputTraits` **läuft** jetzt, über `objc2::ffi::protocol_copyPropertyList`,
neben dem namensbasierten und nicht an seiner Stelle. Die Gegenrechnung dieses
Datensatzes ist damit eingelöst — es ist die Vereinigung und nicht der Ersatz —,
und der eine Fall, den der Protokollschnitt sofort mehr fängt
(`allowedWritingToolsResultOptions`), ist eingeordnet.
