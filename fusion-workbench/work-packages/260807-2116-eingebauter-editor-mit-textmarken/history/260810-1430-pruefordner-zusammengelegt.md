# Der selbstabraeumende Pruefordner: zwoelf Fassungen auf drei

**Status:** Complete
**Agent:** coder
**Datum:** 260810-1430
**Aufgabe:** Behebung von
`issues/260810-1330_*_derselbe-selbstabraeumende-pruefordner-steht-zwoelfmal-im-baum.md`

## Ergebnis

Zwoelf Fassungen sind drei geworden, eine je Kiste. Drei bleiben es, und der
Grund liegt nicht an der Sorgfalt, sondern an der Kistenstruktur von Rust.

```
vorher                                   nachher
------                                   -------
krk-core/tests/ablage.rs        \
krk-core/tests/belegung.rs       \
krk-core/tests/navigation.rs      >----- krk-core/tests/gemeinsam/mod.rs
krk-core/tests/operation.rs       /        (mod gemeinsam; je Testziel)
krk-core/tests/text.rs           /
krk-core/tests/verzeichnis.rs   /

krk-ui/src/vorschaumodell.rs    \
krk-ui/src/editormodell.rs       >------ krk-ui/src/pruefordner.rs
krk-ui/src/leistenmodell.rs      /         (#[cfg(test)] mod in main.rs)
krk-ui/src/kommandos/pfadeingabe.rs /

krk-bench/src/fixture.rs        \
krk-bench/src/messen.rs          >------ krk-bench/src/wegwerfordner.rs
                                           (#[cfg(test)] mod in main.rs)
```

## Warum nicht eine

Die Frage war ausdruecklich gestellt: welche der zwoelf Fassungen erreichen
einander, und nur die sind zusammenlegbar. Die Antwort steht in drei Tatsachen
ueber die Ziele des Workspace.

Jede Datei unmittelbar in `crates/<x>/tests/` ist ein eigenes Testziel und damit
eine eigene Kiste. Die sechs Fassungen des Kerns koennen einander nichts
einbinden, wohl aber alle sechs ein Modul in einem **Unterverzeichnis** von
`tests/`: `tests/gemeinsam/` ist kein Testziel, weil Cargo nur `tests/*.rs` und
`tests/*/main.rs` als Ziel nimmt. Das ist der uebliche Weg und er trug hier.

`krk-ui` und `krk-bench` haben je nur ein Binaerziel. Ihre Fassungen liegen
darin, also in je einer Kiste, und ein `#[cfg(test)] mod` neben den uebrigen
Modulen genuegt. `krk-ui/tests/syntaxkiste.rs` erreicht den Code der Kiste
uebrigens nicht und traegt auch keine Fassung; das passt zueinander.

Ein Ort, den alle drei Ziele einbinden koennten, gibt es nicht. Er entstuende erst
mit einem Bibliotheksziel fuer `krk-ui` oder einer eigenen Kiste im Workspace.
Das erste ist Gegenstand der offenen Nutzerentscheidung
`decisions/260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md`
und war ausdruecklich vom Auftrag ausgenommen; das zweite haette die Wurzel-
`Cargo.toml` und `krk-bench/Cargo.toml` verlangt und lag ausserhalb der
Dateigrenze. Keine der drei Kisten-Cargo.toml ist am Ende angefasst worden.

Damit die naechste Lesung nicht wieder mit "eine genuegte" anfaengt, sagt jeder
der drei Modulkoepfe selbst, gegen welche der beiden anderen er nicht
zusammenlegbar ist und warum.

## Was die Zusammenlegung nebenbei geheilt hat

**Vier Zaehler in einem Prozess.** Alle Einheitsproben von `krk-ui` uebersetzen in
ein Probenprogramm. Die vier Fassungen fuehrten je einen eigenen `AtomicU64`,
also konnten zwei Proben mit demselben Zweck in verschiedenen Modulen dieselbe
Laufnummer und damit denselben Ordnernamen bekommen. Jetzt zaehlt ein Zaehler
fuer die Kiste, und der Fall ist ausgeschlossen. Aufgefallen ist das erst beim
Zusammenlegen; es war in keinem Datensatz benannt.

**Abraeumen gegen entzogene Rechte, jetzt in allen sechs Zielen des Kerns.** Nur
`operation.rs` drehte einem Eintrag mit `0o000` die Rechte zurueck, bevor es ihn
loeschte. Die gemeinsame Fassung tut es fuer alle, aber in zwei Stufen: zuerst
`remove_dir_all`, das einen Ordner mit 5.000 Eintraegen in einem Zug abraeumt,
und nur wenn der scheitert, der Abstieg mit einem `set_permissions` je Eintrag.
Ein Abstieg als erster Weg haette die Abnahme des Verzeichnislesers teurer
gemacht, ohne etwas zu koennen, was der schnelle Weg nicht kann.

## Die vereinheitlichten Unterschiede

Die zwoelf Fassungen unterschieden sich nicht im Kern, sondern in den Helfern.
Vier Bedeutungen von `datei` waren darunter, und die Wahl fiel so:

| vorher | nachher | Grund |
|---|---|---|
| `datei(name, &str)` und `datei(name, &[u8])` | `datei(name, impl AsRef<[u8]>)` | beide Aufrufformen bleiben unveraendert |
| `datei(name, usize)`, `datei(name)`, `datei_mit(name, usize)` | `fuelldatei(name, usize)` | eine Datei aus N Fuellbytes ist etwas anderes als eine mit Inhalt |
| `ordner(name)` und `unterordner(name)` | `ordner(name) -> PathBuf` | `ordner.ordner("x")` ist im Baum schon die Schreibweise |
| `verknuepfung(name, &str)` und `verknuepfung(name, &Path)` | `verknuepfung(name, impl AsRef<Path>)` | beide Aufrufformen bleiben unveraendert |

`ablage_mit` ist **nicht** mitgewandert, sondern eine freie Funktion in
`tests/belegung.rs` geworden. Der gemeinsame Pruefordner haelt Ordner und
Dateien; eine `Ablage` ist ein Gegenstand des Kerns, und nur diese eine Datei
braucht sie. `luecke`, `roehre` und `verstecken` sind dagegen mitgewandert,
obwohl je nur eine Datei sie ruft: sie sind Dateimechanik, also genau der
Gegenstand des Moduls, und wer morgen eine Roehre braucht, findet sie dort statt
sie neu zu schreiben.

In `krk-ui` gibt es zwei Erzeuger. `neu` legt den Ordner an, `nur_name` liefert
nur den Namen. Die Gueltigkeitsproben des `leistenmodell` brauchen denselben Pfad
einmal vorhanden und einmal fehlend und schalten mit `anlegen` und `loeschen`
dazwischen; ohne den zweiten Erzeuger haetten sie den frisch angelegten Ordner
erst wieder loeschen muessen, um die Ausgangslage zu haben.

`#![allow(dead_code)]` steht im Kopf von `tests/gemeinsam/mod.rs`, nicht an
einzelnen Funktionen. Die sechs Testziele uebersetzen das Modul je einzeln, und
was das eine nicht braucht, ist in dessen Uebersetzung ungenutzt.

## Der ueberholte Satz

Der Modulkopf von `tests/verzeichnis.rs` sagte bis heute, ein
Pruefordner-Erzeuger sei "bewusst noch nicht" da und komme mit Schritt 3 der
Runde 1. Mit dem Abschluss der Runde 2 war der Satz ueberholt; er sagt jetzt,
woher der Pruefordner kommt. Dieselbe Berichtigung in `tests/text.rs`, dessen
Kopf auf die Fassung in `verzeichnis.rs` verwies, und in den Koepfen von
`editormodell`, `leistenmodell` und `vorschaumodell`, die je zwei bis drei
Schwesterfassungen namentlich nannten.

## Abnahme

| Kommando | Ergebnis |
|---|---|
| `cargo build --workspace` | exit 0 |
| `cargo test --workspace` | exit 0, 15 Testprogramme, 0 Fehlschlaege |
| `cargo clippy --workspace --all-targets` | exit 0, keine Warnung |
| `cargo fmt --all --check` | exit 0 |

Nach dem vollen Testlauf liegt unter `$TMPDIR` und unter `/tmp` kein einzelner
Pruefordner mehr. Der Bestand vor und nach dem Lauf war derselbe: neun
`krk-messplan-*.toml` mit Zeitstempeln vom 260805 bis 260807, also aelter als der
Lauf, und die fuenf Abnahme-Pruefordner `krk-pruefordner-{a,b,a-l6,gross}` und
`krk-probe`, die der Nutzer angelegt hat.

## Zwei Defekte daneben gefunden

**`Planordner` ist die dreizehnte Fassung.** Er steht in
`krk-ui/src/messmodus.rs:1685`, tut unter seiner Messplan-Logik dasselbe und war
im Datensatz nicht gezaehlt. `messmodus.rs` lag ausserhalb der Dateigrenze, also
abgelegt als
`issues/260810-1430_*_planordner-in-messmodus-ist-die-dreizehnte-fassung-und-kann-jetzt-auf-die-gemeinsame-aufsetzen.md`.
Er kann jetzt auf `crate::pruefordner::Pruefordner` aufsetzen und braucht dafuer
ein `ordner(name)`, das die `krk-ui`-Fassung noch nicht fuehrt.

**Die neun Messplan-Reste haben eine Ursache.** `plan_schreiben` legt
`krk-messplan-<pid>.toml` an, und abgeraeumt wird die Datei an genau einer
Stelle, hinter der Rundenschleife. Jeder Abbruch in der Schleife laesst sie
liegen, und der haeufige Abbruch ist `NICHT_IM_VORDERGRUND`. Abgelegt als
`shared/issues/260810-1430_*_ein-abgebrochener-messlauf-laesst-seinen-messplan-im-temporaerverzeichnis-liegen.md`,
im geteilten Speicher, weil der Befund an der Messstrecke der Runde 1 haengt und
nicht an der Directive dieses Circles. Die vorgeschlagene Behebung ist dieselbe
Bauform, die dieser Defekt gerade dreimal aufgeschrieben hat: ein Halter mit
`Drop`.

## Geaenderte Dateien

Neu:

- `crates/krk-core/tests/gemeinsam/mod.rs`
- `crates/krk-ui/src/pruefordner.rs`
- `crates/krk-bench/src/wegwerfordner.rs`

Geaendert:

- `crates/krk-core/tests/{ablage,belegung,navigation,operation,text,verzeichnis}.rs`
- `crates/krk-ui/src/{main,vorschaumodell,editormodell,leistenmodell}.rs`
- `crates/krk-ui/src/kommandos/pfadeingabe.rs`
- `crates/krk-bench/src/{main,fixture,messen}.rs`

Keine `Cargo.toml` ist angefasst, `resources/**` und das Plandokument auch nicht.
