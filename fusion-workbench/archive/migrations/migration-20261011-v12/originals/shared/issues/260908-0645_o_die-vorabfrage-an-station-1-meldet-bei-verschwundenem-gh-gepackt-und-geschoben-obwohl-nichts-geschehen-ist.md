Die Vorabfrage an Station 1 meldet bei verschwundenem `gh` „gepackt und geschoben", obwohl nichts geschehen ist

---

`release_frei_pruefen` (`xtask/src/veroeffentlichung.rs`) ist die zweite Vorabfrage von
Station 1 und ruft `release_steht`. Lässt sich `gh` dort nicht starten, macht `release_steht`
daraus `spaet_ohne_gh_meldung` — und die sagt: „Gepackt ist bereits, und geschoben ist
ebenfalls schon: HEAD und der Tag stehen auf der Gegenseite." An Station 1 ist nichts
gepackt, nichts geschoben und nichts eingereicht.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Domain:** code
**Schwere:** Niedrig — der Fall verlangt, dass `gh` zwischen `gh_pruefen` und
`release_frei_pruefen` verschwindet, also zwischen zwei unmittelbar aufeinanderfolgenden
Aufrufen. Die Meldung wäre dann aber das Gegenteil dessen, was auf der Platte steht.
**Baumstand:** `fe5fe9c` plus die Änderungen dieses Durchgangs
**Betroffen:** `xtask/src/veroeffentlichung.rs`, `release_steht` und `release_frei_pruefen`

## Befund

`release_steht` hat zwei Rufer mit verschiedenem Stand des Laufs:

| Rufer | Stand, wenn `gh` sich nicht starten lässt |
|---|---|
| `release_frei_pruefen` (Station 1) | nichts übersetzt, nichts gepackt, nichts geschoben |
| `releaseseite_anlegen` (Station 8) | gepackt und geschoben |

Beide bekommen dieselbe Meldung, und sie beschreibt den Stand der achten Station.

Das ist derselbe Schnitt, den dieses Modul an drei anderen Stellen schon zieht und dort mit
zwei Funktionen bezahlt: `vorab_ohne_gh_meldung` neben `spaet_ohne_gh_meldung`, und
`release_steht_vorab_meldung` neben `release_steht_meldung`. Die Doc-Kommentare beider Paare
begründen die Trennung ausdrücklich mit „der Stand des Laufs ist an den zwei Stellen
verschieden". `release_steht` selbst zieht sie nicht: es kennt seinen Rufer nicht.

## Warum es nicht in demselben Durchgang behoben ist

Gefunden beim Beheben von
`260821-2105_*_ein-angemeldetes-gh-das-das-vorhaben-nicht-erreicht-schiebt-erst-und-nennt-dann-die-falsche-abhilfe.md`,
das eine andere Frage stellt (eine Meldung, die zwei einander ausschließende Ursachen unter
einer Abhilfe führt). Ein Befund, eine Behebung.

## Abnahmebedingung

`release_steht` gibt an Station 1 eine Meldung, die den Stand von Station 1 nennt, und an
Station 8 die von Station 8 — auf demselben Weg wie die zwei vorhandenen Paare, also ohne
dass die Funktion ihren Rufer raten muss. Eine Probe hält beide Richtungen, wie
`jede_gh_meldung_nennt_den_stand_der_an_ihrer_stelle_gilt` es für das erste Paar tut.

**Herkunft:** gemeinsamer Speicher. Kein Circle ist aktiv, und der Befund betrifft die
Auslieferungskette des ganzen Projekts.
