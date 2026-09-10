# Coder-Sitzung: Schritt 3, der Merker der gemeldeten Fassung

**Date:** 2026-09-10, 260910-1420
**Filed by:** coder, Kai Stalmann <kai@qantr.com>
**Status:** Complete
**Circle:** `260910-0707-krk-meldet-neuerungen-in-readers-settings-keymap`
**Plan:** `260910-0818_*_plan-krk-meldet-neuerungen-in-readers-settings-keymap.md`, Schritt 3
**Entscheid:** `260910-0818_*_wo-merkt-sich-krk-fuer-welche-fassung-es-die-neuerungen-schon-gemeldet-hat.md`, Möglichkeit 2
**HEAD:** `59e9910` (nicht committet; der Nutzer committet)

## Was getan wurde

`reported.toml` ist die achte Ablagedatei und trägt den einen Schlüssel
`gemeldete_fassung`. Geschrieben wird sie über `Zugang::sichern`, gelesen über
`Zugang::laden`, beides unter der Schreibsperre — und damit gilt sie für **jede** Instanz,
was der Ausschlag gegen ein Feld auf `Sitzung` war. `session.toml` ist nicht angefasst, und
die offene Frage `260907-1407` bleibt unangetastet.

### `crates/krk-core/src/ablage/merker.rs` (neu)

- `LAUFENDE_FASSUNG: &str = env!("CARGO_PKG_VERSION")`. Die Zahl steht im Workspace an einer
  Stelle, jedes Mitglied erbt sie über `version.workspace = true`; ein Weg über die
  `Info.plist` bräuchte das Bündel und lieferte im Prüflauf gar nichts.
- `Merker { gemeldete_fassung: String }`, mit `#[serde(default)]` und **ohne**
  `deny_unknown_fields` — aus demselben Grund, den der Modulkopf der Ablage für
  `session.toml` ausschreibt: eine spätere Fassung darf ein Feld hinzufügen, ohne dass eine
  frühere die Datei verwirft und ihre Meldung ein zweites Mal zeigt.
- `Merker::meldung_steht_aus(&self, fassung) -> bool` ist eine **Ungleichheit** und kein
  Größenvergleich. Damit fallen die drei Abnahmefälle auf eine Zeile: ein fehlender Merker
  liefert den Auslieferungszustand und also die leere Zeichenkette, ein leerer trägt sie
  selbst, und beide sind ungleich jeder Fassungsnummer. Eine ältere Fassung über einer
  neueren meldet damit ebenfalls, und das ist gewollt.
- `laden(&Zugang) -> Geladen<Merker>` und `vermerken(&Zugang, &str) -> io::Result<()>`.
  **Die `Ersetzung` wird weitergereicht und nicht verschluckt**, anders als in
  `neuerungen::erheben`: diese Datei hat keinen zweiten Leser, der sie meldete.

### `crates/krk-core/src/ablage/pfade.rs`

`Datei::Merker` steht zwischen `Leser` und den zwei Zetteln, `Datei::ALLE` ist `[Datei; 8]`.
Die drei je Datei beantworteten Fragen tragen die neue Antwort: `Format::Toml`,
`Leerbefund::Beschaedigt`, `Ersatz::Auslieferungszustand`. `Beschaedigt`, weil KRK die Datei
selbst schreibt und dabei nie ohne obersten Schlüssel — das ist gemessen, siehe die Probe
unten, und nicht geraten.

### `crates/krk-core/src/ablage/neuerungen.rs`

Drei Fallunterscheidungen bekommen ihren Zweig: `Vergleichsform::fuer` → `Nicht`,
`eigene_eintraege_moeglich` → `false`, `auslieferung` → `None`. Der Merker trägt keinen
Bestand des Nutzers und kann deshalb nicht hinter einer Auslieferungsfassung zurückliegen.

### `crates/krk-core/src/ablage/mod.rs`

`pub mod merker;`, die Skizze der Modulreihenfolge um `merker` erweitert und neu
ausgerichtet, ein Absatz über die Datei, und die Prosazahlen nachgezogen.

## Was der rote Lauf genannt hat

`keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien` wurde beim ersten
Lauf rot und nannte fünf Stellen, alle in `ablage/mod.rs`: zweimal „sieben Ablagedateien",
dreimal „fünf TOML-Dateien". Nachgezogen sind sie und dazu die Stellen, die die Probe
**nicht** sieht, weil sie außerhalb von `src/ablage/` liegen oder ein Zahlwort vor „der"
tragen:

- `pfade.rs`: Modulkopf, `Datei::ALLE`, `Ablageort`, `Ersatz` (zweimal „sechs der sieben",
  jetzt ohne Zahl formuliert — die Aussage ist „jede bis auf eine", und die bleibt wahr).
- `mod.rs`: `Ersetzung`, `Zugang::laden`, `Zugang::sichern`, `beiseite_legen`.
- `tests/ablage.rs`: `geladen`, `gesichert`, `toml_dateien`, `genau_readers_toml_bekommt_keinen_ersatz`
  und zwei Zettelproben.
- `tests/baum.rs`: der Doc-Kommentar der Prosaprobe selbst (seine Beispiele zitieren die
  Sätze, die sie liest) und die Begründung in `UNLESBARE_ALLE_LISTEN`, jetzt „acht Einträge
  zu sieben Varianten".

Nicht angefasst ist `xtask/src/veroeffentlichung.rs:1374`: „Bis zum 260905 nannte der Text
vier der sieben Dateien" ist eine datierte Aussage über einen vergangenen Stand und war
damals wahr.

## Was der Übersetzer genannt hat

Genau die vier Stellen, die der Plan vorhergesagt hat — `format`, `leerbefund`, `ersatz`,
`Vergleichsform` —, dazu `eigene_eintraege_moeglich`, `auslieferung` und eine
Fallunterscheidung in `tests/ablage.rs`. Keine Stelle in `krk-ui`: die Oberfläche verzweigt
nirgends über `Datei`.

## Was er nicht genannt hat, und was deshalb rot wurde

Drei Proben, die über `Datei::ALLE` oder `toml_dateien()` laufen und die achte Datei
stillschweigend mitzählten:

- `der_ablageordner_liegt_unter_application_support` hält die Namensliste wörtlich.
- `alle_toml_dateien_ueberstehen_schreiben_und_wiedereinlesen` prüft, dass jede TOML-Datei
  danach dasteht — `reported.toml` wurde nie geschrieben. Der Rundlauf schreibt sie jetzt
  mit, über `beispielmerker()`.
- `jede_toml_datei_wird_bei_beschaedigung_zur_seite_gelegt` hält die Länge von
  `ersetzungen_der_toml_dateien` gegen `toml_dateien().count()`. Genau dafür ist die
  Längenzusicherung 260824 eingebaut worden, und sie hat getan, was sie soll.

**Eine vierte war grün und trotzdem falsch.**
`eine_leere_datei_meldet_bei_den_drei_von_hand_gepflegten_nichts` filterte
`Datei::Lesezeichen | Datei::Sitzung` **namentlich** heraus. `reported.toml` ist die dritte
strenge Datei und wäre dem Filter durchgerutscht: die Probe hätte sie mit `""` beschrieben
und ihr Ergebnis nicht angesehen. Der Filter fragt jetzt `welche.leerbefund() ==
Leerbefund::Vorgabe` — dieselbe abgeleitete Frage statt einer zweiten Liste, die der Baum
überall sonst schon vorzieht.

## Neue Proben

In `crates/krk-core/tests/ablage.rs`, Abschnitt „Der Merker der gemeldeten Fassung":

- `ein_fehlender_merker_heisst_noch_nie_gemeldet` — und das Lesen legt die Datei **nicht**
  an; sie entsteht erst, wenn gemeldet worden ist.
- `ein_leerer_merker_heisst_ebenfalls_noch_nie_gemeldet`.
- `ein_merker_mit_anderer_fassung_heisst_melden_und_mit_derselben_nicht`, mit einer älteren
  und einer neueren fremden Zahl.
- `jede_geschriebene_reported_toml_traegt_einen_obersten_schluessel` — die Messung, an der
  `Leerbefund::Beschaedigt` hängt, in der Bauform der Schwesterprobe für `session.toml`. Die
  Struktur ist ausgeschrieben und nicht über `..Default` gebaut, damit ein neues Feld die
  Messung erneut erzwingt.
- `der_merker_wird_nicht_verglichen` — `Vergleichsform::Nicht`, und der `Bestand` der
  Neuerungen führt ihn nicht.

## Abweichung von der Dateiliste

Keine über das hinaus, was der Planschritt selbst offen lässt: er nennt „jede Prosastelle
unter `crates/krk-core/src/ablage/`, die die Probe beim ersten roten Lauf namentlich nennt".
Dazu kommen `tests/ablage.rs` und `tests/baum.rs`, beide im Schritt genannt.

## Was offen bleibt

**Der Merker wird noch von niemandem gelesen.** Das ist Schritt 4: er liest ihn in
`sitzung_laden`, hält ihn gegen `LAUFENDE_FASSUNG`, erhebt bei Ungleichheit und schreibt ihn
zurück. `cargo clippy` beanstandet die unbenutzten öffentlichen Funktionen nicht, weil
`krk-core` ein Bibliotheksziel ist.

**Der Entscheid trägt weiter `_a_`.** Die Umsetzungsnotiz verlangt den Commit-Hash, und
dieser Auftrag verbietet das Committen ausdrücklich. Wer Schritt 3 committet, hängt an
`260910-0818_*_wo-merkt-sich-krk-fuer-welche-fassung-es-die-neuerungen-schon-gemeldet-hat.md`
eine `Implemented:`-Zeile mit dem Hash und benennt die Datei nach `_i_` um.

## Prüfung

`make check` — Rückgabewert 0, alle fünf Kommandos, in seiner Reihenfolge.
