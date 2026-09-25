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
---
Resolved: `Abnahmemass::AnteilImBild` trägt sein Maß jetzt vollständig — Bildlänge,
geforderter Anteil und eine Obergrenze je Einzelwert in Bildlängen
(`crates/krk-bench/src/messen.rs:380-408`). Die Kistenkonstante
`ANTEIL_IM_BILD_PROZENT` ist damit weggefallen; L1 steht an seinen beiden
Fundstellen auf 95 Prozent ohne Obergrenze, L9 auf 85 Prozent mit `Some(2)`.
`Zusage::gehalten_in` prüft beide Hälften in derselben Runde und kommt weiterhin
ohne zweites Argument aus. Beide Berichte weisen die zweite Hälfte aus: die
Zahlentabelle trägt neben der Spalte "im Bild" die neue Spalte "hoechstwert" mit
dem größten Einzelwert aller Runden in Bildlängen, und der Abschnitt "Der Anteil
im naechsten Bild, Runde fuer Runde" nennt je Zusage eine Zeile "Anteil" und eine
Zeile "hoechstwert"; er steht seither einmal in `bericht::anteil_je_runde` statt
zweimal nebeneinander. Nachgewiesen an den vorliegenden Zahlen statt an einem
Abnahmelauf: der Test `l9_haelt_die_neue_fassung_in_allen_fuenf_gemessenen_runden`
fährt die 100 Einzelwerte aus `messungen/260805-2207-MacBookPro15-1-abnahme.txt`
(Zeilen 288 bis 313) durch die Auswertung und bestätigt 90, 85, 90, 100 und
85 Prozent bei Höchstwerten von 19,153 bis 23,429 ms, also gehalten in allen fünf
Runden; `dieselbe_reihe_verfehlt_das_ungesenkte_mass` hält dagegen, dass dieselbe
Reihe das alte Maß in vier von fünf Runden verfehlt, und
`eine_eingabe_ueber_zwei_bildlaengen_reisst_l9_trotz_gehaltenem_anteil` prüft den
erfundenen Gegenfall. `make check` grün.
