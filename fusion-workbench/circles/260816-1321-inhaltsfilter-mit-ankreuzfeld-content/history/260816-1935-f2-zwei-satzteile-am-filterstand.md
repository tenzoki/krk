# F2: Zwei Satzteile am Filterstand

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt F2
**Baumstand vor der Arbeit:** `3dd799a` plus A1 `5c7f5b9`, C1 `4a54212`, A2 `7283d55`, B1 `32fd038`, D1 `09baffd`, Strang E `37ca972`, F1 `f7cf88b`
**Vorbedingungen:** D1 — der Tab führt `zu_gross` seit `09baffd`
**Erfüllt:** C4.8, C4.9, C4.10 (die probengestützte Hälfte; die Bündelhälfte von C4.8 bleibt Nutzerarbeit)
**Nicht committet:** auf Ansage des Nutzers.

## Was entstanden ist

Drei Dateien. Die zwei, die der Plan nennt, und `tabs.rs` dazu; warum, steht
unten unter „Die dritte Datei".

**Zwei Felder am `Filterstand`.** `liest_inhalt: bool` und `zu_gross: u64`,
beide dokumentiert mit ihrer Quelle. Die Struktur bleibt `Copy` und `Default`,
und der Rang bleibt einer von sechs.

**Der Satz hat jetzt einen Kern und drei Zusätze.** `filterstand_text` setzt
ihn in der entschiedenen Reihenfolge zusammen:

```text
Filter „notiz“: 38 von 4.812 angezeigt        Kern, immer
, Inhalt wird gelesen                         solange ein Inhaltsdurchlauf läuft
, 12 Dateien zu groß                          wenn der Lauf Dateien übergangen hat
, 3 Markierungen ausgeblendet                 wenn der Filter Markierungen verdeckt
```

Der Größenhinweis hat einen Singularzweig wie der Markierungshinweis unter ihm
(`, eine Datei zu groß` gegen `, 12 Dateien zu groß`), und seine Zahl geht
durch `zahl()`, trägt also denselben Tausenderpunkt wie jede andere Zahl der
Oberfläche. `zu_gross` kommt als `u64` vom Tab und `zahl` nimmt ein `usize`;
die Umrechnung sättigt (`usize::try_from(…).unwrap_or(usize::MAX)`) statt
abzuschneiden, wie es die Größenbeschriftung der Tabelle daneben schon tut.

**Die Füllung.** `DateifensterQuelle::gerechnete_raenge` leiht den sichtbaren
Tab jetzt als Ganzes statt nur sein Modell und stellt ihm zwei Fragen mehr.
Gerechnet wird dort weiterhin nichts.

## Die dritte Datei

`crates/krk-ui/src/tabs.rs` steht nicht in der Dateiliste des Plans und ist
trotzdem angefasst, aus zwei Gründen.

**Erstens fällt dort die angekündigte Zeile.** `Tabinhalt::zu_gross` trug seit
D1 ein `#[expect(dead_code, …)]`, dessen Begründung den Ableser aus F2 nannte.
Mit dem ersten Ruf wird die Erwartung unerfüllt, und
`unfulfilled_lint_expectations` hält unter `-D warnings` den Bau an. Die Zeile
ist entfernt und nicht zu einem `allow` umgebaut; ihr Doc-Kommentar nennt jetzt
den Ableser, den es gibt, statt den, den es geben wird. Der Absatz der Probe
`die_zahl_der_zu_grossen_dateien_steht_auch_nach_dem_ende_des_laufs`, der
erklärte, warum sie das Feld und nicht die Methode liest, ist mit seinem Grund
weggefallen; die Probe liest jetzt die Methode.

**Zweitens steht die Bedingung des Lesehinweises dort und nicht beim Ableser.**
Der Plan schreibt sie in `gerechnete_raenge`: wahr, wenn ein Durchlauf läuft
**und** `inhalt_wirkt()` gilt. Sie steht stattdessen als
`Tabinhalt::liest_inhalt` an dem Typ, der den `Durchlauf` hält, und
`gerechnete_raenge` liest nur noch ab. Der Unterschied ist die Prüfbarkeit:
`gerechnete_raenge` braucht AppKit-Objekte auf dem Hauptfaden, `tabs.rs` nicht,
und die Regel ist damit in einer Probe ohne Fenster festgehalten. Die
Alternative hätte ohnehin eine neue öffentliche Methode gebraucht, nämlich
`durchlauf.is_some()`; das Feld ist privat. Eine Methode entsteht so oder so,
und die mit der ganzen Regel darin ist die bessere.

**Die zweite Bedingung ist nicht schmückend.** Ohne sie bekäme auch ein reiner
Namensdurchlauf über den Unterbaum den Lesehinweis, und der Satz wäre bei
ausgeschaltetem „Content" nicht mehr zeichengleich mit dem der Runde 10. Genau
diese Zeichengleichheit ist die tragende Bedingung der Bauentscheidung zur
Reihenfolge.

## Was nicht entstanden ist

- **Kein siebter Rang.** `Rang::ALLE` hat unverändert sechs Werte, `Rang::art`
  ist nicht angefasst, der Filterstand bleibt `Art::Vorgang` und damit nicht
  rot (C4.10). Eine Probe hält beides am vollen Satz fest.
- **Keine zweite Statuszeile** (C4.9). Es bleibt bei der einen aus der Runde 6.
- **Keine Kürzungsvorrichtung.** AppKit kürzt am rechten Rand, und
  `Statuszeile::kurzhinweis_nachziehen` hängt bei Kürzung den vollen Satz als
  Kurzhinweis an. `filterstand_text` bleibt rein und ohne Fenster prüfbar; eine
  Messung der Breite zöge das Fenster in sie hinein. Die benannte Lücke daneben
  — gemessen wird beim Setzen des Texts, nicht bei einer Fensteränderung — ist
  weder behoben noch verschlimmert.
- **Keine Abkürzungen.** `liest` und `12 zu groß` waren Möglichkeit 4 der
  Bauentscheidung und sind nicht gewählt.

## Die AppKit-Berührung: es gibt keine

Beide angefassten Dateien unter `appkit/` sprechen keine AppKit-Methode an, die
sie nicht schon ansprachen. `filterstand_text` ist eine reine Funktion über
Rust-Werte, und `gerechnete_raenge` liest Felder des Tabmodells. Der Abschnitt
`# Ab welchem macOS die angesprochenen Klassen stehen` bleibt in beiden
Modulköpfen unverändert, und das ist kein Versäumnis, sondern der Befund.

## Abnahme

`make check` — exit 0. Alle vier Kommandos grün, darunter
`cargo clippy --workspace --all-targets -- -D warnings` und
`cargo fmt --all --check`. 19 Prüfziele, 0 Fehlschläge. Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist im selben Lauf
durchgelaufen und nicht angefasst.

**Sechs neue Proben in `statuszeile.rs`**, alle ohne Fenster:

| Probe | Was sie hält |
|---|---|
| `jede_kombination_der_vier_satzteile_steht_in_der_festgelegten_reihenfolge` | alle acht Kombinationen der drei Zusätze, Satz für Satz ausgeschrieben |
| `ohne_zu_grosse_datei_steht_der_groessenhinweis_nicht_da` | null zu große Dateien heißt: der Teil steht nicht da |
| `der_groessenhinweis_trennt_eine_datei_von_mehreren` | Singularzweig und Tausenderpunkt |
| `ohne_inhaltsdurchlauf_ist_der_satz_der_der_runde_zehn` | Zeichengleichheit mit dem Satz vor dieser Runde |
| `der_volle_satz_bleibt_ein_rang_und_kein_fehler` | sechs Ränge, `Art::Vorgang`, nicht rot |
| `die_neuen_teile_heben_die_beiden_abbruchgruende_nicht_auf` | leerer Filtertext und ausstehender Ersatz gehen vor |

Die erwarteten Sätze stehen in der ersten Probe ausgeschrieben da und werden
nicht aus denselben Bausteinen zusammengesetzt wie `filterstand_text`; eine
Probe, die die Regel nachbaut, prüft sie nicht.

**Eine Zusicherung mehr in `tabs.rs`:** die Probe zur Zahl der zu großen
Dateien hält jetzt daneben fest, dass `liest_inhalt` nach dem Ende des Laufs
falsch ist, auch wenn „Content" steht.

**Die vier vorhandenen Proben des Filterstands sind unberührt und grün**, ohne
eine Zeile Änderung: ihr Erzeuger `stand(…)` setzt die zwei neuen Felder auf
den Stand der Runde 10 und prüft damit die Zeichengleichheit gleich mit.
`jeder_der_sechs_raenge_hat_genau_ein_feld` und
`ueber_alle_zwoelf_bewerber_gewinnt_genau_eine_aussage` ebenso.

## Was Nutzerarbeit bleibt

**C4.8 am laufenden Bündel.** Kein Agent kann den Lauf fahren, der Abnahmelauf
verlangt KRK im Vordergrund. Zu prüfen ist:

- Ein Inhaltsdurchlauf über einen großen Unterbaum lässt an der Zeile erkennen,
  dass gelesen wird, und der Zusatz vergeht mit dem Lauf.
- Der Größenhinweis steht nach dem Lauf noch da, auch bei einem kleinen Ordner,
  in dem der Lauf durch ist, bevor die Zeile das nächste Mal rechnet.
- Im schmalen Fenster fällt der Markierungshinweis zuerst hinter den rechten
  Rand und der Lesehinweis zuletzt, und der Kurzhinweis trägt den vollen Satz.

Der Prüfordner dafür braucht drei Dateien: eine mit der Folge im Inhalt, eine
über 1 MB, und mindestens eine markierte, die der Filter ausblendet. Erst dann
stehen alle vier Satzteile zugleich da.

## Offen geblieben

Die zwei Datensätze, die F2 umsetzt, stehen weiter auf `_a_`:

- `shared/decisions/260816-1310_a_was-zeigt-die-eine-statuszeile-waehrend-der-inhalt-gelesen-wird.md`
- `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/decisions/260816-1359_a_in-welcher-reihenfolge-stehen-die-satzteile-des-filterstands-und-was-faellt-im-schmalen-fenster-weg.md`

Beide sind mit diesem Schritt in Code umgesetzt und gehören auf `_i_` mit einer
`Implemented:`-Zeile. Die Zeile zitiert den Commit-Hash, und diesen Schritt hat
der Nutzer ausdrücklich nicht committen lassen. Der Nachzug gehört deshalb an
den Commit, der F2 aufnimmt — dieselbe Lage wie bei F1.
