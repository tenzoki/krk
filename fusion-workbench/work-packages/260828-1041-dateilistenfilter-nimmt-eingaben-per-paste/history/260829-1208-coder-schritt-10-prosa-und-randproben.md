# Coder: Schritt 10 — die Prosa des Menüs und des Kommandoverzeichnisses, zwei Randproben

**Circle:** 260828-1041-dateilistenfilter-nimmt-eingaben-per-paste
**Plan:** planning/260829-1102_p_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md, Schritt 10
**Status:** Complete

## Geändert

- `crates/krk-ui/src/appkit/menue.rs`: Modulkopf, der Absatz zur Ausnahme von der `true`-Antwort (bisher `:100-101`) nennt alle drei Selektoren als vom Delegierten beantwortet und der Regel unterstellt, `paste:` füllt den Filtertext; der Absatz zum Einhängepunkt (bisher `:126-134`) sagt, dass beide Runden ihn ganz besetzt haben, der Circle gefahren ist und der offene Datensatz zur Dateizwischenablage bleibt. Doc der Tafel `GEMESSEN`: „`copy:`, `cut:` und `paste:` ja“, mit dem Namen der Probe beim Delegierten. Die Tafel selbst zeichengleich.
- `crates/krk-ui/src/kommandos/mod.rs`: Verzeichniszeile `operationen` nennt die vier Sätze des Einfügens (C2 der Runde 21); der Absatz zu `zulaessigkeit` sagt, dass der zweite Eingang seit der Runde 21 drei Selektoren bedient und kein dritter entsteht.
- `crates/krk-ui/src/appkit/betrachter.rs`: Probe umbenannt in `nspasteboard_steht_nicht_im_betrachter_und_copy_cut_und_paste_stehen_an_genannten_stellen`, dritte Nadel `concat!("unsafe(method(pas", "te:))")` mit Erwartung `[("krk-ui/src/appkit/anwendung.rs", 1)]`; Doc der Probe und Modulkopf ziehen nach.
- `crates/krk-ui/src/belegungsmodell.rs`: neue Probe `die_tippsuche_kennt_keinen_platzhalter` (C5.8, B9) über `trefferzeilen`. **Sie steht im Modul `suchproben` und nicht im Modul `tests` (`:915-`), das der Plan nennt:** `trefferzeilen` wohnt in `suchproben`, und die Probe braucht daneben `modell()` und `tippen` von dort. Sie hält die Treffer gegen die Zeilen mit wörtlichem `a*b` (in der Auslieferungsbelegung keine), gegen die Zielzeile, und daneben, dass Zeilen mit `a` vor `b` existieren, die Platzhalter-Lesart also Treffer hätte. Sie ruft weder `traegt_die_folge` noch `Muster`, damit die Zählprobe aus Schritt 9 bei drei Vergleichsrufern bleibt.

## Verifikation

`make check` — exit 0 (Bau, Tests, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`). `cargo fmt --all` lief vorher über den ganzen Arbeitsbereich; ob es in `tabelle.rs` oder `tests/verzeichnis.rs` (S7, S9) etwas umgebrochen hat, habe ich nicht geprüft, weil kein baumweites git-Kommando erlaubt war.

Kein git-Kommando gelaufen, nichts committet.
