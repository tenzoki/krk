Zwei Modulköpfe nennen `aufteilung::sichtbar_im` als Vorbild, und die Funktion gibt es nicht mehr

---

`crates/krk-ui/src/spalten.rs:12` und `crates/krk-ui/src/appkit/tabelle.rs:185` führen im
Präsens dasselbe Beispiel: „Dasselbe Muster tragen `aufteilung::sichtbar_im` und
`aufteilung::rahmenfarbe`". `aufteilung::rahmenfarbe` steht noch
(`crates/krk-ui/src/appkit/aufteilung.rs:414`), `aufteilung::sichtbar_im` nicht mehr — es ist
mit dem Commit `026c665` ersatzlos entfallen, als der Befund
`260812-0539_c_die-zuordnung-von-bereich-auf-sichtbarkeit-steht-seit-schritt-3-zweimal-gleich-da.md`
die doppelte Zuordnung beseitigte.

---

**Schwere:** niedrig (kein falsches Verhalten; zwei Kommentare zeigen auf eine Funktion, die
nicht mehr da ist)
**Gefunden:** reconciler, beim Abgleich der fünften Runde am 260812-0801
**Betroffen:** `crates/krk-ui/src/spalten.rs` (Modulkopf), `crates/krk-ui/src/appkit/tabelle.rs`
(Doc-Kommentar an `kennung`)
**Domain:** code

## Warum es der Übersetzer nicht gefunden hat

Beide Stellen nennen den Namen in einfachen Backticks und nicht als Doc-Verweis in eckigen
Klammern. Ein `[…]`-Verweis auf eine entfallene Funktion wäre ein `rustdoc::broken_intra_doc_links`
und liefe in `make check` unter `-D warnings` auf; ein Backtick-Name ist für den Übersetzer Text.
Dieselben beiden Sätze nennen `crate::fenstermodell::Bereich` und `crate::appkit::tabelle` sehr
wohl als eckige Verweise — die Trennung verläuft also innerhalb eines Satzes.

## Die dritte Stelle steht richtig

`crates/krk-ui/src/fenstermodell.rs:301` nennt `sichtbar_im` ebenfalls, aber im Rückblick:
„[`crate::appkit::aufteilung`] führte bis dahin eine zweite Fassung (`sichtbar_im`) …". Das ist
eine Aussage über einen vergangenen Zustand und bleibt richtig. Betroffen sind allein die beiden
Stellen im Präsens.

## Zwei Wege

1. **Das Beispiel auf `rahmenfarbe` zusammenziehen.** Beide Sätze nennen zwei Vorbilder, eines
   davon trägt noch. Der Satz bleibt gültig, wenn das entfallene wegfällt. Zwei Zeilen.
2. **Den Namen als eckigen Verweis schreiben.** Dann hält der Übersetzer den nächsten solchen
   Fall an. Das geht nur für Namen, die es gibt, also erst nach Weg 1 und nur für `rahmenfarbe`.

Weg 1 und Weg 2 schließen einander nicht aus; zusammen kosten sie dieselben zwei Zeilen und
lassen die nächste Entfernung auffliegen.
