# Planung: Artefakt und Release

**Datum:** 2026-08-21
**Agent:** planner
**Status:** Complete
**Baumstand:** `77b84bb`
**Spec:** `shared/planning/260821-1115_o_spec-artefakt-und-release.md`

## Was entstanden ist

Ein Umsetzungsplan zur achten Station der Auslieferungskette, dem neuen Unterbefehl
`cargo xtask veroeffentlichen <zahl>`:
`shared/planning/260821-1221_o_plan-artefakt-und-release.md`. Elf Schritte in drei Bündeln
(Werkzeug, Dokumentation, Abnahme), alle dem `coder` zugewiesen; alle 40 Abnahmekriterien des
Specs sind genau einmal zugeordnet, gegengeprüft mit einem Abgleich der Schrittlisten gegen die
Zuordnungstabelle.

Weder `ontocoder` noch `analyst` bekommen einen Schritt. Der Umfang berührt keine Datendatei,
kein Manifest und keine Schemadatei; `Cargo.toml` bleibt unangetastet, weil `xtask` weiterhin
keine fremde Kiste führt. Ein strategisches Erzeugnis verlangt kein Schritt, weil die
Untersuchung vom 260820 und der Spec die Grundlage schon tragen.

## Am Baum gemessen

- Das angeheftete Beglaubigungsticket liegt als `Contents/CodeResources` im Bündel und beginnt
  mit den vier Bytes `s8ch`; die Datei trägt die Zeit des Heftungslaufs vom 260820 um 19:44,
  alles andere im Bündel die Bauzeit 11:35. Kein Aufruf unter `xtask/` schreibt sie. Damit ist
  die Ticketfrage aus C2.3 offline entscheidbar, ohne `xcrun stapler validate`, das im Versuch
  bei Apple nachgeladen hat.
- `gh` fehlt auf diesem Gerät (`command -v gh` findet nichts).
- 14 lokale Tags, einer auf `origin`, also 13 fehlende. Der Spec nennt an allen drei Stellen
  dieselbe Zahl 13; eine abweichende Angabe von 15 steht nicht in ihm.
- Sieben Prosastellen im Quellbaum sprechen von sieben Stationen, verteilt auf `README.md`,
  `xtask/src/version.rs`, `xtask/src/main.rs` und `xtask/src/release.rs`.

## Zwei Datensätze abgelegt

- `shared/decisions/260821-1221_o_ruft-xtask-ein-fremdes-werkzeug-ueber-den-suchpfad-wenn-kein-fester-pfad-richtig-ist.md`
  — `gh` ist das erste fremde Werkzeug dieses Baums, für das kein fester Pfad richtig sein kann.
  Drei Optionen mit Kosten, Empfehlung Suchpfad. Hält keinen Schritt auf.
- `shared/issues/260821-1221_o_das-abnahmekriterium-c6-3-enthaelt-die-zeichenfolge-die-es-verbietet.md`
  — C6.3 verlangt die Abwesenheit einer Zeichenfolge, die es selbst trägt. Der Plan begrenzt die
  Zusage auf den Quellbaum und schreibt den Grund aus.

## Was die Abnahme dem Nutzer überlässt

Dreizehn der 40 Kriterien. Sie verlangen ein installiertes und angemeldetes `gh`, einen echten
Lauf gegen GitHub oder einen zweiten Mac ohne Netzverbindung. Vor dem ersten Lauf steht dazu ein
einmaliger Handgriff, `git push origin --tags`. Die Runde schließt damit voraussichtlich
beschränkt, was in diesem Projekt der Regelfall ist.

## Nicht getan

Nichts implementiert, keinen Agenten losgeschickt. Der Nutzer entscheidet, wann der Plan
ausgeführt wird.
