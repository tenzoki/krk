# Coder, Schritt 7: Spaltensichtbarkeit — Ablage, Modell, drei Befehle, beide Tabellen

**Datum:** 260812-0618
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260812-0415_p_bereichsleiste-und-proportionale-breitenregel.md`, Implementierungsschritt 7
**Abnahme:** `make check` — **Exit 0**. Der Baum ist zum ersten Mal seit Schritt 4 wieder grün.

## Auftrag

Schritt 7 des Plans und drei Nachträge: die Feldreihenfolge in `Sichtbarkeit`, Punkt 4 des
Datensatzes `issues/260812-0533_*` (die Prosastellen mit den beiden Zählwerten) und der Abschluss
von `issues/260812-0548_*`. Nicht Schritt 8, nicht die vier Befunde der Durchsicht vom 260812-0539,
nicht committen.

Bindend sind vier Datensätze vom 260812-0306, alle auf `_a_`: die Schalter gelten für beide
Dateifenster gemeinsam, sie überstehen einen Neustart, die Sortierung bleibt beim Wegschalten
stehen, und die drei Kommandos entstehen ohne ausgelieferte Kombination.

Der Baum war beim Beginn rot, mit 28 Fehlschlägen in `krk-ui`, jeder mit
`spalte_groesse_umschalten` in der Meldung.

## Was entstanden ist

**`crates/krk-core/src/ablage/sitzung.rs`**

- `Spaltensichtbarkeit` mit `groesse`, `geaendert`, `typ`, je `bool`, `Default` dreimal `true`,
  `#[serde(default)]` wie jede Struktur dieser Datei. Der Doc-Kommentar nennt drei Dinge: warum
  die Spalte Name kein Feld hat, warum ein Feld für beide Dateilisten gilt, und dass das
  Wegschalten die Sortierung nicht anfasst. **Die Abgrenzung, die der Plan verlangt, steht dabei**:
  bei `erstes_dateifenster` gab es einen Wert, der wechseln kann, und er wurde nur nicht
  gespeichert; hier gibt es keinen Schalter und damit keinen Wert.
- Als Feld `Sitzung::spalten` **zwischen `sichtbar` und `fenster`**. Der Kommentar am Feld
  verweist auf denselben Grund, den `Sitzung::editor` schon ausschreibt: TOML verlangt eine
  Tabelle vor der Tabellenfolge, und `[spalten]` hinter den beiden `[[fenster]]` ließe das
  Schreiben scheitern.
- **Nachtrag 1: `Sichtbarkeit::erstes_dateifenster` steht jetzt hinter `lesezeichen`.** Damit ist
  die Feldreihenfolge wieder die der Fensterzeile von links nach rechts, dieselbe, die `Breiten`
  darüber führt. Der Doc-Kommentar der Struktur sagt es jetzt ausdrücklich und nennt den Grund:
  `serde` schreibt die Zeilen in dieser Reihenfolge, und `session.toml` ist nach C7 zum Lesen von
  Hand gedacht. `Default` und die Beispielsitzung der Proben sind mitgezogen.

**`crates/krk-ui/src/fenstermodell.rs`**

- `spalte_sichtbar_in(&Spaltensichtbarkeit, Spalte) -> bool` als freie Funktion neben
  `sichtbar_in`, vollständig, mit `Spalte::Name => true`. Der Kommentar begründet, warum die
  Namensspalte einen Zweig bekommt, obwohl sie kein Feld hat: sonst brauchte jeder Aufrufer einen
  eigenen, oder die Fallunterscheidung bekäme einen Auffangzweig und eine fünfte Spalte fiele
  still durch.
- `Fenstermodell` hält `spalten`, gibt sie über `spaltensichtbarkeit()` heraus, nimmt sie in
  `aus_sitzung` an — **ohne Zusicherung**, denn jede der acht Kombinationen ist eine Lage, die der
  Nutzer auch über die Schalter herstellen kann — und schreibt sie in `sitzung()`.
- `#[must_use] pub fn spalte_umschalten(&mut self, spalte: Spalte) -> bool`, für `Spalte::Name`
  `false`. Das `#[must_use]` folgt der Regel vom 260811-2140; sein Text nennt, was das stille
  Fallenlassen kostet.
- Der Modulkopf trägt einen neuen Abschnitt über die Spalten und sagt in seinem ersten Satz
  mit, dass dieses Modell jetzt auch sie hält.
- Fünf Proben: der Auslieferungszustand zeigt alle vier Spalten; jede schaltbare kippt für sich
  und nimmt keine andere mit; die Namensspalte lässt sich nicht wegschalten und ihre Abweisung
  fasst kein Feld an; die Spaltensichtbarkeit übersteht den Weg durch `session.toml`; und **das
  Wegschalten der Sortierspalte lässt die Sortierung stehen**.

**Die Sortierprobe misst an dem, was in `session.toml` landet**, und das ist Absicht: der
Sortierschlüssel wohnt in den Tabs, die als `Fensterzustand` durch `sitzung()` reisen. Änderte das
Schalten einer Spalte an ihnen etwas, wäre es dort zu sehen. Dass `spalte_umschalten` an die Tabs
gar nicht herankommt, **ist** die Zusage aus C3.3; die Probe hält sie fest, bevor eine spätere
Runde die beiden Aufzählungen zusammenlegt (`issues/260812-0415_o_die-spalten-und-die-sortierschluessel-…`).

**`crates/krk-core/src/tasten/belegung.rs`**

- `Kommando` trägt drei Varianten mehr, `KENNUNGEN` steht auf 73. Der Doc-Kommentar an
  `SpalteGroesseUmschalten` trägt die Begründung für alle drei, die beiden anderen verweisen
  darauf.
- `wirkungsbereich`: alle drei `Ueberall`, bei den Umschaltbefehlen. **Der Kommentar dort nennt
  einen anderen Grund als für die Bereichsschalter darüber**, und der Unterschied trägt: ein
  Bereichsschalter braucht seinen Bereich nicht, weil er ihn herstellt; ein Spaltenschalter
  braucht kein Dateifenster im Fokus, weil er **beide** Listen trifft und es keine Seite gibt, auf
  die er sich bezöge.

**`crates/krk-ui/src/belegungsmodell.rs`**

- `bereich_des_kommandos`: alle drei zu `Funktionsbereich::Dateilisting`, wo
  `versteckte_umschalten` schon steht. Der Kommentar sagt, warum "beide Listen" hier keinen
  zweiten Ort aufmacht: die Gliederung fragt nach der Gegend der Anwendung, und die Dateiliste ist
  eine, gleich wie viele es davon gibt.

**`crates/krk-ui/src/appkit/tabelle.rs`**

- `Dateifenster::spalte_verbergen(spalte, verborgen)` sucht die Spalte über ihre Kennung —
  dieselbe Funktion `kennung`, die den Kopf beim Aufbau benannt hat — und setzt `setHidden:`.
  Findet sich keine, geschieht nichts; das wäre ein Fehler im Aufbau und keiner hier.
- Der Kommentar hält fest, dass eine verborgene Spalte in `tableColumns` und in `numberOfColumns`
  bleibt (`NSTableColumn.h:78`), und dass genau das zwei Zusagen ohne einen einzigen Zweig trägt:
  die Sortierung bleibt stehen, und die Datenquelle liefert weiter dieselben Zellen. Ein
  `removeTableColumn:` täte beides nicht.
- Der Modulkopf-Abschnitt zur macOS-Untergrenze nennt `hidden` unter 10.5 (`NSTableColumn.h:80`,
  am SDK nachgelesen). `tableColumnWithIdentifier:` trägt im Kopf des Systems keine Angabe
  (`NSTableView.h:242`) und fällt damit unter den Auffangsatz derselben Aufstellung.

**`crates/krk-ui/src/appkit/anwendung.rs`**

- Drei Zweige in `kommando_ausfuehren`, alle über die gemeinsame Funktion
  `spalte_umschalten(Spalte)`. Sie ändert das Modell und ruft danach `spaltenanzeige_nachziehen`.
- `spaltenanzeige_nachziehen` schreibt für beide Dateifenster und **alle vier** Spalten den
  Zustand aus dem Modell. Alle vier und nicht die drei schaltbaren, damit der Aufbau und der
  Schalter dieselbe Zeile nehmen; `spalte_sichtbar_in` beantwortet die Namensspalte ohnehin mit
  `true`, und eine Liste der drei daneben wäre eine zweite Aufzählung.
- Der Aufbau ruft sie einmal, direkt hinter `aufteilung_nachziehen`. Ohne diese Zeile erreichte
  eine geladene Sitzung die Anzeige nicht: die Tabelle baut ihre vier Spalten immer sichtbar.
- **Kein `Zeilenmass` und kein eigener Nachzug der Aufteilung.** Eine Spalte liegt in der
  Dateiliste und nicht in der Fensterzeile; die Breiten stehen vorher und nachher gleich (C3.4).

**`crates/krk-core/tests/ablage.rs`**

- Der Rundlauf nennt das neue Feld einzeln, wie die Probe es für jedes Feld hält.
- `eine_sitzung_ohne_den_spaltenabschnitt_bleibt_lesbar`: eine Datei ohne `[spalten]` gilt nicht
  als beschädigt und nimmt dreimal `true` an (C7.4).
- `die_spaltensichtbarkeit_ueberlebt_den_rundlauf_byteweise`: zwei Schreibvorgänge statt eines
  Strukturvergleichs, nach dem Vorbild der Editorfelder, samt der Zusage, dass der Abschnitt in
  der Datei steht, die der Nutzer nach C7 von Hand liest (C7.2).

## Zwei Proben, die dieser Schritt zusätzlich rot gemacht hat

Beide hängen an gebauten Kommandos und konnten deshalb erst jetzt anschlagen. Sie sind dieselbe
Wurzel wie die drei aus `issues/260812-0533_*`: **eine Auslieferungsbelegung darf seit dem
Entscheid vom 260812-0306 eine Funktion ohne Kombination führen.** Beide sind mitbehoben, und der
Nachtrag steht in jenem Datensatz, weil es dort dieselbe Zusage ist.

1. `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` (`crates/krk-core/tests/belegung.rs`)
   verlangte von jedem gebauten Kommando mindestens eine Kombination. Sie nimmt jetzt dieselbe
   Ausnahme wie die Probe über die Belegungsdatei.
2. `innerhalb_eines_abschnitts_bleibt_die_reihenfolge_der_datei` (`crates/krk-ui/src/belegungsausgabe.rs`)
   verglich die Zeilen der Markdown-Ausgabe mit **allen** Funktionen aus `nach_bereichen`; die
   Ausgabe schreibt nur die belegten. Die Erwartung filtert jetzt dieselbe Bedingung.

**`OHNE_KOMBINATION_AB_WERK` ist dabei aus dem Rumpf der einen Probe an den Kopf von
`crates/krk-core/tests/belegung.rs` gewandert.** Zwei Proben lesen sie jetzt, und zweimal dieselbe
Liste hinzuschreiben wäre die zweite Wahrheit gewesen, die der Datensatz an anderer Stelle
ausdrücklich vermeidet. Die Begründung und der Verweis auf den Datensatz sind mitgezogen.

**Die in `issues/260812-0533_*` als ungeprüft benannte Zusage ist jetzt geprüft.** Der letzte
`assert_eq!` in `jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` wird erreicht und
hält: ab Werk sind genau die drei Spaltenschalter unbelegt.

## Nachtrag 2: die neun Prosastellen

Neun Stellen rechneten mit 74 Funktionen und 68 Kommandos; die zehnte war mit Schritt 5
weggefallen. **Beide Zahlen sind nachgezählt und nicht übernommen**: `grep -c '^\[\[funktion\]\]'`
über `resources/default-keymap.toml` liefert 79, sechs Einträge tragen `gehalten_von`, und
`Kommando::KENNUNGEN` steht auf 73 — 79 minus 6 geht auf. Die neun Stellen tragen jetzt 79 und 73;
ein `grep` nach `74` und nach `68` findet in `belegungsausgabe.rs` und `menue.rs` keine mehr.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | Exit 0 |
| `cargo fmt --all --check` | Exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | Exit 0 |
| `cargo test --workspace` | Exit 0 |
| **`make check`** | **Exit 0** |

`krk-ui` steht bei 380 bestandenen Proben im Binärziel, `krk-core` bei 41 in `tests/ablage.rs` und
45 in `tests/belegung.rs`. Die 28 Fehlschläge, die seit Schritt 4 standen, sind weg: mit den drei
Varianten in `Kommando` greift der erste Zweig von `belegungsmodell::bereich`, und `nach_bereichen`
bricht nicht mehr ab.

**Was der Abnahmelauf nicht abdeckt, ist der Augenschein.** Dass die Spalten am laufenden Bündel
wirklich verschwinden (C3.1, C3.2, C3.4), gehört zu Schritt 8 und ist Nutzerarbeit.

## Was dieser Schritt nicht tut

- **Keine Bereichsleiste.** Die acht Schalter, ihre Fläche und ihr Nachzug sind Schritt 8. Die drei
  Spaltenkommandos sind bis dahin nur über eine selbst zugewiesene Taste erreichbar.
- **Keiner der vier Befunde vom 260812-0539.** Sie haben einen eigenen Auftrag.
- **Kein Marker an den vier Datensätzen gewechselt.** Sie bleiben auf `_a_`: die Zeile
  `Implemented:` verlangt einen Commit-Hash, und dieser Schritt committet nicht. Derselbe Grund,
  den der ontocoder für Schritt 4 notiert hat.
- Kein Commit; der Orchestrator trägt ein.

## Am Plan und an den Datensätzen nachgezogen

- Schritt 7 steht auf `[DONE]`, mit einer Anmerkung, die den grünen Baum, die zwei zusätzlichen
  Proben und die drei Nachträge nennt.
- `issues/260812-0533_*` ist mit `Resolved:` abgeschlossen und auf `_c_` umbenannt.
- `issues/260812-0548_*` ebenso. **Punkt 1 seines Vorschlags ist bewusst nicht umgesetzt**: die
  Abnahme der Schritte 4, 5 und 6 im Plan umzuschreiben hieße, sie rückwirkend auf einen
  Zwischenstand zu senken, den es nicht mehr gibt — `make check` ist für alle drei jetzt
  erreichbar und grün, und was der jeweilige Schritt bei seiner Ausführung vorfand, halten die
  Protokolle unter `history/` fest.
