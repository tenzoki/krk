# Coder: Schritt 2 der Runde 19 — die zwei Schlüssel in der Gestalt der Profildatei

**Status:** Complete

## Was geändert ist

`crates/krk-core/src/leseprofil/datei.rs`, sonst nichts:

- `Zaehlungsdatei` trägt `typ: Option<Typdatei>` und `versteckt: Option<bool>`; `deny_unknown_fields` bleibt.
- Neue Aufzählung `Typdatei { Datei, Ordner, Verknuepfung }` mit `#[serde(rename_all = "lowercase")]`, Doc-Kommentar nach dem Vorbild von `Anzeigedatei` (warum neben `Typ`; Umschrift nach Festlegung 5 des Plans; ein unbekannter Wert kostet die ganze Datei).
- Zuordnung `typ(Option<Typdatei>) -> Option<Typ>`, vollständig ohne Auffangzweig.
- `baustein_pruefen` reicht `typ` und `versteckt` (`unwrap_or(false)`) in `Baustein::Zaehlung` durch; der Platzhalterkommentar aus Schritt 1 ist weg.
- Modulkopf: die weiteste Reichweite nennt jetzt auch einen unbekannten `typ`-Wert und ein `versteckt`, das kein Wahrheitswert ist.

Kriterien: C3.1, C3.2, C3.6 (die Proben dazu sind Schritt 5).

## Verifikation

- `make check` — exit 2: rot allein in `crates/krk-ui` (Schritt 4 des parallelen Coders, `Inhalt::Metadaten` in Umstellung; `cargo fmt --check` meldet nur `krk-ui`-Dateien).
- `cargo test -p krk-core` — exit 0.
- `cargo clippy -p krk-core --all-targets` — exit 0.
- `rustfmt --check --edition 2024 crates/krk-core/src/leseprofil/datei.rs` — sauber.

Planschritt 2 auf `[DONE]` gesetzt. Nicht committet.
