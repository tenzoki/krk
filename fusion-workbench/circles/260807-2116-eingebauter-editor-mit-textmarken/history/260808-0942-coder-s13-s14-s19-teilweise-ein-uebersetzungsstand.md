# S13, S14 und der `aufteilung.rs`-Anteil von S19 als ein Übersetzungsstand

**Datum:** 2026-08-08, 09:42
**Agent:** `coder`
**Schritte:** S13, S14 und der `aufteilung.rs`-Anteil von S19 aus
`planning/260808-0140_*_plan-eingebauter-editor-mit-textmarken.md`
**Status:** Vollständig

## Warum die drei zusammen gelandet sind

Ein voriger Lauf hat gemessen, dass S13 allein nicht übersetzt: `Bereich::Editor`
zieht vier Stellen außerhalb von `fenstermodell.rs` nach, und zwei davon gehören
Schritten, die nach S13 stehen. Die Abhängigkeit lief im Kreis, weil S14 die
Speicherstelle des Editors trägt und laut Plan an S13 hängt. Der Befund steht als
`issues/260808-0931_c_s13-laesst-sich-nicht-allein-uebersetzen-die-speicherstelle-des-editors-kommt-erst-in-s14.md`
und ist mit diesem Lauf geschlossen.

Der Teil von S13, der ohne die vier Stellen übersetzt, war bereits committet
(`fe022e7`): `ist_beweglich` als vollständige Fallunterscheidung und der Filter
darüber in `bereichsbreiten` statt der Literalliste. Dieser Lauf baut darauf auf.

## Was geändert ist

### `crates/krk-core/src/ablage/sitzung.rs`

`Breiten` trägt jetzt ein fünftes Feld `editor: Option<f64>` mit
`skip_serializing_if`, `Sichtbarkeit` ein viertes Feld `editor: bool`. Beide
Strukturen tragen schon seit der Runde 1 `#[serde(default)]`, deshalb macht das
neue Feld keine bestehende `session.toml` ungültig.

Der Vorgabewert von `Sichtbarkeit::editor` ist **`false`** und nicht `true` wie
bei den drei bestehenden Feldern. Der Grund steht als Kommentar an der Stelle:
beim allerersten Start hält der Editor keine Datei, und ein sichtbarer leerer
Editor nähme den Dateifenstern Platz für nichts.

### `crates/krk-core/tests/ablage.rs`

Die beiden Strukturliterale in `beispielsitzung()` tragen ihr neues Feld, und der
Auslieferungszustand wird zusätzlich darauf geprüft, dass der Editor ausgeblendet
steht. Drei neue Proben decken die drei Abnahmekriterien von S14 ab:

- `eine_sitzung_ohne_die_editorfelder_bleibt_lesbar` schreibt eine
  `session.toml` in der Form der Runde 1, mit `[breiten]` und `[sichtbar]`, aber
  ohne die beiden Editorfelder. Sie gilt nicht als beschädigt, `sichtbar.editor`
  ist danach `false`, `breiten.editor` ist `None`, und Vorschau und Leiste
  behalten ihre Werte.
- `die_editorbreite_ueberlebt_den_rundlauf_byteweise` schreibt, liest und
  schreibt erneut; die beiden Dateien sind byteweise gleich.
- `eine_nicht_gesetzte_editorbreite_steht_nicht_in_der_datei` hält fest, dass
  eine ungesetzte Editorbreite gar keine Zeile bekommt. Die einzige verbleibende
  Zeile mit diesem Namen ist die Sichtbarkeit, `editor = false`. Damit bleibt die
  Datei von Hand lesbar, wie C7 der Runde 1 es verlangt.

### `crates/krk-ui/src/fenstermodell.rs`

`Bereich::Editor` ist der fünfte Wert und steht **hinter** `Vorschau`, weil er
deren Stelle am rechten Rand einnimmt. `ALLE` ist `[Bereich; 5]`, `index()`
liefert 4, `mindestbreite()` liefert 320,0 und `anfangsbreite()` 460,0. Die
Herleitung beider Zahlen aus `### Frage 6` des Plans steht als Kommentar an der
Stelle. `sichtbar`, `umschalten`, `breite` und `breite_setzen` haben ihre Zweige;
`breiten_uebernehmen` und der Rückgabetyp von `bereichsbreiten` stehen auf
`[f64; 5]`.

Der Kopfkommentar von `bereichsbreiten` hält jetzt ausdrücklich fest, warum die
Festlegung des Nutzers vom 260808 ohne eine zweite Regel anfällt:
`Bereich::ALLE` stellt die Leiste vor den Editor, also weicht sie nicht, wenn
beide zugleich stehen, und die beiden Dateifenster rücken zusammen. Erst wenn ihr
Mindestmaß erreicht ist, gibt der Editor nach. **Eine zweite Breitenregel ist
nicht entstanden.**

Vier neue Proben, dazu zwei angepasste:

- `der_eingeblendete_editor_bekommt_seine_breite_und_die_dateifenster_den_rest`
- `am_engen_fenster_gewinnt_das_mindestmass_der_dateifenster`
- `der_ausgeblendete_editor_behaelt_seine_gespeicherte_breite`
- `die_leiste_weicht_dem_editor_nicht`
- `der_auslieferungszustand_zeigt_alle_bereiche_ausser_dem_editor` (umbenannt
  aus `..._zeigt_alle_vier_bereiche`)
- `ein_einziges_dateifenster_nimmt_die_ganze_breite` (fünftes Feld im Vergleich)

### `crates/krk-ui/src/appkit/aufteilung.rs`

`sichtbar_im` bekommt den `Editor`-Zweig. Das ist die **neunte** vollständige
Fallunterscheidung über `Bereich`, und sie trug bis jetzt keinen Schritt des
Plans; sachlich gehört sie zu dem Schritt, der den Bereich anlegt, also zu S13.
`Aufteilung::gemessene_breiten` steht auf `[f64; 5]`, und die Strukturliterale in
`gemessene_breiten(teiler)` und `gemessene_sichtbarkeit` tragen ihr Editorfeld.

`grenze_links` und `grenze_rechts` sind **unverändert**; sie laufen schon über
`Bereich::ALLE` und `mindestbreite()`. Das Abnahmekriterium von S19 verlangt
genau diesen Nachweis am Diff.

Ein Bereich, dessen Unteransicht die Aufteilung noch nicht trägt, liefert in
`gemessene_breiten` `None` und in `gemessene_sichtbarkeit` `false`. Das trifft
heute den Editor, dessen Textfläche S16 einhängt; bis dahin behält er seine
gespeicherte Breite und gilt als nicht stehend. Der Kommentar an beiden Stellen
sagt es.

## Die Breitentabelle

Gemessen von den Proben und nicht gerechnet: die Zahlen stehen als
`assert_eq!` in `die_leiste_weicht_dem_editor_nicht` und
`am_engen_fenster_gewinnt_das_mindestmass_der_dateifenster`.

Editor sichtbar, Vorschau ausgeblendet, beide Dateifenster sichtbar,
Anfangsbreiten. 1280 Punkte ist die Summe der vier Anfangsbreiten der Runde 1.

| Verfügbar | Zustand der Leiste | Leiste | links | rechts | Vorschau | Editor |
|---|---|---|---|---|---|---|
| 1280 | offen | 180 | 320 | 320 | 0 | 460 |
| 1280 | geschlossen | 0 | 410 | 410 | 0 | 460 |
| 900 | offen | 180 | 240 | 240 | 0 | 240 |

Bei 1280 steht der Editor beide Male auf 460, also 36 Prozent der Fensterbreite.
Die Leiste geht zu Lasten der beiden Dateifenster und nicht zu Lasten des
Editors. Am engen Fenster gewinnt das Mindestmaß der Dateifenster: der Editor
fällt auf 240 statt seiner gewünschten 460, die Leiste behält ihre 180.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | sauber |
| `cargo test --workspace` | alle Proben bestanden, 0 gescheitert, 1 übersprungen (dieselbe wie vorher) |
| `cargo clippy --workspace --all-targets` | ohne Meldung |
| `cargo fmt --all --check` | sauber |

`krk-ui` trägt jetzt 195 Proben, vorher 191.

Die Grenze der unsicheren Anteile ist nicht gewachsen:
`grep -rEln '^[[:space:]]*#!?\[allow\(unsafe_code\)\]'` nennt weiterhin genau
`crates/krk-ui/src/appkit/mod.rs` und `crates/krk-core/src/verzeichnis/sys.rs`.

## Eine Berichtigung am Abnahmekriterium von S13

Das Kriterium verlangt, dass
`grep -n 'Bereich::Lesezeichen, Bereich::Vorschau' crates/krk-ui/src/fenstermodell.rs`
nichts mehr findet. Der Aufruf findet zwei Treffer, und **beide sind keine zweite
Wahrheit über die festen Bereiche**: der eine ist der Kommentar in
`bereichsbreiten`, der die entfernte Literalliste beim Namen nennt, der andere
eine Liste in der Probe `das_einblenden_holt_hervor_und_blendet_nie_aus` über die
beim Start sichtbaren Bereiche. Die Aufzählung im Rechenweg ist fort; sie steht
nur noch in `ist_beweglich`.

Ebenso berichtigt: das Kriterium spricht von **acht** vollständigen
Fallunterscheidungen über `Bereich`. Es sind **neun**; die neunte ist
`sichtbar_im` in `aufteilung.rs`, und der geschlossene Defekt hatte sie schon
benannt.

## Was offen bleibt

Von S19 der `anwendung.rs`-Anteil: `breite_aendern`, `sitzung_bauen` und die
Probe darüber, dass eine verstellte Editorbreite in `session.toml` landet und
beim Einlesen wieder herauskommt. Das ist im Plan bei S19 vermerkt.
