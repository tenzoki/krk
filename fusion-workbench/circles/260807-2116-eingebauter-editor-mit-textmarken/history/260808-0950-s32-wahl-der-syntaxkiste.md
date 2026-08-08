# S32: Die Wahl der Kiste für die Syntaxhervorhebung

**Status:** Complete
**Agent:** coder
**Datum:** 260808-0950
**Plan:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_o_plan-eingebauter-editor-mit-textmarken.md` §32
**Bindende Grundlage:** `circles/260807-2116-eingebauter-editor-mit-textmarken/decisions/260807-2147_a_fuer-welche-sprachen-hebt-die-formatansicht-syntax-hervor.md`

## Was entschieden wurde

Es sind **zwei Kisten und nicht eine**: `syntect 5.3.0` trägt die Erkennung und die
Einfärbung, `two-face 0.5.2` trägt die Sprachdefinitionen nach, die `syntect` nicht
mitbringt. Der Vorschlag des Planners hat sich damit in allen Teilen bestätigt,
seine offene Vermutung eingeschlossen.

## Die vier Kriterien, gemessen

**1. Deckt Rust, TOML, Markdown und Shell ab — erfüllt, aber nur zu zweit.** Der
Vorgabesatz von `syntect` führt 75 Sprachdefinitionen und darunter Rust, Markdown
und Shell, **aber kein TOML**. Die Vermutung aus `### Frage 2` des Plans trifft
damit zu. `two-face` bringt 213 Sprachdefinitionen mit, den erweiterten Satz von
`bat`, und schließt die Lücke. Der Befund steht nicht nur hier, sondern als
Prüfzeile `ohne_two_face_fehlt_toml`: sie schlägt fehl, sobald `syntect` TOML
nachreicht, und ist damit der Anlass, `two-face` neu zu bewerten, statt es
mitzuschleppen.

**2. Keine C-Werkzeugkette, `#![deny(unsafe_code)]` unberührt — erfüllt.** Beide
Kisten laufen mit `default-features = false`. Bei `syntect` tauscht `regex-fancy`
die C-Bibliothek Oniguruma gegen `fancy-regex` in reinem Rust; bei `two-face`
schaltet der Vorgabesatz `syntect-onig` sie sonst durch die Hintertür wieder ein,
weshalb dort `syntect-fancy` steht. Geprüft an `cargo tree -p krk-ui -e
normal,build`: 195 Zeilen, keine Kiste mit `-sys` im Namen, keine
Bauabhängigkeit `cc`, kein Oniguruma. `grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]'
crates/krk-core/src crates/krk-ui/src` nennt weiterhin genau zwei Dateien,
`verzeichnis/sys.rs` und `appkit/mod.rs`.

**3. Eine helle und eine dunkle Farbtafel — erfüllt.** Der Vorgabesatz von
`syntect` bringt sieben Tafeln mit: `InspiredGitHub`, `Solarized (dark)`,
`Solarized (light)`, `base16-eighties.dark`, `base16-mocha.dark`,
`base16-ocean.dark` und `base16-ocean.light`. Das Paar `base16-ocean.light` und
`base16-ocean.dark` ist damit ohne eigene Tabelle zu haben; welche zwei es
schließlich werden, entscheidet S34.

**4. Bündelwachstum unter 10 MB — in diesem Schritt nicht messbar.** Gemessen wuchs
`target/release/krk` durch beide Kisten um **128 Byte**, weil kein Modul der
Anwendung sie bisher aufruft und der Übersetzer den eingebetteten Bestand
wegwirft. Die Zahl ist echt und sagt nichts. Die Ersatzmessung an einem
eigenständigen Programm, das die Sätze wirklich lädt: 1.591.544 Byte mit den
Kisten gegen 418.968 Byte ohne, also rund **1,12 MiB**, gut ein Zehntel des
zugestandenen Rahmens. Der Befund ist als Datensatz abgelegt
(`issues/260808-0948_o_das-vierte-kriterium-von-s32-ist-in-s32-nicht-messbar.md`)
mit dem Vorschlag, das Kriterium in S33 zu wiederholen. Das Bündel wiegt am
260808-0948 3.502.046 Byte über vier Dateien; das ist der Ausgangspunkt für S33.

## Geänderte Dateien

- `Cargo.toml` — beide Kisten unter `[workspace.dependencies]`, jede mit der
  geschriebenen Begründung im Ton der vier bestehenden: was sie leistet, warum
  keine bestehende Abhängigkeit es leistet, welche Vorgabemerkmale abgeschaltet
  sind. Der angenommene Preis steht mit `speculation:` dabei und verweist auf den
  Spec-Abschnitt `## Verhältnis zu den zehn Zeitzusagen aus C8 der Runde 1`, der
  die Kiste der späteren Messrunde als vierten Gegenstand neben L1, L4 und L7
  übergibt.
- `crates/krk-ui/Cargo.toml` — beide Kisten als unmittelbare Abhängigkeit, mit der
  kurzen Begründung, warum sie in der Oberfläche und nicht im Kern stehen.
- `Cargo.lock` — 23 neue Kisten, alle in reinem Rust: `syntect`, `two-face`,
  `fancy-regex`, `flate2`, `bincode`, `walkdir` und deren Anhang.
- `crates/krk-ui/tests/syntaxkiste.rs` — neu, fünf Prüfungen, kein Fenster und kein
  Vordergrund nötig.

## Was der Prüfcode belegt

Fünf Zeilen, alle grün: der Satz führt die vier Sprachen; ohne `two-face` fehlt
TOML; eine unbekannte Endung fällt auf einfachen Text zurück, statt einen Fehler zu
melden (das sechste Abnahmekriterium von C3, auf das sich S33 stützen darf); es gibt
eine helle und eine dunkle Tafel, und sie färben verschieden; und die Einfärbung
setzt in Rust wie in TOML mindestens drei Vordergrundfarben gegeneinander ab.

**Was er nicht belegt:** die Geschwindigkeit auf dem Referenzgerät von 2018. Sie
bleibt ungemessen, wie der Nutzer es am 260808-0017 angenommen hat.

## Die vier Prüfkommandos

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0, 568 Prüfungen, davon 5 neue |
| `cargo clippy --workspace --all-targets` | 0, keine Meldung |
| `cargo fmt --all --check` | 0 |
| `cargo xtask bundle` | Bündel gebaut und signiert |

## Zwei Befunde für den Plan

1. `issues/260808-0948_o_das-vierte-kriterium-von-s32-ist-in-s32-nicht-messbar.md`
2. `issues/260808-0949_o_s32-nennt-dump-create-unter-den-abgeschalteten-merkmalen-es-laesst-sich-nicht-abschalten.md`
   — `parsing` zieht `dump-create` mit, das Merkmal ist nicht abschaltbar. Der
   Kommentar in `Cargo.toml` hält das bereits fest.

## Was offen bleibt

Der Entscheidungsdatensatz `260807-2147_a_...` bleibt auf **beantwortet** und geht
nicht auf **umgesetzt**. Die Kisten stehen im Baum, aber die Formatansicht hebt
noch nichts hervor; die Antwort ist erst mit S33 in Code realisiert. Wer sie dort
umsetzt, trägt die `Implemented:`-Zeile nach.

Die einklappbaren Blöcke bringt keine der beiden Kisten mit. `syntect` liefert
Wortarten und keine Blockgrenzen, `two-face` liefert nur weitere Sprachdefinitionen
für dieselbe Mechanik. Das ist der zweite Preis, den der Nutzer am 260808-0017
angenommen hat; die Fähigkeit entfällt in dieser Runde und kommt als eigenes
Vorhaben.

## Ein Nachtrag zur Bauumgebung

Während dieses Schritts war der Arbeitsbereich zweimal für einige Minuten nicht
übersetzbar, weil ein parallel laufender Schritt `Fokus::Editor` in
`crates/krk-ui/src/kommandos/fokus.rs` einführte, bevor die Fallunterscheidung in
`appkit/anwendung.rs:1547` nachgezogen war. Der Zustand hat sich von selbst
aufgelöst; er ist hier nur vermerkt, damit ein späterer Leser den Abbruch in den
Werkzeugausgaben nicht diesem Schritt zuschreibt.
