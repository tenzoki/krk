Zwei Module des Kerns heißen `umbenennen`

---

Seit S17 gibt es `krk_core::umbenennen` (das Regelmodell für den Stapel) und
`krk_core::operation::umbenennen` (das einzelne Umbenennen im Dateisystem, aus
S15). Der Pfad zu S17 steht so in der Dateiliste des Plans und wurde deshalb
genommen; der Doppelname ist trotzdem einer.

---

Er kostet an jeder Fundstelle einen Blick auf den Modulpfad, um zu wissen, ob
gerade gerechnet oder umbenannt wird, und eine `use`-Zeile, die beide
hereinholt, muss eines von beiden umbenennen. Dieselbe Sorte Doppelname, die
der Plan an zwei anderen Stellen ausdrücklich vermieden hat: bei der Sitzung
in S12 und bei den Lesezeichen in S18 (`ablage/lesezeichen.rs` statt eines
zweiten `lesezeichen.rs`).

Auflösung, wenn sie gewollt ist: `krk_core::umbenennen` in
`krk_core::stapelumbenennen` umbenennen, oder unter
`krk_core::operation::stapel` einhängen. Betroffen sind
`crates/krk-core/src/lib.rs`, das Verzeichnis
`crates/krk-core/src/umbenennen/`, `crates/krk-core/tests/umbenennen.rs`, die
beiden Aufrufer in `crates/krk-ui/src/appkit/` und die Dateiliste des Plans.

Gefunden bei der Umsetzung von Schritt 17.
