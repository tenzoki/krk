# Coder-Sitzung: Schritt 2 der Runde 23, die fünfte Spalte und sie bleibt vorerst leer

**Date:** 2026-08-30, 260830-1447
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `planning/260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 2
**HEAD:** `c99d433` (nicht committet; der Orchestrator committet)

## Was getan wurde

### Die Aufzählung

`Spalte::Marke` steht als fünfter Wert hinter `Typ`, `Spalte::ALLE` ist
`[Spalte; 5]` (`crates/krk-ui/src/spalten.rs`). Die sieben Stellen, die der
Modulkopf aufzählt, hat der Übersetzer einzeln genannt, und jede hat die Antwort
bekommen, die der Plan vorschreibt:

| Stelle | Datei | Antwort |
|---|---|---|
| `kennung` | `appkit/tabelle.rs` | `"marke"` |
| `titel` | `appkit/tabelle.rs` | „Marke" (abgeleitet aus `beschriftung`) |
| `beschriftung` | `spalten.rs` | „Marke" |
| `breiten` | `appkit/tabelle.rs` | `(60.0, 45.0)` |
| `ausrichtung` | `appkit/tabelle.rs` | `NSTextAlignment::Left` |
| `beschreibbar` | `spalten.rs` | `false` |
| `beschriften` | `appkit/tabelle.rs` | `String::new()` |

Dazu die vier Stellen, die der Übersetzer außerhalb dieser sieben genannt hat:
`spalte_sichtbar_in` und `spalte_umschalten` (`fenstermodell.rs`),
`kommando_der_spalte` (`appkit/bereichsleiste.rs`) und das Strukturliteral
`Spaltensichtbarkeit { … }` in `crates/krk-core/tests/ablage.rs`.

`Spaltensichtbarkeit` (`crates/krk-core/src/ablage/sitzung.rs`) trägt ein
viertes Feld `marke` an letzter Stelle, `Default` setzt es auf `true` (A13,
C5.10). Die Feldreihenfolge folgt der von `Spalte::ALLE`, weil `serde` die
Zeilen der `session.toml` in Feldreihenfolge schreibt und die Datei nach C7 der
Runde 1 von Hand zu lesen ist.

### Die drei Punkte, an denen der Plan etwas Bestimmtes wollte

**1. `kommando_der_spalte` liefert für `Marke` `None`, und die Zahl der
Spaltenschalter bleibt bei drei.** `Bereichsleiste::bauen` überspringt jede
Spalte ohne Kommando, also entsteht kein viertes Ankreuzfeld; die Leiste zeigt
weiter elf Felder (zehn mit Kommando plus den kommandolosen Git-Schalter aus
Schritt 1). Der Doc-Kommentar von `Bereichsleiste::spaltenschalter` hat einen
Abschnitt `# Was die Feldbreite haelt, und was sie nicht haelt` bekommen, nach
dem Muster, das Schritt 1 an den vier Bereichsstellen angelegt hat: hier hält
die Feldbreite **nichts**, weil das Feld nicht über `ALLE.map(…)` entsteht,
sondern aus einer gefilterten Liste mit `try_into`; das `expect` bricht beim
Start ab und nicht beim Übersetzen. Der Abschnitt nennt die vier Stellen, die
Schritt 8 zusammen auf vier hebt (Feldbreite, `Vec::with_capacity`, der Text des
`expect`, die Zählprobe).

**2. Die leere Zelle ist als Zielverhalten ausgeschrieben**, an drei Stellen, an
denen ein späterer Leser sonst einen unfertigen Zustand vermutete: im
Doc-Kommentar von `Spalte::Marke`, im Doc-Kommentar von
`Tabellenquelle::beschriften` und als Kommentar am Zweig selbst. Der Wortlaut
trennt die zwei Fälle: ein Ordner ohne Repository lässt die Spalte dauerhaft
leer und sie wird trotzdem nicht eingezogen (E5, C6.3); den zweiten Fall trägt
Schritt 6 nach.

**3. `Schluessel` ist nicht angefasst.** Geprüft vor und nach der Arbeit mit
`awk '/pub enum Schluessel/,/^}/' crates/krk-core/src/verzeichnis/sortierung.rs`:
die beiden Ausgaben sind byteweise identisch (`diff` leer), vier Varianten
`Name`, `Groesse`, `Geaendert`, `Typ`. `git diff --stat` auf
`crates/krk-core/src/verzeichnis/sortierung.rs` ist leer. `cmd+1` bis `cmd+4`
stehen unverändert in `resources/default-keymap.toml:373,378,383,388`; ein
fünfter Sortierbefehl ist nicht entstanden (A12, C5.8).

## Die stillen Stellen, von Hand nachgezogen

Der Bau hat keine von ihnen genannt. Jede ist einzeln aufgesucht worden, über
eine Suche nach `Spalte::` im ganzen Baum und eine zweite nach Feldbreiten
(`; 3]`, `; 4]`, `len() == 4`):

1. **`tabs.rs`, `die_dateiliste_bleibt_flach_und_hat_vier_spalten`.** Die Probe
   hielt `Spalte::ALLE.len() == 4` und wäre rot geworden — eine
   Laufzeitprüfung, kein Übersetzerfehler. Umbenannt auf `…_fuenf_spalten`, die
   Zahl auf 5 gezogen, und der Doc-Kommentar sagt jetzt, dass die Zusage von
   C2.9 die **Flachheit** ist und nicht die Zahl.
2. **`fenstermodell.rs`, `jede_schaltbare_spalte_kippt_fuer_sich`.** Der Rumpf
   führte die Liste `[Spalte::Groesse, Spalte::Geaendert, Spalte::Typ]` von
   Hand; die Markenspalte wäre still ungeprüft geblieben. Die Liste ist jetzt
   abgeleitet: jede Spalte außer `Spalte::Name`, und dass die abgewiesen wird,
   hält die Probe daneben.
3. **`fenstermodell.rs`, `der_auslieferungszustand_zeigt_alle_vier_spalten`.**
   Der Name trug eine Zahl, die dieser Schritt falsch gemacht hätte; der Rumpf
   läuft ohnehin über `Spalte::ALLE`. Umbenannt auf
   `der_auslieferungszustand_zeigt_jede_spalte`, ohne Zahl.
4. **`appkit/bereichsleiste.rs`, `jede_schaltbare_spalte_hat_ihr_eigenes_fach`.**
   Die Probe zählte die Fächer von `Name` bis `Typ` auf und hätte über `Marke`
   nichts gesagt. Um `spaltenfach(Spalte::Marke) == None` ergänzt.
5. **`appkit/bereichsleiste.rs`, `genau_drei_spalten_sind_schaltbar`.** Um
   `kommando_der_spalte(Spalte::Marke) == None` ergänzt, damit die Aussage der
   Feldbreite nicht bloß über die Länge der Liste läuft.
6. **`appkit/tabelle.rs`, der Kommentar in
   `die_namensspalte_nimmt_auf_was_bis_zur_sichtflaeche_fehlt`.** Die 603 sind
   Eingaben der reinen Funktion und keine Summe, die sie rechnet; der Kommentar
   sagt das jetzt, statt „vier Spalten" zu behaupten.

## Prosastellen, die der eigene Diff falsch gemacht hätte

Angefasst sind nur diese, nach der Regel, die Schritt 1 gefahren hat; die
Erhebung über alle Zählaussagen gehört Schritt 11 und Schritt 15.

- `spalten.rs`: Modulkopf („vier Spalten" → fünf, „eine fuenfte Spalte haelt
  den Bau an" → eine sechste), der Doc-Kopf der Aufzählung, `ALLE`,
  `beschriftung`, `beschreibbar` und der Doc-Kommentar der Probe.
- `fenstermodell.rs`: Modulkopf (`Spaltensichtbarkeit` mit drei → vier Feldern),
  `mindestbreite` („vier Spalten hineinpassen" → fünf), `spalte_sichtbar_in`,
  `spaltensichtbarkeit`, `spalte_umschalten`.
- `sitzung.rs`: der Kopf von `Spaltensichtbarkeit` („Drei Felder und nicht
  vier"), `Spaltensichtbarkeit::default`, `Sitzung::default` („alle vier
  Spalten sichtbar").
- `appkit/tabelle.rs`: `titel`, `spaltenbreiten_verteilen` (die Aufzählung der
  festen Breiten trägt jetzt „Marke 60"), der Kommentar über
  `setColumnAutoresizingStyle`.
- `main.rs`: Modulkopf („die vier Spalten des Dateifensters").

**Zwei Zahlen sind stehengeblieben, und beide mit Absicht.** „Neun Schalter"
beziehungsweise „zehn Schalter" in `spalten.rs`, `fenstermodell.rs` und im
Modulkopf von `bereichsleiste.rs` sind schon seit Schritt 1 daneben, weil dort
der sechste Bereichsschalter dazugekommen ist; dieser Schritt fügt keinen
Schalter hinzu und macht sie damit nicht falscher. Sie gehören der Erhebung von
Schritt 15, die den Plan zufolge eine Erhebungsvorschrift an die Stelle der Zahl
setzt. Ebenso ist der Modulkopf von `spalten.rs` **nicht** um den Hinweis
ergänzt worden, dass `Spalte::ALLE` die eine Stelle ist, die der Übersetzer
nicht hält: das ist C9.7 und gehört Schritt 15.

## Proben

Neu:

- `eine_sitzung_ohne_das_markenfeld_bleibt_lesbar`
  (`crates/krk-core/tests/ablage.rs`, C5.9): eine `session.toml` mit einem
  `[spalten]`-Abschnitt, der die drei Felder der Bereichsleisten-Runde führt und
  kein `marke`. Sie gilt nicht als beschädigt, die Markenspalte steht, und die
  Felder, die dastehen, gelten unverändert weiter. Der Unterschied zur schon
  vorhandenen Probe ohne `[spalten]`-Abschnitt ist im Doc-Kommentar
  ausgeschrieben.

Ergänzt:

- `der_auslieferungszustand_der_sitzung_erfuellt_c1` (`ablage.rs`) prüft
  `sitzung.spalten.marke` (C5.10).
- `eine_sitzung_ohne_die_spaltenschalter_bleibt_lesbar` (`ablage.rs`) prüft
  `geladen.wert.spalten.marke`.
- `die_spaltensichtbarkeit_ueberlebt_den_rundlauf_byteweise` (`ablage.rs`)
  verlangt `marke = true` in der geschriebenen Datei.
- `beispielsitzung()` (`ablage.rs`) trägt `marke: false`, damit sie sich
  weiterhin in **jedem** Feld vom Auslieferungszustand unterscheidet — das ist
  die Zusage, die ihr Doc-Kommentar über sich selbst macht.

Unverändert grün und decken `Marke` über `Spalte::ALLE` mit ab:
`jede_spalte_hat_eine_eigene_beschriftung`,
`genau_die_namensspalte_ist_beschreibbar`,
`jede_spalte_findet_sich_ueber_ihre_kennung_wieder`,
`jede_spalte_hat_eine_eigene_kennung_und_ueberschrift`,
`der_auslieferungszustand_zeigt_jede_spalte`.

## `#[must_use]`

Kein neuer Rückgabewert. `Fenstermodell::spalte_umschalten` trägt das Attribut
schon und behält es; sein Zweig für `Marke` liefert `true`, weil die Spalte
schaltbar ist, sobald Schritt 8 das Kommando anlegt.

## Untergrenzen-Abschnitt

Keine neue AppKit-Klasse und keine neue Methode. `NSTextAlignment` steht seit
der Runde 1 in `appkit/tabelle.rs` und ist im Abschnitt gedeckt;
`appkit/bereichsleiste.rs` hat gar keine AppKit-Berührung dazubekommen. Beide
Abschnitte stehen vollständig und sind unverändert.

## Prüfung

- `make check` — exit 0, „alle vier gruen"
  (`cargo build --workspace`, `cargo test --workspace`,
  `cargo clippy --workspace --all-targets` unter `-D warnings`,
  `cargo fmt --all --check`)
- `cargo doc -p krk-ui --no-deps`: 46 unaufgelöste Doc-Verweise, dieselbe Zahl
  wie nach Schritt 1. Ein neuer war zwischenzeitlich entstanden
  (`tests::genau_drei_spalten_sind_schaltbar` liegt unter `#[cfg(test)]` und ist
  für rustdoc nicht auflösbar) und ist auf gewöhnliche Codeauszeichnung
  zurückgenommen.

## Auflagen der Sitzung

Kein whole-tree-git-Kommando abgesetzt. Der Vergleich gegen HEAD lief über
`git show HEAD:<pfad>` beziehungsweise `git diff --stat`; `git stash`,
`git checkout .`, `git reset`, `git clean` und `git restore` sind nicht
gelaufen. Nicht committet.

## Kriterien dieses Schritts

Erfüllt: C5.1, C5.2, C5.8, C5.9, C5.10, C6.3 (Bauhälfte: die Spalte steht und
bleibt leer; dass keine Spaltenbreite beim Wechsel zwischen Repository und
gewöhnlichem Ordner springt, ist Nutzerarbeit am Bündel), Bedingung 1 (kein
Schritt hängt an einem Typ, den es noch nicht gibt).

Nicht angefasst und ausdrücklich außerhalb dieses Schritts: `gix` (Schritt 3),
der Gitbefund im Ordnermodell (Schritt 5), die gefüllte Markenzelle (Schritt 6),
`Kommando`, `Funktionsbereich` und die vier Nachzüge in der Bereichsleiste
(Schritt 8), `resources/default-keymap.toml` (Schritt 9, ontocoder).
