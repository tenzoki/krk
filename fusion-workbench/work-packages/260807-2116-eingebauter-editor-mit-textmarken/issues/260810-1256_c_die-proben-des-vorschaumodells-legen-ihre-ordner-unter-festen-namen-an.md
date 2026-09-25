Die Proben des Vorschaumodells legen ihre Ordner unter festen Namen an

---

Sieben Proben in `crates/krk-ui/src/vorschaumodell.rs` legen ihren Pruefordner unter einem festen Namen im Temporaerverzeichnis an, etwa `krk-vorschau-probe-gross`. Zwei gleichzeitige Laeufe derselben Kiste schreiben damit in denselben Ordner, und keine der sieben raeumt ihn wieder ab: die Dateien bleiben nach dem Lauf stehen, eine von ihnen mit 1 MB.

---

**Schwere:** Niedrig
**Gefunden:** coder, bei der Behebung des Defekts `260810-1247`
**Betroffen:** `crates/krk-ui/src/vorschaumodell.rs`
**Domain:** code

## Belegstellen

Die sieben Stellen tragen alle dieselbe Form:

```rust
let ordner = std::env::temp_dir().join("krk-vorschau-probe-gross");
std::fs::create_dir_all(&ordner).expect("Probenordner");
```

Betroffen sind die Proben `eine_textdatei_erscheint_mit_ihrem_inhalt`, `ein_ordner_erscheint_als_metadaten`, `eine_textdatei_ueber_der_grenze_faellt_auf_die_metadaten`, `ein_bild_unter_der_grenze_kommt_mit_seinen_bytes`, `ein_bild_ueber_der_grenze_faellt_auf_die_metadaten`, `keine_utf8_datei_faellt_auf_die_metadaten` und `das_laden_erreicht_den_tab_der_es_bestellt_hat`.

## Fehlszenario

Zwei `cargo test`-Laeufe auf demselben Geraet, wie sie bei parallel arbeitenden Agenten vorkommen, treffen sich in `krk-vorschau-probe-gross`. Der eine Lauf schreibt `gross.txt` mit `TEXTGRENZE + 1` Bytes, der andere liest sie; loescht einer der beiden den Ordner, sieht der andere `laden` auf einen fehlenden Pfad und bekommt `Inhalt::Hinweis` statt `Inhalt::Metadaten`. Ein Fehlschlag daraus benennt nichts, was am Code falsch waere.

## Vorgeschlagene Behebung

Der `Pruefordner` derselben Datei, angelegt bei der Behebung von `260810-1247`, traegt Prozesskennung und Laufnummer im Namen und raeumt sich in `Drop` ab; er ist dieselbe Bauform wie `Pruefordner` in `crates/krk-core/tests/verzeichnis.rs`. Die sieben Proben darauf umzustellen ist eine mechanische Aenderung ohne Aussagewechsel.

**Nicht in derselben Aenderung behoben**, weil die Behebung von `260810-1247` an einem Diff gemessen wird, der die Umstellung des Lesewegs zeigen soll, und sieben umgeschriebene Proben daneben genau diesen Blick verstellen.

## Zustaendigkeit

`coder`.

---
Resolved: Alle sieben Proben nehmen jetzt den `Pruefordner` derselben Datei; ein
zweiter Mechanismus ist nicht entstanden, und nach dem Lauf bleibt kein Ordner
mit festem Namen stehen.

**Umgestellt ist auf den bestehenden `Pruefordner`** (`vorschaumodell.rs:913`),
angelegt bei der Behebung von `260810-1247`. Dazugekommen ist an ihm eine einzige
Zeile, der Lesezugriff `pfad(&self) -> &Path`, denn die sieben bilden einen Pfad
im Ordner und brauchten bisher nur den Ordner selbst; `Pruefordner` in
`krk-core/tests/verzeichnis.rs` traegt denselben Lesezugriff, die Bauform ist also
auch darin dieselbe. Die zwei Zeilen `temp_dir().join("krk-vorschau-probe-<zweck>")`
plus `create_dir_all` sind je Probe durch `Pruefordner::neu("<zweck>")` ersetzt,
die Zwecknamen sind unveraendert uebernommen, und keine Zusicherung hat sich
geaendert. Bei `ein_ordner_erscheint_als_metadaten` ist zusaetzlich der Vergleich
`metadaten.pfad == ordner.pfad()` nachgezogen, weil der Ordner jetzt hinter dem
Waechter steht.

**Der `Pruefordner` von `krk-core` ist von `krk-ui` aus nicht erreichbar**, und
deshalb war er auch schon am 260810-1247 nicht der Weg: er steht in
`crates/krk-core/tests/verzeichnis.rs`, also in einem Testziel und nicht in der
Bibliothek. Ein Testziel ist eine eigene Kiste, die niemand einbinden kann. Ihn zu
teilen verlangte einen Umzug in die Bibliothek hinter `#[cfg(test)]` (dort greift
`cfg(test)` bei einem fremden Nutzer nicht) oder eine eigene Hilfskiste als
`dev-dependency`; beides ist mehr Umbau, als die sieben Proben wert sind. Die
kleinste Form an der Stelle, wo `krk-ui` sie braucht, stand bereits in der Datei.

**Nachgewiesen, dass nichts stehenbleibt.** Vor dem Lauf lagen die sieben Ordner
mit festem Namen unter `$TMPDIR` (`krk-vorschau-probe-text`, `-ordner`, `-gross`,
`-bild-klein`, `-bild-gross`, `-binaer`, `-faden`), Rueckstand aelterer Laeufe; sie
sind vor der Messung geloescht worden. Nach `cargo test --workspace` und nach einem
zweiten, gezielten Lauf der 19 Proben des Vorschaumodells (alle 19 gruen, darunter
die sieben) findet `ls -d "$TMPDIR"krk-vorschau-probe-*` **keinen Eintrag**. Ein
`krk-vorschau-probe-<zweck>-<pid>-<laufnummer>` liegt nur, solange eine Probe
laeuft.

Nebenbefund, als eigener Datensatz abgelegt: dieselbe Bauform steht zwoelfmal im
Baum, unter zwei Namen — `260810-1330_o_derselbe-selbstabraeumende-pruefordner-
steht-zwoelfmal-im-baum.md`. Hier ist bewusst keine Zusammenlegung versucht,
sondern die vorhandene Form genutzt.

Verification: `cargo build --workspace` exit 0, `cargo test --workspace` exit 0,
`cargo clippy --workspace --all-targets` exit 0,
`cargo fmt -p krk-ui -- --check` exit 0, `cargo fmt -p krk-core -- --check` exit 0.
