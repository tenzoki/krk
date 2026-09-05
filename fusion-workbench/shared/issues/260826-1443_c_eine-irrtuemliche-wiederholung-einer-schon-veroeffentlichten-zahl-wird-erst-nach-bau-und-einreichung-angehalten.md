Eine irrtümliche Wiederholung einer schon veröffentlichten Zahl wird erst nach Bau und Einreichung angehalten
---
`./release.sh 1.2.0` bei stehendem und veröffentlichtem `v1.2.0` läuft durch Station 1, drei Übersetzungsläufe, Signierung und Einreichung, und hält erst an der Existenzfrage der Station 8.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/release.rs`, `xtask/src/veroeffentlichung.rs`

## Befund

Ablauf bei `./release.sh 1.2.0`, wenn `v1.2.0` auf HEAD steht und die Releaseseite existiert:

1. `cargo xtask version 1.2.0` → `Vorhaben::NichtsZuTun` (`version.rs:148-154`).
2. Station 1: Tag passt, Baum sauber, `gh` da (`release.rs:202`, `:210`) → grün.
3. Stationen 2 bis 7 laufen: zwei Übersetzungsläufe, `lipo`, Montage, Signierung mit Zeitstempel, Einreichung bei Apple mit `--wait`, Heften.
4. Station 8: `release_steht` (`veroeffentlichung.rs:608`, `:654-663`) → „Auf der Gegenseite steht bereits ein Release v1.2.0" (`:671-683`).

Die Existenzfrage ist ein `gh release view` und lässt den Baum, wie er ist. Der Grundsatz von Station 1 — „eine fehlende Voraussetzung soll auffallen, solange nichts geschehen ist" (`release.rs:204-209`) — träfe sie ebenso wie `gh_pruefen`.

## Was nicht behauptet wird

Kein Schaden: die Seite wird nicht überschrieben, der Push ist ein Leerlauf. Es ist der Preis einer Einreichung bei Apple und einer Viertelstunde, gegen einen Fehlgriff in der Zahl.

## Abhilfe — zu entscheiden

Die Existenzfrage neben `gh_pruefen` an Station 1 stellen, allein auf dem `release`-Weg (`Tagfrage::Erledigt`); der eigenständige Weg behält sie vor dem Anlegen. Kosten: ein zweiter `gh`-Aufruf an Station 1 und eine Probe.

**Schwere:** Low.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, L1

---
Resolved: Die Existenzfrage steht jetzt neben `gh_pruefen` an Station 1, allein auf dem
`release`-Weg. Umgesetzt wie in der Abhilfe beschrieben.

- `xtask/src/veroeffentlichung.rs`, neu: `release_frei_pruefen(wurzel, zahl)` — `pub(crate)`,
  baut den Tag über `tagname` und fragt `release_steht`. Sie liest allein und lässt den Baum, wie
  er ist; im grünen Fall meldet sie „Die Gegenseite fuehrt noch kein Release v<zahl>."
- `xtask/src/release.rs`, `ausfuehren`: der Aufruf steht unmittelbar hinter
  `veroeffentlichung::gh_pruefen()?`, also vor `bundle::vorbereiten`, vor der Identitätssuche und
  vor jedem Übersetzungslauf.
- Der eigenständige Weg (`cargo xtask veroeffentlichen`) behält seine Frage unverändert
  unmittelbar vor dem Anlegen. Eine zweite davor wäre derselbe `gh`-Aufruf zweimal in einem Lauf,
  der nichts dazwischen tut.

**Die Meldung ist eine zweite und keine geteilte**, aus demselben Grund, aus dem
`vorab_ohne_gh_meldung` und `spaet_ohne_gh_meldung` zwei sind: an Station 1 ist nichts übersetzt,
nichts signiert, nichts bei Apple eingereicht und nichts geschoben, an der achten liegt das Zip
und ist geschoben. `release_steht_vorab_meldung` sagt genau das und nennt die Abhilfe, eine neue
Versionszahl. Gehalten von der neuen Probe
`die_zwei_meldungen_zum_stehenden_release_nennen_je_ihren_stand`, die beide Richtungen prüft:
jede Meldung nennt ihren eigenen Stand nicht den der anderen, und beide tragen Lage und Abhilfe.

Nachgezogen sind die zwei Prosastellen in der Grenze dieses Durchgangs: der Modulkopf von
`release.rs` (Station 1 heisst jetzt „Tag, Arbeitsbaum, `gh` und die freie Zahl pruefen", mit
einem eigenen Absatz zur neuen Frage) und der Hilfetext in `xtask/src/main.rs`.

Die `README.md` nennt Station 1 weiter mit drei Bedingungen und lag ausserhalb der Grenze; der
Nachtrag ist als `260906-0008_o_readme-und-claude-md-fuehren-station-1-mit-drei-bedingungen-und-die-beglaubigung-mit-zwei-pruefungen.md`
abgelegt.

Geprüft: `cargo test -p xtask` (164 Proben, Exit 0), `cargo clippy -p xtask -p krk-bench
--all-targets -- -D warnings` (Exit 0), `cargo fmt -p xtask -p krk-bench -- --check` (Exit 0).
Baumstand `ba0c6bd`.
