# Coder: Schritt 1 — Der eine Vergleich wird zum Musterabgleich, und das Muster reist als Typ

**Date:** 2026-08-29
**Agent:** coder
**Plan:** `planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 1
**Status:** Complete

## Was gebaut ist

- `crates/krk-core/src/verzeichnis/filter.rs`: `pub struct Muster { stuecke: Vec<String> }` mit `Muster::aus(filtertext)` (kleinschreiben, an `*` teilen; `""` ergibt `[""]`). `traegt_die_folge(name: &str, muster: &Muster) -> bool` sucht jedes Stück ab dem Ende des vorigen genau einmal, ohne Rückverfolgung; das Vertauschungsargument steht im Doc-Kommentar. Modulkopf: Skizze mit `Muster::aus`, Abschnitt „Der Vergleich ist seit der Runde 21 ein Musterabgleich“ (ein Sonderzeichen, ungebunden an beiden Enden, ohne Rückverfolgung, Zählregel der Schwelle), Zeichenregel mit drei Rufern und dem Namen des dritten (`zwischenablage::filtertext_aus`). Doc von `inhaltsschwelle`: `*` zählt nicht.
- Prüfmodul von `filter.rs`: die bestehenden Vergleichsproben laufen über den Helfer `trifft(name, filtertext)`; `ein_grossgeschriebener_filtertext_findet_nichts` ist zu `das_muster_schreibt_einmal_klein` geworden. Neu: `ein_stern_steht_fuer_eine_beliebige_auch_leere_folge` (C5.2), `zwei_sterne_sind_einer_und_lauter_sterne_treffen_jeden_namen` (C5.3), `ein_stern_am_rand_verankert_nichts` (C5.4), `es_gibt_kein_zweites_sonderzeichen_und_kein_entkommen` (C5.5), `die_schreibung_bleibt_und_gefaltet_wird_nichts` (C5.6), `der_vergleich_sucht_jedes_stueck_genau_einmal_ab_dem_ende_des_vorigen` (C7.3), `traegt_ein_dateiname_nimmt_den_stern` (C5.7), `der_marker_zwischen_zwei_unterstrichen_trifft_jeden_marker_und_keinen_fehlenden` (C5.1, Vergleichshälfte).
- `crates/krk-core/src/verzeichnis/modell.rs`: Feld `filter_klein: String` → `muster: Muster`; `filter_klein()` → `muster() -> &Muster`; `filter_uebernehmen` ruft `Muster::aus`; `name_traegt_den_filter` reicht `&self.muster`; `inhalt_wirkt` zählt `chars().filter(|z| *z != '*')` (Entscheidung 8), Doc sagt es; neu `text_anhaengen(&mut self, text: &str)` neben `zeichen_anhaengen` (Entscheidung 6, mit dem Grund gegen die Schleife). Modulkopf-Absatz zum Befund nennt das Muster.
- `crates/krk-core/src/verzeichnis/inhalt.rs`: `traegt_der_inhalt(pfad, muster: &Muster, grenze)`, Doc sagt, dass Kleinschreibung und Zerlegung im Typ stecken und `*` über Zeilenenden treffen darf.
- `crates/krk-core/src/verzeichnis/durchlauf.rs`: `Durchlauf::starten(…, muster: Muster, …)`, `Auftragslage::muster: &'a Muster`, `durchlauffaden`, `datei_entscheiden`, `unterbaum_entscheiden` auf `&Muster`.
- `crates/krk-core/src/verzeichnis/mod.rs`: `pub use filter::Muster;`.
- `crates/krk-ui/src/tabs.rs`: `tab.modell.muster().clone()`.
- `crates/krk-core/tests/verzeichnis.rs`: allein mechanisch. `use krk_core::verzeichnis::filter::Muster;`; `modell.muster()` gegen `Muster::aus("AaA")` / `Muster::aus("")`; die Helfer und die vierzehn Aufrufe von `traegt_der_inhalt` und die sieben direkten `Durchlauf::starten` reichen `Muster::aus(…)`. Keine Codezeile nennt `traegt_die_folge`.

## Verifikation

- `cargo build --workspace` — exit 0.
- `cargo test --workspace --no-fail-fast`: alle Ziele grün bis auf **genau eine** Probe, `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` in `crates/krk-core/tests/verzeichnis.rs`. Sie fällt an der **Zeichenregel**-Hälfte (`left: [krk-core/src/zwischenablage.rs, tabelle.rs, belegungsmodell.rs]`), also am dritten Zeichenrufer aus Schritt 2, wie der Plan es bis Schritt 9 vorsieht. Die Vergleichshälfte hält: `grep -rln traegt_die_folge crates` außerhalb der Heimat liefert `inhalt.rs`, `durchlauf.rs`, `modell.rs` und eine Doc-Kommentar-Zeile in `kontextmenue.rs:678`, die `code_zeilen` streicht.
- `cargo clippy --workspace --all-targets` — keine Warnung, keine Fehler.
- `cargo fmt --all --check` — exit 0.
- `make check` — exit 2, allein wegen dieser einen Probe (es hält beim Ziel `test` an).

## Hinweis für den Orchestrator

`cargo fmt --all` ist im Lauf einmal über den ganzen Baum gegangen, also auch über die Dateien der parallel laufenden Schritte 2 und 3. Ob es dort etwas umgestellt hat, kann ich aus dem Diff nicht trennen; die zwei Dateien sind sonst nicht angefasst.
