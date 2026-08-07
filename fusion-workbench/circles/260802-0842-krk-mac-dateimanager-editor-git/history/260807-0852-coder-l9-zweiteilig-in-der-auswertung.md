# L9 zweiteilig: die Zusage trägt ihr Maß, der Bericht beide Hälften (Turn 26, R1b)

**Agent:** coder
**Status:** Complete
**Quelle:** `issues/260807-0832_c_die-messstrecke-kann-die-neue-zweiteilige-fassung-von-l9-nicht-abnehmen.md`
**Bindend:** Spec C8, Zeile L9 der Zusagentabelle und der Absatz `Die Vorschrift, prüfbar formuliert` in `planning/260802-1036_o_spec-navigator-geruest.md`

## Der Anlass in einem Satz

Der Nutzer hat L9 am 260807-0832 neu gefasst — mindestens 85 Prozent der Eingaben im ersten Bild, jede Eingabe spätestens im zweiten —, und die Auswertung kannte weder den abweichenden Anteil noch die Obergrenze.

## Was die Konstante verhindert hat

`ANTEIL_IM_BILD_PROZENT: usize = 95` war eine Kistenkonstante, und genau das war ab dem 260807 falsch: L1 und L9 teilten diesen Wert, und der Entscheid hat ihn auseinandergezogen. Eine zweite Konstante daneben hätte den Fehler verdoppelt statt behoben, denn die Zahl gehört nicht der Kiste, sondern der Zusage. Sie ist deshalb ersatzlos weg und in die Variante gewandert:

```rust
AnteilImBild {
    bildlaenge: Duration,
    mindestanteil_prozent: usize,
    obergrenze_bilder: Option<u32>,
}
```

Damit gilt für alle drei Angaben, was der Umbau vom 260803-1845 für die Bildlänge schon hergestellt hatte: eine Zusage trägt ihr Maß vollständig, und `Zusage::gehalten_in` kommt weiter ohne zweites Argument aus. `Option<u32>` statt einer Zahl mit verabredeter Nullbedeutung, weil „keine Obergrenze" ein eigener Zustand ist und kein besonders großer Wert; L1 steht auf `None`, L9 auf `Some(2)`.

Die Obergrenze steht in **Bildlängen** und nicht in Millisekunden, aus demselben Grund, aus dem die erste Hälfte den Anteil zählt: die Zusage lautet „spätestens das zweite Bild", und ein Bild ist der Kehrwert der Bildwiederholrate des Geräts. Eine Obergrenze in Millisekunden würde auf einem 120-Hz-Gerät etwas anderes bedeuten als die zugesagte.

## Das Urteil prüft beide Hälften in derselben Runde

`gehalten_in` verlangt für eine Runde jetzt beides: den Anteil im ersten Bild **und**, wo eine Obergrenze steht, dass kein Einzelwert sie reißt. Der ganzzahlige Vergleich bleibt, weil er die Grenzfälle trägt: 17 von 20 sind genau 85 Prozent und halten, ohne dass das Urteil an einer Rundung im letzten Bit hängt. Genau dieser Fall kommt in der vorliegenden Reihe zweimal vor.

## Der Bericht

Die Zahlentabelle führte je Zusage eine Spalte „im Bild" mit dem schlechtesten Anteil aller Runden. Daneben steht jetzt die Spalte **„hoechstwert"** mit dem größten Einzelwert aller Runden in Bildlängen — dieselbe Bauart wie „im Bild": eine Kennzahl über alle Runden, an der das Urteil hängt, weil gehalten in jeder Runde gehalten heißt. Die Spalte „Abnahme nach" nennt bei L9 seither beide Hälften (`>= 85 %, <= 2 Bilder`), bei L1 unverändert `>= 95 % im Bild`.

Keine zweite Tabelle daneben: der vorhandene Abschnitt „Der Anteil im naechsten Bild, Runde fuer Runde" trägt je Zusage nun zwei Zeilen, „Anteil" und „hoechstwert", Runde für Runde nebeneinander.

Beide Berichte — der Durchstich in `messen.rs` und die Abnahme in `bericht.rs` — brauchten diesen Abschnitt Wort für Wort gleich und hielten ihn bis dahin zweimal nebeneinander. Er steht jetzt einmal, als `bericht::anteil_je_runde`. Zwei Fassungen desselben Textes, die beide dieselbe Änderung brauchen, sind die Sorte Duplikat, die beim nächsten Mal auseinanderläuft.

## Der Nachweis, ohne die Strecke zu fahren

Die Messstrecke verlangt KRK im Vordergrund aus einem Terminalfenster; sie wurde nicht gestartet. Nachgewiesen ist an den Zahlen, die vorliegen: die 100 Einzelwerte der fünf L9-Runden aus `messungen/260805-2207-MacBookPro15-1-abnahme.txt` (Zeilen 288 bis 313) stehen wortgleich als Prüffall im Test und laufen durch dieselbe Auswertung, die ein Abnahmelauf benutzt.

| Runde | Anteil erstes Bild | größter Einzelwert | in Bildlängen |
|---|---|---|---|
| 1 | 90,0 % (18/20) | 19,153 ms | 1,15 |
| 2 | 85,0 % (17/20) | 20,913 ms | 1,25 |
| 3 | 90,0 % (18/20) | 23,429 ms | 1,41 |
| 4 | 100,0 % (20/20) | 15,674 ms | 0,94 |
| 5 | 85,0 % (17/20) | 18,825 ms | 1,13 |

Bildlänge 16,667 ms, zwei Bildlängen 33,333 ms. **Nach der neuen Fassung hält L9 in allen fünf Runden**, nach der alten in einer. Das deckt sich mit der Nachrechnung des `planner` in jeder Zeile.

## Geänderte Dateien

| Datei | Was |
|---|---|
| `crates/krk-bench/src/messen.rs` | `ANTEIL_IM_BILD_PROZENT` entfernt (war `:67`); `Abnahmemass::AnteilImBild` um `mindestanteil_prozent` und `obergrenze_bilder` erweitert (`:380-408`); `beschreibung` nennt beide Hälften (`:410-433`); `Zusage::hoechstwerte_in_bildern` und `hoechstwert_in_bildern` neu (`:546-570`); `gehalten_in` prüft beide Hälften (`:572-625`); Hilfsfunktion `in_bildern` (`:641`); die drei Zusagenlisten tragen ihr Maß (L1 zweimal, L9 einmal); Durchstichtabelle um die Spalte „hoechstwert"; der Anteilsabschnitt ruft jetzt `bericht::anteil_je_runde`; Modulkopf und `ZWEI_MASSE` nachgeführt; vier Proben neu, eine erweitert |
| `crates/krk-bench/src/bericht.rs` | `anteil_je_runde` neu (`:408`), von beiden Berichten benutzt; Abnahmetabelle um die Spalte „hoechstwert"; Vorspann und `GESAMT_LESART` nachgeführt; Probe um die beiden Maße und die neue Spalte ergänzt |
| `crates/krk-core/src/operation/mod.rs` | eine Zeile Doc: sie zitierte L9 noch in der Fassung „keine Eingabe wartet länger als 16 ms", die schon seit dem 260803-1810 überholt war |

## Proben

Vier neue in `crates/krk-bench/src/messen.rs`, eine erweiterte in `bericht.rs`:

- `l9_haelt_die_neue_fassung_in_allen_fuenf_gemessenen_runden` — die 100 gemessenen Werte, beide Hälften einzeln nachgeprüft, Urteil `gehalten in allen 5 Runden`.
- `dieselbe_reihe_verfehlt_das_ungesenkte_mass` — dieselben Werte gegen das Maß von L1: gehalten in 1 von 5. Das ist der Befund, aus dem der Nutzerentscheid entstand, und er ist jetzt festgehalten statt nur beschrieben.
- `eine_eingabe_ueber_zwei_bildlaengen_reisst_l9_trotz_gehaltenem_anteil` — der erfundene Gegenfall: 19 von 20 im ersten Bild, also weit über den geforderten 85 Prozent, ein Wert eine Nanosekunde jenseits des zweiten Bildes, Urteil verfehlt. Dazu die beiden Ränder: genau zwei Bildlängen halten noch („spätestens das zweite Bild", nicht „vor dem zweiten"), und vier verpasste erste Bilder sind 80 Prozent und reißen die erste Hälfte.
- `jede_zeile_nennt_ihr_abnahmemass` — um die L9-Zeile `>= 85 %, <= 2 Bilder` erweitert.

## Abnahme

`make check` grün: Bau, Proben (54 in `krk-bench`), Clippy mit `-D warnings`, `fmt --check`.

## Was daneben auffiel

**Der Spec und der gebaute Stand stimmen überein.** Der Absatz `Die Vorschrift, prüfbar formuliert` in C8 nennt 85 Prozent, höchstens drei verpasste von zwanzig und die Obergrenze von zwei Bildlängen (33,333 ms am Referenzgerät). Die Auswertung nimmt genau das ab. Kein Unterschied zu melden.

**Der Modulkopf von `krk-core/src/operation/` war seit dem 260803-1810 falsch.** Er zitierte L9 als „keine Eingabe wartet länger als 16 ms". Das ist keine Auswertungsstelle und lag außerhalb des Auftrags, aber es ist ein Satz über genau die Zusage, die hier umgebaut wurde, und stehenzulassen hätte geheißen, zwei Fassungen im Baum zu dulden. Geändert ist die eine Zeile Doc, keine Zeile Code.

**Die Verfehlung im Hauptfaden ist nicht behoben, nur nicht mehr verfehlt.** Der Entscheid hat die Zusage an die gemessene Wirklichkeit gebunden; die Ursache, aus der zehn von hundert Eingaben ihr erstes Bild verpassen, steht unverändert. Der Spec schreibt diesen Preis unter `Getroffene Festlegungen` aus.

**Kein Stilprofil für Langform geladen.** `fusion-rules coder` gab `stilwerk/chat-voice-de.yaml` aus, kein `default-voice-de.yaml`; dieser Bericht folgt dem Hausstil der übrigen Historie.
