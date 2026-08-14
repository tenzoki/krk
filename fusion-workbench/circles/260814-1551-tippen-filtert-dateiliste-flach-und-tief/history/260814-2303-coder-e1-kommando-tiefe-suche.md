# Sitzung: E1 — Kommando, Wirkungsbereich, Bereich und Ausführungszweig für die tiefe Suche

**Datum:** 260814-2303
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang E, Schritt E1
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C5.1, C5.3, C5.6

## Was umgesetzt ist

`Kommando::TiefeSucheUmschalten` ist die 78. Variante, mit der Kennung
`tiefe_suche_umschalten`. Nachgezählt am Baum, nicht geschätzt:

```sh
awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs \
  | grep -cE '^    [A-Z][A-Za-z]*,$'
# 78
```

Vier Stellen sind angefasst:

1. `crates/krk-core/src/tasten/belegung.rs` — die Variante hinter
   `SpalteTypUmschalten`, der Eintrag in `KENNUNGEN` an derselben Stelle, die Feldbreite
   der Typangabe von 77 auf 78, und eine Zeile in `Kommando::wirkungsbereich` am Zweig
   `Wirkungsbereich::Ueberall`, neben den drei Spaltenschaltern.
2. `crates/krk-ui/src/belegungsmodell.rs` — eine Zeile in `bereich_des_kommandos` am Zweig
   `Funktionsbereich::Dateilisting`, ebenfalls neben den drei Spaltenschaltern, damit der
   Eintrag im Hauptmenü dort steht, wo der Nutzer ihn sucht (C5.4).
3. `crates/krk-ui/src/appkit/anwendung.rs` — ein eigener Zweig in
   `Anwendungsdelegierter::kommando_ausfuehren` (C5.6), der den sichtbaren Tab des
   **aktiven** Dateifensters anspricht und immer `true` liefert.
4. `crates/krk-ui/src/appkit/tabelle.rs` — `DateifensterQuelle::tiefe_suche_umschalten`,
   neben `verstecke_umschalten` und in derselben Bauart. Diese Datei steht **nicht** in der
   Dateiliste des Schritts; die Begründung steht unten und als Datensatz.

Keine AppKit-Klasse ist neu angesprochen; die Untergrenzenabschnitte der beiden
`appkit/`-Modulköpfe bleiben unverändert. `resources/default-keymap.toml` ist nicht
angefasst — das ist E2 und gehört dem `ontocoder`.

## Zwei Befunde, beide als Datensatz abgelegt

**Der Weg an das Tabmodell führt durch eine vierte Datei.** `QuelleIvars::tabs`
(`crates/krk-ui/src/appkit/tabelle.rs:348`) ist modulprivat, und `DateifensterQuelle` trug
keine öffentliche Methode für das Kennzeichen. Aus den drei genannten Dateien ist das
Modell des sichtbaren Tabs damit nicht erreichbar. Schritt E3 trägt dasselbe von der
Leseseite her.
Datensatz: `issues/260814-2303_o_e1-und-e3-nennen-drei-dateien-der-weg-an-das-tabmodell-fuehrt-durch-eine-vierte.md`.

**E1 allein lässt den Baum rot.** `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`
verlangt zu jeder Kennung einen Eintrag in `resources/default-keymap.toml`, und den setzt
E2. Kein Ausführender kann die Zusicherung allein halten.
Datensatz: `issues/260814-2303_o_e1-und-e2-teilen-eine-zusicherung-die-eine-probe-haelt-und-lassen-den-baum-dazwischen-rot.md`.

## Abnahme

```
make check                                                  → exit 2
  cargo build --workspace                                   → 0
  cargo test --workspace                                    → 101 (ein Fehlschlag, siehe oben)
cargo fmt --all --check                                     → 0
cargo clippy --workspace --all-targets -- -D warnings       → 0
```

Der eine Fehlschlag ist der oben beschriebene und kein anderer: 146 Proben in `krk-core`
laufen, eine schlägt fehl; `krk-ui` und `krk-bench` sind grün.
