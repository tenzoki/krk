# Zählprüfungen der Belegung ohne Literale

---
**Datum:** 260804-1015
**Ausführender:** coder
**Status:** Complete
**Defekt:** `issues/260804-0907_c_drei-fest-verdrahtete-zahlen-im-code-brechen-mit-den-neuen-eintraegen-aus-s9b.md`
**Geänderte Dateien:** `crates/krk-core/src/tasten/belegung.rs`, `crates/krk-core/tests/belegung.rs`
**Unangetastet:** `resources/default-keymap.toml` (Prüfsumme vor und nach der Arbeit `13faa4f2ab604a1d2af2c99f778400962889b57f`)
**Commit:** keiner — der Auftrag verlangte ausdrücklich, nicht zu committen

---

## Der Ausgangspunkt

S9b hat drei Funktionen in `resources/default-keymap.toml` nachgetragen. Die
Datei ist damit von 46 auf 49 Funktionen und von 52 auf 55 Kombinationen
gewachsen, und zwei Prüfungen sind umgefallen, weil sie die alten Zahlen als
Literal führten:

    cargo test -p krk-core --test belegung  → 25 passed; 1 failed
    cargo test -p krk-core --lib            → 25 passed; 1 failed

Beide mit `left: 49, right: 46`. Die inhaltlichen Prüfungen
(`die_auslieferungsbelegung_ist_konfliktfrei`,
`jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung`) liefen
durchgehend grün: die Daten waren nie das Problem.

## Warum nicht 46→49 und 52→55

Der naheliegende Fix behebt den Lauf und stellt dieselbe Falle für den nächsten
Nachtrag wieder auf. `default-keymap.toml` ist eine Datentabelle, die mit jeder
Runde wächst. Eine Prüfung, die ihre Größe als Literal führt, prüft die Größe
der Datei und nicht die Zusage, für die sie dasteht. Die vier gemeldeten
Fundstellen sind deshalb einzeln daraufhin befragt worden, was sie eigentlich
behaupten wollen, und haben drei verschiedene Antworten bekommen.

## Die vier Fundstellen, einzeln

### 1. `tests/belegung.rs` — die 46 in `die_nutzerdatei_ersetzt_die_auslieferungsbelegung_und_ergaenzt_sie_nicht`

Die Zusage steht im Namen und zerfällt in zwei Hälften, die beide ohne Zahl
auskommen.

Die erste: die geladene Belegung führt denselben **Wortschatz** wie die
Auslieferungsbelegung. Das ist der Sachverhalt, den die 46 meinte — Funktionen,
die die Nutzerdatei nicht nennt, treten unbelegt hinzu, damit die
Belegungsansicht aus C3 sie weiter auflistet. Geprüft wird das jetzt gegen
`Belegung::auslieferung()`. Ein neuer Helfer `kennungen` sammelt die Kennungen
einer Belegung und sortiert sie, denn `Belegung::bauen` holt die Funktionen, die
die Nutzerdatei nennt, nach vorne; die Reihenfolge ist hier nicht die Zusage.

Die zweite: **belegt** ist allein, was die Nutzerdatei nennt. Die Prüfdatei
nennt `kopieren`, also trägt danach genau `kopieren` Tasten und sonst niemand.
Das ist "ersetzt, ergänzt nicht" in der Form, die der Name verspricht, und es
ist schärfer als die 46: eine Belegung, die die ausgelieferten Kombinationen
zusätzlich einmischt, hätte die 46 passiert, weil sich dabei die Zahl der
Funktionen nicht ändert.

Der Nachsatz über `verschieben` ist entfallen. Er sagte, `verschieben` sei
vorhanden und unbelegt; das erste deckt jetzt die Wortschatzprüfung, das zweite
die Belegtprüfung.

### 2. `src/tasten/belegung.rs` — die 46 und die 52 in `die_auslieferungsbelegung_traegt_die_erwarteten_zahlen`

Diese Prüfung hatte keine Zusage außer den Zahlen selbst. Sie war ein
Änderungsmelder: sie schlug an, wenn die Datei wuchs, und sagte nichts darüber,
ob etwas kaputt war.

Was sie sinnvoll zusichern kann, ist, dass `Belegung::bauen` beim Einlesen
nichts verliert. Sie heißt darum jetzt
`beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren`, liest
`AUSLIEFERUNGSTEXT` ein zweites Mal als rohe `Belegungsdatei` und hält die
gebaute Belegung dagegen: gleich viele Funktionen, gleich viele Kombinationen.

Das ist kein Zirkelschluss und auch nicht gegenstandslos. `bauen` verwirft
stillschweigend eine Kombination, die innerhalb derselben Funktion zweimal
steht (`if !tasten.contains(&kombination)`), und genau dieser Fall fällt hier
auf. Ein `!datei.funktionen.is_empty()` steht davor, damit beide Vergleiche
nicht bei einer leeren Datei gegenstandslos grün werden.

### 3. `src/tasten/belegung.rs:223` — die 52 im Kommentar zu `nachschlag`

Der Kommentar begründet, warum `nachschlag` eine lineare Schleife ist und kein
Nachschlagbaum. Das Argument hängt an der Größenordnung, nicht am Zählwert:
"die wenigen Dutzend ausgelieferten Kombinationen" trägt es genauso und
veraltet nicht. Eine 55 an dieser Stelle wäre die schwächere Wiederholung
derselben Falle, weil Prosa von keiner Prüfung eingeholt wird und still
veraltet. Der Kommentar sagt das jetzt über sich selbst.

### 4. Eine fünfte Fundstelle: `tests/belegung.rs`, `eine_zweite_kombination_an_derselben_funktion_ist_kein_konflikt`

Beim Nachprüfen aufgetaucht und vom Datensatz nicht genannt. Die Prüfung schrieb
`assert_eq!(funktion.tasten().len(), 3)` — zwei ausgelieferte Wege von
`kopieren` plus der zugewiesene `ctrl+k`. Derselbe Fehler in klein: die 3 stammt
aus den Daten, nicht aus der Zusage. Der Kommentar daneben sagt, was gemeint
ist, nämlich dass die ausgelieferten Wege daneben stehen bleiben. Genau das
steht jetzt da: jede Kombination, die `kopieren` ab Werk trägt, muss danach noch
vorhanden sein, und dazu genau eine mehr. Sie bricht erst, wenn `kopieren`
wirklich eine Kombination verliert.

Sie wäre nicht am nächsten Nachtrag gescheitert, sondern erst an einer Änderung
an `kopieren` selbst — dieselbe Kopplung, nur seltener wirksam. Deshalb
mitgenommen.

Andere Zahlen im Bereich sind geprüft und bleiben stehen:
`Kommando::KENNUNGEN: [(Kommando, &'static str); 5]` und
`parser::TASTEN: [Taste; 53]` sind Feldlängen, die der Übersetzer gegen den
Inhalt hält, und `parser.rs` führt Tastencodes, keine Dateigrößen.

## Nachweis, dass der Fix den nächsten Nachtrag überlebt

`include_str!` bindet die Auslieferungsbelegung an ihren festen Pfad, eine Kopie
an anderer Stelle wird also nicht gelesen. Der gleichwertige Weg war deshalb:
Sicherung ziehen, probeweise anhängen, messen, zurückspielen, Prüfsumme
vergleichen.

**Probe A — der vierte Nachtrag.** Ein Block `probe_vierter_nachtrag` mit
`tasten = ["ctrl+p"]` an die Datei gehängt. `ctrl+p` ist ab Werk frei und
kollidiert mit keiner Prüfung.

    cargo test -p krk-core --test belegung  → 26 passed; 0 failed
    cargo test -p krk-core --lib            → 26 passed; 0 failed

Ohne eine Zeile Code anzufassen. Gegen den alten Stand wären beide umgefallen,
genau wie bei S9b.

**Probe B — nicht bloß gegenstandslos grün.** Derselbe Block mit
`tasten = ["ctrl+p", "ctrl+p"]`. `beim_bauen_der_auslieferungsbelegung_geht_kein_eintrag_verloren`
schlägt mit der vorgesehenen Meldung fehl ("eine Kombination der
Auslieferungsbelegung steht doppelt und ist beim Bauen entfallen"), 25 von 26.
Die neue Prüfung fängt also einen echten Fehlerfall und ist nicht nur
unempfindlich geworden.

**Beide Proben zurückgenommen.** `resources/default-keymap.toml` steht wieder
auf `git hash-object` `13faa4f2ab604a1d2af2c99f778400962889b57f`, dem Wert von
vor der ersten Probe. Die Datei ist byteweise unverändert; sie erscheint in
`git status` nur deshalb als geändert, weil S9b noch nicht committet ist.

## Abnahme

Alle mit Rückgabewert 0:

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | 0 |
| `cargo test --workspace` | 0 — 170 bestanden, 2 ausgelassen |
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets` | 0, keine Warnung |
| `cargo test -p krk-core --test belegung` | 0 — 26 von 26 |
| `cargo test -p krk-core --lib` | 0 — 26 von 26 |

Damit erfüllt S9b sein Abnahmekriterium.

## Was liegen bleibt

Der Datensatz verlangt in seinem letzten Abschnitt auch, den Absatz unter
`#### 9b.` in `planning/260802-1428_o_plan-navigator-geruest-runde-1.md`
nachzuziehen: er schließt aus einer Prüfung in Richtung Kommando → Kennung auf
eine Zusage in beide Richtungen. Die Plandatei lag außerhalb der Grenzen dieser
Aufgabe. Damit die Nachbesserung nicht mit dem Schließen des Datensatzes
verschwindet, steht sie jetzt als eigener Datensatz:
`issues/260804-1015_o_planabsatz-zu-9b-behauptet-mehr-als-er-geprueft-hat.md`.

Nicht angefasst, weil außerhalb der Grenzen: `resources/default-keymap.toml`,
`crates/krk-ui/`, `crates/krk-bench/`, `xtask/`, Plandatei und Spec. Die vier
weiteren Defekte, die der `ontocoder` bei derselben Arbeit gemeldet hat, sind
unberührt geblieben.
