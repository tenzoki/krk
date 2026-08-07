Die Messstrecke kann die neue zweiteilige Fassung von L9 nicht abnehmen
---
L9 sagt seit dem Nutzerentscheid vom 260807 zwei Dinge zu: jede Eingabe erreicht
spätestens das zweite Bild, und mindestens 85 Prozent erreichen das erste. Die
Auswertung in `crates/krk-bench/src/messen.rs` bildet keine der beiden Hälften
ab. `Abnahmemass::AnteilImBild { bildlaenge }` (Zeile 387) trägt allein die
Bildlänge; der geforderte Anteil kommt aus der Kistenkonstanten
`ANTEIL_IM_BILD_PROZENT: usize = 95` (Zeile 67) und gilt damit für L1 und L9
gemeinsam. Eine Obergrenze je Einzelwert kennt der Typ gar nicht. Ein Lauf
nähme L9 heute weiter gegen 95 Prozent ab und wiese es als verfehlt aus,
obwohl die vom Nutzer bestätigte Zusage in allen fünf Runden der
Abnahmereihe hält.
---
Betroffen sind vier Stellen im selben Bau: die Konstante (`messen.rs:67`), die
Aufzählung `Abnahmemass` samt `beschreibung` (`messen.rs:378-404`), das Urteil
in `Zusage::gehalten_in` (`messen.rs:512-534`) und die Berichtsspalte "Abnahme
nach", die die Konstante ausschreibt (`messen.rs:400`, `bericht.rs:20` und
`bericht.rs:375`). Die Zusagenlisten setzen `AnteilImBild` dreimal:
für L1 in der Durchstichliste (`messen.rs:677`) und in der vollständigen Liste
(`:966`), für L9 in der vollständigen Liste (`:1020`).

Die Gestalt der Lösung entscheidet der `coder`; naheliegend ist, den geforderten
Anteil und die Obergrenze in die Variante zu ziehen, statt eine zweite Variante
neben `AnteilImBild` zu stellen. Der Kopfkommentar des Moduls (Zeilen 29 bis 39)
und der Berichtstext ab `messen.rs:1749` erklären beide Abnahmemaße im Fließtext
und ziehen mit.

Quelle der Zusage: Spec C8, Zeile L9 der Zusagentabelle und der Abschnitt
`Getroffene Festlegungen`, `planning/260802-1036_*_spec-navigator-geruest.md`.
Nutzerentscheid:
`decisions/260806-0014_*_l9-verfehlt-den-anteil-auch-auf-dem-ruhigen-geraet.md`.
Auswertungsregel im Plan: `### Frage 5`,
`planning/260802-1428_*_plan-navigator-geruest-runde-1.md`.
