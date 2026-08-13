# Wer setzt den ersten Tag `v0.1.0`, und wann?

---
**Domain:** code
**Status:** answered
**Filed by:** shaper (anticipated-circle mode)
**Cross-references:** `circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/_*_circle.md` (Directive, Antwort 2 der Klärungsrunde); `xtask/src/release.rs`; `shared/issues/260813-0026_*_bundle-und-release-schreiben-an-denselben-ort-und-ein-entwicklungsbau-zerstoert-das-beglaubigte-buendel.md`

---

## Question

Nach dieser Runde bricht `cargo xtask release` ab, solange HEAD keinen Tag trägt, der zur Version in der `Cargo.toml` passt. Der Baum trägt heute keinen einzigen Tag, bei sieben geschlossenen Runden, und das Werkzeug darf den Tag nach Antwort 2 der Klärungsrunde nicht selbst erzeugen. Damit ist der Auslieferungsweg vom Abschluss dieser Runde an abweisend, bis jemand von Hand `v0.1.0` setzt. Wer das tut und wann, entscheidet, ob die Runde einen benutzbaren Auslieferungsweg hinterlässt oder einen gesperrten.

## Options

1. **Der Nutzer setzt `v0.1.0` als Teil der Abnahme dieser Runde.** Der Tag steht auf dem Commit, der die Runde schließt.
   - Pro: der Auslieferungsweg ist nach der Runde sofort benutzbar, und die neue Prüfung lässt sich an ihm einmal fahren, statt sie ungeprüft zu hinterlassen.
   - Contra: der Tag sagt "ausgeliefert", und ausgeliefert wird an diesem Tag nichts. Er benennt einen Stand, der die Prüfung erfüllt, und nicht eine Auslieferung.
2. **Kein Tag jetzt; der erste entsteht bei der ersten echten Auslieferung.** Bis dahin bricht `release` ab, und das ist der zugesagte Zustand.
   - Pro: der Tag behält seine Bedeutung, nämlich eine Auslieferung zu benennen. Kein Tag ohne Anlass.
   - Contra: die neue Prüfung ist in dieser Runde nicht an einem grünen Fall gefahren, sondern nur an ihrem Abbruch. Wer nach der Runde ein Bündel für einen zweiten Mac braucht, stößt zuerst auf den Abbruch.
3. **Die sieben geschlossenen Runden bekommen rückwirkend Tags.** Erst danach der Tag auf HEAD.
   - Pro: die Historie trägt Marken, und `git describe` liefert von jedem Punkt aus eine Auskunft.
   - Contra: alle sieben Runden liefen auf derselben Version 0.1.0, es gäbe also sieben Tags für eine Zahl oder sieben erfundene Zahlen. Beides schreibt eine Auslieferungsgeschichte, die es nicht gab.

## Constraints

- Das Werkzeug erzeugt keinen Tag. Ausdrücklich verworfen in Antwort 2 der Klärungsrunde.
- Die Version bleibt in dieser Runde bei 0.1.0. Ein Anheben auf 1.0.0 ist ausgeschlossen.
- Ein Tag ist billig zu setzen und teuer zurückzunehmen, sobald er einmal weitergegeben ist.

## Recommendation

Möglichkeit 1, mit einer Einschränkung: der Tag gehört auf den Commit, der die Runde schließt, und der Abschnitt in `README.md` sagt dazu, dass `v0.1.0` den ersten getaggten Stand benennt und keine Weitergabe. Der Grund für die Empfehlung ist die Prüfung selbst. Eine Prüfung, die in ihrer eigenen Runde nur im Abbruch gesehen wurde, ist zur Hälfte abgenommen, und der grüne Fall braucht einen Tag. Möglichkeit 3 ist zu verwerfen: sieben Marken für eine Zahl.

---
Answered:
Implemented:
Deferred:
Superseded by:

---
Answered: circles/260813-0939-titelleiste-fuehrt-version-und-semantische-tags/history/260813-1006-orchestrator-session.md, Abschnitt "Drei Fragen beantwortet" — Antwort: Möglichkeit 1, der Nutzer setzt v0.1.0 auf den Commit, der die Runde schließt; README.md nennt ihn als ersten getaggten Stand ohne Weitergabe.
