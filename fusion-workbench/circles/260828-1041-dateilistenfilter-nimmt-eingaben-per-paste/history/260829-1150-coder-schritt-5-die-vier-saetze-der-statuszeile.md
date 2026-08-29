# Coder: Schritt 5 der Runde 21 — die vier Sätze der Statuszeile

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-1102_*_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 5

## Was geändert ist

`crates/krk-ui/src/kommandos/operationen.rs`, allein diese Datei.

- Neuer Block „Das Einfuegen in den Filter (Runde 21)" hinter `verweise_abgewiesen`: `#[must_use] pub fn einfuegen_abgewiesen(hindernis: Einfuegehindernis) -> String`, vollständiges `match` über die vier Varianten ohne Auffangzweig, Wortlaut aus A5 mit Umlauten, die Verweiszahl über `zahl`. Doc-Kommentar nach dem Muster von `verweise_abgewiesen`, mit dem Grund, warum ein geglücktes Einfügen keinen Satz bekommt (A5, C2.8).
- Import `use krk_core::zwischenablage::Einfuegehindernis;` (Typ aus Schritt 2, uncommittet im Baum).
- Vier Proben im Prüfmodul, eine je Variante, mit dem Wortlaut als Erwartung; die Verweisprobe hält `3` und `1234` → `1.234 Dateiverweise` (C2.9).
- Modulkopf nennt `einfuegen_abgewiesen` neben den Sätzen der Runde 22.

## Verifikation

- `cargo test -p krk-ui -- operationen` — exit 0, 57 Proben grün, die vier neuen darunter.
- `make check` — exit 2. Rot allein `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` (`krk-core --test verzeichnis`, erwartet bis Schritt 9). `make check` bricht nach `cargo test` ab, deshalb einzeln:
- `cargo test --workspace --no-fail-fast` — exit 101, dieselbe eine Probe, alles andere grün (krk-ui 859 Proben).
- `cargo clippy --workspace --all-targets` — exit 0; eine Warnung `function einfuegen_abgewiesen is never used`, erwartet bis Schritt 7 (der Rufer `aus_zwischenablage_einfuegen`).
- `cargo fmt --all --check` — exit 0.

Beobachtet, nicht meins: während `make check` meldete der Testbau `unused import: krk_core::zwischenablage::Einfuegequelle` in `crates/krk-ui/src/appkit/zwischenablage.rs:245` (Schritt 6, parallel in Arbeit); beim späteren clippy-Lauf war die Warnung weg.
