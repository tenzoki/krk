# coder: dritte Behebungsschleife, Rundendatensätze mit Bezug auf krk-core

**Status:** Complete
**Agent:** coder
**Baumstand beim Beginn:** `2fa1d0e`
**Grenze:** ausschließlich `crates/krk-core/`

## Bestand

139 offene Defektdatensätze unter `fusion-workbench/circles/*/issues/`. Davon zitieren
21 ausschließlich `krk-core` und 25 daneben auch `krk-ui`. Bearbeitet: die 21.

## Geschlossen (12)

| Datensatz | Rundenverzeichnis |
|---|---|
| `260812-1529` UTF-8 in einer Ablagedatei | `260812-1000-teilen-…` |
| `260813-0644` aufgehobener Rest | `260813-0100-suche-…` |
| `260813-0717` Doc-Kommentar `atomar::schreiben` | `260813-0100-suche-…` |
| `260814-0913` „vier übrige Gründe" | `260813-2332-notizzettel-…` |
| `260816-1359` Reichweite der Zeitmessungsprobe | `260816-1321-inhaltsfilter-…` |
| `260824-1852` C3.14 ohne Nachweis | `260823-2208-vorschau-…` |
| `260824-1852` Meldung der Teillesungsprobe | `260823-2208-vorschau-…` |
| `260824-1852` Probe zu C5.10 | `260823-2208-vorschau-…` |
| `260824-1852` C5.8 und C5.9 ohne Probe | `260823-2208-vorschau-…` |
| `260827-1911` `erkennung.rs` und der Rückfallzweig | `260827-0310-vorschau-zählt-…` |
| `260829-1216` alleinstehender Wagenrücklauf | `260828-1041-dateilistenfilter-…` |
| `260831-0855` `NeedsUpdate` unerreichbar | `260830-1045-git-bereich-…` |

## Verhaltensändernd davon

- `Zugang::laden` liest in Bytes und wandelt selbst um; eine Ablagedatei mit ungültigem
  UTF-8 geht als `Grund::Beschaedigt` zur Seite statt als `Grund::NichtLesbar` verloren.
- `filtertext_aus` behandelt einen alleinstehenden `\r` mittendrin als Zeilenende.

## Neue Proben (4)

- `der_leseweg_der_leseprofile_oeffnet_keine_datei_ueber_ihren_pfad` (`tests/baum.rs`),
  negativ geprüft.
- `eine_ablagedatei_mit_ungueltigem_utf_8_geht_zur_seite` (`tests/ablage.rs`).
- `ohne_orchestrator_live_zeigt_allein_die_sitzungszeile_ihren_platzhalter` und
  `die_mitgelieferten_profile_greifen_ausserhalb_einer_werkbank_nicht`
  (`tests/leseprofil.rs`).

## Nicht geschlossen

Sechs Datensätze treffen einen eingefrorenen Spec- oder Plantext und gehören dem Nutzer;
drei weitere warten auf eine Nutzerfestlegung oder eine zweite Frage an die Platte. Die
Aufstellung steht im Bericht an den Auftraggeber.

## Verification

```
cargo test -p krk-core                                    exit 0
cargo clippy -p krk-core --all-targets -- -D warnings      exit 0
cargo fmt -p krk-core -- --check                           exit 0
RUSTDOCFLAGS="-D warnings" cargo doc -p krk-core --no-deps  exit 0
```

Ein Zwischenlauf ließ `ein_wechsel_der_art_unter_dem_oeffnen_haelt_nichts_an`
(`tests/text.rs`) an seiner Fünfzehn-Sekunden-Frist scheitern, während zwei weitere
Agenten dieselbe Maschine bauten; allein läuft die Probe in 3,7 s durch, und im
Abschlusslauf ist sie grün. Der Befund ist bereits erfasst als
`shared/issues/260823-1436_o_die-wettrennprobe-des-oeffnens-braucht-allein-neun-sekunden-von-fuenfzehn-und-faellt-unter-last.md`;
ein neuer Datensatz entsteht deshalb nicht.
