Nach dem Umzug beschreiben fünf Stellen noch die Lage unter `libtest`
---
Das Paket hat den Kommentar am `Namensfeld` in `tabelle.rs` nachgezogen („dieser Grund fällt; ob … eine Probe bekommen, ist nicht entschieden“). Fünf gleichartige Stellen nennen weiter `libtest` als heutige Lage oder verweisen auf die zurückgestellte Frage als offenen Ausweg. Dazu kommt ein Satz in `CLAUDE.md`, der nach dem Einschub nicht mehr trägt.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-1436-wie-kommen-die-appkit-proben-auf-den-hauptfaden.md, 260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md, 261101-1953-reviewer-appkit-proben-hauptfaden.md

Stand `0067f466`:

1. `crates/krk-ui/src/appkit/editor.rs:9366-9368`, Kopf der Proben des Umbauwegs: Der Kommentar nennt das Prüfziel ohne `libtest` als Ausweichform, die an der zurückgestellten Frage `260810-1044_*_ziehen-die-vier-instanzproben-in-ein-pruefziel-ohne-libtest-harness-um.md` hänge. Das Prüfziel steht seit `0a21d901`.
2. `crates/krk-ui/src/appkit/editor.rs:13244` und `:13366` in `messung_der_csv_tabelle_an_16_mib`: Der Textspeicher wird allein gemessen, weil „eine `NSTextView` … sich unter `libtest` nicht beschreiben“ lasse. Die Probe läuft jetzt im Prüfziel auf dem Hauptfaden. `let _ = mtm;` (Zeile 13365) ist ein Rest der aufgelösten Hülle.
3. `crates/krk-ui/src/appkit/editor.rs:9566` und `crates/krk-ui/src/appkit/quicknote.rs:621`: Die Quelltextproben gelten als Ersatz, weil sich die Fläche „unter `libtest` nicht fahren“ lasse. Das bleibt wahr, aber der Grund ist mit dem Prüfziel entfallen, wie am `Namensfeld`.
4. `crates/krk-ui/src/appkit/quicknote.rs:34-36` und `.claude/rules/coding-was-man-nicht-sieht.md`, Absatz „Die Quicknote ist offen genau dann …“: „ohne Schlüsselfenster meldet ein Rückgängig sich nicht, und das ist die Lage unter `libtest`“. Die AppKit-Proben laufen jetzt im Prüfziel `hauptfaden`, und auch dort wird kein Fenster zum Schlüsselfenster (`inference:`, kein Lauf belegt das). Der Satz nennt die falsche Umgebung.
5. `CLAUDE.md`, Absatz `**Jedes Mitglied des Workspace trägt …**`: Nach dem Einschub „je Zeile eine Kistenwurzel; `krk-ui` trägt zwei, …“ hängt „und gegen `members` der Wurzel-`Cargo.toml` zu halten“ grammatisch in der Luft. Der Abgleich mit `members` ist außerdem nicht mehr eins zu eins.

**Abnahme:**
- Die Stellen 1 bis 4 beschreiben den Stand seit dem Prüfziel `hauptfaden` so, wie es der Kommentar am `Namensfeld` (`appkit/tabelle.rs`) tut. Was eine Probe misst, bleibt stehen, und keine neue Probe entsteht nebenbei.
- `let _ = mtm;` fällt, oder ein Kommentar daneben sagt, warum er steht.
- Der Satz in `CLAUDE.md` ist ein ganzer Satz und sagt, wie die Zeilen gegen `members` zu halten sind, wenn `krk-ui` zwei Wurzeln trägt.
- `grep -rn "Pruefziel ohne \`libtest\`" crates/krk-ui/src` liefert keine Zeile mehr, die auf 260810-1044 als offenen Weg verweist. `make check` ist grün.
