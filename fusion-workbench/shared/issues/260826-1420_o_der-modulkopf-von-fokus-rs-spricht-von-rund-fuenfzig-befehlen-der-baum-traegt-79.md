Der Modulkopf von `fokus.rs` spricht von „rund fünfzig Befehlen", der Baum trägt 79

---

`crates/krk-ui/src/kommandos/fokus.rs:34`: „sie muesste fuer jeden der rund fuenfzig Befehle entscheiden, wann sie zu laut wird". `awk '/^pub enum Kommando/,/^}/' crates/krk-core/src/tasten/belegung.rs` zählt am 260826 79 Varianten. Die Zahl ist seit der Runde 2 mit fast jeder Runde falscher geworden, aus demselben Grund, aus dem `CLAUDE.md` für `Kommando` keine Zahl mehr nennt.

---

**Filed by:** coderev, Kai Stalmann <kai@stalmann.org>

Die Aussage des Absatzes trägt ohne die Zahl: „für jeden Befehl". Vorschlag: die Zahl streichen, wie es `mod.rs:10-14` für die Modulzahl getan hat.

Schwere: niedrig.
