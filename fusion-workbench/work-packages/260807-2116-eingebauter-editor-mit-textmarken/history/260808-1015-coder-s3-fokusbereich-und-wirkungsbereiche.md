# S3: Der fünfte Fokusbereich und die drei neuen Wirkungsbereiche

- Agent: `coder`
- Datum: 260808-1015
- Plan: `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`, Phase A, Schritt 3
- Status: Complete

## Was umgesetzt ist

`Wirkungsbereich` (`crates/krk-core/src/tasten/belegung.rs`) wächst von vier auf
sieben Werte: `Vorschau`, `Editor` und `Navigator` kommen dazu. Jeder trägt
seine eigene Begründung als Doc-Kommentar, aus Befund 3 des Plans übernommen.
Der Satz "Ein eigener Vorschau-Wert daneben entsteht nicht, weil kein Befehl
allein im Vorschaufenster wirkt" ist ersetzt: der Kopf sagt jetzt, dass der
Übergang aus der Vorschau in den Editor genau ein solcher Befehl ist und der
alte Satz damit hinfällig ist.

`Navigator` ist positiv aufgezählt (Dateifenster, Leiste, Vorschau) und nicht
als Verneinung von `Editor`. Der Grund steht am Wert und noch einmal als
Kommentar am `match`-Zweig in `wirkt`: die Verneinung ließe `Fokus::Anderswo`
durch, und ein `up` vor der Rückfrage des endgültigen Löschens bewegte die
Auswahl im Ordner dahinter.

`Fokus` (`crates/krk-ui/src/kommandos/fokus.rs`) bekommt den fünften Wert
`Editor`. `holt_hervor` liefert dafür `Some(Bereich::Editor)`; die Bedingung
"sofern der Editor eine Datei hält" steht ausdrücklich beim Aufrufer und nicht
hier, weil `holt_hervor` eine reine Zuordnung ohne Zustand ist. `wirkt` bekommt
die drei neuen Zweige und bleibt eine erschöpfende Fallunterscheidung.
`JEDER_FOKUS` in den Proben wächst von vier auf fünf.

Die drei bestehenden Befehle `fenster_wechseln`, `auswahl_hoch` und
`auswahl_runter` ziehen **nicht** in diesem Schritt von `Ueberall` nach
`Navigator` um; das ist S5. `Wirkungsbereich::Navigator` trägt deshalb heute
noch kein Kommando.

## Proben

- `die_tafel_aus_sieben_wirkungsbereichen_und_fuenf_fokuswerten_geht_auf`
  (`fokus.rs`): die 35 Paare als Tafel, beide Feldbreiten in der Typangabe, so
  dass ein sechster Fokuswert oder ein achter Wirkungsbereich den Bau anhält.
- `der_navigator_endet_am_editor_und_ueberall_nicht` (`fokus.rs`): das zweite
  Abnahmekriterium des Schrittes wörtlich.
- `der_navigator_schliesst_auch_das_stehende_blatt_aus` (`fokus.rs`): die
  Gegenprobe zur Verneinung.
- `jeder_fokusbefehl_holt_seinen_bereich_hervor` (`fokus.rs`) um die Editor-
  Zeile erweitert.
- `jedes_kommando_traegt_genau_einen_wirkungsbereich`
  (`crates/krk-core/tests/belegung.rs`): die Aufzählung im `matches!` von vier
  auf sieben gezogen, samt Meldung und Kommentar.

## Zwei Stellen außerhalb des Schrittumfangs

Der fünfte `Fokus`-Wert hält den Bau an zwei erschöpfenden Fallunterscheidungen
in `crates/krk-ui/src/appkit/anwendung.rs` an. Beide gehören laut Plan zu **S17**
("Der Fokus erkennt den Editor"). Sie tragen je einen minimalen Zweig
`Fokus::Editor => false` und einen Kommentar, der S17 als Ablösung nennt:

- `fokus_setzen` (`anwendung.rs:1090`): solange keine Textfläche existiert, ist
  kein Ersthelfer zu setzen; der Befehl scheitert stumm.
- `bereichskommando` (`anwendung.rs:1547`): der Wert ist unerreichbar, solange
  `Anwendungsdelegierter::fokus` ihn nie liefert. Der Zweig leitet das Kommando
  **nicht** an das Dateifenster um, denn dorthin gehört es nicht.

`Anwendungsdelegierter::fokus` (`anwendung.rs:2072-2100`) bleibt unangetastet:
es ist eine `if`-Kette mit Rückfall und erzwingt nichts. S17 baut den Zweig.

## Abnahme

| Kommando | Ergebnis |
|----------|----------|
| `cargo build --workspace` | grün |
| `cargo test --workspace` | grün, 568 Proben, 0 Fehler, 1 ignoriert |
| `cargo clippy --workspace --all-targets` | grün, keine Warnung |
| `cargo fmt --all --check` | sauber |

Ein erster `cargo test --workspace` scheiterte an
`crates/krk-ui/tests/syntaxkiste.rs` mit "cannot find module or crate
`syntect`". Das war ein Wettlauf mit dem parallel laufenden S32, der
`Cargo.toml` gerade schrieb; der Wiederholungslauf ist grün.
