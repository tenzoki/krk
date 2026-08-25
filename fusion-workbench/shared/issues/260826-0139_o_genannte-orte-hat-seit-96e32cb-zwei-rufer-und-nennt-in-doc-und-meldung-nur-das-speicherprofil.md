# `genannte_orte` hat seit `96e32cb` zwei Rufer und nennt in Doc und Meldung nur das Speicherprofil

---
**Domain:** code
**Filed by:** coderev, Kai Stalmann <kai@qantr.com>
**Cross-references:** `crates/krk-core/tests/leseprofil.rs:2921-2950` (`genannte_orte`, Doc ab `:2921`, Rumpf ab `:2930`); `:3212` (der zweite Rufer, das Projektwurzelprofil); `shared/issues/260825-2233_c_die-beispielzahl-vier-des-projektwurzelprofils-haelt-keine-probe.md` (der Datensatz, dessen Behebung den zweiten Rufer gebracht hat)

---

## Was ist

`genannte_orte` sammelt die verschiedenen Orte, die die Zeilen eines Profils
nennen. Bis `96e32cb` hatte die Funktion genau einen Rufer, das Profil des
gemeinsamen Speichers, und ihr Doc-Kommentar und ihre eine Abbruchmeldung
sprechen entsprechend nur von diesem einen Profil:

- Doc-Kommentar (`:2924-2928`): „Die Zahl der **Unterspeicher** kommt aus der
  Profildatei … das **Speicherprofil** führt keinen [Platzhalter]".
- Abbruchmeldung (`:2941-2943`): „ein Ort des **Speicherprofils** trägt einen
  Platzhalter; die Rechnung ‚ein Ort, ein Leselauf' gilt für ihn nicht mehr".

Seit `96e32cb` ruft der vierte Fall der Probe
`die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` dieselbe
Funktion mit dem Profil `Projektwurzel mit fusion-Werkbank` (`:3212`). Bekäme
dieses Profil je eine Zeile mit Platzhalter im Ort, hielte die Probe an und
benennte das falsche Profil.

## Warum das zählt

Die Meldung ist die einzige Auskunft, die jemand bekommt, wenn die Rechnung
„ein Ort, ein Leselauf" nicht mehr gilt. Sie zeigt dann auf das
Speicherprofil, während die Ursache im Projektwurzelprofil liegt, und die
Suche beginnt an der falschen Stelle. Dieselbe Verwechslung hat dieses Vorhaben
schon einmal einen Fehlbefund gekostet
(`circles/…/issues/260810-1102_*_ein-befehl-waehrend-der-nachfrage-…`, dort über
`appkit/ereignisse.rs`).

Der Befund ist reine Prüfdatei und erreicht kein ausgeliefertes Byte.

Schwere **gering**.

## Möglichkeiten

1. Doc-Kommentar und Meldung auf „das Profil" verallgemeinern und den Namen des
   Profils in die Meldung hineinformatieren, etwa über einen zusätzlichen
   Parameter oder über `profil.name()`. Letzteres ist ohne neuen Parameter zu
   haben, denn die Funktion hält das Profil bereits.
2. Nichts ändern und die Meldung als Ungenauigkeit stehen lassen. Dann trägt
   die Probendatei eine Aussage, die für einen ihrer zwei Rufer falsch ist.

Möglichkeit 1, in der Form über `profil.name()`, kostet eine Zeile und macht die
Meldung an beiden Rufern richtig.
