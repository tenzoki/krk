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

---
Resolved: `krk_core::umbenennen` heißt jetzt `krk_core::stapelumbenennen`. Die erste der beiden vom Datensatz genannten Auflösungen.

**Nicht die zweite (`krk_core::operation::stapel`).** Der Modulkopf des Moduls begründet seit S17 ausdrücklich, warum es *neben* `operation::umbenennen` steht und nicht darin: `operation` fasst das Dateisystem an, dieses Modul rechnet auf Zeichenketten und ist deshalb ohne Prüfordner prüfbar. Es unter `operation` zu hängen, löste den Doppelnamen und nähme dafür eine Begründung zurück, die trägt. Umbenannt ist deshalb der Name, nicht der Ort.

Geändert, wie der Datensatz sie aufzählt:

- `crates/krk-core/src/lib.rs`, die `mod`-Zeile, alphabetisch einsortiert
- `crates/krk-core/src/umbenennen/` → `crates/krk-core/src/stapelumbenennen/` (`git mv`, alle vier Dateien)
- `crates/krk-core/tests/umbenennen.rs` → `crates/krk-core/tests/stapelumbenennen.rs` (`git mv`)
- die beiden Aufrufer in `crates/krk-ui/src/appkit/`: `anwendung.rs` und `blaetter/stapelumbenennen.rs`

Dazu zwei Stellen, die der Datensatz nicht nennt und die sonst still veraltet wären: der Modulkopf verwies auf `tests/umbenennen.rs`, und das Regelmodul verwies über `crate::umbenennen` auf sich selbst. Der Modulkopf trägt jetzt einen Absatz, der den alten Namen und den Grund für den neuen festhält.

**Der fünfte Punkt der Aufzählung, die Dateiliste des Plans, ist offen.** Dieser Durchgang war auf `crates/` begrenzt; die Plandatei durfte ich nicht anfassen. Sie nennt `crates/krk-core/src/umbenennen/…` in der Dateiliste von S17. Eigener Datensatz: `issues/260805-0947_o_die-dateiliste-von-s17-nennt-den-alten-modulpfad-umbenennen.md`.

**Der Name `umbenennen` kommt weiter vor, und das ist richtig so**: als `operation::umbenennen` (das Umbenennen im Dateisystem), als Funktion `umbenennen`, als Kommandokennung `umbenennen_stapel` und als Oberflächendatei `appkit/blaetter/stapelumbenennen.rs`. Doppelt vergeben ist keiner davon mehr; `grep -rn "krk_core::umbenennen\|crate::umbenennen" crates/` findet nichts.

Geprüft am 260805-0947: die vier Abnahmekommandos `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` und `cargo fmt --all --check` enden alle mit 0. `cargo test -p krk-core --test stapelumbenennen` meldet die sieben Prüfungen von S17 unverändert grün. Am laufenden Bündel ist das Stapel-Umbenennen im selben Zug wie der Datensatz `260804-2040_c_die-trennung-von-stamm-und-endung-steht-an-zwei-stellen.md` gegengeprüft; die Messung steht dort.
