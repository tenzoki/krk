# Schritt 1: Cargo-Workspace und Bauzuschnitt

**Datum:** 260802-1741
**Agent:** coder
**Status:** Complete
**Plan:** `circles/260802-0842-krk-mac-dateimanager-editor-git/planning/260802-1428_o_plan-navigator-geruest-runde-1.md`, Abschnitt `## Implementierungsschritte`, Schritt 1
**Commit:** noch keiner. Der Orchestrator committet.

---

## Was entstanden ist

Die erste Zeile Produktcode des Projekts. Vorher gab es keine `Cargo.toml`, kein
Bauskript und keine Tests; wiederverwendet wurde nichts, und `spikes/fn-tasten/`
blieb unberührt.

Angelegt ist ein Cargo-Workspace mit vier Mitgliedern, drei davon unter
`crates/`, das Bauwerkzeug daneben:

```
krk/
├── Cargo.toml               # Workspace, resolver 3, gemeinsame Paketangaben
├── Cargo.lock               # versioniert, weil der Workspace Binaerprogramme baut
├── rust-toolchain.toml      # 1.97.1 festgeschrieben, beide Ziele, rustfmt + clippy
├── rustfmt.toml             # edition 2024
├── .cargo/config.toml       # MACOSX_DEPLOYMENT_TARGET = "15.0", Alias `cargo xtask`
├── crates/
│   ├── krk-core/            # #![deny(unsafe_code)], serde + toml
│   ├── krk-ui/              # #![warn(unsafe_code)], objc2 + app-kit + foundation
│   └── krk-bench/           # leerer Rumpf
└── xtask/                   # leerer Rumpf
```

`krk-ui` und `krk-bench` sind leere Rümpfe, wie der Schritt es verlangt. Es ist
nichts vorgebaut.

## Abnahmekriterium, selbst geprüft

Alle vier Kommandos wurden am 260802-1741 im Projektwurzelverzeichnis
ausgeführt. Die Rückgabewerte sind die von `cargo`, nicht die einer
nachgeschalteten Pipe: der erste Durchlauf hatte den Wert von `tail` gemessen,
der zweite lief mit `set -o pipefail` und ohne Pipe.

| Kommando | Rückgabewert |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo build --workspace --target x86_64-apple-darwin` | 0 |
| `cargo build --workspace --target aarch64-apple-darwin` | 0 |
| `cargo test --workspace` | 0 |

`cargo test --workspace` findet erwartungsgemäß null Tests: vier Testbinaries
laufen, jedes meldet `0 passed; 0 failed`. Tests entstehen ab Schritt 2.

Die drei Zeilen, die der Diff zeigen muss, stehen an ihrer Stelle:

- `crates/krk-core/src/lib.rs:1` → `#![deny(unsafe_code)]`
- `crates/krk-ui/src/main.rs:1` → `#![warn(unsafe_code)]`
- `.cargo/config.toml:5` → `MACOSX_DEPLOYMENT_TARGET = "15.0"`

Zwei Prüfungen über das Kriterium hinaus, beide bestanden:

- `vtool -show-build-version` meldet für `target/debug/krk` und für
  `target/aarch64-apple-darwin/debug/krk` je `minos 15.0`. Die Umgebungsvariable
  aus `.cargo/config.toml` greift also am Binärformat und nicht nur auf dem
  Papier. Das nimmt einen Teil des Nachweises aus S5 vorweg, ohne S5 zu bauen.
- `cargo fmt --all --check` endet mit 0, `cargo build --workspace` gibt keine
  Warnung aus.

## Werkzeugkette

`rust-toolchain.toml` mit `channel = "1.97.1"` hat rustup dazu gebracht, die
Werkzeugkette `1.97.1-x86_64-apple-darwin` neben der vorhandenen `stable`
einzurichten, samt beider Ziele. `rustup show` weist sie als aktiv aus, aktiv
"overridden by rust-toolchain.toml". Die Versionen stimmen mit der geprüften
Werkzeugkette aus dem Plan überein: `rustc 1.97.1`, `cargo 1.97.1`.

Abhängigkeiten, wie sie sich aufgelöst haben: `serde 1.0.229`, `toml 1.1.4`,
`objc2 0.6.4`, `objc2-app-kit 0.3.2`, `objc2-foundation 0.3.2`.

## Drei Festlegungen, die der Schritt offen ließ

Der Schritt nennt Dateien und Inhalte, lässt aber drei Punkte offen. So sind sie
entschieden:

1. **Edition 2024, `resolver = "3"`.** Der Plan nennt keine Edition. Für 2024
   spricht die Notiz im Abschnitt `### Wo die Kosten des Technologieentscheids
   anfallen`: sie schreibt `#[unsafe(super = NSObject)]`, und die
   unsafe-Attributschreibweise ist in Edition 2024 die verlangte Form.
2. **Das Binärprogramm heißt `krk`, nicht `krk-ui`.** Ein `[[bin]]`-Abschnitt in
   `crates/krk-ui/Cargo.toml` setzt den Namen. Das Abnahmekriterium von S5 nennt
   `target/KRK.app/Contents/MacOS/krk`; ohne die Umbenennung träfe es nicht zu.
3. **`.cargo/config.toml` trägt zusätzlich den Alias `xtask = "run --package
   xtask --"`.** Siehe die Lücke unten.

## Eine Lücke im Plan, gemeldet und nicht eigenmächtig umgangen

S5 prüft mit `cargo xtask bundle` ab. `cargo xtask` ist kein eingebautes
Kommando, sondern ein Alias, den ein Projekt in `.cargo/config.toml` selbst
setzt. Kein Schritt des Plans legt diesen Alias an: S1 beschreibt für
`.cargo/config.toml` allein den Abschnitt `[env]`, S5 nennt als Dateien nur
`xtask/src/{bundle.rs,sign.rs}`, `xtask/src/main.rs` und `README.md`. Ohne den
Alias scheitert das Abnahmekommando von S5 daran, dass es das Unterkommando
nicht gibt.

Das ist eine Lücke, kein Widerspruch: der Plan verlangt nirgends etwas anderes.
Aufgelöst ist sie hier, weil der Alias zum Bauzuschnitt gehört und `xtask` sonst
ein Mitglied wäre, das man nicht ansprechen kann. Ein Defektdatensatz ist
**nicht** angelegt, obwohl die Konventionen ihn verlangen: die Aufgabenstellung
untersagt dem coder in dieser Sitzung jeden Schreibzugriff auf
`fusion-workbench/` außer auf diese Historie, weil parallel ein anderer Agent am
Spec und am Circle-Datensatz arbeitet. Die Meldung liegt damit hier und im
Bericht an den Nutzer; das Anlegen der Datei bleibt dem Orchestrator.

Ein zweiter, kleinerer Punkt derselben Art: S1 führt `rustfmt.toml` in seiner
Dateiliste, sagt aber nicht, was darin stehen soll. Gewählt ist die knappste
Fassung, `edition = "2024"`, passend zu Festlegung 1.

## Was nicht angefasst wurde

- `fusion-workbench/` außer dieser Historie. Der Plan ist **nicht** auf `[DONE]`
  gesetzt und der Dateimarker nicht gezogen; beides bleibt dem Orchestrator,
  aus demselben Grund wie oben.
- `spikes/`. Aus dem Prüfcode ist nichts übernommen.
- `.gitignore` ist ergänzt, nicht überschrieben: die vorhandenen Einträge für
  `.claude/settings.local.json` und den flüchtigen fusion-Sitzungszustand stehen
  unverändert, dazugekommen sind `/target/` und `**/*.rs.bk`.
- Kein Commit. Der Orchestrator committet.
