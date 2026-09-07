# Die ersetzende Schaltfläche des Konfliktblattes trägt zwei Beschriftungen statt einer

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was verlangt war

Die Nutzerentscheidung vom 260907-0703 zu
`260826-1221_*_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`:
das Verhalten bleibt ungleich, und die Schaltfläche sagt künftig, was sie tut.
Der erste Anlauf (`260907-0718-zwei-wortlaute-insgesamt-und-ueberschreiben.md`)
hat den Auftrag an der Bahngrenze angehalten, weil `anwendung.rs` und
`operationen.rs` gesperrt waren; die Sperre ist mit `8e7a067` aufgehoben.

## Der Schnitt

Der Vorschlag des Vorlaufs trägt und ist gefolgt: die reine Regel steht neben
`erzeugt_genau_ein_ziel` in `crates/krk-ui/src/kommandos/operationen.rs`, stellt
dieselbe Frage an dieselbe `Art` und ist wie jene eine vollständige
Fallunterscheidung ohne Auffangzweig. Eine siebte Vorgangsart hält damit den Bau
an, statt still in die falsche Beschriftung zu fallen.

Drei Abweichungen vom Vorschlag, jede mit ihrem Grund:

1. **Die Antwort ist eine Aufzählung und kein `bool`.** `Ersetzungsweg` trägt
   `Papierkorb` und `Endgueltig`. Ein `bool` beantwortete dieselbe Frage, ließe
   aber am Rufer offen, welche Seite welche ist.
2. **`zeigen` bekommt eine `Konfliktgestalt` statt zweier Argumente.** Ein achtes
   Argument an `konflikt::zeigen` verletzte `clippy::too_many_arguments` unter
   `-D warnings`. Die Struktur ist daneben der sachlich richtige Schnitt: beide
   Felder sind Antworten derselben Frage an dieselbe `Art` und gehen denselben
   Weg zum Blatt. `konfliktgestalt` fasst die zwei Rechnungen zusammen und
   rechnet nichts selbst.
3. **Die zwei Arten ohne Konfliktzweig bekommen `Endgueltig`.** Weder
   `InDenPapierkorb` noch `UmbenennenImStapel` ruft `konflikt_loesen`; das Blatt
   steht für sie nie. Von den zwei möglichen Irrtümern kostet nur einer etwas:
   eine Beschriftung, die zu viel ankündigt, erschreckt einmal; eine, die zu
   wenig ankündigt, lässt eine Datei für holbar halten, die weg ist. Der Grund
   steht im Doc-Kommentar, damit ein späterer Konfliktzweig die Zeile mitnimmt.

## Die zwei Wortlaute

| Weg | Beschriftung |
|---|---|
| Papierkorb (Packen, Entpacken) | „In den Papierkorb und ersetzen“ |
| endgültig (Kopieren, Verschieben, Abwurf) | „Endgültig löschen und ersetzen“ |

Beide sind parallel gebaut: erst der Verbleib des alten Eintrags, dann „und
ersetzen“. „Endgültig“ steht vorn, damit die Angabe zu lesen ist, ohne den Satz
zu Ende zu lesen. Der Wortlaut „Überschreiben“ steht auf keiner Schaltfläche
mehr; er sagt über den Verbleib nichts, und genau daran hing die Ungleichheit.
Der Tastenhinweis sagt darum „Cmd+Return ersetzt“ statt „überschreibt“ — die
Taste, nicht den Verbleib; der steht auf der Schaltfläche und nicht zweimal.

## Geänderte Dateien

- `crates/krk-ui/src/kommandos/operationen.rs` — `Ersetzungsweg`,
  `ersetzungsweg`, `Konfliktgestalt`, `konfliktgestalt`; zwei Doc-Kommentare an
  `erzeugt_genau_ein_ziel` nachgezogen; zwei neue Proben.
- `crates/krk-ui/src/appkit/blaetter/konflikt.rs` — `ersetzungsbeschriftung`,
  `schaltflaechen` und `zeigen` über die `Konfliktgestalt`, Tastenhinweis,
  Modulkopf; Probenmodul auf vier Vorgaben statt zwei Gestalten erweitert, zwei
  neue Proben für die zwei Beschriftungen.
- `crates/krk-ui/src/appkit/anwendung.rs` — `konflikt_fragen` ruft
  `konfliktgestalt`; Doc-Kommentar um die zweite Rechnung erweitert.
- `crates/krk-ui/src/appkit/blaetter/mod.rs` — die nachgebauten
  Konfliktschaltflächen einer Probe tragen den neuen Wortlaut.
- `crates/krk-core/src/operation/mod.rs` — `ziel_klaeren` schreibt aus, warum
  dieser Zweig endgültig löscht, während Packen und Entpacken in den Papierkorb
  räumen, und wo der Nutzer den Unterschied sieht.

Die Laufzeitprüfung in `blaetter/mod.rs` (`mit_schaltflaechen`) ist unberührt;
jede der vier Vorgaben trägt weiterhin „Abbrechen“ mit `Wirkung::Liegenlassen`,
und die Probe `beide_gestalten_lassen_ueber_abbrechen_liegen` hält das jetzt über
alle vier statt über zwei.

## Verifikation

Alle fünf Abnahmekommandos am vollen Arbeitsbereich, nach dem Formatierlauf noch
einmal von vorn:

```
cargo build --workspace                                     exit 0
cargo test --workspace                                      exit 0   (1914 Proben, 24 Läufe, 0 rot)
cargo clippy --workspace --all-targets -- -D warnings       exit 0
cargo fmt --all --check                                     exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps  exit 0
```

Ein Zwischenlauf hatte `cargo fmt --all --check` auf 1 stehen, an einem
umgebrochenen `assert_ne!` im Probenmodul von `konflikt.rs`; `cargo fmt --all`
hat es gerichtet, danach sind alle fünf grün.

**Nicht geprüft ist das laufende Bündel.** Ob die zwei Wortlaute in der Breite
der `NSAlert`-Schaltflächen ohne Kürzung stehen, ist am Bündel zu sehen und hier
nicht zu behaupten; der längere trägt 30 Zeichen. Das ist Nutzerarbeit wie jeder
Abnahmelauf dieses Vorhabens.

## Der Entscheidungsdatensatz

`260826-1221_*_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`
trägt jetzt eine `Implemented:`-Zeile und den Marker `_i_`. Die Zeile zitiert
dieses Protokoll und keinen Commit-Hash: committet wird nicht von hier.

## Abgelegt

- `260907-0750_*_eine-nachbildung-der-konfliktschaltflaechen-steht-in-blaetter-mod-rs-und-nichts-haelt-sie-am-original.md`
  — die Probe in `blaetter/mod.rs` baut die Schaltflächen des Konfliktblattes von
  Hand nach, und nichts hält die Nachbildung am Original; sie blieb grün, als der
  Wortlaut fiel.
