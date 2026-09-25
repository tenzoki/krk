Die Belegungsprüfung bindet `cmd+right` noch an das Öffnen und schlägt seit der Umbelegung auf die nackten Pfeile fehl

---

`crates/krk-core/tests/belegung.rs:698` führt in der Prüfung
`jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` die Zeile

    ("cmd+right", Kommando::Oeffnen),

Der Nutzerentscheid vom 260805 hat `oeffnen` auf `right` und `ordner_aufwaerts` auf `left` neben `cmd+up` umbelegt; `cmd+right` und `cmd+left` stehen seither in keiner Tastenliste von `resources/default-keymap.toml`. Die Prüfung fällt damit auf `Nachschlag::Sprungmarke` und bricht mit `cmd+right trifft keine Funktion` ab (`crates/krk-core/tests/belegung.rs:702`).

Gemessen nach der Änderung:

    cargo test -p krk-core --test belegung
    test result: FAILED. 31 passed; 1 failed; 0 ignored

Vor der Änderung liefen dieselben 32 Prüfungen durch. Es ist die einzige fehlschlagende Prüfung der Datei; die inhaltlichen Prüfungen der Auslieferungsbelegung laufen alle durch, namentlich `die_auslieferungsbelegung_ist_konfliktfrei`, `jede_funktion_traegt_genau_eine_zeile_und_die_reservierte_keine_taste`, `die_ab_werk_freien_kombinationen_kommen_nicht_vor`, `keine_unbelegte_kombination_mit_zusatztaste_faellt_auf_die_sprungmarke` und `zwei_funktionen_desselben_zustellers_auf_einer_kombination_bleiben_ein_konflikt`.

---

## Das ist die zweite Auflage desselben Defekts

`issues/260804-1214_c_die-belegungspruefung-bindet-return-noch-an-das-oeffnen.md` beschreibt denselben Vorgang mit `return` statt `cmd+right`: dieselbe Prüfung, dieselbe Zeile, derselbe Abbruch, nur eine Umbelegung früher. Die Zeile trägt seit S11c das Beispiel, das die Umbelegung von damals hinterlassen hat, und die Umbelegung von heute macht es wieder falsch.

Das Muster ist damit belegt und nicht mehr vermutet: **jede Umbelegung von `oeffnen` bricht diese Prüfung.** `oeffnen` ist innerhalb eines Tages dreimal gewandert, von `return` über `cmd+right` auf `right`. Ob die Prüfung ihr Beispiel deshalb anders wählen sollte, ist eine Frage an den `coder` und nicht Teil dieses Defekts; die Prüfung hängt ihre Zusage an eine hingeschriebene Kombination, obwohl die Zusage selbst nur lautet, dass ein gebautes Kommando überhaupt an seiner ausgelieferten Taste hängt. Ein Beispiel, das die Kombination aus der Belegung liest statt sie zu wiederholen, trüge dieselbe Zusage und überstünde die nächste Umbelegung. Die vier übrigen Zeilen der Liste (`up`, `down`, `pageup`, `pagedown`) sind von der Frage mit betroffen.

## Warum der `ontocoder` es nicht selbst behoben hat

Der Auftrag vom 260805 begrenzt den Eingriff auf `resources/default-keymap.toml` und schließt `crates/`, `xtask/`, die Plandatei und den Spec ausdrücklich aus. Die Datenänderung verlangt hier eine Codeänderung, und die gehört dem `coder`.

## Was zu tun ist

Die Zeile auf die neue Belegung ziehen:

    ("right", Kommando::Oeffnen),

Die Aussage der Prüfung bleibt unverändert: jedes gebaute Kommando hängt an der Taste, die die Auslieferungsbelegung ihm gibt. Nur das Beispiel wechselt, so wie am 260804 schon einmal.

Der Kommentar in `crates/krk-core/tests/belegung.rs:153` zieht mit. Er begründet dort, warum die Eingabetaste ab Werk frei ist, mit dem Satz "nachdem der Einstieg in den Ordner auf cmd+right gewandert ist"; der Einstieg liegt seit dem 260805 auf `right`.

## Was daran hängt

Das Abnahmekriterium der Umbelegung verlangt `cargo test -p krk-core --test belegung` mit Rückgabewert 0. Solange diese Zeile steht, ist es nicht erfüllbar, und zwar unabhängig davon, ob die Belegungsdatei richtig ist.

Der Bau des Bündels hängt nicht daran: `include_str!` kompiliert die Belegungsdatei ein, sie ist gültiges TOML und konfliktfrei, und die Anwendung startet unverändert. Betroffen ist allein der Prüflauf.

---

Herkunft: gefunden bei der Umbelegung der Ordnernavigation auf die nackten Pfeiltasten am 260805-1356, beim Lauf des Abnahmekriteriums.

---

Resolved: 260805 — Die Ursache behoben statt der Zeile: `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` schreibt keine Kombination mehr hin, sondern liest ihre Paare aus `Kommando::KENNUNGEN` und der Auslieferungsbelegung. Die Zusage lautet jetzt ausbuchstabiert: zu jedem gebauten Kommando gibt es eine ausgelieferte Kombination, und der Nachschlag auf jede davon trifft dieses Kommando. Welche Kombination es ist, sagt allein `resources/default-keymap.toml`. Gemessen wird damit an allen 42 gebauten Kommandos statt an fünf.

Die Frage des `ontocoder` ist damit beantwortet, und zwar mit ja: die Prüfung soll ihr Beispiel aus der Belegung lesen. Die vier übrigen Zeilen der Liste (`up`, `down`, `pageup`, `pagedown`) sind mit derselben Änderung verschwunden.

Fünf weitere Prüfungen derselben Bauart sind mit behoben, gefunden über eine Probe-Umbelegung von `kopieren`: `beide_ausgelieferten_wege_treffen_dieselbe_funktion` führte sechs Zeilen aus Funktionstaste, Cmd-Kürzel und Kennung und sucht die mehrwegigen Funktionen jetzt selbst; `eine_bereits_vergebene_kombination_liefert_einen_konflikt_mit_dem_namen_der_anderen_funktion`, `dieselbe_kombination_zweimal_an_dieselbe_funktion_aendert_nichts`, `zuruecksetzen_stellt_die_eingebettete_tabelle_wieder_her` und `die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht` nannten `f5`, `shift+cmd+k` oder `ctrl+k` und nehmen jetzt die zwei neuen Helfer `ausgeliefert(kennung)` und `frei()`.

Der Kommentar in `die_ab_werk_freien_kombinationen_kommen_nicht_vor` nennt nicht mehr, wohin der Einstieg in den Ordner gewandert ist, sondern nur noch, dass er von der Eingabetaste weg ist; daran hängt die Zusage.

`resources/default-keymap.toml` ist nicht angefasst: SHA-256 vor und nach der Arbeit `4285656823b6722848a38a3503b9f01f43cbafa5ecbef9e832a9ef28d358f064`, `diff` gegen die vorher gezogene Sicherung leer. Die Proben liefen an einer Kopie des Baums unter dem Temporärverzeichnis.

Nachweis der Haltbarkeit, an der Kopie gemessen: `oeffnen` um `cmd+right` ergänzt — 32 von 32; `oeffnen` auf `ctrl+o` verschoben und `kopieren` von `f5`, `shift+cmd+k` auf `ctrl+shift+k` — 32 von 32; `oeffnen` ganz ohne Kombination — 2 Fehlschläge mit der Meldung "Oeffnen ist gebaut, und oeffnen traegt ab Werk keine Kombination", die Prüfung misst also weiter etwas.

Die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` laufen mit Rückgabewert 0.
