Zwei Dokumentationsstände zum zweiten `./release.sh` nach einem Scheitern widersprechen sich, und der Code gibt dem selteneren recht
---
Fünf Stellen sagen, ein zweites `./release.sh <zahl>` nach einem Abbruch der Stationen bräche an Station 1 ab; zwei sagen, es fahre gleich weiter. `stand_pruefen` gibt den zweien recht.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Baumstand:** `c13bf1c`
**Betrifft:** `xtask/src/version.rs`, `xtask/src/beglaubigung.rs`, `xtask/src/main.rs`, `certify-only.sh`, `Makefile`, `README.md`, `CLAUDE.md`

## Befund

**Seite A — „fährt gleich weiter":** `version.rs:60-63` („ein zweiter Lauf desselben `./release.sh 0.2.0` faellt hier durch, ohne etwas zu tun, und faehrt gleich weiter zu `release`"), `README.md:243-252` („Der Handgriff ist derselbe — `./release.sh <version>` noch einmal. Der Lauf sieht, dass Zahl und Tag schon stehen, trägt nichts doppelt ein und fährt gleich weiter."), der Kommentar der Probe `version.rs:666-668`.

**Seite B — „bräche an Station 1 ab":** `certify-only.sh:22-24`, `Makefile:157-160`, `main.rs:130-132`, `README.md:304-310`, `CLAUDE.md` (Absatz „Seit dem 260820 steht daneben ein zweiter Weg"), alle mit der Begründung „weil der Tag nach dem Lauf nicht mehr allein auf HEAD steht".

**Was der Code tut.** `stand_pruefen` (`release.rs:307-314`) fragt zweierlei: steht ein passender Tag auf HEAD (`git tag --points-at HEAD`), ist der Baum sauber (`git status --porcelain --untracked-files=no`). Ein gescheiterter Lauf bewegt HEAD nicht und ändert keine verfolgte Datei; der Halbschritt fällt in `Vorhaben::NichtsZuTun` (`version.rs:148-154`, `:368-370`). Station 1 lässt den zweiten Lauf durch. Sie hält ihn nur an, wenn **zwischen** den Läufen etwas eingetragen oder geändert wurde — am 260820 die Werkbankdateien aus `260813-1515`. Genau das sagt `beglaubigung.rs:16-17` als einzige Stelle vollständig: „den Tag traegt HEAD nach dem Lauf nicht mehr allein, **und der Arbeitsbaum ist inzwischen ein anderer**".

„Nicht mehr allein auf HEAD" ist überdies keine Bedingung, die `stand_pruefen` kennt: mehrere Tags auf HEAD stören nicht (`release.rs:298-300`, Probe `:972-974`).

## Warum es zählt

Die Daseinsberechtigung von `certify-only.sh` ist richtig — kein zweiter Bau, keine zweite Signierung, keine zweite Einreichung —, aber an fünf Stellen falsch begründet. Wer die Begründung liest und nach einem Zeitüberlauf ohne zwischenzeitliche Änderung `./release.sh` fährt, wird nicht an Station 1 aufgehalten, sondern baut und reicht neu ein.

## Abhilfe

Die Begründung an den fünf Stellen auf den Aufwand stellen („übersetzt beide Ziele neu und reicht neu ein") und den Station-1-Satz als Bedingung formulieren („und bricht an Station 1 ab, wenn seither etwas eingetragen oder geändert ist"). `version.rs:60-63` und `README.md:243-252` bleiben so richtig.

**Schwere:** Medium — Dokumentation, aber die Dokumentation eines Wegs, der über das Gerät hinaus wirkt.
**Gefunden:** coderev, Durchsicht `shared/reviews/260826-1440-coderev-vollbaum-xtask-und-die-huellen.md`, M1
