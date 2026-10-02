# Aufraeumrunde: die Beschreibungen zum Verweisziel und die fehlende Socket-Probe

**Status:** Complete
**Agent:** coder
**Datum:** 260815-1850 bis 260815-1900
**Ausgangsstand:** `311693c`
**Domain:** code

## Auftrag

Vier Defektdatensaetze und zwei Stellen in `CLAUDE.md`, ohne Verhaltensaenderung ausser
einer neuen Probe. Kein Commit, Pruefung genau einmal am Ende.

- `shared/issues/260815-1752_*_zwei-modulkoepfe-nennen-das-verweisziel-am-deskriptor-obwohl-es-am-pfad-fragt.md`
- `shared/issues/260815-1812_*_ein-verweis-im-modulkopf-des-verweisziels-zeigt-auf-einen-datensatz-der-nie-so-hiess.md`
- `shared/issues/260815-1845_*_der-doc-kommentar-von-unerreichbar-zaehlt-drei-gruende-auf-und-stat-scheitert-an-mehr.md`
- `shared/issues/260815-1846_*_der-unix-socket-ist-der-einzige-der-drei-gemessenen-fehlfaelle-ohne-probe.md`

## Pruefung

```
cargo fmt --all --check && cargo clippy --workspace --all-targets && cargo test --workspace
```

Exit 0. Keine Warnung, kein Fehlschlag; `eine_verknuepfung_auf_einen_socket_ist_kein_ordner`
laeuft gruen mit.

## A — die sieben Stellen, die „am Deskriptor" sagten

Alle sieben nachgezogen. Die Zeitform ist dabei die eigentliche Arbeit gewesen: was den alten
Zustand beschreibt, steht jetzt im Praeteritum und veraltet damit nicht noch einmal.

| Stelle | Vorher | Nachher |
|---|---|---|
| `krk-core/src/verzeichnis/mod.rs`, Modulskizze | Pfeil `sys ──> verweisziel` | der Pfeil faellt; `verweisziel` steht allein neben der Skizze, „an keinem der acht" |
| `krk-core/src/verzeichnis/mod.rs`, `fcntl(2)`-Rufer | drei Rufer, darunter `verweisziel` | zwei Rufer, wie sie `sys.rs:15-16` fuehrt |
| `krk-core/src/verzeichnis/mod.rs`, Absatz zu `verweisziel` | „haengt als einziges Modul unmittelbar an `sys`", „Gefragt wird sie am Deskriptor" | „haengt als einziges Modul an gar keinem anderen", „Gefragt wird sie am Namen ueber `std::fs::metadata`"; der alte Weg steht im Praeteritum daneben |
| `krk-ui/src/appkit/tabelle.rs:1404` | „ueber `verweisziel::bestimmen` am Deskriptor" | „am Namen" |
| `krk-core/src/verzeichnis/verweisziel.rs:49-53` | „benutzt", „gibt ab", „besteht fort", „kauft nicht weg" | „benutzte", „gab ab", „bestand fort", „kaufte nicht weg" |
| `krk-core/tests/verzeichnis.rs:1844` | „Aufgeloest wird sie erst hier, am Deskriptor" | „am Namen" |
| `krk-core/tests/verzeichnis.rs`, Ring-Probe | „aus demselben `open(2)`" | „aus demselben Aufruf", der Aufruf heisst `stat(2)`; dazu der Satz zu `SYMLOOP_MAX` aus Befund `260815-1845` |

## B — der Verweis auf einen Datensatz, den es nie gab

`verweisziel.rs:95` zeigt jetzt auf
`shared/decisions/260815-1749_*_meldet-der-doppelklick-auf-einen-ordner-ohne-leserecht-oder-schweigt-er-wie-heute.md`.
Speicher und Namensteil sind beide berichtigt, die Sternform bleibt (Entscheid `260815-1145`).
Die zwei weiteren Fundstellen in `issues/` und `history/` sind eingefrorene Aufzeichnungen und
unberuehrt.

## C — `Unerreichbar` zaehlte drei Gruende auf

Der erlaeuternde Satz ist von einer Liste zu einer Regel geworden: der Wert traegt jeden
Fehlschlag von `stat(2)` am Pfad, und die vier haeufigen Faelle sind ausdruecklich Beispiele.
Keine `errno`-Aufzaehlung, die als naechste veralten koennte. Die Zusicherung eine Ebene
darueber („ueberschneidungsfrei und vollstaendig") ist nicht angefasst; sie trug schon vorher.

## D — die Socket-Probe

`Pruefordner::socket` in `crates/krk-core/tests/gemeinsam/mod.rs`, ueber
`std::os::unix::net::UnixListener::bind`. Keine vierte Pruefordner-Fassung, kein Fremdaufruf,
keine neue Abhaengigkeit; `krk-core` fuehrt weiterhin kein `libc`. Die Probe
`eine_verknuepfung_auf_einen_socket_ist_kein_ordner` steht neben den beiden anderen.

Zwei Zahlen nachgemessen statt uebernommen. Der laengste Pfad des einzigen Rufers misst 92
Bytes gegen die 104 von `AF_UNIX`; die Zahl steht am Doc-Kommentar. Und die Probe laeuft nicht
leer: `stat` meldet den Socket als Nicht-Verzeichnis, `open(O_RDONLY|O_NONBLOCK)` scheitert an
ihm mit `errno 102`, also faengt sie einen Rueckfall auf den Deskriptorweg tatsaechlich.

## E — die zwei Stellen in `CLAUDE.md`

Zeile 38: `v0.3.0` → `v0.4.1`. Selbst geprueft an `Cargo.toml:13` (`version = "0.4.1"`) und am
Tag `v0.4.1`, der auf `838432c` zeigt.

Zeile 69: der Klammerzusatz zum Doppelklick fuehrt jetzt drei Ausgaenge statt zweier, den
unerreichbaren Verweis eingeschlossen. Sonst nichts an der Datei angefasst.

## Was daneben gefunden und nicht angefasst wurde

`crates/krk-ui/src/appkit/tabelle.rs:1432` traegt die dreigliedrige Aufzaehlung aus Befund
`260815-1845` ein drittes Mal, im Kommentar des `Unerreichbar`-Zweiges. Die Stelle steht in
keinem der vier Datensaetze; abgelegt als
`shared/issues/260815-1858_*_die-dritte-aufzaehlung-der-unerreichbar-gruende-steht-im-einstiegsweg-und-ist-dieselbe-verengung.md`.

## Datensaetze

Alle vier auf `_c_` gesetzt, je mit einer `Resolved:`-Notiz. Ein neuer Datensatz offen
(`260815-1858`). Kein Commit gefahren.
