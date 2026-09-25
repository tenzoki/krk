# Schritt 12: die Zählproben, C6 wird belegt statt behauptet

**Datum:** 260824-1902
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260823-2208-vorschau-zeigt-profil-zusammenfassung-statt-metadaten`
**Plan:** `planning/260824-0640_o_plan-vorschau-zeigt-profil-zusammenfassung-statt-metadaten.md`, Bündel E, Schritt 12
**Baumstand vorher:** Schritt 11 (`7de937f`) auf HEAD, Arbeitsbaum sauber bis auf die Werkbankdateien

---

## Auftrag

Die neun Kriterien aus C6 und die fensterlose Hälfte von C2.8 durch Proben belegen, die
**Aufrufe** zählen und keine Millisekunden. Der Zähler ist der `Haushalt`, den jeder Lauf
ohnehin führt; die Proben lesen ihn nach dem Lauf aus.

## Die eine Änderung am Produktionscode

`crates/krk-core/src/leseprofil/bausteine.rs:161` — die private `gezaehlt` heißt jetzt
`zusammenfassen_gezaehlt` und ist `pub`, mit `#[must_use]` am Paar. Der Doc-Kommentar der
privaten Fassung hatte diesen Schritt bereits angekündigt („Sie erreichen ihn erst, wenn
diese Fassung dafür geöffnet wird"); er ist jetzt darauf umgeschrieben und sagt, warum es
**kein zweiter Rechenweg** ist: `zusammenfassen` ist diese Funktion ohne die zweite Hälfte
ihres Paares und hat keinen eigenen Rumpf mehr.

`crates/krk-core/src/leseprofil/mod.rs:100` reicht beide weiter
(`pub use bausteine::{zusammenfassen, zusammenfassen_gezaehlt};`).

**Eine zweite Zählstelle ist ausdrücklich nicht entstanden.** Sie hätte gezählt, was die
Probe erwartet, und nicht, was der Lauf tut.

## Die Proben

Alle in `crates/krk-core/tests/leseprofil.rs`.

| Kriterium | Probe | Zeile |
|---|---|---|
| C6.1 | `ein_baustein_kostet_hoechstens_einen_leselauf_und_im_erkannten_ordner_keinen` | 1572 |
| C6.1, Trägheit | `ohne_einen_rufer_wird_der_erkannte_ordner_gar_nicht_gelesen` | 1709 |
| C6.2 | `die_zahl_der_oeffnungen_folgt_der_bausteinsorte` | 1758 |
| C6.3 | `eine_anzahl_ueber_der_grenze_wird_gekappt_und_nicht_abgewiesen` (stand schon) | 380 |
| C6.4, Leseläufe | `dreizehn_zaehlbausteine_erreichen_die_grenze_und_der_rest_traegt_den_platzhalter` | 1864 |
| C6.4, Öffnungen | `die_oeffnungen_gehen_ganz_oder_gar_nicht_und_enden_an_der_grenze` | 1940 |
| C6.5 | `eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet` (stand schon) | 1332 |
| C6.6 | `eine_datei_wird_bis_zur_grenze_gelesen_und_nicht_weiter` | 2013 |
| C6.7 | `die_zwei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen` | 2106 |
| C6.8 | der Haushalt selbst, ausgelesen von jeder Zeile dieser Tabelle | — |
| C6.9 | `eine_zusammenfassung_haelt_nie_mehr_als_einen_deskriptor_zugleich` + Kindprobe | 2354 / 2390 |
| C2.8, fensterlose Hälfte | `ein_boesartiges_muster_haelt_die_auswertung_nicht_an` | 2228 |

Dazu drei Helfer: `gezaehlt` (964, die eine Naht zum Haushalt), `runde` (1467),
`werkbankwurzel` (1503), `ausgelieferte` (1540).

### C6.3 und C6.5 haben keine neue Probe bekommen

Beide waren schon belegt, und eine zweite Probe hätte dieselbe Frage ein zweites Mal
gestellt. `eine_anzahl_ueber_der_grenze_wird_gekappt_und_nicht_abgewiesen` nimmt
`anzahl = 25 → 10` ab, `eine_abgeschnittene_lesung_sagt_nur_was_sie_entscheidet` legt
2.001 Einträge an und prüft `Wert::UeberGrenze(2000)`, also beide Hälften von C6.5: dass
bei 2.000 abgebrochen wird und was die Teillesung dann sagen darf. Der Modulkopf der
Datei nennt beide Stellen jetzt namentlich, damit sie beim nächsten Durchgang nicht als
Lücke erscheinen.

### C6.7, die gemessenen Zahlen

Gegen die **eingebettete** Auslieferungsfassung
(`krk_core::ablage::leseprofile::AUSLIEFERUNGSTEXT`, also `resources/default-readers.toml`)
und je einen Prüfordner in der Gestalt, die das Profil erwartet:

| Profil | Leseläufe | Öffnungen | Zusage |
|---|---|---|---|
| eine Runde (das größte) | **5** | **11** | C6.7: höchstens 7 und 11 |
| die Wurzel der Werkbank | **3** | **5** | C6.4: höchstens 12 und 24 |

Die Zahlen sind genau die, die der Plan unter `### Die fünf mitgelieferten Profile`
vorgerechnet hat. Die Probe prüft sie mit `assert_eq!` und nicht mit `<=`: eine Probe, die
allein „unter der Grenze" prüft, bliebe grün, wenn ein Profil von fünf auf sieben
Leseläufe steigt, und genau dieser Schritt wäre die Nachricht. Die Grenzen aus C6.7 und
C6.4 stehen als zweite Zusicherung daneben.

**Geprüft wird auch, welches Profil gegriffen hat.** Die Beschriftungen der
Zusammenfassung sind der Ausweis; ohne sie könnte ein Prüfordner, auf den ein anderes
Profil passt, dessen Zahlen unter der falschen Überschrift melden. Der Prüfordner `runde`
trägt deshalb **keine** `.fusion-setup`: das erste mitgelieferte Profil erkennt die
Werkbankwurzel daran und gewänne den zweiten Erkennungsdurchgang.

### C6.9, warum die Kindprobe

`cargo test` erbt die angehobene Deskriptorgrenze der Anmeldesitzung. Im selben Prozess
gemessen behauptete die Probe die Zusage: bei tausend freien Deskriptoren liefe auch eine
Auswertung durch, die zehn Dateien gleichzeitig offen hält. Die Form ist die der
Deskriptorproben aus der Runde 10 (`tests/verzeichnis.rs`, `tests/umfang.rs`), über
`gemeinsam::kind_mit_deskriptorgrenze` unter `ulimit -n 24`.

Das Kind stellt den Mangel **her** statt ihn abzuwarten: es nimmt Deskriptoren, bis keiner
mehr kommt, und gibt dann genau einen zurück. Wer zwei zugleich braucht, bekommt beim
zweiten `EMFILE`. Gemessen wird an den Werten und nicht an einem `Some`: ein Titel fällt
bei einem Lesefehler still auf den Dateinamen zurück, also lauten die Titel der
Verlaufsdateien „Verlauf n" und nicht wie ihre Dateien.

**Der Durchgang ohne einen einzigen freien Deskriptor ist die Gegenprobe.** Ohne ihn sähe
der zweite auch dann bestanden aus, wenn `ulimit` nicht gegriffen hätte. Daneben steht die
Zusicherung auf den Vorrat: nachgemessen ohne `ulimit` schlägt die Kindprobe mit „das Kind
bekommt 96 Deskriptoren; die Grenze 24 hat nicht gegriffen, und die Probe messte nichts"
fehl. Die Probe ist damit nachweislich nicht leer.

### C2.8, was belegt ist und was nicht

`(a+)+$` gegen vierzig `a` und ein `b`, an allen vier Stellen, an denen ein Muster aus der
`readers.toml` auf Text trifft: Pfadmuster auf dem vollen Pfad, Kennzeichendatei und
Eintragsmuster auf Namen, Feldmuster auf dem Inhalt. Eine fünfte gibt es nicht.

**Die Zusage ist, dass der Aufruf zurückkehrt.** Die Zeitschranke von zehn Sekunden steht
nur daneben, damit ein Fehlschlag als Fehlschlag erscheint und nicht als hängender
Testlauf; sie ist keine Leistungszusage und keine elfte Zahl neben den zehn aus C8 der
Runde 1. Die sichtbare Hälfte — die Zusammenfassung erscheint, das Fenster bleibt
bedienbar — steht unter `## Nutzerarbeit` des Plans.

## Zwei Stellen, an denen die Proben mehr sagen als das Kriterium

1. **`ohne_einen_rufer_wird_der_erkannte_ordner_gar_nicht_gelesen`** (1709) belegt die
   Trägheit des gemerkten Leselaufs. Trifft ein Pfadmuster, kostet die Erkennung nichts,
   und ein Profil, dessen Zeilen alle in Unterordnern arbeiten, liest den erkannten
   Ordner überhaupt nicht: zwei Bausteine, zwei Leseläufe. Fällt die Trägheit weg, bleibt
   die Tabelle zu C6.1 grün und diese Probe wird rot.
2. **Die letzte Zeile von `die_zahl_der_oeffnungen_folgt_der_bausteinsorte`** (1758) hält
   fest, dass zwei Feldbausteine auf **derselben** Datei zwei Öffnungen kosten. Das ist
   die bewusste Wahl gegen einen Zwischenspeicher; fällt später doch einer hinein, wird
   diese Zeile rot und nicht erst die Zusage aus C6.7.

## Was C6.4 wörtlich sagt und was die Probe misst

C6.4 sagt: „Erreicht ein Profil eine der beiden Grenzen, setzen die übrigen Bausteine
ihren Platzhalter." Der `Haushalt` führt dafür kein Sperrkennzeichen, sondern bucht je
Vorgang. Bei den Leseläufen fallen beide Lesarten zusammen, denn ein Lauf kostet immer
genau einen. Bei den Öffnungen unterscheiden sie sich in einem Fall: sind 20 von 24
verbraucht, scheitert ein Block von zehn, während ein einzelnes Feld danach noch
hineinpasst.

**Das ist kein Verstoß, sondern die Endbedingung enger gelesen:** „erreicht" heißt, dass
der Zähler auf der Grenze steht, und dann passt nichts mehr hinein. Die Probe
`die_oeffnungen_gehen_ganz_oder_gar_nicht_und_enden_an_der_grenze` legt den Verlauf
deshalb so, dass beide Aussagen einzeln dastehen: zwei Blöcke zu zehn (20 von 24), ein
dritter, der ganz ausfällt, vier Felder bis auf 24 genau, und ein fünftes, das den
Platzhalter trägt. Gemeldet wird das hier und nicht als Befund; ein Defektdatensatz stünde
für eine Abweichung, die es nicht gibt.

## Abnahme

```
make check   → exit 0
```

Kein Befund am Produktionscode. Keine Probe musste zurechtgebogen werden, und kein
Kriterium bleibt offen: die neun aus C6 sind belegt, C2.8 zur Hälfte, und die andere
Hälfte steht dort, wo sie hingehört.

## Was offen bleibt

Schritt 12 war der letzte offene Planschritt; alle vierzehn stehen auf `[DONE]`, und die
Statuszeile des Plans ist nachgezogen. Offen ist allein die Nutzerarbeit am laufenden
Bündel unter `## Nutzerarbeit`. **Die Statuszeile führte Schritt 14 noch als offen,
obwohl er seit dem 260824-1650 auf `[DONE]` steht**; das ist mitberichtigt und in der
Zeile selbst vermerkt.

Committet wurde nichts.
