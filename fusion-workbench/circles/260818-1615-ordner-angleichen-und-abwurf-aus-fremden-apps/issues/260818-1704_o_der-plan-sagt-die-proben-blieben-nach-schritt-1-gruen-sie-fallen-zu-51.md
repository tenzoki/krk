Der Plan sagt, die Proben blieben nach Schritt 1 grün; sie fallen zu 51
---
Der Plan der Runde 13 schreibt unter Schritt 1 als Abnahme `cargo test --workspace`
und begründet den Zwischenstand ausdrücklich: „Der Zwischenstand ist gültig: eine
Funktion ohne `Kommando` ist ein bekannter Zustand des Modells, und
`belegungsausgabe.rs:755` zählt weiterhin 78 gegen 78." Gemessen nach dem Lauf von
Schritt 1 fallen 51 Proben.
---
**Gemessen am 260818 gegen den Baumstand `b47355e` plus Schritt 1 und 3.**

Der Lauf `cargo test --workspace` endet mit `635 passed; 51 failed`. Alle 51 fallen
in drei Modulen — `belegungsausgabe`, `belegungsmodell`, `menuemodell` — und alle auf
denselben Panic:

```
panicked at crates/krk-ui/src/belegungsmodell.rs:831:17:
die Funktion ordner_angleichen hat keinen Funktionsbereich;
die Zuordnung steht in belegungsmodell::bereich
```

**Der Plan hat die falsche Zählstelle geprüft.** Seine Begründung nennt
`belegungsausgabe.rs:755`, wo tatsächlich 78 gegen 78 stehen bleibt, weil dort
Kommandos gegen Kennungen gezählt werden und die neue Funktion keines von beidem
mitbringt. Die Stelle, die bricht, ist eine andere: `belegungsmodell.rs:831` verlangt
zu **jeder** Funktion der Belegungsdatei einen Funktionsbereich, und den setzt erst
`bereich_des_kommandos` in Schritt 2. Eine Funktion ohne `Kommando` ist damit gerade
kein bekannter Zustand des Modells, sondern ein Abbruch.

**Folgenlos in dieser Runde, und nur deshalb kein Schaden.** Der Plan legt die
Schritte 1 und 2 ohnehin in einen Commit („Die Schritte 1 und 2 sind ein Commit"),
also hat kein Commit je den roten Zwischenstand getragen. Die Aussage über den
Zwischenstand ist trotzdem falsch, und sie steht an der Stelle, an der ein späterer
Leser sie für gemessen hält.

**Warum das über diese Runde hinausgeht.** Der Plan sagt in seinem Kopf zu, Bau und
Proben stünden nach **jedem einzelnen** Schritt grün. Für Schritt 1 hält die Zusage
nicht. Wer den Plan als Muster für die nächste Runde nimmt, überträgt die Bauform
„eine Funktion vor ihrem Kommando eintragen" und bekommt denselben roten Stand,
diesmal womöglich ohne den rettenden gemeinsamen Commit.

Berichtigt gehört die Abnahmezeile von Schritt 1: entweder nennt sie den roten
Zwischenstand als erwartet und verweist auf den gemeinsamen Commit mit Schritt 2,
oder die beiden Schritte werden zu einem zusammengezogen.

**Verweise:**
- `circles/260818-1615-ordner-angleichen-und-abwurf-aus-fremden-apps/planning/260818-1633_*_plan-ordner-angleichen-und-abwurf-aus-fremden-apps.md`, Schritt 1 und 2
- `crates/krk-ui/src/belegungsmodell.rs:831` (die brechende Stelle)
- `crates/krk-ui/src/belegungsausgabe.rs:755` (die geprüfte, nicht brechende Stelle)
