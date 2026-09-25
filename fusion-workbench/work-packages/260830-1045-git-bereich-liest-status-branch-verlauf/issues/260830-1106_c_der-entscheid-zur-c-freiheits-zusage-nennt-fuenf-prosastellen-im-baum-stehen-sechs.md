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

---
Nachtrag 260831 (Schritt 13 der Runde 23, `coder`): die Erhebungsvorschrift oben trifft
nicht, was sie treffen soll, und beides ist gemessen. Ihre Alternative `einen solchen
Namen` fängt zwei Stellen ein, die von der Zusage nicht handeln, nämlich
`crates/krk-ui/src/kommandos/kontextmenue.rs:719` und `:731`, wo es um Dateinamen geht.
Und seit der Aufnahme von `gix` in Schritt 3 trägt die Wurzel-`Cargo.toml` eine siebte
Stelle, die Begründung zu `gix` selbst; sie stand schon beim Schreiben in der
neugefassten Form und fehlt der Aufzählung oben, weil sie zum Zeitpunkt der Erhebung
nicht da war. Die Vorschrift ist deshalb neu gefasst und steht künftig an einer Stelle,
in `CLAUDE.md` beim Absatz zur C-Freiheits-Zusage:

```sh
grep -rn --exclude-dir=fusion-workbench --include='*.md' --include='*.toml' --include='*.rs' 'Namen auf `-sys`' .
```

Sie ist nur deshalb vollständig, weil jede Stelle die Wendung „Namen auf `-sys`" führt.
Ihre sieben Treffer am 260831: `Cargo.toml:93`, `:153`, `:279`, `:361`, `:515`,
`CLAUDE.md:87`, `crates/krk-core/src/verzeichnis/sys.rs:75`. Das sind die sechs oben
aufgezählten in ihrer neuen Zeilenlage, dazu die `gix`-Begründung.

---
Resolved: Die Hälfte im Quellbaum steht. Alle sechs aufgezählten Stellen tragen die
neugefasste Form aus E7, die siebte trug sie schon: auf den beiden Mac-Zielen kommt weder
`cc` noch ein Paket mit einem Namen auf `-sys` im Baum an, `Cargo.lock` führt daneben
`windows-sys` und `linux-raw-sys`, beide an fremden Zielen, und Prüfmittel ist
`cargo tree --target <ziel> -e normal,build` und nicht mehr ein `grep` über `Cargo.lock`.
Am 260831 gegen `x86_64-apple-darwin` und `aarch64-apple-darwin` nachgemessen: null
Treffer für `cc`, für `onig` und für jeden Namen auf `-sys`, bei 673 beziehungsweise 674
Baumzeilen; die Gegenprobe mit `--target all` findet `windows-sys 0.61.2` und
`linux-raw-sys 0.12.1` und kein `cc`, die Null ist also nicht die eines zu engen Musters.
`crates/krk-core/src/verzeichnis/sys.rs` sagt jetzt, dass der Rang „erstes `-sys`-Paket
neben `windows-sys`" mit dieser Runde an `linux-raw-sys` gefallen ist und dass die Frage
ohnehin am Bauziel entschieden wird. Keine der sieben Stellen nennt eine Zahl der
Prosastellen. `make check` grün, Exit 0.
**Der Marker bleibt `_o_`.** Der Abnahmetest dieses Datensatzes verlangt den Nachtrag in
`260830-1006_*_wie-lautet-die-c-freiheits-zusage-…` und in `260830-0950-orchestrator-session.md`,
und beide sind Schritt 15 der Runde 23 zugeteilt. Mit ihm schließt der Defekt.

---
Resolved (zweite Hälfte, Schritt 15 der Runde 23, `analyst`, 260831-1321): Die
Workbench-Hälfte steht. Beide Aufzeichnungen tragen den Nachtrag, und keine ist
überschrieben:
`260830-1006_*_wie-lautet-die-c-freiheits-zusage-wenn-linux-raw-sys-in-cargo-lock-steht.md`
hinter seiner `Answered:`-Zeile, `260830-0950-orchestrator-session.md` als eigener Abschnitt
am Ende. Beide nennen statt einer Zahl die Erhebungsvorschrift aus `CLAUDE.md`, ihre sieben
Treffer vom 260831-1321 gegen den Stand `9566973` (`Cargo.toml:93`, `:153`, `:279`, `:361`,
`:515`, `CLAUDE.md:89`, `crates/krk-core/src/verzeichnis/sys.rs:75`) und den Grund, aus dem
die alte Erhebung zu kurz griff: sie suchte den Wortlaut der alten Zusage, und die sechste
Stelle führte die Zusage ohne ihn. Die drei Vorkommen der Zahl im Entscheid und das eine in
der Sitzungsaufzeichnung bleiben nach der Ortsregel unangetastet. Die Vorschrift ist selbst
gefahren, nicht abgeschrieben; `CLAUDE.md:89` liegt zwei Zeilen tiefer als in der Erhebung
von Schritt 13, weil Schritt 14 die Rundentabelle erweitert hat. **Der Marker geht auf `_c_`.**
