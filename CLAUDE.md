# KRK

**Language:** de

## Worum es geht

KRK ist eine native macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien, in der Tradition von ForkLift und Norton Commander: Lesezeichen- und Geräteleiste links, zwei Dateifenster mit je mehreren Tabs in der Mitte, Vorschaufenster rechts, dazu ein Editor mit Rohansicht und Formatansicht und eine auf vier Operationen beschränkte Git-Anbindung.

Die vollständige Directive steht im Circle-Datensatz `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_*_circle.md`, Abschnitt `## Directive`. Dieser Abschnitt hier ist die Kurzfassung, nicht die verbindliche Formulierung.

Pfade der Form `planning/…`, `decisions/…`, `analyses/…` und `issues/…` sind in dieser Datei relativ zu diesem Circle-Verzeichnis zu lesen.

## Maximen

Aus `idea.txt`: superschnell, supersimpel, Steuerung über die Tastatur bei zusätzlicher Maus- und Trackpad-Unterstützung.

"Superschnell" trägt in dieser Form keine Abnahmekriterien. Der Spec übersetzt die Maxime in Abschnitt `### C8: Messbare Geschwindigkeit` in zehn Zeitzusagen; das Referenzgerät, auf dem sie gemessen werden, steht im Datensatz `decisions/260802-1036_a_leistungszusagen-navigator.md`.

## Projektstand

Geprüft am 260807-1011. KRK entsteht in Rust mit AppKit über `objc2`. Der Cargo-Workspace steht, das Bündel `target/KRK.app` baut und signiert, und die Anwendung trägt den Navigator der Runde 1: Lesezeichen- und Geräteleiste, zwei Dateifenster mit Tabs, Vorschaufenster, Dateioperationen mit Fortschritt und Abbruch, Terminalaufruf im angezeigten Ordner, Belegungsansicht und ein Messmodus, der die Zeitzusagen aus C8 am laufenden Bündel abnimmt.

```
krk/
├── Cargo.toml            # Workspace mit vier Mitgliedern, Version an einer Stelle
├── rust-toolchain.toml   # Rust 1.97.1, beide Mac-Architekturen
├── .cargo/config.toml    # MACOSX_DEPLOYMENT_TARGET=15.0, Alias `cargo xtask`
├── crates/krk-core/      # Kern ohne AppKit: Verzeichnisleser, Ordnermodell, Tastennormalisierung
├── crates/krk-ui/        # Binärziel `krk`, AppKit-Anteil unter src/appkit/
├── crates/krk-bench/     # Prüfordner-Erzeuger und kopflose Messstrecke
├── xtask/                # Bauwerkzeug: Bündel, Versionsersetzung, Signierung
├── resources/Info.plist  # Bündelbeschreibung mit Versionsplatzhalter
├── Makefile              # Hülle um dieselben Kommandos, setzt den PATH zu cargo selbst
├── messungen/            # Messberichte: kopflose Strecke, Durchstich, Abnahmereihen
├── spikes/fn-tasten/     # Wegwerf-Prüfcode zur Fn-Tastenfrage, nicht weitergepflegt
├── README.md             # Bauen, Signieren, Versionspflege im Einzelnen
├── idea.txt              # der ursprüngliche Entwurf, Quelle der Directive
└── fusion-workbench/     # Circles, Entscheidungen, Issues, Historie
```

Den Ausführungsstand führt der Plan `planning/260802-1428_*_plan-navigator-geruest-runde-1.md`: **alle 38 Schritte tragen `[DONE]`**. Die Abnahme-Messreihe aus S22 (`messungen/260805-2207-MacBookPro15-1-abnahme.txt` samt Begleittext) hält alle zehn Zusagen, seit der Nutzer L9 am 260807 neu gefasst hat: während einer laufenden Kopie erreicht jede Eingabe spätestens das zweite Bild, mindestens 85 Prozent das erste. **Die Runde 1 ist am 260807-1035 als beschränkter Abschluss geschlossen**, Plan und Spec auf `_c_`, der Circle-Datensatz auf `_b_`. Beschränkt heißt: sieben der zehn Zusagen stehen auf der Messreihe vom 260805, und drei Commits danach haben Wege berührt, die sie messen. Der frische Abnahmelauf am gebauten Bündel steht damit aus und ist die erste Nachholarbeit der nächsten Runde; die `## Closure note` des Circle-Datensatzes schreibt es aus. Offene Defekte führt `issues/` (Marker `_o_`); verbindlich ist der Dateibestand, nicht diese Zeile.

`krk-core` und `krk-ui` tragen beide `#![deny(unsafe_code)]`; die Ausnahme `#![allow(unsafe_code)]` steht nur in `krk-core/src/verzeichnis/sys.rs` und `krk-ui/src/appkit/mod.rs`. Der Bau erzwingt diese Grenze.

## Bauen und prüfen

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets
cargo fmt --all --check
cargo xtask bundle          # baut und signiert target/KRK.app im Profil release
```

**`cargo` steht auf diesem Gerät nicht auf dem Standard-PATH.** Es liegt unter `$HOME/.cargo/bin`. Jeder Aufruf braucht deshalb den vollen Pfad oder ein vorangestelltes `export PATH="$HOME/.cargo/bin:$PATH"`.

Das `Makefile` im Projektwurzelverzeichnis nimmt einem genau das ab und ist eine Hülle um dieselben Kommandos, kein zweites Bauwerkzeug. `make help` listet alle Ziele; `make check` fährt die vier Abnahmekommandos in einem Zug, `make bundle` und `make run` bauen und starten, `make menue` und `make tasten` geben die beiden Protokollmodi aus, `make fixture`, `make messen` und `make durchstich` bedienen die Messstrecke. Wer lieber `cargo` tippt, verliert nichts.

`cargo xtask` ist kein eingebautes Kommando, sondern der Alias aus `.cargo/config.toml`. Der Bündelbau **verlangt eine Signaturidentität**, sucht sie in drei Stufen und bricht ohne Bündel ab, wenn keine greift; auf eine Ad-hoc-Signatur weicht er nicht aus. Die drei Stufen, das Anlegen einer Entwicklungsidentität, der Fehler `errSecInternalComponent` und die Versionspflege stehen in `README.md`.

## Was man nicht sieht, wenn man es nicht weiß

Fünf Eigenschaften, die jede von ihnen schon einmal eine Sitzung gekostet haben.

**Der Abnahmelauf verlangt KRK im Vordergrund.** Aus dem Hintergrund gestartet weist die Wirkungsbereichs-Prüfung jeden fokusgebundenen Befehl ab, und die Messstrecke meldet `NICHT_IM_VORDERGRUND` statt Zahlen. Aus einem Terminalfenster im Vordergrund läuft sie durch. Kein Agent kann sie deshalb fahren; das ist Nutzerarbeit. Die Frage dazu ist offen (`decisions/260806-1303_*_wie-kommt-krk-fuer-den-abnahmelauf-in-den-vordergrund.md`). Synthetische Tastendrücke gehören in KRKs eigene Ereignisschlange über `postEvent:atStart:` und nicht über `osascript`.

**Der Messplatz liegt unter `~/Library/Caches/krk-messplatz`**, nicht unter `/tmp`. Prüfordner einzelner Testläufe gehören dagegen nicht dorthin: sie tragen Prozesskennung und Laufnummer und räumen sich in `Drop` selbst auf, siehe `Pruefordner` in `krk-core/tests/verzeichnis.rs`.

**Drei Fallunterscheidungen sind vollständig und haben keinen Auffangzweig.** Das ist Absicht: eine neue Variante hält den Bau an und erzwingt eine bewusste Einordnung. Jedes neue Kommando braucht eine Zeile in `Kommando::wirkungsbereich` (`krk-core/src/tasten/belegung.rs`) und in `bereich_des_kommandos` (`krk-ui/src/belegungsmodell.rs`); jede neue Operationsart eine in `schiebt_auffrischung_auf` (`krk-ui/src/auffrischung.rs`).

**Der Sortierschlüssel entsteht einmal beim Lesen** und trägt die Kollation als Bytefolge. Das ist die Voraussetzung dafür, dass L3 und L10 halten, und darf nicht in einen paarweisen Vergleich zurückfallen.

**Ein Lesevorgang leert sein Ordnermodell nicht vorab**, sondern ersetzt es mit dem ersten gelieferten Stapel (`Ordnermodell::lesevorgang_beginnen`). Wer in dieser Spanne den Bestand befragt, sieht den **alten** Ordner. Wer eine Auswahl setzen will, geht deshalb über `Tabliste::auswahl_auf_namen`: es fragt `liest()` zuerst und merkt den Namen vor, statt ihn im alten Bestand zu finden.

## Technologiewahl

Getroffen am 260802-1150: **Rust mit AppKit über `objc2`**, außerhalb der App-Sandbox, Mindest-Zielsystem macOS 15 bei Unterstützung bis macOS 26. Der Datensatz ist `decisions/260802-1134_i_sprache-und-ui-werkzeugkasten.md`, die Gegenüberstellung der Kandidaten `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md`, beide im aktiven Circle.

## Bindende Grundlage: die Entscheidungsdatensätze

Die Entscheidungsdatensätze sind die bindende Grundlage für jede Planung und jede Implementierung. **Verbindlich ist der Dateibestand, nicht diese Aufstellung.** Den Stand trägt der Marker im Dateinamen: `_o_` offen, `_a_` beantwortet aber noch nicht in Code umgesetzt, `_i_` umgesetzt, `_d_` zurückgestellt, `_s_` überholt. Wer den aktuellen Stand braucht, listet beide Speicher auf, nicht nur einen:

- `fusion-workbench/shared/decisions/` — projektweite Fragen
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/` — Fragen des aktiven Circles

Die Antwort steht jeweils in der Zeile `Answered:` ihres Datensatzes und ausformuliert im Spec oder im Plan; sie wird hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderläuft. Die Aufstellung der offenen Fragen stand hier bis zum 260807 namentlich und ist zweimal in vier Tagen veraltet — deshalb steht sie nicht mehr hier. Wer den Stand braucht, listet beide Speicher auf:

```sh
find fusion-workbench/shared/decisions fusion-workbench/circles/*/decisions -maxdepth 1 -name '*_o_*.md'
```

**Keine offene Frage hält einen Planschritt auf; alle binden künftige Arbeit.** Die Namen liefert das `find` drei Zeilen darüber — eine Zahl an dieser Stelle veraltet auf demselben Weg wie die Aufstellung, die hier bis zum 260807 stand.

Außerhalb des aktiven Circles liegen die KI-Anbindung, ein integrierter Browser, Datei- und Ordnervergleich, Suchen und Ersetzen über mehrere Dateien, Zugriff über Server-Protokolle sowie Git jenseits der vier genannten Operationen. Die Abgrenzung im Einzelnen steht im Circle-Datensatz.

## Sprache

Die Zeile `**Language:** de` oben deklariert Deutsch als Projektsprache. Sie steuert, welche Stilprofile unter `fusion-workbench/stilwerk/` gelten: `$FUSION_PLUGIN_ROOT/bin/fusion-rules` gibt daraufhin `fusion-workbench/stilwerk/chat-voice-de.yaml` und, für Langform-Agenten, `fusion-workbench/stilwerk/default-voice-de.yaml` aus. Ohne die Zeile fällt die Auflösung still auf `en` zurück. Das Format ist in `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, festgelegt — Zeile nicht umformulieren, nicht verschieben in einen anderen Abschnitt und nicht entfernen.

Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen.
