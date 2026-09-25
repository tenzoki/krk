# Coder: Schritt 4 — die Kiste `objc2-pdf-kit` mit Begründung

**Datum:** 260828-0740
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 4
**Status:** Complete

## Was geändert ist

- `Cargo.toml` (Wurzel): hinter `objc2-quartz-core` der Eintrag `objc2-pdf-kit = { version = "0.3", default-features = false, features = ["std", "objc2-app-kit", "objc2-core-foundation", "PDFView", "PDFDocument", "PDFPage", "PDFSelection"] }` mit Begründungskommentar nach dem Muster der Nachbarn (Plan-Entscheidungen 1 und 2, Constraint 4, Alternative ohne Kiste, warum `objc2-core-foundation` nötig und `objc2-core-graphics` aus ist).
- `crates/krk-ui/Cargo.toml`: `objc2-pdf-kit = { workspace = true }` mit Hinweis auf `appkit/betrachter.rs` (Schritt 6).
- `Cargo.lock`: 14 neue Zeilen, ein neues Paket `objc2-pdf-kit 0.3.2` mit den Abhängigkeiten `bitflags`, `objc2`, `objc2-app-kit`, `objc2-core-foundation` — alle lagen schon.

## Befund `Cargo.lock`

`grep -n 'name = "cc"\|-sys"' Cargo.lock` findet allein `windows-sys` (Zeilen 108, 862, 872). `cargo tree --workspace -e normal,build` findet weder `cc` noch `onig` noch ein `-sys`-Paket. Die Kiste trägt `build = false` in ihrer `Cargo.toml` (Registry-Quelle 0.3.2, Zeile 17). Alle sieben Merkmalsnamen aus dem Plan kennt 0.3.2; `cargo tree -e features` zeigt genau sie plus `alloc` und `bitflags` als Folge.

Abweichung vom Plan: der Plan spricht von „vier neuen Zeilen in `Cargo.lock`"; es sind 14 (der Paketblock hat 13 Zeilen, dazu eine Zeile in der Abhängigkeitsliste von `krk-ui`). Die Aussage „ein neues Paket, keine weitere Kiste" stimmt.

## Verifikation

- `cargo build --workspace` — exit 0 (Lauf unmittelbar nach dem Eintrag, 19 s, `objc2-pdf-kit v0.3.2` übersetzt).
- `cargo build -p objc2-pdf-kit` — exit 0.
- `make check` — rot, ausschließlich in fremden, gerade bearbeiteten Dateien: `cargo fmt --check` in `crates/krk-core/src/tasten/belegung.rs` und `crates/krk-ui/src/vorschaumodell.rs`; ein späterer `cargo build --workspace` bricht mit E0004 in `crates/krk-core/tests/belegung.rs:1960`, `crates/krk-ui/src/belegungsmodell.rs:227`, `crates/krk-ui/src/kommandos/fokus.rs:344,764` ab (`Wirkungsbereich::Vorschau` und die drei Vorschau-Kommandos aus Schritt 2, noch nicht überall nachgezogen). Keine dieser Dateien gehört zu diesem Schritt.
