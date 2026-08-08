# S1: Die Ursache der y-Tasten benennen und belegen

**Datum:** 2026-08-08, 09:16
**Agent:** `coder`
**Status:** Complete
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`, Phase A, Schritt 1
**Defekt:** `shared/issues/260807-2112_*_cmd-y-und-shift-cmd-y-loesen-nichts-aus-f3-schon.md`

## Was der Schritt verlangte

Eine Probe nachtragen, die zwei Aussagen festhält, und den Defekt schließen.
**Kein Programmteil wird geändert.** Der Diff hält das ein: die einzige geänderte
Datei am Code ist `crates/krk-core/tests/belegung.rs`, und sie enthält nur
Prüfcode.

## Was ich am Code nachgeprüft habe

Der Plan nennt seine Belege mit Datei und Zeile. Ich habe jeden einzeln
aufgeschlagen, bevor ich ihn in die Abschlussnotiz übernommen habe:

| Behauptung des Plans | Fundstelle | Befund |
|---|---|---|
| `y` liegt auf Code 16, `kVK_ANSI_Y` | `crates/krk-core/src/tasten/parser.rs:209` | stimmt |
| `z` liegt auf Code 6, `kVK_ANSI_Z` | `crates/krk-core/src/tasten/parser.rs:210` | stimmt |
| Das Hauptmenü trägt sieben Einträge, keiner mit `y` | `crates/krk-ui/src/appkit/menue.rs:184-252` | stimmt; die sieben Kürzel sind `cmd+q`, `cmd+x`, `cmd+c`, `cmd+v`, `cmd+a`, `cmd+n`, `shift+cmd+w`, nachgesehen in `resources/default-keymap.toml` an den Zeilen 378, 387, 500, 506, 512, 525 und 533 |
| Die Normalisierung liest vier Bits und vergleicht `u8` gegen `u8` | `crates/krk-core/src/tasten/normalisierung.rs:181-196`, `crates/krk-core/src/tasten/parser.rs:369-410` | stimmt |
| Der Tastencode 6 steht in keiner Tastenliste der Auslieferungsbelegung | `resources/default-keymap.toml`, Suche über alle `tasten`-Zeilen | stimmt; die einzigen Treffer auf `y` oder `z` sind Zeile 101 (`["f3", "cmd+y"]`) und Zeile 349 (`["shift+cmd+y"]`) |

Damit stehen beide Verdächtigen des Defekts widerlegt, und die Ursache ist die,
die der Plan unter Befund 4 nennt.

## Die Probe

`die_y_kuerzel_liegen_auf_kvk_ansi_y_und_die_stelle_kvk_ansi_z_ist_unbelegt` in
`crates/krk-core/tests/belegung.rs`, eingehängt am Ende des Abschnitts
„Der Nachschlag" unter einer eigenen Überschrift.

Sie prüft in dieser Reihenfolge:

1. `Kombination::lesen("cmd+y")` und `Kombination::lesen("shift+cmd+y")` liefern
   beide den Tastencode 16 und die Stelle `kVK_ANSI_Y`.
2. Die Tabelle führt `z` auf Code 6 und `kVK_ANSI_Z`.
3. Keine Funktion der Auslieferungsbelegung trägt eine Kombination auf Code 6.
4. Kein Tastendruck auf Code 6 mit einer der fünfzehn Masken über den vier
   Zusatztasten trifft eine Funktion; alle fünfzehn geben `Nachschlag::Unbelegt`.

Die vierte Prüfung geht über die Wortlautzusage des Schrittes hinaus und kostet
nichts: `masken_mit_zusatztaste` steht in derselben Datei bereits für zwei
andere Proben.

**Zwei hingeschriebene Kombinationen, gegen die Hausregel dieser Datei.** Die
übrigen Proben suchen ihre Kombinationen in der Belegung, damit eine Umbelegung
sie nicht umwirft; drei Defekte in der Runde 1 sind aus dem Gegenteil
entstanden. Hier ist es umgekehrt richtig: die Zusage handelt von genau diesen
beiden Kombinationen und von der Stelle, auf der sie liegen. Der Doc-Kommentar
schreibt das aus und sagt, was zu tun ist, wenn Schritt S2 die Kombinationen
umlegt: dann ist die Probe rot, und sie gehört mitgeschrieben, weil dann die
Erklärung überholt ist.

## Abnahme

```
cargo test -p krk-core     107 + 26 + 37 + 15 + 26 + 7 + 5 + 16 + 9 bestanden, 0 gescheitert
cargo fmt --all --check    sauber
cargo clippy -p krk-core --all-targets   ohne Warnung
```

Die neue Probe läuft im Testprogramm `belegung` mit und ist dort namentlich
bestanden.

## Geänderte Dateien

- `crates/krk-core/tests/belegung.rs` (erweitert: ein Abschnitt mit einer Probe)
- `fusion-workbench/shared/issues/260807-2112_o_...` → `..._c_...` (Abschlussnotiz
  angehängt, Marker von offen auf geschlossen)
- `fusion-workbench/circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md`
  (Schritt 1 trägt `[DONE]`)

## Was offen bleibt

Der Schritt beantwortet, **warum** nichts ausgelöst wird. Was daraus folgt,
entscheidet der Nutzer in
`circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260808-0140_*_die-y-tasten-liegen-auf-einer-deutschen-tastatur-unter-anderen-buchstaben.md`;
Schritt S2 setzt die Wahl um.

Die Bestätigung am laufenden Bündel steht aus und ist Nutzerarbeit: Befehlstaste
und die Taste mit der Aufschrift **Z** drücken. Blendet die Vorschau ein und aus,
ist die Erklärung am laufenden Bündel bestätigt. Für die Abnahme dieses Schrittes
ist sie nicht nötig.
