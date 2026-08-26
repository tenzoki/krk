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
