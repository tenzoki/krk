Die Dateiliste von Schritt 8 legt `objc2`-Code außerhalb von `appkit/` ab und übersetzt damit nicht

---

Schritt 8 im Plan `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md` nennt in seiner Dateiliste `crates/krk-ui/src/messmodus.rs` und weist dieser Datei zwei Aufrufe an AppKit zu:

- `CADisplayLink` für die Messung von L1 (Tastendruck bis Ende des Zeichendurchgangs),
- `NSScreen.maximumFramesPerSecond` für die Bildwiederholrate im Bedingungskopf.

`messmodus.rs` liegt nicht unterhalb von `crates/krk-ui/src/appkit/`. Seit Commit `569e8e0` trägt `crates/krk-ui/src/main.rs` die Regel `#![deny(unsafe_code)]`, und die einzige Ausnahme steht am Kopf von `appkit/mod.rs`. **Der Schritt übersetzt in dieser Form nicht.**

Nachgeprüft am 260803-1345. `objc2` steht in `Cargo.toml` auf Version 0.6, und diese Fassung verlangt selbstgeschriebenes `unsafe` an jeder Berührung mit Objective-C. Die drei Formen kommen im bereits vorhandenen Code vor:

```
$ grep -rn 'unsafe { msg_send\|#\[unsafe(method\|unsafe impl' crates/krk-ui/src/appkit/ | head -4
crates/krk-ui/src/appkit/anwendung.rs:50:    unsafe impl NSObjectProtocol for Anwendungsdelegierter {}
crates/krk-ui/src/appkit/anwendung.rs:53:    unsafe impl NSApplicationDelegate for Anwendungsdelegierter {
crates/krk-ui/src/appkit/anwendung.rs:55:        #[unsafe(method(applicationDidFinishLaunching:))]
crates/krk-ui/src/appkit/anwendung.rs:73:        unsafe { msg_send![super(this), init] }
```

Ein `CADisplayLink`-Rückruf und ein Nachschlag auf `NSScreen` brauchen dieselben Formen. In `messmodus.rs` lässt `deny` den Bau daran scheitern.

---

**Warum das keine Nachlässigkeit des Plans ist.** Die Dateiliste von Schritt 8 wurde geschrieben, als `krk-ui` noch `#![warn(unsafe_code)]` trug. Unter `warn` wäre der Aufruf durchgegangen und hätte nur eine Meldung im Bauprotokoll erzeugt. Der Nutzerentscheid vom 260803, festgehalten in `decisions/260803-1208_i_unsafe-grenze-in-krk-ui-erzwungen-oder-beobachtet.md`, hat aus der Meldung einen Abbruch gemacht. Der Defekt ist die erste Folge dieses Entscheids, und er ist die gewollte: genau dieser Widerstand war der Zweck der Umstellung.

**Was zu tun ist.** Der `planner` teilt Schritt 8 entlang der Grenze auf, statt die Grenze aufzuweichen:

- Der `objc2`-Anteil gehört nach `crates/krk-ui/src/appkit/`, in eine eigene Datei. Das sind der `CADisplayLink`-Rückruf und der Nachschlag auf `NSScreen.maximumFramesPerSecond`, jeweils hinter einer sicheren Hülle.
- `crates/krk-ui/src/messmodus.rs` behält, was kein AppKit berührt: den Ablauf der Messung, die zwanzig Wiederholungen, die Berechnung des 95. Perzentils und den Bericht.

Diese Teilung ist keine Notlösung für den Defekt, sondern dieselbe, die der Abschnitt `## Aufbau` für das ganze Projekt vorschreibt: sichere Hüllen um jeden AppKit-Aufruf, `unsafe` an genau einer Stelle je Kiste.

**Dieselbe Prüfung steht für drei weitere Schritte aus.** Jede Dateiliste, die `objc2` außerhalb von `crates/krk-ui/src/appkit/` ansiedelt, trägt denselben Fehler. Zu prüfen sind:

- **Schritt 13**, Dateien unter `crates/krk-ui/src/kommandos/`,
- **Schritt 16 und Schritt 17**, Dateien unter `crates/krk-ui/src/blaetter/`.

Der `planner` prüft alle drei im selben Zug und zieht nach, was nachzuziehen ist. Ob dabei weitere Schritte betroffen sind, zeigt ein Durchgang durch die Dateilisten aller Schritte, die `krk-ui` anfassen.

**Dringlichkeit.** Schritt 8 ist der nächste umzusetzende Schritt und zugleich das Messgate. Der Defekt ist **vor** seiner Umsetzung zu beheben, sonst scheitert der `coder` am Bau statt an einer Messung, und das Gate liefert keine Aussage über den Technologieentscheid.

---
Resolved: Nachzug im Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md` vom 260803-1530.

**Schritt 8 ist entlang der Grenze geteilt, nicht die Grenze aufgeweicht.** Der `objc2`-Anteil liegt jetzt in `crates/krk-ui/src/appkit/bildtakt.rs`, geschnitten wie die fünf vorhandenen Module: nach dem, was AppKit als eigenständiges Objekt führt. Zwei sichere Hüllen. `Zeichenende` umschließt den `CADisplayLink` auf der Inhaltsansicht samt dem `define_class!`-Ziel, nimmt beim Einrichten eine gewöhnliche Rust-Senke und gibt den Takt beim Fallenlassen frei — dieselbe Form wie `Tastenabgriff` aus S7, der sich in seinem `Drop` bei AppKit abmeldet. `bildwiederholrate` schlägt über `NSWindow.screen()` auf `maximumFramesPerSecond` nach und liefert `None`, wenn das Fenster auf keinem Bildschirm steht, damit der Aufrufer nach der Regel aus S21 abbricht. `crates/krk-ui/src/messmodus.rs` behält den Ablauf der Messung, die zwanzig Wiederholungen, das 95. Perzentil und den Bericht; über die Grenze gehen zwei gewöhnliche Rust-Werte. Das Abnahmekriterium von S8 und seine fünf Zahlen sind unverändert.

**Fünf weitere Schritte waren betroffen, drei davon aus der Vermutung dieses Datensatzes und zwei nicht.** S13 legte das Blatt der Pfadeingabe nach `kommandos/`, S16 und S17 legten sechs Blätter nach `crates/krk-ui/src/blaetter/`; alle sieben liegen jetzt unter `appkit/blaetter/` hinter einer gemeinsamen Hülle, der Ablauf bleibt in `kommandos/operationen.rs`. Dazu kamen zwei, die dieser Datensatz nicht nannte: **S15** nannte den Papierkorb-Aufruf im Fließtext, aber keine Dateiliste eines Schrittes nannte die Datei dazu, sodass die injizierte Schnittstelle ohne Implementierung geblieben wäre (jetzt `appkit/papierkorb.rs`); **S21** legte die synthetischen Tastenereignisse nach `messmodus.rs` (jetzt `appkit/ereignisse.rs`, bei dem Modul, das seit S7 der eine Eintrittspunkt für Tastendrücke ist). Bei S12, S18 und S19 stand die AppKit-Arbeit ohne Zuordnung zwischen zwei Dateien; die Grenze ist dort jetzt ausgeschrieben.

**Sauber und unverändert** sind S1, S6, S7, S14, S20, S22 und S23. In der Gegenrichtung fand die Durchsicht nichts: jede der zwölf Dateien, die vor dem Nachzug neben der `mod.rs` unter `src/appkit/` standen, gehört dorthin. Die FSEvents-Bindung aus S14 ist zwar kein AppKit, aber eine C-Schnittstelle mit demselben unsicheren Fremdaufruf und liegt richtig; `## Aufbau` schreibt diese zweite Aufgabe des Moduls jetzt aus.

**Eine Feststellung dieses Datensatzes war zu weit gefasst.** Er nennt den Nachschlag auf `NSScreen` als zweiten Grund, aus dem S8 nicht übersetzt. Das trifft nicht zu: `NSWindow.screen` und `NSScreen.maximumFramesPerSecond` sind in `objc2-app-kit` 0.3.2 beide als `pub fn` geführt und hätten außerhalb von `appkit/` anstandslos übersetzt. Den Bau bricht allein der `CADisplayLink` ab. Der Nachschlag gehört trotzdem in die Hülle, weil die Grenze aus `## Aufbau` an jedem AppKit-Aufruf hängt und nicht an der Übersetzerregel. Daraus folgt ein eigenständiger Befund, der über diesen Defekt hinausgeht und als `issues/260803-1530_o_appkit-grenze-ist-nur-zur-haelfte-maschinell-erzwungen.md` gemeldet ist: drei der sechs Verstöße hätten den Bau nicht abgebrochen, die Zusage "sichere Hüllen um jeden AppKit-Aufruf" hat also nur zur Hälfte einen maschinellen Träger.
