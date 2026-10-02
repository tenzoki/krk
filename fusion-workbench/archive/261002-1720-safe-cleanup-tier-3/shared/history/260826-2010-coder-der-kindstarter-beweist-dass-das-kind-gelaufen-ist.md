# Der Kindstarter beweist, dass das Kind gelaufen ist

**Agent:** coder
**Datum:** 260826-2010
**Auftrag:** Schritt 3 des Plans `shared/planning/260826-1811_p_plan-die-fuenf-schweren-befunde-der-vollbaum-durchsicht.md`; Datensatz `shared/issues/260826-1302_o_sechs-elternproben-am-gemeinsamen-kindstarter-bleiben-gruen-wenn-der-kindname-nicht-trifft.md` samt Nachtrag R5
**Ausgangsstand:** HEAD `9c02863`
**Status:** Complete

## Was geändert ist

- `crates/krk-core/tests/gemeinsam/mod.rs`: `pub const KINDAUFTRAG = "KRK_KINDPROBE_AUFTRAG"` und `pub fn kindauftrag() -> Option<PathBuf>` als der eine Leser. `kind_mit_deskriptorgrenze(grenze, name, wert)` verliert den Parameter `auftrag`, setzt `KINDAUFTRAG` und hält nach `output()` das Gate: `status.success()` und `stdout` enthält `test result: ok. 1 passed;`, sonst Panik mit Name, Status, stdout und stderr und dem Hinweis auf Nichttreffer oder verlorenes `#[ignore]`. Liefert weiter `Output`. Der Modulkopf schreibt die drei stillen Wege und das Gate aus.
- `tests/umfang.rs`, `tests/verzeichnis.rs`, `tests/leseprofil.rs`: die sechs Konstanten `AUFTRAG_MANGEL`, `AUFTRAG_KETTE`, `AUFTRAG_INHALTSMANGEL`, `AUFTRAG_DESKRIPTOREN` (zweimal), `AUFTRAG_MANGEL` sind gestrichen; die sechs Kinder beginnen mit `let Some(ordner) = kindauftrag() else { return; }`, die nachfolgende `PathBuf::from(ordner)` entfällt, weil der Leser schon einen `PathBuf` liefert. Die sechs `#[ignore = "…"]`-Begründungen nennen jetzt `KRK_KINDPROBE_AUFTRAG`. Die sechs `assert!` der Rufer stehen unverändert als fachliche Zeile. `AUFTRAG_ABBRUCH`, `AUFTRAG_SPERRE` (`ablage.rs`) und `AUFTRAG_ZONE` (`zeit.rs`) nicht angefasst.
- Die Grenzen 24 (umfang, leseprofil) und 64 (verzeichnis) sind unverändert.

## Rot vor grün, beide Mutationen in `tests/umfang.rs`

| Mutation | HEAD `9c02863` | nach der Behebung |
|---|---|---|
| (1) Kindname des Rufers `ein_deskriptormangel_von_aussen_laesst_den_umfang_unentschieden` um ein Zeichen geändert (`…unentschiedem`) | grün, `2 passed` | rot: „die Kindprobe `…unentschiedem` ist nicht als genau ein Kind gelaufen (Status exit status: 0, erwartet `test result: ok. 1 passed;` in stdout)", Kind-stdout zeigt `running 0 tests … 10 filtered out` |
| (2) `#[ignore]` an `kind_zaehlt_die_tiefe_kette_mit_einem_deskriptor` entfernt | grün, `2 passed` | rot: dieselbe Meldung für dieses Kind, Kind-stdout `running 0 tests` |

Beide zurückgenommen (Datei aus der behobenen Fassung wiederhergestellt, `git diff` gegen die Sicherung leer). Der Baum trägt keine Mutation.

## Prüfung

- `cargo test -p krk-core --test umfang --test verzeichnis --test leseprofil -- deskriptor`: alle sechs Eltern grün, sechs Kinder als `ignored` gelistet.
- `cargo fmt --all` einmal nötig (ein umgebrochener Closure-Rumpf in `verzeichnis.rs`).
- `make check` — exit 0, „alle vier gruen".

## Nicht getan

Kein Commit; der Datensatz `260826-1302_o_…` bleibt auf `_o_`, der Orchestrator schließt ihn beim Commit. Plan-Schritt 3 auf `[DONE]`.
