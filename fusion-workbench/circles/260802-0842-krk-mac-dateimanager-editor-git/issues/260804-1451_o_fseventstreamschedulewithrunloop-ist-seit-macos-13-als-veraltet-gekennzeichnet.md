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
