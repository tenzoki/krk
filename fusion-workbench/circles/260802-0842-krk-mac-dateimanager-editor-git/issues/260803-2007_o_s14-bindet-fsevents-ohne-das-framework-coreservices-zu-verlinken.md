S14 bindet FSEvents, ohne das Framework CoreServices zu verlinken

---

S14 bindet `FSEventStream` von Hand als `unsafe extern "C"`, "wie
`getattrlistbulk`". Der Vergleich trägt nicht bis zum Binder: `getattrlistbulk`
liegt in `libSystem`, das jedes Rust-Programm auf macOS ohnehin bekommt,
`FSEventStreamCreate` liegt in `CoreServices`, das niemand verlinkt. Der
Schritt schlägt in der beschriebenen Form beim Binden fehl.

---

**Nachgeprüft am 260803-2007** gegen die Symboltabellen des SDK der Command
Line Tools:

```
$ grep -c FSEventStreamCreate \
    /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/CoreServices.tbd
1
$ grep -c FSEventStreamCreate \
    /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation.tbd
0
```

Die Dateiliste von S14 nennt `objc2-core-foundation` als Abhängigkeit, und
zwar zu Recht: die Parametertypen `CFArrayRef`, `CFStringRef` und
`CFRunLoopRef` kommen von dort, und die Kiste verlinkt `CoreFoundation`. Sie
verlinkt `CoreServices` nicht, und die zweite Zeile oben zeigt, dass
`CoreFoundation` das gesuchte Symbol nicht führt.

**Zwei Wege, und der erste ist der übliche.**

1. **Ein `#[link]`-Attribut am `extern`-Block** in
   `crates/krk-ui/src/appkit/fsevents.rs`:
   `#[link(name = "CoreServices", kind = "framework")]`. Es steht bei der
   Bindung, die es braucht, es entsteht keine weitere Datei, und der Bau
   bleibt ohne Bauskript.
2. Ein `crates/krk-ui/build.rs` mit
   `println!("cargo:rustc-link-lib=framework=CoreServices")`. Damit bekommt
   der Workspace ein Bauskript, das er heute nicht hat, und die Angabe steht
   weit weg von der Bindung.

**Empfehlung:** Weg 1. Der Plan hält `unsafe` und Fremdbindung ohnehin an
einer Stelle je Objekt zusammen, und das Attribut gehört zu dieser Stelle.

**Warum das ein eigener Eintrag ist.** Weg 2 zöge eine Datei nach sich, die in
keiner Dateiliste steht, und die Wahl zwischen den beiden ist eine
Entwurfsentscheidung über den Bauzuschnitt. Das geht über das Ergänzen einer
Dateiliste hinaus.

**Dringlichkeit.** Bindet S14 und keinen Schritt davor. Der Fehler zeigt sich
beim ersten Bau von S14 als nicht aufgelöstes Symbol, nicht als stiller
Fehler; die Meldung hier spart die Suche, sie verhindert keinen Schaden.

**Aufgefallen bei:** der Durchsicht der Dateilisten von S9 bis S23 unter der
erweiterten Regel, `issues/260803-1819_c_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`.
