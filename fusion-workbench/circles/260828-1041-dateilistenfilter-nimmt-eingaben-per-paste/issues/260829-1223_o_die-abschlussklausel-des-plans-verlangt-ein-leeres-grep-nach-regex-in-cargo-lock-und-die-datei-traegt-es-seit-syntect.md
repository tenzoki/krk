# Die Abschlussklausel des Plans verlangt ein leeres `grep` nach `regex` in `Cargo.lock`, und die Datei trägt es seit `syntect`

**Filed by:** reconciler, Kai Stalmann <kai@stalmann.org>
**Severity:** Low (Plan-Prosa, kein Codefehler)
**Executor:** Nutzer oder analyst (Wortlaut); kein Coder

`planning/260829-1102_*_plan-einfuegen-in-den-filter-und-stern-als-platzhalter.md`, `## Where this Circle stops`, fünfte Klausel: „`grep -rn 'regex' Cargo.lock` ist leer". Am Baum liefert das Kommando 12 Treffer (`Cargo.lock:112` `fancy-regex`, `:118` `regex-automata`, `:119` `regex-syntax`, `:304` `regex`), und zwar auf `c6c86cb` vor der Runde wie auf `8d64859` danach: die Kisten kommen über `syntect` herein (Runde 2). Die Absicht der Klausel — die Runde zieht keine Regex-Kiste für den Musterabgleich herein — hält, denn `git diff c6c86cb HEAD -- Cargo.lock Cargo.toml` ist leer, und `traegt_die_folge` (`crates/krk-core/src/verzeichnis/filter.rs:190`) arbeitet mit `str::find`. Wer die Klausel wörtlich nachfährt, findet sie verletzt. Nach der Ortsregel wird der Plan nicht rückwirkend geändert; der Datensatz hält fest, dass die Prüfung „Diff von `Cargo.lock` leer" die Klausel ersetzt und ein späterer Plan das Kommando nicht abschreiben soll.
