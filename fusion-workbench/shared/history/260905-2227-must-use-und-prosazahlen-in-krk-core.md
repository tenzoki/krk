# Behebungslauf: `#[must_use]` und Prosazahlen in `krk-core`

**Agent:** coder
**Status:** Complete
**Umfang:** ausschliesslich `crates/krk-core/` und die Defektdatensaetze unter
`fusion-workbench/shared/issues/`
**Baumstand beim Beginn:** `28c4a47`

## Verification

```
cargo fmt -p krk-core -- --check                          → 0
cargo clippy -p krk-core --all-targets -- -D warnings      → 0
cargo test -p krk-core                                     → 0 (20 Probenziele, alle gruen)
cargo clippy -p krk-ui -p krk-bench --all-targets -- -D warnings → 0
```

Der letzte Lauf ist mitgenommen, weil neue `#[must_use]`-Marken in `krk-core` einen
Rufer in einer der Nachbarkisten rot machen koennten. Keiner tut es: kein Weg dort laesst
einen der neu markierten Werte fallen.

## Gruppe A — die vier `#[must_use]`-Datensaetze

Alle vier geschlossen. Die Marke steht jetzt an jedem reinen Leser und Erzeuger der
genannten Module. Zwei Bauformen sind dabei bewusst gewaehlt:

- **Am Typ statt an den Funktionen**, wo ein Typ mehrere Erzeuger hat: `Geladen<T>`
  (`ablage/mod.rs`) deckt damit alle fuenf Ladewege und jeden sechsten, `Lauf`
  (`operation/fortschritt.rs`) deckt `operation::starten` und jeden kuenftigen Erzeuger.
- **Nicht doppelt**, wo die Standardbibliothek die Marke schon traegt: `Result` und
  `Option` sind selbst `#[must_use]`, und ein zweiter nackter Vermerk bricht an Clippys
  `double_must_use`. Dasselbe gilt fuer `impl Iterator`; die drei Stellen dort tragen
  deshalb einen eigenen Begruendungstext.

Zahl der Attribute im Kern danach: `grep -rEc '^\s*#\[must_use' crates/krk-core/src`.

## Gruppe B — dreizehn von vierzehn Prosadatensaetzen

Der tragende Zug ist ueberall derselbe: wo die Auskunft anderswo im Baum steht, steht
jetzt der Zeiger dorthin statt der Zahl. Wo die Zahl die Aussage traegt, steht sie unter
einer Probe.

**Die eine neue Probe:** `keine_prosastelle_der_ablage_nennt_eine_andere_zahl_von_ablagedateien`
in `crates/krk-core/tests/baum.rs`. Sie zieht die Doc-Kommentare unter
`crates/krk-core/src/ablage/` zeilenuebergreifend zusammen, sucht jedes Zahlwort
unmittelbar vor `Ablagedateien`, `Dateien`, `Nutzdateien` oder `TOML-Dateien` und haelt es
gegen `Datei::ALLE.len()` beziehungsweise gegen die Zahl der Werte mit `Format::Toml`. Der
Zusammenzug ueber die Zeilengrenze ist der Punkt: an genau dieser Bauform sind fuenf
Erhebungen in Folge gescheitert. Gegengeprueft mit einem eingesetzten „vier" in
`sperre.rs` — die Probe wird rot und nennt Datei und Wortlaut.

**Die zweite neue Probe:**
`ein_verschriebener_schluessel_im_profilblock_faellt_nur_ohne_pfad_daneben_auf` in
`crates/krk-core/tests/leseprofil.rs` misst die zwei Lagen des einen Tisches ohne
`deny_unknown_fields` — die laute und die stille. Die stille war bis dahin durch nichts
gemessen.

**Berichtigt statt ersetzt oder verankert, an genau einer Stelle:** die „vier Lagen" im
Doc-Kommentar von `die_meldung_unterscheidet_die_fuenf_lagen_und_bleibt_einzeilig`
(`tests/ablage.rs`) stehen jetzt auf fuenf. Das traegt, weil `Beiseite` eine vollstaendige
Fallunterscheidung ohne Auffangzweig ist und die Probe ihre Werte einzeln aufzaehlt: eine
sechste Variante haelt den Bau an. Der Uebersetzer ist dort die Verankerung.

## Offen geblieben

`shared/issues/260826-1933_*_zwei-prosastellen-an-ohne-warten-oeffnen-zaehlen-fuenf-rufer-und-nennen-den-schwungleser-als-einzigen-file-open-oeffner.md`
steht weiter auf `_o_` und ist unangetastet. Die zwei Stellen in
`crates/krk-core/src/verzeichnis/sys.rs`, die der Titel nennt, sind mit diesem Lauf
geraeumt — der Ordinalsatz nennt den sechsten Rufer und verweist auf das Zaehlkommando,
und „einziger Oeffner" ist auf „einziger Oeffner **dieser Datei**" eingeengt, samt der zwei
`File::open` im Kopieren und im Entpacken. Die dritte Fundstelle des Datensatzes liegt in
`CLAUDE.md`, und die lag ausserhalb des Umfangs dieses Laufs. Der Datensatz bleibt offen,
bis auch sie nachgezogen ist.

## Nebenbei geaendert, ohne eigenen Datensatz

- `ablage/mod.rs`, Modulkopf: „Sieben Module, in der Reihenfolge, in der ein Wert sie
  durchlaeuft" ist durch „Die Module, … — die Skizze ist die Aufzaehlung" ersetzt. Die
  Zahl war **nicht** falsch, aber mehrdeutig: sieben ohne `mod` selbst, acht mit ihm, und
  die Skizze darunter zeigt acht Kaesten.
- `ablage/mod.rs`, `Grund::NichtAnlegbar`: „Bei den drei uebrigen ist eine fehlende Datei
  der erste Start" ist auf „Bei jeder anderen Ablagedatei" gezogen. Dieselbe
  Mehrdeutigkeit — drei uebrige TOML-Dateien oder fuenf uebrige Ablagedateien.
- Einige mehrdeutige Wendungen unter `ablage/` sind auf eindeutige Hauptwoerter gezogen,
  damit die neue Probe sie ueberhaupt beurteilen kann.
- `tests/baum.rs`, Doc-Kommentar von `nur_benannte_dateien_erreichen_das_atomare_schreiben`:
  zitierte die Stelle „einer der vier Dateien" aus `ablage/mod.rs` mit und ist mitgezogen.
