# Coder: Schritt 2 der Runde 21 — die Reinigung im Kern

**Date:** 2026-08-29
**Status:** Complete
**Plan:** `planning/260829-1102_*_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, Schritt 2

## Was gebaut ist

`crates/krk-core/src/zwischenablage.rs`, allein diese Datei:

- `pub enum Einfuegequelle { Verweise(Vec<PathBuf>), Text(String), Leer }` und `pub enum Einfuegehindernis { KeinText, Mehrzeilig, MehrereVerweise(usize), NichtsTragbar }`, beide `Debug, Clone, PartialEq, Eq`.
- `pub fn filtertext_aus(&Einfuegequelle) -> Result<String, Einfuegehindernis>` nach Entscheidung 4 des Plans, vollständiges `match` über die drei Quellen. Ohne `#[must_use]` (C4.4, `clippy::double_must_use`); der Doc-Kommentar sagt es.
- Private Helfer `letzter_bestandteil(&str) -> &str` (`rsplit('/')`, erstes nicht leeres Stück, sonst leer) und `tragbar(char) -> bool` = `traegt_ein_dateiname(z) && z != ':'` (Entscheidung 5). `traegt_ein_dateiname` bleibt unangetastet.
- Eine Auslegung über den Plan hinaus: `Verweise(vec![])` liefert `KeinText`. Die Hülle liefert die leere Liste nicht; ein Absturz oder ein still leerer Filtertext wären die Alternativen.
- Modulkopf: zweiter Ausgang in der Skizze, Abschnitt `# Eine zweite Deutung: was aus der Ablage in den Filter kommt (Runde 21)` mit den fünf Schritten, Doppelpunkt, kein `http:`-Zweig, `#[must_use]`.
- 13 neue Proben im Prüfmodul, alle Fälle aus dem Planschritt (C2.1–C2.7, C2.10, C1.8, `/`, nicht lokaler Verweis, leere Verweisliste).

## Verifikation

- `cargo test -p krk-core --lib zwischenablage` — exit 0, 17 Proben grün.
- `make check` — exit 2, bricht nach `test` ab. Einziger roter Befund: `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei` (`tests/verzeichnis.rs:3257`), planmäßig, Schritt 9 zieht sie nach. Alle übrigen Prüfziele grün.
- `make fmt-check lint` — exit 0 (die zwei Stationen, die `make check` nach dem Abbruch nicht mehr erreicht hat).

Während des ersten Laufs übersetzte `tests/verzeichnis.rs` nicht (`Muster` statt `String` an `Durchlauf::starten`): die parallele Baustelle von Schritt 1. Beim `make check` danach war sie durch.
