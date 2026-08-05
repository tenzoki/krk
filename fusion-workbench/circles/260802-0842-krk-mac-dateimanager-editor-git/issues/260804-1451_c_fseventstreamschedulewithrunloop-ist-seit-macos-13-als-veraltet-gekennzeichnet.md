`FSEventStreamScheduleWithRunLoop` ist seit macOS 13 als veraltet gekennzeichnet

---

S14 hängt den `FSEventStream` über `FSEventStreamScheduleWithRunLoop` in die
Laufschleife des Hauptfadens. Der Kopf des Systems führt den Aufruf als
veraltet: `API_DEPRECATED("Use FSEventStreamSetDispatchQueue instead.",
macos(10.5, 13.0), ios(6.0,16.0))`, nachgelesen am 260804 in
`/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Frameworks/FSEvents.framework/Headers/FSEvents.h:1154`.
KRK läuft auf macOS 26.

---

**Warum es trotzdem so gebaut ist.** Der Plan nennt `CFRunLoopRef` ausdrücklich
als einen der drei Parametertypen, die aus `objc2-core-foundation` kommen
(`### 14.`, Dateiliste). `CFRunLoopRef` tritt in der ganzen Schnittstelle nur
in dieser einen Funktion auf. Der Plan meint also die Laufschleifen-Form, und
der `coder` ist ihr gefolgt, statt still auf etwas anderes auszuweichen.

**Was daran nicht stört.** Rust sieht Apples Veralterungsvermerk nicht: die
Bindung ist von Hand geschrieben, der Übersetzer warnt nicht, und der Aufruf
funktioniert. Am 260804 im laufenden Bündel geprüft: eine im Terminal angelegte
Datei erschien innerhalb einer halben Sekunde im Dateifenster.

**Was daran stört.** Ein Aufruf, den Apple seit drei Hauptversionen als
abgelöst führt, ist eine Zusage auf Zeit. Der Ersatz
`FSEventStreamSetDispatchQueue(strom, dispatch_get_main_queue())` liefert
dasselbe Verhalten, weil die Hauptwarteschlange auf dem Hauptfaden abgearbeitet
wird. Er kostet eine zusätzliche Bindung: `dispatch_get_main_queue()` ist im
Kopf des Systems eine `static inline`-Funktion und keine Ausfuhr, der Weg
dorthin ist das Symbol `_dispatch_main_q` aus `libSystem`.

**Zu entscheiden ist**, ob S14 bei der Laufschleifen-Form bleibt oder auf die
Warteschlangen-Form wechselt. Die Änderung beträfe drei Zeilen in
`crates/krk-ui/src/appkit/fsevents.rs` und nähme dem Modul die beiden
Verwendungen von `CFRunLoop` und `CFRunLoopMode`; die Merkmalsliste von
`objc2-core-foundation` in `Cargo.toml` verlöre dann `CFRunLoop`.

**Aufgefallen bei:** der Umsetzung von S14 am 260804,
`history/260804-1451-s14-dateisystem-beobachtung-und-datentraegerwechsel.md`.

---
Resolved: Gewechselt auf die Warteschlangen-Form. `crates/krk-ui/src/appkit/fsevents.rs` bindet jetzt `FSEventStreamSetDispatchQueue` statt `FSEventStreamScheduleWithRunLoop` und teilt den Strom `DispatchQueue::main()` zu.

**Die zusätzliche Bindung, die der Datensatz erwartet, fällt weg.** Er rechnet damit, dass `dispatch_get_main_queue()` als `static inline`-Funktion des Systemkopfs den Umweg über das Symbol `_dispatch_main_q` verlangt. Diesen Umweg geht die Kiste `dispatch2` bereits (`dispatch2-0.3.1/src/queue.rs:108`, `DispatchQueue::main()` liefert `&_dispatch_main_q`), und KRK führt sie seit Schritt 16 für den Weckruf des Vermittlerfadens. Der Wechsel kostet damit eine `use`-Zeile und keine neue Bindung.

**Zwei Verwendungen sind entfallen**, wie der Datensatz vorhersagt: `CFRunLoop` und `CFRunLoopMode`, dazu das Fremdsymbol `kCFRunLoopCommonModes`. `grep -rn CFRunLoop crates/ --include='*.rs'` findet nichts mehr.

**Das Merkmal `CFRunLoop` in `Cargo.toml` bleibt vorerst stehen.** Die Datei liegt im Projektwurzelverzeichnis und nicht unter `crates/`; dieser Durchgang war auf `crates/` begrenzt. Es ist jetzt unbenutzt und kostet Übersetzungszeit, sonst nichts. Eigener Datensatz: `issues/260805-0905_o_das-merkmal-cfrunloop-in-cargo-toml-hat-keinen-nutzer-mehr.md`.

**Die Überlegung zu den Laufschleifen-Modi entfällt, statt übergangen zu werden.** Die alte Form brauchte ausdrücklich die gemeinsamen Modi, weil der gewöhnliche ruht, solange der Nutzer blättert oder ein Menü offen hält. Eine Warteschlange kennt diese Unterscheidung nicht; die Hauptwarteschlange wird in beiden Fällen abgearbeitet. Der Modulkopf schreibt das aus.

**Nachgemessen am laufenden, signierten Bündel am 260805-0901.** Ein Tab auf `/tmp/krk-fsevents-probe`, dann eine Datei im Terminal angelegt:

```
vorher:  eins.txt zwei.txt
drei.txt sichtbar nach 0.65 s
nachher: drei.txt eins.txt zwei.txt
```

Abgelesen über die Bedienungshilfen an der Dateiliste selbst. 0,65 s liegt innerhalb der Sekunde, die das Abnahmekriterium von S14 zusagt, und entspricht der Sammelverzögerung von 300 ms plus dem Lesevorgang plus der Abtastspanne des Messkommandos. Der Prüfordner unter `/tmp` ist wieder entfernt.

Geprüft am 260805-0901: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0.
