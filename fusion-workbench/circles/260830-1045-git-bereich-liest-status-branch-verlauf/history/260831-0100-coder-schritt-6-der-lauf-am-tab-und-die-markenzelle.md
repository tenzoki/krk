# Coder-Sitzung: Schritt 6 der Runde 23, der Lauf am Tab und die gefüllte Markenzelle

**Date:** 2026-08-31
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Status:** Complete
**Circle:** `circles/260830-1045-git-bereich-liest-status-branch-verlauf`
**Plan:** `260830-1317_*_plan-git-bereich-liest-status-branch-verlauf.md`, Schritt 6
**HEAD:** `7ad8978` (nicht committet; der Orchestrator committet)

## Was gebaut wurde

### `crates/krk-ui/src/gitmodell.rs` (neu) — was der Bereich zeigt, ohne eine Zeile AppKit

`Gitmodell` hält den Kopf, den Verlauf, die Auswahl darin, ob der Verlauf erschöpft ist, und die
Zusammenfassung des Status. Die Schreibseite (`neu`, `zuruecksetzen`, `kopf_setzen`,
`verlauf_anhaengen`, `marken_setzen`) steht in einem Block, die Leseseite in einem zweiten.

**Der Kopf ist ein `Option<Kopf>` und nicht ein `Kopf`, und das ist keine Bequemlichkeit.**
`Kopf::KeinRepository` ist eine **entschiedene** Antwort; als Vorbelegung stünde der Satz „Dieser
Ordner liegt in keinem Git-Repository." schon da, bevor irgendetwas gefragt wurde, und A8 verlangt
für diese Spanne ausdrücklich, dass **nichts** dasteht. `None` heißt hier „noch nicht beantwortet",
dieselbe Trennung, die `krk-core/src/git/mod.rs` zwischen `Oeffnung::KeinRepository` und
`Oeffnung::Unentschieden` zieht.

**Die Zusammenfassung steht als Feld daneben und die Markenliste nicht.** Der Gitbefund hat zwei
Abnehmer aus **einer** Meldung: die Buchstaben gehen ins Ordnermodell, der Satz hierher. Die Liste
zusätzlich hier zu halten hieße, dieselben hunderttausend Namen an zwei Stellen zu führen.

### `crates/krk-ui/src/tabs.rs` — der Lauf hängt am Tab

`Tabinhalt` bekommt `gitlauf: Option<Gitlauf>`, `gitmodell: Gitmodell`, `gitgeneration: u64` und
`wartende_marken`. `Tabliste` bekommt `git_gefragt: bool` mit Setzer und `letzter_gitlauf: u64`.

`Tabliste::gitlauf_nachziehen_an(stelle)` ist nach `durchlauf_nachziehen_an` gebaut: der alte Lauf
fällt zuerst, mit ihm die zurückgehaltene Markenmeldung und der Verlauf (C4.6). Drei Bedingungen —
sichtbar, gefragt, Ordner steht —, **und die dritte ist der Kern des Schritts**: sie heißt nicht
„der Bestand ist gelesen". Der Lauf braucht allein den Pfad und beginnt deshalb in `lesen_starten`
zugleich mit dem Lesevorgang; mit der stärkeren Bedingung wartete der Branchname in einem Ordner
mit hunderttausend Einträgen vier Sekunden auf etwas, das er nicht braucht.

`lesen_starten` stößt ihn **nach** `lesevorgang_beginnen` an, damit `gitgeneration` die eben
gesetzte Generation trägt; sonst wiese `gitmarken_setzen` jeden Befund als fremd ab. `waehlen` ruft
den Nachzug für die verlassene und für die neue Stelle, wie es ihn für den Durchlauf ruft, und
überspringt die zweite, wenn `ungelesenen_aktiven_nachlesen` den Lauf schon gestartet hat.
`abbrechen` nimmt Lauf und Meldung mit. `arbeitet_noch` zählt den dritten Kanal mit — ohne das
hielte der Takt an, während der Statuslauf noch unterwegs ist, und die Spalte bliebe leer.

`einzug_je_tab` bekommt `gitmeldungen_einziehen` als dritten Zug, **hinter** den beiden anderen:
der Gitlauf läuft im Gegensatz zum Durchlauf zugleich mit dem Lesevorgang, und derselbe Takt, der
den Abschluss einzieht, soll die Marken noch eintragen können.

### `crates/krk-ui/src/appkit/tabelle.rs` — die gefüllte Zelle

`beschriften(Spalte::Marke, …)` liefert `Ordnermodell::gitmarke(…).buchstabe()` oder die leere
Zeichenkette. Die Zeile kommt als eigener Parameter herein, weil die Marke über den
**Eintragsindex** angesprochen wird, den der `Eintrag` selbst nicht kennt; die Umrechnung macht
`DateifensterQuelle::zeile_gitmarke`, gebaut wie `zeile_markiert` daneben. **Kein drittes
Kennzeichen** (C5.11): Farbe und Schrift setzt `zellenansicht` für alle fünf Spalten gleich, und
die Marke bekommt keinen eigenen Zweig.

Der Einzugstakt antwortet auf `gitmarken_neu` mit `reloadData` und **ohne** `auswahl_anzeigen`, in
einem eigenen Zweig neben der Kette: derselbe Takt kann den Abschluss des Lesevorgangs **und** die
Marken bringen. `tab_gewechselt` fragt jetzt `arbeitet_noch` statt `liest_noch`, weil ein
Tabwechsel auf einen schon gelesenen Tab einen Gitlauf anstößt und sonst keinen Takt bekäme.

Keine neue AppKit-Klasse und keine neue Methode: `reloadData` steht seit macOS 10.0 und ist im
Abschnitt `# Ab welchem macOS die angesprochenen Klassen stehen` schon gedeckt. Der Abschnitt
bleibt unverändert richtig.

### `crates/krk-ui/src/main.rs`

Modul `gitmodell` angemeldet, sein Absatz im Modulkopf geschrieben, und die Zahl davor von
„Siebzehn" auf „Achtzehn" gezogen — mein eigener Diff hätte sie sonst falsch gemacht.

## Zwei Stellen, an denen der Bau über den Plantext hinausgeht

**1. Die Markenmeldung wartet in einem Feld und nicht im Kanal.** Der Plantext sagt, sie werde
„erst aus dem Kanal genommen, wenn `tab.gelesen && !tab.liest()`". Wörtlich ist das mit
`std::sync::mpsc::Receiver` nicht zu bauen: er kennt kein Vorausschauen, und die drei Meldungen
teilen sich **einen** Kanal in fester Reihenfolge. Wer wartete, bis der Tab gelesen ist, hielte
Kopf und Verlauf genauso lange zurück — und genau das soll dieser Schritt vermeiden. Wer sie
stattdessen sofort einträgt, verliert sie: `gitmarken_setzen` weist ab, solange der Ersatz
aussteht, und der Befund wäre danach für immer weg (er kostet gemessen 12 bis 164 ms, das Lesen
eines großen Ordners vier Sekunden — der frühe Fall ist der **Regelfall** und nicht die Ausnahme).
`Tabinhalt::wartende_marken` ist dieselbe Wartestelle einen Schritt später, mit derselben Zusage:
nichts erreicht das Ordnermodell, bevor der Bestand steht. Die Begründung steht am Feld.

**2. Drei Stellen tragen eine Ausnahme von der Totprüfung, jede mit ihrem Ablaufdatum.**
`Tabliste::git_gefragt_setzen`, `Tabliste::verlauf_nachladen` und `Tabinhalt::gitmodell` haben
ihren Rufer erst in Schritt 7 beziehungsweise 8; dasselbe gilt für die Leseseite des `Gitmodell`.
Ohne Ausnahme wäre `make check` an dieser Stelle rot, und die Abnahme dieses Schritts verlangt es
grün.

Die Form ist **`#[cfg_attr(not(test), expect(dead_code, reason = "…"))]`** und nicht
`#[allow(dead_code)]`, und das ist der Unterschied, den der Baum selbst schon benannt hat:
`editormodell.rs` hält fest, dass die früheren Ausnahmen dort „ohne Ablaufdatum" standen. `expect`
**hat** eines — sobald der Rufer da ist, meldet der Übersetzer die Erwartung als unerfüllt und
zwingt zum Entfernen der Zeile. Das `cfg_attr(not(test))` grenzt sie auf den ausgelieferten Bau
ein, denn im Probenbau sind die Stellen schon heute gerufen. **Wer Schritt 7 und 8 fährt, entfernt
diese vier Zeilen; der Übersetzer sagt es ihm.**

`git_gefragt` fängt deshalb auf `false` an und nicht auf `true`. Das ist der Anfangswert und nicht
der Auslieferungszustand: die Markenspalte steht ab Werk (A13), und `gitbedarf_nachziehen` aus
Schritt 8 zieht den Wert beim Aufbau der Oberfläche sofort nach. Ihn hier vorwegzunehmen hieße,
eine Sichtbarkeit zu behaupten, die `tabs.rs` nicht kennt — und es hätte drei bestehende Proben
rot gemacht, die an `arbeitet_noch` hängen.

## Die Proben

Sieben neue in `tabs.rs`, sechs in `gitmodell.rs`, alle in `#[cfg(test)]`-Modulen neben dem Code.
**Keine ruft `git`**: die Läufe gegen ein angelegtes Repository stehen in
`crates/krk-core/tests/git.rs` aus Schritt 3 und 4, und die Zählprüfung der Runde 8 nimmt nur
`crates/*/tests/` aus. Eine vierte Prüfordner-Fassung entsteht nicht; benutzt wird
`crate::pruefordner::Pruefordner`, die Fassung dieser Kiste.

| Probe | Kriterium |
|---|---|
| `tabs::der_gitlauf_beginnt_zugleich_mit_dem_lesevorgang` | der Kern des Schritts, A8, C7.3 (Bauhälfte) |
| `tabs::ohne_seine_drei_bedingungen_beginnt_kein_gitlauf` | die drei Bedingungen |
| `tabs::zwei_schnelle_ordnerwechsel_lassen_nie_zwei_gitlaeufe_stehen` | C7.11 |
| `tabs::ein_ordnerwechsel_setzt_den_verlauf_auf_die_ersten_fuenfzig_zurueck` | C4.6 |
| `tabs::ein_verspaeteter_gitbefund_schreibt_nichts_in_den_neuen_bestand` | C7.5 (Tabhälfte) |
| `tabs::die_marken_warten_auf_den_bestand_und_gehen_dabei_nicht_verloren` | A8, C7.3, C7.4 am Tab |
| `tabs::ein_nachschlag_faengt_nur_an_wenn_kein_lauf_steht` | E12, C4.3, C7.11 |
| `tabs::der_gitlauf_wird_an_genau_den_stellen_aus_a9_angestossen` | C7.10 |
| `gitmodell::ein_frisches_modell_zeigt_nichts` | A8 |
| `gitmodell::ein_kuerzerer_schwung_erschoepft_den_verlauf` | C4.3 |
| `gitmodell::das_zuruecksetzen_laesst_nichts_vom_vorigen_ordner_stehen` | C4.6 am Modell |
| `gitmodell::ein_repository_ohne_commit_sagt_es_in_der_zweiten_zeile` | A7 |
| `gitmodell::eine_auswahl_jenseits_des_verlaufs_bleibt_leer` | C3.5, Modellhälfte |
| `gitmodell::die_verlaufszeile_kommt_aus_dem_kern` | A5, eine Textquelle |

**Zwei tragen mehr als ihren Namen.** `ein_verspaeteter_gitbefund_…` hält eine **Gegenprobe**
daneben: dieselbe Meldung mit der eigenen Generation kommt an. Ohne sie belegte die erste Hälfte
nur, dass irgendetwas nichts eingetragen hat. Und `die_marken_warten_…` prüft nicht nur, dass
nichts eingetragen wird, sondern dass die Meldung **liegen bleibt** — das ist die Zusage, an der
die Wartestelle hängt.

### Wie die Ruferliste für C7.10 gebaut ist

Zwei Zählungen über `crate::quellbaum::{quelldateien, aufrufstellen}`, beide mit zusammengesetzter
Nadel, weil die Probe in dem Baum liegt, den sie liest.

Die erste hält `Gitlauf::starten(` gegen den ganzen Baum und erwartet **genau eine** Datei mit
**genau zwei** Fundstellen: `krk-ui/src/tabs.rs`. Sie ist zugleich ihre eigene Gegenprobe — eine
Umbenennung ließe die Liste leer und die Gleichheit scheitern, statt die Probe still bestehen zu
lassen.

Die zweite zählt `gitlauf_nachziehen_an(` im Teil der Datei vor dem Prüfmodul und erwartet vier
Aufrufstellen. Die Meldung schreibt sie aus: `lesen_starten` (jedes Neulesen eines Ordners, und
damit `auffrischung::ordner_neu_lesen` samt jeder Navigation), `waehlen` zweimal (der Abbruch am
verlassenen Tab und der Anstoß am neuen) und `git_gefragt_setzen` (das Einschalten des Bereichs
oder der Spalte). Das sind die Auslöser aus A9 und keiner mehr.

**Ihre Blindheit** steht am Doc-Kommentar: ein Aufruf unter anderem Namen — ein `use … as anders;`
— entgeht ihr, wie jeder Suche im Quelltext.

## Der Plan

`## Implementation Steps`, Schritt 6 auf `[DONE]`. `## Data Structures`: die Zeile zum `Gitmodell`
führt jetzt seine fünf Felder aus, mit dem `Option` am Kopf und der Zusammenfassung, die der
Entwurf nicht hatte — derselbe Nachzug, den Schritt 5 für die Gitleser-Signaturen gefahren hat, an
der Stelle, an der Schritt 7 als nächstes liest.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, 875 Proben im Binärziel von `krk-ui`, davon die
dreizehn neuen.

Kein `git stash`, kein `git checkout .`, kein `git reset --hard`, kein `git clean`, kein
`git restore .`. Nicht committet.
