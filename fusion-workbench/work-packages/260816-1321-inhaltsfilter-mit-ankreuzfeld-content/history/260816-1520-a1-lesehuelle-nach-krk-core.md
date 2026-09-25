# A1: die begrenzte Lesehülle zieht nach `krk-core`

**Datum:** 2026-08-16
**Agent:** coder
**Status:** Complete
**Circle:** `circles/260816-1321-inhaltsfilter-mit-ankreuzfeld-content/`
**Plan:** `planning/260816-1359_o_plan-inhaltsfilter-der-dateiliste.md`, Schritt A1
**Baumstand vor der Arbeit:** `28bd78f`
**Erfüllt:** C6.5, C6.6 (Hälfte), C1.8 (Bauartteil)

## Was entstanden ist

**`krk-core/src/text/datei.rs`** trägt zwei neue öffentliche Stücke neben `lesen`
und `EDITORGRENZE`:

- `pub enum Lesehindernis { ZuGross, KeineDatei, Deskriptormangel, Fehler }` —
  vier Werte, ohne Auffangzweig. Überschneidungsfrei sind sie durch die
  Reihenfolge der Prüfungen: das Öffnen scheitert vor jeder Frage an den
  Deskriptor, der Typ steht vor der Größe, und was danach schiefgeht, ist ein
  Lesefehler. `Deskriptormangel` wird an dieser Stelle getrennt, weil allein sie
  den `io::Error` in der Hand hält; die Regel ist die vorhandene
  `verzeichnis::sys::ist_deskriptormangel`.
- `pub fn bis_zur_grenze_lesen(pfad: &Path, grenze: u64) -> Result<Vec<u8>, Lesehindernis>` —
  der Rumpf aus `vorschaumodell.rs` unverändert umgezogen: öffnen über
  `ohne_warten_oeffnen`, `metadata()` am Deskriptor, `take(grenze + 1)`, und die
  Prüfung danach, ob das eine Byte zuviel angekommen ist. Verzweigt wird
  gegenüber der alten Fassung nur da, wo aus einem `None` jetzt ein benannter
  Grund wird.

**`text::datei::lesen` ist nicht angefasst.** Der Doc-Kommentar der neuen Hülle
trägt den Abschnitt „Warum das nicht `lesen` ist": `lesen` gibt den offenen,
zurückgespulten Deskriptor zurück, den der Notizzettel braucht, die Hülle gibt
ihn nicht zurück. Der Modulkopf hat einen Absatz dazu bekommen, damit die zweite
Fassung nicht als Versehen des nächsten Lesers gilt.

**`krk-ui/src/vorschaumodell.rs`** verliert die private Fassung ersatzlos und
gewinnt `use krk_core::text::datei::bis_zur_grenze_lesen;`. `use std::io::Read;`
fällt mit ihr, weil sonst nichts in der Datei ihn braucht. Beide Aufrufer
(Bildzweig mit `BILDGRENZE`, Textzweig mit `TEXTGRENZE`) bilden jedes `Err` auf
ihre Metadatenanzeige ab — genau das vorige Verhalten. `TEXTGRENZE` und
`BILDGRENZE` bleiben, wo sie stehen.

**`krk-core/tests/text.rs`** bekommt vier Proben und eine Zeitschranken-Hülle
`bis_zur_grenze_mit_zeitschranke` in der Bauform der vorhandenen
`oeffnen_mit_zeitschranke`:

| Probe | prüft |
|---|---|
| `die_huelle_liefert_die_bytes_und_haelt_ihre_grenze` | darunter, genau darauf, ein Byte darüber (`ZuGross`) |
| `ein_ordner_ist_fuer_die_huelle_keine_datei` | `KeineDatei` am `fstat` des Deskriptors |
| `eine_benannte_roehre_ist_keine_datei_und_haelt_die_huelle_nicht_an` | `KeineDatei`, und die Antwort kommt überhaupt |
| `ein_fehler_beim_oeffnen_ist_kein_deskriptormangel` | fehlender Pfad und fehlendes Leserecht sind `Fehler` und nicht `Deskriptormangel` |

Die Proben stellen die Grenze über das **Argument** her und nicht über große
Dateien: acht Bytes gegen eine Grenze von acht sagen über die Regel dasselbe wie
1 MB gegen 1 MB und kosten keine Wartezeit. **Der Deskriptormangel wird hier
nicht geprüft**; er hängt an C3.6 und steht bei B1.

## Zwei Doku-Stellen, die der Umzug falsch gemacht hätte

- `krk-core/src/verzeichnis/sys.rs`, Abschnitt „Zwei Aufrufer": er nannte
  `vorschaumodell::bis_zur_grenze_lesen` in `krk-ui` und begründete den fehlenden
  Doku-Verweis damit, dass `krk-ui` an `krk-core` hängt. Beide Aufrufer liegen
  jetzt in `krk-core`, und der Grund ist damit weggefallen; die Stelle nennt sie
  als Verweise.
- `krk-ui/src/vorschaumodell.rs`, Modulkopf: der eine Weg von einem Pfad zu den
  Bytes zeigt jetzt nach `krk-core`, und ein Absatz sagt, dass die Hülle seit der
  Runde 11 dort wohnt und warum.

## Am Diff abzulesen

```sh
grep -rn 'take(grenze + 1)' crates --include='*.rs'
```

nennt genau eine Stelle, `crates/krk-core/src/text/datei.rs`.

## Abnahme

`make check` — exit 0, „alle vier gruen". Die Wettrennprobe
`ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an` ist im selben Lauf grün
(12,43 s, Notbremse bei 15) und nicht angefasst.

## Was neben diesem Schritt lief

Schritt C1 hat `crates/krk-core/src/verzeichnis/modell.rs`, `filter.rs` und
`crates/krk-core/tests/verzeichnis.rs` gleichzeitig geändert. Dieser Schritt hat
keine der drei angefasst, auch nicht für einen Doku-Verweis. Ein
Zwischenzustand jener Arbeit hat den Baum kurz rot gehalten
(`Ordnermodell::inhalt_wirkt` fehlte noch); der Abnahmelauf lief erst danach.
