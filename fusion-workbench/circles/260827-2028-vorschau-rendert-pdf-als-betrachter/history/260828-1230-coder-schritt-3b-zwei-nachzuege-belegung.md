# Schritt 3b: Zwei Nachzüge zur Belegung nach `cmd+plus`, `cmd+minus`, `cmd+0`

**Agent:** coder · **Datum:** 260828-1230 · **Status:** Complete

## Was geändert ist

- `crates/krk-core/tests/belegung.rs`, Probe `jede_ausgelieferte_kombination_traegt_die_kennung_ihrer_tastensorte`: die Bedingung `einbuchstabig && is_ascii_alphanumeric` ist gefallen. Eine Zeichentaste muss jetzt `zeichen_des_namens(taste.name) == Some(zeichen)` erfüllen, eine Stellentaste `zeichen_des_namens(taste.name) == None`. Der Doc-Kommentar nennt die eine Regel und ihren Ort. `krk_core::tasten::parser::zeichen_des_namens` ist `pub` in einem `pub mod`; kein neuer Export nötig.
- `crates/krk-ui/src/belegungsausgabe.rs`: die Zahlen 79 und 85 stehen nicht mehr im Modulkopf, in der Tabelle, am Zweig von `wirkung` und im Doc der Probe. Ersetzt durch die Aussage „jede Funktion mit Kommando, so viele wie `Kommando::KENNUNGEN` Einträge hat“; die Probe `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` hält diese Gleichheit schon (`mit_kommando == Kommando::KENNUNGEN.len()`), und die sechs Textbefehle hält sie ebenso. Keine Zahl im Text ist damit mehr nachzuziehen.
- `crates/krk-ui/src/belegungsmodell.rs` (außerhalb der Dispatch-Liste, dritter Nachzug): die Probe `die_beschriftung_nennt_die_taste_auf_einer_deutschen_tastatur` verlangte für jede Zeichentaste den Großbuchstaben des Zeichens als letztes Glied der Anzeigeform und wurde mit `cmd+plus` rot (`anzeige` schreibt `Cmd+Plus`, die Probe erwartete `+`). Die Regel lautet jetzt: das letzte Glied ist der Tastenname in der Schreibweise von `anzeige()` (`teilanfang_gross`), also `Y`, `0`, `Plus`, `Minus`. `anzeige` selbst ist unverändert; ein nacktes `+` als letztes Glied einer mit `+` gefügten Form wäre unlesbar, und eine Übersetzungsliste ist nach dem Plan der Runde 3 ausgeschlossen.

## Verlauf

Erster `make check` rot an `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` (Ontocoder hatte noch nicht gespeichert). Nach Wartezeit: keymap trägt die drei Einträge; zweiter Lauf rot an der Beschriftungsprobe in `belegungsmodell.rs`. Dritter Lauf grün.

Verification: `make check` — exit 0
