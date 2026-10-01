Der Doc-Kommentar an `TRENNER` in `git/texte.rs` zeigt auf eine Konstante, die `operationen.rs` seit Schritt 8 nicht mehr trägt
---
`crates/krk-core/src/git/texte.rs`, Doc-Kommentar an `const TRENNER`, nennt als Vorbild „`krk-ui/src/kommandos/operationen.rs`, `TRENNER`“. Schritt 8 des Plans `261001-0850_*_plan-oberflaeche-folgt-der-systemsprache-deutsch-franzoesisch-englisch.md` hat diese Konstante aufgelöst: der Mittelpunkt steht seitdem in den zwei Tabelleneinträgen `VorgangWirdVorbereitet` und `VorgangZeile` (`crates/krk-core/src/sprache/tabelle/`), und die Begründung dazu am Doc-Kommentar von `operationen::abbruchhinweis`. Der Zeiger in `git/texte.rs` läuft damit ins Leere; die Datei gehört keinem Schritt dieses Plans, und der Ausführende von Schritt 8 durfte sie nicht anfassen.
---
**Filed by:** code-implementer, Kai Stalmann <kai@stalmann.org>

Evidenzpfad: `grep -n 'TRENNER' crates/krk-core/src/git/texte.rs crates/krk-ui/src/kommandos/operationen.rs` zeigt den Zeiger in `texte.rs` und keinen Treffer in `operationen.rs`.

Abnahme: der Doc-Kommentar an `TRENNER` in `git/texte.rs` nennt die Stelle, an der der Mittelpunkt der Vorgangszeile heute steht (die zwei Tabelleneinträge oder `operationen::abbruchhinweis`), und `grep -n 'operationen.rs`, `TRENNER`' crates/krk-core/src/git/texte.rs` ist leer. Wer Schritt 11 oder 12 ausführt und `git/texte.rs` ohnehin liest, zieht die Zeile mit; sonst ist es eine eigene Kleinaufgabe.
