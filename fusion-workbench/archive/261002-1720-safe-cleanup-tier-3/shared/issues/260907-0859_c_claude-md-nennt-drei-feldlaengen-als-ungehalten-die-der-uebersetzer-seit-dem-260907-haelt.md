CLAUDE.md nennt drei Feldlaengen als ungehalten, die der Uebersetzer seit dem 260907 haelt

---
Der Absatz „Ob eine `ALLE`-Liste neben ihrer Aufzählung vollständig bleibt,
entscheidet ihre Bauform" unter „Was man nicht sieht, wenn man es nicht weiß"
nennt vier Bauformen und sagt, drei davon entschieden die Vollstaendigkeit
nicht: ein Literal (`Aufteilung::rahmen`), ein `[0.0; 6]`
(`Aufteilung::gemessene_breiten`) und ein fester Parameter
(`Fenstermodell::breiten_uebernehmen`).

Alle drei tragen seit dem 260907 die Laenge `Bereich::ALLE.len()` statt einer
Zahl im Quelltext; dieselbe Umstellung ist an `bereichsbreiten`, `anteilig` und
`traegt_eine_ziehbewegung` (`crates/krk-ui/src/fenstermodell.rs`) gefahren. Ein
siebter Bereich haelt den Bau jetzt an `crates/krk-ui/src/appkit/aufteilung.rs`
an, gemessen mit einer Gegenprobe: `Bereich::ALLE` auf fuenf Werte gekuerzt,
`cargo build -p krk-ui` meldet „expected an array with a size of 5, found one
with a size of 6" und nennt die Stelle.

Der Absatz nennt daneben eine offene Nutzerfrage
(`260826-1811_*_wie-wird-die-vollstaendigkeit-einer-alle-liste-neben-einer-aufzaehlung-gehalten.md`);
sie ist am 260907 beantwortet und am selben Tag umgesetzt.

Akzeptanzprobe: der Absatz beschreibt die heutige Bauform der genannten
Stellen, oder er nennt sie nicht mehr.

---
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
Diese Sitzung aendert Code und keine normative Prosa; der Nachtrag in
`CLAUDE.md` gehoert an den Kurator.

---
Resolved: Der Absatz beschreibt die heutige Bauform. Er sagt jetzt, dass die
Vollständigkeit einer `ALLE`-Liste seit dem 260907 eine Probe hält und nicht mehr
die Bauform der einzelnen Stelle; die drei genannten Stellen stehen als das da,
was sie sind — `Aufteilung::rahmen`, `Aufteilung::gemessene_breiten` und
`Fenstermodell::breiten_uebernehmen` tragen alle drei die Länge
`Bereich::ALLE.len()`, und ein siebter Bereich hält den Bau dort an. Was sie
vorher waren, steht als Vergangenheitsform daneben, weil der Preis der Runde 23
die Begründung der heutigen Form ist.

Der Verweis auf die Nutzerfrage nennt sie nicht mehr offen: der Entscheid
`260826-1811_*_…` trägt `_i_`, und der Absatz nennt den Durchlauf
`jede_alle_liste_fuehrt_genau_die_varianten_ihrer_aufzaehlung`, seine zwei
Ausnahmen in `UNLESBARE_ALLE_LISTEN` und seine Grenze auf `crates/`.

Belegt am Baum vor der Änderung: `crates/krk-ui/src/appkit/aufteilung.rs:266` und
`:390`, `crates/krk-ui/src/fenstermodell.rs:1063`.
