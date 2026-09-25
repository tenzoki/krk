Die Dateiliste von S18c nennt zwei nötige Dateien nicht

---

Die Umsetzung von Schritt 18c hat zwei Dateien angefasst, die seine Dateiliste
nicht führt:

- `crates/krk-ui/src/kommandos/fokus.rs` — die Prüfung
  `ein_befehl_mit_dem_bereich_dateifenster_wird_in_der_leiste_stumm_abgewiesen`
  steht seit S18 dort und trägt im Kopf den Satz "Ein Kommando gibt es fuer C11
  noch nicht; sobald es eines gibt, faellt es unter dieselbe Zeile". Genau das
  ist mit diesem Schritt eingetreten; die Prüfung nennt jetzt
  `Kommando::TerminalOeffnen` und heißt
  `der_terminal_befehl_wird_in_der_leiste_stumm_abgewiesen`. Ohne die Änderung
  bliebe im Programmtext ein Verweis auf einen Zustand stehen, den es nicht mehr
  gibt.
- `crates/krk-ui/src/kommandos/mod.rs` — der Modulkopf listet die fünf Module
  mit ihrer Zuständigkeit, und `operationen` bekam mit diesem Schritt eine
  zweite (C11 neben C4).

---

Beides sind kleine Änderungen, und beide folgen zwingend aus dem, was der
Schritt vorschreibt: der Plan legt `Kommando::TerminalOeffnen` fest und weist
die beiden Antworttexte ausdrücklich `kommandos/operationen.rs` zu.

Es ist derselbe Befund wie bei S7, S8, S12, S14, S16 und S17 — festgehalten in
`issues/260803-1819_c_dateilisten-von-s9-bis-s23-noch-nicht-unter-der-erweiterten-regel-durchgegangen.md`,
das den Durchgang für alle Schritte ab S9 zugesagt hat. Für S18c hat er die
beiden Dateien nicht erwischt. Der Nachtrag gehört in die Dateiliste des
Schritts.

---
Resolved: kommandos/fokus.rs und kommandos/mod.rs sind mit Begründung in die Dateiliste von S18c nachgetragen (Plannachzug 260806-1313).
