Die Standvergleiche der Einzugsmethoden sind nur als Zeichenketten bewacht
---
Schwere: Mittel. Die Entscheidungen, die eine späte Antwort verwerfen, stehen inline in den Einzugsmethoden, und ihre Proben suchen Wortlaut. 3.14.3 hat nur `make check` gesehen, also ruht die Aussage über diese Wege allein auf solchen Proben.
---
**Filed by:** reviewer, Kai Stalmann <ks@qantr.com>

**Beleg.** `crates/krk-ui/src/appkit/anwendung.rs:15152`, `eine_auswahl_wirkt_allein_gegen_den_stand_der_frage`, prüft `einziehen.contains("gefragt == selbst.leistenauswahlstand()")`. `:15383`, `f2_stellt_auf_dem_hauptfaden_keinen_systemaufruf`, verlangt unter anderem `"gefragt.nummer != jetzt.nummer"` und `"if gilt {"` (`:15434`, `:15435`). Beide würden ein `let _ = gefragt == selbst.leistenauswahlstand();` oder einen vertauschten Zweig durchlassen.

`notizordner_einziehen` (`:6729`) fällt eine Dreiwege-Entscheidung inline: Hindernis, überholte Nummer, gültiger Stand (`:6741`, `:6753`, `:6758`, `:6777`). Eine reine Funktion dafür gibt es nicht. `eine_doppelte_f2_verliert_keinen_satz` (`:15447`) prüft nur den `Vec` von `Notizordnerbefehle`, nicht den Weg dorthin. Dasselbe Muster tragen `geheimnisgroesse_einziehen` (`crates/krk-ui/src/appkit/editor.rs:5033`) und `loeschbefund_einziehen` (`anwendung.rs:8818`).

Dass es anders geht, zeigt `gegen_den_ort_der_frage` (`crates/krk-ui/src/appkit/tabelle.rs:777`), ebenso `Fragetakt` und `Schreibrechtmerker`: reine Werte mit Verhaltensproben.

**Überbehauptung.** Der Kommentar an `die_lesezeichengueltigkeit_stellt_auf_dem_hauptfaden_keinen_systemaufruf` (`anwendung.rs:15081`) sagt, die zwei Funktionen würden „allein aus den zwei Huellen gerufen“. Die Probe zählt keine Rufer.

**Betriebscode, den eine Probe geformt hat.** `tabelle.rs:3822` bis `:3826`: ein `#[allow(clippy::redundant_closure)]`, weil die Probe `die_zwischenablage_springt_nie_an_einen_serverpfad` den Aufruf mit der Klammer liest. Dort folgt der Code der Probe und nicht umgekehrt.

**Richtung.** Ich schlage reine Entscheidungsfunktionen vor, etwa einen `Notizordnerausgang` aus Stand der Frage, jetzigem Stand und Hindernis, und einen zweiten für die Leistenauswahl, je mit einer Tafelprobe. Die Rumpfproben dürfen danach nur noch die Abwesenheit von Systemaufrufen halten. Die Rufer zählt die Gültigkeitsprobe entweder wirklich, oder der Kommentar verliert den Satz.

**Abnahme.** Eine Mutation im Vergleich (`gilt` immer wahr, Nummernvergleich entfernt) macht eine Verhaltensprobe rot. Der Kommentar in `:15081` sagt nichts, was die Probe nicht prüft. Die Klammer in `tabelle.rs:3825` ist ohne `allow` zulässig oder die Probe liest die Form nicht mehr.
