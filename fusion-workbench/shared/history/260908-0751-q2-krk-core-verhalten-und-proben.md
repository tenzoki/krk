# Q2: die offenen Befunde, deren Behebung in `krk-core` landet

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>

## Was gefragt war

Die offenen Defektdatensätze abarbeiten, deren Behebung in `crates/krk-core/` landet und
nicht zugleich in `crates/krk-ui/`. Die Liste selbst erheben; keine Datei außerhalb von
`crates/krk-core/` anfassen; nicht committen.

## Die Erhebung

```sh
find fusion-workbench/shared/issues fusion-workbench/circles/*/issues \
     -maxdepth 1 -name '*_o_*.md'
```

189 offene Datensätze insgesamt. Geschnitten wurde je Datensatz danach, ob der Pfad
`crates/krk-core/` darin vorkommt und `crates/krk-ui/` nicht:

```sh
while read -r f; do
  c=$(grep -c 'crates/krk-core/' "$f"); u=$(grep -c 'crates/krk-ui/' "$f")
  [ "$c" -gt 0 ] && [ "$u" -eq 0 ] && echo "$f"
done < <alle offenen>
```

**46 Datensätze**, nicht die im Auftrag genannten rund 33. Der Unterschied ist der Schnitt:
ein erster Lauf über die bloßen Kistennamen (`krk-core` ohne `crates/`-Vorsatz) liefert 39,
und sieben weitere nennen beide Kisten im Text, führen aber keinen einzigen `krk-ui`-Pfad —
darunter die Kollisionsprüfung, die Probenziele des Kerns und die zwei Wettrennproben. Alle
46 sind gegen den heutigen Baum gelesen.

## Behoben

| Datensatz | Was getan | Datei |
|---|---|---|
| `260825-2127_*_eine-unlesbare-zugriffszeit-*` | die zwei Zip-Zusatzfelder hängen nur noch am Änderungsdatum; Probe mit Zugriffszeit von 1969 | `src/operation/zippen.rs`, `tests/operation.rs` |
| `260825-2127_*_die-kindproben-in-tests-zeit-rs-*` | `kind_ist_durchgelaufen` hält zusätzlich `test result: ok. 1 passed;` | `tests/zeit.rs` |
| `260818-0415_*_one-reflowed-line-in-umfang-rs-*` | Absatz auf 78 Zeichen umgebrochen | `src/verzeichnis/umfang.rs` |
| `260907-1225_*_die-begruendung-am-kopf-von-ohne-kombination-*` | vier Spaltenschalter von den zwei Sucheinstellungen getrennt | `tests/belegung.rs` |
| `260826-1303_*_der-rundlauf-von-readers-toml-*` | `readers.toml` geht jetzt über `geladene_leseprofile` zurück | `tests/ablage.rs` |
| `260826-1303_*_die-juengsten-entscheidungsdatensaetze-*` | die drei Entscheidungsdatensätze bekommen `geaendert_setzen` | `tests/leseprofil.rs` |
| `260826-1303_*_die-generationsprobe-filtert-ueber-eine-schleifeninvariante-*` | Weg 2: das Verwerfen wird wirklich gefahren, zweites Modell als Gegenprobe | `tests/verzeichnis.rs` |
| `260826-2152_*_die-sechs-fachlichen-assert-*` | `zusage: &str` in den Kindstarter, sieben tote `assert!` weg | `tests/gemeinsam/mod.rs`, `tests/{umfang,verzeichnis,git,leseprofil}.rs` |
| `260826-1302_*_die-msdos-zeitprobe-*` | Gegenprobe `zonenversatz(SOMMER) != zonenversatz(WINTER)` | `tests/operation.rs` |
| `260826-1302_*_die-abbruchprobe-des-stapels-*` | Sperre und fünf Versuche; die Zahl im Modulkopf durch ein Zählkommando ersetzt | `tests/operation.rs` |
| `260815-1019_*`, `260816-0055_*`, `260823-1436_*` | die Wettrennprobe misst Stillstand statt Gesamtdauer | `tests/text.rs` |
| `260826-1221_*_die-zwei-abbruchwege-des-lesefadens-*` | „an beiden nicht", mit Begründung; gemessen statt abgewogen | `src/verzeichnis/leser.rs` |
| `260826-1221_*_die-zweite-uebertragungsart-verliert-copyfile-excl-*` | `COPYFILE_EXCL` als eigene Konstante in beide Arten; Probe je Art | `src/verzeichnis/sys.rs`, `tests/operation.rs` |
| `260826-0139_*_zwei-behauptungen-der-c6-7-probe-*` | beide Meldungen nennen die Reihenfolge; Kopplung im Doc-Kommentar | `tests/leseprofil.rs` |
| `260826-0903_*_die-zeichengleichheit-der-zwei-werkbankpaare-*` | neue Probe `die_zwei_bloecke_eines_werkbankpaares_tragen_dieselben_zeilen` | `tests/leseprofil.rs` |
| `260826-0902_*_keine-probe-im-baum-haelt-die-vier-zahlen-der-flight-profile*` | neue Probe `die_zwei_flight_profile_bleiben_unter_ihren_zahlen` samt flight-Prüfordner | `tests/leseprofil.rs` |
| `260907-0858_*_zwei-proben-bleiben-unter-root-gruen-*` | beide Proben tragen `rechtesperre_haelt_oder_abbruch` | `tests/verzeichnis.rs` |
| `260907-2026_*_die-antwortzeile-zu-cmd-f-*` | Zusatz unter der `Answered:`-Zeile berichtigt die Begründung | Werkbank-Datensatz |

## Ohne Änderung geschlossen

- `260814-0912_*_neun-stellen-sprechen-weiter-von-vier-ablagedateien-*` und
  `260814-1002_*_die-erhebung-zu-vier-ablagedateien-*`: keine der neun Stellen steht mehr da,
  und `keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`
  (`tests/baum.rs`) rechnet die Zahl heute aus `Datei::ALLE`.
- `260825-2107_*_der-l7-entscheid-nennt-fuer-das-groesste-mitgelieferte-profil-fuenf-*`: der
  L7-Entscheid trägt seit dem 260907 den Marker `_i_`; die Begründung des Befunds — der
  Datensatz sei offen und werde als Entscheidungsgrundlage gelesen — trifft nicht mehr zu.

## Offen gelassen

- `260826-1221_*_der-freie-name-gibt-nach-tausend-versuchen-*`: `freier_name` hat seit dem
  Stand des Datensatzes einen zweiten Rufer in `krk-ui` (das Konfliktblatt).
- `260826-1223_*_lesen-trennt-den-deskriptormangel-nicht-*`: die Notizzettel-Hälfte liegt in
  `krk-core`, die Editor-Hälfte führt über `Abweisung` nach `krk-ui`.
- `260905-2254_*_das-zaehlkommando-fuer-ohne-warten-oeffnen-*`: die zwei Stellen in
  `sys.rs` sind behoben und geben sechs Zeilen für sechs Aufrufer aus; die Zeile in
  `CLAUDE.md` liegt außerhalb der Bahn.

Die übrigen 22 verlangen eine Wahl, die ihr Datensatz offen lässt; sie stehen im Bericht an
die aufrufende Stelle unter „braucht eine Entscheidung".

## Verifikation

```
cargo build --workspace                                    exit 0
cargo test --workspace                                     exit 0  (25 Ziele, 0 failed)
cargo clippy --workspace --all-targets -- -D warnings      exit 0
cargo fmt --all --check                                    exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps  exit 0
```

Alle fünf grün. Ein Zwischenstand um 08:10 hatte `cargo fmt --all --check` auf 1, an genau
einer Stelle in einer fremden Bahn (`crates/krk-ui/src/appkit/zwischenablage.rs:482`,
unbeglichene Arbeit der `krk-ui`-Bahn); sie ist inzwischen dort behoben. Über die vierzehn
Dateien dieser Bahn war `cargo fmt --check` durchgehend sauber.

## Neue Datensätze

Keine. Was neben der Bahn auffiel, ist als `Also seen:` an den jeweils vorhandenen offenen
Datensatz geschrieben (drei Stellen, siehe „Offen gelassen").
