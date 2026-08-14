# A2 — Die Sprungmarke fällt, die Zeichenregel bleibt

**Date:** 2026-08-15
**Agent:** coder
**Status:** Complete
**Plan:** `planning/260814-2102_o_plan-tippen-filtert-dateiliste-flach-und-tief.md`, Strang A, Schritt A2
**Spec:** `planning/260814-1830_o_spec-tippen-filtert-dateiliste-flach-und-tief.md`, C1.4, C1.5, C1.12
**Verification:** `make check` — exit 0

## Was gefallen ist

`crates/krk-core/src/verzeichnis/sprungmarke.rs` ist mit `git mv` zu `filter.rs`
geworden, und aus der Datei sind gefallen: der Typ `Sprungmarke` samt `neu`, `puffer`,
`zuruecksetzen` und `tippen`, die Konstante `PAUSE`, die Zeilensuche `erste_zeile_mit`
und der Import `std::time::{Duration, Instant}`. B1 hatte den Ivar der Ansicht schon
entfernt; hier fällt das Modul.

Der Sprungmarken-Abschnitt aus `crates/krk-core/tests/navigation.rs` ist mit ihm
gefallen, fünf Proben: die drei über Puffer und Pause und die zwei über die Zeilensuche.
Die Datei behält den Aufstieg und die Markierungsbefehle.

## Was geblieben ist, und wo es jetzt steht

`filter.rs` trägt zwei Regeln und keinen Zustand:

```text
Taste ohne Zusatztaste ──> traegt_ein_dateiname ──> Filtertext des Tabs
                                                           │
                       traegt_die_folge(Name, Filtertext) <┘
                             ^                    ^
                   modell::sichtbar        durchlauf
```

`traegt_ein_dateiname` ist Zeichen für Zeichen unverändert umgezogen und behält ihre
zwei Rufer (C1.4): `appkit/tabelle.rs`, die Senke des Tippens aus B1, und
`belegungsmodell.rs`, die Tippsuche der Belegungsansicht aus der Runde 7. Beide
`use`-Pfade sind nachgezogen — der Plan nennt nur den zweiten.

`traegt_die_folge` ist der Vergleich, den F1 in `durchlauf.rs` angelegt und mit einem
Doc-Kommentar auf A2 versehen hatte. Er steht jetzt hier, und beide Stellen rufen ihn:
`Ordnermodell::sichtbar` ersetzt sein eingesetztes `to_lowercase().contains`, der
Durchlauf verliert seine eigene Fassung.

## Warum das Modul die zwei Regeln zusammen trägt

Sie beantworten beide eine Frage über ein einzelnes Zeichen beziehungsweise einen
einzelnen Namen, und beide haben je zwei Rufer, die sich nicht kennen. Läge die
Zeichenregel in `filter.rs` und der Vergleich in `modell.rs`, hätte der Durchlauf einen
Grund, `modell` zu importieren, und die Abhängigkeitsrichtung des Verzeichnismoduls
liefe rückwärts.

Das Bild im Kopf von `verzeichnis/mod.rs` zeigt `filter` deshalb **unter** `modell` und
`durchlauf` und nicht in der Kette `sys → leser → eintrag → modell`.

## Wie C1.5 geprüft ist

`im_filter_steht_keine_zeitmessung` in `crates/krk-core/tests/verzeichnis.rs`. Sie liest
vier Dateien aus dem Quellbaum und sucht in deren **Code**-Zeilen drei Nadeln, jede
zusammengesetzt mit `concat!`:

| Nadel | wofür |
|---|---|
| `Instant` | ein monotoner Zeitpunkt |
| `Duration` | eine Spanne |
| `::now(` | das Ablesen einer beliebigen Uhr |

Die vier Dateien sind `verzeichnis/filter.rs`, `verzeichnis/modell.rs`,
`verzeichnis/durchlauf.rs` und `krk-ui/src/appkit/tabelle.rs`, also die drei Module, die
den Filter tragen, und die eine Senke, in die das getippte Zeichen läuft.

**`SystemTime` ist ausdrücklich keine Nadel**, und das steht im Doc-Kommentar der Probe:
ein `Eintrag` trägt seine Änderungszeit als `SystemTime`, und das Prüfmodul von
`modell.rs` baut damit seine Einträge. Eine Uhr liest man ab, ein Datum steht am
Eintrag. Der erste Lauf der Probe hat genau diese Stelle rot gemacht, und die Nadel ist
daraufhin auf `::now(` verengt worden, das den `UNIX_EPOCH`-Fixwert nicht findet und
`SystemTime::now()` sehr wohl.

**Was die Probe nicht entscheidet, steht in ihrem Kopf.** Der Weg eines getippten
Zeichens führt vorher durch `krk-ui/src/appkit/anwendung.rs`, und diese Datei führt eine
Uhr — für den Anzeigeverzug der Dateioperationen, der mit dem Filter nichts zu tun hat.
Sie steht deshalb nicht in der Liste, und damit deckt keine Nadel den ganzen Weg.
Gedeckt ist der Filter selbst.

## Wie C1.12 geprüft ist

`die_sprungmarke_steht_nirgends_mehr_im_baum`, im **ganzen** Baum und nicht in einer
Datei: eine stehengebliebene Aufrufstelle irgendwo wäre genau der Befund. Vier Nadeln
mit je null Fundstellen: `struct Sprungmarke`, `Sprungmarke::tippen`, `erste_zeile_mit`,
`PAUSE`.

`Nachschlag::Sprungmarke` behält seinen Namen und ist deshalb keine Nadel; keine der
vier findet ihn. Der Plan hatte das so entschieden: der Wert benennt „eine Taste ohne
Zusatztaste, die keiner Funktion gehört", und das trifft nach dieser Runde weiter zu.
Umzubenennen kostete `crates/krk-core/tests/belegung.rs` an fünf Stellen und änderte
nichts am Verhalten.

## Wie C1.4 geprüft ist

`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`. Sie
prüft, dass `filter.rs` beide Regeln als `pub fn` erklärt, und sammelt über den ganzen
Baum die Dateien ein, die eine der beiden nennen. Erwartet werden zwei Listen, jede mit
genau zwei Einträgen. Gezählt werden Dateien und keine Aufrufe: welche Datei fragt, ist
die Aussage des Kriteriums; wie oft sie innerhalb ihrer selbst fragt, ist es nicht.

## Prosa, die durch den Umzug falsch geworden ist

Der Modulname `krk_core::verzeichnis::sprungmarke` stand an mehreren Stellen in Prosa.
A2 nennt dafür zwei Modulköpfe. Der eine, `appkit/tabelle.rs`, war schon richtig — B1
hat ihn beim Umbau der Senke mitgezogen. Richtiggestellt sind stattdessen:

- `krk-core/src/tasten/belegung.rs` — Modulkopf und `Nachschlag::Sprungmarke`, die von
  A2 genannte Nachschlagart. Der Doc-Kommentar sagt jetzt ausdrücklich, dass der Name
  benennt, was der Wert aussagt, und nicht, wohin das Zeichen läuft.
- `krk-ui/src/appkit/ereignisse.rs` — sieben Stellen, darunter der Modulkopf, der die
  Zeichenregel in einem Modul verortete, das es nicht mehr gibt.
- `krk-ui/src/kommandos/zulaessigkeit.rs` — eine Zeile.
- `krk-ui/src/belegungsmodell.rs` — sechs Stellen, die die Sekundenregel der Sprungmarke
  als bestehend beschrieben, darunter der Doc-Kommentar der Zählprobe
  `die_suche_fuehrt_keinen_zeitgeber`, die genau über das Fehlen einer Uhr geht.

## Geänderte Dateien

| Datei | was |
|---|---|
| `crates/krk-core/src/verzeichnis/filter.rs` | aus `sprungmarke.rs` per `git mv`; Sprungmarke raus, Vergleich rein, sechs Proben |
| `crates/krk-core/src/verzeichnis/mod.rs` | `pub mod`, `pub use`, Bild und Prosa des Modulkopfs |
| `crates/krk-core/src/verzeichnis/modell.rs` | `sichtbar` ruft `traegt_die_folge`; ein Doc-Pfad |
| `crates/krk-core/src/verzeichnis/durchlauf.rs` | eigene Fassung des Vergleichs raus, `use` rein |
| `crates/krk-core/src/tasten/belegung.rs` | Prosa der Nachschlagart |
| `crates/krk-core/tests/navigation.rs` | Sprungmarken-Abschnitt raus, Modulkopf |
| `crates/krk-core/tests/verzeichnis.rs` | drei Zählproben, zwei Hilfsfunktionen |
| `crates/krk-ui/src/appkit/tabelle.rs` | `use`-Pfad und ein Doc-Link |
| `crates/krk-ui/src/appkit/ereignisse.rs` | Prosa |
| `crates/krk-ui/src/belegungsmodell.rs` | `use`-Pfad und Prosa |
| `crates/krk-ui/src/kommandos/zulaessigkeit.rs` | eine Zeile Prosa |

## Abweichungen und Befunde

- **Elf Dateien statt der vier genannten.** Datensatz:
  `issues/260814-2357_o_c2-nennt-zwei-dateien-…`, Nachtrag vom 260815 zu A2. Der Befund
  trägt damit sieben der vierzehn Schritte und eine vierte Ursache: eine Dateiliste
  nennt den Herkunftsort einer umgezogenen Funktion und nicht ihre Rufer.
- **Eine dritte wortgleiche Fassung des Vergleichs steht weiter im Baum**, in
  `Belegungsmodell::zeile_traegt` — genau der Stelle, die C1.3 als Maßstab nennt. Nicht
  mitgezogen, weil A2 sie nicht nennt und C1.4 die Runde-7-Seite unverändert lässt.
  Datensatz:
  `issues/260815-0230_o_belegungsmodell-zeile-traegt-fuehrt-denselben-vergleich-eingesetzt-und-ruft-die-eine-fassung-nicht.md`.
- **Die Zählproben stehen in `tests/verzeichnis.rs` und nicht in der genannten
  `tests/navigation.rs`.** Dort stehen die übrigen Filterproben dieser Runde, und dort
  führt `der_durchlauf_liest_ueber_den_schwungleser_und_setzt_keine_grenze` schon die
  Bauform, die sie brauchen; dessen eingesetzte Zeilenfilterung ist auf die neue
  Hilfsfunktion `code_zeilen` gezogen, statt eine zweite danebenzustellen.

## Nicht angefasst

Der Arbeitsbaum steht; committen ist Sache des Nutzers. `resources/default-keymap.toml`
nennt in zwei Kommentarzeilen „das Tippen der Anfangsbuchstaben aus C2" — die Datei
gehört dem `ontocoder` und ist deshalb hier nicht berührt.
