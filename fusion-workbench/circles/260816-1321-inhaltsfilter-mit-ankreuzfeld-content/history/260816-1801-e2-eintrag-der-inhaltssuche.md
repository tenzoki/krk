# E2: Der Eintrag der Belegung

**Datum:** 2026-08-16
**Agent:** ontocoder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt E2
**Baumstand:** `09baffd` mit den unverbuchten Änderungen aus C1, A1, A2, B1, D1 und E1
**Vorbedingungen:** E1
**Erfüllt:** C2.7
**Nicht committet:** E1, E2 und E3 landen zusammen; erst nach E3 ist auch `make check` wieder grün.

## Was entstanden ist

Eine Datei, ein Eintrag, zwei Stellen: `resources/default-keymap.toml`.

**Der `[[funktion]]`-Block** steht unmittelbar hinter dem von
`tiefe_suche_umschalten` und vor der Trennlinie zu C4, mit
`id = "inhaltssuche_umschalten"`, dem Namen „Inhaltssuche ein- und
ausschalten" aus C2.7 des Spec und **leerer Tastenliste**. Die Stellung ist
nicht gleichgültig: `belegungsmodell::nach_bereichen` gibt die Funktionen einer
Gruppe in der Reihenfolge dieser Datei zurück, und Belegungsansicht,
Markdown-Ausgabe und Menüleiste zeigen sie so. „Content" steht damit im
Hauptmenü direkt hinter „Tiefe Suche ein- und ausschalten", so wie das
Ankreuzfeld aus E3 auf dem Schirm rechts von „Deep" stehen wird.

**Der Kommentarblock darüber** schreibt die Wahl in der Form aus, die die Datei
durchgehend führt: warum der Eintrag hier und nicht bei den Spaltenschaltern
steht, warum die Kennung deutsch und die Aufschrift englisch ist, und warum
keine Kombination ausgeliefert wird. Für den letzten Punkt nennt er den
Nutzerentscheid vom 260814-1610
(`circles/260814-1551-tippen-filtert-dateiliste-flach-und-tief/decisions/260814-1552_a_welche-tastenkombination-schaltet-die-tiefe-suche.md`)
und die Erwägung, dass ein zweiter Schalter derselben Art derselben Form folgt,
statt eine der dort frei gehaltenen Kombinationen zu belegen.

**Der Dateikopf** sagt in Zeile 34 jetzt 85 statt 84 Funktionen. Die Zahl der
Kombinationen bleibt bei 90, weil die Tastenliste leer ist.

## Keine Kombination zu prüfen, und warum das trotzdem geprüft wurde

Der Auftrag verlangte, die vorgesehene Kombination gegen alle ausgelieferten
auf Konfliktfreiheit zu prüfen. Eine solche Kombination gibt es nicht: Plan und
Spec verlangen beide ausdrücklich `tasten = []` und nicht `reserviert_fuer`.
Die Prüfung ist damit gegenstandslos, und der Eintrag kann mit keiner
bestehenden Kombination kollidieren.

Gezählt wurde die Datei trotzdem, und dabei ist ein Nebenbefund angefallen. Der
Kommentarblock von „Deep" nennt vier Kombinationen, die „weiterhin frei"
seien: `shift+cmd+f`, `opt+cmd+f`, `ctrl+cmd+f` und der nackte Tabulator. Die
ersten drei stimmen, der vierte nicht. `tab` liegt seit der Runde 1 auf
`fenster_wechseln` (`resources/default-keymap.toml`, Block
`id = "fenster_wechseln"`). Die Aussage stammt aus der Antwortzeile des
Entscheids vom 260814-1610 und ist von dort in den Kommentar gewandert. Der
Entscheid ist eine Aufzeichnung eines Standes und bleibt, wie er ist; der
Kommentar in der Belegungsdatei ist nicht Gegenstand dieses Schritts und wurde
nicht angefasst. Der Befund ist im Bericht an den Orchestrator genannt.

## Abnahme

`cargo test --workspace` — Rückgabewert 0. 1229 Proben bestanden, 0
fehlgeschlagen, 8 übersprungen.

Die vier Proben, die nach E1 rot standen, sind grün:

| Probe | Ort |
|---|---|
| `jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` | `krk-core/src/tasten/belegung.rs` |
| `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | `krk-core/tests/belegung.rs` |
| `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` | `krk-ui/src/belegungsausgabe.rs` |
| `die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` | `krk-ui/src/belegungsausgabe.rs` |

Dazu die zwei, die der Plan für E2 als Abnahme nennt, und die dritte für C2.8:
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`,
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` und
`jede_funktion_der_belegung_steht_genau_einmal_im_menue`.

Die Datei ist gegengezählt: 85 `[[funktion]]`-Blöcke, 90 Kombinationen, keine
doppelte Kennung. Die Zahlen stimmen mit dem Kopf, was die Probe
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch` unabhängig
bestätigt.

## Was rot bleibt

`cargo clippy --workspace --all-targets` meldet weiterhin
`method inhaltssuche_steht is never used` in `krk-ui`. Der Rufer entsteht in
E3. Dagegen wurde nichts gebaut, kein `allow` und keine Hilfskonstruktion. Der
Aufruf selbst gibt 0 zurück, weil eine Warnung kein Fehler ist; `make lint`
setzt `-D warnings` und hält deshalb an, und mit ihm `make check`.

## Nicht angefasst

Doc-Kommentare im Code nennen seit E1 die Gesamtzahl 85, an vier Stellen
(`krk-ui/src/belegungsausgabe.rs:45`, `:256`, `:730` und
`krk-ui/src/appkit/menue.rs:867`, die letzten beiden in der Form „79 der 85
Funktionen"). Mit diesem Eintrag stimmen sie. Sie stehen in `.rs`-Dateien und
gehören nicht zu diesem Schritt; angefasst wurde keine.

## Geänderte Dateien

- `resources/default-keymap.toml`
- `fusion-workbench/circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md` — E2 auf `[DONE]`
