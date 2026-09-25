# S13: `Bereich::Editor` im Fenstermodell

**Datum:** 2026-08-08, 09:31
**Agent:** `coder`
**Schritt:** S13 aus `planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`
**Status:** Teilweise umgesetzt — der Kern des Schrittes ist blockiert

## Was umgesetzt ist

`crates/krk-ui/src/fenstermodell.rs`, die beiden Stellen aus Befund 6 des
Plans, also die Beseitigung der zweiten Wahrheit ueber die festen Bereiche:

- `Bereich::ist_beweglich` ist von `matches!` zu einer vollstaendigen
  Fallunterscheidung geworden. Ein neuer Bereich haelt jetzt den Bau an, statt
  still als unbeweglich durchzugehen.
- `bereichsbreiten` fuehrt die festen Bereiche nicht mehr als Literalliste
  `[Bereich::Lesezeichen, Bereich::Vorschau]`, sondern filtert
  `Bereich::ALLE` ueber `!ist_beweglich() && sichtbar`. Damit steht die
  Aufzaehlung "welche Bereiche sind fest" nur noch in `ist_beweglich`.
- Der Kopfkommentar von `bereichsbreiten` haelt ausdruecklich fest, dass hier
  die eine Breitenregel des Programms steht und dass die Reihenfolge von
  `Bereich::ALLE` eine Zusage ist: wer vorn steht, behaelt seine Wunschbreite,
  wenn es eng wird.
- Neue Probe `jeder_feste_bereich_bekommt_seine_breite_ohne_zweite_aufzaehlung`.

`cargo test -p krk-ui`: 191 bestanden, 0 gescheitert. `cargo clippy --workspace
--all-targets` und `cargo fmt --all --check` sauber.

## Was nicht umgesetzt ist und warum

`Bereich::Editor` selbst. Der Schritt laesst sich nicht allein uebersetzen; der
Befund steht als
`issues/260808-0931_o_s13-laesst-sich-nicht-allein-uebersetzen-die-speicherstelle-des-editors-kommt-erst-in-s14.md`.
Vier Stellen ausserhalb von `fenstermodell.rs` muessen mit, zwei davon in
`crates/krk-core/` (Inhalt von S14) und zwei in `crates/krk-ui/src/appkit/`
(eine davon Inhalt von S19, eine keinem Schritt zugeordnet). Der Auftrag dieser
Sitzung schliesst beide Verzeichnisse aus, weil dort gleichzeitig andere
Schritte arbeiten; waehrend dieser Sitzung hat S11 dort tatsaechlich
`crates/krk-core/tests/ablage.rs` geaendert.

## Was gemessen ist

Die vollstaendige Aenderung wurde in einer abgeschirmten Kopie des Arbeitsbaums
gebaut und gefahren. Mit allen fuenf Dateien: `cargo test --workspace` sauber
(darunter `krk-ui` mit 193 Proben), `cargo clippy --workspace --all-targets`
ohne Meldung, `cargo fmt --all --check` sauber. Die Zahlen unten stammen aus
diesem Lauf und sind nicht gerechnet, sondern von den Proben abgenommen.

### Die Aenderung in `fenstermodell.rs`

- `Bereich::Editor` als fuenfter Wert **hinter** `Vorschau`, `ALLE: [Bereich; 5]`,
  `index()` gleich 4.
- `mindestbreite()` gleich 320,0 und `anfangsbreite()` gleich 460,0, beide mit
  der Herleitung aus `### Frage 6` als Kommentar.
- Zweige in `sichtbar`, `umschalten`, `breite`, `breite_setzen`.
- `breiten_uebernehmen` und der Rueckgabetyp von `bereichsbreiten` von
  `[f64; 4]` auf `[f64; 5]`.
- Vier neue Proben, dazu die Anpassung von
  `der_auslieferungszustand_zeigt_alle_vier_bereiche` (der Editor ist beim
  allerersten Start ausgeblendet) und der Feldvergleich in
  `ein_einziges_dateifenster_nimmt_die_ganze_breite`.

### Die vier Stellen ausserhalb

- `crates/krk-core/src/ablage/sitzung.rs`: `Breiten::editor: Option<f64>` mit
  `skip_serializing_if`, `Sichtbarkeit::editor: bool` mit dem Vorgabewert
  `false`.
- `crates/krk-core/tests/ablage.rs`: die beiden Strukturliterale in
  `beispielsitzung()`.
- `crates/krk-ui/src/appkit/aufteilung.rs`: der `Editor`-Zweig in
  `sichtbar_im`; `gemessene_breiten` von `[f64; 4]` auf `[f64; 5]`; das
  `editor`-Feld in den Literalen von `gemessene_breiten(teiler)` und
  `gemessene_sichtbarkeit`.

### Die Breiten bei 1280 verfuegbaren Punkten

Editor sichtbar, Vorschau ausgeblendet, beide Dateifenster sichtbar,
Anfangsbreiten:

| Lesezeichenleiste | Leiste | links | rechts | Vorschau | Editor |
|---|---|---|---|---|---|
| offen | 180 | 320 | 320 | 0 | 460 |
| geschlossen | 0 | 410 | 410 | 0 | 460 |

Der Editor steht beide Male auf 460, also 36 Prozent der Fensterbreite. Die
Leiste geht zu Lasten der beiden Dateifenster und nicht zu Lasten des Editors —
genau die Festlegung des Nutzers vom 260808, und sie faellt aus der bestehenden
Regel an, weil `Bereich::ALLE` die Leiste vor den Editor stellt. Eine zweite
Regel entsteht nicht.

Am engen Fenster (900 Punkte) gewinnt das Mindestmass der Dateifenster: Leiste
180, Dateifenster je 240, Editor 240 statt seiner gewuenschten 460.

## Verbleibende Arbeit

S13, S14 und der `aufteilung.rs`-Anteil von S19 sind ein Uebersetzungsstand.
Sie zusammen zu dispatchieren, oder S14 vor S13 zu ziehen, loest den Kreis auf.
Der `sichtbar_im`-Zweig braucht einen Schritt, der ihn traegt.
