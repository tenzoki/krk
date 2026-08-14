# Sitzung: E2 — der 84. Eintrag der Auslieferungsbelegung

**Datum:** 260814-2320
**Agent:** ontocoder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang E, Schritt E2
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C5.2, C5.5, C5.7

## Was umgesetzt ist

Eine Datei ist angefasst, `resources/default-keymap.toml`, an zwei Stellen.

**Der neue Eintrag** steht unmittelbar hinter `spalte_typ_umschalten` und vor der Trennzeile
zu C4, damit die Reihenfolge der Datei die Reihenfolge im Hauptmenü bleibt:

```toml
[[funktion]]
id = "tiefe_suche_umschalten"
name = "Tiefe Suche ein- und ausschalten"
tasten = []
```

Die Beschriftung ist deutsch wie alle 83 vorhandenen; keine trägt eine englische Aufschrift
aus der Oberfläche. Die Aufschrift des Kästchens bleibt „Deep" und steht in E3, nicht hier.

**Ein Kommentar steht darüber**, nach dem Muster des Blocks über den drei Spaltenschaltern:
warum der Eintrag an dieser Stelle steht, warum die Tastenliste leer ist und nicht
`reserviert_fuer` trägt, mit dem Datensatz dazu, und dass `shift+cmd+f`, `opt+cmd+f`,
`ctrl+cmd+f` und der nackte Tabulator ausdrücklich frei bleiben. Er nennt daneben die
Folge für die Markdown-Ausgabe (siehe den zweiten Befund unten). **Keine Zahl über die
Menge der Einträge steht darin**; die Zahlen der Datei stehen an der einen Stelle, an der
sie schon standen.

**Die Kopfzeile ist nachgezogen**, von 83 auf 84 Funktionen. Die Zahl der Kombinationen
bleibt bei 90, denn der Eintrag trägt keine. Das ist keine freie Zutat: die Probe
`die_zwei_zahlen_im_kopf_der_auslieferungsbelegung_stimmen_noch`
(`crates/krk-core/src/tasten/belegung.rs:1603`) liest beide Zahlen aus der Zeile
`# Ausgeliefert sind …` und zählt die Datei dagegen. Sie hält.

Nachgezählt am Dateibestand, nicht geschätzt:

```sh
grep -c '^\[\[funktion\]\]' resources/default-keymap.toml
# 84
```

## Was der Eintrag erfüllt und was nicht

- **C5.2** — genau ein Eintrag mit `tasten = []` nach dem Muster von
  `spalte_typ_umschalten`, der 84. der Datei. Erfüllt.
- **C5.7** — ausgeliefert wird keine Kombination, der Nutzer kann jederzeit eine vergeben.
  Erfüllt, und die Umsetzung des Nutzerentscheids vom 260814-1610
  (`decisions/260814-1552_a_welche-tastenkombination-schaltet-die-tiefe-suche.md`).
- **C5.5** — zur Hälfte erfüllbar. Die Belegungsansicht führt den Eintrag, die
  Markdown-Ausgabe nicht. Der Grund ist ein anderer Nutzerentscheid und kein Defekt am
  Code; der Widerspruch liegt zwischen C5.5 und C5.7 selbst. Datensatz unten.

## Verifikation

```
make check   # Exit 2
```

**Der Baum ist nicht grün, und das liegt nicht an dieser Datei.** Die Probe, um
derentwillen E2 dringend war, hält jetzt:
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` ist grün, und der
Zwischenstand aus `issues/260814-2303_o_e1-und-e2-teilen-eine-zusicherung-…` ist damit
aufgelöst.

Drei andere Proben fallen, alle drei in Rust und alle drei aus demselben Grund: sie führen
die ab Werk unbelegten Funktionen als ausgeschriebene Liste mit fester Länge, und der
vierte Eintrag passt nicht hinein.

| Probe | Ort |
|---|---|
| `jede_funktion_traegt_genau_eine_zeile_und_eine_reservierte_keine_taste` | `crates/krk-core/tests/belegung.rs:237` |
| `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` | `crates/krk-core/tests/belegung.rs:871` |
| `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` | `crates/krk-ui/src/belegungsausgabe.rs:566` |

Die dritte steckt hinter dem Abbruch von `make`: der Lauf hält beim ersten Fehlschlag an
und kommt an `krk-ui` nicht mehr heran. Einzeln gefahren mit
`cargo test -p krk-ui --bin krk belegungsausgabe` und bestätigt.

`make tasten` und `make menue` sind **nicht** gefahren. Beide hängen an `bundle`, das eine
Signaturidentität verlangt und die Anwendung im Vordergrund startet; der Aufruf lief in die
Zeitgrenze und ist abgebrochen worden. Das ist Nutzerarbeit, wie der Abnahmelauf.

## Zwei Befunde, beide als Datensatz abgelegt

1. `issues/260814-2320_o_drei-proben-fuehren-die-funktionen-ohne-kombination-als-liste-und-e2-macht-sie-vierstellig.md`
   — die drei Proben oben. Die Warnung stand bereit:
   `crates/krk-core/tests/belegung.rs:105` sagt wörtlich „Wer eine vierte Funktion ohne
   Kombination ausliefert, traegt sie mit ihrem Datensatz hier nach." Keiner der Schritte
   E1 bis E3 nennt die Stellen, und der vorhandene Datensatz `260814-2303` sagt „Nach E2
   ist der Baum wieder grün", was nicht stimmt. Die Arbeit gehört dem `coder`.
2. `issues/260814-2320_o_c5-5-verlangt-den-eintrag-in-der-markdown-ausgabe-und-c5-7-schliesst-ihn-daraus-aus.md`
   — C5.5 und C5.7 widersprechen sich. Die Markdown-Ausgabe nimmt eine Funktion nur auf,
   wenn sie mindestens eine Kombination trägt (`crates/krk-ui/src/belegungsausgabe.rs:178`);
   das ist der Nutzerentscheid vom 260811-0110. Nachzuziehen ist der Spec, nicht der Code.

## Was nicht angefasst ist

`crates/` in jeder Form, auch die drei Proben oben: Rust gehört dem `coder`. Die
Änderungen aus E1 liegen unverändert im Arbeitsbaum. Nichts ist eingecheckt; der Nutzer
committet E1 und E2 zusammen.
