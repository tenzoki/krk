# A2: `verzeichnis::inhalt` — die eine Antwort auf „trägt diese Datei die Folge"

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt A2
**Baumstand vor der Arbeit:** `4a54212`
**Erfüllt:** C1.4, C1.5, C1.6, C6.1, C6.3, C6.7, C6.8, C6.9

## Was entstanden ist

**`crates/krk-core/src/verzeichnis/inhalt.rs`** ist neu und trägt zwei
öffentliche Stücke:

- `pub enum Inhaltsbefund { Traegt, TraegtNicht, ZuGross, Unentschieden }` —
  vier Werte, ohne Auffangzweig, in Entsprechung zu den vier Werten von
  `Lesehindernis` aus A1.
- `#[must_use] pub fn traegt_der_inhalt(pfad: &Path, filter_klein: &str, grenze: u64) -> Inhaltsbefund`

Die Abbildung ist der ganze Rumpf und steht ohne Auffangzweig da: `Ok(bytes)`
geht durch `String::from_utf8`, gelingt es, entscheidet
`filter::traegt_die_folge`, misslingt es, ist die Datei kein Text und der Befund
`TraegtNicht`. `Err(ZuGross)` wird `ZuGross`, `Err(KeineDatei)` und
`Err(Fehler)` werden `TraegtNicht`, `Err(Deskriptormangel)` wird
`Unentschieden`. Auch der innere Zweig über `String::from_utf8` ist
ausgeschrieben und nicht als `Ok(_) | Err(_)` zusammengefasst.

Der Modulkopf trägt vier Abschnitte: warum die Datei **ganz** und nicht
streifenweise gelesen wird (die Typfrage müsste sonst je Streifen beantwortet
werden, und eine erst spät ungültige Datei hätte aus ihren ersten Streifen schon
Treffer gemeldet), warum **kein Abbruchkennzeichen** hier steht, warum die
Grenze als Argument reist, und wie die Abhängigkeit zwischen `text` und
`verzeichnis` läuft.

**`crates/krk-core/src/verzeichnis/mod.rs`** bekommt `pub mod inhalt;` und den
Wiederausfuhr `pub use inhalt::{Inhaltsbefund, traegt_der_inhalt};`. Das Bild im
Modulkopf zieht nach: aus neun Modulen werden zehn, `inhalt` steht unter
`filter`. Der Absatz zu `filter` nennt jetzt drei Regeln statt zweier und drei
Rufer des Vergleichs statt zweier; ein eigener Absatz beschreibt `inhalt`.

**`crates/krk-core/src/verzeichnis/modell.rs`** — eine Doku-Stelle, die der
Schritt falsch gemacht hätte: der Doc-Kommentar von `name_traegt_den_filter`
sagte „Der Vergleich hat damit weiter genau zwei Rufer, diese Datei und den
Durchlauf". Er nennt jetzt drei und behält seine eigentliche Aussage, dass in
`krk-ui` nichts nachgebaut wird. Dieselbe Bauart wie A1s Eingriff in `sys.rs`:
der Satz fällt mit der Änderung, die ihn falsch gemacht hat.

## Die Proben

**`crates/krk-core/tests/verzeichnis.rs`** bekommt eine eigene Abteilung mit
sieben Proben und einer Zeitschranken-Hülle `inhalt_mit_zeitschranke` in der
Bauform von A1s `bis_zur_grenze_mit_zeitschranke`:

| Probe | prüft |
|---|---|
| `ein_text_mit_der_folge_traegt_sie_und_einer_ohne_nicht` | `Traegt`, `TraegtNicht`, und die Schreibung zählt so wenig wie beim Namen |
| `eine_datei_ohne_gueltiges_utf8_traegt_nichts` | C1.6: die Folge steht als ASCII in der Datei, die Datei ist trotzdem kein Text |
| `eine_datei_ueber_der_grenze_bleibt_ungelesen` | `ZuGross` ist kein `TraegtNicht`; genau auf der Grenze wird gelesen |
| `die_folge_in_den_letzten_bytes_vor_der_grenze_wird_gefunden` | C1.5: 4096 Bytes, die Folge in den letzten neun |
| `was_keine_gewoehnliche_datei_ist_traegt_nichts` | benannte Röhre unter Zeitschranke und ein Ordner |
| `eine_datei_ohne_leserecht_traegt_nichts` | `EACCES` ist `TraegtNicht` und nicht `Unentschieden` |
| `der_name_und_der_inhalt_geben_dieselbe_antwort` | C6.9, sechs Gegenstände gegen acht Folgen |

**Die Nebeneinanderprobe misst über beide Wege und nicht über die geteilte
Regel.** Links steht `Ordnermodell::name_traegt_den_filter`, rechts
`traegt_der_inhalt` an einer Datei, deren ganzer Inhalt derselbe Text ist. Eine
Probe gegen `traegt_die_folge` selbst sagte nur, dass die Funktion sich verhält
wie sie selbst — und sie hätte den Namen der Regel in die Testdatei geschrieben,
womit die Zählprobe darunter einen vierten Rufer gefunden hätte. Die Reihe deckt
die drei Eigenschaften des Vergleichs ab: Stelle, Schreibung, keine Faltung
(`Cafe` gegen `café`, `Äpfel` gegen `apfel`).

## Die beiden nachgezogenen Zählproben

**`die_zeichenregel_und_der_vergleich_stehen_je_einmal_und_haben_je_zwei_rufer`
heißt jetzt `die_zeichenregel_hat_zwei_rufer_und_der_vergleich_drei`.** Die
Probe behält ihre namentliche Liste und ihre Meldung; ersetzt ist nur der Name,
der „zwei Rufer" für beide Regeln behauptete. Die Zeichenregel bleibt bei zwei
(C6.4), der Vergleich steigt auf drei.

**Die Zahl ist mit drei aufgelaufen, wie der Plan es vorsieht.** Ein vierter
Rufer wäre ein Rufer gewesen, den der Plan nicht vorsieht, und hätte den Schritt
angehalten statt die Zahl.

**`inhalt.rs` steht an zweiter und nicht an erster Stelle der Liste.** Der Plan
schreibt „mit `verzeichnis/inhalt.rs` an alphabetisch erster Stelle";
`quelldateien()` sortiert nach dem ganzen Pfad, und `durchlauf.rs` steht vor
`inhalt.rs`. Die Reihenfolge im Baum lautet `durchlauf.rs`, `inhalt.rs`,
`modell.rs`.

**`im_filter_steht_keine_zeitmessung` liest zwei Dateien mehr**, jetzt sieben:
`krk-core/src/verzeichnis/inhalt.rs` und `krk-core/src/text/datei.rs`. Beide sind
frei von `Instant`, `Duration` und `::now(`. Der Doc-Kommentar sagt jetzt auch,
warum `verzeichnis/sys.rs` der Liste **nicht** beitritt, und nennt den Defekt
`260816-1359_o_die-probe-gegen-zeitmessung-im-filter-erreicht-zwei-dateien-des-filterwegs-nicht.md`.

## Eine Abweichung vom Plantext, benannt

Der Plan sagt unter „Abzulesen an": „Proben im Prüfmodul von `inhalt.rs`, jede
an einem Prüfordner." **Beides zusammen geht in dieser Kiste nicht.** Der
Prüfordner von `krk-core` wohnt in `tests/gemeinsam/mod.rs`, und ein
`#[cfg(test)]`-Modul unter `src/` erreicht ihn nicht; eine vierte
Prüfordner-Fassung anzulegen verbietet CLAUDE.md ausdrücklich. Die Files-Liste
desselben Schritts nennt `crates/krk-core/tests/verzeichnis.rs`, und dort stehen
die Proben — dieselbe Auflösung, die A1 für `tests/text.rs` gewählt hat.
`inhalt.rs` hat deshalb **kein** Prüfmodul; ein leeres wäre schlechter als
keines.

## Am Diff abzulesen

```sh
grep -rl 'traegt_die_folge' crates --include='*.rs'
```

nennt genau vier Dateien: `filter.rs` als Heimat und die drei Rufer
`durchlauf.rs`, `inhalt.rs`, `modell.rs`.

## Abnahme

`make check` — exit 0, „alle vier gruen". Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist im selben Lauf grün
und nicht angefasst. `cargo fmt --all` hat eine Leerzeile am Dateiende von
`inhalt.rs` entfernt; ein erster Lauf war daran gescheitert.

## Was dieser Schritt nicht getan hat

- **Kein Rufer von `traegt_der_inhalt` im Baum.** Er entsteht in B1 beim
  Durchlauf, zusammen mit dem Abbruch und der Zählung der zu großen Dateien.
- **Der Deskriptormangel ist ungeprüft.** `Unentschieden` hat noch keine Probe;
  sie hängt an C3.6 und steht bei B1, wie schon bei A1.
- **`verzeichnis/durchlauf.rs` ist nicht angefasst**, auch nicht für einen
  Doku-Verweis. Der Absatz in `mod.rs`, der `inhalt` beschreibt, sagt deshalb
  „sein Rufer wird der Durchlauf" und nicht „ist".
