# C1: Schalter, Schwelle und der sechste Zweig des einen Prüfschritts

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt C1
**Baumstand vor der Arbeit:** `28bd78f`
**Erfüllt:** C1.1, C1.2, C1.3 (Sichtbarkeitshälfte), C1.10, C2.6, C2.9, C2.10, C5.4, C5.5

## Was entstanden ist

### Die dritte Regel des Filters

`krk-core/src/verzeichnis/filter.rs` trägt jetzt `pub fn inhaltsschwelle(tief: bool) -> usize`
mit fünf bei eingeschalteter tiefer Suche und drei sonst. Der Doc-Kommentar führt
die Herleitung aus dem Spec mit: ein tiefer Inhaltsfilter liest die Dateien eines
ganzen Unterbaums statt eines Ordners, und zwei Zeichen bezeichnen wenig und
treffen entsprechend viel. Der Modulkopf sagt seither „die drei Regeln des
Filters", und sein Bild führt die Schwelle mit ihrem einen Rufer.

Die Datei bleibt die Heimat aller drei Regeln und fällt aus der Zählprobe wie
bisher. **Die Zählprobe selbst ist unangetastet**: `inhaltsschwelle` ist keine
ihrer beiden Nadeln, und der Vergleich `traegt_die_folge` hat unverändert seine
zwei Rufer, `modell.rs` und `durchlauf.rs`. Der dritte kommt mit Schritt A2.

### Das Ordnermodell

`krk-core/src/verzeichnis/modell.rs` bekommt das Feld `inhalt: bool`, anfangs
`false`, mit `inhalt()` und `inhalt_setzen(bool)`. Der Setzer folgt `tief_setzen`
Zeile für Zeile: beim Einschalten fallen die Befunde auf `Unentschieden` zurück,
beim Ausschalten bleibt der Vektor stehen, und die Sicht wird in beiden Fällen
neu aufgebaut.

`pub fn inhalt_wirkt(&self) -> bool` ist die eine Stelle, an der die Schwelle
geprüft wird: `inhalt` steht **und** `filtertext.chars().count()` erreicht
`filter::inhaltsschwelle(tief)`. Gezählt werden Zeichen und keine Bytes.

### Zwei Eingänge, ein Rumpf

Der Ausgang des Prüfschritts, der bisher für eine gewöhnliche Datei `false`
lieferte, lautet jetzt `return self.inhalt_entscheidet(index as u32)`. Die
Zweigfolge davor ist unverändert.

- `fn inhalt_entscheidet(&self, i: u32) -> bool` — privat, der Rumpf **ohne**
  Vorbedingungen: `self.inhalt_wirkt() && matches!(self.befund(i), Befund::Treffer)`.
  Gültig allein dort, wo die Vorbedingungen schon feststehen, und das ist der
  Dateizweig von `sichtbar`.
- `pub fn steht_wegen_des_inhalts(&self, i: u32) -> bool` — derselbe Rumpf mit
  allen Vorbedingungen davor, für die Dateizelle: kein Index außerhalb des
  Bestands, kein Ordner und keine symbolische Verknüpfung, ein stehender
  Filtertext, kein Namenstreffer.

Der Doc-Kommentar der zweiten Funktion trägt den Grund für die Trennung mit: die
Vorbedingungen im Prüfschritt ein zweites Mal zu prüfen kostete je Eintrag einen
weiteren Aufruf von `name_traegt_den_filter`, und der schreibt den Namen einmal
um — bei 100.000 Einträgen also 100.000 Umschreibungen je Neuaufbau der Sicht.

### `Befund` behält seine drei Werte

Eine vierte Variante für „zu groß" ist ausdrücklich nicht entstanden; sie wäre
ein dritter Trefferzustand. Der Doc-Kommentar der Aufzählung sagt das jetzt und
schreibt daneben aus, dass die drei Werte für eine Datei genau dasselbe tragen
wie für einen Ordner. Wie viele Dateien wegen ihrer Größe ungelesen blieben,
gehört in die Statuszeile und nicht an die Zeile.

### Der Modulkopf

Das Bild der Zweige führt den Dateizweig mit, die Zahl der Eingaben steigt von
fünf auf sechs, und ein neuer Abschnitt schreibt aus, warum die beiden
Treffergründe sich nicht überschneiden: nicht durch eine zusätzliche Regel,
sondern durch den Kurzschluss des Namens, hinter dem der Inhaltszweig liegt.

## Die Proben

Elf neue Proben, davon eine in `filter.rs` und zehn in
`crates/krk-core/tests/verzeichnis.rs` unter der Überschrift „Der Inhaltsfilter
aus C1, C2 und C5". **Keine von ihnen fasst die Platte an.** Zwei Hilfsfunktionen
tragen das: `handeintrag` baut einen `Eintrag` samt Sortierschlüssel, `handmodell`
ein fertiges `Ordnermodell` daraus. Der Grund steht im Doc-Kommentar von
`handmodell`: der Prüfschritt entscheidet über den **Befund** und nicht über eine
Datei, und wer den Befund von Hand setzt, misst genau den Zweig, um den es geht.
Wer eine Datei wirklich liest, misst `verzeichnis::inhalt`, und das steht mit
Schritt A2 anderswo.

| Probe | Kriterium |
|---|---|
| `der_inhaltsfilter_wirkt_ab_drei_zeichen_und_darunter_nicht` | C1.1, C1.2, C1.10 |
| `die_tiefe_suche_hebt_die_schwelle_auf_fuenf_zeichen` | C2.10 |
| `die_schwelle_zaehlt_zeichen_und_keine_bytes` | die Zeichenregel der Schwelle |
| `ohne_filtertext_aendert_der_inhaltsfilter_nichts` | C2.6 |
| `das_ausschalten_nimmt_die_inhaltszeilen_weg_und_laesst_den_befund_stehen` | C2.9 |
| `ein_namentlicher_treffer_steht_ohne_jeden_befund` | C1.3, Sichtbarkeitshälfte |
| `steht_wegen_des_inhalts_antwortet_nur_fuer_die_eine_lage` | C5.4, C5.5 |
| `eine_verknuepfung_steht_nie_wegen_ihres_inhalts` | C5.5, zweite Hälfte |
| `unter_der_schwelle_steht_keine_zeile_wegen_ihres_inhalts` | die Zelle rechnet die Schwelle nicht nach |
| `die_inhaltsschwelle_steht_bei_drei_und_bei_fuenf` (in `filter.rs`) | die Staffelung selbst |

**Eine Eigenschaft, die den Proben ihre Form gibt:** jede Änderung des
Filtertexts und jedes Einschalten eines der beiden Schalter setzt die Befunde
zurück, weil sie Auskünfte über eine früher gestellte Frage wären. Die Proben
setzen den Befund deshalb nach der Änderung neu, so wie der Durchlauf ihn in der
Anwendung neu liefert.

## Am Diff abzulesen

Beide Zusagen des Plans gemessen, nicht behauptet:

- `Ordnermodell::sichtbar` hat weiterhin genau zwei Rufer, `anhaengen` und
  `sicht_neu_aufbauen` (`modell.rs:350` und `:934`). Die vielen Treffer in
  `krk-ui/src/fenstermodell.rs` gehören einer gleichnamigen Methode über
  `Bereich` und sind ein anderer Gegenstand.
- `inhaltsschwelle` hat genau einen Rufer im Code, `inhalt_wirkt`
  (`modell.rs:844`). Die zwei weiteren Fundstellen stehen in Doc-Kommentaren.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, keine Probe fehlgeschlagen.

**Der Baum trug beim Lauf auch die Arbeit von Schritt A1**, der nebenher lief
(`text/datei.rs`, `vorschaumodell.rs`, `tests/text.rs`, `sys.rs`). Der Lauf misst
damit beide Schritte zusammen; die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist durchgelaufen.

## Was dieser Schritt nicht getan hat

- **Die Zählprobe des Filters ist unangetastet.** Sie steigt mit Schritt A2, und
  an einer anderen Stelle. Kein neuer Rufer von `traegt_die_folge` ist entstanden.
- **Kein Bezug auf `krk-ui`.** Die 1 MB reisen erst mit Schritt D1 als Argument
  herein; `krk-core` kennt die Zahl nicht.
- **Der Durchlauf ist nicht angefasst.** Woher ein Befund für eine Datei kommt,
  entscheidet Schritt B1; dieses Modell nimmt ihn über das vorhandene
  `befunde_setzen` entgegen.
