# Coder: Schritt 9 — die drei Ausführungszweige beim Anwendungsdelegierten

**Circle:** 260827-2028-vorschau-rendert-pdf-als-betrachter
**Plan:** planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md, Schritt 9
**Status:** Complete

## Was gebaut ist

- `crates/krk-ui/src/appkit/anwendung.rs`: `kommando_ausfuehren` trägt vor dem Auffangzweig drei eigene Zweige, `Kommando::VorschauVergroessern/Verkleinern/Ausgangsgroesse => self.vorschau().zoomen(Zoom::…)`, mit dem Kommentar, warum sie nicht über `bereichskommando` gehen (C3.8; `Vorschaufenster::kommando_ausfuehren` führt allein die Tabbefehle). `false` aus `zoomen` heißt kein Nachzug, Tastendruck verbraucht (A6, C3.7). Import `use super::betrachter::Zoom;`.
- Neues Prüfmodul `zoomproben` in derselben Datei, zwischen `zettelproben` und `angleichproben`, nutzt deren `diese_datei`/`rumpf`: `die_drei_zoombefehle_haben_genau_hier_ihren_zweig` (jede Kennung genau einmal als `Kommando::… =>` im Rumpf von `kommando_ausfuehren`) und `die_dateiliste_traegt_keinen_zoomzweig` (null Treffer in `tabelle.rs`).
- `crates/krk-ui/src/appkit/vorschau.rs` und `betrachter.rs`: die zwei `#[allow(dead_code)]` samt Ablauf-Kommentar „Schritt 9" entfernt; der Rufer steht jetzt.

## Verifikation

`make check` bricht an der ersten roten Kiste ab; deshalb daneben `cargo test --workspace --no-fail-fast`, `cargo clippy --workspace --all-targets` (exit 0, keine Warnung) und `cargo fmt --all --check` (exit 0). Rot sind genau die drei erwarteten Proben, die auf die Belegungseinträge des Ontocoders warten: `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste`, `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander`. Sonst nichts rot. Kein Commit.
