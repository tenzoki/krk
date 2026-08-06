# Welche Programmiersprache und welcher UI-Werkzeugkasten tragen KRK?

---
**Domain:** code
**Status:** implemented
**Filed by:** analyst
**Cross-references:** `circles/260802-0842-krk-mac-dateimanager-editor-git/analyses/260802-1134-sprache-und-ui-werkzeugkasten.md` (die Analyse, auf der dieser Datensatz beruht), `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1036_o_spec-navigator-geruest.md` Abschnitte C3, C8, C9 und `## Offen für den Planner`, `circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/260802-1036_a_leistungszusagen-navigator.md`, `shared/decisions/260802-0842_a_f-tasten-unter-macos-systembelegung.md`, `shared/decisions/260802-0842_o_code-sdk-fuer-ki-integration.md`

---

## Question

Der Spec beschreibt, was KRK leistet. Womit KRK gebaut wird, ist ausdrücklich offen, und `CLAUDE.md` verbietet jedem Agenten, die Wahl nebenbei zu treffen. Die Frage muss jetzt beantwortet werden, weil sie vor dem Plan steht: die Antwort bestimmt, wie ein Verzeichnis gelesen und dargestellt wird, wie Tastenereignisse abgefangen werden und welche Signierung die Anwendung braucht. Alle drei Punkte stehen im Spec unter `## Offen für den Planner`.

Die Wahl wird von zwei Abschnitten des Specs eingeschränkt. C8 nennt zehn Zeitzusagen, darunter 16 ms von Tastendruck bis sichtbarer Reaktion, 400 ms bis zur vollständigen Anzeige eines Ordners mit 10.000 Einträgen im warmen Zustand und 1000 ms Kaltstart. C3 verlangt, dass jede Tastenkombination zur Laufzeit umbelegbar ist und dass systemseitig vorbelegte Tasten erreicht werden. Alle Zusagen gelten auf dem vom Nutzer benannten Referenzgerät: MacBook Pro `MacBookPro15,1`, 15 Zoll, 2018, Intel Core i9 mit acht Kernen und 2,3 GHz, 16 GB Arbeitsspeicher, Bildschirm 2880×1800 bei 60 Hz, macOS 15.7.7.

## Options

1. **Swift mit AppKit** — die Dateiliste ist eine `NSTableView`, die Tastenbelegung ein `NSEvent`-Abgriff mit einer zur Laufzeit geladenen Tabelle.
   - Pro: `NSTableView` verwendet Zeilenansichten wieder, sodass die Anzeigearbeit mit der Zahl der sichtbaren Zeilen skaliert und nicht mit der Zahl der vorhandenen. Ein einziger Eintrittspunkt für Tastenereignisse erfüllt C3 ohne zweite Belegungsart und ohne Rückfallweg. Die für KRK entscheidende Verbesserung, die Schätzung der Zeilenhöhen, liegt seit macOS 13 vor und ist auf dem Referenzgerät verfügbar. Die beiden Vorbilder aus dem Spec, ForkLift und Marta, sind nach Herstellerangabe beide in Swift geschrieben.
   - Contra: die Oberfläche entsteht nicht deklarativ, was für die Nebenbereiche wie die Lesezeichenleiste und die Belegungsansicht mehr Code bedeutet als SwiftUI. Für die spätere Syntaxhervorhebung und die Git-Anbindung muss Swift über die C-Schnittstelle auf tree-sitter und libgit2 zugreifen, was Rust nativ hätte.

2. **Swift mit SwiftUI, einschließlich der Mischform mit eingebetteten AppKit-Ansichten** — die Oberfläche entsteht deklarativ; wo SwiftUI nicht trägt, wird eine AppKit-Ansicht über `NSViewRepresentable` eingebettet.
   - Pro: die Nebenbereiche der Oberfläche entstehen mit erheblich weniger Code. Apple entwickelt SwiftUI weiter, AppKit im Wesentlichen nicht mehr.
   - Contra: SwiftUIs `List` erzeugt auf macOS die Ansicht jeder Zeile, auch der nie sichtbaren; das ist im Apple-Entwicklerforum an einem Minimalbeispiel dokumentiert. Zu `Table`, der für KRK nötigen Form mit Spalten, liegt ein Bericht über eine 13 Sekunden lange Hängung bei rund 1.000 Zeilen auf einem Mac Studio mit M2 Max vor, also auf deutlich schnellerer Hardware als das Referenzgerät und bei einem Zehntel der zugesagten Einträge. Die Mischform verlagert die Dateiliste und die Tastenbehandlung ohnehin nach AppKit und fügt eine Grenze hinzu, über die Eingabefokus, Ersthelfer-Status und Auswahl abgestimmt werden müssen.

3. **Rust mit AppKit über die Bindungsbibliothek `objc2`** — dieselbe `NSTableView` und dieselben `NSEvent`-Abgriffe wie in Möglichkeit 1, angesprochen aus Rust.
   - Pro: erreicht die Zusagen über genau dieselben Systembausteine wie Möglichkeit 1. Die Bibliothek ist gepflegt, nicht experimentell: Version 0.6.4 vom 26. Februar 2026, Bindungen aus dem SDK von Xcode 16.4, 35,1 Millionen Bezüge in der jüngeren Zählung. Für die späteren Runden öffnet der Kandidat das Rust-Ökosystem, also tree-sitter für die Syntaxhervorhebung und `git2-rs` für Git, ohne Umweg über eine C-Schnittstelle.
   - Contra: es gibt keinen Oberflächenbau, jede Ansicht entsteht im Code. Jedes Objective-C-Protokoll, das KRK erfüllen muss, mindestens `NSTableViewDataSource`, `NSTableViewDelegate`, `NSApplicationDelegate` und die Fensterdelegierten, ist über das Makro `define_class!` von Hand zu deklarieren. Jeder AppKit-Aufruf ist ein unsicherer Fremdaufruf. Diesem dauerhaften Aufwand steht gegenüber Möglichkeit 1 kein Gewinn an Reaktionszeit gegenüber, weil beide dieselbe Tabelle bedienen.

## Constraints

Diese Randbedingungen muss jede Antwort erfüllen. Die ersten drei stammen aus dem Spec, die letzten drei sind Ergebnisse der Analyse und binden den Plan zusätzlich.

- Die zehn Zeitzusagen aus C8 gelten unverändert auf dem Referenzgerät von 2018. Der Vergleich hat keine von ihnen als unerreichbar erwiesen; der Ablösevorbehalt aus C8 greift nicht.
- C3 verlangt eine Laufzeit-Umbelegung jeder Kombination und Zugriff auf systemseitig vorbelegte Tasten. Eine Umgebung, die nur eine feste Menge von Kürzeln zulässt, trägt C3 nicht.
- Die Maxime "supersimpel" wirkt als Ausschlussgrund: eine Lösung, die eine Fähigkeit mit einer eigenen Sonderregel, einer eigenen Ausnahme und einem eigenen Rückfallweg erkauft, verfehlt sie.
- **KRK muss außerhalb der App-Sandbox ausgeliefert werden.** C9 verlangt Zugriff auf jeden Pfad des lokalen Dateisystems einschließlich `/Volumes`. In der Sandbox gibt es für den Schreibtisch keine passende Berechtigung; der vorgesehene Weg über ein `NSOpenPanel` mit sicherheitsbereichsbezogenem Lesezeichen wäre ein Dialog vor jedem neuen Ort. Der Zugriff auf geschützte Ordner läuft stattdessen über den Systemmechanismus für Transparenz, Zustimmung und Kontrolle, der am signierten Anwendungsbündel angreift und für jeden Werkzeugkasten gleich funktioniert.
- **Das minimale Zielsystem liegt bei macOS 15, solange das Gerät von 2018 die Abnahme trägt.** macOS 26 Tahoe ist die letzte Version mit Intel-Unterstützung, und sie unterstützt nur vier Modelle; das `MacBookPro15,1` ist keines davon. Jede Schnittstelle, die Apple ab macOS 26 einführt, steht KRK damit nicht zur Verfügung. Das Bauen für beide Architekturen bleibt möglich: Xcode 26 erzeugt universelle Binärdateien ab Werk, und das macOS-27-SDK unterstützt Rückwärtsziele bis macOS 12.
- **Eine Annahme aus C3 ist ungeprüft und muss vor der Implementierung geprüft werden.** Dass Fn+F3 bis Fn+F8 auf einem unveränderten Mac als gewöhnliche Tastenereignisse ankommen, während die nackten Funktionstasten vom System verbraucht werden, ließ sich nicht belegen. Die Prüfung ist ein Zehnzeiler und werkzeugunabhängig.

## Recommendation

**Wir empfehlen Möglichkeit 1, Swift mit AppKit, sofern das Gerät von 2018 das Abnahmegerät bleibt.**

Die Empfehlung entscheidet sich an einer einzigen Zusage. L3 verlangt einen Ordner mit 10.000 Einträgen vollständig in 400 ms, L10 schreibt das auf 100.000 Einträge in 4 s fort. `NSTableView` skaliert die Anzeigearbeit mit der Zahl der sichtbaren Zeilen; SwiftUIs Listen tun das auf macOS nachweislich nicht. Bei einer Bildschirmseite von etwa 50 Zeilen und 100.000 Einträgen liegen zwischen beiden drei Größenordnungen. Alle anderen Achsen trennen die Kandidaten weniger scharf.

Zwei weitere Punkte stützen die Empfehlung. C3 wird von einem einzigen `NSEvent`-Abgriff mit Nachschlagetabelle vollständig erfüllt, ohne Sonderregel und ohne Rückfallweg, was die Maxime "supersimpel" nicht nur zulässt, sondern belohnt. Und die Systemobergrenze des Referenzgeräts trifft AppKit nicht: die entscheidende Verbesserung liegt seit macOS 13 vor, während SwiftUIs Listenschwäche sich nach Aussage der Praxis erst mit Systemversionen bessert, die dieses Gerät nie erhalten wird.

**Zwei Bedingungen würden die Empfehlung kippen, und beide sind benennbar.** Stellt der Nutzer das Referenzgerät auf einen Mac um, der macOS 26 und spätere Versionen erhält, verliert das Argument aus der Systemobergrenze seine Schärfe; die dokumentierte Listenschwäche von SwiftUI bliebe davon unberührt. Verlangt eine spätere Runde aus eigenem Grund einen Rust-Kern, etwa für die Suche über mehrere Dateien aus einem späteren Circle, verschiebt sich die Rechnung zugunsten von Möglichkeit 3, und ein Wechsel nach Runde 1 wäre teuer.

Die Empfehlung ist eine Abwägung auf Basis dokumentierter Mechanismen, keine Messung. Zu L1 und L4 existiert für keinen Kandidaten eine veröffentlichte Vergleichsmessung auf einem Intel-Mac; die Analyse hält das ausdrücklich fest. Die Entscheidung liegt beim Nutzer.

---
Answered: Nutzerentscheidung am 260802-1150, festgehalten in `circles/260802-0842-krk-mac-dateimanager-editor-git/history/260802-1014-orchestrator-session.md` — **Möglichkeit 3, Rust mit AppKit über `objc2`.** Der Nutzer ist der Empfehlung nicht gefolgt und hat das begründet abgewogen bekommen: die Analyse wertet Rust vor allem deshalb ab, weil sein Vorteil (tree-sitter für die Syntaxhervorhebung, `git2-rs` für Git, beide ohne Umweg über eine C-Schnittstelle) erst in späteren Runden greife. Dieser Circle enthält Editor und Git-Anbindung jedoch ausdrücklich als spätere Runden derselben Arbeit, nicht als Möglichkeit. Der Vorteil fällt damit innerhalb des Circles an, und der vom Analysten genannte Kipp-Punkt ("verlangt eine spätere Runde aus eigenem Grund einen Rust-Kern, verschiebt sich die Rechnung zugunsten von Möglichkeit 3, und ein Wechsel nach Runde 1 wäre teuer") ist bereits erfüllt.

Zwei Randbedingungen der Analyse bleiben unverändert bindend: KRK wird außerhalb der App-Sandbox ausgeliefert, und die zehn Zeitzusagen werden auf dem Gerät von 2018 abgenommen (Nutzerentscheidung derselben Runde). Beide sind werkzeugunabhängig.

Ergänzend entschieden: KRK unterstützt macOS 26 und bleibt rückwärtskompatibel bis macOS 15. Das Mindest-Zielsystem ist macOS 15; Schnittstellen ab macOS 26 werden zur Laufzeit abgefragt und haben jeweils einen Ersatzweg. Damit ist die Systemobergrenze des Referenzgeräts kein Ausschluss, sondern Aufwand.

Zwei Annahmen sind auf Wunsch des Nutzers vor dem Plan zu prüfen: (a) ob Fn+F3 bis Fn+F8 als gewöhnliche Tastenereignisse ankommen, (b) wie `objc2` Schnittstellen behandelt, die es erst ab macOS 26 gibt. Punkt (b) hat die Analyse nicht betrachtet, weil er erst durch diese Wahl entsteht.
Implemented:
Deferred:
Superseded by:

---
Implemented: 7dc5ea6 bis 6b4fb2d — der Cargo-Workspace steht in Rust mit vier Mitgliedern, `krk-ui` bindet AppKit über `objc2`, `objc2-app-kit` und `objc2-foundation` und trägt seit `569e8e0` ein Fenster mit einer echten Dateiliste. Nachgeprüft am 260803-1330: `crates/krk-ui/Cargo.toml` führt die drei `objc2`-Kisten, `crates/krk-ui/src/appkit/` enthält fünf Module mit den vier `define_class!`-Deklarationen, und `cargo build --workspace` beendet mit 0.
