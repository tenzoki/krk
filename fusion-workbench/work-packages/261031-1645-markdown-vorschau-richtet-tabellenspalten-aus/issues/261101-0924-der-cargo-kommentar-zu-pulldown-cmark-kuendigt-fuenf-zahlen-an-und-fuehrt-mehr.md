Der Kommentar über pulldown-cmark in der Wurzel-Cargo.toml kündigt vier plus eine Zahl an und führt danach weitere
---
Wurzel-`Cargo.toml`, Kommentarblock über `pulldown-cmark = { version = "0.13", default-features = false }`: „**Vier Zahlen, am 260812 … erhoben**, und eine fuenfte vom 261031 beim Merkmal fuer Tabellen unten.“ Unten folgen die fünfte Zahl (261031, 18,8 bis 19,2 ms über 1,05 MB Tabelle) und danach zwei weitere Messreihen vom 261101 (7,3 bis 7,5 ms für `tabellen-gross.md`, 2,4 bis 2,5 ms für 20 000 Zellen). Die Ankündigung nennt eine Zahl, die der Block nicht mehr trägt (`rules/critical-stance.md` §5). Die fünfte Zahl misst außerdem eine Datei mit 125 848 Zellen, die seit `75850c88` über `ZELLENGRENZE` liegt; der Weg, den sie misst, wird für diese Datei nicht mehr gefahren.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>
**Cross-references:** 261101-0924-reviewer-schlusspruefung-markdown-tabellen.md; 261101-0015-ab-welcher-groesse-zeigt-die-vorschau-eine-markdown-tabelle-nicht-mehr-als-eine-nstexttable.md

## Abnahme

Die Ankündigung zählt nicht mehr, oder sie zählt, was darunter steht. Die fünfte Zahl ist als Stand vor der Zellengrenze gekennzeichnet oder fällt zugunsten der Messung vom 261101. Nur Kommentar; `cargo build --workspace` bleibt grün.
