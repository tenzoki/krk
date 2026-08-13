Eine vierte Wegwerfordner-Fassung steht in `xtask`, und die Probe liest diese Kiste nicht

---

`crates/krk-core/tests/baum.rs:114`, die Probe
`genau_drei_pruefordner_fassungen_stehen_im_baum`, sucht nach dem Gegenstand
statt nach dem Namen: eine Datei mit `impl Drop for`, `temp_dir()` und
`remove_dir_all` zugleich. Sie liest dafür `quelldateien()`
(`crates/krk-core/tests/gemeinsam/mod.rs:233`), und die sammelt
ausschliesslich unter `crates/`.

`xtask` ist das vierte Mitglied des Workspace und liegt nicht unter `crates/`.
Dort steht seit der Runde, die die Grenzprüfung baute, eine vierte Fassung:
`struct Wegwerfwurzel` in `xtask/src/release.rs:719`, mit `impl Drop`
(`:729`), `std::env::temp_dir()` (`:736`) und `fs::remove_dir_all` (`:731`).
Ihr eigener Doc-Kommentar sagt es: „Ein Wegwerf-Wurzelordner, wie ihn die Proben
des Kerns benutzen."

Die Probe kann sie nicht sehen, und die Zusage „genau drei Fassungen, eine je
Kiste" stimmt damit nur, wenn „Kiste" stillschweigend „Kiste unter `crates/`"
heisst. `CLAUDE.md` sagt es ohne diese Einschränkung, und der Spec dieser Runde
wiederholt es als Abnahmekriterium C6.8.

---

**Warum das gerade jetzt zählt**

Diese Runde baut die Tag-Prüfung in `xtask`. Braucht eine ihrer Proben ein
Wegwerf-Verzeichnis, ist die richtige Antwort die vorhandene `Wegwerfwurzel` und
keine fünfte Fassung. Wer die Zusage für bare Münze nimmt, sucht sie unter
`crates/`, findet sie nicht und schreibt eine neue.

Der Umsetzungsplan dieser Runde entschärft die Lage, ohne sie zu beheben: die
Vergleichsfunktion der Tag-Prüfung ist rein und wird gegen Zeichenketten
abgenommen, also braucht keine ihrer Proben ein Verzeichnis.

**Was zu tun ist**

Eine der beiden Fassungen wählen, nicht beide:

1. Die Zusage weiten und die Probe mit ihr. Dann heisst sie „genau vier
   Fassungen, eine je Kiste", `quelldateien()` liest `crates/` und `xtask/`, und
   die vierte steht in der Liste der anerkannten. Der Preis ist eine Änderung an
   `quelldateien()`, und die hat zwei Fassungen, die einander nachgezogen werden
   müssen (Modulkopf von `crates/krk-ui/src/quellbaum.rs`).
2. Die Zusage ausdrücklich auf `crates/` beschränken. Dann bleibt alles, wie es
   ist, und der Doc-Kommentar der Probe sagt, dass `xtask` ausserhalb ihres
   Blickfelds liegt und warum.

Nicht Gegenstand dieser Runde. Gefunden beim Bau ihres Umsetzungsplans
(`planning/260813-1110_o_plan-titelleiste-fuehrt-version-und-semantische-tags.md`,
Strang D).
