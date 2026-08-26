Eine Probe in `bericht.rs` löscht einen festen Namen im echten Temporärverzeichnis

---

`ohne_messungenordner_wird_kein_bericht_geschrieben` (`crates/krk-bench/src/bericht.rs:923-929`)
baut sich ihren Pfad als `std::env::temp_dir().join("krk-bench-gibt-es-nicht")` und ruft darauf
`fs::remove_dir_all`. Fester Name, kein Prozesskennzeichen, keine Laufnummer, kein
`Wegwerfordner` — die einzige Probe dieser Kiste, die das noch tut.

---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig
**Betroffen:** `crates/krk-bench/src/bericht.rs`
**Cross-references:** `shared/issues/260810-1925_c_eine-probe-schreibt-ins-echte-temporaerverzeichnis-und-raeumt-dort-jetzt-fremde-messplaene-ab.md`, `shared/issues/260809-1106_c_die-probenordner-der-vorschau-tragen-feste-namen-im-temporaerverzeichnis.md`

## Der Maßstab dieser Kiste steht drei Dateien weiter

`crates/krk-bench/src/wegwerfordner.rs:39-47` ist die Fassung, die die Kiste dafür führt: Zweck,
Prozesskennung und Laufnummer im Namen, Abräumen in `Drop`. `CLAUDE.md` hält denselben Maßstab
unter „Was man nicht sieht": „Prüfordner einzelner Testläufe … tragen Prozesskennung und
Laufnummer und räumen sich in `Drop` selbst auf." Jede andere Probe in `messen.rs` und
`fixture.rs` geht über den `Wegwerfordner`; diese eine nicht.

## Was es praktisch kostet

Wenig, und das gehört zur ehrlichen Einschätzung: unter dem Namen liegt nie etwas, weil die Probe
ihn gerade **deshalb** wählt. Zwei gleichzeitige `cargo test`-Läufe dieser Kiste räumen
denselben nicht vorhandenen Pfad ab und stören einander nicht.

Der Grund, es trotzdem zu melden, ist der Namensraum und nicht der Schaden. Diese Kiste hat den
Befund „eine Probe greift mit festem Namen ins echte Temporärverzeichnis" bereits zweimal geführt
und beide Male behoben (`260809-1106`, `260810-1925`). Ein `fs::remove_dir_all` auf einen von
außen wählbaren, festen Pfad ist die Form, in der beide Male der Schaden entstanden ist; dass
hier zufällig nichts darunter liegt, ist eine Eigenschaft des heutigen Namens und keine des
Verfahrens.

## Denkbarer Weg

Einen `Wegwerfordner` nehmen und den Pfad **darin** wählen (der Wegwerfordner legt nichts an, das
passt hier genau), statt einen festen Namen im Temporärverzeichnis. Drei Zeilen, und die Kiste
hat danach genau zwei Stellen, die `std::env::temp_dir` nennen: `plan_schreiben` und
`Wegwerfordner::neu`.
