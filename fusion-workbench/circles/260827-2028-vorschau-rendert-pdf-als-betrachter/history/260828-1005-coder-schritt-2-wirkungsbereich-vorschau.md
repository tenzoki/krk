# Coder-Sitzung — 260828-1005

**Aufgabe:** Schritt 2 des Plans der Runde 20 — `Wirkungsbereich::Vorschau` und die drei Zoombefehle
**Circle:** 260827-2028-vorschau-rendert-pdf-als-betrachter (aktiv)
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 2 und Entscheidung 4
**Status:** Complete

## Was geändert wurde

- `crates/krk-core/src/tasten/belegung.rs`: achter Wert `Wirkungsbereich::Vorschau` (nach `Navigator`, vor `Ueberall`) mit Doc-Kommentar; Beschriftung `"Vorschau"`; der Doc-Kommentar der Aufzählung sagt „Acht Werte" und beschreibt die Rückkehr des Werts mit drei Trägern, der an `Dateibereiche` verweist auf den neuen Wert. `Kommando` trägt `VorschauVergroessern`, `VorschauVerkleinern`, `VorschauAusgangsgroesse` (Doc-Kommentare nennen A1, A2, A6); `KENNUNGEN` steht auf 82 mit `vorschau_vergroessern`, `vorschau_verkleinern`, `vorschau_ausgangsgroesse`; `wirkungsbereich` gibt den dreien `Wirkungsbereich::Vorschau`.
- `crates/krk-core/tests/belegung.rs`: `matches!` in `jedes_kommando_traegt_genau_einen_wirkungsbereich` um `Vorschau` erweitert; Zahl in der Doc von `jede_variante_von_kommando_steht_genau_einmal_in_kennungen` auf 82; `SIEBEN_BESCHRIFTUNGEN` → `ACHT_BESCHRIFTUNGEN` (8), `stelle_in_den_sieben` → `stelle_in_den_acht`, Halbsatz „und damit im Feld" gestrichen; neue Hilfsfunktion `jeder_wirkungsbereich_im_quelltext` über `varianten_der_aufzaehlung`, die drei Beschriftungsproben laufen darüber; neue Probe `die_drei_zoombefehle_tragen_die_vorschau_allein`, die `die_drei_faelle_aus_c5_tragen_die_bereiche_die_c5_verlangt` mit aufruft.
- `crates/krk-ui/src/belegungsmodell.rs`: die drei bei `Funktionsbereich::Vorschau`.
- `crates/krk-ui/src/kommandos/fokus.rs`: `wirkt` mit `Wirkungsbereich::Vorschau => fokus == Fokus::Vorschau`; Tafel auf acht Zeilen, Probe heißt `die_tafel_aus_acht_wirkungsbereichen_und_fuenf_fokuswerten_geht_auf`; der `match` in `die_befehle_des_dateifensters_enden_am_editor` (Prüfmodul) nimmt `Vorschau` in den abweisenden Zweig.
- `crates/krk-ui/src/kommandos/zulaessigkeit.rs`: `STELLVERTRETER` auf acht (`Vorschau` → `VorschauVergroessern`), Tafel auf 320 Fälle (`die_tafel_aus_dreihundertzwanzig_faellen_geht_auf`); neue Proben `jeder_wirkungsbereich_hat_einen_stellvertreter` (Varianten aus `belegung.rs` über `quelldateien`), `die_drei_zoombefehle_wirken_mit_dem_fokus_in_der_vorschau` (C3.7) und `die_drei_zoombefehle_wirken_ausserhalb_der_vorschau_nicht` (C3.5). Der Doc-Kommentar an `STELLVERTRETER` behauptete, ein achter Wert halte den Bau an — dort steht kein `match`, also hielt er ihn nicht; der Kommentar sagt das jetzt und verweist auf die neue Probe.
- Defekt `shared/issues/260826-1302_*_ein-achter-wirkungsbereich-…` mit `Resolved:` geschlossen und `_o_` → `_c_` umbenannt.
- Plan: Schritt 2 auf `[DONE]`.

## Verifikation

`make check`: `cargo build --workspace` grün, `cargo clippy --workspace --all-targets` ohne Warnung, `cargo fmt --all --check` grün. `cargo test --workspace --no-fail-fast`: drei Proben rot, alle drei hängen an den drei Einträgen in `resources/default-keymap.toml` (Schritt 3, Ontocoder) und nicht an diesem Schritt:
- `krk-core` lib: `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`
- `krk-core` tests/belegung.rs: `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`
- `krk-ui`: `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` (zählt `mit_kommando` gegen `KENNUNGEN.len()`, 79 gegen 82)

Alle übrigen Proben grün, darunter jede neue.

## Nicht angefasst

`tasten/parser.rs`, `tasten/mod.rs`, `appkit/menue.rs` (S1), `Cargo.toml`/`Cargo.lock` (S4), `vorschaumodell.rs` (S5), `resources/default-keymap.toml` (S3). Keine Ausführungszweige in `anwendung.rs` (Schritt 9). `crates/krk-ui/src/belegungsausgabe.rs` trägt in Prosa „79 mit Kommando" und „85 Funktionen" (Modulkopf `:45-56`, `:256`, `:730-731`); nach Schritt 3 sind es 82 und 88 — nicht in der Dateiliste dieses Schritts, deshalb hier vermerkt statt geändert. CLAUDE.md nennt für `Wirkungsbereich` sieben Werte; das gehört dem Kurator.
