Der Kommentar an `PLATTENFRAGE` nennt die Pfadeingabe und „Ort wählen…“ und lässt Löschweg und Abwurf aus
---
Schwere: Niedrig. Veralteter Kommentar seit `5cdf39e3` und `63046f0d`.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/nebenfaden.rs:52` und `:53`: „die Pfadeingabe und „Ort wählen…“ (Plan …, Schritte 5 und 6)“. `PLATTENFRAGE` nehmen seitdem auch `anwendung.rs` `loeschbefund_erheben_lassen` und `tabelle.rs:5445` `schreibrecht_fragen_lassen`. Die Aufzählung wächst mit jedem neuen Weg.

**Abnahme.** Der Kommentar nennt keine Rufer mehr, sondern die Art der Frage, oder er nennt alle Rufer zusammen mit einem Befehl, der sie zählt (`grep -rn 'PLATTENFRAGE,' crates/krk-ui/src`).

---
Resolved: Der Kommentar an `PLATTENFRAGE` (`appkit/nebenfaden.rs`) nennt keine Rufer und keinen Planschritt mehr, sondern die Art der Frage: eine Frage, die ein Hauptfadenweg über `fragen_lassen` stellt, weil die Ereignisschleife nicht auf ihre Antwort warten darf.
