Die längste Zeile gilt in `glaetten::umgebrochen` immer als umgebrochen: Strip schiebt Befehle in Kommentare und hängt getrennte Befehle aneinander, Format zieht Tabellen und Skizzen auf eine Zeile
---
Bei Text aus dem Chat ist die Einrückung kein zweites Signal, denn der Chat rückt jede Zeile nach der ersten ein. Übrig bleibt die Länge, und die längste Zeile erfüllt sie per Definition. Jede längste Zeile ab 40 Zeichen wird deshalb mit ihrer Folgezeile verbunden, sobald diese nicht leer ist. Plan, Modulkopf und `HowTo.md` nennen das einen seltenen Grenzfall („deren erster fast die volle Breite einnimmt“). Es ist der Regelfall jeder Eingabe, deren längste Zeile nicht die letzte ist.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** `261031-1034-plan-quicknote-sichert-sich-als-scratch-txt-und-traegt-strip-und-format.md` (E9, E10, E11), `261031-1156-reviewer-abschlussdurchsicht-quicknote-scratch-strip-format.md`

**Ort:** `crates/krk-core/src/text/glaetten.rs`, `fn umgebrochen` (Bedingung `laenge(zeile) + 1 + laenge(wort) > breite - breite / NACHLASS_TEILER`). Ist `laenge(zeile) == breite`, gilt sie für jedes Folgewort. Die Bedingung `text.len() == folgezeile.len()` (Folgezeile nicht eingerückt) greift bei Chat-Text nie, weil der Chat jede Zeile außer der ersten mit zwei Leerzeichen einrückt (Brief, Abschnitt „Beispiel des Nutzers“).

**Belege:** am Stand `be1a725d` gemessen, mit `glaetten.rs` unverändert per `#[path]` in ein Wegwerfprogramm eingebunden:

```
Strip, Eingabe:
# Erst die Abhaengigkeiten und dann die Proben des Kerns bauen
  cargo build
  cargo test
Strip, Ausgabe:
# Erst die Abhaengigkeiten und dann die Proben des Kerns bauen cargo build
cargo test
```

`cargo build` steht danach im Kommentar und läuft nicht. Damit ändert Strip die Bedeutung, und genau das soll Strip laut E11 nicht tun.

```
Strip: cargo test --workspace --all-targets -- --skip appkit::
         cargo clippy --workspace
    -> cargo test --workspace --all-targets -- --skip appkit:: cargo clippy --workspace
Strip: cd /Users/kai/Projects
         git log --oneline 135960bf..be1a725d -- crates/krk-ui/src
         git status
    -> cd /Users/kai/Projects
       git log --oneline 135960bf..be1a725d -- crates/krk-ui/src git status
```

Format zieht eine Tabelle in Kastenzeichen auf eine Zeile, ebenso eine Baumskizze (`├──`), denn jede ihrer Zeilen ist gleich lang und damit die längste (gekürzt dargestellt, gemessen an Zeilen von 51 Zeichen):

```
  ┌──────────┬──────────┐          ┌──────────┬──────────┐ │ Datei │ … │ ├──…┤ │ … │ └──…┘
  │ Datei    │ Zustand  │   ->
  ├──────────┼──────────┤
```

Ohne Auswahl wirkt Format auf den ganzen Text. Ein ganzer Chatabschnitt mit Tabelle oder Skizze geht damit verloren, bis `cmd+z` ihn zurückholt.

Die Proben in `crates/krk-core/tests/glaetten.rs` sehen das nicht: `strip_laesst_kurze_befehle_untereinander_stehen` stellt die längste Zeile ans Ende, und keine Probe trägt einen Kommentar vor einem Befehl, eine Tabelle oder eine Skizze.

**Zu klären, bevor gebaut wird:** Eine Nachbesserung, die nur die Längenschwelle verschiebt, behebt das nicht: die längste Zeile bleibt die längste. Gebraucht wird ein zweites Signal, das bei Chat-Text tatsächlich unterscheidet, oder für Strip eine Regel nach Shell-Lage. Zwei Ansätze kommen in Frage: Strip verbindet nicht über ein Zeilenende in einem Kommentar, und Zeilen in Rahmen- oder Kastenzeichen sind nie umgebrochen. Beide sind Sonderfälle, und wenn das das einzige Mittel ist, gehört die Frage nach `rules/critical-stance.md` §4 an den Planer zurück. Was auch immer gebaut wird, `HowTo.md` und der Modulkopf beschreiben danach die tatsächliche Bedingung und nicht „fast die volle Breite“.

**Abnahme:**
- Eine Probe in `crates/krk-core/tests/glaetten.rs` hält, dass Strip die Eingabe `"# <Kommentar ab 50 Zeichen>\n  cargo build"` nicht zu einer Zeile macht.
- Eine Probe hält zwei Befehle, deren erster die längste Zeile ist, für eine Eingabe, an der das neue Signal greift; welche Eingabe das ist, folgt aus der gewählten Regel.
- Eine Probe hält, dass Format eine Tabelle in Kastenzeichen und eine Baumskizze Zeile für Zeile erhält.
- `das_beispiel_des_nutzers_wird_zeichen_fuer_zeichen_zur_erwarteten_ausgabe` bleibt grün und unverändert.
- `HowTo.md` (Abschnitt „Strip und Format“) und der Modulkopf von `glaetten.rs` nennen die Fälle, die bleiben, so, wie sie auftreten.

**Resolved:** 261031, fixed in `b21ca032` (Breite erst mit zwei vollen Zeilen, kein Verbinden nach Kommentar, Geruestzeilen bleiben). Nachdurchsicht `261031-1226-reviewer-nachdurchsicht-quicknote-behebungen.md`: behoben, Restfaelle in `261031-1226-strip-verbindet-getrennte-befehle-aehnlicher-laenge-und-howto-verspricht-das-gegenteil.md`.
