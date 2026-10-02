# L9 fordert 65 statt 85 Prozent im ersten Bild

**Status:** Complete
**Agent:** coder
**Datum:** 260807-1920
**Auftrag:** dringende Einzelaufgabe, kein Circle aktiv
**Datensatz:** `shared/decisions/260807-1904_a_l9-verfehlt-auch-die-gesenkte-schwelle-wie-weiter.md`

## Was der Auftrag verlangte

Den Nutzerentscheid vom 260807-1900 umsetzen: L9 fordert künftig mindestens
65 Prozent der Eingaben im ersten Bild statt 85, bei unveränderter Obergrenze
von zwei Bildlängen je Einzelwert. L1 bleibt bei 95 Prozent ohne Obergrenze.

## Die Zahl

`crates/krk-bench/src/messen.rs:1148` — `mindestanteil_prozent` der Zusage L9
in `Gesamtlauf::fahren`, von 85 auf 65. `obergrenze_bilder` bleibt `Some(2)`.
Der Kommentar darüber nennt beide Senkungen desselben Tages, den tragenden
Datensatz und den weiterhin offenen Ursachendefekt
`shared/issues/260807-1748_o_l9-ist-seit-dem-260805-messbar-schlechter-geworden.md`.

## Die Proben

Die drei Prüfungen aus `d569f8a` und eine neue vierte:

- Die Reihe vom 260805-2207 (90, 85, 90, 100, 85 Prozent) bleibt als Probe
  stehen, umbenannt in `l9_haelt_die_reihe_vom_260805_in_allen_fuenf_runden`.
  Ihr Kommentar sagt jetzt, dass sie die Zusage mit zwanzig Punkten Spielraum
  trägt, statt sie zu begrenzen.
- **Neu:** `l9_haelt_die_reihe_vom_260807_erst_nach_der_zweiten_senkung` nimmt
  die hundert Einzelwerte aus `messungen/260807-1538-alle-zusagen.txt`,
  Zeilen 299 bis 323, wortgleich auf. Sie prüft beide Hälften (90, 75, 80, 65,
  70 Prozent; Höchstwerte 1,13 / 1,20 / 1,26 / 1,26 / 1,70 Bildlängen), hält
  in allen fünf Runden und verfehlt gegen die Fassung des Vormittags in vier.
  Das Paar aus alter und neuer Schwelle ist der Beleg für die Änderung.
- `dieselbe_reihe_verfehlt_das_ungesenkte_mass` bleibt unverändert: die Reihe
  vom 260805 gegen die 95 Prozent von L1, gehalten in 1 von 5 Runden.
- `eine_eingabe_ueber_zwei_bildlaengen_reisst_l9_trotz_gehaltenem_anteil`
  behält seine Obergrenzen-Prüfung unangetastet. Sein dritter Block prüfte die
  Schärfe der ersten Hälfte an vier verpassten Bildern (80 Prozent) und wäre
  gegen 65 grün geworden; er sitzt jetzt auf der neuen Grenze und prüft beide
  Seiten: sieben Fehlschläge sind 13 von 20 und halten genau, acht sind
  60 Prozent und verfehlen.

## Was sonst mitzog

Die Schwelle stand an fünf weiteren Stellen als Zahl oder als Prosa:

- `crates/krk-bench/src/bericht.rs:552` — der Fließtext im Berichtskopf nannte
  „L9 fordert 85 statt 95 Prozent". Jetzt 65, mit beiden Senkungen und dem
  Hinweis, dass die Ursache offen bleibt.
- `crates/krk-bench/src/bericht.rs:864` und `:909` — die Zusage und die
  erwartete Spaltenaufschrift im Berichtstest.
- `crates/krk-bench/src/messen.rs:379` — der Doku-Absatz an `Abnahmemass` trägt
  die zweite Senkung nach.
- `crates/krk-bench/src/messen.rs:402` — das Beispiel am Feld
  `mindestanteil_prozent` rechnete mit 17 von 20; jetzt 13 von 20, der
  Grenzfall, den Runde 4 des Laufs vom 260807 genau trifft.
- `crates/krk-core/src/operation/mod.rs:23` — der Modulkommentar formuliert die
  Zusage aus und nannte 85 Prozent.

## Weist der Bericht die neue Schwelle aus?

Ja, nach Fundstellen, nicht nach einem Lauf. Die Spalte „Abnahme nach" kommt in
`bericht.rs:359` aus `Abnahmemass::beschreibung()`, und die formatiert in
`messen.rs:434` direkt aus `mindestanteil_prozent`. Die Zeile für L9 liest
künftig `>= 65 %, <= 2 Bilder`. Das Urteil derselben Zeile hängt über
`gehalten_in` an demselben Feld. Der Berichtstest prüft die Aufschrift.
Ein Messlauf war nicht beauftragt und ist nicht gefahren worden.

## Nicht angefasst

`crates/krk-ui/`, Spec, Plan, Entscheidungsdatensätze, die neun übrigen
Zusagen. Kein `make bundle`, kein Messlauf. Nicht committet, wie beauftragt.

## Abnahme

`make check` grün: Bau, 55 Tests in `krk-bench` plus die übrigen Kisten,
`clippy -D warnings`, `fmt --check`.
