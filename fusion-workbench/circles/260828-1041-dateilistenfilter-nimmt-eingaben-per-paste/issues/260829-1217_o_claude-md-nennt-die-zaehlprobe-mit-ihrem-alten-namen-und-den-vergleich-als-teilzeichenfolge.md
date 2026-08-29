CLAUDE.md nennt die Zählprobe mit ihrem alten Namen und beschreibt den Vergleich als Teilzeichenfolge
---
`CLAUDE.md`, Absatz „Das Tippen im Dateifenster filtert seit der Runde 10“, trägt zwei Aussagen, die `415ef6f` überholt hat. Erstens den Probennamen `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`; die Probe heißt seit Schritt 9 der Runde 21 `die_zeichenregel_hat_drei_rufer_und_der_vergleich_drei` (`crates/krk-core/tests/verzeichnis.rs:3694`), weil `krk_core::zwischenablage::filtertext_aus` der dritte Rufer der Zeichenregel ist. Ein Leser, der den alten Namen greppt, findet nichts. Zweitens „Der Vergleich ist eine Teilzeichenfolge ohne Rücksicht auf Groß- und Kleinschreibung“; seit `f4ba58d` ist `traegt_die_folge` ein Musterabgleich mit `*` als einzigem Sonderzeichen, an beiden Enden ungebunden, und nimmt ein `Muster` statt eines `&str`. Der Absatz nennt daneben „drei Regeln“ und deren Rufer nicht als Zahl, was hält; dieser Datensatz verlangt keine Zahl.
---
**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>
**Severity:** Low
**Executor:** curator (CLAUDE.md-Tor)

Wirkt mit dem Abschluss der Runde 21, nicht vorher: bis dahin ist der Baum auf `main` noch der alte. Abnahme: `grep -c die_zeichenregel_hat_zwei_rufer CLAUDE.md` liefert 0, und der Absatz nennt den Platzhalter `*` mit Verweis auf den Modulkopf von `filter.rs` statt der Zahl der Rufer oder einer zweiten Beschreibung der Regel.
