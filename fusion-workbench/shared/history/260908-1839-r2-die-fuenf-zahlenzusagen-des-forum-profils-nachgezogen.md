# R2 — die fünf Zahlenzusagen nachgezogen, die das Forum-Profil falsch gemacht hat

**Status:** Complete
**Filed by:** coder, Kai Stalmann <kai@stalmann.org>
**Datum:** 260908-1839

## Auftrag

Die Bahn R1 hat `resources/default-readers.toml` um ein dreizehntes Profil und
je eine Zeile in den zwei gekoppelten Wurzelprofilen erweitert und ist an der
Grenze zu `crates/` stehen geblieben; `cargo test -p krk-core` kam mit 101
zurück. Nachzuziehen waren fünf reine Zählwerte in drei Dateien, nach dem
durchgemessenen Befund
`260908-1754_*_das-forum-profil-macht-fuenf-zahlenzusagen-in-drei-dateien-unter-crates-falsch.md`.
`resources/default-readers.toml` durfte nicht angefasst werden und ist
unangetastet.

## Geändert

`crates/krk-core/src/ablage/leseprofile.rs`, `crates/krk-core/tests/ablage.rs`,
`crates/krk-core/tests/leseprofil.rs`.

1. **Drei Zählstellen `12` → `13`** — zwei Zusicherungen im Lib-Ziel, eine in
   `tests/ablage.rs`, eine im Helfer `ausgelieferte()` von
   `tests/leseprofil.rs`. Jede vorher am Baum nachgezählt
   (`grep -c '^\[\[' resources/default-readers.toml` → 13).
2. **Der Prüfordner bekommt `shared/forum`** — `werkbankbestand` legt den
   Ordner mit einem Eintrag an. Nicht die Zusicherung
   `leselaeufe == projektorte.len() + 1` wurde umgeschrieben: ein genannter
   Ort, den es nicht gibt, wird gar nicht gelesen, und eine Zusicherung, die
   ihm ausweicht, belegt den Halbsatz „plus einen Lauf für die Erkennung"
   nicht mehr.
3. **Sieben Zusicherungen in `tests/leseprofil.rs`** — die Zeile „Nachrichten"
   in den zwei Beschriftungslisten, `Wert::Zahl(1)` in den drei Wertelisten,
   `fusion-workbench/shared/forum` in der Ortsliste, die Haushalte auf `(4, 5)`
   und `(5, 5)`. Die Öffnungen bleiben bei fünf: `zaehlung` liest den
   Verzeichniseintrag und öffnet keine Datei.
4. **Der Doc-Kommentar über
   `die_drei_groessten_mitgelieferten_profile_bleiben_unter_ihren_zahlen`** ist
   samt seiner Tabelle mitgezogen, dazu die Köpfe von `werkbankbestand`,
   `projektwurzel` und der Probe zu C5.8.

## Zahl oder Zählkommando, je Stelle entschieden

**Die Zusicherungen behalten ihre Zahl.** Sie prüfen sie, statt sie zu nennen,
und das ist in diesem Projekt der Sinn einer fest verdrahteten Zahl: ein
vierzehntes Profil soll auffallen und bewusst eingeordnet werden. Eine aus
`AUSLIEFERUNGSTEXT` abgeleitete Zahl wäre grün geblieben und hätte die Stelle
stillgelegt.

**Die Prosa verliert ihre Zahl.** Zahlwörter in Abbruchmeldungen sind neben
einem `assert_eq!`, das `left` und `right` ohnehin ausgibt, die zweite Kopie
derselben Angabe — und die Kopie ist es, die driftet. Dasselbe gilt für die
Zeilenzahlen der zwei Wurzelprofile („dieselben sieben Zeilen") und für die
Nummer eines Profils in der Datei. Der Kopf von
`keine_mitgelieferte_zeile_nennt_typ_oder_versteckt` nennt an ihrer Stelle das
Zählkommando aus `CLAUDE.md`.

**Die gemessenen Haushaltszahlen bleiben Zahlen**, auch in der Prosa: sie
stehen dort als Erläuterung der Zusicherung unmittelbar darunter, und die hält
sie wahr.

## Zwei Befunde nebenbei

- Der Kopf von `projektwurzel` sagte „das achte mitgelieferte Profil". Mit dem
  dreizehnten Profil ist „Projektwurzel mit fusion-Werkbank" das neunte. Der
  Kopf nennt das Profil jetzt beim Namen — genau die Regel, die
  `profil_der_auslieferung` für sich schon ausschreibt. Kein eigener Datensatz:
  die Stelle gehört zum Nachzug.
- Der Kopf der Probe zu C5.9 nannte „sechs Profile mit Pfadmuster" und „sechs
  mit Kennzeichendatei". Die Datei trug schon am Tag der Niederschrift
  (`56e5c2d`, 260906) sieben und fünf. Das ist ein Befund vor dieser
  Änderung und trägt einen eigenen Datensatz,
  `260908-1839_*_der-doc-kommentar-zu-c5-9-nennt-sechs-pfadmuster-und-sechs-kennzeichen-die-datei-trug-bei-seiner-niederschrift-sieben-und-fuenf.md`,
  geschlossen mit derselben Änderung.

## Abnahme

Zweimal gefahren. Der erste Lauf lief noch, als zwei Zeilenumbrüche in
Doc-Kommentaren nachgezogen wurden, und belegt deshalb nicht den Endstand; der
zweite ist vollständig auf dem Endstand gefahren. Alle fünf Kommandos mit
Exit 0:

```
cargo build --workspace                                  exit 0
cargo clippy --workspace --all-targets -- -D warnings    exit 0
cargo fmt --all --check                                  exit 0
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps  exit 0
cargo test --workspace                                   exit 0
```

23 Probenreihen, null Fehlschläge. Nicht committet.
