Der Entscheid zur C-Freiheits-Zusage nennt fünf Prosastellen, im Baum stehen sechs
---
`shared/decisions/260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md:12`
sagt: „Die Zusage aus der Technologiewahl steht heute an fünf Stellen in derselben Form".
Die Antwortzeile desselben Datensatzes und `shared/history/260830-0950-orchestrator-session.md:96`
wiederholen die Zahl als „Fünf Prosastellen sind nachzuziehen". Keine der beiden Lesarten
liefert fünf.

**Wörtlich in der zitierten Form** („`Cargo.lock` führt kein `cc` und außer `windows-sys`
kein `-sys`-Paket") stehen im Quellbaum **zwei** Stellen: `Cargo.toml:274-275` und
`CLAUDE.md:87`.

**Die Zusage in irgendeiner Form** tragen im Quellbaum **sechs** Stellen, am 260830-1106
erhoben mit
`grep -rn "Namen auf \`-sys\`\|kein \`-sys\`-Paket\|\`-sys\`-Paket neben\|einen solchen Namen" --include='*.md' --include='*.toml' --include='*.rs' .`
über den Baum ohne `fusion-workbench/`:

1. `Cargo.toml:91-95` (Begründung zu `regex`): „findet im ganzen Baum weder `cc` noch `onig` noch einen Namen auf `-sys`. `windows-sys` steht wie zuvor allein in `Cargo.lock`"
2. `Cargo.toml:150-153` (Begründung zu `zip`): „`cargo tree -e normal,build` über den Prüf-Workspace findet keinen solchen Namen"
3. `Cargo.toml:274-275` (Begründung zu `objc2-pdf-kit`): die wörtliche Form
4. `Cargo.toml:352-356` (Begründung zu `syntect` und `two-face`): „findet im ganzen Baum weder `cc` noch `onig` noch einen Namen auf `-sys`"
5. `CLAUDE.md:87`: die wörtliche Form
6. `crates/krk-core/src/verzeichnis/sys.rs:66`: „eine Zeitkiste brächte auf macOS das erste `-sys`-Paket neben `windows-sys` herein"

Die sechste ist die, die keine Erhebung nach dem Wortlaut findet und die trotzdem falsch
wird: nach der Aufnahme von `gix` ist `linux-raw-sys` das erste `-sys`-Paket neben
`windows-sys` in `Cargo.lock`, und der Satz behauptet den Rang für eine Zeitkiste.

**Warum das bindet:** die Zahl steht in einem beantworteten Entscheidungsdatensatz, der den
Plan dieser Runde bindet. Ein Plan, der fünf Stellen nachzieht, lässt eine stehen, und
welche das ist, sagt weder der Datensatz noch die Antwortzeile.

**Abnahme:** die Zahl in `260830-1006_*_wie-lautet-die-c-freiheits-zusage-…` und in
`260830-0950-orchestrator-session.md` ist durch die Erhebungsvorschrift ersetzt (das
`grep` oben), oder die sechs Stellen sind namentlich aufgezählt. Beide Aufzeichnungen
behalten dabei ihren Stand nach der Ortsregel; berichtigt wird durch einen Nachtrag und
nicht durch Überschreiben.
---
**Filed by:** shaper, Kai Stalmann <kai@stalmann.org>
Gefunden beim Schneiden des Specs der Runde 23 (Circle `260830-1045-git-bereich-liest-status-branch-verlauf`),
beim Abgleich der vier beantworteten Entscheidungen gegen den Baum. Stand `d1fbaac`,
Arbeitsbaum ohne Änderung unterhalb von `crates/`.
