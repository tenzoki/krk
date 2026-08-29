# Coder: Schritt 6 der Runde 21 — der dritte Leser der Hülle

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-1102_*_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 6 (Planerantwort 3)

## Was gebaut ist

`crates/krk-ui/src/appkit/zwischenablage.rs`, allein diese Datei:

- `lesen()` ist der Einzeiler `lesen_aus(&NSPasteboard::generalPasteboard())`; `pub fn lesen_aus(ablage: &NSPasteboard) -> Option<String>` trägt den bisherigen Rumpf (FileURL vor String, leerer Weißraum zählt nicht). Muster von `text_schreiben`/`text_auf_ablage_schreiben`.
- Neu `pub fn einfuegequelle() -> Einfuegequelle` (Einzeiler auf `generalPasteboard`, `#[allow(dead_code)]` mit Ablauf Schritt 7) und `pub fn einfuegequelle_aus(ablage: &NSPasteboard) -> Einfuegequelle`: `dateiverweise(ablage)` nicht leer → `Verweise` (alle, in Reihenfolge); sonst `lesen_aus` → `Text` oder `Leer`. Zusammensetzung der zwei bestehenden Leser, kein dritter Griff an die Ablage; keine dritte Sorte (A11). Der Typ kommt aus `krk_core::zwischenablage` (Schritt 2).
- Doc-Kommentare: Rangfolge aus A2, Verweiszahl für A4, kein dritter Sortentyp nach A11, die 0,13 ms je Verweis als Auskunft mit Verweis auf die Messung bei `dateiverweise`.
- Modulkopf: Skizze mit `lesen_aus`, `einfuegequelle ──> krk_core::zwischenablage::filtertext_aus (Runde 21)` und `einfuegequelle_aus`; der Absatz zur `paste:`-Hälfte sagt, dass sie seit der Runde 21 vom Filter und nicht von einer Dateizwischenablage besetzt ist, mit Verweis auf `decisions/260828-1041_o_…`; der Absatz „`lesen` bekommt keinen Parameter" trägt den Halbsatz zur gereichten Form.
- Vier Proben auf `probenablage` mit festen Namen (`einfuegequelle-text`, `-verweise`, `-rangfolge`, `-leer`): Text allein → `Text`; zwei Verweise aus dem `Pruefordner` → `Verweise` in Reihenfolge; ein Verweis mit Namenszeile → `Verweise`, nicht `Text`; geleert → `Leer`.
- `die_huelle_um_die_zwischenablage_steht_in_genau_einer_datei` unverändert und grün; `generalPasteboard` steht weiter allein hier (C4.1). Da nur diese Datei angefasst ist, trifft `grep -rn NSPasteboard crates/krk-ui/src` außerhalb der Hülle dieselben Stellen wie vor dem Schritt (30 Zeilen).

## Untergrenzen-Abschnitt (C4.6)

Gelesen und bestätigt: der Schritt spricht keine neue Klasse und keine neue Methode an. `einfuegequelle_aus` ruft allein `dateiverweise` (`readObjectsForClasses:options:`, 10.6) und `lesen_aus` (`stringForType:`, 10.0); beide stehen im Abschnitt. Der Abschnitt bleibt zeichengleich.

## Verifikation

- `cargo test -p krk-ui -- zwischenablage` — exit 0, 14 Proben grün (10 bestehende, 4 neue).
- `rustfmt --check --edition 2024 crates/krk-ui/src/appkit/zwischenablage.rs` — exit 0.
- `cargo clippy -p krk-ui --all-targets -- -D warnings` nennt `zwischenablage.rs` in keiner Meldung.
- `make check` — exit 2. Rote Befunde, keiner in dieser Datei:
  - `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` (`krk-core/tests/verzeichnis.rs`), planmäßig rot bis Schritt 9.
  - Clippy `dead_code` an `einfuegen_abgewiesen` (`kommandos/operationen.rs:1239`), Schritt 5 der parallelen Baustelle; Schritt 7 legt den Rufer an.
  - `cargo fmt --all --check` rot allein in `krk-core/tests/verzeichnis.rs`, Schritt 9 der parallelen Baustelle.
