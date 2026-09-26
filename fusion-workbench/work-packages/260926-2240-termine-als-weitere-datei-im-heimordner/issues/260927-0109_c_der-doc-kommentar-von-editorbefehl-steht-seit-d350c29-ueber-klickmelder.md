Der Doc-Kommentar von editorbefehl steht über klickmelder
---
In `crates/krk-ui/src/appkit/anwendung.rs` sind `klickmelder` und `termine_richtung_umkehren` zwischen den Doc-Kommentar von `editorbefehl` und dessen `fn` eingefügt worden. Der Doc-Kommentar von `klickmelder` beginnt deshalb mit „Fuehrt einen Editorbefehl aus, der genau eine Meldung liefert. **Die eine Stelle fuer die vier Befehle ohne Blatt** …“ und geht erst danach in „Der Melder fuer ein Kommando aus einem Klick …“ über; `fn editorbefehl` steht ohne Doc-Kommentar da. `cargo doc` meldet das nicht, weil beide Funktionen privat sind.

Mit dem Umzug fällt eine zweite, ältere Ungenauigkeit mit: der Absatz nennt „die vier Befehle“, und `editorbefehl` hat heute mehr Rufer (`grep -n 'self.editorbefehl(' crates/krk-ui/src/appkit/anwendung.rs`), seit dieser Arbeit auch `termine_richtung_umkehren`.
---
**Filed by:** reviewer, Kai Stalmann <kai@stalmann.org>
**Cross-references:** `260927-0109-reviewer-termine-als-weitere-datei-im-heimordner.md`

Abnahme: Der Doc-Kommentar „Fuehrt einen Editorbefehl aus …“ steht unmittelbar über `fn editorbefehl`, der von `klickmelder` beginnt mit „Der Melder fuer ein Kommando aus einem Klick“, und der Absatz nennt keine Zahl der Rufer mehr, sondern den `grep`, der sie erhebt (CLAUDE.md-Regel zu Zahlen in der Prosa).

---
Resolved: Der Doc-Kommentar „Fuehrt einen Editorbefehl aus …“ steht wieder unmittelbar über `fn editorbefehl` (`crates/krk-ui/src/appkit/anwendung.rs`), der von `klickmelder` beginnt mit „Der Melder fuer ein Kommando aus einem Klick“. Der Absatz nennt statt „der vier Befehle“ keine Zahl mehr, sondern den `grep -n 'self.editorbefehl(' crates/krk-ui/src/appkit/anwendung.rs`, der die Rufer erhebt. Commit folgt mit diesem Datensatz (docs(anwendung)).
