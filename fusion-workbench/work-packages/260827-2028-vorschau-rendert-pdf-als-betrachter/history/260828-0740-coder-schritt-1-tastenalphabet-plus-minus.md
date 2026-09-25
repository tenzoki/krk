# Schritt 1: Das Tastenalphabet trägt `plus` und `minus`

**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Plan:** `planning/260828-0712_p_plan-vorschau-rendert-pdf-als-betrachter.md`, Schritt 1

## Was getan ist

- `crates/krk-core/src/tasten/parser.rs`: neue `pub const fn zeichen_des_namens(name) -> Option<char>` (Einbuchstabenregel, `plus` → `+`, `minus` → `-`). `Taste::kennung` fragt sie statt der Namenslänge. `zeichen_als_kennung` lässt ein Zeichen genau dann zu, wenn ein Name der Tabelle es nach `zeichen_des_namens` trägt. `TASTEN` wächst auf 63 um `dokumentiert("plus", 69, "kVK_ANSI_KeypadPlus")` und `dokumentiert("minus", 78, "kVK_ANSI_KeypadMinus")`; die Codes sind am SDK nachgelesen (`HIToolbox/Events.h:246` 0x45 und `:250` 0x4E). Modulkopf und Tabellenkopf nennen die zwei Zeichentasten, die Zehnerblock-Ausnahme mit Grund und den offenen Datensatz `260828-0712` zur US-Hälfte.
- Proben: `jede_taste_traegt_genau_eine_kennung_und_keine_zwei_dieselbe` prüft gegen `zeichen_des_namens`; `die_tabelle_deckt_die_ganze_schreibweise_ab` nennt `plus` und `minus`; neu `plus_und_minus_gehen_ueber_das_zeichen_und_pageup_ueber_die_stelle` (C3.2, deutsche Hälfte und Zehnerblock: `Kombination::lesen("cmd+plus")` und `aus_tastendruck` mit gemeldetem `+` bei den Codes 30, 69 und 1000 treffen dieselbe Taste; ohne Zeichen trifft Code 69 nichts; `pageup` geht über die Stelle); `nur_ascii_buchstaben_ziffern_plus_und_minus_taugen_als_zeichenkennung` hält `+`, `-` zugelassen und `=` abgewiesen.
- `crates/krk-core/src/tasten/mod.rs`: Doc-Kommentar an `Tastendruck::zeichen` zählt `+` und `-` mit. Kein Codeänderung nötig: das Feld geht schon durch `zeichen_als_kennung`.
- `crates/krk-ui/src/appkit/menue.rs`: `zeichen_der_taste` verliert die eigene Einbuchstabenregel und ruft `taste.zeichen()`; Funktions- und Sondertasten unverändert. Die Proben `jede_taste_der_tabelle_hat_ein_menuekuerzel` und `keine_zwei_tasten_teilen_sich_ein_menuekuerzel` decken die zwei neuen Einträge mit ab.

## Verifikation

- Im Arbeitsbaum ist `make check` rot, und zwar ausschließlich durch die parallel laufenden Schritte 2, 4 und 5 (`Wirkungsbereich::Vorschau` und die drei Zoomkommandos ohne Zweig in `belegungsmodell.rs` und `fokus.rs`, ein `cargo fmt`-Diff in `belegung.rs` und `vorschaumodell.rs`, eine Kennung ohne Eintrag in der Belegung). Keine Meldung betrifft die drei Dateien dieses Schritts.
- Deshalb Abnahme an einer Kopie des Baums im Scratchpad, in der alle fremden Änderungen auf `git show HEAD:` zurückgesetzt sind und nur die drei Dateien dieses Schritts abweichen: `make check` — exit 0 (build, test, clippy `-D warnings`, fmt).
- Im Arbeitsbaum zusätzlich: `cargo test -p krk-core --lib tasten::` — 22 bestanden, 1 rot (`belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`, Schritt 2 wartet auf Schritt 3); `cargo clippy -p krk-core --lib` — exit 0.

## Hinweis für Schritt 2/3

`crates/krk-core/tests/belegung.rs:1241` `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte` verlangt für jede Zeichentaste der Auslieferungsbelegung `einbuchstabig && is_ascii_alphanumeric` und `taste.name == zeichen.to_string()`. Sobald Schritt 3 `cmd+plus` und `cmd+minus` in `default-keymap.toml` einträgt, wird sie rot. Die Datei gehört Schritt 2; die Probe muss dort auf `zeichen_des_namens(taste.name) == Some(zeichen)` umgestellt werden.
