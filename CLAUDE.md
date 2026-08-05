# KRK

**Language:** de

## Worum es geht

KRK ist eine native macOS-Anwendung zum Navigieren, Bearbeiten und Versionieren lokaler Dateien, in der Tradition von ForkLift und Norton Commander: Lesezeichen- und Geräteleiste links, zwei Dateifenster mit je mehreren Tabs in der Mitte, Vorschaufenster rechts, dazu ein Editor mit Rohansicht und Formatansicht und eine auf vier Operationen beschränkte Git-Anbindung.

Die vollständige Directive steht im Circle-Datensatz `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/_t_circle.md`, Abschnitt `## Directive`. Dieser Abschnitt hier ist die Kurzfassung, nicht die verbindliche Formulierung.

Pfade der Form `planning/…`, `decisions/…`, `analyses/…` und `issues/…` sind in dieser Datei relativ zu diesem Circle-Verzeichnis zu lesen.

## Maximen

Aus `idea.txt`: superschnell, supersimpel, Steuerung über die Tastatur bei zusätzlicher Maus- und Trackpad-Unterstützung.

"Superschnell" trägt in dieser Form keine Abnahmekriterien. Der Spec übersetzt die Maxime in Abschnitt `### C8: Messbare Geschwindigkeit` in zehn Zeitzusagen; das Referenzgerät, auf dem sie gemessen werden, steht im Datensatz `decisions/260802-1036_a_leistungszusagen-navigator.md`.

## Projektstand

Geprüft am 260806-0014. KRK entsteht in Rust mit AppKit über `objc2`. Der Cargo-Workspace steht, das Bündel `target/KRK.app` baut und signiert, und die Anwendung trägt den Navigator der Runde 1: Lesezeichen- und Geräteleiste, zwei Dateifenster mit Tabs, Vorschaufenster, Dateioperationen mit Fortschritt und Abbruch, Terminalaufruf im angezeigten Ordner, Belegungsansicht und ein Messmodus, der die Zeitzusagen aus C8 am laufenden Bündel abnimmt.

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
├── messungen/            # Messberichte: kopflose Strecke, Durchstich, Abnahmereihen
├── spikes/fn-tasten/     # Wegwerf-Prüfcode zur Fn-Tastenfrage, nicht weitergepflegt
├── README.md             # Bauen, Signieren, Versionspflege im Einzelnen
├── idea.txt              # der ursprüngliche Entwurf, Quelle der Directive
└── fusion-workbench/     # Circles, Entscheidungen, Issues, Historie
```

Den Ausführungsstand führt der Plan `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`: 34 der 36 Schritte tragen dort `[DONE]`, offen sind S6b (Abbruch beim fehlenden Tastenabgriff) und S23 (Auslieferungspaket). Die Abnahme-Messreihe aus S22 (`messungen/260805-2207-MacBookPro15-1-abnahme.txt` samt Begleittext) hält neun der zehn Zusagen in jeder Runde; L9 verfehlt den Anteil, die Frage dazu steht in `decisions/260806-0014_o_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`. Offene Defekte führt `issues/` (Marker `_o_`); verbindlich ist der Dateibestand, nicht diese Zeile.

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

## Technologiewahl

Getroffen am 260802-1150: **Rust mit AppKit über `objc2`**, außerhalb der App-Sandbox, Mindest-Zielsystem macOS 15 bei Unterstützung bis macOS 26. Der Datensatz ist `decisions/260802-1134_i_sprache-und-ui-werkzeugkasten.md`, die Gegenüberstellung der Kandidaten `analyses/260802-1134-sprache-und-ui-werkzeugkasten.md`, beide im aktiven Circle.

## Bindende Grundlage: die Entscheidungsdatensätze

Die Entscheidungsdatensätze sind die bindende Grundlage für jede Planung und jede Implementierung. **Verbindlich ist der Dateibestand, nicht diese Aufstellung.** Den Stand trägt der Marker im Dateinamen: `_o_` offen, `_a_` beantwortet aber noch nicht in Code umgesetzt, `_i_` umgesetzt, `_d_` zurückgestellt, `_s_` überholt. Wer den aktuellen Stand braucht, listet beide Speicher auf, nicht nur einen:

- `fusion-workbench/shared/decisions/` — projektweite Fragen
- `fusion-workbench/circles/260802-0842-krk-mac-dateimanager-editor-git/decisions/` — Fragen des aktiven Circles

Beantwortet oder umgesetzt sind am 260803-1321 sieben Fragen. Die Antwort steht jeweils in der Zeile `Answered:` ihres Datensatzes und ausformuliert im Spec oder im Plan; sie wird hier nicht wiederholt, damit sie nicht an zwei Stellen auseinanderläuft.

**Offen** sind fünf Fragen, drei projektweite und zwei des Circles:

- `260802-0842_o_git-verwerfen-bedeutung.md` — verwirft "revert" die Änderungen der Datei oder nimmt es einen Commit zurück?
- `260802-0842_o_editor-formatansicht-je-dateityp.md` — was zeigt die Formatansicht bei Text, bei Code und bei Markdown?
- `260802-0842_o_code-sdk-fuer-ki-integration.md` — welches Code-SDK trägt die spätere KI-Anbindung?
- `260802-1428_o_verfuegbarkeitspruefung-fuer-macos-26-schnittstellen-in-objc2.md` — wie steuert KRK aus Rust eine Schnittstelle an, die es erst ab macOS 26 gibt?
- `260802-1810_o_sortierung-ohne-sprachsensitive-kollation.md` — sortiert KRK Dateinamen sprachsensitiv, und wonach ordnet "Sortierung nach Typ"?

Die Sortierfrage bindet Schritt S12, die Verfügbarkeitsprüfung erst die Runde, die eine Schnittstelle über macOS 15 hinaus anspricht. Außerhalb des aktiven Circles liegen die KI-Anbindung, ein integrierter Browser, Datei- und Ordnervergleich, Suchen und Ersetzen über mehrere Dateien, Zugriff über Server-Protokolle sowie Git jenseits der vier genannten Operationen. Die Abgrenzung im Einzelnen steht im Circle-Datensatz.

## Sprache

Die Zeile `**Language:** de` oben deklariert Deutsch als Projektsprache. Sie steuert, welche Stilprofile unter `fusion-workbench/stilwerk/` gelten: `$FUSION_PLUGIN_ROOT/bin/fusion-rules` gibt daraufhin `fusion-workbench/stilwerk/chat-voice-de.yaml` und, für Langform-Agenten, `fusion-workbench/stilwerk/default-voice-de.yaml` aus. Ohne die Zeile fällt die Auflösung still auf `en` zurück. Das Format ist in `$FUSION_PLUGIN_ROOT/rules/fusion-workbench-conventions.md`, Abschnitt `## Project language`, festgelegt — Zeile nicht umformulieren, nicht verschieben in einen anderen Abschnitt und nicht entfernen.

Prosa in diesem Projekt ist deutsch. Bezeichner im Code, Commit-Messages und maschinenlesbare Artefakte folgen den üblichen englischen Konventionen.
