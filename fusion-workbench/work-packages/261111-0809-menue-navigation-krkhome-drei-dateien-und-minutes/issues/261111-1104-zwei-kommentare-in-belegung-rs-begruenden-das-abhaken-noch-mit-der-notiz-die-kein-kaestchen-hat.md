Zwei Kommentare in `belegung.rs` begründen das Abhaken noch mit der Notiz, die kein Kästchen hat
---
`crates/krk-core/src/tasten/belegung.rs` begründet an zwei Stellen, warum das Abhaken einen eigenen Wirkungsbereich trägt, mit der Notiztabelle, die am 261111 gefallen ist: der Doc-Kommentar von `Wirkungsbereich::Aufgaben` („Der Wert allein des Abhakens: eine Notiz hat kein Kaestchen.“) und der Kommentar über dem Zweig der Eintragsbefehle in `Kommando::wirkungsbereich` („das Abhaken allein in der Aufgabentabelle, denn eine Notiz hat kein Kaestchen“). Der Doc-Kommentar von `Kommando::AufgabeAbhaken` in derselben Datei ist in `36846a9b` schon berichtigt und nennt das Geheimnis und die Zeile einer CSV; die zwei Stellen daneben sind stehen geblieben.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

Gefunden in der Schlussdurchsicht des Bereichs `02c2f859..2cae597a` (Domäne Code). Schweregrad: niedrig, allein Kommentare; kein Verhalten hängt daran.

Beleg: `grep -n 'eine Notiz hat kein' crates/krk-core/src/tasten/belegung.rs` nennt die zwei Zeilen, eine im Rumpf von `pub enum Wirkungsbereich`, eine im Rumpf von `Kommando::wirkungsbereich`.

Abnahme: derselbe `grep` liefert nichts mehr, und beide Stellen nennen die Tabellen, die es gibt (ein Geheimnis und eine Zeile einer CSV haben kein Kästchen), wie der Doc-Kommentar von `Kommando::AufgabeAbhaken`. `make check` bleibt grün.

---
Resolved: `crates/krk-core/src/tasten/belegung.rs`, der Doc-Kommentar von `Wirkungsbereich::Aufgaben` und der Kommentar über dem Zweig der Eintragsbefehle in `Kommando::wirkungsbereich` begründen das Abhaken jetzt wie der Doc-Kommentar von `Kommando::AufgabeAbhaken`: ein Geheimnis und eine Zeile einer CSV haben kein Kästchen. `grep -n 'eine Notiz hat kein' crates/krk-core/src/tasten/belegung.rs` liefert nichts mehr; geändert sind allein Kommentarzeilen.
