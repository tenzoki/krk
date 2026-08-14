# Schritt C1 — die Regel der Rückschritt-Taste als reine Funktion

**Datum:** 260814-2254
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang C, Schritt C1
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C6.10, dazu die Tafel aus `## Die Rückschritt-Taste und was sie erreicht`

---

## Was umgesetzt ist

`crates/krk-ui/src/kommandos/rueckschritt.rs` (neu)

- `pub enum Rueckschritt { ZeichenZurueck, Nichts, InDenPapierkorb }`, die drei Senken des Unterbaums aus dem dritten Bild des Spec. Als Aufzählung und nicht als zwei Wahrheitswerte, weil „nichts" ein eigener Ausgang ist und kein Sonderfall von „räumen".
- `#[must_use] pub fn rueckschritt(filtertext_steht: bool, wiederholung: bool, merker: bool) -> (Rueckschritt, bool)`. Der Rumpf ist ein `match` über das Tripel mit genau den vier Zeilen der Tafel, ohne Auffangzweig; die Vollständigkeit hält der Übersetzer.
- Keine Zeile AppKit, keine Zeile aus `krk_core`. Das Modul steht neben `zulaessigkeit.rs` und ist ohne Fenster prüfbar.
- Der Modulkopf schreibt aus: die drei Wahrheitswerte samt ihrer Herkunft, warum die zweite Größe des Spec in zwei davon zerfällt (C1.18 gegen C1.20), die drei Größen, an denen die Regel **nicht** hängt (Treffer des Filtertextes, bestehende Auswahl, „Deep"), warum die Regel hinter `zulaessigkeit::zulaessig` steht und nicht darin, und dass `cmd+delete`, `f8`, `opt+cmd+delete` und `ctrl+delete` sie nie erreichen.
- Proben: sechs. `die_tafel_aus_acht_faellen_geht_auf` schreibt alle acht Wahrheitskombinationen mit Ausgang und Merker danach aus, in der Form der Tafel aus `zulaessigkeit.rs`. Vier Proben tragen die vier Wege der Spec-Tabelle einzeln mit ihrer Begründung (C1.14/C1.15/C6.9, C1.16, C1.18, C1.20). Die sechste hält fest, dass der Merker nicht `filtertext_steht` in Verkleidung ist: der letzte Anschlag mit Filtertext setzt ihn, der nächste ohne Filtertext liest ihn.

`crates/krk-ui/src/kommandos/mod.rs`

- `pub mod rueckschritt;` dazu, „Sechs Module" → „Sieben Module", ein Eintrag in der Modultafel, die zwei Zählwörter der Reihenfolge nachgezogen („vor den fünf anderen" → „sechs", „vor den vier übrigen" → „fünf").
- Ein neuer Absatz, der `rueckschritt` an die dritte Stelle des Weges setzt und begründet, warum die Frage nicht in `zulaessigkeit` gehört: dort liegt das nachgeschlagene Kommando vor und nicht der Tastendruck, `delete` und `cmd+delete` sind zu diesem Zeitpunkt dasselbe `Kommando::InPapierkorb`, und eine Antwort dort graute den Menüeintrag aus (C1.19, C6.11).

## Ein Befund am Bauverfahren, im Modul selbst aufgelöst

**Die Regel hat bis Schritt C2 keinen Aufrufer, und `make check` bricht darüber ab.** `krk-ui` ist ein Binärziel; `pub` allein ist dort keine Verwendung, und `cargo clippy --all-targets -- -D warnings` macht `dead_code` zum Fehler.

Die Ausnahme steht als `#[cfg_attr(not(test), expect(dead_code, reason = "…"))]` an beiden Stücken, und beide Hälften der Schreibweise sind nötig:

- **`expect` statt `allow`**, weil `expect` das Ablaufdatum mitbringt, das der Modulkopf von `editormodell.rs` an einer stehenden Ausnahme vermisst: sobald C2 den Aufrufer setzt, ist die Erwartung unerfüllt und der Bau hält an, bis die zwei Zeilen weg sind. Es ist das erste `#[expect]` in diesem Baum.
- **`cfg_attr(not(test), …)` davor**, weil `--all-targets` das Binärziel zweimal übersetzt. Im Lauf mit `cfg(test)` rufen die Proben beide Stücke, dort wäre die Erwartung schon heute unerfüllt. Ohne die Bedingung bleibt `make check` stehen — einmal gefahren und gesehen.

Beides steht mit dieser Begründung im Modulkopf, damit C2 die Zeilen nicht sucht, sondern findet.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün: Bau, Proben, Formatprüfung, Clippy unter `-D warnings`. Die sechs neuen Proben einzeln nachgefahren mit `cargo test -p krk-ui rueckschritt`: 6 passed.

## Datensätze

- Neu abgelegt: `issues/260814-2254_o_c6-10-sagt-zwei-groessen-und-keine-dritte-die-signatur-traegt-drei-wahrheitswerte.md`
- Plan: Schritt C1 auf `[DONE]`

## Für C2 nachgesehen, nichts steht im Weg

Der Schritt C2 ist mit dem Baum, den C1 hinterlässt, so zu bauen, wie der Plan ihn beschreibt. Drei Stellen nachgeprüft: `Tastendruck` (`krk-core/src/tasten/mod.rs:60`), `Maske::ist_leer` (`krk-core/src/tasten/normalisierung.rs:119`) und `code_von_pflicht` (`krk-core/src/tasten/parser.rs:357`) stehen alle drei da und tragen die Prüfung „nackte Rückschritt-Taste" wie geplant. Die Aufruferzählung, die C2 vorsieht, geht ebenfalls auf: `quellbaum::aufrufstellen` weist eine Fundstelle nur ab, wenn das Zeichen davor alphanumerisch oder `_` ist, und zählt `rueckschritt::rueckschritt(` deshalb genau einmal, obwohl Modul und Funktion denselben Namen tragen.

## Nicht angefasst

Alles außerhalb der zwei genannten Dateien. Insbesondere `appkit/ereignisse.rs` und `appkit/anwendung.rs` (Schritt C2), `zulaessigkeit.rs`, `resources/default-keymap.toml`.

Nicht committet — das ist Sache des Nutzers.
