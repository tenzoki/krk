# Reicht für `cargo xtask release` ein passender Tag auf HEAD, oder muss der Arbeitsbaum sauber sein?

---
**Domain:** code
**Status:** implemented
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_*_circle.md` (Antwort 2 der Klärungsrunde); `xtask/src/release.rs`; `shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-aber-nicht-was-sie-fuer-die-weitergabe-bedeutet.md`

---

## Question

Antwort 2 der Klärungsrunde sagt, `cargo xtask release` bricht ab, wenn HEAD keinen Tag trägt, der zur Version in der `Cargo.toml` passt. Ein Tag zeigt aber auf einen Commit und nicht auf den Arbeitsbaum. Wer eine Datei ändert und nicht einträgt, hat weiterhin einen passenden Tag auf HEAD, und `release` baut das Bündel aus dem geänderten Baum. Das ausgelieferte Bündel trägt dann eine Zahl, die einen Stand benennt, aus dem es nicht gebaut ist. Die Frage ist, ob die Prüfung das zulässt, denn genau das soll die Deckung der Zahl verhindern.

## Options

1. **Nur der Tag auf HEAD, wie Antwort 2 es wörtlich sagt.**
   - Pro: eine Abfrage, kein zweiter Fehlerfall, und die Prüfung hält niemanden auf, der zwischen zwei Läufen an einer Kleinigkeit arbeitet.
   - Contra: die Lücke oben bleibt offen, und sie ist die, wegen der die Kopplung von Anzeige und Tag überhaupt besteht.
2. **Tag auf HEAD und sauberer Arbeitsbaum.** `release` bricht auch ab, wenn `git status` etwas meldet.
   - Pro: das gebaute Bündel entspricht dem getaggten Stand, und die Zahl ist gedeckt. Wer ausliefert, liefert einen benannten Stand aus.
   - Contra: unbeachtete Dateien und der Zustand der Workbench würden mitzählen, wenn die Abfrage nicht genau gefasst ist. Dieses Verzeichnis führt in jeder Sitzung geänderte Dateien.
3. **Tag auf HEAD hart, sauberer Arbeitsbaum als Warnung.** Der Lauf geht weiter und sagt am Ende, dass der Baum vom getaggten Stand abweicht.
   - Pro: hält niemanden auf und benennt die Folge.
   - Contra: das Projekt hat am 260812 erlebt, was eine Meldung wert ist, die eine Folge nennt, ohne sie zu verhindern (`shared/issues/260812-1628_*_der-buendelbau-nennt-die-signaturidentitaet-…`). Eine Warnung am Ende eines Laufs, der Minuten dauert, kommt zudem nach der Beglaubigung.

## Constraints

- Die Prüfung berührt `cargo xtask bundle` und `make check` nicht. Ausdrücklich festgelegt in Antwort 2.
- `fusion-workbench/` trägt in jeder Sitzung geänderte und unbeachtete Dateien. Eine Abfrage auf einen sauberen Baum muss sagen, was sie zählt: eingetragene Änderungen, nicht eingetragene, unbeachtete.
- `xtask` ruft heute kein `git`. Was die Prüfung fragt, legt zugleich fest, wie tief das Bauwerkzeug in den Zustand des Arbeitsbaums schaut.

## Recommendation

Möglichkeit 2, beschränkt auf verfolgte Dateien. Unbeachtete Dateien bleiben außen vor, sonst hält die Prüfung an einem Bauergebnis oder einer Notiz an. Der Grund für die Strenge: eine Auslieferung ist selten, dauert Minuten und geht an andere Geräte, und der Preis eines Abbruchs vor dem ersten Übersetzungslauf ist ein `git stash` oder ein Commit. Der Preis der Lücke ist ein weitergegebenes Bündel, dessen Zahl den falschen Stand benennt.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md, Abschnitt "Drei Fragen beantwortet" — Antwort: Möglichkeit 2, beschränkt auf verfolgte Dateien; unbeachtete Dateien bleiben außen vor.

---
Implemented: f9e5137 — `stand_pruefen` (`xtask/src/release.rs:226`) vergleicht Version, Tags auf HEAD und geänderte verfolgte Dateien und liefert im Verletzungsfall eine Meldung, die beide Befunde nennt. Die Beschränkung auf verfolgte Dateien steht in der Konstanten `GIT_STAND` (`release.rs:127`): `git status --porcelain --untracked-files=no`, ohne Pfadfilter. Station 1 ruft sie als erste Zeile von `release::ausfuehren` (`release.rs:137`), vor `bundle::vorbereiten()` in `:139`. `cargo xtask bundle` und `make check` sind unangetastet, gehalten von der Probe `allein_release_fragt_nach_tag_und_arbeitsbaum` (`release.rs:1173`) und einem leeren Diff am `Makefile`. Abgeglichen am 260813-1345.
