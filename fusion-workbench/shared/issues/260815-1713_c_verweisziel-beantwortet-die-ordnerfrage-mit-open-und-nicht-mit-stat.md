`verweisziel::bestimmen` beantwortet die Ordnerfrage mit `open(2)`, das sie nicht entscheiden kann

---

`crates/krk-core/src/verzeichnis/verweisziel.rs:84-94` fragt „steht hinter diesem Namen ein
Verzeichnis?", indem es das Ziel **öffnet**:

```rust
pub fn bestimmen(pfad: &Path) -> Verweisziel {
    let datei = match sys::ohne_warten_oeffnen(pfad) {
        Ok(datei) => datei,
        Err(fehler) => return unerreichbar(&fehler),
    };
    match datei.metadata() { … }
}
```

`open(2)` scheitert an Einträgen, die es sehr wohl gibt und die sehr wohl kein Verzeichnis
sind. Jeder dieser Fälle kommt als `Unerreichbar` zurück statt als `KeinOrdner`. Die Frage
selbst ist entscheidbar, nur nicht mit diesem Aufruf: `stat(2)` beantwortet sie für alle
diese Fälle richtig, blockiert an keiner Röhre und öffnet nichts.

---

**Schwere:** hoch. Kein Absturz und kein Datenverlust; falsch sind die Einordnung und die
Meldung, die der Nutzer bekommt, und die Zusicherung des Doc-Kommentars trägt nicht.
**Gefunden von:** coderev, Durchsicht des Bereichs `a2670db..8c06747`
**Betroffen:** `crates/krk-core/src/verzeichnis/verweisziel.rs:29-38`, `:48-73`, `:84-94`;
mitbetroffen `crates/krk-core/tests/verzeichnis.rs:1920-1946`
**Domain:** code

## Am Referenzgerät gemessen

macOS 24.6.0, uid 502, je eine Verknüpfung auf das genannte Ziel, `open(O_RDONLY|O_NONBLOCK)`
gegen `stat` verglichen:

| Ziel der Verknüpfung | `stat` sagt | `open` sagt | `bestimmen` liefert | richtig wäre |
|---|---|---|---|---|
| Unix-Socket | kein Verzeichnis | `EOPNOTSUPP` | `Unerreichbar` | `KeinOrdner` |
| gewöhnliche Datei, Modus 000 | kein Verzeichnis | `EACCES` | `Unerreichbar` | `KeinOrdner` |
| Verzeichnis, Modus 0111 | **Verzeichnis** | `EACCES` | `Unerreichbar` | `Ordner` |
| benannte Röhre ohne Schreiber | kein Verzeichnis | ok | `KeinOrdner` | `KeinOrdner` |
| `/dev/null` | kein Verzeichnis | ok | `KeinOrdner` | `KeinOrdner` |

Der Socket ist damit **nicht** der einzige Fehlfall, wie die Abschlussnotiz von
`260814-1612` und `shared/history/260815-1658-coder-…` annehmen, und er scheitert auf diesem
Gerät auch nicht mit `ENXIO`, sondern mit `EOPNOTSUPP`. Der praktisch häufigste Fehlfall ist
die zweite Zeile: eine Verknüpfung auf eine Datei ohne Leserecht bekommt jetzt
„lässt sich nicht öffnen: Permission denied" in die Statuszeile, statt wie jede andere Datei
an das Standardprogramm zu gehen.

## Die Überschneidungsfreiheit ist damit nicht gegeben

`verweisziel.rs:50-52` sagt zu: „**Drei Werte, ueberschneidungsfrei und vollstaendig, ohne
Auffangzweig.**" Die Aussage stimmt für die drei **Ausgänge des Verfahrens** und nicht für
die Zustände, die die Werte benennen. `KeinOrdner` beschreibt sich selbst als „Etwas, das
kein Verzeichnis ist: eine gewoehnliche Datei, eine Geraetedatei, eine benannte Roehre"
(`:57-60`) — und eine gewöhnliche Datei ohne Leserecht ist genau das und kommt trotzdem als
`Unerreichbar` zurück. Die beiden Doc-Kommentare beschreiben denselben Zustand.

Das ist der Fall aus `rules/critical-stance.md` §4: der Schnitt ist nicht überschneidungsfrei,
weil das Verfahren eine andere Frage beantwortet als die, die die Werte benennen.

## Die zwei genannten Gründe für den Deskriptorweg tragen hier beide nicht

Der Modulkopf (`:29-38`) begründet die Wahl von `sys::ohne_warten_oeffnen` mit den zwei
Gründen, aus denen der Editor sie 260809 bekommen hat. Keiner gilt an dieser Stelle:

1. **Das Fenster zwischen Prüfung und Öffnen.** Es verschwindet nur, wenn derselbe
   Deskriptor danach benutzt wird. `bestimmen` gibt seinen Deskriptor sofort wieder ab, und
   der Aufrufer öffnet den Namen ein zweites Mal: `tabelle.rs:1444` ruft
   `ordner_lesen(&ziel, None)`, und der Lesevorgang öffnet den **Pfad**. Das Fenster besteht
   unverändert fort; der Deskriptorweg kauft es nicht weg.
2. **Das Blockieren an einer benannten Röhre.** Es tritt nur bei `File::open` auf. `stat(2)`
   blockiert an einer Röhre nicht — das steht wörtlich in der Abschlussnotiz des Bauenden
   selbst (`shared/history/260815-1658-coder-…`, Abschnitt „Offen"): „`stat(2)` blockiert an
   einer Röhre nicht". Die Probe `eine_roehre_haelt_die_frage_nach_dem_verweisziel_nicht_an`
   (`tests/verzeichnis.rs:1920-1946`) misst die Zusage zwar wirklich und läuft in eine
   benannte Zeitschranke; ihr Doc-Kommentar stellt ihr aber `File::open` als Gegenstück
   gegenüber, und das war nie die Alternative.

Damit bleibt für die Wahl kein Grund übrig, und ihr Preis sind die drei Fehlfälle oben plus
eine Nebenwirkung: `bestimmen` **öffnet** das Ziel. Bei einer Verknüpfung auf eine
Gerätedatei — eine serielle Schnittstelle unter `/dev/cu.*` etwa — hat schon das Öffnen eine
Wirkung am Gerät. Ein Doppelklick in einem Dateimanager soll das nicht auslösen.

## Die Regel steht schon einmal im Baum, mit dem richtigen Aufruf

`krk-ui/src/kommandos/pfadeingabe.rs:60` beantwortet dieselbe Frage — „führt dieser Pfad,
Verknüpfungen gefolgt, auf ein Verzeichnis?" — mit `std::fs::metadata`, und der Kommentar
darüber schreibt genau die Trennung aus, die auch `in_zeile_einsteigen` zieht:

```rust
// `metadata` folgt einer Verknuepfung. Das ist hier richtig: eine
// Verknuepfung auf einen Ordner ist als Ziel eines Sprungs derselbe Ordner.
```

Der Modulkopf jener Datei nennt sich „**Die eine Stelle, die einen Pfad prueft**" und warnt
ausdrücklich: „Ein zweiter Navigationsweg daneben waere die zweite Wahrheit darueber, was
KRK fuer einen gangbaren Pfad haelt, und die erste Abweichung zwischen beiden faende keine
Pruefung." Genau diese zweite Wahrheit ist mit `verweisziel` entstanden, und sie weicht
bereits ab: derselbe Socket ist über die Pfadeingabe eine Datei und über den Doppelklick
unerreichbar. Der Doc-Kommentar von `in_zeile_einsteigen` (`tabelle.rs:1412-1416`) beruft
sich auf `pfadeingabe::pruefen` als denselben Schnitt, während der Rumpf einen anderen
Mechanismus nimmt.

## Vorschlag

`bestimmen` fragt `std::fs::metadata(pfad)` statt `sys::ohne_warten_oeffnen`. Das ist ein
Systemaufruf statt dreien (`open`, `fcntl`×2, `fstat`, `close`), blockiert an keiner Röhre,
öffnet nichts und liefert die drei Fälle oben richtig. `Unerreichbar` bleibt und trägt dann,
was es benennt: ins Leere, im Ring, oder ohne Recht am Pfad. `krk-core` braucht dafür kein
`unsafe` und keine Hülle, und die Zusage „kein zusätzlicher Systemaufruf beim Lesen" bleibt
unberührt — sie hängt am Aufrufer und nicht an der Form des Aufrufs.

Zu klären ist dabei eine Nutzerfrage: soll `Verweisziel::Ordner` zusätzlich das Leserecht
prüfen? `pfadeingabe::pruefen:70-77` tut es (`read_dir`) und begründet es mit C2, „ein
Ordnerwechsel in eine leere Liste waere die wortlose Variante". Der heutige `open`-Weg tut
es als Nebenwirkung und meldet; ein Wechsel auf `stat` nähme das weg. Der Einstieg auf einen
gewöhnlichen `Typ::Ordner` ohne Leserecht ist heute schon wortlos, also stünde danach eine
Regel statt zweier.

## Abgrenzung

Der gemeldete Defekt `260814-1612` ist behoben: eine Verknüpfung auf einen erreichbaren
Ordner lässt sich betreten, und die Zusage „kein zusätzlicher Systemaufruf beim Lesen" hält
(nachgeprüft: `bestimmen` hat genau einen Rufer, `tabelle.rs:1426`, im Zweig
`Typ::Verknuepfung`, und kein Modul unter `verzeichnis/` ruft es). Dieser Datensatz betrifft
allein den Mechanismus der Auflösung.

## Ablage

Gemeinsamer Speicher. Betrifft den Kern und den Einstiegsweg der Oberfläche und nicht die
Directive einer Runde.

---
Resolved: `verweisziel::bestimmen` fragt `std::fs::metadata` statt
`sys::ohne_warten_oeffnen` (`crates/krk-core/src/verzeichnis/verweisziel.rs`). Ein
Systemaufruf statt fuenf, geoeffnet wird nichts. Am Referenzgeraet vorher und nachher
gemessen, uid 502, ueber `bestimmen` selbst: Unix-Socket `Unerreichbar {"Operation not
supported on socket (os error 102)"}` → `KeinOrdner`; Datei Modus `000` `Unerreichbar
{"Permission denied (os error 13)"}` → `KeinOrdner`; Verzeichnis Modus `0111` `Unerreichbar
{"Permission denied (os error 13)"}` → `Ordner`; benannte Roehre ohne Schreiber `KeinOrdner`
→ `KeinOrdner` (unveraendert richtig).

Der Modulkopf traegt jetzt die Unterscheidung, an der der Fehlschluss haengt: wer den
Deskriptor danach benutzt, oeffnet; wer nur fragt, was hinter dem Namen steht, fragt am
Namen. Die Zusicherung „drei Werte, ueberschneidungsfrei und vollstaendig" gilt nach dem
Wechsel auch fuer die Zustaende, die die Werte benennen, und sagt das jetzt ausdruecklich.
`Unerreichbar` heisst „der Name loest sich nicht auf", also ohne Recht **am Pfad**. Die
Roehrenprobe stellt nicht mehr `File::open` gegenueber, sondern ein blockierendes `open(2)`.
Zwei Proben sind dazugekommen: Verknuepfung auf eine Datei ohne Leserecht und auf ein
Verzeichnis mit Modus `0111`.

Die Nutzerfrage nach einer Leserechtspruefung in `Verweisziel::Ordner` ist mit „nein, nicht
in dieser Runde" beantwortet; die aeltere Ungleichheit zwischen Pfadsprung und Doppelklick
steht als eigener Datensatz
`shared/issues/260815-1749_o_der-pfadsprung-meldet-den-ordner-ohne-leserecht-und-der-doppelklick-schweigt.md`.
Vier Beschreibungsstellen ausserhalb der Auftragsgrenzen sind noch nachzuziehen:
`shared/issues/260815-1752_o_zwei-modulkoepfe-nennen-das-verweisziel-am-deskriptor-obwohl-es-am-pfad-fragt.md`.
