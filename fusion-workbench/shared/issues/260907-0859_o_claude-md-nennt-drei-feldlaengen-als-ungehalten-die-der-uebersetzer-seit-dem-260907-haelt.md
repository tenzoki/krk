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
