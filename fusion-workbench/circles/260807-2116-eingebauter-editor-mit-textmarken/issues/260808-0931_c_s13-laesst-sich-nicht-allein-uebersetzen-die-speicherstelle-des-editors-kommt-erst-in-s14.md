# S13 laesst sich nicht allein uebersetzen: die Speicherstelle des Editors kommt erst in S14

**Datum:** 2026-08-08, 09:31
**Gefunden von:** `coder`, beim Umsetzen von S13
**Betrifft:** `circles/260807-2116-eingebauter-editor-mit-textmarken/planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`, Schritte S13, S14, S19

## Der Befund

S13 nennt als Abhaengigkeit "keine" und als Abnahmekriterium, dass
`cargo build --workspace` uebersetzt. Beides zusammen ist nicht einzuloesen.
`Bereich::Editor` in die Fensterzeile aufzunehmen zieht vier weitere Dateien
mit, und zwei davon gehoeren Schritten, die **nach** S13 stehen.

Gemessen, nicht abgeleitet: die vollstaendige Aenderung wurde in einer
abgeschirmten Kopie des Arbeitsbaums gebaut und gefahren. Erst mit allen fuenf
Dateien beenden `cargo test --workspace`, `cargo clippy --workspace
--all-targets` und `cargo fmt --all --check` sauber; mit weniger bricht der
Uebersetzer ab.

## Die vier Stellen ausserhalb von `fenstermodell.rs`

1. **`crates/krk-core/src/ablage/sitzung.rs`** — `Breiten` und `Sichtbarkeit`
   tragen kein Feld `editor`. Ohne die beiden Felder haben
   `Fenstermodell::sichtbar`, `breite` und `breite_setzen` fuer den Editor
   keine Speicherstelle. **Das ist der Inhalt von S14, und S14 haengt an S13** —
   die Abhaengigkeit laeuft im Kreis.

2. **`crates/krk-core/tests/ablage.rs`** — `beispielsitzung()` baut `Breiten`
   und `Sichtbarkeit` als Strukturliterale ohne `..Default::default()`
   (`ablage.rs:112` und `:118`). Ein neues Feld haelt hier den Bau an. Ebenfalls
   S14.

3. **`crates/krk-ui/src/appkit/aufteilung.rs`, `sichtbar_im`** — eine
   vollstaendige Fallunterscheidung ueber `Bereich` (`aufteilung.rs:280-287`).
   Ein fuenfter Wert haelt den Bau an. **Diese Stelle traegt kein Schritt des
   Plans**; Befund 6 zaehlt zwei Stellen auf und diese ist eine dritte. Sie ist
   die neunte vollstaendige Fallunterscheidung ueber `Bereich`, waehrend das
   Abnahmekriterium von S13 von acht spricht — die acht liegen alle in
   `fenstermodell.rs`.

4. **`crates/krk-ui/src/appkit/aufteilung.rs`, die drei Breitenwege** —
   `gemessene_breiten` liefert `[f64; 4]` und wird an
   `Fenstermodell::breiten_uebernehmen` gereicht (`anwendung.rs:1707` und
   `:2472`); dazu bauen `gemessene_breiten(teiler)` und
   `gemessene_sichtbarkeit` je ein Strukturliteral. Das ist der Inhalt von
   **S19**, und S19 haengt ueber S18 und S17 an S16.

## Was daraus folgt

S13, S14 und der `aufteilung.rs`-Anteil von S19 sind **ein** Uebersetzungsstand
und nicht drei. Sie muessen zusammen landen, oder S14 muss vor S13 gezogen
werden und S19 seinen `gemessene_breiten`-Anteil abgeben.

Der `sichtbar_im`-Zweig gehoert keinem Schritt und muss einem zugeordnet
werden. Sachlich gehoert er zu dem Schritt, der `Bereich::Editor` anlegt.

## Stand

Umgesetzt ist der Teil von S13, der ohne die vier Stellen uebersetzt: die
Beseitigung der zweiten Wahrheit ueber die festen Bereiche aus Befund 6.
`ist_beweglich` ist von `matches!` zu einer vollstaendigen Fallunterscheidung
geworden, und die Literalliste `[Bereich::Lesezeichen, Bereich::Vorschau]` in
`bereichsbreiten` ist durch den Filter ueber `ist_beweglich` ersetzt. Eine
Probe haelt fest, dass die Aufzaehlung der festen Bereiche nur noch einmal
vorkommt.

**Offen bleibt der Kern von S13**: `Bereich::Editor` selbst mit seinen
Mindest- und Anfangsbreiten und `[f64; 5]`.

## Abschlussnotiz, 260808

**Geschlossen.** Der Befund traf zu, und die vorgeschlagene Auflösung ist
gefahren: S13, S14 und der `aufteilung.rs`-Anteil von S19 sind als **ein**
Übersetzungsstand gelandet. Die Zahlen dieses Datensatzes stammten aus einer
abgeschirmten Kopie; sie sind jetzt am Arbeitsbaum bestätigt.

Die fünf Dateien, wie der Befund sie aufzählt:

- `crates/krk-ui/src/fenstermodell.rs` — `Bereich::Editor` als fünfter Wert
  hinter `Vorschau`, `ALLE: [Bereich; 5]`, `index()` gleich 4,
  `mindestbreite()` gleich 320 und `anfangsbreite()` gleich 460 mit der
  Herleitung aus `### Frage 6` als Kommentar, die Zweige in `sichtbar`,
  `umschalten`, `breite` und `breite_setzen`, `[f64; 5]` in
  `breiten_uebernehmen` und `bereichsbreiten`.
- `crates/krk-core/src/ablage/sitzung.rs` — `Breiten::editor: Option<f64>` mit
  `skip_serializing_if`, `Sichtbarkeit::editor: bool` mit dem Vorgabewert
  `false`.
- `crates/krk-core/tests/ablage.rs` — die beiden Strukturliterale in
  `beispielsitzung()`, dazu drei neue Proben.
- `crates/krk-ui/src/appkit/aufteilung.rs` — der `Editor`-Zweig in
  `sichtbar_im`, `Aufteilung::gemessene_breiten` auf `[f64; 5]`, das
  `editor`-Feld in den Literalen von `gemessene_breiten(teiler)` und
  `gemessene_sichtbarkeit`.

`cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace
--all-targets` und `cargo fmt --all --check` beenden alle sauber; `krk-ui`
trägt jetzt 195 Proben.

**Die dritte Feststellung des Befunds ist ebenfalls erledigt**: der
`sichtbar_im`-Zweig, der keinem Schritt gehörte, ist S13 zugeordnet, weil S13
den Bereich anlegt. Der Plan hält das bei S13 fest, samt der Berichtigung, dass
es **neun** vollständige Fallunterscheidungen über `Bereich` sind und nicht
acht.

**Was von S19 offen bleibt**, ist im Plan bei S19 vermerkt: der
`anwendung.rs`-Anteil mit `breite_aendern` und `sitzung_bauen`.
