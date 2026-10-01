CLAUDE.md nennt acht Wortlautproben der Umlaut-Umstellung; der Beleg zählt vier plus drei plus eins plus eins
---
`CLAUDE.md` unter „Sprache“ sagt: „61 Zeichenketten in `krk-core` und `krk-ui` sind nachgezogen, und acht Proben halten den Wortlaut mit.“ Der Beleg, das Sitzungsprotokoll `260907-0826-umlaute-in-nutzersichtbarem-text.md`, schreibt „Acht Proben“ und zählt dann „vier in `crates/krk-core/tests/ablage.rs`, drei in `crates/krk-core/tests/operation.rs`, je eine in `crates/krk-ui/src/appkit/weitereinstanz.rs` und `crates/krk-ui/src/kommandos/operationen.rs`“, also neun. Das Protokoll ist eingefroren und bleibt stehen; `CLAUDE.md` übernimmt die Summe, die der eigene Beleg nicht trägt. Heute hält `grep -rn 'assert.*[äöüß]' crates` 76 Zeilen, die Zahl acht sagt über den Bestand ohnehin nichts mehr.
---
**Filed by:** requirements-designer, Kai Stalmann <kai@stalmann.org>
Gefunden bei der Bestandserhebung für den Spec zur Lokalisierung der Oberfläche, Stand `e984b3f`. Abnahme: `CLAUDE.md` nennt an der Stelle keine Zahl, sondern den Erhebungsbefehl, nach dem Muster der übrigen Zählstellen der Datei.
