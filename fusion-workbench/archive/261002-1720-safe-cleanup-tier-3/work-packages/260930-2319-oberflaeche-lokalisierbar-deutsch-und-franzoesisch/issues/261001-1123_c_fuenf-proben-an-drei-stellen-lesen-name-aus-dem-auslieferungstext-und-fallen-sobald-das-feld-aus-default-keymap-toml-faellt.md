Fuenf Proben an drei Stellen lesen `name` aus dem Auslieferungstext und fallen, sobald das Feld aus `default-keymap.toml` faellt
---
Drei Stellen in Prüfcode lesen das Feld `name` nicht ueber `Belegung::bauen`, sondern am rohen `belegung::AUSLIEFERUNGSTEXT`: ein Helfer liest es per `toml::Table` aus dem ersten Block, zwei Proben suchen einen wörtlichen Blocktext mit `name = "…"` per `contains`/`matches`. Schritt 5 des Plans (`43518fd`) hat den Leser umgestellt und diese drei Stellen nicht getroffen; Schritt 6 (`261001-0850_*_plan-oberflaeche-folgt-der-systemsprache-deutsch-franzoesisch-englisch.md`) entfernt das Feld aus `resources/default-keymap.toml`, und danach sind fuenf Proben rot.
---
**Filed by:** data-implementer, Kai Stalmann <kai@stalmann.org>

**Evidenz.** `cargo test --workspace --no-fail-fast` auf dem Baum mit der Auslieferungsbelegung ohne `name`-Zeilen (`grep -c '^name = ' resources/default-keymap.toml` ist 0), Lauf vom 261001-1123, am Stand `43518fd` plus diese eine Datei:

```
crates/krk-core/tests/ablage.rs:5183:32: die erste Funktion nennt kein name
  eine_keymap_in_falscher_schreibweise_gilt_als_beschaedigt
  eine_namensliste_jenseits_der_kuerzungsgrenze_endet_mit_und_n_weitere
crates/krk-core/tests/belegung.rs:1218:5: der Block der Werkseinstellungen steht nicht genau einmal in der Auslieferung (left: 0, right: 1)
  eine_eigene_belegung_ohne_die_werkseinstellungen_laedt_und_fuehrt_sie_unbelegt
crates/krk-ui/src/menuemodell.rs:974:9: der Eintrag von editor_sichern steht nicht mehr in dieser Form in der Auslieferung
  menuemodell::tests::bei_einer_geteilten_kombination_behaelt_der_fruehere_befehl_das_kuerzel
  menuemodell::tests::keine_zwei_eintraege_tragen_dieselbe_kombination
```

Jedes andere Prüfziel des Arbeitsbereichs ist in demselben Lauf gruen (26 gruene `test result`-Zeilen); die Belegungsproben `jede_funktion_der_auslieferung_hat_einen_schluessel`, `jede_kennung_beider_listen_steht_in_der_auslieferung` und die Kopfzahlenprobe in `belegung.rs` laden die Datei ohne `name`. Die Dispatch-Aussage zu Schritt 6, keine Probe lese `name` aus der Auslieferungsfassung, ist widerlegt; die drei Stellen lesen den Text und nicht den Leser, weshalb die Umstellung des Lesers sie nicht erreicht hat.

**Die drei Stellen** (Zeilen am Stand `43518fd`):

1. `crates/krk-core/tests/ablage.rs`, `erste_ausgelieferte_funktion` (Zeile 5171): liefert `(id, name)` des ersten Blocks und bricht ohne `name` ab. Die zwei Rufer (Zeilen 5212 und 5614) schreiben daraus eine eigene `keymap.toml` mit einer `name`-Zeile (die `format!`-Zeilen 5216 und 5632). Beide pruefen die Schreibweise einer Kombination und die Kuerzung der Namensliste, nicht die Beschriftung.
2. `crates/krk-core/tests/belegung.rs`, Zeile 1217: `block` ist der woertliche Werkseinstellungen-Block mit `name = "Auf Werkseinstellungen zurücksetzen…"` und wird per `matches(...).count() == 1` im Auslieferungstext gesucht und dann herausgeschnitten.
3. `crates/krk-ui/src/menuemodell.rs`, `mit_geteilter_kombination` (Zeile 971): `alt` ist der woertliche Block von `editor_sichern` mit `name = "Sichern"`, per `contains` gesucht und per `replacen` gegen `cmd+2` getauscht.

Seit Schritt 5 ist `Eintrag.name` ein `Option<String>` mit `default`, beim Lesen nie gelesen und in einer Nutzerdatei geduldet (Doc-Kommentar an `Eintrag`, `crates/krk-core/src/tasten/belegung.rs`); keine der fuenf Proben braucht das Feld fuer ihre Aussage.

**Behebung** (Prüfcode, `code-implementer`): an Stelle 1 liefert der Helfer allein die Kennung und die zwei `format!`-Zeilen schreiben keine `name`-Zeile; an den Stellen 2 und 3 fallen die `name`-Zeilen aus den woertlichen Bloecken, sodass sie den Text ohne das Feld treffen. Ein fester Ersatzwert fuer `name` ginge an Stelle 1 ebenso, hielte aber eine Zeile am Leben, die der Leser nie liest.

**Abnahme.** `make check` gruen auf dem Baum mit `resources/default-keymap.toml` ohne `name`-Zeilen; die fuenf genannten Proben laufen durch; `grep -n 'feld("name")' crates/krk-core/tests/ablage.rs` liefert nichts, und `grep -rn 'name = \\"' crates/krk-core/tests/belegung.rs crates/krk-ui/src/menuemodell.rs` trifft keine Zeile mehr, die am Auslieferungstext sucht.

---
Resolved: 261001, code-implementer. Stelle 1: `erste_ausgelieferte_funktion` in `crates/krk-core/tests/ablage.rs` heisst jetzt `erste_ausgelieferte_kennung`, liefert allein die Kennung, und die zwei Pruefdateien der Rufer (`eine_keymap_in_falscher_schreibweise_gilt_als_beschaedigt`, `eine_namensliste_jenseits_der_kuerzungsgrenze_endet_mit_und_n_weitere`) fuehren nur noch `id` und `tasten`; der Kommentar ueber dem zweiten Block nennt die zwei Pflichtfelder statt „jedes Pflichtfeld“. Stelle 2: der woertliche Werkseinstellungen-Block in `crates/krk-core/tests/belegung.rs` (`eine_eigene_belegung_ohne_die_werkseinstellungen_laedt_und_fuehrt_sie_unbelegt`) steht ohne `name`-Zeile und trifft den Auslieferungstext genau einmal. Stelle 3: `mit_geteilter_kombination` in `crates/krk-ui/src/menuemodell.rs` sucht und ersetzt den `editor_sichern`-Block ohne `name`-Zeile. Keine Probe gestrichen, keine Erwartung geschwaecht; `resources/default-keymap.toml` unberuehrt. Die zwei verbliebenen Treffer von `grep -rn 'name = \\"'` in `belegung.rs` (`eine_vollstaendige_nutzerbelegung_mit_notizzettel_laedt_ohne_ersetzung`) suchen in der von `Belegung::sichern` geschriebenen Nutzerdatei und lesen den Namen ueber `Funktion::name()`, nicht am Auslieferungstext; sie waren und bleiben gruen. Abnahme: `make check` Exit 0 auf dem Baum ohne `name`-Zeilen in der Auslieferung; `grep -n 'feld("name")' crates/krk-core/tests/ablage.rs` liefert nichts.
