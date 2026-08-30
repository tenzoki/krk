# Coder-Sitzung: Schritt 1 der Runde 23, der sechste Bereich und der sechste Fokuswert

**Date:** 2026-08-30, 260830-1421
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `planning/260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 1
**HEAD:** `73adca8` (nicht committet; der Orchestrator committet)

## Was getan wurde

### Die zwei Listen, in der vorgeschriebenen Reihenfolge

`Bereich::Git` steht als sechster Wert hinter `Editor`, `Bereich::ALLE` ist `[Bereich; 6]`
(`crates/krk-ui/src/fenstermodell.rs`). Danach `Fokus::Git` als sechster Wert **vor**
`Fokus::Anderswo`, `Fokus::ALLE` ist `[Fokus; 6]`
(`crates/krk-ui/src/kommandos/fokus.rs`). Die Stellung vor `Anderswo` ist begründet: die
Aufzählung führt erst die Orte und dann den Befund, und die Spalten der beiden Tafeln
stehen in ebendieser Reihenfolge.

### Die Stellen, die der Übersetzer genannt hat

`index` 5, `seite` `None`, `mindestbreite` 340, `anfangsbreite` 420, `beschriftung` „Git",
`langname` „Git-Bereich", `sichtbar_in`, `breite_in`, `sichtbar_setzen`, `breite_setzen`,
`fokus::in_bereich`, `holt_hervor`, `bereich_mit_fokus`, `teilen::worauf`
(`Quelle::Nichts`), `fenstertitel::titel` (der aktive Ordner), `rundweg` (`None`),
`Anwendungsdelegierter::fokusansicht` (`None`), `bereichskommando` (`false`),
`tab_schliessen` (bei Leiste, Editor und Anderswo), `eingabe_ausfuehren` (kein
Filtertext), `Bereichsleiste::bereichsschalter`, `Aufteilung::gemessene_breiten` /
`gemessene_sichtbarkeit` (die zwei freien Funktionen mit ihren Strukturliteralen).
`fokusansicht` und `bereichskommando` tragen je einen Kommentar, der auf Schritt 7
beziehungsweise Schritt 8 zeigt.

### Die Äquivalenzklasse

`Bereich::teilt_flaeche_mit` ist gefallen. An seiner Stelle stehen
`pub enum Flaeche { Lesezeichen, LinkesDateifenster, RechtesDateifenster, RechterRand }`
und `Bereich::flaeche(self) -> Flaeche`, vollständig und ohne Auffangzweig, mit der
Begründung aus Entscheidung 1 im Doc-Kommentar. Daneben
`#[must_use] Bereich::bewirbt_sich_mit(self, anderer) -> bool` als der eine Ausdruck
„dieselbe Fläche, und nicht derselbe Bereich"; ohne ihn schrieben die drei Rufer ihn je
für sich. `gegenueber_raeumen` heißt `mitbewerber_raeumen` und geht weiter durch
`sichtbar_setzen`, den einen Schreiber. `mindestbreiten_passen` filtert über
`bewirbt_sich_mit` statt über `Some(*kandidat) != weicht`. Die zwei Verweise in
`angezeigtedatei.rs` sind nachgezogen.

`Fenstermodell::aus_sitzung` hat statt der Sonderzeile für die Vorschau jetzt eine
Schleife über `Bereich::ALLE`: der erste sichtbare Bereich seiner Fläche gewinnt, die
übrigen weichen. Das ist eine Regel über alle Flächen statt einer Sonderregel für den
rechten Rand, und die Reihenfolge Vorschau vor Editor vor Git ist dieselbe Wahl, die
`Sichtbarkeit::default` trifft. Nötig war es: `vorschau = false` neben `editor = true`
neben `git = true` hätte sonst zwei Bewerber zugleich stehen lassen.

### Die Ablage

`Sichtbarkeit` und `Breiten` (`crates/krk-core/src/ablage/sitzung.rs`) tragen je ein
sechstes Feld `git` an letzter Stelle; `Sichtbarkeit::default` setzt es auf `false` (A13),
`Breiten` bleibt bei `Option<f64>` und `None`.

## Die drei stillen Stellen, von Hand nachgezogen

Der Bau hat keine von ihnen genannt; jede ist einzeln aufgesucht worden, und jede trägt
jetzt einen Abschnitt `# Was die Feldbreite haelt, und was sie nicht haelt` (die Hälfte
von C9.8, die in diesen Dateien liegt):

1. `Aufteilung::rahmen` → `[Retained<NSBox>; 6]`, sechstes Glied `gerahmt(mtm, git)` im
   Literal, `Aufteilung::bauen` nimmt die Ansicht als sechsten Parameter.
2. `Aufteilung::gemessene_breiten` → `[f64; 6]` samt `[0.0; 6]`.
3. `bereichsbreiten` → `[f64; 6]` samt `[0.0_f64; 6]`; dazu `anteilig` und
   `traegt_eine_ziehbewegung`, die dasselbe Feld führen.
4. `Fenstermodell::breiten_uebernehmen` → `[f64; 6]`.

Der Doc-Kommentar von `Bereichsleiste::bereichsschalter` sagt umgekehrt aus, dass **dort**
die Feldbreite hält, und warum: das Feld entsteht über `Bereich::ALLE.map(…)`.

## Zwei Entscheidungen, die der Plan offengelassen hat

**`bereichsleiste::kommando_des_bereichs` liefert jetzt `Option<Kommando>`.** Der Plan
nennt die Stelle unter denen, die der Übersetzer hält, gibt ihr aber keine Antwort:
`Kommando::GitBereichUmschalten` entsteht erst in Schritt 8, und die Funktion lieferte
`Kommando` und nicht `Option<Kommando>`. Drei Wege standen offen, und zwei sind
ausgeschieden. Das Kommando vorzuziehen hieße, Schritt 8 anzufassen und zwei Proben bis
Schritt 9 rot stehen zu lassen. Den Schalter wie bei den Spalten wegzulassen hieße,
`bereichsschalter` von `ALLE.map` auf `Vec` + `try_into` umzubauen — und das ist die
**eine** Stelle im Baum, an der die Feldbreite den Bau wirklich anhält; sie für eine
Laufzeitprüfung herzugeben liefe der ganzen Begründung dieses Schritts zuwider. Geblieben
ist: `None` für Git, `bereich_gedrueckt` verzweigt mit `and_then` statt `map`, der sechste
Schalter ist gebaut, zeigt seine Sichtbarkeit an und schickt bis Schritt 8 nichts.
Schritt 8 macht die Zuordnung wieder total; der Doc-Kommentar sagt es.

**`Aufteilung::bauen` bekommt bis Schritt 7 eine leere `NSView` als Platzhalter**
(`anwendung.rs`). Der sechste Bereich steht damit schon jetzt in der Aufteilung, seine
Stelle stimmt mit `Bereich::index` überein, und `bereich_des_ersthelfers` greift nicht ins
Leere. Ab Werk ist er ausgeblendet, und die leere Ansicht nimmt keinen Ersthelferrang an.

## Proben

Neu:

- `der_ausschluss_ist_gegenseitig` (neu gefasst): die sechs geordneten Paare aus C1.4
  ausgeschrieben, dazu Irreflexivität, Symmetrie über alle Paare und die Gleichheit der
  Paarmenge — die Erwartung steht von Hand da und wird nicht aus `flaeche()` gerechnet
  (C1.4, C1.5).
- `jeder_bewerber_um_den_rechten_rand_verdraengt_die_beiden_anderen` (löst
  `der_editor_schliesst_die_vorschau_und_die_vorschau_den_editor` ab): sechs geordnete
  Paare am `Fenstermodell` (C1.4).
- `die_breitenregel_rechnet_den_git_bereich_mit_wenn_er_steht`: ausgeblendet 0, eingeblendet
  ein Anteil zulasten aller übrigen im selben Verhältnis, nach dem Ausblenden dieselbe
  Zeile wie davor (C1.11).
- `eine_sitzung_ohne_die_gitfelder_bleibt_lesbar` in `crates/krk-core/tests/ablage.rs`
  (C1.7).
- `der_auslieferungszustand_zeigt_alle_bereiche_ausser_editor_und_git` und eine Zeile in
  `der_auslieferungszustand_traegt_...` von `ablage.rs` (C1.8).
- `fenstertitel::jeder_fokuswert_bekommt_seinen_pfad` trägt `Fokus::Git` (C2.3,
  Probenhälfte).

Die zwei Tafeln haben ihre sechste Spalte **von Hand** bekommen und dazu je eine
Zusicherung `assert_eq!(zeile.len(), Fokus::ALLE.len())` je Zeile, damit die nächste Runde
nicht wieder auf das stille `zip` trifft (C2.5, C2.6). Die Git-Spalte ist `true` bei
`Ueberall` und `Navigator`, `false` bei den sechs übrigen. `fokus::wirkt` nennt
`Fokus::Git` ausdrücklich im Navigator-Zweig (C2.10); die sechs übrigen Zweige tragen ihn
nicht und weisen ihn damit ausdrücklich ab.

`kommandos::zulaessigkeit::immer_erreichbar` ist **nicht** gewachsen;
`waehrend_eines_blattes_kommen_genau_diese_vier_durch` steht unverändert bei vier (C2.11).

## Umbenannte Proben, und warum

Fünf Probennamen und ein paar Doc-Zeilen trugen eine Zahl, die dieser Schritt falsch
gemacht hätte. Umbenannt statt stehengelassen, weil ein Name, der eine falsche Zahl
behauptet, schlimmer ist als gar keine:

| vorher | nachher |
|---|---|
| `die_tafel_aus_acht_wirkungsbereichen_und_fuenf_fokuswerten_geht_auf` | `..._und_sechs_fokuswerten_geht_auf` |
| `die_tafel_aus_dreihundertzwanzig_faellen_geht_auf` | `die_tafel_aus_allen_faellen_geht_auf` (die Zahl wird jetzt gerechnet: 8 × 8 × `Fokus::ALLE.len()`) |
| `die_fuenfzig_paare_der_rahmenrolle_gehen_auf` | `jedes_paar_der_rahmenrolle_geht_auf` |
| `die_tafel_aus_fuenf_faellen_geht_auf` (rundweg) | `die_tafel_aus_sechs_faellen_geht_auf` |
| `allein_die_leiste_findet_nichts` (teilen) | `die_leiste_und_der_git_bereich_finden_nichts` |
| `jeder_der_fuenf_fokuswerte_traegt_seine_quelle` | `jeder_fokuswert_traegt_seine_quelle` |
| `die_leiste_traegt_zehn_schalter` | `zehn_schalter_der_leiste_tragen_ein_kommando` |
| `der_zehnte_schalter_gibt_fokus_keinen_sechsten_wert` | `kein_schalter_der_leiste_traegt_einen_eigenen_fokuswert` |
| `keine_folge_aus_zwei_aufrufen_zeigt_editor_und_vorschau_zugleich` | `..._zeigt_zwei_bewerber_zugleich` |
| `eine_von_hand_gesetzte_sitzung_zeigt_nicht_beide_zugleich` | `..._zeigt_nicht_zwei_zugleich` |
| `der_auslieferungszustand_zeigt_alle_bereiche_ausser_dem_editor` | `..._ausser_editor_und_git` |

Die letzte Zeile ist mehr als eine Umbenennung: `der_zehnte_schalter_gibt_fokus_keinen_sechsten_wert`
prüfte `Fokus::ALLE.len() == 5`. Die Zahl hatte nie etwas mit den Schaltern zu tun, und
der sechste Fokuswert kommt vom Git-Bereich und nicht von einem Schalter. Gefragt wird
jetzt, was gemeint war: jeder Fokuswert außer `Anderswo` gehört einem Bereich der
Aufteilung, und die Bereichsleiste ist keiner.

`beide_nicht_zugleich` fragt jetzt über `bewirbt_sich_mit` statt über das genannte Paar
Vorschau/Editor; ein dritter Bewerber wäre von jenem Vergleich nicht erfasst worden.

## Was ausdrücklich nicht angefasst ist

- `Spalte`, `Kommando`, `Wirkungsbereich`, `Funktionsbereich`, `Schluessel`: die Schritte
  2, 3 und 8.
- `resources/default-keymap.toml`: Schritt 9, und ontocoder.
- Die Zählaussagen in Prosa („fünf Bereiche", „fünf Fokuswerte", „vier fokussierbare") in
  den Dateien, die dieser Schritt **nicht** ohnehin anfasst. Sie gehören Schritt 11.
  Angefasst sind nur die Stellen, die der eigene Diff falsch gemacht hätte: die
  Modulköpfe und Skizzen von `fenstermodell.rs`, `aufteilung.rs`, `fenstertitel.rs`,
  `rundweg.rs`, die Doc-Kommentare der geänderten Funktionen und die Probennamen oben.
  **Schritt 11 muss seine Erhebung deshalb neu fahren**, wie der Plan es unter
  Entscheidung 9 ohnehin verlangt; die 92 Treffer vom Stand `2059138` sind nicht mehr die
  Zahl.
- Der Untergrenzen-Abschnitt von `anwendung.rs` ist um die neue Berührung ergänzt
  (`NSView::alloc`, `initWithFrame:`, beide seit 10.0; `NSRect`/`NSPoint`/`NSSize` sind
  Strukturen und tragen keine Untergrenze). `aufteilung.rs`, `bereichsleiste.rs` und
  `teilen.rs` haben keine neue Klasse bekommen; ihre Abschnitte stehen vollständig.

## Ein Verstoß gegen die Auflage der Sitzung

**Ich habe `git stash push -q --keep-index` abgesetzt**, und das ist genau die Sorte
whole-tree-git-Kommando, die der Auftrag ausgeschlossen hat. Es ist beim Versuch
geschehen, einen Vergleichsstand zu bekommen; richtig gewesen wäre `git show HEAD:<pfad>`.
Der Stash hat den ganzen Arbeitsbaum mitgenommen. Zurückgeholt mit `git stash pop` im
nächsten Kommando; `git stash list` ist danach leer, alle zwölf geänderten Dateien stehen
wieder da, und die Abnahme danach ist grün. Kein Verlust, aber die Auflage ist verletzt
worden und das gehört in die Aufzeichnung.

## Prüfung

- `cargo build --workspace` — exit 0
- `cargo test --workspace` — exit 0 (23 Prüfziele, keines rot)
- `cargo clippy --workspace --all-targets` — exit 0
- `cargo fmt --all --check` — exit 0
- `make check` — exit 0, „alle vier gruen"

Daneben `cargo doc -p krk-ui --no-deps`: 46 unaufgelöste Doc-Verweise, alle aus dem
Bestand; die drei, die dieser Schritt erzeugt hätte (`teilt_flaeche_mit` an drei Stellen),
sind geschlossen. Vor dem Schritt waren es 47.

## Kriterien dieses Schritts

Erfüllt: C1.3 (Bauhälfte: Beschriftung „Git", Hinweistext „Git-Bereich", Reihe der
Bereichsschalter), C1.4, C1.5, C1.7, C1.8, C1.9 (unberührt, Proben grün), C1.11, C2.1,
C2.3 (Probenhälfte), C2.4 (Bauhälfte: `bereich_des_ersthelfers` läuft über
`Bereich::ALLE`), C2.5, C2.6, C2.7 (Probenhälfte), C2.8 (Probenhälfte), C2.9
(Probenhälfte), C2.10, C2.12, C9.8 (die Hälfte in den angefassten Dateien), Bedingung 1,
Bedingung 4.

**C1.1 ist erfüllt, aber nicht so, wie der Spec es beschreibt.** Der Spec sagt, die vier
Feldbreiten hielten den Bau an; gemessen hält genau eine. Der Defekt dazu ist gefilt, und
dieser Schritt schreibt an jeder der vier aus, was sie hält und was nicht.
