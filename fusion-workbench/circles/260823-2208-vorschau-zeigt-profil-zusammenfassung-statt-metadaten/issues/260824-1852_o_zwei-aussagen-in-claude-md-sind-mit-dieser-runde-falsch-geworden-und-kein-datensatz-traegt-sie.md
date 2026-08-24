Zwei Aussagen in CLAUDE.md sind mit dieser Runde falsch geworden, und kein Datensatz trägt sie

---

`CLAUDE.md:146` sagt: „**Die Hülle hat zwei Aufrufer, und beide liegen seit der Runde 11 in
`krk-core/src/text/datei.rs`:** `lesen` ... und `bis_zur_grenze_lesen`". Seit Schritt 4 dieser
Runde sind es drei. `CLAUDE.md:120` zählt auf, was sich KRK unter
`~/Library/Application Support/KRK/` merkt, und nennt die Leseprofile nicht, obwohl `readers.toml`
seit Schritt 2 die siebte Ablagedatei ist.

---

**Gemessen am 260824-1852:** `grep -rn 'sys::ohne_warten_oeffnen(' crates/` liefert drei Treffer,
alle in `crates/krk-core/src/text/datei.rs` — `lesen` (`:434`), `bis_zur_grenze_lesen` (`:620`)
und `anlesen` (`:692`). Die Aufzählung in `:146` nennt zwei und schreibt den Prüfbefehl gleich
daneben, was den Fehler beim nächsten Nachzählen sichtbar macht, aber nicht abstellt.

**Der Plan hat beide Stellen vorhergesehen** und in seiner Tabelle `## Risks & Mitigations`
mit Zeilennummer festgehalten, mit der Bemerkung: „Beides ist Arbeit des Kurators und gehört
nicht in einen Schritt dieses Plans." Ein Eintrag in einer Risikotabelle ist aber kein Datensatz.
`rules/fusion-workbench-conventions.md` `## Issue and Decision Filing` schließt genau das aus:
ein Befund gehört in eine eigene Datei und nicht in ein Plandokument, weil er dort mit dem Plan
abgelegt wird. Dieser Datensatz holt ihn heraus; der Absatz im Plan bleibt stehen.

**Beide Aussagen sind vom Typ, den dieses Projekt am häufigsten falsch führt.** Der gemeinsame
Speicher trägt heute drei offene Datensätze über gezählte Aussagen in `CLAUDE.md`
(`shared/issues/260820-2056_*_claude-md-nennt-eine-zaehlprobe-…`,
`260823-1336_*_claude-md-nennt-einen-empfaenger-…`, `260823-1649_*_claude-md-sagt-die-version-…`);
die Datei selbst schreibt an mehreren Stellen aus, dass eine Zahl in ihr veraltet und deshalb
nicht dastehen soll.

**Abstellen:** Arbeit des Kurators, `/fusion:cleanup --only claude-md`. Die zwei Stellen sind
oben mit Zeilennummer genannt.

Gefunden beim Abgleich zum Abschluss der Runde 16, 260824-1852.
