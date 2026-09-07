# Die zweite Zahl der Löschabfrage bekommt „insgesamt"; die Beschriftung der Schaltfläche „Überschreiben" bleibt liegen

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was verlangt war

Zwei Nutzerentscheidungen vom 260907-0703, festgehalten in
`260905-2008-orchestrator-session.md` `## Fuenf weitere Entscheidungen am 260907-0703
beantwortet`:

1. Die zweite Zahl der Löschrückfrage bekommt das Wort „insgesamt"
   (`260818-0512_*_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`,
   Möglichkeit 2).
2. Die Schaltfläche „Überschreiben" des Konfliktblattes wird je nach Fall beschriftet, weil
   sie im Kontextmenü in den Papierkorb räumt und beim Kopieren, Verschieben und Abwurf
   endgültig löscht
   (`260826-1221_*_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`).

Der Dispatch hat vier Dateien ausdrücklich gesperrt, weil eine zweite Bahn gleichzeitig am
Kontextmenü arbeitet: `crates/krk-ui/src/kommandos/kontextmenue.rs`,
`crates/krk-ui/src/kommandos/operationen.rs`, `crates/krk-ui/src/appkit/anwendung.rs` und
`crates/krk-ui/src/appkit/tabelle.rs`.

## Auftrag 1: erledigt

Geändert wurde allein `crates/krk-ui/src/kommandos/loeschwarnung.rs`.

- `Warngrund::wortlaut` liefert für den sechsten Auslöser jetzt „mit 25 Einträgen insgesamt"
  und „mit mehr als 25 Einträgen insgesamt".
- Die Wortlauttafel im Modulkopf zieht mit.
- Der Doc-Kommentar von `wortlaut` schreibt aus, warum die beiden Wortlaute seit dem 260907
  von der Spalte „Wortlaut in der Frage" des Specs abweichen und warum das Abnahmekriterium
  trotzdem hält: es verlangt die Zahl 25 beziehungsweise „mehr als 25" im Wortlaut, und beide
  tragen sie.
- Zwei bestehende Proben nachgezogen (`jeder_grund_traegt_seinen_wortlaut`, die erwartete
  Erläuterung in `die_frage_nennt_den_ersten_grund_und_die_erlaeuterung_die_uebrigen`), dazu
  drei Kommentarstellen, die den alten Wortlaut wörtlich zitierten.
- Eine neue Probe `die_zweite_zahl_sagt_dass_sie_insgesamt_zaehlt` hält beide Fragen als ganze
  Zeichenketten: „Diese 25 Einträge mit 25 Einträgen insgesamt in den Papierkorb räumen?" und
  die Form mit „mehr als". Das ist der Fall, um den die Entscheidung läuft.

Die Übersetzungszeit-Zusicherung `nennt_die_zahl` hält unverändert, weil die Dezimalschreibung
der Schwelle in beiden Wortlauten stehen bleibt. Kein Verhalten geändert.

## Auftrag 2: an der Bahngrenze angehalten, nichts geschrieben

Die Unterscheidung darf nicht geraten werden, sie muss der Wirkung des jeweiligen Weges
folgen. Diese Auskunft hängt an der `Art` des Vorgangs, und die erreicht das Blatt nur über
zwei der vier gesperrten Dateien:

- `konflikt::zeigen` (`crates/krk-ui/src/appkit/blaetter/konflikt.rs:234`) hat genau einen
  Rufer, `Anwendungsdelegierter::konflikt_fragen`
  (`crates/krk-ui/src/appkit/anwendung.rs:7121`). Jede zusätzliche Angabe an das Blatt ändert
  diesen Aufruf.
- Die reine Regel gehörte neben `erzeugt_genau_ein_ziel`
  (`crates/krk-ui/src/kommandos/operationen.rs:501`), die dieselbe Frage über dieselbe `Art`
  schon beantwortet und dafür eine vollständige Fallunterscheidung ohne Auffangzweig führt.
- Ein dritter Weg über `Konfliktfrage` scheidet aus: die Struktur steht in
  `operationen.rs:357` und wird in `anwendung.rs:8690` gebaut.

Es gibt keinen Zuschnitt, der ohne diese Dateien auskommt, und eine halbe Umsetzung ließe den
Baum entweder rot oder mit einer geratenen Vorgabe stehen. Also nichts geschrieben und
gemeldet.

## Verifikation

Der geteilte Baum war während des Laufs rot, und zwar aus fremder Ursache: die zweite Bahn hat
`Kontextbefehl::ImFinderZeigen` in `ImFinderAnzeigen` umbenannt, während `anwendung.rs` den
alten Namen noch nannte. Geprüft wurde deshalb in einem `git worktree` auf `354dac3`, in den
allein die geänderte `loeschwarnung.rs` kopiert wurde. Dort sind alle fünf Abnahmekommandos
grün:

```
cargo build --workspace                          exit 0
cargo test --workspace                           exit 0   (1905 Proben, 0 rot)
cargo clippy --workspace --all-targets -D warnings exit 0
cargo fmt --all --check                          exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps  exit 0
```

Der Prüfbaum ist danach mit `git worktree remove` wieder abgeräumt.

## Was noch offen ist

- `260818-0512_*_wie-lautet-die-frage-wenn-der-umfang-der-genannte-grund-ist-und-die-zahl-doppelt-dasteht.md`
  steht weiter auf beantwortet. Der Code trägt die Antwort; der Vermerk `Implemented:` braucht
  den Commit-Hash, und committet wird nicht von hier.
- `260826-1221_*_raeumt-ueberschreiben-auch-beim-kopieren-und-verschieben-in-den-papierkorb.md`
  bleibt beantwortet und nicht umgesetzt, bis die zweite Bahn ihre vier Dateien freigibt.
- Am Befund
  `260820-0602_*_make-check-prueft-den-ganzen-arbeitsbereich-und-bricht-bei-parallelen-agenten-an-fremden-dateien-ab.md`
  steht jetzt eine `Also seen:`-Zeile: der Fall tritt eine Stufe härter auf, wenn die Datei
  der anderen Bahn nicht bloß unformatiert ist, sondern nicht übersetzt.
