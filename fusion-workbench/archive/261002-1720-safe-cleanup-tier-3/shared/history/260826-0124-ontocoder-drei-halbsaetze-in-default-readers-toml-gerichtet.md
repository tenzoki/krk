# Drei Halbsätze in `default-readers.toml` gerichtet

**Agent:** ontocoder
**Datum:** 2026-08-26, ab 01:05
**Aufgabe:** S-1, Runde 3 der Sitzung zur Runde 18 — die drei niedrigen Befunde der Nachdurchsicht
`fusion-workbench/shared/reviews/260825-2233-ontorev-nachdurchsicht-der-leseprofile-nach-der-behebungsrunde.md`
(N1, N2, N3), Datensätze `shared/issues/260825-2233_*_*.md`
**Status:** Complete

## Was geändert ist

**N1 — der Vorbehalt für `pfad`** (`resources/default-readers.toml:61-67`). Der Absatz über den
`[[profil]]`-Block sagte, `kennzeichnen` statt `kennzeichen` lasse das Profil ohne Erkennungsmuster
zurück und falle in die zweite Reichweite. Das gilt nur ohne `pfad` daneben. Er sagt jetzt: „nimmt
dem Profil sein Erkennungsmuster, und es fällt in die zweite Reichweite oder greift, steht ein
`pfad` daneben, still über diesen allein". Möglichkeit 1 des Datensatzes; am Mechanismus ist nichts
angefasst.

**N2 — das falsche Beispiel** (`:628-634`). Der Halbsatz „also der Zustand vor `/fusion:setup`" ist
gestrichen. Setup legt das Verzeichnis mit `mkdir -p` samt Unterordnern an oder hält davor ganz an,
und git führt keine leeren Verzeichnisse; der Zustand kommt auf keinem der zwei Wege vor. Die zwei
Beispiele, die vorkommen, stehen weiter da: ein leeres Verzeichnis dieses Namens und eine Datei
dieses Namens. Der Preis daneben ist unverändert.

**N3 — nichts geändert, nur geprüft** (`:237-240`). Die Herleitung der Vier steht als: das
Wurzelprofil kostet drei Läufe, das Projektwurzelprofil „mit denselben sieben Zeilen vier: dort
trägt jede Zeile eine Ortsangabe, und den erkannten Ordner liest allein die Erkennung". Sie trägt
die Drei über „denselben sieben Zeilen" mit und schreibt die zwei Bedingungen des Erkennungslaufs
aus. Gegen den vierten Prüfordner gehalten, den `coder` parallel in
`crates/krk-core/tests/leseprofil.rs` angelegt hat: er hält `(4, 5)` genau, prüft die drei genannten
Orte (`fusion-workbench`, `fusion-workbench/circles`, `fusion-workbench/shared/issues`) einzeln und
eigens, dass keiner davon der erkannte Ordner selbst ist, und hält daneben `orte.len() + 1`. Das ist
dieselbe Herleitung. Kein Eingriff nötig.

**Die Länge.** 801 Zeilen vorher, 801 nachher: N1 kostet eine Zeile, N2 gibt eine zurück.

## Gemessen, nicht hingesehen

Die Messhilfe der Durchsicht (`scratchpad/profilprobe`, lädt über `toml::from_str` und
`leseprofil::datei::pruefen`, fährt `zusammenfassen_gezaehlt`) neu übersetzt und gefahren.

Für N1 beide Lagen an abgewandelten Fassungen:

| Fassung | gemessen |
|---|---|
| `kennzeichnen = 'x'` neben dem `pfad` der zwei Speicherprofile | 12 Profile, keine Meldung, `shared/history` bekommt sein Profil, 1 Leselauf, 10 Öffnungen |
| `kennzeichnen` statt `kennzeichen` am Wurzelprofil (kein `pfad`) | 11 Profile, Meldung „es nennt weder ein Pfadmuster noch eine Kennzeichendatei", `fusion-workbench` ohne Profil |

Für N3 an den wirklichen Orten: `krk` 4 Leseläufe / 4 Öffnungen, `krk/fusion-workbench` 3 / 4,
`krk/fusion-workbench/shared` 10 / 0.

Die geänderte Datei durch dieselbe Hilfe geladen: 12 Profile, keine Meldung, dieselben Zahlen.

## Was ausdrücklich nicht angefasst wurde

`crates/` (`coder` arbeitet parallel in `crates/krk-core/tests/leseprofil.rs`), `README.md`,
`Cargo.toml`, `CLAUDE.md`, das Ereignisprotokoll. Kein Git-Kommando außer `git status --short`
und `git diff --stat` auf die eine Probendatei.

**Nicht behoben, weil sie `coder` gehört:** der Modulkopf von `crates/krk-core/src/leseprofil/datei.rs`
trägt unter „Wo `deny_unknown_fields` steht und wo nicht" denselben Satz mit derselben Lücke, die N1
in der Profildatei geschlossen hat. Die Nachdurchsicht hält es als Beobachtung für `coderev` fest;
der Datensatz zu N1 nennt die Stelle.

## Abnahme

`Verification: make check — exit 0` (mit `PATH="$HOME/.cargo/bin:$PATH"`, „alle vier gruen").
`cargo test -p krk-core --lib leseprofile`: 10 grün, `die_eingebettete_fassung_besteht_ihre_eigene_pruefung`
darunter. `cargo fmt` nicht nötig: geändert ist eine TOML-Datei, keine Rust-Datei.

Die drei Datensätze stehen auf `_c_` mit Auflösungsvermerk.
