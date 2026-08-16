# E1: Das Kommando, die zwei Fallunterscheidungen und die zwei Probenlisten

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt E1
**Baumstand vor der Arbeit:** `09baffd`
**Vorbedingungen:** C1 (`4a54212`)
**Erfüllt:** C2.7 (Kommandohälfte), C2.8, C2.9
**Nicht committet:** E1 landet zusammen mit E2. Der Baum ist dazwischen rot, und das ist der vom Plan angesagte Zustand.

## Was entstanden ist

Sieben Dateien, und die sechs Stellen des Plans stehen alle.

**`Kommando::InhaltssucheUmschalten`** entsteht in
`crates/krk-core/src/tasten/belegung.rs` unmittelbar hinter
`TiefeSucheUmschalten`, mit der Kennung `inhaltssuche_umschalten` in
`Kommando::KENNUNGEN`; die Feldbreite steigt von 78 auf 79. Der Doc-Kommentar
sagt, dass der Befehl das Kennzeichen kippt und die Mindestlänge des
Filtertexts nicht selbst fragt: ob das Kennzeichen wirkt, beantwortet
`Ordnermodell::inhalt_wirkt` an einer Stelle, und ob der Befehl zulässig war,
entscheidet der Wirkungsbereich. Das ist dieselbe Trennung, die „Deep" bei
fehlendem Filtertext schon zieht.

**Die zwei übersetzerpflichtigen Fallunterscheidungen** haben je eine Zeile
bekommen: `Kommando::wirkungsbereich` ordnet die Variante
`Wirkungsbereich::Ueberall` zu, neben „Deep" und aus derselben Erwägung (ein
Schalter der Bereichsleiste fällt aus jedem Fokus an);
`belegungsmodell::bereich_des_kommandos` ordnet sie
`Funktionsbereich::Dateilisting` zu, damit sie im Hauptmenü neben der tiefen
Suche steht. `Wirkungsbereich` behält seine sieben Werte, `Fokus` seine fünf.

**Der Zweig in `Anwendungsdelegierter::kommando_ausfuehren`** steht da. Er ist
die eine der sechs Stellen, für die weder Übersetzer noch Probe bürgt: das
`match` endet in einem Auffangzweig über `bereichskommando`, und ein Kommando
ohne eigenen Zweig fiele dort still hindurch. Der Kommentar darüber schreibt
das aus. Adresse und Rückgabewert folgen „Deep" Zeile für Zeile: das **aktive**
Dateifenster und nicht das fokussierte, und immer `true`.

**`DateifensterQuelle` bekommt `inhaltssuche_umschalten` und
`inhaltssuche_steht`**, gebaut nach `tiefe_suche_umschalten` und
`tiefe_suche_steht`: Ausleihe, `inhalt_setzen(!inhalt)`, dann
`durchlauf_nachziehen`, `umsortiert`, `meldung_gewechselt`. Beide fragen und
schreiben am aktiven Dateifenster. Das Neuaufbauen der Sicht gehört
`Ordnermodell::inhalt_setzen`, das Anstoßen und Abbrechen des Laufs
`Tabliste::durchlauf_nachziehen`; hier fällt für beides kein Zweig an.

**Die zwei Probenlisten sind nachgezogen** und nicht als Zahl geändert:
`OHNE_KOMBINATION_AB_WERK` (`crates/krk-core/tests/belegung.rs`) wächst von vier
auf fünf Einträge, das Literal in
`belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte`
ebenso. Beide Doc-Kommentare nennen den Datensatz, dem die Wahl folgt: die
Nutzerantwort vom 260814-1610 zu „Deep", der ein zweiter Schalter derselben Art
folgt, statt eine der frei gehaltenen Kombinationen zu belegen. Die Doppelung
der Liste bleibt bestehen und ist weiter offen
(`circles/260814-1551-…/decisions/260814-2326_o_wird-die-liste-der-funktionen-ohne-kombination-an-einer-stelle-gefuehrt.md`).

**Nicht angefasst**, und das ist eine Feststellung und kein Vergessen:
`kommandos/zulaessigkeit.rs`, `Fokus`, `schiebt_auffrischung_auf`.

## Was der Plan nicht angesagt hat

**Fünf Zahlen in Doc-Kommentaren zählen mit, und der Plan nennt sie nicht.**
Vier in `crates/krk-ui/src/belegungsausgabe.rs` (Modulkopf zweimal, Tabelle
einmal, der Rumpf von `wirkung`, der Doc-Kommentar der Probe) und eine in
`crates/krk-ui/src/appkit/menue.rs` sagten „78 der 84 Funktionen"; sie sagen
jetzt „79 der 85". Keine Probe hält sie, und genau das ist der Grund, aus dem
sie nachgezogen sind: stille Zahlen in Prosa sind der wiederkehrende Defekt
dieses Projekts. Die Zahl 85 gilt erst mit E2; beide Schritte landen in einem
Zug.

**Eine Zahl ist bewusst stehen geblieben.** Der Doc-Kommentar von
`TiefeSucheUmschalten` sagt, die Kennung folge „der Schreibweise der 77
vorhandenen". Das beschreibt den Stand, zu dem „Deep" hinzukam, und gehört zur
Begründung eines anderen Befehls. Sie umzuschreiben hieße, diese Begründung
umzudatieren.

**Nach E1 ist der Baum an zwei Stellen rot, nicht an einer.** Der Plan sagt die
Paarungsprobe an. Daneben hält `cargo clippy -- -D warnings` an, weil
`inhaltssuche_steht` bis E3 keinen Rufer hat; sein Rufer ist
`bereichsleiste_nachziehen`, und der entsteht dort. Ein `#[allow(dead_code)]`
ist bewusst **nicht** gesetzt: es wäre Rückstand, den nach E3 niemand mehr
sucht. Ein grünes `make check` gibt es damit erst nach E3 und nicht schon nach
E2.

## Prüfung

| Kommando | Ausgang |
|---|---|
| `cargo build -p krk-core` | exit 0 |
| `cargo build --workspace` | exit 0, eine Warnung `dead_code` an `inhaltssuche_steht` |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | **rot**, `dead_code` an `inhaltssuche_steht` |
| `cargo test --workspace` | **rot**, vier Proben |
| `make check` | **exit 2**, hält an `test` an |

Die vier roten Proben, alle mit derselben Wurzel — die Belegungsdatei kennt
`inhaltssuche_umschalten` noch nicht, und das ist E2:

- `krk-core --lib`: `tasten::belegung::tests::jede_kennung_der_kommandos_steht_in_der_auslieferungsbelegung` — „InhaltssucheUmschalten nennt die Kennung inhaltssuche_umschalten, die Auslieferungsbelegung kennt sie nicht"
- `krk-core --test belegung`: `jedes_gebaute_kommando_haengt_an_seiner_ausgelieferten_taste` — „die Auslieferungsbelegung kennt die Funktion inhaltssuche_umschalten nicht"
- `krk-ui --bin krk`: `belegungsausgabe::tests::jede_belegte_funktion_steht_in_der_datei_und_keine_unbelegte` — die Liste der unbelegten Funktionen hat vier Einträge, erwartet sind fünf
- `krk-ui --bin krk`: `belegungsausgabe::tests::die_dritte_spalte_haelt_die_vier_begruendungslagen_auseinander` — 78 Funktionen mit Kommando in der Belegung gegen 79 in `Kommando::KENNUNGEN`

Die dritte und die vierte nennt der Plan nicht namentlich; beide fallen mit E2
von selbst. Alle übrigen Ziele sind grün.

## Geänderte Dateien

- `crates/krk-core/src/tasten/belegung.rs`
- `crates/krk-core/tests/belegung.rs`
- `crates/krk-ui/src/belegungsmodell.rs`
- `crates/krk-ui/src/belegungsausgabe.rs`
- `crates/krk-ui/src/appkit/anwendung.rs`
- `crates/krk-ui/src/appkit/tabelle.rs`
- `crates/krk-ui/src/appkit/menue.rs`
